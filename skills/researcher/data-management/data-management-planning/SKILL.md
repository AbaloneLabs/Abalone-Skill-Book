---
name: data_management_planning.md
description: Use when the agent is writing a data management plan, preparing a grant data section, planning collection and storage, defining roles for data handling, or setting up a project to meet funder and journal data expectations.
---

# Data Management Planning

A data management plan is not grant paperwork. It is the design of how data will live throughout the project and after it. Decisions made at the start about formats, storage, documentation, access, and preservation determine whether the data survive, whether they can be analyzed correctly, whether they can be shared, and whether participants are protected. When data management is treated as an afterthought, three harms follow. Data are lost or corrupted because storage was not planned. Data become unusable because no one documented what the fields mean. And sharing or verification becomes impossible, undermining the study's credibility and the funder's investment.

The agent should use this skill when writing a data management plan, setting up a new project's data infrastructure, responding to funder data requirements, or rescuing a project whose data have become chaotic. The goal is to keep the agent from treating the plan as a compliance document and instead treat it as the blueprint for data that must remain trustworthy from collection to long-term preservation.

## Core Rules

### Describe The Data Before Planning How To Handle It

You cannot manage data you have not characterized. Start by describing what the data are.

- Types of data, quantitative, qualitative, images, signals, genomic, text, sensor, administrative.
- Source, primary collection, secondary use, or linkage of existing datasets.
- Volume and rate of accumulation.
- Format and structure, including file types and schemas.
- Sensitivity and re-identification risk.
- Relationship to other datasets in the project.

The description drives every later choice. A plan that talks generically about data without naming types and formats cannot be implemented.

### Choose Open And Stable Formats For Preservation

File formats decay as software changes. Choose formats that are open, documented, and widely supported for long-term preservation, even if working formats differ for analysis.

- Prefer open formats such as CSV, JSON, Parquet, OME-TIFF, or HDF5 over proprietary formats.
- Prefer plain text or openly documented binary formats over vendor-locked formats.
- Document any proprietary working format and a migration path to an open one.

For each data type, state the working format used during analysis and the preservation format for deposit. A dataset locked in a discontinued proprietary format a decade later is effectively lost.

### Plan Storage, Backup, And Security Together

Storage is where data live; backup is how they survive loss; security is how they stay protected. Plan all three.

- Working storage with enough capacity and performance for analysis.
- Backup with geographic redundancy, following a rule such as three copies on two media with one offsite.
- Versioning so earlier states can be recovered.
- Access controls matching the sensitivity of the data.
- Encryption for data at rest and in transit where appropriate.
- Audit trails for sensitive or regulated data.

A single laptop or single server with no backup is a data-loss incident waiting to happen. Plan redundancy before, not after, a failure.

### Define Documentation And Metadata Standards

Data without documentation are opaque. Plan what documentation will accompany the data at every stage.

- A data dictionary defining each variable, unit, and code.
- A codebook for categorical values and missing data.
- A protocol describing collection and any deviations.
- Metadata in a standard relevant to the discipline, such as Dublin Core, DataCite, or domain-specific schemas.
- Provenance records showing how raw data became the analyzed dataset.

Decide metadata standards early so collection tools can capture the needed fields. Retrofitting metadata at the end is slow and error-prone.

### Assign Data Roles And Responsibilities

Data management is work, and work needs owners. Assign roles for the following.

- Data capture and entry quality.
- Data cleaning and transformation.
- Documentation and metadata maintenance.
- Storage, backup, and security monitoring.
- Access management and request handling.
- Preservation and deposit.

Name a single data steward accountable for the integrity of the dataset. Without an owner, data quality drifts and no one notices until analysis breaks.

### Plan For Data Quality Throughout The Lifecycle

Quality is not a final check; it is built into collection and processing. Plan quality controls.

- Validation rules at data entry to catch impossible values.
- Range and consistency checks.
- Double entry or verification for critical fields.
- Monitoring of missingness and completion rates.
- Periodic data quality audits.
- A process for correcting errors with an audit trail.

State how errors will be recorded and corrected so that the analyzed dataset is traceable to the raw data. Silent corrections destroy reproducibility.

### Plan Access, Sharing, And Reuse

Anticipate how data will be accessed during the project and shared after. Connect this to consent, ethics, and funder requirements.

- Who within the team can access which data and when.
- How external requests will be handled.
- What will be shared, with whom, under what terms.
- Which repository will be used and its access model.
- Embargo periods and timing relative to publication.
- Licensing for open data and data use agreements for controlled data.

A plan that promises sharing without naming a repository or mechanism is a hollow promise. Specify the real path.

### Plan Preservation And Long-Term Stewardship

Data often outlive the project and the people who produced them. Plan for what happens after the grant ends.

- Which data will be preserved long-term and which will not, with reasons.
- The repository or archive that will hold them.
- The retention period required by funder, institution, or regulation.
- Who is responsible for stewardship after the project.
- How future users will find and cite the data, through persistent identifiers.

Without a preservation plan, data sit on a departing researcher's laptop and vanish. Preservation is the difference between a dataset that contributes to science and one that disappears.

### Align The Plan With Funder And Institutional Requirements

Funders increasingly require data management plans and specify what they expect. Read the funder requirements and align the plan to them.

- Required sections and headings.
- Required sharing and preservation periods.
- Allowed costs for data management.
- Required repositories or standards.
- Reporting and review expectations.

An internally excellent plan that ignores funder requirements can be rejected or create compliance problems later. Align from the first draft.

## Common Traps

### Writing The Plan Only To Satisfy The Funder

A plan written for compliance and never implemented protects no one's data. Treat it as a real operating document.

### Choosing Proprietary Formats For Preservation

Proprietary or undocumented formats become unreadable as software changes. Choose open formats for preservation.

### Relying On A Single Storage Location

One laptop, one server, or one cloud bucket is a single point of failure. Build in redundancy and offsite backup.

### Skipping Metadata Until The End

Metadata captured at the end is incomplete and inconsistent. Capture it during collection.

### Assuming Data Quality Will Be Checked Later

Errors introduced at collection compound through processing. Build validation into entry and processing.

### Promising Sharing Without A Mechanism

Promising availability on request without a repository or process is a hollow commitment. Specify the real path.

### Leaving Data Roles Implicit

When no one owns data quality, no one maintains it. Name a data steward.

### Forgetting Long-Term Stewardship

Data that outlive the project need a steward and a home. Plan preservation or the data vanish.

## Self-Check

- Does the plan describe the data types, sources, volume, formats, and sensitivity before handling choices?
- Are open and stable formats chosen for preservation, with working formats documented?
- Is storage, backup, versioning, and security planned with redundancy and access controls?
- Are documentation and metadata standards chosen, with capture planned during collection?
- Are data roles assigned, including a single accountable data steward?
- Are data quality controls built into collection and processing with audit trails?
- Is access and sharing planned with a real repository, terms, and timing?
- Is long-term preservation and stewardship planned beyond the project end?
- Is the plan aligned with funder, institutional, and journal requirements?
- For sensitive, regulated, or large-scale data, has a data manager, privacy officer, or institutional data governance advisor reviewed the plan before implementation?
