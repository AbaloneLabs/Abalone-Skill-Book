---
name: cpp_raii_and_resource_management.md
description: Use when the agent is writing or reviewing C++ code involving constructors and destructors, exception safety guarantees, move and copy semantics, resource acquisition order, destructor reverse-order teardown, custom deleters, raw resource wrapping, or any class that owns a handle, file, socket, lock, allocation, or external object whose cleanup must be deterministic and exception-safe.
---

# RAII And Resource Management In C++

RAII (Resource Acquisition Is Initialization) is the central organizing principle of C++ resource management: tie every resource's lifetime to an object's lifetime, acquire in the constructor, release in the destructor, and let normal scope exit and object destruction handle cleanup deterministically. Done well, this makes C++ code leak-free and exception-safe almost for free. Done badly — with manual new/delete, raw resource handles held as members without ownership semantics, or destructors that can throw — it produces leaks, double-frees, partial initialization, and undefined behavior that only surfaces under the rare error path that was never tested.

Agents often write C++ as "C with classes," sprinkling `new` and `delete` and assuming the happy path is enough. The judgment problem is that the happy path is never the problem. The problem is the path where the second allocation fails after the first succeeded, where an exception propagates through a half-constructed object, where a move leaves the source in a state the destructor still has to handle, where two members depend on each other's teardown order. RAII is not a style preference; it is the only systematic way to make cleanup correct across every exit path, and getting it right means thinking about acquisition order, destruction order, exception guarantees, and the copy/move contract before writing the constructor.

## Core Rules

### Make Every Resource A Member With Ownership Semantics

A raw resource (pointer, file descriptor, socket handle, mutex, GL object) held as a bare member is a bug waiting for the right error path. Wrap every resource in a type whose destructor releases it, and prefer standard wrappers (`std::unique_ptr`, `std::shared_ptr`, `std::fstream`, `std::lock_guard`, `std::vector`) over hand-rolled ones.

- A class that owns a resource should either be a thin RAII wrapper around that single resource, or should compose existing RAII types as members. Avoid classes that own multiple raw resources directly.
- If you must manage a raw resource, follow the Rule of Five (or Zero): either define all of destructor, copy constructor, copy assignment, move constructor, move assignment, or define none of them and rely on members that handle themselves (Rule of Zero).
- Rule of Zero is the default goal. A class whose members are all RAII types needs no hand-written special members, which eliminates a large class of copy/move/destructor bugs.

Strong choice: `std::vector<byte>` and `std::unique_ptr<Handle>` as members, no custom destructor. Weak choice: a raw `byte* data; size_t len;` pair with a hand-written destructor that the copy constructor forgot to update.

### Guarantee The Basic Exception Safety, Aim For Strong Where Affordable

Every operation should offer at least the basic exception guarantee: if it throws, the program remains in a valid state, no resources leak, and no invariants are broken. The strong guarantee (commit-or-rollback: if it throws, the state is unchanged as if the operation never started) is desirable for operations that can afford it but is not always required.

- The basic guarantee is the floor, not an aspiration. Code that leaks or corrupts state on an exception is incorrect, even if the exception is rare.
- Achieve the strong guarantee via copy-and-swap or by performing all allocating/may-throw work before mutating shared state. Construct the new state in temporaries, then commit with non-throwing operations (pointer swaps, moves of nothrow-move types).
- Know which operations can throw. Allocation (`new`), most operations on user types, and anything that calls back into user code can throw. Move, swap of pointers, and primitive operations generally do not. The strong guarantee is only real if the commit step is genuinely non-throwing.

### Order Member Initialization And Destruction Deliberately

Members are initialized in the order they are declared in the class, not the order they appear in the initializer list; bases are initialized before members. Destruction is the exact reverse. This ordering is load-bearing when members depend on each other.

- If member B depends on member A (e.g., B holds a reference to A, or B's constructor reads A), A must be declared before B. The initializer-list order does not override declaration order; compilers warn when they differ, and you should fix the declaration order rather than silence the warning.
- Destruction runs in reverse, so a member that other members depend on must be destroyed last, i.e., declared first. If a destructor of one member touches another member that has already been destroyed, that is undefined behavior.
- Base classes are destroyed after member destruction in the most-derived to base order; ensure bases do not hold references to derived members that vanish first.

State the dependency order in a comment when it is non-obvious, because the next maintainer will reorder members "for readability" and break it.

### Define The Copy And Move Contract Explicitly

Every type has a copy/move contract, whether you write it or not. The compiler-generated versions may be wrong for resource-owning types. Decide and implement the contract deliberately.

- **Copyable and copy is expensive (deep copy):** typical for value-semantic types that own resources. The copy constructor allocates new resources; copy assignment must release the old resources and acquire new ones safely (use copy-and-swap to get self-assignment and exception safety for free).
- **Move-only:** the type owns a unique resource and cannot be meaningfully copied (`std::unique_ptr`, file handles, sockets). Delete the copy operations, define move operations, and ensure the moved-from object is in a valid (typically empty) state that its destructor can handle.
- **Non-copyable, non-movable:** rare; for types whose identity matters (mutexes, locks). Delete all four special members.
- **Reference types (shared ownership):** use `shared_ptr` rather than hand-rolling reference counting; hand-rolled refcounts are a classic source of use-after-free and leaks.

A moved-from object must be destructible and assignable-to. The most common move bug is leaving the source in a state where the destructor double-frees; the fix is to null/reset the source's resource pointers in the move constructor and assignment.

### Never Let Destructors Throw

A destructor that throws is a serious defect. If a destructor throws during stack unwinding from another exception, `std::terminate` is called and the program aborts. Even outside unwinding, a throwing destructor breaks the basic exception guarantee for every type that contains it.

- Destructors must catch and swallow (or log) any exceptions from their cleanup operations. This applies to member and base destructors too, which is why standard-library types have noexcept destructors.
- If cleanup can genuinely fail in a way the caller must know about, do not put it in the destructor. Provide a separate `close()`/`flush()`/`commit()` method that can throw, and have the destructor call a best-effort fallback that swallows.
- Mark destructors `noexcept` (they are by default in C++11+) so the contract is enforced; a throwing destructor in a noexcept context terminates.

### Initialize Fully, Or Fail Before Construction Completes

A constructor that throws leaves no object behind; its members' destructors run for the members already constructed, and the memory is freed. This is the mechanism that makes RAII exception-safe. Use it.

- Acquire all resources in the constructor, and if any acquisition fails, throw so the partially-constructed object is cleaned up automatically. Do not return a "half-alive" object via a two-phase init.
- Prefer member initializers and in-class initializers so members always have a value, reducing the chance of an uninitialized read on an error path.
- For resources that need a non-throwing failure mode (e.g., embedded or no-exceptions builds), use factory functions that return `std::optional<T>` or status-plus-out, but keep the constructed object's invariant intact.

### Make Move Cheap And noexcept Where Possible

Move semantics let resources be transferred without copying, but only if moves are actually cheap and the compiler can use them. A throwing move defeats many optimizations and forces copies.

- Mark move constructors and move assignment `noexcept` when they cannot throw (which is the common case for pointer/resource transfers). Containers like `std::vector` check `is_nothrow_move_constructible` and will only use move during reallocation if it is noexcept; otherwise they copy to preserve the strong guarantee.
- A move that allocates or calls user code is not really a move; it is a copy with extra steps. Keep moves to pointer swaps and primitive assignments.

## Common Traps

### Two-Phase Init That Defeats RAII

A class with a constructor that leaves the object "uninitialized" and a separate `init()` that must be called before use throws away RAII's main benefit: the guarantee that a constructed object is usable and that failure cleans up. Prefer constructor arguments, or a factory function that returns a fully-constructed object or fails. If you must use two-phase init, make the intermediate state explicitly invalid and check it everywhere — but recognize this is a smell.

### Copy Assignment That Leaks On Self-Assignment Or Throws

`T& operator=(const T& other)` that does `delete data; data = new Resource(*other.data);` leaks on self-assignment (deletes its own data before copying) and corrupts state if `new` throws (data already deleted). The copy-and-swap idiom fixes both: take other by value (or by const-ref then copy), then swap, letting the old data be destroyed when the local goes out of scope.

### Move That Leaves The Source Undestructible

`T(T&& other) : data(other.data) {}` without nulling `other.data` means both the new object and the moved-from object point at the same resource; both destructors will free it. Always reset the source's resource pointers to null/empty in the move constructor and assignment.

### Forgetting The Rule Of Five After Adding A Destructor

Writing a destructor to free a resource but leaving the compiler-generated copy constructor means the copied object shares the pointer and both destructors free it (double-free). If you write any one of the five special members for a resource-owning type, you must consider all five. Prefer Rule of Zero (compose RAII members) to avoid this entirely.

### Virtual Destructor Missing On A Polymorphic Base

A base class with virtual functions that lacks a `virtual ~Base()` causes undefined behavior when a derived object is deleted through a base pointer: the derived destructor never runs, leaking its resources. Any class meant to be a polymorphic base needs a virtual destructor. Conversely, a non-polymorphic class should not have a virtual destructor (it adds overhead for no benefit).

### Destructor Touching A Member After Another Member Destroyed It

If member B holds a pointer/reference to member A and A is declared after B, then during destruction B is destroyed before A... wait, destruction is reverse declaration order, so A (declared first) is destroyed last. The trap is the reverse: if A is declared after B, A is destroyed before B, and B's destructor touching A reads destroyed memory. Declare members in dependency order.

### Throwing In A Destructor During Unwinding

A destructor that calls a function which may throw, during unwinding from another exception, calls `std::terminate`. Wrap all cleanup in try/catch inside destructors, or move throwing cleanup to an explicit method.

## Self-Check

- [ ] Every owned resource is held by an RAII member (smart pointer, container, fstream, lock guard) rather than a raw handle, and the class follows the Rule of Zero or explicitly implements all of the Rule of Five.
- [ ] Every operation offers at least the basic exception guarantee (no leaks, valid state on throw), and operations where it is affordable offer the strong guarantee via copy-and-swap or prepare-then-commit.
- [ ] Member declaration order matches the dependency order, so dependents are constructed after and destroyed before the members they depend on, and non-obvious ordering is commented.
- [ ] The copy/move contract is explicit: copyable types deep-copy safely (self-assignment and exception safe), move-only types delete copy and leave moved-from objects destructible, and shared ownership uses `shared_ptr` rather than hand-rolled refcounts.
- [ ] No destructor throws; cleanup that can fail is exposed via a separate method, and destructors swallow or log exceptions from their internal cleanup.
- [ ] Constructors either fully initialize the object or throw so partial construction is cleaned up automatically, with no two-phase init that leaves a constructed-but-unusable object.
- [ ] Move constructors and move assignment are marked `noexcept` where they cannot throw and reset the source's resource pointers so the moved-from object remains destructible.
- [ ] Polymorphic base classes have a virtual destructor, and non-polymorphic classes do not.
- [ ] No member's destructor reads or touches another member that has already been destroyed, given reverse-order destruction.
- [ ] The class has been reasoned about under the failure path where the second resource acquisition fails after the first succeeded, and that path cleans up correctly.
