---
name: error_handling_and_errors_is_values.md
description: Use when the agent is writing or reviewing Go error handling — returning errors, wrapping with fmt.Errorf, designing sentinel errors and errors.Is/errors.As, custom error types, panic/recover boundaries, defer in cleanup, error inspection at package boundaries, mapping errors to HTTP/gRPC/CLI status, or diagnosing swallowed errors, lost context, or panics in goroutines. Covers the errors-are-values model, wrapping conventions, sentinel vs typed errors, panic discipline, and the discipline of never ignoring a returned error.
---

# Error Handling And Errors Are Values

Go treats errors as ordinary values. A function that can fail returns its result alongside an `error`, the caller checks it, and the program continues. There is no exception unwinding, no automatic propagation, and no compiler enforcement that the caller actually look at the error. This simplicity is the model's strength — control flow is explicit and readable — and also its central hazard: because ignoring an error is a one-character operation (`result, _ := f()`), the difference between robust and fragile Go code is almost entirely a matter of error-handling discipline. A swallowed error hides a failure that surfaces later as corrupted state, a wrong answer, or a crash in a completely unrelated function.

The judgment problem is not "how do I return an error" but three questions at every boundary: does this error preserve enough context to be diagnosed (what operation, what inputs, what underlying cause), can the caller distinguish the kinds of failure it needs to act on (retry vs report vs give up), and is the error actually handled rather than discarded. Agents coming from exception-based languages tend to either over-use `panic` (treating it like `throw`) or under-handle errors (treating a returned error like a checked exception to be logged and forgotten). Both produce Go code that looks idiomatic and fails opaquely.

## Core Rules

### Never Ignore A Returned Error

The single most important rule. If a function returns an error, the caller must do something with it: handle it, wrap and propagate it, or — only in rare, documented cases — explicitly discard it with a comment explaining why the error is irrelevant. `result, _ := f()` is a bug unless you can articulate why `f`'s error is genuinely uninteresting in this exact context. `defer file.Close()` ignoring its error is a bug when the file was opened for writing (a close error can mean buffered writes were lost). Linters (`errcheck`, `staticcheck`) exist to enforce this; treat their findings as real.

The discipline scales: every error returned by a function you call is a contract you must honor. If you do not know what an error means, find out before discarding it. The cost of handling an error is small; the cost of a silently swallowed error is a debugging session weeks later.

### Add Context With `fmt.Errorf` And `%w`, Not By String Concatenation

When propagating an error, wrap it with context about what you were doing: `fmt.Errorf("reading config %s: %w", path, err)`. The `%w` verb (Go 1.13+) wraps the error so that `errors.Is` and `errors.As` still match against the underlying cause, while the message accumulates a readable chain. String concatenation (`errors.New("reading config: " + err.Error())`) discards the underlying error's identity and breaks inspection — callers can no longer match it.

The context you add should answer "what was the operation and what were the key inputs" — the path, the id, the URL, the offset — not repeat information already in the wrapped error. Avoid wrapping the same error many times with redundant messages ("failed to: error: could not: ..."). A clean chain reads top-down: the outermost message says what the program was trying to do, each inner layer adds the next level of detail, and the innermost is the root cause.

### Choose Between Sentinel Errors, Typed Errors, And Opaque Errors Deliberately

Go offers three styles of error that callers can inspect, and the choice is a design decision:

- **Sentinel errors** (`io.EOF`, `sql.ErrNoRows`) are package-level `var` values compared with `errors.Is`. Best for a small, stable set of specific conditions callers must distinguish. Keep the set small; a package with thirty sentinels is a smell.
- **Typed errors** (`type QueryError struct { Query string; Err error }`) carry structured data callers extract with `errors.As`. Best when callers need details beyond identity (which query failed, what status code, which field). Define a clear type per failure category; do not let one mega-error type carry every field.
- **Opaque errors** are errors the caller cannot inspect — it only knows "something failed." Best for internal logic and for cases where the caller cannot usefully distinguish causes. Many internal errors should stay opaque; not every error needs to be a public contract.

The trap is over-designing: turning every error into a typed error with a sentinel, when most callers only log and propagate. Inspectability is a public API; expose only what callers actually branch on, and keep the rest opaque. `errors.Is` for identity, `errors.As` for structured detail, and `if err != nil` for everything else.

### Use `errors.Is` And `errors.As`, Not Direct Comparison Or Type Assertion

Pre-1.13 code compared errors directly (`if err == io.EOF`) and type-asserted (`if e, ok := err.(*QueryError); ok`). These break when the error is wrapped, because the wrapped error is a different value and a different type. Always use `errors.Is(err, io.EOF)` and `errors.As(err, &target)`, which unwrap the chain. This is a mechanical rule with no downside: the new functions handle both wrapped and unwrapped errors, so they are strictly more correct.

When defining a custom error type that wraps another error, implement `Unwrap() error` so your type participates in the chain. A typed error that holds an inner error but does not implement `Unwrap` silently breaks inspection for every caller.

### Map Errors To Status Codes At Boundaries, And Keep The Mapping In One Place

At a system boundary (HTTP handler, gRPC service, CLI command, job entry), errors must be translated into the boundary's vocabulary: an HTTP status, a gRPC code, an exit code, a log level. This translation belongs in one place — the boundary layer — not scattered through business logic. Business logic returns domain errors; the boundary maps each domain error to the appropriate external representation.

The mapping must be deliberate and consistent. A `NotFound` error is a 404, a validation error is a 400, an auth error is a 401/403, a conflict is a 409, an unavailable dependency is a 503, and an unexpected error is a 500. Do not return 500 for everything (clients cannot retry intelligently), do not return 200 with an error body (breaks HTTP semantics and clients that check status), and do not leak internal error details (stack traces, SQL, internal ids) to external callers. The boundary is also where to distinguish retryable from terminal: a network timeout may be retried, a validation error must not.

### Reserve `panic` For Truly Exceptional Conditions, And `recover` Only At Boundaries

`panic` is not Go's exception mechanism. It unwinds the stack and, if not recovered, crashes the goroutine (and, in `main`, the process). Reserve it for conditions that mean the program is in an impossible, unrecoverable state: invariant violations, impossible type states, initialization failures in `init` or package-level `var` declarations. A function that panics on bad input is a bug — bad input is an expected condition and should return an error.

`recover` belongs only at well-defined boundaries: the top of an HTTP handler (to return 500 instead of crashing the server), the top of a goroutine (to log and continue), the top of a long-running worker. Do not use `recover` to turn panics into normal control flow inside libraries — that hides bugs and makes failures non-local. A recovered panic should be logged with a stack trace, because a panic almost always indicates a bug that needs fixing, not a transient condition to swallow.

### Use `defer` For Cleanup, And Remember It Runs On Return And Panic

`defer` is the cleanup mechanism: closing files, releasing locks, undoing setup. Deferred functions run when the enclosing function returns, including on a panic, which makes `defer` the right tool for ensuring resources are released even on the error path. The cost is that deferred calls run in LIFO order and their arguments are evaluated at defer time, not at run time — `defer f(i)` captures the value of `i` now, not when the defer fires.

Two discipline rules. First, pair every resource acquisition with a defer that releases it, in the same function, immediately — `f, err := os.Open(...); if err != nil { return err }; defer f.Close()`. This guarantees cleanup on every path. Second, check the error from `Close` when the resource was written: a write that buffered may fail on close, and ignoring it loses the failure. The common pattern for write paths is to `Close` explicitly, check the error, and use defer only as a fallback.

### Distinguish Retryable From Terminal, And Make Retries Idempotent

When an operation fails, the caller must decide whether to retry. This requires the error to carry enough information: a network timeout is usually retryable; a validation error never is; a conflict (409) may be retryable after re-reading state. Encode retryability in the error type or a documented contract, and have retry logic check it rather than blindly retrying every error.

When you retry, the operation must be idempotent — retrying a non-idempotent operation (a charge, an insert) can double the effect. If the underlying operation is not idempotent, make it so with an idempotency key, or accept that retries are unsafe and document it. Retries without idempotency are a leading cause of double-charges and duplicate records in distributed systems.

## Common Traps

### Discarding Errors With `_`

`result, _ := f()` swallows the failure. Unless the error is genuinely irrelevant (and documented), handle or propagate it. Lint with `errcheck`.

### Ignoring `defer file.Close()` Errors On Write Paths

For files opened for writing, a close error can mean buffered data was lost. Close explicitly and check the error, using defer only as a fallback.

### Wrapping With String Concatenation Instead Of `%w`

`errors.New("foo: " + err.Error())` breaks `errors.Is`/`errors.As` and loses the cause. Use `fmt.Errorf("foo: %w", err)`.

### Comparing Wrapped Errors Directly

`if err == io.EOF` fails when the error is wrapped. Use `errors.Is(err, io.EOF)`.

### Using `panic` For Expected Failures

Panicking on bad input, missing config, or a network error turns a recoverable condition into a crash. Return an error; reserve panic for impossible states.

### Recovering Panics And Swallowing Them Silently

`recover` without logging hides bugs. A recovered panic should be logged with a stack trace and usually surfaced as an error, not discarded.

### Returning 500 For Every Error

A blanket 500 prevents clients from retrying transient failures or handling validation errors. Map domain errors to specific status codes at the boundary.

### Leaking Internal Details In External Errors

Returning SQL text, stack traces, internal ids, or raw library messages to external callers leaks implementation and can leak secrets. Sanitize at the boundary.

### Retrying Non-Idempotent Operations

Retrying a charge or insert without an idempotency key causes duplicates. Make operations idempotent before retrying, or document that retries are unsafe.

## Self-Check

- [ ] No returned error is silently discarded with `_` unless the discard is documented and the error is genuinely irrelevant; `errcheck`/`staticcheck` findings are resolved.
- [ ] Errors are propagated with `fmt.Errorf("...: %w", err)` adding operation and key-input context, never with string concatenation that loses the underlying cause.
- [ ] The error-inspection style (sentinel `var` + `errors.Is`, typed error + `errors.As`, or opaque) is chosen deliberately per failure category, and only errors callers actually branch on are made inspectable.
- [ ] Custom error types that wrap an inner error implement `Unwrap() error`, and all error comparison uses `errors.Is`/`errors.As` rather than direct `==` or type assertion.
- [ ] At every system boundary (HTTP, gRPC, CLI, job), domain errors are mapped to specific status/exit codes in one place, with retryable vs terminal distinguished, and no internal details leak to external callers.
- [ ] `panic` is reserved for impossible/invariant states and package init failures; expected failures return errors; `recover` is used only at well-defined boundaries and logs the stack trace rather than swallowing silently.
- [ ] Every resource acquisition is paired with a `defer` that releases it in the same function, and write-path `Close` errors are checked rather than ignored.
- [ ] Retried operations are idempotent (or carry an idempotency key), and retry logic checks error retryability rather than retrying every failure blindly.
- [ ] The error chain reads cleanly top-down (outer message says what was attempted, inner layers add detail, innermost is the root cause) without redundant wrapping.
