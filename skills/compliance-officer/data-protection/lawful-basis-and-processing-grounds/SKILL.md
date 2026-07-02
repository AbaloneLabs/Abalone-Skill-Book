---
name: lawful-basis-and-processing-grounds.md
description: Use when the agent is selecting a lawful basis for processing personal data under GDPR or other regimes, distinguishing consent from legitimate interests, evaluating necessity and proportionality, balancing competing bases for the same processing activity, or documenting the legal basis for a data processing operation.
---

# Lawful Basis and Processing Grounds

Every processing of personal data must rest on a lawful basis. Under the GDPR and comparable regimes, there is no default right to process personal data; the controller must identify and document a specific legal ground before processing begins. The choice of basis is not a formality — it determines the data subject's rights (the right to erasure is near-absolute when the basis is consent but limited when the basis is legal obligation), the controller's flexibility, and the exposure if the basis is challenged. Selecting the wrong basis, or claiming a basis that cannot be substantiated, is a foundational defect that undermines the entire processing operation. This skill addresses the judgment involved in identifying, documenting, and defending the lawful basis for processing.

## Core Rules

### Identify and document the lawful basis before processing begins

The lawful basis must be determined ex ante, not reconstructed after a complaint. For each processing activity, identify the basis from the available grounds under the applicable regime. Under the GDPR, the six bases are:

- **Consent**: the data subject gave clear, specific, freely given, informed consent;
- **Contract**: processing is necessary to perform a contract with the data subject or pre-contractual steps;
- **Legal obligation**: processing is necessary to comply with a legal obligation;
- **Vital interests**: processing is necessary to protect someone's life;
- **Public task**: processing is necessary for a task in the public interest or official authority;
- **Legitimate interests**: processing is necessary for the controller's or a third party's legitimate interests, except where overridden by the data subject's rights.

Document which basis applies, the reasoning, and the date of determination. A processing activity with no documented basis is non-compliant by definition.

### Match the basis to the reality of the processing, not to convenience

The most convenient basis is not always the correct one. Common mismatches:

- Claiming "contract" when the processing is not actually necessary to perform the contract (for example, marketing is not necessary to deliver a purchased service);
- Claiming "legitimate interests" for processing that is actually marketing, where the ePrivacy-specific consent rule applies;
- Claiming "consent" in an employment context where consent is not freely given due to the power imbalance;
- Claiming "legal obligation" without identifying the specific law that mandates the processing.

Test the basis against the actual necessity: is this processing genuinely necessary for the stated purpose under this basis, or is it merely convenient?

### Apply the legitimate interests balancing test rigorously

Legitimate interests is the most flexible but most demanding basis. It requires a three-part test:

- **Purpose test**: is there a legitimate interest (a real, lawful purpose, not merely a preference)?
- **Necessity test**: is the processing necessary to achieve that interest, or is there a less intrusive alternative?
- **Balancing test**: do the data subject's interests, rights, and reasonable expectations override the controller's interest?

The balancing test must be documented with the interests weighed, the impact on data subjects assessed, and safeguards applied to mitigate impact. A bare assertion of "legitimate interests" without the balancing analysis is indefensible. Where the data subject would not reasonably expect the processing, or where the impact is significant, the balance may fall against the controller.

### Understand that consent has strict quality requirements

Consent is valid only if it is:

- **Freely given**: no imbalance of power, no detriment for refusal, no bundling of unrelated consents;
- **Specific**: granular, per purpose, not a blanket consent;
- **Informed**: the data subject knows what they are consenting to;
- **Unambiguous**: a clear affirmative action, not silence or pre-ticked boxes.

Consent must also be as easy to withdraw as to give. In contexts of power imbalance (employment, public services), consent is rarely freely given and another basis should be used. Document how consent was obtained and how withdrawal operates.

### Recognize that the basis determines the data subject's rights

The choice of basis has downstream consequences:

- If the basis is consent, the data subject has a strong right to erasure and can withdraw consent at any time;
- If the basis is legitimate interests, the data subject has the right to object, and processing must stop unless compelling grounds are shown;
- If the basis is legal obligation, the right to erasure and objection are limited;
- If the basis is contract, portability may apply.

Do not select a basis without considering the rights it confers and whether the operation can function under those rights.

### Distinguish special category and criminal data with their additional conditions

Special category data (health, race, ethnicity, religion, political opinions, trade union membership, genetic data, biometric data for identification, sex life, sexual orientation) requires both a lawful basis and a separate additional condition under Article 9 of the GDPR. Criminal conviction data has its own regime. Identify the additional condition (explicit consent, employment law, vital interests, substantial public interest, etc.) and document it. Processing special category data on a standard basis alone is non-compliant.

### Re-evaluate the basis when the processing changes

The lawful basis is tied to the specific processing purpose and context. If the purpose expands, the recipients change, the retention extends, or the data becomes more sensitive, the original basis may no longer apply. Establish a process to re-assess the basis on material changes to the processing.

## Common Traps

### Defaulting to consent because it feels safest

Consent is often the weakest basis because it can be withdrawn, must be actively managed, and is frequently not freely given in practice. For processing that must continue regardless of the data subject's choice (payroll, legal compliance), consent is the wrong basis.

### Claiming legitimate interests without a documented balancing test

Legitimate interests requires the three-part test. A claim without the documented analysis is a bare assertion that fails on challenge. The balancing test is the evidence that makes the basis defensible.

### Treating "necessary for the contract" loosely

Many processing activities are convenient for the business but not necessary to perform the contract with the data subject. Personalization, analytics, and marketing are typically not contract-necessary. Reserve the contract basis for processing genuinely required to deliver what the data subject requested.

### Bundling consent for multiple purposes

A single consent for marketing, profiling, and data sharing is not specific. Each purpose requires separate, granular consent. Bundled consent is invalid.

### Ignoring the additional condition for special category data

Controllers identify a lawful basis but overlook that health or biometric data needs a separate Article 9 condition. The processing is non-compliant on the missing condition alone.

### Never re-evaluating the basis as processing evolves

A basis selected at launch may not cover processing added later (new analytics, new sharing, new retention). Without a change-triggered re-assessment, the operation drifts off-basis.

## Self-Check

- Is the lawful basis for each processing activity identified and documented before processing begins?
- Is the basis matched to the genuine necessity of the processing, not selected for convenience?
- For legitimate interests, is the three-part test (purpose, necessity, balancing) documented with the data subject's interests weighed?
- For consent, is it freely given, specific, informed, unambiguous, and as easy to withdraw as to give?
- Have I considered the downstream rights each basis confers and whether the operation can function under them?
- For special category or criminal data, is both a lawful basis and the required additional condition identified and documented?
- Is there a process to re-assess the basis when the processing purpose, recipients, retention, or sensitivity changes?
- Is the basis determination documented with reasoning and date, retrievable for audit or regulator inquiry?
