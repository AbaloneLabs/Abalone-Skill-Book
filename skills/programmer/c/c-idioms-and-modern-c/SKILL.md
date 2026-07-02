---
name: c_idioms_and_modern_c.md
description: Use when the agent is writing or reviewing C code using C99, C11, or C17 features such as designated initializers, compound literals, variable-length arrays, _Generic, _Static_assert, fixed-width integer types, stdatomic, threads.h, bool, inline functions, restrict, or deciding which C standard version to target and how to keep code portable across compiler versions and feature support.
---

# C Idioms And Modern C

Modern C (C99 onward, through C11, C17, and C23) added substantial features that make C safer and more expressive than the "K&R with ANSI" dialect many programmers still default to: designated initializers, compound literals, `_Generic`, `_Static_assert`, `<stdbool.h>`, `<stdint.h>` fixed-width integers, `<stdatomic.h>`, `<threads.h>`, and `restrict`. Used deliberately, these produce clearer, more portable, and more checkable code. Used carelessly — reaching for a C11 feature in a codebase that must compile on a C99 toolchain, assuming variable-length arrays are universally supported, or writing `_Generic` dispatch that silently falls through — they produce code that compiles on one compiler and fails on another, or that works until a feature is quietly optional and unsupported. The judgment problem is to know which features are standard-mandated versus optional, to use modern idioms that improve clarity without sacrificing portability, and to pin the C standard version deliberately rather than letting the compiler default decide.

Agents tend to write C as if it were one uniform language, then discover that VLAs are optional in C11, that `<threads.h>` is not implemented in some mainstream toolchains, or that `_Generic` does not do what they assumed for qualified or promoted types. The harm is portability breakage and subtle bugs: a designated initializer that works in C99 but the team's embedded compiler does not support, an `_Generic` that handles `int` but not the `unsigned int` an expression promotes to, an atomic operation that looks portable but relies on a lock-based fallback. This skill is about treating the C standard version as a real choice, knowing which features are load-bearing for portability, and using modern idioms where they earn their place rather than as decoration.

## Core Rules

### Pin The Standard Version Deliberately, And Know What Each Version Adds

The C standard version is a compile-time contract (`-std=c11`, `/std:c11`). Pin it explicitly rather than relying on a compiler default that varies across toolchains and versions. Know what each version added so you can choose the lowest version that supports the features you need.

- **C99** added designated initializers, compound literals, `<stdbool.h>`, `<stdint.h>`, `//` comments, mixed declarations, variable-length arrays (VLAs), `long long`, and `snprintf` standardization. Most of modern C's expressiveness comes from C99.
- **C11** added `_Generic`, `_Static_assert`, `<stdatomic.h>`, `<threads.h>`, aligned allocation, and made VLAs optional. C11 also added `char16_t`/`char32_t`.
- **C17/C18** was mostly bug fixes and clarifications over C11.
- **C23** added `nullptr`, `typeof`, `constexpr`, `#embed`, `bool`/`true`/`false` as keywords, and more.

Pin the version with a build flag and a feature-test macro, and document the minimum required version so a contributor does not accidentally use a newer feature.

### Use Designated Initializers And Compound Literals For Clarity

Designated initializers (`.field = value`) and compound literals (`(struct Point){.x = 1, .y = 2}`) are C99 features that make struct and array initialization readable and order-independent. They are almost always an improvement over positional initialization.

- `struct Config cfg = {.timeout = 30, .retries = 3, .host = "x"};` is clearer than positional `{30, 3, "x"}`, which breaks silently if the field order changes. Designated initializers also zero-initialize unmentioned fields, which is usually the safe default.
- Compound literals let you create an unnamed struct or array inline: `draw_line((struct Point){0,0}, (struct Point){10,10});`. They are useful for passing aggregate values without a named variable.
- Both require C99. If you must support a pre-C99 compiler (rare today), you cannot use them; otherwise, prefer them.

### Treat Optional C11 Features As Conditional, Not Guaranteed

C11 made two notable features optional, meaning a conforming C11 compiler may not implement them. Treating them as guaranteed produces code that compiles on one toolchain and fails on another.

- **Variable-length arrays (VLAs)** became optional in C11 because some platforms (notably security-focused and embedded) cannot implement them safely. Check `__STDC_NO_VLA__` before using VLAs, or avoid them — they have their own safety issues (unbounded stack allocation; a VLA whose size comes from external input can overflow the stack). Prefer `alloca` with explicit bounds, or a fixed-size buffer with a size check, or `malloc`.
- **`<threads.h>` and `<stdatomic.h>`** are technically mandatory in C11 but several major toolchains (notably older MSVC) did not ship `<threads.h>`. Check for availability (`__STDC_NO_THREADS__`) or use a portability library (pthreads on POSIX, C11 threads where available, a wrapper) rather than assuming `<threads.h>` exists.
- **`<stdatomic.h>`** is widely available but check the compiler's atomic support for your target; some embedded targets lack lock-free atomics for wider types and fall back to locks.

For any optional feature, gate with a feature-test macro and provide a fallback, or avoid the feature if portability matters more than the convenience.

### Use _Generic For Type-Dispatch, But Mind Promotion And Qualifiers

`_Generic` (C11) selects an expression based on the type of its controlling expression, enabling type-generic macros. It is powerful and easy to misuse.

- `_Generic(x, int: f_int, double: f_double, default: f_other)(x)` dispatches to the right function by type. Useful for type-generic APIs (a `print` macro that handles `int`, `double`, `char*`).
- The controlling expression undergoes default argument promotion and lvalue conversion: `char` and `short` promote to `int`, array-to-pointer decay applies, and top-level qualifiers (`const`, `volatile`) are stripped. A `_Generic` that lists `char` will never match a `char` argument because it promotes to `int`. List the promoted types (`int`, `unsigned int`, `double`, etc.), not the small integer types.
- Always include a `default` association, because a `_Generic` with no matching association and no default fails to compile. Decide whether `default` should be an error or a fallback, deliberately.
- `_Generic` is a compile-time selection; it does not do runtime dispatch. Each branch's function must exist and will be type-checked.

### Prefer Fixed-Width Integer Types From stdint.h

`int`, `long`, and `short` have implementation-defined sizes that vary across platforms (a `long` is 4 bytes on Windows/LLP64 and 8 bytes on most Linux/macOS LP64). For sizes that matter — wire formats, file formats, bit manipulation, counts that can grow — use the fixed-width types from `<stdint.h>`.

- Use `int32_t`, `uint64_t`, `uint8_t` for exact widths. Use `size_t` for object sizes and array indices (it is the type of `sizeof` and the correct width on every platform). Use `int_fast32_t`/`int_least16_t` when you want the fastest/smallest type of at least a width.
- Avoid `long` and `long long` for portable sizes; their widths vary. `long long` is at least 64 bits (C99 guarantee) but may be more.
- For printing and scanning fixed-width types, use the `PRI*`/`SCN*` macros from `<inttypes.h>` (`printf("%" PRId64 "\n", (int64_t)value)`), because the format specifier for `int64_t` differs across platforms (`%lld` vs `%ld`).

### Use _Static_assert For Compile-Time Invariants

`_Static_assert(cond, msg);` (C11; `static_assert` as a keyword in C23) enforces a compile-time condition. It is a free, high-value check that catches size, layout, and configuration assumptions before runtime.

- Assert struct sizes and offsets against wire-format or ABI requirements: `_Static_assert(sizeof(struct Header) == 16, "Header must be 16 bytes");` catches a padding or field-size change that would corrupt the format.
- Assert configuration assumptions: `_Static_assert(MAX_CLIENTS <= 1024, "MAX_CLIENTS exceeds table size");`.
- `_Static_assert` evaluates a constant expression at compile time and emits the message on failure. It has no runtime cost. Use it liberally for any invariant the code depends on.

### Use restrict And const To Express Intent And Enable Optimization

`restrict` (C99) is a promise that a pointer is the only way to access the pointed-to data in a given scope, which lets the compiler optimize (avoid reloading, vectorize). `const` marks data that will not be modified.

- `void add(int *restrict dst, const int *restrict src, size_t n);` promises `dst` and `src` do not overlap, enabling vectorization. Violating the promise (passing overlapping pointers) is undefined behavior, so only use `restrict` when you can guarantee no aliasing.
- Use `const` on pointer parameters when the function will not modify the data; it documents intent, lets the compiler optimize, and allows callers to pass `const` data.
- `restrict` and `const` are documentation as much as optimization hints; use them to state the contract, and do not violate the `restrict` promise.

## Common Traps

### Using A C11 Feature In A C99 (Or Older) Codebase

`_Generic`, `_Static_assert`, and `<threads.h>` require C11. Code using them will not compile under `-std=c99`. Pin the standard version and ensure the whole codebase targets it, or gate features with `__STDC_VERSION__` checks.

### VLA Whose Size Comes From Untrusted Input

`int buf[n];` where `n` is read from input allocates on the stack and can overflow it with a large `n`, crashing or creating a security hole. VLAs are also optional in C11. Validate the size and use a fixed buffer or `malloc` for untrusted sizes.

### _Generic Missing The Promoted Type

`_Generic(x, char: f_char, ...)` never matches a `char` argument because `char` promotes to `int`. List `int` and `unsigned int`, and remember `char` may be signed or unsigned (implementation-defined).

### Assuming long Is 64 Bits Everywhere

`long` is 32 bits on Windows (LLP64) and 64 bits on most Linux/macOS (LP64). Code that stores a pointer in a `long`, or a 64-bit file offset in a `long`, breaks on one platform. Use `intptr_t`/`uintptr_t` for pointer-sized integers and `int64_t` for fixed 64-bit widths.

### Printing A Fixed-Width Type With The Wrong Format Specifier

`printf("%ld", (int64_t)value)` is correct on LP64 (where `int64_t` is `long`) but wrong on LLP64 Windows (where `int64_t` is `long long`, needing `%lld`). Use `PRId64` from `<inttypes.h>` for portability.

### Assuming <threads.h> Exists On Every C11 Compiler

`<threads.h>` is mandatory in the C11 standard but several toolchains did not implement it. Check `__STDC_NO_THREADS__` or use a portability wrapper (pthreads on POSIX).

### Violating A restrict Promise

Passing overlapping pointers to a `restrict`-qualified function is undefined behavior; it may work, may produce wrong results, or may miscompile under optimization. Only use `restrict` when you can guarantee no aliasing.

### Designated Initializers Relying On Field Order

While designated initializers are order-independent, mixing designated and positional initialization (`{.a = 1, 2}`) is confusing and error-prone. Use all-designated or all-positional, not a mix.

## Self-Check

- [ ] The C standard version is pinned explicitly in the build (`-std=c11` or similar) and documented as the minimum, and contributors know which features are available.
- [ ] Optional C11 features (VLAs, `<threads.h>`) are gated with feature-test macros or avoided; VLAs are never used with untrusted sizes.
- [ ] Designated initializers and compound literals are used for readable, order-independent struct initialization, and unmentioned fields are understood to be zero-initialized.
- [ ] `_Generic` lists promoted types (`int`, `unsigned int`, `double`), includes a deliberate `default`, and does not rely on matching small integer or qualified types.
- [ ] Fixed-width integer types (`int32_t`, `uint64_t`, `size_t`) are used where size matters, and fixed-width types are printed with `PRI*`/`SCN*` macros for portability.
- [ ] `_Static_assert` enforces struct sizes, offsets, and configuration invariants at compile time.
- [ ] `restrict` is used only where no-aliasing is guaranteed, and `const` marks non-modifying pointer parameters to document intent.
- [ ] No code assumes `long`/`int` widths or that `<threads.h>` is universally available; portability-sensitive sizes use `<stdint.h>` types.
- [ ] The code compiles cleanly under the pinned standard version on every target toolchain, with no reliance on compiler-specific extensions disguised as standard features.
