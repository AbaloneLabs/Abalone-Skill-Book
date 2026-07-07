---
name: rust_testing_strategy.md
description: Use when the agent is writing or organizing Rust tests, deciding between unit tests (in-module #[cfg(test)]), integration tests (tests/ directory), and doc tests, choosing a testing approach for async code (#[tokio::test], block_on), designing testable APIs by separating pure logic from IO, using property-based testing (proptest, quickcheck), mocking and dependency injection in Rust, measuring coverage, or debugging tests that pass locally but fail in CI. Covers test organization, the testability boundary, async testing, property tests, the difficulty of mocking in Rust, and the tradeoff between exhaustive unit tests and meaningful integration tests.
---

# Testing Strategy

Rust's type system and ownership model eliminate large classes of bugs, which makes the remaining testing work both more valuable and differently shaped than in less safe languages. The judgment problem is not "write tests" but deciding what to test, where each test lives, how to make IO-bound code testable without mocking frameworks, and how to balance fast unit tests against integration tests that catch real wiring bugs.

Agents tend to either over-test trivial getters that the type system already guarantees, or skip integration tests because unit tests pass. They reach for mocking frameworks that fight Rust's ownership model, write async tests that deadlock, or bury logic inside functions that read files and network sockets so it cannot be tested without the environment. The harm appears as green suites that miss real failures, tests that are expensive to maintain because they assert implementation details, and async tests that hang in CI. The real work is separating pure logic from effects, placing tests at the right level, and choosing property tests where edge cases matter.

## Core Rules

### Separate Pure Logic From Effects To Make Code Testable

The most important testability principle in Rust is to lift pure, deterministic logic out of functions that perform IO. A function that reads a file, parses it, validates it, and writes a result is hard to test because each test needs a real file. Split it into a pure `parse_and_validate(data: &str) -> Result<...>` and a thin `load_and_process(path)` that does the IO and calls the pure function.

- Test the pure function exhaustively with cheap, fast unit tests, including edge cases and malformed input.
- Test the IO wrapper with a small number of integration tests using temp files.
- This separation also improves error handling and readability, because the pure function has a clear contract.

The goal is that the bulk of your logic lives in functions whose inputs are arguments and whose outputs are return values, with no hidden dependencies on the filesystem, clock, or network.

### Place Tests At The Right Level: Unit, Integration, Doc

Rust has three first-class test kinds, each with a distinct role.

- **Unit tests** (`#[cfg(test)] mod tests` inside the source file) test individual functions and modules with access to private items. They are fast and live next to the code. Use them for logic, edge cases, and internal helpers.
- **Integration tests** (files under `tests/`) test the crate's public API as an external consumer would, using only `pub` items. They catch wiring bugs and verify the public surface works end to end. They compile as separate crates, so they also catch issues with your public API design.
- **Doc tests** (`///` examples) are executable code in documentation that is compiled and run. They serve double duty as documentation and as a test that the documented example actually works. Keep them focused and runnable.

A common mistake is putting everything in unit tests, which tests internals but misses whether the public API composes. Another is putting everything in integration tests, which cannot reach private helpers and run slower. Use all three for their respective strengths.

### Test Async Code With The Right Runtime Harness

Async functions return futures that do nothing unless driven. A test must provide a runtime.

- Use the runtime's test macro, e.g., `#[tokio::test]` for Tokio, which wraps the test in a runtime so `await` works.
- For libraries that are runtime-agnostic, test with a concrete runtime chosen in the test, or use `futures::executor::block_on` for simple cases.
- Be careful with tests that spawn tasks or hold locks across await points: they can deadlock or hang the test thread. Add timeouts to tests that involve concurrency so a hang fails fast instead of stalling CI.

Async tests that block the runtime (e.g., calling a synchronous blocking function inside an async test without `spawn_blocking`) can stall the single-threaded test runtime. Keep blocking work off the async path, even in tests.

### Use Property-Based Testing For Edge Cases And Invariants

When a function should satisfy a property for all inputs in a class (e.g., "parse then serialize is the identity", "sorting is idempotent", "the result is always non-negative"), property-based testing (proptest, quickcheck) generates many inputs and checks the property, shrinking failing cases to a minimal reproducer.

- Property tests shine for parsers, encoders, state machines, and numeric functions where enumerating edge cases by hand is infeasible.
- They are weaker for "does this produce the exact expected output for this input" — use example-based tests for that.
- Always constrain generated input to realistic shapes; an unbounded generator produces inputs no real caller will ever send.

Property tests catch edge cases that hand-written examples miss, but they are slower and can be flaky if the generator is poorly designed. Use them where the invariant is clear and the input space is large.

### Prefer Dependency Injection And Fakes Over Mocking Frameworks

Rust's ownership and strict typing make traditional mocking frameworks (which replace methods on objects) awkward. The idiomatic approach is to define a trait for the dependency, take it as a generic or `dyn` parameter, and provide a hand-written fake implementation in tests.

- Define `trait Clock { fn now(&self) -> Instant; }` and inject it, rather than reading the system clock directly.
- In tests, use a `FakeClock` you control. In production, use `SystemClock`.
- This makes dependencies explicit, improves design, and avoids the brittleness of mock libraries that tie tests to call sequences.

Fakes (lightweight working implementations) are usually more robust than mocks (objects that assert on call sequences), because they decouple the test from the internal call order. Reach for a mock only when you specifically need to assert that a side effect occurred.

### Measure Coverage As A Signal, Not A Target

Coverage tells you which lines ran during tests, which is useful for finding untested branches. It is a weak signal of test quality: a line can be executed without its behavior being asserted.

- Use coverage to find code that runs in zero tests, then write meaningful tests for it.
- Do not chase a coverage percentage by adding assertions that exercise lines without checking outcomes.
- High coverage with shallow assertions gives false confidence; the goal is meaningful assertions on important behavior.

## Common Traps

### Testing Implementation Details Instead Of Behavior

Tests that assert on internal call sequences or private state break on every refactor and catch few real bugs. Assert on observable behavior (outputs, returned errors, state changes visible through the public API).

### Untestable God Function

A single function that reads input, computes, and writes output cannot be unit-tested without the environment. Split out the pure logic so it is testable with arguments.

### Async Test That Hangs

A test that awaits something that never completes, or that blocks the single-threaded runtime, hangs indefinitely in CI. Add timeouts and keep blocking work off the async path.

### Over-Mocking With Sequence Assertions

Mocks that assert "method A was called then method B with these exact args" are brittle and couple tests to implementation. Prefer fakes and behavioral assertions.

### Green Suite That Misses Integration Bugs

Unit tests on pure functions pass, but the public API is never exercised end to end, so wiring and configuration bugs slip through. Maintain a core of integration tests.

### Doc Tests That Cannot Compile

Doc examples that are not run rot quickly. Ensure doc tests are valid, runnable code (use `no_run` or `ignore` only when execution is genuinely impossible, and prefer `# hidden` lines to keep them compilable).

### Property Test With A Bad Generator

A generator that produces unrealistic input (empty strings only, or astronomically large sizes) yields tests that are either trivial or pathologically slow. Constrain generators to realistic domains.

## Self-Check

- [ ] Pure logic is separated from IO so the bulk of behavior is testable with cheap unit tests that take arguments and assert return values.
- [ ] Tests are placed at the right level: unit tests for logic and internals, integration tests for the public API, doc tests for documented examples.
- [ ] Async tests use the correct runtime harness, avoid blocking the runtime, and include timeouts for concurrency that could hang.
- [ ] Property-based tests are used where invariants span a large input space, with generators constrained to realistic inputs.
- [ ] Dependencies are injected via traits with hand-written fakes in tests, rather than brittle sequence-asserting mocks.
- [ ] Coverage is used to find untested code, not as a target satisfied by shallow assertions.
- [ ] Tests assert observable behavior, not internal call sequences, so refactors do not break them spuriously.
- [ ] The suite includes integration tests that exercise the public API end to end, not only unit tests on internals.
- [ ] Doc tests compile and run (or are explicitly marked `no_run`/`ignore` with reason).
