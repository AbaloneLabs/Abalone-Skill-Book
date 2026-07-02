---
name: typescript_strict_mode_and_tsconfig.md
description: Use when the agent is configuring or migrating a TypeScript project's tsconfig, enabling or interpreting strict options (strict, noImplicitAny, strictNullChecks, noUncheckedIndexedAccess, exactOptionalPropertyTypes), managing any/unknown, writing .d.ts declaration files for libraries, deciding whether a type error is a real bug or a config gap, or planning a gradual migration from loose to strict typing. Covers tsconfig compiler options, strictness tradeoffs, declaration file authoring, and the trap of treating type errors as noise rather than signals.
---

# Strict Mode And tsconfig

TypeScript's value depends almost entirely on which compiler options are enabled. The same code under `strict: false` and `strict: true` is a different language: one permits implicit `any`, unsound null handling, and unchecked index access; the other catches those classes of bugs at compile time. The judgment problem is choosing a strictness configuration that matches the project's maturity and risk tolerance, migrating existing code without losing the safety net, and treating type errors as signals rather than noise to suppress.

Agents tend to leave defaults, reach for `any` or `// @ts-ignore` the moment a type error appears, or write declaration files that declare more than the runtime provides. The harm is structural: a project that "uses TypeScript" but with `noImplicitAny` off gets almost none of the safety benefit, a `.d.ts` that lies about a module's exports causes confusing runtime errors downstream, and a migration that disables strict checks module-by-module often never re-enables them. The real work is configuring strictness deliberately, understanding what each option actually checks, and reserving escape hatches for genuine needs.

## Core Rules

### Enable `strict` And Understand What It Bundles

`strict: true` is a family of checks, not one. It enables at least `noImplicitAny`, `strictNullChecks`, `strictFunctionTypes`, `strictBindCallApply`, `strictPropertyInitialization`, `noImplicitThis`, `useUnknownInCatchVariables`, and `alwaysStrict`. New strict flags are added over time, so enabling `strict` future-proofs the config.

- For new projects, start with `strict: true`. The cost of fixing types early is far lower than retrofitting safety later.
- For existing projects, enabling `strict` wholesale may surface hundreds of errors. Plan a migration (below) rather than disabling the flag.
- Know which sub-flags matter most for your code: `strictNullChecks` (the single highest-value flag) and `noImplicitAny` catch the most bugs.

Do not run a "strict TypeScript" project with `strict: false`; it is TypeScript in syntax only.

### Treat `strictNullChecks` As Non-Negotiable

Without `strictNullChecks`, `null` and `undefined` are assignable to every type, so the compiler cannot tell you where a `null` access might crash. With it, a type `string` cannot be `null`; you must opt in with `string | null`. This single flag eliminates a large class of runtime `TypeError: Cannot read properties of null` bugs.

- If you can enable only one strict flag, enable this one.
- Model absence explicitly with `T | null` (or `T | undefined`), and narrow before access.
- Do not defeat it with non-null assertions (`x!`) everywhere; each `!` is an unchecked claim that the value is non-null.

### Manage `any`, `unknown`, And `noImplicitAny` Deliberately

`any` disables type checking for a position and propagates. `unknown` is the safe top type: it accepts anything but forces narrowing before use. `noImplicitAny` makes implicit `any` (untyped parameters, untyped variables) an error, which is where most type erosion starts.

- Keep `noImplicitAny` on; fix the annotations rather than disabling it.
- Replace `any` with `unknown` when you genuinely accept unknown input, then narrow or validate.
- Reserve explicit `any` for interop with untyped JS where a precise type is impractical, and contain it — do not let it flow through the codebase.
- Track `any` usage; consider lint rules (`@typescript-eslint/no-explicit-any`, `no-unsafe-*`) to surface it.

### Consider The High-Value Extra Flags Beyond `strict`

Several flags outside the `strict` bundle catch important bugs and are worth enabling on mature projects:

- `noUncheckedIndexedAccess`: indexing an array (`xs[i]`) yields `T | undefined`, because the index may be out of range. Catches a huge class of off-by-one and lookup bugs. Noisy at first, high-value once adopted.
- `exactOptionalPropertyTypes`: distinguishes "property absent" from "property present and `undefined`", catching bugs where optional fields are set to `undefined` unexpectedly.
- `noImplicitOverride`, `noFallthroughCasesInSwitch`, `noUnusedLocals`, `noUnusedParameters`, `noImplicitReturns`: catch common mistakes at compile time.
- `forceConsistentCasingInFileNames`: avoids cross-platform path bugs.

Enable these incrementally; each has a migration cost, but each catches a real bug class.

### Plan Gradual Migration For Existing Codebases

Migrating a loose codebase to strict is a project, not an edit. Approach it deliberately:

1. Enable `strict` in a CI-only "type check" build first, without failing the main build, to see the error count and hotspots.
2. Fix module by module, using per-file overrides (`// @ts-nocheck` or a separate tsconfig) as temporary scaffolding — and track them as debt to remove.
3. Prioritize `strictNullChecks` and `noImplicitAny`; they give the most safety per unit of effort.
4. Avoid the trap of disabling a flag globally to avoid fixing one module; scope the relaxation narrowly and time-box it.

A migration that disables strict checks globally "for now" usually stays that way. Make the relaxation local and visible.

### Write Declaration Files That Match Reality

A `.d.ts` declaration file is a contract for consumers of a library. If it declares exports, types, or optionality the runtime does not provide, consumers get confusing runtime errors that the compiler said were impossible.

- Generate declarations from source (`declaration: true`) rather than hand-writing them, so they stay in sync.
- When hand-writing (for JS libraries, ambient modules, or complex module shapes), verify every declared export exists at runtime.
- Model optionality honestly: a field that may be `undefined` at runtime must be optional or `| undefined` in the declaration.
- Use `@types` packages for popular JS libraries rather than writing your own; they are maintained and tested.
- For modules with no types, prefer a minimal accurate `declare module` over a permissive `any`-filled one.

### Separate Type Errors From Configuration Gaps

When a type error appears, decide whether it reveals a real bug or a configuration gap. The two need different responses:

- **Real bug**: the code does not match its intended contract. Fix the code.
- **Config gap**: the types are correct but the compiler cannot see it (missing declaration, wrong `moduleResolution`, library without types). Fix the config or add the declaration.
- **Suppression**: a last resort. Use `// @ts-expect-error` (which errors if the suppression becomes unnecessary) over `// @ts-ignore` (which silently stays). Each suppression should name the reason.

Treating every type error as a config gap to suppress defeats the purpose of the type system. Investigate first; suppress narrowly.

## Common Traps

### `strict: false` With TypeScript Syntax

Using TypeScript only for types-as-comments, with strictness off, gives a false sense of safety. The compiler approves almost everything, including null-dereferences and implicit-any chains. Either enable strict or acknowledge the project is untyped.

### Non-Null Assertion (`!`) Everywhere

`x!` asserts non-null without checking. A scatter of `!` reproduces the bugs `strictNullChecks` was meant to catch. Narrow with checks; reserve `!` for places where a runtime check is genuinely redundant and documented.

### `// @ts-ignore` Instead Of `// @ts-expect-error`

`@ts-ignore` silently persists even after the error is fixed, hiding the fact that the suppression is now dead. `@ts-expect-error` errors if there is no error, forcing removal when no longer needed. Prefer the latter.

### Disabling A Flag Globally To Fix One File

Turning off `strictNullChecks` for the whole project to avoid fixing one module throws away the safety net everywhere. Scope the relaxation to the file or region and time-box it.

### Declaration Files That Lie

A `.d.ts` that declares an optional field as required, or an export that does not exist, causes consumers to trust a contract the runtime breaks. Generate from source where possible; verify hand-written declarations against runtime behavior.

### `skipLibCheck` Hiding Real Errors

`skipLibCheck` skips type checking of `.d.ts` files, speeding builds but hiding errors in declarations (including your own). It is reasonable for build speed on large projects, but do not rely on it to excuse sloppy declarations; check them in isolation.

### Treating `unknown` As `any`

`unknown` is safe only if you narrow it before use. Casting `unknown` straight to a precise type (`x as User`) reproduces `any`'s unsafety. Validate or narrow `unknown` through a real check (see the narrowing skill).

### Ignoring `moduleResolution` And `target` Mismatches

`moduleResolution` must match how modules are actually resolved (Node, bundler, nodenext), and `target`/`lib` must match the runtime environment. Mismatches cause "works in dev, fails in build" errors and missing/extra APIs. Set them deliberately, not by default.

## Self-Check

- [ ] `strict: true` is enabled; the project is not TypeScript-in-syntax-only.
- [ ] `strictNullChecks` is on, and absence is modeled with `T | null`/`T | undefined` rather than defeated by scattered `!` assertions.
- [ ] `noImplicitAny` is on; implicit `any` is fixed with annotations, and explicit `any` is contained and tracked.
- [ ] High-value extra flags (`noUncheckedIndexedAccess`, `exactOptionalPropertyTypes`, `noImplicitReturns`, etc.) are enabled where the project is mature enough, with migration debt tracked.
- [ ] Existing-codebase migration is incremental and scoped; no global flag relaxation "for now" without a removal plan.
- [ ] Declaration files are generated from source where possible; hand-written `.d.ts` files match runtime exports and optionality.
- [ ] Suppressions use `// @ts-expect-error` with a reason, not bare `// @ts-ignore`; each suppression is necessary and reviewed.
- [ ] `unknown` is narrowed or validated before use, not cast directly to a precise type.
- [ ] `moduleResolution`, `target`, and `lib` match the actual runtime and module system.
- [ ] Type errors are investigated as potential bugs before being treated as configuration gaps.
