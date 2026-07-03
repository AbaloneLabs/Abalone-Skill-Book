---
name: control_design_assessment.md
description: Use when the agent is assessing whether an internal control is well designed, mapping a control to the risk it addresses, evaluating control objectives and precision, reviewing preventive versus detective design, applying a control framework such as COSO, or deciding whether a control is capable of preventing or detecting misstatement before testing its operation.
---

# Control Design Assessment

Design assessment is the question that comes before operating effectiveness: if this control were performed exactly as described, by a competent and independent person, with reliable information, would it actually prevent or detect the risk it is meant to address? A control can operate every single day and still be useless if it was designed to answer the wrong question, review the wrong data, or catch the error too late. Agents frequently conclude that a control is adequate because it is documented and signed, without ever asking whether the design itself could achieve the control objective. That mistake makes the entire controls-reliance approach collapse on its first real test.

Use this skill before testing operating effectiveness, before deciding to rely on a control, and whenever a control is being newly designed, redesigned after a finding, or evaluated against a framework such as COSO. The goal is to judge whether the control, as designed, is capable of addressing the risk, and to refuse to rely on controls whose design cannot meet that test.

## Core Rules

### Start From The Risk And The Control Objective

A control has no meaning except in relation to a risk. Before judging design, restate the risk and the assertion the control is meant to address, then define the control objective in testable terms.

For each control, establish:

- the specific risk or what-could-go-wrong statement;
- the relevant assertion, such as existence, completeness, accuracy, valuation, cutoff, or authorization;
- the control objective, stated as what the control should achieve;
- the process step and population the control covers;
- the source of information the control uses;
- the expected outcome when the control works and when it fails.

A control objective such as "ensure expenses are appropriate" is too vague to assess. "Detect and prevent payment of expenses lacking documented business purpose and approval, before disbursement" is assessable. If the objective cannot be stated precisely, design cannot be judged.

### Judge Whether The Design Can Achieve The Objective

The core design question is capability. Imagine the control performed perfectly. Would the risk be prevented or detected?

Assess capability against:

- precision: does the control look at the right data, at the right level of detail, with the right thresholds;
- coverage: does it apply to the entire population or only a subset, and are exclusions justified;
- timing: does it operate before the risk materializes, or detect it after, and is that acceptable;
- independence: is the performer independent enough of the activity being controlled;
- competence: does the performer have the knowledge to spot the error;
- information quality: does the control rely on complete, accurate, timely data;
- authority: can the performer stop, correct, or escalate the problem;
- evidence: does the control leave a record that it occurred and what was found.

A control fails the capability test if, even performed perfectly, it would not catch the risk. For example, a review that signs a variance report without defined thresholds or exception follow-up is not capable of detecting manipulation hidden within tolerance.

### Distinguish Preventive, Detective, And Corrective Design

Design type changes what the control can achieve and how it must be supplemented.

- Preventive controls stop an error before it occurs, such as a system block on unauthorized payments or a mandatory approval before posting. They are strongest but depend on configuration and access controls.
- Detective controls find an error after it occurs, such as a reconciliation or an exception report review. They are only effective if detection happens soon enough to allow correction before the statements are issued or before harm compounds.
- Corrective controls fix a detected error and prevent recurrence, such as reprocessing, recovery, and root-cause remediation. They matter but do not by themselves prevent or detect the original error.

A process that relies only on detective controls operating long after period end may detect too late. Judge whether the mix of preventive and detective design is adequate for the risk and timing.

### Evaluate Precision And Exception Handling

Imprecise controls are a common design weakness. A review that glances at a total, or an approval that rubber-stamps a batch, is not precise enough to catch a material error buried inside.

Probe precision by asking:

- what specific data points or attributes the control examines;
- what thresholds, tolerances, or criteria trigger investigation;
- whether the control tests individual items or only aggregates;
- whether the reviewer is required to explain variances and document the explanation;
- whether the control would catch a single large error, a pattern of small errors, or an unusual item;
- what happens when an exception is found, including who is notified and how it is resolved;
- whether exception resolution is tracked to completion.

A control with no defined exception criteria, or with exceptions that are noted but never resolved, is not precisely designed and should not be relied upon without a compensating control.

### Assess Coverage And Population Completeness

A control that covers only part of the population leaves the rest exposed. Design assessment must confirm what is in and out of scope.

Check:

- whether the control applies to all relevant transaction types, locations, and systems;
- whether manual entries, adjustments, and non-routine items are included;
- whether system exclusions, overrides, or bypass paths exist;
- whether the population definition matches the risk, for example whether a sample covers a full year or only a quarter;
- whether the control relies on a report whose completeness has been confirmed.

A reconciliation that covers the core system but excludes manual journal entries to the same account has a coverage gap that an attacker or an error can exploit.

### Evaluate The Reliability Of Information The Control Uses

A control is only as good as the information it consumes. Many controls depend on system reports, extracts, or master data that may be incomplete or inaccurate.

For each control, identify:

- the source of the information used;
- whether it is system-generated, manually prepared, or a mix;
- whether the completeness and accuracy of that information is itself controlled or tested;
- whether the information could be manipulated by the person whose activity is being controlled;
- whether the information is timely enough for the control to act on it.

If a manager reviews a report that a subordinate can alter, or that is generated by an untested interface, the control design is undermined at its foundation. Assess the information layer as part of design.

### Consider Independence, Segregation, And Override

Design must account for who performs the control and whether they can also perform or conceal the activity being controlled.

Assess:

- whether the performer is independent of the transaction being controlled;
- whether duties are segregated so that no one person can initiate, approve, record, and reconcile the same item;
- whether management can override the control without detection;
- whether the control is performed by someone with sufficient authority to act on findings;
- whether the control is self-reviewed, which negates its value.

A control performed by the same person who posts the entries, or one that management can override silently, fails the independence test regardless of how often it is performed.

### Apply A Control Framework Consistently

Frameworks such as COSO provide structure for assessing whether control design is complete and coherent. Use one consistently rather than mixing ad hoc criteria.

Across the framework, assess:

- the control environment, including tone at the top, ethics, and structure;
- risk assessment, including how the entity identifies and responds to risks;
- control activities, including the specific preventive and detective controls;
- information and communication, including the quality of information and reporting lines;
- monitoring activities, including ongoing evaluations and separate assessments.

A strong control activity sitting inside a weak control environment, or supported by poor information and communication, may not be reliably designed in practice. Judge design in context, not in isolation.

### Decide Reliance And Document The Design Conclusion

After assessing design, reach an explicit conclusion before any operating effectiveness testing. Reliance on a control is only justified when design is capable.

Document:

- the risk and control objective restated;
- the design assessed, including type, precision, coverage, timing, and information source;
- the independence and override considerations;
- the framework elements considered;
- the design conclusion: capable, partially capable with compensating controls, or not capable;
- the implications for the audit approach and for planned operating effectiveness testing.

If design is not capable, do not invest in testing operation. Move to a substantive approach or require redesign, and record the deficiency.

## Common Traps

### Concluding Design Is Effective Because The Control Is Documented

A written procedure is not a capable control. Design must be judged against the risk and objective, not against the existence of a document.

### Reviewing The Happy Path Only

A design that works for routine transactions but has no exception criteria or override handling is not capable across the real population. Always assess exception and edge-case handling.

### Ignoring The Information Layer

A precise-looking review built on an untested or manipulable report is not reliable. Assess the source data as part of design.

### Assuming Automated Means Effective

Automation removes human error but introduces configuration, access, and change-control risk. An automated control with a wrong rule or unprotected configuration is misdesigned even if it runs daily.

### Treating A Detective Control As If It Were Preventive

A reconciliation detects after the fact. If detection occurs too late to correct before reporting, the design does not fully address the risk and may need a preventive complement.

### Overlooking Coverage Gaps

A control that excludes manual entries, adjustments, or certain locations leaves those populations uncontrolled. Confirm coverage matches the risk.

### Judging Design In Isolation From The Control Environment

A capable control activity inside a weak environment, with poor information and no monitoring, may not operate reliably. Assess design in its framework context.

### Moving To Operating Effectiveness Testing Before Settling Design

Testing whether a poorly designed control operated is wasted effort. Resolve the design question first.

## Self-Check

- Is the risk and the relevant assertion restated, and is the control objective defined in testable terms?
- Has the control been judged for capability, including precision, coverage, timing, independence, competence, information quality, authority, and evidence?
- Is the design type, preventive, detective, or corrective, identified and assessed for whether the timing is adequate for the risk?
- Are precision and exception handling evaluated, including thresholds, individual versus aggregate review, and tracking of exception resolution?
- Is population coverage confirmed against all relevant transaction types, locations, systems, manual entries, and overrides?
- Is the reliability of the information the control uses assessed, including completeness, accuracy, timeliness, and susceptibility to manipulation?
- Are independence, segregation of duties, and management override considered in the design?
- Is a control framework such as COSO applied consistently to assess the control in its environment, risk, activity, information, and monitoring context?
- Is an explicit design conclusion reached, capable, partial with compensating controls, or not capable, before any operating effectiveness testing?
- Where design is not capable, is the audit approach shifted to substantive or redesign, with the deficiency recorded rather than papered over?
