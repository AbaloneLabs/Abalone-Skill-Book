---
name: c_preprocessor_and_portability.md
description: Use when the agent is writing or reviewing C code that uses the preprocessor for macros, conditional compilation, platform abstraction, portability shims, compiler-specific features, fixed-width integer sizes, endianness handling, alignment, packing, or cross-platform builds where the same source must compile and behave correctly across compilers, operating systems, architectures, and ABIs.
---

# Preprocessor And Portability In C

The C preprocessor is a text-substitution pass that runs before compilation, and it is both indispensable for portability and the source of some of the language's worst readability and correctness traps. Macros do not respect scope, types, or evaluation order; conditional compilation (`#ifdef`) creates multiple programs from one source file, only one of which is ever compiled and tested; platform abstraction built on preprocessor detection can be wrong because the detected platform does not match the actual target ABI. The preprocessor is how C code adapts to Windows versus POSIX, x86 versus ARM, little-endian versus big-endian, and GCC versus MSVC — and it is how that adaptation silently goes wrong when a macro expands in an unexpected context or a `#ifdef` branch is never compiled on the developer's machine.

Agents tend to overuse macros (because they "work" without type friction), to scatter `#ifdef` across function bodies (creating untested code paths), and to assume platform detection is reliable (it is not). The harm is code that compiles on the author's platform, hides a different program on another platform, and fails in the field where no one can reproduce it. The judgment problem is to use the preprocessor sparingly and for what it is genuinely good at (platform abstraction, compile-time configuration), to write macros that are safe (fully parenthesized, statement-safe, side-effect-aware), to keep conditional compilation coarse and testable rather than fine and scattered, and to abstract platform differences behind a clean API rather than letting `#ifdef _WIN32` leak into every function.

## Core Rules

### Prefer Functions And Inline To Macros; Use Macros Only For What They Alone Can Do

Function-like macros bypass the type system, ignore scope, and can evaluate arguments multiple times. A macro that looks like a function but behaves differently (double-evaluating an argument, not short-circuiting, capturing surrounding variables) is a recurring bug source. Reach for a real function first, and for `static inline` (C99) for a type-generic or performance-sensitive helper. Reserve macros for what only the preprocessor can do: compile-time configuration (`#define MAX 100`), conditional compilation, stringification/concatenation (`#`/`##`), and generic programming via `_Generic`.

When you must write a function-like macro, make it safe: fully parenthesize every parameter and the whole expression (`#define SQ(x) ((x)*(x))`), avoid multiple evaluation of arguments with side effects (or document that the argument is evaluated N times), and prefer an inline function when the macro is just "a function the compiler could not inline." A macro that doubles its argument (`#define DOUBLE(x) ((x)+(x))`) evaluates `DOUBLE(i++)` as `((i++)+(i++))`, which is undefined behavior; an inline function evaluates its argument once.

### Write Macros That Are Safe In Every Context

Macros are textual substitution, so they must be robust to the context in which they are used. Three rules cover most macro safety.

- **Fully parenthesize.** `#define BAD (a+b)*c` used as `BAD * 2` expands to `(a+b)*c * 2` (wrong precedence); `#define GOOD (((a)+(b))*(c))` is safe. Parenthesize every parameter and the entire expansion.
- **Use the do-while-zero idiom for multi-statement macros.** `#define FOO() a(); b();` used as `if (x) FOO(); else ...;` binds only `a()` to the `if`. Wrap multi-statement macros in `do { a(); b(); } while (0)` so they behave as a single statement.
- **Avoid argument re-evaluation.** `#define MAX(a,b) ((a)>(b)?(a):(b))` evaluates `MAX(i++, j++)` with one argument incremented twice. Document this, or use a `_Generic`-based or inline-function approach for type-generic max.

### Keep Conditional Compilation Coarse, Documented, And Tested

`#ifdef` creates alternate programs. Every `#ifdef` branch is code that compiles only under one configuration, and if that configuration is never built or tested, the branch is dead or wrong without anyone knowing. Minimize the surface and centralize it.

- Prefer coarse-grained conditionals at the file or function level (a whole `posix_io.c` vs `windows_io.c`) over fine-grained `#ifdef`s scattered through function bodies. A function with five `#ifdef _WIN32` blocks has five untested paths on non-Windows.
- Define a platform abstraction layer: detect the platform once in a config header, define a set of internal macros or function declarations, and implement them per-platform in separate files. Callers use the abstraction, never the raw `#ifdef`.
- Test every configuration. If you ship a Windows and a POSIX build, CI must build and test both. A platform branch that is never compiled is a branch that is almost certainly broken.

### Detect Platforms And Features Reliably, Not Optimistically

Platform detection macros (`_WIN32`, `__linux__`, `__APPLE__`, `__GNUC__`, `_MSC_VER`) tell you what the compiler claims, which usually matches the target but can be misleading (cross-compilation, compatibility layers, macro redefinition). Detect features, not platforms, where possible.

- Prefer feature detection (`__STDC_VERSION__`, `__STDC_NO_THREADS__`, `__SSE2__`, `__builtin_expect` availability) over platform detection. "Does this compiler have `<stdatomic.h>`?" is a better question than "Is this GCC?" because a compatible compiler may offer the feature without being GCC.
- When you must detect platforms, use the canonical macros (`_WIN32` for all Windows including 64-bit and ARM; `__linux__` for the kernel; `__APPLE__` for macOS/iOS) and remember that some targets match more than one (Cygwin defines `_WIN32` and `__CYGWIN__`).
- Never assume detection is exhaustive. Always provide a fallback (`#else #error "unsupported platform"` or a portable default) so an undetected platform fails loudly at compile rather than silently miscompiling.

### Abstract Endianness Explicitly, Never Via Reinterpretation

Byte order (endianness) differs across architectures (x86 is little-endian; many ARM and PowerPC modes can be either; network byte order is big-endian). Code that reinterprets memory as a wider integer assumes an endianness and breaks on the other.

- For wire formats, file formats, and network protocols, use explicit byte-shift assembly: `uint32_t v = ((uint32_t)buf[0] << 24) | ((uint32_t)buf[1] << 16) | ...`. This is correct regardless of host endianness.
- Use the byte-order conversion functions (`htonl`, `ntohl`, `htons`, `ntohs` for network; or `__builtin_bswap32`/C23 `stdc_bswap_*` for explicit swaps) rather than pointer casts.
- Detect endianness at compile time (`__BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__`) only to choose a fast path, and always provide the portable fallback. Do not assume the host endianness matches the data's endianness.

### Handle Integer Sizes And Alignment Portably

Integer sizes and alignment requirements differ across platforms and ABIs. Code that assumes a specific size or alignment breaks on a different target.

- Use `<stdint.h>` fixed-width types (`uint32_t`, `int64_t`) for sizes that matter; never assume `long` or `int` widths (see the modern-C skill).
- Do not assume struct layout, padding, or alignment. The compiler inserts padding for alignment; a struct that is 16 bytes on one platform may be 24 on another. If a struct maps to a wire format, use explicit serialization (byte-shift assembly) or a packed struct with documented assumptions, and `_Static_assert` the size.
- For aligned allocation, use `aligned_alloc` (C11) or the platform-specific aligned allocator, and check alignment of computed pointers before dereferencing wider types.

### Isolate Compiler-Specific Features Behind Portability Shims

Compiler extensions (`__attribute__`, `__builtin_*`, `__declspec`, `#pragma`) are non-portable. Use them, but wrap them in macros that fall back gracefully.

- Define portability macros: `#if defined(__GNUC__) #define UNUSED __attribute__((unused)) #else #define UNUSED #endif`. Centralize these in one portability header.
- Prefer standard equivalents where they exist (`_Static_assert` over `__attribute__((static_assert))`, `_Noreturn`/`[[noreturn]]` over `__attribute__((noreturn))`).
- Never let a compiler-specific feature leak into a portable API without a fallback for the other compilers you support.

## Common Traps

### Macro That Evaluates An Argument Twice

`#define MAX(a,b) ((a)>(b)?(a):(b))` with `MAX(i++, j++)` increments one variable twice — undefined behavior if `i` and `j` are the same. Use an inline function for typed max, or document the single-evaluation contract.

### Unparenthesized Macro Expansion

`#define SQR(x) x*x` with `SQR(a+b)` expands to `a+b*a+b` (wrong precedence). Fully parenthesize: `#define SQR(x) ((x)*(x))`.

### Multi-Statement Macro Not Wrapped In do-while

`#define LOG(x) printf(x); fflush(stdout);` inside `if (cond) LOG("x"); else ...;` binds only the `printf` to the `if`, breaking the else. Wrap in `do { ... } while (0)`.

### #ifdef Branches Never Compiled Or Tested

A Windows-only code path in a project developed on Linux, never built in CI, that ships broken. Build and test every configuration; centralize platform differences behind an abstraction.

### Reinterpreting A Byte Buffer As A Wider Integer

`uint32_t v = *(uint32_t*)buf;` assumes host endianness and alignment, breaking on big-endian or misaligned access. Use byte-shift assembly or `memcpy` into a typed variable.

### Assuming long Or int Is A Specific Width

`long` is 32 bits on Windows LLP64 and 64 bits on Linux/macOS LP64. Use `int32_t`/`int64_t` for fixed widths, and `intptr_t` for pointer-sized integers.

### Detecting Platform Instead Of Feature

`#ifdef __GNUC__` to use `<stdatomic.h>` fails on Clang (which also supports atomics but may not define `__GNUC__` the same way) or on a GCC build with atomics disabled. Detect the feature (`__STDC_NO_ATOMICS__`) instead.

### Struct Assumed To Have A Specific Size Or Layout

A struct mapped to a file format that gains padding on a different platform, corrupting the format. Use `_Static_assert(sizeof(struct) == expected)` and explicit serialization for wire formats.

## Self-Check

- [ ] Function-like macros are minimized in favor of functions and `static inline`; macros that remain are fully parenthesized, statement-safe (do-while-zero), and their argument-evaluation behavior is documented.
- [ ] Conditional compilation is coarse (file/function level) and centralized behind a platform abstraction, with every configuration built and tested in CI.
- [ ] Platform and feature detection prefers features over platforms, uses canonical macros, and provides a loud `#error` fallback for undetected targets.
- [ ] Endianness is handled with explicit byte-shift assembly or byte-order functions, never by reinterpreting memory as a wider integer.
- [ ] Integer sizes use fixed-width `<stdint.h>` types where size matters, and no code assumes `long`/`int` widths or struct layout/padding.
- [ ] Compiler-specific features are isolated behind portability macros with fallbacks, and standard equivalents are preferred where available.
- [ ] No macro expands unsafely in any plausible context (precedence, multi-statement, side-effecting arguments), and macros that double-evaluate are documented.
- [ ] Every `#ifdef` branch has been compiled and tested on its target platform, with no platform-specific path left unverified.
- [ ] The code compiles and passes its tests on every supported platform/compiler combination, with no reliance on a single developer's machine or toolchain.
