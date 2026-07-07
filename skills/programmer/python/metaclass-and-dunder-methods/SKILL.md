---
name: python_metaclass_and_dunder_methods.md
description: Use when the agent is writing or reviewing Python dunder (magic) methods (__init__, __repr__, __eq__, __hash__, __getattr__, __getitem__, __iter__, __enter__, __call__, __new__), designing data classes and rich comparison, implementing operator overloading, building metaclasses or __init_subclass__ hooks, using __slots__, controlling instance creation, or debugging subtle object behavior from missing or inconsistent dunders. Covers dunder lookup on the type vs the instance, data model contracts (hash/eq consistency, repr/str, comparisons), metaclass vs __init_subclass__ vs class decorators, __slots__ tradeoffs, and the pitfalls of overusing metaclasses or breaking dunder invariants.
---

# Metaclass And Dunder Methods

Python's data model is built on dunder (magic) methods: `__init__`, `__repr__`, `__eq__`, `__getitem__`, `__iter__`, and dozens more. These methods are how your objects participate in Python's syntax and built-ins — addition, iteration, comparison, context management, attribute access. Metaclasses are the mechanism that customizes class creation itself. The judgment problem is implementing dunders correctly (they have contracts, not just signatures), knowing when metaclass-level customization is justified versus overkill, and not breaking invariants the data model depends on.

Agents tend to add dunders reactively without knowing their contracts (defining `__eq__` without `__hash__`, `__lt__` without the other comparisons), reach for a metaclass when `__init_subclass__` or a class decorator would do, or implement magic methods that subtly violate the expectations of built-in functions. The harm appears as unhashable objects, broken set/dict behavior, inconsistent comparisons, infinite recursion in `__getattr__`, and metaclass hierarchies that nobody can follow. The real work is respecting each dunder's contract, choosing the lightest class-customization mechanism that works, and keeping the data model consistent.

## Core Rules

### Respect Each Dunder's Contract, Not Just Its Signature

Dunders are not arbitrary hooks; each has a contract that built-in functions and syntax rely on. Violating the contract produces objects that look right but misbehave in collections, comparisons, and introspection.

- **`__eq__` and `__hash__` are linked.** If you define `__eq__`, Python sets `__hash__` to `None` (the object becomes unhashable) unless you also define `__hash__`. Equal objects must have equal hashes; if two objects compare equal but hash differently, they break dict/set lookup. Define both, or neither.
- **Rich comparisons should be consistent.** If you define `__eq__` and `__lt__`, consider `functools.total_ordering` to fill in the rest, or define the full set. Inconsistent comparisons (a < b and b < a both true) break sorting and break invariants that `sorted`/`bisect` rely on.
- **`__repr__` should be unambiguous and ideally eval-able.** `__str__` is for users; `__repr__` is for developers and debugging. A `__repr__` that looks like `ClassName(field=value, ...)` lets you reconstruct the object mentally and in logs. Do not make `__repr__` return a user-friendly string at the cost of debuggability.
- **`__len__` and `__bool__` interact.** If you define `__len__`, `bool(obj)` is `False` when length is 0 unless you also define `__bool__`. An empty container being falsy is expected; an object you did not intend to be falsy becoming falsy because of `__len__` is a surprise.

Read the data model documentation for each dunder before implementing it. The signature is the easy part; the contract is what makes the object behave correctly with the rest of Python.

### Understand Implicit Dunder Lookup Happens On The Type, Not The Instance

This is a subtle but critical rule: for implicit invocations (syntax and built-in functions), Python looks up dunder methods on the *type*, not the instance. `len(obj)` calls `type(obj).__len__`, not `obj.__len__` directly. This means:

- Defining a dunder on an *instance* (e.g., `obj.__len__ = lambda: 5`) does NOT affect `len(obj)` — the type's method is used. Instance attributes are bypassed for implicit dunder calls.
- A subclass that sets a dunder on the instance, or a proxy that forwards attribute access, will not intercept implicit dunder calls unless the dunder is defined on the class/type.
- Proxy objects that must forward all operations need to define each dunder on the proxy class explicitly (or use `__getattr__` carefully, which still does not catch implicit dunders).

If you are building a wrapper or proxy and `len()`/`iter()`/`+` do not work on it, this is why. The dunder must live on the class.

### Use __init_subclass__ Or Class Decorators Before Metaclasses

Customizing class creation is occasionally necessary (registration, validation, plugin systems). Python offers three mechanisms of increasing power and complexity; use the lightest that works.

- **`__init_subclass__`**: a hook called when a class is subclassed. It lets a base class customize its subclasses without a metaclass. This covers most "register subclasses" and "validate subclass shape" needs and is far simpler than a metaclass. Prefer it.
- **Class decorators**: a function that takes a class and returns a (possibly modified) class. Good for registration, adding methods, post-creation augmentation. Applied explicitly per class.
- **Metaclasses**: customize class creation itself (`__new__`/`__init__` of the metaclass run when a class is defined). The most powerful and the most overused. Reach for a metaclass only when you need to control class creation in ways `__init_subclass__` and decorators cannot (e.g., intercepting class definition generically, or matching an existing framework's metaclass).

The trap is reaching for a metaclass because it sounds advanced. A metaclass adds a layer of magic that makes the code harder to follow, harder to debug, and harder to combine (multiple metaclasses on one class conflict). Exhaust the simpler options first.

### Decide On __slots__ Deliberately

`__slots__` restricts a class to a fixed set of attributes, preventing the creation of a per-instance `__dict__`. This saves memory (meaningful for millions of instances) and catches typos in attribute names at the cost of flexibility.

- Use `__slots__` when you have many instances of a class and memory matters; the savings can be large.
- The cost: instances cannot have attributes beyond the declared slots, which breaks patterns that add attributes dynamically, weakref (unless `__weakref__` is in slots), and some serialization/pickling. Subclassing a slotted class without its own `__slots__` reintroduces a `__dict__`.
- Do not add `__slots__` reflexively to every class; for a handful of instances the memory saving is irrelevant and the inflexibility is a cost.

### Keep __getattr__, __getattribute__, And __setattr__ From Recursing

Customizing attribute access is powerful and a common source of infinite recursion. The rules:

- **`__getattr__`** is called only when normal attribute lookup fails. It is safe for "compute a missing attribute" patterns. To access the real storage, use normal attribute access (it will not recurse because `__getattr__` is only called on failure).
- **`__getattribute__`** is called for *every* attribute access. Accessing `self.x` inside it calls `__getattribute__` again → infinite recursion. Access the underlying storage via `object.__getattribute__(self, "x")` or `super().__getattribute__`.
- **`__setattr__`** is called for every attribute set. `self.x = v` inside it recurses; use `object.__setattr__(self, "x", v)` or store in `__dict__` directly.

If you see a `RecursionError` near attribute access, this is almost always the cause. Use the `object`/`super` form to break the recursion.

### Make Iteration And Container Protocols Consistent

If you implement `__getitem__`, Python's iteration protocol will fall back to it (old-style iteration: call `__getitem__(0)`, `__getitem__(1)`, ... until `IndexError`). If you implement `__iter__`, that takes precedence. Decide which protocol your container supports and implement it consistently:

- Implement `__iter__` (returning an iterator) for modern containers; it is cleaner than `__getitem__`-based iteration.
- If you implement `__contains__` (`in`), make it consistent with iteration; otherwise `in` falls back to scanning via `__iter__`/`__getitem__`.
- `__len__`, `__getitem__`, and `__iter__` should agree: an object whose `__len__` says 3 but whose iteration yields 5 items is broken.

## Common Traps

### __eq__ Without __hash__

Defining `__eq__` makes the object unhashable (sets `__hash__ = None`), silently breaking dict/set use. Define both, consistently.

### Inconsistent Rich Comparisons

Defining `__lt__` but not `__le__`/`__gt__` leaves those operations to fall back in surprising ways; `a <= b` may become `not (a > b)` with odd results. Use `total_ordering` or define the full set.

### Dunder Defined On The Instance

`obj.__len__ = ...` does not affect `len(obj)`; implicit lookup uses the type. Define dunders on the class.

### Metaclass Where __init_subclass__ Would Do

A metaclass added for subclass registration or validation when `__init_subclass__` covers the need. The metaclass adds combinatorial complexity (metaclass conflicts in multiple inheritance) for no benefit.

### Recursion In __getattribute__ Or __setattr__

Accessing `self.x` inside `__getattribute__`/`__setattr__` recurses infinitely. Use `object.__getattribute__`/`object.__setattr__` to reach storage.

### __repr__ That Lies Or Is Unreadable

A `__repr__` returning a user-facing string, or omitting key fields, makes debugging harder. `__repr__` should be unambiguous and reconstructable.

### __slots__ Breaking Pickling Or Dynamic Attributes

Adding `__slots__` then relying on dynamic attributes, weakref, or pickling that needs `__dict__`. Weigh the memory saving against the lost flexibility.

### __bool__ Implicitly Derived From __len__

An object that becomes falsy when empty because of `__len__`, when emptiness was not meant to imply falsiness. Define `__bool__` explicitly if the default is wrong.

## Self-Check

- [ ] `__eq__` and `__hash__` are defined together and consistent (equal objects have equal hashes); objects are hashable if they will be used in sets/dicts.
- [ ] Rich comparisons are consistent (full set or `total_ordering`); sorting and `bisect` behave correctly.
- [ ] `__repr__` is unambiguous and developer-facing (reconstructable shape); `__str__` is separate and user-facing where needed.
- [ ] Dunder methods are defined on the class/type, not the instance; implicit lookups (`len`, `iter`, `+`) work because the type carries the methods.
- [ ] Class customization uses the lightest mechanism: `__init_subclass__` or class decorators before metaclasses; a metaclass is justified only when the simpler options cannot express the need.
- [ ] `__slots__` is used deliberately for memory-sensitive many-instance classes, with the lost flexibility (dynamic attrs, weakref, pickling) acknowledged.
- [ ] `__getattr__`/`__getattribute__`/`__setattr__` avoid infinite recursion by using `object.__getattribute__`/`object.__setattr__` or `super()` to reach storage.
- [ ] Container protocols (`__iter__`, `__getitem__`, `__len__`, `__contains__`) are consistent and agree with each other.
- [ ] `__bool__`/`__len__` interaction is intentional; objects do not become unexpectedly falsy.
- [ ] No metaclass is added where `__init_subclass__` would suffice; metaclass conflicts in multiple inheritance were considered.
