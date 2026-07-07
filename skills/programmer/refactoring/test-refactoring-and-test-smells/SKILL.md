---
name: test_refactoring_and_test_smells.md
description: Use when the agent is refactoring tests, fixing test smells (brittle tests, slow tests, duplicate tests, tests that test the wrong thing), improving test maintainability, reducing test execution time, addressing test duplication and over-mocking, converting brittle assertion-heavy tests to behavior-focused tests, or diagnosing a test suite that is hard to maintain, slow, or unreliable. Covers test smell categories (brittle, slow, duplicate, over-mocked, testing implementation not behavior), the refactoring patterns (extract test helper, parameterize, use fixtures, reduce mocking), test pyramid health, and the discipline that tests are code that must be maintained with the same care as production code.
---

# Test Refactoring And Test Smells

Tests are code, and like all code, they degrade without care: they become brittle (failing on unrelated changes), slow (accumulating unnecessary work), duplicative (the same setup and assertions repeated across dozens of tests), over-mocked (so many mocks that the test tests the mocks, not the behavior), and misfocused (asserting on implementation details rather than behavior). A degraded test suite is a liability: it slows development (every change touches many brittle tests), it provides false confidence (tests pass but do not catch real bugs), and it is eventually abandoned (too painful to maintain, so tests are skipped or deleted). The discipline of test refactoring is treating tests as production code that must be readable, maintainable, and focused on behavior, recognizing the characteristic test smells that degrade suites, and applying the refactoring patterns that restore the suite's health. The goal is a suite that is fast (runs in seconds, enabling the feedback loop), reliable (does not flake — see flaky-test-and-test-failure-debugging), focused (tests behavior, not implementation), and maintainable (changes to production code require proportionate, not disproportionate, test changes).

Agents tend to write tests that work (they pass) without considering maintainability, to over-mock (testing interactions rather than behavior), and to let test duplication accumulate. The judgment problem is recognizing that test quality is measured by maintainability and behavior-focus (not just coverage), that the characteristic test smells have known refactoring patterns, and that an unhealthy test suite slows development as surely as bad production code. This skill covers the discipline of test refactoring: test smell categories, refactoring patterns, the test pyramid, and the maintainability mindset.

## Core Rules

### Identify And Fix Brittle Tests

A brittle test fails when unrelated production code changes, even though the behavior the test verifies is unchanged. Brittleness erodes trust and wastes time.

- **Brittleness from asserting on implementation details: the test checks how the code works, not what it does.** A test that asserts on the specific methods called, the internal state, or the exact sequence of operations is brittle: a refactoring that changes the implementation (but preserves the behavior) breaks the test. Refactor to assert on observable behavior (outputs, state changes visible from outside), not implementation.
- **Brittleness from over-specification: the test asserts too much, including irrelevant details.** A test that asserts the entire output object when only one field matters is brittle: any change to an unrelated field breaks it. Assert only what the test cares about; use precise assertions (assert the specific field) or partial-match assertions.
- **Brittleness from coupling to unrelated modules: the test depends on modules unrelated to the behavior under test.** A unit test that sets up a database, a message queue, and three external services to test a pure calculation is brittle (any of those changing breaks it) and slow. Decouple the test from unrelated dependencies (mock them, or test the unit in isolation).
- **Refactor brittle tests toward behavior assertion.** Replace implementation assertions with behavior assertions, reduce over-specification to the relevant fields, and decouple from unrelated dependencies. The test should pass for any implementation that produces the correct behavior.

### Reduce Test Execution Time

A slow test suite (minutes to run) breaks the feedback loop: developers avoid running it, or run only parts of it, missing regressions. Speed is a correctness property of the suite.

- **Identify the slow tests (profile the suite).** A few slow tests often dominate the suite's runtime; profile to find them. A unit test taking seconds (vs milliseconds) is a suspect; an integration test taking minutes may be necessary but should be minimized.
- **Reduce unit test dependencies on slow resources (database, network, file system).** A unit test that hits a real database or network is orders of magnitude slower than one using in-memory stubs. Mock or stub the slow dependencies in unit tests; reserve real dependencies for integration tests.
- **Eliminate unnecessary sleeps and fixed waits.** A test that sleeps a fixed duration (waiting for an async operation) is slow and flaky. Replace sleeps with condition polling or async synchronization (the test completes as soon as the condition holds). See flaky-test-and-test-failure-debugging.
- **Parallelize test execution.** Tests that do not share state can run in parallel, reducing wall-clock time. Ensure tests are isolated (no shared mutable state) before parallelizing; a test that depends on another's state breaks under parallelism.
- **Balance the test pyramid: many fast unit tests, fewer slower integration tests, very few end-to-end tests.** A top-heavy pyramid (many slow end-to-end tests, few unit tests) is slow and brittle; a bottom-heavy pyramid (many fast unit tests, targeted integration tests) is fast and reliable. Push testing down the pyramid where possible.

### Eliminate Test Duplication

Duplicate tests (same setup, same assertions, across many tests) bloat the suite, slow it, and make changes painful (a change to the setup requires touching dozens of tests).

- **Extract common setup into test fixtures or helper methods.** Setup repeated across tests (creating a standard object, initializing a standard state) belongs in a fixture or helper, not copied into each test. Extract it; tests reference the fixture, reducing duplication.
- **Parameterize tests with the same structure but different inputs.** Tests that differ only in input and expected output (testing the same behavior with multiple cases) should be a single parameterized test with a table of cases, not many near-identical tests. Parameterization reduces duplication and makes adding cases trivial.
- **Extract assertion helpers for repeated assertion patterns.** Assertions repeated across tests (checking an object is in a specific valid state) belong in a helper, not copied. Extract the helper; tests call it.
- **Do not over-extract (removing readability).** Extraction that hides what the test does (a test that is all helper calls with no visible assertion) harms readability. Extract when the duplication is substantial and the helper is clear; keep the test's intent visible.

### Reduce Over-Mocking

Over-mocking (mocking every dependency, asserting every interaction) produces tests that verify the mocks, not the behavior, and that are brittle (any interaction change breaks them) and low-signal (they pass when the mocks are set up correctly, regardless of real behavior).

- **Mock at the boundaries (external dependencies), not at every internal collaborator.** Mock the database, the external API, the message queue (real dependencies that are slow or nondeterministic); do not mock every internal class (which couples the test to the implementation). Internal collaborators should be real (the test exercises the real interaction).
- **Prefer stubs (fake behavior) over mocks (interaction verification).** A stub provides fake behavior (a fake repository that returns a canned result); the test asserts on the outcome. A mock verifies interactions (asserting the code called specific methods); the test asserts on the interactions. Stubs are less brittle (they do not break on interaction changes) and focus on behavior.
- **Assert on state and output, not on interaction.** A test that asserts the code produced the correct output (or changed the state correctly) verifies behavior; a test that asserts the code called specific methods (in a specific order) verifies implementation. Prefer the former; use the latter sparingly (only when the interaction is the behavior, e.g., verifying a side effect).
- **Use fakes (in-memory implementations) for complex dependencies.** A fake (an in-memory repository, an in-memory queue) provides realistic behavior without the real dependency's slowness or nondeterminism. Fakes are more realistic than mocks and less brittle; use them for complex dependencies where stubs are insufficient.

### Focus Tests On Behavior, Not Implementation

A test focused on behavior (what the code does) survives refactoring; a test focused on implementation (how the code does it) breaks on every refactoring. Behavior focus is the key to maintainable tests.

- **Test through the public interface, not the internals.** A test that calls the public methods and asserts on the observable results tests behavior; a test that inspects private state or calls private methods tests implementation. Prefer public-interface testing; it survives internal refactoring.
- **One logical assertion per test (one behavior).** A test that verifies one behavior is clear and fails for a specific reason; a test that verifies many behaviors is unclear and fails for any of them. Split multi-behavior tests into single-behavior tests.
- **Name tests for the behavior they verify, not the method they call.** A test named "createUser_validates_email" describes behavior; a test named "testCreateUser" or "test1" describes nothing. The name should communicate what the test verifies, so a failure is immediately understood.
- **Test edge cases and error paths, not just the happy path.** A suite that tests only the happy path misses the edge cases (empty input, boundary values, error conditions) where bugs hide. Ensure edge cases and error paths are covered, not just the typical flow.

### Treat Tests As Production Code: Readable, Maintainable, Reviewed

Tests are maintained alongside production code and deserve the same care: they must be readable (a new contributor understands them), maintainable (changes are proportionate), and reviewed (test quality is checked, not just production quality).

- **Tests should be readable: clear setup, clear action, clear assertion.** A test structured as given-when-then (or arrange-act-assert) is readable; a test with tangled setup and assertions is not. Structure tests for clarity.
- **Tests should be maintainable: changes to production code require proportionate test changes.** If a small production change requires large test changes, the tests are too coupled (brittle). Refactor for proportionate change.
- **Review test quality in code review, not just production code quality.** A test that is brittle, duplicative, or misfocused should be caught in review, just as a production bug is. Review tests with the same standards.
- **Do not disable or delete tests to make the suite pass.** A failing test is a signal (a bug or a stale test); disabling it to make the suite green silences the signal. Fix the cause (the bug or the stale test), do not disable the test. See flaky-test-and-test-failure-debugging for the quarantine workflow.

## Common Traps

### Asserting On Implementation Details

Tests that check how the code works (methods called, internal state), breaking on refactoring. Assert on behavior (outputs, observable state).

### Over-Mocking Every Collaborator

Mocking internal classes and verifying interactions, producing brittle tests that test the mocks. Mock at boundaries; prefer stubs and fakes.

### Test Duplication (Copy-Paste Tests)

Setup and assertions repeated across many tests, bloating the suite and making changes painful. Extract fixtures, parameterize, use assertion helpers.

### Fixed Sleeps In Tests

Tests that sleep a fixed duration, slow and flaky. Replace with condition polling or async synchronization.

### Top-Heavy Test Pyramid

Many slow end-to-end tests, few unit tests, making the suite slow and brittle. Push testing down the pyramid; many fast unit tests, targeted integration tests.

### Over-Specified Assertions

Asserting the entire output when only one field matters, breaking on unrelated changes. Assert only what the test cares about.

### Tests Named For Methods, Not Behavior

Tests named "testCreateUser" rather than for the behavior, making failures hard to understand. Name for the behavior verified.

### Disabling Tests To Make The Suite Pass

Disabling failing tests rather than fixing the cause, silencing the signal and losing coverage. Fix the cause; use quarantine for flaky tests with a tracking issue.

## Self-Check

- [ ] Brittle tests (failing on unrelated changes) are identified and refactored toward behavior assertion — implementation assertions replaced with behavior assertions, over-specification reduced to relevant fields, and unrelated dependencies decoupled — so the test passes for any implementation that produces the correct behavior.
- [ ] Test execution time is managed: slow tests are profiled and identified, unit tests are decoupled from slow resources (database, network, file system) via stubs and fakes, unnecessary sleeps are eliminated, tests are parallelized (with isolation verified), and the test pyramid is balanced (many fast unit tests, fewer integration tests, very few end-to-end).
- [ ] Test duplication is eliminated: common setup is extracted into fixtures or helpers, structurally-similar tests are parameterized (a table of cases), repeated assertion patterns are extracted into helpers, and extraction does not harm readability (the test's intent remains visible).
- [ ] Over-mocking is reduced: mocks are used at boundaries (external dependencies), not on every internal collaborator; stubs and fakes are preferred over interaction-verifying mocks; assertions focus on state and output rather than interactions; and complex dependencies use realistic fakes (in-memory implementations) where stubs are insufficient.
- [ ] Tests focus on behavior, not implementation: they test through the public interface, verify one logical behavior per test, are named for the behavior they verify (not the method), and cover edge cases and error paths (not just the happy path).
- [ ] Tests are treated as production code: readable (given-when-then structure), maintainable (production changes require proportionate test changes), reviewed in code review (test quality is checked), and not disabled to make the suite pass (the cause is fixed, or flaky tests are quarantined with a tracking issue).
- [ ] The suite's health is measured (runtime, pass rate, coverage of behavior and edge cases, brittleness rate) and tracked over time, with degradation addressed proactively rather than tolerated.
- [ ] New tests are written to the same standards (behavior-focused, non-brittle, non-duplicative, appropriately mocked, fast), and review catches test smells before they enter the suite — because a degraded test suite slows development as surely as bad production code.
