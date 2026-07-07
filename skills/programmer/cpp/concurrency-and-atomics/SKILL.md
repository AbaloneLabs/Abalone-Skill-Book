---
name: cpp_concurrency_and_atomics.md
description: Use when the agent is writing or reviewing C++ concurrent code using std::thread, std::async, std::mutex/lock_guard/unique_lock/shared_mutex, std::atomic and memory_order, condition variables, futures and promises, std::jthread and cooperative cancellation, thread pools, lock-free data structures, or diagnosing data races, deadlocks, lost wakeups, false sharing, and memory-ordering bugs on weak architectures.
---

# C++ Concurrency And Atomics

C++ gained a memory model and a threading standard library in C++11 and has expanded it since (shared_mutex in C++17, jthread and atomic refinements in C++20). Unlike C, the C++ memory model is defined in terms of the language itself, and `std::atomic` gives portable, well-defined atomic operations with explicit memory ordering. But the library gives you the building blocks; it does not make concurrency correct. A `std::mutex` that is never locked protects nothing, a `std::atomic` with the wrong memory order is a data race, a future whose shared state is never waited on loses an exception, and a `std::async` return that is discarded can block unexpectedly. The judgment problem is that C++ concurrency is a set of sharp tools whose correct use requires understanding the memory model, the lifetime of shared state, and the synchronization that each primitive does and does not provide.

Agents tend to add a mutex and assume correctness, or reach for `std::atomic` without reasoning about memory ordering, or spawn a `std::thread` that captures references to locals that go out of scope. The judgment problem is to match the synchronization primitive to the actual sharing pattern, to understand that atomics have memory-ordering semantics that must be chosen deliberately, and to manage object lifetimes across threads where no language feature helps. This skill is about making C++ concurrency choices deliberately, with the memory model and lifetime in mind, rather than reaching for the first primitive that compiles.

## Core Rules

### Match The Synchronization Primitive To The Sharing Pattern

Different sharing patterns need different primitives. Choosing the wrong one is either incorrect (no synchronization where needed) or needlessly slow (a mutex where an atomic would do).

- **Single flag or counter read/written by multiple threads**: `std::atomic<bool>` / `std::atomic<int>` with appropriate memory order. No mutex needed.
- **Multi-field invariant that must hold across operations**: `std::mutex` (or `std::shared_mutex` if reads dominate and writers are rare) protecting the whole group of fields. Every access — read or write — goes through the lock.
- **Producer-consumer handoff**: a queue (thread-safe, or `std::mutex`+`std::condition_variable`), or a lock-free queue if profiling shows the lock is the bottleneck.
- **One-shot result from a worker**: `std::future`/`std::promise`, or `std::async`. The future is how the caller waits for and receives the result or exception.

Do not protect a single counter with a mutex (use atomic), and do not protect a multi-field invariant with a single atomic (it does not make the fields consistent with each other).

### Choose Memory Order Deliberately, Defaulting To seq_cst

`std::atomic` operations default to `memory_order_seq_cst`, the strongest and safest ordering. Weakening it (`acquire`, `release`, `relaxed`) is an optimization that requires you to prove the happens-before relationships are still correct. The C++ memory model defines a data race as concurrent non-atomic access to the same memory, or atomic access with insufficient synchronization; a data race is undefined behavior.

- `relaxed`: atomicity only, no ordering. Safe for independent counters where nothing else depends on the count value.
- `acquire` (load) / `release` (store): the publish/consume pair. A store-release of a pointer publishes the pointed-to data; a load-acquire by another thread that sees the pointer is guaranteed to see the data written before the release. This is the core of lock-free message passing.
- `seq_cst`: a single total order of all seq_cst operations. The safe default.

Default to `seq_cst`. Only weaken when you can articulate the happens-before argument and have run ThreadSanitizer. Premature weakening is a common source of bugs that pass on x86 (which is strongly ordered) and fail on ARM/POWER.

### Never Let A Thread Outlive The Data It References

`std::thread` (and the callable passed to `std::async`) captures arguments and references. If the thread accesses a reference to a local variable that goes out of scope, that is a use-after-free. Detaching a thread that references stack data is a bug by construction. The rule: a thread must either be joined before the data it uses is destroyed, or it must own (copy or smart-pointer) all the data it needs.

`std::jthread` (C++20) helps: it joins on destruction, and supports cooperative cancellation via `stop_token`. Prefer `jthread` over `thread` in C++20+; if using `thread`, ensure every path (including exceptions) joins or detaches deliberately, because a `std::thread` that is destroyed while joinable calls `std::terminate`.

### Use RAII For Locks; Never Unlock Manually

`std::lock_guard<Mutex>` acquires in its constructor and releases in its destructor, so the lock is released on every exit path including exceptions. `std::unique_lock` adds the ability to lock/unlock/defer and to transfer ownership, needed for condition variables. `std::scoped_lock` (C++17) locks multiple mutexes deadlock-free using a deadlock-avoidance algorithm — use it instead of nesting `lock_guard`s when you need multiple locks at once.

Never call `mtx.lock()` and `mtx.unlock()` manually; the manual path leaks the lock if an exception or early return intervenes. Always wrap in an RAII type. For read-heavy shared state, `std::shared_mutex` with `std::shared_lock` for readers and `std::unique_lock` for writers allows concurrent reads.

### Always Loop Around condition_variable::wait

`std::condition_variable::wait` (and `wait_for`/`wait_until`) can return spuriously — the standard explicitly permits spurious wakeups. The wait must be inside a loop that re-checks the predicate, or use the three-argument form `wait(lock, pred)` that does the loop for you. Never write `if (pred) cv.wait(lock);`.

Signal/broadcast (`notify_one`/`notify_all`) should be called after changing the shared state the predicate depends on. It does not matter whether the mutex is held when notifying (holding it is slightly less efficient but never wrong); what matters is that the state change happens-before a waiter can observe a wakeup. The predicate must be checked under the mutex.

### Handle Futures And Exceptions Explicitly

A `std::future` from `std::async` or a `std::promise` carries either a value or an exception. If the worker throws, the exception is stored and rethrown when `future::get` is called. A future whose `get` is never called loses the exception silently (it is destructed with an unhandled exception state). Always call `get` on a future, exactly once.

Beware `std::async` without an explicit launch policy: with the default `async::deferred | async::async`, the implementation may defer execution until `get` is called, which means the work may never run if `get` is not called, and may run synchronously on the calling thread when it is. Pass `std::launch::async` if you need guaranteed asynchronous execution. For a real thread pool, prefer a dedicated library (Boost.Asio, TBB, or a hand-rolled pool) over `std::async`, which is widely considered underspecified for production use.

### Avoid False Sharing In Performance-Sensitive Concurrent Code

When multiple threads write to different variables that happen to share a cache line (typically within 64 bytes), the cache coherency protocol serializes them even though no logical sharing exists — this is false sharing, and it can make "parallel" code slower than serial. For per-thread counters or scratch data accessed by different threads, pad or align so each thread's data is on its own cache line (`alignas(64)`, or a padding struct).

This is a performance issue, not a correctness issue, but it is a common reason that naive parallelization underperforms. Profile before and after.

## Common Traps

### Protecting A Multi-Field Invariant With A Single Atomic

`atomic<int> size; atomic<int> capacity;` does not make size and capacity consistent with each other; a reader can see an updated size with a stale capacity. Protect related fields with a mutex, or pack them into one atomic.

### Weakening Memory Order Without A Happens-Before Argument

`memory_order_relaxed` for a flag that publishes data is a data race: the reader may see the flag set but the data not yet visible. Use release/acquire for publish/consume.

### Detaching A Thread That References Stack Data

`std::thread t([&]{ use(local); }); t.detach();` — `local` is destroyed when the function returns, but the thread may still access it. Join, or copy/own the data.

### Destroying A joinable std::thread

A `std::thread` destroyed while joinable calls `std::terminate`. Always join, detach deliberately, or use `jthread`. This includes the exception path — wrap thread creation so the join is guaranteed.

### Manual lock/unlock Leaking On An Exception

`mtx.lock(); ...; mtx.unlock();` leaks the lock if the middle throws. Use `lock_guard`/`unique_lock`.

### if Around condition_variable::wait

`if (ready) cv.wait(lock);` misses spurious wakeups and the predicate-changed-before-wait race. Use `while (!ready) cv.wait(lock);` or `cv.wait(lock, [&]{ return ready; })`.

### Discarding A Future And Losing The Exception

`std::async(std::launch::async, work);` (return value discarded) — if `work` throws, the exception is lost or rethrown at destruction. Always capture the future and call `get`.

### Nested Lock Acquisition Deadlock

Locking mutex A then B in one thread and B then A in another deadlocks. Use `std::scoped_lock(a, b)` which acquires multiple locks with deadlock avoidance, or establish a global lock order.

### False Sharing Slowing "Parallel" Code

Per-thread counters on the same cache line serialize through cache coherency. Pad per-thread data to cache-line boundaries.

## Self-Check

- [ ] The synchronization primitive matches the sharing pattern: atomics for single flags/counters, mutexes (or shared_mutex) for multi-field invariants, futures for one-shot results, queues for producer-consumer.
- [ ] Memory ordering is `seq_cst` by default and weakened to acquire/release/relaxed only with a reasoned happens-before argument and ThreadSanitizer validation.
- [ ] No thread outlives the data it references; threads are joined before shared data is destroyed, or they own (copy/smart-pointer) their data, and `jthread` is preferred in C++20+.
- [ ] All locks are acquired through RAII types (`lock_guard`, `unique_lock`, `scoped_lock`, `shared_lock`); no manual `lock`/`unlock`.
- [ ] Every `condition_variable::wait` is inside a predicate loop (or uses the three-argument form), and notify is called after the predicate's state is changed.
- [ ] Every future has `get` called exactly once so exceptions are not lost, and `std::async` uses an explicit launch policy when asynchronous execution is required.
- [ ] Multiple-mutex acquisition uses `scoped_lock` or a documented global lock order to avoid deadlock.
- [ ] Performance-sensitive per-thread data is cache-line-aligned to avoid false sharing, verified by profiling.
