---
name: cpp_smart_pointer_selection.md
description: Use when the agent is writing or reviewing C++ code that chooses between unique_ptr, shared_ptr, weak_ptr, or raw pointers, designs ownership graphs, breaks reference cycles, writes custom deleters, decides between make_unique/make_shared and direct new, handles array vs single-object deleters, or reasons about the overhead, thread-safety, and exception-safety consequences of each smart pointer choice.
---

# Smart Pointer Selection In C++

Smart pointers are how C++ encodes ownership after RAII. Each kind expresses a different ownership model: `unique_ptr` for exclusive ownership, `shared_ptr` for shared ownership via reference counting, `weak_ptr` for non-owning observation of a shared object, and raw pointers (or references) for non-owning borrows. Choosing the wrong one does not just add overhead — it misstates the design, invites cycles that leak, creates dangling weak references, or forces shared ownership where none was intended and thereby extends object lifetimes in surprising ways.

Agents often default to `shared_ptr` because it "just works" and removes the need to think about ownership. That convenience is the trap. `shared_ptr` makes lifetimes non-obvious, hides cycles, adds atomic-reference-count overhead on every copy, and turns what should be a clear ownership graph into an implicit web that no one fully understands. The judgment problem is to pick the pointer that expresses the real ownership intent, to prefer the cheapest one that is correct, and to recognize when shared ownership is a genuine design decision rather than a way to avoid making one. The default should be `unique_ptr`; everything else needs a reason.

## Core Rules

### Default To unique_ptr, Escalate Only With Justification

`std::unique_ptr` expresses exclusive ownership with zero overhead relative to a raw pointer. It is movable but not copyable, it deletes the object in its destructor, and it is the right default for any owned heap object whose lifetime is tied to a single owner.

- Use `unique_ptr` when exactly one owner is responsible for the object's lifetime and can transfer that ownership (e.g., factory functions returning a newly created object, a container of polymorphic objects, a class member that owns a subsystem).
- Pass `unique_ptr` by move to transfer ownership; pass a raw pointer or reference (`T*`, `T&`) when a callee just needs to use the object without owning it. Do not pass `unique_ptr` by value except to transfer ownership, and never by const-ref just to "be safe."
- Escalate to `shared_ptr` only when ownership is genuinely shared across multiple independent lifetimes that cannot be expressed as a single owner plus borrows. This is rarer than it first appears; most "shared" needs are actually one-owner-plus-many-borrowers.

Strong choice: `std::unique_ptr<Widget> make_widget();` with callers taking `Widget&` or `Widget*`. Weak choice: `std::shared_ptr<Widget>` everywhere "so I don't have to think about lifetimes."

### Use shared_ptr For Genuine Shared Ownership, And Document Why

`std::shared_ptr` uses atomic reference counting to keep an object alive as long as any shared_ptr points to it. It is the right tool when the object's lifetime is genuinely shared and cannot be reduced to a single owner — for example, a node in a graph that several other nodes reference, or a resource handed to asynchronous callbacks whose relative completion order is unknown.

- Each `shared_ptr` copy is an atomic increment (and the destructor an atomic decrement), which is more expensive than a raw pointer copy. This is fine when ownership actually changes hands; it is wasteful when a function only needs to read.
- `shared_ptr` extends lifetime implicitly: any copy anywhere keeps the object alive. This makes it hard to reason about when an object is actually destroyed, which complicates resource management (open files, locks, GPU handles) that needs deterministic teardown.
- The control block (the refcount allocation) is shared by all shared_ptrs and weak_ptrs to the same object. Constructing a shared_ptr from the same raw pointer twice creates two control blocks and double-deletes. Always create a shared_ptr once and copy it.

When you reach for `shared_ptr`, write a one-line comment explaining why ownership is shared and cannot be single-owner. If you cannot articulate the reason, it is probably a `unique_ptr`.

### Break Cycles With weak_ptr, And Use It For Non-Owning Observation

`shared_ptr` cycles leak: if A holds a shared_ptr to B and B holds a shared_ptr to A, neither refcount ever reaches zero and both leak forever. The fix is `weak_ptr`, which observes a shared object without keeping it alive.

- Use `weak_ptr` for back-pointers in a parent-child graph where the child should not keep the parent alive (child holds `weak_ptr<Parent>`), and for caches that should not pin their entries.
- To use a `weak_ptr`, lock it into a `shared_ptr` (`auto sp = wp.lock();`) and check that the result is non-null before use; the object may have been destroyed between observations. This lock is the only safe way to access the object.
- `weak_ptr` itself does not dangle in a use-after-free sense (locking a dangling weak_ptr returns null), but it does require the discipline of always checking the lock result.

Cycles are the single most common `shared_ptr` defect. Any time two types mutually reference each other via shared_ptr, one direction must be weak.

### Prefer make_unique And make_shared Over Direct new

`std::make_unique<T>(args...)` and `std::make_shared<T>(args...)` allocate and construct in one step, avoid writing the type twice, and (for make_shared) coalesce the object and control block into a single allocation.

- `make_shared` is a real performance win: one allocation instead of two, and better cache locality. Prefer it whenever the type's destructor does not need to run promptly (see the tradeoff below).
- The make_shared tradeoff: because the control block and object share an allocation, the object's memory cannot be freed until all weak_ptrs are also gone, even if all shared_ptrs are destroyed. For large objects or objects holding scarce resources, this can delay teardown noticeably; in those cases, `shared_ptr(new T(...))` (two allocations) frees the object as soon as the last shared_ptr dies.
- Never mix: do not construct a shared_ptr from a raw pointer that is already managed by another shared_ptr; this creates a second control block and double-deletes.

### Use Custom Deleters Deliberately, Not As A Patch

`unique_ptr<T, Deleter>` and `shared_ptr<T>` (which accepts a custom deleter in its constructor) can call a custom function instead of `delete`. This is the right tool for non-memory resources wrapped in smart-pointer ownership (C APIs that return a handle and provide a close function, `FILE*` with `fclose`, `int` fd with `close`, GL objects).

- The custom deleter for `unique_ptr` is part of the type, so `unique_ptr<FILE, decltype(&fclose)>` and `unique_ptr<FILE, void(*)(FILE*)>` are different types. This affects containers and function signatures; consider a type alias or a stateless functor deleter.
- For `shared_ptr`, the deleter is stored in the control block and is not part of the type, so `shared_ptr<FILE>` with different deleters are the same type — more flexible but with a small runtime cost.
- Prefer wrapping a C resource in a small dedicated RAII class rather than spreading `unique_ptr<T, weird_deleter>` through the codebase; the dedicated class documents the resource type and centralizes the deleter.

### Distinguish Array And Single-Object Forms

`unique_ptr<T[]>` uses `delete[]`; `unique_ptr<T>` uses `delete`. Mixing them is undefined behavior. `shared_ptr` has no array specialization before C++17 and even then lacks `operator[]`; prefer `std::vector` or `std::array` over `shared_ptr<T[]>` for arrays.

- Prefer `std::vector<T>` over `unique_ptr<T[]>` in almost all cases; the vector is a complete RAII type with size, bounds-checked access (via `.at()`), and iterator support.
- Use `unique_ptr<T[]>` only when interfacing with APIs that require a raw array buffer you must own, and even then consider `std::vector` plus `.data()`.

### Use Raw Pointers And References For Non-Owning Borrows

Not every pointer needs to be smart. A function that takes an object and uses it without extending its lifetime should take `T&` (non-null, non-owning) or `T*` (nullable, non-owning), not a smart pointer. Smart pointers in parameter lists should signal ownership transfer.

- `void process(Widget& w)` — borrows, non-null, does not extend lifetime. The best default for "I need to use this object."
- `void process(Widget* w)` — borrows, may be null. Use when null is a meaningful input.
- `void take(std::unique_ptr<Widget> w)` — takes ownership by value (sink). Use when the function becomes the owner.
- `void observe(std::shared_ptr<Widget> w)` — extends lifetime. Rarely the right signature; usually the function should take `Widget&` instead. Only do this when the function genuinely needs to keep the object alive beyond the call (e.g., storing it for async use).

Avoid `const std::unique_ptr<T>&` and `const std::shared_ptr<T>&` parameters that neither transfer nor extend ownership; they force callers to wrap in a smart pointer for no benefit and obscure that the function only borrows.

## Common Traps

### shared_ptr Everywhere To Avoid Thinking About Ownership

Spraying `shared_ptr` across a codebase does not remove ownership questions; it hides them. Lifetimes become implicit, cycles leak silently, and refcount churn adds overhead. Default to `unique_ptr` and raw borrows; reserve `shared_ptr` for genuine shared ownership with a stated reason.

### Reference Cycles That Leak

Two classes holding `shared_ptr` to each other leak forever because neither refcount reaches zero. The symptom is a slow memory leak that does not show up in normal testing. Audit any mutual reference for a weak back-pointer.

### Constructing Two shared_ptrs From One Raw Pointer

`Thing* p = new Thing; shared_ptr<Thing> a(p); shared_ptr<Thing> b(p);` creates two independent control blocks; both destructors delete `p`. Always create the shared_ptr once and copy it, or use `make_shared`.

### Passing shared_ptr By Value When A Borrow Suffices

`void use(shared_ptr<Widget> w)` forces an atomic refcount increment/decrement on every call and signals ownership transfer that may not be intended. If the function only reads the object, take `Widget&` or `const Widget&`. Reserve shared_ptr parameters for genuine lifetime extension (e.g., storing into a structure that outlives the call).

### make_shared Delaying Teardown Of A Large Or Scarce Resource

Because make_shared coalesces the object and control block, the object's memory is not freed until the last weak_ptr is gone. For a large buffer or a resource that must be released promptly (a file, a lock, a GPU buffer), this delay can matter; use `shared_ptr(new T(...))` or release the resource explicitly before the last shared_ptr dies.

### weak_ptr Used Without Locking

Dereferencing a `weak_ptr` directly is impossible; you must `lock()` it to get a shared_ptr and check for null. Forgetting the check, or holding the raw pointer obtained some other way while the shared_ptr is alive, leads to use-after-free once the object is destroyed. Always lock and check.

### Custom Deleter That Captures State Unsafely

A `shared_ptr` custom deleter that captures a reference or pointer to local state can dangle if the shared_ptr outlives that state. Keep deleters stateless or ensure captured state outlives all copies of the shared_ptr.

### unique_ptr With The Wrong Deleter Type In A Container

`std::vector<std::unique_ptr<FILE, decltype(&fclose)>>` requires the deleter as part of the type and a constructor argument per element, which is awkward. Prefer a dedicated RAII wrapper type (e.g., `struct FileHandle { unique_ptr<FILE, ...> ... };`) so the container holds a clean type.

## Self-Check

- [ ] The default for owned heap objects is `unique_ptr`, and every use of `shared_ptr` has a stated reason why ownership is genuinely shared rather than single-owner-plus-borrows.
- [ ] No `shared_ptr` reference cycle exists; mutual references use `weak_ptr` on at least one side, and any parent-child or graph relationship has been audited for cycles.
- [ ] Each shared object has exactly one control block; no raw pointer is used to construct more than one `shared_ptr`, and `make_shared`/`make_unique` are preferred over direct `new`.
- [ ] `weak_ptr` observers always `lock()` into a `shared_ptr` and check for null before use; no code dereferences or assumes a weak_ptr's target is alive without locking.
- [ ] Non-owning function parameters take `T&` or `T*`, not smart pointers; smart-pointer parameters signal ownership transfer (`unique_ptr` by value) or genuine lifetime extension (`shared_ptr` by value), not mere borrowing.
- [ ] Custom deleters are deliberate and documented, wrap a real non-memory resource, and either are stateless or capture state that provably outlives all copies.
- [ ] Arrays use `unique_ptr<T[]>` only when a raw owned buffer is required, and `std::vector`/`std::array` are preferred; single-object and array forms are not mixed.
- [ ] Where `make_shared` is used for a large or scarce resource, the delayed-teardown tradeoff has been considered, and `shared_ptr(new T)` or explicit release is used if prompt teardown matters.
- [ ] The ownership graph is locally readable: a reader can determine, for each pointer, whether it owns, borrows, shares, or observes, without tracing the whole program.
- [ ] No `shared_ptr` is being used as a substitute for a design decision about who owns the object.
