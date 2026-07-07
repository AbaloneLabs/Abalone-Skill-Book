---
name: elixir_testing_and_exunit.md
description: Use when the agent is writing Elixir tests with ExUnit (case tests, describe blocks, async tests, data_case/conn_case), designing testable code (dependency injection, Mox mocks, bypass for HTTP), testing GenServers/processes, testing Ecto sandbox transactions, property-based testing with StreamData, testing LiveView, organizing test suites, or is diagnosing "tests are flaky with processes/GenServers", "async tests cause shared-state races", "Mox not expecting or not allowed", "Ecto sandbox ownership issues", or test-isolation problems. Covers ExUnit structure and async safety, Mox for behavior-based mocking, Bypass for HTTP, Ecto sandbox/ownership, process/GenServer testing, StreamData property tests, and isolation/flakiness pitfalls.
---

# Testing And ExUnit In Elixir

ExUnit is Elixir's testing framework, and its concurrency model (`async: true` runs test modules in parallel) makes tests fast but demands careful isolation. Agents flip `async: true` on tests that share global state (the Application environment, a named process, a single database table) and get races; misuse Mox (forgetting `Mox.defmock`/`expect`/`verify`/`set_mox_from_context`/`verify_on_exit!`); fight the Ecto sandbox ownership rules (a test's DB connection is owned by the test process, so a background GenServer started by the test cannot use the DB without `allow`); or test GenServers by poking internals rather than through their public API. The judgment problem is to keep tests isolated (only `async: true` when truly stateless), to mock via Mox behaviors with proper setup, to manage Ecto sandbox ownership across processes, to test processes through their public API, and to use property-based testing (StreamData) where it adds value.

Agents race async tests, fight Mox/Ecto ownership, or test process internals. The remedy is disciplined isolation, behavior-based mocking with proper allow/verify, ownership-passing for background processes, and API-level testing.

## Core Rules

### Structure Tests And Use async: true Only When Truly Stateless

ExUnit runs test *modules* marked `async: true` in parallel. This is fast but unsafe if the module touches shared mutable state: the Application environment (`Application.put_env`), a named/global process (`Registry`, a singleton GenServer), a shared Ecto repo without sandbox, or global files/ports. Default a test module to `async: false` if it mutates global state; `async: true` only for pure/stateless tests (pure functions, per-test-spawned processes, sandboxed DB with ownership). Within a module, `describe` blocks group related tests; `setup`/`setup_all` run per-test/per-module. Keep `setup` fast and per-test (avoid `setup_all` for mutable fixtures).

- `async: true` only for stateless tests; `async: false` when touching global state (Application env, named processes, shared resources).
- `describe` for grouping; `setup` per-test (fast, isolated); avoid `setup_all` for mutable fixtures.
- Default to `async: false` if unsure; opt into `async: true` deliberately.

### Mock Via Mox (Behaviors) With Proper Setup

Mox mocks an Elixir *behavior* (a `@callback`-defined contract): define the behavior, define a mock with `Mox.defmock(MyMod.Mock, for: MyBehavior)`, configure the app to use the mock (`Application.put_env` in `config/test.exs`), then in tests `expect`/`allow`/`verify`. Setup is critical: `Mox.set_mox_from_context` (so `expect`s respect async), `Mox.verify_on_exit!` (assert all expects called), and `allow(Repo.Mock, self(), some_pid)` to permit a *background process* (a GenServer the test spawned) to call the mock (mocks are owned by the test process by default). Forgetting `verify_on_exit!` lets an unmet expectation pass; forgetting `allow` makes a spawned process's mock call fail. Mock at boundaries (HTTP clients, third-party APIs), not internal modules.

- Define a behavior + `Mox.defmock`; configure the app to use the mock in `config/test.exs`.
- `Mox.set_mox_from_context` + `Mox.verify_on_exit!` in setup; `allow(Mock, self(), pid)` for spawned processes.
- Mock at boundaries (HTTP, third-party), not internal modules; assert on real behavior where possible.

### Use Bypass For HTTP, Not A Real External Service

For HTTP, `Bypass` (a local HTTP server in the test) lets you stub/assert on outbound calls without hitting a real service. Start `Bypass.open()`, `Bypass.expect(conn -> ...)` to respond, point the app at the Bypass URL (`Application.put_env`), and assert the call was made. This is more robust than mocking the HTTP client (it tests the real HTTP path) and avoids network flakiness. Use it for any outbound HTTP the code makes; combine with Mox only where Bypass is impractical.

- `Bypass` for outbound HTTP: stub responses, assert calls, no real network.
- Point the app at the Bypass URL via `Application.put_env`; prefer over mocking the HTTP client.

### Manage Ecto Sandbox Ownership Across Processes

Ecto's sandbox (`Ecto.Adapters.SQL.Sandbox`) runs each test in a transaction that's rolled back, giving isolation. The transaction is *owned* by the test process; a background process (a GenServer the test spawned) cannot use the DB unless ownership is passed with `allow(repo, self(), pid)` (or `Ecto.Adapters.SQL.Sandbox.allow`). For tests that spawn processes accessing the DB, `allow` each one. For ownership mode, `Ecto.Adapters.SQL.Sandbox.mode(Repo, {:shared, self()})` shares with all processes (less isolated, use sparingly). `DataCase`/`ConnCase` boilerplate handles the common path; understand it so spawned-process DB access works.

- Sandbox transaction is owned by the test process; `allow(repo, self(), pid)` to pass to spawned processes.
- `{:shared, self()}` mode shares with all (less isolated); use sparingly.
- `DataCase`/`ConnCase` setup handles the common path; `allow` for background processes accessing the DB.

### Test GenServers/Processes Through Their Public API

Test a GenServer through its public API (`GenServer.call`/`cast`/name), not by sending raw messages to its internals or inspecting its state. This tests the contract (stable) rather than the implementation (fragile). For process lifecycle, use `start_supervised` (links the process to the test, ensures cleanup). For timing/flakiness, avoid `Process.sleep` (flaky); use `assert_receive`/`refute_receive` with a timeout to wait for expected messages deterministically. For a process that should crash/restart, assert on the supervisor's behavior, not internals.

- Test processes via their public API (`call`/`cast`/name), not raw messages or state inspection.
- `start_supervised` for lifecycle/cleanup; `assert_receive`/`refute_receive` (not `Process.sleep`) for timing.
- Assert on observable contract/supervisor behavior, not internals.

### Use StreamData For Property-Based Testing Where It Adds Value

StreamData generates many inputs for a property, finding edge cases. Use it for pure functions with clear invariants (a parser round-trips, a sort is stable, an encoding is bijective). Write generators for domain types (`StreamData.list_of/2`, custom `gen`); keep properties deterministic and pure. Property tests are not a replacement for example tests; use both. A failing property's counterexample should be added as a regression example test.

- StreamData for pure functions with clear invariants; custom generators for domain types.
- Properties are deterministic/pure; add failing counterexamples as regression examples.
- Combine with example tests, not replace them.

## Common Traps

### async: true With Shared State

Global env/named processes/shared DB cause races. Use `async: false` or per-test isolation.

### Forgetting Mox verify_on_exit! / allow

Unmet expects pass silently; spawned processes can't call the mock. Add both in setup.

### Testing Process Internals

Sending raw messages/inspecting state couples tests to implementation. Use the public API.

### Process.sleep Causing Flaky Tests

Timing-based sleeps race or slow tests. Use `assert_receive` with a timeout.

### Ecto Sandbox Ownership Errors

A spawned GenServer can't access the DB. `allow(repo, self(), pid)` to pass ownership.

### Mocking Internal Modules

Mocking your own modules couples tests to implementation. Mock boundaries (HTTP, third-party).

### set_mox_from_context Missing In Async Tests

Expects aren't scoped per-test in async. Call `Mox.set_mox_from_context` in setup.

### Property Tests Replacing Examples

Properties find edge cases but examples document intent. Use both.

## Self-Check

- [ ] Test modules use `async: true` only when truly stateless; modules touching global state (Application env, named processes, shared resources) use `async: false`.
- [ ] Mocks use Mox with a defined behavior, `Mox.defmock`, `set_mox_from_context`, `verify_on_exit!`, and `allow` for spawned processes; mocks target boundaries, not internal modules.
- [ ] Outbound HTTP is tested with `Bypass` (no real network), with the app pointed at the Bypass URL.
- [ ] Ecto sandbox ownership is managed: the test process owns the transaction, and `allow` passes ownership to spawned/background processes accessing the DB.
- [ ] GenServers/processes are tested through their public API with `start_supervised` for lifecycle and `assert_receive`/`refute_receive` (not `Process.sleep`) for timing.
- [ ] StreamData property tests cover pure functions with clear invariants and custom domain generators, alongside example tests.
- [ ] Tests are isolated (per-test `setup`, no shared mutable fixtures in `setup_all`) and deterministic (no `Process.sleep`).
- [ ] The suite has been considered under async races, Mox ownership, Ecto sandbox ownership, process timing, and global-state leakage, and remains stable.
