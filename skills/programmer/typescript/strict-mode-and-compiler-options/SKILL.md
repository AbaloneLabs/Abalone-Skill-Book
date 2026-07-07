---
name: typescript_strict_mode_and_compiler_options.md
description: Use when the agent is selecting or tuning individual TypeScript compiler options beyond the strict bundle — noUncheckedIndexedAccess, exactOptionalPropertyTypes, noPropertyAccessFromIndexSignature, alwaysStrict vs strict, module/moduleResolution/target/lib interplay, isolatedModules, verbatimModuleSyntax, incremental/project references, build mode, or deciding which options to enable on a library vs an application. Covers the granular compiler-option landscape, the interaction between module/target/lib, monorepo and project-reference configuration, and the tradeoffs of each strictness flag. Distinct from the strict-mode migration philosophy and declaration-file authoring, which are covered separately.
---

# Strict Mode And Compiler Options: Granular Configuration

Beyond the `strict` bundle lies a landscape of individual compiler options that each catch a different class of bug or control a different aspect of compilation: `noUncheckedIndexedAccess` makes array indexing honest, `exactOptionalPropertyTypes` distinguishes absent from `undefined`, `module`/`moduleResolution`/`target`/`lib` must agree with the runtime, and `isolatedModules`/`verbatimModuleSyntax` constrain what transpilers can safely do. The judgment problem is choosing the right *granular* options for a library versus an application, making the module/target/lib triple consistent with the real runtime, and configuring monorepos and project references so type-checking and builds are correct and fast.

Agents tend to leave the defaults, copy a `tsconfig` from another project without understanding it, set `target`/`module`/`lib` to mismatched values that cause "works in dev, fails in build" errors, or enable a strict flag globally and drown in errors. The harm appears as index-access bugs the compiler could have caught, optional-property ambiguity that causes runtime `undefined`, transpile errors when a bundler (which compiles file-by-file) rejects code that type-checks under whole-program mode, and monorepo builds that re-check everything or miss cross-package type errors. The real work is choosing each option deliberately, keeping the module/target/lib triple consistent, respecting transpiler constraints, and structuring project references for correct and incremental builds.

## Core Rules

### Choose Granular Strict Flags Beyond The `strict` Bundle

The `strict` bundle is a baseline, not a ceiling. Several flags outside it catch real bugs and are worth enabling on mature projects, each with a migration cost.

- **`noUncheckedIndexedAccess`**: `arr[i]` and `obj[key]` yield `T | undefined` instead of `T`, because the lookup may be out of range or missing. This catches a large class of off-by-one and missing-key bugs. It is noisy at first (every index access needs a check or assertion) but high-value. Strongly recommended for applications.
- **`exactOptionalPropertyTypes`**: distinguishes a property that is *absent* from one that is *present and set to `undefined`*. Without it, `{ a?: string }` accepts `{ a: undefined }`; with it, you must omit the property to mean "absent." Catches bugs where optional fields are unexpectedly `undefined`; changes API compatibility, so enable deliberately on libraries.
- **`noPropertyAccessFromIndexSignature`**: forces bracket access (`obj["key"]`) for index-signature properties, distinguishing known keys from dynamic ones. Prevents typos that silently hit the index signature.
- **`noFallthroughCasesInSwitch`, `noImplicitReturns`, `noImplicitOverride`, `noUnusedLocals`, `noUnusedParameters`**: catch common mistakes (missing `break`, missing `return`, missing `override`, dead code). Low cost, steady value.

Enable incrementally; each has a one-time migration cost. Track which are on and which are deferred as deliberate debt.

### Keep `module`, `moduleResolution`, `target`, And `lib` Consistent With The Runtime

These four options must describe the same reality, or you get code that type-checks but fails at runtime.

- **`target`**: the JS syntax the compiler emits (ES2020, ESNext, etc.). Must be supported by your oldest runtime, or by the transpiler/bundler that further downlevels it.
- **`module`**: the module system in the output (`commonjs`, `es2020`, `esnext`, `nodenext`, `preserve`). Must match how the output is consumed (Node CJS, Node ESM, a bundler).
- **`moduleResolution`**: how import specifiers resolve (`node`, `node16`/`nodenext`, `bundler`). Must match the actual resolver: `nodenext` for Node ESM (requires extensions in relative imports), `bundler` for code that goes through a bundler (allows extensionless imports and conditions).
- **`lib`**: the ambient type definitions (DOM, ES2020, etc.). Must match what the runtime provides. A `lib` that includes APIs the runtime lacks lets you type-check code that will crash; a `lib` that omits them causes false errors.

A common failure: `module: esnext` with `moduleResolution: node`, which type-checks extensionless ESM imports that Node ESM rejects. Match the triple to the real execution path, and revisit it when the runtime or bundler changes.

### Respect Transpiler Constraints: `isolatedModules` And `verbatimModuleSyntax`

Single-file transpilers (esbuild, swc, Babel, Vite) compile each file independently and cannot do whole-program type analysis. Two options constrain your code so these tools work.

- **`isolatedModules`**: ensures each file can be transpiled in isolation. It forbids constructs that need cross-file analysis: re-exporting a type without `export type`, `const enum` (whose inlining needs the definition), and some namespace patterns. Enable it whenever a fast transpiler is in the pipeline, even if `tsc` also type-checks.
- **`verbatimModuleSyntax`** (the stricter successor to `importsNotUsedAsValues`/`isolatedModules`' type-elision rules): requires explicit `import type`/`export type` for type-only imports, so the transpiler knows what to drop. It prevents the "value imported but only used as a type" runtime error in ESM.

If your build uses a bundler/transpiler (most projects do), enable `isolatedModules` at minimum and prefer `verbatimModuleSyntax` for clarity. Code that compiles only under whole-program `tsc` will break the transpiler.

### Configure Project References And Build Mode For Monorepos

In a monorepo or multi-package project, project references (`composite: true`, `references`) let `tsc` type-check incrementally and correctly across packages, reusing compiled declarations instead of re-checking everything.

- Each package gets its own `tsconfig` with `composite: true`, referencing its dependencies.
- `tsc --build` orchestrates the build in dependency order, rebuilding only what changed and using `.d.ts` outputs for downstream packages.
- This is dramatically faster than one giant `tsconfig` over the whole monorepo, and it enforces package boundaries (a package cannot use another's internals that are not exported).

Misconfigured references (circular references, missing `composite`, paths that bypass the declaration output) cause either "cannot find type definitions" errors or silent re-checking of everything. Set references to match the real dependency graph, and use `tsc --build` rather than ad-hoc per-package `tsc`.

### Separate Type-Checking From Transpilation Deliberately

Modern setups usually split roles: a fast transpiler (esbuild/swc) emits JS, and `tsc --noEmit` type-checks. Decide which tool does what.

- **Transpiler emits, `tsc` checks**: the common setup. `tsc` is configured with `noEmit` (or `emitDeclarationOnly` for libraries) and run in CI/IDE for type errors; the bundler emits the actual JS. Ensure the transpiler's target/module match `tsc`'s, or the emitted JS differs from what was type-checked.
- **`tsc` emits**: simpler, slower. Fine for small projects or libraries publishing declarations.
- **`emitDeclarationOnly`**: for libraries, `tsc` emits `.d.ts` (which the transpiler cannot), while the transpiler emits JS. Common library setup.

Run type-checking in CI (`tsc --noEmit`) so type errors fail the build even when the bundler would emit anyway. A build that emits but does not type-check ships type errors silently.

### Match Library Options To Consumer Expectations

Libraries face additional constraints because consumers depend on the emitted declarations and on the module format.

- Emit declarations (`declaration: true`) so consumers get types; for monorepo libraries, ensure project references emit them.
- Choose `module`/`moduleResolution` for what consumers will use (ESM-first is increasingly viable; dual packages need care).
- Be conservative with strict flags that change API shape: `exactOptionalPropertyTypes` changes whether consumers can pass `{ a: undefined }`, which can break them. Document the strictness your declarations assume.
- `target` for a library should be conservative (lower) so consumers' older runtimes are supported, unless you document a minimum runtime.

A library that enables `exactOptionalPropertyTypes` or `noUncheckedIndexedAccess` and emits declarations pushes those constraints onto consumers. Decide deliberately and document.

## Common Traps

### `module`/`moduleResolution` Mismatch With The Runtime

`module: esnext` + `moduleResolution: node` type-checks extensionless ESM imports that Node ESM rejects. Match the pair to the real resolver (`nodenext` for Node ESM, `bundler` for bundler pipelines).

### `target`/`lib` Including APIs The Runtime Lacks

A high `lib` lets you call `Array.prototype.at` or `Object.hasOwn` that an older runtime does not have, causing runtime crashes. Align `lib`/`target` with the actual runtime, or add polyfills.

### `isolatedModules` Off When A Transpiler Is Used

Without `isolatedModules`, code that type-checks under whole-program `tsc` (re-exported types, `const enum`) breaks the single-file transpiler. Enable it whenever a transpiler is in the build.

### Forgetting `import type` Under `verbatimModuleSyntax`

Type-only imports must use `import type` (or inline `import { type X }`) under `verbatimModuleSyntax`, or the emitted JS tries to import a non-existent value. Be explicit about type-only imports.

### Project References That Bypass Declarations

If a package imports another's source directly (via `paths`) instead of through its emitted declarations, `tsc --build` re-checks the source and breaks incremental builds. Wire references to consume declarations.

### `exactOptionalPropertyTypes` Breaking Consumers

Enabling it on a library changes whether consumers can pass `{ a: undefined }`, which can break downstream code. Enable deliberately on libraries and document the assumption.

### `noUncheckedIndexedAccess` Noise Defeating Adoption

The flag makes every index access `T | undefined`, which is noisy; teams sometimes disable it to avoid the churn. Adopt it incrementally and accept the checks, rather than disabling a high-value flag for convenience.

### Type-Check Not Run In CI

A bundler emits JS regardless of type errors; if CI only runs the bundler, type errors do not fail the build. Run `tsc --noEmit` in CI.

## Self-Check

- [ ] Granular strict flags (`noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noPropertyAccessFromIndexSignature`, `noImplicitReturns`, etc.) are enabled where the project is mature enough, with deferred flags tracked as debt.
- [ ] `module`, `moduleResolution`, `target`, and `lib` are mutually consistent and match the actual runtime/bundler (e.g., `nodenext` for Node ESM, `bundler` for bundler pipelines).
- [ ] `isolatedModules` (and preferably `verbatimModuleSyntax`) is enabled whenever a single-file transpiler is in the build; type-only imports use `import type`.
- [ ] Monorepo/multi-package projects use project references (`composite`, `references`) and `tsc --build`, with references matching the real dependency graph.
- [ ] Type-checking runs in CI (`tsc --noEmit` or `tsc --build`), independent of the transpiler that emits JS.
- [ ] The roles of `tsc` (type-check, emit declarations) and the transpiler (emit JS) are explicit and their target/module settings agree.
- [ ] Libraries emit declarations (`declaration: true`) and choose strict flags that affect API shape (`exactOptionalPropertyTypes`) deliberately, documented for consumers.
- [ ] `lib` does not include APIs the runtime lacks without a corresponding polyfill.
- [ ] `noUncheckedIndexedAccess` noise is handled by checks/assertions, not by disabling the flag.
- [ ] The `tsconfig` is understood option-by-option, not copied from another project without review.
