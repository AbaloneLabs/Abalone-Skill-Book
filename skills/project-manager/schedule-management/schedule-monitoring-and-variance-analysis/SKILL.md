---
name: schedule_monitoring_and_variance_analysis.md
description: Use when the agent is monitoring project schedule performance, calculating schedule variance and SPI, analyzing schedule slippage trends, determining whether delays threaten critical path or milestones, or diagnosing why a project drifted off schedule without being detected until too late.
---

# Schedule Monitoring And Variance Analysis

A schedule is a prediction, and predictions diverge from reality. The purpose of schedule monitoring is to detect that divergence early, understand its causes, and respond before small slips compound into missed milestones. The judgment problem is that schedule status is often reported as a simple percentage complete or a color-coded status that hides whether the project is actually on track. A task can report 80% complete while the remaining 20% holds the hardest work, or a slip on a non-critical task can look harmless while consuming all available float. Without structured variance analysis, schedule problems remain invisible until they become crises.

Use this skill before establishing schedule monitoring processes, before reporting schedule status, when analyzing whether a delay threatens the critical path, or when diagnosing why schedule problems were detected late. The goal is to prevent the agent from relying on optimistic self-reported status, from missing critical path threats, from confusing activity with progress, or from detecting schedule problems too late to recover.

## Core Rules

### [ ] Measure Progress Against A Baseline

Variance is meaningless without a baseline. The approved schedule baseline defines what should have happened; actual performance is compared against it. Without a baseline, "on track" is a subjective assertion rather than a measurable fact.

- [ ] Maintain an approved schedule baseline as the comparison reference.
- [ ] Compare actual progress against the baseline, not against a revised plan.
- [ ] Re-baseline only through formal change control, not silently.
- [ ] Preserve historical baselines to analyze trends.

### [ ] Use Earned Value For Objective Measurement

Earned value management (EVM) provides objective measures of schedule performance: Schedule Variance (SV) and Schedule Performance Index (SPI). These quantify whether the project is ahead or behind and by how much, replacing subjective status with measurable data.

- [ ] Calculate SV and SPI to measure schedule performance objectively.
- [ ] Use SPI to track whether work is being completed at the planned rate.
- [ ] Track trends in SPI over time, not just point-in-time values.
- [ ] Complement EVM with narrative explanation of causes.

### [ ] Monitor The Critical Path Continuously

Slippage on the critical path directly threatens project completion; slippage on non-critical tasks may be absorbed by float. Monitoring must distinguish between the two and prioritize attention on critical path tasks. A project can be "on track" overall while the critical path is quietly slipping.

- [ ] Identify and track the critical path throughout the project.
- [ ] Prioritize monitoring and response for critical path tasks.
- [ ] Monitor float consumption on near-critical paths.
- [ ] Re-calculate the critical path as the schedule changes.

### [ ] Track Float And Near-Critical Paths

Float is the buffer that absorbs slippage without delaying the project. As float is consumed, previously non-critical paths become critical. Monitoring float consumption provides early warning before a path becomes critical.

- [ ] Track remaining float on all paths, not just the critical path.
- [ ] Identify near-critical paths with little remaining float.
- [ ] Watch for float consumption trends that signal emerging risk.
- [ ] Re-assess criticality as float is consumed.

### [ ] Analyze Variance Causes, Not Just Magnitudes

Knowing the project is two weeks behind is less useful than knowing why. Variance analysis must dig into causes: scope changes, resource unavailability, underestimated effort, dependencies, or external delays. Cause analysis enables targeted response.

- [ ] Investigate the root cause of every significant variance.
- [ ] Categorize causes to identify systemic versus one-time issues.
- [ ] Distinguish causes within the project's control from external ones.
- [ ] Use cause analysis to inform corrective action.

### [ ] Distinguish Progress From Effort

Hours spent or tasks active are measures of effort, not progress. A team can be fully busy on work that is not advancing the schedule. Measure progress in terms of deliverables completed and milestones achieved, not activity.

- [ ] Measure progress by deliverables and milestones, not hours or activity.
- [ ] Watch for tasks that consume effort without advancing completion.
- [ ] Question status reports that emphasize activity over outcomes.
- [ ] Validate that reported progress reflects real deliverable completion.

### [ ] Forecast Completion Using Trends

Current variance predicts future performance if uncorrected. Use SPI and variance trends to forecast completion date and compare to the baseline. Forecasting surfaces whether the project will hit its deadline before the deadline arrives.

- [ ] Forecast completion date using current SPI and remaining work.
- [ ] Compare forecast to baseline to quantify expected slippage.
- [ ] Update forecasts as new actuals arrive.
- [ ] Use forecasts to trigger corrective action before milestones are missed.

### [ ] Report Schedule Status Honestly And Early

Optimistic status reporting hides problems until they are unrecoverable. Report schedule status honestly, including slippage and risks, and report early enough for corrective action. Honesty builds trust; optimistic reporting destroys it when reality arrives.

- [ ] Report schedule status accurately, including unfavorable variances.
- [ ] Avoid optimistic projections not supported by data.
- [ ] Report variances early enough for corrective action.
- [ ] Pair unfavorable status with options and a recommended response.

### [ ] Trigger Corrective Action Based On Thresholds

Define thresholds that trigger corrective action, for example, SPI below 0.9 or float consumed beyond 50%. Thresholds prevent the project from drifting while waiting for the problem to resolve itself. Action should be triggered by data, not by crisis.

- [ ] Define quantitative thresholds that trigger corrective action.
- [ ] Act when thresholds are breached, not when crisis arrives.
- [ ] Document the threshold-based action process so responses are consistent.
- [ ] Adjust thresholds based on project risk tolerance.

### [ ] Connect Schedule Monitoring To Risk Management

Schedule variances often signal underlying risks materializing. Connect schedule monitoring to the risk register: a variance may indicate a risk has occurred, and a risk may threaten future schedule performance. The two processes inform each other.

- [ ] Cross-reference schedule variances with the risk register.
- [ ] Update risk probability when related schedule variances appear.
- [ ] Use schedule forecasts to identify emerging risks.
- [ ] Ensure risk responses address schedule variance causes.

## Common Traps

### [ ] No Baseline Comparison

Reporting status without comparing to an approved baseline.

### [ ] Subjective Status

Color-coded or percentage status that hides whether the project is truly on track.

### [ ] Ignoring The Critical Path

Treating all slippage equally and missing threats to project completion.

### [ ] Float Blindness

Not monitoring float consumption until a path becomes critical.

### [ ] Effort Mistaken For Progress

Reporting busy-ness rather than deliverable completion.

### [ ] Optimistic Reporting

Hiding slippage with optimistic projections until recovery is impossible.

### [ ] No Action Thresholds

Drifting while waiting for problems to self-resolve.

### [ ] Disconnected From Risk

Treating schedule variance as a scheduling issue rather than a signal of materialized risk.

## Self-Check

- [ ] Is schedule progress measured against an approved, formally controlled baseline?
- [ ] Are SV and SPI used to measure schedule performance objectively?
- [ ] Is the critical path identified, tracked, and prioritized for monitoring?
- [ ] Is float consumption on near-critical paths monitored for early warning?
- [ ] Are variance causes analyzed, not just magnitudes reported?
- [ ] Is progress measured by deliverables and milestones, not by effort or activity?
- [ ] Is completion forecast using variance trends and compared to the baseline?
- [ ] Is schedule status reported honestly and early, including unfavorable variances?
- [ ] Are quantitative thresholds defined that trigger corrective action?
- [ ] Is schedule monitoring connected to the risk register and risk responses?
