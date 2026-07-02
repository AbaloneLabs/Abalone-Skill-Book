---
name: compliance_control_testing_design.md
description: Use when the agent is designing compliance control tests, defining sampling approaches, performing attribute testing, establishing population completeness, or deciding how to prove a control actually operates as designed.
---

# Compliance Control Testing Design

Control testing is the act of obtaining evidence that a control operated effectively during a defined period. It is distinct from risk assessment, which decides what to test, and from monitoring, which runs continuously. A control test that samples the wrong population, applies the wrong attribute, or tests only whether an approval stamp exists rather than whether the approval was meaningful will produce a false conclusion of effectiveness. Regulators and auditors routinely reject conclusions built on weak test design.

Use this skill before designing a test plan, defining sample sizes, selecting items, writing test steps, or concluding on control operation. The goal is to make the agent treat test design as an evidence-gathering exercise with defined populations, attributes, tolerances, and documentation, not as a box-checking walkthrough.

## Core Rules

### Tie Every Test To A Specific Control Objective And Risk

A test exists to answer a question about a control. Do not test a control because a framework lists it. State, for each test:

- the control being tested and its unique identifier;
- the control objective and the risk it mitigates;
- the control type (preventive, detective, corrective, manual, automated, IT-dependent manual);
- the control owner and who performs it;
- the frequency of operation (per transaction, daily, weekly, monthly, quarterly, annual);
- the assertion being tested (occurrence, completeness, accuracy, authorization, cutoff, classification, restriction).

If the control objective is vague, the test will be vague. "Ensure gifts are approved" is not a control objective; "prevent any gift above the approval threshold from being processed without documented approval from the compliance officer" is.

### Define And Reconcile The Complete Population

Population completeness is the most common failure in control testing. A sample drawn from an incomplete population cannot support a conclusion about the control.

For each test, obtain the complete population of transactions or events the control is supposed to govern, then reconcile it:

- source system and report used to generate the population;
- date range covered and how it maps to the testing period;
- filters applied and their justification;
- total record count, total monetary value, and other relevant totals;
- reconciliation to a control total, ledger, or independent source;
- treatment of voided, reversed, deleted, manual, and exception items;
- treatment of items processed outside the normal system;
- identification of any items that should have been in the population but were not.

Test for completeness by comparing the population total to an independent source. If the population cannot be reconciled, document the limitation and avoid a clean conclusion. A missing slice of the population is often where the risk lives.

### Choose The Right Testing Approach For The Control Type

Different control types require different evidence. Applying the wrong method wastes effort and produces unconvincing results.

- For automated controls, test configuration, logic, exception handling, and a sample of both passing and failing transactions. Inspect the rule itself, not only outputs.
- For manual controls performed per transaction, inspect the evidence of performance for selected items.
- For detective controls, confirm the detection occurred, the exception was identified, and the follow-up action was documented and timely.
- For IT-dependent manual controls, confirm the system-generated input is reliable and the human judgment was exercised.
- For reconciliations, confirm the reconciliation was performed, differences were investigated, and the investigation was timely and resolved.

Do not test an automated control by sampling a few outputs and concluding it works. The logic, parameters, and change management around the rule are part of the control.

### Set Defensible Sample Sizes And Selection Methods

Sampling decisions must be documented and defensible. Do not pick a round number out of habit.

Decide:

- whether to test the entire population (appropriate for small or high-risk populations, or key automated controls);
- statistical sampling with a defined confidence level and tolerable deviation rate;
- judgmental sampling targeting high-value, unusual, new, manual, or high-risk items;
- sample size based on risk, frequency, prior failures, population size, and required assurance level;
- selection method (random, systematic, haphazard, targeted) and its rationale.

For a control operating frequently, a larger sample is generally expected. For a control operating a few times a year, testing all occurrences may be appropriate. Document the expected deviation rate and the conclusion threshold before testing, so that the conclusion is not rationalized after exceptions appear.

### Define The Attribute And Deviation Criteria Before Testing

The attribute is the specific, observable condition that must be present for the control to be considered operating. Define it before selecting items, so that judgment does not drift during testing.

For each attribute specify:

- the exact evidence required (signed approval, system timestamp, matching document, exception log entry);
- whether the evidence must be contemporaneous or can be reconstructed;
- what constitutes a deviation (missing evidence, late evidence, wrong approver, evidence that does not correspond to the item, system bypass);
- whether timing matters and the acceptable window;
- whether partial or reconstructed evidence counts, and if so under what conditions.

Reconstructed evidence is a red flag. If approvals are routinely backdated or reconstructed for the test, the control is not operating as designed. Treat reconstructed evidence as a deviation or escalate it.

### Test Both That The Control Ran And That It Worked

A common error is confirming that a step was performed without confirming it achieved its objective. An approval stamp that was applied without review, a reconciliation that was signed without investigating differences, and a review that rubber-stamped exceptions are all evidence of performance but not of effectiveness.

For higher-risk controls, design tests that probe whether the control actually caught errors:

- introduce or identify a known exception and confirm the control detected it;
- test items that should have been rejected and confirm they were;
- review the disposition of detected exceptions for timeliness and appropriateness;
- examine whether reviewers changed anything, or only signed.

### Handle Exceptions And Deviations Honestly

Exceptions are evidence. Do not explain them away to preserve a clean conclusion.

When a deviation is found:

- document the specific item, the nature of the deviation, and the cause;
- determine whether it is isolated or systemic by expanding the sample or analyzing the population;
- assess whether it indicates a design deficiency, an operating failure, a training gap, or a management override;
- evaluate the effect on the conclusion and on related controls;
- consider whether the deviation reflects a broader control failure or a one-off error.

A single deviation in a small sample can be material. Predefined decision rules prevent the temptation to dismiss exceptions as isolated.

### Sequence Testing From Design To Operation

Effective testing usually proceeds from understanding the design to confirming operation. Test design first, because testing the operation of a poorly designed control wastes effort.

A reasonable sequence:

- confirm the control is designed to address the risk;
- walk through the control with the owner to confirm understanding;
- evaluate design effectiveness;
- define the population and attribute;
- select and test the sample;
- evaluate deviations;
- conclude on operating effectiveness;
- link the conclusion to the risk and to any reliance decision.

### Link Test Results To Risk And Reliance Decisions

A test conclusion is not an end in itself. It feeds decisions about reliance, remediation, and reporting.

Connect each result to:

- whether the control can be relied upon for the period tested;
- the effect on the risk assessment if the control failed;
- whether compensating controls reduce the residual exposure;
- whether the failure affects financial reporting, regulatory compliance, or operations;
- the remediation required and the retest plan;
- the reporting to management, the audit committee, or regulators if applicable.

## Common Traps

### Sampling From A Subset And Calling It The Population

Drawing a sample from one system when transactions also flow through another, or filtering out exceptions before sampling, hides the highest-risk items. Reconcile the full population first.

### Treating A Signature As Proof Of Review

A signature proves only that someone signed. For meaningful assurance, test whether the reviewer identified and resolved exceptions, not merely that a line was initialed.

### Defining Attributes Loosely To Avoid Deviations

Vague attributes let testers rationalize missing or late evidence as acceptable. Define the precise evidence and deviation criteria before testing begins.

### Testing Automated Controls By Sampling Outputs Only

An automated control can produce correct outputs while the underlying rule is wrong, outdated, or bypassed for certain cases. Inspect the configuration and logic, not only the output.

### Ignoring Manual Overrides And Out-Of-System Items

The most serious failures often occur through manual journal entries, off-system approvals, or overrides. If the population excludes these, the conclusion is incomplete.

### Dismissing Exceptions As Isolated Without Analysis

Calling an exception a one-off without expanding the sample or analyzing the population is a form of conclusion-driven testing. Investigate cause and prevalence.

### Concluding Effective Based On A Single Period

A control that passed last quarter may have degraded. Conclusions are period-specific. Do not extrapolate a clean result to future periods without continued testing.

## Self-Check

- Is each test tied to a specific control, control objective, risk, owner, type, frequency, and assertion?
- Has the complete population been defined, reconciled to an independent source, and checked for excluded segments, overrides, and out-of-system items?
- Does the testing method match the control type, including configuration and logic testing for automated controls?
- Is the sample size and selection method documented with a rationale tied to risk, frequency, prior failures, and required assurance?
- Are the attribute and deviation criteria defined before testing, including whether reconstructed or late evidence counts?
- Does testing probe whether the control actually detected and resolved exceptions, not merely that a step was performed?
- Are exceptions documented, analyzed for cause and prevalence, and assessed for effect on the conclusion and related controls?
- Is the testing sequenced from design assessment to operating effectiveness before reliance decisions?
- Is each conclusion linked to a reliance decision, residual risk, compensating controls, remediation, and reporting?
- Is the conclusion limited to the period tested, with no unsupported extrapolation to future periods?
