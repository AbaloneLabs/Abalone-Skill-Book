---
name: real_time_monitoring.md
description: Use when the agent is designing or operating continuous monitoring of controls and transactions, setting alert thresholds, distinguishing normal variation from genuine exceptions, tuning monitoring rules to avoid alert fatigue, or deciding which business processes should be monitored continuously rather than sampled periodically.
---

# Real-Time Monitoring

Continuous monitoring shifts auditing from periodic snapshots toward ongoing observation of transactions and controls as they occur. The promise is seductive: full population coverage, early detection, and reduced reliance on point-in-time sampling. But continuous monitoring fails in practice when alerts are poorly tuned, when monitoring captures noise instead of risk, or when the volume of exceptions overwhelms the capacity to investigate them. A monitoring dashboard that fires thousands of alerts a week teaches users to ignore all of them, and a monitor that never fires because the threshold is too loose provides false assurance that nothing is wrong. The auditor's job is not to build a surveillance system but to target monitoring at the risks that matter and to ensure that detected exceptions are actually resolved.

Use this skill when selecting processes for continuous monitoring, designing monitoring rules, calibrating thresholds, managing alert volumes, or evaluating whether an existing monitoring function is effective. The goal is to ensure monitoring produces actionable intelligence rather than noise, silence, or false confidence.

## Core Rules

### Target Monitoring At Risk, Not At Data Availability

The easiest things to monitor are rarely the most important. Systems generate vast logs of routine activity, and it is tempting to monitor whatever data is readily available. But continuous monitoring should begin from risk, not from data supply.

For each candidate process, assess:

- what is the inherent risk being monitored (fraud, error, override, control bypass, compliance breach)?
- what is the likelihood and impact of the event if undetected?
- how quickly must detection occur to prevent or limit harm?
- is continuous monitoring proportionate, or would periodic sampling suffice?
- what is the cost of monitoring relative to the risk it addresses?

Prioritize monitoring for events that are high-impact, time-sensitive, difficult to detect through periodic testing, or where early detection materially reduces loss. Do not monitor low-risk activity simply because the data exists; it consumes capacity and dilutes attention.

### Distinguish Control Monitoring From Transaction Monitoring

Two distinct modes of continuous monitoring serve different purposes, and conflating them weakens both.

Control monitoring observes whether controls are operating as designed: reconciliations completed on time, approvals present, segregation of duties enforced, access reviews performed. It answers whether the control system is functioning.

Transaction monitoring observes the transactions themselves for anomalies: duplicate payments, unusual amounts, off-hours activity, policy violations, patterns indicative of fraud. It answers whether transactions conform to expectation even when controls may have passed.

A strong continuous monitoring program uses both. Control monitoring tells you the system is working; transaction monitoring catches what slips through. Relying on only one leaves a blind spot: controls can pass while transactions are still anomalous, and clean transactions do not prove controls operated.

### Calibrate Thresholds Deliberately

Thresholds determine whether monitoring is useful. Set them too tight and every normal fluctuation triggers an alert; set them too loose and genuine exceptions pass undetected. Threshold setting is not a one-time configuration but an ongoing calibration informed by data.

Approach threshold calibration by:

- analyzing historical data to understand the baseline distribution of the metric;
- identifying what constitutes normal variation versus a genuine outlier;
- setting initial thresholds and reviewing false-positive and false-negative rates;
- iterating as the business changes, volumes shift, or processes are modified;
- documenting the rationale for each threshold and the data that supports it.

Avoid copying thresholds from another organization or setting them by intuition. A threshold that worked elsewhere may not fit this environment's volumes, risk appetite, or control structure.

### Manage Alert Volume And Avoid Alert Fatigue

Alert fatigue is the most common failure of continuous monitoring. When reviewers face hundreds or thousands of alerts, they begin to clear them without genuine investigation, or they stop looking altogether. The monitoring system becomes a compliance artifact rather than a detection tool.

To keep alert volume manageable:

- aggregate related alerts rather than sending each individually;
- route alerts by type and severity to the right reviewer;
- suppress known, accepted exceptions after documented review;
- tune rules that produce high false-positive rates;
- measure alert disposition: how many are investigated, how many are true exceptions, how many are closed without action and why;
- periodically retire monitoring rules that no longer add value.

A healthy monitoring program should have a meaningful signal-to-noise ratio. If the vast majority of alerts are false positives, the rules need tuning, not more reviewers.

### Ensure Detected Exceptions Are Actually Resolved

Monitoring that detects exceptions but does not drive resolution is surveillance, not control. The value of continuous monitoring is realized only when detected exceptions are investigated, root-caused, and corrected.

For each exception workflow, confirm:

- there is a defined owner responsible for investigation;
- there is a service-level expectation for how quickly alerts are reviewed;
- investigations are documented with findings and disposition;
- genuine exceptions are escalated and remediated;
- recurring exceptions are analyzed for systemic causes;
- metrics track time-to-resolution and recurrence rates.

Review the backlog of unresolved alerts periodically. A growing backlog signals that monitoring capacity is mismatched to alert volume, or that the organization is not committed to acting on what monitoring finds.

### Validate The Monitoring System Itself

A monitoring system is itself a control, and like any control it can fail, be circumvented, or degrade silently. The auditor must periodically validate that the monitoring is functioning as intended.

Validate by:

- testing that monitoring rules are still active and running;
- confirming data feeds are complete and not silently failing;
- injecting test transactions to confirm alerts fire as expected;
- reviewing changes to monitoring rules for unauthorized or unreviewed modifications;
- verifying that excluded or suppressed items were legitimately excluded;
- checking that monitoring covers the full population and not a stale subset.

A monitoring dashboard showing green may simply mean the monitor stopped working. Independent validation prevents false assurance.

### Respect Privacy And Proportionality In Monitoring

Continuous monitoring of employee activity, transactions, or behavior raises privacy and proportionality concerns. Monitoring should be risk-justified, transparent where appropriate, and bounded by policy and law.

Consider:

- is the monitoring proportionate to the risk, or does it surveil routine behavior indiscriminately?
- are employees informed that monitoring occurs, where policy or law requires notice?
- is access to monitoring data restricted to those with a legitimate need?
- is monitoring data retained only as long as necessary and then purged?
- could the monitoring be perceived as targeting individuals rather than patterns?

Monitoring that overreaches can create legal exposure, damage trust, and still fail to detect the risks it was meant to catch.

## Common Traps

### Monitoring Everything And Resolving Nothing

Building a comprehensive monitoring suite that fires constantly, with no capacity to investigate, produces noise and false assurance. Prioritize and tune before expanding scope.

### Thresholds Set Once And Never Revisited

Business volumes, processes, and risk profiles change. Thresholds that were appropriate at launch become obsolete. Schedule periodic recalibration based on current data.

### Treating A Clean Dashboard As Evidence Of Control

A dashboard with no alerts may mean all is well, or it may mean the monitor is broken, the data feed failed, or the threshold is too loose. Validate the monitoring system independently.

### Confusing Alert Volume With Effectiveness

A high alert count is not a sign of strong monitoring; it is often a sign of poor tuning. Measure outcomes, not activity.

### Monitoring Controls But Ignoring The Transactions They Miss

Control monitoring confirms controls ran; it does not confirm transactions were correct. Complement control monitoring with transaction anomaly detection.

### Suppressing Exceptions Without Documentation

Suppressing known, accepted exceptions is legitimate, but undocumented suppression hides risk. Require a rationale, an approver, and a review date for every suppression.

### Relying On Vendor Defaults

Monitoring tools ship with default rules and thresholds designed for generic environments. They rarely match this organization's risk profile, volumes, or control structure. Customize and validate.

## Self-Check

- Is continuous monitoring targeted at genuine, high-impact, time-sensitive risks rather than at whatever data happens to be available?
- Does the program include both control monitoring and transaction monitoring, recognizing the different questions each answers?
- Are thresholds set from baseline data analysis, documented, and periodically recalibrated as conditions change?
- Is alert volume actively managed through aggregation, routing, tuning, and suppression to prevent alert fatigue?
- Is there a defined workflow ensuring detected exceptions are investigated, resolved, and tracked to closure with measurable service levels?
- Is the monitoring system itself periodically validated through data-feed checks, test transactions, and rule-change review?
- Are privacy, proportionality, notice, and data-retention considerations addressed in the monitoring design?
- Do metrics measure true-positive rates, time-to-resolution, and recurrence rather than merely alert counts?
- Has the program avoided vendor defaults by customizing rules and thresholds to this environment?
- Could an independent reviewer confirm that monitoring produces actionable intelligence rather than noise or false assurance?
