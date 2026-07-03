---
name: evm_indices_and_variance_interpretation.md
description: Use when the agent is interpreting SPI CPI schedule variance cost variance and variance trends, diagnosing what indices reveal about project health, deciding whether a variance is real or timing, or avoiding misreading EVM signals near project end.
---

# EVM Indices And Variance Interpretation

Earned value indices condense a project's cost and schedule health into a few ratios, and that condensation is exactly why they are misused. The Schedule Performance Index (SPI), Cost Performance Index (CPI), Schedule Variance (SV), and Cost Variance (CV) are signals, not verdicts. They tell you that something is diverging from plan; they do not tell you why, whether it will persist, or whether the underlying data even deserves trust. Agents tend to read an index below one as "the project is failing" and an index near one as "all is well," then report the number without interpretation. Both reactions flatten the signal into noise.

The judgment problem is how to read indices together rather than in isolation, how to distinguish a real performance problem from a timing or data artifact, how to weight trend over single-period value, and how to know when an index has lost meaning, especially SPI near project end. Interpretation is the skill; the arithmetic is trivial.

## Core Rules

### Read SPI And CPI Together As A Pattern, Not As Separate Numbers

SPI (EV divided by PV) and CPI (EV divided by AC) each describe one dimension. Their combination reveals the project's situation. SPI below one and CPI below one means behind schedule and over budget, the worst case, often indicating rework or underestimation. SPI below one with CPI near one may mean scope started late but is executing efficiently. SPI near one with CPI below one often means work is being done on time but at higher cost than planned. Never report one index without the other, and never report either without naming the pattern they form together.

### Treat Variance As A Symptom That Requires A Cause

SV (EV minus PV) and CV (EV minus AC) are symptoms, not diagnoses. A negative schedule variance could mean work is genuinely late, or that the baseline was front-loaded, or that a dependency slipped, or that the team is reworking a deliverable. A negative cost variance could mean real overrun, an accrual timing difference, a scope change not yet baselined, or miscoded actuals. Before drawing any conclusion, ask what is causing the variance and whether it will persist. A variance without an investigated cause is a number waiting to mislead.

### Weight Trend Over Single-Period Value

A single period's index can be distorted by a late invoice, a milestone slipping across a boundary, or a one-time rework charge. The trend across several periods is far more informative. Look at the direction and acceleration: is CPI drifting down gradually, or did it cliff-drop? A gradual decline suggests a systemic efficiency problem; a cliff-drop suggests a discrete event. Establish the trend before deciding whether to act, and define thresholds that trigger investigation based on sustained movement, not a single noisy point.

### Distinguish Real Performance From Timing And Data Artifacts

Not every variance is real performance. Accrual timing can make a period look over budget when cost was simply recognized late. Front-loaded or back-loaded baselines can make early periods look ahead or behind for arithmetic reasons. Reclassifications between capital and expense, or between work packages, can manufacture variance that has no operational meaning. Investigate whether the variance will reverse, persist, or compound. Reporting a timing artifact as an overrun, or a real overrun as timing, both corrupt decisions.

### Know When SPI Loses Meaning

SPI mechanically trends toward 1.0 as a project nears completion, because all planned value is eventually earned regardless of how late. This means SPI is a poor late-project schedule indicator: a project six months late can show SPI climbing back toward one as the last work packages earn their value. Once you are in the final third of the project, switch to time-based schedule measures, forecast completion date and days late, rather than SPI. Knowing when a metric stops working is part of using it correctly.

### Interpret Indices In The Context Of Scope And Quality

EVM indices measure cost and schedule efficiency, not whether the work is correct, complete in intent, or wanted. A project can show healthy CPI and SPI while accumulating technical debt, shipping the wrong scope, or storing up rework. A cost underrun may reflect delayed work rather than efficiency. Always read indices alongside scope completion, quality metrics, and open defects. Do not let the precision of the index crowd out the qualitative judgment that determines whether the project is actually succeeding.

### Attribute Variance To Its Source Before Reporting

A useful variance report does not just state the number; it attributes it. Is the variance concentrated in one control account, one team, one work package, or one external dependency? Distributed small variances across many packages suggest systemic estimation error; a single large variance in one account suggests a discrete problem. Roll down from the project-level index to the accounts driving it before reporting, so that the explanation is specific and actionable rather than vague.

### Define Thresholds That Trigger Investigation, Not Panic

Establish tolerance bands that separate normal noise from signals worth investigating. A CPI of 0.98 within tolerance may warrant monitoring; a CPI below 0.90, or a sustained downward trend, warrants cause analysis and a response. The thresholds should be agreed in advance so that reactions are consistent rather than emotional. Acting on every micro-variance wastes effort; ignoring a sustained adverse trend surrenders the chance to recover.

## Common Traps

### Reporting An Index Without Interpretation

Publishing SPI 0.85 with no explanation of cause or implication leaves the audience unable to act. The trap is that the number looks rigorous while conveying nothing useful. Always pair the index with its pattern, cause, and implication.

### Reading SPI And CPI In Isolation

Treating each index separately hides the combined pattern that reveals the real situation. The trap is that two individually ambiguous numbers become clear only when read together. Report and interpret them as a pair.

### Treating A Single-Period Spike As A Trend

Reacting to one noisy period over-responds to artifacts like late invoices. The trap is wasted corrective effort chasing phantom problems. Look at the trend across periods first.

### Trusting SPI Late In The Project

Reporting SPI climbing toward one as recovery, when it is mechanically driven by earned value catching up, gives false comfort. The trap is that the project looks like it is healing while still finishing late. Switch to time-based measures near the end.

### Confusing Timing Artifacts With Real Overruns

An accrual or reclassification can look exactly like a cost overrun. The trap is launching a recovery plan for a variance that will reverse on its own. Investigate the cause before acting.

### Ignoring Quality And Scope Behind Healthy Indices

Good CPI and SPI can coexist with technical debt, wrong scope, or stored rework. The trap is that cost and schedule efficiency mask a project heading toward a deliverable no one wants. Read indices with quality and scope context.

### Reporting Project-Level Variance Without Rolling Down

A single rolled-up number hides which account is driving it. The trap is that the report names a problem but not its source, so no one knows where to act. Attribute variance to its control account.

### Acting On Every Micro-Variance Or None At All

Without agreed thresholds, the team either overreacts to noise or learns to ignore all variance. The trap is that both extremes destroy the signal value of EVM. Define and respect tolerance bands.

## Self-Check

- [ ] Are SPI and CPI reported and interpreted together as a combined pattern, not as separate isolated numbers?
- [ ] Does every reported schedule or cost variance carry an investigated cause and a judgment on whether it will persist?
- [ ] Is the multi-period trend weighed before reacting, rather than a single noisy period?
- [ ] Have timing artifacts, accrual differences, and reclassifications been ruled out before declaring a real performance variance?
- [ ] Is SPI supplemented by time-based schedule measures once the project enters its final third, when SPI loses meaning?
- [ ] Are indices read alongside scope completion, quality metrics, and open defects rather than in isolation?
- [ ] Is project-level variance rolled down to the specific control account or work package driving it before reporting?
- [ ] Are agreed tolerance thresholds documented and used to separate normal noise from signals requiring investigation?
- [ ] Can you explain, in plain language, what each reported index means, what is causing it, and what action it implies?
- [ ] Has the interpretation avoided treating an index near one as automatic health when scope or quality context contradicts it?
