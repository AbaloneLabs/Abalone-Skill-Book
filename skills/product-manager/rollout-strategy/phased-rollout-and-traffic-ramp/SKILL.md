---
name: phased_rollout_and_traffic_ramp.md
description: Use when the agent is planning a phased rollout, deciding traffic percentages and ramp speed, choosing rollout stages from internal to canary to general availability, or balancing speed against risk during incremental release.
---

# Phased Rollout And Traffic Ramp

A phased rollout is the plan for how a finished, ready feature reaches real traffic in increasing exposure over time. The point is not caution for its own sake; it is to learn from a small blast radius before a defect or a bad interaction reaches everyone, and to balance the cost of going slow against the cost of going fast and being wrong. The ramp plan, the percentages, and the hold times are the levers, and they should be chosen deliberately rather than inherited from habit.

Agents miss this because "roll out gradually" sounds like a single obvious idea. They pick a default ramp like 5, 50, 100, apply it everywhere, and treat the schedule as the strategy. The harm is that a risky, irreversible change gets the same ramp as a trivial copy fix, a canary stage runs with no defined reason to stop, and the team either over-cautions a safe change into months of delay or under-cautions a dangerous one into a full incident. The opposite failure is rolling out so slowly that learning takes a quarter and the team loses the will to iterate.

Use this skill before answering broad questions such as "how should we phase this rollout", "what percentage should we start at", "how fast should we ramp", "do we need a canary stage", or "when can we go to general availability". The goal is to prevent the agent from choosing a ramp schedule without weighing risk, reversibility, learning need, and the cost of delay.

## Core Rules

### Choose The Ramp By Risk, Reversibility, And Learning Need

There is no universally correct ramp. The right schedule depends on how bad a failure would be, how reversible the change is, and what the team needs to learn before full exposure. A reversible, low-risk change can move fast; an irreversible migration must move slowly and deliberately.

For each rollout, classify the change. Reversible changes behind a flag with no data migration can ramp quickly. Customer-visible strategic changes need a slower ramp with qualitative feedback at each stage. Irreversible changes such as schema migrations, deletions, or breaking API changes need the slowest, most gated path the team can tolerate, because a failure at full exposure cannot be undone. Let the risk class set the ramp, not the calendar.

### Define Each Stage By What It Learns

A rollout stage earns its place by teaching something the previous stage could not. Internal dogfooding catches obvious breakage and UX confusion. A private beta gathers qualitative signal and validates value with close customers. A canary at low traffic catches production-only failures at small scale. A percentage ramp confirms that metrics hold as exposure grows. General availability is the stage where the team commits.

State, for each stage, what signal it is meant to produce and what result would prevent moving to the next stage. A stage with no defined learning goal is just delay. A ramp where every stage measures the same thing is a slower version of one stage, not a phased rollout.

### Set Starting Percentage And Hold Times Deliberately

The starting exposure and the hold time at each level are the two most consequential ramp parameters, and both are often chosen by feel. The starting percentage should be large enough to produce signal and small enough to contain the blast radius of a failure. The hold time should be long enough to observe the relevant effects and short enough that learning does not stall.

Choose the starting percentage based on traffic volume and failure detectability. On high-volume services, a small percentage produces enough events to detect problems quickly; on low-volume services, a too-small percentage produces no signal at all. Set hold times based on how long it takes for the relevant effects, including delayed ones like a daily batch or a weekly cycle, to appear. Default to longer holds when effects are delayed or seasonal.

### Decide Manual Gates Versus Automatic Progression

A ramp can progress automatically on healthy metrics, pause for manual review at each stage, or some combination. Automatic progression is fast but expands on momentum even when the signal is ambiguous. Manual gates add judgment and latency but catch the subtle regressions that thresholds miss.

Most ramps use both: automatic rollback or pause on clear failure, manual review before expanding past a defined checkpoint. Decide in advance which stages are automatic and which require a human sign-off, and who has authority to expand, hold, or roll back. A ramp that is nominally automatic but waits on a chat message for someone to approve the next step is a manual ramp with extra latency and unclear ownership.

### Plan The Path To General Availability, Not Just The Ramp

A rollout plan that defines the ramp but not the exit leaves the feature stuck at partial exposure indefinitely. Define what general availability requires: which metrics must hold, for how long, across which segments, before the team commits to full exposure.

State the GA criteria up front, including the guardrail metrics that must not regress and the qualitative signals that must be positive. Without explicit GA criteria, features linger at 50% for months because no one is sure whether to finish, and the team accumulates partial rollouts that complicate every future change.

### Account For Population Composition During The Ramp

The users exposed at each stage are not a random sample of the final population unless the ramp is designed that way. Internal users behave differently than customers; beta users are more tolerant; early percentage cohorts may be skewed by region, account size, or device. A ramp that confounds exposure with population composition produces misleading signals.

Ensure the ramp population is representative where it matters, or acknowledge the bias explicitly. If the canary is employees only, do not treat their experience as predictive of customer behavior. If the percentage ramp is region-gated, do not generalize regional results globally without checking for regional differences.

### Balance Speed Against The Cost Of Delay

Going slow has a cost. Every day a feature sits at partial exposure is a day some users lack it, a day the team context-switches, a day the unlaunched capability does not generate value. The ramp should be as fast as the risk allows, not as slow as the caution permits.

Weigh the cost of delay against the cost of failure. For low-risk changes, aggressive caution is wasteful. For high-risk changes, the cost of a failed full launch dwarfs the cost of a careful ramp. Make the tradeoff explicit rather than defaulting to either maximum speed or maximum caution.

## Common Traps

### One Default Ramp For Every Change

Applying the same 5-50-100 schedule to a copy fix and a data migration either over-cautions the safe change or dangerously under-cautions the risky one. Risk class should set the ramp.

### Stages With No Learning Goal

Adding canary or beta stages that measure the same things as the next stage produces delay without information, slowing the rollout without reducing risk.

### Starting Percentage Too Small For Signal

On low-traffic services, a 1% canary generates too few events to detect anything, giving false confidence that the rollout is safe when it has simply not been tested.

### Automatic Progression On Ambiguous Signal

Expanding automatically because metrics are not clearly broken lets subtle regressions ride momentum to full exposure, turning a containable issue into a widespread one.

### No Defined GA Criteria

Without explicit criteria for reaching general availability, features stall at partial exposure for months, blocking future work and leaving the rollout permanently unfinished.

### Confounding Exposure With Population

Treating internal, beta, or region-gated cohorts as representative of all users leads to confident conclusions from skewed samples that collapse at full exposure.

### Maximum Caution As A Default

Defaulting to the slowest possible ramp treats caution as free. For low-risk changes, the cost of delay is real and the benefit of caution is negligible.

## Self-Check

- [ ] The ramp schedule was chosen based on risk, reversibility, and learning need, not inherited from a default.
- [ ] Each stage has a defined learning goal and a defined result that would prevent moving to the next stage.
- [ ] The starting percentage is large enough to produce signal on this service's traffic volume, and hold times reflect how long effects take to appear.
- [ ] The plan specifies which stages progress automatically and which require manual sign-off, and who has authority to expand, hold, or roll back.
- [ ] General availability criteria are defined up front, including guardrail metrics and qualitative signals.
- [ ] The ramp population is representative where it matters, or known biases are acknowledged rather than ignored.
- [ ] The cost of delay is weighed against the cost of failure, and the ramp is as fast as the risk allows.
- [ ] Irreversible changes such as migrations have a slower, more gated path than reversible changes behind a flag.
- [ ] Delayed effects (daily batches, weekly cycles, seasonal patterns) are accounted for in hold times.
- [ ] No stage exists purely to add ceremony; every stage earns its place by teaching something new.
