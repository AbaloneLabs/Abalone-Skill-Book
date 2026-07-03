---
name: schedule_development_and_critical_path.md
description: Use when the agent is building a project schedule, sequencing activities, identifying the critical path, analyzing schedule slack, or reviewing whether a timeline is realistic and defensible against dependencies and capacity.
---

# Schedule Development And Critical Path

A schedule is a model of when work can realistically happen given sequence, dependencies, and capacity. It is not a set of hopeful dates. The critical path method exposes which work determines the project end date and where slack exists. Without this analysis, a schedule is a list of tasks with dates that collapse on the first delay. The project manager must sequence honestly, find the critical path, protect it, and use slack deliberately rather than treating every task as equally urgent.

Use this skill before creating a schedule, performing critical path analysis, defending a timeline, or diagnosing why a schedule keeps slipping. The goal is to prevent the agent from producing a calendar that looks complete but has no structural understanding of what actually drives the end date.

## Core Rules

### Sequence Activities Before Assigning Dates

A defensible schedule starts with logic, not with dates. Map which activities must precede, follow, or overlap others before any calendar date is attached.

Dependency relationships to model:

- finish-to-start, the most common, where one activity must finish before the next starts;
- start-to-start, where activities can begin together after a predecessor starts;
- finish-to-finish, where activities should complete together;
- start-to-finish, rare, where a successor finishes when a predecessor starts.

Also capture leads and lags deliberately, not accidentally. External, resource, and decision dependencies should appear as constraints on the network, not as assumptions.

### Identify The Critical Path

The critical path is the longest sequence of dependent activities that determines the earliest possible project completion. Any delay on a critical path activity delays the project unless scope, resources, or logic change.

To find it, perform forward and backward pass analysis to compute early start, early finish, late start, and late finish for each activity. Activities where early and late dates coincide have zero slack and sit on the critical path.

There can be more than one critical path, and a near-critical path can become critical after a small delay. Track both the critical path and the near-critical paths.

### Understand And Use Slack Deliberately

Slack, or float, is the amount an activity can slip without delaying the project. Total float is slip relative to the project end. Free float is slip relative to the next activity.

Slack is a resource, not an invitation to relax. Use it to absorb uncertainty, absorb resource conflicts, and protect the critical path. Do not let slack be silently consumed by low-priority work, because once it is gone the previously non-critical activity becomes critical.

### Resource-Level The Schedule

A logic-only schedule assumes infinite resources. Real projects share people, specialists, equipment, and budget. Resource leveling adjusts the schedule so no resource is overallocated, which usually extends duration.

Distinguish resource leveling, which resolves overallocation and may change the critical path, from resource smoothing, which adjusts within available float without changing the end date. The resource-leveled critical path, sometimes called the resource-critical path, is often longer than the logic-only critical path and is the more honest forecast.

### Protect The Critical Path

Because critical path delays directly extend the project, protect it actively. Techniques include adding buffer before critical milestones, starting critical activities early, securing critical resources, reducing critical path scope, fast-tracking by overlapping activities, or crashing by adding resources where the work can absorb them.

Be honest about the limits of compression. Fast-tracking increases risk and rework. Crashing increases cost and may not help if the work cannot be parallelized.

### Distinguish Committed, Forecast, And Target Dates

A target date is a wish. A forecast date is what the schedule predicts given current logic and capacity. A committed date is a defensible promise, usually carrying buffer or scope flexibility.

Communicate which kind of date each milestone carries. Presenting a forecast as a commitment invites failure. Presenting a target as a forecast hides the gap that must be resolved.

### Update And Re-Baseline The Schedule

A schedule is a living model. As actual progress, delays, and changes accumulate, update the schedule and compare against the baseline. Variance analysis shows whether the project is ahead or behind and where the slippage originates.

When changes are approved, re-baseline the affected portions and record why. A schedule that is never updated or never re-baselined stops reflecting reality and stops being useful.

## Common Traps

### Dating Before Sequencing

Assigning dates without understanding dependencies produces a schedule that looks tidy but breaks on the first real interaction.

### Ignoring Resource Constraints

A logic-only schedule assumes infinite capacity. The real, resource-leveled duration is usually longer.

### Treating All Tasks As Equally Urgent

When everything is urgent, the critical path is invisible and attention scatters. Focus protects the activities that actually drive the end date.

### Silent Consumption Of Float

Allowing non-critical work to absorb slack turns near-critical paths into critical paths and removes the project's ability to absorb shocks.

### False Compression

Adding people to late work, or overlapping activities that share dependencies, often increases cost and rework without saving time.

### Single Critical Path Assumption

Assuming only one critical path misses near-critical paths that become critical after minor slippage.

### Stale Schedule

A schedule updated only at major milestones, or never re-baselined after change, diverges from reality and misleads stakeholders.

### Confusing Effort With Duration In The Network

Using effort estimates as durations ignores availability and waiting, producing an optimistic network.

## Self-Check

- [ ] Activities are sequenced with explicit dependency relationships before dates are assigned.
- [ ] Critical path and near-critical paths are identified through forward and backward pass analysis.
- [ ] Total and free float are known and protected rather than silently consumed.
- [ ] The schedule is resource-leveled to reflect real capacity, not only logical sequence.
- [ ] The critical path is actively protected through buffer, early starts, secured resources, or defensible compression.
- [ ] Compression techniques are applied honestly, with their cost and risk acknowledged.
- [ ] Target, forecast, and committed dates are distinguished in communication.
- [ ] The schedule is updated regularly and re-baselined when approved changes occur.
- [ ] Variance against the baseline is analyzed to locate the source of slippage.
- [ ] External, resource, and decision dependencies are modeled as constraints, not assumptions.
