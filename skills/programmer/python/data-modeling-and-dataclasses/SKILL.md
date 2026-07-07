---
name: python_data_modeling_and_dataclasses.md
description: Use when the agent is designing Python data structures, choosing between dataclass, NamedTuple, TypedDict, attrs, Pydantic models, plain classes, or raw dicts, deciding whether a value object should be mutable or frozen, modeling nested or recursive data, controlling equality/hashing/repr behavior, adding validation, or migrating from untyped dicts to typed models. Covers dataclass vs NamedTuple vs TypedDict vs Pydantic tradeoffs, mutability and hashing, slots, field defaults and factories, validation boundaries, and the tradeoff between lightweight data containers and full validation-bearing models.
---

# Data Modeling And Dataclasses

Python offers many ways to model a bundle of related fields: plain dicts, hand-written classes, `dataclass`, `NamedTuple`, `TypedDict`, `attrs`, and `Pydantic` models. Each makes different tradeoffs around mutability, validation, typing, performance, and ceremony. The judgment problem is choosing the right container for the data's lifecycle and the system's validation needs, rather than defaulting to a dict or to the heaviest available framework.

Agents tend to reach for plain dicts (no structure, no validation, typo-prone keys) or for Pydantic everywhere (heavy validation on data that is already trusted, slowing hot paths). They make value objects mutable when they should be frozen, forget `__hash__` when objects land in sets, or use mutable default arguments that are shared across instances. The harm appears as silent typos in dict keys, models that mutate unexpectedly, hashable-but-actually-mutable objects corrupting sets, and validation that runs redundantly on already-validated data. The real work is matching the container to the data's role, controlling mutability deliberately, and placing validation at the boundary where untrusted data enters.

## Core Rules

### Match The Container To The Data's Role And Lifecycle

Different containers serve different roles; choosing the first one you know leads to friction.

- **Plain dict / TypedDict**: for data that is genuinely shapeless or short-lived, or for interop with JSON and external APIs. `TypedDict` adds type-checkable keys without runtime overhead. Use when the data flows in and out of untyped boundaries.
- **`dataclass`**: the default choice for structured domain objects. Lightweight, readable, customizable (frozen, slots, eq, order), and part of the standard library. Use for most value objects and entities in application code.
- **`NamedTuple`**: immutable, tuple-compatible, and memory-efficient. Use when you want immutability, tuple unpacking, and minimal overhead, and do not need mutation or inheritance.
- **`attrs`**: a third-party superset of dataclass features, useful when you need validation, converters, or slots on Python versions predating modern dataclass features, or for advanced features dataclasses lack.
- **`Pydantic`**: models that validate at construction time, parse from untrusted input, and integrate with serialization. Use at system boundaries (HTTP request bodies, config loading) where input is untrusted and validation is the point. Avoid in hot internal paths where data is already validated.

The heuristic: start with `dataclass`. Move to `NamedTuple` if you need immutability and tuple semantics. Move to `TypedDict` for JSON-shaped interop. Move to `Pydantic` at trust boundaries. Move to plain dict only when the shape is genuinely dynamic.

### Decide Mutability Deliberately: Frozen vs Mutable

Whether a data object should be mutable is a design decision with real consequences.

- **Frozen (`@dataclass(frozen=True)` or `NamedTuple`)**: the object cannot be changed after creation. This makes it hashable (usable as a dict key or set member), safe to share across threads, and easy to reason about. Use frozen for value objects that represent a fixed fact (a coordinate, a configuration snapshot, a money amount).
- **Mutable (default dataclass)**: fields can be reassigned. Use for entities whose state legitimately changes over time (a shopping cart, a session, a build progress object). Mutable objects are not hashable by default, which is correct because their identity can change.

A common mistake is leaving value objects mutable when they should be frozen, then discovering they cannot be used as dict keys, or that a "constant" got mutated by accident. Default value objects to frozen; make entities mutable only when state change is part of the model.

### Handle Defaults Correctly: Factories, Not Shared Mutables

The classic Python trap is a mutable default argument shared across all instances. `field: list = []` in a dataclass shares one list among every instance. The fix is `field: list = field(default_factory=list)`, which creates a fresh list per instance.

- Use `field(default_factory=...)` for any mutable default (list, dict, set, or any custom mutable type).
- Immutable defaults (int, str, tuple, frozenset, frozen dataclass) are safe to share directly.
- This applies to plain functions too: `def f(items=[])` shares the list across calls. Use `None` and create inside the function.

### Control Equality, Hashing, And Representation Deliberately

Dataclasses generate `__eq__` by default (comparing fields), which is usually right. But `__hash__` is generated only if `frozen=True` (or `unsafe_hash=True`), because hashing a mutable object is dangerous. Decide explicitly:

- If objects should be hashable (used as dict keys, in sets, cached), make them frozen so `__hash__` is generated consistently with `__eq__`.
- If objects are mutable entities identified by identity, leave them unhashable and rely on `id()`.
- `__repr__` is generated by default and is invaluable for debugging; keep it unless you have a reason to suppress it.
- For `order` (`__lt__`, etc.), enable `order=True` only if ordering the objects is meaningful; generating comparisons you never use adds noise.

### Place Validation At The Trust Boundary, Not Everywhere

Validation has a cost, and validating already-valid data repeatedly is waste. Place validation where untrusted data enters the system.

- At an HTTP or config boundary, use Pydantic (or a dataclass plus a validation function) to parse and validate once. After that, the typed object is trusted.
- Internally, pass the validated typed object; do not re-validate on every function call.
- For dataclasses, you can add a `__post_init__` that validates fields, but be aware it runs on every construction. Reserve it for invariants that must always hold, not for expensive checks.

The mistake is treating every function as untrusted and re-validating its arguments, which slows the program and clutters the code. Validate at the edge; trust internally.

### Use Slots For Memory Efficiency When You Have Many Instances

`@dataclass(slots=True)` (Python 3.10+) generates `__slots__`, which prevents adding arbitrary attributes and reduces per-instance memory. This matters when you create millions of instances (parsed records, graph nodes). The tradeoff is that you lose the ability to add attributes dynamically and some inheritance flexibility. Use slots for high-cardinality value types; skip it for singleton-ish config objects.

## Common Traps

### Mutable Default Argument Shared Across Instances

`items: list = []` in a dataclass (or `def f(x=[])`) shares one object. Use `field(default_factory=list)` or `None` plus in-function creation.

### Mutable Value Object Used As A Dict Key

A mutable dataclass is unhashable by default; forcing `unsafe_hash` makes it hashable but inconsistent if it mutates, corrupting dicts and sets. Freeze value objects that need hashing.

### Pydantic Everywhere, Even For Trusted Internal Data

Running full validation on every internal construction slows hot paths and adds a heavy dependency. Validate at boundaries; use lighter dataclasses internally.

### Plain Dict For Structured Domain Data

Using a dict for a value object with known fields invites typos (`data["contry"]`), loses type checking, and scatters validation. Use a dataclass or TypedDict.

### Forgetting `__hash__` When Putting Objects In Sets

A mutable dataclass placed in a set raises `TypeError`. Either freeze it or reconsider whether set membership is appropriate for a mutable entity.

### Redundant Validation On Every Call

Re-validating arguments inside every function clutters code and wastes cycles. Validate at the trust boundary and trust the typed object downstream.

### `eq=True` With `frozen=False` And Manual `__hash__`

Mixing generated `__eq__` with a hand-written `__hash__` on a mutable object produces inconsistent hashing. Let `frozen=True` generate both consistently, or omit hashing.

## Self-Check

- [ ] The container type matches the data's role: dataclass for domain objects, NamedTuple for immutable tuple-like values, TypedDict for JSON interop, Pydantic at trust boundaries, plain dict only for dynamic shapes.
- [ ] Value objects are frozen when they represent fixed facts; entities are mutable only when state change is part of the model.
- [ ] Mutable defaults use `field(default_factory=...)` (or `None` plus in-function creation), never a shared mutable literal.
- [ ] Equality and hashing are consistent: frozen objects are hashable; mutable objects are not forced into sets or dict keys.
- [ ] Validation runs at the trust boundary (HTTP/config entry) and is not redundantly repeated on already-validated internal data.
- [ ] `__repr__` is retained for debuggability; `order` is enabled only where comparison is meaningful.
- [ ] `slots=True` is used for high-cardinality value types where memory matters, with the tradeoff understood.
- [ ] Structured domain data uses typed containers, not plain dicts that invite typos and lose checking.
