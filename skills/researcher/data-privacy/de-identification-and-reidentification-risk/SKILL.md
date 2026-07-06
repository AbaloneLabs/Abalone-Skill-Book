---
name: de_identification_and_reidentification_risk.md
description: Use when the agent is de-identifying research data, assessing re-identification risk, deciding between anonymization and pseudonymization, handling direct and indirect identifiers, or planning data release with privacy protection.
---

# De-Identification And Re-Identification Risk

De-identification is the attempt to reduce data so that it can no longer be linked to individuals. It is harder than it looks. Removing names is not enough, because combinations of attributes can re-identify people, and datasets that seem anonymous in isolation become identifiable when linked. Researchers who treat de-identification as a simple step produce data releases that re-identify participants and cause real harm. The judgment problem is choosing de-identification methods that match the data and the release context, and assessing re-identification risk honestly rather than assuming it away.

Use this skill when de-identifying data, assessing re-identification risk, choosing between anonymization and pseudonymization, and planning data release. The goal is to keep the agent from equating de-identification with removing direct identifiers and to make the residual re-identification risk explicit and managed.

This is a high-stakes domain because re-identification can expose participants to stigma, discrimination, legal action, or harm. When the data are rich, the population small, or the release broad, the agent should consult a data privacy specialist or the IRB rather than asserting that de-identification is adequate.

## Core Rules

### Distinguish Direct And Indirect Identifiers

Direct identifiers name or uniquely point to a person. Indirect identifiers, alone or in combination, can re-identify a person. Both must be addressed.

Address:

- direct identifiers such as names, identification numbers, contact details, and full dates;
- indirect identifiers or quasi-identifiers such as age, birth date, postcode, occupation, rare diagnoses, and combinations thereof;
- the way combinations of seemingly innocuous variables can become unique;
- the need to generalize, suppress, or perturb quasi-identifiers, not just strip names.

Removing direct identifiers alone leaves datasets re-identifiable through quasi-identifier combinations. The combination risk is the central challenge.

### Choose Between Anonymization And Pseudonymization Deliberately

The two terms describe different levels of protection, and confusing them misrepresents the data's risk.

Distinguish:

- anonymization, which irreversibly prevents re-identification, and which is very hard to guarantee for rich data;
- pseudonymization, which replaces identifiers with codes but remains reversible given a key, and which is still personal data under most laws;
- the implications for what can be shared, with whom, and under what legal basis.

Calling pseudonymized data anonymous is both inaccurate and, in many jurisdictions, unlawful. State the method honestly and the residual risk it leaves.

### Assess Re-Identification Risk For The Specific Dataset And Context

Re-identification risk depends on the data and on what an adversary could plausibly do. A static, context-free judgment is usually wrong.

Assess risk considering:

- the uniqueness of records in the dataset, especially for rare conditions or small populations;
- the richness of the variables and how many are needed to single out an individual;
- the population the sample represents and how visible its members are;
- what auxiliary data a motivated adversary could plausibly obtain;
- the release context, whether trusted enclave, restricted access, or open release;
- the harm that would result from successful re-identification.

The same dataset may be low-risk in a trusted enclave and high-risk in open release. Match the protection to the realistic adversary and the release context.

### Apply Appropriate De-Identification Techniques

Different techniques suit different data and risk levels. The choice should follow the risk assessment.

Consider:

- generalization, such as replacing exact ages with bands and postcodes with regions;
- suppression of cells or records that remain identifying after generalization;
- perturbation, such as adding noise, where analytic validity can tolerate it;
- k-anonymity and related models that bound the minimum group size for re-identification;
- synthetic data generation where sharing real data is too risky;
- controlled access or data enclaves where de-identification alone is insufficient.

No technique removes all risk. The goal is to reduce risk to a level acceptable for the context while preserving analytic value, and to document the tradeoff.

### Evaluate The Interaction Between Utility And Privacy

Stronger privacy protection usually reduces analytic utility. The de-identification plan must balance both, explicitly.

Balance:

- how much generalization, suppression, or noise the analysis can tolerate before it becomes meaningless;
- whether the protected data can still answer the research question;
- whether the protections needed for open release destroy utility that a restricted-access model would preserve;
- whether tiered access, offering different versions to different users, resolves the tradeoff.

Over-protection can make data useless; under-protection can make them harmful. The balance must be reasoned, not defaulted to the easiest option.

### Plan For Linked And Combined Data

Data that are anonymous in isolation can become identifiable when linked with other data. De-identification must account for linkage.

Consider:

- whether the data can be linked to public or commercial datasets to re-identify;
- whether repeated releases allow mosaic re-identification over time;
- whether genetic or familial data allow re-identification through relatives;
- whether combining with data already held by a motivated adversary changes the risk;
- whether data use agreements and governance can constrain linkage.

Linkage risk is the reason that rich health and genomic data are rarely safe for open release. Address it explicitly in the release plan.

### Maintain Governance Even After De-Identification

De-identified does not mean ungoverned. Ongoing governance manages residual risk and deters misuse.

Maintain:

- data use agreements that constrain who can use the data and how;
- review of proposed uses for re-identification or misuse risk;
- audit trails of who accessed what;
- prohibitions on attempts to re-identify, with consequences;
- periodic re-assessment as auxiliary data and methods evolve.

De-identification is a point-in-time judgment in a moving landscape. Governance keeps the risk managed as that landscape changes.

## Common Traps

### Equating De-Identification With Removing Names

Removing direct identifiers leaves quasi-identifier combinations that re-identify. The combination risk is the central challenge.

### Calling Pseudonymized Data Anonymous

Pseudonymized data remain reversible and are personal data under most laws. Mislabeling them is inaccurate and unlawful.

### Judging Risk In Isolation From Release Context

The same dataset is low-risk in a trusted enclave and high-risk in open release. Context determines risk.

### Ignoring Linkage And Mosaic Re-Identification

Data anonymous alone can become identifiable when linked. Linkage risk must be part of the plan.

### Over-Protecting Until Utility Is Lost

Excessive suppression or noise can make data useless. The utility-privacy tradeoff must be reasoned.

### Releasing Once And Forgetting Governance

De-identified data still need governance. Residual risk and evolving methods require ongoing management.

### Assuming Rare Conditions Are Protected By Small Samples

Small samples of rare conditions are often highly re-identifiable. Rarity increases, not decreases, risk.

## Self-Check

- Are both direct identifiers and indirect quasi-identifiers addressed, including combination risk?
- Is the method honestly labeled as anonymization or pseudonymization, with the legal implications recognized?
- Is re-identification risk assessed for the specific dataset, population, adversary, release context, and harm?
- Are de-identification techniques chosen to match the assessed risk, with the tradeoff documented?
- Is the balance between privacy protection and analytic utility reasoned rather than defaulted?
- Are linkage, mosaic, and familial re-identification risks planned for in the release strategy?
- Is governance maintained after release through data use agreements, audit, and periodic re-assessment?
- Where data are rich, the population small, or release broad, has a data privacy specialist or the IRB been consulted rather than asserting adequacy?
