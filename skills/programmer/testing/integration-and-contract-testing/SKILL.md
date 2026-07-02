---
name: integration_and_contract_testing.md
description: Use when the agent is designing integration tests, defining contract tests or consumer-driven contracts, scoping integration boundaries, isolating test environments, testing external service interactions, or diagnosing flaky integration tests and shared-environment coupling.
---

# Integration and Contract Testing

Unit tests verify that a component works in isolation; integration tests verify that components work together. The gap between "passes unit tests" and "works in the real system" is where most production bugs live, because the assumptions each component makes about its collaborators are exactly what unit tests cannot check. But integration testing is expensive, slow, and notoriously flaky when done poorly, and the instinct to "test everything end-to-end" produces suites that are impossible to maintain. The art is choosing the right integration boundaries and the right level of realism for each.

The judgment problem is deciding what to integrate, what to replace with doubles, how to verify cross-service compatibility without running the whole world, and how to keep integration tests deterministic and isolated. The agent should not equate "integration test" with "spin up every dependency"; the goal is to verify the seams that actually break.

This skill applies whenever you are designing tests that cross process, service, module, or database boundaries, or reviewing a test suite whose integration tests are slow or unreliable.

## Core Rules

### Choose integration scope by what is most likely to break

Integration tests pay for themselves only at the seams that actually fail. Useful integration boundaries include:

- **Application-to-database**: verify that ORM mappings, migrations, constraints, and queries behave as the application expects. This catches a large class of bugs that mocking the repository hides.
- **Service-to-service**: verify that two services agree on request/response shapes, error semantics, and auth. This is where contract testing shines.
- **Application-to-infrastructure**: verify behavior against a real cache, queue, or filesystem rather than an in-memory fake, because fakes often diverge from real semantics.

Avoid integration tests that verify things unit tests already cover. The integration test should exercise the wiring and the assumptions, not re-test internal logic.

### Use contract testing to verify cross-service compatibility cheaply

When two services communicate over an API, end-to-end tests that run both are expensive and brittle. Contract testing decouples them:

- **Provider contract**: the provider publishes (or verifies against) examples of the requests it accepts and the responses it produces.
- **Consumer-driven contracts (CDC)**: each consumer publishes the specific interactions it relies on; the provider verifies it satisfies all consumers' expectations. This catches the common failure where a provider changes a response shape that a consumer depends on, without running both together.
- **Pact-style verification**: consumers generate pacts from their tests; providers replay those pacts against their own code. This gives each side fast, isolated tests while still verifying compatibility.

Contract testing is most valuable when a service has many consumers or when end-to-end environments are scarce. It is overkill for a monolith with internal boundaries.

### Replace external dependencies deliberately, not reflexively

For third-party services (payment processors, email providers, cloud APIs), decide between:

- **Test doubles/stubs**: fast and deterministic, but only as accurate as your model of the service. They drift from the real API silently.
- **Sandboxed/recorded real calls**: use the provider's sandbox, or record real responses and replay them (VCR-style). More realistic but can break when the provider changes.
- **Contract verification against the provider's published contract**: catches shape changes but not behavioral changes.

The trap is mocking a third-party service once and trusting the mock forever. Pair stubs with periodic real-call smoke tests or contract verification to catch drift.

### Isolate integration test environments

Integration tests are flaky most often because of shared state or shared environments:

- **Database isolation**: each test should start from a known state. Use per-test transactions that roll back, per-test schemas/databases, or aggressive cleanup. Tests that depend on data left by earlier tests couple execution order and break under parallelism.
- **Service isolation**: avoid a shared always-on integration environment that multiple branches pollute. Prefer ephemeral, per-run environments (containers, testcontainers) so tests cannot interfere.
- **Time and randomness**: freeze clocks and seed RNGs so results are reproducible.

### Keep integration tests fast enough to run regularly

A suite that takes 20 minutes to run integration tests gets skipped, which means it protects nothing. Techniques to keep it runnable:

- Run only the affected subset in CI based on changed paths.
- Parallelize independent tests, but only after ensuring no shared-state coupling.
- Use lightweight real dependencies (in-process embedded databases, testcontainers) rather than full managed services.
- Reserve the slowest, most realistic end-to-end tests for a separate, less frequent pipeline, and keep the fast integration suite on the main path.

### Verify error and edge paths across boundaries, not just happy paths

Integration tests tend to cover the happy path and miss the seams that fail under stress. Deliberately test:

- Connection failures and timeouts to dependencies.
- Malformed or unexpected responses from collaborators.
- Partial failures (one of three downstream calls fails).
- Retry and idempotency behavior across the boundary.
- Transaction rollback and consistency when a step fails mid-flow.

## Common Traps

### Mocking the database and calling it an integration test

Mocking the repository layer and testing the service against the mock verifies nothing about real SQL, constraints, or transaction behavior. It is a unit test wearing an integration test's costume. Use a real database (embedded or containerized) for integration tests of persistence.

### Shared mutable state across tests

Tests that rely on or modify a shared database leave state that changes other tests' outcomes. The suite passes in one order and fails in another, or fails under parallelism. Every test must be independent.

### A single polluted shared environment

A long-lived "integration environment" that every branch deploys to becomes a source of cross-branch interference and drift from production. Prefer ephemeral environments per run.

### Trusting a third-party mock indefinitely

A stub of a payment API written once will not tell you when the real API changes a field or a status code. Without periodic real verification or contract checks, the mock gives false confidence.

### End-to-end tests for everything

End-to-end tests are the most expensive and flakiest. Using them to verify logic that a unit or narrow integration test could cover makes the suite slow and unreliable. Reserve end-to-end for the few critical user journeys that justify the cost.

### Ignoring failure-path integration

Testing only the happy path across services misses the wiring failures that cause production incidents: timeouts, partial failures, and inconsistent error handling. Deliberately inject failures at integration boundaries.

### Coupling tests to implementation order

Tests that must run in a specific sequence (because each sets up state for the next) are brittle and cannot be parallelized. Each test should set up and tear down its own world.

## Self-Check

- Is each integration test exercising a seam that unit tests cannot cover (database, service boundary, infrastructure), rather than re-testing internal logic?
- For cross-service compatibility, are you using contract or consumer-driven contract testing instead of running every service together for every test?
- Are external third-party dependencies stubbed with a periodic real-call or contract-verification safety net to catch drift?
- Is each test isolated: independent database state, no reliance on execution order, parallelizable?
- Are environments ephemeral per run rather than a single shared long-lived environment?
- Is the integration suite fast enough to run on the main CI path, with the slowest end-to-end tests in a separate pipeline?
- Do integration tests cover failure paths (timeouts, partial failures, malformed responses, rollback) across boundaries, not just the happy path?
- Are clocks frozen and randomness seeded so results are reproducible?
- For persistence integration tests, are you using a real database rather than a mocked repository?
- Have you confirmed the suite is not flaky by running it repeatedly and under parallelism?
