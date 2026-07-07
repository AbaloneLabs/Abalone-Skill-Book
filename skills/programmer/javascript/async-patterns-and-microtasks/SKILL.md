---
name: javascript_async_patterns_and_microtasks.md
description: Use when the agent is designing asynchronous JavaScript patterns and abstractions — concurrency pools and limiters, retry with backoff, async queues and semaphores, async iterators and generators, streaming pipelines, event emitters with async listeners, error boundaries for async work, or building supervisors that track and cancel concurrent tasks. Covers the design of async control-flow patterns and the pitfalls of unbounded concurrency, lost errors, zombie tasks, and unawaited promises. Distinct from the event loop mechanics and microtask scheduling internals, which are covered separately.
---

# Async Patterns And Microtasks

Once you understand the event loop, the next challenge is *designing* with it: building concurrency pools that do not overwhelm a downstream, retry logic that backs off correctly, async iterators that stream data, and supervisors that track and cancel concurrent tasks without losing errors or leaking work. The judgment problem is structuring async control flow so that concurrency is bounded, errors are observed, tasks are cancellable, and resources are released — the patterns that separate robust async code from code that works until load or failure exposes it.

Agents tend to fire unbounded `Promise.all` calls that overwhelm rate limits, write retry loops that retry the wrong things or never give up, lose errors in fire-and-forget tasks, or build async iterators that leak resources when the consumer stops early. The harm appears as rate-limit bans, thundering-herd retries, "unhandled rejection" crashes, zombie timers running after the user navigated away, and streams that hold file handles open forever. The real work is bounding concurrency deliberately, designing retry with the right policy, tracking concurrent tasks so none are lost, and wiring cancellation through every async operation.

## Core Rules

### Bound Concurrency With A Pool, Not Unbounded Promise.all

`Promise.all(items.map(asyncWork))` starts every operation at once. For a dozen items this is fine; for ten thousand API calls or database queries it overwhelms the downstream, trips rate limits, and exhausts connections. Bound concurrency deliberately.

- **Concurrency pool / limiter**: run at most N operations at a time, starting the next as each finishes. Implement with a simple pool (a counter and a queue) or use a library (`p-limit`, `async` pool, `p-map` with concurrency).
- Choose N by the downstream's limits (API rate limit, DB connection pool size, memory), not by guess. Measure and tune.
- `Promise.all` is appropriate when the set is small and bounded and the downstream can handle the parallelism; otherwise reach for a pool.
- Beware that even "concurrent" JS is interleaved on one thread — the pool controls how many in-flight async operations exist, not CPU parallelism. CPU-bound work needs workers, not a pool.

### Design Retry With The Right Policy

Retrying failed operations is essential for transient failures (network blips, throttling) but harmful for permanent failures (bad request, auth error). Design the policy deliberately.

- **Retry only idempotent, transient failures.** Retrying a non-idempotent operation (a payment, a create) on a network error can double-charge if the first call actually succeeded but the response was lost. Retry only when a repeat is safe.
- **Distinguish retryable errors.** Retry on timeouts, 5xx, 429 (with the `Retry-After` header), and connection errors. Do not retry on 4xx (except 429), validation errors, or auth errors — they will fail forever.
- **Back off, with jitter.** Exponential backoff (1s, 2s, 4s, ...) prevents thundering herd, but deterministic backoff still synchronizes retries across clients. Add jitter (randomized delay) so retries spread out.
- **Cap attempts and total time.** Infinite retry loops hide permanent failures and waste resources. Set a max attempts and a deadline; after that, fail.
- **Honor `Retry-After`.** When a 429 or 503 includes `Retry-After`, respect it instead of your own backoff.

### Track Concurrent Tasks So None Are Lost

Fire-and-forget async work (`doThing()` without awaiting) is dangerous: if it rejects, the error is unhandled; if the process exits, the work may be cut off. Track concurrent tasks in a supervisor.

- Keep a `Set` of in-flight Promises; add on start, remove on completion (`promise.finally(() => tasks.delete(promise))`).
- On shutdown, await or cancel all tracked tasks so no work is orphaned.
- Surface errors from tracked tasks to a central handler rather than letting them become unhandled rejections.
- For long-running servers, a task supervisor lets you drain in-flight work on shutdown (graceful drain) instead of cutting it off.

### Make Every Async Operation Cancellable

JS Promises have no built-in cancellation. An async operation started without a cancellation channel runs to completion regardless of whether the caller still cares. Wire cancellation through.

- **`AbortController`/`AbortSignal`** is the standard. `fetch(url, { signal })`, and increasingly other APIs, accept a signal. Thread the signal through your own async functions and check `signal.aborted` at await points.
- Pass the signal to every layer; a signal that stops at the top does not cancel work happening in a callee that never received it.
- On cancellation, settle the Promise (reject with an abort error or resolve with a sentinel) so callers' `await` resumes; do not leave them hanging.
- Cancel child operations when a parent is cancelled; a request that is aborted should cancel the sub-requests it spawned.

### Use Async Iterators And Generators For Streaming

Async iterators (`for await...of`, async generator functions) let you consume data as it arrives rather than buffering it all. They are the right tool for streams, pagination, and large datasets.

- An async generator (`async function*`) `yield`s values as they become available; the consumer processes them lazily, one at a time.
- For paginated APIs, an async generator that fetches the next page on demand is cleaner than collecting all pages into an array.
- **Always clean up.** An async iterator that holds a resource (file handle, connection) must release it when the consumer finishes *or* stops early (`break`, `return`, throw). Use `finally` in the generator, or `for await` with a `break` that triggers cleanup via the iterator's `return` method.
- Backpressure: if the consumer is slower than the producer, an async generator naturally applies backpressure (it does not produce the next value until the current is consumed). Unbounded buffering defeats this.

### Handle Errors At The Right Boundary

Async errors propagate through `await` and `Promise.all`, but the boundary where you catch them determines resilience.

- Catch at the level where you can actually recover (retry, fall back, degrade). Catching too high (in a global handler) loses context; catching too low (every await) swallows errors and hides bugs.
- `Promise.all` rejects on first failure; if you need all results including failures, use `Promise.allSettled` and inspect each outcome.
- For a pool, decide whether one failure cancels the batch or is recorded alongside successes. Use `allSettled`-style aggregation when partial success is acceptable.
- Never swallow errors with empty `.catch(() => {})`. If you must ignore an error, log it and narrow what you ignore.

### Sequence Dependent Operations Explicitly

Async code looks synchronous but is interleaved. Dependent operations must be sequenced explicitly, and shared state across `await`s is a race risk.

- `await a(); await b();` runs sequentially only because each `await` waits. `Promise.all([a(), b()])` runs concurrently. Make the intent explicit; accidental concurrency or accidental serialization are both bugs.
- For read-modify-write on shared state across an `await`, either serialize with a queue, make the critical section synchronous (no `await` between read and write), or use an immutable snapshot.

## Common Traps

### Unbounded Promise.all Overwhelming The Downstream

`Promise.all(thousands.map(callApi))` fires everything at once, tripping rate limits and exhausting connections. Use a concurrency pool sized to the downstream's capacity.

### Retrying Non-Idempotent Operations

Retrying a payment or create on a network error can double-charge if the first call succeeded but the response was lost. Retry only idempotent operations; for others, query state before retrying.

### Retry Without Jitter Synchronizing Clients

Deterministic exponential backoff makes all clients retry in lockstep after a widespread failure, re-overwhelming the service. Add jitter.

### Fire-And-Forget Tasks With Lost Errors

`doThing()` without awaiting loses any rejection as an unhandled rejection (which crashes Node). Track tasks and surface errors.

### No Cancellation, Leaving Zombie Tasks

An async operation without an `AbortSignal` runs to completion even after the user navigated away or the request was aborted. Wire cancellation through every layer.

### Async Iterator Leaking Resources On Early Exit

A generator holding a file handle that the consumer abandons via `break` leaks the handle unless the generator cleans up in `finally`. Always release resources in `finally`.

### Empty `.catch` Swallowing Errors

`.catch(() => {})` silences every error including programming bugs. Log or report; narrow what you ignore.

### `Promise.all` Canceling Nothing On First Failure

`Promise.all` rejects on first failure but the other input Promises keep running (their results are ignored). If you need to actually cancel siblings, wire an `AbortSignal` and cancel it on failure.

### Shared State Race Across Awaits

Two concurrent operations reading and updating the same variable across `await`s clobber each other. Serialize, make the critical section synchronous, or snapshot.

## Self-Check

- [ ] Concurrency is bounded by a pool sized to the downstream's limits; no unbounded `Promise.all` over large sets.
- [ ] Retry is applied only to idempotent, transient failures; non-idempotent operations check state before retrying; 4xx (except 429) is not retried.
- [ ] Retry uses exponential backoff with jitter, honors `Retry-After`, and has a capped attempt count and deadline.
- [ ] Concurrent tasks are tracked in a supervisor; fire-and-forget work is absent or its errors are surfaced; shutdown drains or cancels in-flight tasks.
- [ ] Cancellation is wired via `AbortSignal` through every layer; cancelled operations settle their Promises; child operations are cancelled with their parent.
- [ ] Async iterators/generators release resources in `finally` on both normal completion and early exit; backpressure is preserved.
- [ ] Errors are caught at the boundary where recovery is possible; `Promise.allSettled` is used when partial success is acceptable; no empty `.catch` swallows errors.
- [ ] Dependent operations are sequenced explicitly; accidental concurrency and accidental serialization are both avoided; shared state across `await`s is protected.
- [ ] CPU-bound work is moved to workers, not assumed parallelized by a concurrency pool.
- [ ] No zombie timers or listeners remain after the relevant request/component is gone; teardown cancels and cleans up.
