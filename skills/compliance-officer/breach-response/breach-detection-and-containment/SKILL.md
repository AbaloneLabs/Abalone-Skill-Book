---
name: breach-detection-and-containment.md
description: Use when the agent is responding to a suspected data breach or security incident, triaging and classifying incidents, containing and isolating affected systems, preserving forensic evidence, coordinating the initial incident response, or determining whether a personal data breach has occurred under data protection law.
---

# Breach Detection and Containment

The first hours of a suspected data breach determine how much damage occurs and how defensibly the organisation responds. Detection is often delayed (the average breach goes undetected for weeks), containment decisions are made under pressure with incomplete information, and evidence that is not preserved in the first hours is lost forever. The judgment problem is not only technical — it is also legal, because the moment a personal data breach is confirmed, a regulatory clock starts ticking (72 hours under the GDPR) and every action or inaction is later scrutinised. This skill addresses the judgment involved in detecting, triaging, and containing a breach while preserving the conditions for a lawful and effective response.

## Core Rules

### Define what constitutes a personal data breach broadly

A personal data breach is any breach of security leading to accidental or unlawful destruction, loss, alteration, unauthorised disclosure of, or access to personal data. This is broader than "a hacker exfiltrated data." It includes:

- Confidentiality breaches (unauthorised disclosure or access);
- Integrity breaches (unauthorised alteration);
- Availability breaches (loss of access, destruction, ransomware encryption).

A lost laptop, an email sent to the wrong recipient, a misconfigured cloud bucket, a ransomware lockout — all are personal data breaches if personal data is affected. Train responders to recognise the full scope, not only cyber-exfiltration.

### Establish detection channels and triage criteria

Breaches are detected through many channels: security monitoring, employee reports, vendor notifications, data subject complaints, or external researcher reports. Establish:

- A single intake point for all suspected incidents (a security or privacy hotline);
- Defined triage criteria to classify the severity and type of the incident;
- An on-call response team with defined activation thresholds;
- A clear distinction between a security incident (technical event under investigation) and a confirmed personal data breach (legal trigger).

Triage criteria should escalate quickly: when in doubt, treat it as a potential breach and involve the DPO and privacy counsel early. Late triage is the most common cause of missed notification deadlines.

### Contain first, but preserve evidence

The immediate priority is to stop the bleeding: isolate affected systems, revoke compromised credentials, block malicious access, and prevent further data loss. But containment must not destroy forensic evidence. Balance:

- **Containment actions**: disconnect systems, reset credentials, block IPs, disable accounts, restore from clean backups;
- **Evidence preservation**: capture logs, memory images, disk images, network captures, and configuration states before they are overwritten or systems are rebuilt.

Document the containment actions taken, by whom, and when, and the evidence captured. Actions taken without documentation are difficult to reconstruct and defend.

### Confirm whether a personal data breach has occurred

Not every security incident is a personal data breach. The confirmation step determines whether personal data was actually affected. Investigate:

- Was personal data present on the affected systems?
- Was it accessed, exfiltrated, altered, or destroyed, or only potentially exposed?
- What categories and volumes of data were involved?
- Who are the affected data subjects?

The confirmation must be based on evidence (logs, forensic analysis), not assumption. Document the basis for the determination. An unconfirmed incident that is treated as a non-breach and later revealed to be a breach is a missed notification with compounded exposure.

### Start the regulatory clock deliberately and accurately

Under the GDPR, the controller must notify the supervisory authority within 72 hours of becoming "aware" of a personal data breach. "Aware" means the controller has a reasonable degree of certainty that a breach has occurred. The clock starts at awareness, not at detection. Document when awareness was reached and the basis for that determination, because the timing will be scrutinised. Stretching the definition of awareness to delay notification is a violation and is detectable from the incident timeline.

### Maintain an incident response plan that is tested, not theoretical

An incident response plan must exist, be accessible during an incident (not on the compromised system), and be tested through exercises. The plan should define:

- Roles and responsibilities (incident lead, technical, legal, communications, DPO);
- Activation and escalation criteria;
- Containment, investigation, and evidence procedures;
- Notification decision pathways;
- Communication protocols (internal, regulator, data subjects, media).

A plan that has never been exercised will fail under real pressure. Conduct tabletop exercises periodically.

### Coordinate across security, privacy, legal, and communications from the start

Breach response is multidisciplinary. Involve from the earliest stages:

- **Security/IT**: containment, forensics, remediation;
- **Privacy/DPO**: breach determination, regulatory notification, data subject rights;
- **Legal**: privilege, regulatory exposure, law enforcement, litigation hold;
- **Communications**: internal and external messaging, media response;
- **Senior management**: decisions on disclosure, business continuity, resource allocation.

Late involvement of legal or the DPO leads to missed notification deadlines and unprotected communications. Define the team and activate it together.

### Consider law enforcement and third-party notifications

Depending on the breach, there may be obligations or strategic reasons to notify:

- Law enforcement (cybercrime, extortion);
- Cybersecurity authorities (under regimes like NIS2 or sector-specific requirements);
- Insurers (under cyber policy notification conditions);
- Affected processors or controllers (if the breach occurred at a processor);
- Other regulators (sector-specific, financial, health).

Map the notification landscape early so obligations are not missed in the chaos.

## Common Traps

### Treating only cyber-exfiltration as a breach

Lost devices, misdirected emails, and misconfigured storage are breaches too. Narrow definitions cause missed notifications for the most common breach types.

### Containment that destroys forensic evidence

Wiping and rebuilding systems before capturing logs and images eliminates the evidence needed to confirm scope, prove notification decisions, and support any enforcement defence. Preserve before rebuilding.

### Late triage and late DPO involvement

The incident is handled by IT for days before privacy and legal are informed. The 72-hour clock may have started, and notification is missed or rushed. Involve the DPO and counsel early.

### Stretching "awareness" to delay the clock

Claiming the organisation was not "aware" until late, despite clear indicators, is detectable from the incident timeline and is itself a violation. Document awareness honestly.

### No incident response plan, or a plan that has never been tested

Under real pressure, an untested plan fails. Roles are unclear, escalation is ad hoc, and critical steps are missed. Exercise the plan.

### Uncoordinated communications

Different functions communicate inconsistently to regulators, data subjects, and the media, creating contradictions and reputational damage. Centralise communications.

## Self-Check

- Is the definition of personal data breach understood broadly (confidentiality, integrity, availability), beyond cyber-exfiltration?
- Are there defined detection channels, triage criteria, and an on-call response team with activation thresholds?
- Did containment actions preserve forensic evidence (logs, images, captures) before systems were rebuilt?
- Is the breach confirmation based on evidence of actual data impact, with the basis documented?
- Is the moment of awareness documented with reasoning, and did the 72-hour clock start accurately?
- Is there a tested, accessible incident response plan defining roles, escalation, containment, notification, and communications?
- Are security, privacy/DPO, legal, and communications coordinated from the start of the response?
- Are law enforcement, cybersecurity authorities, insurers, processors, and other regulators considered in the notification landscape?
