---
name: legacy_code_comprehension_and_testing.md
description: Use when the agent is reading or modifying code that lacks tests or clear structure, trying to understand what unfamiliar legacy code does, adding the first tests to untested code, breaking hard-wired dependencies to make code testable, using wrapper or seam patterns to characterize behavior, or verifying that a change to legacy code preserves its existing behavior.
---

# Legacy Code Comprehension And Testing

Legacy code — code without tests, often without clear structure, and usually carrying years of accumulated assumptions that no one fully remembers — is where careful engineering practices matter most and are hardest to apply. The defining problem is that you cannot safely change what you cannot verify, and you cannot verify what you cannot test, and you cannot test what has dependencies hard-wired into it. So the legacy code that most needs improvement is the code that resists improvement at every step: it has no tests to catch a regression, no seams to inject a test, and no documentation to tell you what behavior is intentional versus accidental. Changing it blind is how regressions are introduced and how the codebase earns its reputation as untouchable.

Agents tend to approach legacy code the way they approach greenfield code: read enough to find the spot, make the change, and move on. In tested, well-structured code this is reasonable; in legacy code it is reckless, because there is no safety net to catch the subtle behavior change and no clear boundary to say what the change should and should not affect. The judgment problem is to invest in comprehension and testability *before* changing — to read the code strategically rather than linearly, to capture its current behavior (including its quirks) in characterization tests, to break the dependencies that block testing through seams and wrappers, and to verify behavior preservation at every step, so that the change is made with evidence rather than hope. This skill is about getting legacy code to a state where it can be changed safely; the sequencing of the changes themselves is covered by the safe-refactoring skill.

## Core Rules

### Understand The Code's Behavior Before Changing It

The first failure mode in legacy code is changing something you did not understand. Legacy code accumulates behavior that exists for reasons — a workaround for a bug in another system, a constraint imposed by old data, a quirk that some caller depends on — and these reasons are invisible in the code. A change that "improves" such behavior breaks the thing it was working around. Before changing, build a model of what the code does and, as far as you can recover, why.

Strategic reading practices:

- **Read for behavior, not implementation.** Trace what the code does for representative inputs and edge cases — what it returns, what it writes, what it calls — before worrying about how it is structured. The behavior is what you must preserve; the structure is what you may change.
- **Identify the inputs and outputs at the boundary** of the code you intend to change. What goes in, what comes out, what side effects occur? These boundaries are where you will later pin behavior with tests.
- **Look for the implicit contracts** — the assumptions callers make about timing, ordering, error handling, nullability, and side effects. Legacy code is full of contracts that are enforced only by the fact that callers happen to rely on them.
- **Find the historical context where it exists** — comments, commit messages, linked issues, the people who wrote it. Even partial context distinguishes intentional behavior from accident.

Do not try to understand everything before acting; legacy codebases are too large. Understand the part you will change and its immediate dependencies deeply, and the rest well enough to know you are not affecting it.

### Characterize The Current Behavior Before You Change It

The foundation of safe legacy change is a test that captures the code's *current* behavior, so that a change which alters that behavior is detected. These are characterization tests (or golden-master tests): they do not assert what the behavior *should* be, only what it *is*, including its quirks and bugs. Their purpose is to make behavior change visible, not to judge correctness.

How to characterize effectively:

- **Cover the paths you will touch**, with representative and edge-case inputs. You do not need to characterize the whole codebase, only the behavior your change could affect.
- **Pin observable outputs** — return values, written state, calls made to dependencies — for each input. Where output is complex or non-deterministic, capture a snapshot (a "golden master") and compare future runs to it.
- **Include the quirks.** If the code has a bug you intend to keep (because callers depend on it) or a bug you intend to fix, characterize both — the current behavior first, so the fix is a deliberate, visible test change rather than an accidental one.
- **Characterize before refactoring, not after.** The tests must capture pre-change behavior so they can prove the refactor preserved it; writing them after assumes the behavior was already understood.

Characterization tests are the safety net that legacy code lacks. Building them is not overhead; it is the precondition for changing the code at all without flying blind.

### Break Dependencies Through Seams To Make The Code Testable

The reason legacy code is hard to test is usually that its dependencies are hard-wired: it constructs its collaborators directly, reads global state, talks to the database or network in line with the logic, and offers no way to substitute these for a test. To characterize the behavior, you must first create a place where a test can observe or substitute — a seam. Creating seams is itself a behavior-preserving change, and it must be done carefully.

Common seam techniques:

- **Extract and override** — extract the dependency access (a direct construction, a global read, a network call) into a method, then override that method in a test subclass to substitute a test double. The production behavior is unchanged; the test gains an injection point.
- **Parameterize** — add a parameter for the dependency where it was previously constructed internally, defaulting to the production value so existing callers are unaffected.
- **Wrap with a facade** — put a thin wrapper around the legacy unit that the rest of the system calls, so the wrapper can be tested or substituted even when the underlying unit cannot.
- **Introduce an interface at the boundary** — where the code calls an external system, define an interface and have the code depend on it, with the production implementation and a test implementation both satisfying it.

Each of these is a small, behavior-preserving transformation whose only purpose is to create testability. They carry their own risk (a botched seam can change behavior), so apply them one at a time with the characterization tests running after each, and keep them minimal — the goal is a seam, not a redesign.

### Add Tests Without Changing Behavior, Then Change With Tests Guarding

The discipline is strict: the work of making the code testable (seams, wrappers, characterization tests) must not change the code's behavior, and the work of changing the behavior (the actual fix or feature) must happen only after the tests are in place guarding it. Mixing the two — changing behavior while also adding the tests that should be catching behavior changes — makes it impossible to tell whether a test failure is a regression or an intended new behavior.

Sequence the work:

1. **Comprehend** the behavior you will touch.
2. **Create seams** where needed, behavior-preserving, one at a time.
3. **Characterize** the current behavior with tests, including quirks.
4. **Change** the behavior, with the characterization tests now guarding preservation of the behavior you did not intend to change; update the tests deliberately for the behavior you did intend to change.

This sequence converts legacy work from blind modification into evidence-backed change. The investment in comprehension and characterization is the price of safely changing code that has no other safety net, and it is always less expensive than the regression it prevents.

### Preserve Behavior Unless A Change Is Explicitly Intended

Legacy code's behavior is the system's current contract, including the parts that look wrong. A "fix" that changes behavior callers depend on is a regression, however correct the new behavior seems. Distinguish rigorously between behavior you are preserving (which must not change, and which characterization tests guard) and behavior you are intentionally changing (which is a separate, deliberate step with its own tests asserting the new behavior).

When you encounter a quirk or apparent bug during legacy work:

- **Characterize it as-is first**, so its current behavior is captured.
- **Decide explicitly whether to preserve or fix it.** If callers may depend on it, preserve it; if it is genuinely a bug to fix, fix it in a separate, deliberate change with its own test, not bundled into refactoring or testability work.
- **Never silently "clean up" behavior while making the code testable.** A cleanup bundled into seam creation is a behavior change disguised as a structural change, and it undermines the characterization tests' ability to detect regressions.

### Verify Preservation Through The Tests, Not Through Reading

Once characterization tests are in place, they are the verification that behavior was preserved — but only if they actually cover the behavior that changed. A green test suite is meaningless if the tests do not exercise the paths the change touched. Before relying on a green run as proof of preservation, confirm the tests cover the changed behavior; if they do not, add characterization tests for the uncovered paths before trusting the result.

This guards against the most seductive legacy failure: a change that passes all existing tests because the tests did not cover the affected behavior, and that ships a regression that the absent tests could not catch. Coverage of the changed paths, not overall coverage or a green bar, is what makes the characterization meaningful.

## Common Traps

### Changing Before Understanding

Modifying legacy code based on a partial read, without recovering the behavior and its reasons, breaks the workarounds and contracts the code carries. Understand the behavior and its context before changing, especially the implicit contracts callers depend on.

### Skipping Characterization Tests

Changing legacy code without first capturing its current behavior in tests is modification by faith; any behavior change, intended or not, goes undetected. Characterize the paths you will touch before changing them, quirks included.

### Hard-Wired Dependencies Blocking Tests

Legacy code that constructs its dependencies directly or reads global state in line with its logic cannot be tested without a seam. Create seams (extract-and-override, parameterize, facade, interface) as behavior-preserving steps before the tests can be written.

### Mixing Behavior Change With Testability Work

Changing behavior while also adding the tests that should catch behavior changes makes test failures ambiguous — is it a regression or the intended new behavior? Sequence strictly: testability and characterization first (behavior-preserving), then behavior change (with tests guarding).

### Silently "Fixing" Quirks During Refactor

A behavior change bundled into structural or testability work is a regression disguised as cleanup, and it undermines the characterization tests. Preserve behavior during structural work; fix bugs in separate, deliberate, test-backed changes.

### Trusting A Green Suite That Does Not Cover The Change

A passing test run proves nothing if the tests do not exercise the changed behavior. Confirm the characterization tests cover the paths touched before relying on them as proof of preservation.

### Trying To Understand Everything Before Acting

Legacy codebases are too large to fully comprehend before any change. Understand deeply the part you will change and its immediate dependencies, and the rest well enough to know your change does not affect it; do not let comprehension become paralysis.

### Redesigning Instead Of Creating A Minimal Seam

Turning a dependency-breaking step into a redesign introduces risk and defeats the purpose of a behavior-preserving transition. Keep seams minimal — the goal is testability, not a new architecture.

## Self-Check

- [ ] The behavior of the code to be changed was understood before changing — inputs, outputs, side effects, and implicit contracts (timing, ordering, error handling, nullability) that callers depend on — not just the implementation structure.
- [ ] Characterization tests capture the current behavior of the paths that will be touched, including quirks and apparent bugs, pinning observable outputs for representative and edge-case inputs, written before any change.
- [ ] Dependencies that blocked testing were broken through minimal, behavior-preserving seams (extract-and-override, parameterization, facade, interface), applied one at a time with tests run after each.
- [ ] Testability and characterization work was strictly sequenced before behavior change, so no behavior change is bundled into the work that should be guarding behavior change.
- [ ] Behavior is preserved unless a change is explicitly intended; quirks were characterized as-is and either deliberately preserved or fixed in a separate test-backed change, never silently "cleaned up."
- [ ] The characterization tests relied upon as the preservation check actually cover the behavior changed, not merely unrelated code that happens to be green.
- [ ] Comprehension was focused on the change and its dependencies rather than attempting to understand the entire codebase before acting.
- [ ] Seams were kept minimal and behavior-preserving, not expanded into redesigns that introduce their own risk.
