---
name: test_pyramid_and_strategy.md
description: Use when the agent is deciding how to distribute tests across unit, integration, and end-to-end levels, choosing where a specific behavior should be verified, balancing test speed against confidence, designing a test suite for a new feature or service, reviewing why a test suite is slow or flaky, deciding whether to write an E2E test or push the check down to a unit test, or planning test investment for regression safety. Also covers the test pyramid and its variants (testing trophy, honeycomb), test cost versus value, fast feedback loops, flakiness budget, and deciding what each test level is for.
---

# Test Pyramid And Strategy

A test suite is not a pile of tests; it is a distribution of confidence across levels. The judgment problem is not "should we test this" but "where should this confidence live, and what are we willing to pay for it." Every test has a cost — runtime, maintenance, flakiness, setup, and the cognitive load of understanding a failure — and a value — the kind of regression it catches and how fast it gives feedback. A suite that puts the same check at the wrong level accumulates debt invisibly: it works at first, then becomes slow, then becomes flaky, then becomes the thing developers learn to ignore.

Agents tend to default to the level they can write fastest, which is often the end-to-end level because it exercises the real system and "obviously works," or the pure unit level because it is fast and isolated. Both defaults miss the point. The pyramid (and its modern variants) is not a shape to copy; it is a reasoning tool about feedback speed versus realistic confidence. This skill exists to make the placement of each test a deliberate decision about what failure mode it guards against and what cost it accepts.

## Core Rules

### Place Each Test At The Level That Owns The Risk

The level of a test should match the integration boundary where the risk lives. A pure function with no collaborators belongs at the unit level; a bug that can only appear when two modules exchange data belongs at the integration level; a bug that only appears when the full request path, including infrastructure, is exercised belongs at the end-to-end level. Pushing a check to a higher level than it needs wastes time and adds flakiness; pushing it to a lower level than the risk requires produces a test that passes while the real system is broken.

For each behavior you are about to test, ask:

- What is the smallest set of real collaborators that could reveal this bug?
- Does this bug depend on a real database, network, filesystem, or third-party service to manifest?
- If I test this in isolation, am I re-asserting an assumption that the integration actually holds?

The correct level is the lowest one that still exercises the boundary where the failure would occur. Going lower creates a false-positive-prone test; going higher creates a slow, flaky, expensive one.

### Understand What Each Level Is Actually For

The levels are not interchangeable; each answers a different question.

- **Unit tests** verify the logic of a single unit in isolation. They are fast, deterministic, and numerous. Their job is to pin down branch behavior, edge cases, and local invariants. They are cheap to write and cheap to run, so they should carry the bulk of logic-level coverage.
- **Integration tests** verify that components collaborate correctly — module-to-module, service-to-database, service-to-message-bus. Their job is to catch contract mismatches, serialization bugs, transaction boundary errors, and configuration drift that no unit test can see.
- **End-to-end tests** verify that a user-visible journey works through the real system. Their job is not to find logic bugs (unit tests do that) but to confirm that the assembled parts produce the expected outcome, including infrastructure, routing, and deployment configuration.

If you find an end-to-end test asserting a calculation, that assertion belongs lower. If you find a unit test mocking six collaborators to simulate a request flow, that flow belongs higher.

### Cost Tests Honestly: Speed, Flakiness, And Maintenance

A test that takes 50 milliseconds and never flakes is not comparable to a test that takes 5 seconds and flakes once a week, even if both "cover" the same line. The real cost of a test has several dimensions:

- **Runtime cost** — multiplied across every developer and every CI run, slow tests compound. A 10-minute E2E suite run a hundred times a day is a full engineering week of waiting per week.
- **Flakiness cost** — a test that fails intermittently trains the team to ignore failures. A flaky test is worse than no test, because no test at least does not erode trust.
- **Maintenance cost** — tests coupled to implementation details break on every refactor; tests coupled to behavior break only when behavior changes.
- **Debugging cost** — when a high-level test fails, the cause could be anywhere. A unit test failure points at one place.

Prefer the level that minimizes the sum of these costs while still catching the target failure mode. A slightly slower integration test that is deterministic and points clearly at a real contract is usually worth more than a fast unit test that mocks away the very thing that breaks.

### Choose A Shape That Fits The System, Not A Shape You Memorized

The classic pyramid (many unit, fewer integration, few E2E) fits systems where logic dominates and integration is stable. It fits poorly when:

- The system is mostly integration glue over stable third-party services (a thin orchestration layer). Here the logic is thin and the integration is the risk, so a "testing trophy" shape with more integration tests is correct.
- The system is a UI or workflow engine where behavior only emerges from component composition. A "honeycomb" shape with broad integration coverage fits better.
- The system is a library or pure algorithm with no external dependencies. Here almost everything can be unit tests, and E2E may not exist at all.

Name the dominant risk in your system — logic, integration, or assembled behavior — and bias the shape toward the level that exercises that risk. Do not impose a pyramid on a system whose risk is elsewhere.

### Make The Fast Feedback Loop Non-Negotiable

Developers optimize for the feedback loop they actually have. If the fast loop (unit and focused integration tests) is missing or slow, they will either skip tests or run the slow suite constantly. The single most leveraged property of a test suite is that a developer can run a meaningful subset locally in seconds and get a trustworthy signal.

Protect this property deliberately:

- Keep the unit and focused-integration tiers runnable in under a few seconds for a typical change.
- Make the slow tiers (full integration, E2E) opt-in for local runs and mandatory only in CI on the right triggers.
- Tag or partition tests by speed so the fast loop is selectable.

A suite where every test is "medium speed" and the fast loop does not exist will, over time, be run less and trusted less.

### Decide What Each Level Will Not Cover

A test strategy is also a statement of what you accept as uncovered at each level. Unit tests will not catch integration bugs; integration tests will not catch full-path configuration bugs; E2E tests will not economically cover every branch. Making these limits explicit prevents the suite from being asked to do something it cannot.

For each level, write down:

- What failure modes is this level responsible for catching?
- What failure modes is this level explicitly allowed to miss?
- Which lower or higher level catches what this one does not?

If a gap has no level responsible for it, that is a real hole. If every level tries to catch everything, the suite becomes redundant and slow.

### Treat Flakiness As A Bug, Not A Nuisance

A flaky test is a signal that the test depends on something non-deterministic: a real clock, network ordering, shared mutable state, a third-party service, a test that mutates data another test reads, or an assertion that races with asynchronous work. Flakiness does not fix itself; it erodes the value of the entire suite because every failure becomes suspect.

When a test flakes:

- Treat it as a defect with the same priority as a production bug.
- Find the non-deterministic source. Quarantine the test (skip, not delete) until it is fixed, so it stops poisoning trust in the suite.
- Decide whether the test belongs at its current level at all. Many flaky E2E tests are really integration checks that were placed too high and inherit the nondeterminism of the full path.

A suite with a known flaky test that is not addressed is a suite whose green signal means nothing.

### Separate Confidence Tests From Regression Pin-Down Tests

Not all tests serve the same purpose. Some tests encode confidence in new code written test-first; they drive design and verify intent. Other tests pin down existing behavior to make refactoring safe; they may not express ideal design, but they guard a legacy path. Treating them the same leads to over-investment in pin-down tests at the wrong level and under-investment in the confidence tests that guide new work.

Be explicit:

- Confidence tests should be written at the level that drives the design — usually unit, sometimes integration.
- Pin-down (characterization) tests for legacy code are legitimate but should be marked as such, so they are not mistaken for the design intent and can be replaced when the underlying code is refactored.

## Common Traps

### The Inverted Pyramid

A suite dominated by end-to-end tests, with few unit tests, feels safe because every test exercises the real system. It is in fact slow, expensive to maintain, and flaky, and it catches logic bugs inefficiently because each E2E test only touches a few branches. The inversion usually happens because E2E tests are easy to write at first and unit tests require understanding the code. The result is a suite that takes an hour to run and that developers avoid touching.

### Testing Through The Wrong Level Because It Was Easier

Writing an E2E test to verify a calculation, or a unit test with six mocks to simulate a full request, are the same mistake in opposite directions: the test is at the wrong level for the risk. The E2E version is slow and points nowhere when it fails; the over-mocked unit version passes while the real integration is broken. The shortcut feels productive and produces a test that does not earn its cost.

### Treating Coverage Percentage As The Strategy

A high coverage number says lines were executed, not that behavior was verified. A suite can hit 90% coverage and still miss every edge case, every error path, and every integration contract. Optimizing the suite for coverage leads to shallow tests that execute code without asserting meaningful outcomes. Use coverage as a map of untested areas, never as the goal.

### One Slow Test That Everyone Runs Anyway

A single test that takes 30 seconds does not sound like much, but when it runs on every save and every CI job, it dominates the feedback loop. Slow tests accumulate because each one seems acceptable in isolation. Audit runtime regularly; a test that is an outlier in duration either belongs in a slower tier or needs to be rewritten at a lower level.

### Mocking So Much That The Test Proves Nothing

A unit test that mocks every collaborator, including the database, the message bus, and the downstream service, is testing that the code calls the mocks in the expected order. It proves the code matches the mocks, not that the system works. When the real collaborators change, the mocks do not, and the test stays green while production breaks. This is the unit-level version of testing at the wrong level.

### E2E Tests That Duplicate Unit Tests

When an E2E test and a unit test assert the same calculation, one of them is redundant. Usually the unit test is the right one to keep for the logic and the E2E test should assert only the assembled outcome, not re-verify the internals. Duplication across levels inflates runtime and maintenance without adding confidence.

### Trusting A Green Suite That Has Not Run Against The Real System

A suite of unit and mocked-integration tests can be entirely green while the real system is broken, because the real database, real network, and real third-party services were never exercised. If no test level touches the real integration boundary, the gap is invisible until production. Ensure at least one tier exercises real collaborators for the riskiest integrations.

### Adding Tests Without A Flakiness Budget

Without an explicit flakiness budget, flaky tests accumulate until the suite's signal is noise. Teams then either rerun until green (hiding real failures) or disable tests wholesale (losing coverage). Decide upfront how much flakiness is tolerable and enforce it; a suite above the budget is a defect to fix, not a status quo to tolerate.

## Self-Check

- [ ] Each test is placed at the lowest level that still exercises the boundary where its target failure mode would occur; no logic-only check lives only at E2E, and no integration-dependent check lives only as an over-mocked unit test.
- [ ] The distribution shape (pyramid, trophy, honeycomb) was chosen to match the system's dominant risk (logic, integration, or assembled behavior), not copied from a default.
- [ ] Each level has a written responsibility for which failure modes it catches and which it is allowed to miss, and every gap has some level accountable for it.
- [ ] The fast feedback loop (unit and focused integration) runs in seconds for a typical change and is selectable separately from the slow tiers.
- [ ] Test runtime was audited recently; outlier-slow tests were either moved to a slower tier or rewritten at a lower level rather than tolerated.
- [ ] No known flaky test is left active without a fix or a quarantine plan; flakiness is treated as a defect, not background noise.
- [ ] Coverage is used as a map of untested areas, not as the success metric; tests assert meaningful behavior, not just line execution.
- [ ] At least one tier exercises the real integration boundary for each risky integration (database, message bus, third-party service), so a green suite implies the real system was touched.
- [ ] Confidence tests (driving new design) and pin-down tests (guarding legacy behavior) are distinguishable, so pin-down tests are not mistaken for design intent.
- [ ] No test duplicates the same assertion at two levels without reason; where duplication exists, each level asserts only what it is responsible for.
