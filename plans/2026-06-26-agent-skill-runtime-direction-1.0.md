# Agent Guidance Runtime Direction

This is the detailed runtime direction. The consolidated final design is `2026-06-26-final-product-design-1.0.md`.

## Core Definition

Abalone is an MCP server for agents.

An agent calls Abalone when it is about to perform work, answer a broad work question, or make a domain-specific judgment. Abalone recommends existing skill guidance documents that may help the agent avoid missing important considerations.

Abalone is not a user-facing Q&A product. It does not answer the user's question directly. It supports the agent's workflow.

## Why This Exists

Agents often know the mechanics of a task but miss important constraints while doing the work.

Examples:

- When designing an API response, an agent may return the whole persistence model because it is easy, leaking private or internal fields.
- When analyzing a market, an agent may produce a confident narrative without separating observation, pricing, scenarios, and confidence.
- When designing a button, an agent may focus on appearance while missing affordance, accessibility, states, and touch target constraints.

The purpose of a skill is to capture these easy-to-miss considerations in a reusable guidance document.

## What A Skill Is

A skill is a guidance document, not a short reminder and not a generic tutorial.

`SKILL.md` may be substantial. A4 2-5 pages is acceptable when the domain requires it. The point is to include the important rules, traps, edge cases, tradeoffs, and checks an agent is likely to miss.

When an agent opens a skill, the server returns the full `SKILL.md`. The server must not summarize, slice, or rewrite the skill body.

## Skill Size Policy

Skills should be large enough to be useful as standalone guidance documents.

Target size:

- Preferred: A4 2-5 pages.
- Minimum: roughly A4 2 pages of substantive guidance.
- Maximum: not a hard limit, but if a skill becomes hard to read as one coherent guidance document, split it.

The purpose of the minimum is to prevent tiny checklist fragments from becoming separate skills. A tiny skill usually means one of these is true:

- It belongs inside an existing broader skill.
- It is a `Common Traps` subsection, not a standalone skill.
- It is a `Self-Check` item, not a standalone skill.
- The author has not yet captured enough real failure modes, edge cases, and checks.

The purpose of the maximum is to prevent a skill from becoming a book. When a skill grows too large, split it into coherent standalone skills. Each split skill must still satisfy the minimum size policy.

Examples:

- Good standalone skill: `investor/analyze/market/market-analysis`
- Too small as standalone: `check-credit-spreads`
- Better placement: a section inside `market-analysis` or `credit-risk`

Splitting rule:

- Split by distinct work intent, not by arbitrary section length.
- Each resulting skill should have its own trigger description, core rules, traps, and self-check.
- Do not split so aggressively that the agent must open many tiny skills to understand one normal work situation.

Required structure:

```markdown
---
name: market_analysis.md
description: Use when the agent needs to analyze a market, market regime, asset-class backdrop, macro environment, sector cycle, or broad investment context before forming a view on risk, opportunity, timing, valuation pressure, liquidity, or portfolio exposure.
disable-model-invocation: false
---

# Market Analysis

## Core Rules
...

## Common Traps
...

## Analysis Structure
...

## Special Cases
...

## Self-Check
- [ ] ...
```

Required section headings:

- `## Core Rules`
- `## Common Traps`
- `## Self-Check`

The required headings must be spelled exactly as shown. `Self Check`, `Checklist`, `Completion Check`, or translated variants are invalid for runtime extraction. Additional sections such as `## Analysis Structure`, `## Special Cases`, `## Examples`, or `## References` are allowed, but they do not replace the required sections.

Required frontmatter:

- `name`: stable skill identifier.
- `description`: trigger text for recommendation, not a short human summary.

Optional frontmatter:

- `disable-model-invocation`: true for user-invoked workflow skills that should not be surfaced automatically.

## Skill Validation Contract

Skill creation and update must be validated before the filesystem is changed.

`create_skill`, `update_skill`, and `validate_skill` must use the same validator. If validation fails, `create_skill` and `update_skill` must not write partial files, update indexes, or change the active skill state.

Hard validation rules:

- Path must follow `skills/<role>/<area>/<subject>/[optional-context...]/<skill-name>/SKILL.md`.
- Path must have at least 4 logical segments below `skills/`.
- Path segments must use only ASCII letters, digits, hyphen, and underscore.
- Frontmatter must be valid YAML bounded by `---`.
- Frontmatter must include `name` and `description`.
- `description` must be trigger-oriented enough for recommendation; a short label is invalid.
- Body must include exact headings `## Core Rules`, `## Common Traps`, and `## Self-Check`.
- The exact required headings must appear once each.
- A normal skill must satisfy the minimum substantive size policy.
- A tiny checklist fragment must be merged into a broader skill instead of created as a standalone skill.

Validation response:

```json
{
  "ok": false,
  "errors": [
    {
      "code": "missing_required_section",
      "path": "programmer/api/response/data-minimization",
      "message": "Missing required section: ## Self-Check",
      "rule": "Every SKILL.md must include exact headings: ## Core Rules, ## Common Traps, ## Self-Check.",
      "fix": "Add a ## Self-Check section with concrete checks the agent should verify before completion."
    }
  ],
  "rules": [
    "Use YAML frontmatter with name and trigger-oriented description.",
    "Use exact required headings: ## Core Rules, ## Common Traps, ## Self-Check.",
    "Write a substantive guidance document, normally A4 2-5 pages.",
    "Merge tiny fragments into an existing broader skill."
  ]
}
```

The error message is part of the product behavior. It should teach the agent how to rewrite the skill, not merely say that validation failed.

## Description Style

`description` is recommendation trigger text.

Weak:

```yaml
description: Things to consider before buying a stock.
```

Better:

```yaml
description: Use when the agent is considering a stock purchase, entering a position, adding to a public-equity holding, or evaluating whether a stock investment has a clear thesis, valuation discipline, downside scenario, and position sizing rationale.
```

Weak:

```yaml
description: Button design checklist.
```

Better:

```yaml
description: Use when the agent is designing or implementing a button, CTA, icon button, toolbar control, or clickable UI element where affordance, accessibility, label clarity, visual priority, loading/disabled state, or touch target size affects usability.
```

If a short human-facing label becomes necessary later, add a separate `summary` field. Do not weaken `description` for readability.

## Taxonomy

One leaf folder is one skill package.

Path rule:

```text
skills/<role>/<area>/<subject>/[optional-context...]/<skill-name>/SKILL.md
```

Minimum path depth:

```text
<role>/<area>/<subject>/<skill-name>
```

Rules:

- First segment is the agent role.
- Last segment is the skill package.
- Middle segments are routing context.
- Middle depth is flexible.
- Broad reusable skills should live higher in the tree.
- Narrow sector/framework/context skills can live deeper.

Examples:

```text
skills/investor/analyze/market/market-analysis/SKILL.md
skills/investor/trade/stock/position-sizing/SKILL.md
skills/investor/trade/stock/tech-company/multiple-compression-risk/SKILL.md
skills/investor/portfolio/rebalancing/concentration-drift/SKILL.md

skills/designer/frontend/button/accessibility/SKILL.md
skills/designer/frontend/header/visual-hierarchy/SKILL.md
skills/designer/frontend/layout/image-button-composition/SKILL.md

skills/programmer/api/response/data-minimization/SKILL.md
skills/programmer/api/auth/authorization-boundary/SKILL.md
skills/programmer/web/rust/axum/error-responses/SKILL.md
```

Avoid burying a generic skill under a narrow context. For example, if `position-sizing` applies broadly to stock trades, do not put it under `tech-company` just because the first example was a technology stock.

## Session Identity

Abalone should use the MCP call/session context as the work-session identity.

The agent should not have to pass a manual `session_id` argument through every tool call. `recommend_skills` starts or updates the active recommendation session for the current MCP session. `get_skill` records opened skills against that active recommendation session. `get_completion_checklist` reads from that same active recommendation session unless a `recommendation_id` is explicitly supplied to disambiguate.

Transport notes:

- For stdio, use the stable connection/request context exposed by the MCP server runtime.
- For streamable HTTP, use the transport/session identifier exposed by the runtime.
- If a transport cannot expose a stable session identity, completion tracking for that transport is not complete until an adapter provides one.

## Core Workflow

### 1. Scope Discovery

The agent may inspect the skill tree before choosing a recommendation scope.

Tools:

```text
list_skill_tree(path?, depth?)
list_skills(path?)
```

`list_skill_tree` answers: "what scopes exist?"

`list_skills` answers: "what skill packages are under this path?"

### 2. Recommendation

The agent calls:

```text
recommend_skills(
  intent,
  context?,
  domain?,
  domain_mode = "boost",
  stack?,
  changed_files?,
  limit = 8
)
```

`intent` is the agent's current work intent, not a user-facing API contract. It may describe a broad work question or a concrete task.

Examples:

```text
intent: "Analyze what to consider when investing in stocks"
domain: "investor/trade/stock"
domain_mode: "boost"
```

```text
intent: "Design a header component"
domain: "designer/frontend/header"
```

```text
intent: "Design the API response shape for a user profile endpoint"
domain: "programmer/api/response"
stack: "rust axum"
```

Recommendation returns a ranked list of up to 8 skills.

It does not return skill bodies.

Each result should include:

- rank
- path
- name
- description
- score
- reason
- evidence

Example:

```json
{
  "rank": 1,
  "path": "investor/analyze/market/market-analysis",
  "name": "market_analysis.md",
  "score": 91.2,
  "reason": "Intent asks for market-level investment considerations; this skill triggers on market regime, asset-class backdrop, macro environment, risk, valuation pressure, liquidity, and portfolio exposure.",
  "evidence": {
    "matched_fields": ["description", "path"],
    "matched_terms": ["market", "investment", "risk", "valuation"],
    "domain_mode": "boost"
  }
}
```

The ranking itself is the guidance. The agent decides which skills to open based on rank, reason, evidence, and available context budget.

### 3. Skill Reading

The agent explicitly opens the skills it chooses to read:

```text
get_skill(path)
```

`get_skill` returns the full `SKILL.md`.

The server records that the skill was opened under the active recommendation session for the current MCP session. If no active recommendation session exists, `get_skill` still returns the full skill body, but completion checks have no work session to attach it to.

### 4. Completion Check

Before claiming the work is complete, the agent calls:

```text
get_completion_checklist(recommendation_id?)
```

The server returns exact `## Self-Check` sections from the skills opened during that recommendation session.

Ordering:

- Use `opened_at` order.
- This respects the agent's own reading path.
- If no opened skills are tracked, return an explicit empty result. Do not fall back to top-ranked recommendation results.

If an opened skill is missing `## Self-Check`, that is a skill validation failure. The completion output should report the offending skill path and missing required section; it must not invent replacement checklist items.

The server does not judge whether the work passes. It only brings the relevant self-checks back into attention.

## Recommendation Scope

The agent chooses scope.

`domain_mode`:

- `boost`: search all skills, but boost skills under `domain`.
- `filter`: only search under `domain`.
- `none`: ignore `domain` and search globally.

Recommended behavior:

- If unsure, use global search or `boost`.
- If the agent wants a strict scope, use `filter`.
- If recommendations are too broad, inspect `list_skill_tree` and retry with a narrower domain.
- If recommendations miss obvious adjacent concerns, retry with `boost` or global search.

## Search And Ranking

Recommendation should optimize invocation accuracy.

Primary searchable text:

- path
- `name`
- `description`

Secondary searchable text:

- `SKILL.md` body

Body matches are useful but weaker. A body-only match should not dominate a strong description/path match.

Semantic ranking is part of the core product.

Default embedding behavior:

- Use a local embedding model for recommendation ranking.
- Embed `path + name + description` as the primary semantic representation.
- Do not embed full `SKILL.md` bodies by default; body text is too broad and can dilute trigger accuracy.
- Store model name, vector dimension, embedded text hash, and vector in SQLite.
- Recompute embeddings only when the embedded text or model identity changes.
- Combine semantic score with deterministic lexical/domain scoring so recommendations remain explainable.

FTS5 remains required. It provides exact lexical evidence, deterministic debugging, and fallback diagnostics when semantic results look wrong.

If the embedding model is missing or cannot load, the server should report degraded indexing/recommendation state clearly. It should not silently pretend the semantic ranker is active.

## Recommendation Sessions

Recommended tables:

```sql
CREATE TABLE recommendation_sessions (
    id              TEXT PRIMARY KEY,
    mcp_session_key TEXT NOT NULL,
    intent          TEXT NOT NULL,
    context         TEXT,
    domain          TEXT,
    stack           TEXT,
    created_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE mcp_session_state (
    mcp_session_key          TEXT PRIMARY KEY,
    active_recommendation_id TEXT,
    updated_at              TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE recommendation_session_skills (
    recommendation_id TEXT NOT NULL,
    skill_path        TEXT NOT NULL,
    rank              INTEGER,
    score             REAL,
    reason            TEXT,
    opened_at         TEXT,
    PRIMARY KEY (recommendation_id, skill_path)
);
```

The session records:

- what was recommended
- in what rank
- which skills were actually opened
- when they were opened

Completion check uses opened skills, not all recommended skills.

## Tool Surface

Core tools:

```text
get_usage_guide()
list_skill_tree(path?, depth?)
list_skills(path?)
recommend_skills(intent, context?, domain?, domain_mode?, stack?, changed_files?, limit?)
get_skill(path)
get_completion_checklist(recommendation_id?)
search_skills(query, domain?, domain_mode?)
validate_skill(path?, body?)
create_skill(path, name, description, body)
update_skill(path, description?, body?)
delete_skill(path)
```

`search_skills` is for explicit lookup when the agent already knows what it wants to find.

`recommend_skills` is the main workflow.

## Non-Goals

- Do not generate new advice in the server.
- Do not summarize skill bodies.
- Do not slice skill bodies for recommendation responses.
- Do not make repository rename or remotes part of the current design decision.

## Current Direction

Rebuild the MCP server cleanly in Rust.

Previous TypeScript and experimental Rust server code should not be carried forward. Preserve product docs, skill documents, and model assets, then rebuild the server around the final runtime contract:

- Filesystem is the source of truth.
- SQLite is an index/cache/session store.
- FTS5 provides deterministic lexical retrieval and evidence.
- Local embeddings are a required semantic ranking signal.
- Recommendation, skill opening, and completion checks are the core workflow.
