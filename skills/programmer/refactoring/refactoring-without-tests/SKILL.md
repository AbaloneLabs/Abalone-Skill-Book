---
name: refactoring_without_tests.md
description: Use when the agent is refactoring code that lacks test coverage, writing characterization tests to capture current behavior, verifying behavior preservation in small reversible steps, using automated refactoring tools, or assessing and mitigating risk when no safety net exists.
---

# Refactoring Without Tests

Refactoring is defined as changing internal structure without changing behavior. Without tests, you cannot prove behavior is preserved, so "refactoring without tests" is technically a contradiction—you are not refactoring, you are modifying code and hoping. Yet most real codebases have untested regions that still must change, and the practical question is not whether to touch them but how to do so safely enough. The discipline is to build a safety net (characterization tests) before changing anything, to take steps small enough that each is reversible, and to lean on tooling and risk assessment to compensate for missing coverage.

The judgment problem is establishing confidence in current behavior before changing it, choosing steps and tools that minimize the chance of an unintended behavior change, and knowing when the risk of changing untested code is too high and the work should be declined or gated. The agent should not refactor untested code freely; it should first capture the existing behavior and then change in small, verifiable increments.

This skill applies whenever you are asked to refactor code that lacks tests, or whenever you are modifying legacy code without a safety net.

## Core Rules

### Build a characterization net before changing anything

A characterization test captures the current behavior of the code, whatever it is—even if that behavior includes bugs. The goal is not to assert the correct behavior but to lock in the actual behavior so that your refactor does not silently change it:

- Exercise the code through its public interface (or the most stable entry point) with inputs that cover the main paths and known edge cases.
- Assert on the actual outputs the code produces today, including quirks. If a function returns `-1` for not-found today, the characterization test asserts `-1`, even if `null` would be "better."
- Capture behavior at a boundary that will not move during the refactor (the public API, the HTTP response, the persisted state), so the tests remain valid as internals change.

Once characterization tests exist, you have a net: a behavior change during refactoring shows up as a test failure.

### Capture behavior at the right granularity

Characterization tests are most valuable at the boundary that matters and that will remain stable:

- Too fine-grained (private methods): the tests break during legitimate refactoring and provide noise rather than safety.
- Too coarse (full end-to-end): the tests are slow, brittle, and hard to localize when they fail.
- Right level: the public contract of the unit or module being refactored—its observable inputs and outputs.

Prefer the coarsest boundary that still lets you detect a behavior change in the code under refactor.

### Refactor in small, behavior-preserving steps

Each step should be small enough that it is obviously behavior-preserving and individually reversible:

- One refactoring move at a time (extract method, rename, move) with verification between steps.
- After each step, run the characterization tests (and any existing tests, linters, type checkers). If green, proceed; if red, revert and reconsider.
- Commit after each green step so you can roll back to the last known-good state easily.

Large, multi-move refactors in untested code are dangerous because a behavior change cannot be localized to a single step.

### Lean on automated, behavior-preserving tooling

Tools that guarantee behavior preservation reduce the risk of manual error:

- **Rename/extract/move refactorings in IDEs** that operate on the syntax tree and update all references are safer than find-and-replace.
- **Automated code transforms** (codemods, language-level tools) that are proven behavior-preserving can be trusted more than hand edits.
- **Type systems and linters** catch whole classes of behavior change (type errors, unused variables, signature mismatches).

Use the safest available tool for each move. Reserve hand-editing for moves no tool can do, and verify those moves extra carefully.

### Assess and mitigate risk per change

Not all untested refactors carry the same risk. Assess:

- **Criticality of the code**: refactoring a billing calculation is higher risk than refactoring a logging helper. Higher-criticality code demands a stronger net (more characterization tests, smaller steps, more review).
- **Complexity of the behavior**: code with many edge cases, non-obvious side effects, or concurrency is riskier and needs more thorough characterization.
- **Reversibility**: a change that can be reverted in minutes is lower risk than one that migrates data or changes a public API.

For high-risk, low-coverage code, the answer may be to invest heavily in characterization first, or to decline the refactor until coverage exists, rather than to proceed and hope.

### Make changes reversible

Favor changes that can be undone quickly:

- Keep the old and new code paths available behind a flag during the transition, so you can flip back if a problem appears.
- Avoid destructive moves (deleting the old implementation) until the new one is proven in production.
- Commit frequently so the last good state is one revert away.

If a refactor cannot easily be reversed (e.g., a data format change), treat it as high-risk and require correspondingly strong validation.

### Distinguish fixing bugs from preserving behavior

Characterization tests lock in current behavior, bugs included. If you discover a bug during refactoring, do not fix it in the same change as the refactor:

- Refactor first (behavior preserved, characterization tests stay green).
- Fix the bug separately (with a test that asserts the corrected behavior, updating the characterization tests intentionally).

Mixing refactor and bug-fix in one change makes it impossible to tell whether a test failure is an unintended regression or an intended behavior change.

## Common Traps

### Refactoring before capturing current behavior

Changing untested code without first writing characterization tests means any behavior change is invisible. The net must come before the move.

### Characterization tests at the wrong granularity

Tests on private internals break during legitimate refactoring; tests that are too end-to-end are slow and hard to localize. Choose the stable public boundary.

### Asserting "correct" behavior instead of actual behavior

A characterization test that asserts what the code *should* do, rather than what it *does* do, is not a safety net—it is a wish. Capture actual outputs, bugs included.

### Large multi-move refactors without intermediate verification

Bundling many refactoring moves into one change means a behavior change cannot be attributed to a single step, making diagnosis and rollback hard. One move, one verification, one commit.

### Hand-editing where a behavior-preserving tool exists

Manual find-and-replace or copy-paste refactoring is error-prone where an IDE refactoring or codemod would guarantee behavior preservation. Use the tool.

### Mixing refactor and bug-fix in one change

Combining a behavior-preserving refactor with a behavior-changing bug fix makes it impossible to verify behavior was preserved. Separate the two.

### Treating all untested code as equally safe to refactor

A logging helper and a billing calculation are not the same risk. Refactoring high-criticality, low-coverage code without a strong net invites production incidents.

### Deleting the old implementation before the new one is proven

Removing the fallback before the new path is validated in production removes your escape route. Keep the old path reversible until confidence is established.

## Self-Check

- Before changing untested code, have you written characterization tests that capture the current actual behavior (including quirks and known bugs) at a stable public boundary?
- Do the characterization tests assert actual outputs rather than desired outputs, so they function as a true safety net?
- Is each refactoring step small, individually behavior-preserving, verified against the net, and committed separately?
- Are you using behavior-preserving tooling (IDE refactorings, codemods, type systems) wherever possible instead of hand-editing?
- Have you assessed the risk of the change (code criticality, behavior complexity, reversibility) and scaled the strength of the net accordingly?
- Are changes reversible, with old paths kept available (behind a flag or in git history) until the new behavior is proven?
- Are bug fixes separated from refactors, so behavior preservation can be verified independently?
- For high-criticality, low-coverage code, have you either invested in strong characterization or declined the refactor until coverage exists?
- Have you confirmed the characterization tests cover the main paths and known edge cases, not just the happy path?
- After the refactor, do all characterization and existing tests remain green, with no behavior change except where intentionally introduced?
