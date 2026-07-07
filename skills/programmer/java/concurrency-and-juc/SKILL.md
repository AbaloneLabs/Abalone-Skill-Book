---
name: concurrency_and_juc.md
description: Use when the agent is writing or reviewing Java concurrency using java.util.concurrent — threads, ExecutorService/thread pools, CompletableFuture, Lock/ReadWriteLock/StampedLock, atomic variables, ConcurrentHashMap, BlockingQueue, CountDownLatch/Phaser/Semaphore, virtual threads, or diagnosing deadlocks, thread pool starvation, CompletableFuture lost exceptions, visibility bugs, blocking-while-holding-a-lock, or pool sizing problems. Covers the JMM, happens-before, thread pool sizing, structured concurrency, the hazards of unbounded queues and shared mutable state, and choosing between synchronized, Lock, atomics, and j.u.c. collections.
---

# Concurrency And JUC

Java's `java.util.concurrent` (j.u.c.) package, added in Java 5 and expanded since, is one of the richest concurrency libraries in any language: thread pools, futures, locks, atomics, concurrent collections, synchronizers, and now virtual threads. Its breadth is also its hazard — there are usually several plausible tools for a job, they have subtly different semantics, and the wrong choice compiles, passes a smoke test, and deadlocks or starves under load. A `newFixedThreadPool` with an unbounded queue silently serializes everything behind one slow task; a `CompletableFuture` chain swallows an exception unless someone calls `join()`; a `synchronized` block held across a blocking call freezes every waiter behind it; an `AtomicInteger` makes one field safe while a compound operation across two atomics is still a race.

The judgment problem is not "which class do I import" but "what invariant must survive every interleaving, what blocks, what is the lifecycle of the work, and what happens under failure and shutdown." Agents tend to reach for `synchronized` out of familiarity (correct but often too coarse), or for raw `Thread`/`ExecutorService` without bounding (correct but leaky), or for `CompletableFuture` chains that look declarative and silently drop exceptions. The cost ranges from a deadlock that takes down a service to a thread leak that slowly exhausts a pool to an exception that vanishes and leaves a request hanging forever.

## Core Rules

### Bound And Lifecycle Every Thread Pool; Never Use The Unbounded Defaults Blindly

An `ExecutorService` is a long-lived resource with a lifecycle: it must be shut down explicitly, and its work must be bounded. The defaults are dangerous: `Executors.newFixedThreadPool` and `newSingleThreadExecutor` use an unbounded `LinkedBlockingQueue`, so submitted tasks pile up without limit until the heap exhausts; `newCachedThreadPool` creates threads without limit, so a burst of tasks creates a burst of threads until the OS refuses. Both are fine for bounded, trusted work and catastrophic for unbounded request handling.

Choose pool sizing deliberately. For CPU-bound work, size to the processor count (N or N+1 threads); for I/O-bound work, size to the expected concurrency (which may be much higher) or use virtual threads. Set an explicit bounded queue and a rejection policy (`CallerRunsPolicy` for back-pressure, `AbortPolicy` to surface overload, `DiscardPolicy` only when dropping is acceptable). Always shut down pools in a lifecycle hook (`shutdown()` then `awaitTermination` with a timeout, then `shutdownNow`), and never let a pool be created per-request (it leaks threads). A pool whose queue or thread count is unbounded under any input is a defect.

### Establish Happens-Before Through j.u.c., Not Through Hope

The Java Memory Model applies to j.u.c. just as it does to `synchronized` and `volatile`: every shared write needs a happens-before edge to every read, or the read may observe a stale or partially-constructed value. The j.u.c. primitives each establish documented happens-before guarantees: actions in a thread before submitting a task happen-before the task's execution; the task's actions happen-before the result is retrieved from the `Future`; writes before `CountDownLatch.countDown` happen-before the corresponding `await` returns; writes before putting into a `BlockingQueue` happen-before the corresponding `take`; writes before `Semaphore.release` happen-before the corresponding `acquire`.

The rule: do not assume visibility. If thread A prepares data and hands it to thread B via a `BlockingQueue`, the queue establishes happens-before and B sees A's writes — but only if B reads through the queue. If A also stashes the data in a plain field that B reads directly, there is no happens-before for that read and B may see stale values. Route all cross-thread data through a happens-before-establishing mechanism, and never read shared state through a plain field that was written by another thread without synchronization.

### Match The Synchronization Mechanism To The Invariant

Java offers a spectrum of synchronization, and the choice is a design decision:

- **`synchronized` (intrinsic locks).** Reentrant, simple, good for compound operations on a moderate amount of state. The cost is serialization and the risk of holding the lock across blocking code. Use for short critical sections over cohesive state.
- **`Lock` / `ReadWriteLock` / `StampedLock`.** Explicit, support tryLock/timeout/fairness, and `ReadWriteLock` allows concurrent readers. Use when you need timed or interruptible lock acquisition, read-heavy access, or a non-reentrant lock. `StampedLock` is faster for read-heavy cases but has pitfalls (validation, non-reentrancy).
- **`Atomic*` (`AtomicInteger`, `AtomicReference`, `LongAdder`).** Best for a single field or a small set with independent updates. `LongAdder` over `AtomicLong` under high contention. Atomics do not make compound operations atomic — a read-modify-write across two atomics is still a race.
- **Concurrent collections (`ConcurrentHashMap`, `CopyOnWriteArrayList`, `BlockingQueue`).** Best for shared data structures; they handle their own synchronization and document their consistency level. `ConcurrentHashMap` is weakly consistent (iterators reflect some state at iteration time) and does not lock the whole map.

Match the mechanism to the invariant: a single counter is an `AtomicInteger` or `LongAdder`; a compound transfer is a `synchronized` block or `Lock` over both accounts; a read-heavy registry is a `ReadWriteLock` or `ConcurrentHashMap`. The weak choice is `synchronized` on a whole class "to be safe" — it serializes unrelated work and often fails to protect the real compound operation.

### Never Hold A Lock Across A Blocking Call

Holding a `synchronized` block, a `Lock`, or any monitor across a blocking operation — a network call, a database query, a `Future.get`, another lock acquisition, a `Thread.sleep` — freezes every other thread waiting for that lock behind the block. This is the single most common cause of throughput collapse and deadlock in Java servers. The lock is a serialization point; holding it across I/O serializes all the waiters behind that I/O.

The discipline: critical sections contain only pure memory operations (read fields, compute, write fields, return). Do the blocking work outside the lock: read a snapshot under the lock, release, do the I/O, reacquire the lock to apply the result. If you must coordinate blocking work, use a concurrent collection or a message-passing pattern (`BlockingQueue`) instead of a held lock. The same rule applies to `CompletableFuture` chains that block inside a callback running on a shared pool — see below.

### Make CompletableFuture Not Silently Swallow Exceptions

`CompletableFuture` is a powerful composition tool, but its default behavior hides failures: if no one ever calls `join()`, `get()`, or attaches an `exceptionally`/`handle`/`whenComplete` stage, an exceptional completion is silently dropped — the request just never returns, with no log, no error, no trace. This is the CompletableFuture equivalent of swallowing an exception, and it is extremely common in code that chains futures and forgets the terminal.

Defend with three habits. First, always terminate a chain: every `CompletableFuture` must reach a terminal that observes both success and failure (`join`, `get`, or an `exceptionally`/`handle` stage that logs). Second, use a timeout (`orTimeout`, `completeOnTimeout`) so a stuck chain cannot hang forever — an uncompleted future with no timeout is a resource leak. Third, run blocking work on a dedicated executor, not the common pool: `supplyAsync(supplier, executor)`; the common `ForkJoinPool` is shared and small, and blocking on it starves every other `CompletableFuture` in the process. A `CompletableFuture` chain that blocks on the common pool, has no timeout, and has no terminal is a three-way recipe for a hung service.

### Use Concurrent Collections Instead Of Synchronized Wrappers For Shared State

`Collections.synchronizedList` / `synchronizedMap` wrap a collection and synchronize every method. This is correct but coarse: every operation takes the lock, iterators must be manually synchronized externally, and compound operations (`putIfAbsent`, check-then-act) are not atomic unless you hold the wrapper's lock. `ConcurrentHashMap`, `CopyOnWriteArrayList`, and `ConcurrentLinkedQueue` are designed for concurrency: fine-grained locking (or copy-on-write), atomic compound operations (`compute`, `computeIfAbsent`, `merge`, `putIfAbsent`), and documented iterator consistency.

Prefer the concurrent collections for shared mutable state. Use `ConcurrentHashMap.computeIfAbsent` instead of `synchronized (map) { if (!map.containsKey(k)) map.put(k, v); }` — the latter is a race even with the synchronized wrapper unless you hold the lock across both calls. Use `CopyOnWriteArrayList` for read-heavy, write-rare listener lists. The synchronized wrappers are fine for low-contention or compatibility cases, but in concurrent code the dedicated collections are both faster and safer.

### Treat Virtual Threads As A Tool For Blocking I/O, Not For CPU Work

Virtual threads (Project Loom, Java 21+) are lightweight threads that the JVM parks cheaply during blocking I/O, allowing you to write blocking, sequential code at massive scale without thread-pool exhaustion. They shine for request handling where each request does I/O (database, HTTP, file): a virtual thread per request, blocking freely, scales to millions because parked virtual threads cost almost nothing. They do not help — and can hurt — for CPU-bound work, because a virtual thread running CPU code occupies a platform (carrier) thread just like a platform thread does.

The hazards: virtual threads are pinned (cannot unmount) when running `synchronized` blocks or native/FJNI code, so frequent `synchronized` under load defeats the model — use `ReentrantLock` instead of `synchronized` in code that will run on virtual threads. Virtual threads are cheap to create but not free; reuse them through an executor where it makes sense, and avoid one-virtual-thread-per-tiny-task patterns that thrash. And virtual threads do not make blocking free: they make it scalable, but the underlying I/O still takes the time it takes. Use virtual threads for I/O-bound concurrency at scale; keep CPU-bound work on a sized platform-thread pool.

### Structure Concurrency So Cancellation And Failure Propagate

In unstructured concurrency (fire-and-forget threads, leaked `ExecutorService`), a cancelled parent leaves children running, a failed child never notifies the parent, and shutdown races with in-flight work. Structured concurrency (Java 21's preview `StructuredTaskScope`, or the discipline of `ExecutorService` + `awaitTermination` + `shutdownNow`) makes the parent-child relationship explicit: children are scoped to a block, the block does not return until children finish, and cancellation propagates downward. This makes leaks and orphaned work far harder to write.

Even without the preview API, apply the discipline: every thread or task has an owner that waits for it or cancels it; a request's work completes (success or failure) before the request returns; shutdown cancels and waits. A `Thread` started with no owner, an `ExecutorService` never shut down, or a `CompletableFuture` with no terminal are all unstructured and leaky. The cost of unstructured concurrency is bugs that only appear under cancellation, timeout, or shutdown — exactly the conditions tests rarely exercise.

## Common Traps

### Unbounded Pool Queues Or Thread Counts

`Executors.newFixedThreadPool` (unbounded queue) or `newCachedThreadPool` (unbounded threads) under request load → OOM or thread explosion. Set explicit bounds and a rejection policy.

### Holding A Lock Across I/O

`synchronized` block containing a database call or HTTP request, serializing every waiter behind the I/O. Keep critical sections to pure memory operations.

### CompletableFuture With No Terminal Or Timeout

A chain with no `join`/`get`/`exceptionally` silently drops exceptions; a chain with no `orTimeout` hangs forever. Always terminate and timeout.

### Blocking On The Common ForkJoinPool

`supplyAsync` without an executor runs on the shared common pool; blocking there starves every other `CompletableFuture`. Use a dedicated executor for blocking work.

### Assuming `Atomic*` Makes Compound Operations Safe

Two `AtomicInteger`s updated in sequence are not atomic together; a reader can see them inconsistent. Compound state needs a lock or a single atomic holder.

### Synchronized-Wrapper Iterators Without External Locking

Iterating `Collections.synchronizedList` without `synchronized (list) { ... }` around the iteration throws `ConcurrentModificationException` or races. Use `CopyOnWriteArrayList` or `ConcurrentHashMap`.

### `synchronized` Inside Virtual Threads Under Load

`synchronized` blocks pin the carrier thread, defeating virtual-thread scalability. Use `ReentrantLock` in code that runs on virtual threads.

### Fire-And-Forget Threads Or Pools Never Shut Down

A `Thread` or `ExecutorService` with no owner or shutdown, leaking on every request or redeploy. Every concurrent unit needs a lifecycle owner.

### Pool Sized For CPU When The Work Is I/O-Bound

A CPU-count-sized pool handling I/O-bound requests starves under modest concurrency. Size pools to the work profile, or use virtual threads for I/O.

## Self-Check

- [ ] Every `ExecutorService` has an explicit bounded queue and thread count (or a documented reason for unbounded), a rejection policy chosen for the desired back-pressure, and a shutdown path (`shutdown` + `awaitTermination` + `shutdownNow`) in a lifecycle hook.
- [ ] Pool sizing matches the work profile (CPU-count for CPU-bound, higher or virtual threads for I/O-bound), and no pool is created per-request.
- [ ] Every cross-thread shared write has a documented happens-before edge to every read (via a j.u.c. primitive, `Lock`, `volatile`, `Atomic*`, or thread start/join); no shared state is read through a plain field written by another thread.
- [ ] The synchronization mechanism matches the invariant: `Atomic*`/`LongAdder` for single fields, `synchronized`/`Lock` for compound state, `ReadWriteLock`/`ConcurrentHashMap` for read-heavy shared structures; no `synchronized`-on-whole-class over-protection.
- [ ] No lock (`synchronized`, `Lock`, monitor) is held across a blocking call (I/O, `Future.get`, `Thread.sleep`, another lock); critical sections contain only memory operations, with snapshots taken under the lock and I/O done outside.
- [ ] Every `CompletableFuture` chain reaches a terminal that observes success and failure, has a timeout (`orTimeout`/`completeOnTimeout`), and runs blocking work on a dedicated executor rather than the common pool.
- [ ] Shared mutable collections use `ConcurrentHashMap`/`CopyOnWriteArrayList`/`ConcurrentLinkedQueue` with atomic compound operations, not `synchronized` wrappers with check-then-act races or unsynchronized iteration.
- [ ] Virtual threads are used for blocking-I/O concurrency at scale (not CPU-bound work), `synchronized` is avoided in virtual-thread code in favor of `ReentrantLock`, and virtual threads are not created in thrashing one-per-tiny-task patterns.
- [ ] Concurrency is structured: every thread/task/pool has an owner that waits or cancels it, request work completes before the request returns, and shutdown cancels and waits for in-flight work.
- [ ] The design was exercised under cancellation, timeout, and shutdown (not just the happy path), since those are where unstructured concurrency leaks and orphans surface.
