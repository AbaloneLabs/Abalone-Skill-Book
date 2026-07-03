---
name: rollout_risk_and_rollback_management.md
description: Use when the agent is managing rollout risk, defining rollback triggers and thresholds, planning kill-switch criteria, or deciding when to pause, roll back, or continue a rollout based on monitoring signals.
---

# Rollout Risk And Rollback Management

A rollout reduces risk only if something is watching the traffic and is willing to act on what it sees. The ramp itself does not make a launch safe; the monitoring, the thresholds, and the willingness to pause or roll back do. Rollout risk management is the discipline of deciding, in advance and calmly, what would make you stop, and then actually stopping when that happens.

Agents miss this because the ramp feels like the safety mechanism. They expose a small percentage, watch a dashboard, and expand because nothing is obviously on fire, never having decided what "obviously on fire" means. The harm is that subtle regressions ride momentum to full exposure, the team expands on optimism rather than evidence, and a containable defect at 5% becomes a full incident at 100%. The opposite failure is over-triggering, rolling back on every blip until the team loses trust in the alarms and starts ignoring them, including the real ones.

Use this skill before answering broad questions such as "what should trigger a rollback", "what thresholds should we set", "when should we pause the rollout", "how do we decide whether to continue", or "what monitoring do we need during the ramp". The goal is to prevent the agent from treating monitoring as observation and forgetting that it only helps if it drives a decision.

## Core Rules

### Define Health Metrics Across Technical And Business Dimensions

A rollout can fail technically while business metrics look fine, or fail on business outcomes while technical metrics are green. Error rate and latency tell you the system is healthy; conversion, activation, and revenue guardrails tell you the feature is actually working. Both dimensions must be watched, because a feature that loads fast and breaks nothing can still quietly regress the metric it was meant to improve.

Define a health set that includes technical metrics (error rate, latency, resource saturation) and business guardrails (conversion, retention proxies, revenue, support volume). A rollout that only watches technical metrics will ship features that are technically flawless and commercially harmful.

### Set Thresholds Before The Ramp, Not During It

Thresholds decided during the rollout are contaminated by what the team wants to be true. If the metric drifts past a number, the temptation is to redefine the threshold rather than stop. Decide, in advance and calmly, what level of regression or error is acceptable and what level triggers a pause or rollback.

For each metric, define a green band, an amber zone that triggers investigation, and a red line that triggers automatic action. Write the thresholds down before exposure begins, and treat them as commitments, not guidelines. A threshold no one is willing to enforce is not a threshold.

### Distinguish Pause, Rollback, And Continue As Separate Decisions

The response to a worrying signal is not binary. Pause holds exposure at the current level while the team investigates. Rollback reduces or removes exposure to stop the harm. Continue expands exposure, committing further. These are different decisions with different costs, and conflating them leads to either premature rollback or dangerous expansion.

Define when each is the right response. Pause is appropriate when the signal is ambiguous and reversible. Rollback is appropriate when the signal is clearly bad or the change is irreversible. Continue is appropriate only when metrics are within the green band for the full hold time. Making the three explicit prevents the default of expanding on momentum.

### Account For Detection Lag And Delayed Effects

Metrics do not update instantly. A rollout can look healthy at hour one and fail at hour twenty-four, because the failure depends on a daily batch, a weekly cycle, a cache expiry, or user behavior that only emerges over time. Hold times must be long enough for delayed effects to surface, or the green light at hour one is meaningless.

Identify which effects are delayed in this system and set hold times accordingly. A change that affects a daily report must be held past a daily cycle. A change that affects billing must be held past a billing run. A change that affects seasonal behavior must be held long enough to observe the relevant period, or the rollout commits before the risk has been observed.

### Decide Automatic Versus Human Action By Confidence And Speed

Some failures are clear and fast, and an automatic rollback protects the system without waiting for a human. Others are subtle and ambiguous, and a human judgment is worth the latency. The split between automatic and human action should be deliberate, based on how clearly the failure signature is known and how fast the response must be.

Use automatic rollback for clear, well-understood failure signatures where speed matters: sharp error spikes, latency collapse, saturation. Use human review for ambiguous signals where the cost of a false rollback is high and the cost of delay is tolerable. Tune automatic triggers carefully to avoid flapping on noise, which erodes trust in the entire system.

### Plan The Rollback Mechanics, Not Just The Trigger

Deciding to roll back is useless if the rollback itself is slow, broken, or partially effective. The team must know how rollback is executed, how long it takes, what state it leaves users in, and who can perform it. A rollback path that has never been exercised is a hypothesis, not a plan.

Verify the rollback mechanics before the rollout. Confirm that the flag, config, or deploy that reverses exposure actually works, that it covers all entry points, and that the team can execute it under pressure. For irreversible changes where rollback is not possible, the rollout plan must be even more conservative, because there is no recovery once committed.

### Watch For Confounding From Concurrent Changes

A rollout rarely happens in isolation. Other deploys, experiments, infrastructure changes, or external events can move the same metrics the rollout is watching, making it hard to attribute a regression to the right cause. A regression attributed to the new feature may actually come from an unrelated change, and a feature that looks safe may be masked by a compensating improvement elsewhere.

Track what else is changing during the rollout window, and avoid stacking risky changes when possible. When a regression appears, the team should be able to distinguish the new feature's effect from concurrent noise, or the decision to roll back is based on a guess about cause.

## Common Traps

### Watching Only Technical Metrics

A rollout that monitors error rate and latency but not business guardrails can ship a feature that is technically perfect and commercially harmful, and the dashboard stays green while value erodes.

### Thresholds Decided During The Rollout

When the metric drifts, the team redefines the threshold to justify continuing, turning a commitment into a rationalization and expanding on contaminated judgment.

### Expanding On Momentum

When nothing is clearly broken, the default becomes "expand", even when the hold time has not elapsed or the signal is ambiguous. Momentum replaces evidence.

### Hold Times Shorter Than Delayed Effects

Releasing past a one-hour hold when the failure depends on a daily cycle commits the team to a green light that has not yet observed the relevant risk.

### Untested Rollback Path

Assuming rollback works without verifying it leads to the discovery, under incident pressure, that the reversal is slow, incomplete, or broken.

### Automatic Triggers That Flap On Noise

Over-sensitive automatic rollback that fires on normal variance trains the team to ignore or disable the alarms, including the ones that matter.

### Ignoring Concurrent Changes

A regression during rollout gets attributed to the new feature when it came from an unrelated change, or a real regression gets masked by a concurrent improvement, and the decision is based on misattributed cause.

## Self-Check

- [ ] The health metric set includes both technical metrics (error rate, latency, saturation) and business guardrails (conversion, revenue, support volume).
- [ ] Thresholds for green, amber, and red were defined before exposure began and are treated as commitments.
- [ ] Pause, rollback, and continue are defined as separate decisions with distinct criteria, not collapsed into a binary choice.
- [ ] Hold times are long enough to surface delayed effects such as daily batches, weekly cycles, or seasonal patterns.
- [ ] The split between automatic and human action is deliberate, based on failure-signature clarity and required response speed.
- [ ] Automatic triggers are tuned to avoid flapping on normal variance so the team continues to trust them.
- [ ] The rollback mechanics have been verified to work, cover all entry points, and are executable under pressure within a known time.
- [ ] For irreversible changes, the rollout is more conservative because rollback is not a recovery option.
- [ ] Concurrent changes during the rollout window are tracked so regressions can be attributed to the right cause.
- [ ] The decision to continue is based on metrics held within the green band for the full hold time, not on momentum or optimism.
