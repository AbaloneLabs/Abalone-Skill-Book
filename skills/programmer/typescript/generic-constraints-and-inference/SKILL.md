---
name: typescript_generic_constraints_and_inference.md
description: Use when the agent is designing TypeScript generics with precise constraints and inference goals — choosing between extends constraints and keyof/in constraints, controlling inference with const type parameters and noUncheckedIndexedAccess-aware inference, multi-parameter inference relationships, default type parameters, overloads vs generics for inference, higher-order function typing, or debugging cases where inference widens, collapses to never/any, or fails to infer. Covers the mechanics of constraint design and inference control, distinct from the broader design philosophy of generics and the type-level programming of conditional/mapped types, which are covered separately.
---

# Generic Constraints And Inference

Generics earn their complexity only when they preserve real type relationships, and the quality of that preservation depends on two things: the *constraints* you place on type parameters (what they must satisfy) and how *inference* resolves them at the call site (what the compiler concludes from the arguments). The judgment problem is designing constraints that are tight enough to let the body do what it needs yet loose enough to remain reusable, and steering inference so it resolves to the intended type instead of widening to a union, collapsing to `never`, or silently becoming `any`.

Agents tend to leave type parameters unconstrained when the body needs methods, over-constrain to concrete classes, rely on inference in cases where it widens or fails, or add defaults that quietly turn the generic into `any`. The harm appears as generics that reject valid call sites (too tight), accept and break on valid call sites (too loose), or infer types that disable checking (`any`) or collapse data (`never`). The real work is matching each constraint to the body's real requirements, predicting what inference will conclude at realistic call sites, and using the mechanisms (const type parameters, constraints, defaults, overloads) that steer inference toward the intended type.

## Core Rules

### Match The Constraint To What The Body Actually Requires

An unconstrained `T` lets you move values but not call anything on them. A constraint should grant exactly the capabilities the body uses — no more (which over-constrains and rejects valid callers) and no less (which under-constrains and produces errors at every call site).

- If the body only reads `length`, constrain to `{ length: number }`, not to `string | any[]`. The structural constraint admits strings, arrays, and any custom object with a length.
- If the body calls `.map`, constrain to a structure with a `map` method (or use a more specific shape), not to `Array<T>`.
- Prefer structural constraints (small interfaces) over nominal ones (concrete classes), so test doubles and compatible shapes satisfy them.

The discipline: enumerate what the body does to `T`, and write the minimal interface that covers it. Over-constraining to `User` when the body reads three fields rejects every caller with a compatible shape.

### Use `keyof` And `in` Constraints For Key-Value Relationships

When a function takes an object and a key into it, the key's type should be derived from the object so they cannot drift. `keyof` and `in` constraints express this.

- `function get<T, K extends keyof T>(obj: T, key: K): T[K]` ties the key to the object; the return type is the property's type, and a misspelled key is a compile error.
- `function pick<T, K extends keyof T>(obj: T, keys: K[]): Pick<T, K>` constrains the keys array to valid keys and returns a precisely typed subset.
- The `in` operator as a constraint (`K extends keyof any`) or in narrowing ties dynamic keys to known shapes.

Without `keyof`, a key parameter is `string` and the compiler cannot catch typos or relate the key to the value type. With it, the relationship is enforced.

### Control Inference Width With `const` Type Parameters

By default, inference widens: `fn([1, 2, 3])` infers `number[]`, and `fn({ kind: "a" })` infers `kind: string`, losing the literal. When you need the literal type (to preserve a discriminant, a tuple shape, or exact keys), use a `const` type parameter (TS 5.0+) to opt out of widening.

- `<const T>(x: T)` infers the most specific type: `fn([1, 2, 3])` infers `readonly [1, 2, 3]`, and `fn({ kind: "a" })` infers `{ readonly kind: "a" }`.
- Use `const` when the caller's literal values matter (building discriminated unions, inferring exact keys, tuple types).
- `const` can over-constrain: if the caller passes a variable (not a literal), the inferred type is the variable's type, not a literal. Use `const` when callers pass literals whose exact shape you need.

Before `const` type parameters, the workaround was `as const` at the call site; the parameter modifier moves the intent into the signature, which is clearer and less error-prone.

### Preserve Multi-Parameter Inference Relationships

When a generic has multiple type parameters that should relate (two arguments of the same type, or a function and its argument), design the signature so inference relates them rather than inferring each independently and rejecting mismatches.

- `function pair<A, B>(a: A, b: B): [A, B]` infers each independently; that is usually right.
- `function merge<T, U>(a: T, b: U): T & U` infers an intersection; the relationship is in the return type.
- For a function-and-argument pattern (`function apply<T, R>(f: (x: T) => R, x: T): R`), inference resolves `T` from the argument and `R` from the function; verify it resolves as expected at realistic call sites.

When two parameters must be the *same* type and inference infers a union instead, you may need to constrain one in terms of the other or restructure. Multi-parameter generics are where inference most often surprises; test them.

### Choose Defaults Deliberately; Avoid Accidental `any`

A type parameter can have a default (`<T = string>`), used when the caller does not supply the argument and it cannot be inferred. Defaults are useful but dangerous when they default to `any`.

- `<T = any>` silently disables checking when the caller omits the argument; this is usually a bug, not a feature. Prefer a concrete default (`<T = never>` to force inference, or a real type) or require the argument.
- A default is appropriate when there is a sensible "most common" type and the generic is genuinely optional.
- Document non-obvious defaults; a reader who sees `<T = ...>` should understand what happens when it is omitted.

Audit generic defaults for accidental `any`; each one is a hole in the type system.

### Prefer Overloads Over Complex Conditional Signatures For Call-Site Ergonomics

When a function's return type depends on its argument shape in ways a single generic signature cannot express cleanly, overloads often produce better inference and better error messages than a clever conditional return type.

- Overloads let each call site match the most specific signature, so the return type is precise and errors point at the relevant overload.
- A single signature with a complex conditional return type can defeat inference (callers must annotate) and produce errors that point at the whole conditional.
- Use overloads when there are a few distinct input/output shapes; use a conditional type when the relationship is truly continuous (any input maps to a derived output).

Overloads have their own pitfall (the implementation signature is untyped relative to the overloads, so it can drift), but for call-site ergonomics they often beat a convoluted generic.

### Verify Inference At Realistic Edge Call Sites

Inference that works on the obvious call site often fails on edges: empty arrays, single-element tuples, callbacks, overloaded functions, and variables whose declared type is wider than the literal. Verify generics against these.

- `first([])` infers `T = never` (empty array); decide whether to annotate, provide a default, or accept `never`.
- A callback parameter may infer a wider type than intended; annotate the callback or constrain the generic.
- An overloaded function passed as an argument uses its last overload for inference, which may not be what you want.

A generic that requires every realistic caller to annotate type arguments is misdesigned. Test with the full range of call sites, not just the demo case.

## Common Traps

### Unconstrained `T` When The Body Calls Methods

`function len<T>(x: T) { return x.length }` errors because `T` has no `length`. Constrain to `{ length: number }`.

### Over-Constraining To A Concrete Class

`<T extends User>` when the body reads three fields rejects callers with compatible shapes and test doubles. Constrain to the minimal structural interface.

### Inference Widening Losing Literal Types

`fn({ kind: "a" })` infers `kind: string`, losing the discriminant. Use a `const` type parameter or `as const` at the call site when the literal matters.

### Default Type Parameter Of `any`

`<T = any>` silently disables checking when the argument is omitted. Use a concrete default or require inference.

### `first([])` Inferring `never`

An empty array infers `T = never`. Decide explicitly: annotate, default, or accept `never` and handle it.

### Complex Conditional Return Type Defeating Inference

A return type built from nested conditionals can prevent callers from inferring type arguments, forcing annotations and producing opaque errors. Prefer overloads for distinct input/output shapes.

### Two Parameters That Should Match Inferring A Union

`function eq<A>(a: A, b: A)` called with `eq(1, "x")` infers `A = string | number`. If they must be identical, restructure or constrain; if a union is acceptable, leave it.

### Inferring From An Overloaded Function's Last Overload

Passing an overloaded function as an argument uses the last overload for inference, which may not match the intended call. Restructure or annotate.

### Unused Or Decorative Type Parameter

`function parse<T>(s: string): T` cannot infer `T` and forces every caller to annotate, providing no safety. Infer from an input, or return a concrete/validated type.

## Self-Check

- [ ] Each type parameter's constraint grants exactly the capabilities the body uses; constraints are structural and minimal, not over-constrained to concrete classes.
- [ ] Key-value relationships use `keyof`/`in` constraints so keys are tied to objects and typos are compile errors.
- [ ] `const` type parameters are used where literal types must be preserved (discriminants, tuples, exact keys); widening is intentional where literals do not matter.
- [ ] Multi-parameter generics are tested for the relationships they should preserve; cases where inference produces an unwanted union are restructured or constrained.
- [ ] No generic defaults to `any` without intent; defaults are concrete or documented, and inference is required where appropriate.
- [ ] Overloads are preferred over convoluted conditional return types for distinct input/output shapes; the implementation signature is kept in sync with the overloads.
- [ ] Inference is verified at edge call sites (empty arrays, tuples, callbacks, overloaded arguments, widened variables), not just the obvious case.
- [ ] No decorative type parameters that cannot be inferred and force every caller to annotate; each parameter appears in an inferrable position.
- [ ] `first([])`-style `never` inference is handled deliberately (annotation, default, or explicit handling), not left to surprise.
- [ ] The generic's call-site ergonomics are validated: realistic callers infer correctly without forced annotations.
