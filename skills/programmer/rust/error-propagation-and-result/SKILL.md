---
name: rust_error_propagation_and_result.md
description: Use when the agent is designing Rust error types, implementing From conversions, choosing between thiserror and anyhow, propagating errors with the ? operator, deciding between Result and Option, handling panics vs errors, adding context to propagated failures, mapping error variants at module boundaries, or reviewing code that swallows errors or converts everything to String. Covers error type design, the ? operator and From, library vs application error strategies, context enrichment, downcasting, and the boundary between recoverable errors and unrecoverable panics.
---

# Error Propagation And Result

Rust separates recoverable errors (`Result`) from unrecoverable bugs (`panic`) at the type level, and forces you to handle `Result` explicitly. This is a strength, but it shifts the design burden to you: you must decide what counts as recoverable, how errors flow across boundaries, how much context survives propagation, and when a panic is the correct contract. The judgment problem is designing error types and propagation paths that preserve meaning, support recovery where it matters, and do not collapse every failure into an opaque string.

Agents tend to reach for `String` or `Box<dyn Error>` because it is quick, convert everything to a single error enum, swallow errors with `unwrap` or empty matches, or add `?` without thinking about what crosses a boundary. The harm appears as retry logic that cannot distinguish transient from permanent failures, logs that say "io error" with no path or operation, HTTP handlers that return 500 for validation bugs, and libraries whose error types leak internal dependencies. The real work is modeling errors by boundary, preserving source context, and choosing panic deliberately rather than by default.

## Core Rules

### Distinguish Recoverable Errors From Unrecoverable Bugs

The first decision for any failure is whether the caller can meaningfully react to it. This determines `Result` vs `panic`.

- **Recoverable (Result)**: the caller may retry, fall back, report to a user, or ignore. File not found, network timeout, parse failure, permission denied, rate limited. These belong in `Result` because the caller decides.
- **Unrecoverable bug (panic)**: a broken invariant the program cannot safely continue past. Index out of bounds when the index was just bounds-checked, a `None` that the logic guarantees cannot happen, a corrupted internal data structure. These are programmer errors, not runtime conditions.

The trap is using `unwrap`/`expect`/panic for conditions that are actually recoverable. If a user's input could trigger it, if a network could cause it, if a file could be missing — it is recoverable, and a panic turns a manageable failure into a crash. Reserve panic for invariants you are asserting, and document the expectation in the message.

### Design Error Types By Boundary, Not By Source

A single error type that enumerates every library's errors (`enum Error { Io(io::Error), Reqwest(reqwest::Error), Serde(serde_json::Error), ... }`) leaks implementation details to callers and couples them to your dependencies. Design error types by the boundary they cross.

- **Internal helpers** can use precise, technical error types tied to the operation.
- **Module/domain errors** describe domain failure in the module's vocabulary, hiding the underlying source. A `UserRepoError::NotFound` is meaningful; `Error::Sqlx(sqlx::Error::RowNotFound)` leaks the database library.
- **API boundary errors** map to client-facing outcomes: a validation error becomes a 400, a not-found becomes a 404, an internal failure becomes a 500 with no detail leaked.

Convert between layers with `From` implementations or explicit mapping. The boundary is where low-level detail is translated into domain meaning. Do not let `sqlx::Error` appear in your HTTP response shape.

### Use `?` With `From` To Propagate, But Map At Boundaries

The `?` operator propagates an error and converts it via `From`. This makes propagation concise, but `From` conversions are automatic and lossy of context. Use `?` for straightforward propagation within a layer, and add explicit context when crossing a boundary.

- Within a layer: `let f = File::open(path)?;` — the io::Error propagates with its source.
- Crossing a boundary: `File::open(path).map_err(|e| RepoError::Read { path: path.into(), source: e })?` — the context (which path, what operation) is added.

Libraries like `anyhow` (`.context()`) and manual `map_err` both serve this. The point is that by the time an error reaches a human or a retry decision, it carries the operation and subject that failed, not just the low-level cause.

### Choose Thiserror For Libraries, Anyhow For Applications

The two dominant error strategies serve different roles; mixing them carelessly causes friction.

- **`thiserror`** (or hand-written error enums with `#[derive(thiserror::Error)]`): for libraries and modules that need a precise, documented error type callers can match on. Callers can `match` on variants to decide retry vs report. The error type is part of the public API and should be stable.
- **`anyhow`** (or `eyre`): for applications, binaries, and the top level where you want to collect errors with context and report them, not match on them. `anyhow::Error` is opaque; callers cannot (easily) match on variants, which is fine at the application top where you just log and exit.

A library that returns `anyhow::Error` forces every caller into opaque handling and prevents retry logic. An application that builds a giant `thiserror` enum for errors it never matches adds ceremony for nothing. Match the tool to the layer.

### Preserve The Error Chain, Do Not Flatten

When you wrap an error, keep the source available via `source()` (the `Error::source` trait method, surfaced by `thiserror`'s `#[from]` and `#[source]`). This lets logging and debugging walk the full chain: `ReadFailed → IoError → OsError`. Flattening (`format!("{}", e)` and storing the string) destroys the chain and the ability to downcast.

- Use `#[from]` on the source field so `From` and `source()` are derived.
- When adding context manually, store the source error, not its string form.
- At the reporting boundary, walk the chain for the full message; at the user boundary, show only the top-level domain message.

### Decide Option vs Result Deliberately

`Option<T>` represents absence; `Result<T, E>` represents failure. They are not interchangeable, and choosing wrong loses information.

- **Option**: the value may legitimately not exist, and there is no error to report. `HashMap::get` returns `Option` because a missing key is not a failure. No context is lost because there is no failure context.
- **Result**: an operation was attempted and failed, and the failure has a reason. `File::open` returns `Result` because the failure carries an io::Error.

If callers need to know *why* something is missing (not found vs permission denied vs timeout), it is a `Result`. If absence is a normal state with no cause, it is an `Option`. Do not return `Option` where the cause matters, and do not return `Result` with a unit error where absence is normal.

### Never Swallow Errors Silently

`let _ = send_result;`, `match result { Ok(_) => {}, Err(_) => {} }`, and `unwrap_or_default()` on an error path all discard failure information. Swallowing an error means a failure happens and nobody knows, which is worse than crashing in many systems.

- If you must ignore an error, log it at the appropriate level with context.
- `_` bindings are acceptable only for values you genuinely discard, not for errors.
- `unwrap_or_default()` is fine for a default-on-failure contract, but document that the failure is intentionally ignored and consider logging it.

## Common Traps

### Converting Every Error To String

`Box<dyn Error>` or `String` errors lose structure: no downcasting, no variant matching, no retry decisions. Preserve typed errors until the boundary where a string is genuinely all that is needed.

### One Giant Error Enum Leaking Dependencies

`enum Error { Sqlx(sqlx::Error), Reqwest(reqwest::Error), ... }` exposes your HTTP and database libraries to every caller. Wrap them in domain variants and hide the source type behind the domain error.

### Using `unwrap` In Library Code

`unwrap` in a library turns a recoverable caller situation into a panic the caller cannot handle. Return a `Result` unless the panic is an invariant assertion documented as part of the contract.

### Swallowing With Empty Match Arms

`Err(_) => {}` discards the error silently. If the error is truly ignorable, log it; otherwise propagate it. Silent swallowing creates debugging black holes in production.

### `From` Conversions That Lose Context

Automatic `From` propagates the error but adds no context about *which* operation or subject failed. Add context with `map_err` or `.context()` when crossing a meaningful boundary, not just at the top.

### Returning `anyhow::Error` From A Library

`anyhow::Error` is opaque; library callers cannot match on variants to decide retry vs report. Use `thiserror` or a documented error enum for library public APIs; reserve `anyhow` for application-internal code.

### Panicking On User Input

A parse failure, missing file, or invalid request is recoverable. Panicking on it crashes the process for a condition the caller could have handled. Return a `Result`.

### Forgetting `#[from]` And Breaking `?`

Without `#[from]` (or a manual `From` impl), `?` cannot convert the source error, forcing verbose `map_err` everywhere. Derive `From` for the common source conversions so propagation stays concise.

## Self-Check

- [ ] Recoverable failures use `Result`; panics are reserved for invariant assertions documented as the contract, not for user input or environmental conditions.
- [ ] Error types are designed by boundary: internal helpers are precise, module errors use domain vocabulary, API errors map to client outcomes, and dependency types do not leak across boundaries.
- [ ] `?` propagation is used within a layer; explicit `map_err`/`.context()` adds operation and subject context when crossing a boundary.
- [ ] Libraries use `thiserror` or documented error enums callers can match on; applications use `anyhow`/`eyre` at the top level where matching is not needed.
- [ ] The error chain is preserved via `source()`/`#[from]`; errors are not flattened to strings before the reporting boundary.
- [ ] `Option` vs `Result` is chosen by whether the caller needs the cause; absence is `Option`, failure-with-reason is `Result`.
- [ ] No error is swallowed silently; ignored errors are logged with context or documented as intentionally discarded.
- [ ] `unwrap`/`expect` are absent from fallible runtime paths in library code unless panic is the documented contract.
- [ ] Downstream retry/report logic can distinguish failure kinds because the error type is precise, not collapsed to a string.
- [ ] The top-level error report walks the full source chain for diagnostics while showing only domain-meaningful messages to end users.
