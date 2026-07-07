---
name: scala_collections_and_performance.md
description: Use when the agent is choosing or optimizing Scala collections (immutable vs mutable, Vector vs List vs Array, Seq vs List, Set/Map variants), reasoning about parallel collections (.par) and their overhead/correctness, diagnosing performance problems in collection-heavy code, converting between Java and Scala collections (JavaConverters/CollectionConverters), dealing with collection break-out / building, using views or iterators to avoid intermediate collections, or is diagnosing "Scala collections are slow", "List head/tail vs random access", "parallel collection gives wrong results", "too much allocation/GC pressure", or Java interop collection issues. Covers immutable collection selection and complexity, Vector's balanced trees, parallel collection safety, views/laziness, Java collection interop, and specialization/boxing.
---

# Collections And Performance In Scala

Scala's collections library is rich (immutable and mutable, sequential and parallel, strict and lazy) and its API ergonomics (`map`, `filter`, `flatMap`, `foldLeft`) encourage expressive pipelines, but the same expressiveness hides performance cliffs. `List` is a singly-linked list — O(1) head/tail but O(n) random access, so `list(i)` or `list.update` is catastrophically slow; `Vector` is the default immutable indexed sequence (effectively O(log n) random access via balanced trees) and should be the default for general use. Chaining strict operations (`xs.map(f).filter(g).map(h)`) creates intermediate collections, multiplying allocation and GC pressure; views (`xs.view.map(f).filter(g)`) or lazy streams avoid them. Parallel collections (`.par`) add fork-join overhead and require the operation to be side-effect-free and order-independent, or they give wrong results. And Java interop requires explicit conversion (`scala.jdk.CollectionConverters._` / `.asScala`/`.asJava`), with the conversion itself allocating. The judgment problem is to choose the right collection by access pattern, to avoid intermediate allocation with views or fusion, to use parallel collections only when safe and worthwhile, and to handle Java interop deliberately.

Agents default to `List` everywhere (slow random access), chain strict ops creating GC pressure, use `.par` on side-effecting or tiny collections, or fumble Java/Scala collection conversion. The remedy is to match the collection to the access pattern and to be deliberate about laziness and parallelism.

## Core Rules

### Choose The Collection By Access Pattern

- `Vector` (immutable, indexed): the general-purpose default. Effectively O(log₃₂ n) (near-constant) random access, update, head, tail, append. Use unless you specifically need another structure's properties.
- `List` (immutable, singly-linked): O(1) head/tail/prepend, O(n) random access and reverse. Use for LIFO/stack patterns, recursive head/tail decomposition, or when you prepend and iterate only. Never index into a `List`.
- `Array` (mutable, JVM native): fast random access, fixed size, specialized to avoid boxing for primitives (with limitations). Use for tight numeric loops or Java interop; it is mutable and invariant.
- `Set`/`Map` (immutable): default `HashSet`/`HashMap` (hash-array-mapped trie, effectively O(log n)). `TreeSet`/`TreeMap` for ordered. `BitSet` for dense integer sets.
- `mutable.ArrayBuffer` for a growable mutable buffer; `mutable.HashMap`/`mutable.HashSet` for mutable keyed access.

Match the structure to how you access it: prepend/iterate → `List`; random access/update → `Vector`; primitive numerics → `Array`; growable mutable → `ArrayBuffer`.

### Avoid Intermediate Collections With Views Or Fusion

A chain `xs.map(f).filter(g).map(h).toList` on a strict collection builds three intermediate collections. For large collections this is significant allocation and GC. Options:

- `xs.view.map(f).filter(g).map(h).toList` — a view is lazy; operations are fused and applied once per element on traversal. Call a terminal (`.toList`, `.foreach`) to force.
- `xs.iterator.map(f).filter(g).map(h).toList` — iterators are single-use and lazy; good for one-pass pipelines.
- Direct fusion where the API allows (e.g., a single `foldLeft` combining the steps).

Use views/iterators for large pipelines; strict collections for small data or when you reuse the intermediate result. Beware: views are non-indexable and recompute on each traversal unless forced.

### Use Parallel Collections Only When Safe And Worthwhile

`.par` (parallel collections) splits work across the fork-join pool, speeding CPU-bound, order-independent, side-effect-free operations on large collections. But: (1) the operation must be side-effect-free and associative/commutative where reducing (a non-associative reduce gives nondeterministic results); (2) side effects (mutating a shared `var`/`mutable.Map`) cause data races and wrong results — use `reduce`/`aggregate`/`fold` or a thread-safe accumulator instead; (3) the overhead (splitting, synchronization) exceeds the benefit on small collections or cheap operations. Measure before adopting `.par`; for fine control use a custom `ForkJoinPool` (the default global pool can be contended). For large-scale parallelism, consider Akka Streams or a dedicated framework rather than `.par`.

- `.par` only for side-effect-free, order-independent operations on large, CPU-bound data.
- Side effects in `.par` cause races; use functional reductions (`fold`/`reduce`) or thread-safe accumulators.
- Measure: overhead exceeds benefit on small/cheap collections; the global pool may contend.

### Convert Between Java And Scala Collections Explicitly

Scala and Java collections do not interoperate automatically. Use `scala.jdk.CollectionConverters` (Scala 2.13+) or `scala.collection.JavaConverters` (older) and the `.asScala`/`.asJava` extension methods to convert explicitly. The conversion allocates a wrapper (lazy or strict depending on the direction); repeated conversions in a hot loop are wasteful. For primitive arrays, prefer `Array[Int]` directly (Java `int[]`) over boxed conversions. When a Java library returns a `java.util.List`, convert once at the boundary and work in Scala collections inside. Beware of mutable Java collections passed to Scala code: the wrapper is a live view, so Scala-side mutation affects the Java side.

- Use `CollectionConverters`/`JavaConverters` with `.asScala`/`.asJava` at boundaries.
- Conversions allocate wrappers; convert once, not in a hot loop.
- Mutable Java collections wrapped to Scala are live views; mutation crosses the boundary.

### Be Aware Of Specialization, Boxing, And Allocation

Generic Scala collections box primitives (`Int` → `java.lang.Integer`), adding allocation and indirection; this matters in tight numeric loops. `Array[Int]`/`Array[Double]` are unboxed JVM arrays (fast). Some collections are `@specialized` for a few primitive types (reducing boxing) but specialization is incomplete and increases bytecode size. For numeric hotspots, consider `Array` of primitives, the `spire` library, or specialized libraries. Beyond boxing, every `map`/`filter` on immutable collections allocates a new collection; in hot paths, prefer in-place mutation (`ArrayBuffer`, `Array`) or a single-pass `foldLeft` over chained strict ops. Profile (JFR/async-profiler) to find allocation/GC hotspots before optimizing.

- Generic collections box primitives; `Array[Int]`/`Array[Double]` are unboxed for numeric hotspots.
- Immutable operations allocate; in hot paths use mutation or single-pass folds.
- Profile before optimizing; allocation/GC hotspots are the usual Scala performance issue.

## Common Traps

### Indexing Into A List

`list(i)`/`list.update` is O(n) (or worse, O(n²) in a loop). Use `Vector` for random access.

### Chained Strict Operations Creating Intermediate Collections

`xs.map.filter.map` builds intermediates → GC pressure. Use `.view`/`.iterator` or single-pass folds.

### Side Effects In Parallel Collections

Mutating a shared accumulator in `.par` races. Use functional `fold`/`reduce` or thread-safe accumulators.

### Non-Associative Reduce In Parallel

`(a - b)` reduce gives nondeterministic results across partitions. Ensure associativity/commutativity.

### Using .par On Small Or I/O-Bound Collections

Overhead exceeds benefit; the global pool contends. Measure; reserve `.par` for large CPU-bound work.

### Fumbling Java/Scala Collection Conversion

Implicit conversions are deprecated; use explicit `.asScala`/`.asJava` and convert once at the boundary.

### Defaulting To List From Habit

`List` is idiomatic in teaching but wrong for random access. Default to `Vector` unless prepend/iterate.

### Boxing In Numeric Hotspots

`Vector[Int]` boxes each element. Use `Array[Int]` or specialized libraries for numeric inner loops.

## Self-Check

- [ ] The collection type matches the access pattern (`Vector` for random access/update, `List` for prepend/iterate, `Array`/`ArrayBuffer` for mutable/numeric, `Set`/`Map` variants by need); no O(n) indexing into `List`.
- [ ] Large pipelines use views (`.view`) or iterators to avoid intermediate collections, or a single-pass `foldLeft`; strict chains are reserved for small/reused data.
- [ ] Parallel collections (`.par`) are used only for side-effect-free, order-independent, associative operations on large CPU-bound data, with functional reductions (no shared mutable state).
- [ ] Java/Scala collection conversions use explicit `CollectionConverters`/`.asScala`/`.asJava` at boundaries, converted once (not in hot loops), with mutable-wrapper aliasing understood.
- [ ] Numeric hotspots use unboxed `Array[Int]`/`Array[Double]` or specialized libraries; boxing and allocation in hot paths have been profiled (JFR/async-profiler) and addressed.
- [ ] The collection choice has been considered under the operation's complexity (random access, prepend, iteration, lookup) and the data size, not chosen by habit.
- [ ] Views are forced (terminal `.toList`/`.foreach`) before reuse, and iterators are treated as single-use.
- [ ] Performance claims are measured, not assumed; the parallel/strict/lazy trade-offs have been reasoned about for the specific workload.
