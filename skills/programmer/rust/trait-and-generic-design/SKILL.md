---
name: trait_and_generic_design.md
description: Use when the agent is designing traits and generic APIs in Rust, choosing between generics (static dispatch) and trait objects (dynamic dispatch), defining trait bounds, using associated types vs generics, designing object-safe traits, or refactoring abstractions that have grown awkward. Also covers the failure mode of over-engineered trait hierarchies, generics that proliferate bounds and compile slowly, trait objects that pay hidden vtable cost or cannot use generic methods, traits that leak implementation details, and the tension between flexibility and the compile-time and readability costs of heavy generics.
---

# Trait And Generic Design

Traits and generics are Rust's primary abstraction tools, and their design determines whether an API is flexible, readable, and performant, or whether it becomes a tangle of bounds, wrappers, and compile times. The judgment problem is that each choice — generics vs. trait objects, static vs. dynamic dispatch, associated types vs. generic parameters, where to place bounds — trades flexibility against cost, and the costs are often invisible until the abstraction is widely used. A generic function with a long bound list (`T: Clone + Send + Sync + 'static + Foo + Bar`) is hard to call, hard to read, and slow to compile; a trait object (`dyn Trait`) hides the concrete type behind a vtable, paying an indirection cost and excluding traits with generic methods; a trait designed without object safety cannot be used as a trait object at all, even when that would be the natural choice. The discipline is to design traits around coherent behavior (not capability bags), to choose generics for performance and trait objects for type erasure, to place bounds where they are needed (not preemptively at the definition), to prefer associated types when the type relationship is functional, and to resist over-abstraction until a second concrete need justifies it.

Agents tend to over-abstract (building trait hierarchies for a single implementation) or to choose dispatch without considering the cost. The harm appears as APIs burdened with bound lists that callers must satisfy, as compile times that grow with heavy generic monomorphization, as trait objects that pay unexpected vtable cost or that exclude desired methods, and as traits that leak implementation details or that cannot evolve without breaking downstream. The judgment is to abstract on observed need, to match the dispatch mechanism to the use case, to keep bounds minimal and local, to use associated types for functional type relationships, and to design traits for object safety when trait objects are plausible. Good trait design is invisible; bad trait design infects every caller.

## Core Rules

### Design Traits Around Coherent Behavior, Not Capability Bags

A trait should express one coherent behavior with a clear contract (invariants, error cases, what implementing it promises), not a grab-bag of methods that happen to be convenient together. Coherent traits compose; capability bags force every implementor to stub methods they do not need.

- **One trait, one coherent responsibility.** A `Storage` trait that does get, put, delete, list, batch, and transaction is probably several traits; an implementor that cannot transact should not have to stub `transact`.
- **Document the contract, not just the signatures.** A trait method's signature says what to call; the contract says what it promises (performance characteristics, error semantics, ordering guarantees, thread safety). Implementors and callers rely on the contract.
- **Prefer small, composable traits to large ones.** Small traits combine to express rich behavior and let implementors opt in granularly; large traits force all-or-nothing implementation.
- **Avoid default methods that quietly change semantics.** A default method that does something subtle (locking, caching) can surprise implementors who override it; defaults should be straightforward conveniences.

### Choose Generics For Performance, Trait Objects For Type Erasure

Generics (static dispatch) monomorphize a separate copy of the code per concrete type, which is fast (no indirection, inlinable) but increases binary size and compile time. Trait objects (`dyn Trait`, dynamic dispatch) use one copy of the code with a vtable indirection, which is slower but allows heterogeneous collections and type erasure.

- **Use generics when the concrete type is known at the call site and performance matters.** Static dispatch enables inlining and avoids vtable cost; for hot paths, generics are the right choice.
- **Use trait objects when you need type erasure** — a collection of different types behind one interface (`Vec<Box<dyn Trait>>`), or a return type that hides the concrete implementation.
- **Be aware of the costs.** Heavy generic monomorphization inflates binary size and compile time; trait objects pay a per-call vtable indirection and inhibit inlining. Match the choice to the call frequency and the need for type erasure.
- **You can offer both.** A generic public function that internally uses a `dyn Trait` implementation gives callers static dispatch while keeping the implementation shared.

### Place Bounds Where Needed, Not Preemptively

Trait bounds should appear where they are required (on the function that needs the capability), not preemptively on every definition. Bounds that propagate upward burden every caller; bounds placed locally keep the API usable.

- **Put bounds on the items that need them.** A `fn` that clones its argument needs `T: Clone`; a struct that merely holds a `T` does not. Do not bound the struct preemptively.
- **Avoid bound cascades.** A bound on a struct forces the bound on every impl and every function using the struct, cascading upward; prefer to bound functions and methods, leaving the struct unbound.
- **Keep bound lists short.** A long bound list signals an abstraction trying to do too much; consider splitting the function or reducing what it requires.

### Prefer Associated Types For Functional Type Relationships

When a trait has a type that is determined by the implementing type (a storage trait's key type, an iterator's item type), an associated type expresses that the relationship is functional (one implementor, one associated type). A generic parameter expresses that the relationship is parametric (one implementor, many parameter choices).

- **Use associated types when each implementor has exactly one related type.** `Iterator::Item` is associated because each iterator yields one item type; this is the common case.
- **Use generic parameters when an implementor can work with multiple type parameters.** A `From<T>` is generic because one type can convert from many; an associated type would force one conversion.
- **Associated types simplify bounds.** With an associated type, callers bound `T::Item` rather than threading a second parameter; with a generic parameter, every use must carry it.

### Design For Object Safety When Trait Objects Are Plausible

A trait is object-safe if it can be used as `dyn Trait`, which requires (among other things) that it has no generic methods and that `Self` does not appear in positions that vary by implementor. If trait objects are plausible for the trait, design for object safety from the start; retrofitting it is breaking.

- **Avoid generic methods on traits you may want to use as objects.** A generic method makes the trait non-object-safe; if a method needs to be generic, consider a separate trait or a different design.
- **Be careful with `Self` in method signatures.** Methods returning `Self` or taking `Self` by value are not object-safe; use `&Self` or boxed returns where trait objects are plausible.
- **Decide upfront whether the trait will be used as an object.** Object safety is easier to design in than to retrofit; if there is any chance of type erasure, design for it.

### Resist Over-Abstraction Until A Second Concrete Need Justifies It

The temptation is to build a trait for the first implementation, anticipating flexibility. A trait with one implementation is a guess; it usually predicts the wrong abstraction, and it adds indirection and bounds for no benefit. Wait for a second concrete need, then abstract what the two have actually in common.

- **Abstract on observed need, not anticipated need.** Two concrete implementations reveal the right abstraction; one implementation reveals only a guess.
- **Prefer concrete types until abstraction is justified.** A concrete struct is simpler, faster to compile, and easier to evolve than a trait; reach for the trait when a second need appears.
- **When you do abstract, abstract the shared behavior.** Two implementations show which methods and which bounds are actually needed; abstract those, not the speculative extras.

## Common Traps

### Over-Engineered Trait Hierarchy For One Implementation

A trait hierarchy built for a single implementation, predicting the wrong abstraction and adding indirection and bounds for no benefit. Abstract on observed need; wait for a second concrete implementation.

### Long Bound Lists Burdening Callers

A generic function with a long bound list that every caller must satisfy, hard to call and slow to compile. Keep bounds minimal and local; put them on the items that need them, not preemptively on structs.

### Heavy Monomorphization Inflating Compile Times

Generics everywhere, monomorphizing a copy per concrete type, inflating binary size and compile time. Use trait objects for rarely-called or heterogeneous code; offer generic-and-dyn hybrids.

### Trait Object With Hidden Vtable Cost Or Excluded Methods

A `dyn Trait` paying vtable cost on a hot path, or a trait that turns out not to be object-safe when a trait object is needed. Match dispatch to call frequency; design for object safety when trait objects are plausible.

### Non-Object-Safe Trait Retrofit

A trait with generic methods or `Self` in signatures that cannot be used as `dyn Trait` when type erasure becomes needed, requiring a breaking redesign. Decide upfront whether the trait will be an object; design accordingly.

### Capability-Bag Trait Forcing Stub Implementations

A large trait combining many capabilities, forcing implementors that lack some to stub methods or panic. Split into small composable traits; one trait, one responsibility.

### Associated Type Vs Generic Parameter Confusion

Using a generic parameter where the relationship is functional (forcing callers to thread a parameter that is always determined by the implementor), or an associated type where the relationship is parametric. Use associated types for functional relationships, generic parameters for parametric ones.

## Self-Check

- [ ] Traits are designed around coherent behavior (one responsibility per trait, documented contracts covering invariants/error semantics/performance/ordering/thread-safety), small and composable rather than capability bags that force stub implementations, with default methods that are straightforward conveniences rather than quiet semantic changes.
- [ ] Dispatch is matched to the use case: generics (static dispatch) for hot paths where the concrete type is known at the call site, trait objects (dynamic dispatch) for type erasure (heterogeneous collections, hidden implementations), with awareness of monomorphization's compile-time/binary cost and vtable's per-call cost, and generic-and-dyn hybrids where appropriate.
- [ ] Bounds are minimal and local: placed on the items that need them (functions/methods) rather than preemptively on structs, avoiding bound cascades that burden every caller, with short bound lists signaling coherent abstractions.
- [ ] Associated types are used for functional type relationships (each implementor has one related type, simplifying bounds) and generic parameters for parametric relationships (one implementor, many parameter choices), not confused.
- [ ] Traits that may be used as objects are designed for object safety from the start (no generic methods, careful use of `Self` in signatures), because retrofitting object safety is breaking.
- [ ] Abstraction follows observed need: concrete types are preferred until a second concrete implementation justifies the trait, and the trait abstracts the shared behavior the two reveal rather than speculative extras.
- [ ] The highest-risk cases were verified — a hot path using static dispatch, a heterogeneous collection using trait objects, a bound list kept short by local placement, and a trait designed object-safe before it was needed as an object — not only the clean single-implementation path.
