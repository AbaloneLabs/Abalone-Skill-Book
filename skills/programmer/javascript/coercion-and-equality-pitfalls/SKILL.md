---
name: javascript_coercion_and_equality_pitfalls.md
description: Use when the agent is reasoning about JavaScript's abstract operations and coercion mechanics — ToPrimitive, valueOf, toString, Symbol.toPrimitive, the abstract equality algorithm, ordering of coercion in operators, loose vs strict comparison internals, or debugging deeply surprising comparison results like [] == ![] being true. Covers the underlying coercion algorithms, operator overloading via well-known symbols, the precedence and direction of coercion, and defensive patterns at trust boundaries. Distinct from everyday equality/truthiness rules, which are covered separately.
---

# Coercion And Equality Pitfalls: The Abstract Operations

JavaScript's coercion is not random — it follows defined *abstract operations* (`ToPrimitive`, `ToNumber`, `ToString`, `ToBoolean`) and an *Abstract Equality Algorithm* that together produce results that look bizarre (`[] == ![]` is `true`) but are deterministic once you know the rules. The judgment problem is reasoning about coercion at the level of these algorithms: which conversion fires first, whether `valueOf` or `toString` is preferred, how operators order their coercions, and how user-defined `Symbol.toPrimitive`/`valueOf`/`toString` can intercept the process.

Agents tend to dismiss coercion as "just use `===`" and move on, which works for everyday code but leaves them unable to explain or defend against the genuinely surprising cases — especially when objects, `Date`, `Symbol`, or user-defined conversion methods are involved, or when a library overrides `valueOf`. The harm appears as comparisons that pass or fail inexplicably, sorting that orders objects by an unintended key, arithmetic that triggers `toString` instead of `valueOf`, and security-relevant bugs where attacker-controlled objects coerce in unexpected ways. The real work is understanding the abstract operations, knowing when coercion is invoked and in what order, and isolating trust boundaries where untrusted objects could hijack coercion.

## Core Rules

### Know The Abstract Operations And Their Order

Every coercion funnels through a small set of abstract operations. Knowing them lets you predict any conversion.

- **`ToPrimitive(input, hint)`**: converts an object to a primitive. It calls methods on the object in a defined order to get a primitive. The `hint` is `"number"`, `"string"`, or `"default"`.
- **`ToNumber`**: converts to a number. For objects, it calls `ToPrimitive` with hint `"number"`, then converts the resulting primitive. `ToNumber("")` is `0`, `ToNumber([])` is `0`, `ToNumber([1])` is `1`, `ToNumber([1,2])` is `NaN`, `ToNumber({})` is `NaN`.
- **`ToString`**: converts to a string. For objects, it calls `ToPrimitive` with hint `"string"`, then converts. `ToString([])` is `""`, `ToString([1])` is `"1"`, `ToString({})` is `"[object Object]"`.
- **`ToBoolean`**: converts to boolean. This is the simple one: the seven falsy values (`false`, `0`, `-0`, `0n`, `""`, `null`, `undefined`, `NaN`) become `false`; everything else (including `[]`, `{}`, `"0"`) becomes `true`.

The surprising results (`[] == 0`, `[1] == 1`) all reduce to `ToNumber([])` being `0` and `ToNumber([1])` being `1`. Once you can compute `ToNumber`/`ToString` for a value, the comparison table becomes predictable rather than mysterious.

### Understand `ToPrimitive`, `valueOf`, And `toString`

`ToPrimitive` is where user-defined behavior enters coercion. For an object with a given hint, it tries methods in order:

- If `Symbol.toPrimitive` exists, it is called with the hint and its return value is used (must be a primitive, else `TypeError`).
- Otherwise, for hint `"number"`: `valueOf` is tried first, then `toString`. For hint `"string"`: `toString` first, then `valueOf`. For `"default"`: `valueOf` then `toString`.
- If the first method returns a non-primitive, it falls through to the next; if all return non-primitives, `TypeError`.

This means:

- **Arithmetic and equality** (`+`, `-`, `==`, `<`) use hint `"number"` or `"default"`, so `valueOf` is preferred. A `Date` overrides this: for `==` and `+`, `Date` uses `"default"` which it maps to `toString`.
- **String contexts** (template literals, `String(x)`, property keys) use hint `"string"`, so `toString` is preferred.
- A user-defined class can override `valueOf`/`toString`/`Symbol.toPrimitive` to control how it coerces — which is also an attack surface if untrusted objects reach arithmetic or comparison.

When a comparison surprises you, ask: which hint does this operator use, and what do the object's `valueOf`/`toString` return?

### Trace The Abstract Equality Algorithm Step By Step

`==` (loose equality) runs the Abstract Equality Algorithm, which coercively compares. The full algorithm is large, but the key moves are:

- If types differ, it coerces toward a common type: `Number` vs `String` converts the string to number; `Boolean` vs anything converts the boolean to number (`false`→`0`); `Object` vs `String`/`Number`/`BigInt`/`Symbol` converts the object to primitive via `ToPrimitive`.
- `null == undefined` is `true`, and each is only loosely equal to the other (and itself via `===`).
- `NaN` is loosely equal to nothing (even itself).

Walking `[] == ![]`: `![]` is `false` (empty array is truthy, negated), so it becomes `[] == false`. `false` coerces to `0`, so `[] == 0`. The array coerces to primitive with hint default→`ToNumber([])` = `0`. So `0 == 0` is `true`. The algorithm is deterministic; the surprise is only that each step changes types.

The practical rule remains "use `===`," but understanding the algorithm lets you *explain* and *defend* rather than just avoid.

### Watch Operator Coercion Direction And Overloading

Operators coerce their operands in defined directions, and the `+` operator is uniquely ambiguous.

- **`+`**: if either operand is a string (after `ToPrimitive`), it concatenates; otherwise it adds numbers. `[1,2] + [3,4]` is `"1,23,4"` because both arrays coerce to strings. `{} + {}` varies by context (block vs expression). This is why `+` is the most error-prone operator.
- **`-`, `*`, `/`, `%`**: always numeric. `ToPrimitive` then `ToNumber` on both operands. `[1] - [1]` is `0` because both coerce to `1`.
- **`<`, `>`, `<=`, `>=`**: if both operands become strings after `ToPrimitive`, compare lexicographically; otherwise convert both to numbers. `["10"] < ["9"]` is `true` because both coerce to strings `"10"` and `"9"`, and `"10" < "9"` lexicographically.

The relational operators' string-vs-number switch is a frequent source of sorting bugs: an array of numeric strings sorts lexicographically unless converted.

### Isolate Trust Boundaries Where Coercion Is Hijackable

Because `valueOf`/`toString`/`Symbol.toPrimitive` are user-overridable, any object that reaches arithmetic, comparison, or string interpolation can run arbitrary code via its coercion methods. This is a real attack and reliability surface.

- `JSON.parse` produces plain objects, but deserialized data passed through libraries that call `valueOf`/`toString` on unknown shapes can trigger unexpected behavior. Prefer explicit extraction (`Number(x)`, `String(x)`, `x.valueOf()` you control) at boundaries.
- Never let attacker-controlled objects reach `==`, `+`, or template literals without sanitization; a malicious `toString` can cause side effects or skew comparisons.
- For money, coordinates, or any value where exact representation matters, never rely on coercion; use explicit types (BigInt, a decimal library, typed structures) and explicit conversions.

### Use Explicit Conversion Over Implicit Coercion

The robust pattern is to convert explicitly at the point where a type is needed, so the intent is visible and coercion does not happen silently downstream.

- `Number(x)` instead of `+x` or `x - 0`; `String(x)` instead of `x + ""`; `Boolean(x)` instead of `!!x` (though `!!` is idiomatic and acceptable).
- Validate at trust boundaries (form input, URL params, parsed JSON) and convert to the known type there, so inner code relies on a real type rather than coercing repeatedly.
- Explicit conversion is auditable; implicit coercion hides which abstract operation fired and in what order.

## Common Traps

### `[] == ![]` Being `true`

Each step coerces a type (`![]`→`false`→`0`, `[]`→`0`), yielding `0 == 0`. The algorithm is deterministic; the lesson is to use `===` and not rely on loose equality with objects.

### `+` Concatenating When You Meant Addition

If either operand is a string after `ToPrimitive`, `+` concatenates. `[1] + [2]` is `"12"`. Convert both operands to numbers explicitly when addition is intended.

### Relational Operators Comparing Strings Lexicographically

`"10" < "9"` is `true` (string comparison), but `10 < 9` is `false`. Numeric strings sort wrong unless converted; always `Number()` before comparing quantities.

### `valueOf` Overridden Changing Arithmetic

A class that overrides `valueOf` changes how `+`, `==`, and `<` treat its instances. Untrusted objects with overridden `valueOf` can skew comparisons or run side effects; sanitize at boundaries.

### `Date` Coercing Via `toString` For `==` And `+`

Unlike other objects, `Date` uses `toString` (not `valueOf`) for `==` and `+`, so `new Date(0) == new Date(0)` is `false` (different strings) while `+d1 === +d2` is `true` (numeric timestamp). Compare dates by their numeric value.

### `ToNumber({})` Being `NaN`

Plain objects coerce to `NaN` numerically, silently poisoning arithmetic. Detect and reject objects where numbers are expected.

### `Symbol.toPrimitive` Returning A Non-Primitive

If a custom `Symbol.toPrimitive` returns an object, it throws `TypeError`. Ensure conversion methods return primitives.

### `null == 0` Being `false`

`null` is only loosely equal to `undefined` (and itself via `===`); `null == 0` is `false` despite both being "falsy." Check `x == null` for null-or-undefined, or explicit `=== null`.

## Self-Check

- [ ] Coercion is reasoned about via the abstract operations (`ToPrimitive`, `ToNumber`, `ToString`, `ToBoolean`), not dismissed as random; surprising comparisons can be explained step by step.
- [ ] `ToPrimitive`'s hint system and `valueOf`/`toString`/`Symbol.toPrimitive` precedence are understood; the operator in use determines the hint.
- [ ] The `+` operator's string-vs-number ambiguity is resolved by explicit `Number()` conversion where addition is intended.
- [ ] Relational comparisons (`<`, `>`) convert numeric strings to numbers before comparing quantities, avoiding lexicographic surprises.
- [ ] `Date` is compared by its numeric timestamp (`+d` or `d.getTime()`), not by `==` (which uses `toString`).
- [ ] Trust boundaries (parsed JSON, form input, untrusted objects) sanitize and explicitly convert before the value reaches arithmetic, comparison, or interpolation.
- [ ] `valueOf`/`toString`/`Symbol.toPrimitive` overrides are treated as a coercion attack surface; untrusted objects do not reach operators unsanitized.
- [ ] Money, coordinates, and exact-representation values use explicit types (BigInt, decimal libraries), never implicit coercion.
- [ ] Explicit conversion (`Number(x)`, `String(x)`, `Boolean(x)`) is preferred over implicit coercion at points where a type is needed, for auditability.
- [ ] Loose equality (`==`) is avoided except the deliberate `x == null` null-or-undefined check; `===` is the default and every `==` is justified.
