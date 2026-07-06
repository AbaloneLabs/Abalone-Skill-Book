---
name: mocking_and_test_doubles.md
description: Use when the agent is deciding whether to replace a dependency with a test double in a unit or integration test, choosing between a stub, mock, spy, fake, or dummy, evaluating whether a test is over-mocked or too coupled to implementation, dealing with a test that breaks on every refactor, designing a dependency so it is testable without mocking, or reviewing a test suite for fragile or brittle tests. Also covers London versus classical (Detroit) test style, mocking frameworks versus hand-rolled fakes, the costs of mocking third-party libraries, and the boundary between real collaborators and doubles.
---

# Mocking And Test Doubles

Replacing a dependency with a test double is a decision about what the test is allowed to ignore. Every double hides a real collaborator behind a stand-in, and that stand-in carries an assumption: "the real thing behaves the way I have programmed the double to behave." When the assumption holds, the test is fast and focused. When it drifts — because the real collaborator changed semantics, error format, ordering, or timing — the double keeps the test green while production breaks. The judgment problem is not "should I mock this" but "what am I willing to stop verifying, and is that risk acceptable."

Agents tend to over-mock because mocking makes tests fast and because most testing frameworks make mocking the path of least resistance. The harm is delayed and invisible: a suite of over-mocked tests passes for years while the real integrations rot, and refactors that should be safe break dozens of tests not because behavior changed but because the calls the mocks asserted changed. The reverse failure is also real — refusing to mock anything produces slow, flaky tests that depend on real databases and networks and that cannot run in isolation. This skill exists to make each doubling decision deliberate and to choose the right kind of double for the right kind of dependency.

## Core Rules

### Decide Per Dependency: Real Or Double, And Why

For each collaborator a test touches, decide explicitly whether to use the real thing or a double, and name the reason. The legitimate reasons to double a dependency are narrow:

- **It is non-deterministic or slow** — a real clock, a random source, a network call, a service with rate limits. Doubling makes the test deterministic and fast.
- **It has side effects you cannot afford in a test** — charging a real card, sending a real email, mutating shared production-like state.
- **It is expensive to set up in a particular state** — forcing a rare error from a real third party is hard; forcing it from a double is trivial.
- **It does not exist yet or is owned by another team** — you are testing against a contract that the real implementation will fulfill later.

If none of these apply, prefer the real collaborator. A real in-memory database, a real value object, a real pure function, and a real local service are all better collaborators than doubles, because they verify the actual behavior instead of an assumption about it. Defaulting to "mock everything" is a smell; defaulting to "real everything" is also a smell. The decision is per dependency, not per suite.

### Choose The Right Kind Of Double For The Question You Are Asking

The vocabulary of test doubles exists because each kind answers a different question. Using the wrong one produces tests that verify the wrong thing.

- **Dummy** — passed but never used; fills a parameter list. Use when a collaborator is required by signature but irrelevant to the test.
- **Stub** — returns canned answers to queries. Use when the test needs the dependency to provide specific data so the unit under test can be exercised. Stubs do not verify interaction; they only feed inputs.
- **Spy** — records calls made to it so the test can assert on them afterward. Use when the meaningful outcome is that a side effect occurred (an email was sent, a metric was emitted) and the real side effect is too costly to perform.
- **Mock** — pre-programmed with expectations about the calls it should receive and fails if they are not met. Use sparingly; mocks couple the test to the exact sequence and arguments of calls, which is implementation, not behavior.
- **Fake** — a working but simplified implementation (an in-memory database, an in-memory queue). Use when you want real behavior semantics without the real infrastructure cost. Fakes are the most powerful double because they preserve behavior while removing cost.

The progression from stub to mock is a progression from verifying outcomes to verifying interactions. Outcomes are stable across refactors; interactions are not. Bias toward stubs and fakes; reserve mocks for the specific case where the interaction itself is the contract.

### Verify Behavior, Not Call Sequences

A test that asserts "the code called `repository.save` exactly once with these arguments" is verifying how the unit accomplished its job, not whether it accomplished its job. Such tests break on every internal refactor even when the observable outcome is unchanged. They turn the test suite into a change detector rather than a correctness detector.

Prefer to assert on observable outcomes — the return value, the state change, the event published, the record that now exists. When you must assert on interaction (the side effect is genuinely the point, e.g., "an audit log entry was written"), assert that the interaction happened, not the exact call sequence or count, unless the count is itself a requirement.

Ask of every mock expectation: "If I refactored the implementation but kept the behavior correct, would this test still pass?" If the answer is no, the test is coupled to implementation and will resist future change.

### Prefer Fakes For Stateful Collaborators

When a collaborator is stateful — a repository, a queue, a cache, a session store — a fake is usually a better choice than a set of stubs and mocks. A fake implements the same interface with simpler in-memory semantics, so the test exercises real behavior (store, retrieve, update, delete) without the cost of the real infrastructure. Multiple tests share one fake, which evolves with the real interface, and the test does not have to reprogram canned responses for every scenario.

A fake has a maintenance cost: it must stay behaviorally consistent with the real implementation. That cost is paid once and shared; the cost of per-test mocking is paid in every test and compounds. For a collaborator used by many tests, the fake wins.

Reserve fakes for collaborators whose behavior is simple enough to replicate faithfully. If the real collaborator has subtle concurrency, ordering, or consistency semantics, a fake that ignores them gives false confidence; in that case a real integration test is more honest than a fake.

### Mock Your Own Abstractions, Not Third-Party Types

A test should double interfaces you own, not concrete types owned by external libraries. Mocking a library's concrete class couples your test to that library's internal API, which changes without notice and breaks the test in ways unrelated to your behavior. The durable pattern is to define a narrow interface in your own code that represents what you need from the outside world, implement it with the real library in production, and double your interface in tests.

This is not ceremony; it is how you keep the seam between your code and the world under your control. When the third-party library changes, you update one adapter, not a hundred tests. When you replace the library entirely, your tests do not change at all.

### Make Dependencies Explicit So They Can Be Doubled Selectively

A dependency that can only be reached through a global singleton, a static method, or deep object construction cannot be selectively doubled — it is either fully real or fully impossible to test against. The testability of a unit is determined before the test is written, by how the unit accepts its collaborators. Prefer constructor or parameter injection of interfaces, so that a test can pass a real collaborator, a fake, or a stub as appropriate.

When you find yourself reaching for heavy mocking frameworks to intercept static calls or to construct an object with twenty dependencies just to test one, the signal is that the unit's dependency surface is too wide. The fix is to narrow the surface (depend only on what you use, behind an interface), not to add more mocking machinery.

### Match The Double's Realism To The Risk

A double can be unrealistic in two directions: too simple (it hides behavior the test should verify) or too realistic (it reimplements the real thing and inherits its cost and flakiness). Match the realism to what is at risk.

- If the risk is logic in the unit under test, a simple stub is enough; the collaborator's behavior is not the question.
- If the risk is the contract between the unit and the collaborator, a fake or a real collaborator is needed; a stub asserts nothing about the contract.
- If the risk is the collaborator's own behavior under edge conditions, no double will do; you need a real integration test against the real thing.

Doubling a dependency whose contract is the thing you are worried about gives a test that proves nothing about the worry. Always ask: "Is the thing I am worried about the thing I just replaced with a double?"

### Choose A Mocking Style Deliberately: London Versus Classical

Two durable schools of thought shape how much a suite mocks. The London (mockist) style isolates the unit under test by doubling every collaborator and verifies that the unit issues the correct interactions. The classical (Detroit) style keeps real collaborators wherever they are fast and deterministic, using doubles only for what is genuinely costly or non-deterministic, and verifies outcomes rather than calls.

Neither is universally correct; each optimizes for a different thing. The London style makes the unit's design explicit (every collaborator is an interaction point) and makes failures localize to one unit, but it couples tests to call structure and can drift from real behavior. The classical style keeps tests honest about real collaboration and resilient to internal refactor, but it can produce larger, slower tests that are harder to localize when they fail.

Decide per project, and ideally per test, which pressure matters more:

- **Bias London** when the unit's interactions are themselves the design (a controller orchestrating services, a workflow emitting commands) and when fast, localized failure is worth the coupling cost.
- **Bias classical** when collaborators are cheap and deterministic (value objects, in-memory repositories, pure helpers) and when the real collaboration is part of what should be verified.

A common, durable compromise is classical by default and London at the seams where interaction is the contract. Mixing the two is fine; the error is applying one style dogmatically to a unit where the other fits.

### Prefer Hand-Rolled Fakes Over Heavy Frameworks Where Behavior Matters and keep Doubles Close To The Real Interface

Mocking frameworks make it cheap to program a double's return values and expectations inline, and that cheapness encourages over-use. For a one-off stubbed query, a framework is appropriate. For a collaborator used across many tests, a hand-rolled fake — a small class implementing the real interface with in-memory semantics — is usually better: it is shared, it evolves with the interface, it expresses real behavior rather than per-test expectations, and it does not require every test to re-state the collaborator's contract.

Reach for the framework when the double is trivial and local; reach for a fake when the collaborator is stateful, shared, or behaviorally meaningful. The trap is reaching for the framework by reflex and ending up with a suite where every test re-programs the same collaborator's behavior from scratch.

A double that drifts from the real interface is worse than no double, because it makes tests pass against a fictional collaborator. Keep doubles honest by deriving them from the same interface the real implementation satisfies, and by having at least one integration test that exercises the real implementation so the contract is verified somewhere. A suite where every collaborator is doubled and the real integrations are never exercised can be entirely green while the system is broken.

## Common Traps

### Mocking Everything By Default

When the testing framework makes mocking one line, the easy path is to mock every collaborator in every test. The result is a suite that is fast, green, and proves almost nothing about the real system. Each mock encodes an assumption about a collaborator, and no test verifies any of those assumptions. The suite becomes a mirror of the code's current call patterns, not a check on its behavior.

### Over-Specified Mock Expectations

Asserting exact argument values, exact call counts, and exact call order produces tests that break on any internal change, even refactors that preserve behavior. These tests do not catch bugs; they catch change. They make refactoring expensive and train developers to avoid the refactors that would improve the code.

### The Fake That Drifted From The Real Thing

A hand-rolled in-memory repository that was accurate when written but never updated as the real repository gained transactions, constraints, or ordering guarantees gives tests false confidence. The fake says "this works"; the real database says "this deadlocks." Fakes need an owner and a check — usually a shared contract test run against both the fake and the real implementation.

### Mocking Value Objects And Pure Functions

Doubling a pure function or a value object is almost always wrong. These collaborators are deterministic, fast, and free of side effects — there is no cost to using the real one, and using the real one verifies real behavior. Mocking them adds coupling and removes verification for no benefit.

### Reaching Into Private State To Set Up A Mock

If a test has to reflect into private fields, weaken access modifiers, or expose test-only constructors to inject a double, the unit's dependency surface is wrong. The fix is to make the dependency explicit in the public interface, not to break encapsulation from the test. Test-only access widening is boundary erosion that spreads.

### Mocking Third-Party Concrete Classes

Mocking a library's concrete class binds the test to that library's private API. When the library upgrades, the test breaks for reasons unrelated to your code, and the failure messages point at the library, not at your behavior. Always mock an interface you own that wraps the library.

### The Mock That Hides The Bug You Are Looking For

When the bug you are chasing lives in the interaction with a real collaborator — a serialization mismatch, a transaction boundary, an ordering assumption — mocking that collaborator away removes the very thing that would have revealed the bug. The test passes; the bug ships. Before doubling, confirm the bug is not in the thing being doubled.

### Assuming A Green Mocked Suite Means The System Works

A suite where every external dependency is mocked verifies only that the code under test matches its own assumptions. It says nothing about whether those assumptions match reality. Without at least one tier that exercises real collaborators, the gap between the suite and production is unknown and usually larger than expected.

## Self-Check

- [ ] Each doubled dependency was chosen deliberately and the reason (non-determinism, cost, side effects, not-yet-existing) is identifiable; real collaborators were preferred where none of these reasons applied.
- [ ] The kind of double (dummy, stub, spy, mock, fake) matches the question the test asks; mocks are reserved for cases where the interaction itself is the contract.
- [ ] Tests assert on observable outcomes (return values, state, published events) rather than exact call sequences, counts, or arguments, unless the interaction is genuinely the requirement.
- [ ] Stateful collaborators used by many tests are backed by a shared fake that stays consistent with the real implementation, not by per-test stubs and mocks.
- [ ] Doubles implement interfaces owned by your code, not concrete classes from third-party libraries; the seam between your code and external libraries is under your control.
- [ ] No test required private-field reflection, access-modifier weakening, or test-only constructors to inject a double; dependencies are explicit in the public interface.
- [ ] Pure functions and value objects are used for real, not doubled.
- [ ] At least one integration tier exercises the real collaborator for each doubled dependency, so the contract the double assumes is verified somewhere.
- [ ] The mocking style (London/classical) was chosen per project or per test based on whether the unit's interactions are the design or whether real collaboration should be verified, not applied dogmatically; [ ] Framework doubles are used for trivial, local stubs; stateful or shared collaborators are backed by hand-rolled fakes rather than re-programmed per test
- [ ] For each mock expectation, a refactor that preserves behavior but changes internal calls would still pass the test; if not, the test is coupled to implementation; [ ] The bug being investigated is not in a collaborator that was doubled away to make the test pass
