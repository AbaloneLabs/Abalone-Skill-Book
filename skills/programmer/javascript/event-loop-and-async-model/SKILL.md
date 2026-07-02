---
name: javascript_event_loop_and_async_model.md
description: Use when the agent is writing or debugging JavaScript/Node.js asynchronous code, reasoning about microtasks vs macrotasks, the task/job queues, callback/Promise/async-await relationships, Promise chaining and error propagation, concurrent vs parallel execution, scheduling with setTimeout/setImmediate/queueMicrotask, unhandled rejections, or reviewing code that looks synchronous but behaves asynchronously. Covers the event loop model, starvation, cancellation, and the gap between synchronous appearance and asynchronous reality.
---

# Event Loop And Async Model

JavaScript executes on a single thread with an event loop that drains queues of work. There is no true parallelism in plain JS; there is concurrency via interleaving. The model has two queue tiers (tasks and microtasks) with different priorities, and a set of constructs (callbacks, Promises, async/await) that map onto them in non-obvious ways. The judgment problem is reasoning about *when* code runs and *in what order*, because the surface syntax hides the scheduling.

Agents tend to treat async code as if it were synchronous ("the next line runs right after"), assume Promises and callbacks are interchangeable, or believe `async/await` removed the need to understand the loop. The harm is concrete: microtask starvation where a recursive `.then` chain never lets a render or I/O callback run, errors that vanish because a rejected Promise was never awaited, race conditions between interleaved async operations, and "this works most of the time" timing bugs. The real work is knowing which queue each construct uses, what guarantees order provides, and where the synchronous-looking surface hides asynchronous traps.

## Core Rules

### Know The Two Queue Tiers And Their Priority

The event loop has, at minimum, two queues with strict priority:

- **Microtask queue**: drained completely after every task and after every microtask, before any macrotask runs. `Promise.then/catch/finally` callbacks, `queueMicrotask`, `MutationObserver` callbacks, and `await` continuation go here.
- **Macrotask (task) queue**: drained one task at a time, with rendering and I/O between tasks. `setTimeout`/`setInterval`, I/O callbacks, `setImmediate` (Node), and message events go here.

The consequence: a chain of `.then` calls (all microtasks) can starve the task queue indefinitely, blocking rendering, timers, and I/O. `setTimeout(fn, 0)` does not run "next"; it runs after all pending microtasks and any already-queued tasks. Reason about which tier a callback lands in before assuming an order.

### Understand The Callback → Promise → Async-Await Lineage

These are not three separate systems; async/await is syntax over Promises, and Promises formalize callback patterns. Knowing the lineage prevents category errors.

- A `Promise` is a state machine (pending → fulfilled/rejected) with guaranteed-asynchronous `.then` handlers (always a microtask, even if already settled).
- `async` functions always return a Promise; `await` suspends the function, yields to the loop, and resumes when the awaited Promise settles, scheduling the continuation as a microtask.
- `await` is roughly sugar for `.then`, but with synchronous-looking control flow. Errors from an awaited rejected Promise throw at the `await` site, so `try/catch` works.

Do not mix models carelessly: a callback-based API called inside an async function still needs to be promisified (`util.promisify`, or a `Promise` constructor) to be awaited correctly. A `Promise` whose rejection is never observed produces an "unhandled rejection."

### Await Every Promise, Or Attach A Handler

A Promise that rejects with no `.catch` or `await` becomes an unhandled rejection, which in modern Node terminates the process and in browsers logs loudly. The rule: every Promise you create or receive must either be awaited within a `try` or have a `.catch` attached.

- In async functions, `await` the Promise inside `try/catch`.
- For fire-and-forget work, attach a `.catch` that at least logs, or route through a supervisor that tracks the task.
- Do not call an async function without handling its returned Promise, even if you "don't care about the result" — an unhandled rejection can still crash the process.

### Distinguish Concurrency From Parallelism

JavaScript is single-threaded for JS execution, so async gives concurrency (interleaved progress), not parallelism (simultaneous compute). True parallelism requires Web Workers (browser) or worker threads/child processes (Node).

- I/O-bound work benefits enormously from async concurrency: while one request waits, the loop handles others.
- CPU-bound JS work blocks the loop entirely; making it `async` does not help because there is no await point inside the compute. Move it to a worker or chunk it with cooperative yielding (`setImmediate`/`await` between batches).
- `Promise.all` runs operations concurrently (interleaved), not in parallel; the actual I/O parallelism comes from the underlying async APIs, not from JS threads.

### Use The Right Scheduling Primitive

Different schedulers have different semantics; picking the wrong one changes timing and starvation behavior.

- `queueMicrotask(fn)` / `Promise.resolve().then(fn)` — run as soon as possible, before any task. Use for "do this after the current sync work but before I/O/render." Overuse starves tasks.
- `setTimeout(fn, 0)` (clamped to ~4ms for nested timeouts in browsers) — run on a later task. Use to yield to rendering/I/O.
- `setImmediate(fn)` (Node only) — run after I/O events in the current phase. Semantics differ from `setTimeout`; do not assume portability.
- `requestAnimationFrame` (browser) — run before paint; use for visual work, not generic deferral.

When you need to yield to keep a UI responsive during long work, break the work into chunks separated by `await new Promise(r => setTimeout(r))` or scheduler APIs (`scheduler.yield()` where available).

### Make Cancellation And Cleanup Explicit

JS has no built-in Promise cancellation. A Promise, once started, runs to settlement; the only way to "cancel" is to ignore its result or signal the underlying operation. Design cancellation deliberately.

- `AbortController`/`AbortSignal` is the standard cancellation channel for fetch and many APIs; thread it through your async operations.
- `finally` blocks run on both normal completion and thrown errors; use them for cleanup (closing streams, releasing locks) so it happens regardless of outcome.
- A "cancelled" async operation should settle its Promise (resolve with a sentinel or reject with an abort error) so callers' `await` resumes; do not leave callers hanging.

### Treat "Synchronous-Looking" Code As Asynchronous

The biggest mental-model hazard: `async/await` makes asynchronous code read top-to-bottom, but the execution is still interleaved and non-atomic. Between any two `await`s, other code can run and mutate shared state.

- Do not assume a value is unchanged across an `await` without re-checking it.
- Do not hold a "lock" via a flag across an `await` and assume no other task entered the section — cooperative flags can be bypassed by interleaving.
- Sequence dependent operations with explicit ordering; do not rely on the visual layout to imply atomicity.

## Common Traps

### Forgetting To `await` A Promise

`doAsync()` without `await` starts the work but the caller proceeds immediately; errors become unhandled rejections and the return value is the Promise, not the result. Lint rules (`no-floating-promises`) catch this; enable them.

### `Promise.all` Rejecting On First Failure

`Promise.all` rejects as soon as any input rejects and does not wait for the others (though they keep running). If you need all results including failures, use `Promise.allSettled` and inspect each outcome. If you need first-success, use `Promise.any`.

### Microtask Starvation From Recursive `.then`

A loop built from `.then(() => Promise.resolve().then(loop))` never yields to the task queue, freezing rendering and I/O. Break such loops with a real task (`setTimeout`) or a counter-based yield.

### Treating `async` Functions As Making Things Parallel

Wrapping a function in `async` does not parallelize it. `await a(); await b();` runs them sequentially even if both are async. For concurrency, start both then await: `const [ra, rb] = await Promise.all([a(), b()]);`.

### Swallowing Errors With Empty `.catch`

`.catch(() => {})` silences every error, including programming bugs. If you must swallow, log or report the error and narrow what you ignore. An empty catch is a debugging black hole.

### Assuming `setTimeout(fn, 0)` Runs Immediately

It runs on a future task, after pending microtasks. Code that depends on "run right after this" should use a microtask; code that depends on "yield to the event loop" should use a task. Mixing them up causes ordering bugs.

### Creating Promises You Never Settle

A `new Promise((resolve, reject) => { ... })` where a code path never calls `resolve`/`reject` leaves callers awaiting forever. Ensure every branch settles, including error paths; prefer promisifying existing callback APIs over hand-rolling constructors.

### Race Conditions From Interleaved Async On Shared State

Two async functions reading and updating the same variable across `await`s can clobber each other. Either serialize access (a queue), make the critical section synchronous (no awaits between read and write), or use an immutable snapshot pattern.

## Self-Check

- [ ] Every Promise is either `await`ed inside `try/catch` or has a `.catch`/handler attached; no floating Promises produce unhandled rejections.
- [ ] The microtask-vs-task tier of each callback is known; no recursive microtask chain can starve the task queue.
- [ ] `Promise.all`/`allSettled`/`any` is chosen to match whether first-failure, all-results, or first-success semantics are needed.
- [ ] Concurrent vs sequential intent is explicit: independent awaits are started together then awaited, not serialized by accident.
- [ ] CPU-bound work is moved to a worker or chunked with yields; it is not assumed to be parallelized by `async`.
- [ ] Cancellation uses `AbortController`/`AbortSignal` or an equivalent explicit channel; cancelled operations still settle their Promises.
- [ ] Cleanup runs in `finally` regardless of success or failure.
- [ ] Shared mutable state across `await`s is protected (serialization, synchronous critical section, or snapshot) against interleaving races.
- [ ] Scheduling primitives (`queueMicrotask`, `setTimeout`, `setImmediate`, `requestAnimationFrame`) match the required timing and yield behavior.
- [ ] No empty `.catch` silently swallows errors; swallowed errors are logged or narrowed.
