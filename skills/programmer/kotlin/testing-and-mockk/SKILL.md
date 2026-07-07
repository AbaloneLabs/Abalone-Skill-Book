---
name: kotlin_testing_and_mockk.md
description: Use when the agent is writing Kotlin tests with MockK, JUnit 5, Kotest, or MockK's relaxed mocks, everyFee/reified mocking, mocking top-level and final functions and objects, coEvery and suspend function mocking, verifying call counts and order, mockk vs spy, fakes vs mocks, coroutine test dispatchers (runTest, TestDispatcher), or is diagnosing "mock not relaxed enough", "UnfinishedMockingSession", "cannot mock final class", suspend function verification issues, tests that pass but mock everything, or flaky coroutine tests from real dispatchers. Covers Kotlin-specific mocking (finals, objects, top-level, suspend), coroutine testing, mock-vs-fake-vs-spy decisions, and the pitfalls of over-mocking and non-deterministic coroutine tests.
---

# Testing And MockK In Kotlin

Kotlin testing has a set of pitfalls distinct from Java testing, because Kotlin's language features interact with mocking and test frameworks in ways that quietly break. Kotlin classes are final by default, which defeated classic Java mocking tools (Mockito could not mock finals without config) and made MockK the de-facto Kotlin choice (it mocks finals, objects, top-level functions, and extension functions via bytecode instrumentation). Kotlin's coroutines need their own test machinery: a test must not use real dispatchers and real delays (slow, non-deterministic), but `runTest` with a `TestDispatcher` that controls virtual time. And Kotlin's `suspend` functions need `coEvery`/`coVerify`/`coAnswers` rather than the synchronous mock APIs. Layered on these is the universal testing judgment: mock at boundaries, not internal collaborators; prefer fakes over mocks where a fake is more honest; verify behavior, not implementation. The judgment problem is to use MockK's Kotlin-specific capabilities correctly, to test coroutines deterministically with virtual time, and to keep the test honest (exercising real behavior) rather than theatrical (mocking everything so it passes).

Agents reach for `mockk<T>()` on every collaborator, mock `suspend` functions with `every` (which does not compile or behaves wrong), run coroutines on real dispatchers with real `delay` (slow and flaky), and verify call counts that couple the test to the implementation. The remedy is to mock at boundaries with the coroutine-aware APIs, to drive coroutines under `runTest` with a `TestDispatcher`, to prefer fakes for complex collaborators, and to assert on outcomes.

## Core Rules

### Mock At Boundaries, Prefer Fakes For Complex Collaborators

The universal rule applies in Kotlin too: mock (or stub) at boundaries — external services, the system clock, the database, the network — where the real dependency is slow, non-deterministic, or destructive. Within the domain, exercise real collaborators so the test proves the integration. For a collaborator with non-trivial behavior that you would otherwise mock heavily, a hand-written fake (an in-memory implementation of the interface) is often more honest and more durable than a mock with many stubbed calls, because it implements the contract rather than asserting on call sequences. Use `mockk` for simple boundary stubs, fakes for complex stateful collaborators, and `spyk` (partial mock) rarely and deliberately (it couples the test to the implementation).

- Mock boundaries (payment gateway, clock, HTTP); exercise real domain collaborators.
- A fake repository (in-memory map) over a mocked repository with many `every` calls for stateful data.
- `spyk` only when you must wrap a real object's individual methods; it is a smell when overused.

### Use MockK's Kotlin-Aware APIs For Finals, Objects, Top-Level, And suspend

MockK mocks Kotlin constructs that classic tools cannot: final classes and methods (`mockk<FinalClass>()`), `object` singletons (`mockkObject(MyObject)`), top-level functions (`mockkStatic("pkg.FileKt")`), extension functions, and `suspend` functions. For `suspend` functions use the coroutine-aware APIs: `coEvery { repo.fetch() } returns ...`, `coVerify { repo.fetch() }`, `coAnswers { ... }`. Using `every` on a `suspend` function does not work correctly. For `object`/top-level mocking, remember it is global state: unmock in `after`/`relax` so later tests are not affected.

- `coEvery`/`coVerify`/`coAnswers` for `suspend` functions, not `every`/`verify`.
- `mockkObject`/`mockkStatic` are global; clean up (`unmockkAll`/`unmockkStatic`) so tests do not leak.
- `relaxed = true` for mocks where you do not want to stub every call (returns defaults); use sparingly to avoid hiding missing stubs.

### Test Coroutines Deterministically With runTest And TestDispatcher

A coroutine test must not use real dispatchers and real `delay`: it is slow (a 1-second delay waits 1 second) and non-deterministic (real scheduling). Use `runTest { ... }` with a `TestDispatcher` (`StandardTestDispatcher` or `UnconfinedTestDispatcher`) which controls virtual time: `delay(1000)` skips instantly, and `advanceUntilIdle()`/`runCurrent()` drive the scheduler. Inject the dispatcher (a `CoroutineDispatcher` parameter) into the code under test so the test supplies the test dispatcher; do not hardcode `Dispatchers.Default`/`Dispatchers.Main`. For `Dispatchers.Main` (Android), use `Dispatchers.setMain(testDispatcher)`/`resetMain()`.

- `runTest { ... }` with an injected `TestDispatcher`; no real `delay` in tests.
- Inject the dispatcher; do not hardcode `Dispatchers.*` in code under test.
- `Dispatchers.setMain(testDispatcher)` for code using `Dispatchers.Main`, reset in `after`.

### Verify Behavior And Outcomes, Not Implementation Call Counts

`verify(exactly = 1) { repo.save(any()) }` couples the test to the implementation: a correct refactor that saves twice (or zero times via caching) fails the test even though the outcome is right. Prefer to assert on the outcome (the returned state, the persisted record via a fake, the published event) and reserve `verify` for boundary interactions where the call itself is the contract (a payment was attempted, an email was sent). Use `verify` sparingly and on boundaries, not on internal collaborators.

- Assert on outcomes (return value, state, persisted data, boundary event).
- `verify` only for boundary calls that are themselves the contract.
- Avoid `verify(exactly = N)` on internal methods; it breaks under correct refactors.

### Handle Mock Lifecycle: relax, Unmock, Avoid Leaked Sessions

MockK uses a mocking session per test; an unfinished session (`UnfinishedMockingSession`) usually means a `mockk`/`every` was called outside a test or after teardown. `mockkObject`/`mockkStatic` modify global state and must be unmocked (`unmockkAll`, `unmockkStatic`, `unmockkObject`) or scoped with `mockkStatic(...) { ... }` so they do not leak to later tests. Relaxed mocks (`mockk<T>(relaxed = true)`) return defaults for un-stubbed calls, convenient but able to hide a missing stub; use relaxed where the default is acceptable and strict where a missing stub should fail.

- Scope global mocks (`mockkStatic { }`) or unmock in teardown.
- Use strict mocks where a missing stub should be an error; relaxed where defaults are fine.
- Keep `mockk`/`every` inside the test body, not in shared init that may run outside a session.

### Structure Tests For Readability: given/when/then, Descriptive Names

Kotlin test readability benefits from structure: descriptive test names (backtick names in JUnit/Kotest: `` `given a paid order when refunding then it marks refunded` ``), clear arrange/act/assert sections, and focused tests (one behavior per test). Kotest frameworks ( Kotest's behavior/string spec) and JUnit5's `@DisplayName` support this. A test whose name describes the behavior and whose body has clear sections is far easier to maintain than a numbered test with an opaque assertion.

## Common Traps

### every On A suspend Function

`every { suspendFn() }` does not work for suspend functions; use `coEvery`. The mock either fails to compile or behaves wrong at runtime.

### Real Dispatchers And Real delay In Tests

`runBlocking { delay(1000); ... }` waits a real second and is flaky. Use `runTest` with a `TestDispatcher` and virtual time.

### Dispatchers.Main Not Set In Test

Code using `Dispatchers.Main` throws in a test unless `Dispatchers.setMain(...)` is called. Set and reset it.

### mockkObject/mockkStatic Leaking Across Tests

A global mock not unmocked affects later tests, producing order-dependent failures. Scope or unmock.

### UnfinishedMockingSession

A `mockk`/`every` called outside a test's session (e.g., in a lazy init). Keep mocking inside the test body.

### Over-Mocking Internal Collaborators

Mocking every internal service and verifying call counts couples the test to the implementation; refactor breaks it. Exercise real collaborators; mock boundaries.

### relaxed Mock Hiding A Missing Stub

A relaxed mock returns a default for an un-stubbed call, masking a setup gap. Use strict where a missing stub is a bug.

### verify(exactly = N) On An Internal Method

Asserting an internal method was called N times breaks under a correct refactor. Assert on outcomes.

## Self-Check

- [ ] Mocks target boundaries (external services, clock, network, DB), real domain collaborators are exercised, and complex stateful collaborators use fakes rather than heavily-stubbed mocks.
- [ ] `suspend` functions use `coEvery`/`coVerify`/`coAnswers`; final classes, objects, top-level, and extension functions are mocked with the correct MockK APIs and cleaned up (`unmockk*`/scoped) to avoid leakage.
- [ ] Coroutines are tested under `runTest` with an injected `TestDispatcher` (virtual time, no real `delay`), dispatchers are injected not hardcoded, and `Dispatchers.Main` is set/reset via `Dispatchers.setMain`.
- [ ] Tests assert on outcomes (return values, state, persisted records, boundary events); `verify` is reserved for boundary calls that are the contract, not internal call counts.
- [ ] Mock lifecycle is managed: no `UnfinishedMockingSession`, global mocks are scoped or unmocked, and strict/relaxed is chosen deliberately per mock.
- [ ] No real dispatcher or real delay makes tests slow or flaky; virtual time controls scheduling.
- [ ] No over-mocked internal collaborator or `verify(exactly=N)` couples the test to implementation; a correct refactor would not break the test.
- [ ] The test suite has been considered under coroutine timing, mock leakage, random order, and refactor, and remains deterministic, honest, and maintainable.
