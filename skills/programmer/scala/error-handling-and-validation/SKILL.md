---
name: scala_error_handling_and_validation.md
description: Use when the agent is designing error handling in Scala (Option/Either/Try/Validated, exceptions vs typed errors, accumulating vs failing-fast validation, raising errors in effect types, error channels in ZIO/cats-effect Resource, custom error ADTs, mapping errors at boundaries), accumulating form/input validation with cats Validated, using Either for fallible computations, choosing between exceptions and typed errors, or is diagnosing "exceptions used for control flow", "validation stops at first error but user needs all errors", "Either chains lose context", "unhandled Future exception", or error-propagation design issues. Covers Option/Either/Try/Validated semantics, fail-fast vs accumulating validation, exceptions vs typed errors, error ADTs, and boundary error mapping.
---

# Error Handling And Validation In Scala

Scala offers several error representations — `Option` (presence), `Either` (a typed failure), `Try` (captured exceptions), `cats.data.Validated` (accumulating failures), and effect types' error channels (`IO`/`ZIO`) — and choosing the wrong one or mixing them inconsistently produces fragile, confusing code. Exceptions used for control flow hide failure paths from the type system and the caller; `Either` chains that short-circuit lose the context of *all* the errors (a form with three invalid fields reports only the first); `Try` captures exceptions but couples you to the exception model; and unmapped errors at boundaries leak internal ADTs to callers. The judgment problem is to choose the right error representation (typed errors over exceptions for expected failures; exceptions for truly exceptional/unrecoverable ones), to use fail-fast (`Either`/`flatMap`) vs accumulating (`Validated`) validation appropriately, to define clear error ADTs, and to map errors deliberately at boundaries.

Agents throw exceptions for expected failures, use `Either` where all errors should accumulate, or leak internal errors across boundaries. The remedy is to type expected errors, accumulate where the caller needs all failures, and map at boundaries.

## Core Rules

### Prefer Typed Errors Over Exceptions For Expected Failures

An expected, recoverable failure (a user not found, a validation error, a parse failure) should be a value in the type system (`Option`/`Either`/`Validated`/effect error channel), not a thrown exception. Exceptions are for truly exceptional, unrecoverable conditions (a corrupt invariant, an out-of-memory); using them for expected control flow hides the failure path from the signature and the caller, and incurs stack-trace cost. `def findUser(id): Option[User]` (might be absent) and `def parse(s): Either[ParseError, A]` (might fail) make failure explicit; `def findUser(id): User` that throws hides it. Reserve exceptions for the genuinely unexpected.

- Expected/recoverable failures → typed values (`Option`/`Either`/`Validated`/effect error channel).
- Exceptions → truly exceptional, unrecoverable conditions (invariant violations, OOM).
- Typed errors make failure visible in the signature; exceptions hide it.

### Choose Fail-Fast (Either/flatMap) vs Accumulating (Validated) By Caller Need

When combining fallible computations, the choice depends on whether the caller needs *all* the errors or just the first:

- Fail-fast (`Either`/`Option` with `flatMap`/for-comprehension): stops at the first failure. Use when later steps depend on earlier results (a user must exist before you check their permissions) or when only the first error matters.
- Accumulating (`cats.data.Validated` with `ValidatedNel`/`mapN`/`product`): collects *all* failures. Use for form/input validation where the user should see every error at once (name, email, and password all invalid).
- Convert between them deliberately (`Validated.fromEither`, `.toEither`, `.toValidated`); do not silently switch semantics.

A form that reports one error at a time (fail-fast) frustrates users; use `Validated` to accumulate. A pipeline that continues past a missing dependency (accumulating) is wrong; use `Either` to fail fast.

- `Either`/`flatMap` for fail-fast (dependent steps, first-error-matters).
- `Validated`/`mapN` for accumulating (form validation, all-errors-at-once).
- Convert deliberately; do not switch semantics silently.

### Define Clear Error ADTs And Map Them At Boundaries

Define error types as sealed ADTs (`sealed trait AppError; case class NotFound(id: String) extends AppError; case class InvalidField(field: String, msg: String) extends AppError`) so the compiler checks exhaustivity in `match`. At boundaries (an HTTP handler, an API edge), map the internal error to the appropriate external representation (a 404 for `NotFound`, a 400 with field errors for `InvalidField`) — do not leak the internal ADT or a raw stack trace to the caller. Centralize this mapping (a single `def toHttpError(e: AppError): HttpResponse`) so error→response logic is consistent and auditable. Within a layer, use a specific error type; at the boundary, fold it to the external contract.

- Errors as sealed ADTs (exhaustive `match`, compiler-checked).
- Map internal errors to external representations at boundaries (HTTP status, API contract); do not leak internals/stack traces.
- Centralize boundary mapping for consistency and auditability.

### Use Try For Exception-Capturing Interop, Not As A Default Model

`Try[A]` captures exceptions as values (`Success`/`Failure`), useful for interop with exception-throwing libraries (JDBC, legacy Java) where you want to treat the exception as a value. But it is coupled to the exception model (the failure is a `Throwable`), so it is less precise than a custom error ADT and loses the typed-error benefits. Prefer `Either[MyError, A]` for your own logic; use `Try` only to wrap exception-throwing interop, then `.toEither` and map the `Throwable` to your error type. Do not use `Try` as the default error model for new code.

- `Try` to wrap exception-throwing interop (JDBC, legacy Java); then `.toEither` and map to your ADT.
- Prefer `Either[MyError, A]` for your own logic; `Try`'s `Throwable` failure is less precise.

### Use Effect-Type Error Channels For Effectful Failures

For `IO`/`ZIO`, the error is in the type's error channel (`IO[Error, A]`, `ZIO[R, E, A]`), so effectful failures are typed and composable (`handleErrorWith`, `mapError`, `catchAll`). Use the error channel for expected failures (typed), and reserve untyped/`Throwable` fatal errors for the unrecoverable. `Resource` (cats-effect) and `ZLayer`/`Scope` handle acquisition/release errors with cleanup guarantees. Do not throw inside an `IO`/`ZIO` (it escapes the error channel); use `.attempt`/`.either`/`catchAll` to surface it. Compose errors across layers with `mapError` at boundaries.

- Effect failures live in the error channel (`IO[E, A]`/`ZIO[R, E, A]`); do not throw inside effects.
- Use `mapError`/`handleErrorWith`/`catchAll` to compose and map errors; `Resource`/`Scope` for cleanup.
- Typed error channel for expected failures; `Throwable` for fatal/unrecoverable.

## Common Traps

### Exceptions For Expected Failures

A thrown exception for "user not found" hides the failure path. Use `Option`/`Either`.

### Either Where All Errors Should Accumulate

A form reporting one error at a time frustrates users. Use `Validated`/`mapN`.

### Swallowing Errors Silently

`.getOrElse(default)` or an empty `catch` hides failures. Handle explicitly or log.

### Leaking Internal Errors Across Boundaries

A raw `Throwable` or internal ADT reaches the API caller. Map at the boundary.

### Try As The Default Error Model

`Try`'s `Throwable` is less precise than a custom ADT. Use `Either`; reserve `Try` for interop.

### Throwing Inside An Effect Type

A `throw` inside `IO`/`ZIO` escapes the error channel. Use `IO.raiseError`/`ZIO.fail` or `catchAll`.

### Non-Sealed Error Types

A non-sealed error trait makes `match` non-exhaustive; new cases silently unhandled. Seal error ADTs.

### Inconsistent Error Models Across Layers

One layer uses `Either`, another exceptions, another `Try`. Pick one model per layer; map at boundaries.

## Self-Check

- [ ] Expected/recoverable failures are typed values (`Option`/`Either`/`Validated`/effect error channel); exceptions are reserved for truly exceptional/unrecoverable conditions.
- [ ] Fail-fast (`Either`/`flatMap`) is used where only the first error matters or steps are dependent; accumulating (`Validated`/`mapN`) where the caller needs all errors (form validation).
- [ ] Errors are sealed ADTs with exhaustive `match`, mapped to external representations at boundaries (HTTP status/API contract) without leaking internals or stack traces.
- [ ] `Try` is used only to wrap exception-throwing interop (JDBC/legacy Java), then `.toEither` and mapped to the domain error type; it is not the default error model.
- [ ] Effect-type failures use the typed error channel (`IO[E,A]`/`ZIO[R,E,A]`) with `mapError`/`handleErrorWith`/`catchAll`; no `throw` inside effects; `Resource`/`Scope` for cleanup.
- [ ] Errors are not swallowed silently (no bare `.getOrElse`/empty `catch`); failures are handled, logged, or propagated explicitly.
- [ ] The error model is consistent within a layer and mapped deliberately at boundaries between layers.
- [ ] The design has been considered under partial failure, multiple simultaneous errors, boundary mapping, and recovery, and remains clear and robust.
