---
name: typescript_module_resolution_and_path_aliases.md
description: Use when the agent is configuring TypeScript module resolution, choosing moduleResolution (node, node16, bundler, nodenext), setting up path aliases (paths, baseUrl) in tsconfig, mapping bare imports to source via bundler aliases (vite, webpack, tsconfig-paths), reasoning about relative vs absolute imports, monorepo package imports and project references, handling extension resolution (.js vs .tsx), debugging "Cannot find module", circular imports, or runtime-vs-compile-time path mismatches, or migrating from classic node resolution to modern resolution. Covers moduleResolution strategy selection, path aliases and their runtime counterpart, project references, extension and import-kind rules, and the tradeoff between ergonomic aliases and portability/tooling friction.
---

# Module Resolution And Path Aliases

TypeScript must resolve every import to a file at compile time, and the runtime (Node or a bundler) must resolve the same import to the same file at runtime. These two resolution processes are governed by different settings and can drift, producing "works in the editor, fails at runtime" bugs. The judgment problem is choosing a `moduleResolution` strategy that matches the runtime, configuring path aliases with their runtime counterparts, and keeping compile-time and runtime resolution in sync across tools.

Agents tend to set `paths` aliases without a bundler/runtime equivalent (so the build fails), mix `moduleResolution` strategies that disagree, use relative imports that break on file moves, or hit "Cannot find module" because of an extension or casing mismatch. The harm appears as code that type-checks but crashes at runtime, imports that resolve differently in different tools, circular import surprises, and monorepo packages that cannot find each other. The real work is aligning `moduleResolution` with the actual runtime, pairing every path alias with a runtime resolver, and understanding the extension and import-kind rules of the chosen strategy.

## Core Rules

### Choose `moduleResolution` To Match Your Runtime And Tooling

`moduleResolution` controls how TypeScript resolves imports. The choice must match how the code will actually run or be bundled.

- **`node` (classic Node CJS resolution)**: the legacy default; resolves like Node's CommonJS algorithm. Does not understand `package.json` `exports`, enforces fewer rules. Use only for older CJS projects.
- **`node16` / `nodenext`**: modern Node resolution; understands `exports`, requires correct file extensions in some cases, distinguishes ESM from CJS rules. The correct choice for code that runs in modern Node.
- **`bundler`**: for code that runs through a bundler (vite, webpack, esbuild, turbopack). Relaxes Node-specific rules (no mandatory extensions), supports `exports`, and matches how bundlers resolve. The right choice for application code processed by a bundler.

The mistake is using `node` for a bundler project (missing `exports` support, wrong rules) or `bundler` for code that runs directly in Node (runtime resolves differently than the editor). Match the strategy to the actual consumer of the code.

### Pair Every Path Alias With A Runtime Resolver

`paths` in `tsconfig` maps a bare import prefix (`@/components/Button`) to a filesystem location, but this mapping exists only at compile time. At runtime, Node or the bundler knows nothing about `tsconfig.paths` unless you configure it.

- **Bundler projects**: configure the bundler's alias (vite `resolve.alias`, webpack `resolve.alias`, esbuild `--alias`) to mirror the `tsconfig` `paths`.
- **Node projects**: use `tsconfig-paths` (for ts-node/tsx) or a runtime loader to register the alias mapping.
- **Tests**: ensure the test runner (jest `moduleNameMapper`, vitest `resolve.alias`) mirrors the aliases.

Without the runtime counterpart, imports type-check in the editor but fail at runtime with "Cannot find module". Every alias must exist in both worlds.

### Understand Relative vs Bare Imports And When Each Applies

- **Relative imports** (`./foo`, `../bar`): resolved relative to the importing file's location. Robust to module resolution settings but brittle to file moves (a deep relative path breaks when a file relocates).
- **Bare imports** (`react`, `@/components/Button`, `my-monorepo-package`): resolved via `node_modules` lookup or `paths`/`exports`. Stable across file moves but require correct resolution configuration.

Path aliases (`@/...`) exist to make deep imports readable and move-resilient. Use them for cross-directory imports within a project; use relative imports for nearby siblings where the relative path is short and stable.

### Get File Extensions Right For The Resolution Strategy

Different strategies have different extension rules, and getting this wrong causes "Cannot find module".

- **`node16`/`nodenext` with ESM**: requires explicit extensions for relative imports in some cases (`./foo.js` even when the source is `foo.ts`), because ESM resolution does not append extensions. This is a frequent surprise.
- **`bundler` and `node` (CJS)**: append extensions automatically, so `./foo` resolves to `foo.ts`/`foo.tsx`.
- **Casing**: imports are case-sensitive on Linux but case-insensitive on macOS/Windows. An import written as `./Button` when the file is `button.tsx` works on macOS and fails on the Linux CI. Always match the filesystem casing exactly.

### Use Project References For Large Codebases And Monorepos

`tsconfig` project references let a large codebase be split into sub-projects that compile incrementally with clear dependencies. In a monorepo, each package has its own `tsconfig` referencing the others.

- Project references speed up type-checking (only rebuild changed projects) and enforce dependency direction (a referenced project cannot import back).
- Each referenced project must have `composite: true` and a clear root.
- The referencing `tsconfig` lists `references` to the sub-projects.

In monorepos, prefer packages that import each other by name (with the bundler/workspace resolving to source) over deep relative imports across package boundaries, so each package has a clean public surface.

### Handle `package.json` `exports` For Library Entry Points

When consuming or publishing a library, the `exports` field controls which files are reachable. TypeScript respects `exports` under `node16`/`nodenext`/`bundler`.

- A library must declare its entry points in `exports` (with a `types` condition first) for consumers to resolve them.
- Deep imports into a package work only if the package's `exports` exposes those subpaths; otherwise they fail under modern resolution (which is the intended encapsulation).
- If you maintain the library and the consumer, ensure the `exports` map exposes everything the consumer imports.

### Avoid And Diagnose Circular Imports

Circular imports (A imports B, B imports A) can cause subtle initialization-order bugs, especially when one module uses a value from the other at module-evaluation time. TypeScript does not warn about cycles by default.

- Structure modules to avoid cycles: extract shared code to a third module that both depend on.
- Use lazy imports (dynamic `import()`) where a cycle is unavoidable and the value is only needed at call time, not at module load.
- Tools (madge, eslint-plugin-import) detect cycles; run them in CI for large codebases.

## Common Traps

### `paths` Alias Without Runtime Counterpart

The editor resolves `@/foo`, but Node/the bundler does not, causing a runtime "Cannot find module". Mirror aliases in the bundler/test runner.

### Wrong `moduleResolution` For The Runtime

Using `node` for a bundler project misses `exports`; using `bundler` for Node-only code diverges from runtime resolution. Match the strategy to the consumer.

### Missing Extensions Under `node16`/`nodenext`

ESM resolution requires explicit extensions for relative imports. Omitting them fails resolution even though the file exists.

### Casing Mismatch Failing On Linux CI

`import { X } from './Button'` when the file is `button.tsx` works on macOS, fails on Linux. Match filesystem casing exactly.

### Relative Imports Breaking On File Moves

Deep relative imports (`../../../utils/foo`) break when a file moves. Use path aliases for cross-directory imports.

### Circular Import Initialization Bug

A and B importing each other can produce `undefined` at module-evaluation time if one uses the other's value eagerly. Restructure or use dynamic import.

### Deep Import Into A Package Not In Its `exports`

Importing `lib/dist/internal` fails under modern resolution because `exports` does not expose it. Import only exposed entry points.

### Editor Resolves But Build Does Not (Tooling Divergence)

Different tools (editor, bundler, test runner) resolving imports differently because each has its own alias config. Keep all alias configs in sync.

## Self-Check

- [ ] `moduleResolution` matches the runtime: `bundler` for bundler-processed code, `node16`/`nodenext` for modern Node, `node` only for legacy CJS.
- [ ] Every `tsconfig` `paths` alias has a matching runtime resolver (bundler alias, `tsconfig-paths`, test runner `moduleNameMapper`).
- [ ] Relative imports are used for nearby siblings; path aliases for cross-directory imports to stay move-resilient.
- [ ] File extensions follow the resolution strategy (explicit extensions under `node16`/`nodenext` ESM), and import casing matches the filesystem exactly.
- [ ] Large codebases and monorepos use project references with `composite: true` and clean package boundaries rather than deep cross-package relative imports.
- [ ] Library entry points are declared in `exports` with a `types` condition first, and consumers import only exposed paths.
- [ ] Circular imports are avoided via shared extraction or dynamic import, and a cycle detector runs in CI.
- [ ] Alias configuration is kept in sync across the editor, bundler, and test runner so resolution does not diverge between tools.
