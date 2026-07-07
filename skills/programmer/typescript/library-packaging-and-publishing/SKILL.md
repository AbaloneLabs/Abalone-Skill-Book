---
name: typescript_library_packaging_and_publishing.md
description: Use when the agent is building or publishing a TypeScript library, configuring package.json exports/imports/main/module/types fields, choosing module format (ESM/CJS/UMD) and build targets, deciding what to ship (source, declarations, sourcemaps, transpiled), setting up tsconfig for declaration emit and project references, designing the public type surface and breaking-change policy, handling dual-package (ESM+CJS) hazards, tree-shaking and sideEffects, peer dependencies, or debugging "Could not find a declaration file", dual-package import errors, or types missing from the published package. Covers package.json exports maps, ESM/CJS dual publishing, declaration file shipping, public API surface design, peer vs runtime dependencies, and the tradeoff between broad compatibility and a clean, tree-shakable package.
---

# Library Packaging And Publishing

Publishing a TypeScript library is a different problem from building an application. A library must compile cleanly under consumers' varied configurations, ship correct type declarations, expose a deliberate public surface, support the module formats its consumers use, and avoid pulling in unwanted dependencies. The judgment problem is configuring the package so that consumers get working runtime code, accurate types, tree-shakable imports, and a stable API — across Node, bundlers, ESM, and CommonJS.

Agents tend to ship the source instead of built output, omit declaration files, set only `main` and forget `module`/`types`/`exports`, declare runtime dependencies that should be peer, or create dual ESM+CJS packages that break under Node's strict resolution. The harm appears as "Could not find a declaration file" errors for consumers, runtime crashes from dual-package instance splits, bloated bundles because `sideEffects` was not set, and breaking changes shipped as patch releases. The real work is building and shipping deliberate artifacts, configuring the `exports` map, marking peer dependencies, and treating the public type surface as a versioned contract.

## Core Rules

### Ship Built Artifacts, Not Your Source, Unless You Specifically Support Source

Consumers should not have to compile your TypeScript. Build the library to JavaScript (ESM and/or CJS) and ship `.d.ts` declaration files alongside.

- Configure `tsconfig` with `"declaration": true` (and `"declarationMap"` for debugging) to emit `.d.ts`.
- Build to the formats you support (ESM via a modern build, CJS for older consumers), and point `package.json` fields at them.
- Shipping source is appropriate only for libraries that explicitly support source consumption (e.g., via a bundler that recompiles), and even then, ship declarations for type-only consumers.

The default is: build, emit declarations, ship the artifacts.

### Configure The `exports` Map To Define Entry Points

The `exports` field in `package.json` is the modern way to define a package's public entry points and the conditions under which each is used. It supersedes `main` for capable resolvers.

- Define `import` (ESM), `require` (CJS), and `types` conditions: `"exports": { ".": { "types": "./dist/index.d.ts", "import": "./dist/index.mjs", "require": "./dist/index.cjs" } }`.
- The `types` condition must come first for TypeScript to resolve types correctly under modern resolution (`moduleResolution: bundler`/`node16`).
- Use subpath exports (`./utils`, `./server`) to expose multiple entry points so consumers can import submodules without reaching into internal paths.
- Encapsulate internals: without an `exports` entry, deep imports into your package are discouraged (and blocked under modern resolution).

Set `main`, `module`, and `types` as fallbacks for older tooling, but treat `exports` as the source of truth.

### Mark Peer Dependencies For Singletons And Ecosystem Contracts

A `peerDependency` declares that your library expects the consumer to provide a dependency at a compatible version, rather than bundling its own copy. This is critical for singletons and ecosystem contracts.

- React, Vue, and other UI runtimes must be peer dependencies: if your library bundles its own copy, hooks and context break because there are two runtimes.
- A styling or state library that the consumer also uses directly should be peer, so there is one instance.
- Use a regular `dependency` for internal utilities the consumer never sees directly.

Getting this wrong causes the "invalid hook call" / "multiple instances" class of bugs. Peer for shared singletons and ecosystem contracts; regular dependency for private internals.

### Design The Public Type Surface As A Versioned Contract

The types you export are part of your semver contract. Changing them can break consumers even if the runtime behavior is unchanged.

- Export only what consumers should use. Internal types should not be re-exported.
- Be conservative with exported type names: renaming an exported type is a breaking change.
- Use `exports` to expose a curated set of entry points; do not let consumers import deep internal paths that you then refactor.
- Document the public API and treat additions as minor, removals/renames as major.

A common mistake is exporting everything and then being unable to refactor internals without breaking consumers. Curate the surface.

### Handle Dual ESM+CJS Publishing Deliberately (Or Pick One)

Supporting both ESM and CJS lets a library serve all consumers, but dual-package publishing has hazards.

- Node treats ESM and CJS imports of the same package as potentially two separate module instances, which breaks singletons (two copies of state). Use dual-package hazard mitigations (re-export from a common CJS module, or use a single neutral format).
- `package.json` `exports` with both `import` and `require` conditions is the standard dual setup, but you must ensure the two entry points share state (e.g., the CJS build re-exports the ESM build, or vice versa, via a neutral internal module).
- Alternatively, publish ESM-only (modern) or CJS-only (legacy) if your consumer base allows it, avoiding the hazard entirely.

If you do not need both, pick one. If you need both, understand the dual-instance hazard and mitigate it.

### Set `sideEffects` For Tree-Shaking

Bundlers tree-shake (eliminate unused exports) only when they know it is safe. The `sideEffects` field in `package.json` declares whether your modules have side effects on import.

- `"sideEffects": false` tells bundlers every module is pure and unused exports can be dropped. Set this when your library has no top-level side effects (no global polyfills, no CSS injection on import).
- `"sideEffects": ["./src/polyfill.js", "*.css"]` lists modules that do have side effects, so bundlers keep them.
- Without the field, bundlers conservatively keep all imports, bloating consumer bundles.

If your library is side-effect-free, set `false`. It materially improves consumer bundle sizes.

### Version Deliberately And Communicate Breaking Changes

Follow semver: breaking API or type changes are major, additive are minor, fixes are patch. Use pre-release tags (`1.0.0-beta.1`) for unstable releases, and a CHANGELOG to communicate what changed. Consumers pin ranges and depend on this discipline; a breaking change in a patch release breaks their builds.

## Common Traps

### Shipping Source Without Declarations

Consumers get no types or must compile your source. Build to JS and emit `.d.ts`.

### Missing Or Mis-Ordered `types` Condition

Under modern resolution, `types` must be the first condition in `exports`, or TypeScript fails to resolve declarations.

### Runtime Dependency Where Peer Is Needed

Bundling React or another singleton as a runtime dependency creates duplicate instances and breaks hooks/context. Use peer.

### Dual-Package Instance Split

Shipping separate ESM and CJS builds that each hold their own state causes singletons to split. Mitigate by sharing a neutral module.

### No `sideEffects` Field

Without it, bundlers keep all imports, bloating consumer bundles. Set `false` if the library is pure.

### Exporting Internal Types

Re-exporting internal types makes them part of the contract; refactoring internals then breaks consumers. Curate the public surface.

### Breaking Change In A Patch Release

Renaming an exported type or changing a signature in a patch violates semver and breaks consumer builds. Reserve breaking changes for majors.

### Deep Imports Into Internal Paths

Consumers importing `your-lib/dist/internal/util` bypass the public surface; refactoring breaks them. Use `exports` to expose only intended entry points.

## Self-Check

- [ ] The package ships built JavaScript and `.d.ts` declaration files, not raw source (unless source consumption is explicitly supported).
- [ ] The `exports` map defines entry points with `types` as the first condition, plus `import`/`require` as needed; `main`/`module`/`types` are set as fallbacks.
- [ ] Shared singletons and ecosystem contracts (React, Vue, shared state libraries) are declared as `peerDependencies`, not runtime dependencies.
- [ ] The public type surface is curated and versioned; internal types are not exported; breaking type changes are reserved for major versions.
- [ ] Dual ESM+CJS publishing, if used, mitigates the dual-instance hazard; or a single format is chosen if the consumer base allows.
- [ ] `sideEffects` is set (`false` for pure libraries, or an explicit list) so bundlers can tree-shake.
- [ ] A CHANGELOG and semver discipline communicate changes; pre-release tags are used for unstable versions.
- [ ] Subpath `exports` expose intended entry points and encapsulate internals; deep imports into internal paths are not relied upon.
