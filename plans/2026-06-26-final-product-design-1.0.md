# Final Product Design

## Status

This document is the final product direction for Abalone before implementation resumes.

Abalone is an MCP server for agents. It is not a user-facing Q&A service and it does not generate new advice. It recommends existing skill guidance documents, records which documents the agent actually opened, and returns completion checks from those opened documents.

## Product Goal

Agents often know how to perform a task but miss important constraints while working. Abalone exists to put those easy-to-miss considerations back into the agent workflow at two points:

- before meaningful work starts;
- before the agent claims the work is complete.

The system should work for programming, investing, design, operations, writing, and other agent roles. It is not limited to API servers or software development.

## Core Runtime Flow

```text
recommend_skills(intent, context?, domain?, domain_mode?, stack?, changed_files?, limit?)
get_skill(path)
get_completion_checklist(recommendation_id?)
```

1. The agent calls `recommend_skills` with its current work intent.
2. Abalone returns up to 8 ranked skill summaries. It does not return skill bodies.
3. The agent chooses which skills to open.
4. `get_skill(path)` returns the full `SKILL.md` and records that the skill was opened in the active MCP session.
5. Before completion, the agent calls `get_completion_checklist()`.
6. Abalone returns exact `## Self-Check` sections from the skills actually opened in that session, ordered by open time.

Recommendations are guidance for the agent. The server does not decide which skills must be read.

## Session Model

Abalone uses MCP call/session context as the work-session identity.

Public tool schemas must not require manual `session_id`, `sessionId`, or `__sessionId` arguments. `recommend_skills` creates or replaces the active recommendation session for the current MCP session. `get_skill(path)` records opened skills against that active recommendation session. `get_completion_checklist()` reads from that same active session unless a `recommendation_id` is supplied to disambiguate.

If a transport cannot provide a stable session key, that transport is incomplete for completion tracking until an adapter supplies one.

## Skill Document Contract

A skill is a substantial guidance document. It is not a tiny reminder, a one-off checklist, or a generic tutorial.

Target size:

- Preferred: A4 2-5 pages.
- Minimum: roughly A4 2 pages of substantive guidance.
- Oversized skills should be split by distinct work intent.
- Tiny fragments should be merged into broader skills.

Required frontmatter:

```yaml
---
name: market_analysis.md
description: Use when the agent needs to analyze a market, market regime, asset-class backdrop, macro environment, sector cycle, or broad investment context before forming a view on risk, opportunity, timing, valuation pressure, liquidity, or portfolio exposure.
---
```

Required body headings:

```markdown
## Core Rules

## Common Traps

## Self-Check
```

The required headings must be spelled exactly. `Self Check`, `Checklist`, translated headings, and other variants are invalid.

Additional sections such as `## Analysis Structure`, `## Special Cases`, `## Examples`, and `## References` are allowed when useful.

## Taxonomy

One leaf folder is one skill package.

```text
skills/<role>/<area>/<subject>/[optional-context...]/<skill-name>/SKILL.md
```

Rules:

- Minimum logical path depth is `<role>/<area>/<subject>/<skill-name>`.
- First segment is the agent role.
- Last segment is the skill package.
- Middle segments are routing context.
- Broad reusable skills live higher in the tree.
- Narrow sector/framework/context skills can live deeper.

Examples:

```text
skills/investor/analyze/market/market-analysis/SKILL.md
skills/investor/trade/stock/position-sizing/SKILL.md
skills/investor/portfolio/rebalancing/concentration-drift/SKILL.md
skills/designer/frontend/button/accessibility/SKILL.md
skills/programmer/api/response/data-minimization/SKILL.md
```

## Skill Validation

Skill creation and update must be validated before the filesystem or index changes.

`validate_skill`, `create_skill`, and `update_skill` use the same validator. Failed create/update calls must be atomic: no partial file write, no index update, and no session-state mutation.

Hard validation:

- valid path taxonomy;
- valid YAML frontmatter;
- required `name`;
- trigger-oriented `description`;
- exact `## Core Rules`, `## Common Traps`, and `## Self-Check` headings;
- substantive minimum size;
- no tiny standalone checklist fragments.

Validation failures return structured feedback with:

- `code`;
- `message`;
- `rule`;
- `fix`;
- full authoring `rules`.

The validation response should teach the agent how to rewrite the skill.

## Recommendation And Ranking

Recommendation optimizes invocation accuracy.

Primary trigger fields:

- path;
- `name`;
- `description`.

Secondary field:

- `SKILL.md` body.

Embeddings are a core product requirement. The default semantic representation is:

```text
path + name + description
```

The full skill body is not embedded by default because long guidance documents can dilute trigger accuracy.

Ranking combines:

- semantic similarity from local embeddings;
- SQLite FTS5 lexical scores;
- path/domain/stack scope signals;
- weak body keyword matches.

Recommendation evidence must expose enough information for an agent to understand why a skill was recommended.

## Storage

Filesystem is the source of truth for skill documents.

SQLite stores:

- parsed skill metadata;
- FTS5 index;
- embedding vectors and model metadata;
- recommendation sessions;
- active MCP session state;
- opened skill records.

Embeddings store:

- model identity;
- vector dimension;
- embedded text hash;
- vector bytes.

Embeddings are recomputed only when the embedded text or model identity changes.

## Completion Check

Completion checks use opened skills only.

`get_completion_checklist()` returns exact `## Self-Check` sections from skills opened during the recommendation session, ordered by `opened_at`.

If no skill was opened, return an explicit empty result. Do not fall back to top-ranked recommendations.

If an opened skill is missing exact `## Self-Check`, report a validation error for that skill. Do not generate replacement checklist items.

## Technical Decisions

- Language: Rust.
- MCP runtime: `rmcp`.
- Source of truth: filesystem.
- Index/cache/session storage: SQLite.
- Lexical search: SQLite FTS5.
- Embedding runtime: local BGE-M3 model assets under `models/bge-m3` by default.
- ONNX inference: `ort`.
- Tokenization: `tokenizers`.
- Skill frontmatter parsing: YAML parser, not ad hoc line parsing.
- Markdown extraction: heading-aware parser or strict Markdown section scanner.
- Tests: `cargo test`, deterministic mock embedding provider for ranking tests, MCP-compatible transport tests for protocol behavior.

## Non-Goals

- Do not answer user questions directly.
- Do not generate new domain advice inside the server.
- Do not summarize, slice, or rewrite skill bodies.
- Do not include unopened recommended skills in completion checks.
- Do not expose manual session arguments in public tool schemas.
- Do not carry forward the previous TypeScript or experimental Rust implementation.

Repository rename and new git remotes are separate decisions.
