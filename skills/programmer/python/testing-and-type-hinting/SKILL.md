---
name: python_testing_and_type_hinting.md
description: Use when the agent is writing or reviewing Python tests with pytest or unittest, designing test fixtures and parametrization, choosing what to test (unit vs integration vs property-based), mocking and patching (unittest.mock, monkeypatch), measuring coverage, structuring a test suite, or deciding how tests interact with type hints and runtime validation. Covers test pyramid decisions, fixture scope and cleanup, parametrize vs subtests, mock vs fake vs stub, coverage as a signal not a target, testing typed and untyped boundaries, and the pitfalls of over-mocking, testing implementation details, or chasing coverage numbers.
---

# Testing And Type Hinting

Testing and type hinting are the two main correctness tools in Python, and they interact: types describe contracts statically, tests verify behavior dynamically, and over-relying on either produces false confidence. The judgment problem is deciding what to test (and at what level), structuring tests so they catch real regressions without testing implementation details, and keeping the type contract and the test suite aligned so neither drifts from reality.

Agents tend to over-mock (mocking the very thing under test), chase coverage percentages instead of behavior, write tests that pass but prove nothing, or treat type hints and tests as substitutes rather than complements. The harm appears as green suites that miss real bugs, brittle tests that break on every refactor, mocks that hide integration failures until production, and type annotations that drift from what the tests actually exercise. The real work is testing behavior at the right level, using mocks surgically, treating coverage as a floor not a goal, and keeping the type contract honest with the tested behavior.

## Core Rules

### Test Behavior At The Right Level Of The Pyramid

Not every test should be a unit test, and not every test should hit the database. The test pyramid (many fast unit tests, fewer integration tests, fewest end-to-end tests) balances speed, confidence, and cost. Choose the level by what could fail and how expensive the failure is to catch.

- **Unit tests** verify a single function or class in isolation, fast and deterministic. Use for pure logic, parsers, algorithms, and business rules. Dependencies are fakes/stubs, not the real collaborators.
- **Integration tests** verify that components work together (your code + the real database, real HTTP client, real library). Use to catch contract mismatches, serialization bugs, and boundary errors that unit tests with mocks cannot see. Slower, but they catch the bugs that matter.
- **End-to-end tests** verify the whole system through its external interface (HTTP API, CLI). Fewest of these; they are slow and brittle but catch wiring failures.
- **Property-based tests** (Hypothesis) generate inputs and assert invariants hold for all of them. Use for parsers, codecs, and any function with a clear invariant; they find edge cases hand-written tests miss.

The common error is testing everything at the unit level with heavy mocking, which produces fast green tests that miss integration failures. Invest in integration tests at the boundaries that change (database schema, external API, serialization format).

### Mock Collaborators, Not The Thing Under Test

Mocking is for replacing *external* collaborators (a database, an HTTP API, the filesystem, time) so the test is fast and deterministic. The thing under test should be real. Two failures:

- **Mocking the unit under test** means you are testing the mock, not the code. If you mock the class's methods to test the class, the test proves nothing about the real behavior.
- **Over-mocking collaborators** hides integration failures. If you mock the database's query method, you never test whether the real query works, whether the schema matches, or whether the ORM serializes correctly. Mock the collaborator at a stable boundary; test the real integration separately.

Prefer **fakes** (a lightweight in-memory implementation of the same interface) over **mocks** (objects that record calls and assert on them). A fake lets the test exercise real behavior through a simpler implementation; a mock couples the test to the exact sequence of calls, which is brittle. Use mocks for verifying interaction (was the email sent?); use fakes for verifying behavior (did the order total compute correctly?).

### Design Fixtures For Reuse And Clean State

pytest fixtures provide setup (and teardown via `yield`) shared across tests. Design them so tests are independent and fast.

- **Scope fixtures appropriately.** Function scope (default) gives every test fresh state — safest but slowest. Module/session scope is faster but risks state leaking between tests; use it only for expensive, read-only setup (a loaded config, a compiled regex).
- **Teardown via `yield`.** A fixture that `yield`s a resource must clean up after the yield so the resource is released even if the test fails. Forgetting teardown leaks files, connections, and processes.
- **Prefer factory fixtures over parameterized monoliths.** A fixture that returns a factory lets each test build the exact object it needs; a fixture that returns one fixed object forces tests to fit the fixture.
- **Keep fixtures small and composable.** A giant fixture that sets up the whole world couples every test to the same large state; prefer small fixtures that compose.

### Parametrize Over Duplication, But Not At The Cost Of Clarity

`@pytest.mark.parametrize` runs one test function against many inputs, eliminating copy-pasted test functions. Use it for genuine input/output tables (a parser with many cases, a formatter with many formats).

- Parametrize when the test logic is identical and only inputs/expected outputs vary.
- Use `pytest.subtests` or separate tests when different inputs need different assertions or setup; forcing them into one parametrized test hides which case failed and why.
- Name parametrize cases (`ids=[...]`) so failures point to the meaningful case, not `case0`.
- Do not parametrize across unrelated behavior to reduce test count; clarity beats brevity in tests.

### Treat Coverage As A Floor, Not A Target

Coverage measures which lines ran during the test suite. It is a useful signal (uncovered code is definitely untested) but a poor goal (covered code may be tested trivially).

- Use coverage to find untested error paths and branches; 100% line coverage does not mean 100% branch coverage or 100% behavior coverage.
- A test that calls a function and asserts nothing exercises the line but proves nothing. Coverage rewards such tests; behavior does not.
- Set a coverage floor (e.g., 80%) to catch regressions, but do not optimize for 100%. Time spent raising coverage from 95% to 100% on trivial code is better spent on integration tests for risky boundaries.
- Mutation testing (mutmut, cosmic-ray) measures whether tests catch real bugs by mutating the code; it is a stronger signal than coverage.

### Keep Type Hints And Tests Aligned

Types and tests verify different things and drift independently. Keep them honest with each other.

- A function annotated `-> User` that tests only ever pass with `None` reveals the type is aspirational, not real. Test the annotated behavior, including the `None`/edge cases the type permits.
- Untyped boundaries (`**kwargs`, `Any`, dynamic attributes) cannot be protected by types; cover them with tests and runtime validation.
- When you change a type, check whether the tests still exercise the new contract; when you change behavior, check whether the type still describes it.
- Use runtime checkers (`beartype`) in tests to catch type violations that static analysis misses, without paying the overhead in production.

## Common Traps

### Mocking The Unit Under Test

Mocking the class's own methods to test the class tests the mock, not the code. Test the real unit; mock only its collaborators.

### Over-Mocking Hiding Integration Failures

Mocking the database, HTTP client, and serializer means the test never exercises the real integration. Use fakes or real collaborators for integration tests at risky boundaries.

### Testing Implementation Details

A test that asserts on the exact sequence of internal calls (did it call helper A then B?) breaks on every refactor. Test observable behavior (the output, the side effect), not the internal call sequence.

### Chasing Coverage Instead Of Behavior

Adding tests that call code without meaningful assertions raises coverage without catching bugs. Write tests that would fail if the behavior broke.

### Fixture State Leaking Between Tests

A module-scoped fixture that mutates shared state causes tests to pass or fail depending on execution order. Keep fixtures function-scoped unless read-only, and clean up via `yield`.

### Parametrize Forced Across Unrelated Cases

Cramming unrelated scenarios into one parametrized test to reduce count makes failures unclear. Split when assertions or setup differ.

### Ignoring The Untyped Boundary

Code that crosses an untyped boundary (`**kwargs`, `Any`, external JSON) is not protected by types; if it lacks tests and runtime validation, it is a hole.

### Green Suite That Tests Nothing

Tests with no assertions, or assertions on trivially-true properties, pass while proving nothing. Every test should assert something that would fail if the behavior broke.

## Self-Check

- [ ] Tests are at the right level of the pyramid: fast unit tests for logic, integration tests for risky boundaries (DB, HTTP, serialization), few end-to-end tests for wiring.
- [ ] The unit under test is real; only external collaborators are replaced, and fakes are preferred over interaction-asserting mocks where behavior is what matters.
- [ ] Fixtures are scoped to the shortest safe lifetime, clean up via `yield`, and are small/composable; no state leaks between tests.
- [ ] `parametrize` is used for genuine input/output tables with named cases, not forced across unrelated scenarios.
- [ ] Coverage is treated as a floor to find untested paths, not a target to maximize; tests assert meaningful behavior, not just exercise lines.
- [ ] Tests assert observable behavior, not internal call sequences, so refactors do not break tests that should still pass.
- [ ] Type hints and tests are aligned: tests exercise the annotated contract including edge cases; untyped boundaries have tests and runtime validation.
- [ ] Integration tests cover the boundaries most likely to fail in production (schema, API contract, serialization), not just unit-tested logic with mocked boundaries.
- [ ] Every test has an assertion that would fail if the behavior broke; no green-tests-prove-nothing suites.
- [ ] Property-based testing is considered for logic with clear invariants (parsers, codecs) to find edge cases hand-written tests miss.
