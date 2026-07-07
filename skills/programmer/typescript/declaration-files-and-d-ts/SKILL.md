---
name: typescript_declaration_files_and_d_ts.md
description: Use when the agent is writing or reviewing TypeScript .d.ts declaration files — ambient module declarations, module augmentation, global augmentation, writing types for untyped JavaScript libraries, authoring library public type surfaces, handling default exports and CommonJS interop in declarations, declaration merging, or debugging "Could not find a declaration file for module" errors. Covers the mechanics and pitfalls of hand-authored declaration files, distinct from generating declarations from source and the broader strict-mode/tsconfig configuration covered separately.
---

# Declaration Files And .d.ts

A `.d.ts` declaration file describes the types of a JavaScript module or global without containing any runtime code. Declarations are the contract consumers rely on: when they are accurate, TypeScript catches misuse; when they lie (declare an export that does not exist, mark an optional field required, flatten a union to `any`), consumers get runtime errors the compiler promised were impossible. The judgment problem is authoring declarations that match runtime reality — whether generated from source or hand-written for untyped JS — and using ambient/module/global augmentation deliberately rather than as a way to silence type errors.

Agents tend to write permissive `declare module "x";` shims that turn an entire library into `any`, hand-write declarations that drift from the runtime, abuse module augmentation to patch over missing types, or misunderstand how default exports and CommonJS interop appear in declarations. The harm appears as `any`-typed dependencies that disable checking across a codebase, declarations that promise fields the runtime never returns (causing crashes), and global augmentations that conflict or break in unexpected build orders. The real work is generating declarations from source where possible, writing accurate hand-authored declarations for untyped JS, modeling interop and optionality honestly, and treating augmentation as a deliberate, narrow tool.

## Core Rules

### Generate Declarations From Source Wherever Possible

For TypeScript libraries, the declarations should be emitted by the compiler from the source, not hand-written. Generated declarations stay in sync with the code; hand-written ones drift the moment a refactor changes a return type.

- Enable `declaration: true` (and for monorepos, ensure project references emit declarations).
- The emitted `.d.ts` is the public type surface; what is not exported is not visible to consumers. Design exports deliberately.
- For libraries with complex build pipelines, `emitDeclarationOnly` lets `tsc` produce declarations while a transpiler produces JS.

Hand-writing is reserved for JS libraries without types, ambient globals, and cases where the generated declaration needs hand-editing (rare; prefer fixing the source).

### Write Accurate Declarations For Untyped JavaScript

When a dependency ships no types and no `@types` package exists, you may need to author a declaration. The cardinal rule: the declaration must match what the runtime actually exports, not what you wish it did.

- Start from the library's actual API (read its docs/source, call it in a scratch file and inspect), not from a guess.
- Model optionality honestly: a field that may be `undefined` at runtime must be optional (`field?: T`) or `T | undefined`, not declared required.
- Model unions honestly: a function that returns `string | null` must be declared `string | null`, not flattened to `string`.
- Prefer a narrow, accurate declaration over a permissive one. A declaration that types half the library correctly and the rest as `unknown` is better than one that types everything as `any`.
- Place the declaration in a `types/` or `@types/`-style file and reference it via `typeRoots`, `paths`, or a local `declare module`.

A `declare module "x";` with no body makes the whole module `any`, disabling checking everywhere it is used. Avoid the empty shim; write at least the surface you consume.

### Model Default Exports And CommonJS Interop Correctly

JavaScript modules export in several shapes, and the declaration must match the runtime export shape or consumers get `undefined` imports.

- **ESM default export** (`export default function foo()`): declare as `declare function foo(): ...; export default foo;` or via the equivalent syntax.
- **CommonJS `module.exports = fn`**: the default import in ESM consumers receives the function; declare with `export = fn` so both `import fn from` and `const fn = require()` work.
- **CommonJS `module.exports = { a, b }`**: declare named exports; be aware that static named-import detection from CJS is not guaranteed at runtime in Node (see the ESM/CJS skill).
- **Mixed/hybrid shapes**: model exactly what the runtime provides; do not assume ESM defaults exist when the library is CJS.

Interop mismatches ("the default import is `undefined`") almost always trace to a declaration that says `export default` when the runtime uses `module.exports`, or vice versa. Match the export shape to the runtime.

### Use `declare module` Augmentation Narrowly And Deliberately

Module augmentation adds to an existing module's types — extending a library's interface, adding a field to a framework's options, or declaring a module for CSS/asset imports. It is powerful and easy to misuse.

- **Asset imports** (`import styles from "./x.css"`): declare `declare module "*.css" { const styles: Record<string, string>; export default styles; }` so the import type-checks. This is the legitimate, common use.
- **Library augmentation** (`declare module "lib" { interface X { myField: string } }`): use when a plugin adds capabilities to a library's types. Must match the library's real runtime extension behavior, or consumers trust types the runtime does not honor.
- Scope augmentations narrowly (in a single `types/` file or a `.d.ts` included by the project) and document why. Broad augmentations scattered across the codebase conflict and surprise.

Augmentation is not a tool to silence "could not find declaration" by typing a module as `any`; that is the empty-shim anti-pattern. Augment to add accurate types, not to remove them.

### Use Global Augmentation Sparingly

Global augmentation (`declare global { ... }`) adds to the global scope. It is appropriate for true globals (a script injected by the runtime, `window` properties set by a non-JS asset) and inappropriate for module-scoped code.

- Prefer module imports over globals. If a value comes from a module, import it; do not put it on the global scope.
- When a global is real (e.g., a runtime-injected config on `window`), declare it in a single `global.d.ts` and keep it minimal.
- Global augmentations from multiple files can conflict or depend on build order; centralize them.

Most "I need a global" cases are better solved by a module that exports the value. Reserve `declare global` for values that genuinely live on the global scope at runtime.

### Understand Declaration Merging And Its Limits

Declaration merging lets multiple declarations of the same name (interface, namespace, function) combine into one. It is useful but has rules and limits.

- **Interfaces merge**: two `interface Foo` declarations combine their members. Used for augmentation and for splitting large interfaces across files.
- **Namespaces merge** with classes/functions/enums, adding static members.
- **Functions merge** via overloads: multiple `function f` declarations become overload sets.
- Merging does not work across all declaration kinds (a `type` alias does not merge with an `interface`), and conflicting merged members cause errors.

Use merging for its intended purposes (augmentation, overloads); do not rely on it to paper over inconsistent declarations.

### Verify Declarations Against Runtime Behavior

A declaration is a claim about runtime behavior; verify it. This is especially important for hand-written declarations and for libraries whose runtime may differ from their docs.

- For libraries you consume, spot-check that declared exports exist and have the declared shape (a quick script that imports and inspects).
- For libraries you publish, consume your own declarations in a fresh project before release; this catches "works in dev, breaks when installed" errors.
- When a consumer reports a type error that "shouldn't happen," check whether the declaration matches the runtime before assuming the consumer is wrong.

Declarations that are never verified drift toward fiction. Treat the declaration/runtime boundary as a thing to test.

## Common Traps

### Empty `declare module "x";` Turning Everything To `any`

A declaration with no body types the whole module as `any`, disabling checking across every consumer. Write at least the surface you use, typed accurately.

### Declaration Marking An Optional Field Required

`{ count: number }` when the runtime sometimes omits `count` causes consumers to trust a field that is `undefined`. Use `count?: number` to match reality.

### Default Export Mismatch With CommonJS

Declaring `export default` when the library uses `module.exports = fn` makes the default import `undefined` at runtime. Use `export = fn` for CJS.

### Flattening A Union To A Single Type

Declaring `function f(): string` when the runtime returns `string | null` hides a real case. Preserve the union.

### Augmenting A Module To Silence A Missing-Type Error

Using `declare module "x" { const x: any; export = x; }` to make an error go away disables checking rather than fixing it. Author accurate types or find an `@types` package.

### Global Augmentation For Module-Scoped Values

Putting a value on `declare global` when it should be imported from a module creates hidden dependencies and ordering issues. Import from a module instead.

### Hand-Written Declarations Drifting From Runtime

A hand-written `.d.ts` that is not regenerated or verified diverges from the library over time. Generate from source, or keep a test that verifies the declaration against runtime behavior.

### Declaration Merging Conflicts

Two augmentations that define the same property with different types, or a `type` alias that tries to merge with an `interface`, cause errors. Coordinate augmentations and use only mergeable declaration kinds.

## Self-Check

- [ ] Library declarations are generated from source (`declaration: true`); hand-written declarations are reserved for untyped JS, ambient globals, and asset modules.
- [ ] Hand-written declarations match runtime reality: optionality, unions, and export shapes are modeled honestly, not flattened or assumed.
- [ ] Default/CommonJS exports are declared to match the runtime export shape (`export =` for CJS `module.exports`, `export default` for ESM defaults).
- [ ] No empty `declare module "x";` shims turn dependencies into `any`; the consumed surface is typed at minimum.
- [ ] Module augmentation is used narrowly (asset imports, library plugin extensions) and matches runtime extension behavior; it is not used to silence missing-type errors.
- [ ] Global augmentation is reserved for true runtime globals and centralized; module-scoped values are imported, not globalized.
- [ ] Declaration merging is used for its intended purposes (interface augmentation, overloads) with mergeable declaration kinds and no conflicts.
- [ ] Declarations are verified against runtime behavior (consuming your own published declarations; spot-checking consumed libraries).
- [ ] `@types` packages are used for popular JS libraries rather than hand-writing declarations.
- [ ] The public type surface (what is exported) is designed deliberately; internal helpers are not leaked into declarations.
