---
name: cpp_class_design_and_special_members.md
description: Use when the agent is designing or reviewing a C++ class — choosing value vs reference vs polymorphic semantics, the Rule of Zero/Five, virtual and protected base classes, inheritance hierarchies, abstract interfaces, CRTP, operator overloading, friend declarations, data member layout and access specifiers, single-responsibility class boundaries, or diagnosing slicing, accidental copying, god-classes, or hierarchies that should be type erasure or composition instead.
---

# C++ Class Design And Special Members

A C++ class is several things at once: a value type with copy/move semantics, a namespace for its members, a layout for its data, and (if it has virtual functions) a polymorphic dispatch table. The decisions you make when designing a class — value vs reference semantics, copyable vs move-only, polymorphic base vs concrete type, public data vs encapsulated — interact in ways that determine whether the class is easy to use correctly, hard to misuse, and maintainable as it evolves. The judgment problem is that these decisions are made at design time but their consequences surface as slicing bugs, accidental copies, god-classes, and rigid hierarchies that resist change.

Agents tend to default to "a class with a constructor, public data, and a virtual destructor" — a Java-influenced shape that is often wrong for C++, where value semantics and composition are usually the better fit. The judgment problem is to decide deliberately whether a type is a value (copyable, comparable), an entity (identity, non-copyable), or a polymorphic interface (virtual, used through pointer/reference), to apply the Rule of Zero or Five correctly, and to prefer composition and type erasure over deep inheritance. This skill is about making class-design choices that fit C++ idioms rather than fighting them.

## Core Rules

### Decide Value, Entity, Or Polymorphic, And Design Accordingly

Every class falls into one of three categories, and the right design differs sharply:

- **Value type**: represents a value (a Point, a String, a Config). Copyable, often comparable with `==`, passed by value or const reference, no identity. Default to value semantics — it is the most composable and least surprising in C++. The Rule of Zero applies if members handle their own copying.
- **Entity type**: has identity and lifetime (a Connection, a Session, a Lock). Often non-copyable (copying a connection does not make sense), maybe movable. Passed by reference or pointer. Delete copy, define move, or make it immobile.
- **Polymorphic type**: meant to be used through a pointer/reference to a base with virtual functions. The base has a virtual destructor, derived classes implement behavior. This is the case for `virtual`, and it should be the exception, not the default.

Decide which category a class is in before writing it, because the copy/move/virtual decisions flow from that. A class that is "a bit of all three" is usually a design smell.

### Default To The Rule Of Zero; Apply The Rule Of Five Deliberately

The Rule of Zero: if all your members are RAII types (smart pointers, containers, fstreams), the compiler-generated destructor, copy/move constructors, and copy/move assignment are all correct. Write no special members. This eliminates a large class of bugs (double-free, missing copy, etc.).

The Rule of Five: if you write any one of (destructor, copy ctor, copy assign, move ctor, move assign), you must consider all five, because the compiler-generated versions are usually wrong for a resource-owning type. This applies when the class manages a raw resource directly. Prefer to wrap the raw resource in an RAII type and return to the Rule of Zero.

The trap is writing a destructor to free a resource and leaving the compiler-generated copy constructor, which copies the pointer and double-frees. If you have a resource, apply Five or refactor to Zero.

### Prefer Composition Over Inheritance; Reserve virtual For Genuine Polymorphism

Inheritance is for substitutability (a Derived is-a Base, usable wherever a Base is expected through a base pointer/reference with virtual dispatch), not for code reuse. Using inheritance to reuse base-class code produces deep, rigid hierarchies where changes to the base break unknown derived classes, and where the "is-a" relationship is often a fiction.

For code reuse, prefer composition (the class has-a member that provides the reused behavior) or free functions. For runtime polymorphism with a closed set of types, consider `std::variant` + `std::visit` (sum types) instead of a class hierarchy — it is more efficient (no virtual dispatch, cache-friendly) and exhaustive. For runtime polymorphism with an open set of types or cross-library boundaries, virtual inheritance remains correct.

When you do use inheritance: the base of a polymorphic hierarchy has a `virtual` destructor; a non-polymorphic base should not (it adds overhead). Prefer non-public (protected or private) inheritance for implementation reuse, and public inheritance only for genuine is-a.

### Make Data Members Private And Invariants Enforced

Public data members expose the representation and prevent the class from enforcing invariants — a caller can set a member to any value, including one that breaks the class's contract. Make data members private and expose behavior through methods. This lets you change the representation without breaking callers and lets the class validate state.

The exception is "plain old data" aggregates (a Point with x, y) where there is no invariant and the type is a dumb bundle of values — there, public members and aggregate initialization are fine, and adding getters/setters adds nothing. Reserve encapsulation for types that have invariants.

### Avoid Slicing: Pass Polymorphic Types By Reference Or Pointer

If a class hierarchy is used polymorphically, passing a Derived by value to a function taking a Base *copies the Base part and drops the Derived part* — this is slicing, a silent correctness bug. Pass polymorphic types by reference or pointer (usually `Base&`/`const Base&` or `std::unique_ptr<Base>`), never by value.

Slicing also happens on assignment: `Base b = derived;` slices. And on catch: `catch (Base e)` slices; use `catch (const Base& e)` for exception types.

### Use CRTP For Static Polymorphism, Knowing Its Tradeoffs

The Curiously Recurring Template Pattern (`class Derived : public Base<Derived>`) gives compile-time polymorphism: the base can call methods on the derived type without virtual dispatch overhead. It is useful for performance-critical code and for mixin-style composition. Its costs: the base must be templated on the derived type (so each derived instantiates a separate base), there is no runtime polymorphism (you cannot store a heterogeneous collection of CRTP-derived types in one container), and error messages are template-heavy.

Use CRTP when you need polymorphic dispatch without runtime cost and the set of types is known at compile time. Do not use it as a default; prefer composition or `std::variant`.

### Overload Operators To Match Built-In Semantics, Not For Convenience

Operator overloading is powerful and easily abused. Overload operators only when the semantics are clear and match the built-in operators' expectations: `==` for equality, `<` for ordering, `()` for callable/functor, `[]` for indexing, assignment/arith for value types that behave like numbers. Do not overload operators to mean something clever (`<<` for "send a message" in a non-IO context) — it confuses readers.

When overloading, provide the full set: if you have `==`, consider `!=`; if `<`, consider the other comparisons (or use the spaceship operator `<=>` in C++20 to generate them all). Make sure copy assignment is self-assignment-safe and exception-safe (copy-and-swap).

## Common Traps

### Slicing A Polymorphic Type By Passing By Value

`void process(Base b)` called with a Derived copies only the Base part. Pass by reference/pointer, or hold in `unique_ptr<Base>`.

### Writing A Destructor But Forgetting Copy (Rule Of Five Violation)

A class with a raw pointer and a destructor that frees it, but a compiler-generated copy constructor, double-frees. Apply the Rule of Five or refactor to the Rule of Zero.

### Public Data Members That Cannot Enforce Invariants

`class Account { public: int balance; };` lets anyone set balance to any value, including negative. Make it private and provide methods that enforce the invariant.

### Inheritance For Code Reuse Instead Of Substitutability

A deep hierarchy where derived classes exist to reuse base code, producing fragile coupling. Prefer composition or free functions; reserve inheritance for is-a.

### Missing virtual Destructor On A Polymorphic Base

Deleting a Derived through a Base* where Base lacks a `virtual ~Base()` is undefined behavior (the Derived destructor never runs, leaking resources). Any base with virtual functions needs a virtual destructor.

### God-Class With Many Responsibilities

A class that does everything (a "Manager" that holds state, does IO, parses, and renders) violates single-responsibility and becomes unmaintainable. Split by responsibility.

### virtual Everywhere "For Flexibility"

Marking every function virtual by default adds dispatch overhead and signals a hierarchy that may not be needed. Use virtual only where runtime polymorphism is genuinely required.

### CRTP Misused For Runtime Polymorphism

CRTP is compile-time; you cannot store heterogeneous CRTP types in one container. If you need runtime polymorphism, use virtual inheritance or `std::variant`.

## Self-Check

- [ ] Each class has a clear category (value, entity, or polymorphic) and its copy/move/virtual design follows from that category.
- [ ] The Rule of Zero is the default; the Rule of Five is applied only where a class manages a raw resource directly, with all five special members consistent.
- [ ] Composition is preferred over inheritance for code reuse; inheritance is reserved for genuine is-a substitutability, and non-public inheritance is used for implementation reuse.
- [ ] Data members are private for types with invariants (with behavior exposed through methods); only invariant-free aggregates have public members.
- [ ] Polymorphic types are passed by reference or pointer (or `unique_ptr<Base>`), never by value, to avoid slicing; catch blocks use `const T&`.
- [ ] Polymorphic bases have a virtual destructor; non-polymorphic types do not; CRTP is used only for compile-time polymorphism with a known type set.
- [ ] Operators are overloaded only where semantics match built-in usage, the full related set is provided (or `<=>` in C++20), and copy assignment is self-assignment and exception safe.
- [ ] No god-class accumulates many responsibilities; classes are split by single responsibility.
