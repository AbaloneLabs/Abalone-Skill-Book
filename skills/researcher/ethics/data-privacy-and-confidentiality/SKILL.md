---
name: data_privacy_and_confidentiality.md
description: Use when the agent is protecting participant privacy, de-identifying or anonymizing datasets, assessing re-identification risk from quasi-identifiers or small cells, choosing encryption access control or storage for research data, sharing data under privacy constraints, navigating GDPR HIPAA or local data protection rules, or balancing open science against confidentiality obligations.
---

# Data Privacy And Confidentiality

Privacy protection is not achieved by removing names. A dataset can be stripped of direct identifiers and still re-identify individuals through combinations of quasi-identifiers, rare attributes, or linkage with other data. Researchers who believe they have anonymized data often have only pseudonymized it, and the difference matters because it changes what may be shared, with whom, and under what legal basis. The judgment problem is that privacy, openness, and analytic usefulness pull in different directions, and the researcher must decide how to protect participants without destroying the value of the data or making promises about anonymity that the data cannot keep.

Use this skill when designing data collection, storing or transferring research data, de-identifying datasets for sharing, assessing re-identification risk, responding to data access requests, navigating data protection law, or deciding how to reconcile open science commitments with confidentiality. The goal is to keep the agent from treating de-identification as a mechanical step that guarantees anonymity, from sharing data that can be re-linked, and from letting enthusiasm for open science override the promises made to participants. The agent has freedom to choose technical and governance controls, but must ground those choices in a realistic assessment of re-identification risk and applicable law.

## Core Rules

### Understand The Gap Between Anonymous And Anonymized

Truly anonymous data is data from which an individual cannot be identified by any means reasonably likely to be used, including linkage with other available data. Most datasets described as anonymized are actually pseudonymized: direct identifiers have been removed or replaced, but the underlying information can still be linked back to a person. The distinction is legal and practical.

Recognize:

- removing names, addresses, and contact details is de-identification, not anonymization;
- pseudonymized data is still personal data under most data protection regimes;
- the threshold for anonymity depends on what other data a motivated party could combine with it;
- a dataset anonymous today may become re-identifiable as external data accumulates;
- rich behavioral, genomic, location, or network data is especially hard to anonymize.

Do not label data anonymous unless re-identification is genuinely not reasonably likely. Otherwise, label it accurately and apply the protections pseudonymized data requires.

### Assess Re-Identification Risk Realistically

Re-identification risk rises with the uniqueness of combinations of attributes and the availability of auxiliary data. A combination that is rare in the general population, such as a specific age, occupation, postcode, and diagnosis, can identify a single person even when no field alone is identifying.

Assess:

- which quasi-identifiers are present, including age, birthdate, sex, postcode, occupation, ethnicity, rare conditions, and dates of events;
- how unique common combinations are in the relevant population;
- whether small cells in tabular outputs could identify members of a group;
- whether the data could be linked to public registers, social media, or other datasets;
- whether participants are public figures or members of small, visible communities;
- whether the data is genomic, geospatial, network, or longitudinal, which carry elevated risk.

Quantify where possible, such as the fraction of records unique on available quasi-identifiers, and apply additional protection to high-risk records or fields.

### Layer Technical Controls Rather Than Relying On One

No single technique makes data safe. Effective protection combines several controls, each addressing a different threat.

Combine:

- removal or generalization of direct identifiers and high-risk quasi-identifiers;
- aggregation, suppression of small cells, and rounding to prevent disclosure in tables;
- perturbation, noise addition, or swapping where analytic value tolerates it;
- k-anonymity, l-diversity, or differential privacy for formal guarantees where applicable;
- encryption of data at rest and in transit;
- access controls limiting who can reach which data, with authentication and logging;
- secure enclaves or remote analysis for data too sensitive to release directly;
- data use agreements governing permitted analyses and prohibition of re-identification attempts.

Match the controls to the data's sensitivity and the strength of the re-identification threat. Over-protection can destroy analytic value; under-protection exposes participants.

### Apply Access Control And Storage Discipline

Privacy is not only a property of the dataset but of the systems around it. A well-de-identified file on an open server is not private; an identifiable file in a locked, access-controlled enclave can be.

Govern:

- where data is stored, on approved institutional or cloud systems with appropriate assurances;
- who has access, on what legal or contractual basis, and how access is revoked;
- how data is transferred between collaborators and across borders;
- how long data is retained and how it is securely destroyed;
- whether backups, laptops, removable media, and personal devices are in scope;
- whether access and analysis actions are logged and auditable.

Treat the full lifecycle, not just the moment of collection. Data left on an unmanaged laptop after a project ends is a breach waiting to happen.

### Know The Applicable Legal And Ethical Framework

Data protection obligations vary by jurisdiction, sector, and data type, and they impose specific duties that cannot be improvised after collection.

Be aware of:

- GDPR and equivalent regimes, including lawful basis, purpose limitation, data minimization, storage limitation, and data subject rights;
- HIPAA and health-sector privacy rules where applicable, including the handling of protected health information;
- sector-specific rules for education, financial, genetic, or criminal justice data;
- cross-border transfer restrictions;
- funder and institutional data governance policies;
- the consent terms under which participants provided data, which may limit secondary use;
- the ethics approval conditions, which may specify storage, sharing, and retention.

When in doubt, consult the institutional data protection officer or legal counsel. Do not self-certify that data sharing is lawful because it seems reasonable.

### Reconcile Open Science With Privacy Deliberately

Open science calls for sharing data and code so that findings can be checked and built upon. Privacy calls for restricting access to data that could harm participants. These are in genuine tension, and the resolution is not to pick one but to design sharing that serves both.

Options include:

- sharing fully anonymized data where anonymization is genuinely achievable;
- sharing through controlled or gated access, where qualified researchers apply and agree to terms;
- providing analysis code and synthetic or heavily perturbed data so methods are reproducible without exposing real records;
- remote analysis or data enclaves where researchers run code on data they cannot download;
- clear documentation of what can and cannot be shared and why.

Do not default to refusing to share because privacy is complex, and do not default to dumping identifiable data because openness is valued. Design the sharing mechanism to fit the data's risk profile.

### Honor The Promises Made To Participants

Consent and information sheets create obligations. If participants were told their data would be used only for a specific study, shared only in aggregate, or destroyed after a period, those promises bind the researcher regardless of later convenience.

Check:

- what participants were told about data use, storage, sharing, and future research;
- whether broad or dynamic consent was obtained for secondary use;
- whether participants can be re-contacted for new consent;
- whether withdrawal extends to data already collected and how it is implemented;
- whether the planned sharing or reuse falls within the scope of the original consent.

Changing the rules after collection, even for a good scientific purpose, can breach trust and violate the ethical basis on which the data was gathered.

### Plan For Breach And Incident Response

Privacy protection includes preparing for failure. A lost laptop, a misdirected email, a compromised account, or an over-shared folder can expose participant data, and the response shapes the harm.

Plan for:

- how breaches will be detected and reported internally;
- the legal and ethical notification duties to participants, institutions, regulators, and funders;
- how affected participants will be informed and supported;
- how the root cause will be addressed to prevent recurrence.

A project with no incident plan tends to conceal or delay reporting, which compounds the harm and the liability.

## Common Traps

### Equating De-Identification With Anonymization

Removing names produces pseudonymized data, not anonymous data. Treating it as anonymous leads to over-sharing data that can be re-linked to individuals.

### Underestimating Quasi-Identifier Uniqueness

Combinations of ordinary attributes such as birthdate, postcode, and sex are often unique in modest samples. Rare diagnoses or occupations can identify a person in a small community even when no single field is identifying.

### Publishing Small Cells In Tables

A table cell representing one or two individuals can disclose their attributes to anyone who knows one of them. Suppress or aggregate small cells before release.

### Treating Genomic Or Geospatial Data As Ordinary

Genetic data can identify relatives, not just the donor. Precise location traces can reveal home, work, and routines. These data types require stronger protection than standard survey variables.

### Sharing Data Because It Seems Useful

Scientific usefulness does not override privacy obligations. The decision to share depends on risk, consent, and law, not on whether another researcher could benefit.

### Relying On Consent To Justify Any Later Use

Consent is bounded by what participants were told. Broad consent has limits, and reuse beyond its scope requires new approval or re-contact, not creative reinterpretation.

### Assuming A Cloud Or Institutional System Is Secure By Default

Hosting on a familiar platform does not guarantee compliant configuration. Access settings, encryption, data residency, and audit logging must be verified, not assumed.

### Promising Anonymity That Cannot Be Guaranteed

Telling participants their data will be anonymous, when the data is only pseudonymized, is a misleading promise that can undermine trust and create legal exposure if the data is later shared.

## Self-Check

- Is the data correctly classified as anonymous, pseudonymized, or identifiable, with the label matching the realistic re-identification risk?
- Have quasi-identifiers and small cells been assessed for uniqueness and disclosure risk, with high-risk records or fields given additional protection?
- Are multiple technical controls layered, including de-identification, aggregation, encryption, access control, and data use agreements, rather than relying on a single technique?
- Is storage, transfer, retention, access, logging, and destruction governed across the full data lifecycle, including backups and personal devices?
- Has the applicable legal and ethical framework, including GDPR, HIPAA, sector rules, funder policy, and ethics approval conditions, been identified and followed?
- Is open science reconciled with privacy through an appropriate sharing mechanism, rather than defaulting to refusal or over-sharing?
- Do data use, sharing, and reuse stay within the scope of what participants were told and consented to?
- Is there a breach detection, notification, and response plan, with duties to participants, institutions, regulators, and funders understood in advance?
- Are sensitive data types such as genomic, geospatial, network, or behavioral data given protections proportionate to their elevated re-identification risk?
- Have promises about anonymity or confidentiality been stated only where they can genuinely be kept, avoiding misleading assurances?
