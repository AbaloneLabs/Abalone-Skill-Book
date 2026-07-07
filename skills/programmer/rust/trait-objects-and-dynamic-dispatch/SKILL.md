---
name: rust_trait_objects_and_dynamic_dispatch.md
description: Use when the agent is choosing between generics and dyn Trait in Rust, designing object-safe traits, building heterogeneous collections (Vec<Box<dyn Trait>>), using dyn dispatch on hot vs cold paths, implementing plugin or strategy patterns with trait objects, reasoning about vtable cost and inlining, or hitting "trait cannot be made into an object" errors. Covers object safety rules, the size and indirection of dyn values, downcasting with Any, fat pointers, lifetime and Send/Sync bounds on trait objects, and when dynamic dispatch is the right tool versus when generics or an enum are better.
---

# Trait Objects And Dynamic Dispatch

A trait object (`dyn Trait`) lets you hold values of different concrete types behind a single interface, paying a runtime vtable indirection in exchange for type erasure. This is Rust's escape hatch from monomorphization: one copy of the code, many types. The judgment problem is knowing when that tradeoff is worth it, how to design traits that can be used as objects, and how to avoid the costs and constraints that dynamic dispatch imposes.

Agents tend to reach for `dyn Trait` because it looks like an interface from other languages, build non-object-safe traits and then fight the compiler, pay vtable cost on hot paths unknowingly, or use trait objects where an enum would be simpler and faster. The harm appears as erased types that cannot be downcast when you need them, vtable indirection on performance-critical loops, traits that cannot evolve without breaking object safety, and abstractions that hide more than they reveal. The real work is matching the dispatch mechanism to the situation and designing for object safety when type erasure is plausible.

## Core Rules

### Know What You Buy And Pay With dyn Trait

A `dyn Trait` value is a fat pointer: a pointer to the data plus a pointer to the vtable (the table of method implementations for the concrete type). Every method call goes through the vtable, which means an indirection and (usually) the loss of inlining. Understand the costs:

- **Indirection cost**: each call dereferences the vtable to find the function pointer, then calls it. On a hot loop this adds up; on a cold path it is negligible.
- **No monomorphization**: one copy of the code serves all types, which reduces binary size and compile time but prevents the compiler from specializing.
- **Type erasure**: the concrete type is gone. You cannot get it back without `Any` and downcasting, which is itself a vtable operation and a runtime check.
- **Allocation usually required**: `dyn Trait` has no statically known size, so it typically lives behind `Box<dyn Trait>`, `Rc<dyn Trait>`, or `Arc<dyn Trait>`. That is a heap allocation per value.

Use `dyn Trait` when type erasure is the point (heterogeneous collection, plugin system, hiding implementation) and the call frequency is low enough that vtable cost is irrelevant. Use generics when the concrete type is known at the call site and performance matters.

### Design For Object Safety From The Start

A trait is object-safe only if it can be used as `dyn Trait`. The rules exist because the vtable must be able to dispatch every method without knowing the concrete type. The core constraints:

- **No generic methods.** A generic method `fn foo<T>(&self, x: T)` cannot be in the vtable because there are infinitely many monomorphizations. If a method must be generic, the trait is not object-safe.
- **No `Self` by value or in return position in a way that varies by type.** Methods returning `Self` or taking `Self` by value are not object-safe because the concrete size is unknown. Return `Box<dyn Trait>` or take `&Self` instead.
- **No associated functions without a receiver.** A method must have `&self`, `&mut self`, `self`, `Box<Self>`, or similar as the first parameter; bare associated functions cannot be dispatched through a vtable.
- **All methods must be object-safe.** One non-object-safe method makes the whole trait non-object-safe.

If there is any chance the trait will be used as an object, design for object safety from the beginning. Retrofitting object safety is a breaking change: removing a generic method or changing a `Self`-returning method changes the trait's API. Split a trait into an object-safe core (the methods you want to dispatch on) and a companion trait for the generic or `Self`-typed methods.

### Add Bounds To The Trait Object Explicitly

`dyn Trait` is `dyn Trait + 'static` by default in many contexts, but you often need additional bounds. Trait objects can carry `Send` and `Sync` bounds: `Box<dyn Trait + Send + Sync>` is a trait object whose concrete type is sendable across threads. These bounds are part of the type and must be stated where the object is stored and returned.

- If a trait object will cross threads (in an `Arc`, sent to a thread pool), add `Send + Sync`.
- If it borrows data, add a lifetime: `Box<dyn Trait + 'a>`.
- Be consistent: a function returning `Box<dyn Trait>` cannot satisfy a caller wanting `Box<dyn Trait + Send>`. Decide the bounds at the design stage.

The trap is adding `Send + Sync` only where the compiler complains, leading to inconsistent bound sets across the codebase. Decide the object's concurrency contract up front and apply it uniformly.

### Prefer An Enum When The Set Of Types Is Closed

If the set of concrete types is small and known (two parser backends, three event kinds), an enum is often better than a trait object. An enum:

- is statically dispatched (no vtable, inlinable);
- allows pattern matching with exhaustiveness checks;
- stores the value inline (no allocation, no fat pointer);
- makes adding a method affect all variants at compile time (no silent omission).

The tradeoff: adding a new variant to an enum is a breaking change that touches every match, while adding a new implementor of a trait is non-breaking. Use an enum when the set is closed and performance matters; use a trait object when the set is open (plugins, user-provided implementations) or when you genuinely need type erasure.

### Use Any For Downcasting Sparingly

Type erasure is usually the point of `dyn Trait`, but sometimes you need the concrete type back. The `std::any::Any` trait supports downcasting via `downcast_ref`/`downcast_mut`. This is occasionally necessary (serialization frameworks, diagnostic tools) but is a code smell:

- It defeats the purpose of the abstraction: you are special-casing concrete types behind an interface.
- It adds runtime checks and branching.
- It couples the code to specific types the trait was meant to hide.

Prefer designing the trait so callers do not need the concrete type (add methods to the trait). Reserve `Any` for genuine meta-operations where the abstraction cannot express what you need.

### Offer A Generic Shell Around A dyn Core

A common high-performance pattern: a public generic function that callers use with their concrete type (static dispatch, inlinable), which internally delegates to a `dyn Trait` implementation (shared code, one copy). This gives callers the speed of monomorphization at the boundary while keeping the core logic non-duplicated. Reach for this when you have a hot public API but want to avoid code bloat.

## Common Traps

### dyn On A Hot Path

Using `dyn Trait` in a tight loop or per-element operation pays vtable cost on every iteration. If the loop runs millions of times, switch to generics or an enum. Profile before assuming; cold paths do not care.

### Non-Object-Safe Trait Discovered Late

A trait with a generic method or `Self` return cannot be used as `dyn Trait`. Discovering this when you need type erasure forces a breaking redesign. Design for object safety early if trait objects are plausible.

### Inconsistent Send/Sync Bounds

`Box<dyn Trait>` here, `Box<dyn Trait + Send>` there: the codebase fragments into incompatible object types. Decide the concurrency contract up front and apply it everywhere the object appears.

### Trait Object For A Closed Set Of Two Types

`Vec<Box<dyn Animal>>` where `Animal` is only ever `Dog` or `Cat` pays allocation and vtable cost for no openness benefit. An enum is faster, simpler, and exhaustively matchable.

### Erased Type Then Immediate Downcast

`Box<dyn Trait>` followed by `downcast_ref::<Concrete>()` everywhere means the abstraction is not actually abstract. Either add the needed methods to the trait or use the concrete type directly.

### Forgetting The Allocation

`Box<dyn Trait>` allocates. In a loop building many objects, that is many allocations. Consider an arena, a small-box optimization crate, or an enum if allocation count matters.

### Assuming dyn Reduces Binary Size Enough To Matter

Trait objects do reduce monomorphization, but the binary-size win is usually negligible unless generics are heavily instantiated. Do not choose `dyn` for binary size alone; choose it for type erasure.

## Self-Check

- [ ] `dyn Trait` is used where type erasure is the point (heterogeneous collections, plugins, hidden implementations) and the call frequency is low enough that vtable cost is acceptable; hot paths use generics or enums.
- [ ] Traits that may be used as objects are designed object-safe from the start (no generic methods, careful `Self` usage); object safety was not retrofitted as a breaking change.
- [ ] `Send`/`Sync` and lifetime bounds on trait objects are decided up front and applied consistently across storage and return types.
- [ ] Closed sets of types use enums (static dispatch, exhaustive matching, inline storage) rather than trait objects, unless openness is genuinely needed.
- [ ] Downcasting via `Any` is rare and justified; the trait exposes the methods callers need rather than forcing them to recover the concrete type.
- [ ] The allocation cost of `Box<dyn Trait>` is acknowledged; arenas or enums are used where allocation count matters.
- [ ] Where performance and shared code both matter, a generic public shell wraps a `dyn` core to get static dispatch at the boundary without code duplication.
- [ ] The decision between generics, `dyn`, and enum is documented for non-obvious cases so future maintainers understand the tradeoff.
- [ ] No trait object pays vtable cost on a measured hot path without a deliberate reason.
- [ ] Object safety constraints were verified, not assumed — the trait compiles as `dyn Trait` where it needs to.
