---
name: data_privacy_and_de_identification.md
description: Use when the agent is de-identifying or anonymizing research data, assessing re-identification risk, applying privacy models like k-anonymity or differential privacy, designing secure data sharing, handling sensitive or special-category data, or judging whether a dataset can be shared or linked without harming participants.
---

# Data Privacy And De-Identification

De-identification is not a button that makes data safe. It is a trade-off between utility and risk, and researchers routinely get it wrong in both directions. They remove obvious identifiers and call the data anonymous while rich quasi-identifiers remain that make re-identification trivial, or they lock data away so tightly that legitimate secondary analysis becomes impossible. Both errors harm participants and science: the first exposes people to privacy loss, stigma, or legal risk; the second wastes data that participants contributed at some cost.

Use this skill when de-identifying data, when assessing re-identification risk, when designing data sharing, when handling sensitive categories, and when deciding whether datasets can be linked or released. The goal is to keep the agent from equating identifier removal with anonymity, from underestimating re-identification risk, and from treating privacy as a final stamp rather than a continuing obligation. The agent has latitude in techniques, but must justify the privacy-utility trade-off and the residual risk for the specific dataset and context.

## Core Rules

### Treat De-Identification As Risk Assessment, Not A Checklist

Removing names and direct identifiers is necessary but nowhere near sufficient. Re-identification risk depends on the combination of variables that remain, the population, and the auxiliary data an adversary might hold.

Assess risk by considering:

- direct identifiers, such as names, numbers, and contact details;
- quasi-identifiers, such as dates, geography, occupation, and rare conditions;
- the granularity of each remaining variable;
- the uniqueness of records in the residual dataset;
- the population from which the sample was drawn;
- plausible auxiliary data an adversary could combine;
- the motivation and capability of potential adversaries;
- the harm that would follow re-identification.

A dataset stripped of names can still be uniquely identifying if enough quasi-identifiers remain. Risk is a function of the whole record, not of any single field.

### Recognize That Anonymity Is Rarely Absolute

True anonymity, where no party can ever link a record to a person, is hard to achieve and harder to guarantee. Most "anonymized" data is more accurately described as de-identified to a stated standard against a stated threat model.

Be precise about claims:

- de-identified means identifiers have been removed or altered under a defined process;
- anonymized often implies a stronger, sometimes unattainable, guarantee;
- pseudonymized data still allows re-linkage with a key;
- the appropriate standard depends on the data, the recipient, and the threat;
- claims should match the actual residual risk, not the aspiration.

Overstating anonymity misleads data holders, recipients, and participants about the real risk.

### Apply Formal Privacy Models Where Stakes Are High

For sensitive data or broad sharing, informal de-identification is insufficient. Formal models provide quantifiable guarantees, at a cost to utility.

Consider:

- k-anonymity and its variants, l-diversity and t-closeness, which bound record uniqueness;
- differential privacy, which provides mathematical guarantees against inference about individuals;
- secure multi-party computation and homomorphic encryption for computation on encrypted data;
- synthetic data generation that preserves some statistical properties;
- data enclaves or secure data centers for analysis without release;
- the utility cost each model imposes on the data.

No model is universally best. Match the model to the threat, the data, and the analyses the data must support.

### Manage Quasi-Identifiers Through Generalization And Suppression

Quasi-identifiers, alone or in combination, are the most common route to re-identification. They must be deliberately weakened.

Techniques include:

- generalizing geography, such as replacing a postcode with a region;
- generalizing dates, such as replacing a birth date with a year or band;
- generalizing occupations and rare conditions into broader categories;
- suppressing top-coding or bottom-coding extreme values;
- suppressing rare combinations that would identify individuals;
- applying generalization consistently across linked datasets;
- checking residual uniqueness after each transformation.

Each generalization reduces risk and utility. Record the trade-off and justify it against the intended use.

### Treat Sensitive And Special-Category Data With Heightened Care

Some data carries greater harm if exposed: health, genetics, sexual behavior, religion, ethnicity, political views, immigration status, biometrics, and location traces. Legal regimes often impose stricter obligations on these categories.

For sensitive data:

- apply stronger de-identification and access controls;
- limit collection to what is genuinely necessary;
- restrict secondary use and require explicit consent or legal basis;
- consider whether the data should be linked or released at all;
- provide additional safeguards for vulnerable populations;
- document the legal basis and any conditions or waivers;
- plan for breach response and participant notification.

The harm from exposing special-category data is severe and often irreversible; the precautions must match.

### Design Data Sharing Around The Threat And The User

Different sharing scenarios carry different risks. Sharing with a trusted collaborator under contract is not the same as public release.

Match controls to the scenario:

- public release demands the strongest de-identification or formal privacy;
- controlled access through a data committee allows richer data under terms;
- data enclaves allow analysis without extraction;
- secure file transfer under a data use agreement supports collaboration;
- synthetic or heavily masked subsets support methodological work;
- linkage across datasets requires additional controls and approvals.

State who can access what, under what terms, for what purpose, and with what residual risk. Vague "data available on request" language is not a sharing plan.

### Address Linkage And Re-Identification Risk Across Datasets

Combining datasets multiplies both utility and risk. Two anonymized datasets, when linked, can become identifying.

Plan for linkage by:

- assessing what an adversary could learn by combining with public data;
- using trusted third parties or secure environments for linkage;
- minimizing the variables used as linkage keys;
- releasing only the analytic outputs, not the linked microdata;
- reviewing disclosure risk of all outputs before release;
- considering that future datasets may increase re-identification risk over time.

A dataset that is safe today may become identifying when a new public dataset appears. Privacy is not a one-time property.

### Preserve Participant Rights And Consent Throughout

Privacy protection is part of the promise made to participants. It must be consistent with consent, the stated data uses, and the right to withdraw.

Maintain:

- consent or legal basis that covers the intended uses and sharing;
- transparency about what was de-identified and what residual risk remains;
- a mechanism for participants to learn how their data is used;
- a path to honor withdrawal, including destruction of identifiable data where feasible;
- re-consent when new uses or linkages exceed the original consent;
- clear governance and accountability for data stewardship.

De-identification does not suspend the ethical obligations under which the data were collected.

## Common Traps

### Equating Identifier Removal With Anonymity

Stripping names does not address quasi-identifiers, and quasi-identifiers are usually how re-identification actually happens.

### Overstating Privacy Guarantees

Calling data anonymous when residual risk remains misleads recipients and exposes participants.

### Ignoring Linkage Risk Across Datasets

Two safe datasets can become identifying when combined, especially with future public data.

### Locking Data Away Indefinitely

Excessive restriction wastes participant-contributed data and undermines the scientific return without commensurate privacy benefit.

### Underestimating Sensitive-Data Harm

Special-category data exposure can cause severe and irreversible harm and demands stronger safeguards than ordinary data.

### Vague Data Sharing Promises

"Available on reasonable request" without governance, terms, or risk assessment is not a real sharing plan.

### Treating Privacy As A One-Time Stamp

Re-identification risk evolves as auxiliary data accumulates. Privacy requires ongoing review.

## Self-Check

- [ ] Is de-identification treated as risk assessment against a stated threat model rather than a checklist of removed fields?
- [ ] Are claims about anonymity matched to the actual residual risk, with precise terminology?
- [ ] Are formal privacy models considered where stakes or sharing scope demand quantifiable guarantees?
- [ ] Are quasi-identifiers managed through deliberate generalization and suppression, with trade-offs recorded?
- [ ] Is sensitive and special-category data given heightened protection proportionate to the harm of exposure?
- [ ] Is data sharing designed around the threat, the user, and the purpose, with clear access controls?
- [ ] Is linkage and re-identification risk assessed across datasets and over time, not only within one dataset?
- [ ] Are consent, transparency, withdrawal, and governance maintained throughout the data lifecycle?
- [ ] Is the privacy-utility trade-off documented and justified for the intended analyses?
- [ ] Is there ongoing review of residual risk as new auxiliary data becomes available?
