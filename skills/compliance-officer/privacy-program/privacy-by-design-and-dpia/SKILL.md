---
name: privacy_by_design_and_dpia.md
description: Use when the agent is deciding whether a data protection impact assessment is required, applying privacy by design and default, running the DPIA threshold test, designing mitigations, or consulting the DPO or supervisory authority on high-risk processing.
---

# Privacy By Design And DPIA

Privacy by design asks that protection be built into systems from the start, and the data protection impact assessment (DPIA) is the instrument that forces high-risk processing to be examined before it launches. Both are routinely treated as paperwork: a DPIA written after deployment to satisfy a checkbox, or privacy by design reduced to a line in a requirements document. The consequence is that risk is discovered late, when change is expensive or impossible, and the organization is exposed to both harm and the specific GDPR obligation to conduct a DPIA before processing.

Use this skill before launching a new product, deploying a system that involves profiling or large-scale data, or deciding whether a DPIA is needed. The goal is to make the agent identify high-risk processing early, run a genuine assessment, and consult the right people before committing to a design that cannot be undone.

## Core Rules

### Apply Privacy By Design And Default From The Start

Privacy by design and by default are not optional principles; they are binding obligations under GDPR Article 25. They require that protection be engineered into systems and that the default configuration be the most privacy-protective.

Embed privacy by design by:

- considering privacy at the design stage of any system, product, or feature that processes personal data;
- building in data minimization, pseudonymization, and purpose limitation from the outset;
- ensuring defaults collect only necessary data and limit processing to what is necessary;
- designing access controls, retention, and deletion into the architecture rather than bolting them on;
- documenting the design decisions and their privacy rationale.

Privacy by default means the default state favors the individual: minimal collection, minimal sharing, minimal retention, and minimal exposure. Defaults that maximize collection or sharing breach this duty.

### Apply The DPIA Threshold Test

A DPIA is required for processing likely to result in a high risk to individuals. The threshold is not "anything involving personal data"; it is a defined set of high-risk triggers. The first decision is whether the threshold is met.

A DPIA is generally required where processing involves:

- evaluation or scoring, including profiling and prediction;
- automated decision-making with legal or similarly significant effect;
- systematic monitoring, including large-scale public areas;
- sensitive or special category data on a large scale;
- data concerning vulnerable individuals, including children;
- innovative technology or new technological solutions;
- denial of access to services or contracts on a large scale;
- large-scale processing;
- matching or combining datasets;
- processing that prevents individuals from exercising rights or using services.

When in doubt, conduct the assessment. The cost of an unnecessary DPIA is far lower than the cost of an omitted one for processing that turns out to be high risk.

### Conduct A Genuine DPIA, Not A Template

A DPIA must be a real assessment: it describes the processing, assesses necessity and proportionality, identifies risks to individuals, and designs mitigations. A template filled with generic text does not satisfy the obligation.

A genuine DPIA includes:

- a description of the processing operations and purposes;
- an assessment of necessity and proportionality;
- the risks to the rights and freedoms of individuals;
- the measures envisaged to address the risks, including safeguards and mechanisms;
- the residual risk after mitigations and the decision to proceed.

The DPIA should be specific to the actual processing, reference the real data flows, and identify concrete mitigations rather than aspirations.

### Involve The Right People At The Right Time

A DPIA is not a solo exercise. It requires input from the people who understand the processing, the risks, and the controls.

Involve:

- the business owner accountable for the processing;
- technical and security teams who understand the architecture;
- the data protection officer, whose advice must be sought and documented;
- where relevant, the individuals or their representatives, for consultation on the risks;
- vendors and processors whose processing contributes to the risk.

Seek DPO advice before deciding to proceed, and record the advice and how it was addressed. A DPIA that does not document DPO involvement is incomplete.

### Consult The Supervisory Authority For Residual High Risk

Where, even after mitigations, the residual risk remains high, the controller must consult the supervisory authority before processing. This is a hard gate, not a courtesy.

Consult when:

- the DPIA concludes that the processing would result in high risk in the absence of measures;
- mitigations do not reduce the risk below the high threshold;
- the controller is uncertain whether the residual risk remains high.

Failure to consult where required is a standalone violation, independent of any harm.

### Treat DPIAs As Living Documents

A DPIA is not valid forever. It must be reviewed when the processing changes in a way that affects risk.

Review the DPIA when:

- the purposes, data categories, or recipients change;
- the technology, scale, or geography changes;
- new risks emerge from external developments;
- the system is significantly modified or integrated differently.

A DPIA that does not match the current processing provides no real protection and weak evidence of compliance.

### Connect DPIA To Lawful Basis And Notice

The DPIA should align with the lawful basis decision and the transparency obligations. Inconsistency between the DPIA, the notice, and the basis analysis is a red flag.

Ensure the DPIA:

- references the lawful basis and any Article 9 condition;
- confirms the processing is within the purposes disclosed in the notice;
- identifies any new notice or consent obligations triggered by the processing;
- feeds into the RoPA and the data inventory.

### Document The Decision And Residual Risk

The output of a DPIA is a documented decision: to proceed, to proceed with mitigations, to modify the processing, or not to proceed. The decision and its basis must be recorded.

Document:

- the residual risk level after mitigations;
- the decision and the decision-maker;
- the mitigations to be implemented and their owners;
- the review triggers and date.

## Common Traps

### DPIA As Post-Deployment Paperwork

Writing a DPIA after launch to satisfy a checkbox defeats the purpose, because the design decisions are already locked in.

### Missing The Threshold Test

Assuming a DPIA is not needed because the processing seems routine, when it involves profiling, large-scale sensitive data, or vulnerable individuals, is a frequent and serious error.

### Generic Template DPIAs

Fill-in-the-blank DPIAs that do not reference the actual processing provide no risk analysis and weak evidence.

### Skipping DPO Consultation

A DPIA without documented DPO advice, or that ignores DPO advice, is incomplete.

### Failing To Consult The Authority On Residual High Risk

Proceeding without authority consultation when residual risk remains high is a standalone violation.

### Stale DPIAs

A DPIA that is never reviewed becomes inaccurate as processing evolves and provides no current protection.

### Privacy By Default Violated By Defaults

Defaults that collect maximum data or enable maximum sharing breach Article 25 even if a privacy policy mentions minimization.

### Inconsistency With Basis And Notice

A DPIA that describes processing beyond the disclosed purposes or the chosen basis exposes a deeper compliance problem.

## Self-Check

- Is privacy by design and by default embedded from the design stage, with defaults that minimize collection, sharing, retention, and exposure?
- Has the DPIA threshold test been applied, recognizing triggers such as profiling, automated decisions, large-scale sensitive data, children, and innovative technology?
- Does the DPIA genuinely describe the processing, assess necessity and proportionality, identify risks, and design concrete mitigations?
- Are business owners, technical and security teams, the DPO, and where relevant individuals and vendors involved and documented?
- Where residual risk remains high, has the supervisory authority been consulted before processing begins?
- Is the DPIA reviewed when purposes, data, technology, scale, or geography change?
- Does the DPIA align with the lawful basis, Article 9 conditions, the privacy notice, the RoPA, and the data inventory?
- Is the decision to proceed, with residual risk and mitigation owners, documented?
- Are mitigations tracked to implementation rather than left as aspirations?
- Is a DPIA conducted before launch rather than written after deployment to satisfy a checkbox?
