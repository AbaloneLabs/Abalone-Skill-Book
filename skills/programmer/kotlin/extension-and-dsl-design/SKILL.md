---
name: extension_and_dsl_design.md
description: Use when the agent is writing Kotlin extension functions, designing builder DSLs, using receiver lambdas and function types with receiver, applying @DslMarker, creating type-safe builders, scope functions, or diagnosing extension resolution ambiguity, receiver shadowing, and over-engineered DSLs in Kotlin applications.
---

# Extension and DSL Design

Kotlin's extension functions and function types with receiver make it uniquely good at building fluent, type-safe DSLs, and that power is the source of its most over-engineered code. An extension function looks free ("I am just adding a method to String"), but it participates in overload resolution, can shadow member functions under conditions, and creates an implicit contract that callers come to depend on. A DSL built on nested receiver lambdas reads beautifully at the call site and is nearly impossible to debug or extend at the definition site, because the available scope is the union of every receiver in the chain. The judgment problem is not "how do I write an extension function" but when an extension or a DSL is the right tool versus a regular function, a method, or a plain data structure.

The recurring failure mode is a developer who builds a DSL because it is elegant, accumulates layers of receiver scopes and builder classes, and then cannot add a field without breaking the fluent chain or resolve an ambiguity the compiler flags. The opposite failure is scattering utility logic as top-level extension functions on every type, so that "where does this method come from" becomes unanswerable. Extension and DSL design are tools for reducing accidental complexity at a real, repeated call site; when used to reduce complexity that does not exist, they add it.

## Core Rules

### Use extension functions to separate concerns, not to avoid class design

An extension function is appropriate when behavior belongs to a type conceptually but the type cannot or should not hold it (e.g., a method on a third-party class, or a utility that depends only on public API). It is the wrong tool when:

- The behavior is core to the type and the type is yours; make it a member.
- The behavior needs private state; extensions cannot see it, so you end up exposing internals.
- You are adding extensions to every type to avoid designing a service or repository.

Prefer members for intrinsic behavior, extensions for orthogonal utility, and services for behavior that coordinates multiple objects or state.

### Understand extension resolution and its limits

Extension functions are resolved statically by the declared (compile-time) type, not dynamically. An extension on `String?` is not the same as one on `String`. Member functions always win over extensions with the same signature. Rules:

- Do not define an extension that conflicts with a member; the member silently wins and the extension is dead code.
- Nullable-receiver extensions (`fun String?.isNullOrBlank()`) are idiomatic for null-safe utility; use them when the null case is part of the utility.
- Extensions on `Any` or `Any?` pollute every type's available scope; reserve for genuinely universal utilities and name them distinctively.

### Design DSLs around a real, repeated call site

A DSL is worth building when the same nested configuration appears in many places and the fluent form is materially clearer than the equivalent constructor calls. Rules:

- Write the desired call site first, by hand, for several real examples. If it is not clearly better, do not build the DSL.
- Keep the DSL shallow; each level of nesting adds a receiver scope and cognitive load.
- Provide sensible defaults so the minimal call site works; a DSL that requires every field is not simpler than a constructor.

### Use `@DslMarker` to prevent implicit receiver leakage

In a nested receiver DSL, an inner lambda can accidentally call functions from an outer receiver, producing subtle bugs (e.g., adding a child to the wrong parent). `@DslMarker` annotates an annotation that, when applied to your builder classes, prevents calling outer-receiver methods implicitly inside an inner lambda. Rules:

- Define a `@DslMarker` annotation and apply it to all builder classes in the DSL.
- This forces the caller to use an explicit label (`this@outer`) to reach an outer receiver, which is almost always what you want.

Without `@DslMarker`, nested DSLs silently allow outer-receiver calls and are a common source of "why did this get added to the wrong node" bugs.

### Keep receiver lambdas focused on one receiver

A function type with receiver (`A.() -> R`) makes `A` the receiver inside the lambda. Composing multiple receivers (e.g., a lambda that needs both `A` and `B` as receivers) is not directly possible and leads to workarounds that obscure the API. Rules:

- One receiver per lambda; pass additional context as parameters.
- If you need two contexts, prefer passing one as a parameter over tricks that combine receivers.

### Choose scope functions (`let`, `run`, `apply`, `also`, `with`) by intent

These are built-in DSL primitives. Each has a specific intent:

- `let`: transform a value, null-check via `?.let`. Returns the lambda result.
- `run`: compute a result with the receiver as `this`. Returns the lambda result.
- `apply`: configure an object and return it. Returns the receiver.
- `also`: perform a side effect (e.g., logging) and return the receiver.
- `with`: call multiple members on a receiver without repeating its name.

Do not chain them reflexively; each adds indirection. Use them when they clarify intent, not when a plain statement would do.

### Version and document public DSLs as APIs

A published DSL (library or module API) is a contract. Adding a required node, renaming a function, or changing a default breaks callers. Treat the DSL surface like any public API: deprecate before removing, add overloads for new parameters with defaults, and document the intended call shape.

## Common Traps

### Extension on `Any` polluting scope

`fun Any.myHelper()` appears as an available method on every type, cluttering autocomplete and risking conflicts. Scope it to the specific type, or make it a top-level function taking the value as a parameter.

### Member-vs-extension conflict

Defining an extension with the same signature as a member means the member always wins and the extension is unreachable. Check for member conflicts before adding extensions to your own classes.

### Over-nested DSL without `@DslMarker`

A nested builder DSL without `@DslMarker` lets inner lambdas call outer receivers, producing "added to the wrong parent" bugs. Always annotate DSL builders with a shared marker.

### DSL that requires every field

A builder that has no defaults is not simpler than a data class constructor. Provide defaults so the minimal call site is valid.

### Receiver ambiguity in multi-receiver contexts

Trying to have two receivers in scope at once leads to `this` ambiguity and workarounds. Pass additional context as a parameter instead.

### Extensions that hide business logic

Moving core domain logic into extension functions on entities scatters the domain model and makes the call path hard to follow. Keep domain logic in services or domain methods, not in a web of extensions.

### Scope function chains that obscure intent

`x?.let { it.run { this.apply { } } }` is write-only code. Use scope functions singly and only when they clarify; otherwise use plain statements.

## Self-Check

- For each extension function, is it orthogonal utility (appropriate) rather than core behavior that should be a member or a service?
- Have you checked that no extension conflicts with an existing member, and that extensions on `Any`/`Any?` are genuinely universal and distinctly named?
- For each DSL, did you write the desired call site first for several real examples and confirm the fluent form is materially clearer than constructors?
- Are all builder classes in a nested DSL annotated with a shared `@DslMarker` to prevent implicit outer-receiver calls?
- Is each receiver lambda focused on a single receiver, with additional context passed as parameters rather than combined?
- Are scope functions used to clarify intent, not chained reflexively where a plain statement would do?
- If the DSL is a public API, are changes (new required nodes, renames, default changes) versioned and documented with deprecation paths?
- Is core domain logic kept in services or domain methods rather than scattered across extension functions?
