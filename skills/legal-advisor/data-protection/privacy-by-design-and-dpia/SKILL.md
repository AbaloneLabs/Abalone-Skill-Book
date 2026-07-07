---
name: privacy-by-design-and-dpia.md
description: Use when the agent is advising on privacy-by-design and data protection by design and by default, data minimization and purpose limitation at the design stage, data protection impact assessments (DPIAs) and transfer impact assessments (TIAs), legitimate-interest balancing tests, high-risk processing triggers, children's data and special-category data, new product or feature launch readiness, and AI/automated decision-making impact assessment.
---

# Privacy by Design and DPIA

Privacy by design and data protection impact assessments (DPIAs) are the upstream controls that prevent most data-protection failures before a product ships or a processing activity begins. The defining judgment problem is that privacy considerations are cheapest and most effective when built into requirements and architecture, yet teams routinely treat them as a launch-gate checklist applied to a finished design. An agent advising here must shift the analysis to the design stage, force an honest assessment of risk, and distinguish a genuine impact assessment from a rationalization of a predetermined outcome.

## Core Rules

### Design and default are distinct obligations

- **Data protection by design** requires embedding privacy controls into the architecture and processing from the outset: data minimization, pseudonymization, access controls, retention, and security proportionate to risk.
- **Data protection by default** requires that, by default, only data necessary for the specific purpose are processed, and that the data subject is not required to take active steps to protect their privacy. Defaults must be the most privacy-protective that still enable the purpose.
- These are not aspirational; under regimes such as the GDPR they are affirmative legal obligations enforceable alongside the substantive processing rules.

### Know when a DPIA is mandatory, not optional

- A DPIA is required for processing likely to result in a high risk to the rights and freedoms of natural persons. Common mandatory triggers include large-scale systematic monitoring, large-scale processing of special-category data, systematic profiling with legal or similarly significant effects, and innovative technology whose risks are not fully understood.
- When in doubt, document the screening decision. A reasoned decision not to conduct a DPIA is itself a compliance artifact; an undocumented skip is a gap.
- Some jurisdictions require submission of the DPIA or prior consultation with the supervisory authority where residual high risk remains after mitigation.

### Structure the assessment to drive decisions, not paperwork

A credible DPIA should contain, at minimum:

- A systematic description of the processing operations, purposes, necessity and proportionality.
- An assessment of risks to data subjects (not only to the organization), considering both likelihood and severity, for rights such as freedom, dignity, non-discrimination, financial loss, and reputational harm.
- Mitigation measures and a residual-risk determination, including whether residual risk is acceptable or whether prior consultation is required.

The DPIA must be produced before the processing begins, revisited when the processing changes materially, and should influence the design — not merely describe it.

### Purpose limitation and data minimization are design decisions

- New purposes require a new lawful basis and, often, fresh notice and consent. "We might want to use it later" is not a lawful basis for collection now.
- Minimization applies to categories of data, volume, granularity, retention, and access. Collecting data "because it might be useful" violates minimization even if the data is never used.
- Special-category data (health, biometric, genetic, racial/ethnic origin, political opinions, religious beliefs, trade union membership, sex life, criminal data) triggers heightened obligations and often an explicit-condition requirement.

### Legitimate-interest balancing is not a rubber stamp

- Where legitimate interests is the lawful basis, a three-part test is required: (1) a genuine, lawful interest of the controller or third party; (2) necessity — the processing must be necessary, not merely convenient; and (3) a balancing test weighing the data subject's reasonable expectations, rights, and freedoms against the interest.
- The balancing must be documented and revisited. It is weakest where the data subject has no meaningful relationship with the controller or where the processing is unexpected or intrusive.

### Children, automated decisions, and AI

- Children's data attracts heightened protection: age-appropriate design, parental-consent thresholds that vary by jurisdiction, and bans or restrictions on profiling for advertising to minors in some regimes.
- Solely automated decisions with legal or similarly significant effects are restricted; an exception requires an explicit lawful basis, meaningful information, and an effective human-review and contestation right.
- AI systems that process personal data require assessment of training-data lawfulness, model outputs that may reveal personal or special-category data, and the risk of inference and re-identification.

## Common Traps

- **Treating the DPIA as a launch checklist.** Conducting the assessment after design is fixed means it cannot influence architecture; it becomes a rationalization.
- **Risk to the organization, not to data subjects.** A DPIA that assesses only regulatory or reputational risk to the company misses the legally required subject matter.
- **Skipping the DPIA based on "we've always done it."** New technology, new scale, new data categories, or new purposes can convert previously low-risk processing into high-risk.
- **Over-collecting "just in case."** Collecting data without a current, specific purpose violates minimization regardless of later intent.
- **Rubber-stamping legitimate interests.** Documenting a balancing test that always concludes in favor of the controller is not a genuine assessment.
- **Ignoring special-category data inferred by AI.** Models can infer health, ethnicity, or other special categories from non-sensitive inputs; the heightened obligations still apply.
- **Defaults that require opt-out.** A default that shares or processes data unless the user acts is usually not a compliant default.
- **Stale assessments.** A DPIA conducted once and never revisited does not reflect material changes in scope, scale, or technology.

## Self-Check

- Is privacy by design and by default embedded in requirements and architecture before launch, rather than added at the gate?
- Has a documented screening determined whether a DPIA is mandatory, with the reasoning preserved even when the answer is no?
- Does the DPIA assess risk to data subjects (likelihood and severity), not only to the organization, and does it drive design changes?
- Are purpose limitation and data minimization applied to categories, volume, granularity, retention, and access — not only to a high-level purpose statement?
- For special-category, children's, or criminal data, are heightened obligations and explicit conditions identified?
- Where legitimate interests is the lawful basis, is the three-part test (interest, necessity, balancing) documented and genuinely balanced against data subjects' expectations and rights?
- For AI and automated decisions, have training-data lawfulness, inference risk, and human-review rights been assessed?
- Have I flagged where residual high risk may require prior consultation with the supervisory authority, and recommended qualified data-protection counsel (or a DPO) for jurisdiction-specific determinations?
- Have I stated the jurisdictional limits of general guidance and recommended counsel qualified in each relevant jurisdiction for enforcement-sensitive decisions?
