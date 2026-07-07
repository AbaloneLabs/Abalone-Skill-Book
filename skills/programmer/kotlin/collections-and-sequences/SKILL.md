---
name: kotlin_collections_and_sequences.md
description: Use when the agent is choosing between Kotlin List/Set/Map and Sequence, Collection vs MutableCollection, read-only vs mutable collection views, collection operators (map/filter/reduce/fold/groupBy/associate), lazy evaluation with Sequence and asSequence, performance of chained operators (intermediate list creation), equality and structural sharing, or is diagnosing "the chained map/filter is slow and allocates many lists", "modifying a list while iterating", ConcurrentModificationException, unexpected mutation of a read-only view, or "asSequence made it slower". Covers the eager-collection vs lazy-sequence decision, read-only vs mutable collection semantics, operator chaining performance, and the pitfalls of aliasing and concurrent modification.
---

# Collections And Sequences In Kotlin

Kotlin's collection story has two axes that agents routinely conflate, and each conflation produces a distinct class of bug. The first axis is read-only vs mutable: `List`/`Set`/`Map` are read-only *views* (the compiler prevents mutation through them), but the underlying instance may be a `MutableList` cast to `List`, so a read-only reference does not guarantee immutability — another holder with a mutable reference can change it. The second axis is eager vs lazy: the standard collection operators (`map`, `filter`, `groupBy`) are eager — each step builds a complete intermediate `List` — while `Sequence` is lazy, processing element-by-element through the whole chain. Chaining eager operators on a large collection builds N intermediate lists (memory and time); converting to `Sequence` (`asSequence()`) pipelines elements one at a time, which can be dramatically faster and lighter — or, for small collections, slower due to overhead. The judgment problem is to choose read-only views for API safety while understanding they are not deeply immutable, to choose `Sequence` vs eager `List` by collection size and chain length, and to avoid the aliasing and concurrent-modification traps.

Agents chain `.map { }.filter { }.map { }` reflexively, building several full-size intermediate lists on a million-element source; or they pass a `MutableList` as a `List` and are surprised when a downstream change mutates it; or they call `asSequence()` on a tiny collection and wonder why it got slower. The remedy is to understand the eager-by-default model, to reach for `Sequence` on large collections or long chains, to expose read-only types in APIs while documenting deep-immutability expectations, and to iterate safely.

## Core Rules

### Choose Read-Only Types For APIs, But Know They Are Views

Kotlin's `List<E>`, `Set<E>`, `Map<K,V>` are read-only: the type prevents `add`, `remove`, `put` through that reference. Expose read-only types from APIs (`fun all(): List<User>`) so callers cannot mutate your internal state through the returned reference. But a read-only type is a view, not a guarantee of deep immutability: the instance behind it may be a `MutableList`, and another reference holding the mutable type can change it. If you need a defensive copy (so later mutation cannot affect the snapshot), copy (`toList()`); if you need a guaranteed-immutable instance, use `copyOf()`/`toList()` or an immutable-collections library. Do not assume a `List` parameter is unchanging across a suspension or async boundary.

- Return `List`/`Set`/`Map` (read-only) from APIs; accept read-only types as parameters where mutation is not needed.
- Defensive copy (`toList()`) when you store a passed-in mutable collection and need stability.
- A read-only view of a mutable instance is not thread-safe and not deeply immutable.

### Choose Sequence For Large Collections Or Long Chains

Eager collection operators build an intermediate list per step: `list.map { }.filter { }.map { }` creates two intermediate lists before the final one. For a large source or a long chain, this is O(n × steps) time and memory. `Sequence` is lazy: `list.asSequence().map { }.filter { }.map { }.toList()` pipelines each element through the whole chain before processing the next, building no intermediate lists. Use `Sequence` when the source is large (thousands+) or the chain has several steps; use eager `List` operations for small collections (where Sequence's per-element overhead exceeds the savings) and where you need random access or multiple passes.

- `asSequence()` for large sources or multi-step chains; `.toList()` at the end to materialize.
- Sequence also enables short-circuiting (`first { }`, `take(n)`) without processing the whole source.
- For small collections, eager is faster; measure rather than assume.

### Use The Right Operator Semantics

Kotlin's collection operators have specific semantics worth knowing: `mapNotNull` (map + filter nulls), `flatMap` (map each to a collection, flatten), `groupBy` (Map<key, List>), `associate`/`associateBy` (Map without grouping), `partition` (Pair of matched/unmatched), `fold`/`reduce` (fold takes an initial accumulator and is safer than reduce which throws on empty). Choose the operator that expresses the intent directly rather than chaining generic `filter`/`map` to approximate it. `reduce` throws on an empty collection; prefer `fold` (with an initial value) or `reduceOrNull` where emptiness is possible.

- `fold(initial) { acc, x -> ... }` over `reduce` when the collection may be empty.
- `associateBy { it.id }` over `groupBy` when keys are unique (a Map, not Map<K, List<V>>).
- `mapNotNull` over `map { } .filterNotNull`.

### Iterate Safely: No Structural Modification During Iteration

Modifying a collection while iterating it (adding/removing elements) throws `ConcurrentModificationException` (for non-concurrent collections) or produces undefined behavior. Collect the changes and apply after iteration, use the iterator's `remove()` (on mutable lists), or use an immutable pattern (build a new list from the old). For concurrent access from multiple threads, use a thread-safe collection (`CopyOnWriteArrayList`, `ConcurrentHashMap`) or synchronize; the standard `MutableList` is not thread-safe.

- Do not `add`/`remove` on a list while `for`/`forEach`-ing it; collect changes or use `removeIf`.
- `removeIf` (on `MutableList`) is the safe in-place filter.
- For concurrent mutation, use a concurrent collection or explicit synchronization.

### Understand Equality And Content Comparison

Two `List`s with the same elements in the same order are `==` (structural equality) in Kotlin, unlike Java's reference equality. `data class` equals compares properties. This is usually what you want, but it means comparing large collections is O(n), and a `List` in a `data class` makes equality and `hashCode` O(n). Be aware when collections are keys or compared in hot paths. For reference equality use `===`.

- `listOf(1,2,3) == listOf(1,2,3)` is `true` (structural).
- A `data class` containing a large list has O(n) equality/hash; consider whether that matters.

### Prefer Immutable Construction And Copying Patterns

Kotlin favors building new collections over mutating in place: `list + element` returns a new list, `map`/`filter` return new lists, `data class.copy(...)` returns a modified copy. Prefer these for clarity and to avoid aliasing bugs. Reserve `MutableList`/`MutableMap` for genuine in-place accumulation (a builder loop), and expose the result as a read-only type. The immutable style composes with coroutines and concurrency better (no shared mutable state).

## Common Traps

### Chained Eager Operators Allocating Many Intermediate Lists

`bigList.map{}.filter{}.map{}` builds two full intermediate lists. Use `asSequence()` for large sources or long chains.

### asSequence On A Tiny Collection Slower

Sequence has per-element overhead; for small collections eager is faster. Measure; do not apply blindly.

### Read-Only View Of A Mutable Instance Changing

A `List` returned that is actually a `MutableList` can be mutated by another holder. Defensive-copy (`toList()`) when stability matters.

### ConcurrentModificationException From Mutating During Iteration

`for (x in list) list.remove(x)` throws. Use `removeIf` or collect changes.

### reduce Throwing On Empty

`list.reduce { ... }` throws on an empty list. Use `fold(initial)` or `reduceOrNull`.

### groupBy Where associateBy Was Meant

`groupBy` produces `Map<K, List<V>>`; for unique keys, `associateBy` produces `Map<K, V>`.

### Mutable Collection Shared Across Threads

A `MutableList` mutated from two threads corrupts or throws. Use a concurrent collection or synchronize.

### Equality Of Large Collections In Hot Paths

O(n) equality/hash for a data class holding a large list; consider the cost if compared frequently.

## Self-Check

- [ ] APIs expose read-only types (`List`/`Set`/`Map`) and accept read-only parameters; defensive copies (`toList()`) are made where deep stability is required, and no read-only view is assumed deeply immutable.
- [ ] `Sequence` is used for large collections or multi-step chains (with `toList()` to materialize), and eager operations are kept for small collections where Sequence overhead would dominate.
- [ ] The right operator expresses the intent (`mapNotNull`, `associateBy`, `fold`, `partition`) rather than approximating with generic `filter`/`map`.
- [ ] No structural modification happens during iteration; `removeIf` or post-iteration application is used, and concurrent mutation uses thread-safe collections or synchronization.
- [ ] `fold`/`reduceOrNull` is used where the collection may be empty (not `reduce`), and equality/hash costs of collections in data classes are considered in hot paths.
- [ ] Immutable construction (`+`, `map`, `copy`) is preferred over in-place mutation; `MutableList`/`MutableMap` are reserved for genuine accumulation and exposed as read-only.
- [ ] No `asSequence()` is applied blindly to tiny collections, and no eager chain builds excessive intermediate lists on large data.
- [ ] The collection design has been considered under large data, multi-step pipelines, aliasing, and concurrency, and remains correct and performant.
