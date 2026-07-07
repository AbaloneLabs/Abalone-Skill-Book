---
name: cpp_modern_features_and_migration.md
description: Use when the agent is writing or reviewing C++ code that uses C++17/20/23 features — structured bindings, std::optional/variant/any, if constexpr, fold expressions, std::string_view, std::span, ranges and views, concepts and requires, modules, coroutines, std::format, designated initializers, constinit/consteval, three-way comparison — or when deciding which standard to target, migrating legacy C++ to modern idioms, or diagnosing feature-support and portability across compiler versions.
---

# C++ Modern Features And Migration

Modern C++ (C++17, C++20, C++23) added a large set of features that, used deliberately, make code safer, clearer, and more expressive: `std::optional` and `std::variant` replace pointer-or-null and tagged unions, `std::string_view` and `std::span` replace `const std::string&` and pointer+size pairs, `if constexpr` replaces SFINAE, concepts replace enable_if, ranges replace hand-rolled iterator loops, and modules promise to replace the preprocessor-heavy include model. Used carelessly — reaching for a C++20 feature in a C++17 codebase, using `string_view` to outlive its string, or treating coroutines like threads — they produce code that fails to compile, dangles, or performs worse than the code they replaced. The judgment problem is to know which standard to target, which features earn their place, and how to migrate legacy code without rewriting the world.

Agents tend to either ignore modern features (writing C++11 as if it were the ceiling) or over-adopt them (using ranges and concepts everywhere, including where a simple loop is clearer). The judgment problem is to choose the standard version deliberately based on compiler and library support, to adopt features where they improve correctness or clarity (not as decoration), and to migrate incrementally with a clear understanding of each feature's lifetime and performance implications. This skill is about treating the standard version and feature set as a real choice, not a default.

## Core Rules

### Pin The Standard Version Based On Real Compiler And Library Support

The C++ standard version (`-std=c++20`, `/std:c++20`) is a compile-time contract, but feature support lags the standard in every compiler. GCC, Clang, and MSVC each have a support matrix (cppreference maintains one), and `<filesystem>`, `<ranges>`, `<format>`, modules, and coroutines became usable at different points in different compilers. Pin the version to the lowest your toolchains support, and verify each feature you use is actually implemented (not just "in C++20").

For a library consumed by others, the standard version is part of the ABI/compatibility contract: if you require C++20, every consumer must have a C++20 compiler. Document the minimum. Do not assume the latest standard is available; many production codebases are pinned to C++17 or even C++14 for toolchain-stability reasons.

### Use std::optional And std::variant To Replace Pointers-As-Optionals And Tagged Unions

`std::optional<T>` expresses "a T or nothing" without a pointer and without a sentinel value. It is almost always clearer than returning a `T*` that may be null, or a `bool` out-parameter. `std::variant<A, B, C>` expresses "one of these types" with type-safe access via `std::visit` and `std::get_if`, replacing hand-rolled tagged unions and `union` with a discriminator.

- Prefer `std::optional<T>` for function returns that may have no value, rather than `T*` (which implies ownership questions) or a sentinel.
- Prefer `std::variant` for a closed set of alternative types, with `std::visit` for exhaustive handling. This is C++'s sum type and is often a better fit than a class hierarchy for a closed set of cases.
- `std::any` is the type-erased "anything" alternative; it is rarely the right choice over a variant with known alternatives, and should be avoided unless the set of types is genuinely open and dynamic.

### Treat string_view And span As Non-Owning, And Never Let Them Outlive Their Source

`std::string_view` (C++17) is a non-owning reference to a string-like sequence; `std::span` (C++20) is the same for a contiguous sequence of any type. They are excellent for function parameters (avoid the `const std::string&` allocation for non-owning reads) but they are *not* owning and they *do* dangle.

- Never store a `string_view` or `span` as a member that outlives the call, unless you can prove the source outlives it. A `string_view` member initialized from a temporary string dangles immediately.
- Never return a `string_view` to a temporary. Returning `string_view` from a function is safe only if it refers to a string with longer lifetime (a static, a parameter that the caller owns).
- Be careful with C-string literals: a `string_view` from a `const char*` literal is fine (literals have program lifetime), but a `string_view` from a `std::string` local dangles after the local dies.

When in doubt, take `const std::string&` (owning, safe) rather than `string_view` (fast, dangling risk), or overload both.

### Use if constexpr And Concepts To Replace SFINAE

Pre-C++17, compile-time branching required SFINAE (`std::enable_if`), which produced unreadable code and terrible error messages. `if constexpr` (C++17) lets you branch at compile time based on a condition, with the untaken branch not even type-checked — far clearer for template code that handles multiple types. Concepts (C++20) replace `typename = std::enable_if_t<...>` with named requirements (`template<std::integral T>`) that produce readable errors when violated.

Adopt `if constexpr` for compile-time branching in templates, and concepts for constraining templates, in any C++17/20 codebase. They are strict improvements over SFINAE. Do not use them where a runtime `if` would do — `if constexpr` is for cases where the branches would not both compile.

### Adopt Ranges And Views Where They Improve Clarity, Not Everywhere

C++20 ranges and views (`std::views::filter`, `std::views::transform`, `std::views::reverse`) let you compose lazy pipelines over sequences without writing iterator loops. They are a clarity win for multi-step transformations (`vec | views::filter(is_even) | views::transform(square)`), and they are lazy (no intermediate allocations).

But ranges are not free: deeply composed pipelines can be hard to debug, error messages are notoriously long, and some views have surprising complexity or invalidation behavior. Use ranges where the pipeline is genuinely clearer than a loop; do not force a simple loop into a range pipeline just to be "modern." A `for` loop that filters and transforms is sometimes the clearest code.

### Approach Coroutines, Modules, And format Cautiously

These three major C++20 additions have caveats:

- **Coroutines (`co_await`, `co_yield`, `co_return`)**: the language provides the mechanism but the library (`std::task`, etc.) is minimal; you typically need a framework (cppcoro, C++23 `<generator>`, or a hand-rolled promise type). Coroutines are not threads — they are suspendable functions, and misunderstanding this leads to lifetime bugs (a coroutine referencing destroyed locals). Adopt only with a clear library strategy.
- **Modules**: replace `#include` with a faster, more encapsulated model, but compiler support and build-system integration (CMake) are still maturing. Adopt when your toolchain and build system support them stably; otherwise wait.
- **`std::format` (C++20) / `std::print` (C++23)**: replace `sprintf`/`iostream` with a Python-style format-string API. Use them where available; they are safer and often faster than iostreams.

### Migrate Legacy Code Incrementally, Not In A Big Bang

Migrating a legacy C++03/11 codebase to modern idioms is valuable but risky if done all at once. Migrate incrementally: compile under the new standard first (fixing warnings), then adopt specific idioms in new code and during refactoring (use `auto`, range-for, smart pointers, `optional`/`variant`), and test each change. Do not rewrite working code purely for modernity without a behavior-preserving test in place. The highest-value migrations are usually: raw pointers to smart pointers (RAII), manual loops to algorithms/ranges, and `NULL`/0 to `nullptr`.

## Common Traps

### string_view/span Outliving Their Source

A `string_view` member or return value referring to a temporary or local string dangles. Treat them as non-owning borrows valid only for the call.

### Using A C++20 Feature In A C++17 Codebase

`std::format`, concepts, ranges, modules require C++20. Code using them fails to compile under `-std=c++17`. Pin the standard and verify feature support per compiler.

### if constexpr Used Where A Runtime if Would Do

`if constexpr` is for compile-time conditions where branches would not both compile; using it for a runtime condition adds nothing and confuses readers.

### Ranges Pipeline That Is Less Clear Than A Loop

Forcing a simple filter+transform into a ranges pipeline because it is "modern" can be harder to read and debug than a `for` loop. Adopt ranges where they genuinely clarify.

### Coroutine Referencing Destroyed Locals

A coroutine that captures references to locals and suspends may resume after the locals are destroyed. Coroutines need careful lifetime management; they are not drop-in replacements for callbacks.

### Returning optional<T> Where T Has No Null State And Ignoring Empty

`optional<T>` requires the caller to check `has_value()` before dereferencing; dereferencing an empty optional is undefined behavior (in C++17) or throws (in C++23 with `.value()`). Callers that forget to check get UB.

### Over-Adopting Features As Decoration

Using structured bindings, `auto`, ranges, and concepts everywhere because they are new, including where they obscure intent. Adopt where they improve correctness or clarity, not for fashion.

### Modules Without Build-System Support

Enabling modules in a build system (CMake) that does not support them produces confusing failures. Verify the whole toolchain supports modules before adopting.

## Self-Check

- [ ] The C++ standard version is pinned explicitly based on verified compiler/library support, and the minimum is documented for library consumers.
- [ ] `std::optional` replaces pointer-or-null and sentinel returns; `std::variant` with `std::visit` replaces tagged unions and class hierarchies for closed type sets; `std::any` is avoided unless types are genuinely open.
- [ ] `std::string_view` and `std::span` are used only as non-owning parameters or short-lived borrows, never stored as members or returned where they would outlive their source.
- [ ] `if constexpr` is used for compile-time branching where branches would not both compile, and concepts replace SFINAE for template constraints in C++20.
- [ ] Ranges and views are adopted where a pipeline is clearer than a loop, not forced onto simple cases; the choice is justified by clarity, not modernity.
- [ ] Coroutines, modules, and `std::format` are adopted only with a clear library/build-system strategy and understanding of their caveats (coroutine lifetimes, module toolchain support).
- [ ] Legacy migration is incremental and behavior-preserving, with tests covering each change, prioritizing raw-pointer-to-smart-pointer and manual-loop-to-algorithm conversions.
- [ ] No feature is adopted as decoration; each use of a modern feature improves correctness, clarity, or maintainability over the alternative.
