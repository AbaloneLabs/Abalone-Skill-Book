---
name: go_slices_maps_and_memory.md
description: Use when the agent is writing or reviewing Go code that uses slices (append, capacity, sub-slicing, copy), maps, preallocation, the slices and maps packages (Go 1.21+), escape analysis and heap allocation, value vs pointer semantics, struct layout and padding, sync.Pool, reducing allocations in hot paths, or diagnosing slice aliasing bugs, map iteration order surprises, large struct copies, GC pressure from per-request allocation, or accidental heap escapes of stack-allocated data.
---

# Go Slices, Maps, And Memory

Go's slices and maps are the workhorse data structures, and their semantics — dynamic arrays with aliasing, and unordered hash maps — have specific performance and correctness edges. A slice's backing array is shared by sub-slices (aliasing bugs), a `map`'s iteration order is randomized (and a map modified during iteration is undefined), and every allocation that escapes to the heap is GC work. The judgment problem is that Go's apparent simplicity hides memory-model decisions: preallocation avoids reallocation churn, `sync.Pool` reuses allocations, struct layout affects size and cache behavior, and escape analysis decides what is heap vs stack. Choosing carelessly produces slice-aliasing bugs, map-iteration surprises, and GC pressure that only shows under load.

Agents tend to append to slices without preallocation (quadratic-ish reallocation), to sub-slice and mutate without realizing the aliasing, to store large structs by value (copying), or to allocate per request without pooling. The judgment problem is to preallocate slices and maps when the size is known, to understand slice aliasing and copy when independence is needed, to lay out structs for size and cache, and to reduce heap allocations in hot paths via `sync.Pool` and escape-awareness. This skill is about treating Go memory choices as performance and correctness decisions, not conveniences.

## Core Rules

### Preallocate Slices And Maps When The Size Is Known Or Bounded

`make([]T, 0, n)` preallocates a backing array of capacity `n`, so subsequent `append`s do not reallocate (and copy) until capacity is exceeded. `make(map[K]V, n)` hints the map to allocate enough buckets for `n` entries, reducing rehashing. When you know or can bound the size, preallocate — it eliminates reallocation churn and is a one-line change with large impact in hot paths.

```
out := make([]Result, 0, len(inputs))  // preallocate
for _, in := range inputs {
    out = append(out, process(in))
}
```

Do not preallocate unboundedly (a huge `cap` from untrusted input wastes memory); bound it. For maps, the size hint is a hint, not a cap; the map still grows. The `slices.Grow` and `maps.Clone` helpers (Go 1.21+) express intent clearly.

### Understand Slice Aliasing; Copy When Independence Is Needed

A slice is a (pointer, length, capacity) triple pointing at a backing array. Sub-slicing (`b := a[1:3]`) creates a new slice header pointing at the same backing array — `b` and `a` share memory. Mutating through one is visible through the other. This is the source of subtle bugs: returning a sub-slice of a large array keeps the whole array live (memory leak), and mutating a returned sub-slice corrupts the original.

- When you need an independent copy, `copy(dst, src)` (or `slices.Clone` in Go 1.21+) duplicates the backing data.
- When you return a slice, decide whether the caller may mutate it; if not, return a copy or document immutability.
- Be careful with `append`: if the append fits within capacity, it mutates the shared backing array; if it exceeds capacity, it allocates a new array and the original is unaffected. This non-obvious branching is a classic bug source.

A sub-slice also pins the backing array: `return bigSlice[:10]` keeps the entire `bigSlice` backing array live even though only 10 elements are used. Copy the sub-slice (`return slices.Clone(bigSlice[:10])`) to release the rest.

### Know Map Semantics: Random Iteration, No Concurrent Read/Write, Zero-Value Access

Go maps are hash maps with several specific behaviors:

- **Iteration order is randomized**: `for k, v := range m` visits keys in an undefined order that changes per run. Never depend on map iteration order; if you need order, maintain a separate slice of keys or use an ordered structure.
- **Concurrent read/write is a race**: a map accessed by multiple goroutines with at least one writer is a data race and may panic ("concurrent map writes"). Use `sync.Map` for the specific concurrent pattern (append-only keys, many readers), or protect a regular map with a `sync.Mutex`/`sync.RWMutex`, or shard the map.
- **Zero-value access returns the zero value**: `m[missingKey]` returns the zero `V` with no error; use the comma-ok form (`v, ok := m[k]`) to distinguish present-but-zero from missing.
- **Map values are not addressable**: you cannot take `&m[k]`, which matters for maps of structs you want to mutate field-by-field (you must reassign the whole struct: `s := m[k]; s.f++; m[k] = s`).

### Lay Out Structs For Size And Cache Locality

Go structs are laid out in declaration order with alignment padding. A struct with fields ordered poorly can be much larger than necessary due to padding. Order fields by descending alignment (pointers/ints first, then smaller types) to minimize padding, or use the `fieldalignment` tool (part of `go vet`) to detect and reorder. For large arrays of structs (millions of entries), the size reduction compounds.

Cache locality matters for performance-critical loops: a struct of arrays (separate slices per field) can outperform an array of structs when a loop touches one field across many entries, because it is cache-friendly. This is an advanced optimization; profile before adopting.

### Reduce Heap Allocations In Hot Paths: sync.Pool, Escape Awareness, []byte Reuse

Every allocation that escapes to the heap is GC work. In hot paths (per-request, per-frame), reduce allocations:

- **`sync.Pool`**: reuse objects across requests. `pool.Get()` returns a reused object (or new), `pool.Put(obj)` returns it. Reset the object's state before use. Pools reduce GC pressure for short-lived, repeatable allocations (buffers, scratch objects). Note pools are cleared on GC, so they are a soft cache, not a guarantee.
- **Escape analysis**: the compiler decides what stays on the stack (cheap) vs escapes to the heap (GC work). `go build -gcflags="-m"` shows escape decisions. Common escape triggers: returning a pointer to a local, storing a local in an interface, a closure capturing a local by reference. Restructure to avoid unnecessary escapes (return values instead of pointers for small types; avoid `interface{}` where a concrete type works).
- **`[]byte` reuse**: for byte buffers, use `bytes.Buffer` (reset and reuse) or a pooled `[]byte` rather than allocating per operation.

Profile (`pprof`) to find the actual allocation sources; do not optimize blindly.

### Use Value Vs Pointer Semantics Deliberately

Go passes everything by value, including structs (copied) and pointers (the pointer is copied, the pointee is shared). For small structs, pass by value (copying is cheap and avoids aliasing and escape). For large structs, pass by pointer (avoids the copy). For mutation, pass by pointer. The tradeoff: pointers can escape to the heap (GC work) and introduce aliasing; values are stack-friendly and independent but copy.

A common pattern: methods on a small value type take a value receiver (`func (p Point) Dist()`), methods that mutate or on large types take a pointer receiver (`func (s *Server) Start()`). Be consistent within a type (mixing value and pointer receivers is confusing and disallowed in some cases).

## Common Traps

### append Mutating A Shared Backing Array

`b := a; b = append(b, x)` may mutate `a`'s backing array if `b` has capacity, or may not — depending on capacity. Copy or use full-slice expressions (`a[low:high:max]`) to control capacity.

### Sub-Slice Pinning A Large Array

`return big[:10]` keeps the entire `big` backing array live. Clone the sub-slice to release the rest.

### No Preallocation, Quadratic Reallocation

`var out []T; for ... { out = append(out, x) }` without preallocation reallocates and copies repeatedly. `make([]T, 0, n)` when size is known.

### Concurrent Map Access

A map read/written by multiple goroutines races and may panic. Protect with a mutex, shard, or use `sync.Map` for the append-only concurrent pattern.

### Depending On Map Iteration Order

Map iteration order is randomized; code that depends on order is flaky. Maintain a separate key slice or use an ordered structure.

### Large Struct Passed By Value Everywhere

A large struct copied per call is expensive. Pass by pointer for large types (and consider whether it escapes the heap).

### sync.Pool Object Not Reset

A pooled object returned to the pool with stale state surprises the next user. Reset all fields before use after `Get`.

### Per-Request Allocation Without Pooling

Allocating buffers per request creates GC pressure. Pool with `sync.Pool` or reuse a `bytes.Buffer`.

## Self-Check

- [ ] Slices and maps are preallocated (`make([]T, 0, n)`, `make(map[K]V, n)`) when the size is known or safely bounded, eliminating reallocation churn.
- [ ] Slice aliasing is understood: sub-slices share the backing array, `append` may mutate shared memory, and returned sub-slices are cloned (`slices.Clone`) when they would pin a large array or invite caller mutation.
- [ ] Map semantics are respected: iteration order is not depended upon, concurrent read/write is protected (mutex, shard, or `sync.Map`), missing keys use comma-ok, and map values are reassigned wholesale when mutated.
- [ ] Structs are laid out to minimize padding (fields ordered by alignment, checked with `fieldalignment`), and array-of-structs vs struct-of-arrays is considered for cache-critical loops.
- [ ] Hot paths reduce heap allocations via `sync.Pool` (with reset-on-Get), `bytes.Buffer` reuse, and escape-awareness (`-gcflags="-m"`), validated by `pprof`.
- [ ] Value vs pointer semantics are chosen deliberately: small structs by value, large structs by pointer, mutation by pointer, with consistent receiver types per type.
- [ ] The race detector (`-race`) passes, including for any map or slice shared across goroutines.
- [ ] No code depends on map iteration order, and no returned slice pins a large backing array unintentionally.
