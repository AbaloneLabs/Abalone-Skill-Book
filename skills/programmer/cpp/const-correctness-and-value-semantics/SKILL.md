---
name: cpp_const_correctness_and_value_semantics.md
description: Use when the agent is writing or reviewing C++ code involving const correctness, const member functions, const parameters and return types, mutable members, constexpr and consteval, passing by value vs reference, return value optimization, deciding when a type should have value semantics vs reference semantics, avoiding accidental copies, or diagnosing const-cast abuse, accidental mutations, needless copies from returning by value, or APIs that force callers to copy.
---

# C++ Const Correctness And Value Semantics

`const` is one of C++'s most powerful tools for correctness: it lets the compiler enforce that a value is not modified, that a function does not mutate its receiver, that a parameter is read-only. Const correctness — applying `const` consistently so the compiler catches accidental mutations — is a discipline that pays dividends in safety and enables optimization. Paired with it is the question of value semantics: whether a type behaves like a value (copied independently, no shared identity) or a reference (shared, identity-based). C++ defaults to value semantics, and used well this makes code easier to reason about than reference-heavy styles; used badly it produces needless copies and surprising aliasing. The judgment problem is to apply `const` thoroughly, to choose value vs reference deliberately per type and per parameter, and to understand when the compiler can elide copies (RVO) and when it cannot.

Agents tend to under-use `const` (leaving parameters and locals mutable "just in case"), to pass large objects by value (copying), or to over-use references and pointers (introducing aliasing and lifetime questions). The judgment problem is to make every parameter and local `const` unless it must change, to pass by value for cheap-to-copy types and for sink parameters, by const reference for expensive read-only types, and to design types with value semantics where possible. This skill is about using `const` and value semantics as deliberate design tools, not afterthoughts.

## Core Rules

### Be Const-Correct Throughout: Propagate const From The Bottom Up

Const correctness is viral: if a member function is `const`, the members it calls on `this` must also be `const`-callable. The practical approach is to make functions and parameters `const` from the leaf functions up. A `const` member function promises not to modify the object's observable state; mark every member function that does not need to mutate as `const`, so it can be called on `const` instances and through `const` references.

The trap is retrofitting const: a codebase with no const cannot easily add it later because a single non-const call deep in the chain forces everything above it to be non-const. Start const-correct from the beginning, or migrate leaf-up. The compiler will tell you where const cannot be added (because the function genuinely mutates), which is itself useful information.

### Choose Parameter Passing By Cost And Intent

The choice of value vs reference for a parameter is a cost and intent decision:

- **By value** for cheap-to-copy types (primitives, small structs, smart pointers, iterators) and for "sink" parameters (the function will store or mutate its own copy). Taking a sink by value and moving into place (`void add(std::string s) { vec_.push_back(std::move(s)); }`) is the modern idiom and avoids a copy when the caller passes an rvalue.
- **By const reference** for expensive-to-copy types (containers, large structs) that the function only reads.
- **By reference (non-const)** only when the function modifies the argument in place and that is the API contract (rare; prefer a return value).
- **By rvalue reference** only for move constructors/assignment and for functions that explicitly consume (move from) the argument; do not take rvalue references for general parameters.

The rule of thumb: if in doubt, take cheap types by value and expensive types by const reference. For sink parameters (the function stores the data), take by value and move.

### Mark Member Functions const When They Do Not Mutate Observable State

A `const` member function promises not to modify the object's observable state. Mark every getter, every read-only computation, every function that does not change the object as `const`. This lets the function be called on `const` instances and through `const` references, which is essential for const-correct APIs.

Use `mutable` for members that may change without changing observable state — typically caches, memoization, and lazy initialization. `mutable` lets a `const` function modify a specific member (e.g., a cache), while still presenting a const (non-mutating) interface to callers. Do not abuse `mutable` to mutate real state from a const function; that defeats the contract.

### Prefer Value Semantics; Use Reference Semantics Deliberately

Value semantics (a type is copied independently, has no shared identity) are easier to reason about: no aliasing, no lifetime questions, no synchronization needed for distinct copies. C++ containers and standard types have value semantics by default, and you should prefer value semantics for your own types where feasible.

Reference semantics (shared state, identity) are necessary when: the object is expensive to copy and shared (use `shared_ptr`), when the object represents a unique resource (a file, a connection — use `unique_ptr` or move-only), or when polymorphism is needed (use a pointer/reference to a base). But every reference is a potential aliasing and lifetime bug. Default to value; reach for reference only when value is too expensive or semantically wrong.

### Understand RVO/NRVO And Do Not Pessimize Returns

The compiler can elide copies on return (Return Value Optimization, RVO; Named RVO, NRVO), so returning a large object by value is often free — the object is constructed directly in the caller's space. Since C++17, copy elision is mandatory in certain cases (prvalue returns). Do not "optimize" by returning a reference or out-parameter instead of by value; the by-value return is often as fast or faster and is clearer.

- Return by value for functions that produce a new value. Trust RVO/NRVO; do not take an out-parameter to "avoid a copy."
- Return by `const T&` only for accessors that expose an existing member (and document that the reference is valid only while the object is alive).
- Never return a reference or pointer to a local; it dangles. Return by value.

In modern C++ with move semantics, even when RVO does not apply, returning by value moves rather than copies for move-enabled types, which is cheap.

### Use constexpr For Compile-Time Computation Where It Earns Its Place

`constexpr` marks a function or variable as evaluable at compile time (given constant arguments). It lets you compute constants, table sizes, and lookup tables at compile time with no runtime cost, and it makes functions usable in template arguments and `static_assert`. `consteval` (C++20) forces compile-time evaluation.

Use `constexpr` for functions that compute compile-time-known values (table sizes, hash constants, format strings). Do not sprinkle `constexpr` on everything — it constrains the function (no non-constexpr calls inside) and is only valuable when the function is actually used in a constant context. Apply it where compile-time evaluation is the point.

### Do Not const_cast Away const To Mutate

`const_cast` is occasionally legitimate (interfacing with a C API that is not const-correct, or removing const on something you know was originally non-const), but using it to mutate a truly `const` object is undefined behavior. If you find yourself needing `const_cast` to mutate, the design is wrong: either the function should not be const, or the data should not be const. Treat `const_cast` as a red flag to review.

## Common Traps

### Not Bothering With const On Parameters And Locals

Leaving everything non-const "for flexibility" means accidental mutations are not caught and const-correct callers cannot pass const data. Make everything const that does not need to change.

### Passing Expensive Types By Value

`void process(std::vector<int> v)` copies the vector on every call. Pass expensive types by const reference, unless the function is a sink that needs its own copy.

### Returning A Reference To A Local

`const T& f() { T local; ...; return local; }` dangles. Return by value; trust RVO.

### Taking An Out-Parameter To "Avoid A Copy"

`void compute(Result* out)` to avoid copying the return is a pessimization in modern C++ — by-value return with RVO/move is as fast and clearer. Reserve out-parameters for true multi-return or performance-proven cases.

### mutable Abused To Mutate Real State

Using `mutable` to let a const function change observable state breaks the const contract and surprises callers. Reserve `mutable` for caches and lazy init that do not change observable behavior.

### const_cast To Mutate A const Object

Modifying an object that was originally declared `const` through `const_cast` is undefined behavior (the compiler may have placed it in read-only memory). Do not do this.

### Non-const Getter Forcing Copies

`std::string& name()` (non-const) forces callers with a const instance to copy. Provide a `const` overload (`const std::string& name() const`) so const instances get a reference.

### Forgetting const On A Member Function, Breaking Const-Correct Callers

A getter without `const` cannot be called on a const instance or through a const reference, forcing callers to remove const or copy. Mark read-only member functions const.

## Self-Check

- [ ] Parameters and locals are `const` unless they must change; member functions that do not mutate observable state are marked `const`; const-correctness is propagated leaf-up.
- [ ] Parameter passing matches cost and intent: cheap types by value, expensive read-only types by const reference, sink parameters by value then move; non-const reference parameters are rare and documented.
- [ ] `mutable` is reserved for caches and lazy initialization that do not change observable state, not used to mutate real state from a const function.
- [ ] Types default to value semantics; reference semantics (via smart pointers or references) are used only where sharing, unique resources, or polymorphism require it.
- [ ] Functions return by value (trusting RVO/NRVO and move semantics); no references or pointers to locals are returned; out-parameters are not used to "avoid copies."
- [ ] `constexpr`/`consteval` are applied where compile-time computation is the point, not sprinkled on functions that are never used in a constant context.
- [ ] `const_cast` is rare and reviewed, never used to mutate a truly const object; non-const getters have const overloads so const instances are not forced to copy.
- [ ] The codebase is const-correct: a const instance can call all read-only operations without copying or casting.
