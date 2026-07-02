---
name: cpp_move_semantics_and_perfect_forwarding.md
description: Use when the agent is writing or reviewing C++ code using rvalue references, move constructors and move assignment, std::move and std::forward, universal references and forwarding references, perfect forwarding, return value optimization (RVO and NRVO), noexcept move, dangling references after move, or deciding when to take parameters by value versus by reference and how to avoid unnecessary copies without creating use-after-move bugs.
---

# Move Semantics And Perfect Forwarding In C++

Move semantics, added in C++11, let resources be transferred (stolen) from one object to another rather than copied, eliminating expensive deep copies when a value is about to be discarded anyway. This is how `std::vector` returned from a function does not copy its heap buffer, how `std::unique_ptr` transfers ownership, and how `std::string` concatenation stays cheap. The mechanism — rvalue references (`T&&`), move constructors, and `std::move` — is straightforward in the happy case and treacherous at the edges: a `std::move` that moves nothing (because the type has no move, or because it was applied to a `const` lvalue), a moved-from object used as if it still held its value, a forwarding reference that binds the wrong way and silently copies, a return-by-value that the compiler did not optimize because of a missed `return` form. The judgment problem is to know when a move actually happens and when it does not, to leave moved-from objects in a valid-but-unspecified state and never use them as if they held their old value, and to use perfect forwarding correctly without creating surprising overload resolution.

Agents tend to sprinkle `std::move` on every return ("to make it efficient") and `std::forward` on every template parameter, then discover that `std::move` on a local return defeated copy elision, that a moved-from string was read as if it still had content, or that a "universal reference" constructor absorbed a copy constructor's job. The harm is silent inefficiency (a move that copies), subtle use-after-move bugs, or overload-resolution surprises. This skill is about treating move and forwarding as operations with precise conditions, not as incantations.

## Core Rules

### Know When A Move Actually Happens (And When It Does Not)

A move occurs only when (a) the source is an rvalue (a temporary, or an lvalue explicitly cast with `std::move`), (b) the type has a move constructor or move assignment, and (c) the destination can use it. Several common cases silently do not move.

- `std::move` does not move anything; it casts an lvalue to an rvalue reference. The actual move happens in the move constructor/assignment that the cast enables. If the type has no move operations (they were not declared or were implicitly deleted), `std::move(x)` falls back to a copy with no warning.
- `std::move` on a `const` object (`const std::string s; std::move(s);`) yields a `const rvalue`, which binds to the copy constructor (because move constructors take non-const `T&&`), so it copies. This is a frequent silent inefficiency in code that stores values in `const` containers.
- A member that lacks a move (a `const` member, a member with a deleted move) makes the enclosing class's move constructor fall back to copy for that member, or be deleted entirely. `std::move` on a class with a non-movable member copies that member.

The discipline: before assuming a move happened, verify the type is movable and the source is a non-const rvalue. A move that silently copies is a performance bug that profiling reveals.

### Treat Moved-From Objects As Valid But Unspecified

After `auto y = std::move(x);`, `x` is in a "valid but unspecified" state: it is destructible and assignable, but you must not assume anything about its value. Reading `x`'s contents as if it still held its old value is a use-after-move bug.

- Standard-library types leave moved-from objects in a valid, empty-ish state (`std::string` is usually empty, `std::vector` is usually empty), but the standard only guarantees valid-but-unspecified. Do not rely on a specific moved-from value.
- After moving from a local, either do not use it again, or assign to it before reading. The most common bug is `return std::move(x); ... use x;` or moving a member in a constructor and then reading it.
- Compiler warnings (`-Wuse-after-move`) catch many of these; enable them. The bug is subtle because the moved-from object often "happens to" hold a plausible value in testing and fails when the allocator reuses the memory.

Strong choice: `auto y = std::move(x); x = make_new(); /* then use x */`. Weak choice: `auto y = std::move(x); process(x.data()); /* x.data() is unspecified */`.

### Do Not std::move A Return Value Of Local Scope

Returning a local by value is already optimal: the compiler applies copy elision (NRVO) or, failing that, treats the local as an rvalue for the return, so the move happens implicitly. Wrapping the return in `std::move` defeats NRVO and can pessimize the code.

- `return x;` where `x` is a local of the same type as the return — the compiler tries NRVO (constructing the return value directly in the caller's storage), which is better than a move. `return std::move(x);` inhibits NRVO and forces a move.
- The exception: when returning a member or a value that is not a same-type local (e.g., returning `std::move(member)` from a class that is being destroyed), `std::move` may be appropriate. But for a plain local return, do not move.

This is one of the few cases where adding `std::move` makes code slower. Linters and `-Wpessimizing-move` flag it.

### Take Parameters By Value For Sink Parameters, By Reference Otherwise

The choice of parameter type expresses intent and affects copies.

- For a **sink** parameter (the function will store or take ownership of the value), take it by value and move into place: `void add(std::string s) { items_.push_back(std::move(s)); }`. The caller's lvalue is copied into `s` then moved into the vector; the caller's rvalue is moved into `s` then moved again. This is one overload that handles both cases reasonably.
- For a **read** parameter (the function only inspects the value), take `const T&` for non-trivial types (avoid a copy and a move), and `T` for trivial types (a copy is cheaper than an indirection).
- For a parameter that must be forwarded elsewhere with its value category preserved (a factory, an `emplace`-style constructor), use a forwarding reference (`T&&`) and `std::forward`.

### Distinguish Forwarding References From Rvalue References

A `T&&` in a deduced context (`template<typename T> void f(T&& x);` or `auto&& x = ...`) is a forwarding reference (universal reference): it binds to both lvalues and rvalues, deducing `T` as `T&` for an lvalue and `T` for an rvalue. A `T&&` in a non-deduced context (`void f(std::string&& x);`) is a plain rvalue reference: it binds only to rvalues.

- A forwarding reference that should be an rvalue reference (or vice versa) is a common source of overload-resolution surprises. A templated constructor `template<typename T> Wrapper(T&& x)` accepts everything, including lvalues and const values, and may silently shadow the copy constructor or bind to unintended types.
- Use `std::forward<T>(x)` to preserve the value category of a forwarding reference when passing it along; use `std::move(x)` only when you know `x` is an rvalue (a non-deduced `T&&` parameter or a local you are done with). Using `std::move` on a forwarding reference moves an lvalue the caller passed, which is usually a bug.
- Constrain templated forwarding-reference constructors with concepts or `enable_if`/SFINAE so they do not absorb the copy/move constructors or bind to unrelated types.

### Perfect-Forward Through std::forward, And Know Its Single-Use Limit

Perfect forwarding passes an argument to another function preserving its value category (lvalue stays lvalue, rvalue stays rvalue), which is how `std::make_unique<T>(args...)` and `emplace_back(args...)` forward constructor arguments without copies. `std::forward<T>(x)` does this for a forwarding reference `x`.

- `std::forward<T>(x)` is a conditional move: if `T` deduced to an rvalue, it casts to rvalue; if `T` deduced to an lvalue, it casts to lvalue (a no-op). It must be called with the deduced `T`.
- `std::forward` may move, so forwarding the same argument twice moves it the second time (the second forward sees a moved-from object). Forward each argument exactly once.
- Forwarding does not work through a `const` barrier: `std::forward<const T&>` always yields an lvalue, so a function taking `const T&` cannot perfectly forward a mutable rvalue. Design forwarding chains to preserve mutability where forwarding matters.

### Mark Move Operations noexcept Where They Cannot Throw

A `noexcept` move constructor/assignment lets containers and algorithms use move during reallocation and reorganization; a throwing move forces them to copy to preserve the strong exception guarantee. `std::vector` checks `is_nothrow_move_constructible` and copies element-by-element if the move can throw.

- Mark move constructors and move assignment `noexcept` when they only transfer pointers/primitives (the common case). This is a real performance win for containers of movable types.
- If a move genuinely allocates or calls user code that can throw, do not mark it `noexcept` (a throwing `noexcept` calls `std::terminate`). But most moves are pointer swaps and should be `noexcept`.

## Common Traps

### std::move On A const Or Non-Movable Object Silently Copying

`std::move(const_string)` yields a `const rvalue` that binds to the copy constructor, so it copies. Verify the type is movable and the source is non-const before assuming a move happened.

### std::move On A Return Defeating NRVO

`return std::move(local);` inhibits named return value optimization and forces a move where copy elision would have been free. Return the local directly: `return local;`.

### Using A Moved-From Object As If It Held Its Value

`auto y = std::move(x); log(x.size());` reads `x` after move; `x.size()` is valid but unspecified. Either do not use `x` or reassign it first.

### std::move On A Forwarding Reference Moving An lvalue

`template<typename T> void f(T&& x) { target(std::move(x)); }` moves an lvalue the caller passed, which the caller did not expect. Use `std::forward<T>(x)` to preserve the caller's value category.

### Forwarding The Same Argument Twice

`target(std::forward<T>(x), std::forward<T>(x));` moves `x` on the first forward and forwards a moved-from object on the second. Forward each argument once.

### Templated Constructor Absorbing The Copy Constructor

`template<typename T> Wrapper(T&& x)` accepts lvalues, const values, and unrelated types, shadowing the copy constructor and binding where a copy was intended. Constrain it with concepts or SFINAE (`std::enable_if_t<...>` or `requires`).

### noexcept Move That Actually Throws

Marking a move `noexcept` when it calls allocating or user code that can throw, then watching `std::terminate` when it throws. Only mark `noexcept` for moves that genuinely cannot throw.

### Assuming std::vector Will Move During Reallocation

`std::vector<ThrowingMove>` copies element-by-element during reallocation because the move can throw and the strong guarantee requires it. Mark moves `noexcept` to enable move-during-realloc.

## Self-Check

- [ ] Every `std::move` is on a non-const, movable source whose value category genuinely should become rvalue; no `std::move` silently copies due to a `const` or non-movable type.
- [ ] No moved-from object is read as if it held its old value; moved-from locals are either unused or reassigned before reading, and `-Wuse-after-move` is enabled and clean.
- [ ] No `std::move` wraps a return of a same-type local (which would defeat NRVO); locals are returned directly so copy elision applies.
- [ ] Sink parameters are taken by value and moved into place; read parameters use `const T&` (non-trivial) or `T` (trivial); forwarding parameters use forwarding references.
- [ ] Forwarding references (`T&&` in a deduced context) are forwarded with `std::forward<T>`, not `std::move`; templated forwarding constructors are constrained so they do not absorb copy/move constructors.
- [ ] Each forwarded argument is forwarded exactly once; no double-forward of the same argument.
- [ ] Move constructors and move assignment are `noexcept` where they cannot throw (the common case), enabling container move-during-reallocation; throwing moves are not marked `noexcept`.
- [ ] The distinction between forwarding reference (deduced `T&&`) and rvalue reference (non-deduced `T&&`) is clear at each use, and overload resolution has been checked for surprises.
- [ ] Profiling confirms that intended moves actually move (no silent copies from `const`, non-movable members, or `std::move`-on-return).
