---
name: typescript_type_narrowing_and_guards.md
description: Use when the agent is reasoning about TypeScript control-flow analysis (CFA) and how narrowing propagates across branches, assignments, and closures; writing assertion functions (asserts x is T), branded or opaque types, or custom guard predicates that the compiler can track; debugging lost narrowing inside callbacks or after reassignment; or deciding between a type guard, an assertion function, and a branded type for a given safety goal. Covers the mechanics of CFA-based narrowing, its limitations, and advanced guard patterns. Distinct from discriminated unions and runtime validation boundaries, which are covered separately.
---

# Type Narrowing And Guards: Control-Flow Analysis Mechanics

TypeScript narrows types by tracking checks through *control-flow analysis* (CFA): as the compiler walks branches, assignments, and returns, it refines the type of a variable at each point. Most narrowing "just works" for simple `if`/`typeof`/discriminants, but real code has closures, reassignment, aliased conditions, and async boundaries where CFA gives up and narrowing is silently lost. The judgment problem is understanding *how CFA tracks narrowing, where it stops tracking, and which mechanism* — a type guard, an assertion function, or a branded type — actually achieves the safety goal without fighting the compiler.

Agents tend to assume narrowing survives any code shape, write guards or assertions that the compiler cannot follow (so they provide no static safety), lose narrowing inside callbacks and blame the compiler, or reach for `as` when an assertion function would track. The harm appears as code that looks type-safe but where the compiler has actually widened back to the broad type (so a later access can still crash), guards that return a boolean without narrowing the caller's type, and assertion functions that mutate without being declared as assertions. The real work is predicting where CFA narrows and where it does not, choosing the narrowing mechanism that the compiler actually propagates, and not confusing runtime checks with static narrowing.

## Core Rules

### Understand What CFA Tracks And What It Drops

CFA refines a variable's type based on checks *in the same synchronous flow it can follow*. It tracks:

- `typeof`, `instanceof`, `in`, equality, truthiness, and discriminant checks in `if`/`switch`/ternary.
- Assignments (`x = "hi"` narrows `x` to `string` after the assignment).
- Early returns, throws, and `break`/`continue` (narrowing in the surviving branch reflects what was ruled out).
- Negated checks (`if (x === null) return;` narrows `x` to non-null afterward).

It *drops* narrowing when it cannot prove the value is unchanged:

- Inside a closure (callback) that runs later, narrowing from the enclosing scope is often lost, because the variable could be reassigned between the check and the callback's execution.
- After a function call where the variable is reachable and could be mutated, narrowing on object properties is often invalidated (the call might have changed the property).
- For `let` variables that are reassigned, narrowing resets to the declared type at the assignment.

Predicting these boundaries is the core skill. When narrowing "mysteriously" disappears, the cause is almost always a closure, a function call between the check and the use, or a mutable `let`.

### Preserve Narrowing Across Closures With A Const Local

The classic lost-narrowing case: you narrow a variable, then reference it inside a callback, and the compiler widens back. This happens because the variable is a mutable `let` (or a property that a call could change), and CFA conservatively assumes it may have changed by the time the callback runs.

- Capture the narrowed value in a `const` local after the check, and use the local in the callback. A `const` cannot be reassigned, so CFA keeps the narrowed type into the closure.
- For object properties, destructure the checked value into a local (`const { kind } = obj; if (kind === "x") ...`) so the narrowed primitive is stable.
- If the value genuinely could change, re-narrow inside the callback with a fresh check.

Do not fight the compiler by asserting; capture a stable narrowed local instead.

### Choose Between Type Guards, Assertion Functions, And Branded Types

Three mechanisms serve different narrowing goals; picking the wrong one gives no safety.

- **Type guard (`x is T`)**: a function returning a boolean that narrows in the `true` branch of a check. Use when the caller checks and branches: `if (isError(res)) { res.message }`. The guard narrows only where its return is tested.
- **Assertion function (`asserts x is T` / `asserts condition`)**: a function that throws if the condition fails, narrowing for *all subsequent code* without a branch. Use when validation should halt on failure: `assertNonNull(user); user.name`. The narrowing applies after the call returns.
- **Branded/opaque type**: a nominal-ish type that can only be created through a validating constructor, so any value of the type is guaranteed valid by construction. Use when a value's validity is a long-lived invariant (a validated `Email`, a sanitized `UserId`) that you want the type system to enforce everywhere, not just at one check site.

Guards and assertions narrow at a point; branded types enforce an invariant across the codebase. Choose by whether the safety is local (guard/assertion) or global (branded).

### Write Assertion Functions Correctly

An assertion function uses `asserts x is T` (narrowing `x`) or `asserts condition` (narrowing based on a passed boolean) in its return type. The compiler treats the code after the call as if the assertion held.

- `function assertNonNull<T>(x: T): asserts x is NonNullable<T> { if (x == null) throw new Error(); }` — after `assertNonNull(u)`, `u` is narrowed to non-null.
- `function assert(condition: any, msg?: string): asserts condition { if (!condition) throw new Error(msg); }` — after `assert(obj.kind === "x")`, the discriminant is narrowed.
- The body must actually throw on failure; the signature is a promise to the compiler. A body that does not throw but claims `asserts` is a lie that produces unsound narrowing.

Assertion functions are the right tool when you want validation to throw and to narrow for all subsequent lines, instead of forcing every caller into an `if`.

### Write Type Guards That Actually Narrow

A type guard `function isFoo(x): x is Foo` must return a boolean and is only useful where the return value is tested in a branch. Common mistakes:

- A guard that returns `true` too broadly narrows invalid data into the target type (covered in the discriminated-unions skill). Verify the shape in the body.
- A guard whose return is ignored (`isFoo(x); useFoo(x)`) provides no narrowing — the caller must `if (isFoo(x))`.
- A guard over an object property that is later invalidated by a function call loses its narrowing; re-check or capture a local.

Prefer an assertion function when you do not want callers to branch; prefer a guard when callers legitimately branch on the result.

### Use Branded Types For Invariants That Span The Codebase

When a value must be validated once and then trusted everywhere (a parsed-and-validated config, a sanitized identifier, a checked email), a branded type enforces the invariant globally rather than at each use.

- `type UserId = string & { __brand: "UserId" };` — a nominal flavor of string that can only be produced by a validating function.
- The validating function checks input and returns the branded type; everywhere else, `UserId` is trusted because it cannot be constructed by plain assignment.
- Branded types make "this value has been validated" a compile-time fact, not a runtime hope.

Do not brand frivolously; each branded type is a conversion boundary. Brand where the invariant is real and span-wide.

### Do Not Confuse Runtime Checks With Static Narrowing

A runtime check (`if (x.foo)`) only narrows if CFA can track it. A check inside a string passed to `eval`, a check in a different file, or a check whose result is stored and reused later may run at runtime but provide no static narrowing. The static type is what protects the *other* code paths and refactorings; if narrowing is lost, a future edit can introduce an unsafe access that the compiler will not catch. Always confirm the compiler actually narrowed (hover the variable) rather than assuming the runtime check suffices.

## Common Traps

### Lost Narrowing Inside A Callback

Narrowing from an outer `if` is dropped inside a callback because the variable could change before the callback runs. Capture a `const` local with the narrowed value and use that in the callback.

### Guard Return Ignored (No Narrowing)

Calling `isFoo(x)` without testing the return narrows nothing. Either `if (isFoo(x))` or switch to an assertion function that narrows unconditionally after the call.

### Assertion Function Body That Does Not Throw

`function assertFoo(x): asserts x is Foo { /* no throw */ }` lies to the compiler; subsequent code is narrowed unsoundly. The body must throw on failure.

### Using `as` Where An Assertion Function Would Track

`as` is a one-off unchecked claim; an assertion function (`asserts x is T`) narrows and is reusable. Prefer the assertion function when the validation is real and recurring.

### Narrowing On A Property Invalidated By A Call

`if (obj.kind === "x") { doSomething(); obj.xField }` may lose the `kind` narrowing after `doSomething()` because the call could mutate `obj`. Re-check or capture the discriminant in a local before the call.

### Branded Type Constructed Without Validation

If a branded type can be created by a plain `as` cast anywhere in the codebase, the invariant is unenforced. Route construction through validating functions and lint against direct casts.

### `let` Reassignment Resetting Narrowing

`let x: string | null = ...; if (x) { use(x) }; x = getX(); use(x)` — after reassignment, `x` resets to the declared type. Re-narrow after reassignment.

### Assuming `in` Narrows Across All Union Members

`"foo" in x` narrows to union members that have `foo`, but if multiple members have it (or the check is on an optional property), the narrowing may be looser than expected. Verify the narrowed type with a discriminant where possible.

## Self-Check

- [ ] Narrowing is predicted by CFA rules: it survives in synchronous tracked flows and is expected to be lost in closures, after mutating calls, and on `let` reassignment.
- [ ] Narrowing needed inside a callback is preserved by capturing a `const` local (or destructuring a stable property), not by asserting.
- [ ] The narrowing mechanism matches the goal: type guard for branch-on-result, assertion function for throw-and-narrow, branded type for a span-wide invariant.
- [ ] Assertion functions throw on failure in the body; their `asserts` signature is backed by real validation.
- [ ] Type guards are tested at the call site (return value used in a branch); ignored guard returns are recognized as providing no narrowing.
- [ ] Branded types are constructed only through validating functions; direct `as` casts into branded types are absent or linted.
- [ ] Narrowing on object properties that could be invalidated by an intervening call is re-established or captured before the use.
- [ ] `as` is not used where an assertion function or guard would propagate narrowing; one-off assertions are reserved for genuine external knowledge.
- [ ] The compiler's actual narrowed type is verified (hover/inspection), not assumed from the presence of a runtime check.
- [ ] `let` variables are re-narrowed after reassignment; no code relies on pre-reassignment narrowing surviving the assignment.
