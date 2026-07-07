---
name: ruby_concurrency_and_threads.md
description: Use when the agent is writing concurrent Ruby using Threads, Ractors, Mutex and Queue, Fiber and Async, Sidekiq or ActiveJob workers, GVL (GIL) implications, thread-safe collections and shared state, condition variables and monitors, or is diagnosing race conditions, deadlocks, "it works single-threaded but fails under load", GVL contention, thread leakage, or shared mutable state corruption in Ruby and Rails applications. Covers the GVL concurrency model, the choice between threads/Ractors/fibers/jobs, synchronization primitives, thread-safety of shared state and gems, and the pitfalls of assuming atomicity.
---

# Concurrency And Threads In Ruby

Ruby's concurrency story is defined by the Global VM Lock (GVL, formerly GIL) in MRI, and misunderstanding it is the source of most Ruby concurrency bugs. The GVL means only one thread executes Ruby code at a time within a process, which gives many agents the false belief that Ruby code is automatically thread-safe. It is not. The GVL is released during I/O and during C extensions, so two threads can interleave at any method boundary, any I/O wait, any native call — and shared mutable state mutated from multiple threads races exactly as it would without a GVL. The classic failure is a check-then-act sequence (`return if cache[key]; cache[key] = compute`) that two threads execute concurrently: both see the missing key, both compute, one overwrites — a race the GVL does not prevent because Ruby releases the lock between the read and the write. The judgment problem is to know what the GVL does and does not protect, to identify shared mutable state, to protect it with the right primitive (`Mutex`, `Queue`, `Monitor`, or a concurrency gem), and to choose the right concurrency unit (thread for I/O concurrency, Ractor for true parallelism, Fiber/Async for cooperative I/O, background job for durable work).

Agents reach for `Thread.new` for "make it faster" without identifying the shared state, or they wrap a whole method in a `Mutex` and create a serialized bottleneck, or they assume a gem is thread-safe when it is not. The remedy is to minimize shared mutable state (prefer message passing via `Queue`, prefer immutability, prefer per-thread state), to protect what is shared with the narrowest correct lock, and to verify under concurrent load rather than under a single request.

## Core Rules

### Understand What The GVL Protects And What It Does Not

The MRI GVL ensures only one thread runs Ruby bytecode at a time, so pure-CPU Ruby work does not parallelize across threads. But the GVL is released during I/O (network, file, sleep) and during many C-extension calls, so threads interleave at those points. The consequence: any operation that is not a single atomic Ruby primitive can be interrupted between its internal steps, and shared mutable state races. The GVL makes individual built-in operations (`Hash#[]=`, `Array#<<`) atomic in the sense that they won't be torn apart, but it does not make a *sequence* of operations atomic.

- A read-modify-write (`h[k] = h[k] + 1`) is three operations and races; the GVL does not make it atomic.
- I/O concurrency works: multiple threads blocked on I/O release the GVL, so threads speed up I/O-bound work even under the GVL.
- For true CPU parallelism in MRI, use Ractors (no shared mutable state) or multiple processes; JRuby/TruffleRuby have no GVL and parallelize threads natively.

### Identify Shared Mutable State And Minimize It

The root of concurrency bugs is shared mutable state: a class attribute, a memoized class variable (`@@cache`), a constant holding a mutable collection, a global, a Rails initializer-set service object with internal state. The first design move is to eliminate it: make state per-request, per-thread, or immutable; pass values explicitly instead of stashing them in shared places. What remains shared and mutable must be protected.

- Prefer immutability (frozen objects, value objects) for anything shared across threads.
- Prefer message passing (`Queue`, `SizedQueue`) over shared memory: threads communicate by handing off values, not by mutating a shared structure.
- Per-thread state (`Thread.current[:foo]`) avoids sharing but is easy to leak; use it sparingly with cleanup.

### Protect Shared State With The Right Primitive

When shared mutable state is unavoidable, protect every access with a synchronization primitive, chosen by the operation:

- `Mutex` for mutual exclusion around a critical section (`mutex.synchronize { ... }`). Keep the section short; a long-held lock serializes threads and can deadlock.
- `Queue`/`SizedQueue` for producer-consumer handoff (thread-safe by construction, no explicit lock).
- `Monitor`/`MonitorMixin` for a re-entrant lock (a thread can acquire the same monitor twice without deadlocking), useful when methods that lock call each other.
- `ConditionVariable` to wait for/sign a state change while holding a mutex.
- A concurrency gem (`concurrent-ruby`) for thread-safe atomics, maps, and executors; do not hand-roll what it provides.

Lock all reads and all writes of the shared state with the same mutex; an unlocked read races with a locked write.

### Avoid Deadlocks And Lock Convoys

A deadlock occurs when two threads each hold a lock the other needs, or when a thread waits on a condition that will never be signaled. Avoid acquiring multiple locks when possible; when you must, acquire them in a consistent global order. Never hold a lock across an external call (network, callback) that might block or call back into the same lock. Keep critical sections short so threads do not queue (a "lock convoy" that serializes throughput). Use a timeout on waits (`ConditionVariable.wait(mutex, timeout)`) where a missed signal would otherwise hang forever.

### Choose The Right Concurrency Unit For The Work

Different concurrency units fit different work:

- **Thread** — I/O-bound concurrency within a process (a web server handling requests). The GVL is fine because the work waits on I/O.
- **Ractor** — true CPU parallelism in MRI, with the constraint that Ractors share no mutable state (communicate by message passing with copy/move semantics). Use for CPU-heavy parallel computation; the isolation is the safety.
- **Fiber / Async** — cooperative concurrency within a single thread; cheap, no GVL contention, but requires non-blocking I/O and explicit yielding. Good for many concurrent I/O-bound operations (thousands of connections).
- **Background job (Sidekiq/ActiveJob)** — durable, retryable, out-of-process work. Use for work that must survive a request crash, run on a schedule, or be retried; not for in-request parallelism.

Match the unit to the requirement: in-request I/O parallelism (thread/async), CPU parallelism (ractor/process), durability (job).

### Verify Thread-Safety Of Gems And Frameworks

A gem or framework is not thread-safe by default; many are not. Before using a library across threads, verify its thread-safety claim. Rails is thread-safe by default (since Rails 4) but only if application code is: class-level mutable state (`@@cache`, class attributes mutated at runtime) is not. Connection pools, memoization, and shared service objects need care. A gem that uses class variables, global state, or non-reentrant internal caches may corrupt under concurrency. Check the gem's docs and, when in doubt, isolate it behind a mutex or a single-threaded facade.

## Common Traps

### Check-Then-Act Race Under The GVL

`return if @cached; @cached = compute` races: two threads both see `nil`, both compute. The GVL releases between the check and the act. Protect with a mutex or use `Concurrent::Map#compute_if_absent`.

### Assuming Hash Or Array Operations Are Atomic Across A Sequence

`h[k] = h[k] + 1` is read, add, write — three operations, races. Wrap in `mutex.synchronize` or use an atomic.

### Long-Held Mutex Serializing Throughput

A mutex held across a network call or a large computation makes all threads wait, defeating concurrency. Keep critical sections to pure memory operations.

### Deadlock From Nested Or Out-Of-Order Locks

Two locks acquired in opposite orders by two threads deadlocks. Acquire in a global order; avoid nested locks; never hold a lock across a callback that may re-enter.

### Class-Level Mutable State In A Threaded Server

`@@cache` or a class attribute mutated per request races across request threads. Move state to instances, the database, or a thread-safe structure.

### Thread Leaked Past Its Work

`Thread.new { loop { ... } }` with no stop condition runs forever, leaking. Track threads and ensure they terminate; prefer structured concurrency or a job queue.

### Gem Assumed Thread-Safe

A library with internal mutable state corrupts under concurrent use. Verify or wrap behind a mutex.

### Ractor Sharing Mutable State

Ractors reject sharing of mutable objects; attempting it raises. Design Ractor code around copy/move message passing, not shared references.

## Self-Check

- [ ] The GVL's scope is understood: I/O and C-extension calls release it, sequences of operations are not atomic, and CPU parallelism requires Ractors or multiple processes.
- [ ] Shared mutable state has been identified and minimized (immutability, per-request/per-thread state, message passing via `Queue`); what remains shared is protected.
- [ ] Every read and write of shared mutable state is protected by the same mutex (or a thread-safe structure/atomic), critical sections are short, and no lock is held across external calls.
- [ ] Locks are acquired in a consistent order, nested locks are avoided, waits use timeouts where a missed signal would hang, and no deadlock is reachable.
- [ ] The concurrency unit matches the work: threads for I/O concurrency, Ractors/processes for CPU parallelism, fibers/async for cooperative I/O, jobs for durable/retryable work.
- [ ] Gems and frameworks used across threads have been verified thread-safe, and application class-level mutable state has been eliminated or protected.
- [ ] No check-then-act sequence on shared state is unprotected, and no `@@`-variable or class attribute is mutated concurrently.
- [ ] The design has been considered under concurrent load, thread/ractor lifecycle, and failure mid-operation, and remains correct, leak-free, and free of deadlock.
