---
name: typescript_enums_unions_and_literals.md
description: Use when the agent is choosing between TypeScript string enums, numeric enums, const enums, union of string literals, or `as const` objects to model a fixed set of values, deciding exhaustiveness checking strategy, converting between enum names and values, handling enum reverse mapping, migrating from enums to union types, or debugging enum issues with tree-shaking, isolation, and runtime behavior. Covers enum vs literal-union vs const-object tradeoffs, exhaustiveness via never, reverse mapping, runtime cost, and the decision of when a union of literals outperforms an enum.
---

# Enums, Unions, And Literals

TypeScript offers several ways to model a fixed set of values: numeric enums, string enums, `const enums`, unions of string literals, and `as const` objects. They differ in runtime behavior, exhaustiveness checking, serialization, tree-shaking, and how they interact with plain JavaScript. The judgment problem is choosing the representation that gives you the safety you need (exhaustiveness, named members) without paying costs you do not want (runtime objects, isolation issues, reverse-mapping surprises).

Agents tend to default to string enums everywhere, use numeric enums that reverse-map and leak values, rely on `const enum` that breaks under `isolatedModules`, or miss that a union of literals often gives the same safety with no runtime cost. The harm appears as enums that do not tree-shake, `const enum` failures across tooling, exhaustive switches that silently miss a new case, and serialization mismatches when an enum's runtime value differs from its name. The real work is matching the representation to the need for naming, exhaustiveness, and runtime presence.

## Core Rules

### Understand The Four Representations And Their Runtime Behavior

The options have sharply different runtime footprints, which drives the choice.

- **Numeric enum** (`enum E { A, B, C }`): compiles to a runtime object with both forward (`E.A` → `0`) and reverse (`E[0]` → `"A"`) mapping. Reverse mapping is a frequent surprise and a serialization hazard. Numeric values are not self-describing.
- **String enum** (`enum E { A = "a", B = "b" }`): compiles to a runtime object with forward mapping only (no reverse). Values are self-describing in logs and serialized data. Still a runtime object.
- **`const enum`** (`const enum E { A = "a" }`): inlined at compile time to the literal value, no runtime object. Fast and zero-cost, but breaks under `isolatedModules`/`verbatimModuleSyntax` and across some bundlers, because each file must see the enum's values to inline them.
- **Union of string literals** (`type Direction = "north" | "south" | "east" | "west"`): zero runtime cost (it is just a string), full exhaustiveness checking, no runtime object. No named member access (`Direction.NORTH` does not exist).
- **`as const` object** (`const Direction = { North: "north", South: "south" } as const; type Direction = typeof Direction[keyof typeof Direction]`): gives named member access and a runtime object, with literal-typed values. A common enum replacement that tree-shakes well.

The default for new code should usually be a union of literals (zero cost, full safety) or an `as const` object (named access plus runtime values). Reach for a real `enum` only when you specifically need its features.

### Prefer Union Of Literals When You Do Not Need Named Member Access

A union of string literals gives exhaustiveness checking and zero runtime cost. It is the lightest representation.

- `type Status = "pending" | "active" | "archived"` — no runtime object, full type safety.
- Exhaustiveness works: `switch (status) { case "pending": ...; case "active": ...; case "archived": ...; default: const _exhaustive: never = status; }` forces a compile error if a case is missing.
- The cost: no `Status.Pending` named constant, so you type the literal `"pending"` everywhere. For small, stable sets this is fine; for large sets or sets referenced in many places, the lack of named access hurts.

Use a union of literals when the set is small, stable, and you do not mind typing strings. It is the simplest, cheapest option.

### Use `as const` Object When You Want Named Access And Runtime Values

When you need named member access (`Direction.North`) and/or a runtime iterable of values, an `as const` object is a robust enum replacement.

- `const Color = { Red: "red", Green: "green", Blue: "blue" } as const; type Color = typeof Color[keyof typeof Color];` gives both the value object and the union type.
- `Object.values(Color)` iterates the values at runtime, which a union of literals cannot do.
- It tree-shakes well and works under `isolatedModules`, unlike `const enum`.

This pattern gives most of what enums offer without the pitfalls. Prefer it over real enums in most new code.

### Use Real Enums Deliberately, Knowing Their Costs

Real enums are still valid when you want the language-level construct, but understand the costs.

- **String enums** are self-describing and have no reverse mapping; reasonable when you want named access and a runtime object.
- **Numeric enums** reverse-map (`E[0]` → `"A"`), which is rarely wanted and can leak names into serialized data. Prefer string enums or `as const` over numeric enums.
- **`const enum`** inlines values and has no runtime cost, but it requires that every consuming file can see the enum definition at compile time. Under `isolatedModules` (required by many bundlers and by `verbatimModuleSyntax`), `const enum` is discouraged or erased unpredictably. Avoid `const enum` in library code that consumers compile with different settings.

If you choose a real enum, document why its features (the construct itself, reverse mapping, inlining) justify the cost over a union or `as const` object.

### Enforce Exhaustiveness With `never`

Whichever representation you choose, exhaustiveness checking catches missing cases when the set of values grows. The pattern is a `switch` with a `default` that assigns to `never`:

- `switch (kind) { case A: ...; case B: ...; default: const _: never = kind; throw new Error(\`unhandled: ${kind}\`); }`
- When a new value `C` is added to the union, the `default` no longer type-checks (because `kind` can be `C`, not `never`), forcing you to handle it.

Without this, adding a new variant silently leaves a switch incomplete. Always wire exhaustiveness for closed sets handled by switch.

### Be Careful Serializing And Comparing Enums

Enum runtime values differ from their names, and numeric enums reverse-map. When an enum crosses a serialization boundary (JSON, API), only the runtime value travels.

- A string enum `Status.Active = "active"` serializes as `"active"`, which is self-describing — good.
- A numeric enum `Status.Active = 1` serializes as `1`, opaque without the mapping — usually bad for APIs.
- Comparing an incoming API value to an enum: the API sends the runtime value (`"active"` or `1`), not the member name (`Active`). Validate and narrow at the boundary.

For data that crosses API boundaries, prefer string-valued representations (string enums, literal unions, `as const` with string values) so the serialized form is self-describing.

## Common Traps

### `const enum` Breaking Under `isolatedModules`

`const enum` requires cross-file inlining, which `isolatedModules` and many bundlers cannot guarantee. Prefer `as const` objects or literal unions, especially in libraries.

### Numeric Enum Reverse Mapping Leaking Names

`E[0]` returns `"A"`, which can leak into serialized data or logs unexpectedly. Prefer string enums or literal unions.

### Exhaustive Switch Missing A New Case

Without a `never`-based default, adding a variant does not flag the incomplete switch. Always wire exhaustiveness for closed sets.

### Comparing Enum Name Instead Of Value

An API sends `"active"` (the value), not `Active` (the member name). Compare against the runtime value, not the name.

### Defaulting To Enums When A Union Suffices

Many fixed sets need only exhaustiveness, which a literal union provides at zero runtime cost. Do not reach for a runtime enum object when a type-level union is enough.

### Enum Not Tree-Shaking

Real enums compile to runtime objects that bundlers may not eliminate. Unions and `as const` objects (when values are unused) shake better.

## Self-Check

- [ ] The representation is chosen deliberately: literal union for zero-cost type-only sets, `as const` object for named access plus runtime values, real enum only when its specific features are needed.
- [ ] `const enum` is avoided (especially in libraries) due to `isolatedModules`/`verbatimModuleSyntax` inlining problems.
- [ ] Numeric enums are avoided or their reverse mapping is understood and not leaked into serialized data.
- [ ] Exhaustiveness is enforced via a `never`-typed default in switches over closed sets, so new variants produce a compile error.
- [ ] Enum/runtime values used at serialization boundaries are self-describing (string-valued), not opaque numeric codes.
- [ ] Comparisons against incoming data use the runtime value, not the member name.
- [ ] The runtime cost of the chosen representation (object vs inlined vs none) is understood and justified for the use case.
