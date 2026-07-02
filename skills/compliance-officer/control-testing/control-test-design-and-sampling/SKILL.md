---
name: control_test_design_and_sampling.md
description: Use when the agent is designing compliance control tests, defining test objectives and procedures, selecting sampling methodologies and sample sizes, determining what population to test, or deciding how much evidence a control test needs to support a conclusion about operating effectiveness.
---

# Control Test Design And Sampling

A control test is only as credible as its design. A test that examines the wrong population, draws a sample too small to support its conclusion, or applies procedures that cannot detect control failure will report that a control is effective even when it is not. The harm is double: the organization believes it has assurance it does not have, and when the control fails for real, the testing record shows green right up to the moment of failure, which regulators and auditors treat as evidence that testing was inadequate, not that the failure was unforeseeable. The judgment problem is designing tests whose objectives, populations, samples, and procedures are sufficient to support a defensible conclusion about whether the control actually works, and being honest about the limits of what a test can prove.

Use this skill before designing a control testing plan, selecting sampling approaches, defining test procedures, scoping populations, or reviewing whether existing tests are rigorous enough to rely on. The goal is to make the agent treat test design as an evidence-and-conclusion discipline where every choice, population, sample, procedure, and threshold, affects what the test can and cannot prove.

## Core Rules

### Define The Test Objective In Terms Of The Control Assertion

Before selecting a sample or writing a procedure, define precisely what the test is trying to conclude. A vague objective such as test the approval control produces a vague test that cannot be defended. The objective must state the specific assertion about the control's operation.

Define the objective by specifying:

- which control is being tested and the risk it mitigates;
- the assertion being tested, such as that the control operated throughout the period, operated on all relevant transactions, or operated without exception;
- whether the test addresses design effectiveness, does the control make sense, or operating effectiveness, did it actually work during the period;
- the period covered and why that period was selected;
- the conclusion the test is expected to support, such as effective, effective-with-deficiency, or ineffective.

A test without a defined assertion cannot fail in a meaningful way, because there is no standard against which to judge the results. Define what success and failure look like before testing begins.

### Identify The Correct Population And Verify Completeness

Testing the wrong population, or a population that is incomplete, invalidates the conclusion. If the sample is drawn from a subset that excludes the riskiest transactions, the test will look clean while missing the exposure.

Establish the population by:

- defining the complete set of items to which the control should have applied during the period;
- obtaining the population from an independent source where possible, such as a system-generated report reconciled to the general ledger, rather than from the control owner who has an interest in a clean result;
- reconciling the population total, count, or value to an independent record to confirm completeness;
- identifying and explaining any items excluded from the population, such as voided transactions or system-outage periods, and assessing whether exclusion creates a coverage gap;
- documenting the population source, extraction method, reconciliation, and any exclusions.

A population provided by the control owner without independent verification is not a reliable basis for testing. The most common testing failure is a population that quietly excludes the items where the control is weakest.

### Choose A Sampling Approach Matched To The Objective And Risk

Sampling methodology determines what the results can support. Statistical sampling supports quantified conclusions about the population; judgmental sampling is useful for targeting risk but cannot support population-wide claims. Mismatching method to conclusion is a fundamental design error.

Select the approach by considering:

- statistical sampling, which uses defined sample sizes and selection methods to support a quantified confidence level about the population, appropriate when a population-wide effectiveness conclusion is needed;
- judgmental or targeted sampling, which focuses on high-risk, unusual, or override-prone items, appropriate when the objective is to find problems rather than to prove population-wide effectiveness;
- full-population testing, which examines every item, appropriate when the population is small, the risk is high, or the control is automated and testable comprehensively;
- the interaction between sampling approach and the conclusion: do not claim population-wide effectiveness based on a judgmental sample, and do not waste statistical rigor on a sample too small to support its stated confidence.

Document the sampling approach, the rationale, and the limitations of the conclusions it supports. A statistical sample with an undocumented methodology is no more defensible than a guess.

### Determine Sample Size Based On Risk, Confidence, And Tolerance

Sample size is not arbitrary. Too small a sample cannot support its conclusion; too large a sample wastes effort. The size must reflect the risk of the control, the desired confidence, and the tolerable deviation rate.

Determine size by considering:

- the risk rating of the control and the underlying process, with higher risk warranting larger samples;
- the desired confidence level, typically 90 to 95 percent for compliance testing, and the tolerable deviation rate, the maximum failure rate the organization can accept;
- the expected deviation rate based on prior testing or pilot sampling, which affects the sample size needed to detect deviations;
- whether the control is manual, requiring larger samples due to inconsistency, or automated, where a smaller sample may suffice because the control performs identically each time;
- the interaction with sampling approach: statistical sample sizes follow defined formulas, while judgmental samples are sized based on risk and coverage goals.

A sample of five items for a high-risk manual control is almost always too small to support an effectiveness conclusion. Be honest when the sample size limits the conclusion, and either increase the sample or narrow the claim.

### Design Procedures That Can Detect Control Failure

A test procedure that cannot detect a control failure will always report success, but that success is meaningless. The procedure must be capable of identifying whether the control actually operated as intended.

Design effective procedures by:

- defining what evidence demonstrates the control operated, such as a signature, a system log, an approval timestamp, or a reconciliation;
- testing the evidence itself for authenticity, such as confirming that an approval was not backdated, a signature is not forged, or a system flag was not overridden;
- testing for completeness of the evidence, such as confirming that all required approvals are present, not just that some approvals exist;
- testing for timeliness, such as confirming approvals occurred before rather than after the event they are meant to control;
- considering whether the evidence could exist without the control actually operating, such as a checkbox that was ticked without verification, and designing procedures to detect this;
- performing reperformance where the tester independently executes the control procedure to confirm the owner's result.

The strongest procedures combine inspection of evidence with reperformance and corroboration from independent sources. Inspection of a document that the control owner created is weak evidence; corroboration from a system log or third party is strong.

### Define Deviation Handling And Escalation Before Testing

What happens when a test finds a deviation must be defined before testing begins, not discovered ad hoc. Otherwise, deviations get explained away, reclassified as non-errors, or buried to protect a clean conclusion.

Define deviation handling by:

- establishing what constitutes a deviation, with clear criteria distinguishing a control failure from an acceptable exception or a documentation gap;
- defining how multiple deviations affect the conclusion, including thresholds beyond which the control is deemed ineffective;
- requiring investigation of each deviation to determine root cause, pervasiveness, and whether it indicates a systemic issue;
- determining whether deviations trigger expanded testing, such as extending the sample when initial deviations exceed expectations;
- escalating significant or systemic deviations to the appropriate authority for remediation decisions rather than closing them within the test.

The temptation to rationalize a deviation as a one-off is strong, especially when reporting it creates work. Pre-defined criteria and escalation paths resist that temptation.

### Document The Test Sufficiently To Be Relied Upon By Others

A control test that cannot be understood or reproduced by someone who did not perform it cannot be relied upon by audit, regulators, or future testers. Documentation is not supplementary; it is the test.

Document by recording:

- the objective, the control, the assertion, and the period;
- the population, its source, reconciliation, and exclusions;
- the sampling approach, methodology, size, and selection method;
- the procedures performed, the evidence examined, and any reperformance;
- the deviations found, their investigation, root cause, and disposition;
- the conclusion, the basis for it, and any limitations on what the test proves;
- who performed and reviewed the test and when.

A test documented only by a conclusion with no supporting detail is an assertion, not evidence. The documentation should allow a reviewer to reach the same conclusion independently.

## Common Traps

### Testing Without A Defined Assertion

A test with no clear objective cannot meaningfully fail and produces an undefendable conclusion. Define the assertion before sampling.

### Population Provided By The Control Owner Without Verification

A self-selected population may exclude the riskiest items. Reconcile to an independent source.

### Claiming Population-Wide Effectiveness From A Judgmental Sample

Judgmental samples find problems but cannot prove population-wide effectiveness. Match the conclusion to the method.

### Sample Size Too Small For The Risk

A handful of items cannot support an effectiveness conclusion for a high-risk manual control. Size the sample to the risk and confidence level.

### Procedures That Cannot Detect Failure

Inspecting owner-created evidence without corroboration or reperformance may always look clean. Design procedures capable of finding failure.

### Rationalizing Deviations As One-Offs

Without pre-defined deviation criteria, testers explain away failures to preserve a clean conclusion. Define and enforce deviation handling before testing.

### Conclusion Without Supporting Documentation

A conclusion without population, sample, procedures, and evidence is an assertion, not a test. Document so others can reproduce the conclusion.

## Self-Check

- Is the test objective defined as a specific assertion about a named control, including design versus operating effectiveness, the period, and what success and failure look like?
- Is the population defined completely, sourced independently of the control owner, reconciled to an independent record, with exclusions explained and assessed for coverage gaps?
- Is the sampling approach, statistical, judgmental, or full-population, matched to the objective, with the conclusion limited to what the method can support?
- Is sample size determined by risk, confidence level, tolerable deviation rate, expected deviation rate, and manual versus automated nature, rather than chosen arbitrarily?
- Do procedures test evidence authenticity, completeness, and timeliness, include reperformance where appropriate, and detect cases where evidence exists without the control operating?
- Are deviation criteria, conclusion thresholds, investigation requirements, sample expansion triggers, and escalation paths defined before testing begins?
- Is the test documented with objective, control, population, sampling, procedures, deviations, conclusion, limitations, and reviewer, sufficient for independent reproduction?
- Is the test honest about what it can and cannot prove, with conclusions narrowed where evidence or sample limits warrant?
