---
name: async_runtime_and_event_driven.md
description: Use when the agent is designing or debugging asynchronous, event-driven, or reactive code, choosing or configuring an async runtime, reasoning about event loops and task scheduling, detecting blocking calls inside async code, designing backpressure, implementing cancellation and timeouts, choosing between async and threads, or diagnosing stalls and starvation in an event-driven system. Covers the event loop model and cooperative scheduling, the cardinal sin of blocking the runtime thread, task scheduling and fairness, structured concurrency, backpressure and bounded channels, cancellation and timeout semantics, the difference between async and OS threads and when each fits, runtime selection and configuration, and the discipline of treating async as a resource-management discipline rather than a syntax. Also use when async code stalls or deadlocks, when a runtime thread is pinned, when cancellation does not propagate, or when deciding whether a workload should be async at all.
---

# Async Runtime And Event-Driven

Asynchronous programming is sold as a way to do more with less — many tasks on few threads — and it delivers that, but only when the code respects the model. The model is cooperative: tasks run until they yield (await), and the runtime schedules the next runnable task on the same thread. The cardinal sin is blocking that thread: a synchronous call that waits on I/O, holds a lock, spins, or calls into a CPU-bound library without yielding. While one task blocks its thread, every other task that the runtime would have scheduled on that thread waits too, and the throughput advantage of async collapses — or worse, a single blocking call stalls the whole runtime if the thread pool is small. Async systems fail not because async is hard in principle, but because the boundary between async-safe and blocking code is invisible: a function call that looks innocent (`read`, `sleep`, a lock acquisition, a library call) can freeze the runtime, and nothing in the type system warns you.

Agents tend to treat async as syntax (async/await keywords) rather than as a resource-discipline that pervades every call. The defects live at the boundaries: a `.await` on a future that internally blocks; a lock held across an await point, deadlocking when another task on the same thread needs it; an unbounded channel that buffers infinitely until memory exhausts because the producer is faster than the consumer; a cancellation that drops a task mid-operation, leaving a half-written resource or an un-awaited side effect; a CPU-bound computation run on the async runtime, starving all other tasks. The judgment problem is treating async as a whole-system discipline — every call must be async-aware, backpressure must be explicit, cancellation must be structured, and blocking work must be offloaded — rather than a local keyword choice.

This skill is about designing and debugging async and event-driven systems. It complements the thread-safety skill (shared-state correctness) and the actor skill (a specific concurrency model); here the question is the async runtime model, its failure modes, and the disciplines that keep it correct and fast.

## Core Rules

### Never Block The Runtime Thread; Offload Blocking And CPU-Bound Work

The single most important rule of async: a task that blocks its thread blocks every other task the runtime would schedule on it. Because async runtimes use a small thread pool (often one thread per core), a single blocking call can stall a large fraction of the system.

- **Identify blocking calls and keep them off the async runtime.** Synchronous file I/O, blocking network calls, locks/mutexes from the sync world, `thread::sleep`, foreign function calls into blocking libraries — all block the thread. Wrap them in an offload primitive (a dedicated blocking thread pool, `spawn_blocking`, or an equivalent) so the async runtime threads stay free.
- **Offload CPU-bound work.** A tight computation loop does not block on I/O but it occupies the thread without yielding, starving other tasks. Run CPU-bound work on a separate pool or yield periodically; do not run it directly on the async runtime.
- **Suspect blocking whenever an async system stalls.** If throughput collapses or latency spikes under load with no obvious cause, a blocking call on the runtime is the prime suspect. Profile to find tasks that run long without yielding.
- **Hold locks only across non-awaiting regions, or use async-aware locks.** A standard mutex held across an await point can deadlock if another task on the same thread needs it, or serialize the runtime. Use async mutexes where the lock must span an await, and question whether the lock needs to span an await at all.

### Make Backpressure Explicit With Bounded Channels

In an event-driven system, producers and consumers run at different rates, and without backpressure a fast producer overwhelms a slow consumer, exhausting memory or dropping work. Backpressure must be explicit, not emergent.

- **Use bounded channels between producers and consumers.** A bounded channel forces the producer to wait (or apply a strategy) when the consumer is behind, propagating slowness backward instead of buffering infinitely. An unbounded channel hides the imbalance until memory exhausts.
- **Choose a backpressure strategy deliberately.** When the channel is full: wait (apply backpressure, the safest default), drop (for lossy/ephemeral data), or sample/aggregate (for streams where recent matters more than all). Each has consequences; waiting can cascade slowness, dropping can lose work, aggregating changes semantics.
- **Propagate backpressure end-to-end.** If component A reads from a bounded channel and writes to another, a full downstream channel must slow A, which slows its upstream. End-to-end backpressure prevents any single component from buffering unbounded work.
- **Beware fire-and-forget spawns.** Spawning a task without bound (e.g., one task per incoming request, with no limit) is an unbounded producer; under load, tasks accumulate and exhaust memory. Bound concurrency with a semaphore or a worker pool.

### Make Cancellation Structured And Semantically Safe

Async tasks are often cancelled — by a timeout, by the caller dropping the future, or by a shutdown. Cancellation must be safe: the task must not leave resources in an inconsistent state or skip necessary cleanup.

- **Cancellation can happen at any await point.** When a task is cancelled, it stops at its current await; code after the await does not run unless it is in a cleanup path. Assume any await can be the last thing the task does.
- **Use RAII or defer/finally for cleanup.** Resources acquired before an await must be released even if the await is cancelled; rely on destructors or explicit cleanup blocks, not on code that runs after the await.
- **Avoid leaving shared state half-modified.** If a task modifies shared state across multiple awaits, cancellation between them can leave the state inconsistent. Either make the modification atomic (compute then commit in one step) or design the state to tolerate partial updates.
- **Make timeouts meaningful.** A timeout that cancels a task but leaves the underlying operation running (e.g., an HTTP request whose response is now ignored but which the server still processes) does not actually bound work; the operation continues to consume resources. Where possible, propagate cancellation to the underlying operation.
- **Beware of un-awaited tasks that escape cancellation.** A task spawned fire-and-forget is not cancelled when its spawner is cancelled; it runs to completion. If its lifetime should be tied to its parent, use structured concurrency (below).

### Use Structured Concurrency To Tie Child Lifetimes To Parents

Structured concurrency is the discipline of ensuring child tasks do not outlive their parent, so the program's task graph is a tree (with predictable lifetimes) rather than a tangle of detached tasks.

- **Spawn children into a scope that awaits their completion.** The parent does not return until all children have completed or been cancelled, preventing task leaks and making the program's concurrency structure explicit.
- **Propagate cancellation from parent to children.** When a parent is cancelled, its children are cancelled too; a child cannot run on after its parent has given up. This prevents orphaned tasks doing work whose results will never be used.
- **Collect child results or errors deterministically.** Structured concurrency makes it clear where child results go and how child errors propagate, rather than leaving them in detached tasks.
- **Prefer structured over unstructured spawning.** Fire-and-forget spawning has legitimate uses (a background daemon), but most concurrency should be structured so lifetimes and error propagation are predictable.

### Understand Scheduling, Fairness, And Starvation

The runtime decides which runnable task runs next, and its scheduling policy affects latency, throughput, and fairness. Tasks that do not yield can starve others.

- **Cooperative scheduling requires yielding.** A task that runs a long computation without awaiting starves other tasks on the same thread. Yield periodically (await a yield, or break work into chunks) on long CPU-bound paths, or offload them.
- **Beware priority inversion and starvation.** A task that is always runnable can starve tasks that yield often; a busy loop can monopolize a thread. The runtime's fairness guarantees (if any) determine how bad this can get.
- **Choose a runtime and configuration that fits the workload.** Work-stealing runtimes balance load across threads; single-threaded runtimes avoid cross-thread synchronization but limit throughput. Thread pool size, scheduling policy, and blocking-thread-pool size are tuning knobs that matter; understand them rather than accepting defaults.

### Choose Async Versus Threads For The Right Workload

Async is not universally better than threads; it is better for workloads with many concurrent, mostly-waiting tasks. Threads are better for few, CPU-bound or blocking tasks.

- **Async shines for many I/O-bound concurrent tasks.** Thousands of mostly-waiting connections are cheap as async tasks and expensive as OS threads. Use async for servers handling many concurrent I/O-bound requests.
- **Threads shine for few CPU-bound or unavoidably blocking tasks.** A handful of CPU-bound workers are simpler as threads; forcing them into async adds complexity without benefit (they still occupy a thread). Use threads for compute-heavy parallelism.
- **Mixing is normal and correct.** An async server can offload CPU-bound or blocking work to a thread pool; the two models coexist. The error is forcing everything into one model when the workload has both kinds of work.
- **Do not choose async for syntax alone.** If the workload is not I/O-bound and concurrent, async adds complexity (the disciplines above) without benefit. Choose async for its resource advantages, not because the keywords look modern.

## Common Traps

### Blocking The Runtime Thread

A synchronous I/O call, lock, sleep, or blocking library call on the async runtime thread, stalling every task the runtime would schedule on it. Keep blocking calls off the runtime; offload to a blocking thread pool.

### Holding A Sync Lock Across An Await

A standard mutex held across an await point, deadlocking when another task needs it or serializing the runtime. Use async-aware locks where a lock must span an await, or restructure to avoid holding across awaits.

### Unbounded Channel Hiding An Imbalance

An unbounded channel between a fast producer and slow consumer, buffering infinitely until memory exhausts. Use bounded channels with an explicit backpressure strategy.

### Fire-And-Forget Spawn Without Bound

Spawning a task per request with no concurrency limit, accumulating tasks under load until memory exhausts. Bound concurrency with a semaphore or worker pool.

### Cancellation Leaving Inconsistent State

A task cancelled mid-update leaving shared state half-modified, or a resource acquired before an await not released because the post-await cleanup did not run. Use RAII/cleanup blocks; make multi-step updates atomic or tolerant of partial completion.

### Timeout That Does Not Cancel The Underlying Operation

A timeout that gives up on a task but leaves the underlying request running, consuming resources for a result that will be ignored. Propagate cancellation to the underlying operation where possible.

### Detached Tasks Outliving Their Parent

A spawned task that continues after its parent has been cancelled or returned, doing work whose result is no longer needed (a task leak). Use structured concurrency to tie child lifetimes to parents.

### CPU-Bound Work Starving The Runtime and choosing Async For Syntax Rather Than Workload

A long computation on the async runtime that never yields, starving all other tasks on the thread. Offload CPU-bound work or yield periodically.

Using async because the keywords look modern, for a workload that is not I/O-bound and concurrent, adding the disciplines' complexity without the resource benefit. Choose async for many concurrent I/O-bound tasks; use threads for few CPU-bound tasks.

## Self-Check

- [ ] No blocking calls (synchronous I/O, sync locks, sleep, blocking libraries) run on the async runtime thread; they are offloaded to a dedicated blocking thread pool, and CPU-bound work is offloaded or yields periodically.
- [ ] No standard mutex or blocking primitive is held across an await point; async-aware locks are used where a lock must span an await, or the code is restructured to avoid it.
- [ ] Communication between producers and consumers uses bounded channels with an explicit backpressure strategy (wait, drop, or aggregate), and backpressure propagates end-to-end rather than being hidden by unbounded buffering.
- [ ] Concurrency is bounded (semaphore or worker pool) rather than fire-and-forget per request, preventing unbounded task accumulation under load.
- [ ] Cancellation is safe at every await point: cleanup uses RAII or defer blocks so resources are released, shared-state updates are atomic or tolerant of partial completion, and timeouts propagate cancellation to the underlying operation.
- [ ] Child task lifetimes are tied to parents via structured concurrency (children do not outlive parents, cancellation propagates downward, results and errors are collected deterministically), avoiding detached task leaks.
- [ ] Scheduling and fairness are considered: long computations yield or are offloaded to prevent starvation, and the runtime's configuration (thread pool size, blocking pool, scheduling policy) is understood rather than accepted as default.
- [ ] Async was chosen because the workload is many concurrent I/O-bound tasks (where async's resource advantages apply), not for syntax alone; CPU-bound or few-task workloads use threads, and the two models coexist where the workload is mixed.
