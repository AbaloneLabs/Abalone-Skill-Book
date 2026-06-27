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

Before finishing, the agent calls `get_completion_checklist()`. Abalone Skill Book returns exact `## Self-Check` sections from the skills that were actually opened, ordered by open order.

## Recommended AGENTS.md Pattern

Use Abalone as an agent-facing guidance runtime.

### Work Start

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

### Skill Reading

`recommend_skills` returns a ranked list only. It does not include skill bodies.

Recommendations are ranked with local semantic embeddings plus lexical/path evidence. Use the returned reason and evidence to decide which skills to open.

Open the skills you decide are relevant:

```text
get_skill(path)
```

Abalone records opened skills against the active recommendation session for the current MCP session.

### Work Completion

Before claiming the work is complete, call:

```text
get_completion_checklist()
```

Abalone returns the exact `## Self-Check` sections from the skills actually opened during the recommendation session, ordered by open time.

Verify those checks against the work before finalizing.

### Skill Authoring

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

- split by distinct work intent, not arbitrary length;
- each split skill must still be useful as a standalone guidance document;
- each split skill should have its own trigger-oriented `description`;
- each split skill should still satisfy the minimum size guideline.

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

The primary runtime is a central MCP Streamable HTTP server. Multiple programs and agents can connect to the same endpoint while sharing the same skill library and search index. Per-agent workflow state is separated by the MCP HTTP session id.

Run checks:

```bash
cd mcp-server
cargo test
```

Run the central HTTP MCP server:

```bash
cd mcp-server
cargo run --quiet -- serve
```

Default endpoint:

```text
http://127.0.0.1:8732/mcp
```

Run the stdio compatibility mode for local subprocess clients:

```bash
cd mcp-server
cargo run --quiet -- stdio
```

By default the server loads local BGE-M3 assets from `models/bge-m3`. Override paths and HTTP settings with:

```bash
ABALONE_SKILLS_ROOT=/path/to/skills
ABALONE_DATABASE_PATH=/path/to/abalone.sqlite3
ABALONE_MODEL_DIR=/path/to/bge-m3
ABALONE_HTTP_HOST=127.0.0.1
ABALONE_HTTP_PORT=8732
ABALONE_MCP_PATH=/mcp
ABALONE_HTTP_ALLOWED_HOSTS=localhost,127.0.0.1
ABALONE_HTTP_ALLOWED_ORIGINS=http://localhost:3000
```

For local use, keep `ABALONE_HTTP_HOST=127.0.0.1`. If binding to `0.0.0.0`, set explicit allowed hosts and put the server behind normal network controls.

## Concurrency

Abalone Skill Book is designed for several agents using one central server.

Shared state:

- `skills/`;
- SQLite skill index;
- embedding cache;
- FTS search index.

Session-separated state:

- active recommendation;
- opened skills;
- completion checklist source.

The HTTP transport issues an MCP session id and clients send it back as `Mcp-Session-Id`. Abalone uses that id internally, so two agents connected to the same `/mcp` endpoint do not share opened-skill or completion-check state.

SQLite concurrency safeguards:

- one in-process mutex serializes access to the shared `rusqlite` connection;
- file-backed databases use WAL mode;
- SQLite busy timeout is set to 5 seconds;
- recommendation creation and skill-open tracking use immediate write transactions;
- opened skill order is stored as an integer sequence, not inferred from timestamp precision.

If one logical agent sends overlapping `recommend_skills` calls in the same MCP session, the most recent committed recommendation becomes that session's active recommendation. Use the returned `recommendation_id` with `get_completion_checklist(recommendation_id)` when an agent intentionally keeps multiple recommendation flows alive.

## Docker Compose

Download the embedding model first, then start the central server:

```bash
docker compose up --build
```

The compose service exposes:

```text
http://127.0.0.1:8732/mcp
```

Mounted paths:

```text
./skills  -> /app/skills
./models  -> /models
volume    -> /data
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
