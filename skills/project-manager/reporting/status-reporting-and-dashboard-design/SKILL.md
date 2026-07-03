---
name: status_reporting_and_dashboard_design.md
description: Use when the agent is designing project status reports, building project dashboards, selecting project metrics and KPIs, reporting progress to sponsors and steering committees, creating executive summaries, or deciding what project information to surface to which audiences.
---

# Status Reporting and Dashboard Design

Status reporting is the primary mechanism by which project truth reaches decision-makers. A well-designed report gives a sponsor the information needed to act in the time they have; a poorly designed one buries signal under noise, presents optimism where there is risk, and leaves decision-makers surprised by problems that were visible but invisible in the reporting. The harm is asymmetric: a report that understates risk can cause a late, expensive surprise, while a report that overstates risk merely costs a conversation.

The judgment problem is how to select metrics that reflect real project health rather than activity, how to design reports that match each audience's decision needs, and how to present variance and risk honestly without alarmism or spin. Agents tend to report what is easy to measure, fill dashboards with vanity metrics, and soften bad news until it is too late to act.

## Core Rules

### Design Reports Around Audience Decisions, Not Available Data

Each report should be designed for the decisions its audience must make. A sponsor needs to know whether the project will deliver its objectives on time and on budget, and what help they need to provide. A steering committee needs to know about cross-project dependencies and risks requiring governance action. A team lead needs task-level detail. Start from the decision, then select the data that informs it. Reporting that starts from available data and works forward produces dashboards full of numbers no one acts on.

### Separate Health Indicators From Activity Measures

Activity measures (hours worked, tasks completed, story points burned) show effort, not health. Health indicators (schedule variance, cost variance, scope completion against plan, defect trends, risk exposure) show whether the project is on track to deliver. Dashboards dominated by activity metrics create the illusion of progress while the project slips. Lead with health indicators; use activity measures only as supporting context.

### Report Variance Against Baseline, Not Absolute Progress

Saying "we are 60 percent done" is meaningless without the baseline. "We planned to be 70 percent done and are at 60 percent, a 10 percent schedule variance" is actionable. Always report against the approved baseline, and make the variance explicit. When the baseline has been re-baselined, preserve the historical variance so performance trends are not erased. Variance against baseline is the core truth of status reporting.

### Make Risk and Issues Visible, Not Buried

Risk and issue status belongs in the report, not in a separate document no one reads. Surface the top risks and issues, their current status, and the actions being taken. Softening or omitting bad news to avoid uncomfortable conversations guarantees that the news surfaces later, when it is worse and trust is damaged. Honesty about risk is not alarmism; it is the precondition for help.

### Use Forecasting to Show Where the Project Is Heading

Status reports that show only where the project is today miss the point. Decision-makers need to know where it is heading: the forecast at completion for cost and schedule, the trend of the variance, and the confidence in the forecast. Use earned value forecasts, trend lines, or burn projections to show trajectory. A project that is slightly behind but trending to recovery is different from one slightly behind and trending to further slip.

### Match Granularity and Cadence to the Decision Cycle

Executive sponsors need concise, decision-oriented reports at a cadence matched to their governance cycle, often monthly. Team leads need detailed, frequent updates. Over-reporting to executives wastes their attention; under-reporting leaves them blind. Define the cadence and granularity for each audience and resist the pressure to add detail to executive reports, which dilutes the signal.

### Distinguish Signal From Noise With Thresholds and Color Coding

Color coding (green/amber/red) is useful only when the thresholds are defined objectively. "It feels green" is not a status. Define what variance, risk exposure, or issue severity triggers each color, and apply them consistently. When everything is green until it suddenly is red, the color coding is not providing early warning. Calibrate thresholds so amber appears before red, giving time to act.

### Maintain a Single Source of Truth for Reported Data

Reports should draw from a single authoritative data source, not from separately maintained spreadsheets that drift apart. When each report is hand-maintained, the numbers disagree and decision-makers lose trust. Automate where possible and ensure all reports trace to the same baseline and actuals.

## Common Traps

### Activity Metrics Masquerading as Health

The dashboard shows hours logged and tasks done, implying progress, while schedule and cost variance worsen unnoticed. The trap is that activity is easy to measure and health is hard. Lead with health indicators.

### Softening Bad News

The report downplays a slipping schedule or growing risk to avoid an uncomfortable conversation, and the problem surfaces later as a crisis. The trap is that delay compounds the harm. Surface risk honestly.

### Reporting Without Baseline Reference

"60 percent complete" with no baseline is meaningless and often misleading. The trap is that absolute progress feels informative but is not. Report variance against baseline.

### Vanity Metrics and Dashboard Clutter

The dashboard displays many impressive-looking numbers that no decision depends on, diluting the real signal. The trap is that more metrics feel more rigorous. Report only what informs decisions.

### Color Coding With No Thresholds

Status is green until it suddenly turns red, with no amber warning. The trap is subjective color assignment that provides no early warning. Define objective thresholds.

### Hand-Maintained Competing Reports

Each report is maintained separately, the numbers disagree, and trust erodes. The trap is that manual reporting feels controlled but produces inconsistency. Use a single source of truth.

### No Forecast, Only Current State

The report shows today's position but not the trajectory, so decision-makers cannot tell if the project is recovering or deteriorating. The trap is that current state feels sufficient. Add forecasting.

## Self-Check

- [ ] Is each report designed around the decisions its audience must make, rather than around available data?
- [ ] Do reports lead with health indicators (schedule, cost, scope variance, risk) rather than activity measures?
- [ ] Is all progress reported as variance against the approved baseline, with historical variance preserved through re-baselines?
- [ ] Are top risks and issues visible in the report with their status and mitigation actions, not hidden?
- [ ] Do reports include forecasts (estimate at completion, trend lines) showing trajectory, not just current state?
- [ ] Are report granularity and cadence matched to each audience's decision cycle?
- [ ] Are color-coding thresholds defined objectively and applied consistently, with amber providing early warning before red?
- [ ] Do all reports draw from a single authoritative source of truth rather than competing hand-maintained spreadsheets?
- [ ] Would a sponsor receiving the report know what decision or action it is asking of them?
- [ ] Is bad news surfaced early and honestly, or is it softened until it becomes a crisis?
