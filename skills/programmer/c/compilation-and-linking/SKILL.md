---
name: c_compilation_and_linking.md
description: Use when the agent is writing or reviewing C code that spans multiple translation units, declaring vs defining functions and globals, designing headers, managing static and dynamic linking, resolving linker errors such as undefined reference or multiple definition, controlling symbol visibility, reasoning about the one definition rule, build flags, or diagnosing build failures where the compiler accepts the code but the linker rejects it.
---

# Compilation And Linking In C

A C program is not built in one step. Each source file is compiled independently into an object file containing symbols, and a separate linker step resolves references between those object files and libraries. This split is the source of C's compilation speed and modular build model, and it is also the source of a class of bugs that the compiler cannot catch: a function declared with one signature in a header and defined with another in the source compiles cleanly in both files and misbehaves at link time or runtime; a global defined in two translation units produces a confusing "multiple definition" error; a missing or wrong library produces "undefined reference" errors that point at the wrong cause. The compiler checks each translation unit in isolation, and only the linker sees the whole program — but the linker checks symbol names and (sometimes) types, not semantics.

Agents tend to treat compilation and linking as a black box that "just works" until a red error appears, then patch the error symptomatically without understanding the symbol-resolution model underneath. The harm is delayed and confusing: undefined references that are "fixed" by adding a library that masks a real signature mismatch, multiple-definition errors resolved by sprinkling `extern` or `static` without understanding why, header cycles that compile in one order and fail in another, or build flags that work on one platform and produce subtly different symbol layouts on another. The judgment problem is to think about the program as a set of translation units connected by declared contracts (headers), to distinguish declarations from definitions deliberately, to understand how the linker resolves symbols and what can go wrong, and to read a linker error as a statement about symbol resolution rather than a random failure.

## Core Rules

### Distinguish Declarations From Definitions, And Put Declarations In Headers

A declaration tells the compiler that a name exists and what its type is; a definition provides the actual storage or code. The distinction is load-bearing because definitions create symbols that the linker must resolve uniquely, while declarations can appear in every translation unit that needs them.

- A function prototype (`int foo(int);`) is a declaration; the function body (`int foo(int x) { ... }`) is the definition. Put the prototype in the header, the body in exactly one `.c` file.
- A global variable `extern int count;` is a declaration; `int count = 0;` is the definition. Define the variable in one `.c` file and declare it `extern` in a header that other files include.
- A header should contain declarations (prototypes, `extern` declarations, type definitions, macros), not definitions. Defining a global or a function body in a header means every translation unit that includes it gets its own definition, producing "multiple definition" linker errors (or, for `static`, silent duplicated copies).

Strong choice: `config.h` declares `extern struct Config g_config;`, and `config.c` defines `struct Config g_config;`. Weak choice: defining `struct Config g_config;` directly in the header, so every includer creates a separate symbol.

### Honor The One Definition Rule Across Translation Units

Each external (non-`static`) symbol must be defined exactly once across the entire program. Violating this produces either a linker error (multiple definition) or, worse, undefined behavior (the standard calls it undefined behavior, though most linkers error).

- A function or global defined in a header and included into multiple translation units yields multiple definitions. The fix is to declare in the header and define in one source file, or to mark the header definition `static` if you genuinely want a private copy per translation unit (which is rarely the intent for a function meant to be shared).
- `inline` functions have special rules: a non-`static inline` function defined in a header must have exactly one `extern` declaration (a file providing the external definition) across the program, or you risk undefined behavior. In C99+, prefer `static inline` in headers for small functions, which gives each translation unit its own copy with no linker resolution needed.
- Tentative definitions (a file-scope `int x;` without an initializer and without `extern`) are a C-specific footgun: each file can have a tentative definition, and if multiple files do, the behavior is implementation-defined (some linkers merge them, some error). Be explicit — use `extern` for declarations and a single initialized definition.

### Understand How The Linker Resolves Symbols And Libraries

Linker errors fall into recognizable categories, and reading them correctly requires understanding the resolution model.

- **Undefined reference (`undefined reference to 'foo'`):** the symbol is used (declared and called) but never defined anywhere the linker searched. Causes: a source file not compiled or not passed to the linker, a library not linked (`-lfoo`), a library linked in the wrong order (see below), a name typo between declaration and definition, or a conditional compilation block that excluded the definition.
- **Multiple definition (`multiple definition of 'foo'`):** the symbol is defined more than once. Causes: a definition in a header included multiple times, a global defined in two source files, a library that defines a symbol your code also defines.
- **Library ordering matters.** With static libraries (`-lfoo`), the linker resolves symbols left-to-right: a library must appear after the object files that reference it, and if library B depends on library A, B must come before A on the command line. Getting this wrong produces "undefined reference" errors even though the library is present. With `--start-group`/`--end-group` or modern linkers, the ordering constraint is relaxed, but it remains a common trap.

Read a linker error as "the linker could not find a unique definition for this symbol," then trace where the symbol should be defined and why that definition is not visible.

### Control Symbol Visibility With static And Linker Attributes

Not every symbol should be visible outside its translation unit or library. Excess visible symbols pollute the global namespace, cause accidental collisions, and (for shared libraries) prevent the linker from optimizing and increase load cost.

- Mark file-local functions and globals `static` so they are visible only within their translation unit. This avoids name collisions, lets the compiler optimize more aggressively (inlining, dead-code removal), and is the C mechanism for encapsulation.
- For shared libraries (`.so`/`.dylib`/`.dll`), control which symbols are exported. Use `__attribute__((visibility("hidden")))` for internal symbols and default visibility for the public API, or compile with `-fvisibility=hidden` and mark exported symbols explicitly. Exporting everything bloats the symbol table and creates an accidental public API that constrains future changes.
- A library should expose a deliberate, documented API surface and hide its internals. Treat the exported symbol set as a contract: once a symbol is exported, callers may depend on it.

### Manage Static And Dynamic Linking Tradeoffs Deliberately

Static linking bundles a library into the executable; dynamic linking resolves it at load time from a shared library. Each has consequences for deployment, performance, security, and versioning.

- **Static linking** produces a self-contained executable with no runtime dependency on the library, simpler deployment, and potentially better startup time. The cost is larger binaries, inability to patch the library without rebuilding, and licensing obligations (some libraries, e.g., GPL, have implications when statically linked).
- **Dynamic linking** keeps the library separate, so a security patch to the library updates all dependents without rebuilding them, and multiple processes share one copy in memory. The cost is a runtime dependency (the right `.so` version must be present), load-time symbol resolution overhead, and version-compatibility risk (the loaded library may differ from the one built against).
- For dynamic libraries, versioning matters: use SONAME versioning (`libfoo.so.1`) and semantic versioning so an incompatible ABI change bumps the major version. A program built against `libfoo.so.1` should not silently load `libfoo.so.2` with an incompatible ABI.

Decide the linking model deliberately based on deployment constraints, and do not let the build default decide for you.

### Keep Headers Self-Contained And Cycle-Free

A header should compile on its own when included into an empty `.c` file, and including it twice (directly or transitively) should be harmless. These properties are not automatic; they require include guards and dependency discipline.

- Every header needs an include guard (`#ifndef FOO_H / #define FOO_H / ... / #endif`) or `#pragma once` to prevent multiple inclusion in a single translation unit. Without it, a header included twice (directly or via two other headers) redefines types and produces errors.
- A header should include every other header it depends on for its own declarations (self-containment), rather than relying on the includer to include dependencies in the right order. If `config.h` uses `size_t`, it should include `<stddef.h>` itself. Headers that only compile when included after some other header are fragile and order-dependent.
- Avoid include cycles: if `a.h` includes `b.h` and `b.h` includes `a.h`, neither can compile unless you use forward declarations to break the cycle. Prefer forward declarations (`struct Foo;`) in headers and include the full definition only in `.c` files where the complete type is needed.

## Common Traps

### Defining A Function Or Global In A Header

Putting `int counter = 0;` or a function body in a header means every translation unit that includes it defines the symbol, producing "multiple definition" errors. Declare in the header (`extern int counter;`), define in one source file. The exception is `static inline` functions, which are intentionally per-translation-unit.

### Signature Mismatch Between Declaration And Definition

`int foo(int);` in the header and `int foo(unsigned x) { ... }` in the source compile cleanly in each file (the compiler trusts the declaration it sees) and may link, but the calling convention or argument size differs, producing garbage at runtime. Keep declarations and definitions synchronized; the header is the single source of truth for the signature.

### Library Linked In The Wrong Order

`gcc main.c -lm -lfoo` where `main.c` depends on `libfoo` which depends on `libm` produces "undefined reference" because the linker processes `libm` before `libfoo` needs it. Put dependent libraries after their dependents: `gcc main.c -lfoo -lm`.

### Treating A Linker Error As A Compiler Error

An "undefined reference" is a linker error about symbol resolution, not a syntax error. Adding a stray semicolon or reordering code will not fix it; you must provide the missing definition or library, or fix the symbol name. Diagnose by tracing where the symbol should be defined.

### `static` Applied To Silence A Multiple-Definition Error

When two files each define `int counter;` and the linker complains, slapping `static` on both makes the error vanish but creates two independent counters where the author expected one shared global. Understand whether you want one shared symbol (declare `extern`, define once) or per-file symbols (intentional `static`) before "fixing" the error.

### Confusing Declaration With Definition For Globals

`int x;` at file scope is a tentative definition, not just a declaration. Multiple tentative definitions across files are implementation-defined. Use `extern int x;` for the declaration in headers and `int x = 0;` for the single definition to be unambiguous.

### Header That Only Compiles In A Certain Include Order

A header that uses `size_t` without including `<stddef.h>`, relying on some other header to have included it first, compiles in some files and fails in others depending on include order. Make every header self-contained.

### Shared Library With No Versioning

Shipping `libfoo.so` with no SONAME or version means a library update may replace it with an ABI-incompatible version that loads fine and crashes at runtime. Use SONAME versioning and bump the major version on ABI-breaking changes.

## Self-Check

- [ ] Every header contains declarations (prototypes, `extern`, types, macros) and no definitions of functions or globals; definitions live in exactly one `.c` file, and headers use include guards or `#pragma once`.
- [ ] Each external symbol is defined exactly once across the program; the one-definition rule is respected, and `inline` functions in headers use `static inline` or provide a single external definition.
- [ ] Linker errors are diagnosed as symbol-resolution problems (missing definition, missing or mis-ordered library, name mismatch), not treated as syntax errors, and the root cause is fixed rather than the symptom.
- [ ] File-local functions and globals are `static`, and shared libraries expose a deliberate, minimal, versioned API surface with internal symbols hidden.
- [ ] Static vs dynamic linking is a deliberate decision based on deployment, patching, and versioning needs, and dynamic libraries use SONAME/semantic versioning to prevent ABI mismatches.
- [ ] Every header is self-contained (compiles when included into an empty file, includes its own dependencies) and free of include cycles (forward declarations break cycles where needed).
- [ ] Function signatures in headers match their definitions exactly, and the header is treated as the single source of truth for the contract.
- [ ] No `static` or `extern` has been added to silence a linker error without understanding whether one shared symbol or per-file symbols is intended.
- [ ] A clean build from scratch (all object files removed) succeeds and produces the same result, with no reliance on stale object files masking a real error.
