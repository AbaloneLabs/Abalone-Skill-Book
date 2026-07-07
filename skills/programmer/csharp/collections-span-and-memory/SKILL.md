---
name: csharp_collections_span_and_memory.md
description: Use when the agent is choosing or reviewing C# collection types (List, Dictionary, HashSet, ImmutableList, Array, ReadOnlyDictionary), using Span<T>, ReadOnlySpan<T>, Memory<T> and ReadOnlyMemory<T>, stackalloc, ArrayPool, ref struct constraints, avoiding allocations in hot paths, passing contiguous memory without copying, or diagnosing GC pressure, LINQ-to-Collections allocation churn, unnecessary array copies, or Span lifetime violations.
---

# C# Collections, Span, And Memory

C# collection and memory choices determine both correctness and performance. The BCL offers many collection types (`List<T>`, `Dictionary<K,V>`, `HashSet<T>`, arrays, `ImmutableList<T>`, `ConcurrentDictionary`), and since C# 7.2 it offers `Span<T>` and `Memory<T>` — stack-only and heap-safe views over contiguous memory that let you process buffers without copying. The judgment problem is that the right collection depends on access pattern (sequential, keyed, concurrent, immutable), and that `Span<T>` is powerful but constrained (it is a `ref struct`, cannot be boxed, cannot be stored in fields, cannot cross await) — misusing it produces compile errors or, worse, code that allocates where it meant to avoid allocation. Choosing the wrong collection or memory abstraction produces GC pressure, unnecessary copies, and concurrency bugs.

Agents tend to reach for `List<T>` for everything, to allocate arrays in hot paths where `ArrayPool` or `stackalloc` would avoid GC pressure, or to misuse `Span<T>` (storing it in a field, awaiting with it on the stack). The judgment problem is to choose the collection by access pattern and concurrency, to use `Span<T>`/`Memory<T>` to pass contiguous memory without copying, and to use pooling and stack allocation to reduce GC pressure in hot paths. This skill is about matching the collection and memory abstraction to the workload rather than defaulting to `List<T>` and `new[]`.

## Core Rules

### Choose The Collection By Access Pattern And Concurrency

Different collections are optimized for different access patterns. Choose by how the collection is used:

- **Sequential indexed access**: `List<T>` (resizable) or `T[]` (fixed, fastest). Use an array when the size is known and stable.
- **Keyed lookup**: `Dictionary<K,V>` (unordered, O(1) lookup) — not `List<T>.Find` (O(n)).
- **Set membership**: `HashSet<T>` (O(1) contains) — not `List<T>.Contains` (O(n)).
- **Sorted/ordered**: `SortedDictionary<K,V>`/`SortedList<K,V>` (ordered by key), or sort a `List<T>` when order is needed occasionally.
- **FIFO/LIFO**: `Queue<T>` / `Stack<T>`, not `List<T>` with index manipulation.
- **Immutable**: `ImmutableList<T>`, `ImmutableDictionary<K,V>` (structural sharing, safe for sharing across threads or as public API return values). Do not mutate after returning.
- **Concurrent**: `ConcurrentDictionary<K,V>` for multi-threaded read/write; do not use a plain `Dictionary` with manual locking (use the purpose-built type) or `ImmutableDictionary` for high-frequency writes (each write allocates).

The performance differences are large: a `List<T>.Contains` over 10,000 items is 10,000 comparisons; a `HashSet<T>.Contains` is one hash. Profile-guided choice matters, but the access pattern usually dictates the type up front.

### Use Span<T> To Pass Contiguous Memory Without Copying

`Span<T>` (and `ReadOnlySpan<T>`) is a stack-only view over contiguous memory — an array slice, a string's characters, a `stackalloc` buffer, native memory. It lets a function accept "some bytes" or "some chars" without forcing the caller to allocate an array or copy. For high-throughput parsing, text processing, and buffer manipulation, `Span<T>` is the key to avoiding allocations.

- Accept `ReadOnlySpan<byte>`/`ReadOnlySpan<char>` in parsing functions so callers can pass arrays, stackalloc buffers, or string slices without copying.
- Use `stackalloc` for small temporary buffers (a few hundred bytes) to avoid heap allocation: `Span<byte> buf = stackalloc byte[64];`.
- Slice with `span.Slice(start, length)` or `span[start..end]` — slicing is free (no copy).

`Span<T>` is a `ref struct`, which gives it stack-only semantics. This is what makes it safe (it cannot outlive the stack frame that owns the underlying memory), but it imposes constraints (see the trap below).

### Respect The ref struct Constraints On Span<T>

Because `Span<T>` is a `ref struct`, the compiler enforces several constraints to guarantee it cannot escape the stack or outlive its memory:

- It cannot be boxed (no converting to `object` or `dynamic`).
- It cannot be a field of a non-`ref struct` (so it cannot be stored in a class or regular struct).
- It cannot be captured in a lambda or local function that lifts it to the heap.
- It cannot be used across `await` (the async state machine boxes locals) or `yield` in an iterator.
- It cannot be a generic type argument (pre-C# 11; C# 11 relaxes this with `allows ref struct` constraints).

These constraints are the point — they prevent dangling spans. When you hit one, the answer is usually `Memory<T>` (see below) or restructuring. Do not fight the constraints with `Unsafe.As` or reflection; they exist for safety.

### Use Memory<T> When Memory Must Live On The Heap Or Cross Await

`Memory<T>` (and `ReadOnlyMemory<T>`) is the heap-safe counterpart of `Span<T>`: it can be stored in fields, captured in lambdas, and used across `await`. You convert `Memory<T>` to `Span<T>` via `.Span` when you need to do synchronous work, and the conversion is cheap. Use `Memory<T>` for async APIs that need to pass a buffer through awaits, and `Span<T>` for synchronous, stack-scoped work.

The pattern: an async method accepts `ReadOnlyMemory<byte>` and, at each synchronous step, gets a `ReadOnlySpan<byte>` via `.Span` to do the actual processing. This respects the `ref struct` constraints while allowing async flow.

### Pool Arrays With ArrayPool To Reduce GC Pressure In Hot Paths

`ArrayPool<T>.Shared.Rent(minSize)` rents an array from a shared pool (no allocation if one is available) and `ArrayPool<T>.Shared.Return(array)` returns it. For hot paths that need temporary arrays (a buffer per request, a scratch array per iteration), pooling eliminates the per-call allocation and the GC pressure that comes with it.

The contract: you must `Return` every rented array, ideally in a `try`/`finally` so it is returned even on exception. The rented array may be larger than requested (you get a length >= minSize), so track the logical length separately. Do not return an array twice, and do not use it after returning. Pooling is a significant win in high-throughput scenarios; it is unnecessary complexity for cold paths.

### Avoid LINQ Allocation Churn In Hot Paths

LINQ (`Where`, `Select`, `ToList`) is expressive but allocates: each operator may allocate an iterator, and `ToList`/`ToArray` allocate the result collection. In a hot path (per-request, per-frame, per-tick), LINQ chains create GC pressure. For hot paths, prefer `for`/`foreach` loops over the source, preallocated buffers, and `Span<T>`-based processing.

LINQ is fine for cold paths and for readability where allocation does not matter. The judgment is: profile, and where a hot path allocates, replace LINQ with explicit loops and pooled buffers. Do not prematurely optimize cold paths.

### Know When Collections Allocate And When They Do Not

Understanding allocation helps you write allocation-free hot paths:

- `foreach` over an array or `List<T>` uses a struct enumerator (no allocation); over `IEnumerable<T>` may box the enumerator (allocation).
- LINQ operators allocate iterators.
- `params` arrays allocate if the caller passes individual arguments.
- Lambda captures that close over locals allocate a closure object and may box.
- Boxing a value type (struct to `object`/interface) allocates.

In hot paths, prefer array/`List<T>` `foreach` (no allocation), avoid LINQ, avoid closure-allocating lambdas (use static lambdas or pass state explicitly), and avoid boxing. Measure with a profiler (PerfView, dotMemory) to find the actual allocations.

## Common Traps

### List<T>.Contains / Find For Lookup Instead Of HashSet/Dictionary

`list.Contains(item)` over a large list is O(n); `set.Contains` is O(1). Use the right collection for the access pattern.

### Storing Span<T> In A Field Or Across Await

`Span<T>` is a `ref struct` and cannot be stored in a field, captured in a lambda, or used across `await`. Use `Memory<T>` for heap/async scenarios.

### Allocating Arrays In A Hot Path Instead Of Pooling

`new byte[4096]` per request allocates and creates GC pressure; `ArrayPool<byte>.Shared.Rent(4096)` reuses. Pool temporary buffers in hot paths.

### LINQ Chain In A Per-Request Hot Path

`items.Where(...).Select(...).ToList()` allocates iterators and a list every call. Replace with a `for` loop and a preallocated buffer in hot paths.

### Not Returning A Rented Array (Or Using It After Return)

`ArrayPool` arrays must be returned in a `finally`; forgetting leaks the pool slot, and using after return corrupts state. Always wrap rent/return in `try`/`finally`.

### Concurrent Access To A Plain Dictionary

A `Dictionary<K,V>` is not thread-safe for concurrent writes; use `ConcurrentDictionary<K,V>`. A plain dictionary under multi-threaded write can corrupt or infinite-loop.

### Boxing A Struct Via Interface Or object

`struct` boxed into `interface` or `object` allocates and loses copy semantics; mutating the boxed copy does not affect the original. Avoid boxing value types in collections of `object`/non-generic interfaces.

### Mutable List Returned From A Public API

Returning the internal `List<T>` exposes it for mutation by callers. Return it as `IReadOnlyList<T>` or copy, or use `ImmutableList<T>` if immutability is the contract.

## Self-Check

- [ ] The collection type matches the access pattern (Dictionary/HashSet for keyed/set lookup, Queue/Stack for FIFO/LIFO, immutable/concurrent variants for those semantics), not `List<T>` for everything.
- [ ] `Span<T>`/`ReadOnlySpan<T>` are used for synchronous, stack-scoped contiguous-memory processing without copying, and `stackalloc` is used for small temporary buffers.
- [ ] The `ref struct` constraints on `Span<T>` (no boxing, no fields, no lambda capture, no await) are respected, and `Memory<T>` is used where the buffer must live on the heap or cross `await`.
- [ ] Hot paths use `ArrayPool<T>` for temporary arrays (with `try`/`finally` return), avoiding per-call allocation and GC pressure.
- [ ] LINQ is avoided in hot paths in favor of explicit loops and preallocated buffers; LINQ is retained for cold paths where readability matters more than allocation.
- [ ] Concurrent collections (`ConcurrentDictionary`) are used for multi-threaded access, not plain collections with manual locking; public APIs return immutable or read-only views of internal collections.
- [ ] Allocation sources in hot paths (LINQ iterators, closure-allocating lambdas, boxing, `params`) have been identified by profiling and eliminated where they matter.
- [ ] Rented arrays are always returned (in `finally`), not used after return, and the logical length is tracked separately from the (possibly larger) rented array.
