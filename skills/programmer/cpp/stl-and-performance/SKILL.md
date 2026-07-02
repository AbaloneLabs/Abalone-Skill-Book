---
name: cpp_stl_and_performance.md
description: Use when the agent is writing or reviewing C++ code that uses STL containers and algorithms, choosing between vector, deque, list, map, unordered_map, set, array, or string, reasoning about iterator invalidation, algorithmic complexity guarantees, allocator selection, small string optimization, cache locality, element placement and ordering, or diagnosing performance problems caused by container choice, unnecessary allocations, copies, or poor algorithm selection in standard library code.
---

# STL And Performance In C++

The C++ Standard Template Library provides containers, algorithms, and iterators that are correct, generic, and — when used as intended — as fast as hand-written code, because templates let the compiler inline and specialize. The performance catch is that the STL is a toolkit of containers and algorithms with sharply different complexity and memory characteristics, and the "obvious" choice is frequently the wrong one. A `std::list` used where a `std::vector` would do is an order of magnitude slower due to allocation overhead and cache misses; a `std::map` used for lookup where a `std::unordered_map` or a sorted `std::vector` would do turns an O(1) lookup into an O(log n) tree walk with pointer chasing; an element inserted in the middle of a `std::vector` shifts every subsequent element; an iterator captured before a reallocation dangles. The judgment problem is to choose the container and algorithm whose complexity and memory behavior match the actual access pattern, to know what invalidates iterators, and to avoid the copies and allocations that the STL makes easy to write and hard to notice.

Agents tend to pick containers by familiarity (`std::vector` for everything, or `std::map` because "I need a map") and to call algorithms without considering their complexity. The harm is code that is correct and slow in ways that scale badly: a loop that copies a vector each iteration, a `std::map` of a million entries doing cache-missing tree walks, an `std::find` on a list where a hash set would be O(1). This skill is about choosing STL components by their measured behavior on the actual workload, respecting iterator invalidation rules, and recognizing the patterns that turn "just use the STL" into a performance problem.

## Core Rules

### Choose Containers By Access Pattern And Complexity, Not By Familiarity

Each STL container has a complexity profile for insertion, deletion, lookup, and iteration, plus a memory layout that affects cache behavior. The right container is the one whose dominant operations are cheap for your workload.

- **`std::vector`** is the default. Contiguous storage, O(1) random access and push_back (amortized), O(n) middle insert/erase. Cache-friendly (contiguous memory), minimal per-element overhead. Use unless you have a specific reason not to. Its weakness is middle insert/erase and the need for a contiguous block (reallocation copies on growth).
- **`std::deque`** for fast push/pop at both ends with random access; slightly higher overhead than vector, non-contiguous.
- **`std::list`/`std::forward_list`** for O(1) insert/erase anywhere you already have an iterator, and frequent splicing. Almost always the wrong choice otherwise: per-node allocation, poor cache locality, no random access. "I insert in the middle a lot" usually means you wanted a vector plus an index, or you are solving the wrong problem.
- **`std::map`/`std::set`** (ordered, tree-based) for ordered iteration and O(log n) lookup. Use when you need ordering; otherwise prefer `unordered_map`.
- **`std::unordered_map`/`std::unordered_set`** (hash-based) for O(1) average lookup. Use for lookup-dominated workloads without ordering needs. Mind the hash function quality, rehashing cost, and that pointers/ints hash to themselves by default (fine for keys, but a custom type needs a hash).
- **`std::array`** for a fixed-size, stack-allocated array with STL interface; no allocation overhead.
- **`std::string`/`std::string_view`**: use `string_view` for non-owning string parameters to avoid copies; do not let a `string_view` outlive its source.

Strong choice: `std::vector<Order>` with index-based access for an order book. Weak choice: `std::list<Order>` "because I insert and erase," paying per-node allocation and cache misses for an operation that a vector handles faster in practice.

### Respect Iterator Invalidation Rules For Each Container

Operations that insert, erase, or reallocate may invalidate iterators, pointers, and references into a container. The rules differ per container, and using an invalidated iterator is undefined behavior.

- **`std::vector`**: reallocation (on growth past capacity) invalidates all iterators, pointers, and references. `reserve` to avoid reallocation during a known growth phase. `erase` and `insert` invalidate iterators at and after the point of modification.
- **`std::deque`**: any insertion or deletion at either end invalidates all iterators (but not pointers/references to elements); middle operations invalidate everything.
- **`std::list`/`std::map`/`std::set`**: insertion does not invalidate iterators; erase invalidates only the erased element's iterator. This is the property that makes them safe for "modify while iterating," but it does not justify choosing them over a vector for performance.
- **`std::unordered_map`**: rehashing (on growth) invalidates iterators but not pointers/references to elements; a `reserve` avoids rehashing during a known insertion phase.

When modifying a container while iterating, use the container's documented pattern (`it = vec.erase(it);` for vector/deque; safe for list/map). Do not capture an iterator, modify the container, and use the iterator.

### Prefer Algorithms With The Right Complexity, And Reserve Bulk Operations

The `<algorithm>` header provides `std::sort`, `std::find`, `std::transform`, `std::accumulate`, `std::copy`, etc., each with a documented complexity. Choose the algorithm whose complexity matches the need, and prefer the bulk forms over hand-written loops (they are often specialized and vectorized).

- Use `std::find` (O(n)) for unsorted, `std::binary_search`/`std::lower_bound` (O(log n)) for sorted, and `std::unordered_set`/`unordered_map` (O(1)) for repeated lookup. A loop calling `std::find` on a vector N times is O(n^2); build a set once.
- `std::sort` is O(n log n); `std::stable_sort` preserves equal-element order at a higher cost; `std::partial_sort` for "top k." Choose the sort that matches the need.
- `std::transform`, `std::copy`, `std::move` (the algorithm, on ranges) are clearer and often faster than hand-written loops because the compiler can vectorize them.
- Reserve capacity before bulk insertion (`vec.reserve(n)` then a loop of `push_back`) to avoid repeated reallocation. For unordered containers, `reserve(n)` plus `max_load_factor` tuning avoids rehashing.

### Avoid Unnecessary Copies Of Container Elements

The STL makes copying easy, and copies of non-trivial elements (strings, vectors, objects with heap allocations) are expensive. Use moves and references to avoid them.

- Range-for over a container of non-trivial types: `for (const auto& e : vec)` (reference) rather than `for (auto e : vec)` (copy of each element). The copy version silently duplicates every element per iteration.
- `emplace_back(args...)` constructs an element in place from constructor arguments, avoiding a temporary and a move/copy. Prefer `emplace_back` over `push_back(Temporary{...})` where the arguments construct directly.
- Pass containers by `const std::vector<T>&` or `std::span<T>` (C++20) for read access; avoid passing by value unless you intend to take a copy. `std::span` is the non-owning view for contiguous data, like `string_view` for bytes.
- Move elements out when transferring: `sink(std::move(vec[i]))` or `vec2.push_back(std::move(vec[i]))`.

### Understand Small String Optimization And Custom Allocators

`std::string` (and some other small-buffer types) use Small String Optimization (SSO): short strings are stored inline in the string object without heap allocation, so short strings are cheap to construct, copy, and move. This means `sizeof(std::string)` is larger than a pointer (typically 16-32 bytes), and short-string operations avoid the allocator entirely.

- SSO makes short-string-heavy code fast; do not assume every `std::string` allocation is expensive. Conversely, long strings allocate, and copying a long string is a heap allocation.
- For allocation-controlled contexts (embedded, real-time, arenas), use a custom allocator (`std::vector<T, MyAlloc>`, polymorphic memory resources in C++17) to control where memory comes from. A custom allocator can dramatically reduce allocation overhead for many small allocations.
- Be aware that SSO boundary and allocator behavior differ across standard-library implementations (libstdc++, libc++, MSVC); do not rely on exact SSO thresholds.

### Reason About Cache Locality When Choosing Data Structures

Modern CPUs are far faster than memory; cache locality (accessing memory that is already in cache) dominates performance for data-structure-heavy code. Contiguous structures (`std::vector`, `std::array`) are cache-friendly; pointer-chasing structures (`std::list`, `std::map`, `std::unordered_map`) cause cache misses on every hop.

- A `std::vector` of values, iterated end-to-end, is dramatically faster than a `std::list` of the same values, because the vector's contiguous memory streams through cache while the list's per-node pointers miss cache on each element. The O(n) "scan the vector" often beats the O(log n) "walk the tree" for moderate n because of cache.
- A `std::vector<std::unique_ptr<T>>` (vector of pointers) loses locality compared to `std::vector<T>` (vector of values), because the pointers scatter the `T`s across the heap. Prefer values in the vector where `T` is small and movable.
- For lookup-heavy workloads on a static set, a sorted `std::vector` with `std::binary_search`/`std::lower_bound` often beats `std::unordered_map` due to cache locality and lower overhead, despite the same or worse big-O.

### Use spans, string_views, And Ranges For Zero-Coverhead Views

C++17 `std::string_view` and C++20 `std::span` are non-owning views over existing data; C++20 ranges let you compose algorithm pipelines lazily. They avoid copies but introduce lifetime concerns.

- `std::string_view` for a function parameter that only reads a string avoids a copy whether the caller passes a `std::string` or a C string. But a `string_view` does not own; storing one past the source's lifetime dangles.
- `std::span<T>` is the equivalent for a contiguous sequence of `T` (a view over a `vector`, array, or raw buffer); use it for function parameters instead of `(T*, size)`.
- C++20 ranges (`std::views::filter | std::views::transform`) compose lazily; the pipeline executes on iteration. Useful for readability, but beware that complex range pipelines can be harder to debug and may hide O(n) work behind a readable syntax.

## Common Traps

### std::list Where std::vector Would Be Faster

Choosing `std::list` "because I insert and erase in the middle," then paying per-node allocation and cache misses that make it slower than a vector plus index shift for almost all realistic sizes. Default to vector; use list only when O(1) splice or stable iterators are genuinely needed.

### std::map For Lookup Where unordered_map Or Sorted Vector Would Do

Using `std::map` for lookup-dominated code without needing ordering, paying O(log n) tree walks with pointer chasing. Use `std::unordered_map` for O(1) lookup, or a sorted `std::vector` with binary search for cache-friendly lookup on a static set.

### Iterator Used After Reallocation

Capturing `it = vec.begin()`, then `push_back`-ing until reallocation, then using `it` (dangling). Reserve capacity, or re-fetch iterators after growth.

### Copying Every Element In A Range-For

`for (auto e : vec_of_strings)` copies each string per iteration; `for (const auto& e : vec)` does not. The copy version compiles and runs, just slowly and with allocation churn.

### Modifying A Container While Iterating Incorrectly

`for (auto it = vec.begin(); it != vec.end(); ++it) { if (cond) vec.erase(it); }` invalidates `it` and skips elements. Use `it = vec.erase(it);` (and don't increment in that case), or the erase-remove idiom (`vec.erase(std::remove_if(...), vec.end())`).

### Unordered Container With A Bad Or Default Hash

A custom key type with the default hash (identity for pointers/ints) or a poor hash function clusters entries and degrades lookup toward O(n). Provide a good hash, and for custom types specialize `std::hash`.

### vector Of unique_ptr Losing Cache Locality

`std::vector<std::unique_ptr<T>>` scatters the `T`s across the heap, causing a cache miss per element on iteration. Use `std::vector<T>` (values) where `T` is small and movable.

### string_view Or span Outliving Its Source

Storing a `string_view` or `span` in a struct that outlives the source container, then using it after the source is destroyed. Views do not own; their lifetime is bounded by the source.

## Self-Check

- [ ] Each container was chosen against the access pattern and complexity needs (vector as default, list only for genuine O(1) splice/stable-iterator needs, unordered_map for lookup, sorted vector for cache-friendly static lookup), not by familiarity.
- [ ] Iterator invalidation rules for each container are respected; no iterator, pointer, or reference is used after an operation that invalidates it, and capacity is reserved before known growth phases.
- [ ] Algorithms match the needed complexity (binary search on sorted, hash lookup for repeated membership, the right sort variant), and bulk operations are preferred over hand-written loops.
- [ ] Elements are not copied unnecessarily (range-for uses references, `emplace_back` constructs in place, containers passed by const-ref or span/string_view, elements moved when transferred).
- [ ] Custom allocators are used where allocation control matters, and SSO behavior is understood rather than assumed consistent across library implementations.
- [ ] Cache locality is a factor in container choice (values over pointers in vectors, contiguous structures over pointer-chasing structures where the workload is iteration-heavy).
- [ ] `std::string_view`, `std::span`, and ranges are used for zero-copy views with their lifetime constraints respected (no view outlives its source).
- [ ] No O(n^2) pattern lurks behind repeated linear scans or lookups on a vector/list where a set/map would be O(1) or O(log n).
- [ ] Profiling on representative data confirmed the container and algorithm choices, rather than relying on big-O reasoning alone (cache effects often dominate).
