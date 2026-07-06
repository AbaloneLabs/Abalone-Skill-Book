---
name: open_data_and_sharing.md
description: Use when the agent is sharing research data openly, depositing data in a repository, writing a data availability statement, de-identifying data for sharing, choosing a data license, managing data sharing agreements, or deciding what data can and cannot be shared publicly.
---

# Open Data And Sharing

Sharing research data lets others verify findings, reuse data for new questions, and build cumulative knowledge. But sharing is not simply uploading files. Data must be findable, accessible, interoperable, and reusable, which requires documentation, formatting, licensing, and protection of participants whose information the data contain. Agents often share data as an afterthought, dumping raw files without documentation or consent review, which makes the data useless to others or, worse, exposes participants to re-identification. Responsible data sharing is a planned activity that balances the value of openness against privacy, ethics, and the effort required to make data genuinely usable.

The harm this skill prevents is datasets that cannot be used because they are undocumented, privacy breaches from inadequately de-identified shared data, compliance failures with sharing mandates, and the loss of scientific value when data vanish with the project. The agent has freedom in repository and format choice, but the requirements of documentation, privacy protection, and licensing are binding.

## Core Rules

### Plan Data Sharing From The Start Of The Project

Data sharing decided at the end is done poorly, because the documentation, consent, and formatting needed for sharing must be built in throughout. Plan sharing during design so that consent forms, data management, and documentation serve the eventual deposit.

Plan from the start:

- state sharing intentions in the data management plan;
- include data sharing in participant consent forms;
- structure data collection to facilitate later sharing;
- document variables, codes, and processing as you go;
- budget time and resources for deposit and curation.

### Apply FAIR Principles To Make Data Reusable

Data that cannot be found, understood, or used provides no open-science benefit. The FAIR principles, findable, accessible, interoperable, and reusable, guide how to prepare data so others can actually work with it.

Apply FAIR:

- findable: deposit in a searchable repository with a persistent identifier (DOI);
- accessible: retrievable via a standard protocol, with clear access terms;
- interoperable: use standard formats and vocabularies;
- reusable: include rich documentation, clear license, and provenance.

### Protect Participant Privacy Through De-Identification

Sharing data that can be traced back to individuals is an ethical and legal breach. De-identification is essential before sharing, but it is harder than it appears, because combinations of variables can re-identify individuals even when direct identifiers are removed. Assess re-identification risk honestly.

Protect privacy:

- remove direct identifiers (names, contact details, IDs);
- assess quasi-identifiers that could combine to re-identify;
- consider generalization, suppression, or perturbation for sensitive variables;
- evaluate re-identification risk for the specific dataset;
- when risk cannot be eliminated, use controlled rather than open access;
- consult privacy experts for sensitive or rare populations.

### Choose The Right Level Of Access For The Data

Not all data can or should be open. Some data can be shared openly without restriction; some require controlled access where approved researchers gain access under terms; some cannot be shared at all. Match the access model to the data's sensitivity.

Choose access level:

- open access for fully de-identified, low-risk data;
- controlled or managed access for sensitive but shareable data;
- restricted or no sharing for data that cannot be adequately protected;
- use data use agreements for controlled access;
- document the rationale for the chosen access model.

### Document Data Thoroughly With A Data Dictionary And Metadata

Data without documentation is opaque. A reader must understand what each variable means, how it was measured, what codes signify, and how the data were processed. Documentation is what transforms raw files into a reusable resource.

Document:

- a data dictionary defining each variable, type, units, and codes;
- the study design and data collection procedures;
- any cleaning, transformation, or derivation applied;
- missing data codes and conventions;
- file formats and relationships between files;
- provenance: who collected what, when, and how.

### License Data Clearly So Others Know How To Use It

Without a license, the legal status of shared data is ambiguous, and many will not use it for fear of infringement. A clear license tells others what they may do. Choose an appropriate open license and state it explicitly.

License clearly:

- use a recognized open license like CC0 or CC-BY for data where possible;
- state the license prominently in the repository and documentation;
- clarify attribution expectations;
- note any restrictions, such as data use agreement terms;
- ensure the license aligns with funder and institutional requirements.

### Use A Trusted Repository With Persistent Identifiers

Where data live matters for their longevity. A personal website or lab page disappears; a trusted repository provides long-term preservation and a persistent identifier that does not break. Choose a repository suited to the data type and discipline.

Choose a repository:

- use a discipline-specific or general repository (e.g., Zenodo, Dryad, OSF);
- ensure it provides a persistent identifier (DOI);
- confirm it offers long-term preservation and access policies;
- check that it supports the access model you need;
- verify funder or journal acceptance of the repository.

### Write A Clear Data Availability Statement

Published papers should include a data availability statement telling readers where the data are, how to access them, and any restrictions. Vague or missing statements prevent verification and violate many journal and funder policies.

Write the statement to:

- name the repository and provide the identifier or link;
- state the access model (open, controlled) and any terms;
- explain restrictions clearly if data cannot be shared;
- reference the data use agreement or application process;
- include code availability where analysis code is relevant.

## Common Traps

### Sharing Undocumented Raw Files

Data without a dictionary or metadata is unusable. Document thoroughly before deposit.

### Inadequate De-Identification

Removing names is not enough; quasi-identifiers can re-identify. Assess risk seriously.

### Open Access For Sensitive Data

Some data must be controlled-access, not open. Match access model to sensitivity.

### No Clear License

Ambiguous legal status deters reuse. Apply and state a recognized open license.

### Depositing In An Unstable Location

Personal sites vanish. Use a trusted repository with persistent identifiers.

### Sharing Decided Only At The End

Late sharing decisions miss the consent and documentation built in earlier. Plan from the start.

### Missing Or Vague Data Availability Statements

Readers cannot find or verify data without a clear statement. Write it explicitly.

## Self-Check

- [ ] Is data sharing planned from the start, including in consent and documentation?
- [ ] Does the deposit follow FAIR principles (findable, accessible, interoperable, reusable)?
- [ ] Has the data been de-identified with re-identification risk assessed?
- [ ] Is the access model (open, controlled, restricted) matched to data sensitivity?
- [ ] Is there a thorough data dictionary and metadata documenting variables and processing?
- [ ] Is a recognized open license applied and stated clearly?
- [ ] Is the data deposited in a trusted repository with a persistent identifier?
- [ ] Does the published paper include a clear, specific data availability statement?
- [ ] Are any restrictions explained with their rationale?
- [ ] Is analysis code shared alongside data where relevant?
