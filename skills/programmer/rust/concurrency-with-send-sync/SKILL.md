---
name: rust_concurrency_with_send_and_sync.md
description: Use when the agent is writing or reviewing Rust concurrent code using threads, Arc, Mutex, RwLock, channels (mpsc/crossbeam), rayon, or thread pools, reasoning about Send and Sync marker traits, designing lock order to avoid deadlocks, sharing state across threads, choosing message passing vs shared memory, handling poisoning, or debugging "cannot be sent between threads safely" errors. Covers what Send and Sync actually prove and do not prove, lock granularity and ordering, channel backpressure and capacity, structured concurrency with scopes, and the boundary between the compiler's thread-safety proof and real race-free logic.
---

# Concurrency With Send And Sync

Rust's concurrency safety rests on two marker traits, `Send` and `Sync`, that the compiler uses to prove data can cross thread boundaries soundly. This is what lets Rust catch data races at compile time — most of them. The judgment problem is understanding exactly what `Send` and `Sync` prove (and what they do not), choosing the right sharing mechanism (locks, channels, atomics), and avoiding the deadlocks, priority inversions, and logic races that the compiler cannot detect.

Agents tend to wrap everything in `Arc<Mutex<_>>` reflexively, hold locks across `.await` or nested calls, assume `Send + Sync` means the logic is race-free, use unbounded channels that grow without limit, or pick shared-memory where message passing would express the design better. The harm appears as deadlocks that hang services, lock contention that serializes throughput to a single thread, channels that exhaust memory under load, and logic races that compile fine but corrupt state. The real work is matching the sharing mechanism to the access pattern, keeping critical sections minimal, and treating the compiler's proof as a floor, not a ceiling.

## Core Rules

### Understand What Send And Sync Prove — And What They Do Not

`Send` and `Sync` are the foundation; misreading them is the root of most concurrency confusion.

- **`Send`**: a type is safe to *move* ownership of to another thread. Most types are `Send`; the exceptions are types with thread affinity (`Rc`, `RefCell`, raw pointers, some OS handles).
- **`Sync`**: a type is safe to share a `&T` reference to across threads — meaning `&T` can be read from multiple threads without data races. A type is `Sync` if `&T` is `Send`. `Mutex<T>` is `Sync` even when `T` is not, because the lock serializes access.
- **What they prove**: the compiler can verify that data crossing a thread boundary is `Send` (for moves) or behind a `Sync` wrapper (for sharing). This eliminates *data races at the memory level*.
- **What they do NOT prove**: `Send + Sync` does not mean your logic is race-free, deadlock-free, idempotent, or correctly ordered. Two threads each correctly locked can still produce a logic race (check-then-act across two locks), a deadlock (lock ordering), or a lost update (read-modify-write across a release-and-reacquire).

Treat `Send + Sync` as "the compiler will not stop you," not "the concurrency is correct." The logic-level races are still your responsibility.

### Choose The Sharing Mechanism By Access Pattern

Rust offers several ways to share data across threads; each fits a different access pattern. Pick deliberately, not by reflex.

- **Message passing (channels)**: best when the natural model is "one owner, others send it commands." A single thread owns the state and processes a mailbox of messages (`mpsc`, `crossbeam`, `flume`). This avoids locks entirely and makes the ownership clear. Good for actor-style designs.
- **`Mutex<T>`**: best for short, exclusive read-write critical sections. The lock serializes access. Use when multiple threads need to read and mutate shared state and the critical section is brief.
- **`RwLock<T>`**: best when reads vastly outnumber writes. Multiple readers or one writer. Beware: writer starvation and the cost of tracking readers can make `RwLock` slower than `Mutex` unless the read-heavy pattern is real.
- **Atomics (`AtomicUsize`, etc.)**: best for a single counter or flag. Cheaper than a lock, but only for primitive operations; do not build complex state from atomics without understanding memory ordering.
- **`Arc<T>` (immutable shared)**: best for shared read-only data (config, a large lookup table). No lock needed if the data is never mutated.

The reflexive `Arc<Mutex<_>>` is often wrong: if the data is read-only, `Arc<T>` alone suffices; if the model is command-based, a channel is clearer; if it is a counter, an atomic is cheaper. Match the mechanism to the pattern.

### Keep Critical Sections Minimal And Lock-Free Where Possible

A critical section is the code between acquiring a lock and releasing it. Long critical sections serialize throughput: every thread waiting on the lock is idle. Rules:

- Do the minimum work inside the lock: read or write the shared state, then release. Do I/O, computation, allocation, and especially `.await` outside the lock.
- Never hold a lock across an `.await` unless the lock is async-aware (`tokio::sync::Mutex`) and you have a deliberate reason. Holding a `std::sync::Mutex` across `.await` can deadlock the runtime (the task holding the lock is suspended and cannot release it while another task needs it) and is often a compile error via `Send` bounds.
- Compute results outside the lock, then take the lock only to commit the update. This pattern (read snapshot, compute, commit) minimizes contention.

Lock-free is not automatically better. Lock-free data structures are hard to write correctly (memory ordering, ABA problems) and often slower under contention than a well-scoped lock. Use a vetted crate (`crossbeam`, `dashmap`) rather than hand-rolling lock-free code.

### Prevent Deadlocks With Consistent Lock Ordering

Deadlock occurs when two threads each hold a lock the other needs, waiting forever. The compiler cannot detect this. Prevent it with discipline:

- **Establish a global lock acquisition order.** If multiple locks must be held, always acquire them in the same order across all code paths. Document the order.
- **Avoid nested locking.** If you can redesign so a code path holds only one lock at a time (via a single coarser lock, or by restructuring), do. Nested locks are where deadlocks live.
- **Use `try_lock` with timeouts for non-critical paths** where you would rather fail fast than risk a hang.
- **Beware lock-then-callback patterns.** Holding a lock while calling user-provided code (which may try to acquire another lock) is a deadlock factory.

Audit multi-lock code paths deliberately. A deadlock in production is hard to reproduce and diagnose; prevention by ordering is far cheaper.

### Handle Poisoning Deliberately

`std::sync::Mutex` (and `RwLock`) *poison* when a thread panics while holding the lock. After poisoning, subsequent `.lock()` calls return `Err` (a `PoisonError`). This is a feature: it signals that the protected data may be in an inconsistent state because a critical section was interrupted by a panic.

Decide your poisoning strategy deliberately:

- **Fail fast**: treat poisoning as fatal (propagate the error, restart the process). Safe for state where inconsistency is dangerous.
- **Recover**: call `.lock().unwrap_or_else(|e| e.into_inner())` to access the possibly-inconsistent data and repair it. Only do this if you understand how to restore consistency.
- **Never ignore silently**: `.lock().unwrap()` panics on poisoning, which may be acceptable in a binary but is usually wrong in a library or long-running service.

Poisoning is easy to forget because it rarely triggers in tests (no panics). Decide the strategy up front and apply it consistently.

### Use Bounded Channels For Backpressure

Channels decouple producers from consumers, but an unbounded channel lets a fast producer outrun a slow consumer, growing memory until OOM. Use bounded channels (`sync_channel(N)`, `tokio::sync::mpsc::channel(N)`) so a full channel blocks the producer, applying backpressure.

- Size the bound to the consumer's catch-up capacity, not to "infinity."
- Decide what happens when the channel is full: block the producer (backpressure), drop messages (lossy), or spawn more consumers (scale out). Each has different correctness implications.
- For request/response, a bounded channel plus an await on the response preserves backpressure end-to-end.

Unbounded channels are appropriate only when the producer cannot outrun the consumer by construction (e.g., a fixed set of messages known up front). Otherwise, bound them.

## Common Traps

### Arc<Mutex<_>> For Everything

Wrapping all shared state in `Arc<Mutex<_>>` regardless of access pattern. Read-only data needs only `Arc`; counters need atomics; command flows need channels. Match the mechanism to the pattern.

### Holding A Lock Across .await

Holding a `std::sync::Mutex` across `.await` deadlocks async runtimes and violates `Send` bounds. Release locks before awaiting; use async-aware locks only when you must await under a lock.

### Assuming Send + Sync Means Race-Free

`Send + Sync` proves memory-level safety, not logic correctness. Check-then-act across locks, lost updates, and inconsistent reads across multiple shared values are logic races the compiler cannot catch.

### Inconsistent Lock Ordering

Acquiring locks A then B in one path and B then A in another deadlocks. Establish and document a global acquisition order; avoid nested locks where possible.

### Unbounded Channels Under Load

`mpsc::channel()` (unbounded) grows without limit if the producer is faster, causing OOM. Use bounded channels for backpressure unless the producer is provably bounded.

### Ignoring Poisoning

`.lock().unwrap()` panics on poisoning, which may be fine in a binary but crashes a service after an unrelated panic. Decide the poisoning strategy and apply it consistently.

### RwLock Where Mutex Would Do

`RwLock` is slower than `Mutex` unless reads truly dominate writes. The reader-tracking overhead and writer-starvation risk make `RwLock` a poor default; reach for it only with a measured read-heavy pattern.

### Blocking The Runtime Thread

Calling `std::thread::sleep`, a blocking syscall, or CPU-heavy work inside an async task blocks the runtime worker thread, stalling all other tasks on that thread. Offload blocking work to `spawn_blocking`.

## Self-Check

- [ ] The sharing mechanism matches the access pattern: read-only uses `Arc`, counters use atomics, command flows use channels, read-write shared state uses `Mutex`/`RwLock` with brief critical sections.
- [ ] `Send` and `Sync` are understood as memory-level proofs, not logic-correctness guarantees; logic races (check-then-act, lost updates) were reviewed manually.
- [ ] Critical sections are minimal: no I/O, computation, allocation, or `.await` inside a lock; results are computed outside and committed inside.
- [ ] No `std::sync::Mutex` is held across `.await`; async-aware locks are used only with deliberate reason and released before awaiting external work.
- [ ] Multi-lock code paths follow a documented global acquisition order to prevent deadlocks; nested locking is avoided or audited.
- [ ] Channels are bounded to apply backpressure; unbounded channels are used only where the producer is provably bounded.
- [ ] Poisoning strategy is decided (fail-fast or deliberate recovery) and applied consistently; `.lock().unwrap()` is not used silently in service code.
- [ ] `RwLock` is used only with a measured read-heavy pattern; `Mutex` is the default for exclusive access.
- [ ] Blocking work inside async tasks is offloaded to `spawn_blocking` or a dedicated thread; runtime worker threads are not blocked.
- [ ] Lock-free code uses vetted crates (`crossbeam`, `dashmap`) rather than hand-rolled atomics with manual memory ordering.
