---
name: thread_safety_and_shared_state.md
description: Use when the agent is writing or reviewing code where multiple threads, goroutines, async tasks, or event-loop callbacks can access the same data at the same time — including shared fields, singletons, caches, registries, static or global state, background workers mutating request state, in-memory counters, lazy initialization, connection pools, or any mutable object reachable from more than one execution context. Covers thread safety, shared mutable state, synchronization, locks and deadlocks, memory visibility and happens-before, lock-free and CAS, immutability and confinement strategies, and choosing between mutexes, channels, actors, and STM. Also use when diagnosing heisenbugs that pass single-threaded tests but fail under load, stale reads, lost updates, torn writes, or intermittent corruption.
---

# Thread Safety And Shared State

Thread safety is a decision about what state may be observed by more than one execution context at once, and how that observation stays correct when the contexts overlap. It is not the act of sprinkling `synchronized`, `Mutex`, `lock`, or `atomic` over fields. The hardest concurrency bugs do not crash on every run — they pass every single-threaded test, then corrupt state once in a thousand runs, only under production load, at 3am, in a way that vanishes the moment you add a print statement.

Agents tend to under-invest in this judgment for three reasons. First, the single-threaded path usually works, so the concurrency risk is invisible until late. Second, the language or framework often makes sharing look trivially easy (a global, a singleton, an `Arc`, a shared map) while hiding that easy sharing is exactly where races live. Third, the failure modes are non-deterministic: a race is not a deterministic logic error you can reason about by reading the code top to bottom, it is an interleaving of operations that may or may not happen. The judgment problem is not "how do I lock this" but "should this state be shared at all, and if it is shared, what invariants must survive every possible interleaving."

This skill focuses on in-process shared mutable state and the correctness of concurrent access to it. It is distinct from the idempotency skill, which covers duplicate execution and "happens once" invariants across retries, queues, and processes; and from the resilience skill, which covers timeouts, retries, and circuit breakers across service boundaries. Here the question is local: when two threads touch the same memory, what keeps the memory correct.

## Core Rules

### Prefer No Sharing Over Safe Sharing

The strongest concurrency design is the one with no shared mutable state. Before asking how to synchronize, ask whether the state needs to be shared at all. Every piece of shared mutable state is a potential race, a potential deadlock, and a permanent testing burden. Options that eliminate sharing, in rough order of preference:

- **Immutable values.** If a value never changes after construction, any number of threads can read it freely. Build new immutable snapshots instead of mutating shared structures. This is why functional, copy-on-write, and persistent data structures are concurrency-friendly.
- **Thread / task confinement.** Keep mutable state confined to the single thread or task that owns it. If only one context ever touches the state, there is no race by construction. UI thread models, actor inboxes, and per-worker state all rely on confinement.
- **Message passing.** Instead of sharing memory, pass ownership of data through channels, queues, or mailboxes. The receiver becomes the sole owner for the duration of its work. "Don't communicate by sharing memory; share memory by communicating."
- **Shared immutable, private mutable.** Share read-only configuration widely; keep each thread's working state private and merge results at a synchronization point.

Sharing is sometimes unavoidable (caches, registries, shared counters, connection pools). When it is, the sharing is a deliberate design decision with a documented invariant, not an accident of where a variable happened to be declared.

### Define The Invariant Before Choosing A Lock

A lock does not make code correct; it serializes access. Correctness comes from the invariant the serialization preserves. Before adding synchronization, state the invariant that must hold across all interleavings:

- A counter must equal the total number of increments applied (lost-update freedom).
- A map entry, once readable, must not be observed half-constructed (publication safety).
- A transfer must debit one account and credit another together, never one without the other (compound atomicity).
- A lazy singleton must be constructed exactly once (initialization safety).

Each invariant implies a different synchronization need. A single counter may need only an atomic increment. A compound transfer needs a lock or transaction covering both accounts. A lazy singleton needs either a lock, a once-cell primitive, or a correctly-double-checked pattern with the right memory ordering. Reaching for `synchronized` on the whole class to "be safe" both under-protects (it may not cover the real compound operation) and over-protects (it serializes unrelated work). Name the invariant first; the mechanism follows.

### Match The Synchronization Mechanism To The Operation

There is no single correct locking style. Match the mechanism to the shape of access:

- **Atomics (CAS, fetch-add, memory ordering).** Best for a single field or a small fixed set of fields with independent updates. Low overhead, no blocking, but only correct if the operation is genuinely a single atomic step. A read-modify-write that is not expressed as one atomic instruction is still a race even with atomics nearby.
- **Read-write locks.** Best when reads vastly outnumber writes and the critical section is non-trivial. They add fairness and writer-starvation concerns; under heavy write contention they can be slower than a plain mutex.
- **Mutex / monitor.** Best for compound operations that must be atomic together. The cost is serialization and the risk of deadlock, so keep critical sections short and avoid nested calls to unknown code.
- **Lock-free and wait-free structures.** Best for hot, contended paths where blocking is unacceptable. They are far harder to write correctly than they look; prefer battle-tested library implementations over hand-rolled lock-free code.
- **Higher-level abstractions (channels, actors, STM, async runtimes' primitives).** Best when they express the workflow more clearly than raw locks. They move the synchronization into a tested runtime and let you reason at a higher level.

The weak choice is reaching for the same mechanism everywhere. A global mutex around a counter that could be an atomic serializes the whole program; a hand-rolled CAS loop around a compound update is a latent bug.

### Establish Memory Visibility, Not Just Mutual Exclusion

Many concurrency bugs are not races in the "two operations overlap" sense but visibility bugs: thread A wrote the value, why does thread B still see the old one? Most languages with shared memory have a memory model that defines when a write by one thread becomes visible to another. The guarantee is usually expressed as a happens-before relationship: a release (unlock, store-release, volatile write, channel send) paired with an acquire (lock, load-acquire, volatile read, channel receive) establishes that everything the writer did before the release is visible to the reader after the acquire.

Implications that are easy to miss:

- Publishing a reference to a mutable object does not guarantee the object's fields are visible to the reader. The object must be safely published: through a lock, a volatile/atomic field, a final-field constructor guarantee, or a memory barrier.
- A plain boolean `ready` flag checked in a loop by one thread and set by another is not guaranteed to ever become visible without the right memory ordering. This is the classic "why does my background thread hang" bug.
- Double-checked locking is broken unless the inner field has the correct memory ordering. Many historical "optimizations" of lazy init were subtle publication bugs.
- Caching values in CPU registers or reordering by the compiler means code that "obviously" runs in order may not, across threads, without synchronization.

If you cannot articulate the happens-before relationship that makes a write visible to another thread, assume the visibility is undefined and add the synchronization that establishes it.

### Order Locks To Prevent Deadlock

When a path acquires more than one lock, deadlock becomes possible: thread A holds lock 1 and waits for lock 2 while thread B holds lock 2 and waits for lock 1. The defenses are well known but easy to forget under deadline:

- **Global lock ordering.** Acquire locks in a consistent order everywhere. If every path acquires lock 1 before lock 2, no cycle can form. Document the order; an ordering that exists only in one author's head will be violated by the next.
- **Lock timeout with back-off.** Acquire with a timeout; on timeout, release everything and retry. Avoids permanent deadlock at the cost of livelock risk under contention.
- **Acquire all-or-nothing.** Acquire all needed locks atomically (try-lock all, release and retry if any fails) so a thread holds either all or none.
- **Avoid holding locks across unknown code.** Calling a callback, virtual method, or external function while holding a lock is a deadlock invitation: the callee may try to acquire a lock the caller already holds, or may block indefinitely. Acquire, do the minimal protected work, release, then call out.

Deadlocks often hide in async code too: holding a lock across an `await` that itself awaits work needing the same lock serializes or deadlocks the runtime. Treat any lock held across a suspension point as suspicious.

### Size Lock Granularity Deliberately

Lock granularity is a tradeoff between correctness and throughput, and the wrong choice fails in opposite directions:

- **Too coarse** (one big lock around a whole module). Correct but serializes unrelated operations, destroying throughput under load. Also tends to grow into a deadlock surface as more paths share the lock.
- **Too fine** (a separate lock per field). Fast under low contention but fails to protect compound invariants — two fields updated "atomically" under separate locks are not atomic together. Also increases lock overhead and deadlock surface.

Choose granularity by the invariant: group under one lock everything that must change together, and separate locks only for state that changes independently. A common strong pattern is sharding — many locks keyed by id (one lock per account, per shard, per bucket) so unrelated operations do not contend, while each shard's invariants are fully protected.

### Make Lazy And Once-Only Initialization Actually Once

Lazy initialization, singletons, and one-time setup are a classic race source because the obvious code — "if not initialized, initialize" — lets two threads both see "not initialized" and both initialize. The correct options:

- Use a language-provided once primitive (`std::sync::Once`, `sync.Once`, `.NET` `Lazy<T>`, Java's holder-idiom or `volatile` with proper ordering). These are tested and get the memory model right.
- Use a lock around the whole check-and-init, accepting the cost.
- Use atomic double-checked locking only if you understand the memory ordering required; the naive version is broken on most platforms.

The cost of getting this wrong ranges from a duplicated singleton (two connection pools, two config parses) to corrupted shared state if construction has side effects.

### Test Concurrency With Concurrency, Not Single Threads

Single-threaded tests cannot prove thread safety. A green test suite proves only that the sequential path works. To have any confidence in concurrency, the tests must actually run concurrent operations:

- Run many threads doing the same operation in a tight loop and assert the aggregate invariant (counter equals number of operations, no lost updates, no duplicates).
- Use property-based or stress tests with randomized scheduling, high thread counts, and many iterations.
- On languages that support it, use model checkers, sanitizers (ThreadSanitizer), or loom-style concurrency testing to catch races the scheduler did not happen to produce.
- Test the deadlock surface: paths that acquire multiple locks should be exercised together.

A concurrency design that has only single-threaded tests is untested for its actual risk.

## Common Traps

### Check-Then-Act On Shared State Without Synchronization

"If the flag is false, set it true" is a race when two threads can interleave: both read false, both set true, both proceed as if they were first. The same shape appears in lazy init, "if map lacks key, insert," rate limiters, and dedup sets. The fix is an atomic compare-and-swap, a lock around the compound operation, or a concurrent data structure that does the check-and-act atomically.

### Assuming "Atomic" Means "Thread-Safe"

An atomic operation is safe against itself. It does not make a compound operation atomic. `count++` as an atomic keeps the counter correct, but "read balance, add interest, write balance" using atomic reads and writes is still a lost-update race. Atomicity is per-operation; compound invariants need a lock or transaction spanning the whole operation.

### Publishing A Mutable Object Before It Is Fully Constructed

Storing a reference to a mutable object in a shared field, cache, or registry before the object is fully populated lets another thread observe it half-built. The fields may appear in any order (or default values) to the reader. Safely publish: populate fully, then publish through a volatile/atomic/locked write, or make the object immutable.

### Holding A Lock Across A Callback, Virtual Call, Or Await

Calling code you do not control while holding a lock is a deadlock and reentrancy trap. The callee may acquire the same lock (reentrancy or deadlock depending on the lock), may block on I/O holding your lock, or may call back into your module in surprising ways. Do the protected mutation, release, then invoke external code.

### Treating A Process-Local Lock As A Cross-Process Guarantee

An in-process mutex protects only threads within that process. Multiple containers, workers, cron jobs, or replicas each have their own mutex and do not see each other. If the race crosses processes, you need a database constraint, a distributed lock, or an external coordinator — not a `Mutex`. Confusing the two is a common source of "but we have a lock, how did this double-charge."

### The Naive Double-Checked Locking Pattern

`if (x == null) { lock; if (x == null) x = new ...; unlock; }` is broken without correct memory ordering, because the outer read is unsynchronized and may see a partially constructed object. Either use a once-primitive, hold the lock for the whole check, or use the language's correct volatile/atomic semantics. The pattern "looks" like an optimization and is a classic publication bug.

### Static And Global Mutable State As Implicit Sharing

A static field, module-level global, or singleton is shared across every thread by default. Code that looks single-threaded ("I just read and write this map") is concurrent when the map is global. Globals are the most common silent source of races because the sharing is not visible at the call site. Treat any mutable global as shared-by-default and synchronize or confine it.

### Volatile / Atomic As A Substitute For Locking

A volatile or atomic field guarantees visibility and per-operation atomicity. It does not make surrounding compound operations atomic, and in most languages it does not provide mutual exclusion. "I made it volatile so it's thread-safe" is a frequent and wrong conclusion. Volatile solves visibility; locks solve mutual exclusion; neither alone solves compound atomicity.

### Optimistic Concurrency Without Handling The Conflict

Compare-and-swap loops and optimistic version checks only work if the retry path is actually implemented and bounded. A CAS that fails and silently falls through, or that retries forever under contention (live-lock), is a bug. When using optimistic concurrency, handle the failure explicitly: retry with a bound, back off, or surface a conflict.

## Self-Check

- [ ] For every piece of shared mutable state, the decision to share it was deliberate — and alternatives (immutability, confinement, message passing) were considered and rejected for a stated reason, not because sharing was the default.
- [ ] The invariant that synchronization must preserve is named (lost-update freedom, compound atomicity, publication safety, once-only initialization) — and the chosen mechanism actually protects that invariant, not merely "a lock exists."
- [ ] The synchronization mechanism matches the operation: atomics for single fields, locks for compound operations, RW-locks only where reads dominate, and no hand-rolled lock-free code where a tested library exists.
- [ ] The happens-before relationship that makes each shared write visible to readers is articulable — via lock release/acquire, volatile/atomic ordering, safe publication, final-field semantics, or channel send/receive — and not assumed by intuition.
- [ ] Mutable objects are fully constructed and safely published before any other thread can observe the reference; no half-built object escapes through a shared field, cache, or registry.
- [ ] Every path that acquires more than one lock follows a documented global ordering, uses timeout/back-off, or acquires all-or-nothing — and no lock is held across callbacks, virtual calls, unknown code, or async suspension points.
- [ ] Lock granularity groups together what must change together and separates what changes independently; there is no single giant lock serializing unrelated work, and no per-field locking that fails a compound invariant.
- [ ] Lazy initialization, singletons, and once-only setup use a language once-primitive or correctly-ordered locking; the naive unsynchronized or broken double-checked pattern is absent.
- [ ] Process-local locks are not relied upon for races that cross processes, containers, workers, or replicas; cross-process invariants use database constraints, distributed locks, or external coordination.
- [ ] Concurrency correctness is tested with actual concurrent execution (multi-thread stress, randomized scheduling, sanitizers such as ThreadSanitizer, or loom-style model checking) — not only single-threaded unit tests that cannot observe a race.
