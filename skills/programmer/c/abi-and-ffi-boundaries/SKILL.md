---
name: c_abi_and_ffi_boundaries.md
description: Use when the agent is writing or reviewing C code that defines an ABI or is called from another language (Python ctypes/cffi, Rust FFI, Java JNI, Go cgo, C# P/Invoke), or when writing C headers meant for FFI, designing stable struct layouts, controlling symbol visibility, handling calling conventions, callback registration, cross-language ownership transfer, or diagnosing ABI drift, struct padding mismatches, and segfaults at language boundaries.
---

# C ABI And FFI Boundaries

C is the lingua franca of foreign function interfaces. Almost every language with a native-call capability (Python ctypes, Rust `extern "C"`, Java JNI, Go cgo, C# P/Invoke, Swift) calls into C because the C ABI is the one stable, documented binary interface that every platform defines. But "C ABI" is not one thing: it is per-platform (System V AMD64, Windows x64, ARM AAPCS, etc.), it depends on struct layout and padding rules, calling conventions, and symbol naming, and it breaks silently when a struct field is reordered or a type width changes. The judgment problem is that a C header that compiles fine is not a stable ABI; a stable ABI is a contract about layout, visibility, and ownership that must be designed and versioned deliberately.

Agents tend to expose internal C types directly across an FFI boundary, then discover that adding a field breaks every downstream caller, that a struct's padding differs across compilers, that a callback receives the wrong calling convention, or that ownership of a returned pointer is ambiguous and one side frees memory the other is still using. The judgment problem is to design the boundary as an opaque-handle API, to fix struct layouts with explicit padding or pack them deliberately, to document ownership of every pointer crossing the boundary, and to version the ABI so changes do not silently corrupt callers. This skill is about treating the FFI boundary as a published contract, not a convenience.

## Core Rules

### Design The Boundary As Opaque Handles, Not Exposed Structs

The safest FFI API exposes only opaque handles (a `typedef struct Foo Foo;` where the struct body is not in the public header) and functions that operate on `Foo*`. Callers never see the struct layout, so you can add, remove, or reorder fields without breaking the ABI. This is the pattern used by every long-lived C library (FILE*, sqlite3*, SDL_Window*).

If you must expose a struct, treat its layout as frozen: every field, its type, its order, and the padding are part of the ABI and cannot change without a version bump. Prefer opaque handles; expose structs only when callers must allocate them inline (rare) or when the layout is a wire format.

### Fix Struct Layouts Deliberately With Padding And Alignment

C compilers insert padding to align fields to their natural alignment, and the rules differ subtly across compilers and platforms. A struct that is 16 bytes on one compiler may be 24 on another because of alignment differences. For any struct that crosses an ABI or wire boundary, fix the layout explicitly:

- Order fields to minimize padding naturally (largest alignment first), or
- Insert explicit reserved/padding fields to control the layout, or
- Use `#pragma pack(n)` / `__attribute__((packed))` to disable padding — but packed structs are slower and may fault on architectures that require aligned access (some ARM).

Then assert the layout at compile time: `_Static_assert(sizeof(struct Header) == 16, "");` and `_Static_assert(offsetof(struct Header, flags) == 8, "");`. These assertions catch any future field change that would break the ABI. Never assume the compiler's default layout matches another compiler's.

### Control Symbol Visibility And Naming

By default, every non-`static` function and global in a C translation unit is exported with a platform-dependent symbol name (usually the C name, possibly with a leading underscore on macOS). For a library, decide which symbols are the public API and which are internal. Use `static` for file-local symbols, and use visibility attributes (`__attribute__((visibility("hidden")))` on ELF, `__declspec(dllexport)`/`__dllimport` on Windows) to control what is exported.

On Windows, decide between `__cdecl` (default, caller cleans stack) and `__stdcall` (callee cleans, used by Win32) calling conventions; they are not compatible and a mismatch corrupts the stack. On Unix the calling convention is fixed by the platform ABI. For maximum portability, use a macro that maps to the right convention per platform.

### Document Ownership Of Every Pointer Crossing The Boundary

The single most common FFI bug is a memory-ownership mismatch: C allocates and returns a pointer, the foreign caller does not know whether to free it (and with which allocator), or the foreign side allocates and passes it in, and C frees it with the wrong allocator. The result is a double-free, a use-after-free, or a heap corruption from mixing allocators (e.g., freeing a libc `malloc` block with a foreign runtime's free).

Document, for every pointer parameter and return value, who owns it and who is responsible for freeing it, and which allocator applies. The robust pattern is: C allocates and C frees — provide a `foo_free(Foo*)` function, and the foreign side never calls `free` directly. For buffers passed in, document whether C may hold the pointer beyond the call (borrow vs copy). Never let the foreign side free C memory except through a provided C function, because allocators are not interchangeable across the boundary.

### Make Callbacks Explicit About Calling Convention And Thread Context

When C calls back into the foreign language (event handlers, iteration callbacks, qsort comparators), the callback's calling convention must match what C expects (usually `extern "C"` / `__cdecl`), and the callback must be reentrancy- and thread-safe if C may call it from an unexpected thread or recursively. Document whether callbacks may be called from threads other than the one that registered them, and whether they may be called reentrantly.

The callback signature should include a `void *user_data` parameter so the foreign side can pass context (C has no closures). Document whether C stores the callback/user_data beyond the call (for async events) and for how long the pointers must remain valid.

### Version The ABI For Evolvability

A published C ABI should be versioned so callers can detect the version and changes do not silently corrupt them. Common patterns: a version integer returned by an init function, a version field in a struct passed in by the caller (with the C side filling/validating it), or separate symbol versions (ELF symbol versioning). When you must change the layout, add a new function (`foo_new_v2`) rather than changing `foo_new` in place, so old callers keep working.

Avoid removing fields or changing field types in an exposed struct; prefer adding new fields at the end (with a size field so old callers ignore them) or a new versioned struct.

## Common Traps

### Exposing An Internal Struct Across The Boundary

`typedef struct { int x; char *name; } Config;` in the public header, then adding a field, breaks every caller that allocated `Config` by size. Use an opaque handle, or freeze and version the layout.

### Struct Padding Mismatch Across Compilers

A struct laid out by GCC assumed to match one laid out by MSVC may differ in padding, so the foreign side reads fields at the wrong offsets. Fix layout with explicit padding and assert `sizeof`/`offsetof`.

### Freeing C Memory With The Wrong Allocator

The foreign side calls its own `free` on a pointer C allocated with `malloc`, or vice versa. Allocators are not interchangeable. Provide a `foo_free` and require the foreign side to use it.

### Calling Convention Mismatch On Windows

Declaring a callback as `__cdecl` when the library expects `__stdcall` (or vice versa) corrupts the stack. Match the convention explicitly with a per-platform macro.

### Holding A Borrowed Pointer Past The Caller's Lifetime

C stores a pointer the foreign side passed in, but the foreign side frees or moves the underlying memory before C is done. Document borrow vs copy; copy if C needs the data beyond the call.

### Adding A Field To An Exposed Struct Without Versioning

Adding `int new_field;` to a public struct changes its size and offsets, silently corrupting callers compiled against the old header. Add at the end with a size check, or version the struct.

### Symbol Name Differences Across Platforms

A symbol exported as `foo` on Linux may be `_foo` on macOS (older) or mangled differently. Use `extern "C"` (or the C compiler) and visibility controls, and do not rely on symbol-name quirks.

### Callback Called From An Unexpected Thread

C calls the foreign callback from a worker thread the foreign runtime did not expect, causing a crash in a runtime that is not thread-safe (e.g., Python without the GIL held). Document the threading context of callbacks.

## Self-Check

- [ ] The FFI boundary exposes opaque handles (`typedef struct Foo Foo;` with the body private) rather than internal struct layouts, except where inline allocation is genuinely required.
- [ ] Any struct crossing the boundary has its layout fixed deliberately (field order, explicit padding) and asserted with `_Static_assert(sizeof(...) == N)` and `offsetof` checks.
- [ ] Symbol visibility is controlled (public API exported, internals `static` or hidden), and calling conventions are explicit and per-platform on Windows.
- [ ] Every pointer parameter and return value has documented ownership (who allocates, who frees, which allocator), and the foreign side frees C memory only through provided `foo_free` functions.
- [ ] Callbacks declare an explicit calling convention, include a `void *user_data` for context, and document whether they may be called from other threads or reentrantly.
- [ ] The ABI is versioned (version query, size-checked structs, or versioned entry points) so layout changes do not silently corrupt existing callers.
- [ ] No exposed struct has had a field added, removed, or retyped without a version bump or a size-checked compatibility mechanism.
