---
name: unsafe-rust
description: Rust unsafe code rules and invariants to preserve when writing or reviewing unsafe blocks, FFI, raw pointers, or manually implementing Send/Sync. Use whenever unsafe code is involved, including raw pointer dereferencing, calling external functions, or implementing unsafe traits.
---

# Unsafe Rust

`unsafe` does not turn off the borrow checker - it gives you extra powers that come with **manual invariants you must uphold.** Violating them is undefined behavior (UB), which can corrupt memory and cause impossible-to-debug failures. Keep these rules sacred.

## What `unsafe` Enables

1. Dereference raw pointers (`*const T`, `*mut T`)
2. Call unsafe functions (including FFI)
3. Implement unsafe traits (`Send`, `Sync`, `GlobalAlloc`...)
4. Access/mutate static `mut` variables
5. Access union fields

## The Golden Rule

**Minimize the unsafe surface.** Keep `unsafe` blocks as small as possible. Encapsulate them behind a safe API that is impossible to misuse. The safety invariant should be maintained by the safe wrapper, not pushed onto the caller.

## Invariants You Must Uphold

### Aliasing and mutation (the silent killer)
Rust assumes `&T` is never mutated and `&mut T` is exclusive. In unsafe code:
- Never have a live `&mut T` alias with any other reference to the same data.
- Never mutate through a reference while a `&T` to the same data exists.
Violating this is UB even if it "works" on your machine.

### Dereferencing raw pointers
A raw pointer is valid to dereference ONLY if:
- It is non-null and properly aligned.
- It points to initialized memory of the right type.
- The memory is still allocated (not freed/moved).

### Implementing Send / Sync manually
This is the most common source of soundness bugs. Ask:
- `Send`: can this type be moved to another thread without data races?
- `Sync`: can `&T` be shared across threads safely?
Only implement these if you've proven thread safety. When in doubt, do NOT implement them.

### FFI boundaries
- C strings: `CString`/`CStr`, never assume NUL-termination of Rust `str`.
- Ownership across FFI: be explicit about who allocates and who frees. Mismatched allocators (e.g., `malloc` vs Rust allocator) corrupt the heap.
- C's `NULL` must be handled; Rust has no null.

## Common Traps

### Assuming "it compiles, so it's safe"
`unsafe` code compiles even when it's UB. Compilers optimize assuming no UB, so UB manifests as random corruption, miscompilation, or crashes far from the cause. Test with Miri.

### Uninitialized memory
`std::mem::MaybeUninit<T>` is the correct way to handle uninitialized memory. Reading `MaybeUninit` before initialization is UB. Never use `mem::uninitialized()` (deprecated and UB-prone).

### Lifetime erasure via raw pointers
Raw pointers carry no lifetime. It's easy to create a pointer that outlives its data. When converting `&T` to `*const T`, track the lifetime manually.

### Data races via static mut
`static mut` shared across threads without synchronization is a data race (UB). Use an `Atomic*` or wrap in a `Mutex` instead.

## Validation Tools

- **Miri** (`cargo +nightly miri test`): executes your code in an interpreter that detects many classes of UB (use-after-free, data races, unaligned access, uninitialized reads). Run it on any code touching unsafe.
- **Tests**: unsafe invariants are not checked by the compiler; tests are your safety net.

## Self-Check

- [ ] Is the `unsafe` block as small as possible?
- [ ] Is there a safe wrapper API that callers cannot misuse?
- [ ] Are all raw pointer dereferences proven valid (non-null, aligned, initialized, alive)?
- [ ] Are no aliasing/mutation invariants violated?
- [ ] Is manual `Send`/`Sync` proven sound?
- [ ] Has the code been run through Miri?
