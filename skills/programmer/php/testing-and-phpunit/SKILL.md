---
name: php_testing_and_phpunit.md
description: Use when the agent is writing or reviewing PHP tests with PHPUnit, Pest, setting up test suites and phpunit.xml, using mocks stubs and Prophecy/getMockBuilder, data providers and fixtures, testing database code with transactions or refresh, testing Laravel (TestCase, RefreshDatabase, factories) or Symfony (WebTestCase, KernelTestCase), HTTP and controller testing, time and external service isolation, or is diagnosing flaky tests, slow suites, order-dependent failures, "passes alone fails in suite", mock leakage, database state bleed, or tests that exercise the wrong thing. Covers PHPUnit test design, dependency isolation, database and time isolation, framework test helpers, and the pitfalls of over-mocking and shared state.
---

# Testing And PHPUnit In PHP

PHP applications live or die by their test suite, because the language's dynamic typing and the frameworks' heavy "magic" (Laravel facades, Symfony autowiring, Eloquent dynamic properties) produce bugs that nothing but an executed test will catch. The same dynamism makes tests easy to write wrongly: a mock that hides the real service's behavior, a database transaction that bleeds into the next test, a facade whose static state is not reset, a data provider that couples the test to fixture order, a `RefreshDatabase` migration that makes every test slow. The judgment problem is to test behavior at the right level (unit for pure logic, feature/integration for HTTP and database paths), to isolate dependencies at boundaries (clock, mail, HTTP, queue) rather than mocking internal services, to keep database and static state clean between tests, and to recognize when a test is theatre — passing because it mocks everything, proving nothing about the real system.

Agents reach for `getMockBuilder` on every class, run tests against a shared database with no cleanup, use `time()` and real HTTP in tests, and assert on internal method calls. The result is a suite that is slow, flaky, and fragile. The remedy is to test at the level where behavior is observable, to mock at boundaries, to use transactional database isolation or `RefreshDatabase`, to freeze time and stub external I/O, and to keep the suite order-independent.

## Core Rules

### Test At The Level Where Behavior Is Observable

PHP frameworks offer layered test types: pure unit tests (a class in isolation), framework-aware tests (Laravel `TestCase` unit, Symfony `KernelTestCase`), and feature/HTTP tests (Laravel feature tests, Symfony `WebTestCase`) that exercise the full request-response cycle or database. Choose the level by what you need to prove: pure logic as a unit test (fast, no framework); a controller/service with dependencies as a framework test; an end-to-end HTTP path as a feature test. Testing everything as feature tests is slow; testing everything as unit tests with heavy mocking proves nothing about integration. Balance: most tests at the unit/service level, a thinner layer of feature tests for the critical paths.

- Unit test for pure logic and isolated services (no I/O, no framework).
- Framework test (KernelTestCase/TestCase) for services needing the container or a real database.
- Feature/HTTP test for the critical request-response paths and integration.

### Mock At Boundaries, Not Internal Collaborators

Mocking (PHPUnit's `createMock`, Prophecy, Mockery, Pest's mocks) is for boundaries: external HTTP, mail, queue, the filesystem, the clock, payment gateways — dependencies that are slow, non-deterministic, or destructive in a test. Mocking internal domain services (a controller's own service, a service's collaborator in the same codebase) produces tests that pass in isolation and fail in integration, because the mock drifts from the real implementation. Prefer real collaborators within the domain; mock the edges. Use fakes (an in-memory repository) for stateful collaborators where a mock would be heavy.

- Mock boundaries (HTTP via HttpFake/guzzle mock handler, mail via log/fake transport, queue via the sync/fake driver, time via travel helpers).
- Real domain collaborators; fake repositories for stateful data.
- Reserve `createMock` for the boundary contract, not for the unit under test's internals.

### Isolate Database State Per Test

Database state bleeding across tests is the dominant source of flaky PHP suites. Use one of: `RefreshDatabase` (Laravel, re-migrates per test — safe but slower), database transactions that roll back per test (wrap each test in a transaction that is never committed — fast), or a dedicated test database reset between runs. Do not run tests against the dev database, and do not rely on seed data that one test reads and another writes. The suite must pass in any order; an order-dependent failure reveals hidden state coupling.

- `RefreshDatabase` (Laravel) or `TransactionAwareTest`/per-test transaction (Symfony) for isolation.
- A dedicated test database; never the dev/shared database.
- Create the rows each test needs; do not assert on global counts that include other tests' data.

### Freeze Time And Stub External I/O

Non-determinism makes tests flaky. Freeze the clock (Laravel's `$this->travelTo(...)`, Carbon's `Carbon::setTestNow()`, or a `Clock` interface you inject), stub HTTP (Laravel `Http::fake`, Guzzle mock handler), and use fake transports for mail and the sync/fake driver for queues. A test that reads `time()`, hits a live API, or sends real mail will fail intermittently across time zones, daylight saving, and outages, eroding trust in the suite.

- `Carbon::setTestNow('2024-01-01')` / `$this->travelTo(...)` for time-dependent logic.
- `Http::fake([...])` / Guzzle mock handler for HTTP; log/fake mailer; sync/fake queue.
- Inject a `Clock` so time is controllable and the production code is testable.

### Use Data Providers And Factories Deliberately

Data providers run the same test against many inputs (good for edge cases), but they are computed at test discovery time (before `setUp`), so they cannot depend on per-test state — keep them pure. Factories (Laravel factories, `Factory` classes) build domain entities with sensible defaults and overrides per test; prefer `make` (in-memory) over `create` (persisted) where the database is not needed, and keep factories minimal (only required fields, no cascades that slow every test). Name fixtures meaningfully.

- Data providers pure (no per-test state); use them for parameterized edge cases.
- `Factory::new()->make()` over `create()` where persistence is not needed.
- Minimal factories (required fields only, override per test); no cascade callbacks that build unrelated data.

### Keep Static And Facade State Reset Between Tests

Laravel facades and static state retain values across tests unless reset. Most framework test cases reset the container per test, but manually set static state (a facade mock, a `Config::set`, a container singleton override) can leak. Reset in `tearDown` or use the framework's reset hooks. A facade that one test mocks and the next test expects real will fail confusingly if the mock leaks.

- Reset container/facade overrides in `tearDown` or rely on the framework's per-test reset.
- Avoid global/static mutable state in production code; it makes tests order-dependent.

## Common Traps

### Mocking The Unit Under Test

`createMock(TheService::class)` then testing it asserts nothing real. Test the real service; mock its dependencies.

### Database State Bleeding

Tests pass alone, fail in the suite (or vice versa) because one leaves rows another reads. Use transactional isolation; run in random order.

### Real Time And Real HTTP

`time()` and live HTTP make tests flaky. Freeze time; fake HTTP.

### Feature Tests For Everything

Testing every unit through a full HTTP request is slow and couples tests to routing/HTTP. Use unit/service tests for logic.

### Factory Cascades Slowing The Suite

A factory that builds a user, a company, a billing profile per `create` makes every test slow. Keep factories minimal.

### Data Provider Depending On Per-Test State

Data providers run before `setUp`; referencing per-test state breaks. Keep providers pure.

### Facade/Static State Leaking

A mocked facade or container override not reset affects later tests. Reset in teardown.

### Asserting On Method Calls Instead Of Outcomes

`->expects($this->once())->method('save')` couples to implementation; a correct refactor that caches fails it. Assert on the outcome.

## Self-Check

- [ ] Tests run at the right level (unit for pure logic, framework/service for container/DB, feature for critical HTTP paths), with the bulk at the unit/service level and a thinner feature layer.
- [ ] Mocks target boundaries (HTTP, mail, queue, time, payment); real domain collaborators are exercised; stateful collaborators use fakes rather than heavily-mocked mocks.
- [ ] Database state is isolated per test (`RefreshDatabase` or per-test transactions) on a dedicated test database, the suite passes in random order, and no test depends on another's rows.
- [ ] Time is frozen (`travelTo`/`setTestNow`/injected `Clock`), HTTP/mail/queue/filesystem are stubbed or faked, and no test depends on real time or live external services.
- [ ] Data providers are pure (no per-test state), factories are minimal (`make` over `create`, no cascades), and fixtures are meaningfully named.
- [ ] Facade/static/container state is reset between tests (framework reset or explicit `tearDown`), and no global mutable state in production code makes tests order-dependent.
- [ ] No test mocks the unit under test, and assertions target observable outcomes rather than internal method call counts.
- [ ] The suite has been considered under random order, parallel execution, and long-running CI, and remains fast, deterministic, and trustworthy.
