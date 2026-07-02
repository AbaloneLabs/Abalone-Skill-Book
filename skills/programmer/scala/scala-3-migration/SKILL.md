---
name: scala_3_migration.md
description: Use when the agent is migrating a Scala 2 codebase to Scala 3, using given and using clauses, enums, extension methods, new control syntax, intersection and union types, porting implicits to Scala 3 givens, handling cross-compilation, configuring compiler flags, or diagnosing compatibility, binary, and source migration issues in Scala projects.
---

# Scala 3 Migration

Scala 3 is a significant language redesign, and migrating to it is not a version bump. The new syntax, the replacement of `implicit` with `given`/`using`, enums, extension methods, and the revised type system change how code reads and how the compiler resolves things, and the migration surface includes source compatibility, binary compatibility, macro compatibility, and ecosystem (library) compatibility. The judgment problem is not "how do I enable Scala 3" but deciding the migration strategy (big-bang vs. incremental cross-compilation), understanding which Scala 2 patterns translate directly and which require redesign, and sequencing the work so that the codebase compiles and the team can move in controlled steps rather than a long-lived broken branch.

The recurring failure mode is a team that flips the Scala version flag expecting "mostly the same syntax," then discovers that their heavy use of macros, advanced implicits, or library dependencies does not compile, and enters a months-long migration branch that diverges from main. The opposite failure is migrating syntax superficially (rewriting `implicit` to `given` mechanically) without adopting the cleaner patterns Scala 3 enables, leaving the codebase in a hybrid state that gets neither the simplicity of idiomatic Scala 3 nor the familiarity of Scala 2. Migration is a sequence of deliberate decisions about compatibility, tooling, and idiom adoption, and the right path depends on the codebase's reliance on macros, implicits, and binary-compatible dependencies.

## Core Rules

### Choose the migration strategy by the codebase's macro and implicit footprint

- **Cross-compilation (Scala 2.13 + 3.x from the same sources)**: suitable for libraries and codebases with moderate complexity. Requires source that compiles under both, using the `-Xsource:3` flag on Scala 2 and avoiding Scala-3-only syntax. Lets you publish for both and migrate consumers gradually.
- **Big-bang (move the whole module to Scala 3)**: suitable for applications (not published libraries) or modules with no external consumers. Faster to adopt new syntax, but requires all dependencies to have Scala 3 artifacts.

Assess macro usage first: Scala 2 macros (whitebox especially) do not port and must be rewritten using Scala 3 macros or metaprogramming, or replaced. Macro-heavy libraries are the most common migration blocker.

### Sequence the migration: dependencies, then build, then syntax, then idioms

A workable order:

1. **Dependencies**: confirm every dependency publishes a Scala 3 artifact (or a cross-built 2.13+3 one). Replace or fork those that do not.
2. **Build**: configure the build for Scala 3 (sbt `scalaVersion`), enable `-Xsource:3` on the Scala 2 side if cross-compiling.
3. **Syntax**: the Scala 2.13 compiler with `-Xsource:3` accepts much Scala 3 syntax, letting you migrate syntax incrementally and keep it compiling under both.
4. **Implicits to givens**: convert `implicit def`/`val` to `given` and `implicit` parameters to `using` in a focused pass, since this is the largest semantic change.
5. **Idioms**: adopt enums, extension methods, opaque types, and the new type system where they simplify, after the mechanical migration is stable.

Do not attempt all steps at once on a large codebase; do it module by module.

### Translate implicits deliberately, not mechanically

Scala 3 replaces `implicit` with `given` (definitions) and `using` (parameters), and changes resolution rules. Rules:

- `implicit def x: T = ...` becomes `given x: T = ...`; `implicit val` similarly.
- `implicit` parameters become `using` parameters; `implicitly[T]` becomes `summon[T]`.
- Scala 3 givens have different priority and import semantics; `import` brings givens into scope like before, but `export` and the new import selectors change resolution.
- Implicit conversions are now opt-in (`import scala.language.implicitConversions`) and discouraged; prefer `Conversion` or extension methods.

Mechanical conversion compiles but may not be idiomatic; review resolution after conversion, since the rules differ.

### Adopt enums, extension methods, and opaque types where they simplify

Scala 3 introduces first-class features that replace Scala 2 workarounds:

- **`enum`**: replaces sealed-trait-plus-case-objects for ADTs, including parameterized cases. Prefer for new ADTs.
- **Extension methods** (`extension (x: T)`): replace implicit classes for syntax extensions, with clearer scoping.
- **Opaque types** (`opaque type T = U`): replace value classes for zero-cost newtypes, without the value-class restrictions.

Adopt these after the mechanical migration; they are improvements, not requirements, and bulk-rewriting working code during migration adds risk.

### Handle binary compatibility deliberately

Scala 2 and Scala 3 artifacts are not binary-compatible: a Scala 2 application cannot use a Scala 3 library's compiled artifact and vice versa. Rules:

- For published libraries, cross-build (2.13 + 3) so consumers on either can depend on you.
- For applications, ensure all dependencies have Scala 3 versions before switching.
- The Scala 2.13 and Scala 3 compilers can consume each other's sources via TASTy (for Scala 3 reading Scala 2) in limited cases, but do not rely on binary interop; depend on cross-published artifacts.

### Configure compiler flags and the new syntax level

Scala 3 changes flag names and defaults. Rules:

- Review `-explain`, `-new-syntax`, `-indent` (significant indentation), and `-language` imports; adopt what fits the team.
- The `-Wunused` and `-deprecation` flags help find dead code and migration issues.
- Significant indentation is the new default style; braces remain available. Pick one style per codebase and be consistent.

### Migrate macros and metaprogramming explicitly

Scala 2 macros (def macros, whitebox macros, quasiquotes) do not compile under Scala 3. Rules:

- Identify all macro usage; each must be rewritten using Scala 3 metaprogramming (inline + quotes/splices, reflection) or replaced with a non-macro implementation.
- Whitebox macros (which generate types) have no direct equivalent; redesign around them.
- Macro-heavy libraries (some serialization, schema generation, logging) are common blockers; check for Scala 3 versions before migrating.

### Test and verify behavior, not just compilation

Scala 3 changes resolution and some semantics; code that compiles may behave differently. Rules:

- Run the full test suite after migration, including property and integration tests.
- Pay attention to implicit/given resolution order changes that may pick a different instance.
- Verify serialization, reflection, and runtime-generated code paths that the compiler cannot check.

## Common Traps

### Flipping the version flag expecting syntax parity

Scala 3 changes enough syntax and semantics that a flag flip breaks compilation in macros, advanced implicits, and library deps. Assess the surface first.

### Macro-heavy code with no Scala 3 path

Scala 2 macros do not port directly. Identify and plan macro rewrites before committing to migration.

### Mechanical implicit-to-given conversion without reviewing resolution

The conversion compiles but Scala 3's resolution rules differ; a different `given` may be selected. Test behavior, not just compilation.

### Cross-compiling without `-Xsource:3`

Without `-Xsource:3` on the Scala 2 side, the same sources will not accept Scala 3 syntax, breaking cross-compilation. Enable it to share syntax.

### Forgetting binary incompatibility between Scala 2 and 3

A Scala 2 app cannot consume a Scala 3 library artifact. Ensure cross-published artifacts for libraries, or full Scala 3 versions for app dependencies.

### Bulk-rewriting to new idioms during migration

Converting all sealed traits to enums and all implicit classes to extension methods during migration adds risk to a large change. Do the mechanical migration first, adopt idioms later.

### Ignoring compiler flag and syntax defaults

Scala 3 changes flag names and defaults (e.g., significant indentation). Review and configure them deliberately; defaults may not match team preference.

## Self-Check

- Has the migration strategy (cross-compilation vs. big-bang) been chosen based on the codebase's macro/implicit footprint and whether it is a published library or an application?
- Is the migration sequenced (dependencies, build, syntax, implicits-to-givens, idioms) rather than attempted all at once on a large codebase?
- Have implicits been translated to `given`/`using` deliberately, with resolution reviewed (not mechanically), and implicit conversions replaced or opted into explicitly?
- Are Scala 3 idioms (enums, extension methods, opaque types) adopted where they simplify, after the mechanical migration is stable rather than during it?
- Is binary compatibility handled (cross-published artifacts for libraries, all-dependencies-have-Scala-3 for applications), with no reliance on Scala 2/3 binary interop?
- Are compiler flags and syntax style (`-new-syntax`, significant indentation, `-language` imports) configured deliberately and consistently across the codebase?
- Have all Scala 2 macros been identified and a rewrite/replacement plan made, with whitebox macros redesigned since they have no direct equivalent?
- Does the full test suite (including property and integration tests) pass after migration, with attention to given-resolution order, serialization, and reflection paths?
