---
name: third-party-ongoing-monitoring-and-incident-response.md
description: Use when the agent is designing the ongoing monitoring of a third-party relationship (performance metrics, control testing, incident reporting, periodic reassessment), responding to a third-party compliance incident or breach (data breach, bribery, sanctions, control failure at the third party), escalating and remediating, or deciding whether to continue or terminate the relationship after an incident. Applies throughout the relationship lifecycle, after onboarding and contract execution.
---

# Third-Party Ongoing Monitoring And Incident Response

Onboarding is the beginning of the relationship, not the end of the risk. The third party's controls, financial condition, ownership, and conduct change over the life of the relationship, and a third party that passed due diligence at onboarding can become high-risk through acquisition, financial deterioration, control decay, or misconduct. Ongoing monitoring is the control that detects these changes, and incident response is the control that manages the consequences when a third party fails. Without ongoing monitoring, the organization discovers the third party's failure when it becomes the organization's failure (a data breach, a regulator inquiry, a supply disruption), and without incident response, the organization manages the consequences reactively and incompletely.

The harm this skill prevents is a third-party failure that the organization discovers late (a breach disclosed months after it occurred, a control decay detected only at the next periodic review), a failure whose consequences the organization bears because it lacked the monitoring to detect and the response to contain, or a failure that cascades because the organization had no contingency for the third party's disruption. Compliance officers tend to invest in the onboarding diligence and underinvest in the ongoing monitoring, treating the relationship as managed once the contract is signed, but the risk persists and changes, and the ongoing monitoring is the control that keeps the risk visible.

## Core Rules

### Establish Risk-Based Ongoing Monitoring

Ongoing monitoring must be risk-based, with the intensity matched to the third party's risk tier. A high-risk third party requires continuous or frequent monitoring (monthly performance reviews, annual on-site assessments, real-time screening alerts), while a low-risk third party may require only an annual desk-based review. A monitoring program that applies the same intensity to all third parties either wastes resources on low-risk relationships or under-monitors high-risk ones.

Define the monitoring requirements by risk tier: the frequency of review, the metrics tracked, the assessments conducted, the evidence collected. Tie the monitoring to the specific risks the relationship creates (financial monitoring for a financial-risk third party, control testing for a control-dependency third party, sanctions re-screening for a jurisdiction-risk third party). The monitoring must be sufficient to detect the changes that would alter the risk.

### Monitor Performance Against Contractual And Compliance Requirements

The ongoing monitoring must track the third party's performance against the contractual and compliance requirements, not just the operational service levels. This includes the compliance obligations (the data protection commitments, the anti-bribery certification, the sanctions screening), the control operation (the security controls, the access controls, the process controls), and the reporting obligations (the incident reports, the periodic compliance reports, the certifications).

Collect the evidence that verifies the requirements are met: the SOC 2 reports, the security attestations, the compliance certifications, the incident reports, the periodic self-assessments. A monitoring program that tracks only operational metrics (uptime, response time) misses the compliance performance, and a third party can meet the operational service levels while failing the compliance obligations.

### Monitor For Changes In The Third Party's Risk Profile

The third party's risk profile changes over the relationship, and the monitoring must detect the changes: a change in ownership (acquisition by a high-risk entity), a change in financial condition (deterioration that increases fraud or failure risk), a change in conduct (regulatory action, litigation, adverse media), a change in the operating environment (a new jurisdiction, a new subprocessor). These changes can elevate a low-risk third party to high-risk.

Establish the mechanisms to detect changes: periodic re-screening (sanctions, adverse media, litigation), periodic financial review (for financially sensitive third parties), change notification requirements in the contract (ownership change, subprocessor change), and continuous monitoring tools (adverse media alerts, financial distress indicators). A monitoring program that assesses the third party only at the periodic review, without continuous change detection, will miss the changes that occur between reviews.

### Define Third-Party Incident Reporting Requirements

The contract must require the third party to report incidents that affect the organization's compliance or risk: a data breach, a security incident, a bribery allegation, a sanctions match, a control failure, a regulatory inquiry, a business disruption. The reporting must be timely (within hours for a data breach, within days for other incidents), complete (the facts, the scope, the mitigation), and ongoing (updates as the incident develops).

Without a contractual reporting requirement, the third party may delay or omit reporting, and the organization discovers the incident late. Define the reporting triggers, the timeline, the content, and the escalation path. The reporting requirement is the mechanism that makes the third party's incidents visible to the organization in time to respond.

### Respond To Third-Party Incidents With A Defined Process

When a third-party incident occurs, the organization must respond with a defined process: the assessment of the incident's impact on the organization, the containment and remediation actions (at the third party and at the organization), the regulatory and stakeholder notifications (if the organization has obligations), the root cause analysis, and the decision on the relationship's continuation.

Integrate the third-party incident response with the organization's incident management (the data breach response, the anti-bribery investigation, the business continuity activation). The organization may have regulatory obligations triggered by the third party's incident (a data breach notification, a suspicious activity report), and the response must identify and meet these obligations within their timelines. A third-party incident managed as the third party's problem, without the organization's own assessment and response, leaves the organization's obligations unmet.

### Escalate And Remediate Control Failures

When ongoing monitoring detects a control failure or compliance breach at the third party, the organization must escalate and remediate: the assessment of the failure's severity, the requirement for a corrective action plan, the timeline for remediation, the verification of the remediation, and the consequences for continued or repeated failure (enhanced monitoring, financial penalties, termination).

Track the remediation to completion, and verify the corrective actions are effective, not merely implemented. A control failure that is identified but not remediated is a known risk that the organization has chosen to accept without formal acceptance. For repeated or severe failures, escalate to the termination decision, because a third party that cannot or will not remediate is a relationship that must end.

### Plan For Third-Party Disruption And Exit

Ongoing monitoring must include the preparedness for the third party's disruption or exit: the contingency for a service interruption (a backup provider, an in-house capability), the exit plan (the data return, the transition, the substitution), and the trigger conditions for activation. A third party that becomes unavailable (through failure, termination, or disruption) must not create an unmanageable gap.

For critical third parties, maintain and test the contingency and exit plan, because the disruption of a critical supplier is an operational resilience event. The exit plan developed during onboarding must be kept current through the ongoing monitoring, because the conditions for exit (the data, the processes, the dependencies) change over the relationship.

## Common Traps

### Monitoring Stops At Onboarding

Treating the relationship as managed once the contract is signed, with no ongoing monitoring, leaves the changing risk undetected. The trap is the onboarding-focus bias.

### Monitoring Only Operational Metrics

Tracking uptime and response time but not the compliance obligations and control operation misses the compliance performance. The trap is operational metrics without compliance metrics.

### No Continuous Change Detection

Assessing the third party only at the periodic review, without sanctions re-screening or adverse media alerts, misses the changes between reviews. The trap is periodic-only monitoring.

### No Contractual Incident Reporting

Without a reporting requirement, the third party delays or omits reporting, and the organization discovers the incident late. The trap is relying on voluntary reporting.

### Treating The Third-Party Incident As The Third Party's Problem

Managing the incident without assessing the organization's own obligations (regulatory notifications) leaves the organization's duties unmet. The trap is the boundary error.

### Control Failures Identified But Not Remediated

A known control failure that is not tracked to remediation is an accepted risk without formal acceptance. The trap is identification without closure.

### No Contingency Or Exit Plan For Critical Third Parties

A critical supplier disruption without a contingency creates an unmanageable gap. The trap is dependency without resilience.

## Self-Check

- Is ongoing monitoring risk-based, with intensity matched to the third party's risk tier?
- Does monitoring track performance against contractual and compliance requirements, not just operational service levels?
- Does monitoring detect changes in the third party's risk profile (ownership, financial, conduct, environment) through continuous mechanisms, not just periodic review?
- Does the contract require timely and complete incident reporting, with defined triggers and timelines?
- Is there a defined process for responding to third-party incidents, integrated with the organization's incident management and regulatory obligations?
- Are control failures escalated, remediated with corrective action plans, and verified, with consequences for repeated failure?
- Is there a contingency and exit plan for critical third parties, maintained and tested through ongoing monitoring?
- Is the ongoing monitoring sufficient to detect the changes and failures that would alter the risk, before they become the organization's failure?
