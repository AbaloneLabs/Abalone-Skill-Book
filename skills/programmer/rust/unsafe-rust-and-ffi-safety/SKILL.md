---
name: rust_unsafe_and_ffi_safety.md
description: Use when the agent is writing or reviewing Rust unsafe blocks, raw pointers, FFI bindings to C or C++ libraries, calling extern functions, exposing Rust functions to C via #[no_mangle], wrapping C libraries, managing memory across the FFI boundary, reasoning about aliasing and uninitialized memory, using MaybeUninit, transmute, or split_at_mut, or auditing unsafe code for soundness. Covers the proof obligations unsafe imposes, the boundary between safe and unsafe APIs, C string and ABI handling, ownership transfer across FFI, unwind safety, and how to contain unsafety behind a narrow tested safe wrapper.
---

# Unsafe Rust And FFI Safety

`unsafe` does not turn off Rust's rules; it moves the proof obligations from the compiler to you. Inside an `unsafe` block, the compiler trusts that you have manually verified the operations are sound. FFI (foreign function interface) is the largest source of `unsafe` in real codebases, because every call into C crosses a boundary where Rust's guarantees end and C's conventions begin. The judgment problem is containing unsafety, documenting the invariants you are upholding, and never letting an unsound assumption leak into safe code that other code trusts.

Agents tend to write large `unsafe` blocks, skip documenting which invariant they are upholding, transmute between types casually, assume C functions behave like Rust functions, leak memory across the FFI boundary, or expose raw pointers through a "safe" wrapper that does not actually enforce the preconditions. The harm is the worst kind of Rust bug: memory unsafety, undefined behavior, data races, and soundness holes in code that *looks* safe because it is wrapped in a safe function. The real work is making unsafe blocks tiny, stating every proof obligation, and building a narrow safe API that enforces the invariants the unsafe code relies on.

## Core Rules

### Treat unsafe As A Proof Obligation, Not A Permission Slip

Every `unsafe` block is a promise: "I have verified that the operations inside are sound, and I can state why." Before writing an unsafe block, enumerate the proof obligations for each operation:

- **Pointer validity**: is the pointer non-null, aligned, and pointing to initialized, valid data of the right type? For how long?
- **Aliasing**: do you have exclusive access where mutation requires it (`&mut` semantics), or is shared read access sufficient? Are there other live references that violate Rust's aliasing rules?
- **Initialization**: is the memory initialized? Reading uninitialized memory is undefined behavior, even if you "know" what it contains.
- **Lifetime**: does the pointed-to data live as long as you use the reference? Does it outlive any FFI call?
- **Thread safety**: is the operation safe across threads, or does it require synchronization the C side does not provide?

Write a comment on each unsafe block stating which invariant you are upholding. If you cannot state it, you do not understand the block, and it is probably unsound. Prefer a safe alternative (a stdlib method, a vetted crate) whenever one exists; reach for unsafe only when no safe option does what you need.

### Keep unsafe Blocks Small And Localized

A large unsafe block mixes operations that need unsafe with operations that do not, obscuring which line carries the proof obligation and which invariant applies. Keep unsafe blocks to the minimum set of operations that actually require it.

- Extract the single unsafe operation into its own function or block with a documented precondition.
- Do safe setup and validation outside the unsafe block; enter unsafe only for the operation that needs it.
- If a block grows, split it. Each unsafe block should have one clear reason for existing and one stated invariant.

Small unsafe blocks are auditable. Large ones hide bugs because the reviewer cannot tell which operation is load-bearing.

### Build A Safe Wrapper That Enforces The Preconditions

Unsafe code that is exposed directly forces every caller to reason about soundness. The correct pattern is a narrow unsafe core (the FFI calls, the raw pointer operations) wrapped by a safe public API that enforces the preconditions the unsafe code relies on.

- The safe wrapper validates inputs (non-null, in-range, correct types) before calling the unsafe core.
- The safe wrapper manages ownership and lifetimes so callers cannot hold a dangling pointer.
- The safe wrapper translates C error codes or null returns into `Result` or `Option`.
- The unsafe core's preconditions are documented as comments so future edits to the wrapper do not violate them.

The soundness of the whole module rests on the wrapper enforcing the preconditions. If a future change lets an invalid input reach the unsafe core, the module becomes unsound while still compiling. Test the wrapper's input validation as deliberately as you test its happy path.

### Handle The FFI Boundary's Ownership And Memory Rules

FFI crosses between Rust's ownership model and C's manual memory management. The single most important question at any FFI boundary is: who owns this memory, and who is responsible for freeing it?

- **C allocates, Rust uses**: Rust gets a pointer it must not free; it must finish using it before C frees it (or copy the data into owned Rust memory).
- **Rust allocates, C uses**: Rust must keep the memory alive and properly laid out for as long as C uses it, and free it through the right allocator. Do not hand C a pointer into a Rust `String`/`Vec` and then resize or drop it.
- **Passing strings**: C expects null-terminated `char*`. Use `CString`/`CStr`, not raw `String` slices (which are not null-terminated and whose internal pointer is invalidated on resize). Be aware of interior null bytes, which `CString` rejects.
- **Repr and layout**: use `#[repr(C)]` on structs passed across FFI so the layout matches C's expectations. Rust's default struct layout is unspecified and can reorder fields.

Memory bugs across FFI are the most common source of crashes and security holes in Rust FFI code. Document ownership for every pointer parameter and return value. When in doubt, copy data across the boundary into clearly-owned memory.

### Respect The ABI And Calling Conventions

`extern "C"` uses the C calling convention, which is the lingua franca for FFI, but conventions vary by platform and function. Be explicit and correct:

- Declare `extern "C"` (or the specific ABI like `"stdcall"`) on both sides; do not rely on defaults.
- Use C-compatible types at the boundary: `#[repr(C)]` structs, fixed-width integers (`u32`, not `usize`), and C strings. Do not pass Rust-specific types (`Vec`, `String`, `Box`) directly across FFI unless you control both sides and understand the layout.
- Variadic functions and callbacks have additional ABI constraints; verify them per-platform.
- A C function that can unwind (calls back into Rust that panics) crossing the FFI boundary is undefined behavior. Use `catch_unwind` at the boundary or mark functions `extern "C"` with abort-on-panic, and never let a panic unwind into C.

### Use MaybeUninit Instead Of uninitialized Memory

`mem::uninitialized()` is deprecated because it is instant undefined behavior for most types. To handle genuinely uninitialized memory (e.g., an output buffer C will fill), use `MaybeUninit<T>`:

- Create `MaybeUninit::<T>::uninit()`, pass a pointer to it to C, and only call `assume_init()` after C has fully initialized it.
- Never read a `MaybeUninit` before it is initialized; never create a reference to uninitialized data.
- For arrays/buffers, `MaybeUninit<[T; N]>` or a `Vec<MaybeUninit<T>>` lets you fill memory then transmute to initialized.

This is the safe pattern for "C writes into a buffer I provide." Skipping it (using `mem::zeroed` and hoping, or raw `uninitialized`) is a soundness bug waiting to happen.

### Avoid transmute Unless You Understand The Layout

`std::mem::transmute` reinterprets the bits of one type as another. It is powerful and dangerous: it can transmute between types of different sizes, lifetimes, and representations, producing undefined behavior if the reinterpretation is invalid.

- Use `transmute` only when the two types have identical, guaranteed layouts and you can state why.
- Do not transmute references to extend or change lifetimes; this is soundness-critical and almost always wrong.
- Prefer typed alternatives: `From`/`Into`, `as` casts for primitives, `Box::from_raw`/`into_raw` for pointer conversion, `try_from` for checked conversion.
- `transmute` between a reference and a pointer, or between different pointer types, usually has a safer method.

## Common Traps

### Large undocumented unsafe Blocks

A block spanning many lines with no comment hides which operation needs unsafe and what invariant it relies on. Keep blocks tiny and comment the proof obligation on each.

### Exposing Raw Pointers Through A "Safe" Function

A function marked safe that returns a raw pointer or does not validate FFI preconditions is unsound: callers will misuse it and the compiler will not warn. The safe wrapper must enforce every precondition the unsafe core needs.

### Forgetting Ownership Across FFI

Freeing Rust memory from C (or C memory from Rust with the wrong allocator) corrupts the heap. Document and enforce who owns every pointer; copy data across the boundary when ownership is unclear.

### Reading Uninitialized Memory

Reading `MaybeUninit` before it is filled, or using deprecated `mem::uninitialized`, is undefined behavior. Always initialize before reading; use `assume_init` only after the producer guarantees initialization.

### transmute To Change Lifetimes

`transmute` to extend a reference's lifetime produces a dangling reference the compiler cannot detect. Never transmute lifetimes; restructure the code so lifetimes are correct by construction.

### Assuming C Functions Are Thread-Safe

A C library that uses global state or is not reentrant is not safe to call from multiple Rust threads, even though the Rust side compiles. Verify the C library's thread-safety guarantees; add synchronization or restrict to a single thread if C is not thread-safe.

### Letting A Panic Unwind Into C

A Rust panic unwinding across an FFI boundary into C is undefined behavior. Catch panics at the boundary (`catch_unwind`) or configure the function to abort on panic.

### Passing Rust Layout Structs To C

A struct without `#[repr(C)]` has an unspecified layout; C will read garbage fields. Always use `#[repr(C)]` (or the matching repr) for structs crossing FFI.

## Self-Check

- [ ] Every `unsafe` block is small, localized to the operations that need it, and carries a comment stating the invariant being upheld.
- [ ] Raw pointer operations are wrapped in a safe public API that validates preconditions (non-null, alignment, in-range, correct types) before the unsafe core runs.
- [ ] Ownership and memory responsibility at every FFI boundary is documented: who allocates, who uses, who frees, and with which allocator.
- [ ] Strings crossing FFI use `CString`/`CStr` (null-terminated, no interior nulls); Rust `String`/`&str` are not handed directly to C.
- [ ] Structs passed across FFI use `#[repr(C)]` and C-compatible types; Rust-specific types do not cross the boundary directly.
- [ ] Uninitialized memory uses `MaybeUninit` and is only read after `assume_init` once the producer guarantees initialization; deprecated `mem::uninitialized` is absent.
- [ ] `transmute` is used only between types with identical guaranteed layouts, never to change lifetimes; safer typed alternatives are preferred.
- [ ] The calling convention (`extern "C"` or platform-specific) is explicit on both sides of every FFI function.
- [ ] Panics do not unwind across the FFI boundary; `catch_unwind` or abort-on-panic guards the boundary.
- [ ] The C library's thread-safety guarantees were verified before calling it from multiple Rust threads; non-thread-safe C is serialized or single-threaded.
- [ ] A safe alternative (stdlib method, vetted crate) was preferred over hand-written unsafe wherever one existed.
