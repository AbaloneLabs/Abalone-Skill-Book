---
name: c_undefined_behavior.md
description: Use when the agent is writing or reviewing C code involving signed integer arithmetic, sequence points, order of evaluation, pointer validity, strict aliasing, null dereference, shifts, integer conversions, volatile, signal handlers, or any construct where the standard leaves behavior unspecified or undefined and the compiler is permitted to optimize assuming the code never triggers it.
---

# Undefined Behavior In C

Undefined behavior (UB) is the gap the C standard deliberately leaves so that compilers can assume programs do not do certain things, and optimize accordingly. The consequence is not "the program might misbehave" — it is "the compiler is allowed to assume this never happens, and to delete, reorder, or transform code based on that assumption." A program with UB has no defined meaning at all, from the point of the UB onward (and, under modern optimization, often before it). The bug is not localized; it can manifest anywhere the optimizer reasoned about the assumption.

Agents frequently reason about C as if it were "the assembly my compiler emits today." It is not. C is defined by an abstract machine in the standard, and the gap between that abstract machine and any real binary is exactly where UB lives. Signed overflow that "wraps" on x86, a NULL dereference that "just segfaults," an uninitialized read that "returns zero" — all of these are observed behavior of a particular binary, not guarantees of the language. The harm is that these programs pass testing and then break catastrophically under a new compiler version, a higher optimization level, or LTO. This skill is about learning to read C against the standard, not against the disassembler, and to treat any UB as a defect that must be fixed even when the current binary behaves.

## Core Rules

### Treat The Standard, Not The Binary, As The Contract

Before relying on the behavior of any edge-case construct, identify whether the standard defines it, leaves it unspecified, leaves it implementation-defined, or leaves it undefined. These are different categories with different consequences.

- **Defined:** the standard guarantees the behavior across all conforming implementations. Rely freely.
- **Implementation-defined:** the implementation must document its choice (e.g., whether `char` is signed). Rely only if you have read the documentation and do not need portability.
- **Unspecified:** the implementation may pick any of several allowed behaviors, need not document it, and need not be consistent. Do not depend on a particular choice.
- **Undefined:** anything may happen, and the optimizer may assume it does not occur. This is the dangerous category.

The discipline is to read code and, for each operation, ask "what does the standard say?" If the answer is "nothing" or "it is undefined," fix it regardless of what the current binary does. Tools (`-fsanitize=undefined`, `-fsanitize=address`, static analyzers) exist precisely because the binary cannot be trusted to reveal UB.

### Never Rely On Signed Integer Overflow Wrapping

Signed integer overflow is undefined behavior. Unsigned overflow is defined to wrap modulo 2^N. This asymmetry is intentional and is exploited by optimizers.

- `INT_MAX + 1` is UB, not a defined wrap to `INT_MIN`, even though x86 happens to wrap. A compiler may conclude "this loop increments a signed counter, overflow is UB, therefore the loop cannot overflow, therefore the termination test can be simplified," and produce an infinite loop or an unexpectedly-unrolled one.
- For sizes, counts, and any value that can legitimately grow large, prefer unsigned types (`size_t`, `uint32_t`) whose overflow is defined.
- When you need signed arithmetic that may overflow, check before the operation: `if (b > 0 && a > INT_MAX - b) /* overflow */`. Compilers do not always optimize these checks correctly unless you use compiler builtins (`__builtin_add_overflow`) or compile with `-fwrapv` to make signed overflow defined (a non-standard but widely supported option).
- Be especially careful with expressions like `a + b < c` used as an overflow check; the compiler may delete the check on the grounds that overflow is UB and therefore cannot occur.

### Respect Sequence Points And Evaluation Order

C does not define a full left-to-right evaluation order, and modifying the same object twice without an intervening sequence point is UB. The classic examples are `i = i++`, `a[i] = i++`, and `printf("%d %d", i++, i)`, but the trap is broader.

- Between sequence points, an object may be modified at most once, and if it is modified, all other accesses to it must be for the purpose of computing the new value. Violating this is UB.
- Function arguments, operands of most operators (except `&&`, `||`, `?:`, `,`, and a few others), and the operands of an assignment's right and left side have unspecified evaluation order. Do not write expressions whose correctness depends on the order, e.g., `f(i++, i)` or `arr[i] = i++`.
- Side effects in function arguments (mutation of a shared object, `i++`) combined with reading the same object in another argument are a frequent source of order-dependent bugs. Split such expressions into separate statements.

In C11+, the evaluation-order rules were slightly tightened, but the safe rule remains: do not read and write the same object in the same full expression unless the language guarantees the sequencing.

### Do Not Dereference Null, Freed, Or Out-Of-Bounds Pointers

These are all UB, and modern optimizers exploit the assumption that they cannot occur.

- Dereferencing NULL is UB. On some platforms it traps; on others (with kernel-mapped zero page) it silently reads garbage; under optimization, the compiler may assume "this pointer was dereferenced, therefore it is not NULL, therefore this earlier NULL check is dead code" and delete the check. This is a real, documented optimization.
- Dereferencing a freed pointer, or reading memory after free, is UB. The allocator may have reused the block.
- Forming (not just dereferencing) a pointer one past the end of an array is allowed, but dereferencing it, or forming a pointer more than one past the end, is UB. Pointer arithmetic that leaves the bounds of the allocated object is UB even if the result is never dereferenced.

The implication: defensive NULL checks are not redundant — they are required — but they must be written so the optimizer cannot delete them. Compilers generally preserve explicit checks; the danger is when a check is "obviously" unnecessary and the compiler proves it away.

### Handle Shifts, Divisions, And Conversions Carefully

Several arithmetic operations have narrow UB traps that are easy to overlook.

- Shifting by a negative amount, or by an amount greater than or equal to the width of the type, is UB. `x << 32` on a 32-bit type is UB, not zero.
- Left-shifting a 1 into or past the sign bit of a signed integer is UB (prior to C20 for some cases). Use unsigned types for bit manipulation.
- Division or modulo by zero is UB. Guard divisors explicitly.
- Integer conversions that cannot represent the value are implementation-defined for signed targets and defined (wraparound) for unsigned, but narrowing conversions that silently truncate are a frequent bug source even when defined.

For bit manipulation, prefer unsigned types throughout; this removes most of the shift-related UB surface.

### Use volatile Only For Its Actual Purpose

`volatile` tells the compiler that an object may change in ways the program cannot detect, so loads and stores must not be optimized away or reordered with respect to other volatile accesses. It does not make operations atomic, it does not provide a memory barrier, and it does not make non-atomic access thread-safe.

- Correct uses: memory-mapped hardware registers, variables modified by signal handlers, `setjmp`/`longjmp`-restored locals, and (sometimes) blocking on a flag in a busy loop.
- Incorrect uses: implementing synchronization between threads (`volatile` is not a substitute for atomics or mutexes), "preventing the compiler from optimizing" a normal variable, or forcing a particular memory layout.
- For inter-thread communication, use C11 `<stdatomic.h>` or platform atomics with the appropriate memory ordering. `volatile` alone is a data race and therefore UB under the C11 memory model.

### Keep Signal Handlers And Restricted Environments Minimal

Code running in a signal handler, an interrupt context, or after `fork` before `exec` runs in a restricted environment where most library functions are not async-signal-safe. Calling `malloc`, `printf`, or any function that takes a lock is UB in a signal handler.

- In a signal handler, only assign to a `volatile sig_atomic_t` flag and call async-signal-safe functions (`write`, `_exit`). Defer all other work to the main loop.
- After `fork` in a multithreaded program, only async-signal-safe functions may be called before `exec`, because locks held by other threads at fork time are still held and the state is inconsistent.

## Common Traps

### "It Wraps On x86, So It Is Fine"

Signed overflow that wraps on x86 is still UB, and GCC and Clang will exploit that UB at `-O2` and above to delete overflow checks, assume loop bounds, or unroll loops. A program that has worked for years can break under a new `-O` level or under LTO. Never justify signed overflow by observed wrapping.

### Reading Uninitialized Memory And Expecting Zero

Reading an uninitialized automatic variable is UB. It is not "returns whatever was on the stack," and it is certainly not "returns zero." Compilers may propagate the indeterminate value in surprising ways, including assuming the variable was never read. Always initialize, or use calloc for heap memory, or memset.

### The Optimizer Deleted My NULL Check

After a pointer is dereferenced, the compiler may conclude it is non-NULL and delete a subsequent NULL check as dead code. This is legal under the standard. The fix is to structure code so the check happens before any dereference, and to avoid patterns where the compiler can prove the pointer was already used. Sanitizers and `-fno-delete-null-pointer-checks` can reveal or suppress this, but the real fix is correct ordering.

### Assuming Evaluation Order From Left To Right

`func(i++, i)` does not evaluate left-to-right in general; the order of argument evaluation is unspecified, so the second argument may see the pre- or post-increment value. Split into separate statements. The same applies to `a[i] = i++` and similar mixed read/write expressions.

### Using volatile For Thread Synchronization

`volatile` does not prevent data races, does not provide memory ordering, and does not make a read-modify-write atomic. Two threads incrementing a `volatile int` still race, and the result is UB. Use atomics or a mutex.

### Shift By The Type Width

`1u << 32` where `unsigned` is 32 bits is UB, not zero and not a rotation. Mask or range-check the shift amount before shifting, especially when the shift count comes from external data.

### Relying On Two's Complement Sign Representation Via Pointer Casts

Reading the bytes of a negative `int` through a `unsigned char *` is defined (character-type access), but assuming a specific two's-complement layout across platforms is not portable. Use explicit conversion functions for endianness and signedness rather than reinterpreting memory.

## Self-Check

- [ ] No signed integer operation can overflow on any input; sizes and counts use unsigned types, and overflow checks use compiler builtins or are compiled with `-fwrapv` if wrapping is intended.
- [ ] No expression modifies and reads the same object without an intervening sequence point; mixed read/write expressions have been split into separate statements.
- [ ] No NULL, freed, or out-of-bounds pointer is dereferenced, and no pointer arithmetic leaves the bounds of its allocated object.
- [ ] Shift counts are range-checked against the type width, bit manipulation uses unsigned types, and all divisors are guarded against zero.
- [ ] `volatile` is used only for hardware registers, signal-handler flags, or `setjmp`/`longjmp` locals, never as a substitute for atomics or mutexes in threaded code.
- [ ] Signal handlers only assign to `volatile sig_atomic_t` and call async-signal-safe functions; post-`fork` pre-`exec` code respects the same restriction.
- [ ] No automatic variable is read before being initialized, and heap memory is zeroed with `calloc` or `memset` where a defined initial value is required.
- [ ] NULL checks are placed before any dereference so the optimizer cannot delete them, and defensive checks are preserved rather than "optimized" away by hand.
- [ ] The code has been run under `-fsanitize=undefined,address` and a static analyzer, and every reported issue has been resolved rather than rationalized.
- [ ] No "it works on my machine" justification remains for any construct the standard leaves undefined, unspecified, or implementation-defined.
