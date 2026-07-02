---
name: typescript_type_narrowing_and_discriminated_unions.md
description: Use when the agent is writing TypeScript that narrows types via type guards, instanceof, in operators, user-defined TypeGuard/TypeIs predicates, or discriminated unions; designing exhaustiveness checks; deciding whether to use a type assertion (as) or a runtime validator (zod, valibot, runtypes, io-ts); or reviewing code where runtime data may not match its static type. Covers narrowing correctness, exhaustiveness via never, assertion dangers, and the boundary between compile-time types and runtime validation.
---

# Type Narrowing And Discriminated Unions

TypeScript narrows a type within a branch based on checks (`typeof`, `instanceof`, `in`, truthiness, equality, user-defined guards). Narrowing is how a broad union becomes a specific shape inside an `if` or `switch`, and it is where the static type system touches runtime control flow. The judgment problem is designing narrows that are correct (the runtime check actually proves the narrowed type) and knowing where static narrowing ends and runtime validation must begin.

Agents tend to overuse `as` assertions to "make the type fit," write type guards that claim more than they prove, trust that a discriminated union is exhaustive without an exhaustiveness check, or annotate untrusted data with a precise type and assume it matches. The harm is real: an `as` hides a shape mismatch that crashes at runtime, a loose type guard lets invalid data through, a missing union member silently falls through a switch. The real work is narrowing with checks the compiler can verify, asserting only with justification, and validating untrusted data before treating it as its static type.

## Core Rules

### Narrow With Checks The Compiler Can Verify

Prefer narrowing mechanisms TypeScript understands and tracks, so the narrowed type is sound and stays correct when types change.

- `typeof x === "string"` narrows primitives.
- `x instanceof Date` narrows class instances.
- `"kind" in x` narrows by property presence (good for discriminated unions without a literal tag).
- Equality and truthiness narrow literal types and `null`/`undefined`.
- User-defined type guards (`x is Foo`) and `TypeIs` narrow based on a predicate function.

These mechanisms keep the narrowed type in sync with the check; if the type changes, the compiler re-evaluates. Prefer them over manual reasoning that the compiler cannot follow.

### Design Discriminated Unions With A Literal Tag

A discriminated union is a union where every member shares a property with a unique literal value (the discriminant). This lets narrowing be mechanical: check the discriminant, and the compiler narrows to the matching member.

- Make the discriminant a literal union (`kind: "circle" | "square"`), not `string`.
- Give every member exactly the fields valid for its case; do not pile all fields into one optional-heavy type.
- Narrow by switching on the discriminant; the compiler tracks each branch.

A well-designed discriminated union makes invalid states unrepresentable (see the type-system-design skill) and makes narrowing safe and exhaustive. A poorly designed one (string discriminant, overlapping fields) defeats both.

### Enforce Exhaustiveness With `never`

When you switch over a discriminated union, a missing case silently falls through. Enforce exhaustiveness by assigning the narrowed value to `never` in a default branch:

```ts
function area(s: Shape): number {
  switch (s.kind) {
    case "circle": return Math.PI * s.r ** 2;
    case "square": return s.side ** 2;
    default:
      const _exhaustive: never = s;
      return _exhaustive;
  }
}
```

If a new member is added to `Shape` later, the assignment to `never` fails, forcing the author to handle the new case. Without this, new members are silently unhandled. Treat an unguarded switch over a union as incomplete.

### Write Type Guards That Prove What They Claim

A user-defined type guard (`function isFoo(x): x is Foo`) is only as good as its implementation. The signature promises the narrowing; the body must actually verify it. A guard that returns `true` too broadly narrows invalid data into a type it does not satisfy.

- Implement guards with real checks (`typeof`, property presence, shape validation), not `return true`.
- For complex shapes, validate the structure (required fields, their types) rather than a single property.
- `TypeIs` (TS 5.5+) narrows more precisely than `TypeGuard` for some cases; prefer it where available.
- Test guards with values that should pass and values that should fail, especially near the boundary.

### Reserve `as` For Genuine External Knowledge

`as` is an unchecked assertion: it tells the compiler "trust me, this is `T`," with no runtime verification. Every `as` is a place where the static type can diverge from the runtime value. Reserve it for cases where you have external knowledge the compiler lacks:

- A value from an untyped JS library or `any`-typed source that you have verified.
- A narrowing the compiler cannot follow but that a runtime check (kept in the code) guarantees.
- Interop with `unknown` after a real check.

Do not use `as` to silence a type error you do not understand, or to "convert" between unrelated types (`x as unknown as Foo` is a red flag — it bypasses all checking). Prefer a runtime check, a type guard, or a redesign.

### Validate Untrusted Data At The Boundary

Static types describe the world *after* validation. Data from `JSON.parse`, `fetch`, form input, URL params, `localStorage`, or untyped libraries is `unknown` in truth and must be validated before it is treated as a precise type. Static narrowing cannot protect this data because the check happens at compile time, not runtime.

- Use a schema validator (`zod`, `valibot`, `runtypes`, `io-ts`, `ajv`) to parse and validate at the boundary, producing a value whose type is derived from the schema.
- Treat validation as parsing, not assertion: invalid input should produce a typed error, not be coerced or partially accepted.
- After validation, the value's static type is honest; downstream code can rely on it.

The boundary is the line where `unknown` becomes a known type. Crossing it without validation is the most common source of "the type said it was there but it wasn't" runtime errors.

### Keep Runtime Checks And Static Types In Sync

A type guard or validator that drifts from the static type provides false safety. When the type changes, the validator must change too. Mitigations:

- Derive the static type from the validator (`type User = z.infer<typeof UserSchema>`), so they cannot drift.
- Review type guards when their target type changes.
- Treat `as`-heavy code as a smell: each `as` is a place where type and runtime may diverge, with no sync mechanism.

## Common Traps

### `as` To Silence A Type Error

When the compiler disagrees with you, `as` hides the disagreement instead of resolving it. Often the compiler is right and the runtime value does not match. Investigate the error; use a runtime check or fix the type instead of asserting.

### Type Guard Returning `true` Too Broadly

`function isUser(x): x is User { return !!x; }` narrows any truthy value to `User`, letting invalid data through. The guard must verify the shape, not just existence.

### Missing Discriminant In A Union

A union without a literal tag requires `in` checks or casts to narrow, and exhaustiveness does not work. Always include a discriminant so narrowing and exhaustiveness are mechanical.

### No Exhaustiveness Check

A switch over a union without a `never` default silently drops new members when the union grows. Add the exhaustiveness check so additions are forced into the code.

### Trusting `JSON.parse` Output As A Typed Value

`const user: User = JSON.parse(text)` compiles but the runtime value is whatever the string contained. Validate with a schema first; the static type is a claim, not a guarantee.

### `as unknown as T` Double Assertion

Chaining through `unknown` bypasses all structural checking. It is occasionally legitimate (truly unrelated types with verified runtime equivalence) but usually a sign of forcing a type the compiler correctly rejects.

### Narrowing That Does Not Survive A Callback

Narrowing is lost when a value is captured in a closure that runs later, because the compiler cannot prove the value has not changed. Re-narrow inside the callback, or capture a narrowed local that cannot be reassigned.

### Validator And Type Drifting Apart

If the static `User` type is hand-written and the `UserSchema` is separate, adding a field to one but not the other creates a silent mismatch. Derive the type from the schema, or keep them under a test that checks consistency.

## Self-Check

- [ ] Narrowing uses compiler-verifiable checks (`typeof`, `instanceof`, `in`, equality, type guards); assertions are reserved for genuine external knowledge.
- [ ] Unions carry a literal discriminant so narrowing is mechanical and exhaustiveness works.
- [ ] Every switch over a union has a `never`-based exhaustiveness check so new members are forced into the code.
- [ ] User-defined type guards verify the real shape, not just existence; they are tested with passing and failing inputs.
- [ ] No `as` silences an unexplained type error; every assertion has a justification, and `as unknown as T` is rare and documented.
- [ ] Untrusted data (`JSON.parse`, `fetch`, forms, URL params, `localStorage`) is validated with a schema at the boundary before being treated as its static type.
- [ ] Static types are derived from validators (`z.infer`/equivalent) where possible, so type and validator cannot drift.
- [ ] Narrowing that crosses a callback is re-established inside the callback or captured in a stable local.
- [ ] Type guards and validators were reviewed when their target types changed.
- [ ] The runtime checks actually prove the narrowed type; there is no gap between what the check verifies and what the type claims.
