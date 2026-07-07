---
name: generics_and_type_erasure.md
description: Use when the agent is writing or reviewing Java generics — declaring generic classes and methods, bounded type parameters, wildcards (? extends / ? super), designing library APIs with generics, mixing generics with arrays, reflection over parameterized types, implementing Comparable/Comparator, or diagnosing unchecked warnings, ClassCastException at runtime due to heap pollution, raw type misuse, or wildcard capture errors. Covers type erasure, bridge methods, the PECS principle, reified vs erased types, and the hazards of arrays being covariant while generics are invariant.
---

# Generics And Type Erasure

Java generics were added in Java 5 as an erasure-based, compile-time-only type system layered on top of the pre-existing JVM. This design choice — erasure rather than reification — is the single fact that explains almost every quirk, limitation, and footgun in Java generics. An `ArrayList<String>` and an `ArrayList<Integer>` are the same class at runtime; the type parameter `String` exists only in the source and the class file's metadata, not in the running object. This makes generics backward-compatible and cheap, and it also means you cannot do `new T()`, you cannot do `instanceof List<String>`, you cannot have `List<int>` (primitives are not objects), and a `ClassCastException` can surface far from the line that actually inserted the wrong type.

The judgment problem is not "how do I write `List<T>`" but "how do I design a generic API so that callers get compile-time type safety, the compiler's warnings actually mean something, and the erasure boundary does not create a hole where wrong types slip through." Agents tend to either overuse wildcards (producing APIs no one can call without capture errors) or ignore them entirely (producing APIs that are too rigid to use), and they routinely reach for raw types or `@SuppressWarnings("unchecked")` to silence warnings whose cause they have not understood. The cost is APIs that look type-safe and fail at runtime with `ClassCastException`, or APIs that are so rigid they force callers into unchecked casts.

## Core Rules

### Internalize That Generics Are Erased, Not Reified

Type erasure means: at runtime, `List<String>`, `List<Integer>`, and `List<?>` are all the same class (`ArrayList`), with the same methods, and the type parameters are gone. The compiler inserts casts at use sites to give the illusion of reified types, but the runtime types carry no parameter information. Consequences you must accept as design constraints:

- You cannot `new T()` — there is no `T` at runtime; pass a `Supplier<T>` or `Class<T>` instead.
- You cannot do `instanceof List<String>` — only `instanceof List` (raw); the parameter is erased.
- You cannot have `class Foo<T> { static T bar(); }` — statics are shared across all parameterizations, so `T` is meaningless there.
- You cannot overload `void f(List<String>)` and `void f(List<Integer>)` — after erasure they have the same signature.
- You cannot have `List<int>` — type parameters must be reference types; primitives box.

Design around these. When you need a runtime type token, pass `Class<T>` (or, for richer reflection, `TypeToken`/`ParameterizedTypeReference`). When you need a fresh instance, pass a factory. When you need to distinguish parameterizations at runtime, you need a discriminator field, not the type parameter. Fighting erasure produces convoluted code; accepting it produces clean APIs.

### Use Bounded Type Parameters To Express Real Constraints

`<T>` with no bound means "any reference type," which is rarely what you want. Bounds (`<T extends Comparable<T>>`, `<T extends Number & Serializable>`) express that `T` must support certain operations, and they let you call those operations inside the method. A bound is a contract: it tells the compiler (and the reader) what the method assumes about `T`, and it lets the method use `T` accordingly.

Choose bounds to be as precise as the logic requires and no more. A method that sorts needs `<T extends Comparable<? super T>>` (note the wildcard — see PECS below). A method that just puts things in a `List<Object>` needs no bound. Over-bounding (`<T extends Object>` for no reason) adds noise; under-bounding (no bound when the method calls `T.compareTo`) fails to compile. Multiple bounds are allowed (`<T extends Number & Comparable<T>>`), with at most one class bound listed first.

### Apply PECS: Producer `extends`, Consumer `super`

The wildcard rules are the most-misunderstood part of Java generics, and PECS (Producer Extends, Consumer Super) is the mnemonic that resolves them. If a parameterized parameter is a source you read from, use `? extends T` (it produces `T`s). If it is a sink you write to, use `? super T` (it consumes `T`s). If you both read and write, you cannot use a wildcard — you need the exact type `T`.

`Collection<? extends T>` lets the caller pass a `List<Dog>` when you only need to read `Animal`s — without it, the caller would be forced to pass a `Collection<Animal>` and could not reuse their `List<Dog>`. `Collection<? super T>` lets the caller pass a `List<Animal>` when you want to add `Dog`s — without it, the caller could only pass `Collection<Dog>`. The `Comparator<? super T>` in `Collections.sort` and the `Comparator<? super T>` in `Stream.sorted` are PECS in action: the comparator consumes `T`s, so it accepts a comparator for any supertype. Get PECS right and your generic APIs become flexible; get it wrong and callers fight capture-of-? errors.

### Eliminate Unchecked Warnings By Understanding Them, Not Suppressing Them

An unchecked warning (`unchecked cast`, `unchecked call`, `unchecked conversion`) means the compiler cannot verify type safety because erasure has hidden the information. These warnings are real signals: each one marks a place where a wrong type could slip through and surface as a `ClassCastException` far away. The correct response is to understand why the warning appears and fix the design so it disappears — usually by fixing a raw type, adding a type parameter, or correcting a wildcard.

`@SuppressWarnings("unchecked")` is acceptable only when you have understood the warning, proved the cast is actually safe (often because of invariants the compiler cannot see), and added a comment explaining the proof. Blanket suppression at class or method level hides real bugs and is a smell. Treat every unchecked warning as a question to answer, not noise to silence. A codebase with dozens of suppressed warnings has not been audited for type safety.

### Avoid Heap Pollution: It Pushes ClassCastException Far From The Bug

Heap pollution occurs when a variable of a parameterized type refers to an object that is not of that parameterized type — for example, a `List<String>` that actually contains an `Integer`, usually because a raw type or an unchecked cast let the wrong type in. The pollution is silent at insertion (erasure means the wrong type is stored without complaint) and surfaces only when a later read casts the value to `String`, throwing `ClassCastException` on a line that did nothing wrong.

The defenses are: never use raw types (`List` instead of `List<?>` or `List<T>`), never let unchecked casts insert into a parameterized collection, and use `Collections.checkedList` and friends when you must interoperate with untrusted/raw code (they add a runtime type check at the collection boundary). Heap pollution is especially dangerous in long-lived shared collections and caches, where the wrong value can sit for hours before being read. A `ClassCastException` at a read site is almost always a heap-pollution bug planted elsewhere.

### Do Not Mix Arrays And Generics

Arrays are covariant and reified; generics are invariant and erased. The combination is unsound, which is why `new List<String>[]` is a compile error and `T[]` in a generic type is a heap-pollution source. Covariance means `Object[]` can hold any array of reference type, so `Object[] o = new Integer[10]; o[0] = "x";` compiles and throws `ArrayStoreException` at runtime — the array knows its real type. Generics cannot do this (no `ArrayStoreException` equivalent), so `List<String>[]` would let wrong types in with no runtime check.

Avoid generic arrays. Prefer `List<List<String>>` over `List<String>[]`, `Collection<T>` over `T[]`, and `ArrayList<T>` (backed by `Object[]` internally, with casts at reads) over hand-rolled generic arrays. When you must create an array of a generic type (some APIs require it), use `@SuppressWarnings("unchecked")` with a comment, or `Array.newInstance(cls, n)` with a `Class<T>`. Mixing arrays and generics is a recurring source of heap pollution and `ArrayStoreException`; treat any `T[]` in generic code as suspicious.

### Design Generic APIs For Caller Ergonomics, Not Implementer Convenience

A generic API should let callers express what they have and get what they need with minimal ceremony. This usually means: take the most general input the logic allows (wildcards, PECS), return the most specific output the logic can produce (concrete parameterized types), and let type inference do the work so callers do not write explicit type arguments. The target-typed inference introduced in Java 8+ means most generic method calls need no explicit `<T>` — write the API so inference works.

Avoid the trap of parameterizing for the implementer's convenience: a method that takes `<T extends FooBar>` just because the implementation uses `FooBar` methods, when the caller only cares about a narrower interface. Bounds and wildcards should reflect what the caller needs to pass, not what the implementation happens to call. Test generic APIs by writing calling code: if a caller has to write awkward casts, capture-error workarounds, or raw types, the API design is wrong.

### Use Type Tokens When You Need Runtime Type Information

Because generics are erased, generic code that needs runtime type information (creating instances, checking types, deserializing) must be given a type token explicitly. The options are `Class<T>` (simple, loses generic parameters of `T`), `TypeToken<T>`/`ParameterizedTypeReference<T>` (Guice/Guava/Spring, preserves full parameterization via subclassing), or a `Supplier<T>`/`Function` factory. Choose based on how much type information you need: `Class<T>` for simple instantiation, `TypeToken` for `List<String>`-shaped types, a factory when construction has side effects or dependencies.

This is the standard escape hatch for erasure, not a workaround to be ashamed of. Generic serialization frameworks, mappers, and DI containers all rely on type tokens. The discipline is to require the token at construction (so it is always available) rather than trying to recover it later via reflection on erased types (which fails for parameterized types).

## Common Traps

### Reaching For Raw Types

`List` instead of `List<?>` or `List<T>` defeats type safety and generates unchecked warnings. Use `List<?>` when the element type is unknown and does not matter, or parameterize properly.

### Suppressing Unchecked Warnings Without Understanding Them

`@SuppressWarnings("unchecked")` to make the warning go away, when the cast is not actually safe. Understand each warning; suppress only with a documented proof of safety.

### Wildcard-Free Generic APIs That Callers Cannot Use

`void addAll(Collection<T>)` that forces the caller to copy their `List<Dog>` into a `List<Animal>` first. Apply PECS (`Collection<? extends T>`) so callers pass what they have.

### `new T()` Or `T[].class` In Generic Code

Erasure makes both impossible. Pass a `Supplier<T>` or `Class<T>`/`TypeToken<T>` for runtime type information.

### Overloading On Erased Parameterizations

`void f(List<String>)` and `void f(List<Integer>)` collide after erasure. Rename one method or use a single parameterized method.

### Heap Pollution From Raw Types Or Unchecked Casts

A `List<String>` silently holding an `Integer`, surfacing as `ClassCastException` at a distant read. Use `Collections.checkedList` for untrusted boundaries; never insert via raw types.

### Mixing Generic Arrays

`T[]` or `List<String>[]` causing `ArrayStoreException` or heap pollution. Prefer collections over arrays in generic code.

### Over-Bounding Type Parameters

`<T extends Object>` or `<T extends HeavyBase>` when the logic uses none of it, adding noise and restricting callers. Bound only to what the logic requires.

## Self-Check

- [ ] The API design accepts erasure as a constraint: no `new T()`, no `instanceof Parameterized<T>`, no statics referencing `T`, no `List<int>`, no overload collision; runtime type information is passed via `Class<T>`/`TypeToken`/factory where needed.
- [ ] Type parameters are bounded to exactly what the logic requires (`<T extends Comparable<? super T>>` for sorting, etc.), with no over-bounding noise and no missing bounds that force unchecked casts.
- [ ] PECS is applied: producer parameters use `? extends T`, consumer parameters use `? super T`, and read-write parameters use the exact `T`; callers can pass what they have without capture errors or manual casts.
- [ ] Every unchecked warning has been understood and either eliminated by a better design or suppressed with a documented proof of safety; there is no blanket class/method-level suppression hiding real type-safety gaps.
- [ ] No heap pollution is possible: raw types are absent, unchecked casts never insert into parameterized collections, and `Collections.checked*` guards untrusted boundaries; long-lived collections and caches are audited.
- [ ] Generic arrays (`T[]`, `List<String>[]`) are avoided in favor of collections, and any unavoidable array-of-generic is created via `Array.newInstance` with a documented reason.
- [ ] Generic APIs were tested by writing calling code: type inference works without explicit type arguments, no caller needs awkward casts or raw-type workarounds, and bounds reflect caller needs rather than implementer convenience.
- [ ] Runtime type information (instances, type checks, deserialization) is provided explicitly via type tokens required at construction, never recovered via reflection over erased parameterized types.
