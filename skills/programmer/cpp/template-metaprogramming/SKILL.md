---
name: cpp_template_metaprogramming.md
description: Use when the agent is writing or reviewing C++ code using templates, template specialization, partial specialization, SFINAE, type traits, constexpr and consteval, concepts and requires clauses (C++20), variadic templates, fold expressions, CRTP, or deciding whether compile-time computation and type-level abstraction are justified versus over-engineered, and diagnosing template error explosion, slow compilation, or hard-to-read metaprogramming.
---

# Template Metaprogramming In C++

Templates are C++'s mechanism for generic programming and compile-time computation: write the algorithm or type once, instantiate it for each type used, and let the compiler generate specialized code. Done well, templates give you type-safe containers, zero-overhead abstraction, and compile-time verification (`std::vector<T>` works for any `T`, `std::sort` is faster than C's `qsort` because the comparator is inlined). Done poorly, they produce the worst failure modes in the language: error messages thousands of lines long pointing deep into the standard library, compilation times measured in minutes, code that no reader can trace, and abstractions invented to save typing that cost far more in build time and maintainability than they save. The feature's danger is that the compiler happily instantiates enormous dependency graphs and reports failures in a vocabulary (SFINAE, substitution failure, nested requirements) that obscures the actual mistake.

Agents tend to reach for templates and metaprogramming because they feel powerful and idiomatic, then discover the costs at scale: a header-only library that rebuilds the world on every change, a SFINAE construct that works on one compiler and rejects valid code on another, a `constexpr` function that silently falls back to runtime because a sub-expression is not constant. The judgment problem is to use templates where genericity or compile-time computation genuinely pays off, to prefer the modern readable forms (concepts, `if constexpr`, `constexpr`) over legacy SFINAE gymnastics, to keep compilation times and error messages manageable, and to resist the temptation to solve a runtime problem with a type-level mechanism. Getting this wrong produces code that is technically clever and operationally expensive.

## Core Rules

### Use Templates For Genuine Genericity, Not To Save Typing

A template earns its cost when the same logic must work for multiple types with the same code, or when compile-time computation eliminates runtime overhead. It does not earn its cost when it has one or two instantiations that could be ordinary code, or when the "genericity" exists only to avoid writing a second function.

- A `vector<T>`, a `sort` that works for any random-access iterator, a `convert<T,U>` that handles type-safe conversions — these are genuine genericity: one algorithm, many types, real reuse.
- A `Processor<T>` instantiated only for `T = Order` is a non-template class with a concrete type; making it a template adds error-message and build-time cost for no reuse.
- Before templating, ask: how many distinct types will instantiate this? If the answer is "one or two known types," write concrete code. If the answer is "an open set of types," a template is justified.

Strong choice: a templated `Buffer<T>` used for `int`, `float`, and a user struct. Weak choice: a templated `OrderRepository<T>` instantiated only for `T = Database`, where a concrete class is clearer and faster to compile.

### Prefer Concepts And if constexpr Over Legacy SFINAE

C++20 concepts (`template<typename T> requires Comparable<T>`) and `if constexpr` replaced most of what SFINAE (`std::enable_if`, `typename = std::enable_if_t<...>`) used to do, and they are dramatically more readable. Concepts produce error messages that name the unmet requirement; SFINAE produces error messages that name a substitution failure deep in a trait. `if constexpr` discards the false branch at compile time; SFINAE overload-resolution tricks to select a branch are inscrutable.

- Use concepts to express type requirements: `template<std::ranges::forward_range R> void process(R&& r)`. The error message says "R does not model forward_range," not "no matching function."
- Use `if constexpr (cond) { ... } else { ... }` to branch on compile-time properties without SFINAE or tag dispatch. Both branches must be syntactically valid, but the false branch is not instantiated.
- Reserve SFINAE for maintaining pre-C++20 codebases or for the narrow cases concepts cannot express. New code should not introduce SFINAE where a concept or `if constexpr` works.

### Keep Error Messages And Compilation Times Manageable

Templates are instantiated on demand, and each instantiation generates code that must be type-checked and optimized. This is the source of C++'s notorious compile times and error verbosity. Manage both deliberately.

- Constrain templates with concepts so failures surface at the call site with a clear requirement, not deep inside the template body. A constrained template reports "T does not satisfy Hashable"; an unconstrained one reports a failure inside `std::hash<T>` three levels down.
- Factor large templates into non-templated cores operating on type-erased or concrete data, with a thin templated facade. The non-templated core compiles once; only the facade instantiates per type. This is how the standard library keeps `<vector>` manageable.
- Reduce header bloat: move template implementations that only need to be visible at the point of instantiation into source files where possible (explicit instantiation for a known set of types), and avoid including heavy headers from a widely-included template header.
- Use precompiled headers and modules (C++20) to reduce redundant parsing of the same headers across translation units.

### Use constexpr And consteval For Compile-Time Computation, And Know Their Limits

`constexpr` marks a function or variable as evaluable at compile time (and it may also run at runtime); `consteval` (C++20) requires compile-time evaluation. They let you move computation (table generation, parsing, validation) from runtime to compile time, eliminating runtime cost and catching errors earlier.

- A `constexpr` function can be used in a constant expression if its arguments are constant; the same function runs at runtime for non-constant arguments. Use `constexpr` liberally for functions that could be compile-time.
- `consteval` forces compile-time evaluation; use it for functions that must run at compile time (compile-time format string checking, reflection-like metadata).
- A `constexpr` function that silently runs at runtime because one sub-expression is not constant is a common surprise. Mark a result `constexpr` and use it in a constant-expression context to verify it actually evaluates at compile time; if you need compile-time, use `consteval` or a `static_assert` on the result.

### Use Variadic Templates And Fold Expressions; Avoid Recursive Specialization

Variadic templates (`template<typename... Ts>`) and fold expressions (`(args + ...)` in C++17) replaced the recursive-template-and-specialization pattern for handling parameter packs. Prefer them.

- `template<typename... Args> auto sum(Args... args) { return (args + ...); }` is a fold expression; the legacy recursive `sum(head) + sum(tail...)` with a base-case specialization is harder to read and slower to compile.
- Fold expressions (`(... op pack)`, `(pack op ...)`, `(pack op ... op init)`) cover sum, product, logical-and/or, and other reductions over a pack in one expression.
- Reserve recursive template instantiation for genuinely recursive compile-time structures (a compile-time AST), not for reducing a parameter pack.

### Use CRTP For Static Polymorphism, With Awareness Of Its Limits

The Curiously Recurring Template Pattern (`template<typename Derived> struct Base { void interface() { static_cast<Derived*>(this)->impl(); } }; struct Derived : Base<Derived> {...}`) implements static (compile-time) polymorphism: the base calls into the derived without virtual dispatch. It is useful for performance-sensitive polymorphism where the set of derived types is known at compile time.

- Use CRTP for mixin-like composition and zero-overhead polymorphism (a policy-based design, a compile-time interface).
- Know its limits: CRTP cannot do runtime polymorphism (you cannot store a heterogeneous collection of CRTP bases), the derived type must be complete at the point of base instantiation (which complicates some patterns), and debugging through the `static_cast` is harder than through a virtual call.
- Do not use CRTP where a simple `virtual` would do and the performance difference is unmeasured. Measure before choosing static polymorphism for performance.

### Specialize For Behavior Differences, But Prefer Overloading And if constexpr

Template specialization (full and partial) customizes a template's behavior for specific types. It is powerful and easy to misuse: a specialization in a different file from the primary template is invisible to readers, specializations must be in the same namespace as the primary, and function templates cannot be partially specialized (you must overload instead).

- Prefer function overloading and `if constexpr` to customize behavior per type; prefer class template specialization only when overloading does not fit.
- Specialize `std::` templates only where explicitly permitted (`std::hash` for user types); specializing other standard templates is undefined behavior.
- Keep specializations near the primary template and document them, so a reader sees the full set of behaviors.

## Common Traps

### One-Instance Template Adding Build Cost For No Reuse

A `Manager<T>` instantiated only for `T = Config`, adding template instantiation overhead and verbose errors for no reuse. Make it a concrete class.

### SFINAE In New C++20 Code Where A Concept Would Do

`template<typename T, typename = std::enable_if_t<std::is_integral_v<T>>>` in code that targets C++20, when `template<std::integral T>` is clearer and produces better errors. Use concepts; reserve SFINAE for pre-C++20.

### Unconstrained Template Producing Errors Deep In The Standard Library

`template<typename T> void use(T x) { std::cout << x; }` called with a non-streamable type reports the failure inside `operator<<`, far from the call. Constrain with a concept so the error names the missing requirement.

### constexpr Function Silently Running At Runtime

A `constexpr int compute(int n)` used as `compute(runtime_value)` runs at runtime because the argument is not constant. If you need compile-time, use `consteval` or verify with `static_assert` on a constant-argument call.

### Heavy Header-Only Template Library Slowing Every Build

A template library implemented entirely in headers, included everywhere, rebuilding the world on every change. Factor into a non-templated core with explicit instantiation where the type set is known.

### Recursive Template Specialization Where A Fold Expression Suffices

A variadic template reduced via head/tail recursion and a base-case specialization, when a C++17 fold expression is one line and compiles faster.

### Function Template Partial Specialization (Which Does Not Exist)

Attempting `template<typename T> void f<T*>(T*)` as a "partial specialization" of a function template; function templates cannot be partially specialized, and the syntax either fails or defines an overload with surprising resolution. Overload or use `if constexpr`.

### Specializing A std:: Template That Is Not Customizable

`template<> struct std::vector<MyType> { ... };` is undefined behavior; only specific standard templates (`std::hash`, `std::less` for the default comparator) permit user specialization. Add behavior via your own type or overload instead.

### CRTP Used For Performance Without Measuring The Virtual Alternative

Replacing a clear virtual interface with CRTP "for speed" without benchmarking, adding complexity and debug difficulty for an unmeasured gain.

## Self-Check

- [ ] Each template is justified by genuine genericity (an open set of types with real reuse) or compile-time computation, not by avoiding typing or a one-instance abstraction.
- [ ] New C++20 code uses concepts and `if constexpr` to express type requirements and compile-time branching; SFINAE is reserved for pre-C++20 maintenance or cases concepts cannot cover.
- [ ] Templates are constrained so failures surface at the call site with a named requirement, and compilation times are managed (non-templated cores, explicit instantiation, minimal header includes).
- [ ] `constexpr` functions used in constant contexts are verified to evaluate at compile time (or marked `consteval`), and no `constexpr` silently falls back to runtime when compile-time was intended.
- [ ] Variadic templates use fold expressions rather than recursive specialization, and CRTP is used only for measured static-polymorphism needs with its limits understood.
- [ ] Specializations are near their primary templates, function customization uses overloading/`if constexpr` over specialization, and no `std::` template is specialized except where the standard permits.
- [ ] Error messages from template misuse point at a clear, named requirement (concept) rather than a substitution failure deep in a trait or the standard library.
- [ ] No template abstraction exists solely to save typing where a concrete function or class would be clearer and faster to build.
- [ ] The build time impact of templated headers has been considered, and the template design does not rebuild the world on every change.
