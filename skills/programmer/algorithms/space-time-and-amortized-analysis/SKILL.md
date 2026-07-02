---
name: space_time_and_amortized_analysis.md
description: Use when the agent is analyzing space complexity and memory hierarchy effects, reasoning about amortized cost of dynamic arrays and hash tables, evaluating worst-case spikes versus amortized averages in latency-sensitive paths, pre-sizing containers, or comparing theoretical complexity to real measured performance across memory hierarchies.
---

# Space, Time, and Amortized Analysis

Amortized analysis and space reasoning are where theoretically clean algorithms meet the messy reality of hardware. A dynamic array's `push` is O(1) amortized, but the rare resize is O(n)—and if that resize lands on a latency-sensitive request, the amortized bound does not save you. A hash table is O(1) average lookup, but the constant factor of hashing, probing, and cache misses means a linear scan of a small array is faster in practice. Memory hierarchy effects—L1 versus L2 versus RAM versus disk—can make an asymptotically worse algorithm dramatically faster because it respects locality. The skill is reasoning about these real costs, not the simplified notation, and knowing when amortized or space bounds are safe to rely on and when they betray you.

The judgment problem is understanding when amortized bounds are acceptable versus when the spike breaks a deadline, how memory hierarchy and allocation behavior overturn asymptotic verdicts, and how to analyze specific common structures (dynamic arrays, hash tables) for their real cost profile. The agent should not quote "O(1) amortized" as if it answered the performance question; it should know what that bound hides.

This skill applies whenever you are analyzing the real performance of common data structures, reasoning about memory use or allocation, or deciding whether an amortized bound is safe for a latency-sensitive path.

## Core Rules

### Understand what amortized analysis promises and what it hides

Amortized analysis averages the total cost of a sequence of operations across all operations. A dynamic array's `push` is O(1) amortized because the expensive resizes (which copy all elements) happen rarely enough that the per-operation average is constant. This is a valid and useful bound, but it hides two things:

- **The spike**: individual operations can be much more expensive than the amortized average. A resize is O(n) even though the amortized push is O(1).
- **The timing**: the spike lands on whichever request happens to trigger the resize, not spread evenly.

Amortized bounds are safe when the average matters and occasional spikes are tolerable (batch processing, throughput-oriented paths). They are unsafe when any single operation must meet a deadline (real-time, latency-sensitive per-request paths), because the spike can violate the deadline regardless of the favorable average.

### Distinguish aggregate, accounting, and potential methods at a practical level

You rarely need formal amortized analysis, but the intuition matters:

- **Aggregate method**: sum the total cost over n operations, divide by n. Tells you the average but nothing about distribution.
- **Accounting (banker's) method**: charge each operation a bit extra ("tokens") and use the surplus to pay for the rare expensive operation. The insight is that cheap operations build up credit that funds the spike.
- **Potential (physicist's) method**: define a potential function over the data structure's state; the amortized cost is actual cost plus change in potential. Resizing the array increases potential (room for more cheap pushes).

The practical takeaway: amortized bounds rely on the structure accumulating "credit" during cheap operations. If you reset or destroy the structure frequently (so it never accumulates credit), or if you hit the expensive operation early, the amortized bound can fail to materialize.

### Analyze dynamic arrays for their real cost profile

Dynamic arrays (Vec, ArrayList, std::vector) are the canonical amortized structure:

- **Growth factor**: doubling capacity on resize gives O(1) amortized push. Growth factors less than ~1.5 increase the amortized cost; the geometric series must sum to a constant multiple of n.
- **The resize spike**: when capacity is exceeded, a resize allocates a new buffer and copies all elements. For n elements, this is O(n) on the triggering push.
- **Pre-sizing eliminates the spike**: if you know the final size, pre-allocate (`with_capacity`, `reserve`). This converts the amortized bound into a true bound with no spike, which is essential for latency-sensitive paths.
- **Shrinking**: arrays that grow then shrink retain their high-water-mark capacity, wasting memory. Some implementations shrink on demand; decide whether to trim based on memory pressure.

Weak choice: relying on amortized push in a latency-sensitive path and letting resizes cause occasional latency spikes. Strong choice: pre-sizing the array when the size is known or bounded.

### Analyze hash tables for their real cost profile

Hash tables (HashMap, dict, unordered_map) have excellent average bounds but large constants and pathological worst cases:

- **Average O(1) lookup/insert**: true for well-distributed keys, but the constant includes hashing the key, probing/chaining, and a cache miss (hash table access patterns defeat prefetchers).
- **Resize/rehash spike**: like dynamic arrays, hash tables resize when the load factor exceeds a threshold, rehashing all entries. This is an O(n) spike on the triggering operation.
- **Worst case O(n)**: if many keys collide (adversarially or by bad luck), every operation degrades to O(n). For untrusted keys, use a keyed/randomized hash to resist collision attacks.
- **Small-n defeat**: for small collections, a linear scan of an array is often faster than a hash table, because the array is cache-friendly and has no hashing overhead.

Pre-size hash tables when the size is known, and prefer arrays for small bounded collections where the hash table's constant factor dominates.

### Reason about the memory hierarchy, not just abstract space

Real performance is governed by the memory hierarchy: registers and L1 cache are fast (nanoseconds), L2/L3 slower, main memory slower still, and disk/network orders of magnitude slower. An algorithm's asymptotic cost matters less than whether it respects locality:

- **Cache locality**: a contiguous array scanned linearly touches memory the prefetcher can predict, so an O(n) scan can outperform an O(log n) tree that chases pointers across the heap (each pointer chase is a potential cache miss).
- **Working set size**: if the active data fits in cache, access is fast; if it exceeds cache and spills to main memory, every access pays the RAM latency. An algorithm with a smaller working set can be faster than one with better asymptotics but a larger footprint.
- **Allocation cost**: per-element allocation (via pointers/nodes) defeats locality and adds allocator overhead. Contiguous, in-place structures are cache-friendlier.

When two approaches differ by a small asymptotic factor, the one with better locality and smaller working set usually wins in practice.

### Distinguish theoretical complexity from measured performance

Complexity notation guides the search; measurement settles the choice. Common divergences:

- An O(n) array scan beating an O(1) hash lookup for small n, due to constants and locality.
- An O(n log n) sort beating an O(n) radix sort for moderate n, due to radix's larger constant.
- An O(log n) tree lookup being slower than an O(n) scan because of pointer-chasing cache misses.

Measure on representative data and hardware before trusting the asymptotic verdict, especially when the gap is small (a constant factor or a single log factor).

### Decide when to accept amortized bounds versus require worst-case bounds

The decision rule:

- **Throughput-oriented, spike-tolerant paths** (batch jobs, background workers, analytics): amortized bounds are fine. Optimize for total work.
- **Latency-sensitive, deadline-bound paths** (per-request handlers, real-time systems, UI frames): require worst-case bounds or eliminate spikes via pre-sizing. The amortized average is irrelevant if the spike misses the deadline.
- **Memory-constrained paths**: favor in-place or streaming algorithms with bounded space over those that precompute large auxiliary structures.

State which regime the path is in, and choose bounds accordingly.

## Common Traps

### Quoting amortized O(1) for a latency-sensitive path

The amortized bound hides the resize/rehash spike, which can violate a per-request deadline. In latency-sensitive paths, pre-size or use a structure with bounded worst-case operations.

### Ignoring the resize spike in dynamic arrays and hash tables

The resize is O(n) and lands on an arbitrary request. Without pre-sizing, occasional requests pay the full copy/rehash cost, causing tail-latency spikes.

### Preferring a hash table over an array for small collections

For small n, the hash table's constant factor (hashing, probing, cache misses) exceeds a linear scan of a cache-friendly array. Use arrays for small bounded collections.

### Trusting asymptotics over locality

Choosing a pointer-chasing tree (O(log n)) over a contiguous array scan (O(n)) because the asymptotics look better, then losing to cache misses. Memory hierarchy often overturns small asymptotic gaps.

### Resetting an amortized structure before it accumulates credit

Amortized bounds rely on cheap operations funding expensive ones. If a structure is frequently destroyed and rebuilt, it never accumulates credit, and the amortized bound may not hold.

### Forgetting space in time-optimized code

Optimizing for time with large precomputed structures or caches, then exhausting memory, triggering GC pressure, or spilling the working set out of cache and becoming slower anyway.

### Not pre-sizing when the size is known

Letting a container resize repeatedly when the final size is known or bounded, paying avoidable spike costs. Pre-size whenever the size is predictable.

### Assuming the worst case cannot happen for hash tables

Relying on average O(1) for a table keyed by untrusted data, then hitting O(n) under collision attacks or bad key distributions. Use keyed hashing for untrusted keys.

## Self-Check

- For latency-sensitive paths, have you either pre-sized containers to eliminate resize spikes or chosen structures with bounded worst-case operations, rather than relying on amortized bounds?
- For dynamic arrays and hash tables, is the resize/rehash spike accounted for, and are containers pre-sized when the size is known or bounded?
- For small bounded collections, have you considered a cache-friendly array over a hash table, given the hash table's constant factor?
- Have you reasoned about memory hierarchy effects (cache locality, working-set size, allocation patterns) rather than asymptotics alone, especially when approaches differ by a small factor?
- Is the space cost stated alongside the time cost, and does the approach avoid exhausting memory or spilling the working set out of cache?
- For hash tables keyed by untrusted data, is a keyed/randomized hash used to resist collision attacks?
- Have you measured performance on representative data and hardware, rather than trusting the asymptotic verdict for close calls?
- Is it clear whether the path is throughput-oriented (amortized bounds acceptable) or latency/deadline-bound (worst-case bounds required), and is the choice justified by that regime?
- For amortized structures, have you confirmed they are not being reset so frequently that they never accumulate the credit the bound relies on?
- Have you confirmed the chosen structure's real constant factor (hashing, probing, pointer-chasing) does not overturn the asymptotic advantage for the actual input sizes?
