---
name: collection_and_processing.md
description: Use when the agent is planning or executing e-discovery collection from data sources, preserving metadata, using forensic imaging, deduplicating and filtering data, processing electronically stored information for review, handling native files and derivatives, hashing and chain of custody, or managing cross-border data collection.
---

# Collection And Processing

Collection and processing are the technical bridge between preservation and review. Collection extracts the relevant data from its sources in a defensible manner that preserves metadata and chain of custody; processing transforms that raw data into a reviewable, deduplicated, filtered set that review teams can work through efficiently. Agents often treat collection as a simple copy operation, when in fact the method of collection determines whether metadata is preserved, whether the data is admissible, and whether the collection can withstand challenge. Similarly, processing decisions on deduplication, filtering, and form of production shape the cost and quality of the entire review.

The technical choices in collection and processing are also legal choices, because they affect authenticity, completeness, privilege screening, and the ability to meet production deadlines. Use this skill before planning a collection, before selecting collection methods, before processing data for review, before making deduplication and filtering decisions, or before managing cross-border data movement. The goal is to help the agent make defensible technical decisions, not to provide legal advice, which requires qualified counsel and competent e-discovery professionals for the governing forum.

## Core Rules

### Plan Collection Around Scope, Sources, And Defensibility

Collection should follow a written plan that ties the data sources to the dispute scope and documents the method. Ad hoc collection invites challenge.

Plan:

- the custodians and data sources to collect;
- the time period and data types in scope;
- the collection method for each source, forensic image, targeted export, or self-collection;
- the preservation of metadata and folder structure;
- the chain of custody from source to processed set;
- the documentation of each collection step;
- the proportionality of the collection to the dispute;
- the order of collection to avoid altering source data.

Self-collection by custodians is the least defensible method because it risks alteration and incompleteness. Prefer forensic or supervised collection for key sources.

### Choose The Right Collection Method For Each Source

Different sources require different collection methods. The method affects metadata preservation, completeness, and defensibility.

Match methods:

- forensic imaging for full, bit-by-bit preservation of devices and drives;
- targeted export from email systems with metadata preserved;
- structured data extraction from databases and applications;
- collection from cloud and SaaS platforms via native export or API;
- collection from mobile devices with backup or forensic tools;
- collection of ephemeral or messaging platforms before deletion;
- preservation of website and social media content;
- collection of audio, video, and image files with metadata.

Forensic imaging preserves the most metadata and is the gold standard for key custodians, but it is expensive. Targeted export may suffice for routine sources if metadata is preserved.

### Preserve Metadata And Chain Of Custody

Metadata, the data about data, establishes authenticity, timing, authorship, and context. Chain of custody establishes that the collected data is what it purports to be.

Preserve:

- the system and file metadata, dates, authors, recipients, paths;
- the hash values that uniquely identify files;
- the chain of custody log documenting each transfer;
- the collection date, method, and personnel;
- the integrity verification after collection using hashing;
- the storage in a manner that prevents alteration;
- the documentation of any processing steps applied.

A break in chain of custody or altered metadata undermines admissibility. Document every step and verify integrity with hashing.

### Process Data Into A Reviewable Form

Processing transforms raw collected data into a reviewable set. The processing decisions shape review cost and quality.

Process:

- ingestion into the review platform;
- extraction of text from documents, emails, and attachments;
- handling of embedded files and containers, such as ZIP and PST;
- OCR for scanned or image-only documents;
- normalization of dates, encoding, and file formats;
- error handling for corrupted or password-protected files;
- the relationship between parents, attachments, and family threads;
- the creation of load files for the review platform.

Poor text extraction or missed attachments create review gaps. Verify that processing captures the full family structure of emails and containers.

### Apply Deduplication Strategically And Document It

Deduplication reduces volume and cost but can affect completeness and the review record. The strategy must be deliberate and documented.

Decide:

- whether to deduplicate globally, across custodians, or per custodian;
- the deduplication method, exact hash matching versus near-duplicate;
- the handling of attachments and family members;
- the preservation of one copy of each duplicate, and which custodian's;
- the documentation of what was deduplicated and suppressed;
- the effect on custodian-specific production;
- whether to deduplicate before or after processing.

Global deduplication reduces volume most but may suppress custodian-specific context. Per-custodian deduplication preserves context but yields more volume. Document the choice.

### Filter By Date, Custodian, And Content Proportionately

Filtering reduces the review set to what is relevant and proportional. Over-filtering loses evidence; under-filtering increases cost.

Filter:

- by date range tied to the dispute;
- by custodian and source;
- by content terms, keywords, and domains;
- by file type, excluding irrelevant types where defensible;
- by system and application exclusions, such as spam or auto-generated logs;
- by de-NISTing to remove known system and application files;
- the documentation of all filters and their rationale.

Keyword filtering that is too narrow misses relevant material; too broad retains too much. Validate filters with sampling.

### Manage Native Files, Derivatives, And Form Of Production

The form of production affects usability and admissibility. Native files preserve metadata; derivatives such as TIFF or PDF are easier to review but may lose metadata.

Manage:

- whether to produce native files, TIFF or PDF images, or both;
- the preservation of metadata in production;
- the handling of redactions in native versus image form;
- the load file and fielded data accompanying production;
- the extraction of text for searchability;
- the agreement with opposing counsel on form of production;
- the handling of privileged or withheld documents.

Form of production is often governed by agreement or rule. Coordinate the form early to avoid disputes.

### Handle Cross-Border Collection And Data Movement

Collecting data across borders implicates data protection, privacy, and blocking statutes. Moving data without compliance creates legal risk.

Handle:

- the data protection law of the source jurisdiction, such as GDPR;
- the cross-border transfer mechanisms available;
- blocking statutes that restrict data export;
- the privacy rights of employees and third parties;
- the collection from foreign custodians and systems;
- the coordination with local counsel in each jurisdiction;
- the potential need for collection and review in-country.

Cross-border collection can violate foreign law if mishandled. Plan compliance before moving data.

## Common Traps

### Self-Collection By Custodians

Custodians collecting their own data risks alteration and incompleteness. Use supervised or forensic collection for key sources.

### Losing Metadata Or Breaking Chain Of Custody

Simple copies often lose metadata and break chain of custody. Use forensic methods and hash verification.

### Over-Aggressive Deduplication

Global deduplication can suppress custodian-specific context. Choose the method deliberately and document it.

### Narrow Keyword Filtering Without Validation

Unvalidated filters miss relevant material. Sample to validate filter effectiveness.

### Missing Attachments Or Family Structure

Processing that misses email attachments or container contents creates review gaps. Verify family structure capture.

### Producing In An Unagreed Form

Form of production disputes delay and cost. Agree on form early.

### Ignoring Cross-Border Data Law

Moving data across borders without compliance violates privacy and blocking laws. Plan compliance first.

## Self-Check

- Is collection planned around scope, sources, method, metadata preservation, chain of custody, documentation, proportionality, and collection order?
- Is the right collection method matched to each source, including forensic imaging, targeted export, structured data extraction, cloud and SaaS, mobile, ephemeral messaging, web and social, and multimedia?
- Are metadata and chain of custody preserved through system and file metadata, hash values, custody logs, collection documentation, integrity verification, secure storage, and processing documentation?
- Is data processed into reviewable form with ingestion, text extraction, container handling, OCR, normalization, error handling, family structure, and load files?
- Is deduplication applied strategically with global versus per-custodian choice, method, family handling, copy selection, documentation, custodian-specific effect, and timing?
- Is filtering applied proportionately by date, custodian, content, file type, system exclusions, and de-NISTing, with documentation and sampling validation?
- Are native files, derivatives, and form of production managed for metadata preservation, redactions, load files, text extraction, opposing-counsel agreement, and privileged document handling?
- Is cross-border collection and data movement handled for source-jurisdiction data protection, transfer mechanisms, blocking statutes, privacy rights, foreign custodians, local counsel, and in-country review?
- Does the output escalate to qualified counsel and competent e-discovery professionals for collection methods, processing decisions, and cross-border compliance?
