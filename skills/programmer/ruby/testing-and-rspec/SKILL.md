---
name: ruby_testing_and_rspec.md
description: Use when the agent is writing or reviewing Ruby tests with RSpec or Minitest, designing test suites and contexts, using mocks stubs doubles and spies, testing time and I/O dependencies, shared examples and helpers, let vs before hooks, system/feature tests with Capybara, factory_bot vs fixtures, test data isolation and database cleaning, or is diagnosing flaky tests, slow suites, order-dependent failures, "it passes alone but fails in the suite", mock leakage, or tests that exercise the wrong thing. Covers test design judgment, dependency isolation, the tradeoffs of mocking, database and time isolation, and the pitfalls of over-mocking and shared state.
---

# Testing And RSpec In Ruby

Ruby's dynamic nature makes tests both essential and treacherous. Essential, because nothing else catches the class of bugs the type system will not: a misspelled method, a `nil` where an object was expected, a callback that fires in the wrong order, a monkey-patch that changes core behavior. Treacherous, because the same dynamism that makes the bugs also makes tests easy to write wrongly: a stub that hides the real collaborator's behavior, a `let` that memoizes across an example in a surprising way, a global database state that makes tests order-dependent, a mock that asserts the implementation instead of the outcome. The judgment problem is to write tests that exercise behavior (not implementation), to isolate the unit under test from its real dependencies only where doing so makes the test honest and fast, to keep test data and global state clean between examples, and to recognize when a test is theatre — passing because it mocks everything, proving nothing about the real system.

Agents tend to reach for RSpec's heavy machinery (`allow`, `expect`, `double`, `let`, `before`) reflexively, producing suites that are slow (every example rebuilds a large object graph), flaky (shared database rows, real time, real network), and fragile (asserting on method-call counts that change with every refactor). The remedy is to test at the level where behavior is observable, to mock at boundaries (external services, time, randomness) rather than internal collaborators, to use factories or fixtures deliberately with transactional cleanup, and to keep the suite order-independent and fast.

## Core Rules

### Test Behavior, Not Implementation

A good test asserts on observable outcomes (return values, state changes, published messages, persisted records), not on the internal method calls used to produce them. Tests that assert `expect(service).to receive(:compute)` then call the method are testing that the code calls a method, not that the result is correct; they pass while the result is wrong and fail when the implementation is refactored correctly. Prefer to call the real collaborators and assert on the outcome; mock only when the real collaborator is slow, non-deterministic, external, or destructive.

- Assert on the result: `expect(result).to eq(expected)`, on state: `expect(record.reload.status).to eq('paid')`, on side effects at the boundary: `expect(mailbox).to have_received(:deliver)`.
- Reserve `expect(...).to receive(...)` for boundary collaborators (a payment gateway, a mailer, a clock) where calling the real one is wrong in a test.
- Tests that mock the unit under test's own internals provide no regression value; they encode the current implementation.

### Mock At Boundaries, Not At Internal Collaborators

Mocking is a tool for isolation at boundaries — external HTTP services, the system clock, randomness, the filesystem, email, background jobs — where the real dependency is slow, costly, non-deterministic, or destructive. Mocking internal domain collaborators (a service that calls another service in the same codebase) tends to produce tests that pass in isolation and fail in integration, because the mock's behavior drifts from the real collaborator's. Prefer real collaborators within the domain; mock the edges.

- Stub `Time.now`/`Time.current` with `Timecop.freeze` or `travel_to` so time-dependent logic is deterministic.
- Stub external HTTP with WebMock/VCR so tests do not hit the network and are reproducible.
- Use test adapters for background jobs (Sidekiq::Testing, ActiveJob test helpers) so jobs are run inline or captured, not enqueued to a real queue.

### Keep Test Data Isolated And The Suite Order-Independent

A test that depends on rows left by an earlier test is order-dependent and flaky. Use transactional fixtures (each example runs in a transaction that rolls back) or a deterministic cleanup strategy so each example starts from a known database state. Avoid relying on seed data or `let!`-created rows that leak. The suite must pass in random order (`config.order = :random`); a failure under random order reveals hidden state coupling.

- Enable transactional cleanup (`use_transactional_fixtures` in Rails, or DatabaseCleaner with `:transaction` strategy) so DB changes roll back per example.
- Do not assert on `Model.count` across the whole table; create the rows the test needs and assert on them specifically.
- Run the suite in random order in CI; fix order-dependence rather than pinning the order.

### Use let, before, And shared_context Sparingly And Predictably

RSpec's `let` memoizes per example, `before` runs setup per example (or per suite with `:context`), and `shared_context`/`shared_examples` factor repetition. Each is useful and each is a footgun: `let` is lazy (not evaluated unless referenced, which surprises readers who expect setup to run), `before(:each)` that builds expensive objects slows every example, and over-factored `shared_examples` obscure what a specific test actually checks. Keep setup explicit and local where possible; extract to `let`/`shared_context` only when the repetition is real and the abstraction aids reading.

- Prefer local variables in an example for one-off setup; use `let` for values shared across examples in a group.
- Avoid deep `shared_example` hierarchies that require magical block variables; a reader should understand a test without opening three files.
- `let!` forces evaluation (like a `before`); use it deliberately when the lazy version hides intent.

### Choose Factories Or Fixtures Deliberately, And Keep Them Minimal

`factory_bot` factories build records flexibly but can hide required associations and create large cascades (`create(:user)` that builds a company, a plan, a billing profile). Fixtures are static, fast, and shared, but rigid and easy to couple tests to. Either is fine; the trap is factories that create more than the test needs (slow, coupling-prone) or fixtures whose meaning is opaque (`users(:one)`). Keep factories minimal (use `build` over `create` where a DB row is not needed, use `build_stubbed` to avoid the DB entirely), and name fixtures meaningfully.

- `build` over `create` when the test does not need the record persisted; `build_stubbed` when it only needs an object that quacks like a record.
- Define only the required attributes in the factory; override per test. Avoid factory callbacks that create cascades.
- Name fixtures by role (`users(:admin)`, not `users(:one)`) so tests read clearly.

### Test Time, Randomness, And External I/O Deterministically

Non-determinism is the root of flaky tests. Freeze time (`travel_to`), seed randomness (`srand`/`Random.stub`), and stub external I/O (HTTP, mail, filesystem writes) so the same test produces the same result every run. A test that depends on "now", on a random shuffle, on a network call, or on the filesystem state will fail intermittently and erode trust in the suite.

- `travel_to(Time.zone.parse('2024-01-01'))` for any logic that reads the current time.
- Stub or seed `rand`/`Array#sample` where randomness affects the path.
- WebMock/VCR for HTTP; fake filesystem or tmpdir for file writes; test adapters for mail and jobs.

## Common Traps

### Mocking The Unit Under Test

`expect(service).to receive(:helper_method)` inside a test of `service` asserts the implementation and breaks on refactor. Call the real method and assert the outcome.

### let Lazy Evaluation Surprising Readers

A `let(:user)` not referenced in a failing example was never created, so the DB is empty and the failure is confusing. Use `let!` or a `before` when setup must run regardless.

### Order-Dependent Tests From Shared DB State

Tests pass alone and fail in the suite (or vice versa) because one leaves rows another reads. Enable transactional cleanup and random order; fix the coupling.

### Factory Cascades Slowing The Suite

`create(:order)` that builds a user, a product, a company, and a shipping profile makes every order test slow. Keep factories minimal; use `build`/`build_stubbed`.

### Asserting On Method Call Counts

`expect(mailer).to receive(:send).exactly(1).times` couples to implementation and breaks when a refactor sends two legitimate mails. Assert on the observable effect instead.

### Real Time And Real Network In Tests

`Time.now` and live HTTP make tests flaky across time zones, daylight saving, and service outages. Freeze time and stub HTTP.

### before(:each) Building Expensive Objects

A `before(:each)` that creates a large graph slows every example, even those that do not need it. Move expensive setup into the examples that need it or use `let`.

### Stubs That Leak Across Examples

A stub set in one example but not torn down (rare with RSpec's per-example isolation, common with global state or constants) affects later examples. Keep stubs local; reset globals in `after`.

## Self-Check

- [ ] Tests assert on observable outcomes (return values, state, persisted records, boundary side effects), not on internal method calls of the unit under test.
- [ ] Mocks and stubs target boundaries (external HTTP, time, randomness, mail, jobs, filesystem), not internal domain collaborators; real collaborators are exercised within the domain.
- [ ] Each example starts from known database state (transactional cleanup or deterministic strategy), the suite passes in random order, and no test depends on rows left by another.
- [ ] `let`/`before`/`shared_context` are used sparingly and predictably; setup is local and explicit where possible, and lazy `let` surprises are avoided with `let!` or explicit setup.
- [ ] Factories are minimal (`build`/`build_stubbed` over `create`, no cascades), or fixtures are meaningfully named, and tests create only the data they need.
- [ ] Time is frozen, randomness is seeded, and external I/O (HTTP, mail, file, jobs) is stubbed or run through test adapters, so tests are deterministic.
- [ ] No test asserts on implementation details (call counts, private method invocation) that would break under a correct refactor.
- [ ] The suite has been considered under random order, parallel execution, and long-running CI, and remains fast, deterministic, and trustworthy.
