---
name: flaky_test_and_test_failure_debugging.md
description: Use when the agent is diagnosing a flaky test (one that passes sometimes and fails others without a code change), investigating test failures that are intermittent, order-dependent, time-dependent, or environment-dependent; distinguishing a flaky test from a legitimate failure; identifying the root cause of flakiness (race conditions, shared state, time/date dependencies, ordering, resource leaks, network calls, randomness); or establishing quarantine and fix workflows for flaky tests. Covers flakiness root causes (concurrency, shared state, time, order, environment, async timing), distinguishing flaky from real failures, isolation and reproduction techniques, the quarantine-then-fix workflow, and the cultural discipline that flaky tests erode trust in the test suite and must be addressed, not tolerated.
---

# Flaky Test And Test Failure Debugging

A flaky test is a test that passes and fails intermittently without a code change, and it is one of the most corrosive problems a test suite can have. The harm is not just the occasional false alarm; it is the erosion of trust. A test suite with flaky tests teaches developers that a red build may not indicate a real problem, so failures are ignored, rerun until green, or dismissed — and when a real failure arrives, it is lost in the noise. A flaky test is worse than no test: no test provides no signal (known), while a flaky test provides unreliable signal (trusted then betrayed), and the betrayal degrades the entire suite's credibility. The disciplined response to flakiness is not to tolerate or rerun past it, but to treat every flaky test as a bug to be fixed (or, temporarily, quarantined), identify its root cause, and eliminate the nondeterminism that causes it. Flakiness is always rooted in nondeterminism — some factor that varies between runs (timing, order, shared state, the environment, the date, randomness) — and the debugging process is identifying that factor and making the test deterministic.

Agents tend to tolerate flaky tests (rerun until green, disable the test), to misdiagnose flakiness as a product bug rather than a test bug, and to address the symptom (adding retries, sleeps) rather than the root cause. The judgment problem is recognizing that flakiness is nondeterminism that must be eliminated at its root, that the root cause is discoverable through specific techniques (isolation, stress running, dependency analysis), and that tolerating flakiness undermines the suite. This skill covers the discipline of flaky test debugging: root causes of flakiness, distinguishing flaky from real failures, isolation and reproduction, the quarantine-then-fix workflow, and the cultural imperative to address flakiness.

## Core Rules

### Recognize That Flakiness Is Nondeterminism, And Identify Its Source

A flaky test is nondeterministic: some factor varies between runs. Identify the varying factor to find the root cause.

- **Concurrency and race conditions: the test depends on timing.** A test that relies on threads completing in a particular order, on a lock being acquired before a timeout, or on a race resolving a certain way is flaky because the timing varies by machine, load, and scheduling. This is the most common and hardest-to-debug flakiness source.
- **Shared state and test order dependence: the test depends on prior tests' state.** A test that reads or modifies state left by another test (shared database rows, global variables, files) is flaky if the test order changes or the state is not cleaned up. Running the test in isolation (vs in the suite) reveals order dependence.
- **Time and date dependence: the test depends on the current time.** A test that uses the system clock (checking a timestamp is "now," testing date-based logic) is flaky around midnight, month boundaries, daylight saving changes, or when run in different time zones. The "current time" varies.
- **Environment dependence: the test depends on the machine, OS, locale, or available resources.** A test that depends on a specific file path, a specific number of CPU cores, a specific locale (date formatting, string comparison), or available memory/disk is flaky across environments. Tests that pass on the developer's machine but fail in CI often have environment dependence.
- **External dependencies: the test calls a real network service, database, or file system.** A test that hits a real external service (an API, a database, the file system) is flaky when that service is slow, unavailable, or returns varying data. External dependencies introduce the real world's nondeterminism into the test.
- **Randomness: the test uses random data without a fixed seed.** A test that generates random inputs without seeding the random generator is flaky: different inputs on each run may or may not trigger the bug. Seed the generator, or use fixed inputs.
- **Async timing: the test assumes async operations complete by a fixed wait.** A test that waits a fixed time (a sleep) for an async operation to complete is flaky: the operation may take longer under load, and the sleep wastes time when it is faster. Wait for the condition (polling, async await), not for a fixed duration.

### Distinguish A Flaky Test From A Legitimate Failure

Before treating a failure as flakiness, confirm it is actually nondeterministic (flaky) and not a real failure that is intermittent for a real reason (e.g., a bug that manifests under specific data).

- **Reproduce the failure: run the test multiple times, in isolation and in the suite.** A flaky test fails sometimes and passes sometimes on the same code. Run it many times (10, 100) to confirm the intermittency. A test that always fails (even if the failure seems odd) is not flaky; it is a real failure to fix.
- **Run the test in isolation vs in the suite to detect order dependence.** If the test passes in isolation but fails in the suite (or vice versa), it is order-dependent (shared state). If it fails consistently in isolation, the flakiness is internal (timing, environment), not order-related.
- **Check whether the failure correlates with a code change.** A failure that started after a specific commit may be a real regression (the commit introduced a bug), not flakiness. Confirm the failure predates the suspect change (by testing older commits) before attributing it to flakiness.
- **Distinguish "flaky test" (the test is nondeterministic) from "intermittent bug" (the product is nondeterministic).** A flaky test is a test problem (fix the test); an intermittent bug is a product problem (the product has a race or edge case, and the test is correctly catching it intermittently). Investigate which: if the test is asserting correct behavior and the product sometimes violates it, it is an intermittent bug to fix in the product.

### Isolate And Reproduce The Flakiness Reliably

To fix a flaky test, reproduce it reliably. Specific techniques force the nondeterminism to manifest.

- **Run the test many times to trigger the intermittent failure.** A flaky test with a 5% failure rate may need 20-100 runs to fail. Run it in a loop until it fails, capturing the failure state (logs, assertions) for diagnosis.
- **Run under stress (high concurrency, limited resources) to trigger timing-dependent flakiness.** Race conditions and timing flakiness manifest more under stress (many threads, limited CPU, slow I/O). Run the test under stress (a thread sanitizer, a load tool, reduced resource limits) to force the race.
- **Vary the conditions the test might depend on: time, locale, order, environment.** Run the test with different system times (around midnight, month boundaries), different locales, different test orders, different environments (CI vs local) to find which variation triggers the failure.
- **Use determinism-checking tools (sanitizers, property-based testing).** Thread sanitizers detect races; memory sanitizers detect leaks and use-after-free; property-based testing (with many generated inputs) finds edge cases. These tools surface nondeterminism that manual runs miss.

### Fix The Root Cause: Make The Test Deterministic

The fix for flakiness is eliminating the nondeterminism: making the test's outcome independent of the varying factor.

- **For concurrency flakiness: control the timing, do not rely on it.** Use synchronization (latches, barriers, channels) to coordinate threads deterministically, rather than sleeps or assumptions about order. Test the concurrent behavior with controlled coordination, not with timing-dependent assertions.
- **For shared state: isolate each test's state.** Each test sets up its own state (fresh database rows, fresh globals) and cleans up after. Do not depend on or modify other tests' state. Use test fixtures and setup/teardown to guarantee isolation.
- **For time dependence: inject a controllable clock.** Do not use the system clock directly; inject a clock that the test controls (a fake clock the test advances). This makes time-based logic deterministic and testable.
- **For environment dependence: avoid environment-specific assumptions.** Use relative paths, temp directories, and configurable resources; do not hardcode paths, core counts, or locales. Test in a clean, defined environment.
- **For external dependencies: mock or use test doubles.** Do not call real external services in unit tests; use mocks, stubs, or fakes that provide deterministic responses. For integration tests, use a controlled instance (a test database, a mock server) rather than a real external service.
- **For randomness: seed the random generator or use fixed inputs.** Seed the generator with a fixed value (so the same "random" inputs are used each run), or use fixed inputs that exercise the relevant cases. If the seed is logged on failure, a failing run can be reproduced exactly.
- **For async timing: wait for the condition, not a fixed duration.** Poll for the condition (with a timeout), or use async await, rather than sleeping a fixed time. The test completes as soon as the condition holds, deterministically (passing quickly when it passes) and reliably (failing if the condition never holds within the timeout).

### Use A Quarantine-Then-Fix Workflow For Flaky Tests

A flaky test should not be left in the suite eroding trust, but disabling it permanently loses coverage. Use a quarantine-then-fix workflow.

- **Quarantine the flaky test immediately (move it out of the main suite).** A flaky test in the main suite erodes trust and blocks builds. Move it to a quarantine suite (or mark it disabled, with a tracking issue) so it does not affect the main build.
- **Create a tracking issue for the quarantined test.** A quarantined test must not be forgotten; create an issue tracking its flakiness, its suspected root cause, and its owner. The goal is to fix and un-quarantine it, not to leave it quarantined.
- **Fix the root cause and un-quarantine.** Investigate the root cause, make the test deterministic, verify it no longer flakes (run it many times, under stress), and return it to the main suite. The quarantine is temporary, pending the fix.
- **Monitor the quarantine rate as a health metric.** A growing quarantine (more flaky tests accumulating) indicates a systemic problem (insufficient test discipline, concurrency complexity, environment issues) to address at the root, not test by test.

## Common Traps

### Tolerating Flaky Tests (Rerun Until Green)

Rerunning a flaky test until it passes, treating the green as valid, eroding trust in the suite. Quarantine and fix; do not rerun past flakiness.

### Misdiagnosing Flakiness As A Product Bug (Or Vice Versa)

Treating a flaky test (test nondeterminism) as a product bug, or an intermittent product bug (product nondeterminism) as a flaky test. Investigate which; fix the right thing.

### Addressing The Symptom (Retries, Sleeps) Not The Root Cause

Adding retries or sleeps to mask flakiness, rather than eliminating the nondeterminism. Fix the root cause; do not mask it.

### Disabling A Flaky Test Permanently (Losing Coverage)

Disabling a flaky test and never returning to it, losing the coverage it provided. Quarantine with a tracking issue; fix and un-quarantine.

### Not Reproducing Before Fixing

"Fixing" a flaky test without reliably reproducing the failure, so the fix cannot be verified and may not address the cause. Reproduce reliably first.

### Assuming Local Reproduction Is Sufficient

Fixing based on local runs, missing the CI environment's conditions (load, timing, resources) that trigger the flakiness. Reproduce under the conditions that exhibit the flakiness.

### External Dependencies In Unit Tests

Unit tests calling real external services, introducing real-world nondeterminism. Mock or use test doubles for unit tests; reserve real dependencies for integration tests with controlled instances.

### Time-Based Logic Using The System Clock

Tests using the system clock directly, flaking around time boundaries and across time zones. Inject a controllable clock.

## Self-Check

- [ ] Flakiness is recognized as nondeterminism (a varying factor between runs), and the specific source is identified: concurrency/races, shared state/order dependence, time/date dependence, environment dependence, external dependencies, randomness, or async timing.
- [ ] The failure is confirmed to be flaky (intermittent on the same code, reproduced by multiple runs) rather than a legitimate failure or an intermittent product bug — by running multiple times, in isolation vs in the suite, checking correlation with code changes, and distinguishing test nondeterminism from product nondeterminism.
- [ ] The flakiness is reproduced reliably (run many times, under stress, with varied conditions, using sanitizers and property-based testing) so the fix can be verified, rather than fixing blind.
- [ ] The root cause is fixed by making the test deterministic — controlling timing (synchronization, not sleeps), isolating state (fresh fixtures, cleanup), injecting a controllable clock, avoiding environment assumptions, mocking external dependencies, seeding randomness, and waiting for conditions rather than fixed durations.
- [ ] A quarantine-then-fix workflow is used: flaky tests are quarantined immediately (out of the main suite) with a tracking issue, fixed at the root cause, verified (run many times, under stress), and un-quarantined — not left in the suite eroding trust or disabled permanently losing coverage.
- [ ] The quarantine rate is monitored as a health metric, and a growing quarantine is treated as a systemic problem (test discipline, concurrency complexity, environment issues) to address at the root.
- [ ] New tests are written deterministically from the start (no sleeps, no shared state, no system clock, no unseeded randomness, no external dependencies in unit tests), and review catches nondeterministic patterns before they enter the suite.
- [ ] The team culture treats flakiness as unacceptable (not tolerable), because the harm is not the occasional false alarm but the erosion of trust that causes real failures to be ignored — and this culture is reflected in the workflow (quarantine, track, fix) and the metrics (quarantine rate, time-to-fix).
