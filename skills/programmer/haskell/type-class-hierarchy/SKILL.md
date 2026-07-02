---
name: type_class_hierarchy.md
description: Use when the agent is defining Haskell type classes, writing Functor/Applicative/Monad instances, using multi-parameter type classes or functional dependencies, organizing instance modules, debugging orphan instance errors or overlapping instances, or deciding whether a new abstraction should be a type class, a plain function, or a newtype wrapper. Covers the type class hierarchy laws, instance rule compliance, orphan instances, and the cost of over-abstraction with type classes.
---

# Type Class Hierarchy

Haskell's type class hierarchy — `Functor` then `Applicative` then `Monad`, with `Foldable`, `Traversable`, `Semigroup`, `Monoid`, and others alongside — is one of the most carefully designed abstractions in programming, and it is also one of the most abused. Done well, it lets you write `traverse` once and use it on lists, trees, `Maybe`, parsers, and effects; the laws ensure that "this is a `Monad`" actually means something predictable. Done poorly, it produces abstractions that compile and mislead: a `Monad` instance that violates the laws and breaks `do` notation's equational reasoning, an orphan instance that causes "instance not found" or "overlapping instances" in another module, a multi-parameter class with no functional dependencies that makes inference hopeless, or a tower of type classes invented to avoid passing one function argument. The danger is that type classes look like interfaces but carry laws and resolution rules that interfaces do not, so the cost of a wrong abstraction is paid in confusing errors and broken invariants rather than a clear failure.

Agents tend to reach for a new type class whenever they see repeated patterns, the way an OOP programmer reaches for an interface, and to assume that lawful instances are a formality. The judgment problem is deciding when a concept genuinely belongs in the hierarchy (it has laws, multiple distinct instances, and a real abstraction payoff) versus when it is a function in disguise, how to write instances that actually satisfy the laws, and how to place instances so resolution is deterministic and the abstraction does not rot. Getting this wrong produces code that is hard to read, hard to extend, and subtly wrong in ways that only surface when a different instance is in scope.

## Core Rules

### Respect The Hierarchy Because Each Level Adds Laws And Capability

The hierarchy is not arbitrary layering; each level adds laws that the previous level did not have, and those laws are what make the abstractions usable.

- **`Functor`** — mapping over a structure. Law: identity (`fmap id == id`) and composition (`fmap (f . g) == fmap f . fmap g`). Lets you transform "inside" without touching the structure.
- **`Applicative`** — independent effects sequenced with a pure function. Laws: identity, composition, homomorphism, interchange. Lets you combine `f (a -> b)` with `f a` to get `f b`, which `Functor` cannot. Essential for combining multiple independent effects.
- **`Monad`** — effects that depend on previous results (via `>>=`). Laws: left identity, right identity, associativity. Strictly more powerful than `Applicative`; lets a later effect choose what to do based on an earlier effect's result.

Every `Monad` is an `Applicative`, and every `Applicative` is a `Functor`. When you write a `Monad` instance you should also have lawful `Applicative` and `Functor` instances, and the three must be consistent (the `Applicative` derived from the `Monad` via `pure`/`>>=` should match the standalone `Applicative`). The compiler does not check the laws; you do. A `Monad` whose `Applicative` is inconsistent will behave differently depending on which abstraction a library function uses, producing bugs that depend on which functions are called.

### Write Instances That Satisfy The Laws, And Test That They Do

The laws are not documentation; they are the contract that lets generic code (`sequence`, `traverse`, `mapM`, `=<<`) work correctly. An instance that violates the laws breaks every generic function written against the class, in ways that are extremely hard to trace because the failure is in the interaction between your instance and library code you did not write.

For each instance, write the laws as property tests (`quickcheck` properties) and run them. For `Monad`, test left identity (`return x >>= f == f x`), right identity (`m >>= return == m`), and associativity (`(m >>= f) >>= g == m >>= (\x -> f x >>= g)`). An instance that passes its laws is a real instance; one that has not been tested is a suspected instance. Laws are especially easy to violate accidentally for types with strictness, ordering, or state — a `Monad` for a stateful computation that reorders effects violates associativity.

### Avoid Orphan Instances By Placing Instances With The Class Or The Type

An orphan instance is an instance of `Class Type` defined in a module that owns neither `Class` nor `Type`. They compile, and they cause the most confusing errors in Haskell: "instance not found" when the orphan module is not imported, "overlapping instances" when two orphans exist, and silent behavior changes when a different orphan comes into scope. GHC explicitly warns about orphans because they are a known hazard.

Place instances in the module that defines the type (preferred, because the type author knows the laws) or the module that defines the class (when the type is from a library you cannot modify). If you own neither, you generally should not define the instance — wrap the type in a `newtype` and define the instance on the newtype, which is yours. The `newtype` wrapper is the canonical solution to "I want to add a class to a type I do not own": it gives you a type you control, on which you can place instances without orphaning.

### Use Multi-Parameter Classes And Functional Dependencies Only With Justification

Multi-parameter type classes (`class Coll s a | s -> a`) generalize single-parameter classes and are powerful, but they make inference harder and are easy to overuse. Without functional dependencies, inference often fails (the compiler cannot determine the relationship between parameters); with them, you are encoding a functional relationship that must hold for all instances. Use them when the abstraction genuinely relates two types (a collection and its element, a state and its token) and the relationship is functional (one collection type maps to exactly one element type).

Avoid them when a single-parameter class plus a type family, or a plain function, would express the same thing more clearly. Multi-parameter classes with no functional dependencies and many instances produce a maze of ambiguous types; prefer the more constrained option.

### Prefer A Function Or Newtype Over A New Type Class

The most common type class mistake is inventing a class to avoid passing an argument. If the "abstraction" has one instance, or if the methods could be ordinary functions taking the type as an argument, it should not be a class. Type classes earn their cost when there are multiple distinct lawful instances and generic code written against the class is genuinely reusable.

Before adding a class, ask: how many instances will this have? If the answer is "one production, one test," it is a record of functions (the dictionary of methods), not a class. If the answer is "many, and I want to write generic code over them," it may be a class. The same logic applies to extending the hierarchy: do not invent `MonadState`, `MonadReader`, `MonadError`-style classes unless you are building a generic effect library; for one application, pass the state explicitly or use a concrete monad.

### Make Instances Deterministic To Avoid Overlapping And Incoherence

`OverlappingInstances` and `IncoherentInstances` let multiple instances for the same type coexist with a resolution rule, and they are the source of some of the hardest-to-diagnose Haskell bugs: which instance is selected depends on call-site context, the same code behaves differently depending on imports, and adding an instance elsewhere shifts behavior. Avoid them. If you find yourself reaching for overlapping instances, the abstraction is usually wrong — restructure as a newtype per behavior, or as a single instance with runtime dispatch.

The disciplined approach is one canonical instance per type per class, placed in the right module, with no overlap. When you genuinely need behavior to vary, use a sum type or a record of functions at the value level rather than overlapping instances at the type level.

### Use Default Methods And Deriving To Avoid Law-Violating Boilerplate

`deriving (Eq, Ord, Show)`, `deriving Functor`/`Foldable`/`Traversable` (with `DeriveFunctor` etc.), and `GeneralizedNewtypeDeriving` produce lawful instances mechanically and should be preferred over hand-writing them. Hand-written instances are where laws get violated — a hand-written `Eq` that forgets a field, a hand-written `Functor` that mis-orders effects. Default methods in class definitions (where the class provides a default `fmap` in terms of `>>=`) reduce the surface area for mistakes.

Reach for deriving first; hand-write only when the deriving does not apply or produces the wrong semantics, and test the hand-written instance's laws explicitly.

## Common Traps

### Defining A `Monad` Whose `Applicative` Is Inconsistent

Writing a `Monad` instance and a separate `Applicative` instance that do not agree, so `do` notation (which desugars through `Monad`) and `<*>` (which uses `Applicative`) produce different results for the same computation. The `Applicative` derived from `Monad` (`pure = return`, `<*> = ap`) must match the standalone `Applicative`; keep them consistent.

### Inventing A Type Class For A One-Instance Abstraction

Defining `class Repository a` with one production instance and one mock, when a record of functions or a plain module would express the same thing without type-class machinery. Type classes need multiple lawful instances to justify their cost; one-instance "classes" are functions in disguise.

### Defining An Orphan Instance That Causes Overlap Later

Putting `instance ToJson ExternalType` in a service module (owning neither `ToJson` nor `ExternalType`), then colliding with another library's orphan for the same pair. Place instances with the class or the type, or wrap in a `newtype` you own.

### Violating A Law Because The Instance Reorders Or Strictifies Effects

A `Monad` instance for a stateful or ordered computation that subtly reorders effects, violating associativity and breaking every generic function written against `Monad`. Write the laws as property tests; do not assume an instance is lawful because it type-checks.

### Using OverlappingInstances To "Make Inference Work"

Enabling `OverlappingInstances` or `IncoherentInstances` to resolve a type error, then watching behavior shift depending on imports and call-site context. Restructure with newtypes or value-level dispatch instead; overlapping instances are a known source of indeterminate behavior.

### Adding A Multi-Parameter Class Without Functional Dependencies

Defining `class Coll s a` with no fundeps, then hitting "ambiguous type" errors at every use site because the compiler cannot relate `s` and `a`. Add functional dependencies (`s -> a`) when the relationship is functional, or use a type family / plain function when it is not.

### Hand-Writing Instances That Deriving Would Produce Lawfully

Hand-writing `Eq`, `Ord`, `Functor`, or `Foldable` instances that deriving would generate correctly, and introducing a law violation (a missed field, a wrong order) in the process. Prefer deriving; hand-write only when necessary and test the laws.

### Assuming The Laws Are Checked

Treating "it compiles" as evidence the instance is lawful, when GHC checks types but never laws. Laws must be tested as properties; an untested instance is a suspected instance.

## Self-Check

- [ ] Every `Monad` has consistent `Functor` and `Applicative` instances, and the `Applicative` derived from the `Monad` matches the standalone one; consistency was checked, not assumed.
- [ ] Each non-trivial instance has its laws written as property tests (`QuickCheck` properties) that pass; instances for stateful, ordered, or strict types were tested specifically for associativity and identity.
- [ ] No orphan instances exist; instances live with the type or the class, or the type is wrapped in a `newtype` owned by the codebase.
- [ ] No `OverlappingInstances` or `IncoherentInstances` are used to make inference work; behavior variation is expressed with newtypes or value-level dispatch.
- [ ] Multi-parameter classes have functional dependencies where the relationship is functional, and were not used where a single-parameter class, type family, or plain function would suffice.
- [ ] Each type class has multiple distinct lawful instances and supports genuinely reusable generic code; one-instance abstractions are records of functions or plain modules, not classes.
- [ ] `deriving` is used for `Eq`/`Ord`/`Show`/`Functor`/`Foldable`/`Traversable` wherever it applies; hand-written instances exist only where deriving is wrong or unavailable, and their laws are tested.
- [ ] A reader can determine, for any given type and class, the single canonical instance and the module it lives in, without resolving overlaps or chasing imports.
