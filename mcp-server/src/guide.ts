/**
 * The usage guide returned to agents on first contact (or after the session TTL).
 *
 * This explains the repository structure, what a skill should contain, and how
 * to use each MCP tool with a concrete example.
 */

export const USAGE_GUIDE = `# Abalone Skills - Usage Guide

This is a curated library of skills that remind AI agents what to watch for when performing specific tasks. Agents already know *how* to do things; these skills exist to **remind** them of pitfalls and rules that are easy to lose track of while juggling complex work.

## Repository Structure

\`\`\`
<role>/<stack>/<domain>/[<framework>/]<skill-name>/SKILL.md
\`\`\`

- **role** - the agent's responsibility (e.g. \`programmer\`)
- **stack** - the technology (e.g. \`rust\`)
- **domain** - the area of work (e.g. \`core\`, \`web\`, \`gamedev\`)
- **framework** *(optional)* - when a domain splits by framework (e.g. \`axum\`, \`bevy\`)
- **skill** - the specific concern (e.g. \`ownership-and-borrowing\`)

Example path: \`programmer/rust/blockchain/solana/my-skill/SKILL.md\`

## What a Skill Should Contain

Every skill is a single \`SKILL.md\` with YAML frontmatter and a markdown body:

\`\`\`markdown
---
name: my-skill
description: What this skill reminds you about and when to use it. This is the trigger - include what it does and specific contexts for when to use it.
---

# My Skill

## Core Rules (never violate)
- ...

## Common Traps
- ... (with brief examples)

## Self-Check
- [ ] checklist item
\`\`\`

Skills are **reminder checklists, not tutorials**. Write invariants and pitfalls, not explanations of concepts the agent already knows.

## Available Tools

### list_skills
List available skills, optionally filtered by path prefix.

Example:
\`\`\`
list_skills(path: "programmer/rust/blockchain")
\`\`\`

### get_skill
Read a single skill's full content.

Example:
\`\`\`
get_skill(path: "programmer/rust/core/ownership-and-borrowing")
\`\`\`

### create_skill
Create a new skill. Requires the path, name, description, and body content.

Example:
\`\`\`
create_skill(
  path: "programmer/rust/blockchain/solana/account-model",
  name: "account-model",
  description: "Solana account model invariants to preserve when writing programs. Use when designing account layouts, rent rules, or PDA derivation.",
  body: "# Account Model\\n\\n## Core Rules\\n- ..."
)
\`\`\`

### update_skill
Update an existing skill's content. Provide the path and the fields to change (description and/or body).

Example:
\`\`\`
update_skill(
  path: "programmer/rust/core/ownership-and-borrowing",
  body: "# Ownership and Borrowing\\n\\n## Core Rules\\n- ..."
)
\`\`\`

### delete_skill
Delete a skill.

Example:
\`\`\`
delete_skill(path: "programmer/rust/core/old-skill")
\`\`\`

### search_skills
Search skills by keyword across name, description, path, and body content. Use this when you don't know the exact path. Results are ranked by relevance.

Example:
\`\`\`
search_skills(query: "deadlock")
\`\`\`

## Session Requirement
You must read this guide (via the \`get_usage_guide\` tool) at least once before using other tools. This requirement resets every ${"3 days"}, so re-read periodically to stay current.
`;
