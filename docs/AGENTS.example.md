# Recommended AGENTS.md Pattern For Abalone

Use Abalone as an agent-facing guidance runtime.

## Work Start

Before starting a meaningful task or broad work answer, call:

```text
recommend_skills(intent, context?, domain?, domain_mode?, stack?, changed_files?, limit?)
```

Use the best current description of the work. Broad intents are allowed.

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
intent: "Design API response shape for a user profile endpoint"
domain: "programmer/api/response"
stack: "rust axum"
```

If recommendations are too broad or too narrow:

1. Inspect scopes with `list_skill_tree`.
2. Retry with a clearer intent, more context, or a narrower domain.
3. Use `domain_mode: "filter"` only when the work should stay inside that scope.

## Skill Reading

`recommend_skills` returns a ranked list only. It does not include skill bodies.

Recommendations are ranked with local semantic embeddings plus lexical/path evidence. Use the returned reason and evidence to decide which skills to open.

Open the skills you decide are relevant:

```text
get_skill(path)
```

Abalone records opened skills against the active recommendation session for the current MCP session.

## Work Completion

Before claiming the work is complete, call:

```text
get_completion_checklist()
```

Abalone returns the exact `## Self-Check` sections from the skills actually opened during the recommendation session, ordered by open time.

Verify those checks against the work before finalizing.

## Skill Authoring

Treat each `SKILL.md` as a substantial guidance document.

When creating or updating a skill, validate the result before relying on it:

```text
validate_skill(path?, body?)
```

`create_skill` and `update_skill` also run the same validation before writing. If validation fails, follow the returned `rule` and `fix` fields, rewrite the skill, and validate again.

Target size:

- Preferred: A4 2-5 pages.
- Minimum: roughly A4 2 pages of substantive guidance.
- If much larger than this and no longer coherent as one document, split it.

Do not create tiny standalone skills. If a proposed skill is only a few bullets, it probably belongs in an existing skill as:

- a `Core Rules` item;
- a `Common Traps` item;
- a `Self-Check` item;
- or a subsection of a broader guidance document.

When splitting a large skill:

- Split by distinct work intent, not arbitrary length.
- Each split skill must still be useful as a standalone guidance document.
- Each split skill should have its own trigger-oriented `description`.
- Each split skill should still satisfy the minimum size guideline.

Required sections:

```markdown
---
name: example_skill
description: Use when the agent ...
---

# Example Skill

## Core Rules

## Common Traps

## Self-Check
```

Required section headings must be spelled exactly as `## Core Rules`, `## Common Traps`, and `## Self-Check`. Do not substitute `Self Check`, `Checklist`, or translated headings.

`description` should be trigger text for recommendation, not a short label.
