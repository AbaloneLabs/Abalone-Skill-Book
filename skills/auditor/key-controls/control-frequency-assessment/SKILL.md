---
name: control-frequency-assessment.md
description: Use when the agent is assessing the frequency at which a control operates, determining how frequency affects the number of testing instances required, distinguishing daily/weekly/monthly/quarterly/annual controls, planning sample sizes across the period, or deciding whether a control operates "frequently enough" to support reliance across the full audit period.
---

# Control Frequency Assessment

A control's frequency determines how many times it operates during the audit period, which in turn drives how many instances must be tested to support reliance across the whole period. It also determines how large a window of uncontrolled risk exists between operations. Misjudging frequency leads to two opposite errors: under-testing a control by treating an infrequent control as frequent (too few instances, gaps in coverage), and over-testing by treating a frequent control as infrequent (wasted effort). Frequency is also a design question — a control that operates too infrequently for the risk it addresses is weak by design, however well each instance is performed.

## Core Rules

### Classify frequency by actual operation, not by policy

The control's frequency for testing purposes is how often it actually operates, not how often the procedure document says it should. Confirm frequency from operating evidence (dates on reviews, run schedules of reports, reconciliation log) rather than from the policy. A control documented as "monthly" that was actually performed quarterly for half the year is a quarterly control for testing purposes, and the gap is itself a deviation. Record the actual operating frequency and reconcile it to the documented frequency; unexplained gaps are evidence of inconsistent operation.

### Use frequency to determine the required number of testing instances

Frequency drives sample size for tests of controls because reliance must cover the whole period, not just the tested instances. A common framework:

- **Daily controls**: test a sample across the period (commonly a minimum number of instances, e.g., 25–40, spread across months and operators).
- **Weekly controls**: fewer instances than daily (e.g., 15–25).
- **Monthly controls**: test a number of months (e.g., a sample of months, often with a minimum).
- **Quarterly controls**: test each quarter (often a minimum of one instance per quarter, sometimes more).
- **Annual or one-time controls**: test the single instance, or the few instances, with particular attention to whether it covered the full period.

Apply the methodology's specific minimums; the principle is that fewer operations require proportionally higher coverage to support period-wide reliance, with one-time controls requiring examination of the single instance in depth.

### Ensure tested instances span the entire period, including period-end

Frequency-based sampling is only valid if the instances span the period. A sample of 25 daily-control instances all drawn from the first half of the year provides no evidence about the second half. Deliberately spread instances across:

- all quarters or halves of the period;
- different months, including high-risk months (close, year-end, peak volume);
- different operators, where the control is manual;
- different locations or systems, where applicable.

Period-end is especially important: controls sometimes degrade under close pressure, so include instances at or near the balance sheet date.

### Assess whether the frequency matches the risk

Frequency is a design dimension, not just a testing input. A control that operates too infrequently for the risk it addresses is weak regardless of how well each instance runs. Evaluate:

- How quickly could a material error accumulate between operations of this control?
- Is the control's frequency fast enough to catch errors before they become material or before they are concealed?
- Does the frequency match the volatility and volume of the underlying process?

A monthly reconciliation of a high-volume daily cash process leaves up to 30 days of uncontrolled risk; a quarterly review of a volatile estimate may miss large intra-quarter swings. Where frequency is mismatched to risk, raise the design concern and consider whether the control can be relied upon at all.

### Distinguish controls that operate on a schedule from event-driven controls

Some controls operate on a fixed schedule (daily, monthly); others are event-driven (operate each time a triggering event occurs — each journal entry above a threshold, each new vendor, each system change). Event-driven controls may operate many times or few times depending on event volume. For event-driven controls:

- determine the population of triggering events in the period;
- sample from that population using the same period-spanning logic;
- confirm the control actually fired for each triggering event, not just on a schedule.

A common error is treating an event-driven control as if it operated on a fixed schedule, missing instances where the event occurred but the control did not fire.

### Handle controls whose frequency changed during the period

Processes are re-engineered, controls are added or dropped, and frequencies are adjusted. Where a control's frequency changed mid-period (e.g., a monthly review became weekly), treat it as effectively two controls and test each segment appropriately, or test at the lower (less frequent) frequency for the period as a whole if reliance must span the change. Document the change and its effect on the assessed risk and testing approach; a frequency increase mid-period may indicate a problem identified and partially remediated, which is relevant context.

### Factor frequency into the control's ability to support period-end assertions

For assertions tied to period-end balances (existence of receivables, valuation of inventory, completeness of liabilities), the control must operate at or sufficiently near period-end to be relevant. A control that last operated two months before year-end provides weak support for a year-end assertion, regardless of how often it ran earlier. Confirm the control operated at the points that matter for the specific assertion, and supplement with substantive procedures at period-end where the control's timing leaves a gap.

### Use frequency to calibrate the nature of testing, not just the extent

Higher-frequency controls, tested across many instances, support statistical or quasi-statistical conclusions about the population. Low-frequency and one-time controls cannot support population-level conclusions from a few instances; for these, the testing should be more in-depth per instance (examine the single operation thoroughly, re-perform, corroborate) rather than relying on numbers. Match the nature of testing to what the frequency can support.

## Common Traps

- **Using documented frequency instead of actual operating frequency**, under-testing controls that ran less often than policy states.
- **Drawing all sample instances from one part of the period**, leaving other quarters or the period-end untested.
- **Treating an event-driven control as a scheduled control**, missing instances where the triggering event occurred without the control firing.
- **Accepting a frequency that is mismatched to the risk** (e.g., monthly review of a daily high-volume process) without raising it as a design weakness.
- **Failing to test each quarter for quarterly controls**, or each instance for one-time controls, leaving gaps in period-wide coverage.
- **Ignoring period-end operation** for controls that support period-end assertions, leaving a timing gap.
- **Not adjusting testing when frequency changed mid-period**, either over- or under-testing one segment.
- **Applying population-level sampling logic to one-time or low-frequency controls** where a few instances cannot support a statistical conclusion, instead of deepening the per-instance testing.
- **Concentrating samples on a single operator or location** for a manual control, missing variability that defines manual control risk.

## Self-Check

- Did I confirm the control's actual operating frequency from evidence, and reconcile it to the documented frequency, treating gaps as deviations?
- Did I determine the number of testing instances using the methodology appropriate to the control's frequency?
- Do my tested instances span the entire period, including all quarters, high-risk months, and the period-end?
- Is the control's frequency matched to the risk it addresses — fast enough to catch errors before they become material or are concealed?
- For event-driven controls, did I define the triggering-event population and sample from it, confirming the control fired for each sampled event?
- Where frequency changed mid-period, did I test each segment appropriately and document the change's effect?
- For period-end assertions, did I confirm the control operated at or near the balance sheet date, and supplement with substantive procedures if there is a timing gap?
- For low-frequency or one-time controls, did I deepen the per-instance testing rather than rely on instance counts that cannot support a population conclusion?
