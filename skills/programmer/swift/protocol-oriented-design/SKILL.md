---
name: protocol_oriented_design.md
description: Use when the agent is designing Swift protocols, adding default implementations via extensions, constraining generics with protocols, using associated types and protocol inheritance, choosing between protocols and concrete types or generics, modeling polymorphism, or diagnosing existentials, witness tables, type erasure, and protocol conformance performance issues in Swift applications.
---

# Protocol-Oriented Design

Swift is marketed as a "protocol-oriented" language, and that framing leads developers to reach for protocols the way Java developers once reached for class hierarchies. The result is a recurring pattern of over-abstracted designs: a protocol with one conformer, an associated type that forces type erasure everywhere, an existential that boxes value types and destroys their stack-allocation benefits, a default implementation that hides behavior the conformer expected to override. The judgment problem is not "how do I declare a protocol" but when a protocol is the right modeling tool versus a generic constraint, a concrete type, or a closure.

The recurring failure mode is a developer who models a small, fixed set of cases as a protocol with associated types, then spends the rest of the project fighting the type system: writing type erasers, existential-boxing values that should be cheap, and adding conformances that exist only to satisfy a generic constraint. The opposite failure is using a single concrete struct for everything and losing the ability to substitute in tests or swap implementations. Protocol-oriented design is a set of tradeoffs between static dispatch, dynamic dispatch, abstraction cost, and modeling fidelity, and the right choice depends on whether you need polymorphism, substitution, or just shared API.

## Core Rules

### Choose the modeling tool by the problem, not by habit

Swift offers several ways to share API and behavior. Match the tool to the need:

- **Concrete struct/class**: when there is one implementation and no near-term need to substitute. Simplest, fastest, most testable through dependency injection of the value.
- **Generic function with a protocol constraint**: when you need one function to work over many types while preserving static dispatch and value semantics. This is the sweet spot for protocol-oriented design.
- **Protocol existential (`any P`)**: when you need a heterogeneous collection or a stored property whose concrete type varies at runtime. Pays an abstraction cost.
- **Closure**: when you need one behavior to vary (a single function), not a whole type. Often simpler than a single-method protocol.

Default to concrete types and generics. Reach for existentials only when you genuinely need runtime polymorphism over a heterogeneous set.

### Use associated types when the relationship must be part of the contract

An associated type ties together a conforming type and a related type (e.g., a `Collection`'s `Element`) so that the API is self-consistent. The cost is that the protocol cannot be used as a simple existential without type erasure. Rules:

- Use an associated type when the conformer's API exposes a type that must vary by conformer and be consistent across the protocol's methods.
- Avoid associated types when a generic parameter on the method would do; associated types are for type-level relationships, not per-method variation.
- If you need to store `any Collection<...>`, you now need type erasure or `any` with constraints; weigh whether the abstraction is worth it.

### Provide default implementations through extensions, but document them

A protocol extension method provides a default that conformers can override. This is powerful and subtle: conformers override only if the method is a protocol *requirement*, not merely an extension method. Rules:

- If a method must be overridable, declare it as a requirement in the protocol; extension-only methods are statically dispatched and cannot be overridden by conformers (they can only shadow).
- Document which methods are requirements (overridable) versus extension conveniences (not overridable), because this distinction is invisible at the call site.
- Default implementations reduce conformance boilerplate but can hide behavior; a conformer that forgets to override a requirement silently gets the default.

### Constrain generics to express requirements, not to limit

`func f<T: P>(_ x: T)` constrains `T` to conform to `P`. Constraints express what the function needs (e.g., `Equatable`, `Hashable`) and let the compiler static-dispatch. Rules:

- Constrain to the minimal protocol that expresses the requirement; over-constraining (e.g., requiring `Comparable` when `Equatable` suffices) limits reuse.
- Use `where` clauses to add constraints to specific methods or extensions, keeping the base protocol minimal.
- Prefer generic functions over existentials when the caller knows the concrete type, to preserve static dispatch and value semantics.

### Limit protocol inheritance and composition to what is coherent

Protocols can inherit and compose (`protocol P: A, B & C`). Deep inheritance hierarchies and wide compositions create conformer burden and obscure what a protocol requires. Rules:

- Keep protocols small and focused; compose small protocols rather than defining large ones.
- A protocol that requires a dozen methods is a sign it should be several protocols or a concrete type.
- `Hashable: Equatable` is coherent; a custom `Renderable: Loadable & Cacheable & Loggable` is usually accidental complexity.

### Understand the cost of existentials (`any P`)

An existential (`any P`, formerly just `P`) boxes the value and uses a witness table for dispatch. For value types this means heap allocation and dynamic dispatch; for large or frequently-passed values this is measurable. Rules:

- Use `any P` only when you need runtime polymorphism (heterogeneous collections, stored properties of unknown concrete type).
- Prefer generics (`<T: P>`) at function boundaries to preserve static dispatch.
- In performance-sensitive code, existential boxing of small structs can still be costly over many calls; measure.

### Type erasure is a cost, not a feature

`AnyCollection`, `AnyPublisher`, and hand-written type erasers exist to hide associated types behind a concrete type. They add allocation, indirection, and sometimes virtual dispatch. Reach for type erasure when a boundary genuinely needs to abstract over associated types (e.g., a public API), not as a default. Often a generic API or a closure is simpler.

## Common Traps

### Protocol with a single conformer

If there is one conformer and no planned second, the protocol is speculative abstraction. Use the concrete type and add the protocol when a real second conformer appears.

### Existential collections that box value types

`[any Drawable]` boxes every element. If all elements are the same concrete type at runtime, a generic `[T]` or a concrete array is cheaper. Use existentials only for genuinely heterogeneous collections.

### Extension methods mistaken for overridable requirements

A conformer that "overrides" an extension-only method does not; the static type wins. Declare overridable methods as requirements, and test that dynamic dispatch actually occurs.

### Associated types that force type erasure everywhere

If every use of the protocol requires `AnyX`, the associated type is costing more than it earns. Consider replacing with a generic parameter or a concrete type.

### Over-constrained generics

`<T: P & Q & R & Hashable & Comparable>` limits reuse to types satisfying all constraints. Constrain to the minimum and push extra constraints to specific methods via `where`.

### Protocol inheritance mimicking class hierarchies

Deep protocol inheritance (`P: Q: R: S`) recreates the rigidity of class hierarchies without the implementation reuse. Prefer composition of small protocols and concrete types.

### Default implementations that silently mask missing overrides

A conformer that forgets a requirement gets the default, which may be wrong for its case. Mark critical requirements with documentation and consider runtime assertions or tests that verify each conformer provides the intended behavior.

## Self-Check

- For each protocol, is there more than one conformer or a concrete near-term plan for one? If not, is the concrete type a better choice?
- Are associated types used only for genuine type-level relationships, and have you checked whether type erasure is required at every boundary that uses the protocol?
- Are overridable methods declared as protocol requirements, with extension-only conveniences documented as non-overridable?
- Are generic constraints minimal (only what the function needs), with extra constraints pushed to `where` clauses on specific methods?
- Are protocols small and composed, rather than deep-inherited or over-broad?
- Are existentials (`any P`) used only for genuine runtime polymorphism, with generics preferred at function boundaries to preserve static dispatch?
- Where type erasure is used, is it justified by a real boundary need rather than as a default, and has the allocation/dispatch cost been considered?
- For each default implementation, is it documented whether conformers are expected to override, and are critical conformances tested for intended behavior?
