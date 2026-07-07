---
name: scala_testing_in_scala.md
description: Use when the agent is writing Scala tests (ScalaCheck property-based, specs2/munit/ScalaTest frameworks), designing testable code (dependency injection, pure functions, separating effects), mocking (Mockito/ScalaMock), testing effectful/Future/ZIO code, writing property-based generators and shrinking, testing exceptions, organizing test suites, or is diagnosing "tests are flaky with Futures", "property-based test found a bug but shrink is unhelpful", "mocking final classes/sealed traits fails", "testing async code times out", or test-organization issues. Covers framework choice and styles, property-based testing and shrinking, testing effectful/Future/IO code, mocking limitations, testability design, and async/flakiness pitfalls.
---

# Testing In Scala

Scala testing spans unit tests, property-based tests, and effectful (Future/IO) tests, each with its own pitfalls. Frameworks (ScalaTest, munit, specs2) offer multiple styles that, mixed carelessly, produce inconsistent suites. Property-based testing (ScalaCheck) finds edge cases random sampling reveals, but unguided shrinking produces minimal-but-meaningless counterexamples, and non-deterministic/generator-misaligned properties flake. Testing `Future`-based code without `Await`/`FutureResult` semantics causes flaky timeouts; testing `IO`/`ZIO` requires runtime execution. Mocking final classes, case objects, or sealed traits fails (Mockito cannot override finals without special setup; ScalaMock needs traits). And untestable code (side-effecting singletons, hardcoded dependencies) resists testing. The judgment problem is to choose a framework and style consistently, to write properties with meaningful generators and shrinking, to test effectful code with the right async semantics, to design for testability, and to mock only where appropriate.

Agents write flaky Future tests, get unhelpful shrinks, mock final classes and fail, or test side-effecting code by hitting real services. The remedy is consistent framework usage, deliberate property design, correct async handling, and testability-first design.

## Core Rules

### Choose A Framework And Style, And Use It Consistently

ScalaTest (most feature-rich, many styles: `AnyFunSuite`, `AnyFlatSpec`, `AnyWordSpec`, `AnyFreeSpec`, plus `AnyPropSpec`/`AnyFunSpec`), munit (lightweight, opinionated, good defaults, `FunSuite`-like with `test("name")`), and specs2 (BDD `should` style) are the main options. Within ScalaTest, pick *one* style per codebase (mixing `FunSuite` and `FlatSpec` is jarring) and stick to it. munit is a good default for new projects (simple, fast, good integration). Use the framework's assertion primitives (`assert`, `assertEquals`, munit's `assertEquals` with diff) rather than `require`/`if`. For property-based tests, use ScalaCheck integration (`ScalaCheckPropertyChecks` in ScalaTest, or munit's `ScalaCheckSuite`).

- Pick one framework and one style per codebase; do not mix styles.
- munit for new projects (simple, fast); ScalaTest for max features/ecosystem; specs2 for BDD.
- Use framework assertions (`assert`/`assertEquals`), not `require`/manual `if`.

### Write Property-Based Tests With Meaningful Generators And Shrinking

Property-based testing (ScalaCheck) asserts a property holds for *many* generated inputs, finding edge cases hand-written tests miss. Key practices:

- Write generators for your domain types (`Arbitrary[User]` via `Gen`) so inputs are realistic, not arbitrary strings/ints that bypass validation.
- Properties should be deterministic and pure; a property with hidden state or time-dependence flakes.
- Shrinking minimizes a failing counterexample, but if the generator produces nonsensical inputs, the shrink is meaningless. Guide shrinking with custom `Shrink` or `Arbitrary` constrained to valid values.
- Use `forAll` (ScalaCheck) or `forAll` (ScalaTest's `ScalaCheckPropertyChecks`); set adequate `minSuccessful` for confidence.
- A failing property's shrunk counterexample should be added as a regression unit test.

- Generate realistic domain inputs (custom `Arbitrary`), not unconstrained primitives.
- Properties are pure/deterministic; guide shrinking so counterexamples are meaningful.
- Add shrunk failures as regression tests.

### Test Effectful Code (Future/IO) With Correct Async Semantics

A `Future` test that returns a `Future[Assertion]` must be awaited by the framework (ScalaTest's `AsyncWordSpec`/`ParallelTestExecution`, munit's `test("x") { ... Future }` — munit awaits Futures automatically). Calling `Await.result`/`Await.ready` manually is discouraged (it blocks and can deadlock thread pools); let the framework await. For `IO`/`ZIO`, use the effect's test runtime (cats-effect `IORuntime`, ZIO's `test`/`unsafeRun`) and assert on the result. Avoid `Thread.sleep` in tests (flaky, slow); use deterministic test clocks (`TestControl` in cats-effect, `TestClock` in ZIO) for time-based logic. Shared mutable state across async tests causes races; keep tests independent.

- Return `Future[Assertion]`/`IO[Assertion]` and let the framework await; avoid manual `Await`/`Thread.sleep`.
- Use effect runtimes (cats-effect `IORuntime`, ZIO test runtime) for effect types.
- Use deterministic test clocks for time; keep async tests independent (no shared mutable state).

### Design For Testability (Pure Core, Injected Dependencies)

Code is testable when its effects are isolated and its dependencies injectable. Patterns:

- Pure core, effectful shell: put business logic in pure functions returning plain data; push effects (DB, HTTP, clock) to the edges. Test the core exhaustively; test the shell lightly.
- Inject dependencies as parameters/interfaces (a `Repository` trait, a `Clock`), not hardcoded singletons, so tests substitute fakes/stubs.
- Avoid `object` singletons with state; pass the dependency.
- Make implicit effects (system time, random, env) explicit parameters so tests control them.

Untestable code (side-effecting singletons, `new ConcreteRepo()` inside logic) forces mocks or integration tests; designing for testability reduces the need for both.

- Pure core + effectful shell; inject dependencies as interfaces; make time/random/env explicit.
- Avoid stateful singletons; prefer passing dependencies.

### Mock Deliberately, Knowing The Limitations

Mocking has limits: Mockito (via `mockito-scala`) cannot mock `final` classes/methods or `case object`s without the mockito-inline agent; ScalaMock cannot mock classes without traits. Prefer fakes/stubs (a lightweight in-memory `Repository` implementation) over mocks for complex dependencies — they are more robust and readable. Mock only when a fake is impractical (a third-party final class). Over-mocking couples tests to implementation details (every call verified), making refactoring painful; assert on observable outcomes, not call sequences. For final/sealed types, either make them non-final at the test boundary (a trait) or use a fake.

- Prefer fakes/stubs over mocks for complex dependencies; mock only third-party finals.
- Mockito cannot mock `final` without the inline agent; ScalaMock needs traits — design with interfaces.
- Assert on observable outcomes, not call sequences (avoid over-coupling to implementation).

## Common Traps

### Flaky Future Tests From Manual Await Or Thread.sleep

`Await.result`/`Thread.sleep` blocks and can deadlock/time out. Return `Future[Assertion]` and let the framework await; use deterministic clocks.

### Meaningless Shrunk Counterexamples

Unconstrained generators produce nonsensical shrinks. Use custom `Arbitrary` for realistic, valid inputs.

### Mocking Final Classes / Case Objects Failing

Mockito cannot mock `final` without the inline agent. Use a trait/fake, or enable the agent.

### Mixing Test Styles

`FunSuite` + `FlatSpec` + `WordSpec` in one project is inconsistent. Pick one.

### Testing Side-Effecting Singletons

Hardcoded dependencies force integration tests or mocks. Inject interfaces; use fakes.

### Non-Deterministic Properties (Time, Random, Order)

A property depending on `System.currentTimeMillis` or `Random` flakes. Make effects explicit/seeded.

### Over-Mocking (Verifying Every Call)

Tests coupled to call sequences break on refactor. Assert on outcomes.

### Shared Mutable State Across Async Tests

Races cause intermittent failures. Keep tests independent; no shared mutable fixtures.

## Self-Check

- [ ] One framework and one style is used consistently across the codebase, with framework assertions (`assert`/`assertEquals`) rather than manual checks.
- [ ] Property-based tests use realistic custom generators (`Arbitrary`) for domain types, are pure/deterministic, and add shrunk failures as regression unit tests.
- [ ] Effectful tests return `Future[Assertion]`/`IO[Assertion]` for the framework to await (no manual `Await`/`Thread.sleep`), with deterministic test clocks for time-based logic.
- [ ] Code is designed for testability (pure core + effectful shell, injected interface dependencies, explicit time/random/env), reducing the need for mocks/integration tests.
- [ ] Mocks are used deliberately (fakes/stubs preferred for complex deps; Mockito's `final` limitation understood), and tests assert on observable outcomes, not call sequences.
- [ ] Tests are independent (no shared mutable state), and async/parallel execution does not introduce races.
- [ ] Final/sealed types at test boundaries are handled via traits/fakes or the inline agent, not by failing mocks.
- [ ] The suite organization (unit/property/integration separation) and runtime (effect runtimes, test clocks) have been considered for determinism and speed.
