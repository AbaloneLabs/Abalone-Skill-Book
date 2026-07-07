---
name: csharp_pinvoke_interop_and_unsafe.md
description: Use when the agent is writing or reviewing C# code that calls native libraries via P/Invoke and DllImport, using unsafe blocks and pointers, fixed and ref struct, Span over native memory, Marshal and marshalling attributes, NativeLibrary resolution, function pointers (delegate*), COM interop, or diagnosing calling-convention mismatches, marshalling overhead, memory ownership across the managed/native boundary, AccessViolationException, or library-not-found errors across platforms.
---

# C# P/Invoke, Interop, And Unsafe

C# can call into native (C/C++) libraries via P/Invoke (`[DllImport]`), use raw pointers in `unsafe` blocks, marshal data across the managed/native boundary, and interoperate with COM. This is powerful — it is how .NET talks to the operating system, to native libraries, and to performance-critical code — but every crossing of the boundary is a place where the runtime's safety guarantees end. A wrong calling convention corrupts the stack, a wrong marshalling rule doubles memory or leaks, a pointer freed by the wrong side corrupts the heap, and an `AccessViolationException` from native code cannot be caught reliably. The judgment problem is that interop is a contract about calling convention, data layout, and ownership, and the C# compiler checks almost none of it.

Agents tend to write `[DllImport]` without specifying the calling convention, to over-marshal (blittable structs copied directly are faster than marshalled ones), to leak memory by not freeing native allocations, or to use `unsafe` where `Span<T>` and `ref` would do. The judgment problem is to declare P/Invoke signatures exactly (calling convention, charset, marshalling), to prefer blittable and span-based interop for performance, to make ownership across the boundary explicit, and to use `unsafe` only where the safe alternatives cannot express the operation. This skill is about treating the managed/native boundary as a precise contract, not a convenience.

## Core Rules

### Declare P/Invoke Signatures Exactly: Calling Convention, Charset, Marshalling

A `[DllImport]` signature must match the native function exactly in calling convention, character set, and parameter marshalling, or the call corrupts state or crashes:

- **Calling convention**: specify `CallingConvention.Cdecl` (or whatever the native function uses) explicitly. The default (`Winapi`, which maps to `Stdcall` on Windows) is wrong for most C functions, which are `Cdecl`. A mismatch corrupts the stack (on x86) or silently misbehaves.
- **CharSet**: set `CharSet = CharSet.Unicode` (or `CharSet.Ansi`) to match how the native function expects strings. Marshalling a C# `string` to a native `char*` allocates and copies; for one-shot calls this is fine, for hot paths pass `ReadOnlySpan<char>` or a pinned pointer.
- **Marshalling**: by default, blittable types (primitives, structs of blittable fields) are pinned and passed directly (fast). Non-blittable types (strings, arrays of classes, `bool`) are marshalled (copied/converted), which allocates. Design interop structs to be blittable for performance.

Use `[LibraryImport]` (C# 11+, source-generated) instead of `[DllImport]` where available: it generates marshalling code at compile time (faster, AOT-friendly, and avoids the runtime reflection-based marshalling). For new code, prefer `[LibraryImport]`.

### Make Blittable Structs For Direct Interop

A blittable type has the same layout in managed and native memory (primitives, and structs containing only blittable fields). Blittable structs are pinned and passed by pointer with no copying or conversion — the fast path. Non-blittable types (containing `bool`, `string`, `object`, arrays of non-blittable) require marshalling that allocates and copies.

For interop, design structs to be blittable: use primitives and fixed-size buffers, avoid `bool` (use `byte` or `int` and convert), avoid embedded strings and arrays (use pointers). Match the native layout with `[StructLayout(LayoutKind.Sequential)]` (or `Explicit` with field offsets for unions), and assert the size with `Marshal.SizeOf` in tests. Blittable interop is dramatically faster than marshalled interop in hot paths.

### Pin Managed Memory For Native To Access, And Unpin Promptly

When native code must read/write a managed array or struct, pin it with `fixed` (for short-lived, synchronous access) or `GCHandle.Alloc(..., GCHandleType.Pinned)` (for longer-lived access). Pinning prevents the GC from moving the object, giving native code a stable pointer. Pin for the shortest time possible, because pinned objects fragment the GC heap.

- `fixed (byte* p = array) { native_call(p); }` pins for the duration of the block — use for synchronous native calls.
- `GCHandle` pinned is for async or long-lived scenarios; free it (`handle.Free()`) as soon as native code is done, in a `finally`.

Never pass a managed pointer to native code that stores it beyond the call (the GC may move or collect the object after unpinning). If native must hold the memory, allocate native memory (`Marshal.AllocHGlobal`, `NativeMemory.Alloc`) and copy, or use a pinned handle with a documented lifetime.

### Make Ownership Across The Boundary Explicit

The most common interop bug is a memory-ownership mismatch: native allocates and returns a pointer, and managed does not know whether (or how) to free it; or managed allocates and passes it in, and native frees it with the wrong allocator. Document, for every native pointer parameter and return value, who owns it and which allocator applies.

- If native allocates and returns memory, provide (or call) a matching free function (`native_free(ptr)`), and free it from managed. Do not free native memory with `Marshal.FreeHGlobal` unless it was allocated with `Marshal.AllocHGlobal` — allocators are not interchangeable.
- For managed-to-native, prefer passing pinned managed memory (borrow) for synchronous calls, or a native-allocated buffer that managed fills and native owns.
- Use `SafeHandle` to wrap native handles/resources so they are freed reliably (even on exception) by the runtime's finalizer and explicit `Dispose`.

### Use Span<T> And ref Instead Of unsafe Where Possible

Modern C# offers safe alternatives to raw pointers for many scenarios:

- `Span<T>`/`ReadOnlySpan<T>` over native memory (`new Span<byte>(nativePtr, length)`) lets you process native buffers with bounds-checked, safe code.
- `ref T` (and `ref readonly`) passes a managed reference without pointers.
- `fixed`-size buffers and `stackalloc` for small temporary native-style buffers.

Prefer these over `unsafe` pointers where they express the operation; reserve `unsafe` for cases where the safe abstractions cannot (e.g., pointer arithmetic on native memory, function pointers, or performance-critical code the safe abstractions do not optimize). `unsafe` disables the runtime's bounds and type checks, so it must be correct by construction.

### Use Function Pointers (delegate*) For Native Callbacks; Be Careful With Delegate Marshalling

`delegate*<T1, T2, TResult>` (C# 9 function pointers, `unmanaged` calling convention) is the fast, allocation-free way to call native function pointers and to pass managed callbacks to native code. For passing a managed callback to native, a function pointer to a `static` `[UnmanagedCallersOnly]` method avoids the delegate-marshalling overhead and lifetime issues of `Marshal.GetFunctionPointerForDelegate`.

If you must use a managed delegate for a native callback (`Marshal.GetFunctionPointerForDelegate`), the delegate must be kept alive (held by a managed reference) for as long as native might call it — the runtime does not know native holds the function pointer, so it can collect the delegate, and the next native call crashes. Pin the delegate in a field for the duration. Prefer `[UnmanagedCallersOnly]` static methods and function pointers in modern code.

### Resolve Native Libraries Portably

`[DllImport("mylib")]` searches the native library by name with platform-specific rules (`mylib.dll` on Windows, `libmylib.so` on Linux, `libmylib.dylib` on macOS). Use `NativeLibrary.SetDllImportResolver` or the `.NET` default resolution (which respects `runtimes/<rid>/native/` in a NuGet package) to load the right library per platform. Package native libraries in a NuGet package's `runtimes/<rid>/native/` folder so they are extracted for the right RID at runtime. Diagnose "library not found" by checking the search path and the RID.

## Common Traps

### Wrong Calling Convention

A `[DllImport]` defaulting to `Stdcall` for a `Cdecl` native function corrupts the stack (x86) or misbehaves. Specify `CallingConvention` explicitly.

### Over-Marshalling Non-Blittable Structs

A struct with `bool` or embedded `string` is non-blittable and marshalled (copied) on every call. Use blittable fields (`byte` for bool, pointers for strings) for hot-path interop.

### Native Memory Freed With The Wrong Allocator

Freeing native `malloc` memory with `Marshal.FreeHGlobal` (or vice versa) corrupts the heap. Match the allocator; provide a matching free function.

### Delegate For Native Callback Collected

A managed delegate passed to native via `GetFunctionPointerForDelegate` that is not held alive can be GC'd; the next native call crashes. Hold the delegate in a field, or use `[UnmanagedCallersOnly]` + function pointer.

### Pinning Too Long, Fragmenting The GC

Pinning many objects or pinning for long periods fragments the managed heap. Pin for the shortest time (synchronous `fixed` block), and free `GCHandle` promptly.

### Native Storing A Managed Pointer Past Unpin

Native code that stores a managed pointer after the `fixed` block ends reads moved/freed memory. Copy to native memory if native must hold it.

### AccessViolationException That Cannot Be Caught Reliably

An access violation from native code (null deref, bad pointer) may surface as `AccessViolationException`, but catching it is unreliable (the process may be corrupted). The fix is to make the native code correct, not to catch the exception.

### Library Not Found Across Platforms

`[DllImport("mylib")]` fails if the per-platform naming (`libmylib.so` vs `mylib.dll`) or the search path is wrong. Use `NativeLibrary.SetDllImportResolver` and package native libs in `runtimes/<rid>/native/`.

## Self-Check

- [ ] P/Invoke signatures specify the calling convention, charset, and marshalling explicitly to match the native function; `[LibraryImport]` (source-generated) is preferred for new code.
- [ ] Interop structs are blittable (primitives, fixed buffers, no `bool`/`string`/embedded arrays) and use `[StructLayout(Sequential)]` or `Explicit` to match native layout, with sizes asserted in tests.
- [ ] Managed memory accessed by native is pinned (`fixed` for synchronous, `GCHandle` for longer-lived) for the shortest time possible, and native never stores a managed pointer past unpinning.
- [ ] Ownership of every native pointer (parameter and return) is documented, with matching allocator/free pairs; native memory is freed via the correct function or `SafeHandle`, never with a mismatched allocator.
- [ ] `Span<T>`, `ref`, and `stackalloc` are preferred over `unsafe` pointers where they express the operation; `unsafe` is reserved for cases the safe abstractions cannot handle.
- [ ] Native callbacks use `[UnmanagedCallersOnly]` static methods and `delegate*` function pointers where possible; managed delegates passed to native are held alive for the callback's lifetime.
- [ ] Native libraries are resolved portably (`NativeLibrary.SetDllImportResolver`, packaged in `runtimes/<rid>/native/`) so the right library loads per platform.
- [ ] No code relies on catching `AccessViolationException` to handle native bugs; native code is made correct, and `SafeHandle` ensures resources are freed even on exception.
