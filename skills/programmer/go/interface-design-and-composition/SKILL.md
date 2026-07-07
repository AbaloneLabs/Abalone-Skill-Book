---
name: interface_design_and_composition.md
description: Use when the agent is designing or reviewing Go interfaces, deciding where to define them (consumer vs producer side), choosing interface size (small vs large), embedding structs or interfaces, writing mock-friendly code, implementing io.Reader/Writer/Closer or stdlib interfaces, designing packages and APIs, or diagnosing over-abstraction, nil-interface bugs, or packages that are hard to use. Covers accept-interfaces-return-structs, interface satisfaction, composition over inheritance, the empty interface and generics tradeoff, and avoiding premature or leaky abstractions.
---

# Interface Design And Composition

Go's interfaces are structural and implicit: a type satisfies an interface by having the right methods, with no `implements` declaration, and an interface value holds a (type, value) pair that is `nil` only when both are nil. These three properties — structural satisfaction, implicit implementation, and the two-word interface value — are the source of Go's famous flexibility and also of its most characteristic bugs and design smells. A nil concrete pointer assigned to an interface is not a nil interface, and calling a method on it dispatches into a method that receives a nil receiver and often panics. An interface with eight methods is a god-object that every implementer must satisfy in full. An interface defined next to its only implementation is indirection that buys nothing.

The judgment problem is not "how do I declare an interface" but "where should this interface live, how big should it be, and is it pulling its weight." Agents coming from Java or C# tend to over-produce interfaces (one per type, defined by the producer, used to enable mocking) and to build deep inheritance-like hierarchies through embedding. Agents new to Go tend to reach for `interface{}` (now `any`) or `reflect` to express genericity that generics now handle directly. Both produce code that compiles, looks idiomatic to the author, and is hard to maintain, hard to test, and hard to use.

## Core Rules

### Define Interfaces At The Consumer, Not The Producer

The Go idiom is "accept interfaces, return structs." The strongest place to define an interface is in the package that consumes it, sized to exactly the methods that package calls. This keeps interfaces small, makes them reflect real usage rather than speculative generality, and decouples the consumer from any one implementation. The producer package returns a concrete struct (or pointer), which the caller can use directly or pass to a function whose parameter is the consumer-side interface.

The trap is the producer-side interface: a package that declares a big interface and a single implementation next to it. This is the Java-style pattern and it almost always fails to pull its weight — there is one implementation, the interface mirrors the struct's method set exactly, and the only "benefit" is enabling a mock in tests. Prefer to extract a small consumer-side interface in the test (or in the consuming package) instead. The exception is stdlib-style libraries that genuinely define a contract many packages implement (`io.Reader`, `error`, `fmt.Stringer`); those are worth defining at the producer because the contract is the point.

### Keep Interfaces Small, Ideally One Method

The most reusable interfaces in Go are tiny: `io.Reader` is one method, `error` is one method, `fmt.Stringer` is one method. A one-method interface is trivially mockable, trivially implementable, and composes freely. As interfaces grow, every method is a burden every implementer must carry, and the chance that any two types satisfy the same large interface shrinks. The Go community rule of thumb is "the bigger the interface, the weaker the abstraction."

When you find a large interface, ask whether it is really one concept or several. A `Storage` interface with `Get`, `Put`, `Delete`, `List`, `Batch`, `Watch` is often several interfaces (`Reader`, `Writer`, `Lister`, `Watcher`) that callers can accept in smaller slices. Splitting lets a caller that only reads depend only on `Reader`, narrowing coupling. Do not split reflexively — sometimes the methods form one coherent contract — but treat a 5+ method interface as a design smell to justify.

### Beware The Nil Interface Value

An interface value is `nil` only when both its type and value are nil. Assigning a nil concrete pointer to an interface produces a non-nil interface (type is set, value is nil), and a method call on it dispatches normally — the receiver inside the method is a nil pointer, which panics if the method dereferences it. This is the single most common Go bug that surprises programmers from other languages, because `if err != nil` looks like it should catch it but does not.

Defend with two habits. First, functions that return an interface should return a literal `nil` (not a typed nil pointer) on the error path — return `nil, err`, not `var p *T; return p, nil`. Second, methods that can be called on a nil receiver should handle nil explicitly at the top (the classic pattern is a `Size()` method that returns 0 for a nil slice/map). If a type's methods are all safe on a nil receiver, document it; if not, never let a nil concrete value escape into an interface.

### Prefer Composition And Embedding Over Inheritance

Go has no inheritance; it has embedding. Embedding a struct or interface promotes its fields and methods into the outer type, which is composition with delegation, not subclassing. The embedded value is a field, not a base class; method calls on the outer type delegate to the inner, and the inner has no special access to the outer. This avoids the fragile-base-class and diamond-inheritance problems but requires you to think in terms of "has-a" and "delegates-to," never "is-a."

Two embedding pitfalls recur. First, embedding a pointer to allow override (the "decorator" pattern) is fine, but overriding a promoted method while the embedded type calls its own version internally does not call your override — Go methods are statically dispatched on the receiver type, not dynamically resolved through the embedding chain. Second, embedding an interface (rather than a struct) is a powerful way to partially satisfy an interface and forward calls, but it also means a nil embedded interface produces the nil-interface bug above when methods are called. Use embedding deliberately, and prefer it over copying fields when the lifecycle is shared.

### Do Not Reach For `any` Or `reflect` When Generics Will Do

Before Go 1.18, `interface{}` (now `any`) plus type assertions or `reflect` was the only way to write container-like generic code, and it traded type safety and performance for reuse. With generics (`func Map[T, U any](xs []T, f func(T) U) []U`), most of those uses are better expressed with type parameters that preserve compile-time type checking and avoid boxing. Prefer generics for data structures, algorithms, and utility functions where the element type varies but the logic is uniform.

`any` and `reflect` remain correct for genuine runtime polymorphism: decoding arbitrary JSON, plugin systems, serialization frameworks, ORM mapping. But using them to fake generics in 2024+ code is a smell. The tradeoff to weigh is that generics add compile-time complexity and can produce verbose error messages; do not over-parameterize simple code. A good test: if every call site would pass the same concrete type, you do not need a type parameter.

### Make Interfaces Mockable Without A Mock Framework

Because Go interfaces are structural, testing does not need a mocking framework. Define a small consumer-side interface in the package under test (or in the test file), implement a tiny fake type that satisfies it, and pass the fake. This keeps tests readable, avoids generated code and framework lock-in, and forces the interface to stay small (a 12-method interface is painful to fake by hand, which is itself a signal to split it).

The corollary is that if you cannot easily test a function without a heavy mock, the function probably depends on too large an interface or on a concrete dependency that should be behind a smaller one. Testability and interface size are coupled: hard-to-mock is a design signal, not a tooling problem.

### Respect Interface Satisfaction And Sealing

A type satisfies an interface implicitly; you do not declare it. To document intent and catch drift, use a compile-time assertion: `var _ io.Reader = (*MyReader)(nil)`. This fails to compile if the type ever drifts away from the interface, which is invaluable for types meant to satisfy a contract.

For "sealed" interfaces — interfaces whose implementers should all live in one package, used to express a closed set of variants (a sum type) — use the unexported-method trick: give the interface an unexported method, so only types in the same package can satisfy it. This is Go's answer to algebraic data types and lets you build exhaustive switches over the known variants while preventing external implementations. Use it when the set of cases is genuinely closed and you want the compiler to flag missing cases.

### Keep Package Boundaries Aligned With Interface Boundaries

Interfaces are a package-boundary tool. An interface lets one package depend on a contract rather than on another package's concrete types, which breaks import cycles and reduces coupling. When designing packages, think about what each package exports (concrete types and small interfaces that are contracts) and what it imports (other packages' interfaces, accepted as parameters). A package that imports a concrete type from another package and wraps it in its own interface is often hiding a cycle or a coupling that should be made explicit.

Avoid the "interface wrapper that adds nothing" smell: a type that holds another type and forwards every method. If the wrapper does not add behavior (validation, logging, caching, lifecycle), it is indirection for its own sake. Delete it and depend on the concrete type directly, or extract the real consumer-side interface.

## Common Traps

### Returning A Nil Concrete Pointer As An Interface

`var p *T; return p, nil` returns a non-nil interface that panics on method call. Return literal `nil` for the interface on error paths, and check `if err != nil` before constructing values.

### Producer-Side Single-Implementation Interfaces

A package that declares `type Service interface { ... }` with exactly one `serviceImpl` next to it, defined only to enable a mock. Define the interface at the consumer, sized to what the consumer calls.

### God-Interfaces With Many Methods

An interface with 6+ methods that every implementer must fully satisfy. Split into single-purpose interfaces (`Reader`, `Writer`, `Closer`) that callers accept in narrow slices.

### Embedding To Fake Inheritance

Embedding a struct and expecting override semantics like a subclass, then discovering the embedded type's internal calls do not dispatch to your override. Go has composition, not inheritance; model accordingly.

### Using `any` And Type Switches Instead Of Generics

Pre-generics patterns (`interface{}` containers with type assertions) that lose type safety and add boxing. Prefer type parameters when the logic is uniform over the type.

### Mocking Concrete Types Instead Of Extracting Interfaces

Reaching for a mock framework or `monkey`-patching because a dependency is a concrete struct. Extract a small consumer-side interface and pass a hand-written fake.

### Embedding An Interface And Forgetting It Can Be Nil

`type Server struct { Store Store }` where `Store` is an interface and is left nil; the first method call panics inside the embedded interface. Initialize embedded interfaces or guard them.

### Over-Abstracting Simple Code

Wrapping every concrete type in an interface "for flexibility" when there is one caller and one implementation. YAGNI; depend on concretes until a second consumer appears.

## Self-Check

- [ ] Interfaces are defined at the consumer side, sized to exactly the methods that consumer calls; producer packages return concrete types unless the interface is a genuine multi-implementation contract.
- [ ] Interfaces are small (ideally one method, rarely more than three); any interface with five or more methods has been justified or split into single-purpose contracts.
- [ ] Functions returning an interface return literal `nil` (not a typed nil pointer) on error paths, and nil-receiver-safe methods handle nil explicitly at the top.
- [ ] Composition uses embedding as "has-a / delegates-to," never as fake inheritance; promoted-method overrides and nil embedded interfaces have been reviewed.
- [ ] Generic data structures, algorithms, and utilities use type parameters rather than `any` + type assertions/`reflect`; `any` and `reflect` are reserved for genuine runtime polymorphism.
- [ ] Tests use small hand-written fakes against consumer-side interfaces, and any dependency that was hard to mock has been narrowed to a smaller interface.
- [ ] Interface satisfaction is asserted at compile time (`var _ I = (*T)(nil)`) for types meant to satisfy a contract, and sealed interfaces use unexported methods where the variant set is closed.
- [ ] Package boundaries align with interface boundaries: imports depend on contracts, not concrete types; interface wrappers that forward without adding behavior have been removed.
- [ ] No interface exists solely to enable a mock for a single implementation, and no god-interface forces implementers to carry methods they do not need.
