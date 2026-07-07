---
name: javascript_numbers_and_floating_point.md
description: Use when the agent is doing arithmetic, money, or precision-sensitive computation in JavaScript, reasoning about IEEE 754 double precision, the safe integer range (Number.MAX_SAFE_INTEGER), why 0.1 + 0.2 !== 0.3, choosing between Number, BigInt, and libraries (decimal.js, dinero.js), parsing integers and floats with radix, formatting numbers for display (Intl.NumberFormat), handling NaN and Infinity, or debugging off-by-a-cent financial bugs. Covers floating-point representation, the safe integer boundary, integer vs decimal money handling, radix and parsing, display formatting, and the tradeoff between native Number performance and exact decimal correctness.
---

# Numbers And Floating Point

JavaScript has a single numeric type, `Number`, which is an IEEE 754 double-precision (64-bit) floating point. This unifies integer and decimal arithmetic under one type but inherits every floating-point pitfall: fractional values that do not represent exactly, a finite safe-integer range, and silent precision loss past it. The judgment problem is knowing when native `Number` is adequate and when it will silently corrupt results, especially for money and other precision-sensitive domains.

Agents tend to use `Number` for money and hit `0.1 + 0.2 !== 0.3`, exceed `Number.MAX_SAFE_INTEGER` when handling large IDs, parse with `parseInt` without a radix, or compare floats for equality directly. The harm appears as off-by-a-cent financial bugs, corrupted database IDs, NaN propagating through calculations, and locale-wrong number formatting. The real work is recognizing the floating-point boundary, choosing integer-cents or a decimal library for money, respecting the safe-integer limit, and parsing and formatting deliberately.

## Core Rules

### Recognize That `Number` Is IEEE 754 Double, With All Its Limits

Every JavaScript `Number` is a 64-bit float. Integers up to 2^53 − 1 are represented exactly; beyond that, integers lose precision. Most decimal fractions (0.1, 0.2, 0.3) cannot be represented exactly and accumulate small errors.

- `0.1 + 0.2` evaluates to `0.30000000000000004`, not `0.3`. This is not a bug; it is the representation.
- `Number.MAX_SAFE_INTEGER` is `2^53 - 1` (`9007199254740991`). Above it, integers like `9007199254740992` and `9007199254740993` compare equal because they map to the same float.
- Operations do not commute or associate exactly: `(a + b) + c` may differ from `a + (b + c)` in the last digits.

The first step in any numeric code is asking: "does the domain tolerate float error, or does it need exactness?" Money, IDs, and counters usually need exactness; measurements and statistics usually tolerate float.

### Do Not Use Floating-Point For Money

Financial calculations must be exact. Representing dollars as a float guarantees accumulated rounding errors that surface as wrong totals, failed reconciliations, and audit failures.

- **Integer minor units**: store money as integer cents (or basis points) and only convert to a decimal for display. `1099` cents, not `10.99` dollars. Arithmetic on integers is exact within the safe range.
- **Decimal library**: use `decimal.js`, `big.js`, `dinero.js`, or equivalent for arbitrary-precision decimal arithmetic when you need dollars directly or complex rounding rules.
- **Never** store, transmit, or compute money as a float and hope rounding at the end fixes it. The errors compound and sometimes round the wrong way.

This is the single most common numeric bug. If the field is money, integers or a decimal library, never raw float.

### Respect The Safe Integer Boundary For IDs And Counters

Large identifiers (database IDs, snowflake IDs, timestamps in nanoseconds) can exceed `2^53 - 1`. A JSON API returning a 64-bit integer ID is silently rounded when parsed into a JavaScript `Number`, so two distinct IDs become equal.

- If an ID may exceed `MAX_SAFE_INTEGER`, receive and transmit it as a string, and use `BigInt` for any arithmetic.
- `Number.isSafeInteger(x)` checks whether a value is an exactly-representable integer. Use it to validate IDs from external sources.
- Timestamps in milliseconds are safe until the year 2085-ish; nanosecond timestamps are not safe and need `BigInt` or string handling.

Silent ID collision from float parsing is a real production bug in systems with large numeric keys. Receive big IDs as strings.

### Use `BigInt` For Arbitrarily Large Integers, Not For Decimals

`BigInt` represents integers of arbitrary precision exactly. Use it for large IDs, cryptographic values, and combinatorial counts that exceed the safe range.

- `BigInt` has no fractional part; it cannot represent decimals. It is for integers only.
- `BigInt` does not mix with `Number` in arithmetic (`1n + 1` throws). Convert deliberately.
- `BigInt` is slower than `Number` for operations within the safe range; use `Number` where it suffices and `BigInt` only when the range demands.

Do not reach for `BigInt` for money (it has no decimals) — use integer minor units or a decimal library instead.

### Parse With An Explicit Radix And Validate Results

`parseInt(value)` without a radix interprets strings with leading zeros as octal in old environments and is generally error-prone. Always pass the radix: `parseInt(value, 10)`.

- `parseInt("010")` may yield `8` (octal) in legacy mode; `parseInt("010", 10)` yields `10`.
- `parseInt` stops at the first non-numeric character: `parseInt("10px")` is `10`, which can mask malformed input. Prefer `Number(value)` or `Number.parseFloat` and validate the whole string when you need strict parsing.
- `NaN` indicates parse failure. Check with `Number.isNaN(x)` (not the global `isNaN`, which coerces). `NaN` propagates: any arithmetic with `NaN` yields `NaN`, so one bad parse poisons downstream results.

Validate parsed input explicitly rather than trusting that `parseInt` returned a meaningful number.

### Format Numbers For Display With `Intl.NumberFormat`

Display formatting has locale, currency, grouping, and rounding concerns. Hand-rolling string formatting produces wrong grouping for locales and wrong currency symbols.

- `Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD' }).format(1099 / 100)` produces locale-correct currency.
- Use `Intl.NumberFormat` for grouping separators, significant digits, and unit formatting. It respects the user's locale.
- Avoid `toFixed(n)` for money display without understanding its rounding (it uses a specific rounding mode) and locale (it always uses a dot decimal, wrong for many locales).

### Handle `NaN`, `Infinity`, And `-0` Deliberately

`NaN` (not-a-number) results from failed numeric operations (`0/0`, `parseInt("abc")`, `Math.sqrt(-1)`). It is the only value not equal to itself: `NaN === NaN` is `false`. Check with `Number.isNaN`. `Infinity` results from overflow or division by zero; it propagates and may not be what you want. `-0` is a distinct value (`-0 === 0` is true, but `Object.is(-0, 0)` is false) and can surprise sign-sensitive code.

Validate inputs to catch `NaN` and `Infinity` early; do not let them propagate into stored results.

## Common Traps

### Money As Float

Storing or computing currency as a `Number` produces off-by-a-cent errors. Use integer minor units or a decimal library.

### `0.1 + 0.2 !== 0.3`

Float representation error. For exact decimal results, use integer cents or a decimal library; never compare floats for equality directly.

### Exceeding `MAX_SAFE_INTEGER` For IDs

64-bit IDs parsed into `Number` are silently rounded, causing distinct IDs to collide. Receive big IDs as strings; use `BigInt` for arithmetic.

### `parseInt` Without Radix

`parseInt("010")` may be octal. Always pass radix `10`, and validate that the whole string was consumed.

### Comparing Floats With `===`

Float errors make direct equality fail. Compare with an epsilon (`Math.abs(a - b) < epsilon`) or use exact representations.

### Global `isNaN` Coercing

`isNaN("abc")` is `true` but `isNaN("")` is `false` due to coercion. Use `Number.isNaN`, which does not coerce.

### `toFixed` For Locale Display

`toFixed` always uses a dot decimal and a fixed rounding mode, wrong for many locales and rounding rules. Use `Intl.NumberFormat`.

### `NaN` Propagating Silently

One `NaN` input poisons every downstream calculation. Validate inputs and fail fast rather than storing `NaN`.

## Self-Check

- [ ] Code recognizes that `Number` is IEEE 754 double; float error and the `MAX_SAFE_INTEGER` boundary are accounted for in any precision-sensitive logic.
- [ ] Money is represented as integer minor units or via a decimal library, never as floating-point dollars that accumulate rounding error.
- [ ] Identifiers that may exceed `2^53 - 1` are received and transmitted as strings and handled with `BigInt` for arithmetic.
- [ ] `BigInt` is used for large integers only (not decimals), and not mixed with `Number` without deliberate conversion.
- [ ] `parseInt` is always called with an explicit radix, and parsed results are validated for `NaN` rather than trusted.
- [ ] Floats are compared with an epsilon or exact representations, not with direct `===`.
- [ ] `Number.isNaN` (not coercing global `isNaN`) is used to detect parse failures, and `NaN`/`Infinity` inputs are validated before propagation.
- [ ] Number display uses `Intl.NumberFormat` for locale-correct grouping, currency, and rounding rather than hand-rolled string formatting.
