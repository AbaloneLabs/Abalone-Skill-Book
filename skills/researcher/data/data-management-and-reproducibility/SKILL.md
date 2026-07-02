---
name: data_management_and_reproducibility.md
description: Use when the agent is planning research data management, organizing raw data, preparing code or analysis workflows, documenting methods, selecting repositories, sharing scientific data, protecting participant privacy, or checking whether research can be reproduced.
---

# Data Management And Reproducibility

Research data management is part of research integrity. If data cannot be located, understood, protected, audited, or connected to the published result, the finding is weaker than it appears. Reproducibility does not require every study to be perfectly repeatable in the same way, but a researcher should make the path from raw evidence to claim clear enough for appropriate inspection, validation, reuse, or replication.

Use this skill before collecting data, cleaning datasets, writing analysis code, preparing a data management plan, choosing a repository, sharing scientific data, or checking whether a project is reproducible. The goal is to make the agent plan provenance, metadata, privacy, preservation, and analysis traceability before the project becomes too messy to repair.

## Core Rules

### Define What Counts As Research Data

Data management begins by naming the materials the study will create or use. Different projects involve different evidence.

Identify:

- raw measurements;
- survey responses;
- interview transcripts;
- field notes;
- images;
- audio or video;
- biological specimens;
- administrative records;
- sensor logs;
- code;
- model outputs;
- derived variables;
- annotations;
- metadata;
- consent records;
- analysis scripts.

Distinguish raw data, cleaned data, analysis-ready data, outputs, and documentation. Do not overwrite raw data.

### Plan Provenance From The Start

Provenance explains where data came from and how it changed. Without provenance, errors become hard to detect and results become hard to trust.

Track:

- source;
- collection date;
- collector or system;
- instrument or protocol;
- version;
- permissions;
- transformations;
- exclusions;
- cleaning rules;
- merge rules;
- missing data decisions;
- derived variable definitions;
- file checksums where appropriate.

Every reported result should be traceable to data and code or analysis steps.

### Use Organized, Controlled Storage

Data should be stored in a way that protects integrity and access. Personal laptops, email attachments, and ad hoc folders create risk.

Plan:

- folder structure;
- naming convention;
- access roles;
- backup schedule;
- encryption where needed;
- storage location;
- version control;
- retention period;
- deletion process;
- disaster recovery;
- data transfer method.

Sensitive data need stronger controls. Limit access to people with a research need.

### Document Metadata And Context

Future users, reviewers, collaborators, and the researcher themselves need to understand the data.

Document:

- variable names;
- definitions;
- units;
- coding schemes;
- allowed values;
- missing value codes;
- collection instruments;
- sampling frame;
- time zone and date formats;
- language or translation notes;
- equipment settings;
- processing scripts;
- data dictionary;
- README.

If a dataset requires oral explanation from one person to be usable, documentation is insufficient.

### Make Analysis Reproducible Where Practical

The analysis workflow should reduce hidden manual steps.

Use:

- scripted analysis where possible;
- version-controlled code;
- fixed random seeds when appropriate;
- environment documentation;
- package versions;
- workflow notes;
- output generation scripts;
- table and figure scripts;
- computational notebooks with caution;
- test data for complex pipelines;
- independent rerun for critical outputs.

Avoid manual spreadsheet edits that cannot be audited. If manual steps are unavoidable, document them precisely.

### Protect Privacy And Confidentiality

Data sharing and reproducibility must not override participant rights, confidentiality, legal restrictions, or community agreements.

Review:

- informed consent;
- IRB or ethics approval;
- data use agreement;
- identifiers;
- indirect identifiers;
- small-cell risk;
- re-identification risk;
- sensitive attributes;
- access controls;
- de-identification method;
- retention commitments;
- destruction requirements;
- repository access levels.

De-identification is not magic. Consider whether combinations of variables can identify people.

### Plan Sharing And Preservation

Funders, journals, institutions, and communities may require sharing or preservation. NIH's Data Management and Sharing Policy expects planning for managing and sharing scientific data when applicable, while allowing ethical, legal, or technical limits.

Decide:

- what will be shared;
- what will not be shared and why;
- when sharing will occur;
- repository;
- license or terms of use;
- metadata standard;
- file formats;
- access controls;
- citation method;
- preservation period;
- budget for curation.

Do not promise open data if consent, law, contracts, or community obligations prevent it. Do not withhold shareable data without a rationale when sharing is expected.

### Monitor Data Quality During Collection

Quality control should happen before the study ends.

Check:

- missingness patterns;
- outliers;
- range violations;
- duplicate records;
- protocol deviations;
- instrument drift;
- interviewer effects;
- coding consistency;
- interrater reliability;
- data entry errors;
- recruitment imbalances.

Record corrections and reasons. Do not silently clean data to fit expectations.

### Prepare For Audit Or Review

Responsible research requires records that can support the work if questioned.

Maintain:

- protocol;
- approvals;
- consent templates;
- data dictionary;
- raw data;
- cleaning logs;
- code;
- analysis outputs;
- decision log;
- correspondence about major methods changes;
- preregistration if used;
- repository deposit record.

Research misconduct concerns can arise from poor records even when no intent to deceive exists.

## Common Traps

### Cleaning Data Without A Log

Unlogged changes make results hard to verify. Keep raw data unchanged and document transformations.

### Sharing Data Without Checking Consent

Sharing expectations do not override participant consent, ethics approval, law, or data use agreements.

### Treating A Spreadsheet As A Database

Spreadsheets are useful but risky for versioning, formulas, sorting, and hidden edits. Use controlled workflows for important datasets.

### Ignoring Code Environment

Code that runs only on one machine without package versions or environment notes is fragile.

### Assuming De-Identified Means Safe

Indirect identifiers can re-identify people, especially in small samples or rare conditions.

### Saving Only Final Outputs

Figures and tables are not enough. Preserve the data and steps that produced them.

### Deferring Data Management Until Publication

By publication time, missing metadata and lost provenance may be impossible to reconstruct.

## Self-Check

- Are raw, cleaned, analysis-ready, output, and metadata files distinguished?
- Is data provenance tracked from collection through analysis?
- Are storage, access, backup, versioning, retention, and transfer controls defined?
- Is metadata sufficient for another qualified person to understand the data?
- Are analysis steps scripted or documented enough to reproduce key outputs?
- Are privacy, consent, ethics approval, data use agreements, and re-identification risks considered?
- Is sharing planned with repository, timing, access level, license, metadata, and justified limitations?
- Are data quality checks performed during collection and cleaning?
- Are manual changes logged with reasons?
- Would the project withstand audit, peer review, or internal verification of the reported result?
