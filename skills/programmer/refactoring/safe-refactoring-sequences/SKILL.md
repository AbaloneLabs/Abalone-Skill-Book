---
name: safe_refactoring_sequences.md
description: Use when the agent is restructuring existing code without changing its behavior — extracting, inlining, renaming, moving, splitting, or merging modules, replacing a dependency, modernizing legacy code, breaking up a large function or class, or sequencing a large change into safe steps. Also covers behavior-preserving transformations, refactoring without tests, characterization tests, strangle-fig and branch-by-abstraction patterns, small-step commits, when to stop refactoring and return to feature work, and how to keep a refactor reversible and observable at every step.
---

# Safe Refactoring Sequences

A refactor is a behavior-preserving change, and "behavior-preserving" is a property of the *sequence* of changes, not of the final state. The same end result can be reached safely or catastrophically depending on the order of steps and whether each step is verifiable. An agent that rewrites a module to its desired final shape in one pass and then runs the tests has not refactored — it has performed an uncontrolled rewrite whose correctness rests on the hope that nothing important changed, and when it breaks, the cause is buried somewhere in a thousand-line diff.

Agents tend to fail at refactoring by optimizing for the destination instead of the path. The desired design is clear, so the agent jumps to it, conflating refactor with rewrite. The harm is that each intermediate state is unverifiable: a test failure cannot be localized to a step, a partial migration cannot be shipped, a reviewer cannot judge safety, and when something subtle breaks, there is no small step to revert to. The judgment problem is deciding how to decompose a structural change into a sequence where every step is small, individually verifiable, and leaves the system in a working state — so that the refactor is safe to stop, ship, or revert at any point.

This skill is about the *sequencing* of safe change. The design of what the code should become is a separate judgment (see module-boundary and architecture skills); this skill covers how to get there without breaking behavior on the way.

## Core Rules

### Do Not Refactor Without A Behavior Check

A refactor's safety rests entirely on the ability to detect behavior change. Before changing structure, confirm there is a way to know whether behavior was preserved. The strength of that check determines how aggressively you can refactor.

- **Strong existing tests** that exercise the behavior being changed are the best foundation. If they exist and are meaningful, small steps can be verified instantly.
- **Characterization tests** are the substitute when tests are absent. Before refactoring untested legacy code, write tests that capture the *current* behavior (including its quirks) by feeding representative inputs and pinning the outputs. These tests do not assert correctness; they assert "the behavior did not change." Write them for the paths the refactor will touch.
- **A type system, linter, or compiler** catches some classes of behavior change (signature mismatches, missing cases) but never catches semantic change. It is a floor, not a ceiling.

If you cannot observe behavior change, you are not refactoring — you are modifying blind. Either build the behavior check first, or narrow the refactor to what the existing checks can actually validate.

### Decompose Into Steps Small Enough To Verify Individually

Each step in the sequence should be independently correct, compilable, testable, and ideally shippable. The test of a good step size is: if this step were the last one before a release, would the system be safe? If the answer is no, the step is too large or mixes concerns.

A large structural change typically decomposes into phases:

1. **Preparation** — add the new structure alongside the old, without wiring it in. Tests still pass against the old path.
2. **Migration** — move callers or data to the new structure incrementally, one call site or one partition at a time, verifying behavior after each.
3. **Removal** — once nothing references the old structure, delete it.

Each phase is a sequence of small commits, and each commit leaves the system working. A reviewer should be able to understand and verify any single commit without holding the entire plan in their head.

### Preserve Behavior At Every Step, Not Just At The End

The defining property of a refactor is that behavior is preserved *throughout*, not merely restored at the end. A sequence that breaks behavior mid-way and fixes it at the end is a rewrite with a detour, and the intermediate broken states are unshippable and unreviewable.

Concretely, this means:

- Prefer transformations known to preserve behavior: extract function, inline function, rename, move, extract interface, introduce parameter object. These have well-understood safety conditions.
- Avoid mixing a behavior change with a structural change in the same step. If a bug must be fixed or a feature added, do it in a separate commit before or after the refactor, so the refactor commit is purely structural and its diff is easy to verify.
- After each step, run the behavior check. A step that changes behavior must be split until it does not.

If a step cannot be made behavior-preserving, it is not a refactor step; treat it as a separate, explicitly behavior-changing change with its own validation.

### Sequence So That Each Step Is Reversible

A safe refactor can be stopped or rolled back at any point. Design the sequence so that each step is independently revertible and the system is never in a state from which only finishing the whole plan can recover it.

Patterns that preserve reversibility:

- **Parallel structures** — build the new implementation next to the old, switchable by a flag or a single call-site change. You can switch forward or back in one commit.
- **Branch by abstraction** — introduce an abstraction that both old and new implementations satisfy, then swap implementations behind it. The abstraction is the seam that makes the swap reversible.
- **Strangle-fig** — for large systems, route new traffic or new entries to the new implementation incrementally while the old continues to handle the rest. Each increment can be held or rolled back independently.

Avoid sequences with a "point of no return": a single giant migration commit after which the old code is deleted and the only way back is a full revert. A point of no return turns a safe refactor into a high-risk cutover.

### Establish A Safety Net For Legacy Code Before Touching It

Legacy code often has no tests, unclear behavior, and implicit dependencies. Refactoring it directly is the highest-risk case. Before changing structure, invest in making behavior observable.

- Write characterization tests capturing current outputs for representative and edge-case inputs, including inputs that exercise the quirks you intend to preserve.
- Add logging or assertions at the boundaries of the code being changed, so that divergence between old and new behavior is visible.
- Where two implementations will coexist during migration, run them in parallel (shadow traffic, dual-write, or a verification harness) and compare outputs before trusting the new path.

The cost of the safety net is part of the cost of the refactor. Skipping it to save time transfers the risk to production, where it is far more expensive.

### Migrate Incrementally, Not By Big-Bang Switch

When replacing an implementation, a data store, an API, or a module, avoid switching all callers or all data at once. Migrate in partitions that can be verified and held independently: one tenant, one region, one feature, one call site at a time.

Each migration increment should:

- be small enough that a failure is contained and reversible;
- be verified against the behavior check before the next increment proceeds;
- leave a rollback path that does not require undoing all prior increments.

Big-bang switches concentrate risk at a single moment and make rollback all-or-nothing. Incremental migration distributes the risk and lets each step build confidence in the new path. This is especially important for data migrations, where a wrong transformation applied to all records at once can be irreversible.

### Separate Structural Change From Behavior Change

A common cause of unsafe refactors is bundling "while I'm in here, let me also fix this" changes into the structural work. Each bundled change makes the diff harder to verify and entangles a behavior change with a structural one, so a regression cannot be attributed to either.

Keep them separate:

- A **behavior-change commit** fixes a bug or adds a feature, with tests asserting the new behavior.
- A **refactor commit** only restructures, with the same tests passing before and after.

This separation lets reviewers verify the refactor purely on structure ("does it preserve behavior?") and the behavior change purely on intent ("is this the right new behavior?"). It also means a regression introduced by the refactor is isolated to a structural commit and easy to find.

### Know When To Stop Refactoring

Refactoring is a means, not an end. A refactor that has made the next change safe and cheap has done its job; continuing past that point adds risk without value. Decide in advance what the refactor is in service of — a feature, a fix, a risk reduction — and stop when that goal is met.

Stop and return to feature or fix work when:

- the code being changed next is already clear enough to work in safely;
- the remaining structural improvements are speculative ("cleaner") rather than unblocking;
- the refactor has grown large enough that its own risk outweighs the benefit of further cleanup;
- a deadline or release makes the safer choice to ship the current state and resume later.

A refactor that never ends is a sign the goal was never defined. Name the stopping condition before starting, and honor it.

### Keep The Refactor Observable And Communicated

A safe refactor is visible to others: each step is a reviewable commit, the plan is documented, and the system's state is inspectable. This protects against the agent being the only one who understands a half-finished migration.

- Commit in small, behavior-preserving units with messages that name the transformation ("extract X", "migrate caller Y to new Z").
- If the refactor spans multiple sessions or contributors, record the plan and current state (a short design note, an ADR, or even a tracking issue) so it can be resumed or handed off.
- Where a migration is incremental, make the current state visible in the code or configuration (a flag, a list of migrated tenants, a percentage) rather than implicit in someone's memory.

A refactor whose state lives only in an agent's context is fragile; it vanishes when the context ends.

## Common Traps

### The Rewrite Disguised As A Refactor

Rewriting a module to its final shape in one large change and then running the tests is not refactoring; it is an uncontrolled rewrite. When it breaks, the cause is unfindable in a massive diff, and there is no safe intermediate state to ship or revert to. Decompose into small behavior-preserving steps instead.

### Refactoring Without A Behavior Check

Changing structure in code with no tests, no characterization tests, and no type safety is modification by faith. The refactor will appear to work until a subtle behavior change surfaces in production. Build the behavior check first, even if that means writing characterization tests for behavior you do not fully understand.

### Mixing The Fix Into The Refactor

Bundling a bug fix or feature into a structural change makes the diff do two things at once, so a reviewer cannot tell whether a test change reflects intended new behavior or an accidental regression. Split them: structural commits preserve tests, behavior commits change them.

### The Big-Bang Cutover

Switching all callers, all data, or all traffic to a new implementation in a single commit concentrates all risk at one point and makes rollback all-or-nothing. Migrate incrementally so each step is independently verifiable and reversible.

### Deleting The Old Path Too Early

Removing the old implementation before the new one is fully proven removes the rollback path and the comparison baseline. Keep the old path alive (possibly deprecated or feature-flagged) until the new path has demonstrated equivalent behavior in production, then remove it in a final, deliberate step.

### Steps That Cannot Compile Or Pass Mid-Way

A sequence whose intermediate states do not compile or do not pass tests cannot be shipped, reviewed, or bisected. Each commit should leave the system building and green. If a step inherently cannot, split it until every state is whole.

### Continuing Past The Goal

Refactoring indefinitely because the code "could be cleaner" adds risk and review burden without unblocking anything. Define what the refactor is in service of and stop when that is achieved; speculative cleanup is a separate decision with its own justification.

### Trusting Tests That Do Not Cover The Changed Behavior and refactoring Under Deadline Pressure Without Reversibility

A green suite is meaningless if the tests do not exercise the behavior being refactored. Before relying on a test run as proof of behavior preservation, confirm the tests actually cover the paths and edge cases touched by the refactor. Coverage of unrelated code does not protect the code you changed.

When time is short, the temptation is to make one large change to finish faster. This is precisely when reversibility matters most, because a problem late in the cycle leaves no time for a safe recovery. Under pressure, prefer smaller steps and a shippable state at every commit, not fewer larger ones.

## Self-Check

- [ ] A behavior check exists before refactoring starts: meaningful existing tests, or characterization tests written for the current behavior of the paths being changed, covering representative and edge-case inputs.
- [ ] The change is decomposed into steps small enough that each is independently compilable, testable, shippable, and reviewable — no step leaves the system in a broken intermediate state.
- [ ] Behavior is preserved at every step, not just restored at the end; each step passes the behavior check, and no step mixes a behavior change with a structural change.
- [ ] The sequence is reversible at every point: new and old structures coexist behind a flag, abstraction, or parallel path, with no "point of no return" commit.
- [ ] For legacy or untested code, a safety net (characterization tests, parallel-run comparison, or assertions) was built before structural changes began.
- [ ] Migrations are incremental (one caller, tenant, region, or partition at a time), each verified and individually rollback-able, not a big-bang switch.
- [ ] Structural commits and behavior-change commits are separate; refactor commits preserve the same tests, and any test change is in a dedicated behavior commit.
- [ ] The stopping condition was defined before starting, and the refactor stops once the next change is safe and cheap rather than continuing into speculative cleanup.
- [ ] Each step is a small, clearly described commit, and the plan and current state are documented so the refactor can be resumed or handed off without lost context.
- [ ] The tests relied upon as the behavior check actually cover the behavior being refactored, not merely unrelated code that happens to be green.
