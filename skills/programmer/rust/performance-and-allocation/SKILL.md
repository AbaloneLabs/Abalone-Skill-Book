---
name: rust_performance_and_allocation.md
description: Use when the agent is optimizing Rust code for speed or memory, reasoning about heap vs stack allocation (Box, String, Vec, vs fixed-size types), avoiding unnecessary clones, choosing between Vec and slice/iterator chains, diagnosing slow builds or slow runtime, using cargo bench and profiling, deciding whether to inline or generalize a hot loop, reasoning about cache locality and false sharing, or reviewing code where "zero-cost abstraction" silently became costly. Covers allocation awareness, clone cost, iterator vs manual loop tradeoffs, cache locality, benchmarking discipline, and the tradeoff between premature optimization and genuine hot-path work.
---

# Performance And Allocation

Rust's promise of "zero-cost abstractions" is real but not automatic. Abstractions are zero-cost only when used as intended; the same language lets you write code that allocates on every iteration, clones large structures unnecessarily, or defeats the CPU cache. The judgment problem is knowing where performance actually matters, what the real costs of common patterns are, and how to measure before optimizing so you fix genuine bottlenecks rather than imagined ones.

Agents tend to either optimize prematurely (adding unsafe, hand-rolling loops, avoiding idioms for no measurable gain) or ignore allocation entirely (cloning in hot loops, collecting intermediate Vecs, boxing what could be stack-allocated). The harm appears as code that is harder to read for no benefit, or code that is idiomatic but slow because every call allocates. The real work is profiling to find the real bottleneck, understanding the allocation behavior of the types you use, and optimizing only the paths that measurement shows matter.

## Core Rules

### Measure Before Optimizing, And Measure Correctly

Performance intuition is frequently wrong. The only reliable way to find a bottleneck is to profile or benchmark the actual workload.

- Use `cargo bench` (with criterion or built-in `#[bench]`) for microbenchmarks of hot functions, with statistical rigor and warmup.
- Use a profiler (perf, flamegraph, dtrace, sampling profilers) on realistic workloads to see where time actually goes, including in code you did not suspect.
- Benchmark the change against the baseline; many "optimizations" make things slower or no different. A single timed run is not a measurement — use repeated runs and look at variance.

Optimize the path the profiler highlights, not the path you assume is slow. A loop that looks expensive may be negligible next to an allocation you overlooked.

### Know Which Types Allocate, And Where

Allocation (heap allocation) is far more expensive than stack work because it involves the allocator, cache misses, and potential fragmentation. Knowing which operations allocate is foundational.

- **Stack / no allocation**: fixed-size types (`i64`, `[T; N]`, tuples of fixed types), references (`&T`, `&mut T`), enums of fixed-size variants, small structs by value.
- **Heap allocation**: `Box<T>`, `String` and `Vec<T>` (the buffer is heap-allocated), `HashMap`/`BTreeMap`, `Rc`/`Arc`, and any type containing these.
- **Growing reallocates**: pushing to a `Vec` past its capacity copies the entire buffer. Pre-sizing with `Vec::with_capacity` or `reserve` avoids repeated reallocation when the size is predictable.
- **`to_string()` and `format!` allocate** a new `String`. In a hot loop, prefer writing into an existing buffer (`write!` into a `String` or `BufWriter`) over creating new strings.

The goal is not to avoid all allocation — heap allocation is correct and necessary for dynamically sized data — but to avoid *unnecessary* allocation in hot paths.

### Eliminate Unnecessary `clone` On Hot Paths

`clone` is explicit in Rust, which is a strength: every clone is visible. But visible does not mean free. Cloning a `Vec` or `String` copies the entire buffer; cloning an `Arc` is cheap (refcount bump) but cloning the contents it points to is not.

- On hot paths, prefer borrowing (`&[T]`, `&str`) over owning (`Vec<T>`, `String`) when the callee does not need ownership.
- Use `Cow<'a, str>` or `Cow<'a, [T]>` when a function sometimes needs to allocate and sometimes can borrow.
- Re-examine each `clone` in a hot loop: can the original be moved, borrowed, or reused instead?

Do not remove clones that are actually needed (e.g., when sending data to another thread requires ownership); remove clones that exist only because the API was not designed to borrow.

### Prefer Iterators And Slices Over Intermediate Collections

Idiomatic iterator chains (`iter().map().filter().collect()`) are zero-cost when they fuse into a single loop. The cost appears when you `collect()` into a `Vec` only to iterate it again.

- Chain adapters without collecting in between: `iter().map(f).filter(g).sum()` is one loop, no allocation.
- `collect()` only when you genuinely need an owned collection (to return it, to iterate multiple times, to index randomly).
- Passing `&[T]` (a slice) instead of `Vec<T>` to a function that only reads avoids requiring the caller to own or clone.

A common anti-pattern is `let v: Vec<_> = iter.collect(); for x in &v { ... }` where the `Vec` is never needed after the loop. Iterate directly.

### Respect Cache Locality And Data Layout

Modern CPUs are fast at sequential access to compact data and slow at pointer-chasing through scattered allocations. Data layout matters as much as algorithmic complexity for real workloads.

- A `Vec<T>` of plain structs is cache-friendly; a `Vec<Box<T>>` or a tree of boxed nodes pointer-chases and stalls.
- Struct of arrays (SoA) can outperform array of structs (AoS) when a hot loop touches only one field, because it improves locality and vectorization.
- False sharing occurs when independent data used by different threads sits in the same cache line; padding or reordering can fix it, but only measure confirms it matters.

These optimizations are advanced and should follow, not precede, profiling. They can make code harder to read, so reserve them for measured hot spots.

### Generalize Hot Paths With Care

Generics and `#[inline]` can help or hurt. Small generic functions called in hot loops benefit from inlining and monomorphization. But overly generic APIs (many type parameters, deep trait bounds) can bloat compile times and binary size through excessive monomorphization.

- Mark genuinely small, hot functions `#[inline]` so cross-crate callers can inline them.
- Avoid `#[inline]` on large functions; it bloats call sites for little gain.
- Consider `dyn Trait` (dynamic dispatch) when you have many concrete types and monomorphization is exploding binary size, accepting the vtable indirection.

### Build Time Is A Performance Concern Too

Compile time affects developer velocity and CI cost. Heavy generics, large dependency trees, and proc-macros slow builds. Sometimes the right performance decision is to simplify the type machinery or drop a heavy dependency, trading a small runtime difference for a large build-time improvement.

## Common Traps

### Premature Optimization Without Measurement

Hand-rolled loops, `unsafe`, and avoided idioms added "for speed" without a benchmark usually make code worse and slower. Profile first.

### Cloning In A Hot Loop

`items.clone()` inside a loop that runs millions of times copies megabytes per iteration. Borrow, move, or restructure to avoid it.

### Collecting Then Re-Iterating

Allocating a `Vec` just to loop over it once wastes memory and time. Chain iterators directly.

### Assuming `clone` On `Arc` Is Expensive

`Arc::clone` is a refcount bump and is cheap; over-optimizing it (e.g., passing `&Arc` everywhere) adds lifetime complexity for no gain. Conversely, cloning the `T` inside an `Arc` is expensive — know which you are doing.

### Pointer-Chasing Data Layout

A structure of boxed nodes defeats the cache. Where a hot loop traverses data, prefer contiguous storage (`Vec`, arrays).

### Optimizing The Wrong Function

Intuition about which function is slow is often wrong. Without a profiler, you may micro-optimize a function that is 1% of runtime while the real 60% bottleneck is untouched.

### Ignoring Reallocation

Growing a `Vec` in a loop without `with_capacity` triggers log-many reallocations and copies. Pre-size when the bound is known.

## Self-Check

- [ ] Optimization was driven by a profiler or benchmark on a realistic workload, not by intuition about which code is slow.
- [ ] Each `clone` on a hot path was examined and kept only if ownership genuinely requires it; borrowing, moving, or `Cow` used where possible.
- [ ] Heap-allocating types (`Vec`, `String`, `Box`) are used where dynamically sized data is needed, but unnecessary allocations in hot paths are removed.
- [ ] Iterator chains are fused without intermediate `collect()` unless an owned collection is genuinely needed.
- [ ] Collections are pre-sized (`with_capacity`/`reserve`) when the size is predictable, avoiding repeated reallocation.
- [ ] Data layout in measured hot spots favors cache locality (contiguous storage, struct-of-arrays where a single field dominates).
- [ ] `#[inline]` is applied only to small hot functions, not large ones; monomorphization bloat is monitored.
- [ ] A baseline benchmark exists for the optimized path, confirming the change improved measured performance.
- [ ] Build-time impact of heavy generics and dependencies was considered, not just runtime.
