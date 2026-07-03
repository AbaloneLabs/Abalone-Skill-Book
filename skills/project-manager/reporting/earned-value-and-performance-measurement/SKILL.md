---
name: earned_value_and_performance_measurement.md
description: Use when the agent is applying earned value management, calculating SPI CPI EAC and VAC metrics, interpreting earned value results, setting up performance measurement baselines, using EVM for forecasting, or explaining cost and schedule performance to stakeholders and sponsors.
---

# Earned Value and Performance Measurement

Earned value management (EVM) is the most rigorous available method for measuring integrated cost and schedule performance. Its power is that it converts progress into a common currency, the budgeted cost of work performed, allowing objective comparison of what was planned, what was spent, and what was earned. Its danger is that the numbers carry a precision that can exceed the underlying data, and that users interpret the metrics without understanding their assumptions and limitations. EVM misapplied gives false confidence; EVM well-applied gives early, objective warning of cost and schedule problems.

The judgment problem is how to set up and use EVM so that its metrics reflect reality, how to interpret the indicators without over-reading them, and how to translate the numbers into forecasts and decisions that stakeholders can act on. Agents tend to apply EVM mechanically, report the numbers without interpretation, and treat the indices as truth rather than as signals that depend on the quality of the underlying estimates.

## Core Rules

### Establish a Performance Measurement Baseline Before Measuring

EVM requires a baseline: the planned value of work distributed across time, the budget at completion, and the schedule of when each work package will earn its value. Without a credible baseline, the variances are meaningless. The baseline must be time-phased, resource-loaded, and approved. Invest in the baseline before attempting to measure performance against it. A weak baseline produces variances that measure baseline error, not performance.

### Define Earned Value at the Work Package Level With Objective Rules

The question "how much value has been earned" must be answered consistently. Use objective rules: 0/100 (nothing until complete), 50/50 (half at start, half at finish), weighted milestones, or percentage complete against discrete criteria. Subjective "percent complete" estimates, where the team guesses how done something is, produce optimistic earned value and mask slippage. Choose rules that match the work and apply them consistently.

### Interpret SPI and CPI Together, Not in Isolation

The Schedule Performance Index (SPI) and Cost Performance Index (CPI) each tell part of the story. SPI below 1 means behind schedule; CPI below 1 means over budget. Together they reveal the pattern: behind and over budget is the worst case; behind but on budget may indicate scope not yet started; on schedule but over budget may indicate rework or cost overrun. Never report an index without interpretation, and always consider both together with the context of what is causing the variance.

### Use EAC and VAC for Forecasting With Stated Assumptions

The Estimate at Completion (EAC) and Variance at Completion (VAC) forecast where the project will land. Different EAC formulas embed different assumptions: that the remaining work proceeds at the planned rate, at the current efficiency, or at some blended rate. State which assumption your forecast uses. An EAC that assumes the current overrun is a one-off is very different from one that assumes the trend continues. Be explicit about the assumption and sensitive to which is realistic.

### Recognize the Limitations of SPI Near Project End

SPI tends toward 1.0 as the project nears completion, because all planned value is eventually earned, regardless of how late. This makes SPI a poor late-project schedule indicator. Near the end, use time-based indicators (days late, forecast completion date) rather than SPI. Knowing when each metric loses meaning is part of using EVM correctly.

### Validate the Underlying Data Before Trusting the Numbers

EVM metrics are only as good as the actual cost data and the earned value assessments that feed them. If actuals are misreported, earned value is guessed, or work packages are poorly defined, the indices are noise. Periodically validate that actuals are accurate, earned value rules are being followed, and the work breakdown is sound. Do not report EVM metrics whose inputs you have not checked.

### Connect EVM Signals to Action, Not Just Reporting

The purpose of EVM is to enable action: a negative variance should trigger investigation, a persistent adverse trend should trigger corrective planning, and a forecast overrun should trigger scope, schedule, or budget conversation. EVM reported without action is academic. Always pair the numbers with the response: what is causing the variance, what is being done, and what decision is needed.

### Use EVM Alongside Qualitative Context

EVM measures cost and schedule efficiency but not quality, risk, scope completeness, or stakeholder satisfaction. A project can have healthy indices and still be heading toward a deliverable no one wants. Complement EVM with qualitative measures of scope, quality, and risk. Do not let the precision of the numbers crowd out the qualitative judgment that determines project success.

## Common Traps

### Baseline Too Weak to Measure Against

The baseline is poorly defined, so variances reflect baseline error rather than performance. The trap is that the numbers look rigorous but measure the wrong thing. Invest in the baseline.

### Subjective Percent Complete

The team guesses percent complete, producing optimistic earned value that hides slippage. The trap is that subjective assessment feels reasonable but is systematically biased. Use objective rules.

### Reporting Indices Without Interpretation

SPI and CPI are reported as numbers with no explanation of cause or implication. The trap is that the audience cannot act on numbers without context. Always interpret.

### EAC With Unstated Assumptions

A single EAC is reported without saying whether it assumes recovery or trend continuation. The trap is that the forecast looks precise but hides a critical assumption. State the assumption.

### Trusting SPI Late in the Project

SPI is reported as meaningful when it is mechanically trending toward 1.0 regardless of lateness. The trap is that the indicator has lost meaning. Switch to time-based measures late in the project.

### Unvalidated Inputs

Actuals are inaccurate or earned value is guessed, but the indices are reported as authoritative. The trap is false precision. Validate the inputs.

### EVM Without Action

Variances are reported but trigger no investigation or corrective action. The trap is measurement without management. Connect every signal to a response.

### Numbers Crowding Out Qualitative Judgment

Healthy indices create confidence while quality, scope, or risk problems go unreported. The trap is that cost and schedule efficiency do not equal project success. Complement EVM with qualitative measures.

## Self-Check

- [ ] Is there a credible, time-phased, approved performance measurement baseline before EVM is applied?
- [ ] Are earned value rules defined objectively at the work package level (0/100, 50/50, milestones) rather than by subjective percent complete?
- [ ] Are SPI and CPI interpreted together with context, rather than reported as isolated numbers?
- [ ] Does every EAC and VAC forecast state the assumption about whether the remaining work proceeds at planned rate, current efficiency, or a blend?
- [ ] Is SPI supplemented by time-based schedule indicators near project end, when SPI loses meaning?
- [ ] Are the underlying actuals and earned value assessments periodically validated for accuracy?
- [ ] Does every adverse EVM signal connect to an investigation, a corrective action, or a decision request?
- [ ] Is EVM complemented by qualitative measures of scope, quality, risk, and stakeholder satisfaction?
- [ ] Can you explain, in plain language, what each reported index means and what is causing it?
- [ ] Would acting on the EVM signals change the project outcome, or are they reported without consequence?
