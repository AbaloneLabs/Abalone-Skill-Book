---
name: api_and_reference_writing.md
description: Use when the agent is writing API documentation, SDK references, parameter tables, type definitions, endpoint specifications, or any reference material that must be accurate, complete, and usable as an authoritative lookup rather than a tutorial.
---

# API And Reference Writing

Reference writing is the most unforgiving mode of technical documentation. A tutorial can be slightly out of date and still teach the idea; an explanation can be evocative and still be useful. A reference that is wrong, incomplete, or inconsistent with the system becomes an active hazard, because developers build against it, automate against it, and trust it as a contract. The cost of an error in reference material is measured in broken builds, silent bugs, and support load.

Agents tend to miss this because reference writing looks easy: list the endpoints, list the parameters, show a code sample. But the difficulty is not in listing; it is in being complete without being noisy, accurate without being unverifiable, and structured so a reader can find the exact fact they need in seconds. A reference is a tool, not a narrative.

You have latitude in organization and tooling, but the core obligation is fixed: every statement in reference material must match the system it describes, and every gap a reader needs must be filled.

## Core Rules

### Treat The Reference As A Contract

An API reference is not a description of how the system usually behaves. It is a specification the reader will rely on as a contract. If the reference says a field is a string, readers will write code assuming a string. If it says a value is required, readers will treat its absence as an error. Every assertion in reference material should be something you are willing to support and something the system actually guarantees.

Distinguish guarantees from implementation details. A guarantee is something the system commits to across versions. An implementation detail is how it happens to work today and may change. Reference material should document guarantees and avoid encoding incidental behavior as if it were permanent, unless that behavior is load-bearing for users.

### Be Complete And Explicit About Every Field

Reference material fails most often by omission. A field appears in a response but is not documented. A parameter is optional but the default is unstated. An error code is returned but never listed. Each omission becomes a support ticket or a guessed-at integration.

For every parameter and field, document at minimum: the name, the type, whether it is required or optional, the default if optional, the allowed values or format, the meaning, and any constraints. For responses, document every field the reader will encounter, including ones that seem obvious. Do not rely on the reader inferring structure from a single example.

### Make Examples That Match The System

Examples in reference material are read as ground truth. If an example shows a field that does not exist, omits a required field, or uses a value the system would reject, readers will copy it and break. Every example must be runnable against the current system, or it must be clearly marked as illustrative and simplified.

Provide examples at multiple levels: a minimal request that shows the shape, a realistic request that shows typical values, and an edge-case example that shows handling of optional or conditional fields. Show both the request and the full response, not a truncated fragment that hides structure. If you truncate for length, say so.

### Separate Reference From Tutorial And Explanation

A developer reading reference material is in lookup mode, not learning mode. They want the exact signature, the exact type, the exact error. Embedding a paragraph of conceptual explanation inside a parameter table slows them down. Conversely, a tutorial that is nothing but a parameter dump teaches nothing.

Keep reference material authoritative and dense. Link out to tutorials for learning paths and to explanations for the why. If a short note is needed inside the reference, keep it to a sentence or two and clearly set apart. Do not let narrative prose drift into the lookup surface.

### Document Errors, Status Codes, And Failure Modes

Successful responses are half the contract. The other half is what happens when things go wrong. Document every status code the endpoint can return, every error shape, every error code or message, and the conditions that trigger each. A reader building retry logic, validation, or user-facing error messages needs this as much as the success path.

Be explicit about which errors are recoverable, which indicate a client bug, and which indicate a server or system problem. If rate limiting applies, document the limits and the headers that communicate them. If idempotency is supported, document the key and its behavior. Failure documentation is not optional padding; it is core reference content.

### Specify Types, Formats, And Constraints Precisely

Imprecise types create integration bugs. "A string" is rarely enough. Is it a UUID, an email, an ISO 8601 timestamp, a URL, a free-text value with a max length, an enum with fixed values? State the format explicitly. If a numeric field is an integer with a range, state the range. If a field accepts null, say so, and distinguish null from absent.

Document constraints that affect validity: maximum lengths, allowed characters, uniqueness, immutability after creation, dependencies between fields. These constraints are part of the contract. A reader who discovers a length limit only by getting an error has been failed by the reference.

### Keep The Reference In Sync With The Source

Reference material drifts the moment the system changes and the docs do not. Where possible, generate reference content from the source of truth: type definitions, OpenAPI or schema files, code annotations. Generated reference is more likely to be accurate, and it can be regenerated when the system changes. Hand-maintained reference should be reviewed against the source on every release.

When you write reference by hand, name the version of the system it describes and the date it was verified. Treat unverified reference as a known risk. Never present inferred behavior as confirmed fact.

### Structure For Random Access

Reference readers do not read sequentially. They arrive at a specific endpoint, method, or type and want the answer immediately. Structure the material so each item is self-contained and findable: stable, predictable URLs or anchors; consistent heading patterns; a reliable left-nav or search; and cross-links between related items such as an endpoint and the error type it returns.

Within each item, use a consistent order: summary, signature or path, parameters, request body, response, errors, examples. Consistency lets the reader learn your structure once and navigate every page faster thereafter.

## Common Traps

### Documenting The Happy Response Only

Reference that shows only the 200 response leaves readers to discover error shapes in production. Every endpoint has a failure surface, and it belongs in the reference.

### Examples That Cannot Run

A copied example that fails erodes trust instantly. If an example uses placeholder credentials, dummy IDs, or a simplified payload, mark it clearly so the reader knows what to replace.

### Mixing Tutorial Voice Into Reference

A reader in lookup mode resists prose. When reference pages drift into "Let's explore how this works," they stop being usable as a quick lookup and start competing with the tutorial that already exists elsewhere.

### Treating Optional As Self-Evident

Failing to mark a parameter optional, or failing to state its default, forces the reader to experiment. The reference should remove the need to guess.

### Inconsistent Terminology Across Pages

If one page calls it a "workspace" and another calls it a "tenant," readers waste energy reconciling. Maintain a glossary and use terms consistently across the entire reference surface.

### Stale Reference After A Breaking Change

When the system ships a breaking change and the docs are not updated, every reader who arrives is misled. Reference updates are part of the release, not an afterthought.

### Hiding Constraints In Prose

A length limit buried in a sentence is invisible to a reader scanning a table. Put constraints in the structured field documentation, where the eye looks for them.

## Self-Check

Before treating the reference as complete, verify:

- Every parameter and field documents name, type, required status, default, format, meaning, and constraints.
- Every status code and error shape the endpoint can return is documented, not only the success case.
- All examples are runnable against the current system or are clearly marked as simplified.
- Types are precise: formats, enums, ranges, nullability, and max lengths are stated, not implied.
- Reference material is separated from tutorial and explanation, with links between them.
- The material is structured for random access with stable anchors and a consistent per-item layout.
- Terminology is consistent across all pages, with a shared glossary where needed; the reference is generated from or verified against the source of truth, with version and date noted
- Guarantees are distinguished from implementation details that may change; optional fields state their defaults; required fields are clearly marked
- Edge cases such as rate limits, idempotency, pagination, and conditional fields are documented; a developer could integrate against this reference without reading any other source
- Nothing in the reference contradicts the actual behavior of the system; breaking changes are reflected, with migration guidance where readers will be affected
