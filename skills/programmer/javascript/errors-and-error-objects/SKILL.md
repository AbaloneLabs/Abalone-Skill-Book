---
name: javascript_errors_and-error-objects.md
description: Use when the agent is designing JavaScript error handling, throwing and catching errors, subclassing Error, choosing error types and messages, using try/catch/finally, handling Promise rejections and async errors, adding structured properties to errors (cause), differentiating expected errors from programmer bugs, logging and rethrowing, or debugging swallowed errors, unhandled rejections, and errors that lose their stack. Covers Error subclass design, the cause chain, sync vs async error semantics, expected-vs-bug separation, and the tradeoff between typed errors and simple messages.
---

# Errors And Error Objects

JavaScript throws and catches values of any type, but the `Error` class and its stack trace are the meaningful currency of diagnostics. Unlike languages with checked exceptions, JavaScript gives no compile-time signal about what a function may throw, so the design burden is entirely on the author: deciding what to throw, how to make it informative, how async rejections differ from sync throws, and how to distinguish expected conditions from programmer bugs. The judgment problem is building an error strategy that preserves cause and stack, separates recoverable conditions from bugs, and does not swallow failures silently.

Agents tend to throw strings instead of `Error` (losing the stack), catch and swallow errors with empty blocks, lose the original cause when wrapping, or treat programmer bugs as catchable expected errors. The harm appears as stackless errors that cannot be traced, rejections that vanish because nobody awaited them, errors wrapped so the original is lost, and broad `catch` blocks that mask logic bugs as handled failures. The real work is throwing real `Error` subclasses with context, chaining causes, handling async rejections explicitly, and reserving catch for conditions you can actually address.

## Core Rules

### Always Throw `Error` Objects (Or Subclasses), Never Strings or Primitives

JavaScript lets you `throw "not found"` or `throw 404`, but doing so discards the stack trace and forces every catcher to handle arbitrary types. Always throw an `Error` or a subclass.

- `throw new Error("message")` captures a stack trace at construction, which is the single most valuable diagnostic.
- Throw subclasses for categories: `throw new TypeError(...)`, `throw new RangeError(...)`, or custom subclasses (`class NotFoundError extends Error`).
- Catchers can then branch on type: `if (err instanceof NotFoundError)`. A thrown string cannot be reliably distinguished from any other string.

Throwing a primitive is a debugging tax: the stack is gone, the type is ambiguous, and every consumer suffers. Always construct an `Error`.

### Subclass `Error` To Build A Meaningful Error Hierarchy

For an application, define a base error and specific subclasses so catchers can handle at the right granularity.

- `class AppError extends Error` as the base, with `class ValidationError extends AppError`, `class NotFoundError extends AppError`, etc.
- Set the name and capture the stack in modern environments: `super(message); this.name = "ValidationError"`.
- Attach structured fields: `this.field = field; this.value = value;` so the error carries actionable context, not just a message.

A hierarchy lets a boundary catch `AppError` for "any expected failure" while letting unexpected bugs propagate. A flat collection of bare `Error` objects forces catchers to parse messages, which is fragile.

### Preserve The Cause With `Error` Options

When you catch an error and throw a new one, preserve the original as the cause. Modern JavaScript supports `new Error("message", { cause: original })`, which keeps the full chain for diagnostics.

- `throw new AppError("config load failed", { cause: err })` retains `err` accessible via `err.cause`, with its stack and type.
- Without `cause`, wrapping an error discards the original stack and type, making the failure untraceable to its root.
- In older environments, attach the original manually (`this.original = err`) if `cause` is unavailable.

Never catch an error, construct a new message, and drop the original — the stack and type are the most valuable diagnostic information.

### Separate Expected Errors From Programmer Bugs

Not everything that throws should be caught. Distinguish:

- **Expected conditions** (user input invalid, resource not found, network timeout): these are part of the domain and should be represented as typed errors that a boundary catches to respond to the user or retry.
- **Programmer bugs** (`TypeError` from calling undefined, `ReferenceError`, logic errors): these indicate a broken program and should propagate and crash loudly in development, not be masked by a broad `catch`.

A broad `try { ... } catch (e) { }` around a block catches both, turning hard bugs into silent failures. Catch only the specific expected errors you can handle; let bugs surface.

### Handle Async Errors Via Rejection, And Always Await Or `.catch`

A `Promise` that rejects with nobody listening becomes an unhandled rejection, which in modern Node.js crashes the process and in browsers logs a warning. Async errors must be explicitly handled.

- `await` a promise inside `try/catch`, or attach `.catch(handler)`.
- An async function wraps thrown errors into a rejected promise, so `throw` inside an `async` function becomes a rejection the caller must await.
- Never fire-and-forget a promise without handling its rejection. `doAsync()` without `await` or `.catch` leaks an unhandled rejection if it fails.
- Use `Promise.all` carefully: one rejection rejects the whole aggregate, and the other results are lost. Use `Promise.allSettled` when you want all outcomes.

The async error path is easy to miss because there is no synchronous throw at the call site. Every promise must have a defined rejection handler.

### Use `try/catch/finally` With Each Clause In Its Role

- **`try`**: code that may throw.
- **`catch`**: handle a specific thrown error. Keep it narrow — catch the expected types, not everything.
- **`finally`**: always runs, for cleanup (closing resources). Keep it minimal; a throw in `finally` masks the original error.

Putting success-path code in `try` means its errors are caught by the same `catch` meant for the original operation. Structure blocks so each clause does its job.

### Do Not Swallow Errors Silently

`catch (e) {}` discards an error with no trace. If an error is truly expected and ignorable, say so explicitly: log it, or comment why it is safe to ignore. Silent swallowing creates debugging black holes where a failure happens and nobody knows.

If you catch only to log and rethrow, that is legitimate (it adds a diagnostic breadcrumb), but be deliberate: `catch (e) { log(e); throw; }` (rethrow preserves the original stack; `throw e` may reset it in some engines).

## Common Traps

### Throwing A String Or Number

`throw "failed"` loses the stack and forces arbitrary-type handling. Always `throw new Error(...)` or a subclass.

### Broad `catch` Masking Bugs

`catch (e) {}` around a block catches programmer bugs (`TypeError`, etc.) alongside expected errors, hiding bugs. Catch specific types.

### Losing The Cause When Wrapping

`catch (e) { throw new Error("failed") }` discards the original stack and type. Use `{ cause: e }` or attach the original.

### Unhandled Promise Rejection

Firing a promise without `await`/`.catch` leaks an unhandled rejection if it fails. Every promise needs a rejection handler.

### `throw e` Resetting The Stack

In some engines, `throw e` after catching resets the stack to the catch site. Prefer bare `throw;`-style rethrow where possible, or accept the stack frame change deliberately.

### Fire-And-Forget Async Without Context

`doAsync()` without `await` runs detached; if it fails, the rejection is unhandled and the call site has moved on. Await or attach `.catch`.

### Catching To Log And Forgetting To Rethrow

`catch (e) { log(e); }` swallows the error after logging. If the caller needs to know, rethrow; if swallowing is intended, document why.

### `Promise.all` Losing Partial Results

One rejection in `Promise.all` rejects everything and discards the other promises' results. Use `Promise.allSettled` when you need all outcomes.

## Self-Check

- [ ] All throws use `Error` objects or subclasses with captured stacks; no primitives are thrown.
- [ ] The application defines an error hierarchy (a base plus specific subclasses) so catchers can handle at the right granularity, with structured fields carrying context.
- [ ] Wrapped errors preserve the original via `{ cause }` (or manual attachment), retaining the stack and type for diagnosis.
- [ ] Expected conditions are typed errors a boundary catches; programmer bugs are allowed to propagate loudly, not masked by broad `catch`.
- [ ] Every promise has a rejection handler (`await` with `try/catch`, or `.catch`); no fire-and-forget async that leaks unhandled rejections.
- [ ] `try/catch/finally` uses each clause in its role; success-path code is outside the `try`, and `finally` is minimal.
- [ ] No error is swallowed silently; expected-and-ignorable errors are logged or commented, and logged-and-rethrown errors actually rethrow.
- [ ] `Promise.all` vs `Promise.allSettled` is chosen deliberately based on whether partial results matter.
