---
name: allocation_patterns_and_arena.md
description: Use when the agent is choosing an allocator or allocation strategy, designing an arena, pool, or slab allocator, reasoning about fragmentation, alignment, or deallocation order, working under real-time or latency constraints, or evaluating whether a custom allocator is justified. Covers allocator selection and the system allocator's assumptions, arena and pool allocation for bulk lifetime management, external and internal fragmentation, alignment requirements and padding, deallocation order and its effect on fragmentation, real-time allocation constraints and deterministic allocation, the tradeoffs of custom allocators (complexity, debugging, accounting), and the discipline of measuring allocation behavior before adopting a custom allocator. Also use when allocation is a measured bottleneck, when memory grows unexpectedly, when latency spikes correlate with allocation or GC, or when a latency-sensitive path must avoid allocation.
---

# Allocation Patterns And Arena

Memory allocation is not free, and it is not uniform. The system allocator is a general-purpose compromise optimized for the average workload: it handles arbitrary sizes and lifetimes reasonably well, but it is not optimal for any specific pattern, and its costs — fragmentation, contention, latency variance, the bookkeeping of tracking many small allocations — become visible under specific conditions. The recurring failure is treating allocation as a transparent detail and reaching for a custom allocator only after a problem appears, or reaching for one prematurely because "custom allocators are faster." Both errors share a root: a decision made without measuring the actual allocation pattern. An arena that groups many short-lived allocations into one bulk region can eliminate per-allocation overhead and make deallocation O(1); but the same arena applied to long-lived objects wastes memory and complicates lifetimes. The right allocation strategy depends on the actual lifetime and size distribution of allocations, and that distribution must be measured, not guessed.

Agents tend to either ignore allocation entirely (the system allocator is fine, until it isn't) or over-engineer it (a custom allocator for code that does not allocate enough to matter). The defects live in the mismatch: an arena whose objects outlive the arena, causing use-after-free; a pool that returns objects without resetting their state, leaking data across users; fragmentation that grows memory far beyond the live set because deallocation order fights the allocator's layout; alignment violations on types requiring stricter guarantees than the allocator provides; a "real-time safe" path that allocates and triggers a non-deterministic allocator or a GC pause. The judgment problem is treating allocation as a measurable property of the workload — lifetime distribution, size distribution, fragmentation behavior, latency sensitivity — and choosing a strategy (system allocator, arena, pool, slab, region) that fits the measured pattern, with the complexity justified by measured benefit.

This skill is about choosing allocation strategies and designing custom allocators correctly. It complements the memory-leaks skill (lifetime correctness) and the GC-tuning skill (managed-runtime allocation); here the question is the allocation pattern itself and the allocator strategy that fits it.

## Core Rules

### Measure The Allocation Pattern Before Choosing A Strategy

The first question is what the workload's allocation actually looks like, because the right strategy depends on the lifetime and size distribution. Strategies chosen without measurement are guesses.

- **Profile allocations.** How many allocations per unit of work, of what sizes, with what lifetimes? A workload with many small, short-lived allocations benefits enormously from an arena; one with few large, long-lived allocations does not. Tools (allocation profilers, malloc statistics, custom counters) reveal the pattern.
- **Identify the dominant cost.** Is the cost per-allocation overhead (many small allocations), fragmentation (long-lived and short-lived objects interleaved), contention (multi-threaded allocation on a shared heap), or latency variance (the allocator's internal bookkeeping)? Each points to a different strategy.
- **Re-measure after a change.** An allocator that fits the current pattern may not fit after the workload changes; re-profile when the workload shifts.

### Match The Allocator To The Lifetime Pattern

Lifetime distribution is the primary determinant of allocator strategy. The system allocator handles arbitrary lifetimes at the cost of per-allocation overhead and fragmentation; specialized allocators trade generality for efficiency on specific lifetime patterns.

- **Arena (region) allocation for many same-lifetime objects.** An arena bumps a pointer through a large region, returning allocations in O(1) without per-object bookkeeping, and frees everything at once when the arena is destroyed. Ideal when many objects share a lifetime (e.g., per-request objects, a parse tree, a computation's intermediates) — allocation is cheap and deallocation is bulk. Unsuitable when objects must outlive the arena or have wildly different lifetimes.
- **Pool / slab allocation for many same-size objects with varying lifetimes.** A pool pre-allocates a slab of fixed-size slots and hands them out and returns them individually, avoiding per-allocation fragmentation for uniform-size objects (e.g., nodes in a data structure, connection objects). Ideal for high churn of same-size objects; the fixed size eliminates external fragmentation within the pool.
- **The system allocator for heterogeneous, long-lived, or sparse allocations.** Objects with diverse sizes and long or unpredictable lifetimes are what the system allocator is designed for; a custom allocator for these adds complexity without clear benefit.

### Manage Fragmentation Explicitly

Fragmentation is the gap between the memory a workload holds (the live set) and the memory the allocator reserves (the heap), and it can dwarf the live set. Two kinds matter:

- **External fragmentation** is free space scattered between live allocations, too small to satisfy new requests. It grows when allocation and deallocation orders interleave sizes, leaving holes. Arenas (bulk free) and pools (uniform size) avoid it; the system allocator manages it imperfectly.
- **Internal fragmentation** is memory wasted inside an allocation (padding to size classes, alignment rounding). Allocators round small allocations up to size classes, so many slightly-different small sizes waste memory; alignment requirements add padding.
- **Deallocation order matters.** A last-in-first-out deallocation pattern (stack-like) fragments least; a random or first-in-first-out pattern fragments more. Where you control deallocation order, favor LIFO; where you don't, fragmentation management falls to the allocator.
- **Measure fragmentation as heap-size minus live-set.** If the heap is several times the live set, fragmentation is significant; an arena or pool for the fragmented allocation class may recover the memory.

### Respect Alignment Requirements

Types and operations have alignment requirements — the address of an object must be a multiple of some power of two — and violating them causes misaligned accesses (slow on x86, traps or corruption on some architectures) or undefined behavior.

- **Know the alignment requirements of your types.** SIMD types, atomic operations, and some hardware interfaces require stricter alignment than the default; the allocator must provide it. A pool sized for one type may misalign another.
- **Account for alignment padding in sizing.** Padding to alignment is internal fragmentation; a 13-byte object with 16-byte alignment wastes 3 bytes per instance. For many instances, this adds up.
- **Ensure custom allocators honor alignment.** An arena's bump pointer must be rounded up to the required alignment before each allocation; a pool's slots must be aligned for the type they hold. A custom allocator that ignores alignment produces subtle corruption on aligned-access architectures.

### Handle Real-Time And Latency-Sensitive Constraints

In real-time or low-latency paths, allocation must be deterministic — bounded time, no non-deterministic allocator behavior, no GC pauses. Allocation in such paths is a common source of latency variance.

- **Avoid allocation on the hot or real-time path.** Pre-allocate what the path needs (into an arena or pool prepared beforehand) so the path uses memory without calling the allocator. The system allocator's latency is not bounded; a real-time path cannot rely on it.
- **Use bounded, deterministic allocators for the path.** An arena with bump-pointer allocation is O(1) and deterministic; a pool with a free list is O(1) if the free list is managed simply. These are suitable where the system allocator is not.
- **In managed runtimes, allocation can trigger GC.** Allocating on a latency-sensitive path in a GC'd language can pause for collection; avoid allocation, or tune the GC to bound pauses (see the GC-tuning skill).
- **Verify determinism by measurement.** A path claimed to be allocation-free or deterministic must be verified — allocation can hide in library calls, closures, boxing, or string operations that are not obviously allocations.

### Weigh Custom Allocator Complexity Against Measured Benefit

A custom allocator is complexity: it must be implemented, debugged (allocators are notoriously hard to get right, and their bugs corrupt memory in confusing ways), accounted for (tracking usage, bounds), and maintained. The benefit must justify the cost.

- **Justify with measurement.** Adopt a custom allocator when profiling shows allocation is a measured bottleneck (overhead, fragmentation, contention, or latency variance) and a specific strategy addresses it. Do not adopt one speculatively.
- **Prefer existing vetted allocators.** Mature arena/pool libraries and tunable system allocators (jemalloc, tcmalloc, mimalloc) cover most needs; use them before hand-rolling. A hand-rolled allocator should be rare.
- **Account for debugging cost.** Allocator bugs (use-after-free, double-free, arena-overrun) are among the hardest to diagnose; custom allocators should integrate with debugging tools (poisoning, guard pages, leak tracking) or they will be a liability.
- **Re-evaluate as the workload changes.** An allocator tuned for one pattern may hurt under another; if the workload shifts, re-measure and reconsider.

## Common Traps

### Custom Allocator Without Measured Justification

Hand-rolling an arena or pool because "custom allocators are faster," for code that does not allocate enough to matter, adding complexity and debugging liability without benefit. Measure first; adopt a custom allocator only when allocation is a measured bottleneck.

### Arena Objects Outliving The Arena

Objects allocated in an arena that must outlive the arena's destruction, causing use-after-free when the arena is freed. Match object lifetimes to the arena; use the system allocator or a pool for objects that outlive the arena's scope.

### Pool Returning Objects Without Resetting State

A pool handing out a recycled object whose fields still hold the previous user's data, leaking data across users or causing subtle logic bugs. Reset objects to a clean state before returning them from the pool.

### Fragmentation From Mismatched Deallocation Order

Long-lived and short-lived objects allocated interleaved into the same heap, fragmenting it so the heap is several times the live set. Separate allocation classes (arena/pool for the short-lived) or control deallocation order toward LIFO.

### Misaligned Allocation Crashing Or Corrupting

A custom allocator ignoring the alignment requirements of the types it serves, causing misaligned accesses (slow on x86, traps/corruption elsewhere). Honor alignment; round the bump pointer and align pool slots to the type's requirement.

### Allocation On A Real-Time Or Latency-Sensitive Path

Calling the system allocator (or triggering GC) on a path that requires bounded latency, introducing non-deterministic pauses. Pre-allocate into an arena or pool; verify the path is allocation-free by measurement.

### Assuming The System Allocator Is Always Fine

Treating allocation as transparent and never measuring, so fragmentation, contention, or latency variance from the system allocator goes unaddressed until it becomes a production problem. Profile allocation under realistic load.

### Hand-Rolling Instead Of Using A Vetted Allocator

Writing a custom allocator when a mature library (jemalloc, tcmalloc, mimalloc, or an arena/pool library) would serve, introducing bugs the library already solved. Use vetted allocators; hand-roll only with strong justification and debugging support.

## Self-Check

- [ ] The allocation pattern was profiled (allocation count, sizes, lifetimes) before choosing a strategy, and the dominant cost (per-allocation overhead, fragmentation, contention, latency variance) was identified rather than guessed.
- [ ] The allocator matches the lifetime pattern: arena for many same-lifetime objects, pool/slab for many same-size objects with varying lifetimes, system allocator for heterogeneous or long-lived allocations — and arena objects do not outlive the arena.
- [ ] Fragmentation is measured (heap size versus live set) and managed: allocation classes are separated (arena/pool for short-lived), deallocation order favors LIFO where controllable, and internal fragmentation from size classes and alignment padding is accounted for.
- [ ] Alignment requirements of all types are honored by any custom allocator (bump pointer rounded to alignment, pool slots aligned to the type), so no misaligned accesses occur on any target architecture.
- [ ] Real-time or latency-sensitive paths avoid allocation (pre-allocated into an arena or pool) or use bounded deterministic allocators, and the path is verified allocation-free by measurement (including hidden allocations in library calls, closures, boxing, strings).
- [ ] The custom allocator is justified by measured benefit, an existing vetted allocator (library or tunable system allocator) was considered first, and the custom allocator integrates debugging support (poisoning, guard pages, leak tracking) so its bugs are diagnosable.
- [ ] The allocator strategy is re-evaluated when the workload changes, since a strategy tuned for one pattern may hurt under another.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
