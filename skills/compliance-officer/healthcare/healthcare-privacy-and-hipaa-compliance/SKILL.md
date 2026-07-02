---
name: healthcare_privacy_and_hipaa_compliance.md
description: Use when the agent is assessing HIPAA privacy and security rules, protected health information handling, breach notification, patient rights, covered entity and business associate obligations, electronic health records, or health data sharing for a provider, payer, health tech, or wellness company.
---

# Healthcare Privacy And HIPAA Compliance

Health information is among the most sensitive and most regulated data categories. In the United States, HIPAA governs covered entities and business associates, but it is not the only rule: state health privacy laws, substance use disorder records, mental health records, reproductive health, genetic information, and consumer health data laws create overlapping and sometimes stricter obligations. The central compliance challenge is that health data flows through many hands, each with different permitted uses, and a disclosure that is lawful for treatment can be unlawful for marketing, research, or product development.

Use this skill before advising on handling protected health information, reviewing a HIPAA compliance program, structuring a business associate relationship, planning a health data breach response, or building a health tech product. The goal is to make the agent identify the entity's status, the data categories, the permitted purposes, the minimum necessary standard, and the patient rights before concluding a data use is compliant.

## Core Rules

### Determine Covered Entity Or Business Associate Status

HIPAA obligations depend on the entity's role. The same data handling can be compliant for a covered entity and noncompliant for an unrelated business.

Classify:

- covered entity (health plan, healthcare clearinghouse, healthcare provider that transmits health transactions electronically);
- business associate performing functions involving PHI on behalf of a covered entity;
- subcontractor of a business associate, which inherits business associate obligations;
- organization that is neither, in which case HIPAA may not apply but other health privacy laws may.

Status can change with activities. A technology company can be a business associate for one service and an unrelated consumer business for another. Misclassification is a root cause of noncompliance.

### Apply The Minimum Necessary Standard

HIPAA requires that uses, disclosures, and requests of PHI be limited to the minimum necessary to accomplish the purpose, with exceptions for treatment, patient requests, and required disclosures.

Assess:

- whether each use and disclosure is the minimum necessary;
- role-based access controls limiting who sees what;
- segmentation of treatment, payment, and operations uses;
- restrictions on access by job function;
- review of broad or standing disclosures;
- safeguards against over-collection and over-retention.

Minimum necessary does not apply to disclosures to or requests by a healthcare provider for treatment, but it applies broadly elsewhere. A data lake that gives all staff access to all PHI violates the standard.

### Honor Patient Rights Under Privacy Rule

Patients have specific rights that require processes, not merely policies.

Implement processes for:

- right to access and obtain a copy of their PHI in the form and format requested when readily producible;
- right to direct transmission to a third party;
- right to request amendments;
- right to an accounting of disclosures;
- right to request restrictions, including the payer-disclosure restriction;
- right to confidential communications;
- right to a notice of privacy practices.

Access requests have deadlines and cannot be stonewalled. Fees must be reasonable and cost-based. Denials must meet specific grounds and include review rights.

### Implement The Security Rule Safeguards

The Security Rule requires administrative, physical, and technical safeguards for electronic PHI.

Cover:

- risk analysis as the foundation, updated periodically;
- risk management plan addressing identified risks;
- designated security official;
- workforce training and sanctions;
- access controls, unique user identification, and automatic logoff;
- encryption and decryption where reasonable and appropriate;
- audit controls logging access and activity;
- integrity controls;
- transmission security;
- contingency plan including backups and disaster recovery;
- facility access and device controls;
- business associate contract requirements for security.

The risk analysis is the cornerstone. Without a current, documented risk analysis, the security program cannot be defended.

### Manage Business Associate Contracts And Oversight

A covered entity can be liable for a business associate's impermissible use or disclosure if no compliant contract exists, and business associates have direct liability under HIPAA.

For each business associate relationship:

- execute a business associate agreement with required safeguards and permitted uses;
- flow down obligations to subcontractors;
- ensure the business associate uses PHI only as the agreement permits;
- require breach notification within defined timelines;
- require return or destruction of PHI at termination;
- conduct due diligence on the business associate's safeguards;
- monitor for material breaches of the agreement;
- address permitted uses for treatment, payment, healthcare operations, and limited data set uses.

A missing or expired business associate agreement is itself a violation and can extend liability for the associate's conduct.

### Execute Breach Notification Correctly

Breaches of unsecured PHI trigger notification duties to individuals, the regulator, and sometimes the media.

Apply the breach risk assessment:

- whether the information was unsecured (not rendered unusable);
- who acquired or accessed the information;
- whether it was actually viewed or only potentially;
- whether the recipient was authorized;
- extent to which the risk has been mitigated.

If a breach is presumed absent a demonstrated low probability, notify:

- affected individuals without unreasonable delay and within the deadline;
- the regulator at thresholds for magnitude;
- media for large breaches;
- business associates notifying covered entities;
- covered entities notifying individuals for associate breaches.

The breach risk assessment must be documented. Encryption with destruction of the key provides a safe harbor from breach notification.

### Navigate Overlapping State And Sectoral Health Laws

HIPAA is a floor. Many state and sectoral rules are stricter or reach non-HIPAA entities.

Identify:

- state medical privacy and confidentiality laws;
- substance use disorder records with heightened protection;
- mental health and psychotherapy notes;
- reproductive health data protections;
- genetic information privacy laws;
- consumer health data laws reaching wellness apps and devices outside HIPAA;
- minor consent and confidentiality rules;
- public health reporting duties;
- workers' compensation exceptions.

A wellness app that is not a HIPAA covered entity may still be subject to consumer health data laws that require consent, prohibit certain uses, and mandate deletion rights.

### Control Research And Secondary Use Of Health Data

Using PHI for research, product development, AI training, or commercial purposes requires specific authorization or a compliant pathway.

Assess:

- whether the use is treatment, payment, or healthcare operations;
- whether a limited data set with a data use agreement suffices;
- whether de-identification under the safe harbor or expert determination method applies;
- whether an authorization is required and whether it is valid;
- institutional review board or privacy board approval where applicable;
- prohibitions on conditioning treatment on authorization for unrelated uses;
- commercial use restrictions.

De-identification is not trivial. Re-identification risk and the expert determination method require expertise and documentation.

## Common Traps

### Assuming HIPAA Does Not Apply Because The Company Is A Tech Firm

A technology company handling PHI on behalf of a covered entity is a business associate with direct HIPAA duties. Status depends on function, not industry.

### Minimum Necessary Ignored For Internal Access

Broad access to PHI across roles violates the minimum necessary standard. Access must be role-based and justified.

### Business Associate Agreement Missing Or Expired

An absent or outdated BAA is a violation and extends liability. Track expiration and renewal.

### Breach Notification Based On Optimism

The breach presumption favors notification unless a documented risk assessment shows low probability of compromise. Encryption safe harbor requires the key to be destroyed or compromised.

### Patient Access Requests Delayed Or Overcharged

Access rights have deadlines and cost limits. Stalling or overcharging is a violation and an enforcement priority.

### Wellness App Treated As Outside All Health Privacy Law

Consumer health data laws increasingly reach non-HIPAA wellness apps. Consent, use limits, and deletion duties may apply even without HIPAA.

### De-Identification Treated As A Label

De-identification requires meeting a specific method. Re-identification risk must be assessed, and expert determination requires qualified analysis.

### Secondary Use Without Authorization

Using PHI for marketing, AI training, or product development generally requires authorization or a compliant pathway. Operational use does not authorize commercial use.

## Self-Check

- Is the entity classified as covered entity, business associate, subcontractor, or none, with the classification justified by function?
- Is the minimum necessary standard applied to uses, disclosures, and requests, with role-based access controls?
- Are patient rights to access, amend, accounting, restriction, confidential communications, and notice supported by documented processes with deadlines and cost limits?
- Does the security program rest on a current risk analysis, with administrative, physical, and technical safeguards including access controls, encryption, audit logs, and contingency plans?
- Are business associate agreements executed, flowed down to subcontractors, monitored, and terminated with return or destruction of PHI?
- Is the breach risk assessment documented, with notification to individuals, regulator, and media where required, and is the encryption safe harbor understood?
- Are overlapping state, substance use, mental health, reproductive, genetic, and consumer health data laws identified and applied where stricter or broader than HIPAA?
- Is secondary use of PHI for research, AI training, or commercial purposes gated by authorization, limited data set, de-identification, or IRB approval as required?
- Are de-identification methods documented with re-identification risk assessed?
- Is the distinction between treatment, payment, operations, and impermissible commercial use clearly drawn for each data flow?