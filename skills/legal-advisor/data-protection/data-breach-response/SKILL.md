---
name: data_breach_response.md
description: Use when the agent is responding to a data security breach or suspected incident, assessing notification duties to regulators and affected individuals, preserving evidence and investigation integrity, managing law enforcement and insurer coordination, or evaluating breach liability and remediation before escalation to privacy, cybersecurity, or litigation counsel.
---

# Data Breach Response

A data breach is a legal event as much as a technical one. The first hours determine whether the organization meets its notification deadlines, preserves evidence, avoids harmful statements, and coordinates with insurers and authorities. Agents often focus on technical containment and miss the legal clock: many regimes impose notification duties measured in hours, require specific content, and turn on nuanced risk assessments that must be documented. A delayed or mishandled response can convert a manageable incident into regulatory penalties, class actions, and reputational damage.

Use this skill before a breach occurs (to prepare), during an active incident, or when assessing notification duties and liability after the fact. The goal is to force the agent to connect detection, containment, risk assessment, notification duties, evidence preservation, communications, and post-incident remediation, and to escalate to privacy, cybersecurity, and litigation counsel for any significant incident. This skill is decision support, not legal advice.

## Core Rules

### Activate The Incident Response Plan Immediately

Speed and structure matter from the first indication of a breach.

Ensure the plan covers:

- a defined incident-response team (security, privacy, legal, communications, executive);
- activation triggers and severity classification;
- roles and decision authority;
- internal escalation paths;
- external contacts (forensics, outside counsel, insurer, law enforcement);
- a bridge or war-room mechanism;
- documentation from the outset.

An organization without a plan, or that improvises under pressure, loses time and makes avoidable errors. Prepare and rehearse before an incident.

### Contain And Investigate With Evidence Integrity

Containment must not destroy the evidence needed to understand and prove the incident.

Address:

- isolating affected systems without premature wiping;
- preserving logs, memory images, and forensic snapshots;
- maintaining chain of custody for evidence;
- engaging qualified forensic investigators (often through counsel to protect privilege);
- scoping the intrusion (what data, whose data, how accessed, how long);
- eradicating the threat and closing vulnerabilities;
- validating restoration from clean backups.

Wiping systems to stop the bleeding before capturing evidence can foreclose understanding the scope and defending later claims. Preserve first, then remediate.

### Assess The Risk To Determine Notification Duties

Notification duties turn on a risk assessment that must be documented.

Assess:

- the categories and sensitivity of data involved;
- the number of individuals affected;
- whether data was actually accessed, exfiltrated, or merely exposed;
- the likelihood of harm (financial, identity, safety, reputational);
- identifiability of affected individuals;
- encryption or other mitigating factors;
- the applicable legal regimes and their thresholds.

Many regimes require notification only where the breach is likely to result in a risk to individuals, but the threshold and the regulator-versus-individual duties differ by law. Document the assessment and its basis, as regulators will scrutinize it.

### Meet Notification Deadlines And Content Requirements

Notification deadlines are short and content is prescribed.

Track per regime:

- regulator notification deadlines (GDPR: 72 hours where likely risk; U.S. state laws vary, some "without unreasonable delay" or specific windows);
- affected-individual notification triggers and content;
- sector-specific duties (health, financial, telecom) that may be stricter;
- content requirements (nature, categories, likely consequences, measures taken, contact point, mitigation advice);
- method of notice (written, electronic, substitute notice thresholds);
- law-enforcement and media notification for large breaches.

Missing a deadline or providing non-compliant content is an independent violation. Calendar the earliest applicable deadline and treat it as hard.

### Manage Communications To Avoid Harmful Statements

Public and individual communications during a breach are evidence and reputation-defining.

Address:

- coordination between legal, security, and communications;
- accuracy and avoidance of premature certainty;
- avoiding speculation about cause, scope, or attribution;
- consistency across channels;
- avoiding admissions that may waive defenses or insurance;
- regulatory cooperation without oversharing unverified facts;
- guidance to affected individuals on protective steps.

Overstating or understating the incident, or making unverified attribution claims, creates legal and reputational risk. Communicate carefully and update as facts develop.

### Coordinate With Insurers, Law Enforcement, And Authorities

External coordination affects coverage, recovery, and compliance.

Address:

- timely notice to cyber insurers and compliance with policy conditions;
- preserving coverage by avoiding actions that breach the policy;
- law-enforcement engagement where criminal conduct is involved;
- regulator cooperation and voluntary disclosure considerations;
- coordination with processors and third parties whose data is involved;
- managing joint investigations and information sharing.

Late insurer notice or unauthorized actions can forfeit coverage. Notify insurers early and follow their conditions.

### Preserve Privilege Over Investigation Work

Forensic and legal investigation work may be privileged if structured correctly.

Address:

- engaging forensic investigators through counsel;
- directing the investigation for the purpose of legal advice;
- segregating privileged and non-privileged materials;
- limiting distribution;
- understanding that underlying facts are not privileged;
- coordinating with regulators in a way that preserves privilege where possible.

Privilege over breach investigations is contested and fact-specific. Structure it deliberately and do not assume it attaches automatically.

### Handle Affected Individuals, Class Actions, And Liability

Post-breach liability can be substantial and cumulative.

Address:

- support for affected individuals (credit monitoring, guidance, hotlines);
- documentation of mitigation to reduce damages;
- defense of regulatory investigations and private claims;
- class-action exposure and jurisdictional considerations;
- contractual indemnities and limitations with vendors;
- board and stakeholder reporting;
- long-tail monitoring for downstream harm.

Affected-individual support is both a mitigation measure and a factor regulators and courts consider. Provide meaningful assistance, not just a notice.

### Remediate And Improve Post-Incident

A breach reveals weaknesses that must be fixed and documented.

Address:

- root-cause analysis;
- technical and organizational remediation;
- updates to policies, controls, and training;
- verification that fixes are effective;
- documentation of remediation for regulators and litigants;
- lessons-learned review and plan update.

Remediation that is promised but not implemented, or not verified, undermines credibility with regulators and courts. Close the loop and document it.

## Common Traps

### No Plan Or Improvised Response

Improvising under pressure loses time and causes errors. Prepare and rehearse.

### Destroying Evidence While Containing

Premature wiping forecloses scoping and defense. Preserve first.

### Skipping Or Under-Documenting The Risk Assessment

Notification duties turn on a documented assessment. Regulators will scrutinize it.

### Missing Notification Deadlines

Deadlines are short and hard. Calendar the earliest applicable one.

### Premature Or Speculative Public Statements

Unverified claims create legal and reputational risk. Communicate carefully.

### Late Insurer Notice Or Policy-Breaching Actions

These can forfeit coverage. Notify early and follow conditions.

### Assuming Privilege Attaches Automatically

Privilege over breach work is contested. Structure it through counsel.

### Promised Remediation Not Implemented

Unfixed weaknesses undermine credibility. Close the loop and verify.

## Self-Check

- Is the incident-response plan activated immediately with a defined team, severity classification, roles, escalation, external contacts, war-room, and documentation?
- Are affected systems contained and investigated with evidence preservation, forensic snapshots, chain of custody, qualified investigators (often via counsel), scoping, eradication, and clean restoration?
- Is the risk assessment documented covering data categories and sensitivity, number affected, access versus exfiltration, harm likelihood, identifiability, mitigating factors, and applicable regimes and thresholds?
- Are regulator and individual notification deadlines (72-hour, state-law windows, sector-specific), content requirements, methods, and law-enforcement and media duties met and calendared?
- Are communications coordinated, accurate, non-speculative, consistent, free of harmful admissions, and regulator-cooperative without oversharing?
- Are insurers notified timely with policy conditions followed, law enforcement engaged where criminal, regulators coordinated, and processors and third parties aligned?
- Is privilege structured through counsel-directed investigation, material segregation, limited distribution, and understanding that facts are not privileged?
- Are affected individuals supported, mitigation documented, regulatory and private-claim defense prepared, contractual indemnities assessed, and board and stakeholder reporting handled?
- Is post-incident remediation (root cause, technical and organizational fixes, policy and training updates, verification, documentation, lessons learned) completed and verified?
- Does the output escalate significant incidents, multi-regime notification, privilege structuring, and class-action exposure to qualified privacy, cybersecurity, and litigation counsel?