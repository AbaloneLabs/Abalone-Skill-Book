---
name: haskell_ffi_and_c_interop.md
description: Use when the agent is using Haskell's Foreign Function Interface (FFI) to call C from Haskell or expose Haskell to C (foreign import/export/ccall/capi, Ptr/StablePtr/ForeignPtr, marshalling with alloca/with/malloc/free, CStruct/Struct, calling C libraries like libgit2/openssl/sqlite), managing foreign memory and finalizers, handling C callbacks into Haskell, dealing with threading (bound threads for FFI calls), or is diagnosing "segfault in FFI call", "memory leak / use-after-free across the FFI", "FFI call blocks the RTS / deadlocks", "StablePtr becomes invalid", or marshalling/performance issues. Covers FFI imports/exports, memory marshalling (Ptr/ForeignPtr/StablePtr), finalizers and resource management, bound threads and the RTS, C callbacks, and the traps of segfaults, leaks, blocking, and unsafe calls.
---

# FFI And C Interop In Haskell

Haskell's FFI lets you call C from Haskell (`foreign import ccall`) and expose Haskell to C (`foreign export ccall`, `foreign import ccall "wrapper"` for callbacks), bridging to native libraries (openssl, sqlite, libgit2). But the FFI is a sharp tool: a wrong type signature, an unmanaged `Ptr`, or a blocking call in an `unsafe` FFI import segfaults, leaks, or deadlocks the runtime. Agents forget to free foreign memory (leak), use a raw `Ptr` after free (use-after-free/segfault), call a blocking C function via `unsafe` (blocks the whole RTS thread), pass a Haskell callback to C without a `StablePtr` (the GC moves/frees the Haskell object), or ignore that C callbacks into Haskell require a bound thread. The judgment problem is to marshal memory safely (`Ptr`/`ForeignPtr`/`StablePtr` with finalizers), to choose `safe` vs `unsafe` calls by blocking behavior, to manage Haskell-object lifetimes across C, and to handle callbacks and threading correctly.

Agents segfault, leak, block the RTS, or lose Haskell objects to the GC across the FFI. The remedy is disciplined marshalling, `safe` calls for blocking C, `ForeignPtr`/`StablePtr` with finalizers, and bound threads for callbacks.

## Core Rules

### Marshal Memory Safely (Ptr / ForeignPtr / StablePtr With Finalizers)

FFI passes pointers (`Ptr a`) to/from C. A raw `Ptr` is unmanaged — you must `malloc`/`free` it (use `alloca`/`with` for scoped allocation that frees automatically, or `ForeignPtr` with a finalizer for GC-managed lifetime). Rules:

- `alloca`/`with` for short-lived scratch memory (allocated, used in C, freed when the block exits) — prefer these to manual `malloc`/`free`.
- `ForeignPtr` with `addForeignPtrFinalizer` for memory that should live as long as the Haskell object (the finalizer frees the C memory when the GC collects the `ForeignPtr`).
- `StablePtr` to pass a *Haskell* object to C (C holds a stable pointer that the GC won't move/free); `deRefStablePtr` to get it back, `freeStablePtr` when done (or a finalizer).
- Never use a `Ptr` after it's freed (use-after-free → segfault); never let a `Ptr` outlive its allocation without a `ForeignPtr`/finalizer (leak).

- `alloca`/`with` for scoped scratch memory (auto-freed); `ForeignPtr`+finalizer for GC-managed lifetime.
- `StablePtr` to hand Haskell objects to C; `deRefStablePtr`/`freeStablePtr` to manage.
- No use-after-free, no leaks from unmanaged `Ptr`s.

### Choose safe vs unsafe Calls By Blocking Behavior

`foreign import ccall unsafe` is fast (the C call runs without releasing the Haskell thread) but if the C function *blocks* (waits on I/O, sleeps, takes a lock), it blocks the entire RTS capability (no other Haskell thread on that capability runs) — the runtime stalls. `foreign import ccall safe` (or just `ccall`, the default) allows the RTS to run other Haskell threads while the C call blocks, at higher per-call overhead. Rule: `unsafe` only for *non-blocking*, *fast* C functions (a math op, a quick lookup); `safe` for anything that may block or run long (I/O, locks, long computation). Default to `safe` if unsure; profile to justify `unsafe`.

- `unsafe`: fast, but a blocking C call stalls the whole RTS capability. Only for non-blocking, fast C.
- `safe` (default `ccal`): other Haskell threads run while C blocks; higher overhead.
- Default `safe` if unsure; use `unsafe` only for provably fast/non-blocking C, profile-justified.

### Manage Haskell-Object Lifetimes Across C (StablePtr For Callbacks)

When C holds a reference to a Haskell object (a callback's closure, a registered handler), the GC may move or free it unless you pin it with a `StablePtr`. Create `sp <- newStablePtr haskellValue`, pass `sp` to C (as a `Ptr`/`void*`), and in the callback `deRefStablePtr sp` to recover the Haskell value. `freeStablePtr sp` when C no longer needs it (or the object leaks — the GC won't collect it while a `StablePtr` exists). For callbacks specifically, `foreign import ccall "wrapper"` creates a C-callable function pointer from a Haskell function (often wrapping a `StablePtr` to the closure's data). Ensure the `StablePtr`/wrapper is freed when unregistered.

- `StablePtr` to pin a Haskell object held by C; `deRefStablePtr` to recover, `freeStablePtr` to release.
- `foreign import ccall "wrapper"` to make a C-callable function pointer from a Haskell function.
- Free `StablePtr`s/wrappers when unregistered, or the object leaks (GC won't collect while pinned).

### Use Bound Threads For C Callbacks Into Haskell

A C callback that calls back into Haskell (`foreign export ccall`, or a `wrapper` callback) must run in a *bound thread* (a Haskell thread tied to an OS thread) if it uses thread-local state or makes further FFI calls — otherwise the callback may run on an arbitrary RTS thread and deadlock or behave wrongly. Use `forkOS` (bound thread) and `runInBoundThread` for code that interacts with C callbacks. The main thread is bound. For a C library that calls back from its own threads, ensure those threads are registered with the RTS (`hs_add_root`/thread registration in the C init). Mismanaging this causes deadlocks or segfaults in callback-heavy code.

- C callbacks into Haskell need a bound thread (`forkOS`/`runInBoundThread`) if they use TLS or further FFI.
- Register C-created threads with the RTS (`hs_add_root`/thread registration) before they call back.
- Mismanagement → deadlocks/segfaults in callback-heavy FFI.

### Wrap FFI Calls In A Safe, Idiomatic Haskell API

Raw FFI (`Ptr`, `CInt`, manual marshalling) is error-prone and un-Haskell-like. Wrap it in an idiomatic API: a `newtype`/record for the C resource (a `Database` wrapping a `Ptr`), `bracket` for acquire/release (open/close), `ForeignPtr`+finalizer for automatic cleanup, and `IO`-returning functions that marshal to/from Haskell types (`Text`/`ByteString`, `Int`, `Maybe`). Hide the `Ptr`s behind the abstraction; expose only Haskell types. Use `Data.ByteString` for binary, `Text` for strings (marshal via `CString`/`CStringLen` with `peekCString`/`withCString`). This makes the FFI safe (no leaked `Ptr`s) and pleasant to use.

- Wrap FFI in a `newtype`/record with `bracket` acquire/release and `ForeignPtr`+finalizer.
- Expose only Haskell types (`Text`/`ByteString`/`Int`/`Maybe`); hide raw `Ptr`s.
- Marshal strings via `CString`/`CStringLen` (`withCString`/`peekCString`).

## Common Traps

### Use-After-Free Across The FFI

A `Ptr` used after `free` segfaults. Use `alloca`/`with`/`ForeignPtr` for scoped/GC-managed lifetime.

### Leaked Ptr / StablePtr

Unfreed foreign memory or `StablePtr` leaks. Free in `bracket`/finalizers; `freeStablePtr` when done.

### unsafe Call That Blocks

A blocking C function via `unsafe` stalls the RTS. Use `safe` for blocking/long C calls.

### Haskell Object GC'd While C Holds It

C holds a reference to a moved/freed Haskell object. Pin with `StablePtr`.

### Callback Deadlock (No Bound Thread)

A C callback into Haskell on an unbound thread deadlocks. Use `forkOS`/`runInBoundThread`.

### Wrong Type Signature

A mismatched FFI signature (wrong `CInt`/`Ptr`/size) corrupts memory. Match C exactly.

### Forgetting To Register C Threads

A C-created thread calling back without RTS registration segfaults. Register in C init.

### Exposing Raw Ptrs In The API

Leaking `Ptr` to users is error-prone. Wrap in a `newtype` with idiomatic Haskell types.

## Self-Check

- [ ] Foreign memory uses `alloca`/`with` (scoped, auto-freed) or `ForeignPtr`+finalizer (GC-managed); no raw `Ptr` outlives its allocation (no leaks, no use-after-free).
- [ ] `unsafe` FFI imports are used only for provably fast/non-blocking C; blocking/long C calls use `safe` (default `ccall`); the choice is profile-justified.
- [ ] Haskell objects held by C are pinned with `StablePtr` (`deRefStablePtr`/`freeStablePtr`); callbacks use `foreign import ccall "wrapper"` and free their `StablePtr`/wrapper when unregistered.
- [ ] C callbacks into Haskell run in bound threads (`forkOS`/`runInBoundThread`); C-created threads are registered with the RTS before calling back.
- [ ] The FFI is wrapped in an idiomatic Haskell API (`newtype`/record, `bracket` acquire/release, Haskell types exposed, raw `Ptr`s hidden).
- [ ] FFI signatures match the C exactly (types, `Ptr`/`CInt`/sizes); marshalling uses `ByteString`/`Text`/`CString` correctly.
- [ ] Resource cleanup (close/free/unregister) is guaranteed via `bracket`/finalizers even on exceptions.
- [ ] The FFI code has been considered under segfault, leak, RTS blocking, GC-vs-C lifetime, callback threading, and signature-correctness scenarios, and remains correct.
