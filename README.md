# Abalone Skill Book

`abalone-skill-book` is an MCP server for agents, built under the Abalone brand.

It recommends guidance documents before an agent starts meaningful work, records which guidance documents the agent actually opens, and returns completion self-checks from those opened documents before the agent claims the work is done.

## Purpose

Agents often know how to execute a task but miss constraints while working.

Examples:

- API work can accidentally expose private persistence fields.
- Investment analysis can skip downside scenarios or liquidity conditions.
- UI work can miss accessibility, state, or touch target constraints.

Abalone Skill Book stores those easy-to-miss considerations as reusable skills and serves them at the right point in the agent workflow.

## Runtime Flow

```text
recommend_skills(intent, context?, domain?, domain_mode?, stack?, changed_files?, limit?)
get_skill(path)
get_completion_checklist(recommendation_id?)
```

The recommendation call returns a ranked list of up to 8 skill summaries. It does not return skill bodies.

The agent chooses which skills to open. `get_skill(path)` returns the full `SKILL.md` and records that the skill was opened in the active MCP session.

Before finishing, the agent calls `get_completion_checklist()`. Abalone Skill Book returns exact `## Self-Check` sections from the skills that were actually opened, ordered by open time.

## Skill Format

One leaf folder is one skill package:

```text
skills/<role>/<area>/<subject>/[optional-context...]/<skill-name>/SKILL.md
```

Each `SKILL.md` is a substantial guidance document, not a tiny reminder. Preferred size is roughly A4 2-5 pages. Tiny checklist fragments should be merged into broader skills.

Required frontmatter:

```yaml
---
name: market_analysis.md
description: Use when the agent needs to analyze a market, market regime, asset-class backdrop, macro environment, sector cycle, or broad investment context before forming a view on risk, opportunity, timing, valuation pressure, liquidity, or portfolio exposure.
---
```

Required headings must be spelled exactly:

```markdown
## Core Rules

## Common Traps

## Self-Check
```

`Self Check`, `Checklist`, translated headings, or other variants are invalid because runtime extraction depends on exact section names.

## Validation

Skill creation and update must pass validation before the filesystem or index changes.

Core validation rules:

- valid path taxonomy;
- valid YAML frontmatter;
- required `name` and trigger-oriented `description`;
- exact required headings;
- substantive minimum size;
- no tiny standalone checklist fragments.

Validation failures return structured `errors`, `rules`, and `fix` text so an agent can rewrite the skill correctly.

## Search And Ranking

Abalone Skill Book uses both semantic and deterministic signals:

- local embeddings over `path + name + description`;
- SQLite FTS5 over trigger metadata and body text;
- path/domain/stack signals;
- deterministic evidence in recommendation results.

The embedding model is downloaded locally and is not committed to the repository.

## Implementation

The server is a clean Rust rewrite in `mcp-server/`.

Filesystem is the source of truth for skills. SQLite is used for indexing, embedding cache, recommendation sessions, and opened-skill tracking.

Run checks:

```bash
cd mcp-server
cargo test
```

Run as an MCP stdio server:

```bash
cd mcp-server
cargo run --quiet
```

By default the server loads local BGE-M3 assets from `models/bge-m3`. Override paths with:

```bash
ABALONE_SKILLS_ROOT=/path/to/skills
ABALONE_DATABASE_PATH=/path/to/abalone.sqlite3
ABALONE_MODEL_DIR=/path/to/bge-m3
```

## Model Assets

Embedding model files are intentionally excluded from Git. Download them after cloning:

```bash
python -m pip install -U huggingface_hub

hf download BAAI/bge-m3 \
  --local-dir models/bge-m3 \
  --include "config.json" \
  --include "tokenizer.json" \
  --include "tokenizer_config.json" \
  --include "special_tokens_map.json" \
  --include "sentencepiece.bpe.model" \
  --include "onnx/model_fp16.onnx"
```

The default runtime expects this layout:

```text
models/bge-m3/
  config.json
  tokenizer.json
  tokenizer_config.json
  special_tokens_map.json
  sentencepiece.bpe.model
  onnx/model_fp16.onnx
```

For tests or local protocol checks that should not load the large model, set:

```bash
ABALONE_EMBEDDING_PROVIDER=deterministic
```
