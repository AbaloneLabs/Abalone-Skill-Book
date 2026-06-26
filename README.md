# Abalone Skills

A library of skills that remind AI agents what to watch for when performing specific tasks.

## Purpose

Agents already know *how* to do things. These skills exist to **remind** them what to be careful about and what to follow - the pitfalls and rules that are easy to lose track of while juggling complex work.

Each skill is a concise checklist of invariants, common traps, and a self-check section. Not a tutorial.

## Structure

```
<role>/<stack>/<domain>/[<framework>/]<skill-name>/SKILL.md
```

- **role** - the agent's responsibility (e.g. `programmer`)
- **stack** - the technology (e.g. `rust`)
- **domain** - the area of work (e.g. `core`, `web`, `gamedev`)
- **framework** *(optional)* - when a domain splits by framework (e.g. `axum`, `bevy`)
- **skill** - the specific concern (e.g. `ownership-and-borrowing`)

## How It Works

When an agent works on a task, it loads the skills relevant to that task. For example, working on a Solana program loads everything under `programmer/rust/blockchain/solana/`. The skills remind the agent of invariants and pitfalls specific to that context.

## Current Coverage

### `programmer/rust/`

```
rust/
├── core/                    # Language fundamentals (all domains share these)
│   ├── ownership-and-borrowing/
│   ├── error-handling/
│   ├── concurrency/
│   ├── unsafe-rust/
│   ├── cryptography/
│   └── input-validation/
│
├── testing/                 # Cross-cutting
├── observability/           # Cross-cutting (tracing, logging, metrics)
├── serialization/           # Cross-cutting (serde, formats)
├── config/                  # Cross-cutting
├── build/                   # Cargo, workspaces, feature flags
├── ffi/                     # Foreign function interface
│   ├── c/
│   ├── python/
│   └── node/
│
├── web/                     # Web backend
│   ├── axum/
│   ├── actix-web/
│   └── rocket/
├── cli/                     # Command-line tools
├── gui/                     # Native desktop GUI
│   ├── egui/
│   ├── iced/
│   ├── slint/
│   └── dioxus/
├── tauri/                   # Web-based desktop apps
│   ├── ipc/
│   └── permissions/
├── mobile/                  # Android / iOS
├── wasm/                    # WebAssembly target
├── embedded/                # no_std, microcontrollers
├── systems/                 # OS / kernel / drivers
│
├── gamedev/                 # Game development
│   ├── bevy/
│   ├── godot/
│   ├── macroquad/
│   └── fyrox/
├── graphics/                # GPU / rendering
│   ├── wgpu/
│   └── vulkano/
├── audio/                   # Audio processing
├── networking/              # Network protocols
│   ├── tokio/
│   ├── hyper/
│   └── tonic/
├── database/                # DB clients (sqlx, diesel)
├── blockchain/              # Smart contracts / chains
│   ├── solana/
│   ├── substrate/
│   └── cosmwasm/
├── ml-ai/                   # Machine learning
│   ├── burn/
│   ├── candle/
│   ├── tch-rs/
│   └── ort/
├── parser/                  # Parsers / compilers
│   ├── nom/
│   ├── pest/
│   ├── lalrpop/
│   └── winnow/
├── scientific/              # Numerical / scientific computing
├── robotics/                # Robotics (ROS2)
└── cloud/                   # Cloud-native (k8s operators)
```

## Skill Format

Every skill is a single `SKILL.md` with YAML frontmatter (`name`, `description`) and a markdown body containing:
- Core rules / invariants to never violate
- Common traps with brief examples
- A self-check checklist

## How to Add a Skill

1. Create `<role>/<stack>/<domain>/[<framework>/]<skill-name>/SKILL.md`
2. Write the frontmatter `name` and `description` (this is how the skill is discovered and triggered)
3. Write the body as a reminder checklist, not a tutorial
