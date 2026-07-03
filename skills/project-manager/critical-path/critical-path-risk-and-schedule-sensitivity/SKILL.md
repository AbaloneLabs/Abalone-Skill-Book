---
name: critical_path_risk_and_schedule_sensitivity.md
description: Use when the agent is assessing schedule risk on the critical path, analyzing near-critical path sensitivity, running schedule risk analysis or Monte Carlo simulation on a schedule network, or deciding where schedule contingency and buffer belong.
---

# Critical Path Risk And Schedule Sensitivity

The deterministic critical path tells you which chain of activities sets the project duration if every activity finishes exactly on its single estimate. But activities do not finish exactly on estimate; they finish within a range, and when ranges combine along a path, the project's likely finish is later and more uncertain than the deterministic answer suggests. Schedule risk analysis, often through Monte Carlo simulation, models activity duration uncertainty and reveals which paths are most likely to drive the end date, where contingency belongs, and how confident you should be in any committed date. The craft is in modeling uncertainty honestly, reading sensitivity correctly, and placing buffer where it does the most good. Agents tend to trust the deterministic critical path, to place contingency as a flat percentage, and to confuse activity risk with path risk.

Use this skill before committing to a schedule date, running or interpreting a schedule risk analysis, or deciding where to hold contingency. The goal is to prevent the agent from presenting a deterministic finish as if it were a confident forecast and from misplacing the buffer that protects the schedule.

## Core Rules

### Recognize That Deterministic Critical Path Understates Risk

The deterministic critical path assumes each activity hits its estimate exactly. In reality, variability compounds along the path, and because delays are easier than early finishes, the expected project finish is usually later than the deterministic one. A schedule that looks feasible deterministically can have a low probability of finishing on time once uncertainty is considered. Treat the deterministic finish as an optimistic anchor, not a forecast.

This is why projects that hit every activity estimate still finish late: the path that was not critical deterministically becomes critical when variability is included.

### Model Activity Duration Uncertainty As Ranges

Schedule risk analysis replaces single-point durations with ranges, typically three-point estimates of optimistic, most-likely, and pessimistic. The quality of the analysis depends on the quality of these ranges: optimistic cases that are barely better than most-likely, and pessimistic cases that are mild, produce falsely narrow results. The pessimistic tail must capture genuinely adverse but plausible outcomes, including rework, waiting, and external delays.

Use historical data to shape the ranges where possible, and have the people who will do the work inform the tails.

### Run Monte Carlo To See Path Probabilities

Monte Carlo simulation samples the activity ranges thousands of times, computes the project finish for each sample, and produces a distribution of possible finish dates with probabilities. This reveals the chance of finishing by any candidate date and which paths most often end up critical. The output is a probability curve, not a single date, and it should drive the commitment conversation.

Read the results at the path level: which paths appear on the critical path most often, and which activities most influence the finish.

### Distinguish Activity Risk From Path Sensitivity

A single risky activity does not necessarily threaten the project; what matters is whether it sits on a path that can become critical. Schedule sensitivity analysis identifies the activities and paths whose variability most affects the end date. An activity with high duration uncertainty but large float may be irrelevant to project risk, while a modestly uncertain activity on a near-critical path may dominate. Focus risk management on sensitivity, not on raw activity risk.

The criticality index, showing how often an activity is on the critical path across simulations, is more useful than the activity's own range.

### Watch Near-Critical Paths As Risk Sources

Near-critical paths are the main source of schedule surprises. A path with small float and high variability can overtake the critical path frequently under simulation, making the project finish driven by a chain that looked safe deterministically. Rank paths by their probability of becoming critical, not just by deterministic float, and manage the top few together.

Two near-critical paths slipping simultaneously can stretch the project more than either alone.

### Place Contingency Where Sensitivity Is Highest

Contingency and buffer protect the schedule, and they should be placed where simulation shows the most sensitivity, not spread as a flat percentage. A schedule risk analysis tells you which activities and paths most need protection, so buffer can be allocated to feed those chains. Buffer at the end of a critical chain, managed as a shared resource, is often more effective than buffer hidden inside individual activities.

Govern who can consume buffer, and make consumption visible so it is not silently exhausted.

### Communicate Probabilities, Not Point Dates

Stakeholders want a date; give them probabilities. Present the chance of finishing by candidate dates, such as 50 percent, 80 percent, and 90 percent confidence, and let the stakeholder choose the risk level. A committed date should correspond to a defensible confidence level, usually well above 50 percent, with the buffer to support it. Presenting the deterministic or median date as the commitment sets up failure.

Make explicit which date is a target, which is a forecast, and which is a commitment, and the confidence behind each.

### Update The Risk Model As Work Proceeds

A schedule risk analysis is a snapshot. As activities complete, their uncertainty resolves and the remaining risk picture changes. Re-run the analysis at major milestones or when significant changes occur, and watch whether the probability of meeting the committed date is improving or deteriorating. A commitment whose confidence is falling is an early warning that deserves action.

## Common Traps

### Trusting The Deterministic Critical Path As A Forecast

The deterministic finish is optimistic because it ignores variability that compounds along paths. Model uncertainty.

### Narrow Activity Ranges That Hide The Tail

Optimistic and pessimistic cases close to most-likely produce falsely confident results. Make tails genuinely adverse.

### Flat Contingency Regardless Of Sensitivity

A flat percentage over-protects low-sensitivity work and under-protects the paths that drive the finish. Place buffer by sensitivity.

### Confusing Activity Risk With Path Risk

A risky activity off the critical path may not matter; a modestly uncertain one on a near-critical path may dominate. Use sensitivity.

### Ignoring Near-Critical Paths

Paths with small float and high variability overtake the critical path under simulation. Rank by probability of becoming critical.

### Presenting The Median Date As The Commitment

A 50 percent date misses half the time. Commit at a defensible confidence with supporting buffer.

### Hidden Buffer Inside Activities

Buffer buried in activity durations is spent invisibly. Use visible, governed buffer at the chain level.

### Never Re-Running The Analysis

Risk changes as work completes. A stale risk model misleads; re-run at milestones and watch confidence trends.

## Self-Check

- [ ] Is the deterministic critical path treated as an optimistic anchor rather than a confident forecast?
- [ ] Are activity duration ranges modeled with genuinely adverse pessimistic tails, informed by history and doers?
- [ ] Does Monte Carlo simulation produce a probability distribution of finish dates rather than a single date?
- [ ] Is risk management focused on path sensitivity and criticality index rather than raw activity risk?
- [ ] Are near-critical paths ranked by probability of becoming critical and managed alongside the critical path?
- [ ] Is contingency placed where simulation shows highest sensitivity rather than spread as a flat percentage?
- [ ] Is buffer governed and visible, preferably at the chain level, rather than hidden inside activities?
- [ ] Are candidate finish dates communicated with their probability, letting stakeholders choose the risk level?
- [ ] Is the committed date backed by a defensible confidence level above the median?
- [ ] Is the schedule risk analysis re-run at milestones and when confidence in the commitment is falling?
