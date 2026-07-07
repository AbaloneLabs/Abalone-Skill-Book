---
name: javascript_module_systems_esm_cjs.md
description: Use when the agent is reasoning about the runtime semantics of JavaScript ESM and CommonJS modules — live bindings, hoisting of imports, circular dependency handling, top-level await, the module vs script goal, how require and import resolve differently at runtime, or debugging "Cannot use import statement outside a module", "require is not defined", or undefined named exports from a CJS interop. Covers the behavioral differences between ESM and CJS at runtime, distinct from bundler configuration and build tooling which are covered separately.
---

# Module Systems: ESM And CJS Runtime Semantics

ECMAScript Modules (ESM) and CommonJS (CJS) are not just different syntax — they have fundamentally different runtime semantics. ESM imports are *live bindings* that are resolved before the module body runs; CJS `require` is a synchronous function call that returns a cached copy of `module.exports`. These differences determine how circular dependencies resolve, whether re-exported values update, whether top-level code runs once or many times, and which errors surface. The judgment problem is reasoning about what actually happens at runtime when modules load, distinct from how bundlers transform them.

Agents tend to treat `import` and `require` as interchangeable syntax for the same behavior, assume a circular import will "just work," or expect a named import from a CJS module to be defined. The harm appears as `undefined` named imports, values that do not update across a circular dependency, modules that execute in surprising order, and errors ("Cannot use import statement outside a module") that stem from the file being parsed as the wrong module goal. The real work is understanding that ESM and CJS are different execution models, committing to one per boundary, and reasoning about load order, bindings, and circularity deliberately.

## Core Rules

### Recognize ESM And CJS As Different Execution Models

The syntax difference (`import`/`export` vs `require`/`module.exports`) reflects deeper differences in how modules execute.

- **CJS** loads synchronously: `require('./b')` runs module B to completion and returns its `module.exports`, caching the result in `require.cache`. Each module is a function wrapping its body. `this` at the top level is `module.exports`. CJS is designed for Node's synchronous startup.
- **ESM** loads asynchronously and in phases: the module graph is parsed and dependencies are resolved first, then modules execute in dependency order. Imports are *live bindings* — references into the exporting module's bindings, not copies. ESM is always in strict mode, and top-level `this` is `undefined`.

Because ESM resolves the whole graph before execution, an `import` is not a function call you can conditionally skip; it is a static declaration hoisted to the top. `require` can be called conditionally and dynamically because it is just a function. This asymmetry is why ESM enables static analysis (tree shaking) and CJS does not.

### Understand Live Bindings In ESM

When module A does `export let count = 0; export function inc() { count++; }`, and module B does `import { count, inc } from './a'`, B's `count` is a live binding: when A's `inc()` mutates `count`, B sees the new value. This is fundamentally different from CJS, where `module.exports.count` is a snapshot read at `require` time.

- Live bindings mean re-exported values stay current. A barrel file (`export * from './a'`) reflects A's current state.
- The binding is read-only from the importer's perspective: B cannot assign to `count`, only read it. Mutations happen in the exporting module.
- CJS exports are properties of a cached object; `const { count } = require('./a')` snapshots `count` at that moment and never updates. If A later changes `module.exports.count`, earlier require-ers still hold the old value.

This difference matters most with circular dependencies and with values that change over time (configuration, counters, feature flags).

### Handle Circular Dependencies By Phase

Circular imports (A imports B, B imports A) resolve differently in ESM and CJS, and are a common source of `undefined` surprises.

- **CJS**: when A requires B and B requires A, the require of A from B returns A's *partial* `module.exports` — whatever A has assigned so far, which may be an empty `{}` if A has not reached its `module.exports` assignment yet. Exports assigned later are not visible to B. This is why CJS circular deps produce `undefined` for not-yet-assigned exports.
- **ESM**: the module graph is instantiated before execution, so bindings exist even before values are assigned. B's import of A is a live binding that is `undefined` until A's body executes the assignment, then becomes the value. If B uses A's binding *during* its own module-body execution (before A finishes), it sees `undefined` — but if B uses it later (inside a function called after load), it sees the final value.

The safe pattern for circular dependencies: do not use the other module's exports at module top-level/evaluation time. Defer usage into functions that run after the full graph has loaded. If A and B must both reference each other at evaluation time, the cycle is a design smell — break it by extracting the shared piece into a third module.

### Commit To A Module Goal Per File

A file is parsed as either an ESM module or a CJS script ("module goal" vs "script goal"). Mixing syntax (`import` and `require` in the same file) is generally invalid and the goal is determined by context, not by the syntax alone.

- In Node, the goal is set by the nearest `package.json` `"type"` field (`"module"` → ESM, `"commonjs"` or absent → CJS), by file extension (`.mjs` is always ESM, `.cjs` is always CJS), or by the runtime flags.
- A `.js` file under `"type": "module"` is ESM; under no `"type"` it is CJS. Getting this wrong produces "Cannot use import statement outside a module" (file is CJS but uses `import`) or "require is not defined" (file is ESM but uses `require`).
- Browsers determine the goal by `type="module"` on the script tag.

Know which goal each file is parsed under, and do not assume syntax alone decides it.

### Use Top-Level Await Only In ESM

`await` at the top level of a module (outside any function) is an ESM feature. It lets a module's body await a Promise during its (asynchronous) evaluation.

- Top-level await blocks sibling modules that depend on this one from executing until it resolves. Use it for initialization that must complete before dependents run (loading config, warming a cache).
- It is not available in CJS (which evaluates synchronously) or in the top level of a classic script.
- Overuse serializes module loading; a top-level await in a widely-imported module can delay the whole application's startup.

### Bridge ESM And CJS Deliberately

When ESM and CJS must interoperate, the rules are asymmetric and a common source of bugs.

- **ESM importing CJS**: `import cjs from './cjs-module'` gives you the entire `module.exports` as the default import. Named imports (`import { x }`) from CJS are detected by static analysis in some tooling but are *not guaranteed at runtime* in Node; if the CJS module assigns `module.exports` dynamically, named imports may be `undefined`. Use the default import and destructure, or use `createRequire` to call `require` from ESM.
- **CJS importing ESM**: `require()` cannot load an ESM module in Node (it throws). Use dynamic `import()` (which is async) to load ESM from CJS, or migrate the CJS file to ESM.
- A package that exports both formats creates a dual-package hazard: a consumer might load two copies (one via each format), breaking singletons and `instanceof`.

The clean boundary is to commit to one format per package and document the interop points explicitly.

## Common Traps

### Treating `import` And `require` As Interchangeable

They have different semantics (live bindings vs snapshot, static vs dynamic, async vs sync). Interop works only with deliberate rules; mixing them blindly produces `undefined` exports and load errors.

### Circular Dependency Producing `undefined`

In both systems, using a module's export during its own evaluation (before it is assigned) yields `undefined`. Defer circular usage into functions that run after load, or break the cycle.

### Named Imports From CJS Being `undefined`

`import { x } from 'cjs-pkg'` is not guaranteed at runtime if the CJS module assigns `module.exports` dynamically. Use the default import and destructure at runtime.

### Wrong Module Goal For The File

A `.js` file with `import` under a CJS `package.json` throws "Cannot use import statement outside a module." Set `"type": "module"`, use `.mjs`, or convert to `require`.

### CJS Snapshot Not Updating Across A Cycle

`const { count } = require('./a')` snapshots `count`; if A changes it, the require-er keeps the old value. Use `require('./a').count` at use time, or migrate to ESM live bindings.

### `require` Of An ESM Module

`require()` cannot load ESM in Node. Use dynamic `import()` from CJS, or migrate the file to ESM.

### Top-Level Await In CJS Or A Classic Script

Top-level await is ESM-only. In CJS or a classic script it is a syntax error; wrap in an async IIFE if you need await during startup (noting it no longer blocks dependents).

### Dual-Package Double-Loading

A package exporting both ESM and CJS can be loaded twice (once per format) by one consumer, producing two singletons and broken `instanceof`. Publish one format or mitigate the hazard explicitly.

## Self-Check

- [ ] ESM and CJS are understood as different execution models (live bindings vs snapshot, static/async vs dynamic/sync), not interchangeable syntax.
- [ ] The module goal of each file is known and consistent (set by `type`/extension/runtime), so no "Cannot use import" or "require is not defined" errors arise from a wrong goal.
- [ ] Circular dependencies defer usage of the other module's exports into post-load functions; nothing reads a binding/export during evaluation that may be `undefined`.
- [ ] Named imports from CJS are not assumed defined at runtime; the default import or `createRequire` is used where named exports are uncertain.
- [ ] `require` is not used to load ESM in Node; dynamic `import()` is used from CJS instead.
- [ ] Live bindings are relied on for values that change (config, counters) in ESM; CJS snapshots are read at use time (`require('./a').x`) where freshness matters.
- [ ] Top-level await is used only in ESM and only where blocking dependents is intended; it is not used in CJS or classic scripts.
- [ ] The project commits to one format per boundary; dual-package hazards are mitigated or documented where both formats are published.
- [ ] Bundler configuration and resolution (covered separately) is not confused with these runtime semantics; behavior is verified in the actual runtime.
