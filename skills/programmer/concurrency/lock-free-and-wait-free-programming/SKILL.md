---
name: lock_free_and_wait_free_programming.md
description: Use when the agent is writing or reviewing lock-free or wait-free code using compare-and-swap, atomic operations, or memory ordering, designing a lock-free data structure, choosing between lock-free and lock-based approaches, or diagnosing subtle concurrency bugs like ABA, lost updates, or memory reclamation races. Covers CAS loops and the ABA problem, memory ordering and barriers (acquire/release vs sequentially consistent), the distinction between lock-free and wait-free progress guarantees, the complexity and failure modes of lock-free code, when lock-free is and is not justified, backoff and contention management, memory reclamation (hazard pointers, epoch-based reclamation), and the discipline of preferring standard concurrent data structures over hand-rolled atomics. Also use when a CAS loop shows pathological contention, when a lock-free structure has subtle correctness bugs, or when deciding whether a hot path justifies lock-free complexity.
---

# Lock-Free And Wait-Free Programming

Lock-free programming is the use of atomic operations and careful memory ordering to coordinate threads without mutexes, and it is one of the most defect-prone things a programmer can attempt. The appeal is real: under high contention, a lock-free structure can outperform a lock-based one by avoiding blocking, context switches, and priority inversion. The cost is enormous and usually underestimated. A correct lock-free data structure must reason about interleavings that are invisible to testing, memory orderings that differ across architectures, the ABA problem, safe memory reclamation in a concurrent setting, and progress guarantees that are weaker than they appear. The recurring failure is reaching for lock-free because "locks are slow" without measuring whether locks are actually the bottleneck, then shipping a structure with a subtle race that passes every test and corrupts data under the specific contention pattern that appears in production months later.

Agents tend to underestimate lock-free complexity because the primitives (CAS, atomics) look simple in isolation. The defects live in the composition: a CAS loop that is correct in the common case but suffers the ABA problem when a value cycles back to a previous state; an acquire/release ordering that is correct on x86 but breaks on ARM where weaker ordering is the default; a node reclamation scheme that frees a node still referenced by a concurrent reader, corrupting the heap; a "lock-free" structure that is actually only obstruction-free and livelocks under contention. The judgment problem is treating lock-free as a last-resort optimization justified by measured contention, implemented with standard vetted structures where possible, and never as a default concurrency strategy — and when it is used, applying the specific defenses (ABA prevention, correct memory ordering, safe reclamation) that the primitives require.

This skill is about writing and evaluating lock-free and wait-free code correctly. It complements the thread-safety skill (general shared-state correctness) and the memory-ordering skill (the memory model in depth); here the question is the specific design and failure modes of lock-free and wait-free algorithms, and when their complexity is justified.

## Core Rules

### Treat Lock-Free As A Last Resort, Justified By Measured Contention

The first question about lock-free code is whether it should exist at all. Lock-free is an optimization for contention, and like any optimization it requires evidence that the thing it optimizes is actually the bottleneck.

- **Measure before choosing lock-free.** Is the lock-based version actually a bottleneck under realistic contention? Often it is not: a well-tuned lock on an uncontended or lightly-contended path is fast, and the complexity cost of lock-free is not worth paying. Reach for lock-free only when measurement shows lock contention dominating a hot path.
- **Try simpler options first.** A sharded lock (lock per stripe), a read-write lock (many readers, few writers), or a standard concurrent data structure often resolve contention with far less risk than hand-rolled atomics. Exhaust these before going lock-free.
- **Prefer standard, vetted lock-free structures over hand-rolled ones.** Concurrent queues, maps, and skip lists from mature libraries are battle-tested against the subtle bugs below. Hand-rolling a lock-free structure should be rare and require strong justification; the probability of a subtle correctness bug in hand-rolled atomics is high.
- **Accept the maintenance cost explicitly.** Lock-free code is harder to review, harder to modify, and harder to debug than lock-based code. The person who maintains it in two years may not be the person who wrote it; weigh that against the performance benefit.

### Understand And Prevent The ABA Problem

The ABA problem is the canonical defect of CAS-based algorithms, and it is invisible to most testing. A CAS loop reads value A, computes a new value, and CASes A to the new value — succeeding only if the value is still A. The bug: the value may have changed to B and back to A between the read and the CAS, so the CAS succeeds even though the assumption "still A means unchanged" is false.

- **Recognize where ABA bites.** It affects algorithms that CAS a pointer or version and assume "same value means no change." A classic case is a lock-free stack pop: thread 1 reads top=A, A->next=B; thread 2 pops A, pushes C, then pushes A again (recycled); thread 1's CAS of top from A to B succeeds, but B may have been freed — corruption.
- **Prevent ABA with a version tag (tagged pointer).** CAS on a (pointer, version) pair so that recycling a value increments the version and the CAS no longer matches. This requires fitting the version into the atomic word (pointer tagging) or a double-word CAS.
- **Prevent ABA with hazard pointers or epoch-based reclamation.** Instead of tagging, ensure a freed node cannot be recycled while any thread may still hold a reference to it (see the reclamation rule below).
- **Suspect ABA in any intermittent corruption of a CAS-based structure.** ABA bugs are timing-dependent and rarely reproduce in tests; if a lock-free structure occasionally corrupts under load, ABA is a prime suspect.

### Get Memory Ordering Right, And Default To Sequential Consistency Until Proven Otherwise

Atomic operations take a memory ordering argument, and choosing weaker orderings (relaxed, acquire/release) for performance introduces correctness risk. The default should be the strongest ordering, weakened only with evidence and understanding.

- **Default to sequentially consistent ordering.** It is the easiest to reason about and is correct for almost all cases. The performance cost relative to weaker orderings is often small and is worth paying for correctness.
- **Weaken to acquire/release only when you understand the happens-before relationship it creates** and have measured that the weaker ordering provides meaningful benefit. Acquire (on load) and release (on store) establish synchronization for non-atomic data published through the atomic, but only if every access to that data is correctly ordered; a single missing ordering breaks the guarantee.
- **Never use relaxed ordering for synchronization.** Relaxed atomics guarantee atomicity but no ordering or visibility; using them where ordering matters is a silent correctness bug that may pass on x86 (strong ordering) and fail on ARM/POWER (weak ordering).
- **Test on the weakest architecture you ship on.** Code correct under x86's strong memory model may break under ARM's weaker model. If you ship on ARM, test on ARM; do not assume x86 correctness transfers.
- **See the memory-ordering skill for the memory model in depth.** This is the most subtle area of lock-free programming and deserves its own careful treatment.

### Ensure Safe Memory Reclamation In A Concurrent Setting

In a lock-based structure, a lock prevents a thread from freeing a node another thread is using. In a lock-free structure, no lock protects readers from a concurrent free, so freeing a node that a slow reader still references corrupts the heap. Safe memory reclamation is the hard problem that distinguishes a correct lock-free structure from a dangerous one.

- **Use hazard pointers.** A reader publishes the address it is about to dereference into a thread-local "hazard" slot; a reclaimer checks all hazard slots before freeing and defers the free if any thread hazards the address. Bounds the number of deferred nodes and is correct but adds per-access overhead.
- **Use epoch-based reclamation (EBR).** Threads announce their epoch; a node freed in epoch N is not actually reclaimed until all threads have advanced past epoch N+2 (or similar), ensuring no thread holds a stale reference. Lower overhead than hazard pointers but reclaims lazily and can delay reclamation under slow threads.
- **Use reference counting where feasible.** A node is freed when its last reference drops. Tricky to combine with lock-free (incrementing the count must be atomic and race-free with the dereference), but correct when done with care (e.g., split reference counts).
- **Never free a node a concurrent reader may reference without a reclamation scheme.** A bare `free` in a lock-free structure is almost certainly a use-after-free under contention. This is the most common catastrophic bug in hand-rolled lock-free structures.

### Understand The Progress Guarantee, And Do Not Overclaim It

"Lock-free" and "wait-free" are specific technical claims about progress, and they are often conflated or misstated. Knowing which guarantee your algorithm provides tells you what failure modes remain.

- **Wait-free** guarantees every thread makes progress in a bounded number of steps, regardless of other threads. The strongest guarantee; no thread can starve. Rare and expensive; few practical data structures are fully wait-free.
- **Lock-free** guarantees system-wide progress: at least one thread makes progress, but individual threads may starve (livelock under contention). Most "lock-free" structures are this; under high contention a thread may retry its CAS many times.
- **Obstruction-free** guarantees progress only in the absence of contention (a thread makes progress if it runs alone for long enough). Weaker than lock-free; an obstruction-free structure can livelock permanently under sustained contention.
- **Match the guarantee to the requirement.** If a real-time or latency-sensitive path requires bounded per-thread progress, lock-free is insufficient — you need wait-free, or a different design. Do not assume a "lock-free" structure gives per-thread progress guarantees it does not.

### Manage Contention With Backoff And Adaptive Schemes

A CAS loop under high contention can livelock or waste CPU as threads repeatedly invalidate each other's attempts. Contention management makes lock-free structures usable under load.

- **Back off on CAS failure.** After a failed CAS, wait briefly (spin-then-yield, or a randomized short delay) before retrying, to reduce contention. Without backoff, N threads hammering one CAS slot can achieve near-zero throughput.
- **Use exponential or randomized backoff.** Deterministic backoff can synchronize threads (all back off for the same duration and collide again); randomization spreads retries.
- **Consider eliminating the contention rather than managing it.** Sharding the hot atomic across multiple slots (e.g., a counter striped across N atomics) reduces contention more effectively than backoff on a single slot. Sometimes the right answer is less sharing, not better contention management.

## Common Traps

### Lock-Free Without Measured Contention

Hand-rolling a lock-free structure because "locks are slow," when the lock-based version was not a measured bottleneck. Measure first; reach for lock-free only when contention dominates a hot path.

### Hand-Rolling Instead Of Using A Vetted Structure

Writing a custom lock-free queue or map when a mature library provides a tested one, introducing subtle bugs the library already solved. Use standard concurrent data structures; hand-roll only with strong justification.

### The ABA Problem In A CAS Loop

A CAS that succeeds on a recycled value, corrupting the structure, with the bug invisible in testing and intermittent in production. Prevent ABA with version tags or safe reclamation; suspect it in any intermittent CAS-structure corruption.

### Wrong Memory Ordering, Correct On x86, Broken On ARM

Using relaxed or improperly paired acquire/release ordering that happens to work on x86's strong model but fails on ARM's weak model. Default to sequential consistency; weaken only with understanding; test on the weakest architecture shipped.

### Freeing A Node A Concurrent Reader References

A bare `free` in a lock-free structure, creating a use-after-free when a slow reader dereferences a node being reclaimed. Use hazard pointers, epoch-based reclamation, or reference counting; never free without a reclamation scheme.

### Overclaiming The Progress Guarantee

Calling a structure "wait-free" when it is only lock-free (individual threads can starve), or "lock-free" when it is only obstruction-free (can livelock under contention). Know and state the actual guarantee; match it to the requirement.

### CAS Livelock Under Contention

A CAS loop with no backoff, where contending threads repeatedly invalidate each other and achieve near-zero throughput. Add randomized or exponential backoff, or reduce the sharing (sharding).

### Assuming Sequential Consistency Is Too Slow Without Measuring

Weakening memory ordering for performance without measuring that sequential consistency was actually slow, trading correctness for an unmeasured gain. Default to sequential consistency; weaken only with measured benefit and full understanding.

## Self-Check

- [ ] Lock-free was chosen only after measurement showed lock contention dominating a hot path, simpler options (sharded locks, read-write locks, standard concurrent structures) were considered first, and a standard vetted structure is used rather than a hand-rolled one unless hand-rolling is strongly justified.
- [ ] The ABA problem is addressed: the algorithm does not assume "same CAS value means unchanged," and ABA is prevented with version tags (tagged pointers / double-word CAS) or safe memory reclamation (hazard pointers, EBR).
- [ ] Memory ordering is correct: sequential consistency is the default, acquire/release is used only where the happens-before relationship is understood and measured to benefit, relaxed ordering is never used for synchronization, and the code is tested on the weakest architecture shipped (e.g., ARM, not just x86).
- [ ] Memory reclamation is safe: no node is freed while a concurrent reader may reference it; hazard pointers, epoch-based reclamation, or careful reference counting is in place, and there is no bare `free` reachable by a concurrent reader.
- [ ] The progress guarantee (wait-free, lock-free, or obstruction-free) is known and correctly stated, and matches the requirement — a latency-sensitive or real-time path does not rely on a guarantee weaker than it needs.
- [ ] Contention is managed: CAS loops use randomized or exponential backoff, or the hot atomic is sharded across slots to reduce sharing, preventing livelock or near-zero throughput under load.
- [ ] The structure is stress-tested under high contention and on the target architecture, not just unit-tested in the common case, since lock-free bugs are timing- and architecture-dependent and rarely reproduce in simple tests.
- [ ] The complexity and maintenance cost of the lock-free code is justified by measured benefit, and the choice is documented so future maintainers understand why the complexity exists and what invariants they must preserve.
