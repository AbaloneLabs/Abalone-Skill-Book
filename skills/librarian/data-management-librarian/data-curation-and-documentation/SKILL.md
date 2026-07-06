---
name: data_curation_and_documentation.md
description: Use when the agent is curating a research dataset for deposit and reuse, appraising and selecting data, cleaning and transforming files, converting formats for preservation, creating documentation and metadata, assigning persistent identifiers and licenses, performing quality review and curation checks, applying FAIR principles, or ensuring a dataset is understandable and reusable by others beyond the original creator.
---

# Data Curation And Documentation

Data curation is the active management of a dataset from the moment it arrives for deposit to the point where it is independently reusable. A researcher who deposits raw, undocumented files is not sharing data in any meaningful sense; they are offloading a pile of bits that no one else can interpret. Curation is the work that turns a personal research artifact into a shared scholarly asset: appraising what to keep, converting formats so files remain readable, describing the data with metadata and documentation so others understand it, fixing errors that would mislead future users, and verifying that the dataset actually does what the deposit claims. Documentation is the heart of curation, because data without context is opaque, even to the person who made it a few years later.

The judgment problem is that curation is labor-intensive and often invisible. Depositors underestimate how much documentation a stranger needs, and curators are pressured to process volume rather than quality. The temptation is to do the minimum: accept the files, attach a title and a license, and publish. But a dataset published without verification may contain broken links, wrong formats, missing codebooks, identifiers that resolve to nothing, or data that does not match the described methods. Such datasets undermine reproducibility, waste the time of anyone who tries to reuse them, and damage the credibility of the repository. The agent's job is to perform curation as genuine quality work, not as a publishing formality, and to treat documentation as the deliverable that makes data reusable.

Use this skill when curating a dataset for deposit, creating or reviewing data documentation, converting formats, assigning metadata and identifiers, performing curation quality checks, or applying FAIR principles. The goal is to prevent the agent from publishing unverified data, accepting inadequate documentation, skipping format and file checks, treating metadata as decoration, or confusing availability with reusability.

## Core Rules

### Curate For Reuse, Not Just For Deposit

The test of curation is whether a qualified third party can find, understand, trust, and reuse the data. Deposit is the start, not the end.

Reuse-oriented curation:

- ask what a future user needs to understand and reuse this data;
- verify that the dataset is complete, including all files referenced in documentation;
- ensure the data matches what the documentation and metadata describe;
- check that the data supports the conclusions or methods it is associated with;
- remove or explain anomalies that would confuse a reuser.

A dataset that satisfies a deposit form but fails a reuse test is not curated. Always evaluate from the reuser's perspective.

### Appraise And Select Before Curating

Not every file a researcher offers belongs in the deposit. Curation begins with appraisal.

Appraisal decisions:

- distinguish final analysis data from intermediate and raw versions, and decide what to keep;
- identify derived or duplicated files that add no value;
- evaluate whether working files, logs, or scratch data belong in the deposit;
- consider what is necessary for reproducibility versus what is noise;
- document selection decisions and what was excluded and why.

Depositing everything offloads the researcher's filing system onto future users. Curate deliberately.

### Convert To Preservation-Friendly Formats

Formats determine whether files remain readable. Curation includes format assessment and conversion.

Format curation:

- identify proprietary or obsolete formats in the deposit;
- convert to open, well-documented preservation formats where possible, such as CSV for tabular data, TIFF for images, ODF or PDF/A for documents;
- retain original formats alongside preservation copies when conversion may lose information;
- verify conversions preserve data integrity, including precision, encoding, and structure;
- document any transformations applied during curation.

A dataset preserved only in a proprietary format is a dataset with an expiration date. Address formats explicitly.

### Create Documentation That Enables Independent Understanding

Documentation is what makes data reusable. It must be sufficient for someone unfamiliar with the project.

Documentation should include:

- a readme file explaining the dataset's purpose, contents, and structure;
- a data dictionary or codebook defining every variable, unit, code, and abbreviation;
- methodological description covering how data was collected or generated;
- information on processing, cleaning, and transformations applied;
- relationships among files, including code, data, and outputs;
- software, hardware, and environment needed to use the data;
- known issues, anomalies, or limitations.

Test documentation by asking whether a competent stranger could use the data without contacting the creator. If not, the documentation is incomplete.

### Assign Rich, Standards-Based Metadata

Metadata is how datasets are found, cited, and understood at scale. It must be rich and standards-based.

Metadata practices:

- use a discipline-appropriate schema, such as DataCite, Dublin Core, DDI, or Darwin Core;
- complete all required and recommended fields, not just the minimum;
- describe the data accurately and specifically in the abstract;
- assign precise subject keywords using controlled vocabularies where possible;
- capture provenance: creator, contributor, funder, related publications, and grants;
- record dates for collection, processing, and deposit;
- link to related outputs such as publications, code, or other datasets.

Sparse metadata makes data undiscoverable. Rich metadata is the difference between a dataset that is used and one that is invisible.

### Assign Persistent Identifiers And Clear Licenses

A dataset needs a citable identifier and a clear rights statement to be a scholarly asset.

Identifier and licensing:

- assign a persistent identifier, typically a DOI, at publication;
- ensure the identifier resolves to a stable landing page;
- choose an appropriate license, typically an open license such as CC0 or CC-BY for data where rights permit;
- clarify rights for data that includes third-party or licensed components;
- avoid "all rights reserved" or unclear rights, which block reuse;
- document any access restrictions and their basis.

A dataset without a persistent identifier cannot be reliably cited. A dataset without a clear license cannot be safely reused.

### Perform Concrete Curation Checks

Curation is verified through specific checks, not assumed. Run them before publishing.

Curation checks:

- open every file and confirm it is readable and not corrupted;
- verify file formats and check for hidden or temp files;
- check that tabular data opens cleanly and has consistent structure;
- confirm all files referenced in documentation are present;
- validate that persistent identifiers and links resolve;
- review metadata for accuracy, completeness, and consistency;
- check for confidential or sensitive data that should not be公开 deposited;
- verify the license is appropriate for the data's rights status.

A dataset that fails these checks should be returned to the depositor for revision, not published as-is.

### Apply FAIR Principles As A Practical Checklist

FAIR, Findable, Accessible, Interoperable, and Reusable, is a useful framework for evaluating curation quality.

FAIR application:

- Findable: persistent identifier, rich metadata, indexed in a repository;
- Accessible: retrievable via standard protocols, with clear access conditions;
- Interoperable: open formats, standards-based vocabularies, linked to related resources;
- Reusable: thorough documentation, clear license, provenance, and quality review.

Use FAIR as a checklist during curation, not as a marketing label. Each principle maps to concrete actions.

### Document Provenance And Curation Actions

Curation itself should be transparent. Record what was done to the dataset.

Provenance documentation:

- note any files added, removed, or converted during curation;
- record metadata changes and the reasons;
- preserve the original submission alongside the curated version where feasible;
- document communication with the depositor about revisions;
- timestamp curation actions for accountability.

Provenance protects integrity and helps future curators understand the dataset's history.

### Balance Quality Against Practical Constraints

Perfect curation of every dataset is not feasible. Make deliberate tradeoffs.

Tradeoff guidance:

- prioritize curation depth for high-value, high-reuse, or high-risk datasets;
- apply a baseline of checks to all deposits;
- use tiered curation levels based on the dataset's importance and complexity;
- document the curation level applied so users understand what was verified;
- advocate for adequate curation staffing rather than accepting volume over quality.

Curation is a professional service with real costs. Be honest about what level was delivered.

## Common Traps

### Publishing Unverified Data

Accepting files and publishing without opening them risks corrupt, incomplete, or mismatched deposits. Always run concrete curation checks.

### Treating Documentation As Optional

Data without documentation is unusable. Documentation is the core deliverable of curation, not an add-on.

### Sparse Metadata

Minimum-field metadata makes data undiscoverable. Complete rich, standards-based metadata for findability and reuse.

### Depositing Only Proprietary Formats

Data in proprietary or obsolete formats has a limited lifespan. Convert to open preservation formats.

### Confusing Availability With Reusability

Making data available is not the same as making it reusable. Reusability requires documentation, formats, license, and verification.

### Skipping Provenance Records

Failing to record curation actions obscures the dataset's history and undermines integrity. Document what was changed and why.

### Over-Curating Or Under-Curating Indiscriminately

Applying the same depth to every dataset wastes effort on trivial deposits and shortchanges important ones. Use tiered curation based on value and risk.

### Accepting Unclear Rights

Deposits with unclear or overly restrictive rights block reuse. Clarify and assign appropriate licenses before publishing.

## Self-Check

- Has the dataset been evaluated from the perspective of a future reuser, not just the depositor?
- Were appraisal and selection decisions made deliberately, with exclusions documented?
- Are files in open, preservation-friendly formats, with originals retained where conversion may lose information?
- Does the documentation, including readme, data dictionary, and methods, enable a competent stranger to understand and reuse the data?
- Is metadata rich, standards-based, and complete, with precise subject terms and provenance?
- Has a persistent identifier been assigned and verified to resolve, with an appropriate open license applied?
- Have concrete curation checks been run, including opening every file, verifying formats, checking references, and validating links?
- Are FAIR principles applied as a practical checklist with concrete actions, not as a label?
- Is curation provenance documented, including files changed, metadata revisions, and depositor communications?
- Is the curation level appropriate to the dataset's value and risk, and is that level transparent to users?
