---
name: demand-signal-quality-and-assumption-review.md
description: Use when the agent is reviewing the reliability of demand signals, validating assumptions behind volume forecasts, comparing leading indicators, checking demand data quality, assessing forecast bias, or deciding whether operational plans should trust a demand signal.
---

# Demand Signal Quality And Assumption Review

Demand signals are only useful when their source, bias, timing, and relationship to actual workload are understood. Agents often treat dashboards, pipeline, campaign plans, bookings, historical averages, or stakeholder estimates as equally reliable. Operations then staffs against signals that may be stale, inflated, delayed, duplicated, or unrelated to the work that actually arrives.

Use this skill before trusting a demand signal for staffing, capacity, vendor, inventory, or service planning. The goal is to test whether the signal is strong enough for the operating decision it will drive.

## Core Rules

### Identify the signal source and incentive

Every signal has a source and an incentive. Sales pipeline may be optimistic, finance forecast may smooth volatility, marketing plans may reflect campaign ambition, customer success warnings may overrepresent large accounts, and historical averages may miss upcoming change.

Ask who produced the signal, how it was measured, what they optimize for, and what would make it biased. This is not about distrust; it is about knowing how much operational weight the signal can carry.

### Check whether the signal predicts workload

Some signals predict demand, but not operational workload. Website traffic may not predict support tickets. Bookings may not predict onboarding complexity. Product usage may not predict billing contacts. A campaign may create leads but not fulfillable work.

Validate the historical relationship between the signal and the work operations must handle. Use conversion rates, lag times, segment differences, and confidence ranges.

### Review data freshness and definition

Check when the data was last updated, what is included, what is excluded, how duplicates are handled, and whether definitions changed. A "case," "order," "active user," "launch," or "qualified lead" may mean different things across teams.

Data definition drift can look like demand change. Do not adjust capacity until the measurement itself is understood.

### Test assumptions explicitly

List the assumptions behind the signal: adoption rate, contact rate, conversion, cancellation, return rate, defect rate, staffing productivity, vendor capacity, tool uptime, customer behavior, and seasonality. Decide which assumptions are proven, inferred, or speculative.

Assumptions should be owned. If a plan depends on a product team assumption, campaign assumption, or vendor assumption, name the owner and update cadence.

### Compare multiple signals

Use more than one signal where possible: historical actuals, leading indicators, pipeline, scheduled events, customer commitments, product telemetry, marketing calendar, vendor notices, incident trends, and frontline observations. Conflicting signals should trigger investigation rather than averaging.

Frontline signal matters. Staff may see early patterns in tickets, customer questions, or exceptions before dashboards catch up.

### Account for lag and timing

Demand signals often precede workload by hours, days, weeks, or months. Model lag between trigger and operational arrival. A product launch may create support contacts immediately, billing questions at renewal, and cancellation risk later.

If lag is misunderstood, operations may add capacity too early, too late, or in the wrong function.

### Detect duplicates and hidden demand

Signals can double-count demand across systems or miss demand that bypasses official channels. Check for duplicate tickets, reopened cases, multiple orders for one customer event, manual work not logged, shadow spreadsheets, email requests, vendor queues, and backlog outside the main tool.

Operational demand lives where work happens, not only where dashboards look.

### Set confidence and contingency

After reviewing signal quality, assign confidence and decide how much of the plan should depend on it. Low-confidence signals may justify monitoring, flexible staffing, phased commitments, or leadership risk acceptance rather than firm hiring or service promises.

Do not use a weak signal to justify a precise plan. Match plan rigidity to signal quality.

### Review signal performance after actuals

After the period, compare signal to actual workload and service outcomes. Update conversion rates, lags, assumptions, and confidence. This turns demand planning into a learning system.

If the same signal repeatedly misleads operations, change how it is used or require a stronger source.

## Common Traps

- Treating all demand signals as equally reliable because they appear in dashboards.
- Ignoring incentives that make signals optimistic, conservative, delayed, or incomplete.
- Using a signal that predicts commercial activity but not operational workload.
- Missing definition changes that make trends look like real demand change.
- Averaging conflicting signals instead of investigating why they differ.
- Ignoring lag between trigger and workload arrival.
- Missing demand in informal channels, shadow queues, or vendor systems; double-counting the same customer event across multiple systems
- Using low-confidence signals to make high-commitment staffing or service decisions; failing to compare signals to actuals and improve the model

## Self-Check

- Is the signal source, owner, incentive, and possible bias understood?
- Does the signal historically predict the operational workload being planned?
- Are data freshness, definitions, inclusions, exclusions, and duplicate rules clear?
- Are key assumptions listed with owners and confidence levels?
- Have multiple signals been compared, including frontline observations where relevant?
- Are lag times between signal and workload modeled?
- Are hidden channels, shadow queues, vendor queues, reopens, and manual work considered?
- Are duplicate signals and repeated customer events controlled?
- Is plan commitment proportional to signal confidence?
- Will signal performance be reviewed against actual workload and service outcomes?
