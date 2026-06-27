# Recommendation Runtime Implementation Plan

## Status

Implemented for the clean Rust stdio MCP server.

This document tracks the implementation corresponding to `2026-06-26-final-product-design-1.0.md`.

## Implemented Runtime Contract

The Rust server implements the product tool contract:

```text
get_usage_guide()
list_skill_tree(path?, depth?)
list_skills(path?)
recommend_skills(intent, context?, domain?, domain_mode?, stack?, changed_files?, limit?)
get_skill(path)
get_completion_checklist(recommendation_id?)
search_skills(query, domain?, domain_mode?, limit?)
validate_skill(path?, body?)
create_skill(path, name, description, body)
update_skill(path, description?, body?)
delete_skill(path)
```

Public tool schemas do not expose manual `session_id`, `sessionId`, or `__sessionId` arguments.

## Implementation Summary

### Clean Rust Baseline

Completed:

- Previous TypeScript server implementation is no longer used.
- New Rust crate lives under `mcp-server/`.
- Modules are split by config, skill parsing, validation, storage, embedding, recommendation, session, tools, and main server boot.
- `main.rs` starts an rmcp stdio server.

### Skill Parser And Validator

Completed:

- YAML frontmatter parsing.
- `disable-model-invocation` and `disable_model_invocation` alias support.
- Exact Markdown H2 section extraction.
- Exact `## Self-Check` extraction.
- Required heading enforcement:
  - `## Core Rules`
  - `## Common Traps`
  - `## Self-Check`
- Skill path validation for `skills/<role>/<area>/<subject>/[optional-context...]/<skill-name>/SKILL.md`.
- Scope path normalization for list/search/domain prefixes such as `investor` or `programmer/api`.
- Trigger-oriented description validation.
- Minimum substantive size validation.
- Tiny checklist fragment rejection.
- Structured validation reports with `code`, `message`, `rule`, `fix`, `warnings`, and full authoring `rules`.

### Storage And Sync

Completed:

- Filesystem remains the source of truth.
- SQLite migrations are idempotent.
- Storage tables cover:
  - skills;
  - FTS5 index;
  - embedding cache;
  - recommendation sessions;
  - active MCP session state;
  - recommendation-session skill rows with `opened_at`.
- Skill upserts use `INSERT ... ON CONFLICT DO UPDATE`.
- Invalid skills are reported and excluded from indexes.
- Missing filesystem skills are removed from the index during sync.
- Embeddings are recomputed only when model identity or embedded trigger text changes.

### Embedding Runtime

Completed:

- `EmbeddingProvider` trait.
- Deterministic test provider.
- BGE-M3 ONNX provider using:
  - `models/bge-m3` by default;
  - `ABALONE_MODEL_DIR` override;
  - `tokenizers` for tokenization;
  - `ort` for ONNX inference;
  - attention-mask mean pooling;
  - L2 normalization.
- Embedded text is `path + name + description`.
- Missing model assets produce explicit startup errors.
- Default runtime is BGE-M3; deterministic provider is available only by setting `ABALONE_EMBEDDING_PROVIDER=deterministic`.

Implementation note:

- BGE-M3 fp16 model loading uses ORT graph optimization level 1. Higher default optimization levels triggered an ONNX Runtime fp16 graph optimization error on the local model asset.

### Recommendation Engine

Completed:

- `recommend_skills` returns up to 8 ranked metadata-only results.
- Skill bodies are never returned by recommendation.
- Ranking combines:
  - semantic similarity over `path + name + description`;
  - SQLite FTS5 lexical evidence;
  - path/name/description lexical matches;
  - domain and stack scope signals.
- Each recommendation includes rank, path, name, description, score, reason, and evidence.
- Recommendation results are stored under a recommendation id.
- The active recommendation id is stored for the current MCP session.

### MCP Session Behavior

Completed for stdio:

- Public tool schemas contain no manual session argument.
- stdio transport uses a stable server-side session key.
- One stdio MCP session can recommend, open skills, and request completion checks.
- Opened skills are tracked against the active recommendation session.

Not enabled:

- Streamable HTTP transport is not part of the current server binary. If added later, the transport adapter must provide a stable per-client session key before completion tracking is considered complete for that transport.

### MCP Tool Behavior

Completed:

- `get_usage_guide()` returns concise agent workflow guidance.
- `list_skill_tree(path?, depth?)` returns scope entries and skill counts, not bodies.
- `list_skills(path?)` returns metadata summaries, not bodies.
- `recommend_skills(...)` creates the active recommendation session.
- `get_skill(path)` returns full `SKILL.md` source and records the opened skill.
- `get_completion_checklist(recommendation_id?)` returns exact `## Self-Check` sections from opened skills only.
- `search_skills(query, domain?, domain_mode?, limit?)` performs explicit lookup without returning bodies.
- `validate_skill(path?, body?)` uses the shared validator.
- `create_skill(...)` and `update_skill(...)` validate before writing.
- Failed create/update validation does not write files or update indexes.
- `delete_skill(path)` removes the file and index row.

## Verification

Current verification commands:

```text
cargo fmt
cargo test
RUST_LOG=info cargo run --quiet
```

Verified behavior:

- `cargo test` passes with parser, validator, storage, embedding, recommendation, tool, and MCP duplex transport tests.
- BGE-M3 model assets load from `models/bge-m3`.
- Startup sync indexes valid skills and reports invalid skills.
- Recommendation responses contain metadata only.
- `get_skill` returns full `SKILL.md` source.
- Completion checks use opened skills only.
- Completion checks preserve opened order.
- Multiple opened skills return multiple self-check sections.
- Public tool schemas do not expose manual session arguments.
- Failed create/update validation is atomic with respect to files and indexes.

Observed startup note:

- Running the binary directly without an MCP client causes stdio to close before an `initialize` request. That produces a connection-closed error after successful model load and skill sync. This is expected for a raw `cargo run` without an MCP client.

## Remaining Product Work Outside Runtime Core

The runtime is implemented. Remaining work is repository/product packaging rather than core behavior:

- Decide the final repository name.
- Reconnect git remote after the repository name is chosen.
- Add more production-quality skill documents beyond the current `market-analysis` seed skill.
- Rewrite or remove old short Rust example skills that are currently invalid under the new A4 2-5 page rule.
- Add streamable HTTP transport only if the product needs a non-stdio deployment mode.
