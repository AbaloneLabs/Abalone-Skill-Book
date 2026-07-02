---
name: flaky_test_diagnosis_and_prevention.md
description: Use when the agent is diagnosing or preventing flaky tests, investigating nondeterministic test failures caused by timing or ordering or shared state or external dependencies, stabilizing test suites, or deciding when to quarantine versus fix a flaky test.
---

# Flaky Test Diagnosis and Prevention

A flaky test is a test that passes and fails without any code change. One flaky test is tolerable; a suite with many erodes trust until developers stop believing failure signals and start re-running until green. At that point the suite has negative value: it consumes time and attention while providing no confidence. Flakiness is almost never random; it is a deterministic bug hidden behind nondeterministic inputs (time, order, shared state, external systems). The discipline is to find and remove the nondeterminism rather than to tolerate or retry past it.

The judgment problem is identifying which nondeterministic input a flaky test depends on, reproducing the failure reliably, fixing the root cause, and choosing how to handle a flaky test in the interim (quarantine, fix, or disable) without normalizing flakiness. The agent should not treat flakiness as an unavoidable fact of integration testing; nearly all flakiness has a removable cause.

This skill applies whenever you are investigating a test that fails intermittently, hardening a suite against nondeterminism, or deciding policy for quarantining flaky tests.

## Core Rules

### Treat flakiness as a deterministic bug with a hidden input

A flaky test is not "sometimes broken." It is a test whose outcome depends on an input that varies between runs: wall-clock time, execution order, a shared resource, a network call, a random seed, or leftover state. The first step is always to identify the hidden input. Ask: what differs between the run that passed and the run that failed? Common candidates are timing, ordering, external state, and residual data.

### Eliminate nondeterministic inputs at the source

Once the hidden input is identified, remove the dependency:

- **Time**: inject a controllable clock. Tests that read `DateTime.now()` produce different outcomes at different times (timeouts, expiry logic, date-boundary bugs). Freeze or advance time deterministically.
- **Randomness**: inject a seeded RNG. Tests that rely on `Math.random()` without a fixed seed produce different data each run. Seed explicitly and document the seed on failure.
- **Execution order**: tests must not depend on order. Each test must set up and tear down its own state. Run the suite in random order periodically to surface hidden order dependencies.
- **Shared mutable state**: global caches, singletons, class-level or process-level state mutated by one test and read by another. Reset between tests or avoid globals.
- **External systems**: network calls, filesystem outside the test sandbox, real clocks, real threads. Replace with fakes, or run against isolated instances, and assert on deterministic outcomes.
- **Concurrency**: tests that spawn threads/goroutines and join without deterministic synchronization race on timing. Synchronize explicitly or avoid real concurrency in unit tests.

### Make tests reproducible to diagnose

You cannot fix what you cannot reproduce. To force a flaky test to fail reliably:

- Run the single test in a loop (hundreds or thousands of iterations) to surface low-probability failures.
- Run the suite with randomized ordering to expose order dependencies.
- Vary timing (add deliberate delays, run on a loaded machine) to expose races.
- Log the hidden inputs (seed, start time, order) on every run so a failing run can be replayed.

If you cannot reproduce a failure after substantial effort, that itself is information: the dependency is likely on an external system or a rare race, and you should add observability (logging the relevant state) to capture it next time.

### Isolate the environment and dependencies

Flakiness often comes from the environment, not the code:

- **Database**: shared databases with residual data cause order-dependent failures. Use per-test transactions, per-test schemas, or reliable cleanup.
- **Network**: real network calls to external services introduce latency, outages, and rate limits. Stub them or use recorded responses.
- **Filesystem**: tests reading/writing shared paths collide under parallelism. Use per-test temp directories.
- **Ports and resources**: tests binding fixed ports fail when another process holds them. Use ephemeral ports.

### Distinguish test bugs from product bugs

A flaky test sometimes reveals a real product race or timing bug, not a test defect. Before "fixing" the test by making it more lenient, determine whether the flakiness exposes genuine nondeterminism in the product (e.g., a race condition in production code that the test happens to trigger). Loosening a test that catches a real bug hides the bug. Tightening a test that exposes a real race may require fixing the product.

### Avoid timing-based assertions and sleeps

`sleep` and fixed waits in tests are a primary flakiness source: too short and the test races; too long and the suite crawls. Prefer deterministic synchronization:

- Poll for a condition with a timeout (and fail loudly on timeout) rather than sleeping a fixed duration.
- Use synchronization primitives (events, barriers, condition variables) or framework "wait until" helpers.
- For async/eventual-consistency systems, poll for the observable effect rather than guessing how long it takes.

When a sleep seems unavoidable, treat it as a smell: the system under test lacks an observable signal you could wait for instead.

### Establish a flaky-test policy

Without policy, flakiness accumulates. Define:

- **Detection**: run the suite repeatedly and/or in randomized order in CI to catch flakiness before it reaches developers.
- **Quarantine**: a repeatedly-failing flaky test should be quarantined (moved out of the blocking path) with a tracking ticket and a deadline to fix or delete. Quarantine is a temporary measure, not a permanent home.
- **No retries as a fix**: configuring CI to retry failing tests until green hides flakiness and real failures alike. Retries are a last resort for known-external flakiness, not a substitute for fixing causes.
- **Ownership**: every flaky test needs an owner responsible for stabilizing or removing it.

## Common Traps

### Retrying until green

Configuring the suite to retry flaky tests until they pass makes CI green without fixing anything. It hides both flakiness and genuine failures, and trains developers to ignore red builds. Retries are not a fix.

### Sleeps instead of synchronization

A `sleep(100ms)` that makes the test pass on your machine will fail on a slower CI runner or under load. Fixed waits are guesses about timing. Wait for an observable condition instead.

### Order-dependent tests

Tests that pass in order A,B,C but fail in C,B,A rely on residual state. The trap is that the default run order hides the dependency until someone shuffles execution or parallelizes.

### Shared globals and singletons

A cache or singleton mutated by one test and read by another creates invisible coupling. The failure appears only when the mutating test runs first or fails to clean up.

### Real network or clock dependencies

Tests that call real external services or read the wall clock fail when the service is slow, down, or returns different data by time of day. These are not "flaky tests," they are tests with uncontrolled inputs.

### Loosening assertions to stop the flake

Reducing an assertion's strictness to make a test stop failing can hide a real product bug (e.g., widening a timing window, removing an order check that exposed a race). Always determine whether the flake is a test defect or a product defect first.

### Quarantining and forgetting

Moving a flaky test to a quarantine suite without a deadline or owner means it stays broken forever and the underlying defect remains. Quarantine must come with a commitment to fix or remove.

## Self-Check

- Has the hidden nondeterministic input (time, order, shared state, external dependency, randomness, concurrency) behind each flaky test been identified?
- Are clocks injected and frozen, RNGs seeded, and the seed/time logged on failure for reproducibility?
- Is every test independent of execution order, with no reliance on residual state from other tests?
- Are external dependencies (network, database, filesystem, ports) isolated or stubbed so tests do not depend on environment state?
- Have sleeps been replaced with deterministic synchronization (polling for observable conditions, synchronization primitives)?
- Before loosening an assertion to stop a flake, did you confirm whether the flake exposes a real product race rather than a test defect?
- Is there a policy for detecting flakiness (repeated/random-order runs in CI), quarantining with an owner and deadline, and prohibiting retries-as-a-fix?
- Can the failing run be reproduced by logging and replaying the relevant hidden inputs?
- Are shared globals, singletons, and caches reset between tests or avoided entirely?
- Has the suite been run repeatedly and in randomized order to confirm the fixes removed the flakiness?
