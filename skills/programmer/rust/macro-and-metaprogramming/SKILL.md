---
name: macro_and_metaprogramming.md
description: Use when the agent is writing or debugging declarative macros (macro_rules) or procedural macros (derive, attribute, function-like) in Rust, reasoning about macro hygiene, generating code vs hand-writing it, deciding whether a macro is justified over a function or trait, or maintaining macro-heavy code. Also covers the failure mode of macros used where a function would do, macro hygiene bugs that produce confusing errors, procedural macros that are hard to debug, code generation that obscures what is actually compiled, and the readability and compile-time costs of macro overuse.
---

# Macro And Metaprogramming

Macros in Rust generate code at compile time, and they are powerful enough to abstract patterns that functions and traits cannot — variadic interfaces, compile-time code generation from attributes, deriving implementations across many types. The judgment problem is that this power is easy to misuse, and the costs are deferred to the reader and the compiler. A macro used where a function would do adds indirection without benefit; a macro that expands to surprising code makes errors point at the call site rather than the generated code, baffling the user; a procedural macro that is hard to debug turns every change into a trial-and-error cycle; and macro-heavy codebases compile slowly and read poorly because the actual code is hidden behind layers of generation. The discipline is to reach for a macro only when functions and traits are insufficient (when you need code generation, variadic arity, or compile-time derivation), to write hygienic macros that produce predictable expansions, to prefer declarative macros (`macro_rules`) over procedural macros where the declarative form suffices, to make generated code inspectable for debugging, and to weigh the readability and compile-time cost against the abstraction benefit.

Agents tend to reach for macros because they are powerful and elegant, and the harm appears as code that cannot be read without expanding the macro (the actual logic is hidden), as error messages that point at the macro invocation with no hint of the generated code, as procedural macros that take long to compile and are painful to debug, and as abstractions that a function or trait would have expressed more simply. The judgment is to justify each macro against the simpler alternative, to prefer the least powerful mechanism that works, to ensure macros are hygienic and their expansions inspectable, and to document what a macro expands to so readers and debuggers are not lost. A macro is justified when it abstracts something nothing else can; it is overused when it abstracts something a function already does.

## Core Rules

### Reach For A Macro Only When Functions And Traits Are Insufficient

The first question is whether a macro is needed at all. Functions, generics, and traits cover most abstraction needs, and they are simpler, faster to compile, and easier to debug. A macro is justified only when these are insufficient.

- **A macro is justified for code generation** (deriving implementations from attributes, generating boilerplate across many types, generating code from a DSL).
- **A macro is justified for variadic interfaces** that functions cannot express (a variable number of arguments, heterogeneous lists).
- **A macro is justified for compile-time computation or patterns that require inspecting the syntax** (a `vec!`-like constructor, a `println!`-like format string).
- **A macro is not justified when a function would do.** If the abstraction can be expressed as a function (possibly generic), use the function; the macro adds cost without benefit.

### Prefer Declarative Macros Over Procedural Where The Declarative Form Suffices

Declarative macros (`macro_rules`) are pattern-matching templates; procedural macros (derive, attribute, function-like) are full Rust programs that operate on syntax trees. Declarative macros are simpler, faster, and hygienic by default; procedural macros are more powerful but slower to compile, harder to write, and harder to debug.

- **Use `macro_rules` for pattern-based code generation.** When the macro is a template with pattern alternatives, `macro_rules` is simpler and sufficient.
- **Use procedural macros only when `macro_rules` cannot express the need** (deriving from struct attributes, complex syntax analysis, code generation that requires parsing arbitrary input).
- **Be aware of the compile-time cost.** Procedural macros compile as separate crates and run at compile time; heavy proc-macro dependencies slow every build. Prefer lighter alternatives where they exist.

### Write Hygienic Macros With Predictable Expansions

Macro hygiene means identifiers introduced by the macro do not collide with identifiers at the call site, so the macro's expansion is predictable regardless of where it is used. Non-hygienic macros (or manual identifier generation in proc macros) produce confusing errors when a generated identifier collides with a user's identifier.

- **Declarative macros are hygienic by default; respect that.** Do not defeat hygiene by leaking identifiers in ways that collide with the call site.
- **In procedural macros, generate identifiers deliberately** (using spans and `format_ident!` appropriately) so they do not collide with user identifiers or produce confusing errors.
- **Make the expansion predictable.** A reader who knows what the macro expands to should be able to predict the generated code; surprising expansions (side effects, unexpected control flow) make the macro dangerous.

### Make Generated Code Inspectable And Debuggable

Macros hide the actual compiled code behind the invocation, which makes errors and debugging hard. The expansion should be inspectable so that when something goes wrong, the user can see what was generated.

- **Use `cargo expand` to inspect expansions during development.** Seeing the generated code is the primary debugging tool for macros; ensure your macros expand to something inspectable.
- **Generate clear error spans.** A procedural macro that emits an error should point at the relevant input span, not at the whole invocation; good spans make macro errors actionable.
- **Document what the macro expands to.** A doc comment showing the expansion (or a representative example) helps users understand and debug the generated code.
- **Avoid generating enormous expansions.** A macro that expands to thousands of lines inflates compile time and is hard to debug; generate what is needed.

### Weigh Readability And Compile-Time Cost Against Abstraction Benefit

Macros trade readability and compile time for abstraction power. The trade is worth it when the abstraction is genuinely needed and well-designed; it is not worth it when the macro obscures logic that could be plain.

- **A macro should make call sites clearer, not murkier.** If reading a macro invocation is harder than reading the expanded code, the macro is hurting readability.
- **Account for compile-time cost in the decision.** A macro that saves 50 lines but adds seconds to every compile may not be worth it across a large codebase.
- **Prefer fewer, well-designed macros over many ad-hoc ones.** A small set of robust macros is easier to maintain than a sprawl of one-off macros.

### Consider Code Generation (build scripts, codegen) As An Alternative

Sometimes the need is not a macro but code generation: a build script that generates Rust code from a schema, a codegen step that produces boilerplate. This can be clearer than a macro when the generation is substantial and independent of call-site syntax.

- **Consider `build.rs` codegen for substantial, schema-driven generation** (generating types from a protocol definition, generating bindings).
- **Prefer macros for call-site-driven generation** (deriving from a struct the user wrote, a constructor at the call site).

## Common Traps

### Macro Where A Function Would Do

Using a macro for an abstraction that a function (possibly generic) already expresses, adding indirection, compile-time cost, and readability harm without benefit. Reach for a macro only when functions and traits are insufficient.

### Procedural Macro Where Declarative Would Suffice

A procedural macro for pattern-based code generation that `macro_rules` could express, paying the compile-time and complexity cost unnecessarily. Prefer `macro_rules` where it suffices.

### Non-Hygienic Macro Producing Confusing Errors

A macro whose generated identifiers collide with the call site, producing errors that baffle the user because the colliding identifier is invisible. Respect hygiene; generate identifiers deliberately in proc macros.

### Errors Pointing At The Invocation With No Generated-Code Context

A macro error that points at the invocation with no hint of the generated code, leaving the user unable to diagnose. Generate clear error spans; make expansions inspectable with `cargo expand`; document the expansion.

### Macro That Obscures Logic

A macro whose invocation is harder to read than the expanded code, hurting readability for an abstraction that does not justify it. A macro should clarify call sites, not murk them.

### Heavy Proc-Macro Dependencies Slowing Every Build

Procedural macros compiled as separate crates and run at compile time, slowing every build across the codebase. Prefer lighter alternatives; account for compile-time cost in the decision.

### Enormous Expansion Inflating Compile Time

A macro that expands to thousands of lines, inflating compile time and complicating debugging. Generate what is needed; avoid unbounded expansion.

## Self-Check

- [ ] Each macro is justified over the simpler alternative (function, generic, trait): it is used for code generation, variadic interfaces, compile-time computation, or syntax inspection that functions cannot express — not where a function would do.
- [ ] Declarative macros (`macro_rules`) are preferred over procedural macros where the declarative form suffices, with procedural macros used only when `macro_rules` cannot express the need, and the compile-time cost of proc-macro dependencies is accounted for.
- [ ] Macros are hygienic with predictable expansions: declarative macros respect default hygiene, procedural macros generate identifiers deliberately (spans, `format_ident!`) to avoid collisions, and expansions contain no surprising side effects or control flow.
- [ ] Generated code is inspectable and debuggable: `cargo expand` works on the macros, procedural macros emit errors with clear spans pointing at relevant input, expansions are documented, and expansions are not unboundedly large.
- [ ] The readability and compile-time trade is weighed: macros make call sites clearer not murkier, compile-time cost is accounted for across the codebase, and a small set of well-designed macros is preferred over a sprawl of one-offs.
- [ ] Code generation (build scripts, codegen) is considered as an alternative for substantial schema-driven generation, with macros reserved for call-site-driven generation.
- [ ] The highest-risk cases were verified — a macro justified over a function, a declarative macro chosen over a procedural one, an expansion inspectable via `cargo expand` to debug an error, and a proc macro generating clear error spans — not only the clean simple-macro path.
