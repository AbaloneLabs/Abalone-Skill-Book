---
name: fp_in_scala.md
description: Use when the agent is writing functional Scala code, designing immutable data models, using pure functions, working with monads and effect systems like Cats Effect or ZIO, choosing between Future and IO or Task, handling errors functionally, or diagnosing laziness, referential transparency, and side-effect management in Scala applications.
---

# FP in Scala

Functional programming in Scala is a spectrum, not a binary, and the hardest bugs come from mixing levels of discipline inconsistently. A codebase that is "mostly functional" often holds mutable state in a `var` inside a `Future`, throws exceptions from inside a `map`, or uses `Future` for side-effecting work that should be tracked by an effect type. Each of these works in isolation and breaks the compositional guarantees that make FP worth the discipline. The judgment problem is not "how do I use `map`" but deciding the level of referential transparency and effect tracking the code commits to, and then being consistent about it so that the benefits (reasoning, testing, composition) actually materialize.

The recurring failure mode is a developer who adopts FP idioms (immutability, `map`/`flatMap`, `for` comprehensions) without adopting the underlying discipline (pure functions, explicit effects), ending up with code that looks functional but hides side effects, swallows errors, or leaks resources. A `Future` started at construction time executes immediately and is not referentially transparent; an exception thrown inside a `map` bypasses the error channel the type system was supposed to model; a `lazy val` shared across threads races on initialization. The fix is not more syntax; it is understanding what each abstraction promises and using it where its promises hold.

## Core Rules

### Decide the effect discipline and commit to it consistently

The biggest architectural choice is how side effects are tracked:

- **stdlib `Future`**: eager, runs on an `ExecutionContext`, cannot be retried or cancelled cleanly, exceptions are the error channel. Fine for simple async orchestration but not referentially transparent.
- **Cats Effect `IO` / ZIO `ZIO` / Monix `Task`**: lazy, pure values describing effects; referentially transparent, composable, cancellable, with typed errors. Required for disciplined FP and for code that needs cancellation, retry, or resource safety.

Mixing them inconsistently (e.g., calling `unsafeRunSync` inside a `Future`, or wrapping side effects in `IO` at some boundaries and not others) breaks the guarantees. Pick a discipline per module and apply it uniformly.

### Model data immutably, and mutate via copy

Immutable case classes with `copy` are the default data model. Rules:

- Use `case class` with `val` fields for domain data; equality, hashing, and pattern matching come for free.
- Express changes as `obj.copy(field = newValue)`, returning a new instance.
- For "mutable" state, model it as a function from old state to new state, not as an in-place update; a `Ref` (Cats Effect) or `var` inside a single-threaded actor is the escape hatch.

Immutability removes a class of concurrency bugs and makes reasoning local; do not abandon it for perceived performance without measuring.

### Keep functions pure where possible; push effects to the boundary

A pure function's output depends only on its inputs and it has no side effects. Rules:

- Keep domain logic pure: parsing, validation, transformation, business rules. These are trivially testable and composable.
- Push side effects (database, network, console, random, clock) to the edges of the program, described by an effect type (`IO`, `ZIO`) or injected as an interface.
- A function that reads a `var`, calls `System.currentTimeMillis`, or throws is not pure; marking it as such (or wrapping in an effect) is honest.

The boundary between pure core and effectful shell is the most valuable architectural line in an FP codebase.

### Handle errors in the type, not via exceptions

Exceptions bypass the type system and break referential transparency. Rules:

- For expected, recoverable errors, model them in the return type: `Either[Error, A]`, `Option[A]`, or the error channel of an effect type (`IO[Error, A]` / `ZIO[Error, A]`).
- Reserve exceptions for truly unexpected, unrecoverable conditions (bugs, invariant violations).
- In `for` comprehensions over `Either`/`IO`, short-circuiting propagates the first error; this is the compositional error handling FP promises.
- Do not `try`/`catch` around pure code to recover; that re-introduces exceptions as control flow.

### Understand laziness and strictness per abstraction

Scala's collections are strict by default; `View`/`LazyList` are lazy. Effect types are lazy (describing, not running). Rules:

- `lazy val` initializes on first access and is memoized; shared across threads, it races (double init) unless the type handles it.
- `LazyList` (formerly `Stream`) memoizes elements, which can retain large structures; use `Iterator` for one-pass consumption without retention.
- Effect types (`IO`) are values; creating one does not run it. Forgetting to `flatMap` or to run the final effect means the work never happens.

### Resource safety requires `bracket`/`Resource`, not try/finally inside effects

Side-effecting resources (files, connections, handles) inside an effect type must be released even on cancellation or error. Rules:

- Use `Resource` (Cats Effect) or `bracket`/`ensuring` to scope acquisition and release, including on cancellation.
- `try`/`finally` inside an `IO` does not handle cancellation; the `finally` may not run if the effect is cancelled mid-flight.
- Compose resources with `for` comprehensions over `Resource` so release order is correct.

### Choose `Future` vs effect type by the cancellation and purity needs

- Use `Future` for fire-and-forget or simple async where cancellation and retry are not needed and eagerness is acceptable.
- Use `IO`/`ZIO`/`Task` when you need cancellation, retry, timeout composition, referential transparency, or typed errors.
- Bridging `Future` to an effect type (`.fromFuture`) is fine at boundaries; bridging the other way (`unsafeRunAsync`) inside application code is a smell.

### Test the pure core heavily; test the effectful shell thinly

Because the pure core has no side effects, it is trivially property-testable and fast. The effectful shell is thinner and harder to test; mock the interfaces it calls. Rules:

- Push logic into the pure core and test it exhaustively, including property-based tests.
- Keep the effectful shell small: wire interfaces to effects, run them, handle errors.
- Do not unit-test `IO` execution exhaustively; test the values it produces and the sequencing, not the runtime.

## Common Traps

### `Future` started eagerly and treated as pure

`val x = Future { db.query() }` runs at construction, not at use; it is not referentially transparent and not cancellable. Use an effect type, or construct the `Future` inside a function.

### Exceptions thrown inside `map`/`flatMap`

An exception inside a `Future.map` becomes a failed `Future`; inside an `IO.map` it becomes a panicked fiber. Both bypass the typed error channel. Return `Either`/failed effect instead.

### `lazy val` shared across threads

A `lazy val` initializes once but the initialization is synchronized; under contention this is a bottleneck, and if init throws, subsequent accesses re-throw. For shared mutable state in concurrent FP, use `Ref`.

### `LazyList` retaining a large structure

`LazyList` memoizes, so holding the head retains the entire computed prefix. Use `Iterator` for one-pass streaming without retention.

### Mixing `Future` and `IO` inconsistently

Calling `unsafeRunSync` inside a `Future` blocks a thread and breaks composition; wrapping `Future` in `IO` at some boundaries and not others hides where effects run. Pick one effect discipline per module.

### Forgetting to run the final effect

An `IO` or `ZIO` that is never run does nothing. The program's entry point must run the root effect; intermediate values that are not `flatMap`ed are silently dropped.

### `try`/`finally` for resource cleanup inside effects

`try`/`finally` does not run on cancellation of an effect. Use `Resource`/`bracket` so release happens on error and cancellation.

### Treating `var` inside a `Future` as safe

A `var` mutated from a `Future` is shared mutable state; concurrent `Future`s racing on it produce data races. Use `Ref` or model state transitions purely.

## Self-Check

- Has the effect discipline (stdlib `Future` vs `IO`/`ZIO`/`Task`) been chosen per module and applied consistently, without mixing at boundaries except via documented bridges?
- Are domain data models immutable case classes with changes expressed via `copy`, and is shared mutable state modeled via `Ref` rather than `var`?
- Is domain logic pure (no `var`, no clock/random/network, no exceptions), with side effects pushed to the boundary and described by effect types or injected interfaces?
- Are expected errors modeled in the return type (`Either`/`Option`/typed error channel), with exceptions reserved for unrecoverable bugs?
- Are laziness semantics understood per abstraction (`lazy val` memoization/race, `LazyList` retention, effect types not running until executed)?
- Are side-effecting resources scoped with `Resource`/`bracket` so release happens on error and cancellation, not `try`/`finally` inside effects?
- Is the choice between `Future` and effect types based on cancellation/retry/purity needs, with bridges only at documented boundaries?
- Is the pure core tested exhaustively (including property tests) while the effectful shell is kept thin and tested via mocked interfaces?
