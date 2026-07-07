---
name: typescript_type_level_testing.md
description: Use when the agent is writing or reviewing TypeScript type-level code (conditional types, mapped types, template literal types, inference helpers, branded types, utility types) and needs to verify the types behave as intended, writing type-level tests with expect-type/tsd/vitest expectTypeOf, debugging "the type computes wrong but there is no runtime to test", asserting assignability and non-assignability, checking error messages, or diagnosing type tests that pass at runtime but miss type bugs. Covers type-level test design, assignability assertions, the gap between type tests and runtime tests, and when type-level cleverness should be replaced by runtime validation.
---

# Type-Level Testing In TypeScript

TypeScript's type system is Turing-complete enough to express real computations: conditional types that branch, mapped types that transform shapes, `infer` clauses that extract pieces, template literal types that build strings, and recursive types that walk structures. Once a type becomes non-trivial, it has bugs — the wrong branch taken, an inference that widens unexpectedly, a constraint that admits values it should reject — and these bugs are invisible to ordinary runtime tests, because the type system is erased at runtime. A type that "looks right" in an IDE hover can silently accept invalid input or reject valid input across an entire API surface, and the only way to catch that is to test the type itself: assert that specific values are assignable, assert that wrong values are not, and assert that the inferred type is exactly the one intended. The judgment problem is to treat complex types as code that needs tests, to write those tests as executable assertions (not comments), and to recognize the boundary where type-level testing is papering over a type that should be simpler or backed by runtime validation.

Agents frequently write a clever generic, confirm it produces the right hover in one example, and ship it. The type then misbehaves on edge cases (empty objects, optional fields, unions, `any` leaking in) that were never checked, and because there is no runtime, no test fails. The remedy is to add a type test suite using a library (`expect-type`, `tsd`, `vitest`'s `expectTypeOf`) that turns assignability into pass/fail assertions, to test both the positive (this should be accepted) and negative (this should be rejected) directions, and to keep the type simple enough that its tests are readable.

## Core Rules

### Treat Non-Trivial Types As Code That Requires Tests

If a type uses conditionals, `infer`, mapped types, recursion, or template literal types beyond a trivial utility, it is logic and it has edge cases. Write type-level tests for it the way you would write unit tests for a function. A type that is only "checked by hovering" is unchecked.

- Test the happy path (a typical input yields the expected output type).
- Test edge cases: empty object, all-optional, union members, `null`/`undefined`, `any`, `unknown`.
- Test both directions: values that should be assignable, and values that should produce a compile error.

### Assert Both Assignability And Non-Assignability

The two fundamental type-level assertions are "this is assignable" and "this is not assignable." Most libraries express them as `expectTypeOf<T>().toMatchTypeOf<U>()` (T is assignable to U) and `expectTypeOf<T>().not.toMatchTypeOf<U>()` (T is not assignable to U), or `tsd`'s `expectAssignable`/`expectNotAssignable` and `expectError` around an assignment.

- Positive assertion: `expectTypeOf<{ a: 1 }>().toMatchTypeOf<{ a: number }>()`.
- Negative assertion: `expectTypeOf<{ a: string }>().not.toMatchTypeOf<{ a: number }>()`.
- Non-assignability is the assertion that catches the most bugs: a type that is too permissive admits invalid input, and only a "this should be an error" test catches it.

### Assert Exact Types, Not Just Compatibility

Assignability ("T is assignable to U") is weaker than equality ("T is exactly U"). A function that should return `{ a: number; b: string }` but actually returns `{ a: number }` is assignable to a looser target and passes a compatibility test. Use exact-type assertions (`expectTypeOf<T>().toEqualTypeOf<U>()`) when the precise shape matters — for public APIs, library exports, and serialized contracts where extra or missing fields are a bug.

- `toEqualTypeOf` checks mutual assignability (two-way), catching both missing and extra properties.
- Use it for return types of public functions and for types that cross a serialization boundary (API responses), where the exact shape is the contract.

### Test Inference, Not Just Annotation

A generic function's value is in what it infers; test the inferred type of a call, not just the declared signature. `const result = myFn(input); expectTypeOf(result).toEqualTypeOf<ExpectedType>()` verifies inference works. This catches the common bug where the declared signature is correct but inference widens, narrows wrongly, or defaults a type parameter unexpectedly.

- Test inference with concrete inputs: `const r = pick({ a: 1, b: 'x' }, 'a'); expectTypeOf(r).toEqualTypeOf<number>()`.
- Test that inference preserves literal types where intended (often requires `const` type parameters or `as const` input), and widens where intended.

### Run Type Tests In CI As A Compile Step, Not Just In An IDE

A type test that lives only in a hover is not enforced. Type test libraries integrate into the build: `tsd` runs as a CLI over `.test-d.ts` files, `expect-type`/`vitest`'s `expectTypeOf` run as part of the test suite under `tsc` (or vitest's type-check mode). Wire them into CI so a type regression fails the build. The assertions are compile-time: a failing assertion is a type error, surfaced by the compiler.

### Recognize When A Type Should Be Runtime-Validated Instead

Type-level tests verify the static type, but the static type is only as good as the data's provenance. If the input comes from `JSON.parse`, an API response, `localStorage`, or any untrusted boundary, the runtime value may not match its type, and no type test catches that. At such boundaries, pair the type with a runtime validator (zod, valibot, io-ts) that parses and narrows, and test the validator with runtime tests. Type-level testing is for the type system's internal correctness; runtime validation is for data that crosses a trust boundary.

## Common Traps

### Only Testing The Happy Path

A type tested only on a typical input passes while failing on empty objects, optional fields, unions, or `any`. Always include edge-case inputs in the type test suite.

### Compatibility Asserts Where Equality Is Needed

`toMatchTypeOf` passes when the actual type is narrower than expected, hiding missing properties. Use `toEqualTypeOf` for exact-shape contracts (API responses, public exports).

### Forgetting The Negative Direction

Testing only "this is assignable" never catches a type that is too permissive. Always add "this should NOT be assignable" assertions for invalid input.

### any Silently Satisfies Everything

A type leak that produces `any` makes every assignability assertion pass (`any` is assignable to and from everything). Add an explicit assertion that the result is not `any`/`unknown` where purity matters, and enable `noImplicitAny`/strict checks so `any` does not sneak in.

### Type Tests That Pass But Miss Runtime Mismatch

A type test verifies the static type only. If the data comes from `JSON.parse` or an API, the runtime may disagree with the type. Add runtime validation at trust boundaries; do not rely on the type alone.

### Over-Asserting Implementation Details

Testing that an internal helper produces exactly some intermediate type couples the test to implementation. Test the public inferred types; let internals vary.

### Untested Inference

The declared signature is correct, but inference widens `string` from a literal or loses a discriminant. Always assert the inferred type of a real call, not just the annotated signature.

## Self-Check

- [ ] Every non-trivial type (conditional, mapped, `infer`, recursive, template literal) has a type-level test suite covering happy path, edge cases (empty, optional, union, `any`), and both assignability directions.
- [ ] Positive and negative assertions are present: valid values are asserted assignable, and invalid values are asserted to produce compile errors.
- [ ] Public API return types and serialized contracts use exact-type assertions (`toEqualTypeOf`) so missing or extra properties fail the test.
- [ ] Inference is tested by asserting the inferred type of concrete calls, including literal-type preservation and widening behavior.
- [ ] Type tests run in CI as a compile step (tsd, expect-type, vitest type-check), so a regression fails the build rather than surviving in a hover.
- [ ] Data crossing a trust boundary (`JSON.parse`, API, storage) is backed by a runtime validator with runtime tests, not by the static type alone.
- [ ] An assertion guards against `any`/`unknown` leakage where type purity matters, and strict compiler options prevent implicit `any`.
- [ ] The type has been considered for simplification: if its tests are unreadable or it exists only to defeat a limitation, a simpler type or runtime check was evaluated.
