---
name: ruby_object_model_and_method_resolution.md
description: Use when the agent is designing or debugging Ruby's object model, class and module hierarchy, eigenclasses/singleton classes, method resolution order (MRO) and ancestor chain, include vs extend vs prepend, mixins and multiple inheritance emulation, method lookup with super, refinements, const_get and constant lookup, define_method and method definition, ancestors, is_a?/instance_of?/kind_of?, or is diagnosing "no superclass method for super", "method from the wrong module wins", "include order changed behavior", "the same method is called twice", or constant resolution surprises. Covers the method resolution algorithm, module inclusion order and prepend, eigenclass methods, super semantics, and the pitfalls of complex ancestor chains.
---

# Ruby Object Model And Method Resolution

Ruby's object model is uniform and powerful — everything is an object, classes are objects, modules mix in — and that power is exactly why method resolution surprises happen. When you call `obj.foo`, Ruby does not look in one place; it walks the ancestor chain: the object's singleton class (eigenclass), then its class, then each included module in reverse order, then superclasses, recursively, until it finds `foo` or reaches the top (BasicObject) and calls `method_missing`. The order of `include` and `prepend`, the placement of `super`, the presence of a singleton method, and a `refinement` all change which method wins. Agents who treat this as "the method on the class runs" ship code where the wrong method is called, `super` skips or loops, a module included later silently overrides an earlier one, or `method_missing` intercepts calls unexpectedly. The judgment problem is to know the resolution order, to use `include`/`prepend`/`extend` for their distinct purposes, to place `super` deliberately so it composes correctly, and to keep ancestor chains shallow enough to reason about.

The harm is subtle: a test passes because the resolution happens to work for the cases tried, and a refactor (reordering includes, adding a module, renaming) silently changes which method runs. The remedy is to read the ancestor chain (`Klass.ancestors`), to prefer composition over deep mixin hierarchies, to document where `super` is expected, and to verify resolution after any change to the chain.

## Core Rules

### Know The Method Resolution Order And Read The Ancestor Chain

Method lookup walks: singleton class (eigenclass) of the object → the class → modules prepended to the class (in reverse prepend order) → class methods → modules included in the class (in reverse include order) → superclass and its modules → ... → `BasicObject`. The first match wins; `super` resumes the search at the next entry. `Klass.ancestors` returns the exact chain Ruby will walk — read it when resolution is surprising. The chain is the source of truth; intuition about "which module wins" is unreliable without it.

- `ancestors` shows prepend modules before the class, include modules after, in the order they will be searched.
- A method defined directly on the class shadows one in an included module; a prepended module shadows the class.
- Singleton methods (defined on `obj` directly, or via `def obj.foo`) live on the eigenclass and win over class methods.

### Use include, prepend, And extend For Their Distinct Purposes

These three are not interchangeable; each changes resolution differently:

- **`include M`** inserts `M` *after* the class in the ancestor chain, so the class's own methods override `M`'s, and `super` from the class does *not* reach `M`. Use to add methods that the class can override.
- **`prepend M`** inserts `M` *before* the class, so `M`'s methods override the class's, and `super` from `M` reaches the class. Use to wrap or augment the class's methods (the "around" advice pattern, e.g., to add logging around an existing method without modifying it).
- **`extend M`** adds `M`'s methods as *class/singleton* methods (on the eigenclass), not instance methods. Use to add class-level methods (`extend ActiveModel::Naming`).

Mixing them up — using `include` where `prepend` was needed (so the wrapper never runs) or `extend` where `include` was meant (so instance methods are missing) — is a common bug. Choose by where the module's methods must sit in the chain.

### Place super Deliberately So It Composes

`super` (no args) forwards the same arguments up the chain; `super()` (explicit empty parens) calls the superclass method with no arguments. A missing `super` skips initialization or augmentation (a subclass that does not call `super` in `initialize` breaks the parent's setup); a `super` that loops (a module's method calling `super` to a class method of the same name that calls back) hangs. When wrapping with `prepend`, the wrapper calls `super` to reach the original; document that the `super` is the wrapped method, not a parent. In a chain of modules, each `super` advances one entry — verify the full chain calls each level once.

- Always call `super` in `initialize` unless you deliberately replace initialization.
- Distinguish `super` (forward args) from `super()` (no args); the wrong one raises `ArgumentError`.
- In `prepend` wrappers, ensure `super` reaches the intended method and is not skipped or duplicated.

### Prefer Composition Over Deep Mixin Hierarchies

Mixins are Ruby's multiple-inheritance emulation, and like multiple inheritance everywhere, deep hierarchies produce resolution that no one can predict: three modules define `to_s`, the include order decides which runs, and a new module changes everything. Prefer composition (hold a collaborator object, delegate to it) for behavior that does not need to be "is-a"; reserve mixins for genuinely cross-cutting capabilities (comparable, enumerable, serialization) that benefit from being in the chain. Keep ancestor chains shallow; if `ancestors` is longer than a screen, the design is too clever.

- `include Comparable` and define `<=>` is a good mixin (focused, well-understood).
- A module that adds five unrelated methods to a class is often better as a collaborator.
- Delegate explicitly (`Forwardable`) rather than mixing in when only a few methods are forwarded.

### Understand method_missing As A Last Resort With Cost

`method_missing` is invoked only when normal resolution fails, so it is the mechanism for dynamic finders (`find_by_name`), delegation, and DSLs. It is also slow (every miss walks the chain then calls it), it breaks `respond_to?` unless you also override `respond_to_missing?`, and it makes methods invisible to documentation and tooling. Use it for genuinely open-ended APIs (dynamic delegation, DSL verbs) and pair it with `respond_to_missing?`; avoid it for a known finite set of methods (define them explicitly, or use `define_method`).

- Always define `respond_to_missing?` alongside `method_missing` so `respond_to? :dynamic_method` is accurate.
- Prefer `define_method` for a finite set of dynamic methods; reserve `method_missing` for open-ended dispatch.

### Handle Constant Resolution And const_get Carefully

Constant lookup (`Foo`) walks the lexical scope chain, then the ancestor chain of the enclosing class, then `Object`. Reopening a class or nesting modules changes which `Foo` a reference resolves to. `const_get` (and `Module.const_get`) resolves by name and is the metaprogramming path to a class from a string — but it must be scoped (use the right receiver) and the input must be trusted (a user-controlled constant name is an injection vector to arbitrary classes). Prefer explicit references; use `const_get` only for dynamic dispatch and validate the name.

## Common Traps

### include vs prepend Choosing The Wrong Method

`include M` where `M#foo` should wrap `Klass#foo` fails silently (the class method wins, the wrapper never runs). Use `prepend` to wrap.

### super Skipped In initialize

A subclass `initialize` that does not call `super` leaves the parent's ivars unset, producing `nil`-related bugs later. Always `super`.

### Module Include Order Changing The Winner

Two modules define `foo`; the one included *last* wins (it is earlier in the chain). Reordering includes changes behavior with no other edit.

### extend Used Where include Was Meant

`extend M` adds singleton methods, not instance methods; `obj.new_method` is then missing. Use `include` for instance methods.

### method_missing Without respond_to_missing?

`respond_to? :dynamic_thing` returns `false` even though calling it works, breaking duck-typing checks and mocks. Always pair them.

### prepend Wrapper Looping Or Skipping super

A `prepend` module's method that calls `super` incorrectly either skips the wrapped method or loops. Verify the wrapper reaches the original exactly once.

### const_get On Untrusted Input

`Object.const_get(params[:class])` instantiates an arbitrary class — an injection. Validate or whitelist the constant name.

### Deep Ancestor Chain No One Can Predict

Three modules and two superclasses each defining overlapping methods produce resolution that changes with any include reorder. Flatten with composition.

## Self-Check

- [ ] The method resolution order is understood and verified: `Klass.ancestors` reflects the intended chain, and the first-match-wins behavior has been checked for the methods that matter.
- [ ] `include` (after class, overridable), `prepend` (before class, wrapping via `super`), and `extend` (singleton/class methods) are each used for their correct purpose, with no misuse.
- [ ] `super` is placed deliberately: `initialize` and intended augmentations call `super`, `super` vs `super()` is chosen correctly, and no `super` loops or skips.
- [ ] Mixin hierarchies are shallow and focused (comparable/enumerable-style capabilities); behavior that does not need to be in the chain uses composition or explicit delegation.
- [ ] `method_missing` is used only for open-ended dispatch, is paired with `respond_to_missing?`, and finite dynamic methods use `define_method` instead.
- [ ] Constant references resolve to the intended scope, `const_get` is scoped to the right receiver and never called on untrusted input without validation.
- [ ] The ancestor chain has been re-read after any include/prepend/extend change, and resolution still selects the intended method.
- [ ] The design has been considered under refactor (reorder, rename, new module) and remains predictable, with no resolution that depends on fragile ordering.
