---
name: capacity-and-workload-planning.md
description: Use when the agent is planning operational capacity, staffing levels, workload allocation, service coverage, queue capacity, shift demand, handling time assumptions, productivity, or whether a team can reliably meet expected work volume without hidden backlog, burnout, quality loss, or service failure.
---

# Capacity And Workload Planning

Capacity planning is not a headcount guess. It is the translation of demand, service promises, process constraints, skill availability, and variability into a realistic operating plan. Agents often calculate average work volume, divide by average handling time, and miss the details that break real operations: peaks, shrinkage, rework, handoffs, escalations, interruptions, training, and the recovery capacity needed when the plan is wrong.

Use this skill before recommending staffing levels, queue coverage, weekly schedules, workload allocation, service-level commitments, outsourcing, overtime, or automation as a capacity answer. The agent should produce a plan that can survive actual demand behavior, not only a spreadsheet that balances on average.

## Core Rules

### Model demand shape, not only demand total

Start by separating total volume from arrival pattern. Capacity must cover when work arrives, not only how much work exists over a month. Review hourly, daily, weekly, monthly, seasonal, promotional, release-driven, and event-driven patterns. Include backlog carryover, reopening work, batch arrivals, deadline-driven demand, and work that clusters after upstream teams act.

Do not hide peaks in averages. A team may be able to complete 1,000 cases per week and still fail if 500 arrive on Monday morning or if specialized cases arrive after the only qualified person leaves for the day. Model at the smallest time bucket that matters for the service promise.

### Separate work types by effort and skill

Do not treat all units of work as identical. Split work into types that have materially different handling times, complexity, risk, skill requirement, approval need, documentation burden, or customer impact. Include simple transactions, research-heavy cases, exceptions, escalations, quality review, rework, customer communication, data correction, and supervisor approvals.

Use ranges rather than single handling-time numbers when variability is high. Averages can be useful for long-run planning, but staffing decisions need percentiles, outliers, and mix sensitivity. A small increase in complex work can consume more capacity than a large increase in simple work.

### Convert scheduled hours into productive capacity

Scheduled hours are not available production hours. Account for breaks, meetings, training, coaching, onboarding, planned leave, sick time, holidays, system downtime, tool switching, administrative work, quality review, incident response, and cross-functional commitments. This loss is not waste by default; it is part of operating the team.

Be explicit about shrinkage assumptions. If a plan assumes 40 scheduled hours equal 40 queue hours, the plan is already wrong. New hires, temporary staff, and cross-trained backups may also produce less capacity during ramp or when they handle unfamiliar work.

### Match capacity to service promises

Capacity should be planned against the service promise, not against a vague desire to "keep up." Define response time, resolution time, fulfillment time, error tolerance, escalation time, operating hours, and priority tiers. A same-day promise requires different capacity than a weekly processing target even if total volume is identical.

If the current service promise cannot be met with available capacity, do not bury the conflict. Name the options: reduce promise, add staff, extend hours, defer lower-priority work, simplify process, automate safely, outsource, change intake rules, or accept backlog with customer communication.

### Include quality and rework in the plan

Capacity planning that ignores quality creates false capacity. If staff are overloaded, errors, escalations, and rework often rise. Include time for quality checks, peer review, supervisor review, audit response, defect correction, customer recovery, and root-cause improvement.

Do not assume that pushing more work through the same team has no quality cost. A plan that clears volume while increasing rework may only move backlog into the future.

### Identify bottlenecks and constraints

Capacity is often constrained by something other than total headcount. Check specialized skills, licenses, system permissions, supervisor approvals, language coverage, location coverage, equipment, tools, vendor turnaround, downstream capacity, physical space, or customer availability. The limiting resource should drive the plan.

If one approver, queue, tool, or handoff limits throughput, adding general staff may not improve service. The plan should show where additional capacity actually changes output.

### Plan for variability and recovery

Every capacity plan needs an answer for variance. Define buffers, overtime rules, overflow support, cross-training, temporary labor, vendor support, backlog burn-down, priority changes, and communication triggers. A plan without recovery logic turns normal variability into crisis management.

Recovery capacity should be real. If the team is already fully utilized, there is no reserve for spikes, absences, incidents, or quality failures. High utilization can look efficient while making the operation brittle.

### Make tradeoffs visible to decision makers

Capacity choices trade cost, speed, quality, employee load, customer experience, and risk. Show the consequence of each option. For example, lower staffing may save budget but increase backlog and churn; overtime may protect this week's service but create fatigue; automation may reduce simple work but leave a higher-complexity mix for staff.

Use scenarios rather than one "right" number where uncertainty is material. Leadership should see base, high-demand, low-demand, and disruption cases with the resulting service risk and cost.

### Monitor leading indicators after the plan starts

Capacity planning is not complete when the schedule is published. Track arrivals versus forecast, completions, aging, reopen rate, error rate, backlog mix, utilization, overtime, absenteeism, abandonment, escalations, and customer complaints. Define thresholds that trigger action.

If the plan is failing, revise assumptions quickly. Do not wait until the end of the month to discover that backlog aging, staff fatigue, or quality drift has already harmed customers.

## Common Traps

- Dividing total demand by average handling time and calling the result staffing need.
- Planning to monthly or weekly volume while ignoring hourly and daily peaks.
- Treating scheduled hours as productive hours without shrinkage, ramp, meetings, training, and interruptions.
- Assuming all work items require the same time, skill, approval, and quality control.
- Adding general headcount when the bottleneck is senior review, permissions, tooling, vendor turnaround, or downstream capacity.
- Setting service promises before checking whether staffing and recovery capacity can support them.
- Using overtime as normal capacity until quality falls or staff burn out; ignoring rework, reopenings, escalations, and defect correction in workload estimates
- Failing to define what will be paused or deprioritized when demand exceeds capacity; reporting capacity as a single confident number when demand and productivity assumptions are uncertain

## Self-Check

- Is demand modeled by arrival pattern, channel, work type, location, customer segment, and peak period rather than only total volume?
- Are handling-time assumptions separated by complexity, skill, approval, documentation, rework, and escalation burden?
- Are scheduled hours converted into productive capacity after shrinkage, ramp, meetings, breaks, training, leave, interruptions, and non-queue work?
- Are service promises, priority tiers, operating hours, and response or resolution targets explicit?
- Does the plan include quality review, defect correction, rework, customer recovery, and improvement work?
- Are bottlenecks such as senior review, system access, equipment, language, vendor, downstream capacity, or physical space identified?
- Is there a recovery plan for spikes, absences, incidents, forecast error, and backlog burn-down?
- Are tradeoffs across cost, service, quality, staff load, overtime, automation, outsourcing, and customer communication visible?
- Are leading indicators and action thresholds defined after the plan goes live?
- Can the proposed capacity plan work under realistic variability, not only under average conditions?
