---
name: manual-vs-automated-controls.md
description: Use when the agent is distinguishing manual from automated controls, evaluating whether a control truly runs without human intervention, assessing re-configuration risk, deciding how much reliance to place on system-enforced controls, or reviewing the IT-dependent manual controls that sit between pure automation and pure human judgement.
---

# Manual vs Automated Controls

The manual-versus-automated distinction drives how an auditor tests a control, how often it must be tested, how it behaves under turnover, and what kind of failure can break it. A misread here is expensive: treating a control as automated when it actually depends on a person clicking "approve" leads to misplaced reliance on IT general controls (ITGCs) and under-testing of the human element; treating a manual control as automated leads to over-testing and missed override risk.

## Core Rules

### Classify by who or what performs the control action

- **Manual control**: a person performs the control using judgement, documents, or reports. The control's effectiveness depends on the person's competence, diligence, and presence.
- **Automated control**: a system performs the control without human intervention, based on configured rules. The control's effectiveness depends on the configuration being correct and unchanged.
- **IT-dependent manual control**: the system generates a report or exception list, but a person must review it and act. This hybrid is the most common type in modern environments and is often misclassified as automated.

The hybrid category matters because it inherits the weaknesses of both: it depends on ITGCs (the report must be complete and accurate) *and* on the human reviewer (who must actually examine and act). Test both dimensions.

### Confirm automation by tracing the control to its system configuration

Do not accept "the system does it" without evidence. For a genuinely automated control, confirm:

- the specific rule, validation, workflow, or report definition that enforces it;
- where the configuration lives (which module, which parameter, which workflow step);
- who can change it and under what authorisation;
- when it was last changed and whether the change was tested.

If the only evidence is "the system won't let you," ask for the configuration screen, the validation rule, or the workflow design document. Automated controls are only as reliable as the configuration that defines them, and configurations drift.

### Test automated controls through the ITGC layer plus one targeted test

Reliance on an automated control rests on two pillars:

1. **IT general controls** — access security over the configuration, change management over modifications, and IT operations over availability. If these are weak, the configuration cannot be trusted to be unchanged since last tested.
2. **One observation or re-performance of the automated control** — confirm the rule fires as designed on a current transaction or test case.

Do not rely on an automated control if the relevant ITGCs are not tested and effective. This is the most common over-reliance error.

### Test manual controls through inspection of evidence and re-performance

For manual controls, evidence of operation is the primary test:

- signatures, approvals, review initials, annotated reports;
- dated evidence showing the control ran in the period;
- re-performance on a sample to confirm the reviewer would actually catch the error.

For manual controls, test across multiple operators and multiple periods, because effectiveness varies by person and by workload. A control that works when the senior reviewer is on duty may fail when the junior is covering.

### Recognise the stability advantage of automated controls — and its limits

A correctly configured automated control performs identically every time, does not get tired, does not take vacation, and does not turnover. This makes automated controls attractive for high-volume, rules-based processes (three-way match, credit limits, validation of required fields). But:

- the stability holds only while the configuration is unchanged;
- a single bad change can disable or corrupt the control across the entire population instantly;
- automated controls do exactly what they are told, including applying a wrong rule consistently.

Stability is a reason to prefer automation for repetitive tasks; it is not a reason to skip testing the configuration.

### Assess re-configuration and override risk specifically

Automated controls fail in two characteristic ways:

1. **Silent re-configuration** — someone changes the rule (threshold, validation, workflow) and the control now behaves differently, often without anyone noticing.
2. **Override / bypass** — a privileged user can post outside the workflow, force-approve, or use a "super-user" function that skips the control.

For each automated control, ask: who has the access to change or bypass it, is that access logged and reviewed, and is there a detective control that would catch a silent change? If overrides are possible and unmonitored, the automated control is weaker than it appears.

### Match control type to the nature of the judgement required

Automation suits rules that are clear, stable, and high-volume. Manual judgement suits decisions that require context, exceptions, or interpretation:

- approving a non-standard journal entry with a narrative explanation (manual);
- blocking an invoice that fails three-way match (automated);
- assessing whether a provision estimate is reasonable (manual);
- enforcing a credit hold above a threshold (automated).

Forcing automation onto judgement-heavy decisions produces brittle controls that get overridden constantly; forcing manual review onto high-volume rules produces controls that get rubber-stamped. The right type follows the nature of the decision.

### Consider the four-quarter / multi-period reliability question

When planning reliance across the full audit period, automated controls have an advantage: if the configuration is unchanged and ITGCs are effective, one test can support reliance across the whole period. Manual controls require testing across periods and operators because reliability is not uniform. Factor this into the testing plan and sample design — but only after confirming configuration stability through change management testing.

## Common Traps

- **Treating an IT-dependent manual control as automated.** "The system generates a report" is not automation; the human review of that report is the control, and it must be tested as manual.
- **Relying on automated controls without testing ITGCs.** The configuration is only trustworthy if access and change management protect it; skipping ITGCs undermines all automated-control reliance.
- **Assuming "the system won't let you" without seeing the configuration.** Procedure documents describe intent; the actual rule may be disabled, set to warn-only, or bypassable by a privileged user.
- **Ignoring override paths.** A super-user or force-post capability silently defeats the automated control and is often the route used in fraud.
- **Over-testing manual controls as if they were automated** by testing only one operator or one period, missing the variability that defines manual control risk.
- **Under-testing manual controls by assuming automation elsewhere covers the risk.** A manual control that is the only defence for an assertion still needs full evidence-of-operation testing.
- **Forgetting that automated controls apply the wrong rule consistently.** A misconfigured validation blocks or allows the wrong population every single time; one configuration test catches it, but only if the test uses realistic data.
- **Treating automation as inherently stronger.** Automation is stronger only when the configuration is correct, protected, and unchanged; otherwise it concentrates risk.

## Self-Check

- For each control I classified as automated, can I point to the specific configuration (rule, workflow, validation, report definition) and confirm it is unchanged in the period?
- For each IT-dependent manual control, have I tested both the IT dimension (report completeness and accuracy, backed by ITGCs) and the human dimension (review and action evidence)?
- Have I tested the relevant ITGCs (access, change management) before placing reliance on any automated control?
- For each automated control, have I identified who can change or bypass it, and whether overrides are logged and reviewed?
- For manual controls, have I tested across multiple operators and multiple periods, not just a single point in time?
- Did I classify each control by what actually performs the action, not by what the procedure document calls it?
- For judgement-heavy decisions, did I resist forcing automation where it would be brittle or routinely overridden?
- For high-volume rules-based decisions, did I confirm the control is genuinely automated rather than a manual review dressed up as a system process?
