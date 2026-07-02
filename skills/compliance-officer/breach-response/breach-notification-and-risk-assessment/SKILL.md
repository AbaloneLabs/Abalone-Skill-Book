---
name: breach-notification-and-risk-assessment.md
description: Use when the agent is deciding whether to notify the supervisory authority or data subjects of a breach, conducting the risk assessment that drives notification, drafting breach notifications within the 72-hour deadline, applying the high-risk-to-data-subjects standard, or documenting the notification decision and rationale.
---

# Breach Notification and Risk Assessment

Once a personal data breach is confirmed, the controller faces a series of decisions governed by strict timelines and risk-based standards. Under the GDPR, notification to the supervisory authority is required within 72 hours unless the breach is unlikely to result in risk to data subjects; notification to data subjects is required when the breach is likely to result in high risk. These are not binary triggers but risk assessments that must be conducted under time pressure and documented defensibly. A wrong "no risk" determination leads to a missed notification; an over-notification floods the authority and data subjects with low-value notices. This skill addresses the judgment involved in conducting the breach risk assessment and making and executing notification decisions.

## Core Rules

### Conduct the risk assessment against the correct standard for each notification

The GDPR establishes two distinct risk thresholds:

- **Supervisory authority notification**: required unless the breach is unlikely to result in a risk to the rights and freedoms of natural persons. The default is to notify; non-notification requires a justified determination that risk is unlikely.
- **Data subject notification**: required when the breach is likely to result in a high risk to the rights and freedoms of natural persons. This is a higher threshold than authority notification.

Apply the correct standard to each decision. Document the assessment against the applicable threshold. Do not conflate the two.

### Assess risk based on likelihood and severity of the potential impact

The risk assessment considers both dimensions:

- **Likelihood**: how probable is the harm, given the nature of the breach, the data involved, and the mitigations in place (for example, encryption of the data reduces the likelihood of misuse);
- **Severity**: how serious is the potential harm to data subjects (financial loss, identity theft, reputational damage, distress, discrimination, physical safety).

Factors that elevate risk include: special category data, financial data, large volumes, vulnerable data subjects, data that enables identity theft or fraud, and unmitigated exposure. Factors that reduce risk include: strong encryption, prompt containment, limited data, and effective mitigation offered to data subjects.

### Notify the supervisory authority within 72 hours of awareness

If the breach is likely to result in risk (the default for most breaches involving personal data), notify the supervisory authority within 72 hours of becoming aware. The notification must include at minimum:

- The nature of the breach and the categories and approximate number of data subjects and records concerned;
- The name and contact details of the DPO or contact point;
- The likely consequences of the breach;
- The measures taken or proposed to address it and mitigate adverse effects.

If all details are not known within 72 hours, notify in phases: provide what is known and supplement as more information becomes available. A late or absent notification without justified low-risk determination is a violation.

### Document a justified non-notification decision

If the controller determines the breach is unlikely to result in risk and therefore does not notify the authority, that decision must be documented with reasoning. The burden is on the controller to justify non-notification. The documentation should address:

- Why the breach is unlikely to cause risk;
- What data was involved and why it is low-risk;
- What mitigations reduce the likelihood or severity;
- Who made the decision and when.

An unjustified or undocumented non-notification is the most commonly enforced breach-response failure.

### Notify data subjects when high risk is likely

When the breach is likely to result in high risk to data subjects, notify them directly, in clear and plain language, without undue delay. The notification should include:

- What happened;
- The data affected;
- The likely consequences;
- What the controller is doing;
- What the data subject should do to protect themselves (password changes, fraud monitoring);
- The contact point for more information.

Direct notification can be avoided only if: the controller has implemented appropriate technical and organisational protection measures (especially encryption rendering data unintelligible), or the controller has taken subsequent measures ensuring the high risk is no longer likely, or notification would involve disproportionate effort (in which case a public communication is used instead). These exceptions are narrow and must be documented.

### Communicate with data subjects in a way that enables self-protection

The purpose of data subject notification is to enable the affected individuals to take protective action. Ensure the notification:

- Is timely enough to be useful (not weeks after the data is in circulation);
- Is clear about the specific risks (identity theft, phishing, financial fraud) and concrete steps to take;
- Does not downplay the risk or the controller's responsibility;
- Provides a genuine contact channel for questions and support.

A notification that is technically compliant but does not help data subjects protect themselves fails its purpose.

### Coordinate notification with processors, joint controllers, and third parties

If the breach occurred at a processor, the processor notifies the controller, who then assesses and notifies. If multiple controllers are involved, coordinate to avoid duplicate or conflicting notifications. If the breach affects other organisations' data (for example, in a shared service), coordinate notification responsibilities. Define these pathways in contracts and the incident response plan before a breach occurs.

### Maintain documentation of all breaches, notified or not

The controller must document all personal data breaches, including the facts, effects, and remedial action, regardless of whether notification was required. This documentation enables the supervisory authority to verify compliance and supports continuous improvement. Maintain a breach register that records every breach, the risk assessment, the notification decision, and the outcome.

## Common Traps

### Defaulting to non-notification to avoid regulatory scrutiny

The 72-hour notification is treated as optional or undesirable, and breaches are rationalised as low-risk to avoid reporting. The default is to notify; non-notification requires a justified, documented determination.

### Confusing the authority-notification threshold with the data-subject threshold

Notifying the authority but not data subjects when high risk is present, or vice versa. Apply each standard separately and document both decisions.

### Missing the 72-hour deadline due to incomplete information

Waiting until all facts are known before notifying. The regime permits phased notification; notify what is known within 72 hours and supplement later.

### Over-notifying data subjects for low-risk breaches

Notifying data subjects for every minor incident, including those that do not meet the high-risk threshold, causes notification fatigue and unnecessary distress. Reserve data subject notification for genuine high risk.

### Data subject notifications that do not enable self-protection

A vague notification that does not explain the specific risk or concrete steps leaves data subjects unable to protect themselves. Be specific and actionable.

### No documentation of the risk assessment or the non-notification decision

The controller does not notify but also does not document why. When the authority later learns of the breach, the controller cannot justify the decision. Document every decision, notified or not.

## Self-Check

- Was the risk assessment conducted against the correct standard for each notification (risk for authority, high risk for data subjects)?
- Did the assessment weigh both likelihood and severity, considering the data type, volume, vulnerability of subjects, and mitigations?
- If risk was likely, was the supervisory authority notified within 72 hours of awareness, with the required content or phased supplementation?
- If non-notification was decided, is the low-risk determination documented with reasoning, data analysis, mitigations, decision-maker, and date?
- If high risk was likely, were data subjects notified without undue delay, in clear language, with consequences and protective steps?
- Does the data subject notification enable genuine self-protection (specific risks, concrete steps, real contact channel)?
- Are notification responsibilities coordinated with processors, joint controllers, and third parties under pre-defined pathways?
- Is every breach documented in a breach register with facts, effects, risk assessment, notification decision, and remedial action, regardless of notification?
