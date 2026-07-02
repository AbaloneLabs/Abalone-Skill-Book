---
name: personal_data_collection_and_consent_compliance.md
description: Use when the agent is designing data collection practices, managing consent mechanisms, evaluating lawful bases for processing personal data, drafting privacy notices, or ensuring compliance with GDPR, CCPA/CPRA, and other privacy laws for personal data collection and consent.
---

# Personal Data Collection And Consent Compliance

Personal data collection and consent compliance governs how organizations lawfully obtain, use, and disclose personal information. The defining feature is that every data processing activity must rest on a valid lawful basis, must be transparent to the individual, and must be limited to what was disclosed. The central difficulty is that consent standards differ by jurisdiction and by sensitivity of data, that collection practices often outpace privacy notices, and that "we obtained consent" is not sufficient when the consent was not informed, specific, or freely given.

Use this skill before advising on data collection, consent flows, privacy notices, lawful bases, or data minimization. The goal is to make the agent identify the lawful basis for each processing activity, verify that notice and consent match the actual data flows, and check that collection is minimized and purpose-limited before concluding that collection is compliant.

## Core Rules

### Establish A Lawful Basis For Every Processing Activity

Every collection and use of personal data must have a valid legal basis.

Map:

- consent (freely given, specific, informed, unambiguous);
- contract necessity;
- legal obligation;
- vital interests;
- public task;
- legitimate interests (with balancing test);
- the special rules for sensitive data (precise geolocation, biometric, health, children's data);
- the difference between GDPR bases and CCPA/CPRA sale/share opt-out models.

Lawful basis is determined before processing begins and documented. Sensitive data categories require enhanced protection and often explicit consent. The lawful basis cannot be swapped retroactively to rescue a non-compliant collection. Legitimate interests require a documented balancing test weighing the organization's interest against the individual's rights.

### Make Consent Meaningful, Not Performative

Valid consent requires genuine choice and control.

Ensure:

- consent is freely given (no bundling, no detriment for refusal);
- consent is specific (granular per purpose, not blanket);
- consent is informed (clear who, what, why, how long);
- consent is unambiguous (clear affirmative action, not pre-ticked boxes);
- consent is as easy to withdraw as to give;
- records of when, what, and how consent was obtained;
- separate consent for different purposes where required;
- age-appropriate consent for minors (parental consent thresholds).

Consent that is bundled with service terms, conditioned on receiving a service, or presented as a pre-checked box is invalid. Consent walls ("agree or leave") may not be freely given. Withdrawal must be as simple as giving consent. Children's consent has age thresholds (GDPR: 16 default, member states may lower to 13; COPPA: 13) and requires verifiable parental consent.

### Align Privacy Notices With Actual Data Practices

Privacy notices must accurately describe what the organization actually does.

Cover:

- the categories of data collected;
- the purposes of collection and processing;
- the legal bases or consent mechanisms;
- the recipients and categories of sharing;
- retention periods;
- individual rights and how to exercise them;
- international transfers and safeguards;
- how individuals can contact the organization;
- updates and material changes to practices.

A notice that omits a data flow, understates retention, or fails to disclose a sharing practice is deceptive and non-compliant. Notices must be layered (short-form plus full policy), written in plain language, and available at the point of collection. Material changes require re-notice or renewed consent.

### Apply Data Minimization And Purpose Limitation

Collect only what is needed for the stated purpose.

Control:

- collecting only data necessary for the disclosed purpose;
- not repurposing data for unrelated secondary uses without a new basis;
- distinguishing between data needed for the service and data for analytics or advertising;
- avoiding collection "just in case";
- periodic purging of data no longer needed;
- limits on retention tied to purpose;
- documentation of the purpose for each data element.

Data minimization requires justifying each data field against the stated purpose. Purpose limitation prevents using data collected for one purpose for an incompatible secondary purpose. Secondary use requires a new lawful basis or consent. Retention must have a defined period and deletion mechanism.

### Manage Special Category And Sensitive Data

Sensitive data requires enhanced protections and often explicit consent.

Address:

- health and medical data;
- biometric and genetic data;
- precise geolocation;
- data about children;
- financial account credentials;
- government identifiers (SSN, national ID);
- racial, ethnic, religious, political, sexual orientation data;
- the heightened consent and security requirements.

Sensitive data triggers stricter rules: explicit consent under GDPR, additional disclosures under CCPA/CPRA, sector-specific rules (HIPAA for health, GLBA for financial). The risk of harm from a breach is higher. Collection should be avoided unless essential, and security controls must be commensurately stronger.

### Handle Children's Data With Heightened Care

Children's data has specific consent and protection requirements.

Cover:

- age thresholds for parental consent (COPPA at 13, GDPR member state ages);
- actual knowledge of child status;
- age-appropriate design codes (UK Children's Code);
- limitations on behavioral advertising to children;
- restrictions on profiling children for marketing;
- simplified privacy notices for children;
- the duty to act in the best interests of the child.

Children's data requires verifiable parental consent below the applicable age threshold. Services "directed to" children or with actual knowledge of child users trigger COPPA. The UK Age Appropriate Design Code imposes additional defaults (high privacy by default, no nudge techniques, no profiling-based advertising).

### Document And Demonstrate Accountability

Accountability requires demonstrable compliance, not just aspiration.

Maintain:

- records of processing activities (ROPA);
- data inventory and data flow maps;
- lawful basis documentation for each processing activity;
- consent records and withdrawal logs;
- privacy impact assessments / data protection impact assessments;
- vendor and processor agreements;
- training records for staff handling personal data;
- evidence of review and update cycles.

Accountability is itself a legal requirement under GDPR (Article 5(2)) and increasingly elsewhere. Regulators ask for evidence, not assertions. ROPA must be current. DPIAs are mandatory for high-risk processing. Consent records must show what was presented and when.

### Coordinate Across Jurisdictions

Privacy laws differ but increasingly converge on similar principles.

Address:

- GDPR (EU/EEA) and UK GDPR;
- CCPA/CPRA (California) and other US state laws (Virginia, Colorado, Connecticut, Utah, and expanding);
- sectoral US laws (HIPAA, GLBA, FERPA, COPPA);
- Latin American laws (LGPD Brazil, LFPDPPP Mexico);
- Asian laws (PIPL China, APPI Japan, PIPA Korea, PDPA Singapore);
- Canadian PIPEDA and provincial laws;
- the interaction and layering of overlapping requirements.

Multi-jurisdictional compliance requires identifying the most protective applicable standard and building to that. Extraterritorial reach means GDPR applies to non-EU controllers processing EU residents' data. State laws in the US create a patchwork requiring careful scoping.

## Common Traps

### Pre-Ticked Boxes Or Implied Consent

Pre-checked boxes, continued browsing, or inactivity do not constitute valid consent.

### Bundled Consent Conditioning Service On Data Use

Making a service conditional on consent for unnecessary data is not freely given.

### Privacy Notice Does Not Match Actual Data Flows

A notice that omits a data flow or sharing practice is deceptive and non-compliant.

### Collecting Data "Just In Case" Without A Defined Purpose

Purposeless collection violates data minimization and purpose limitation.

### Repurposing Data Without A New Lawful Basis

Using data collected for one purpose for an incompatible secondary purpose is unlawful.

### No Records Of Consent Or Processing Activities

Without records, the organization cannot demonstrate compliance to regulators.

### Treating All Data The Same Regardless Of Sensitivity

Applying uniform controls to sensitive and non-sensitive data under-protects sensitive data.

## Self-Check

- Is a lawful basis established and documented for every processing activity, including the choice among consent, contract, legal obligation, legitimate interests, and special rules for sensitive data?
- Is consent freely given, specific, informed, unambiguous, as easy to withdraw as to give, recorded, separated by purpose, and age-appropriate for minors?
- Do privacy notices accurately describe data categories, purposes, legal bases, recipients, retention, individual rights, international transfers, contact methods, and material changes?
- Is data minimization applied so only necessary data is collected, secondary use is limited, service vs. analytics data is distinguished, "just in case" collection is avoided, and retention has defined periods?
- Is special category and sensitive data (health, biometric, geolocation, children's, financial, government identifiers) handled with enhanced consent and security?
- Are children's data requirements met including age thresholds, actual knowledge, age-appropriate design, advertising limits, profiling restrictions, simplified notices, and best-interests duty?
- Is accountability demonstrated through ROPA, data inventories, lawful basis documentation, consent records, DPIAs, vendor agreements, training records, and review cycles?
- Are multi-jurisdictional requirements coordinated including GDPR, UK GDPR, CCPA/CPRA, other US state laws, sectoral US laws, Latin American, Asian, and Canadian laws?
- Is consent withdrawal processed across all downstream systems, not just the original collection point?
- Are privacy notices reviewed whenever data flows, purposes, or sharing practices change?