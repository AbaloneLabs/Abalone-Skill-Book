---
name: javascript_collections_maps_sets_weakrefs.md
description: Use when the agent is choosing a JavaScript collection type, deciding between Object and Map for keyed data, using Set for uniqueness, choosing numeric vs string keys, reasoning about insertion order and key iteration, using WeakMap/WeakSet for metadata attached to objects without leaking, handling JSON serialization of collections, choosing array methods (filter/map/reduce) vs manual loops, or debugging key collisions, prototype pollution, or memory leaks from retained references. Covers Object vs Map vs Set tradeoffs, insertion-order semantics, key type choices, WeakMap/WeakSet for GC-friendly metadata, and the tradeoff between collection ergonomics and serialization/interop needs.
---

# Collections: Maps, Sets, WeakRefs

JavaScript offers several collection types — plain objects, `Map`, `Set`, `WeakMap`, `WeakSet`, and arrays — each with distinct semantics around key types, ordering, memory, and serialization. The judgment problem is matching the collection to the access pattern, the key type, the memory lifecycle, and the interop needs, rather than defaulting to a plain object for everything.

Agents tend to use plain objects as maps (hitting prototype pollution, string-only keys, and key collisions with inherited properties), use arrays where a Set would express uniqueness, or attach metadata to objects via a Map that then leaks because the Map holds strong references. The harm appears as subtle bugs from `__proto__` keys, lost insertion order assumptions, memory leaks from maps that never release entries, and JSON serialization that silently drops `Map`/`Set` data. The real work is choosing `Map` for keyed data with non-string keys or dynamic entries, `Set` for uniqueness, `WeakMap` for GC-friendly metadata, and plain objects only for fixed-shape records.

## Core Rules

### Choose Object For Fixed-Shape Records, Map For Dynamic Keyed Data

Plain objects and `Map` overlap but serve different purposes.

- **Plain object (`{}`)**: best for fixed-shape records with known string keys (`{ name, age, email }`). Concise literal syntax, fast property access, JSON-serializable. Use when the set of keys is known and stable.
- **`Map`**: best for dynamic keyed data where keys are added/removed at runtime, keys are not strings (objects, numbers as keys without string coercion), or the size is needed frequently. Keys preserve insertion order, any value (including objects) can be a key without coercion, and `map.size` is O(1).

The trap of using a plain object as a dynamic map: keys are coerced to strings (`map[1]` and `map["1"]` collide; object keys collide), the object inherits properties from its prototype (so `map.toString` is not a real entry), and certain keys (`__proto__`, `constructor`) are dangerous or behave unexpectedly. For a dynamic map with arbitrary keys, use `Map` (or `Object.create(null)` to avoid the prototype, but `Map` is clearer).

### Use `Set` For Uniqueness And Membership Testing

A `Set` stores unique values and provides O(1) `has()` checks. Use it when you need to track uniqueness or test membership efficiently.

- Deduplicating: `[...new Set(array)]` removes duplicates.
- Membership: `if (visited.has(node))` is clearer and faster than `if (array.includes(node))` for repeated checks.
- Sets preserve insertion order and accept any value type as an element (using SameValueZero equality, so `NaN` equals `NaN`, unlike `===`).

Do not use an array for repeated membership tests on large data (`includes` is O(n)); use a `Set`. Do not use a `Set` when you need duplicates or ordering by value — that is what an array is for.

### Understand Key Coercion And Equality Semantics

Each collection type has different key semantics, and confusing them causes bugs.

- **Plain object**: keys are coerced to strings. `obj[1]` and `obj["1"]` are the same key. Object keys collide with inherited prototype properties.
- **`Map`**: keys use SameValueZero equality and are not coerced. `map.set(1, ...)` and `map.set("1", ...)` are different entries. An object used as a key is identified by identity, not by its contents (two distinct `{}` are different keys).
- **`Set`**: SameValueZero equality, so `NaN` is deduplicated (unlike arrays where `includes(NaN)` works but `indexOf` does not).

When you need numeric keys that stay numeric, or object keys identified by identity, use `Map`. Plain objects will coerce and collide.

### Use `WeakMap` And `WeakSet` For GC-Friendly Metadata

A `WeakMap` holds key-value pairs where the key is an object and the reference is weak: when the key object is garbage-collected elsewhere, the entry is removed from the `WeakMap` automatically. This is ideal for attaching metadata to objects you do not own, without preventing their collection.

- `weakMap.set(domNode, { listener, state })` attaches data to a DOM node; when the node is GC'd, the entry vanishes, avoiding a leak.
- `WeakSet` tracks whether an object has been seen, without preventing its collection.
- Weak collections are not enumerable (you cannot iterate keys or get `size`) because their contents depend on GC timing. Use them for metadata, not for data you need to enumerate.

The trap of using a regular `Map` for object-attached metadata: the `Map` holds a strong reference to the key, so the object can never be GC'd as long as the `Map` lives — a classic memory leak. Use `WeakMap` when the metadata's lifetime should be tied to the object's.

### Preserve Insertion Order Or Sort Deliberately

- **`Map` and `Set`**: iterate in insertion order. This is guaranteed and useful when order matters.
- **Plain object**: non-integer-string keys iterate in insertion order (modern engines), but integer-like keys (`"0"`, `"1"`) are iterated in numeric order first. Relying on object key order for integer keys is fragile.
- **Arrays**: ordered by index, the natural choice for ordered sequences.

If you need stable iteration order independent of key type, use `Map`. If you need sorted order, sort explicitly rather than relying on insertion or numeric quirks.

### Handle JSON Serialization At The Boundary

`JSON.stringify` serializes plain objects and arrays natively but does **not** serialize `Map` or `Set` — they become `{}` or `{}`. If your data crosses a JSON boundary (HTTP, storage), either use plain objects/arrays, or convert explicitly.

- Convert a `Map` to an array of pairs for JSON: `[...map.entries()]`, and reconstruct with `new Map(pairs)`.
- Convert a `Set` to an array: `[...set]`.
- Or use plain objects throughout if JSON interop is the primary concern and keys are strings.

The mistake is building logic on a `Map`, calling `JSON.stringify`, and silently getting an empty object. Either convert at the boundary or choose a plain object if JSON is central.

### Pick Array Methods By Intent: `filter`/`map`/`reduce` vs Loops

Arrays have rich methods. Use them by intent, and reach for a loop when methods hurt clarity.

- **`map`**: transform each element into a new array of the same length.
- **`filter`**: select elements matching a predicate.
- **`reduce`**: accumulate elements into a single value (sum, group, flatten). Use carefully; a complex `reduce` is often clearer as a `for` loop.
- **`find`/`findIndex`**: locate the first matching element.
- **`some`/`every`**: test whether any/all elements match.

For repeated membership tests, convert to a `Set` first. For grouping, `Object.groupBy`/`Map.groupBy` (modern) or a `reduce` into a `Map` is clearer than nested loops. Prefer the method that expresses the intent; fall back to a `for` loop when the functional form obscures the logic.

## Common Traps

### Plain Object As A Dynamic Map With Prototype Pollution

Using `{}` for arbitrary keys risks `__proto__`/`constructor` collisions and inherited properties. Use `Map` (or `Object.create(null)`) for dynamic keyed data.

### Numeric Key Coercion In Objects

`obj[1]` and `obj["1"]` collide because object keys are strings. Use `Map` when numeric keys must stay distinct from string keys.

### `Map` Holding Strong References And Leaking

Attaching metadata to objects via a regular `Map` prevents those objects from being GC'd. Use `WeakMap` for metadata whose lifetime should track the object.

### JSON Silently Dropping `Map`/`Set`

`JSON.stringify(new Map([["a",1]]))` produces `"{}"`. Convert collections to arrays/objects at the JSON boundary, or use plain objects.

### Array `includes` For Repeated Membership

`array.includes(x)` is O(n); repeated checks on large data are slow. Use a `Set` for membership.

### Relying On Object Key Order For Integer Keys

Integer-like object keys iterate in numeric order, not insertion order. Use `Map` if you need stable insertion order regardless of key type.

### Complex `reduce` That Obscures Logic

A `reduce` with branching accumulation is often clearer as a plain `for` loop. Prefer readability over functional purity.

## Self-Check

- [ ] Plain objects are used for fixed-shape records with known string keys; `Map` is used for dynamic keyed data, non-string keys, or frequent size checks.
- [ ] `Set` is used for uniqueness and O(1) membership testing instead of repeated array `includes`.
- [ ] Key coercion and equality semantics are understood: object keys are strings (with prototype pitfalls), `Map`/`Set` use SameValueZero without coercion.
- [ ] `WeakMap`/`WeakSet` are used for metadata attached to objects the code does not own, so GC is not blocked; strong-reference `Map` is not used for that purpose.
- [ ] Iteration order is deliberate: `Map`/`Set` for guaranteed insertion order, explicit sorting where sorted order is needed, no reliance on integer-key object order.
- [ ] Collections crossing JSON boundaries are converted to plain objects/arrays (or plain objects are used from the start when JSON interop is primary).
- [ ] Array methods (`map`/`filter`/`reduce`/`find`) are chosen by intent, with `for` loops used where the functional form obscures logic.
- [ ] No accidental prototype pollution from `__proto__`/`constructor` keys in plain-object maps.
