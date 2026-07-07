---
name: typescript_async_types_and_promises.md
description: Use when the agent is writing TypeScript with async/await or Promises, typing function return types (Promise<T> vs T), handling Promise typing pitfalls (floating promises, unhandled rejections, Promise<void>), choosing between Promise.all, Promise.allSettled, Promise.race, and Promise.any, typing async iterators and AsyncGenerator, reasoning about thenable vs Promise, avoiding `async` functions that never await, returning Promise<T> vs T from overloads, or debugging "Promise<T> is assignable to Promise<void>" and floating-promise bugs. Covers Promise typing, the floating-promise problem, concurrent-combinator selection, async iterator typing, and the tradeoff between type-level Promise correctness and ergonomic async/await.
---

# Async Types And Promises

TypeScript models asynchronous values as `Promise<T>`, and `async`/`await` makes them ergonomic to write. But the type system does not save you from several runtime pitfalls: floating promises that are never awaited, rejections that nobody handles, combinators whose result types differ, and async functions that accidentally swallow errors. The judgment problem is typing async code so that the compiler catches what it can, while applying discipline for the pitfalls the type system cannot express.

Agents tend to forget to await a promise (creating a floating promise), return `Promise<any>` from untyped library calls, use `Promise.all` where `allSettled` is needed, or write `async` functions that never await and silently return a resolved promise. The harm appears as unhandled rejections that crash processes or vanish, operations that "succeed" but never actually completed, partial results lost when one promise rejects, and async iterators whose types mislead consumers. The real work is awaiting every promise deliberately, typing return types explicitly, choosing combinators by failure semantics, and treating floating promises as errors.

## Core Rules

### Always Await Or Handle Every Promise (No Floating Promises)

A floating promise is a promise created but neither awaited nor given a rejection handler. Its rejection becomes an unhandled rejection, and its completion is detached from the caller's control flow.

- `doAsync()` without `await` runs detached; the caller moves on, and if it rejects, the rejection is unhandled.
- ESLint's `no-floating-promises` rule (in `recommended-type-checked`) catches these at compile time. Enable it for any serious TypeScript project.
- If you intentionally fire-and-forget, attach `.catch(handler)` so the rejection is handled, and comment the intent.

The type system alone does not force awaiting — a call returning `Promise<void>` can be ignored. Discipline (and the lint rule) is required. Treat an unawaited promise as a bug unless explicitly handled.

### Type Async Return Types Explicitly

An `async` function always returns a `Promise`; annotate the return type to make the contract explicit and to catch inference drift.

- `async function fetchUser(id: string): Promise<User>` is explicit; the compiler verifies the resolved value matches `User`.
- Without an annotation, inference may widen (e.g., `Promise<any>` if the body returns an untyped library call), hiding mismatches.
- For functions that may return synchronously or asynchronously, decide deliberately: a function declared `async` always returns a `Promise`, even if it returns a plain value; a non-async function returning `Promise<T>` does not introduce an extra layer.

Annotating `Promise<T>` return types also documents the failure mode: a `Promise<User>` may reject, and the caller must handle it.

### Choose The Combinator By Failure Semantics

`Promise.all`, `Promise.allSettled`, `Promise.race`, and `Promise.any` differ in how they handle rejection, and the choice changes correctness.

- **`Promise.all([p1, p2, ...])`**: waits for all to fulfill; rejects on the first rejection, discarding the others' results. Use when all must succeed and partial results are useless. Type: `Promise<[T1, T2, ...]>`.
- **`Promise.allSettled([...])`**: waits for all to settle (fulfill or reject); never rejects. Returns an array of `{ status, value } | { status, reason }`. Use when you want all outcomes and must handle failures individually. Type: `Promise<PromiseSettledResult<T>[]>`.
- **`Promise.race([...])`**: settles (fulfills or rejects) with the first to settle. Use for timeouts or "first of" patterns. Type: `Promise<T>` (of the first).
- **`Promise.any([...])`**: fulfills with the first fulfillment; rejects only if all reject (with an `AggregateError`). Use when one success is enough.

The common bug is using `Promise.all` when one failing operation should not discard the others' results. Match the combinator to whether partial success is meaningful.

### Distinguish `Promise<T>` From `T`, And Avoid Double-Wrapping

An `async` function wraps its return value in a `Promise`. Returning a `Promise<T>` from an `async` function does not create `Promise<Promise<T>>` — it flattens to `Promise<T>`. But mixing patterns carelessly causes confusion.

- `async function f(): Promise<T> { return somePromiseT; }` — correct, flattens to `Promise<T>`.
- `async function f() { return await somePromiseT; }` — the `await` is redundant inside an async function returning the value; it adds a microtask but does not change the type. Avoid unnecessary `await` in returns.
- A non-async function returning `Promise<T>` vs an async function returning `Promise<T>` differ in stack traces and microtask scheduling but not in type. Be consistent.

### Handle Rejection Explicitly, Do Not Let It Propagate Unhandled

Every `Promise<T>` may reject. The caller must handle it via `try/catch` around `await`, or `.catch()`, or consciously let it propagate to a boundary that handles it.

- `try { const u = await fetchUser(id); } catch (e) { ... }` handles at the call site.
- Letting rejection propagate is valid if a boundary above handles it (e.g., an Express error handler), but document that the function may reject.
- An unhandled rejection crashes modern Node.js and logs a warning in browsers. Never assume a promise "probably resolves."

### Type Async Iterators And Generators Carefully

Async iterators (`AsyncIterable<T>`, `AsyncIterator<T>`) and async generators (`AsyncGenerator<T>`) model streaming async data. Their types must reflect that `next()` returns `Promise<IteratorResult<T>>`.

- `async function* stream(): AsyncGenerator<Item>` yields items asynchronously; consumers use `for await (const item of stream())`.
- The return type of an async generator is `AsyncGenerator<YieldType, ReturnType, NextType>`; annotate at least the yield type.
- Errors in an async generator propagate as a rejected `next()` promise; consumers must handle them in the `for await` loop or catch.

Mistyping an async iterable as a sync iterable causes `for await` to misbehave or fail to type-check. Use the async variants consistently.

### Avoid `async` Functions That Never `await`

An `async` function with no `await` still returns a `Promise` and schedules a microtask, adding overhead and implying asynchrony that does not exist. ESLint's `require-await` flags these. If a function does not need to be async, make it synchronous; if it must conform to an async interface, document why.

## Common Traps

### Floating Promise (Unawaited Call)

`doAsync()` without `await` detaches the operation; rejections are unhandled and completion is untracked. Await or attach `.catch`.

### `Promise.all` Discarding Partial Results

One rejection rejects the whole `Promise.all`, losing the other results. Use `Promise.allSettled` when partial success matters.

### `Promise<any>` From Untyped Library Calls

Calling an untyped library returns `Promise<any>`, hiding mismatches. Add types (declaration files or explicit annotations) and annotate return types.

### Redundant `await` In Async Return

`return await x` inside an async function adds a microtask and (historically) can alter stack traces. Return the promise directly: `return x`.

### Unhandled Rejection Crashing The Process

A rejected promise with no handler crashes modern Node.js. Every promise needs a rejection path, even if it is propagation to a boundary.

### `async` Function That Never Awaits

Adds a Promise wrapper and microtask for no reason, implying false asynchrony. Make it synchronous or add the missing `await`.

### Mistyping Async Iterable As Sync

Using `Iterable<T>` where `AsyncIterable<T>` is meant breaks `for await` and hides that `next()` returns a promise. Use the async iterator types.

### Combinator Result Type Surprise

`Promise.allSettled` returns `PromiseSettledResult<T>[]`, not `T[]`. Access `.value`/`.reason` after checking `.status`, or the types will not narrow.

## Self-Check

- [ ] Every promise is awaited or has an explicit rejection handler; `no-floating-promises` is enabled and no floating promises remain.
- [ ] Async function return types are annotated as `Promise<T>` (or `Promise<void>`) explicitly, with the resolved type verified by the compiler.
- [ ] The concurrency combinator (`all`/`allSettled`/`race`/`any`) is chosen by failure semantics: `allSettled` when partial results matter, `all` when all must succeed.
- [ ] `Promise<T>` vs `T` is not double-wrapped; redundant `return await` is avoided in async functions.
- [ ] Every `Promise<T>` has a defined rejection path (try/catch, `.catch`, or documented propagation to a boundary).
- [ ] Async iterators and generators use `AsyncIterable`/`AsyncGenerator` types, not the sync variants.
- [ ] No `async` function lacks an `await` (`require-await` enforced); synchronous functions are not marked async.
- [ ] Calls to untyped libraries are typed (declarations or annotations) so `Promise<any>` does not hide mismatches.
- [ ] `Promise.allSettled` results are narrowed on `.status` before accessing `.value`/`.reason`.
