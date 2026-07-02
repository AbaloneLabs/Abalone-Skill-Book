---
name: javascript_coercion_equality_and_truthiness.md
description: Use when the agent is writing or reviewing JavaScript comparison, equality, truthiness, or type conversion logic, choosing between == and ===, handling NaN checks, comparing objects/arrays by reference vs value, guarding against implicit coercion bugs, using optional chaining and nullish coalescing, or debugging "why is this undefined/NaN/true". Covers coercion rules, falsy values, Object.is, immutability and reference equality, and defensive patterns to prevent unintended conversions.
---

# Coercion, Equality, And Truthiness

JavaScript performs type coercion in many operations, and its equality and truthiness rules are full of edge cases that look reasonable in isolation and break in combination. `[] == ![]` is `true`; `typeof NaN === "number"`; `null == undefined` but `null === undefined` is false; an empty array is truthy even though it "looks empty." The judgment problem is writing comparisons and conditionals that express the *intended* comparison rather than the one the coercion rules happen to perform.

Agents tend to reach for `==` out of habit, rely on truthiness without enumerating the falsy set, compare objects with `===` expecting value equality, or write `x === NaN` (which is always false). The harm is subtle and intermittent: a form field that is the string `"0"` behaves differently than the number `0`, a default-value expression clobbers `0` and `""` because `||` treats them as falsy, an object comparison always fails because two `{}` are never `===`. The real work is choosing the comparison that matches intent, knowing the falsy set and the coercion rules, and isolating trust boundaries where untyped input arrives.

## Core Rules

### Default To Strict Equality; Justify Every `==`

`===` compares without coercion: same type and same value (or same reference for objects). `==` compares with a complex coercion table that almost never matches intent and frequently hides bugs. The rule: use `===` and `!==` everywhere, and treat any `==` as requiring an explicit justification.

The one legitimate `==` use is checking "null or undefined" together: `x == null` is true for both `null` and `undefined` and nothing else. Even this is clearer written as `x == null` with a comment, or as `x === null || x === undefined`. Every other `==` is a candidate for a bug. Enable the `eqeqeq` lint rule.

### Know The Falsy Set Exactly

Exactly seven values are falsy: `false`, `0`, `-0`, `0n` (BigInt zero), `""` (empty string), `null`, `undefined`, and `NaN`. Everything else — including `"0"`, `[]`, `{}`, `"false"`, and `new Boolean(false)` — is truthy.

This set is small and worth memorizing because most truthiness bugs come from assuming something is falsy when it is not:

- An empty array `[]` is truthy. Checking `if (items)` to mean "has items" is wrong when `items` could be `[]`; use `items.length > 0`.
- The string `"0"` is truthy. A form value of `"0"` is not empty.
- `new Boolean(false)` is an object and thus truthy. Never use the `Boolean` object wrapper.

### Handle NaN Correctly

`NaN` is a numeric value meaning "not a number," and it is the only value not equal to itself: `NaN === NaN` is `false`. This makes naive NaN checks fail.

- Use `Number.isNaN(x)` (not the global `isNaN`, which coerces and returns true for non-numeric strings too).
- `Object.is(x, NaN)` also works, and `Object.is` is the precise equality function (it also distinguishes `-0` from `0`, unlike `===`).
- NaN propagates through arithmetic: any math involving NaN yields NaN. Validate inputs before computing, and check results when a NaN would corrupt downstream logic.

### Compare Objects And Arrays By Value Deliberately

`===` on objects and arrays compares references, not contents. Two separately-constructed `[]` or `{}` are never `===`. This is the source of countless "why doesn't this comparison work" bugs.

- For "is this the same object instance," `===` is correct.
- For "do these have the same contents," you need a deep-equal utility (`structuredEqual`, a library, or a manual comparison), or a structural check appropriate to the shape.
- For primitives, `===` compares values directly.

When you store objects in a `Set` or as `Map` keys, identity is used, so two equal-by-value objects are treated as different. If you need value-based uniqueness, normalize to a primitive key (an id, a serialized form) or use a deep-equal structure.

### Use Optional Chaining And Nullish Coalescing For Defaults

Modern JS gives precise tools that avoid the coercion traps of `||` and manual `&&` chains.

- `a?.b?.c` short-circuits to `undefined` if any link is nullish, instead of throwing. Use it for deep property access on uncertain shapes.
- `a ?? b` returns `b` only when `a` is `null` or `undefined` (nullish), preserving other falsy values like `0`, `""`, and `false`. This is usually what you want for defaults.
- `a || b` returns `b` for *any* falsy `a`, so `count || 10` clobbers a legitimate `0`. Prefer `??` for defaults; reserve `||` for when you genuinely want to replace all falsy values.

### Isolate Trust Boundaries Where Untyped Data Arrives

Coercion bugs concentrate where untyped data enters: form inputs (always strings), URL params, `localStorage`, `JSON.parse`, query results. At these boundaries, convert and validate explicitly rather than letting coercion happen later.

- Convert form strings to numbers explicitly (`Number(value)`, `parseInt(value, 10)`) and validate the result.
- Check for the values you expect, not just truthiness; `Number("")` is `0`, which may or may not be intended.
- Treat the boundary as the place where types become known; downstream code can then rely on them.

### Distinguish Arithmetic And String Concatenation

The `+` operator is overloaded: if either operand is a string, it concatenates; otherwise it adds. This single overloading causes most "why did my number become a string" bugs.

- Ensure both operands are numbers when you mean addition; convert explicitly (`Number(a) + Number(b)`).
- Be careful with values from forms/URLs, which are strings; `a + b` where both are strings concatenates.
- For sums, `a - 0 + (b - 0)` or `Number(a) + Number(b)` is unambiguous; for template strings, use interpolation intentionally.

## Common Traps

### `==` Across Types

`0 == ""`, `0 == "0"`, `false == "0"`, `null == undefined` — the coercion table produces surprising results. The fix is universal: use `===`. If you genuinely need loose comparison, write the explicit conversions so the intent is visible.

### Truthiness Used As Emptiness

`if (items)`, `if (str)`, `if (count)` treat `0`, `""`, and `[]` as empty, which is often wrong. Check the specific emptiness you mean: `items.length`, `str !== ""`, `count > 0`.

### `||` Clobbering Valid Falsy Values

`const limit = options.limit || 10` sets `limit` to `10` when `options.limit` is `0`. Use `??` for defaults: `options.limit ?? 10`.

### `===` Expecting Value Equality For Objects

`obj1 === obj2` is false for two equal-by-value objects. Use a deep-equal check or compare a stable identifier. The same applies to array includes with objects: `[obj].includes(otherObj)` checks identity.

### `typeof null === "object"`

`typeof` reports `"object"` for `null` (a historical bug) and for arrays. To detect arrays use `Array.isArray(x)`; to detect null use `x === null`. `typeof` is reliable only for primitives (`"string"`, `"number"`, `"boolean"`, `"undefined"`, `"symbol"`, `"bigint"`, `"function"`).

### Implicit Numeric Coercion Via Unary `+` And `-`

`+"5"` is `5`, but `+"5px"` is `NaN`, and `+[]` is `0`, `+[1]` is `1`, `+[1,2]` is `NaN`. Coercion through `+x` is a frequent source of NaN. Prefer `Number(x)` for clarity and validate the result.

### `NaN` Propagation

A single NaN in a sum makes the whole result NaN, and NaN comparisons are all false, so guards like `if (total)` silently fail. Validate inputs and check `Number.isNaN` on results that must be finite.

### Comparing `-0` And `0`

`-0 === 0` is `true`, but the two behave differently (`1/-0` is `-Infinity`, `Object.keys` ordering, sign in stringification). Use `Object.is(x, -0)` if the distinction matters (math, graphics, serialization).

## Self-Check

- [ ] All equality uses `===`/`!==`; every `==` has an explicit justification (typically the `x == null` null/undefined check).
- [ ] Truthiness checks distinguish the seven falsy values; `if (items)` is not used to mean "non-empty array," and `0`/`""` are not accidentally treated as absent.
- [ ] NaN is checked with `Number.isNaN` or `Object.is(x, NaN)`, never with `x === NaN`; NaN propagation is guarded at boundaries.
- [ ] Object/array comparisons use `===` only for identity; value equality uses a deep-equal utility or a stable identifier.
- [ ] Defaults use `??` to preserve `0`/`""`/`false`; `||` is used only when replacing all falsy values is intended.
- [ ] Optional chaining (`?.`) is used for deep access on uncertain shapes without throwing.
- [ ] Untyped input (forms, URL params, `localStorage`, `JSON.parse`) is explicitly converted and validated at the boundary before downstream code relies on a type.
- [ ] The `+` operator's string/number ambiguity is resolved by explicit `Number()` conversion where operands may be strings.
- [ ] Array detection uses `Array.isArray`; null detection uses `x === null`; `typeof` is relied on only for primitive types.
- [ ] No code depends on `typeof null` being `"object"` or on `-0 === 0` where the distinction matters.
