---
name: concurrency_and_future.md
description: Use when the agent is writing concurrent Scala code, using Future and Promise, choosing or configuring an ExecutionContext, bridging blocking calls, handling cancellation and timeouts, working with Cats Effect or ZIO concurrency primitives, or diagnosing thread starvation, deadlocks, blocked dispatchers, and uncancellable operations in Scala applications.
---

# Concurrency and Future

Scala's `Future` is eager and runs on a shared `ExecutionContext`, and those two properties are the root of most concurrency bugs. A `Future` begins executing the moment it is constructed, cannot be cleanly cancelled, and consumes a thread from the dispatcher for as long as it runs. Block on a `Future` from within another `Future` on the same dispatcher and you starve the pool; run a long blocking call without `blocking` and the dispatcher's threads exhaust; assume a `Future` can be cancelled and the work keeps running after the caller moved on. The judgment problem is not "how do I write `Future.map`" but understanding that `Future` is an eager, thread-consuming abstraction whose correctness depends on dispatcher health, and that the richer effect systems (Cats Effect `IO`, ZIO) exist precisely because `Future`'s eagerness and lack of cancellation make it unsuitable for disciplined concurrency.

The recurring failure mode is a developer who treats `Future` like a lightweight async marker, chains dozens of them, blocks on a result inside another `Future`, and then discovers the application deadlocks or starves under load because the fixed dispatcher has no free threads. The opposite failure is using `Future` where cancellation and timeout matter (user-facing requests, graceful shutdown) and being unable to stop work that is no longer needed. Concurrency in Scala is a set of tradeoffs between eagerness, cancellation, dispatcher health, and blocking discipline, and the right choice depends on whether the work is short and non-blocking (fine for `Future`), long or blocking (needs `blocking` or a dedicated pool), or cancellable (needs an effect type).

## Core Rules

### Understand that `Future` is eager and runs on the dispatcher

`Future { work }` starts executing immediately on the supplied (or implicit) `ExecutionContext`. This has consequences:

- The work runs whether or not anyone reads the result; constructing a `Future` is a side effect.
- The work occupies a dispatcher thread until it completes; a long-running `Future` reduces pool capacity for everything else.
- `Future` is not referentially transparent: `val f = Future { x }; f.flatMap(...)` runs once, but inlining `Future { x }.flatMap(...)` re-runs `x` each time.

If you need laziness (describe now, run later) or cancellation, `Future` is the wrong tool; use `IO`/`ZIO`.

### Choose and size the `ExecutionContext` by workload

The dispatcher is a shared resource whose sizing determines throughput and headroom. Rules:

- For CPU-bound work, a fixed pool sized to the core count is appropriate; more threads than cores cause context-switching overhead.
- For mixed or blocking work, do not use the CPU-bound pool; blocking calls exhaust it. Use a separate `ExecutionContext` with enough threads for expected blocking concurrency, or wrap blocking in `blocking { }`.
- The global `scala.concurrent.ExecutionContext.global` is a fork-join pool suitable for CPU-bound work; avoid it for blocking work.
- In server frameworks (Akka, Pekko, Play), understand the default dispatcher and whether to use a dedicated dispatcher for blocking or high-volume work.

Mis-sizing the dispatcher is the most common cause of throughput collapse under load.

### Never block on a `Future` from within another `Future` on the same pool

`Await.result(f, ...)` from inside a `Future` body holds a dispatcher thread while waiting, reducing effective parallelism. Under load, all threads wait and the system deadlocks. Rules:

- Compose with `flatMap`/`for` instead of blocking.
- If you must block (interfacing with a blocking API), wrap the blocking call in `scala.concurrent.blocking { }`, which hints the pool to spawn a spare thread.
- For libraries that block, use a dedicated `ExecutionContext` (a `ThreadPoolExecutor` with enough threads) so blocking does not starve the CPU pool.

### Cancellation is not supported by `Future`; design accordingly

A `Future` cannot be cancelled. Once started, it runs to completion regardless of whether the result is still needed. Rules:

- For user-facing requests that may time out or be abandoned, the underlying work continues after the caller gives up, wasting resources.
- Implement timeouts by racing the `Future` against a delayed `Future` (`Future.firstCompletedOf(Seq(work, timeout))`), but understand the work `Future` still runs to completion; you are only abandoning its result.
- For true cancellation, use `IO`/`ZIO`, which support cancellation of fibers, including releasing resources and stopping cooperative loops.

If cancellation matters (requests, shutdown, competitive races), `Future` is insufficient.

### Timeouts must race, and abandoned work must be tolerated

Implement a timeout as a race between the work and a delayed failure. Rules:

- `Future.firstCompletedOf` or a helper gives you the first to complete; on timeout, the work `Future` is abandoned but still running.
- Ensure the abandoned work is tolerable: it should not mutate shared state in a way that breaks correctness, and ideally it should be cheap or stoppable via an external signal.
- In effect systems, `timeout`/`race` cancels the loser, which is the correct semantics for resource cleanup.

### Handle errors in the `Future`, not via thrown exceptions in callbacks

A `Future`'s error channel is `Throwable`; `recover`/`recoverWith` handle failures. Rules:

- Do not throw synchronously inside `map`/`flatMap` expecting it to be caught by `recover`; it will, but it is clearer to return a failed `Future`.
- Reserve `Future.failed` for expected errors that the error channel models; for domain errors, consider `Either`/`Option` inside the `Future` to keep the error typed.
- At the boundary (server handler, main), `recover` to a default or a 500; unhandled failed `Future`s propagate to the dispatcher's reporter and vanish.

### `Promise` is for bridging callback APIs, not for manual future management

A `Promise[A]` is a single-assignment cell you complete manually, producing a `Future`. Use it to bridge callback-based APIs (a listener completes the promise). Do not use `Promise` to manually orchestrate `Future`s that could be expressed with `map`/`flatMap`; that re-implements the combinators poorly.

### Effect systems (Cats Effect, ZIO) solve what `Future` cannot

`IO` and `ZIO` are lazy, referentially transparent, cancellable, and support typed errors and resource safety. Use them when:

- You need cancellation (requests, shutdown, competitive races).
- You need referential transparency (the value describes work, run once at the edge).
- You need typed errors or resource safety across cancellation.
- You need structured concurrency (fibers, race, supervision).

`Future` remains fine for simple, non-blocking, non-cancellable async orchestration. Reach for effect systems when `Future`'s limitations start costing you.

## Common Traps

### Blocking inside a `Future` on the CPU dispatcher

`Future { Thread.sleep(10000) }` on `global` holds a fork-join thread for 10 seconds. Under enough such calls, the pool starves. Use `blocking { }` or a dedicated pool.

### `Await.result` inside a `Future` body

Awaiting from inside a `Future` holds a dispatcher thread, reducing parallelism and risking deadlock when all threads wait. Compose with `flatMap` instead.

### Assuming `Future` can be cancelled

It cannot. A timed-out request's `Future` keeps running, consuming resources. Use an effect type if cancellation is required.

### Using `global` for blocking work

The fork-join `global` dispatcher is for CPU-bound work; blocking calls on it starve the pool. Use a dedicated blocking pool.

### Eager `Future` construction treated as lazy

`val f = Future { sideEffect() }` runs `sideEffect()` immediately. If you wanted to defer, wrap in a function or use an effect type.

### Unhandled failed `Future`s vanishing

A failed `Future` with no `recover` reports to the dispatcher's reporter (often a log) and the result is lost. Always `recover` at boundaries.

### Racing without tolerating the abandoned branch

`firstCompletedOf` returns the winner but the loser keeps running. If the loser mutates state or holds resources, the race produces bugs. Ensure abandoned branches are tolerable, or use a cancellable effect.

### `Promise` used to re-implement combinators

Hand-rolling `Future` orchestration with `Promise` instead of `flatMap`/`for` produces fragile, hard-to-read code. Use `Promise` only to bridge callbacks.

## Self-Check

- Are `Future`s understood as eager and thread-consuming, with no assumption of laziness or cancellability, and is the global dispatcher used only for CPU-bound work?
- Is the `ExecutionContext` sized and chosen by workload (fixed CPU pool for CPU-bound, dedicated blocking pool for blocking), with `blocking { }` used to bridge blocking calls?
- Are there no `Await.result` calls inside `Future` bodies, with composition done via `flatMap`/`for` instead?
- Where cancellation or timeout matters, is an effect type (`IO`/`ZIO`) used, or are timeouts implemented as races whose abandoned branches are tolerable?
- Are errors handled via `recover`/`recoverWith` at boundaries, with no reliance on thrown exceptions inside callbacks for control flow?
- Are `Promise`s used only to bridge callback APIs, not to manually re-implement `map`/`flatMap` orchestration?
- If the codebase uses Cats Effect or ZIO, are fibers, race, and resource safety used to achieve cancellation and structured concurrency that `Future` cannot provide?
- Has the dispatcher been load-tested for headroom, with no thread starvation or deadlock under realistic concurrency?
