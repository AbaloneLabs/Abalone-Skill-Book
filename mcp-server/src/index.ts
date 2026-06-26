#!/usr/bin/env node

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import {
  createSession,
  getSession,
  markGuideRead,
  hasValidGuideRead,
  SESSION_TTL_DESCRIPTION,
} from "./session.js";
import { USAGE_GUIDE } from "./guide.js";
import {
  listSkills,
  getSkill,
  createSkill,
  updateSkill,
  deleteSkill,
  isValidPath,
} from "./skills.js";
import { submitFeedback } from "./feedback.js";

/**
 * Abalone Skills MCP Server
 *
 * Provides CRUD access to the skills library with a session-gated usage guide.
 * Agents must read the usage guide at least once (and again every 3 days)
 * before using the other tools.
 */

const server = new Server(
  { name: "abalone-skills", version: "0.1.0" },
  { capabilities: { tools: {} } }
);

// ---------------------------------------------------------------------------
// Session helpers
// ---------------------------------------------------------------------------

/**
 * Resolve or create a session from the optional __sessionId argument that
 * clients pass with every tool call. Returns the session id and whether the
 * guide gate is satisfied.
 */
function resolveSession(args: Record<string, unknown>): {
  sessionId: string;
  gated: boolean;
} {
  const providedId = typeof args.__sessionId === "string" ? args.__sessionId : null;
  const session = providedId ? getSession(providedId) : null;
  const id = session?.id ?? createSession().id;
  const gated = !hasValidGuideRead(id);
  return { sessionId: id, gated };
}

/** Standard gate message returned when the guide hasn't been read. */
function gateMessage(sessionId: string): string {
  return (
    `You must read the usage guide before using this tool. ` +
    `Call the \`get_usage_guide\` tool first (sessionId: ${sessionId}). ` +
    `This requirement resets every ${SESSION_TTL_DESCRIPTION}.`
  );
}

// ---------------------------------------------------------------------------
// Tool definitions
// ---------------------------------------------------------------------------

const TOOLS = [
  {
    name: "get_usage_guide",
    description:
      "Read the usage guide explaining the repository structure, skill format, and how to use each tool. MUST be called at least once before using other tools, and again every 3 days. Returns the guide text and your session id.",
    inputSchema: {
      type: "object",
      properties: {
        __sessionId: {
          type: "string",
          description: "Your session id from a previous call. Omit on first contact.",
        },
      },
    },
  },
  {
    name: "list_skills",
    description:
      "List available skills, optionally filtered by a path prefix (e.g. 'programmer/rust'). Returns each skill's path, name, and description.",
    inputSchema: {
      type: "object",
      properties: {
        path: {
          type: "string",
          description: "Optional path prefix to filter (e.g. 'programmer/rust/blockchain').",
        },
        __sessionId: { type: "string" },
      },
    },
  },
  {
    name: "get_skill",
    description:
      "Read a single skill's full content (frontmatter + body).",
    inputSchema: {
      type: "object",
      properties: {
        path: {
          type: "string",
          description: "The skill path, e.g. 'programmer/rust/core/ownership-and-borrowing'.",
        },
        __sessionId: { type: "string" },
      },
      required: ["path"],
    },
  },
  {
    name: "create_skill",
    description:
      "Create a new skill. The path must follow role/stack/domain/[framework/]skill-name. Returns the created skill.",
    inputSchema: {
      type: "object",
      properties: {
        path: { type: "string" },
        name: { type: "string", description: "Skill name (matches frontmatter 'name')." },
        description: {
          type: "string",
          description: "Frontmatter 'description' - what the skill covers and when to use it.",
        },
        body: { type: "string", description: "Markdown body of the skill (after frontmatter)." },
        __sessionId: { type: "string" },
      },
      required: ["path", "name", "description", "body"],
    },
  },
  {
    name: "update_skill",
    description:
      "Update an existing skill's description and/or body. Provide only the fields you want to change.",
    inputSchema: {
      type: "object",
      properties: {
        path: { type: "string" },
        description: { type: "string" },
        body: { type: "string" },
        __sessionId: { type: "string" },
      },
      required: ["path"],
    },
  },
  {
    name: "delete_skill",
    description: "Delete a skill (removes its directory).",
    inputSchema: {
      type: "object",
      properties: {
        path: { type: "string" },
        __sessionId: { type: "string" },
      },
      required: ["path"],
    },
  },
  {
    name: "submit_feedback",
    description:
      "Submit feedback or a correction for a skill. Feedback is queued for review and does not modify the skill directly. Use this to report issues, suggest improvements, or note missing pitfalls.",
    inputSchema: {
      type: "object",
      properties: {
        path: { type: "string", description: "The skill path the feedback relates to." },
        content: { type: "string", description: "The feedback content." },
        __sessionId: { type: "string" },
      },
      required: ["path", "content"],
    },
  },
];

// ---------------------------------------------------------------------------
// List tools handler
// ---------------------------------------------------------------------------

server.setRequestHandler(ListToolsRequestSchema, async () => {
  return { tools: TOOLS };
});

// ---------------------------------------------------------------------------
// Call tool handler
// ---------------------------------------------------------------------------

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args = {} } = request.params;

  // get_usage_guide is always allowed (it's how you unlock the gate).
  if (name === "get_usage_guide") {
    const { sessionId } = resolveSession(args);
    markGuideRead(sessionId);
    return {
      content: [
        {
          type: "text",
          text: `${USAGE_GUIDE}\n\n---\n**Your session id: ${sessionId}**\n` +
            `Include this as \`__sessionId\` in subsequent calls. ` +
            `You will need to re-read this guide every ${SESSION_TTL_DESCRIPTION}.`,
        },
      ],
    };
  }

  // All other tools are gated.
  const { sessionId, gated } = resolveSession(args);
  if (gated) {
    return {
      content: [
        { type: "text", text: gateMessage(sessionId) },
      ],
      isError: true,
    };
  }

  try {
    switch (name) {
      case "list_skills": {
        const skills = await listSkills(
          typeof args.path === "string" ? args.path : undefined
        );
        if (skills.length === 0) {
          return text("No skills found. Try a broader path or omit the path filter.");
        }
        return text(
          skills
            .map((s) => `**${s.path}**\n  ${s.name}: ${s.description}`)
            .join("\n\n")
        );
      }

      case "get_skill": {
        const skill = await getSkill(String(args.path));
        return text(
          `# ${skill.name}\n\n` +
          `**Path:** ${skill.path}\n` +
          `**Description:** ${skill.description}\n\n` +
          `---\n\n${skill.body}`
        );
      }

      case "create_skill": {
        const path = String(args.path);
        if (!isValidPath(path)) {
          return error(
            `Invalid path: '${path}'. Must follow role/stack/domain/[framework/]skill-name with at least 4 segments.`
          );
        }
        const skill = await createSkill(
          path,
          String(args.name),
          String(args.description),
          String(args.body)
        );
        return text(`Skill created at ${skill.path}`);
      }

      case "update_skill": {
        const path = String(args.path);
        const changes: { description?: string; body?: string } = {};
        if (typeof args.description === "string") changes.description = args.description;
        if (typeof args.body === "string") changes.body = args.body;
        const skill = await updateSkill(path, changes);
        return text(`Skill updated at ${skill.path}`);
      }

      case "delete_skill": {
        await deleteSkill(String(args.path));
        return text(`Skill deleted at ${args.path}`);
      }

      case "submit_feedback": {
        const fb = await submitFeedback(String(args.path), String(args.content));
        return text(
          `Feedback submitted (id: ${fb.id}). It has been queued for review and will be considered for inclusion. Thank you!`
        );
      }

      default:
        return error(`Unknown tool: ${name}`);
    }
  } catch (err) {
    return error(err instanceof Error ? err.message : String(err));
  }
});

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

function text(t: string) {
  return { content: [{ type: "text", text: t }] };
}

function error(t: string) {
  return { content: [{ type: "text", text: t }], isError: true };
}

// ---------------------------------------------------------------------------
// Start
// ---------------------------------------------------------------------------

async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error("Abalone Skills MCP server running on stdio");
}

main().catch((err) => {
  console.error("Fatal error:", err);
  process.exit(1);
});
