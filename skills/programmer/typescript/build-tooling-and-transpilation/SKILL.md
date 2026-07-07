---
name: typescript_build_tooling_and_transpilation.md
description: Use when the agent is configuring or debugging the TypeScript-to-JavaScript build pipeline, choosing between tsc, SWC, esbuild, Babel, Vite, Turbopack, or tsup, setting target/lib/module/downleveling, isolatedModules and verbatimModuleSyntax, const enum and namespace erasure, source maps, declaration emit and project references, monorepo build ordering, watch mode, or is diagnosing "the build output differs from the IDE", "type information is lost in the bundle", "const enum broke downstream", "decorator metadata missing", or "the library ships the wrong module format". Covers the split between type-checking and transpilation, tool selection by goal, downleveling semantics, and the contract between source and emitted output.
---

# Build Tooling And Transpilation In TypeScript

A TypeScript project has two independent jobs that are easy to conflate: type-checking (does the program type-correct) and transpilation (turning TS syntax into runnable JS). The TypeScript compiler (`tsc`) does both, but it is a slow transpiler, so the modern ecosystem splits them: a fast single-threaded transpiler (SWC, esbuild) strips types and downlevels syntax, while `tsc` (or a separate type-check step) validates types. This split is the source of most build bugs. The transpiler does not understand types — it erases them — so it cannot catch type errors, it cannot honor `const enum` the way `tsc` does, it cannot always reproduce `tsc`'s downleveling for decorators or parameter properties, and it can produce output that diverges from what the IDE (which uses `tsc` semantics) suggests. The judgment problem is to choose the tool by the goal (dev server, production bundle, library build, type-check gate), to configure the source so it survives fast transpilation (`isolatedModules`, `verbatimModuleSyntax`, no cross-file `const enum`), and to keep the type-check step as a separate, enforced gate rather than assuming the bundler validates anything.

Agents commonly set up Vite/esbuild for dev, never run `tsc`, and ship type errors that the bundler happily erased; or they enable a transpiler-only build and hit `const enum` or decorator surprises because those constructs need `tsc`'s whole-program understanding. The remedy is to treat type-checking and transpilation as separate steps with explicit ownership, to write source that is robust to per-file transpilation, and to verify the emitted output's module format, target, and declarations against the consumers' expectations.

## Core Rules

### Separate Type-Checking From Transpilation, And Enforce Both

Decide which tool does each job. Typical splits: `tsc --noEmit` for type-checking (run in CI and as an IDE gate), plus a fast transpiler (esbuild/SWC via Vite/tsup) for emitting JS; or `tsc` for both in a small project. The key rule: never assume the bundler type-checks. esbuild and SWC strip types without validating them, so a build that "succeeds" may be full of type errors. Run `tsc --noEmit` (or `vue-tsc`, `tsc -p` with project references) as a distinct CI step.

- Dev: fast transpiler (esbuild/SWC) for instant rebuilds; type errors surface in the IDE overlay, not the build.
- CI: `tsc --noEmit` as a required gate, independent of the bundler.
- Production library build: `tsc` (for declarations) plus a transpiler (for JS), or a tool that orchestrates both (tsup, unbuild).

### Write Source That Survives Per-File Transpilation (isolatedModules)

Fast transpilers process one file at a time and cannot see across files, so certain TypeScript constructs that need whole-program analysis break or behave differently. Enable `isolatedModules` (and the related `verbatimModuleSyntax`) to make the source robust, and avoid the constructs that require cross-file knowledge.

- `const enum`: under `tsc`, `const enum` members are inlined across files; under a per-file transpiler they are not, and re-exporting a `const enum` from another module breaks downstream. Avoid `const enum` in libraries; use plain objects (`as const`) or unions instead.
- Type-only re-exports: a transpiler cannot tell whether a re-exported name is a type or a value without the source, so `verbatimModuleSyntax` (or explicit `export type`) is required to avoid emitting imports of erased types.
- `isolatedModules` enforces that each file is independently transpilable; treat its errors as real, not pedantic.

### Match The Emitted Module Format To Consumers

The module format of emitted JS (ESM, CommonJS, UMD) must match what consumers expect, and mixing them produces the canonical interop errors ("Cannot use import statement outside a module", "require is not defined", default/named export surprises). For a library, configure `package.json` `exports` with separate `import` (ESM) and `require` (CJS) conditions and emit both; for an application, emit the format the runtime (browser ESM, Node CJS-or-ESM, bundler) expects.

- Set `module`/`moduleResolution` consistently with the runtime: `NodeNext`/`Node16` for modern Node, `Bundler` for bundler-resolved apps, `ESNext`/`Preserve` for ESM-only.
- For dual-package libraries, emit both formats and use the `exports` field to route; beware the dual-package hazard where the same module loaded via ESM and CJS produces two distinct instances (two singletons, two `instanceof` failures).

### Choose target/lib By The Oldest Runtime, Not By Default

`target` controls which syntax features are downleveled (e.g., `async`/`await` to generators for old targets) and which are emitted as-is. `lib` controls which type definitions are available (`DOM`, `ES2022`). Set both to the oldest runtime that must run the code, and let modern runtimes avoid unnecessary downleveling. Over-downleveling (targeting ES5 when only modern browsers run the code) bloats output and uses slower polyfills; under-downleveling ships syntax old runtimes cannot parse.

- For browsers, target by the browserslist/oldest supported browser; for Node, target the Node version (Node 18+ supports modern syntax natively).
- Include the right `lib`: `DOM` for browser code, `ES20xx` for the runtime's feature set; omit `DOM` for pure Node libraries to avoid accidentally using browser globals.

### Emit Declarations For Libraries, And Use Project References For Monorepos

A library must ship `.d.ts` declarations (`declaration: true`) so consumers get types. In a monorepo, project references (`composite: true`, `references`) let `tsc` build packages in dependency order and share declarations without publishing. Misconfigured references produce "cannot find module" errors between packages or stale declarations after edits.

- For libraries: `declaration: true`, and `declarationMap` for debugging into source.
- For monorepos: `composite`/`references` with a root `tsconfig` that orchestrates; rebuild with `tsc --build` so ordering and incremental caches are correct.
- Ensure the published package's `types`/`exports.types` points at the emitted declarations.

### Verify Source Maps And Decorator/Parameter-Property Emit

Source maps (`sourceMap: true`) are required for debugging into original TS and for stack traces that point at source. Some transpilers need explicit configuration to produce source maps and to handle TypeScript-specific emit (decorators, `emitDecoratorMetadata`, parameter properties) — esbuild and SWC support these with flags, but their semantics can differ from `tsc`. If the project relies on decorator metadata (DI), confirm the chosen transpiler emits it; if not, use `tsc` for emit or a tool that does.

## Common Traps

### Bundler Builds Without A Type-Check Gate

esbuild/SWC strip types without checking them; a "successful" build ships type errors. Always run `tsc --noEmit` in CI.

### const enum Breaking Under Per-File Transpilation

`const enum` inlined by `tsc` is not inlined by esbuild; re-exporting across files breaks. Avoid `const enum` in shared code; use `as const` objects or unions.

### verbatimModuleSyntax/isolatedModules Violations

Re-exporting a type without `export type`, or importing a type that is also used as a value, fails under per-file transpilation. Enable the flags and use explicit `type` modifiers.

### Wrong Module Format For Consumers

Shipping ESM to a CJS consumer (or vice versa) produces interop errors. Configure `exports` with both conditions for libraries; match `module`/`moduleResolution` to the runtime.

### Dual-Package Singleton Duplication

A library loaded once via ESM and once via CJS yields two instances, breaking `instanceof` and singletons. Route consistently or document the hazard.

### Over- Or Under-Downleveling

Targeting ES5 for a modern-only app bloats output; targeting ESNext for an old runtime ships unparseable syntax. Set `target` by the oldest supported runtime.

### Missing Declarations Or Stale Project References

A library without `declaration: true` ships no types; a monorepo with stale references produces phantom "cannot find module" errors. Use `tsc --build` for ordered, incremental rebuilds.

### Decorator/Metadata Emit Differing Across Transpilers

esbuild/SWC decorator emit and `emitDecoratorMetadata` differ from `tsc`; DI-by-type can break. Confirm the transpiler emits the needed metadata or use `tsc` for emit.

## Self-Check

- [ ] Type-checking and transpilation are separate, both enforced: a fast transpiler builds JS, and `tsc --noEmit` runs as a required CI gate (not assumed by the bundler).
- [ ] Source is robust to per-file transpilation: `isolatedModules`/`verbatimModuleSyntax` are enabled, `const enum` is avoided in shared code, and type-only re-exports use `export type`.
- [ ] Emitted module format matches consumers: libraries configure `exports` with ESM and CJS conditions, applications set `module`/`moduleResolution` to the runtime, and the dual-package singleton hazard is documented or avoided.
- [ ] `target` and `lib` are set by the oldest supported runtime (browserslist/Node version), avoiding both over-downleveling bloat and under-downleveling unparseable syntax, and `DOM` is omitted for non-browser code.
- [ ] Libraries emit declarations (`declaration: true`, `declarationMap`), and monorepos use `composite`/`references` rebuilt with `tsc --build` so ordering and incremental caches are correct.
- [ ] Source maps are emitted for debugging and stack traces, and TypeScript-specific emit (decorators, `emitDecoratorMetadata`, parameter properties) is confirmed to work under the chosen transpiler or routed to `tsc`.
- [ ] The contract between source and emitted output has been verified: a downstream consumer gets the expected module format, declarations, and runtime behavior, not just a successful local build.
- [ ] The build pipeline has been considered under CI, watch mode, monorepo ordering, and consumer expectations, and produces consistent results across them.
