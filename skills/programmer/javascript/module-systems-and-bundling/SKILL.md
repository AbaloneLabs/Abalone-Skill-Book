---
name: javascript_module_systems_and_bundling.md
description: Use when the agent is setting up or debugging JavaScript/TypeScript module imports, choosing between ESM and CommonJS, resolving interop errors ("Cannot use import statement outside a module", "require is not defined"), configuring bundlers (webpack/vite/rollup/esbuild/turbopack), reasoning about tree shaking, dynamic import and code splitting, polyfills and transpilation targets, module resolution algorithms (node vs bundler), package.json exports/imports fields, or optimizing bundle size vs runtime performance. Covers dual-package hazards, named export pitfalls, and build target strategy.
---

# Module Systems And Bundling

JavaScript has two competing module systems — ECMAScript Modules (ESM, `import`/`export`) and CommonJS (CJS, `require`/`module.exports`) — plus a layer of bundlers that transform, combine, and split modules for delivery. Each system has different syntax, different resolution rules, and different runtime semantics, and the boundaries between them produce a large family of confusing errors. The judgment problem is choosing a module strategy, configuring resolution and bundling deliberately, and understanding what actually ships to the browser or runtime.

Agents tend to copy a bundler config, sprinkle `import` and `require` together, and assume "the build will figure it out." The harm shows up as: a library that works in one tool and breaks in another (dual-package hazard), tree shaking that silently fails because of a CJS dependency, a bundle that includes polyfills no target needs (or omits ones it does), and named imports that are `undefined` because of how CJS interop resolved. The real work is committing to a module system, understanding resolution and the `exports` field, and treating bundle output as a deliverable to be measured.

## Core Rules

### Commit To A Module System; Handle Interop Deliberately

ESM is the standard going forward; CJS is legacy that remains dominant in older Node and many npm packages. Mixing them is the source of most module errors.

- **New projects**: use ESM (`"type": "module"` in `package.json`, `.mjs` files, `import`/`export`). It enables tree shaking, top-level await, and future-compatible tooling.
- **Existing CJS projects**: migrate deliberately, file by file, or stay consistently CJS. Do not mix `import` and `require` in the same project without a clear interop strategy.
- **Libraries**: decide what you publish. Publishing ESM-only (`"type": "module"`, ESM in `exports`) is increasingly viable but excludes pure-CJS consumers. Publishing both (dual package) maximizes compatibility at the cost of dual-package hazard.

When you must interop, know the rules: `import` of a CJS module gives you the `module.exports` value as the default import; named imports from CJS are detected statically by some tooling but are not guaranteed at runtime. Use `createRequire` in ESM when you truly need `require`.

### Configure package.json `exports` To Define The Public Surface

The `exports` field is the modern way to declare a package's entry points and to prevent deep imports into internals. It is also where dual-format and conditional exports live.

- Define `"."` and any subpath entries (`"./feature"`) explicitly.
- Use conditions (`"import"`, `"require"`, `"types"`, `"node"`, `"browser"`) to route consumers to the right file per environment.
- Always include a `"types"` condition (first) so TypeScript resolves declarations correctly.
- Once `exports` is set, paths not listed are not importable from outside — which is usually what you want for encapsulation.

A missing or wrong `exports` field is a common cause of "works in dev, breaks when published." Validate by installing your package into a fresh project before releasing.

### Understand Resolution Differences

Module resolution is not one algorithm. Node resolves ESM and CJS differently; bundlers (webpack, vite, rollup, esbuild) each have their own resolver with configurable extensions, aliases, and conditions.

- Node ESM requires file extensions in relative imports (`./foo.js`, not `./foo`) and does not search `node_modules` the same way for some edge cases.
- Bundlers often allow extensionless imports and apply aliases (`@/components`), which can hide a dependency that breaks when run outside the bundler.
- `tsconfig` `moduleResolution` (`node`, `node16`/`nodenext`, `bundler`) must match how the code will actually be resolved at runtime. `bundler` is for code that goes through a bundler; `nodenext` is for code that runs in Node ESM.

When code "works in webpack but fails in Node," the cause is almost always a resolution mismatch (extensionless import, alias, or condition).

### Make Tree Shaking Possible, Then Verify It

Tree shaking (dead-code elimination) depends on ESM's static structure: imports/exports are analyzable at build time. CJS `require` is dynamic and defeats tree shaking.

- Publish ESM (and mark `"sideEffects": false` in `package.json`, or list the files with side effects) so bundlers can drop unused exports.
- Avoid side effects at module top level (mutating globals, registering listeners) in modules meant to be tree-shaken; mark them in `sideEffects` if unavoidable.
- Verify with the bundler's analyzer (webpack-bundle-analyzer, rollup-plugin-visualizer, vite's `build.rollupOptions.output.manualChunks` inspection). "It should tree shake" is an assumption; the bundle report is evidence.

### Use Dynamic Import And Code Splitting For Load Performance

`import("./module")` returns a Promise and lets the bundler split that module into a separate chunk, loaded on demand. This is the primary tool for reducing initial bundle size.

- Route-level lazy loading (`React.lazy`, `() => import(...)`) keeps initial load small.
- Dynamic import returns a module namespace object; destructure named exports from the awaited result.
- Beware of over-splitting: too many tiny chunks add request overhead. Group related routes/features into sensible chunks.
- Waterfall hazards: a dynamic import that itself dynamically imports creates a load waterfall. Prefetch or hoist critical chains.

### Choose Build Targets And Polyfills By Data, Not Habit

Transpilation and polyfills should match your actual supported environments, not a default copied from a template.

- Use a `browserslist` config (or Node `target`) to declare supported environments; Babel/SWC/postcss and core-js derive transforms and polyfills from it.
- Distinguish syntax transforms (arrow functions, optional chaining — compile-time) from polyfills (`Promise`, `Array.flat`, `Object.fromEntries` — runtime). Modern tooling (`@babel/preset-env` with `useBuiltIns: "usage"`, or `core-js`) injects only the polyfills your targets need.
- Shipping polyfills for evergreen browsers wastes bytes; omitting them for old targets breaks runtime. Decide targets explicitly and revisit them.

### Treat Bundle Output As A Measured Deliverable

Bundle size affects load time, parse time, and execution time, especially on mobile and low-end devices. Measure it.

- Set a budget; fail CI if the bundle exceeds it.
- Watch for duplicate dependencies (the same library at two versions bundled twice), large single dependencies, and unexpectedly included server-only or Node-only code.
- Distinguish first-load JS (initial bundle) from total JS (including lazy chunks). Optimize first load first.

## Common Traps

### Mixing `import` And `require` In The Same Project

This works only with careful interop configuration and breaks predictably when published or run in a different runtime. Pick one system per project or define a clear boundary.

### Dual-Package Hazard

Publishing both ESM and CJS versions of the same package means a consumer can load both copies (one via `import`, one via `require`), producing two instances of singletons, two module-scoped states, and `instanceof` failures. Mitigate with an ESM-first wrapper or `cjs-module-lexer`, and document the canonical entry.

### Named Imports From CJS Being `undefined`

`import { foo } from 'cjs-pkg'` relies on static analysis of `module.exports`. If the CJS module assigns dynamically (`module.exports = compute()`), named imports may be `undefined` at runtime while looking fine to TypeScript. Use the default import for CJS modules unless you have verified named exports.

### Extensionless Imports Breaking In Node ESM

`import './foo'` fails in Node ESM (needs `./foo.js`); it works in bundlers. Code that runs only through a bundler hides this until you try to run it directly. Use `nodenext` module resolution in `tsconfig` to catch it at type-check time.

### `sideEffects: false` When There Are Side Effects

Marking a package side-effect-free when it actually has top-level side effects (CSS imports, global polyfills, registration) lets the bundler drop those imports, silently breaking behavior. Only set `sideEffects: false` when truly accurate, or list the files with side effects.

### Over-Transpiling For Modern Browsers

Compiling everything to ES5 and polyfilling every API "to be safe" bloats the bundle and slows execution for users on modern browsers. Set `browserslist` to your real audience and let tooling scope the output.

### Polyfilling In The Wrong Order Or Twice

Loading your app code before the polyfill, or including two different `Promise` polyfills, causes subtle bugs. Load polyfills first, ideally via a single source (`core-js`), and deduplicate.

### Assuming The Bundler Will Fix Resolution

Bundlers mask resolution differences; code that builds may still fail in Node, tests, or a different bundler. Test in the actual runtime environments, not only in the dev bundler.

## Self-Check

- [ ] The project commits to one module system (ESM preferred for new code); interop, if any, is deliberate and documented.
- [ ] `package.json` `exports` defines the public surface with a `"types"` condition first and environment conditions (`import`/`require`/`node`/`browser`) as needed.
- [ ] `tsconfig` `moduleResolution` matches the actual runtime resolution (e.g., `nodenext` for Node ESM, `bundler` for bundler-only code).
- [ ] Relative imports in ESM include extensions where the runtime requires them.
- [ ] The package is published as ESM (or dual with a documented hazard mitigation) and `sideEffects` is accurate.
- [ ] Tree shaking is verified with a bundle analyzer, not assumed; top-level side effects are flagged.
- [ ] Dynamic import and code splitting are used for load performance; chunk granularity balances size against request overhead.
- [ ] Build targets and polyfills are driven by a `browserslist`/Node target, not a copied template; polyfills load before app code and are deduplicated.
- [ ] Bundle size is measured against a budget in CI; duplicate/large/Node-only dependencies are flagged.
- [ ] The package was installed into a fresh project and imports resolve correctly before release.
