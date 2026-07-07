---
name: typescript_typed_error_handling.md
description: Use when the agent is designing TypeScript error handling, modeling errors in the type system (Result/Option types, discriminated error unions, branded errors), choosing between throws and Result, typing try/catch (unknown vs any), error cause chaining, exhaustive error matching, error boundaries and recovery, typed rejection of promises, or is diagnosing "catch is any so the error type is lost", "every error is unknown so I cast to any", "a thrown error is invisible in the signature", or recovery code that does not match the actual error shape. Covers the tradeoff between exceptions and typed results, narrowing caught values, error taxonomy design, and the boundary where runtime validation must back the type.
---

# Typed Error Handling In TypeScript

TypeScript's error story is split, and the split is the source of most error-handling bugs. The language uses exceptions (`throw`/`try`/`catch`) for failure, but exceptions are not reflected in function signatures: a function declared `function f(): string` may throw anything, and the type system gives the caller no warning. The caught value in `catch (e)` is typed `unknown` (since TS 4.4) precisely because anything can be thrown, which is honest but forces every handler to narrow — and many agents, faced with `unknown`, cast to `any` or `Error` and lose information or lie to the type system. The alternative tradition (the `Result`/`Either` type from fp-ts, neverthrow, true-myth, or a hand-rolled discriminated union) makes failure part of the return type, so the caller cannot ignore it — but it imposes a discipline most TS codebases do not adopt wholesale. The judgment problem is to choose, per layer, between exceptions and typed results; to design an error taxonomy that is narrowable at catch sites; to handle `unknown` honestly rather than casting; and to recognize that a thrown error crossing a trust boundary must be runtime-validated, because the type of a caught value is a hope, not a guarantee.

Agents typically throw a mix of `Error`, plain strings, plain objects, and `Promise` rejections of various shapes, then `catch (e: any)` and read `e.message` or `e.code` without narrowing, producing handlers that crash on the unexpected shape or silently swallow typed information. The remedy is to standardize what is thrown (always `Error` subclasses, with a discriminating `code`/`kind`), to narrow caught values against known shapes, to use the `cause` chain to preserve context across layers, and to reserve `Result` types for the layers where ignoring failure is too costly to permit.

## Core Rules

### Decide Per Layer Between Exceptions And Result Types

Exceptions and `Result` types are both valid; the choice is about who must remember failure exists. Exceptions are appropriate when failure is exceptional and the default (not handling it) is acceptable propagation — I/O at the edges, where a global handler reports it. `Result` (or a discriminated union) is appropriate when failure is an expected outcome the caller must handle — parsing, validation, operations with a meaningful "it didn't work, decide what to do" branch. Mixing both in one layer is fine if the boundary is clear; the trap is using exceptions for expected control flow (a `throw` for "not found" that every caller must catch) or `Result` for truly exceptional conditions (an `Err` for out-of-memory).

- Edge layers (HTTP handlers, CLI entry, top-level): catch exceptions and convert to a response/exit code.
- Domain/core layers: prefer `Result` for expected failures so the type forces handling; throw only for programmer errors and invariant violations.
- Document the choice per layer so callers know whether to `try`/`catch` or to match on a `Result`.

### Always Throw Error Subclasses, Never Primitives

`throw 'not found'` or `throw { code: 42 }` produces a caught value of an unpredictable primitive shape that handlers cannot narrow reliably. Always throw an `Error` (or a subclass). Subclass by category (`ValidationError extends Error`, `NotFoundError extends Error`) so handlers can branch on `instanceof`, and add a discriminating field (`code` or `kind`) for cross-realm and serialization cases where `instanceof` is unreliable.

- A thrown primitive is the single most common reason handlers crash on `e.message` being `undefined`.
- Subclasses let a layer catch a category (`catch (e) { if (e instanceof ValidationError) ... }`).
- A `code`/`kind` string survives serialization (JSON, cross-process, cross-realm) where `instanceof` does not.

### Narrow Caught Values Honestly, Do Not Cast To any

Since TS 4.4, `catch (e)` is `unknown`, which is correct: anything can be thrown. Narrow it before use. Check `e instanceof Error` to access `.message`/`.cause`; for custom fields, narrow further (`e instanceof MyError && typeof e.code === 'string'`); for non-Error throws, guard the shape (`typeof e === 'string'`, `e && typeof e === 'object' && 'status' in e`). Never `catch (e: any)` — it disables narrowing and lets you read fields that may not exist.

- `catch (e) { if (e instanceof Error) console.log(e.message) }` is the safe baseline.
- For a known taxonomy, narrow to each subclass and handle; for unknown shapes, log and rethrow or convert.
- Resist `as any` / `as Error` casts on caught values; they hide the case where the thrown value is not what you assumed.

### Use Error cause To Preserve Context Across Layers

An error caught at a boundary should be re-wrapped with `cause` to add context without discarding the original stack and details: `throw new UpstreamError('failed to load user', { cause: e })`. The `cause` chain preserves the root error for logging and debugging. Avoid rethrowing a new error without `cause` (loses the origin) or rethrowing the original unchanged (loses the layer context of where it was caught).

- `new Error(msg, { cause })` is supported in modern runtimes; for older targets, store `cause` as a field.
- Log the full cause chain at the edge; do not flatten to just the top message.

### Make Error Taxonomy Narrowable And Exhaustive Where It Matters

Design the error set as a discriminated union (a `kind`/`code` field with a finite set of values) so handlers can switch exhaustively. For `Result`-returning APIs, type the error as a union of cases (`type LoadError = NotFound | Unauthorized | NetworkFailed`) and switch on the discriminant; the compiler then flags a missed case. For thrown errors, the equivalent is a set of subclasses with a shared `code` field. Exhaustiveness matters at the edges (an HTTP handler must map every error case to a status), less so in intermediate layers that just propagate.

- Model expected failures as a finite discriminated union; let the type system flag unhandled cases.
- Add a `default`/`never` exhaustiveness check at boundaries so a new error case forces an update.

### Validate Errors Crossing A Trust Boundary

A caught error's type is a static claim, not a runtime guarantee. An error from `JSON.parse`, an API response, a message event, or a third-party library may not match the shape you expect. At trust boundaries, validate the error shape (a guard, a schema) before relying on its fields. This is the runtime analogue of validating input data: the type of a caught value is hopeful, and adversarial or merely buggy upstream code can throw anything.

- `catch (e) { if (isApiError(e)) ... else { throw e } }` with a real guard, not `e as ApiError`.
- Serialize errors to a known shape when crossing processes; deserialize and validate on receipt.

## Common Traps

### Throwing Primitives

`throw 'not found'` makes every handler's `e.message` access a crash. Always throw `Error` subclasses.

### catch (e: any)

Casting caught values to `any` disables narrowing and reads nonexistent fields. Use `unknown` and narrow with guards/`instanceof`.

### Errors Invisible In The Signature

A function `f(): string` that throws gives callers no signal. For expected failures, return `Result` so the type forces handling; reserve `throw` for exceptional conditions.

### instanceof Across Realms/Processes

`e instanceof MyError` is `false` when the error came from another realm (iframe, worker) or was deserialized (JSON has no class). Use a `code`/`kind` discriminant that survives serialization.

### Losing The Original Error On Rewrap

`throw new Error('failed')` at a boundary discards the root cause. Use `{ cause: e }` to preserve the chain.

### Non-Exhaustive Handling At The Edge

An HTTP handler that catches only two of five error cases returns a generic 500 for the rest. Model errors as a discriminated union and switch exhaustively at boundaries.

### Trusting The Caught Type At A Boundary

`catch (e) { return (e as ApiError).statusCode }` assumes the shape; a different throw produces `undefined`. Validate the shape with a guard.

### Swallowed Errors In catch With No Rethrow

`catch (e) { log(e) }` without rethrow converts every failure into a silent `undefined` return. Either rethrow, return a `Result`, or explicitly decide to suppress with a comment.

## Self-Check

- [ ] Each layer has a documented error strategy (exceptions for exceptional propagation, `Result` for expected failures the caller must handle), and the two are not mixed confusingly within a layer.
- [ ] Only `Error` subclasses are thrown (never primitives), and they carry a discriminating `code`/`kind` that survives serialization and cross-realm/cross-process cases where `instanceof` fails.
- [ ] Caught values are typed `unknown` and narrowed honestly with `instanceof` and shape guards; no `catch (e: any)` or `as Error`/`as any` casts on caught values remain.
- [ ] Errors caught at boundaries are rewrapped with `cause` to preserve the original stack and details, and the full cause chain is logged at the edge.
- [ ] Expected failure sets are modeled as finite discriminated unions, and edge handlers switch exhaustively (with a `never`/default check) so a new case forces an update.
- [ ] Errors crossing a trust boundary (`JSON.parse`, API, message, third-party) are runtime-validated with a guard before their fields are trusted.
- [ ] No `catch` block silently swallows errors without an explicit decision to suppress, and every caught error is either handled, rethrown, wrapped with cause, or converted to a `Result`.
- [ ] The error design has been considered under unexpected throw shapes, cross-realm/cross-process errors, and partial failures, and remains correct and informative.
