---
name: csharp_class_and_record_design.md
description: Use when the agent is designing or reviewing C# types — choosing between class and struct, reference vs value semantics, records and record structs, readonly structs, equality and GetHashCode, immutability, primary constructors, init-only properties, pattern matching and deconstruction, sealed classes, or diagnosing boxing, mutable struct bugs, equality inconsistencies, or types that should be records but are written as boilerplate-heavy classes.
---

# C# Class And Record Design

C# offers several type kinds — `class` (reference type), `struct` (value type), `record` (reference type with value-based equality), `record struct` (value type with value-based equality) — and the choice between them determines semantics, allocation behavior, equality, mutability, and how much boilerplate you write. The judgment problem is that these are not interchangeable: a mutable struct produces surprising aliasing bugs (because structs copy on assignment), a class used as a value produces reference-equality surprises, and a hand-written immutable reference type duplicates dozens of lines that a `record` generates for free. Choosing the wrong kind produces subtle bugs and verbose code that fights the language.

Agents tend to default to `class` for everything (the pre-C#-9 habit), to write mutable structs (a classic source of bugs), or to hand-write equality and immutability on classes that should be records. The judgment problem is to choose the type kind by semantics and allocation behavior, to prefer `record` for value-equal immutable data, to make structs readonly and small, and to use modern features (init-only properties, primary constructors, `required`) to reduce boilerplate without sacrificing clarity. This skill is about matching the type kind to the intent rather than defaulting to `class`.

## Core Rules

### Choose class vs struct By Semantics And Allocation, Not Habit

The class-vs-struct choice is about reference vs value semantics and heap vs stack allocation:

- **`class` (reference type)**: allocated on the heap (managed by the GC), assigned and passed by reference (two variables point to the same object), default equality is reference equality. Use for types with identity, types that are large, types that need inheritance, and types whose instances are long-lived or shared.
- **`struct` (value type)**: allocated inline (on the stack for locals, inline in containing objects/arrays), copied on assignment and on parameter passing, default equality is structural (field-by-field). Use for small, immutable, value-like types where copy semantics are correct (a Point, a Money amount, a Date).

The trap is a mutable struct: because structs copy on assignment, mutating a struct through one variable does not affect another variable that holds a copy, which surprises programmers expecting reference semantics. Mutable structs are a well-known source of bugs. Make structs readonly (or readonly struct), small (under ~16 bytes as a rule of thumb, since each copy copies all fields), and value-like. If the type needs mutation or is large, use a class.

### Prefer record For Value-Equal Immutable Reference Data

`record` (C# 9 for reference types, C# 10 for `record struct`) is a class or struct with value-based equality, immutability via init-only properties, and synthesized `Equals`, `GetHashCode`, `ToString`, deconstruction, and `with` expressions (non-destructive mutation). For any reference type that represents a value (a DTO, a configuration object, a domain value), a `record` is almost always better than a hand-written class, because it generates correct value equality and immutability in one line.

`public record Person(string Name, int Age);` gives you a type with positional parameters, init-only properties, value equality, `ToString`, deconstruction, and `person with { Age = 30 }`. Writing the equivalent class by hand is 30+ lines of boilerplate that is easy to get wrong (especially `GetHashCode`). Use `record` for immutable value data; reserve plain `class` for types with identity, mutation, or complex behavior.

### Make Types Immutable Unless They Must Mutate

Immutability eliminates aliasing bugs (no variable can surprise another by mutating shared state), makes types thread-safe by construction, and simplifies reasoning. Prefer immutable types: use `init`-only properties (`public string Name { get; init; }`), `readonly` structs, and `record` types. For mutation, use `with` expressions on records (non-destructive: produce a new copy with changed fields) rather than mutating in place.

Reserve mutable types for cases where the type genuinely has mutable state that is observed and updated over time (a builder, a buffer, a game entity). Even then, make the mutation explicit and contained. A `class` with public settable properties is often a sign of a type that should be immutable with a `with`, or that should expose behavior rather than raw setters.

### Implement Equality And GetHashCode Correctly When Overriding

If you override `==`/`!=` or `Equals`, you must also override `GetHashCode` (equal objects must have equal hash codes), and you should implement `IEquatable<T>` for type-safe, allocation-free equality. The default `object.Equals` for a class is reference equality; for a struct it is reflection-based structural equality (slow). For value-semantic types, override equality to be structural and fast.

The easiest path is to use `record`, which synthesizes correct value equality. If you must hand-write equality on a class or struct, follow the contract: `Equals` and `GetHashCode` must agree, equality must be reflexive/symmetric/transitive, and `==` must agree with `Equals`. A common bug is overriding `Equals` without `GetHashCode`, so equal objects land in different hash buckets and are "lost" in a `Dictionary`.

### Use init-only And required Properties, And Primary Constructors, To Reduce Boilerplate

Modern C# reduces boilerplate without sacrificing clarity:

- **`init` accessors** (`public string Name { get; init; }`): settable only at construction (object initializer), immutable thereafter. Replaces constructor-parameter boilerplate for immutable types.
- **`required` modifier** (C# 11, `required public string Name { get; init; }`): forces the caller to set the property in the initializer, giving compile-time enforcement that required fields are provided without a constructor.
- **Primary constructors** (C# 12, `public class Service(ILogger log, IRepo repo)`): captures parameters in scope of the class, reducing constructor boilerplate. Use for types with straightforward construction.

Adopt these where they reduce boilerplate without obscuring construction logic. Do not combine primary constructors with complex validation or multiple overloaded constructors; fall back to a hand-written constructor when construction has real logic.

### Seal Classes By Default; Use Inheritance Deliberately

C# classes are unsealed by default, but most classes should be `sealed` (cannot be inherited from) unless they are designed as a base for derivation. Unsealed classes create a compatibility surface (virtual members, protected members) that must be maintained, and unintended derivation can break invariants. Seal by default; unseal (and design the protected/virtual surface) only when derivation is an intended extension point.

Prefer composition over inheritance for code reuse, and interfaces (rather than base classes) for polymorphism. A deep class hierarchy is usually a smell in modern C#; favor small sealed classes implementing interfaces.

### Support Pattern Matching And Deconstruction For Modern Consumption

Modern C# consumers use pattern matching (`switch` expressions, property patterns, positional patterns) and deconstruction. Records support positional deconstruction automatically; for other types, add a `Deconstruct` method to enable `var (x, y) = point`. Design types so they work well with pattern matching: simple constructors, clear properties, and (for sum-type-like designs) a discriminated-union pattern via abstract base + sealed nested records or via `OneOf`/`Either` libraries.

## Common Traps

### Mutable Struct With Surprising Copy Semantics

`struct Point { public int X; public int Y; }` — `var b = a; b.X = 5;` does not change `a.X`, because the struct copied. Use a readonly struct or a class if mutation is needed.

### Class Used As A Value With Reference Equality

A `class Money` used as a dictionary key gets reference equality by default, so two equal amounts are different keys. Use a record or override equality for value semantics.

### Overriding Equals Without GetHashCode

Equal objects must have equal hash codes; overriding `Equals` without `GetHashCode` breaks `Dictionary`/`HashSet`. Use `record` or override both consistently.

### Large Struct Copied Everywhere

A struct with many fields copied on every assignment and parameter pass is expensive. Keep structs small (under ~16 bytes); use a class for larger data.

### Hand-Written Immutable Class Instead Of record

A class with 10 init-only properties, hand-written equality, and `with`-style copy methods is 50+ lines that a `record` provides in one. Use `record` for immutable value data.

### Unsealed Class Not Designed For Inheritance

An unsealed class with no protected/virtual surface invites derivation that will break, and creates a maintenance surface. Seal classes by default.

### Public Settable Properties Instead Of Behavior

A class with public getters and setters for every field is an anemic data bag; mutation is uncontrolled. Prefer immutability with `with`, or encapsulate behavior behind methods.

### Boxing A Struct By Interface Or object

A `struct` boxed into an `interface` or `object` allocates on the heap and loses copy semantics; mutating the boxed copy does not affect the original. Be careful with structs in generic collections of `object` or via interfaces.

## Self-Check

- [ ] The type kind (class, struct, record, record struct) matches the semantics: structs are small, readonly, value-like; classes have identity or are large; records provide value-equal immutable reference data.
- [ ] `record` is used for immutable value-equal reference data, replacing hand-written equality/`GetHashCode`/`with` boilerplate; plain `class` is reserved for identity/mutation/behavior.
- [ ] Structs are readonly and small (under ~16 bytes), with no mutable-struct aliasing bugs; types needing mutation or large size are classes.
- [ ] Types are immutable by default (init-only properties, readonly structs, records); mutation uses `with` expressions or is explicitly contained in types that genuinely mutate.
- [ ] Equality and `GetHashCode` are overridden consistently (or synthesized via record) for value-semantic types; `IEquatable<T>` is implemented for allocation-free equality; `==` agrees with `Equals`.
- [ ] `init`, `required`, and primary constructors are used to reduce boilerplate where construction is straightforward, with hand-written constructors retained for complex construction logic.
- [ ] Classes are sealed by default and unsealed only where derivation is a designed extension point; composition and interfaces are preferred over deep inheritance.
- [ ] Types support modern consumption (pattern matching, deconstruction) where it improves caller code, and structs are not boxed into `object`/interfaces where copy semantics would surprise.
