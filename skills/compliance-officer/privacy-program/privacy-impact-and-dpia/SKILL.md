---
name: privacy-impact-and-dpia.md
description: Use when the agent is conducting a data protection impact assessment or privacy impact assessment, identifying high-risk processing requiring a DPIA, evaluating risks to data subjects, consulting with the supervisory authority before high-risk processing, or designing mitigation measures for identified privacy risks.
---

# Privacy Impact and DPIA

A Data Protection Impact Assessment (DPIA) — or privacy impact assessment more broadly — is the structured process for identifying and mitigating privacy risks before a processing operation begins. Under the GDPR, a DPIA is mandatory for high-risk processing, and failure to conduct one when required is itself a violation, independent of any underlying harm. But the value of a DPIA is not in producing a document; it is in forcing a structured risk analysis that changes the design of the processing. A DPIA that is a checkbox exercise, completed after the system is built, adds no protection. This skill addresses the judgment involved in determining when a DPIA is required, conducting it meaningfully, and acting on its findings.

## Core Rules

### Identify whether a DPIA is required before processing begins

A DPIA is mandatory where processing is likely to result in high risk to data subjects. High-risk indicators include:

- Large-scale processing of special category data or data of vulnerable individuals;
- Systematic monitoring of public areas;
- Profiling with significant effects (credit scoring, behavioural advertising with consequences);
- Large-scale systematic monitoring;
- Innovative technology (biometrics, AI, location tracking);
- Data matching or combining datasets from different sources;
- Preventing data subjects from exercising rights or accessing services based on processing.

When in doubt, conduct the DPIA. The cost of an unnecessary DPIA is far lower than the cost of a required DPIA that was not done. Document the determination either way.

### Conduct the DPIA early and use it to shape design

A DPIA must be conducted before the processing begins, integrated into the design process (data protection by design). A DPIA conducted after the system is built can only describe risks, not change the design. Start the DPIA when the processing is being scoped, so that risk findings inform architecture, data minimization, access controls, and safeguards from the outset.

### Structure the DPIA around the required elements

A compliant DPIA includes:

- A description of the processing operations and the purposes;
- An assessment of the necessity and proportionality of the processing (is it necessary? is it minimal? is there a less intrusive alternative?);
- An assessment of the risks to the rights and freedoms of data subjects;
- The measures envisaged to address the risks (safeguards, security, mechanisms to ensure protection).

Address each element substantively. A DPIA that describes the processing but does not genuinely assess necessity, proportionality, and risk is incomplete.

### Assess necessity and proportionality rigorously

The necessity and proportionality assessment is the analytical core. For the processing:

- **Necessity**: is the processing genuinely necessary to achieve the purpose? Could the purpose be achieved without it, or with less data, or with less intrusive means?
- **Proportionality**: is the processing proportionate to the purpose? Does the benefit justify the intrusion on data subjects?

Document the analysis. If the processing is not necessary or not proportionate, that finding must drive a redesign or a decision not to proceed, not be rationalised away.

### Identify and rate risks to data subjects

Enumerate the risks the processing poses to data subjects, considering both likelihood and severity:

- Unauthorised access or breach;
- Excessive data collection or retention;
- Unintended profiling or discrimination;
- Loss of autonomy or control;
- Re-identification of pseudonymised data;
- Function creep (the data being used for other purposes later).

Rate each risk (for example, low/medium/high) and document the rationale. Risks that are dismissed without analysis re-emerge as incidents.

### Design mitigation measures proportional to the rated risks

For each identified risk, define a mitigation:

- Technical (encryption, pseudonymisation, access controls);
- Organisational (policies, training, access review, retention limits);
- Procedural (approval workflows, monitoring, audit).

Ensure the mitigation is proportional to the risk and that it is actually implemented, not merely proposed. Track mitigations to completion. A DPIA whose mitigations were never implemented provides false assurance.

### Seek prior consultation with the supervisory authority when residual risk remains high

If, after mitigation, the DPIA concludes that the processing still presents high risk, the controller must consult the supervisory authority before processing. The authority may provide guidance, impose conditions, or prohibit the processing. Failing to consult when required is a violation. Document the consultation and its outcome.

### Review and update the DPIA when processing changes

A DPIA is not permanent. It must be reviewed when the processing changes in a way that may affect the risk (new purposes, new technologies, new data categories, new recipients). Establish a trigger to re-assess and update the DPIA on material changes, and periodically even without changes for high-risk processing.

## Common Traps

### Treating the DPIA as a post-hoc documentation exercise

The DPIA is written after the system is built, describing risks that are already baked into the design. It cannot change the design and adds no protection. Conduct it early.

### Skipping the DPIA because the processing "seems low risk"

Many processing operations that seem routine are high-risk when examined (large-scale profiling, special category data, vulnerable subjects). Apply the high-risk indicators genuinely; do not rationalise avoidance.

### Describing the processing without assessing necessity and proportionality

A DPIA that details what the processing does but never asks whether it is necessary or proportionate misses the analytical core. The necessity and proportionality assessment is where the real judgment occurs.

### Identifying risks without designing or implementing mitigations

Risks are listed, mitigations are proposed, but no one confirms the mitigations are built and operating. Track mitigations to completion and verify implementation.

### Failing to consult the supervisory authority when residual risk is high

If the DPIA concludes high residual risk and the controller proceeds without consultation, the processing is non-compliant regardless of the DPIA quality.

### Never reviewing the DPIA as processing evolves

A DPIA accurate at launch may be stale after new purposes or technologies are added. Without a change-triggered review, the DPIA drifts from reality.

## Self-Check

- Has it been determined, with documented reasoning, whether the processing requires a DPIA, applying the high-risk indicators?
- Is the DPIA conducted before processing begins, integrated into design rather than written after the fact?
- Does the DPIA address all required elements: description, necessity and proportionality, risk assessment, and mitigation measures?
- Is the necessity and proportionality assessment substantive, asking whether the processing is genuinely necessary and proportionate, with the findings driving design?
- Are risks to data subjects enumerated, rated by likelihood and severity, with documented rationale?
- Are mitigation measures designed proportional to the risks and tracked to actual implementation and verification?
- Where residual risk remains high after mitigation, has the supervisory authority been consulted before processing?
- Is the DPIA reviewed and updated on material processing changes and periodically for high-risk processing?
