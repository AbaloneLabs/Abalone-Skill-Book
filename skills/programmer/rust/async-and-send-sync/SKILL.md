---
name: async_and_send_sync.md
description: Use when the agent is writing, debugging, or reviewing asynchronous Rust code, reasoning about Send and Sync bounds across await points, choosing or configuring an async runtime, diagnosing blocking behavior inside async tasks, handling task cancellation and cancellation safety, or working with Pin and self-referential futures. Also covers the failure mode of blocking the async runtime with synchronous work, holding locks or guards across await points, cancellation-unsafe code that corrupts state when dropped, and the subtle interaction between runtime choice, executor, and reactor.
---

# Async And Send/Sync

Async Rust decouples concurrency from threads: many tasks run on a small pool of worker threads, cooperatively yielding at await points so the runtime can run other tasks. This is a powerful model, but it introduces a set of failure modes that do not exist in threaded code and that are easy to miss because the code looks sequential. The judgment problem is that an async task is not a thread — it does not get preempted, so a task that does CPU-bound or blocking work without yielding freezes the worker thread and every other task parked on it; an `await` is a suspension point where the task may be moved between threads, so anything held across it must be `Send` (and the future itself must be `Send` to be spawned on a multi-threaded runtime); and cancellation is implicit (the runtime can drop a future at any await point), so a future that is not cancellation-safe corrupts its state when dropped mid-operation. The discipline is to never block the runtime (offload blocking and CPU-bound work), to be deliberate about what crosses await points (no non-Send guards, no held locks across await unless the guard is cancellation-safe), to write cancellation-safe futures or to make cancellation explicit, and to understand the runtime's threading model so that `Send`/`Sync` bounds are satisfied where they must be.

Agents tend to write async code as if it were threaded, and the harm appears as a runtime that stalls (one task blocked on a synchronous call freezes a worker thread), as compile errors about `Send` that are hard to diagnose (a non-Send guard held across await), as state corruption when a future is cancelled mid-operation (a lock acquired but never released because the future was dropped before the release), and as mysterious deadlocks from holding a lock across an await point that never resumes. The judgment is to treat every await as a point where the task may be suspended, moved, or cancelled; to offload blocking work to `spawn_blocking` or a dedicated thread; to audit held-across-await state for Send-ness and cancellation safety; and to choose the runtime and its configuration with awareness of the workload. Async Rust is cooperative and cancellation-implicit; code that ignores either will stall or corrupt.

## Core Rules

### Never Block The Async Runtime

An async task runs on a worker thread and is not preempted; it yields only at await points. A task that does blocking work (a synchronous syscall, a long CPU-bound computation, a blocking library call) holds the worker thread for the duration, starving every other task parked on that thread.

- **Offload blocking I/O and syscalls to `spawn_blocking` (or a dedicated thread).** A blocking call inside an async task freezes the worker; move it to a blocking thread pool so the async runtime keeps running other tasks.
- **Offload CPU-bound work to a dedicated thread or rayon.** A long computation without an await holds the worker; either chunk it with periodic yields or move it off the async runtime entirely.
- **Beware blocking calls hidden in libraries.** A library that does blocking I/O (a synchronous database driver, a file read on some platforms) blocks the runtime even if the call looks innocuous; use async-native libraries or spawn_blocking.
- **Size the blocking pool for the workload.** The blocking thread pool has its own limit; too few blocking threads bottleneck blocking work, and the limit is separate from the async worker count.

### Be Deliberate About What Crosses Await Points

An await is a suspension point: the task may be suspended and later resumed on a different worker thread (on a multi-threaded runtime), and it may be dropped (cancelled). Anything held across the await must tolerate this.

- **Anything held across an await must be `Send`** to cross threads on a multi-threaded runtime. A non-Send guard (an `Rc`, an `RefCell` borrow, a non-Send lock) held across an await makes the future non-Send and cannot be spawned on a multi-threaded executor.
- **Do not hold non-reentrant locks across await.** A lock held across an await that resumes on another thread, or that is cancelled before release, can deadlock or corrupt state; acquire, do the synchronous work, release, then await.
- **Prefer short-lived borrows and scoped guards.** Acquire a lock or a borrow, complete the synchronous operation, release it before awaiting; this keeps the held-across-await surface minimal and auditable.
- **Audit the held-across-await surface explicitly.** The compiler will catch non-Send futures, but cancellation-safety and lock-holding across await are not compiler-checked; review them.

### Write Cancellation-Safe Futures Or Make Cancellation Explicit

Cancellation in async Rust is implicit: the runtime can drop a future at any await point (a timeout, a `select!` branch, a dropped task handle). A future that is not cancellation-safe leaves inconsistent state when dropped mid-operation.

- **Make futures cancellation-safe.** A future dropped at any await point must leave its state consistent: locks released (or never acquired across the await), partial writes recoverable, invariants maintained. The canonical failure is acquiring a lock, awaiting, and being dropped before release.
- **Use `select!` and timeouts carefully.** These cancel the unchosen branches by dropping them; if a dropped branch leaves inconsistent state, the bug appears only when that branch is cancelled.
- **Prefer making cancellation explicit where safety is hard.** If a future cannot easily be made cancellation-safe, structure the code so cancellation happens only at safe points, or use a cancellation token the task checks itself.
- **Test cancellation.** Cancellation bugs are rare in normal runs and appear under load or timeouts; deliberately cancel futures at different await points to verify safety.

### Understand The Runtime's Threading Model And Configure It For The Workload

The runtime determines whether futures must be `Send`, how tasks are scheduled, and how blocking work is handled. Choose and configure the runtime with awareness of the workload.

- **Multi-threaded runtimes require `Send` futures** to move tasks between worker threads; single-threaded runtimes do not, allowing non-Send types (`Rc`, `RefCell`) but sacrificing parallelism.
- **Size the worker and blocking pools for the workload.** Workers run async tasks; the blocking pool runs spawn_blocking work; the two have separate limits and should be sized independently based on the workload mix.
- **Do not nest runtimes.** Starting a runtime inside another runtime's task (a blocking call that itself starts an async runtime) deadlocks or behaves unexpectedly; offload to spawn_blocking or restructure.
- **Be aware of the runtime's I/O and timer drivers.** Async I/O and timers depend on the runtime's reactor; a runtime without a reactor (or a future polled without one) cannot do async I/O or sleep.

### Work With Pin Deliberately Where Self-Referential Futures Are Involved

`Pin` ensures a value will not be moved in memory, which is required because self-referential futures (generated by the compiler for async blocks that borrow across awaits) would be unsafe if moved. Most code does not construct Pin directly, but understanding it matters for unsafe code, for storing futures, and for some APIs.

- **Let the runtime pin futures; do not unpin them.** Spawning a future pins it; do not move a pinned future out of its pinning unless you understand the safety implications.
- **Use `Box::pin` or stack pinning deliberately** when you need to store or pass a future whose size or lifetime requires it.
- **Avoid self-referential structs in safe code.** Self-referential data is the reason Pin exists; do not create it manually unless you are writing unsafe code and understand the invariants.

### Document the Basis and the Reasoning

Every conclusion should be traceable to its evidence, assumptions, and alternatives considered. Record not only the outcome but the reasoning path: what was checked, what was ruled out, what uncertainty remains, and what would change the conclusion. Documentation that captures the basis allows another practitioner to review, reproduce, or challenge the work, and it prevents confident conclusions from becoming unrepeatable assertions. A decision made without a recorded basis cannot be audited, improved, or safely handed off.

## Common Traps

### Blocking The Runtime With Synchronous Work

A blocking syscall, a long CPU-bound computation, or a blocking library call inside an async task, freezing the worker thread and starving every task on it. Offload to spawn_blocking or a dedicated thread; use async-native libraries.

### Holding A Non-Send Guard Across An Await

An `Rc`, a `RefCell` borrow, or a non-Send lock held across an await, making the future non-Send and unspawnable on a multi-threaded runtime. Keep held-across-await state minimal and Send; acquire, operate, release before awaiting.

### Holding A Lock Across An Await (Deadlock Or Corruption)

A lock acquired, then awaited before release, deadlocking if the same task re-enters or corrupting state if cancelled before release. Acquire, do synchronous work, release, then await; do not hold locks across await.

### Cancellation-Unsafe Future

A future that leaves inconsistent state when dropped at an await point (a lock acquired but not released, a partial write), producing corruption that appears only under cancellation (timeouts, select!). Make futures cancellation-safe or make cancellation explicit; test cancellation.

### `select!` Or Timeout Cancelling An Unsafe Branch

A `select!` or timeout that drops a branch whose future is not cancellation-safe, corrupting state when that branch loses. Ensure every branch is cancellation-safe.

### Nested Or Missing Runtime

Starting a runtime inside another runtime's task (deadlock), or polling a future without a runtime reactor (async I/O and timers fail). Do not nest runtimes; ensure a reactor is present for async I/O and sleep.

### Misconfigured Pool Sizes

Worker or blocking pools sized wrong for the workload — too few workers bottleneck async tasks, too few blocking threads bottleneck spawn_blocking work. Size the two pools independently for the workload mix.

## Self-Check

- [ ] No blocking work runs on the async runtime: blocking I/O and syscalls are offloaded to spawn_blocking or a dedicated thread, CPU-bound work is offloaded or chunked with yields, blocking libraries are replaced with async-native alternatives or wrapped in spawn_blocking, and the blocking pool is sized for the workload.
- [ ] What crosses await points is deliberate: everything held across an await is Send (no Rc, RefCell borrows, or non-Send locks), locks are not held across await (acquire, operate, release, then await), and the held-across-await surface is minimal and audited.
- [ ] Futures are cancellation-safe: dropping a future at any await point leaves state consistent (locks released or never acquired across the await, partial writes recoverable), `select!` and timeouts are used only with cancellation-safe branches, cancellation is made explicit where safety is hard, and cancellation is tested at different await points.
- [ ] The runtime is chosen and configured for the workload: multi-threaded runtimes require Send futures (single-threaded allow non-Send types but sacrifice parallelism), worker and blocking pools are sized independently, runtimes are not nested, and the reactor is present for async I/O and timers.
- [ ] Pin is handled deliberately: spawned futures are pinned and not moved out, Box::pin or stack pinning is used when storing/passing futures that require it, and self-referential structs are not created manually in safe code.
- [ ] The highest-risk cases were verified — a blocking call moved off the runtime, a non-Send guard caught or removed, a lock held across await refactored to release before await, and a future cancelled at a sensitive await point without corruption — not only the clean sequentially-looking path.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
