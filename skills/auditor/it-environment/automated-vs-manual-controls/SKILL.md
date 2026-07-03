---
name: automated_vs_manual_controls.md
description: Use when the agent is distinguishing automated from manual controls, assessing the reliability and risks of each, evaluating automated control configuration and IT general control dependency, testing automated controls consistently performing, or deciding when to rely on automated versus manual controls in a controls-reliance approach.
---

# Automated Vs Manual Controls

Automated and manual controls have fundamentally different risk profiles, and treating them the same produces mis-targeted testing. An automated control, once correctly configured and protected by IT general controls, performs consistently without human judgment or fatigue; a manual control depends on the performer's competence, diligence, and independence every time it runs. Agents often assume automated means reliable without testing the configuration or the underlying IT general controls, or they test manual controls as if they were automated, expecting consistency that human performance cannot deliver. The disciplined approach is to understand what makes each type reliable, test the conditions for that reliability, and design procedures that fit the control's actual nature.

Use this skill when classifying controls as automated or manual, when assessing the reliability of each, and when designing testing for automated and manual controls. The goal is to test each control in a way that actually supports reliance, given its nature.

## Core Rules

### Classify Each Control Correctly

The first step is to classify each control as automated, manual, or hybrid. Misclassification drives the wrong testing.

Classify by asking who or what performs the control:

- automated controls are performed by the system, based on configured rules, without human judgment in execution;
- manual controls are performed by a person, applying judgment, review, or inspection;
- hybrid controls combine both, such as a system-generated exception report that a person reviews and resolves.

Document the classification for each control. A control's classification determines what conditions must hold for it to be reliable and how it should be tested.

### Understand What Makes Automated Controls Reliable

Automated controls are reliable when three conditions hold: the configuration is correct, the configuration has not changed, and the data the control uses is complete and accurate. Testing an automated control means testing these conditions.

For automated controls, confirm:

- the rule or configuration is correct and addresses the intended risk;
- the configuration has been in place and unchanged for the period, or changes were controlled;
- IT general controls over access, change management, and operations are effective, so the configuration is protected;
- the data the control processes is complete and accurate, including interfaces and master data;
- the control produces evidence of its operation, such as a log or exception report.

An automated control is only as reliable as the IT general controls that protect it. Reliance on an automated control requires reliance on the underlying IT general controls.

### Understand What Makes Manual Controls Reliable

Manual controls are reliable when the performer is competent, independent, diligent, and follows a consistent procedure. Testing a manual control means testing these conditions across the period.

For manual controls, confirm:

- the performer has the competence to identify the error or exception;
- the performer is independent of the activity being controlled;
- the performer actually performs the control consistently, not intermittently;
- the control has defined criteria, thresholds, and exception handling;
- the control produces evidence of its performance, such as a signature or log;
- the performer has authority to act on findings and escalate.

Manual controls do not perform with the consistency of automated controls. Testing must cover multiple occurrences across the period to support an operating effectiveness conclusion.

### Test Automated Controls Through Configuration And ITGC Reliance

Because automated controls perform consistently when correctly configured, testing focuses on the configuration and the IT general controls that protect it, plus limited testing of operation.

Test automated controls by:

- inspecting the configuration or rule and confirming it addresses the risk;
- testing that the configuration has not changed during the period, or that changes were controlled;
- relying on IT general controls testing for access, change management, and operations;
- testing the completeness and accuracy of the data the control uses;
- testing operation on a limited number of occurrences to confirm the control fires as configured.

Do not test an automated control by sampling many occurrences as if human variation were the risk. The risk is configuration and protection, and that is where testing focuses.

### Test Manual Controls Across Multiple Occurrences

Because manual controls depend on the performer, testing must cover multiple occurrences across the period to support that the control operated consistently.

Test manual controls by:

- selecting occurrences across the period, not clustered in one time;
- selecting occurrences from the full population, including different performers if multiple exist;
- inspecting evidence that the control was performed for each selected occurrence;
- confirming the performer applied the defined criteria and handled exceptions;
- assessing whether the performer was independent and competent for each occurrence.

A single occurrence of a manual control does not support an operating effectiveness conclusion. The sample size and spread must reflect the dependence on human performance.

### Assess The Risk Specific To Each Type

Each type carries a characteristic risk that must be assessed and addressed.

Automated control risks:

- the configuration is wrong and consistently produces the wrong result;
- the configuration was changed without control, introducing error;
- IT general controls are weak, allowing unauthorized changes or access;
- the data the control uses is incomplete or inaccurate;
- the control fails silently when an exception occurs.

Manual control risks:

- the performer skips the control or performs it superficially;
- the performer lacks competence to identify the error;
- the performer is not independent of the activity;
- the control has no defined criteria, so performance is inconsistent;
- the control is performed but exceptions are not resolved.

Design testing to address the characteristic risk of each type, not a generic control risk.

### Decide Reliance Based On The Conditions For Each Type

Reliance on a control is justified only when the conditions for its type are met. Document the basis for reliance accordingly.

Rely on an automated control when:

- the configuration is correct and addresses the risk;
- IT general controls are effective;
- the data is complete and accurate;
- operation has been confirmed.

Rely on a manual control when:

- the performer is competent and independent;
- the control operated consistently across the period, supported by multiple tested occurrences;
- exceptions were handled;
- evidence of operation exists.

Do not claim reliance without the conditions for that control type being met and tested. An automated control without ITGC support, or a manual control tested once, cannot support reliance.

## Common Traps

### Assuming Automated Means Reliable

Automation removes human variation but introduces configuration, change, and access risk. An automated control is only reliable when its configuration and protecting IT general controls are tested.

### Testing Automated Controls Like Manual Controls

Sampling many occurrences of an automated control tests human variation that does not exist. The risk is configuration and protection, and that is where testing belongs.

### Testing Manual Controls Once

A single occurrence of a manual control does not support operating effectiveness. Manual controls must be tested across multiple occurrences because they depend on the performer.

### Relying On Automated Controls Without ITGC Support

An automated control is only as reliable as the IT general controls that protect its configuration. Reliance without ITGC testing is unsupported.

### Ignoring The Data The Control Uses

Both automated and manual controls depend on complete, accurate data. A control built on incomplete or inaccurate data is not reliable regardless of how it performs.

### Misclassifying Hybrid Controls

A system-generated report reviewed by a person is hybrid. The system generation and the human review each have their own reliability conditions and must both be tested.

### Expecting Manual Consistency

Manual controls do not perform with automated consistency. Design testing and set expectations to reflect human performance, including the possibility of intermittent execution.

## Self-Check

- Is each control classified correctly as automated, manual, or hybrid, with the classification documented?
- For automated controls, are configuration correctness, period stability, IT general control effectiveness, and data completeness and accuracy confirmed?
- For manual controls, are performer competence, independence, consistent performance, defined criteria, and evidence confirmed across multiple occurrences?
- Are automated controls tested through configuration inspection and ITGC reliance, with limited operation testing, rather than extensive occurrence sampling?
- Are manual controls tested across multiple occurrences spread over the period, rather than a single instance?
- Is the characteristic risk of each type, configuration and protection for automated, performer diligence for manual, assessed and addressed in testing?
- Is reliance claimed only when the conditions for the control's type are met and tested, with the basis documented?
- For hybrid controls, are both the system generation and the human review tested, each against its own reliability conditions?
