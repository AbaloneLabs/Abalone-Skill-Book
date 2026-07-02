---
name: legacy_testing_strategy.md
description: Use when the agent is adding tests to an existing legacy codebase that has little or no test coverage, writing characterization tests to pin down current behavior before refactoring, breaking hard-to-test dependencies in code that was not designed for testing, choosing a black-box vs white-box testing approach for opaque legacy modules, applying risk-based testing when full coverage is infeasible, or building regression protection for code whose correctness is not well understood. Also covers the failure mode of attempting full test coverage on legacy code and stalling, breaking implicit behavior while adding tests, testing implementation details that then block refactoring, and the gap between "we have tests" and "the tests catch real regressions."
---

# Legacy Testing Strategy

Adding tests to legacy code is a different problem from testing new code, and applying new-code testing habits to legacy code stalls or backfires. New code is written test-first or alongside tests, with dependencies designed for substitution and behavior well understood. Legacy code is code without tests, often without seams for substitution, and — critically — whose correct behavior is not fully known even to its maintainers. The central technique is the characterization test (also called a golden or pin-down test): a test that asserts what the code does today, including its quirks and bugs, so that a subsequent change which alters that behavior is caught. The point is not to validate that the current behavior is correct (you often do not know); it is to make change safe by detecting when behavior shifts. The judgment problem is that full coverage of a legacy system is usually infeasible (the code was not written to be tested, dependencies are hard to break, and the effort is enormous), so testing must be targeted by risk, focused on the behavior you intend to preserve, and structured so that the tests enable change rather than freeze the implementation.

Agents tend to either skip legacy testing (because it is hard) or attempt too much (full coverage, which stalls), and both leave the code unsafe to change. The harm appears as regressions that ship because no test caught them, as refactors that break implicit behavior the tests did not capture, and as tests so tied to implementation details that they block the very refactoring they were meant to enable. The discipline is to characterize behavior before changing it, to break dependencies surgically (not rewrite), to test at the highest level that gives regression protection, to prioritize by risk where full coverage is infeasible, and to treat the test suite as a safety net for change rather than a correctness proof. A legacy test suite is good when it gives you the confidence to refactor; it is bad when it forces you to update dozens of tests for every change.

## Core Rules

### Characterize Behavior Before Changing It

The foundational technique for legacy code is the characterization test: a test that captures the code's current behavior — including its bugs and quirks — so that a change which alters that behavior is detected. You write these before refactoring, not after, because the moment you change the code without a net, any behavior shift is invisible.

- **Assert what the code does, not what it should do.** A characterization test pins current behavior; if the code returns 42 for an input where you expected 7, the test asserts 42 (with a note that this may be a bug to fix separately). The goal is to detect change, not to assert correctness you have not verified.
- **Cover the inputs and paths you intend to preserve.** You cannot characterize everything; focus on the behaviors that matter for the change you are about to make and the regressions you most fear.
- **Capture the quirky behavior explicitly.** If the code has a workaround or an edge-case behavior that callers depend on, characterize it; these are exactly the implicit invariants that refactoring breaks.

### Break Dependencies Surgically, Not By Rewriting

Legacy code is often hard to test because its dependencies (databases, clocks, external services, global state) are wired in rather than injected, and there are no seams for substitution. The temptation is to rewrite the module to be testable; the discipline is to break dependencies surgically — introducing the smallest seam that allows substitution — so that testing becomes possible without a rewrite that itself introduces risk.

- **Introduce seams, not rewrites.** Extract an interface, parameterize a dependency, wrap a global in an injectable adapter — the smallest change that lets you substitute the dependency in tests. Preserve the existing behavior; the seam is a means to characterization, not an excuse to refactor.
- **Prefer dependency injection at boundaries over mocking deep internals.** Substituting a repository or a clock at the boundary is more stable than mocking a dozen internal collaborators.
- **Keep the dependency-breaking changes separate from logic changes.** A PR that only introduces a seam (behavior unchanged) is safe and reviewable; bundling a seam with logic changes muddies both.

### Test At The Highest Level That Gives Regression Protection

Low-level unit tests in legacy code are often expensive (because the code was not designed for them) and brittle (because they tie to implementation details that block refactoring). Testing at a higher level — a module's public API, a service boundary, an HTTP endpoint — often gives more regression protection per unit of effort and is more stable across refactors.

- **Prefer black-box tests over white-box for opaque modules.** When you do not fully understand a module's internals, test its observable inputs and outputs; white-box testing of code you do not understand ties tests to details that may be wrong and that you intend to change.
- **Test through the public API where feasible.** A test that exercises a module through its public interface survives internal refactoring; a test that pokes private methods breaks the moment you refactor.
- **Use lower-level tests where the higher level is too slow or imprecise.** Balance coverage against speed and determinism; some critical units warrant direct tests despite the wiring cost.

### Prioritize By Risk When Full Coverage Is Infeasible

Full coverage of a legacy system is rarely feasible, so testing must be targeted by risk. The highest-risk areas — load-bearing modules, high-churn code, incident-prone paths, the code about to be changed — deserve the testing effort; low-risk, stable, well-understood code may warrant little or none.

- **Target the code you are about to change.** The most valuable legacy tests are the ones that characterize the behavior you are about to refactor; write them immediately before the change.
- **Target load-bearing and high-churn modules.** Code that everything depends on, or that changes often, is where regressions hurt most.
- **Target incident-prone paths.** Modules with a history of bugs are likely to have more; characterization tests there prevent recurrence.
- **Accept that some low-risk code may stay lightly tested.** Spreading effort evenly across the whole codebase leaves the risky areas under-protected; concentrate effort where regressions would hurt.

### Treat The Suite As A Safety Net For Change, Not A Correctness Proof

A legacy test suite's job is to make change safe — to give you the confidence to refactor because a regression will be caught. It is not a proof that the system is correct, because the characterized behavior may itself contain bugs. Holding the suite to a correctness standard leads to either stalling (you cannot prove correctness of code you do not understand) or to tests that assert intended rather than actual behavior (and thus miss the real regressions).

- **Optimize the suite for regression detection, not coverage metrics.** A suite that catches the regressions you fear is valuable at 40% coverage; a suite at 90% coverage that misses the regressions you fear is not.
- **Keep tests stable across refactors.** If every refactor requires updating many tests, the tests are tied to implementation rather than behavior; refactor the tests toward the public interface.
- **Fix characterized bugs deliberately, updating the test.** When a characterization test reveals a bug, decide explicitly whether to fix it; if you fix it, update the test to the new (correct) behavior, with a record of the change.

## Common Traps

### Attempting Full Coverage And Stalling

Trying to test the entire legacy system at once, finding the effort enormous and the code uncooperative, and delivering no tests at all. Target by risk; characterize the code you are about to change and the regressions you most fear.

### Breaking Implicit Behavior While Adding Tests

Refactoring to introduce testability and inadvertently altering behavior, shipping a regression in the very act of adding tests. Keep dependency-breaking changes separate from logic changes and characterize behavior before refactoring.

### Testing Implementation Details That Block Refactoring

Writing white-box tests against private internals of code you do not understand, so that every intended refactor breaks dozens of tests and the suite becomes an obstacle. Prefer black-box tests through the public API; test observable behavior, not implementation.

### Characterizing Intended Instead Of Actual Behavior

Writing tests that assert what the code "should" do rather than what it does, so the tests pass against an idealized behavior and miss the real regressions in the actual behavior. Characterize actual behavior, including quirks and bugs; fix bugs deliberately and update the test.

### Mocking Deep Internals

Substituting many internal collaborators with mocks to make a unit testable, producing tests that pass but verify little real behavior and that break when internals change. Prefer dependency injection at boundaries over mocking deep internals.

### Confusing Coverage With Protection

Chasing a coverage percentage on legacy code, achieving high coverage with shallow tests that do not catch the regressions that matter. Optimize for regression detection; a suite that catches feared regressions is valuable regardless of its coverage number.

### Tests That Must Be Updated For Every Change

A suite so tied to implementation that any refactor requires extensive test updates, making the tests a drag on the very changes they were meant to enable. Refactor tests toward the public interface and observable behavior.

## Self-Check

- [ ] Behavior is characterized before changing it: characterization tests assert what the code does today (including quirks and bugs), focused on the inputs and paths to be preserved, with quirky edge-case behavior captured explicitly because those are the implicit invariants refactoring breaks.
- [ ] Dependencies are broken surgically (smallest seam that allows substitution — extracted interface, parameterized dependency, wrapped global) rather than by rewriting, and dependency-breaking changes are kept separate from logic changes so behavior is preserved.
- [ ] Tests are written at the highest level that gives regression protection (public API, service boundary, HTTP endpoint) with black-box testing preferred for opaque modules, balancing coverage against speed and determinism rather than defaulting to low-level unit tests of uncooperative code.
- [ ] Testing is prioritized by risk where full coverage is infeasible: the code about to be changed, load-bearing and high-churn modules, and incident-prone paths get the effort, while low-risk stable code may stay lightly tested rather than spreading effort evenly.
- [ ] The suite is treated as a safety net for change (regression detection) rather than a correctness proof, optimized for catching feared regressions over coverage metrics, and kept stable across refactors so it enables change rather than blocking it.
- [ ] Characterized bugs are handled deliberately (fix explicitly, update the test to the new correct behavior with a record of the change) rather than left as silent assertions of buggy behavior.
- [ ] The highest-risk cases were verified — characterizing behavior before a refactor caught a regression, breaking a dependency did not alter behavior, a black-box test survived an internal refactor, and the suite caught a real regression rather than only proving coverage — not only the clean test-first path.
