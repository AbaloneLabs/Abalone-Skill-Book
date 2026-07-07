---
name: collections_and_streams.md
description: Use when the agent is choosing or using Java Collections Framework types (List/Set/Map/Queue/Deque), selecting between ArrayList/LinkedList/HashSet/TreeSet/HashMap/LinkedHashMap/EnumMap/IdentityHashMap, writing or reviewing Stream pipelines (map/filter/reduce/collect/flatMap/parallel), choosing collectors, using Optional, implementing equals/hashCode/Comparable, or diagnosing ConcurrentModificationException, performance regressions from O(n) contains on a List, broken equals/hashCode contracts, stream misuse (side effects, reuse, infinite streams), parallel stream contention, or wrong Map/Set semantics. Covers collection complexity, mutability, ordering guarantees, stream laziness and fusion, collector choice, and the equals/hashCode contract.
---

# Collections And Streams

The Java Collections Framework and the Stream API are the two most-used parts of the Java standard library, and they are also the two most-misused. The Collections Framework has dozens of concrete types whose performance characteristics and semantic guarantees differ in ways that are invisible until production load: an `ArrayList` and a `LinkedList` both implement `List`, but one has O(1) random access and O(n) insertion in the middle, and the other reverses those costs. A `HashSet` and a `TreeSet` both implement `Set`, but one has undefined iteration order and the other sorts. The Stream API layers lazy, declarative data processing on top, and it is easy to write a pipeline that compiles, looks clean, and is either silently incorrect (side effects in lambdas, mutated shared state) or dramatically slower than the loop it replaced (unfused operations, unnecessary boxing, parallel streams on small or ordered data).

The judgment problem is not "how do I call `.stream().filter().collect()`" but "which collection type matches my access pattern, is my stream pipeline correct and fused, and have I honored the contracts (`equals`/`hashCode`, `Comparable`, fail-fast iteration) that these APIs silently depend on." Agents tend to reach for `ArrayList` and `HashMap` reflexively, ignore ordering and complexity guarantees, write stream pipelines with side effects or shared mutable state, use `parallelStream` without measuring, and implement `equals`/`hashCode` incorrectly (or not at all), producing collections that lose elements or never find them.

## Core Rules

### Match The Collection Type To The Access Pattern, Not To Habit

The single most important decision is choosing the concrete collection type based on how the data is actually accessed, not on what you always use. The key questions: Do you need random access by index (→ `ArrayList`, not `LinkedList`)? Do you need insertion/removal at both ends (→ `ArrayDeque`, not `LinkedList`)? Do you need sorted iteration (→ `TreeSet`/`TreeMap`, or `LinkedHashMap` for insertion order)? Do you need constant-time membership tests (→ any `Set`, never `List.contains` in a loop)? Do you need stable iteration order (→ `LinkedHashMap`/`LinkedHashSet`, since `HashMap`/`HashSet` order is undefined and can change between JVM versions)?

`ArrayList` is the right default for indexed lists: it has O(1) random access, O(1) amortized append, and O(n) insertion/removal in the middle. `LinkedList` is almost never the right choice — its theoretical O(1) middle insertion is defeated by the O(n) traversal to reach the node, and its memory overhead per element (a node object with two pointers) is far higher than `ArrayList`'s backing array. Prefer `ArrayDeque` over `LinkedList` for stacks and queues: it is array-backed, has no per-element node allocation, and is faster in nearly all cases. The reflexive `new LinkedList<>()` is a smell.

For maps, `HashMap` is the default, but consider: `LinkedHashMap` when you need predictable iteration order (insertion-order or access-order for LRU caches), `TreeMap` when you need sorted keys or range queries (`subMap`, `headMap`), `EnumMap` when keys are enum constants (it is an array-backed, extremely fast map with no hashing), and `IdentityHashMap` only when you specifically want reference-equality (rare, and almost always a bug if unintended). Choosing the wrong map type is a silent performance or correctness bug.

### Honor The equals/hashCode/Comparable Contracts

Collections that use hashing (`HashSet`, `HashMap`) or comparison (`TreeSet`, `TreeMap`) depend entirely on `equals`, `hashCode`, and `compareTo` being correct. The contracts are non-negotiable: `equals` must be reflexive, symmetric, transitive, and consistent; `hashCode` must be consistent across calls and must agree with `equals` (equal objects must have equal hash codes); `compareTo` must be consistent with `equals` (for `TreeSet` to behave correctly) and must define a total order. Violating these contracts causes elements to vanish from sets, map values to become unreachable, and sorted collections to silently corrupt.

The most common violations: defining `equals` without `hashCode` (two equal objects with different hash codes land in different buckets — the map "loses" the value), using mutable fields in `hashCode`/`equals` (changing a key after insertion breaks the map — the key becomes unreachable), and asymmetric `equals` (comparing different types in `equals` without checking `getClass()` or using `instanceof` incorrectly). For records and simple value classes, let the tools generate `equals`/`hashCode` (records do it automatically). For mutable entities used as map keys, prefer an immutable surrogate key (an ID) rather than the mutable object itself.

If you put objects into a `HashMap` as keys and then mutate a field that participates in `hashCode`, the key is now in the wrong bucket and `containsKey` will return false even though the object is still in the map. This is one of the most insidious collection bugs. The rule: never use mutable objects as map keys, or if you must, never mutate them after insertion. `String`, `Integer`, `UUID`, and records are safe keys; mutable POJOs are not.

### Treat Streams As Lazy, Stateless, Side-Effect-Free Pipelines

A stream pipeline is lazy: intermediate operations (`map`, `filter`, `flatMap`) are not executed until a terminal operation (`collect`, `forEach`, `reduce`, `count`) is invoked, and even then only as many elements as needed are processed. This laziness enables fusion — the entire pipeline is executed in a single pass over the data, with each element flowing through all operations before the next element is processed. Fusion is what makes streams efficient; breaking it (by, e.g., calling `.sorted()` in the middle, which must buffer the entire stream) defeats the benefit.

The critical correctness rules for stream pipelines: lambdas passed to intermediate operations must be stateless (no mutation of external variables, no dependence on iteration order unless the stream is ordered), side-effect-free (no I/O, no mutation of shared collections — use `collect` with a proper collector instead), and non-interfering (do not modify the stream's source during the pipeline). Violating these produces non-deterministic results, `ConcurrentModificationException`, or silent data corruption, especially under parallel streams.

The most common stream bug is using `forEach` to mutate an external collection instead of using `collect`: `list.stream().filter(...).forEach(x -> result.add(x))` is not thread-safe under parallel streams and is always worse than `.collect(Collectors.toList())`. Another is reusing a stream (streams are single-use; calling a terminal operation twice throws `IllegalStateException`). Another is side effects in `peek` — `peek` is for debugging, not for mutation, and relying on it for logic makes the pipeline order-dependent and non-fusable.

### Choose Collectors Deliberately, And Understand Mutable Reduction

`Collectors.toList()`, `Collectors.toSet()`, `Collectors.toMap()`, `Collectors.groupingBy`, and `Collectors.joining` cover most needs, but each has gotchas. `Collectors.toMap` throws on duplicate keys unless you provide a merge function — the default behavior is to throw `IllegalStateException`, which surprises agents who expected last-write-wins. Always provide a merge function when duplicate keys are possible: `Collectors.toMap(key, value, (a, b) -> b)` for last-wins. `Collectors.toUnmodifiableList/Set/Map` (Java 10+) return immutable collections; mixing these with later mutation attempts throws.

`Collectors.groupingBy` produces a `Map<K, List<V>>` by default, but accepts a downstream collector for richer aggregation: `groupingBy(byCategory, counting())` for counts, `groupingBy(byCategory, mapping(valueMapper, toList()))` for transformed lists, `groupingByConcurrent` for concurrent grouping under parallel streams. Understanding the downstream collector pattern unlocks the real power of collectors and avoids post-processing the result with loops.

For custom mutable reduction, `Collectors.of(supplier, accumulator, combiner)` lets you build any accumulator. The combiner is mandatory for parallel streams and must be associative — ignoring it produces wrong results under `parallelStream`. If you cannot write a correct combiner, your reduction is not parallelizable and you should not use `parallelStream`.

### Never Use parallelStream Without Measuring

`parallelStream` splits the data across the common `ForkJoinPool`, which can speed up CPU-bound pipelines on large datasets — and can dramatically slow down or corrupt pipelines that are small, I/O-bound, order-dependent, or have side effects. The common pool is shared across all parallel streams in the JVM, so blocking I/O in a parallel stream starves other parallel operations. Ordered sources (like `ArrayList` range operations) and ordered terminal operations (`forEachOrdered`, `collect` on an ordered stream) limit parallelism because elements must be processed in encounter order.

The rule: default to sequential streams. Use `parallelStream` only when the dataset is large (tens of thousands of elements or more), the per-element work is CPU-intensive, the operations are stateless and side-effect-free, and you have measured a real improvement. A `parallelStream` on a 100-element list with simple filtering is almost always slower than the sequential version due to splitting and merging overhead.

## Common Traps

### Using LinkedList For Random Access Or As A General-Purpose List

`LinkedList.get(i)` is O(n) — each index access traverses from the head. Use `ArrayList` for indexed access; use `ArrayDeque` for stack/queue operations. `LinkedList` is almost never the right answer.

### Defining equals Without hashCode (Or Vice Versa)

Objects that are `equals` but have different `hashCode` values land in different hash buckets and are never found in a `HashMap`/`HashSet`. Always define both, consistently, using the same fields. Records do this automatically.

### Mutating A HashMap Key After Insertion

The key moves to the "wrong" bucket relative to its new `hashCode`, and `containsKey` returns false. Use immutable keys, or never mutate key fields after insertion.

### Mutating An External Collection In forEach Instead Of Using collect

`stream.forEach(x -> result.add(x))` is not thread-safe under parallel streams and is always inferior to `stream.collect(toList())`. Use collectors for all accumulation.

### Forgetting The Merge Function In Collectors.toMap

Duplicate keys throw `IllegalStateException`. Always provide `(a, b) -> b` (or your merge logic) when the key is not guaranteed unique.

### Side Effects In peek Treated As Pipeline Logic

`peek` is for debugging; its execution depends on whether the operation is needed for the terminal result. Relying on `peek` for side effects makes the pipeline order-dependent and fragile.

### Using List.contains In A Loop (O(n) Per Check)

Checking membership in a `List` inside a loop is O(n²). Convert the `List` to a `Set` first for O(1) membership tests.

### parallelStream On Small, Ordered, Or I/O-Bound Data

Parallel overhead exceeds the gain on small datasets; ordered streams limit parallelism; blocking I/O in a parallel stream starves the shared common pool. Measure before parallelizing.

## Self-Check

- [ ] The concrete collection type matches the access pattern: `ArrayList` for indexed access, `ArrayDeque` for stack/queue, `HashMap`/`LinkedHashMap`/`TreeMap`/`EnumMap` chosen by ordering and key-type needs, `Set` used for membership tests — no `LinkedList` for random access, no `List.contains` in loops.
- [ ] `equals`, `hashCode`, and `compareTo` (where used) honor their contracts: same fields used in `equals` and `hashCode`, no mutable key fields, `compareTo` consistent with `equals`, and records or generated methods used where possible.
- [ ] No mutable object is used as a `HashMap`/`HashSet` key, or if it is, its `hashCode`-relevant fields are never mutated after insertion.
- [ ] Stream pipelines are stateless, side-effect-free, and non-interfering: no external mutation in lambdas, no `forEach` for accumulation (collectors used instead), no stream reuse, and `peek` is debug-only.
- [ ] Collectors are chosen deliberately: `toMap` has a merge function when keys may collide, `groupingBy` uses downstream collectors for aggregation, and custom collectors have a correct, associative combiner.
- [ ] `parallelStream` is used only on large, CPU-bound, unordered, side-effect-free pipelines, and only after measurement confirmed a real improvement; no blocking I/O runs in a parallel stream.
- [ ] Unmodifiable wrappers (`List.of`, `Set.of`, `Collectors.toUnmodifiableList`) are used where immutability is intended, and no code path attempts to mutate them.
- [ ] Fail-fast iteration is respected: no structural modification of a collection during iteration without using `Iterator.remove()` or a `ConcurrentHashMap`/`CopyOnWriteArrayList` when concurrent modification is expected.
