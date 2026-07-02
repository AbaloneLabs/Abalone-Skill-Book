---
name: test_data_and_fixtures.md
description: Use when the agent is designing test fixtures or builders, managing shared test state, cleaning up data between tests, generating production-like test data, or handling sensitive information such as PII or secrets inside test fixtures and seed data.
---

# Test Data and Fixtures

Test data is the substrate of every test, and bad test data produces bad tests in ways that are hard to see. Fixtures that share mutable state make tests order-dependent, hand-written seed data that drifts from production shapes makes tests pass against fiction, and fixtures that embed real customer data or secrets create security and compliance incidents. The data behind a test is as much a design decision as the assertions, and it is usually made carelessly.

The judgment problem is structuring fixtures so tests are independent and readable, generating data that resembles production closely enough to catch real bugs without embedding real sensitive information, and managing lifecycle (creation, sharing, cleanup) so the suite stays fast and reliable. The agent should not paste a giant shared setup block and assume it serves every test.

This skill applies whenever you are creating or reviewing test setup, fixtures, factories, seed data, or test databases.

## Core Rules

### Make fixtures explicit and minimal per test

A test should make visible the data it depends on. The ideal is that a reader can understand the test's preconditions from the test itself. Techniques:

- **Inline minimal data**: create only the entities the test needs, with only the fields that matter. Avoid loading a giant shared fixture file when the test uses two rows.
- **Builders/factories**: provide fluent builders that construct valid entities with sensible defaults, letting each test override only the fields relevant to its scenario. This keeps tests readable while avoiding the duplication of constructing full entities by hand.
- **Object mothers**: for complex domain objects with many valid configurations, a small set of named factory methods ("a paid order," "a cancelled order") communicates intent better than dozens of constructor arguments.

Weak choice: every test re-loading the same 500-line YAML fixture and ignoring most of it. Strong choice: each test building exactly the entities it needs via a factory.

### Avoid shared mutable state

Shared fixtures that tests both read and mutate are the leading cause of order-dependent, non-parallelizable suites. Rules:

- Treat fixtures as immutable by default. If a test needs to modify data, it should create its own copy.
- Never let one test's mutations persist into another test's run. Either rebuild per test, use per-test transactions that roll back, or clean up explicitly.
- Shared read-only reference data (lookup tables, configuration) is fine; shared mutable business data is not.

### Design factories around validity and intent

A factory's default should always produce a valid, persistable entity, so that `aUser()` just works and tests do not have to specify every required field. Layer intent on top: `aUser().banned()` clearly expresses the scenario. Common pitfalls:

- Factories whose defaults produce invalid state (missing required fields) force every test to specify them, defeating the purpose.
- Factories with too many unrelated defaults make it hard to see which fields matter to a given test.
- Factories that auto-generate unrelated associations can create surprising side effects (e.g., creating a whole object graph when the test only wanted one entity).

Keep factories focused: valid defaults, explicit overrides, and minimal automatic association.

### Keep test data production-like in the dimensions that matter

Tests pass against data that does not resemble production, then fail in production. Make test data realistic in the ways that affect correctness:

- **Cardinality and scale**: test with realistic row counts where pagination, batching, and performance matter. A pagination test with 3 rows will not catch a deep-pagination bug.
- **Value distributions**: use realistic distributions, not always `"test"`, `1`, or empty strings. Edge cases (empty, very long, unicode, null) should be deliberately included.
- **Shape evolution**: when production schema or domain shapes change, update factories so tests do not silently test obsolete shapes.

Balance realism against readability: a test should still make its scenario obvious. Use realistic data where it matters, simple data where it does not.

### Handle sensitive information deliberately

Test data is a frequent vector for leaks of real customer data or secrets:

- **Never embed real PII or production data** in fixtures unless it has been anonymized and approved. Production data dumps in test repos are a compliance incident waiting to happen.
- **Never commit real secrets** (API keys, passwords, certificates) to test fixtures. Use dummy/fake values, and load any needed test secrets from environment variables or a secrets manager, not the repository.
- **Anonymize or synthesize** when you need production-like volume: generate synthetic data that matches the statistical shape without containing real records.
- **Scrub CI artifacts**: ensure test logs, screenshots, and database snapshots produced in CI do not retain sensitive values.

### Manage lifecycle and cleanup explicitly

Decide and enforce a data lifecycle strategy:

- **Per-test isolation**: each test creates what it needs and cleans up (or rolls back) afterward. This is the safest default.
- **Per-suite/session setup**: only for genuinely expensive, read-only reference data. Never for mutable business data shared across tests.
- **Cleanup reliability**: prefer transactional rollback or per-test database/schema over manual deletion, because manual cleanup is forgotten when tests fail mid-way.
- **Avoid relying on test teardown for correctness**: a test that "works" only because the next test cleans up its mess is fragile.

### Separate data for different test layers

Different test layers need different data strategies:

- **Unit tests**: plain in-memory objects, no persistence.
- **Integration tests**: factory-built entities persisted in a real (isolated) database.
- **End-to-end/contract tests**: seeded scenarios that represent realistic user journeys.

Do not force one data strategy across all layers; each has different realism and isolation needs.

## Common Traps

### One giant shared fixture for everything

A monolithic seed file loaded by every test couples all tests to the same data, makes individual test preconditions invisible, and guarantees order dependence when tests mutate it. Prefer per-test factories.

### Factories that produce invalid defaults

If `aUser()` is missing a required field, every test must supply it, and the factory provides little value. Defaults must be valid.

### Using real production data or secrets in fixtures

Pasting a real customer record or a real API key "for realism" is a security and compliance incident. Synthesize or anonymize instead.

### Always using the same trivial values

Fixtures where every name is "test" and every amount is 1 will not catch bugs involving long strings, zero, negative numbers, unicode, or null. Deliberately include boundary and edge values.

### Relying on test execution order

Tests that depend on data created by an earlier test cannot be run in isolation or in parallel, and break when the order changes. Each test must be self-contained.

### Forgetting cleanup when a test fails

If cleanup only runs on success, failed tests leave polluted state that breaks subsequent runs. Use transactional rollback or framework-managed teardown that runs regardless of outcome.

### Over-associating in factories

A factory that automatically builds a full object graph (user with orders with items with products) creates noise and side effects when a test only wanted a user. Make associations explicit and opt-in.

### Letting fixtures drift from production shapes

When the domain evolves but factories stay frozen, tests verify obsolete shapes and miss real bugs. Treat factories as production code that must track the domain.

## Self-Check

- Does each test make its data preconditions visible and minimal, via factories or inline construction, rather than relying on a shared monolithic fixture?
- Are factories producing valid defaults with intent-revealing overrides, so tests read clearly?
- Is shared state read-only, with all mutable data created fresh per test and cleaned up via rollback or reliable teardown?
- Is test data realistic in the dimensions that matter (cardinality, distributions, edge values) without sacrificing readability?
- Are real PII, production data, and secrets excluded from fixtures, with synthetic or environment-sourced values used instead?
- Is cleanup reliable regardless of test outcome (transactional rollback preferred over manual deletion)?
- Are tests independent of execution order and parallelizable?
- Are different test layers using data strategies appropriate to their realism and isolation needs?
- Do fixtures deliberately include boundary and edge values (empty, long, unicode, null, zero, negative)?
- Are factories and seed data maintained alongside domain changes so they do not drift from production shapes?
