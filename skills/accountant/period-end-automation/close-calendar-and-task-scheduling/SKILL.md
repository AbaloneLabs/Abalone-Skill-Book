---
name: close_calendar_and_task_scheduling.md
description: Use when the agent is building or optimizing a financial close calendar, scheduling close tasks across entities and time zones, configuring task dependencies and critical paths, or reducing close cycle time through better sequencing and front-loading of period-end work.
---

# Close Calendar And Task Scheduling

The close calendar is the operational backbone of the financial close. It defines what must happen, in what order, by when, and by whom, across every entity, subledger, and time zone the organization operates in. A well-designed close calendar transforms the close from a scramble into a managed sequence. A poorly designed one guarantees slippage, because a single late task on the critical path delays every dependent task downstream. The calendar is not merely a list of deadlines; it is a dependency map, a resource plan, and a risk management tool. Treating it as a simple date list is why so many closes finish late despite everyone working hard.

Use this skill before building or optimizing a close calendar, scheduling close tasks, configuring dependencies, or reducing close cycle time. The goal is to prevent the agent from building a calendar that ignores dependencies, that concentrates work at period end, or that sets target dates without considering the critical path.

## Core Rules

### Map The Close As A Dependency Network, Not A Date List

The close is a network of dependent tasks. A task cannot start until its inputs are ready. Design the calendar around dependencies first, dates second.

Map dependencies so that:

- subledger close precedes GL reconciliation for that subledger;
- bank and intercompany reconciliation precedes consolidation;
- accrual and estimate finalization precedes pre-close trial balance review;
- review gates precede statement preparation;
- entity close precedes group consolidation;
- tax provision inputs precede tax entry finalization.

A calendar that schedules a reconciliation before the subledger closes guarantees rework. The dependency map is the source of truth; the calendar is its projection onto time.

### Identify And Protect The Critical Path

The critical path is the longest sequence of dependent tasks that determines the earliest possible close date. Tasks on the critical path directly determine close duration; tasks off it have float.

Protect the critical path by:

- identifying which tasks are on it and which have float;
- assigning the most reliable resources and the strongest controls to critical-path tasks;
- avoiding scheduling non-critical work that competes for the same resources;
- monitoring critical-path tasks daily during the close;
- building contingency for critical-path tasks most likely to slip, such as complex estimates.

A close slips when a critical-path task slips. A close does not slip when a non-critical task with float slips, unless no one recognizes the float is gone.

### Front-Load Tasks That Do Not Require Final Period Data

Many close tasks can be performed before period end, reducing the work that piles up in the days after period end. Front-loading is the single most effective close acceleration technique.

Front-load tasks such as:

- pre-close account reconciliations for stable accounts;
- preliminary accruals based on prior-period trends;
- inventory counts scheduled before period end;
- fixed asset depreciation runs;
- consolidation structure validation and mapping;
- draft financial statement templates and report packages.

Front-loading shifts effort from the constrained post-period-end window to the less constrained pre-period window, shortening the critical path without cutting any task.

### Sequence Across Entities, Currencies, And Time Zones

For multi-entity organizations, the calendar must sequence across entities, currencies, and time zones. Local closes feed the group close.

Sequence by:

- closing entities in local currency before translating to group currency;
- closing subsidiaries before the parent consolidates them;
- leveraging time zone differences, such as closing Asia-Pacific entities first;
- coordinating intercompany matching before consolidation;
- aligning the group close calendar with the slowest entity's realistic close.

A group close that assumes all entities can close on the same day ignores the reality that some entities are larger, more complex, or in earlier time zones. Design the group calendar around the real constraint.

### Assign Realistic Target Days Based On Actual Capability

Target days must reflect what the team can actually achieve, not what leadership wishes. Unrealistic targets guarantee slippage and demoralize the team.

Set realistic targets by:

- basing target days on historical actual close days, not aspiration;
- accounting for known constraints such as system batch windows and staffing;
- building in buffer for high-risk tasks such as complex estimates;
- reviewing actual versus target after each close and adjusting;
- distinguishing the target close day from the statutory deadline, leaving margin.

A target that is consistently missed is not a target; it is fiction. Calibrate to reality and then improve reality through process change.

### Build The Calendar Around Business Days, Not Calendar Days

Close calendars should be expressed in business days relative to period end, such as BD+1 or BD+3, not in calendar dates that shift each period.

Use business-day notation to:

- create a reusable template that applies to every period;
- account for weekends and holidays that shift the working calendar;
- communicate timing in a way that is stable across periods;
- compare close performance across periods on a like-for-like basis.

A calendar expressed in absolute dates must be rebuilt every period and invites error. Business-day notation makes the calendar a stable, reusable artifact.

### Incorporate Review Gates And Approvals Into The Schedule

Review and approval are tasks that consume time. They must be scheduled, not assumed.

Schedule gates by:

- assigning a target day and an owner to each review, such as reconciliation review at BD+3;
- ensuring reviewers are available and not over-scheduled during the close;
- building in time for rework if review identifies issues;
- sequencing approvals so that they do not become the bottleneck.

A close calendar that lists task completion but not review assumes review takes no time. In reality, review and rework often determine the actual close day.

### Maintain The Calendar As A Living Document And Review After Each Close

The close changes. New entities, new systems, new standards, and process improvements all require calendar updates. The calendar must be maintained and reviewed.

Update the calendar when:

- actual close days consistently differ from targets, indicating the calendar is wrong;
- new entities, accounts, or standards are added;
- tasks are automated, eliminated, or front-loaded;
- a close review identifies a bottleneck or a gap;
- staffing or system changes alter capability.

Review actual versus target after every close. A calendar that is never calibrated to reality will drift and lose its value as a management tool.

### Acknowledge Framework And Professional Limits

The close calendar implements accounting processes that must comply with the applicable reporting framework, tax deadlines, and regulatory reporting requirements. Tasks involving revenue, leases, hedging, intercompany, consolidation, and tax often have framework-specific timing and dependency requirements. Confirm significant close calendar design decisions with qualified accounting professionals, and validate that the scheduled close produces framework-compliant results within statutory deadlines. Do not treat the calendar as purely operational scheduling; it is the operational expression of accounting and reporting obligations.

## Common Traps

### Date List Instead Of Dependency Map

A calendar that ignores dependencies schedules tasks before their inputs are ready and guarantees rework.

### Ignoring The Critical Path

A close slips when a critical-path task slips; non-critical tasks with float do not determine the close day.

### Everything Piled At Period End

Failing to front-load concentrates all work in the constrained post-period window and lengthens the close.

### Unrealistic Target Days

Targets based on aspiration rather than actual capability are consistently missed and demoralize the team.

### Same-Day Assumption For All Entities

A group close that assumes all entities close on the same day ignores size, complexity, and time zone realities.

### Review Not Scheduled

A calendar that lists task completion but not review assumes review takes no time and underestimates the close day.

### Calendar Never Calibrated

A calendar that is never reviewed against actuals drifts from reality and loses its management value.

### Calendar Treated As Pure Scheduling

The close calendar is the operational expression of accounting and reporting obligations; confirm with professionals.

## Self-Check

- Is the close mapped as a dependency network, with tasks sequenced so that inputs are ready before dependent tasks start?
- Is the critical path identified, protected with reliable resources and strong controls, and monitored daily during the close?
- Are tasks that do not require final period data front-loaded to reduce post-period-end work?
- Does the calendar sequence across entities, currencies, and time zones, leveraging differences and coordinating intercompany?
- Are target days based on historical actual capability with buffer for high-risk tasks, not on aspiration?
- Is the calendar expressed in business days relative to period end for stability across periods?
- Are review gates and approvals scheduled with target days, owners, and rework time?
- Is the calendar maintained and calibrated against actuals after each close, and updated for changes?
- Does the scheduled close reflect framework, tax, and regulatory obligations confirmed with qualified professionals?
