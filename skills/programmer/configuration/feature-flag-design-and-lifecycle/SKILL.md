---
name: feature_flag_design_and_lifecycle.md
description: Use when the agent is adding or designing a feature flag, deciding what kind of flag to use, combining a flag with an A/B experiment, planning a flag's rollout and rollback, managing flag dependencies, or auditing and retiring flags that have accumulated and become technical debt.
---

# Feature Flag Design And Lifecycle

A feature flag is a powerful, dangerous tool. It decouples deploy from release, letting incomplete or risky changes ship safely behind a switch, roll out incrementally, and roll back instantly without a redeploy. Used well, flags are how teams move quickly and safely at the same time. Used carelessly, they become a sprawling layer of conditional logic that no one fully understands: flags that are always on, flags that depend on other flags, flags that were meant to be temporary and are still present three years later, flags whose combinations produce code paths no one tested. Feature flag debt is one of the most insidious forms of technical debt, because each flag looks small and each is hard to remove once consumers depend on its behavior.

Agents tend to treat flags as a one-time addition: add the flag, gate the code, ship. They under-invest in the flag's entire lifecycle — its type, its rollout plan, its interaction with other flags, and crucially its removal. The result is flag accumulation, where the conditional surface grows faster than it shrinks, the matrix of flag combinations becomes untestable, and the codebase carries the weight of every experiment and rollout ever conducted. The judgment problem is to choose the right flag for the purpose, to design it with its whole lifecycle in mind from the start, to manage its interactions and rollout, and — most importantly — to treat removal as part of the definition of done, so that flags are temporary by design rather than permanent by neglect.

## Core Rules

### Choose The Flag Type For Its Purpose

Not all flags serve the same purpose, and using the wrong type creates the wrong lifecycle. The main types, each with a different expected lifetime and removal criterion:

- **Release flags** gate incomplete code so it can be deployed before it is released. These are short-lived: once the feature is fully rolled out, the flag and its conditional should be removed. Their purpose is to decouple deploy from release.
- **Experiment flags** (A/B tests) assign users to variants to measure an effect. These are short-lived: once the experiment concludes and a decision is made, the losing variant is removed and the flag is retired. Their purpose is to learn.
- **Ops flags** toggle operational behavior (a kill switch, a degraded mode, a fallback path). These are long-lived by design and must be tested continuously, because they are meant to be flipped under pressure. Their purpose is operational control.
- **Permission flags** gate access to a feature for a subset of users (entitlement, beta, licensing). These are long-lived and are part of the product's access model, not temporary scaffolding.

The type determines everything downstream: how long the flag should live, when it must be removed, whether the off-path must be maintained and tested, and how to name it so its intent is clear. A release flag named like a permanent permission flag invites confusion about whether it can be removed. Name and document the flag's type at creation so its lifecycle is unambiguous.

### Design The Flag With Its Removal In Mind

Every short-lived flag (release, experiment) should be created with a plan for its removal, and that plan should include a specific trigger and owner. "Remove this flag once rollout hits 100% and is stable for two weeks" is a removal plan; "we'll clean it up later" is not. Without a defined trigger, the flag survives long past its purpose, because there is always something more urgent than cleanup and the flag's harm is invisible.

Build removal into the workflow:

- **Set an expiration or milestone** on the flag at creation, with an owner responsible for acting on it.
- **Track active flags** so the inventory is visible and stale flags are surfaced. A flag dashboard that shows age and rollout percentage makes debt visible.
- **Treat flag removal as part of the feature's done definition**, not as optional follow-up. A feature is not complete until its release flag is gone and the conditional is collapsed to the released path.

The discipline matters because flags are easy to add and hard to remove. Once code accumulates around a flag — other flags depending on it, tests covering its combinations, operators relying on its behavior — removal becomes a project rather than a cleanup. Removing early, while the flag is still simple, is vastly cheaper.

### Decide The Default And Make The Off-Path Real

Every flag has a default state when not evaluated (the fallback if the flag service is unavailable, the value for new users, the state at startup). That default must be chosen deliberately, because it is the state the system falls into under failure, and it must be the safe state. For a release flag, the safe default is usually off (do not show the unfinished feature); for a kill switch, the safe default is usually on (the system runs normally) or off (the fallback path runs) depending on design.

Equally important: the off-path (the code that runs when the flag is off) must be real, maintained, and tested, for as long as it can be reached. A flag whose off-path is untested because "it's always on in production" will fail the one time it is flipped — usually during an incident, when the cost of failure is highest. If the flag is a release flag destined for removal, the off-path only needs to survive until removal; if it is an ops flag, the off-path must be tested continuously as long as the flag exists, because it is part of the operational safety mechanism.

### Manage Flag Dependencies And Combinations

Flags do not exist in isolation. Two flags can interact: flag A gates a feature that only works if flag B is also on; turning off B while A is on produces a broken or undefined state. As flags accumulate, the matrix of combinations grows combinatorially, and the number of states the system can be in quickly exceeds what can be tested or reasoned about.

Manage dependencies explicitly:

- **Document and enforce flag dependencies** ("A requires B") so invalid combinations are prevented at evaluation time rather than producing undefined behavior.
- **Limit flag combinations** by limiting the number of simultaneously active short-lived flags. The more flags overlap in the same code, the larger the test matrix and the higher the risk of an untested combination failing.
- **Be wary of flags on flags** — gating a flag's rollout by another flag creates nested conditionals that are hard to reason about and remove. Prefer independent rollouts over deeply nested gating.

The test matrix for N independent boolean flags is 2^N states. Beyond a small number, exhaustive testing is impossible, and the untested combinations are where bugs hide. Keeping the active flag count low is a direct reduction of risk.

### Plan Rollout And Rollback As First-Class Operations

A flag's value is in its rollout and rollback. Rollout is the gradual increase of the flag's exposure (more users, more traffic, more regions), with verification at each step that the new path behaves correctly. Rollback is the instant reversion to the old path when something goes wrong, without a redeploy. Both must be planned, not improvised.

For rollout:

- **Increase exposure in verifiable increments** (1%, 5%, 25%, 50%, 100%), checking metrics and errors at each step before proceeding.
- **Define the rollback trigger in advance** — which metrics, which error rates, which symptoms mean "roll back now" — so the decision is mechanical under pressure rather than a debate during an incident.
- **Ensure the flag evaluation is fast and reliable**, because a slow or failing flag service degrades every request that depends on it.

For rollback:

- **Confirm the old path still works** before relying on it. A rollback to an off-path that has drifted or broken is not a rollback; it is a second failure.
- **Make rollback instant and reversible**, which is the whole point of using a flag rather than a redeploy.

A flag whose rollout and rollback have not been planned is just a conditional that adds risk without delivering the safety it promised.

### Combine With Experiments Deliberately, Not Accidentally

Experiment flags and release flags are often combined — rolling out a feature behind a flag while also measuring its effect — and the combination must be deliberate. An experiment layered on a release flag must define its variants, its assignment (which users see which variant), its success metric, and its stopping condition before it starts. Running an experiment without these defined produces data that cannot be interpreted and experiments that never conclude.

Be especially careful that the experiment's assignment is consistent (a user sees the same variant across requests) and that the experiment does not interact with other active experiments in ways that contaminate the measurement. An experiment that runs alongside an uncontrolled rollout produces confounded results; an experiment with no stopping condition runs forever and becomes permanent debt.

## Common Traps

### Flags That Never Get Removed

Short-lived flags (release, experiment) that survive past their purpose accumulate as permanent conditionals, growing the test matrix and the code's complexity indefinitely. Build removal into the flag's definition of done, with a trigger and owner, and track stale flags so the debt is visible.

### The Untested Off-Path

A flag whose off-path is never exercised because the flag is always on will fail the one time it is flipped, usually during an incident. For ops flags, test the off-path continuously; for release flags, remove the flag before the off-path has a chance to drift.

### Combinatorial Flag Explosion

Each additional active flag doubles the possible states; beyond a few, the matrix is untestable and untested combinations fail in production. Limit simultaneously active short-lived flags and document dependencies to keep the matrix manageable.

### Using A Short-Lived Flag Type For A Long-Lived Need

A release flag used for permanent access control becomes a permanent flag with the wrong lifecycle — never removed, never tested as an ops path, confusing to operators. Match the flag type to the purpose, and use permission or ops flags for long-lived needs.

### Rollback To A Broken Old Path

Rolling back to an off-path that has drifted or was never maintained is not a recovery; it is a second failure. Confirm the old path works before relying on it as a rollback target.

### Undefined Or Confounded Experiments

An experiment without defined variants, assignment, success metric, and stopping condition produces uninterpretable data and never concludes. Define the experiment fully before starting, and isolate it from confounding rollouts.

### Flag Evaluation As A Hidden Dependency And Performance Cost

Code that evaluates flags on every request without caching, or that depends on a flag service whose outage breaks the request, turns the flag system into a reliability and performance liability. Evaluate flags efficiently, cache where appropriate, and ensure the default-on-unavailable state is safe.

## Self-Check

- [ ] Each flag's type (release, experiment, ops, permission) is chosen for its purpose, named and documented accordingly, with a lifecycle matching the type.
- [ ] Short-lived flags have a defined removal trigger and owner at creation, removal is part of the feature's done definition, and stale flags are tracked and surfaced for retirement.
- [ ] The flag's default state is the safe state for failure, and the off-path is real, maintained, and tested for as long as it can be reached (continuously for ops flags).
- [ ] Flag dependencies are documented and enforced, the number of simultaneously active short-lived flags is bounded, and deeply nested flag gating is avoided to keep the combination matrix testable.
- [ ] Rollout proceeds in verifiable increments with metrics checked at each step, and the rollback trigger (which metrics or symptoms) is defined in advance so rollback is mechanical under pressure.
- [ ] The old path was confirmed working before being relied on as a rollback target, so rollback is a recovery rather than a second failure.
- [ ] Experiments combined with release flags have defined variants, consistent assignment, a success metric, and a stopping condition, and are isolated from confounding rollouts.
- [ ] Flag evaluation is fast, cached where appropriate, safe under flag-service unavailability, and not a hidden reliability or performance liability.
