---
name: typescript_conditional_and_mapped_types.md
description: Use when the agent is writing or reviewing TypeScript type-level computations — distributive conditional types, infer extraction, homomorphic mapped types, key remapping with as, modifier removal (-readonly, -?), template literal types, recursive types, or deriving utility types. Covers the mechanics and design of conditional and mapped type machinery, when type-level programming is worth the complexity, and the pitfalls of distributivity, deep recursion, and incomprehensible error messages. Distinct from the broader design philosophy of generics, which is covered separately.
---

# Conditional And Mapped Types: Type-Level Programming

TypeScript's type system is Turing-complete enough to express real computation at the type level: conditional types branch on type relationships, `infer` extracts pieces, mapped types transform shapes, key remapping renames keys, and template literal types build string types character by character. This is powerful machinery for building precise utility types and deriving types from other types so they cannot drift. The judgment problem is knowing *when* type-level computation earns its complexity, how the mechanics actually behave (distributivity, homomorphic mapping, recursion limits), and where a clever type transformation becomes an unreadable, slow, error-obscuring trap.

Agents tend to reach for conditional/mapped types because they look impressive, write distributions that silently change behavior, nest conditionals until error messages point at the wrong place, or build recursive types that hit the compiler's depth limit. The harm appears as types that are correct but unreadable, build times that balloon from deep type instantiation, error messages that say `Type A is not assignable to Type B` with no hint of which transformation failed, and utilities that work on simple inputs but break on optional/index-signature/union types. The real work is using type-level computation where it preserves a real relationship (deriving a read-only view, extracting parameter types, building constrained option types), keeping transformations shallow and named, and verifying them against realistic inputs.

## Core Rules

### Use Type-Level Computation To Preserve Relationships, Not To Show Off

A conditional or mapped type earns its place when it derives one type from another so they cannot drift. If the transformation could be a plain type alias or an explicit interface with no loss of correctness, the machinery is overhead.

- **Worth it**: `type Readonly<T> = { readonly [K in keyof T]: T[K] }` — a read-only view that tracks `T` as it changes. `type ReturnType<T> = T extends (...args: any[]) => infer R ? R : never` — extracting a function's return type so callers stay in sync.
- **Worth it**: deriving an option type from a full type (`Partial<Pick<User, "name" | "email">>`) so adding a field to `User` updates the options automatically.
- **Not worth it**: a deeply nested conditional that computes a type you could write directly, just to avoid repeating a name. Readability and error quality matter more than DRY at the type level.

The test: does this transformation keep two types in sync that would otherwise drift? If yes, it is justified. If it is just shorter, write the explicit type.

### Understand Distributive Conditional Types

A conditional type `T extends U ? X : Y` is *distributive* when `T` is a naked type parameter: it distributes over a union, applying the conditional to each member and unioning the results. This is usually what you want, but it changes behavior in ways that surprise.

- `type ToArray<T> = T extends any ? T[] : never; ToArray<string | number>` is `string[] | number[]`, *not* `(string | number)[]`. Each union member is processed separately.
- To prevent distribution, wrap `T` in a tuple: `type ToArray<T> = [T] extends [any] ? T[] : never;` gives `(string | number)[]`.
- Distribution is the basis for filtering unions (`type ExtractStrings<T> = T extends string ? T : never` pulls out the string members), which is a legitimate and common use.

Know whether you want distribution. A utility that silently distributes when you meant a single combined type, or vice versa, produces types that look right and behave wrong.

### Use `infer` To Extract, Not To Branch

`infer R` declares a type variable within a conditional's `extends` clause, capturing whatever matches that position. It is for extraction (pulling a piece out of a known shape), not for general branching.

- `T extends Promise<infer U> ? U : T` unwraps a Promise, extracting its inner type.
- `T extends (first: infer F, ...rest: infer R) => any ? F : never` extracts a function's first parameter.
- Multiple `infer` positions in one `extends` let you capture several pieces; the conditional branches on whether the match succeeded.

Keep `infer` shallow. Nesting `infer` inside `infer` inside conditionals produces types that are hard to read and produce errors that point at the whole expression. If you need to extract several layers, compose named helper types rather than one deep conditional.

### Prefer Homomorphic Mapped Types And Know Their Properties

A *homomorphic* mapped type has the form `{ [K in keyof T]: ... }` — it maps over the keys of an existing type `T`. Homomorphic maps preserve `T`'s modifiers: if `T`'s properties are `readonly` or optional, the mapped type keeps those unless you explicitly change them.

- `{ [K in keyof T]: T[K] }` preserves optionality and readonly-ness. This is why `Readonly` and the built-in utilities behave predictably.
- To *remove* modifiers, prefix with `-`: `{ -readonly [K in keyof T]: T[K] }` strips readonly; `{ [K in keyof T]-?: T[K] }` strips optionality. Adding is the default (`?`, `readonly`); removing needs the `-`.
- Non-homomorphic maps (`{ [K in SomeKeys]: ... }` where `SomeKeys` is not `keyof T`) do not preserve modifiers and behave differently.

When a mapped type behaves unexpectedly (optionality vanishing, readonly not applied), check whether it is homomorphic and whether modifier syntax is right.

### Use Key Remapping And Template Literal Types For Name Transformation

Key remapping (`[K in keyof T as NewKey<K>]`) lets a mapped type rename or filter keys. Combined with template literal types (`` `${Prefix}${Capitalize<K>}` ``), you can transform key names at the type level.

- `[K in keyof T as `get${Capitalize<string & K>}`]: T[K]` builds getter-name keys from a type's fields.
- `[K in keyof T as T[K] extends Function ? K : never]: T[K]` filters to only method keys (a common "pick the methods" utility).
- Template literal types also build string unions: `` `on${Capitalize<EventName>}` `` generates event-handler name types.

These are genuinely useful for deriving API shapes, but they are also the frontier where types become hard to read. Name the pieces, test with concrete inputs, and document the intent.

### Keep Recursion Shallow And Bounded

TypeScript supports recursive types (a type that references itself), used for recursive data structures and some utilities. But the compiler has a recursion depth limit, and deep recursion produces slow builds and cryptic "Type instantiation is excessively deep" errors.

- Prefer iteration-style utilities over deep recursion where possible.
- Bound recursion with a depth parameter or a base case that terminates.
- Test recursive types against realistic input sizes; a type that works on depth-3 structures may fail at depth 20.

A recursive type that hits the instantiation limit is not "almost correct" — it fails to compile, blocking the build. Keep recursion shallow or restructure.

### Test Type-Level Code Against Concrete Inputs

Type-level code has no runtime behavior to observe; you verify it by resolving it against concrete types and inspecting the result. Do this always.

- `type T1 = MyUtility<string | number>` — hover or assert (`const x: T1 = null as any`) to see the resolved type.
- Test edge cases: empty objects, optional fields, index signatures, unions, `never`. Mapped types often behave differently on these.
- Use `@ts-expect-error` assertions in tests to confirm a type *rejects* what it should reject, not just that it accepts what it should accept.

Assuming a type transformation works because it compiles is the type-level equivalent of not running your code. Resolve it against inputs.

## Common Traps

### Accidental (Or Unwanted) Distribution

`type Wrap<T> = T extends any ? T[] : never` distributes, turning `string | number` into `string[] | number[]`. If you wanted `(string | number)[]`, wrap with a tuple: `[T] extends [any]`.

### Conditional That Always Matches (Or Never Matches)

`T extends any ? X : Y` always takes the `X` branch; `T extends never ? X : Y` behaves specially (distribution over `never` yields `never`). Verify your `extends` clause actually discriminates.

### Mapped Type Dropping Optionality Or Readonly

A non-homomorphic map, or the wrong modifier syntax, silently removes `?`/`readonly`. Check homomorphic form (`keyof T`) and modifier prefixes (`-?`, `-readonly`).

### Deep Recursion Hitting The Instantiation Limit

Recursive utilities that work on small inputs fail with "Type instantiation is excessively deep" on real data. Bound recursion or restructure iteratively.

### Incomprehensible Errors From Nested Conditionals

A chain of `infer` inside `extends` inside another conditional produces errors that point at the whole expression. Compose named helper types; keep each step shallow.

### `keyof` On An Index Signature Yielding `string | number`

`keyof { [k: string]: V }` is `string | number`, which can cascade into surprising mapped-type behavior. Inspect `keyof` results on types with index signatures before relying on them.

### Template Literal Types Exploding The Union

`` `on${EventName}` `` over a large `EventName` union, or recursive template building, can produce huge unions that slow the compiler and produce unreadable types. Bound the input set.

### Assuming A Utility Works On Optional/Index-Signature Types

A mapped type that handles required fields cleanly may break on optional fields or index signatures. Test with the full range of shapes your type encounters.

## Self-Check

- [ ] Type-level computation is used to preserve real relationships (deriving views, extracting pieces), not to be clever or merely shorter.
- [ ] Distributive vs non-distributive behavior is intentional; naked type parameters distribute, tuple-wrapped ones do not.
- [ ] `infer` is used for shallow extraction, not deep nested branching; complex extractions are composed from named helpers.
- [ ] Mapped types are homomorphic where modifier preservation is wanted; modifier removal (`-?`, `-readonly`) is explicit and verified.
- [ ] Key remapping and template literal types are tested against concrete inputs and documented where non-obvious.
- [ ] Recursion is shallow and bounded; recursive types are tested at realistic input sizes, not just trivial cases.
- [ ] Type-level utilities are resolved against concrete types (including edge cases: optional, index signatures, unions, `never`) to verify behavior, not assumed correct.
- [ ] `@ts-expect-error` tests confirm utilities reject what they should, not just accept what they should.
- [ ] `keyof` results on types with index signatures are inspected before mapped types rely on them.
- [ ] Each type-level transformation is named and shallow enough that error messages remain meaningful; deeply nested conditionals are split.
