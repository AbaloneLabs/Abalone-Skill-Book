---
name: coverage_and_meaningful_tests.md
description: Use when the agent is interpreting code coverage numbers, deciding what behavior to test, distinguishing behavior-level testing from implementation-level testing, choosing edge cases and boundary conditions to cover, reviewing whether a test actually verifies anything or merely executes code, setting coverage thresholds, or evaluating whether a high-coverage suite still has real gaps. Also covers line versus branch versus mutation coverage, testing invariants versus testing examples, the difference between executing a path and asserting an outcome, and avoiding shallow tests that inflate metrics without adding safety.
---

# Coverage And Meaningful Tests

Coverage is a map of where tests have executed, not a measure of whether the system is correct. A line covered by a test is a line that ran; it is not necessarily a line whose output was checked, whose edge cases were exercised, or whose contract was verified. The judgment problem is not "how do I raise coverage" but "which behaviors must be verified, and does each test actually verify a behavior or merely pass through one." A suite can have high coverage and low confidence, because coverage rewards execution while safety comes from assertions.

Agents tend to conflate the two because coverage is a number and assertions are not. A number is easy to optimize, easy to report, and easy to mistake for quality. The harm is subtle: a team chases a coverage target, writes tests that call the code without asserting meaningful outcomes, hits the target, and then ships a bug that the suite never caught because no test checked the right thing. This skill exists to keep coverage in its place — as a tool for finding untested code — and to keep the real question, "what should this test verify," at the center.

## Core Rules

### Treat Coverage As A Map, Not A Verdict

Coverage tells you which lines, branches, and functions were reached by at least one test. It does not tell you whether the test asserted anything meaningful about them. Use coverage the way you would use a heat map: to find the dark spots — the error handlers, the fallback branches, the conditions that never executed — and then ask what should be verified there. A covered line is the start of a question, not the end of one.

Specifically:

- **Line coverage** shows which statements ran. It misses branches taken by not running (the implicit `else`), short-circuit evaluation, and dead code.
- **Branch coverage** shows which decision outcomes were taken. It is more honest than line coverage but still says nothing about whether the outcome was correct.
- **Mutation coverage** (mutating the code and seeing if a test fails) is the strongest signal: it tells you whether a test would catch a change to a line. A line with high line coverage but low mutation coverage is a line that runs but is not actually checked.

Prefer the strongest coverage signal you can afford, and never report a coverage number without acknowledging what kind it is.

### Verify Behavior, Not Implementation

A meaningful test asserts that the unit produces the correct observable outcome for a given input and state. It does not assert which internal path was taken, which helper was called, or which data structure was used. The distinction matters because behavior is stable across refactors and implementation is not.

For each test, state the behavior in a sentence that does not mention the code: "when the cart total exceeds the free-shipping threshold, shipping cost is zero." If you can only describe the test by describing the code ("it calls sort then filter then map"), the test is coupled to implementation and will resist change. Rewrite it to assert the outcome and let the implementation vary.

This does not mean tests should be vague. A behavior-level test is specific about inputs, preconditions, and expected outputs; it is just silent about how the unit achieves them.

### Identify The Behaviors That Must Be Verified Before Writing Tests

Before writing a test, enumerate the behaviors the unit is responsible for. Behaviors are not "the functions it contains"; they are the obligations the unit has to its callers. A function may embody several behaviors (happy path, each error case, each boundary), and each deserves its own assertion.

For each unit, ask:

- What is the normal case, and what is the correct outcome?
- For each input that is invalid, missing, empty, null, negative, oversized, or of the wrong type, what is the correct response?
- For each external failure (timeout, unavailable dependency, permission denied), what should the unit do?
- What invariants must hold regardless of input — things that should always be true before, during, and after?

A unit is well-tested when each of these behaviors has an assertion, not when its lines are covered. A function with one happy-path test and 100% line coverage is not well-tested; it is merely reached.

### Test Boundaries And Edge Cases Deliberately

Most bugs live at boundaries, not in the middle of the input space. The middle is where developers think; the boundaries are where assumptions break. Test the boundaries explicitly:

- **Numeric boundaries** — zero, one, negative, the maximum, the minimum, just below and just above a threshold, overflow and underflow.
- **Collection boundaries** — empty, single element, exactly at capacity, one beyond capacity.
- **String boundaries** — empty, one character, very long, unicode, whitespace-only, with delimiters that could be interpreted as structure.
- **Time boundaries** — midnight, leap seconds, time zones, daylight saving transitions, dates that do not exist, timestamps out of range.
- **Concurrency boundaries** — simultaneous access, interleaved operations, operations that complete in different orders.
- **Null and absence** — null, optional-absent, undefined, the difference between "not set" and "set to empty."

A test suite that covers the happy path and one typical case has done the easy 20% of the work. The value is in the boundaries, because that is where the code is least likely to have been thought through.

### Distinguish What A Test Verifies From What It Merely Exercises

A test that calls a function, passes inputs, and checks that no exception was thrown has exercised the code but verified almost nothing. The function could return the wrong value, mutate the wrong state, or silently drop work, and the test would still pass. The same is true of a test whose assertion is trivially true ("the result is not null") or that asserts only a precondition rather than an outcome.

For each test, identify the single assertion that would fail if the behavior broke. If you cannot find one, the test is exercise, not verification. Either add a meaningful assertion or delete the test — a test that verifies nothing still costs runtime and maintenance.

### Separate Invariant Tests From Example Tests

Example-based tests verify the outcome for a specific input ("for input 5, the result is 25"). They are precise but narrow; they prove the behavior at one point. Invariant-based tests verify a property that must hold for a class of inputs ("for any non-negative input, the result is the square"). They are broader and catch edge cases the author did not think to write as examples, but they require identifying a real invariant.

Strong suites use both. Examples pin down specific, important cases (including boundaries). Invariants cover the space between examples and catch inputs no one thought to test. If a unit has a genuine invariant (a sorted output is always sorted, a round-trip serialization always equals the original, a total is always the sum of its parts), express it as a property test in addition to the examples.

### Accept That Some Code Should Not Be Tested Through Assertions

Not every line needs a behavior test. Trivial code — a getter, a data class constructor, a one-line delegation — is exercised by the tests of the code that uses it, and a dedicated test would only assert tautologies. Forcing coverage on such code inflates the suite without adding safety. Let coverage be low on trivial code and high on logic, branching, and error handling, where bugs actually live.

The error is the opposite assumption: that 100% coverage is always the goal. It is not. The goal is verified behavior where behavior is non-trivial.

### Make Failure Messages Point At The Behavior, Not The Code

When a meaningful test fails, the failure message should describe the broken behavior, not the broken assertion. "shipping cost for a $60 cart should be $0 but was $5" tells the reader what is wrong; "expected 0 but got 5" tells them only that a number disagreed. Write assertions and messages so that a failure reads as a statement about the world the test models, because that is what the reader will act on.

## Common Traps

### Chasing A Coverage Number

When the goal becomes "hit 80% coverage," the incentive is to write tests that execute code cheaply rather than tests that verify behavior. The number rises; the safety does not. Coverage targets are useful only as floors that force attention to untested areas, never as ceilings or as the definition of done.

### The Assertion-Free Test

A test that calls the code, catches or avoids exceptions, and asserts nothing (or asserts only that the result is non-null) inflates coverage while verifying nothing. It is worse than no test, because it gives the appearance of safety. Every test should have an assertion that would fail if the behavior broke.

### Testing Only The Happy Path

A suite where every test passes valid input and checks the valid output covers the code the developer was thinking about when writing it. It misses every error path, every boundary, and every invalid input — precisely where most production bugs appear. Coverage looks fine because the happy-path lines ran; the untested branches are invisible until production hits them.

### Coupling Tests To Implementation

Tests that assert on internal calls, private state, or the specific sequence of operations break on every refactor that preserves behavior. They make the codebase expensive to change and train developers to avoid refactors. Assert on observable outcomes and let the implementation vary.

### Mistaking High Coverage For High Safety

Two suites can have identical coverage and very different safety. One asserts meaningful outcomes at every boundary; the other calls the code and checks for null. The coverage number cannot tell them apart. Never report coverage as evidence of safety; report it as a map of where attention has and has not been spent.

### Ignoring Mutation Coverage When It Is Affordable

Line and branch coverage are cheap to collect and weak as signals. Mutation coverage is more expensive but actually answers "would a test catch a change here." When mutation coverage is affordable, it reveals the gap between executed lines and verified lines, and it is often much larger than the line-coverage number suggests.

### Forcing Tests On Trivial Code

Adding tests to getters, data classes, and one-line delegations to satisfy a coverage target produces tautological tests ("getName returns the name that was set"). These tests cost runtime and maintenance and catch no bugs. Let trivial code be covered indirectly by the tests of its consumers.

### Treating Edge Cases As Rare and assuming The Test That Covers A Branch Also Checks It

Edge cases feel rare because each one is unlikely in isolation. In aggregate, across many users and much time, they are not rare — they are the source of most production incidents. Treating them as not worth testing is treating production incidents as not worth preventing.

A branch can be taken by a test that never asserts on the outcome of taking it. The branch is "covered" but its behavior is unchecked. For each non-trivial branch, confirm a test asserts what should happen when that branch is taken, not merely that a test caused it to be taken.

## Self-Check

- [ ] Coverage is reported with its kind (line, branch, mutation) and used as a map of untested areas, not as a verdict on safety or as the definition of done.
- [ ] Each test can be described as a behavior in a sentence that does not mention the code's internal implementation, and asserts an observable outcome rather than internal calls or state.
- [ ] For each unit, the behaviors that must be verified (happy path, each error case, each boundary, each invariant) were enumerated, and each has at least one assertion.
- [ ] Boundary and edge cases are tested explicitly: numeric limits, empty and single-element collections, empty and oversized strings, time and timezone edges, concurrency interleavings, and null/absence distinctions.
- [ ] Every test has an assertion that would fail if the behavior broke; no test merely exercises the code or asserts only a precondition or non-null result.
- [ ] Units with genuine invariants have property-style tests in addition to example tests, so the space between examples is covered.
- [ ] Trivial code (getters, data classes, one-line delegations) is not force-tested for coverage; coverage is concentrated where logic, branching, and error handling live.
- [ ] Mutation coverage was checked where affordable, and the gap between executed lines and verified lines was investigated rather than assumed away.
- [ ] Failure messages describe the broken behavior in terms of the modeled world, not just the disagreed values.
- [ ] No branch is considered tested merely because a test caused it to execute; each non-trivial branch has a test that asserts the outcome of taking it.
