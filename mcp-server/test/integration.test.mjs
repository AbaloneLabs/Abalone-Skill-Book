#!/usr/bin/env node
/**
 * Integration test for the Abalone Skills MCP server.
 *
 * Spawns the server over stdio and exercises the session-gated flow:
 *   1. list_skills before reading guide  -> should be gated
 *   2. get_usage_guide                   -> should return guide + session id
 *   3. list_skills with session          -> should work
 *   4. get_skill                         -> should return content
 *   5. create_skill + get_skill + delete -> round-trip
 *   6. search_skills                          -> should find relevant skills
 *   7. search_skills (no matches)             -> empty result message
 */

import { spawn } from "node:child_process";
import { randomUUID } from "node:crypto";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const __dirname = dirname(fileURLToPath(import.meta.url));
const SERVER_PATH = join(__dirname, "..", "dist", "index.js");

let passed = 0;
let failed = 0;

function assert(condition, message) {
  if (condition) {
    console.log(`  ✓ ${message}`);
    passed++;
  } else {
    console.error(`  ✗ ${message}`);
    failed++;
  }
}

class McpClient {
  constructor(proc) {
    this.proc = proc;
    this.buffer = "";
    this.pending = new Map();
    this.id = 0;
    this.proc.stdout.on("data", (chunk) => this.onData(chunk));
  }

  onData(chunk) {
    this.buffer += chunk.toString();
    let idx;
    while ((idx = this.buffer.indexOf("\n")) >= 0) {
      const line = this.buffer.slice(0, idx).trim();
      this.buffer = this.buffer.slice(idx + 1);
      if (!line) continue;
      try {
        const msg = JSON.parse(line);
        if (msg.id && this.pending.has(msg.id)) {
          const { resolve } = this.pending.get(msg.id);
          this.pending.delete(msg.id);
          resolve(msg);
        }
      } catch {
        // ignore non-JSON
      }
    }
  }

  send(method, params = {}) {
    const id = ++this.id;
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      const msg = JSON.stringify({ jsonrpc: "2.0", id, method, params });
      this.proc.stdin.write(msg + "\n");
      setTimeout(() => {
        if (this.pending.has(id)) {
          this.pending.delete(id);
          reject(new Error(`Timeout waiting for response to ${method}`));
        }
      }, 5000);
    });
  }

  notify(method, params = {}) {
    const msg = JSON.stringify({ jsonrpc: "2.0", method, params });
    this.proc.stdin.write(msg + "\n");
  }

  close() {
    this.proc.kill();
  }
}

async function main() {
  const proc = spawn("node", [SERVER_PATH], {
    stdio: ["pipe", "pipe", "inherit"],
  });

  const client = new McpClient(proc);

  // Initialize handshake
  await client.send("initialize", {
    protocolVersion: "2024-11-05",
    capabilities: {},
    clientInfo: { name: "test", version: "0.0.0" },
  });
  client.notify("notifications/initialized", {});

  console.log("\n1. list_skills before reading guide (should be GATED)");
  {
    const res = await client.send("tools/call", {
      name: "list_skills",
      arguments: {},
    });
    const text = res.result?.content?.[0]?.text ?? "";
    assert(
      text.includes("must read the usage guide"),
      "Gate message returned when guide not read"
    );
    assert(res.result?.isError === true, "Returns isError: true");
  }

  console.log("\n2. get_usage_guide (should return guide + session id)");
  let sessionId;
  {
    const res = await client.send("tools/call", {
      name: "get_usage_guide",
      arguments: {},
    });
    const text = res.result?.content?.[0]?.text ?? "";
    assert(text.includes("Abalone Skills"), "Guide title present");
    assert(text.includes("Repository Structure"), "Structure section present");
    assert(text.includes("session id:"), "Session id returned");
    const match = text.match(/session id:\s*([a-f0-9-]+)/i);
    sessionId = match?.[1];
    assert(!!sessionId, `Got session id: ${sessionId}`);
  }

  console.log("\n3. list_skills with session (should WORK)");
  {
    const res = await client.send("tools/call", {
      name: "list_skills",
      arguments: { __sessionId: sessionId, path: "programmer/rust/core" },
    });
    const text = res.result?.content?.[0]?.text ?? "";
    assert(
      text.includes("ownership-and-borrowing"),
      "Finds ownership-and-borrowing skill"
    );
    assert(!res.result?.isError, "No error");
  }

  console.log("\n4. get_skill (should return full content)");
  {
    const res = await client.send("tools/call", {
      name: "get_skill",
      arguments: {
        __sessionId: sessionId,
        path: "programmer/rust/core/ownership-and-borrowing",
      },
    });
    const text = res.result?.content?.[0]?.text ?? "";
    assert(text.includes("Ownership and Borrowing"), "Skill title present");
    assert(text.includes("Self-Check"), "Self-check section present");
  }

  console.log("\n5. create_skill + get_skill + delete (round-trip)");
  const testPath = "programmer/rust/core/test-skill-" + randomUUID().slice(0, 8);
  {
    const createRes = await client.send("tools/call", {
      name: "create_skill",
      arguments: {
        __sessionId: sessionId,
        path: testPath,
        name: "test-skill",
        description: "A test skill for integration testing.",
        body: "# Test Skill\n\n## Core Rules\n- This is a test.",
      },
    });
    assert(
      createRes.result?.content?.[0]?.text?.includes("created"),
      "Skill created"
    );

    const getRes = await client.send("tools/call", {
      name: "get_skill",
      arguments: { __sessionId: sessionId, path: testPath },
    });
    assert(
      getRes.result?.content?.[0]?.text?.includes("Test Skill"),
      "Created skill is readable"
    );

    const delRes = await client.send("tools/call", {
      name: "delete_skill",
      arguments: { __sessionId: sessionId, path: testPath },
    });
    assert(
      delRes.result?.content?.[0]?.text?.includes("deleted"),
      "Skill deleted"
    );
  }

  console.log("\n6. search_skills (should find relevant skills)");
  {
    const res = await client.send("tools/call", {
      name: "search_skills",
      arguments: {
        __sessionId: sessionId,
        query: "ownership",
      },
    });
    const text = res.result?.content?.[0]?.text ?? "";
    assert(
      text.includes("ownership-and-borrowing"),
      "Search finds ownership-and-borrowing skill"
    );
    assert(!res.result?.isError, "No error");
  }

  console.log("\n7. search_skills with no matches");
  {
    const res = await client.send("tools/call", {
      name: "search_skills",
      arguments: {
        __sessionId: sessionId,
        query: "nonexistent-term-xyz123",
      },
    });
    const text = res.result?.content?.[0]?.text ?? "";
    assert(text.includes("No skills matched"), "Empty search returns no-match message");
  }

  console.log("\n8. Invalid path rejection");
  {
    const res = await client.send("tools/call", {
      name: "create_skill",
      arguments: {
        __sessionId: sessionId,
        path: "bad",
        name: "x",
        description: "x",
        body: "x",
      },
    });
    const text = res.result?.content?.[0]?.text ?? "";
    assert(text.includes("Invalid path"), "Invalid path rejected");
  }

  client.close();

  console.log(`\n${"=".repeat(50)}`);
  console.log(`Passed: ${passed}  Failed: ${failed}`);
  process.exit(failed > 0 ? 1 : 0);
}

main().catch((err) => {
  console.error("Test error:", err);
  process.exit(1);
});
