---
name: security_incident_response_and_breach_notification.md
description: Use when the agent is responding to cybersecurity incidents, evaluating data breach notification obligations, managing incident investigation and containment, coordinating with law enforcement and regulators, or ensuring timely breach notification under GDPR, state breach laws, HIPAA, SEC rules, and sector-specific cybersecurity regulations.
---

# Security Incident Response And Breach Notification

Security incident response and breach notification governs what happens when a cybersecurity incident occurs. The defining feature is that the response window is compressed, that notification obligations proliferate across jurisdictions and regulators, and that decisions made in the first hours shape legal liability for years. The central difficulty is that "incident" and "breach" are legally distinct, that notification triggers and deadlines differ by regime, and that the tension between speed and accuracy must be managed under regulatory and public pressure.

Use this skill before advising on incident response, breach assessment, notification decisions, or regulatory coordination. The goal is to make the agent identify the incident scope, the notification triggers, the applicable deadlines, and the coordination requirements before concluding that the response is compliant.

## Core Rules

### Execute A Structured Incident Response Process

Incident response must follow a defined, practiced process.

Follow:

- detection and identification of the incident;
- containment to prevent further damage;
- eradication of the threat;
- recovery and restoration of systems;
- post-incident review and lessons learned;
- documentation of each phase and decisions made;
- preservation of evidence from the outset;
- the role of incident response retainers and pre-established teams.

The response process must be defined before an incident occurs. Containment must balance stopping the threat against preserving evidence. Eradication must be thorough to prevent re-entry. Recovery must verify system integrity before restoration. Documentation from the outset supports legal defense and regulatory reporting. Evidence preservation must begin immediately to avoid spoliation.

### Assess Whether An Incident Constitutes A Reportable Breach

Not every incident is a breach; not every breach requires notification.

Assess:

- whether personal data was accessed, acquired, or exfiltrated;
- the categories of data involved (sensitive data triggers stricter rules);
- the number of individuals affected;
- whether the data was encrypted or otherwise protected;
- the harm risk to affected individuals;
- the applicable notification regimes and their triggers;
- the distinction between security incidents (operational) and personal data breaches (privacy);
- the documentation of the risk assessment and notification decision.

The breach assessment determines notification obligations. GDPR defines a personal data breach as any security breach leading to accidental or unlawful destruction, loss, alteration, unauthorized disclosure of, or access to personal data. US state laws typically define breach as unauthorized acquisition of computerized data that compromises security, confidentiality, or integrity. The risk of harm assessment varies: GDPR requires notification unless unlikely to result in risk to individuals; US state laws have harm thresholds that may allow deferral.

### Map All Applicable Notification Obligations And Deadlines

Notification obligations vary by regime and can be concurrent.

Cover:

- GDPR: notify the supervisory authority within 72 hours of awareness (unless unlikely to result in risk);
- GDPR: notify affected individuals without undue delay if high risk;
- US state breach notification laws: varying deadlines (e.g., "without unreasonable delay," specific day counts, some with maximums);
- HIPAA Breach Notification Rule: 60 days to affected individuals; HHS notification (60 days for 500+, annually for smaller);
- SEC cybersecurity disclosure rules: material incident disclosure on Form 8-K within 4 business days of materiality determination;
- NYDFS Part 500: 72 hours to superintendent for cybersecurity events;
- sector-specific requirements (GLBA, FERPA, telecom);
- law enforcement coordination and permissible delays;
- international regimes (NIS2, DORA, PIPL).

Deadlines are strict and concurrent. The 72-hour GDPR clock starts at awareness, not confirmation. SEC materiality determination can extend the clock but must be diligent. State laws have varying triggers and harm thresholds. Law enforcement may request delay for investigation. All applicable regimes must be mapped and tracked simultaneously.

### Coordinate With Internal And External Stakeholders

Incident response requires coordinated action.

Coordinate:

- incident response team (technical, legal, compliance, communications, executive);
- external counsel and forensic investigators (under privilege);
- law enforcement (FBI, Secret Service, international equivalents);
- regulators (data protection authorities, sector regulators, SEC);
- cyber insurance carrier and broker;
- affected individuals and customers;
- business partners and data partners;
- media and public relations.

Coordination must be pre-planned. External counsel and forensic investigators should be engaged under privilege. Law enforcement engagement may support investigation and qualify for law enforcement delay. Cyber insurance should be notified early per policy terms. Regulator communication must be accurate and coordinated. Customer and partner notification must be timely and consistent.

### Preserve Evidence And Maintain Privilege

Evidence preservation and privilege protection must begin immediately.

Implement:

- preservation of system logs, images, and forensic evidence;
- litigation hold issuance to relevant custodians;
- engagement of forensic investigators through external counsel to protect privilege;
- documentation of the incident timeline and decisions;
- chain of custody for evidence;
- the distinction between factual findings (generally discoverable) and legal advice (privileged);
- coordination with law enforcement without waiving privilege.

Evidence must be preserved from the outset. Spoliation can result in sanctions. Forensic investigation through external counsel maximizes privilege protection. The attorney-client privilege protects legal advice, not underlying facts; the investigation's findings of fact are generally discoverable even if the investigation was conducted at counsel's direction (though work product doctrine may provide some protection). Chain of custody must be maintained.

### Manage Communications And Public Disclosure

Communications during an incident affect liability and reputation.

Manage:

- internal communications (need-to-know, consistency);
- regulatory notifications (accurate, timely, complete);
- affected individual notifications (clear, actionable, with required content);
- public statements and press releases (coordinated, accurate);
- customer and partner communications;
- media inquiries;
- social media monitoring and response;
- the consistency of messaging across channels.

Communications must be accurate, consistent, and coordinated. Inaccurate initial statements that are later corrected create credibility problems. Regulatory notifications must be complete and not misleading. Individual notifications must include required content (what happened, what data, what to do, contact information). Over-disclosure can create unnecessary alarm; under-disclosure can create liability.

### Handle Ransomware And Extortion Incidents

Ransomware presents unique response challenges.

Address:

- the decision to pay or not pay ransom (legal, ethical, operational);
- OFAC sanctions screening of ransomware actors;
- the prohibition on payments to sanctioned entities;
- the risk of payment without decryption or data return;
- law enforcement engagement before payment decisions;
- the reporting obligations for ransomware payments;
- the risk of repeat attacks and double extortion;
- business continuity alternatives to payment.

Ransomware payment decisions involve legal, ethical, and operational factors. OFAC sanctions prohibit payments to sanctioned entities, with strict liability. Law enforcement should be engaged before payment. Payment does not guarantee decryption or data return. Double extortion (encryption plus data theft) adds breach notification obligations. Business continuity (backups, manual processes) may provide alternatives.

### Conduct Post-Incident Review And Continuous Improvement

Post-incident review drives improvement.

Conduct:

- a structured post-incident review (what happened, what worked, what failed);
- root cause analysis;
- identification of control gaps and remediation;
- updates to incident response plans based on lessons learned;
- updates to detection and prevention controls;
- training and awareness improvements;
- documentation of the review and remediation tracking;
- reporting to senior management and the board.

Post-incident review must be blameless and focused on improvement. Root cause analysis identifies the underlying vulnerability. Remediation must be tracked to completion. Lessons learned should update plans and controls. Board reporting demonstrates oversight and drives accountability.

## Common Traps

### Notification Deadline Missed Due To Slow Internal Escalation

The clock starts at awareness; internal delays are not a defense.

### Incident Assessed As Non-Notifiable Without Documented Risk Assessment

Undocumented non-notification decisions are difficult to defend.

### Overbroad Or Underbroad Notification

Notifying too broadly creates unnecessary alarm; too narrowly creates liability for missed notifications.

### Forensic Investigation Not Protected By Privilege

Investigation conducted without counsel may be fully discoverable.

### Inconsistent Communications Across Channels

Inconsistent messaging undermines credibility and creates legal exposure.

### Ransomware Payment Without OFAC Screening

Paying a sanctioned entity is a sanctions violation with strict liability.

### No Post-Incident Review Or Remediation Tracking

Failing to learn from incidents guarantees repeat incidents.

## Self-Check

- Is a structured incident response process executed with detection, containment, eradication, recovery, post-incident review, documentation, evidence preservation, and pre-established teams?
- Is the incident assessed for breach status covering personal data access/acquisition, data categories, number affected, encryption status, harm risk, notification triggers, incident vs. breach distinction, and documented risk assessment?
- Are all applicable notification obligations and deadlines mapped including GDPR (72 hours), state laws, HIPAA (60 days), SEC (4 business days of materiality), NYDFS (72 hours), sector-specific, law enforcement delay, and international regimes?
- Are internal and external stakeholders coordinated including IR team, external counsel/forensics under privilege, law enforcement, regulators, cyber insurance, affected individuals, partners, and media?
- Is evidence preserved and privilege maintained with log/image preservation, litigation hold, counsel-directed forensics, timeline documentation, chain of custody, fact vs. advice distinction, and law enforcement coordination?
- Are communications and public disclosure managed with internal consistency, accurate regulatory notifications, actionable individual notifications, coordinated public statements, customer/partner comms, media handling, and channel consistency?
- Are ransomware and extortion incidents handled with pay/no-pay decision framework, OFAC screening, law enforcement engagement, payment risk assessment, reporting obligations, double extortion, and business continuity alternatives?
- Is post-incident review and continuous improvement conducted with structured review, root cause analysis, control gap remediation, plan updates, detection/prevention improvements, training, documentation, and board reporting?
- Is the incident response plan tested at least annually through tabletop exercises?
- Are breach notification decisions documented with rationale to support later regulatory or litigation defense?