---
name: deadlock_prevention_and_detection.md
description: Use when the agent is designing or debugging code that uses multiple locks, mutexes, or resources that could deadlock — acquiring locks in inconsistent orders, holding locks while awaiting other locks, lock hierarchies, try-lock and timeout patterns, deadlock detection and avoidance, distributed locks across services, or diagnosing a hung system where threads or processes are blocked waiting on each other. Covers lock ordering, lock hierarchies, avoiding holding locks across await/callback/IO boundaries, timeout and backoff strategies, deadlock detection mechanisms, and the design discipline that prevents deadlocks structurally rather than catching them after the fact.
---

# Deadlock Prevention And Detection

A deadlock is a state where two or more participants are each waiting for a resource the other holds, and none can proceed. In concurrent code, the classic form is two threads each holding one lock and waiting for the other's lock, freezing both forever. In distributed systems, the form is two services each holding a distributed lock and waiting for the other's, or a set of processes in a circular wait. Deadlocks are insidious because the code looks correct in isolation — each thread acquires the locks it needs, does its work, and releases them — and the defect appears only in the specific interleaving where the circular wait forms, which may be rare in testing and frequent in production. A system that deadlocks intermittently under load, hanging requests until timeouts fire, is a system whose concurrency design assumed locks could be acquired in any order, and that assumption is the root of the failure.

Agents tend to add locks reactively ("this shared state needs protection") without considering the order in which locks are acquired across the codebase, or whether a lock is held across a boundary (an await, a callback, an I/O call) where it should not be. The judgment problem is recognizing that deadlock is a structural property of how locks are acquired relative to each other, that preventing it requires a discipline (lock ordering, lock hierarchies, avoiding hold-across-boundary) applied consistently across all code that takes locks, and that detecting deadlocks after they occur is a fallback, not a design. This skill covers the discipline of preventing deadlocks structurally and detecting them when prevention is imperfect: ordering locks, avoiding the patterns that create circular waits, and building systems whose concurrency does not deadlock under the interleavings production will produce.

## Core Rules

### Enforce A Consistent Lock Ordering

The fundamental deadlock-prevention rule is that if locks are always acquired in a consistent global order, no circular wait can form, and circular wait is the necessary condition for deadlock. Inconsistent ordering is the most common deadlock cause.

- **Define a global order for all locks and acquire them in that order everywhere.** If lock A must be acquired before lock B in one code path, it must be acquired before lock B in every code path. A single path that acquires B then A while another acquires A then B is a deadlock waiting for the right interleaving.
- **Order by a stable property (address, id, name) when locks are dynamic.** When the set of locks is not known at design time (locking multiple records by id), acquire them in sorted order (ascending id) so any code path locking the same set acquires them in the same order.
- **Document the ordering and enforce it in review.** A lock ordering that exists only in the original author's head will be violated by the next contributor. Document the hierarchy; review new lock acquisitions against it; consider tooling (lock-order sanitizers, static analysis) that detects violations.
- **Acquire all locks for an operation atomically where possible.** Some libraries provide acquire-multiple-locks-atomically primitives that either acquire all or none, eliminating the partial-acquisition window where deadlock can form. Use them when acquiring multiple locks together.

### Avoid Holding Locks Across Boundaries Where Blocking Occurs

A lock held while the code does something that can block indefinitely — awaiting an async operation, calling a callback, performing I/O, waiting on another lock, sleeping — extends the lock's hold time unboundedly and dramatically increases deadlock risk. The lock becomes held across whatever the boundary does, including waiting for resources that create circular waits.

- **Do not hold a lock across an await, callback, or suspension point in async code.** In async/await runtimes, holding a lock across an await means the lock is held while the task is suspended, potentially for a long time, and any other task needing the lock blocks. Worse, if the runtime runs tasks on a limited thread pool, the blocked tasks can exhaust it (a broader deadlock). Use async-aware synchronization or release the lock before awaiting.
- **Do not hold a lock across I/O or external calls.** A lock held across a network call, a database query, or a file operation is held for the duration of that operation, including its failure modes (hang, timeout). This serializes all other users of the lock behind an unbounded operation. Acquire the lock, do the in-memory work, release it, then do the I/O.
- **Do not hold a lock while acquiring another lock that may block, unless the ordering is guaranteed.** Holding lock A while waiting for lock B is the classic deadlock setup; it is safe only if the global ordering guarantees no one holds B waiting for A. Prefer to acquire all needed locks up front in order, or restructure so locks are not nested.
- **Minimize lock hold time.** The shorter the hold, the lower the contention and the lower the chance of an adverse interleaving. Do the minimum work under the lock; move computation and I/O outside it.

### Prefer Coarse-To-Fine Evolution And Avoid Unnecessary Locking

The more locks a system has, and the more they are nested, the higher the deadlock risk. The discipline is to use the fewest locks that provide the needed correctness, and to add finer locking only when contention demands it.

- **Start with coarse locking and refine only when profiling shows contention.** A single lock protecting a structure is deadlock-free (no second lock to conflict with) and simple; it may contend under high concurrency, at which point finer locking can be introduced deliberately. Starting fine creates deadlock risk before contention justifies it.
- **Prefer lock-free or single-writer designs where feasible.** Message passing (channels, actor model), single-threaded event loops, and immutable data structures avoid shared-state locking entirely and cannot deadlock on locks (though they can deadlock on channels — see below). Choose them where the design allows.
- **Avoid locking around data that does not need it.** Per-thread or per-request data, immutable data, and data whose access is already serialized by design do not need locks. Unnecessary locks add risk without benefit.
- **Beware channel-based deadlocks.** Channels and message queues can deadlock too: a sender blocked on a full channel whose consumer is blocked sending to a full channel back. Bounded channels create backpressure that can deadlock if the topology has a cycle; design the message topology to be acyclic, or break cycles with buffering, timeouts, or dropping.

### Use Timeouts And Try-Lock As A Safety Net, Not A Primary Design

Acquiring locks with a timeout or using try-lock (non-blocking acquire) prevents a thread from waiting forever, providing a fallback if a deadlock forms. This is a safety net, not a substitute for ordering discipline.

- **Use timed lock acquisition for locks at risk of deadlock.** Acquiring with a timeout means a deadlock manifests as a timeout (which can be logged, retried, or escalated) rather than a permanent hang. The timeout should be long enough to avoid firing under normal contention but short enough to detect a deadlock promptly.
- **Have a defined behavior on timeout.** A timed acquire that times out must do something: release locks already held (to break the potential deadlock), retry with backoff, log and continue without the operation, or fail the operation. A timeout with no follow-up action just converts a permanent hang into a repeated one.
- **Do not rely on timeouts as the deadlock strategy.** A system that "solves" deadlock by timing out every lock acquisition is a system that deadlocks frequently and recovers slowly. Use timeouts as detection and recovery layered on top of prevention (ordering, no-hold-across-boundary).
- **Avoid try-lock retry loops that spin.** A try-lock that fails and immediately retries in a tight loop burns CPU. Back off (sleep, yield) between attempts, or block on the lock with a timeout instead.

### Build Deadlock Detection For Systems Where Prevention Is Imperfect

In large or legacy systems, or systems with external locking (distributed locks, database locks), prevention may be incomplete. Detection provides visibility and recovery.

- **Use lock-order detection tooling where available.** Some runtimes and sanitizers (ThreadSanitizer's lock-order checks, Java's built-in deadlock detection) detect potential or actual lock-order violations at test time. Run concurrency tests under them.
- **Implement deadlock detection for thread dumps and monitoring.** A system that can dump its thread states and detect cycles in the wait-for graph can identify a deadlock in production and trigger recovery (interrupting a thread, releasing a lock, restarting a process).
- **For distributed locks, use leases with expiry and fencing.** A distributed lock held indefinitely by a crashed or paused holder blocks everyone. Use leases that expire (so a crashed holder's lock is eventually released) and fencing tokens (so a stale holder's later writes are rejected). See retries-timeouts-and-circuit-breakers and idempotency-and-race-safety.
- **Monitor for hung requests and stuck workers.** Deadlock often manifests as requests or workers that hang without error. Alert on request latency beyond a threshold, worker inactivity, or thread-blocked durations, so a deadlock is detected operationally even without explicit detection logic.

## Common Traps

### Inconsistent Lock Ordering Across Code Paths

One path acquiring A then B, another acquiring B then A, deadlocking under the interleaving where both hold their first lock waiting for the second. Define and enforce a global lock order.

### Holding A Lock Across An Await Or I/O

Holding a lock across an async suspension point or an external call, extending hold time unboundedly and risking deadlock or thread-pool exhaustion. Release before the boundary; re-acquire after.

### Nested Lock Acquisition Without Ordering Guarantee

Acquiring a second lock while holding a first, without a guaranteed global order, creating the circular-wait setup. Acquire all locks up front in order, or restructure to avoid nesting.

### Too Many Fine-Grained Locks

Introducing many fine-grained locks before contention justifies it, multiplying deadlock risk. Start coarse; refine when measured contention demands.

### Channel Topology With Cycles

Bounded channels in a cyclic topology (A sends to B, B sends to A, both full) deadlocking via backpressure. Design acyclic message topologies or break cycles.

### Timeout Without Follow-Up Action

Timed lock acquisition that times out but does nothing (or spins retrying), converting a permanent hang into a repeated one. Define the timeout behavior: release, backoff-retry, or fail.

### Distributed Lock Without Lease Or Fencing

A distributed lock held indefinitely by a crashed holder, or accepted from a stale holder, causing permanent blocking or split-brain writes. Use leases with expiry and fencing tokens.

### No Detection Or Monitoring For Hung Workers

A deadlock that manifests as silently hung requests or stuck workers, undetected because there is no latency or thread-state monitoring. Alert on hung requests and blocked threads.

## Self-Check

- [ ] A global lock ordering is defined and enforced consistently across all code that acquires multiple locks — locks are acquired in the same order in every path (by a stable property for dynamic locks), the ordering is documented, and new acquisitions are reviewed (and statically analyzed where tooling exists) against it.
- [ ] Locks are not held across await/callback/suspension points, I/O or external calls, or while acquiring another blocking lock — the minimum work is done under the lock, computation and I/O are moved outside, and locks needed together are acquired up front in order.
- [ ] Locking starts coarse and is refined to finer granularity only when profiling demonstrates contention, lock-free or message-passing designs are preferred where feasible, and unnecessary locks (around per-thread, immutable, or already-serialized data) are avoided.
- [ ] Channel and message topologies are designed to be acyclic, or cycles are broken with buffering, timeouts, or drop policies, so bounded-channel backpressure cannot deadlock the system.
- [ ] Lock acquisition at risk of deadlock uses timeouts or try-lock as a safety net, with a defined follow-up behavior on timeout (release held locks, backoff-retry, log-and-continue, or fail) — not as the primary deadlock strategy, and without spinning retry loops.
- [ ] Deadlock detection is in place for systems where prevention may be imperfect: lock-order sanitizers in testing, thread-dump cycle detection in production, and distributed locks use leases with expiry and fencing tokens.
- [ ] Monitoring detects deadlocks operationally — alerts on request latency beyond threshold, worker inactivity, and thread-blocked durations — so a deadlock is visible even without explicit detection logic.
- [ ] The concurrency design has been reviewed for the deadlock conditions (mutual exclusion, hold-and-wait, no preemption, circular wait) and structured so that at least one condition cannot hold, typically by eliminating circular wait via consistent ordering or by eliminating hold-and-wait via atomic/all-or-nothing acquisition.
