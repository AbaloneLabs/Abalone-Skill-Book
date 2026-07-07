---
name: swift_value_types_and_copy_on_write.md
description: Use when the agent is designing Swift types and choosing between structs (value types) and classes (reference types), implementing copy-on-write (COW), reasoning about value semantics, mutating methods, inout parameters, equality and hashing for value types, nesting structs and classes, Codable with value types, or is diagnosing "modifying a copy changed the original", unexpected copying cost, "self is immutable", "cannot assign property on immutable value", or identity vs equality confusion. Covers value semantics, the struct-vs-class decision, copy-on-write implementation, mutation model, and the pitfalls of accidental sharing and hidden copies.
---

# Value Types And Copy-On-Write In Swift

Swift's distinguishing design choice is the prominence of value types (`struct`, `enum`) with value semantics, and the recurring mistake is treating that as a minor syntax preference rather than a deep semantic decision. A value type is copied on assignment, so two variables hold independent copies — mutating one never affects the other. This eliminates whole classes of aliasing bugs that plague reference-type languages, but it introduces its own judgment problems: when is a copy expensive (and how do you avoid it with copy-on-write), when does a `struct` that contains a `class` lose value semantics, when does mutation require `mutating` and why, and when should a type be a `class` despite the appeal of value semantics? The judgment problem is to choose value vs reference by the semantics the type needs (identity vs value, copying cost, sharing requirements), to preserve value semantics through nested types and COW, and to recognize when a struct is silently a reference in disguise.

Agents default to `struct` because Swift's guidance and the standard library favor it, then hit "self is immutable" in a method, accidental sharing when a struct holds a class, or surprising performance from large copies. The remedy is to decide struct-vs-class by semantics first, to implement COW where a value type wraps expensive storage, to mark mutation explicitly, and to verify value semantics hold through the type's composition.

## Core Rules

### Choose Value Or Reference By Semantics, Not By Habit

A value type (`struct`/`enum`) is copied on assignment; each copy is independent, identity is irrelevant, and equality is by value. A reference type (`class`) is shared; copies point to the same instance, identity matters (`===`), and equality must be defined deliberately. Choose by what the type *means*:

- Use a value type for data: models, points, sizes, ranges, configuration — things where two equal values are interchangeable and copying is the right default.
- Use a reference type when identity or shared mutable state is essential: a view, a database connection, a file handle, a coordinator that others observe.
- Use a reference type when Objective-C interop requires it (`NSObject` subclasses), or when the type participates in an inheritance hierarchy.

The Swift standard library is overwhelmingly value types (`Array`, `Dictionary`, `Set`, `String`, `Int`) precisely because data should copy. Default to value; reach for reference with a reason.

### Preserve Value Semantics Through Composition

A `struct` has value semantics only if all its stored properties have value semantics. A struct that holds a `class` reference shares that reference across copies — two struct copies point to the same class instance, so mutating through one is visible to the other, breaking value semantics. To preserve value semantics when wrapping a reference type, either make the reference immutable (so there is no mutation to share), implement copy-on-write so mutations copy first, or hold the data in value types instead. A `struct` wrapping a `NSMutableString` or a mutable Objective-C object silently shares state; this is the most common value-semantics bug.

- Audit each stored property: value types compose; reference types require COW or immutability to preserve value semantics.
- `let` on a struct property makes it immutable, but a `let` class reference still allows mutation of the instance's mutable properties — `let` does not freeze a class's contents.

### Implement Copy-On-Write For Value Types Wrapping Expensive Storage

A value type that wraps large storage (a buffer, a class-backed store) should not copy on every assignment — that defeats performance. Copy-on-write (COW) makes the copy cheap (share the storage) and only duplicates the storage when a mutation actually occurs. The standard library's `Array`, `Dictionary`, `Set`, and `String` all use COW. For your own type, implement COW by checking whether the storage is uniquely referenced (`isKnownUniquelyReferenced(&storage)`) before mutating; if not uniquely referenced, copy first.

- The pattern: `mutating func update(...) { if !isKnownUniquelyReferenced(&storage) { storage = storage.copy() }; storage.update(...) }`.
- COW applies only to reference-typed storage; pure value-type storage already copies only on mutation via Swift's deferred-copy optimization for collections.
- A type that wraps a `class` without COW shares the instance across copies; a type with COW preserves value semantics and performance.

### Mark Mutation Explicitly With mutating, And Understand self In Methods

Methods that modify a value type's properties must be declared `mutating`, and they can be called only on a `var` (not a `let`) variable. This makes mutation visible at the call site and the declaration, a deliberate contrast with reference types where any method can mutate the instance. `inout` parameters pass a value by reference for the duration of a call (`func swap(_ a: inout T, _ b: inout T)`), allowing a function to mutate the caller's copy without copying. The `mutating`/`let`/`inout` discipline is the value-type analogue of "const correctness" — it prevents accidental mutation and makes ownership explicit.

- A `let` struct is fully immutable; a `var` struct allows `mutating` methods.
- `mutating` methods can reassign `self` entirely (useful for wrapping an underlying transformation).
- Properties of value type cannot be mutated through a `let` reference; this is a compile error, not a runtime surprise.

### Implement Equatable And Hashable Consistently For Value Types

Value types derive meaning from their data, so `Equatable` (and often `Hashable`) should be synthesized or implemented to compare all relevant properties. Two value instances with equal properties should be `==`, and equal values must hash equally. Inconsistent equality and hashing (equal but different hash, or hash without equality) breaks dictionary keys, set membership, and diffing. For reference types, decide whether equality is identity (`===`) or value-based, and document it; for value types, value-based equality is the default expectation.

- Synthesize `Equatable`/`Hashable` (`struct P: Equatable, Hashable`) when all properties are `Equatable`/`Hashable`.
- Ensure `a == b` implies `a.hashValue == b.hashValue` (the Hashable contract).
- Exclude identity-irrelevant or volatile fields (a cache, a timestamp) from equality if they should not affect comparison.

### Avoid Unintended Copies And Large Struct Performance Traps

Most value-type copies are cheap (COW collections, small structs), but a large struct (many properties, or nested without COW) copied in a hot loop or passed by value through many layers can dominate cost. Pass large structs by `inout` when mutation is needed, or rely on Swift's optimization (small structs are often passed in registers); measure before optimizing. Beware implicit copies in closures (capturing a large struct copies it) and in generic constraints. The remedy is measurement: profile, and apply COW or `inout` where a copy is the bottleneck.

## Common Traps

### Struct Holding A Class Loses Value Semantics

A `struct` with a `class` property shares the instance across copies; mutating through one copy affects the other. Add COW or hold value types.

### let Does Not Freeze A Class's Mutable State

`let obj = MyClass()` prevents reassigning `obj` but `obj.mutableProp = x` still mutates the shared instance. `let` on a reference fixes the reference, not the instance.

### mutating Called On A let

`let point = Point(); point.move()` is a compile error because `move` is `mutating` and `point` is immutable. Use `var`.

### Copy-On-Write Missing, Causing Full Copies

A value type wrapping a class without COW copies the whole backing store on assignment (if implemented naively) or shares it (losing value semantics). Implement COW with `isKnownUniquelyReferenced`.

### Accidental Large Copy In A Closure

Capturing a large struct in an escaping closure copies it. Capture only what is needed, or use a reference type / `inout` where appropriate.

### Equality Without Hashable Consistency

Equal values with different hashes break `Set`/`Dictionary`. Ensure `==` implies equal hash.

### Choosing Class When Value Semantics Were Needed

A model as a `class` introduces aliasing: two references to the same instance, a mutation visible everywhere. Use a `struct` for data.

### Inheritance Need Forced Into A Struct

A type that needs inheritance must be a `class`; a struct cannot inherit (it can only conform to protocols). Do not fight the model with composition if genuine inheritance is required.

## Self-Check

- [ ] Each type is chosen as value or reference by semantics (data/identity, sharing, inheritance, interop), with value as the default and a documented reason for any `class`.
- [ ] Value semantics hold through composition: every stored property of a struct is a value type, or a reference type made safe by immutability or copy-on-write.
- [ ] Value types wrapping expensive storage implement COW via `isKnownUniquelyReferenced`, so copies are cheap and mutations copy first; no naive full-copy or shared-mutation remains.
- [ ] Mutation is explicit: `mutating` methods are called only on `var`, `let` structs are fully immutable, and `inout` is used for by-reference mutation without copying.
- [ ] `Equatable`/`Hashable` are synthesized or implemented consistently (equal values hash equally, volatile/irrelevant fields excluded), so dictionary keys and set membership behave.
- [ ] No `let` reference is assumed to freeze a class's mutable contents, and no struct-with-class silently shares state.
- [ ] Large-struct performance has been considered (hot loops, closure capture, generic boundaries) and addressed with `inout`/COW where measurement showed a copy bottleneck.
- [ ] The type design has been considered under copying, mutation, equality, and interop, and remains correct and performant.
