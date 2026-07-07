---
name: concurrent_data_structure_design.md
description: Use when the agent is designing or implementing a data structure that must be safe for concurrent access — concurrent maps, queues, sets, counters, ring buffers, skip lists, or custom shared structures; choosing between coarse locking, fine-grained locking, lock-free, or wait-free implementations; reasoning about progress guarantees, memory reclamation (epoch/Hazard pointers), ABA, and false sharing; or selecting/evaluating a concurrent collection from a library. Covers the tradeoffs among synchronization strategies for shared structures, progress guarantees, memory reclamation in lock-free structures, and when to build versus use a vetted library implementation.
---

# Concurrent Data Structure Design

A data structure that works correctly for a single thread is often incorrect under concurrency, and making it correct is a design problem with real tradeoffs. The naive fix — wrap every operation in a single lock — is correct but coarse: it serializes all access, eliminating the concurrency the structure exists to support, and under contention it becomes a bottleneck. The sophisticated alternative — lock-free or wait-free structures using compare-and-swap — preserves concurrency but introduces a different set of hazards: the ABA problem, memory reclamation races, weak memory ordering, and progress guarantees that are subtle to reason about. Between these extremes lies a spectrum of strategies (fine-grained locking, lock striping, read-write locks, copy-on-write), each with its own correctness and performance profile. The choice among them is not a matter of "lock-free is better"; it is a matter of matching the structure's access pattern, contention profile, and progress requirements to the strategy that fits.

Agents tend to either over-lock (a single mutex around everything, simple but a bottleneck) or reach for lock-free techniques they do not fully understand (introducing ABA, reclamation races, or incorrect memory ordering that produces rare data corruption). The judgment problem is recognizing that concurrent data structure design is a specialized discipline where correctness is hard-won and subtle, that the vast majority of needs are met by vetted library implementations rather than hand-rolled structures, and that building a custom concurrent structure is justified only when the library options do not fit and the author is prepared to handle the correctness hazards specific to the chosen strategy. This skill covers the discipline of designing and selecting concurrent data structures: choosing the synchronization strategy, understanding the progress and reclamation hazards, and knowing when to build versus use a vetted implementation.

## Core Rules

### Match The Synchronization Strategy To The Access Pattern

Different access patterns favor different synchronization strategies, and the right choice depends on the ratio of reads to writes, the contention level, and the required progress guarantee.

- **Coarse locking (one lock for the whole structure) for low contention or simple structures.** Correct, simple, and often fast enough when contention is low. The default; do not over-engineer with fine-grained locking until profiling shows the coarse lock is a bottleneck.
- **Read-write locks for read-heavy patterns.** Multiple readers can access concurrently; writers exclude all. Effective when reads vastly outnumber writes (a cache, a configuration map). Ineffective when writes are frequent (readers and writers both contend on the lock).
- **Fine-grained locking or lock striping for contended structures.** Partitioning the structure so different parts have independent locks (a hash map with a lock per bucket, or per stripe of buckets) allows concurrent access to different parts. Increases concurrency under contention, at the cost of complexity and the lock-ordering discipline (see deadlock-prevention) needed to avoid deadlock when an operation touches multiple partitions.
- **Copy-on-write for read-heavy, infrequent-write structures.** Writers create a new copy of the structure and atomically swap the reference; readers access the immutable old copy without locking. Excellent for read-heavy patterns (the reads are lock-free); expensive for frequent writes (each write copies).
- **Lock-free or wait-free for high-contention, latency-sensitive structures.** Using compare-and-swap (CAS) to update without blocking allows progress even under contention and avoids thread scheduling issues (priority inversion, convoying). The most complex strategy; reserve for cases where the benefit justifies the correctness cost.

### Understand And Choose The Progress Guarantee

Concurrent structures offer different progress guarantees, and the guarantee matters for whether the system can stall under contention or scheduling pathologies.

- **Blocking (lock-based): a thread holding the lock can block all others, and if it is descheduled or crashes while holding the lock, others wait.** Vulnerable to priority inversion (a low-priority holder blocks a high-priority waiter) and convoys (a burst of waiters forms behind a slow holder). Simple and usually adequate.
- **Lock-free: at least one thread makes progress in a bounded number of steps, but individual threads may starve (retry indefinitely under contention).** No thread can block all others (no lock to hold), but a consistently-unlucky thread may retry forever. The common CAS-based guarantee.
- **Wait-free: every thread makes progress in a bounded number of steps; no starvation.** The strongest guarantee, and the hardest to achieve. Reserved for structures where every operation must complete in bounded time regardless of contention (real-time systems).
- **Match the guarantee to the requirement.** Most structures do not need wait-free progress; lock-free or even blocking is adequate. Do not pay the complexity of a stronger guarantee than the system requires.

### Handle Memory Reclamation In Lock-Free Structures

Lock-free structures using CAS face a specific hazard: a thread may hold a reference to a node that another thread removes and frees, leading to use-after-free when the first thread's CAS accesses freed memory. This is the memory reclamation problem, and it is unique to lock-free structures (lock-based structures know no other thread accesses while the lock is held).

- **Use epoch-based reclamation (epoch RC) or hazard pointers.** Epoch-based reclamation defers freeing nodes until no thread can be in a critical section referencing them; hazard pointers register that a thread is accessing a node, preventing its reclamation. Both are the standard solutions; use a library implementation rather than hand-rolling.
- **Never free a node that a concurrent reader may still reference.** The hazard is invisible in the code (the CAS looks fine) and manifests as rare corruption. A reclamation scheme is mandatory for correct lock-free structures with removal.
- **In garbage-collected languages, reclamation is handled by the GC, simplifying lock-free structures.** This is a real advantage of GC'd languages for lock-free code; in manual-memory languages, the reclamation scheme is part of the structure's correctness.

### Prevent The ABA Problem In CAS-Based Structures

The ABA problem is a classic hazard of CAS-based structures: a thread reads value A, computes a new value, and CASes A to the new value; but between the read and the CAS, the value changed to B and back to A. The CAS succeeds (the value is A again), but the intermediate change is missed, corrupting the structure (a stack where a node was popped and re-pushed, but the CAS's "no change" assumption is violated).

- **Recognize when ABA can occur.** Any CAS on a value that can transition A→B→A is vulnerable — typically pointer-based structures where nodes are freed and reallocated to the same address.
- **Use versioned pointers (tagged pointers, generation counters).** Pair the pointer with a version that increments on each change, so the CAS fails if the value changed even back to the same pointer. This is the standard ABA defense.
- **Use double-word CAS where available.** Some platforms provide CAS on a double-word (pointer + version), enabling versioned pointers directly.
- **Epoch-based reclamation can also prevent ABA** by ensuring a freed node's address is not reused while any thread might CAS on it.

### Respect Memory Ordering And Visibility

Lock-free structures using atomics must use the correct memory ordering, or the visibility of writes across threads is not guaranteed, producing rare corruption. This is covered in depth by the memory-ordering-and-visibility skill; the relevant point here is that concurrent data structure design inherits that requirement.

- **Use the strongest ordering (sequentially consistent) unless profiling shows a weaker ordering is worth it.** Weaker orderings (acquire/release, relaxed) are faster but require precise reasoning about happens-before; getting them wrong produces rare, hard-to-debug corruption. Default to strong; weaken only with justification and review.
- **Do not treat `volatile` as atomic.** In some languages (notably C/C++ and Java before Java 5), `volatile` does not provide atomicity or the needed ordering for concurrent access. Use proper atomic types.

### Prefer Vetted Library Implementations Over Hand-Rolled Structures

Concurrent data structures are difficult to get right, and the defects are subtle (rare corruption, ABA, reclamation races) that testing often misses. The strong default is to use a vetted library implementation.

- **Use the language's concurrent collections (ConcurrentHashMap, sync.Map, concurrent queues, crossbeam, java.util.concurrent).** These are battle-tested, handle the reclamation and ordering hazards, and are almost always correct where a hand-rolled structure would not be.
- **Build a custom concurrent structure only when library options do not fit and the need is clear.** A custom structure is a significant correctness investment; justify it by a concrete requirement the libraries do not meet, and subject it to stress testing, fuzzing, and sanitizer-based verification.
- **If you must build one, base it on a well-understood algorithm with a published correctness argument.** Do not invent a novel concurrent structure; use a known algorithm (Michael-Scott queue, Harris-Michael list, skip list) whose correctness is established, and implement it faithfully.

## Common Traps

### Single Coarse Lock Becoming A Bottleneck

A single mutex around a contended structure, serializing all access and becoming the bottleneck under concurrency. Refine to fine-grained or lock-free only when profiling justifies it.

### Lock-Free Structure Without Memory Reclamation

A CAS-based structure with removal that frees nodes a concurrent reader may reference, causing use-after-free. Use epoch-based reclamation or hazard pointers (or a GC'd language).

### ABA In A CAS-Based Structure

A value transitioning A→B→A between a thread's read and CAS, causing the CAS to succeed incorrectly. Use versioned pointers or reclamation that prevents address reuse.

### Wrong Memory Ordering

Using relaxed or acquire/release atomics without correct happens-before reasoning, producing rare cross-thread corruption. Default to sequentially consistent; weaken only with justification and review.

### Hand-Rolled Structure Where A Library Would Do

Building a custom concurrent map or queue when a vetted library implementation exists and fits, introducing subtle correctness bugs the library does not have. Use the library.

### Inventing A Novel Concurrent Algorithm

Designing a new concurrent structure without a published correctness argument, rather than implementing a known-correct algorithm. Use well-understood algorithms.

### Read-Write Lock For A Write-Heavy Pattern

Using a read-write lock where writes are frequent, so readers and writers contend on the lock and the structure performs no better than a mutex. Match the strategy to the read/write ratio.

### Confusing Progress Guarantees

Assuming lock-free means wait-free (no starvation), or choosing wait-free complexity when lock-free would suffice. Know the guarantee each strategy provides and match it to the requirement.

## Self-Check

- [ ] The synchronization strategy (coarse locking, read-write lock, fine-grained/striped locking, copy-on-write, or lock-free/wait-free) is chosen to match the structure's access pattern, read/write ratio, contention level, and required progress guarantee — not reflexively to "lock-free" or to a single coarse lock.
- [ ] The progress guarantee (blocking, lock-free, wait-free) is understood and matched to the requirement — most structures do not need wait-free progress, and the complexity is not paid without justification.
- [ ] Lock-free structures with removal use a memory reclamation scheme (epoch-based reclamation or hazard pointers, or a GC'd language) so a node is never freed while a concurrent reader may reference it.
- [ ] CAS-based structures defend against ABA (versioned/tagged pointers, double-word CAS, or reclamation preventing address reuse), and the ABA risk has been identified for each CAS site.
- [ ] Memory ordering is correct: sequentially-consistent atomics by default, weakened to acquire/release or relaxed only with precise happens-before reasoning and review, and `volatile` is not used as a substitute for atomics.
- [ ] A vetted library implementation is used wherever it fits; a custom concurrent structure is built only when libraries do not meet a concrete need, and is based on a well-understood, published algorithm rather than a novel design.
- [ ] Custom concurrent structures are verified by stress testing (high concurrency, long duration), fuzzing, and sanitizer-based checking (ThreadSanitizer, AddressSanitizer) that detect the rare races and corruption testing misses.
- [ ] The structure's correctness has been reasoned about for the specific hazards of its strategy: deadlock (for lock-based, via ordering — see deadlock-prevention), reclamation races and ABA (for lock-free), visibility (memory ordering), and false sharing (cache-line padding for hot fields accessed across cores).
