---
name: memory_and_corruption_debugging.md
description: Use when the agent is debugging memory corruption, heap corruption, use-after-free, double free, stack overflow or buffer overflow, choosing between ASan MSan UBSan and Valgrind, interpreting corruption symptoms that crash far from the cause, or selecting the right memory debugging tool for native or unsafe code.
---

# Memory and Corruption Debugging

Memory corruption is the most deceptive class of bug. The symptom—a crash in an unrelated function, a value that changes for no reason, a hang, or a wrong result—bears no obvious relationship to the cause, because the corruption happens silently and manifests only when the corrupted memory is later used. A use-after-free in module A corrupts an object that crashes module Z ten seconds later. The crash site is never the bug; it is merely where the corruption became visible. This distance between cause and symptom is what makes corruption bugs so hard, and why the right tools (sanitizers that catch the corruption at the moment it occurs) are so much more effective than the wrong ones (debuggers that show you the crash site long after the cause).

The judgment problem is recognizing the signature of corruption, choosing the tool that catches the error at its source rather than at its symptom, and interpreting the tool's output to find the actual defect. The agent should not iterate on the crash site; it should use sanitizers to move the diagnosis back to the corruption.

This skill applies whenever you are debugging a crash, hang, or wrong-result bug in native, unsafe, or manually memory-managed code (C, C++, Rust unsafe, GC languages interacting with native code, or manual allocators).

## Core Rules

### Recognize the signature of memory corruption

Suspect corruption when a bug shows any of these signs:

- The crash site is in unrelated, stable code that has not changed, and the stack trace looks nonsensical or points at innocent functions.
- A value changes without any code writing to it, or an object's fields contain garbage.
- The bug is intermittent and sensitive to timing, load, build configuration, or memory layout (classic heisenbug signature).
- The program crashes, hangs, or produces wrong output depending on unrelated changes elsewhere.
- A crash "moves" when you add logging or change unrelated code, because the memory layout shifts.
- Valgrind or a sanitizer reports errors even when the program appears to work.

When you see these signs, stop debugging the crash site and start hunting corruption with the right tools.

### Use sanitizers to catch corruption at the source

Sanitizers instrument the program to detect the error at the moment it occurs, not at the later crash. They are the primary tool for corruption debugging:

- **AddressSanitizer (ASan)**: detects out-of-bounds access (heap, stack, global), use-after-free, double-free, and use-after-return. It adds redzones around allocations and quarantines freed memory so accesses are caught immediately. High overhead (~2x memory, ~2x speed) but extremely effective.
- **MemorySanitizer (MSan)**: detects use of uninitialized memory. Catches reads of uninitialized values that produce nondeterministic behavior. Requires the entire program (and libraries) to be instrumented.
- **UndefinedBehaviorSanitizer (UBSan)**: detects undefined behavior (signed overflow, null pointer dereference, misaligned access, integer shifts, type mismatches). Lightweight; can be enabled selectively.
- **ThreadSanitizer (TSan)**: detects data races (unsynchronized concurrent access). Separate from corruption but often co-occurs in concurrent native code.

The key insight: a use-after-free caught by ASan points at the exact freed-then-accessed memory, moving the diagnosis from the crash site back to the actual error. Always run the relevant sanitizer before reasoning about the crash site.

### Match the tool to the suspected error class

Choose the sanitizer by the symptom:

- **Crash in unrelated code / moving crash / garbage values**: likely heap corruption (out-of-bounds or use-after-free). Use ASan.
- **Nondeterministic values / behavior depends on uninitialized data**: use MSan.
- **Suspicious arithmetic / overflow / alignment / null deref**: use UBSsan.
- **Intermittent corruption under concurrency**: use TSan (race) and ASan (the resulting corruption).
- **Leaks (not corruption, but memory bugs)**: use LeakSanitizer (part of ASan) or Valgrind.

Running the wrong sanitizer wastes time; match it to the symptom class.

### Understand the common corruption patterns and their causes

Know the patterns so you recognize the cause when the sanitizer reports it:

- **Use-after-free**: memory is freed but a dangling pointer is still used. Cause: freeing too early while references remain, or losing track of ownership. ASan quarantines freed memory so the access is caught.
- **Double-free**: the same memory is freed twice, corrupting the allocator's free list. Cause: unclear ownership or missing null-out after free. Often crashes in the allocator.
- **Heap buffer overflow**: writing past the end of a heap allocation, corrupting adjacent allocations. Cause: off-by-one, wrong size, missing bounds check. ASan redzones catch it.
- **Stack buffer overflow**: writing past a stack buffer, corrupting the return address or other stack frames. Cause: unchecked array writes, `strcpy`, `sprintf`. ASan (with stack instrumentation) or stack canaries catch it.
- **Stack overflow (exhaustion)**: unbounded recursion or very large stack allocations exhaust the stack, causing a segfault. Cause: missing base case, or allocating large arrays on the stack. Distinct from buffer overflow; detect via the crash address near the stack guard page.
- **Uninitialized read**: reading memory before writing to it. Cause: a code path that skips initialization. MSan catches it.

### Reproduce under the sanitizer, not just under the debugger

Sanitizers need the error to occur to report it, so you must reproduce the failing behavior under the instrumented build:

- Run the failing test or workload under the ASan/MSan/UBSan build.
- For intermittent corruption, stress the operation (loop it, increase concurrency) to trigger the error under the sanitizer.
- Sanitizers change timing and layout, so they may mask timing bugs (heisenbug effect) while revealing memory bugs—use them for memory errors, and race detectors for races.

A sanitizer report is far more actionable than a crash site, because it names the allocation site, the free site, and the illegal access.

### Trace from the sanitizer report to the root cause

A sanitizer report gives you the illegal access (where), the allocation/free history (when the memory was allocated and freed), and the call stack. Use it to find the root cause:

- For use-after-free: who freed the memory, and why is a reference still held? The fix is correcting ownership/lifetime, not null-checking at the access site.
- For overflow: what size was assumed, and what was the actual bound? The fix is the bounds check or correct sizing, not catching the crash.
- For double-free: who owns the free, and why is it called twice? The fix is single clear ownership.

Always fix the root cause (the lifetime, the bounds, the ownership). Fixing the crash site (adding a null check, catching the exception) hides the corruption without removing it.

### Use Valgrind when sanitizers cannot (legacy binaries, uninstrumentable code)

Valgrind (Memcheck) detects similar errors without requiring recompilation, by running the binary in a synthetic environment. It is slower than sanitizers but works on binaries you cannot rebuild (third-party libraries, legacy code). Use it when:

- You cannot rebuild the code with sanitizers.
- You need to check a binary distribution.
- You are debugging across instrumented and uninstrumented code.

Prefer sanitizers when you can rebuild, because they are faster and more precise.

## Common Traps

### Debugging the crash site instead of the corruption

The crash is where corruption became visible, not where it occurred. Iterating on the crash site (adding checks, catching signals) never finds the cause. Use a sanitizer to move the diagnosis back to the corruption.

### Using the wrong sanitizer for the symptom

Running ASan for an uninitialized-read bug (ASan does not detect it; MSan does), or UBSan for a use-after-free. Match the sanitizer to the error class indicated by the symptom.

### Fixing the symptom rather than the lifetime/ownership

Adding a null check or a try/catch at the crash site, which hides the corruption but leaves the dangling pointer or the bad bounds. The corruption will resurface elsewhere. Fix the root cause.

### Assuming the bug is gone when the crash stops

The crash stopping (due to a layout change, a timing change, or a symptom-level fix) does not mean the corruption is fixed. Verify with the sanitizer that the underlying error is gone.

### Not reproducing under the instrumented build

A sanitizer can only report an error that occurs. If you do not reproduce the failing behavior under the sanitized build, the sanitizer reports nothing and you conclude (wrongly) there is no memory error.

### Ignoring sanitizer reports as "noise"

Sanitizers sometimes report in third-party or library code, and developers dismiss them. A real sanitizer report indicates a real error (or a bug in the sanitizer, which is rare). Investigate rather than suppress.

### Confusing stack overflow with stack buffer overflow

Stack overflow (exhaustion from deep recursion) and stack buffer overflow (writing past a stack array) have similar names and different causes and fixes. Distinguish by the crash address and the report.

### Relying on debug builds that mask corruption

Debug builds change layout and disable optimizations, often hiding the overflow or use-after-free that the release build exhibits. Reproduce with sanitizers (which deliberately expose corruption) in a release-like configuration.

## Self-Check

- Have you recognized the corruption signature (crash in unrelated code, moving crash, garbage values, layout-sensitive intermittent failure) and shifted from crash-site debugging to corruption hunting?
- Are you running the correct sanitizer for the symptom class (ASan for overflow/use-after-free/double-free, MSan for uninitialized reads, UBSan for undefined behavior, TSan for races)?
- Have you reproduced the failing behavior under the sanitized build, since a sanitizer can only report an error that occurs?
- For each sanitizer report, are you tracing from the illegal access to the root cause (lifetime, ownership, bounds) rather than fixing the crash site?
- Does the fix address the root cause (correcting ownership/lifetime, adding the bounds check, fixing the initialization) rather than masking the symptom?
- Have you distinguished stack overflow (recursion exhaustion) from stack buffer overflow (out-of-bounds write), and applied the correct fix?
- Are you using Valgrind for binaries you cannot rebuild with sanitizers, and sanitizers when you can rebuild?
- Have you verified the fix by re-running under the sanitizer to confirm the error is gone, not just that the crash stopped?
- Are sanitizer reports in third-party/library code investigated rather than dismissed as noise?
- Are you reproducing in a release-like configuration with sanitizers, rather than relying on a debug build that may mask the corruption?
