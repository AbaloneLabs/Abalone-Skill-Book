---
name: swift_optionals_and_error_handling.md
description: Use when the agent is writing or reviewing Swift code involving optionals, optional chaining, force-unwrapping, nil-coalescing, guard and if-let binding, throwing functions and try/try!/try?, the Result type, error protocol conformance, choosing between throws and Result, or deciding where optionals should be replaced with a richer type and where force-unwrapping is acceptable.
---

# Optionals And Error Handling In Swift

Swift's optionals are the language's signature safety feature: a value that may be absent is `T?`, and the compiler forces you to acknowledge the absence before using the value. This eliminates entire categories of null-pointer crashes that plague other languages. But optionals are only as safe as the way they are unwrapped, and Swift offers a spectrum from safe (binding, chaining) to dangerous (force-unwrapping) to semantically rich (Result, typed errors). The judgment problem is not "use optionals" — the compiler more or less forces that — but choosing the right way to handle absence and failure at each site, and resisting the temptation to force-unwrap or to use `try?` to make the compiler stop complaining.

Agents frequently reach for `!` (force-unwrap) to silence the compiler, or `try?` to convert a throwing call into an optional, or pile up optional chains that quietly produce `nil` five levels deep with no indication of which link failed. The harm is that these choices move a compile-time guarantee to a runtime crash (force-unwrap) or to a silent loss of error information (`try?`, long optional chains). Worse, the crash from `!` is a `fatalError` with no recovery, in production, from a value the type system already told you could be nil. The skill is to treat every `!` and every `try?` as a deliberate decision with a stated reason, to prefer binding and chaining that preserve information, and to choose between `throws` and `Result` based on whether failure is synchronous-recoverable or carried in a value.

## Core Rules

### Never Force-Unwrap Without A Proven Invariant

The `!` operator crashes the program with `fatalError` if the optional is nil. It is the single most common source of preventable Swift crashes in production. It is justified only when you can prove, independently of the optional, that the value is non-nil at that point — and even then, the proof should be local and robust.

- Acceptable uses: unwrapping a value you just checked (`if let x = opt { /* x is non-nil */ }` — but you would use `if let`, not `!`); outlets that are guaranteed connected after `viewDidLoad` (and even these are fragile); a value loaded from a known-good constant.
- Unacceptable uses: silencing the compiler when you are not sure; unwrapping the result of a lookup, parse, or network call that can genuinely fail; unwrapping a value passed from external code whose nil-ness you do not control.
- The rule: every `!` in the code should be defensible with a one-sentence proof of why the value is non-nil, and that proof should not depend on assumptions about runtime data.

Strong choice: `guard let url = URL(string: str) else { return .failure(.badURL) }`. Weak choice: `let url = URL(string: str)!` "because the string is always valid" (it is not, and the crash is in production).

### Prefer Binding And Chaining Over Implicit Assumption

`if let`, `guard let`, and optional chaining (`?.`) are the safe ways to handle optionals because they branch on absence rather than crashing.

- `guard let x = optional else { ... return/throw }` is the idiomatic way to exit early with a non-nil `x` in scope. Prefer it over `if let` when the absent case means "stop," because it keeps the happy path unindented.
- Optional chaining (`obj?.property?.method()`) short-circuits to nil if any link is nil. Use it for "best-effort access where nil is a fine result," but be aware that a long chain hides which link was nil.
- `??` (nil-coalescing) provides a default. Use it when there is a sensible default; do not use it to paper over a nil that indicates a real error with a made-up default value.

### Distinguish "Absent" From "Failed"

An optional represents absence: there is no value, and that is normal. A thrown error or `Result` represents failure: something went wrong, and the caller should know what. Conflating the two — using an optional where failure information matters, or using `try?` to discard error detail — loses information that debugging and recovery need.

- If a function can fail for several distinguishable reasons (network down, not found, unauthorized), model it with `throws` or `Result<T, Error>`, not with `T?`. `T?` tells the caller "it failed" but not why.
- `try?` converts a throwing call to an optional, discarding the error. Use it only when you genuinely do not care why it failed (e.g., a best-effort cache read). Do not use it to avoid writing `catch`; the discarded error is usually the thing you need when debugging.
- `try!` crashes on failure, like `!` on nil. It is justified only for invariants you can prove (a known-valid regex, a fixed format), never for runtime data.

### Choose throws vs Result By How Failure Is Carried

Swift offers two failure mechanisms: `throws`/`try`/`catch`, and `Result<Success, Failure>`. They are not interchangeable; each fits a different context.

- **`throws`** is for synchronous functions whose failure the caller handles immediately with `do/catch` or propagation. It is the default for "this operation can fail, handle it or propagate." Async functions can also `throw` (in `async throws`), and `await try` composes naturally.
- **`Result`** is for failure that is carried as a value: stored for later, passed through a non-throwing boundary (closures, callbacks that cannot throw), or processed functionally (`map`, `flatMap`). Pre-async callbacks often used `Result`; with async/await, `throws` is usually cleaner.
- Do not wrap every throwing function in `Result` "for explicitness"; `throws` with typed errors is already explicit. Reach for `Result` when the value-carrying nature matters (storing outcomes, combining results, bridging to non-throwing APIs).

### Use Typed Errors To Preserve Failure Meaning

Swift errors are values conforming to `Error`. Untyped `Error` propagation (`catch { error in }`) loses the specific failure type and forces callers to pattern-match blindly. Typed errors (an enum conforming to `Error`, or `Result<T, MyError>`) preserve the failure shape.

- Define a domain-specific error enum for each module or operation, with cases that carry the relevant context (the failing input, the underlying error, a message). This makes `catch` exhaustive and self-documenting.
- At boundaries (UI, API responses, logging), map domain errors to user-facing or transport representations; do not leak internal error types or raw descriptions to users.
- Avoid `Error` as a catch-all return type; it forces every caller to handle an unknown failure shape, which usually means a generic "something failed" path that loses information.

### Do Not Overuse Optionals Where A Richer Type Fits

Optionals are a binary "present or absent." Some domains have more states than that, and forcing them into an optional loses information.

- A function returning `User?` from a lookup conflates "not found" with other failures; a `Result<User, LookupError>` distinguishes them.
- A `String?` for "the user's middle name" is fine (absence is meaningful); a `String?` for "the parsed configuration or nil if invalid" loses the parse error.
- Consider whether the absence has multiple causes, or whether the value has additional states (loading, error, loaded) that a single optional cannot express — in those cases an enum or a small state type is clearer than stacking optionals (`T??`).

## Common Traps

### Force-Unwrapping To Silence The Compiler

`let x = maybeX!` makes the compiler happy and crashes at runtime. The compiler warning exists because the value can be nil; `!` does not make it non-nil, it only moves the failure from compile time to runtime. Replace with `guard let`/`if let` and handle the nil case.

### try? Discarding The Error

`let data = try? fetchData()` returns nil on any failure, throwing away the network error, the parse error, or the authorization error that you need to debug or to show the user. Use `try` with `catch`, or `Result`, when the failure reason matters.

### Long Optional Chains Hiding The Failure Point

`user?.profile?.addresses?.first?.city` returns nil if any of five links is nil, with no indication of which. When the chain is long or the nil is unexpected, bind each step explicitly (`guard let user = user, let profile = user.profile, ...`) so a failure is diagnosable.

### Outlets Force-Unwrapped Crashing In Edge Cases

`@IBOutlet weak var label: UILabel!` is implicitly unwrapped; accessing it before `viewDidLoad` or after the view was unloaded crashes. These are acceptable in straightforward view-controller code but fragile in reused, recycled, or programmatically constructed views; prefer `weak var label: UILabel?` and optional access where the lifecycle is not simple.

### Using Optional Instead Of A Domain Error

Returning `Config?` from a parser conflates "absent" with "malformed," so the caller cannot tell whether to use a default or report a corrupt file. Return `Result<Config, ConfigError>` or `throws` so the failure shape is preserved.

### Catching Error And Doing Nothing

`do { try risky() } catch { }` swallows the error silently. At minimum log it; better, propagate it or map it to a domain error. Empty catches hide bugs that surface as mysterious missing behavior.

### try! On Runtime Data

`let regex = try! NSRegularExpression(pattern: userInput)` crashes on any invalid pattern from the user. Use `try` with `catch` and report the bad pattern; reserve `try!` for compile-time-known-valid input.

### Nested Optionals (T??)

`T??` arises from combining optional-returning operations (e.g., `dict[key]` is `V?`, inside an optional array). Nested optionals are confusing to unwrap and usually signal that the data model should be flattened or restructured. Use `flatMap` to collapse them deliberately.

## Self-Check

- [ ] No force-unwrap (`!`) exists without a local, robust, one-sentence proof that the value is non-nil, and no `!` unwraps runtime data, lookups, parses, or network results.
- [ ] Optionals are handled with `guard let`/`if let` for early exit, optional chaining for best-effort access, and `??` only where a genuine default exists.
- [ ] `try?` is used only where the failure reason is genuinely irrelevant; `try!` is reserved for compile-time-known-valid input; failures with distinguishable causes use `throws` or `Result`.
- [ ] The choice between `throws` and `Result` reflects how failure is carried (immediate catch vs value-carried through callbacks/storage), not a stylistic default.
- [ ] Errors are typed (domain enums conforming to `Error`) so `catch` is exhaustive and self-documenting, and internal error types are mapped at boundaries rather than leaked.
- [ ] No long optional chain silently produces nil from an unknown link where diagnosis matters; critical chains are bound step by step.
- [ ] Optionals are not used to model multi-state domains (loading/error/loaded, distinct failure causes) where an enum or state type would be clearer.
- [ ] No empty `catch { }` blocks swallow errors; every catch logs, propagates, or maps to a domain error.
- [ ] Outlets and implicitly-unwrapped optionals are used only where the lifecycle guarantees non-nil, and fragile lifecycles use explicit optionals.
- [ ] Every `!`, `try!`, and `try?` in the code can be justified against the actual runtime data, not against an assumption that "it will never happen."
