---
name: memory_safety_and_bounds_discipline.md
description: Use when the agent is writing or reviewing code in a memory-unsafe language (C, C++, Rust unsafe, Zig) or any context where out-of-bounds access, buffer overflow, use-after-free, double-free, null or dangling pointer dereference, or uninitialized memory can occur; building parsers, protocol handlers, or code that processes untrusted input byte-by-byte; reasoning about pointer validity, lifetime, and aliasing; choosing between bounds checks, safe abstractions, and unsafe code; or hardening code against memory-corruption exploits. Covers bounds checking, pointer and lifetime discipline, safe-by-construction patterns, the blast radius of memory-unsafe code on untrusted input, and the tradeoffs of unsafe code versus safe alternatives.
---

# Memory Safety And Bounds Discipline

Memory safety is the property that a program cannot access memory it should not — not past the end of a buffer, not through a freed pointer, not uninitialized, not through a misaligned or aliased reference. In a memory-safe language (most modern managed runtimes, and safe Rust), the language enforces this and the programmer rarely thinks about it. In a memory-unsafe language (C, C++, Rust's unsafe subset, assembly), the programmer is responsible, and the consequences of failure are uniquely severe: a buffer overflow is not just a crash, it is a potential arbitrary-code-execution vulnerability, because an attacker who controls the overflowed bytes can redirect control flow. The history of security is, in large part, the history of memory-safety bugs in code that processed untrusted input — and the defining property of these bugs is that they look like ordinary programming errors and produce exploitable vulnerabilities.

Agents tend to write memory-unsafe code as if it were memory-safe, trusting that indices are in range, that pointers are valid, that buffers are large enough, and that the input is well-formed. Each of these assumptions, when violated by malformed or hostile input, becomes a memory-safety bug. The judgment problem is recognizing that memory-unsafe code processing untrusted input is a security boundary, that every memory access is a potential vulnerability if its validity is not guaranteed by construction, and that the safe path — bounds checks, safe abstractions, choosing safe languages or subsets where possible — is not optional caution but the baseline discipline that prevents the most damaging class of bugs. This skill covers the discipline of writing memory-safe code: guaranteeing bounds and lifetime by construction, handling untrusted input defensively, and knowing when unsafe code is justified and when it is a liability.

## Core Rules

### Treat Memory-Unsafe Code On Untrusted Input As A Security Boundary

The first principle is to recognize where memory safety matters most. Code that processes attacker-controlled input — parsers, protocol decoders, file format readers, network handlers — is the highest-risk surface, because an attacker who controls the input controls the memory operations.

- **Identify code that processes untrusted input byte-by-byte or field-by-field.** This is the code where a memory-safety bug becomes an exploit. Treat it as a security boundary requiring the highest discipline: fuzzing, bounds checks, safe abstractions, and review.
- **Prefer memory-safe languages or safe subsets for untrusted-input processing.** Where feasible, parse untrusted input in Rust (safe subset), Go, Java, or a managed runtime, where the language guarantees memory safety. Reserve memory-unsafe languages for contexts where they are necessary and the input is trusted or the surface is minimal.
- **If unsafe code must process untrusted input, isolate and harden it.** Run it in a sandbox, fuzz it extensively, minimize its scope, and subject it to the strongest review. A parser in unsafe code on untrusted input is one of the most dangerous things you can write.

### Guarantee Bounds By Construction, Not By Assumption

Every indexed or pointer-based memory access must be provably in bounds. The safe way to guarantee this is by construction — using abstractions that cannot go out of bounds — rather than by manual index arithmetic that assumes correctness.

- **Use safe abstractions that enforce bounds: arrays/slices with bounds-checked access, span/string_view with bounds, containers that manage their own size.** A bounds-checked access either succeeds or traps cleanly; an unchecked access corrupts memory. Prefer the checked form unless profiling proves it is too costly (and even then, prefer a checked form in a hot loop over unchecked code broadly).
- **Validate sizes and lengths before allocating or copying.** A length field from untrusted input must be validated against sane bounds before it is used to size a buffer or a loop. An attacker-supplied length of 2^32 causing a huge allocation or an integer overflow in size computation is a classic exploit vector.
- **Beware integer overflow in size and index computation.** `a + b` used as a size or index can overflow and wrap to a small value, allocating a too-small buffer for data that then overflows it. Use checked arithmetic or saturating math for sizes derived from untrusted input.
- **Check bounds once, at the boundary, then use a safe view.** Validate the input's structure at the parse boundary; thereafter, pass a typed, bounds-safe representation so downstream code does not re-derive indices from raw bytes.

### Manage Pointer Lifetime And Ownership Explicitly

Use-after-free, double-free, and dangling-pointer bugs occur when code accesses memory through a pointer whose target has been freed or reallocated. Preventing them requires explicit ownership and lifetime discipline.

- **Establish clear ownership for every allocation.** Know who is responsible for freeing each allocation, and ensure it is freed exactly once. Ownership transfer (move semantics) or shared ownership (reference counting, RC) makes this explicit; ambiguous ownership leads to double-free or leak.
- **Do not retain pointers, references, or iterators to memory that may be freed or reallocated.** A pointer to an element of a vector is invalidated when the vector reallocates; an iterator to a map entry may be invalidated by insertion. Either do not retain such references across operations that may invalidate them, or use stable references (indices into a structure that does not relocate, or pointer-stable containers).
- **Prefer RAII and scope-based lifetime.** Tie resource lifetime to scope (RAII in C++ and Rust, defer in Go, try-with-resources in Java) so cleanup is automatic and cannot be forgotten. Manual new/delete or malloc/free is a source of leaks and double-frees.
- **In Rust, prefer safe code; justify every `unsafe`.** Rust's safe subset enforces ownership and lifetime statically. `unsafe` opts out of these checks and must be justified, minimal, and reviewed — it is where memory-safety bugs hide in otherwise-safe Rust.

### Initialize Memory Before Use

Reading uninitialized memory yields indeterminate values, which can cause unpredictable behavior, information leakage (reading stale data from a reused buffer), and undefined behavior in some languages.

- **Initialize all memory before reading it.** Zero buffers, default-construct objects, and ensure every field is set before use. Do not rely on memory being zeroed unless the language or allocator guarantees it.
- **Beware reusing buffers without clearing sensitive data.** A buffer reused across requests may leak data from a previous request if not cleared. For sensitive data, clear the buffer after use (and beware the compiler optimizing away a "dead" clear — use a function that the compiler cannot elide).
- **Do not branch on uninitialized data.** In some languages, branching on an uninitialized value is undefined behavior and can produce arbitrary results. Initialize before any use.

### Respect Aliasing And Alignment Rules

Memory in unsafe languages has rules about how it may be accessed: type-based aliasing (a given memory location has a type, and accessing it as a different type is undefined), and alignment (types have alignment requirements, and misaligned access is undefined or slow).

- **Do not violate strict aliasing.** Accessing memory of one type through a pointer of an unrelated type (e.g., reading an `int` through a `float*`) is undefined behavior in C/C++. Use `memcpy` for type punning, or compile with `-fno-strict-aliasing` if type punning is pervasive (with a performance cost).
- **Ensure alignment for types that require it.** Accessing a misaligned address as a type with alignment requirements is undefined behavior (crash on some architectures, slow on others). Ensure buffers intended for typed access are properly aligned; use alignment-aware allocation.
- **Use safe serialization/deserialization rather than type punning over byte buffers.** Casting a byte buffer to a struct ("struct overlay") is a common source of aliasing, alignment, and portability bugs. Parse fields explicitly from the bytes.

### Fuzz And Test The Untrusted-Input Boundary

Memory-safety bugs in parsers and protocol handlers are difficult to find by manual review or example-based tests, because they often require specific malformed inputs. Fuzzing is the most effective technique.

- **Fuzz all code that processes untrusted input.** Coverage-guided fuzzing (libFuzzer, AFL, OSS-Fuzz) generates malformed inputs that exercise edge cases a human would not think to test, and finds memory-safety bugs automatically. Integrate fuzzing into CI for parsers and protocol handlers.
- **Run under sanitizers during testing.** AddressSanitizer (ASan) detects out-of-bounds and use-after-free at runtime; MemorySanitizer (MSan) detects uninitialized reads; UndefinedBehaviorSanitizer (UBSan) detects undefined behavior. Run tests and fuzzing under sanitizers to catch bugs that do not crash visibly.
- **Test boundary conditions explicitly.** Empty input, maximum-length fields, lengths near buffer boundaries, truncated input, and integer-overflow-inducing sizes are the inputs that trigger memory-safety bugs. Test them deliberately.

## Common Traps

### Unchecked Indexed Or Pointer Access On Untrusted Input

Accessing an array index or dereferencing a pointer derived from untrusted input without a bounds or validity check, producing an out-of-bounds read/write or exploit. Validate bounds; use checked access.

### Integer Overflow In Size Computation

Computing a buffer size as `a + b` from untrusted values, overflowing to a small value, allocating a too-small buffer, and overflowing it on copy. Use checked or saturating arithmetic for sizes.

### Retaining References Across Reallocation

Holding a pointer, reference, or iterator to a container's element across an operation that reallocates the container, then using the invalidated reference. Do not retain such references; use stable indices or pointer-stable structures.

### Ambiguous Ownership Leading To Double-Free Or Leak

Memory freed by two code paths (double-free) or by none (leak) because ownership was unclear. Establish explicit ownership (single owner, move, or shared with reference counting).

### Reading Or Branching On Uninitialized Memory

Using a buffer or field before it is initialized, yielding indeterminate values or undefined behavior. Initialize all memory before use.

### Struct Overlay / Type Punning Over Byte Buffers

Casting a byte buffer to a struct and accessing fields, violating aliasing and alignment rules and producing undefined behavior. Parse fields explicitly; use memcpy for type punning if required.

### Unsafe Rust Used Without Justification

Using `unsafe` in Rust where safe code would suffice, introducing memory-safety risks the safe subset exists to prevent. Justify and minimize every `unsafe` block.

### No Fuzzing Or Sanitizers On Parser Code

Releasing a parser or protocol handler that has only been tested with well-formed inputs, leaving memory-safety bugs reachable by malformed input. Fuzz and run under sanitizers.

## Self-Check

- [ ] Code that processes untrusted input (parsers, protocol handlers, file readers, network decoders) is identified as a security boundary, implemented in a memory-safe language or safe subset where feasible, and isolated/hardened/fuzzed where unsafe code is unavoidable.
- [ ] Every indexed or pointer-based memory access is provably in bounds — via bounds-checked abstractions, validated sizes and lengths (with checked/saturating arithmetic to prevent overflow), and bounds checked once at the parse boundary then carried as a safe typed view.
- [ ] Ownership and lifetime are explicit for every allocation (single owner, move semantics, or shared ownership with reference counting), references/iterators are not retained across invalidating operations, and RAII/scope-based lifetime ensures cleanup cannot be forgotten.
- [ ] All memory is initialized before use, buffers are cleared before reuse to prevent sensitive-data leakage (with non-elidable clears), and no code branches on uninitialized data.
- [ ] Strict aliasing and alignment rules are respected — no type punning over byte buffers (parse fields explicitly or use memcpy), buffers for typed access are properly aligned — and struct overlays are avoided.
- [ ] In Rust, `unsafe` is used only where justified and minimal, and every `unsafe` block is reviewed as a potential memory-safety risk.
- [ ] Untrusted-input code is fuzzed (coverage-guided fuzzing in CI) and tested under sanitizers (ASan, MSan, UBSan), and boundary conditions (empty, max-length, truncated, overflow-inducing inputs) are tested explicitly.
- [ ] The memory-unsafe surface has been reviewed for the specific bug classes: out-of-bounds, use-after-free, double-free, uninitialized read, integer overflow in sizes, aliasing/alignment violations, and invalidated references — each addressed by construction rather than assumption.
