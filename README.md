# Abalone Skills

A library of skills that remind AI agents what to watch for when performing specific tasks.

## Purpose

Agents already know *how* to do things. These skills exist to **remind** them what to be careful about and what to follow - the pitfalls and rules that are easy to lose track of while juggling complex work.

Each skill is a concise checklist of invariants, common traps, and a self-check section. Not a tutorial.

## Structure

```
<role>/<stack>/<domain>/<skill-name>/SKILL.md
```

- **role** - the agent's responsibility (e.g. `programmer`)
- **stack** - the technology (e.g. `rust`)
- **domain** - the area of work (e.g. `programming`, `tauri`, `web`, `security`)
- **skill** - the specific concern (e.g. `ownership-and-borrowing`)

Example:

```
programmer/rust/
├── programming/
│   ├── ownership-and-borrowing/SKILL.md
│   ├── error-handling/SKILL.md
│   ├── concurrency/SKILL.md
│   └── unsafe-rust/SKILL.md
├── tauri/
│   ├── ipc/SKILL.md
│   └── permissions/SKILL.md
├── web/
│   ├── async-handlers/SKILL.md
│   └── error-responses/SKILL.md
└── security/
    ├── input-validation/SKILL.md
    └── cryptography/SKILL.md
```

## Skill Format

Every skill is a single `SKILL.md` with YAML frontmatter (`name`, `description`) and a markdown body containing:
- Core rules / invariants to never violate
- Common traps with brief examples
- A self-check checklist

## How to Add a Skill

1. Create `<role>/<stack>/<domain>/<skill-name>/SKILL.md`
2. Write the frontmatter `name` and `description` (this is how the skill is discovered and triggered)
3. Write the body as a reminder checklist, not a tutorial
