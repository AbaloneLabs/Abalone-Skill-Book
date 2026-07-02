---
name: c_pointer_and_memory_safety.md
description: Use when the agent is writing or reviewing C code involving pointer arithmetic, manual memory management with malloc/free, buffer handling, array indexing, aliasing, strict aliasing rules, pointer casts, ownership transfer, string manipulation, or interfacing with memory-mapped regions and external buffers where the language provides no automatic bounds or lifetime checks.
---

# Pointer And Memory Safety In C

C gives the programmer direct control over memory and expects that programmer to enforce every invariant the language does not. There is no borrow checker, no garbage collector, no runtime bounds check, and no ownership type. A pointer is just a number that happens to be dereferenceable today. The judgment problem is not "how do I make this compile" but "how do I keep this memory access valid for the entire time any alias can reach it," and that question has to be answered by design, not by hope.

Agents tend to treat C memory code as a syntax exercise: cast until the compiler stops complaining, index until the test passes, free at the end of the function. The harm is severe and delayed. A buffer overflow may not crash for years. A dangling pointer may work until the allocator reuses the block. A strict aliasing violation may run correctly on one compiler and miscompile on another after an optimization flag changes. "It works on my machine" is not evidence of correctness in C; it is, at best, evidence that the bug has not been triggered yet. This skill is about building the discipline of stating, for every pointer, who owns it, how long it lives, what may alias it, and what proves the access is in bounds.

## Core Rules

### Make Ownership Explicit At Every Boundary

C has no ownership type, so ownership must be encoded as a convention and stated in the API contract. For every pointer a function takes or returns, decide and document one of three ownership modes.

- **Borrowed (non-owning):** the caller owns the memory and guarantees it stays valid for the duration of the call. The callee must not free it and must not store it past the call unless the lifetime is otherwise guaranteed. This is the default for `const T *` parameters.
- **Transferred:** the callee takes ownership and becomes responsible for freeing it. The caller must not use or free it afterward. This is the contract of functions like `strdup` consumers or `free` itself.
- **Returned-owned:** the caller receives a new allocation and becomes responsible for freeing it with the matching deallocator.

Encode the contract in naming, documentation, and types where possible. A `const char *get_name(...)` almost always borrows; a `char *make_name(...)` almost always returns ownership. When ownership is ambiguous, the convention is to assume borrowed unless documented otherwise, which is the source of countless leaks and double-frees. State the contract in a header comment for every function that handles allocated memory.

Strong choice: `// Returns a newly malloc'd buffer; caller must free(). May return NULL on allocation failure.` Weak choice: a function returning `char *` with no comment, leaving every caller to guess.

### Never Trust A Length You Did Not Just Compute

Buffer overflows come from assuming a size. Every byte-level operation needs a proven, current bound. Decide where the bound comes from before writing the loop.

- Prefer functions that take an explicit size (`memcpy(dst, src, n)`, `snprintf(buf, sizeof(buf), ...)`) over functions that infer size from a terminator (`strcpy`, `sprintf`, `gets`).
- When a buffer is a fixed array, use `sizeof arr` only in the scope where the array has its real type. Once it decays to a pointer, `sizeof` gives the pointer size, not the buffer size — this is one of the most common and most damaging C mistakes.
- When a length arrives from an external source (a struct field, a network packet, a file header), validate it against the actual buffer before using it as a bound. A length field that claims a million bytes is an attack, not an optimization.
- Recompute or re-validate lengths after any operation that can change the contents (reallocation, truncation, concatenation).

The rule of thumb: if you cannot point to the exact line that established the bound you are about to use, you do not have a bound.

### Free Exactly Once, In Exactly One Place, With The Matching Deallocator

Memory management errors (double-free, use-after-free, mismatched allocators, leaks) are all consequences of unclear deallocation responsibility. Establish one owner and one deallocation path.

- Every allocation has exactly one matching deallocator: `malloc`/`free`, `calloc`/`free`, `strdup`/`free`, `fopen`/`fclose`, `asprintf`/`free`, a library-specific allocator with its own free function. Mixing them is undefined behavior.
- Free in the same module that allocated, or transfer ownership explicitly. A function that allocates should provide (or document) the matching free function.
- Set a pointer to `NULL` immediately after freeing if the variable remains in scope, so a stray later free is a harmless no-op rather than a double-free. This is defensive, not a substitute for correct ownership.
- On error paths, free everything allocated so far before returning. The "allocate, then fail halfway" path is where most leaks live; structure code so cleanup is centralized, often via a `goto cleanup` label.

The `goto cleanup` pattern is idiomatic and correct in C for this reason. Do not avoid it out of a habit borrowed from languages with exceptions; in C it is the structured way to guarantee cleanup on multiple error exits.

### Respect Strict Aliasing And Effective Type Rules

The C standard forbids accessing an object through a pointer of an incompatible type (the "strict aliasing" rule). Compilers rely on this rule to optimize, and violating it is undefined behavior that can silently produce wrong code.

- Do not read an `int` through an `int *` that actually points at a `float`, or a `long` through a pointer to a `struct` that happens to have the same layout. Types that are not compatible (or not a character type) must not alias.
- The safe escape hatches are: `memcpy` for type-punning (the compiler knows about it and handles it correctly), `char *` / `unsigned char *` / `uint8_t *` for byte-level access (character types may alias anything), and `union` reinterpretation within the rules of C99+.
- Pointer casts through `void *` do not cure an aliasing violation; they only hide the cast from a casual reader. The access is still UB if the underlying object's effective type does not match.
- Serialization, deserialization, and network code are the most common offenders because they reinterpret byte buffers as structured types. Prefer `memcpy` into a properly typed variable, or a packed-struct approach with documented assumptions about alignment and endianness.

When in doubt, `memcpy`. It is fast (compilers lower it to a single load/store when the size is constant and small) and it is never an aliasing violation.

### Align And Pad For The Target, Not For Your Machine

Misaligned access is undefined behavior on many architectures and a performance trap on others. Pointers obtained from `malloc` are suitably aligned for any standard type, but pointers you compute yourself may not be.

- Pointer arithmetic that produces a misaligned address (e.g., casting a `char *` buffer to a `uint32_t *` when the buffer start plus offset is not 4-byte aligned) is UB. The symptom ranges from a bus error on strict architectures to silent corruption on permissive ones.
- Packing structs for wire formats (`__attribute__((packed))`, `#pragma pack`) removes padding and can create misaligned fields; accessing such fields through a normal dereference is then UB on some platforms. Use `memcpy` to read packed fields safely.
- When building protocol parsers that read multi-byte integers from byte streams, use explicit byte-shift assembly (`x |= (uint32_t)buf[0] << 24;`) rather than pointer casts. This is correct regardless of endianness (once you handle byte order) and alignment.

### Treat NULL As A Valid, Expected Input

In C, dereferencing NULL is undefined behavior and a frequent crash source. But NULL is also a legitimate sentinel that callers will pass. Decide, per function, whether NULL is allowed and handle both cases.

- `malloc` returns NULL on failure; check it every time, even on systems with overcommit, because embedded and hardened environments do fail allocations.
- For pointer parameters, document whether NULL is permitted. If a function documents "must not be NULL," it may legitimately crash or assert on NULL; if it does not, it must handle NULL gracefully.
- Freeing NULL is defined to be a safe no-op, so you need not guard `free(ptr)` with a NULL check. (Guarding it is harmless but signals that the author was unsure of the contract.)

## Common Traps

### Using sizeof On A Pointer And Expecting The Buffer Size

`sizeof(ptr)` returns the size of the pointer (e.g., 8), never the size of the allocation it points to. This compiles, passes naive tests on small inputs, and silently truncates or overflows on real data. The fix is to carry the length alongside the pointer as a separate parameter or struct field, because C cannot recover the allocation size from a pointer.

### Returning A Pointer To A Stack Or Local Variable

A function that returns `&local` or a pointer into a local array returns memory that is invalidated the moment the function returns. The caller may see correct data until the stack frame is reused, then garbage or a crash. This is the classic dangling-pointer bug. Return by value, accept a caller-owned output buffer, or allocate and transfer ownership.

### The "It Works On My Machine" Rationalization For UB

Strict aliasing violations, signed overflow, and NULL dereferences often produce correct output on one compiler, one optimization level, or one architecture. That correctness is accidental and disappears under a different `-O` flag, a newer compiler, or a different CPU. Never treat observed behavior as proof that UB is safe. The standard, not the current binary, defines correctness.

### Forgetting The Null Terminator In String Buffers

C strings require a trailing `'\0'` that is not counted in `strlen`. A buffer of size N holds a string of at most N-1 characters. `snprintf(buf, sizeof(buf), ...)` handles this correctly; manual `memcpy` of a string length does not. Off-by-one here produces either a missing terminator (reads run past the buffer) or a one-byte overflow.

### Realloc Without Preserving The Old Pointer

`realloc(ptr, size)` may move the block and free the old one, and it may return NULL on failure while leaving the original block intact. The pattern `ptr = realloc(ptr, newsize)` leaks the old block and loses all data when realloc fails, because the original pointer is overwritten with NULL. Always assign to a temporary: `void *tmp = realloc(ptr, newsize); if (tmp) ptr = tmp;`.

### Assuming Free Zeroes Or Scrubs Memory

After `free`, the memory may be reused immediately by another allocation, and the old contents may still be readable (or may be overwritten with garbage). Do not read freed memory for any purpose. If secrets must be scrubbed, scrub them before freeing, and be aware that optimizers may remove a scrub that has no observable effect (use `memset_s` or an explicit barrier).

### Confusing Borrowed And Owned Across An API

A library that returns a `const char *` from an internal table is lending a pointer that may be invalidated by the next call to the library. Storing that pointer for later use is a use-after-free waiting to happen. Copy (`strdup`) any borrowed string whose lifetime you do not control before relying on it across calls.

## Self-Check

- [ ] Every pointer parameter and return value has a documented ownership mode (borrowed, transferred, or returned-owned), and the deallocator is named where ownership is transferred.
- [ ] No buffer operation relies on `sizeof` applied to a pointer; every byte-level loop has a bound that was just computed or just validated against the actual buffer.
- [ ] Each allocation has exactly one matching deallocator, error paths free partial allocations via a centralized cleanup label, and freed pointers are nulled if they remain in scope.
- [ ] No type-punning through incompatible pointer types remains; reinterpretation uses `memcpy`, character-type pointers, or a documented union, and packed-struct fields are read via `memcpy`.
- [ ] No pointer cast assumes alignment that the source cannot guarantee; protocol parsing uses byte-shift assembly or `memcpy` rather than casting byte buffers to wider types.
- [ ] `malloc`/`calloc`/`realloc` return values are checked for NULL before use, and realloc uses a temporary variable so failure does not destroy the original pointer.
- [ ] No function returns a pointer to a local or stack variable; outputs are returned by value, written to a caller-owned buffer, or freshly allocated with documented ownership.
- [ ] String buffers account for the null terminator (capacity N holds N-1 characters), and `snprintf`/`strncpy`-with-explicit-termination are used instead of unbounded `strcpy`/`sprintf`.
- [ ] No freed memory is read for any purpose, and secrets are scrubbed with a non-optimizable call before being freed.
- [ ] Any "it works" claim about an aliasing, overflow, or NULL-deref pattern has been re-evaluated against the C standard rather than against observed binary behavior.
