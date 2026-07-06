---
name: administrative_metadata.md
description: Use when the agent is creating administrative and technical metadata for digital objects, recording file format and technical properties, managing rights and access metadata, tracking provenance and lifecycle events, or supporting digital preservation through administrative metadata.
---

# Administrative Metadata

Administrative metadata is the least visible but most operationally critical metadata type. While descriptive metadata helps users find objects, administrative metadata helps the institution manage, preserve, access, and understand the technical and legal reality of those objects over time. It records file formats, technical environments, rights, provenance, ingest events, migrations, and preservation actions. Without it, digital objects become opaque files that cannot be trusted, accessed reliably, or preserved through format obsolescence. The trap is treating administrative metadata as optional technical detail, when in fact it is the metadata that determines whether a digital object survives and remains usable. Good administrative metadata is standards-based, captured at ingest, maintained through the lifecycle, and paired with descriptive and preservation metadata.

Use this skill when creating technical, rights, source, or lifecycle metadata for digital objects, implementing PREMIS or technical metadata schemas, or designing ingest and preservation workflows. The goal is to prevent the agent from skipping administrative metadata, recording it inconsistently, omitting provenance and rights, or failing to capture technical properties that future preservation depends on.

## Core Rules

### Understand The Administrative Metadata Subtypes

Administrative metadata is not one thing. It encompasses several subtypes, each serving a management function, and confusing them produces gaps.

The subtypes:

- technical metadata, file format, encoding, resolution, bit depth, software environment;
- rights metadata, copyright status, licenses, permissions, access restrictions;
- source metadata, the original physical or digital item the object derives from;
- preservation metadata, PREMIS events, agents, and objects tracking lifecycle actions;
- access metadata, location, repository, availability, retrieval information.

Name which subtype each element serves. A collection with strong technical metadata but no rights metadata is unmanageable for access.

### Capture Technical Metadata At Ingest

Technical metadata describes the file as a bitstream and must be captured when the file enters the repository, because that is when the file is in its initial managed state. Recreating it later is unreliable.

Capture at ingest:

- file format and version, using a registry like PRONOM;
- MIME type;
- file size in bytes;
- checksums for fixity;
- creating application and environment if known;
- for images, dimensions, resolution, color space, bit depth;
- for audio and video, codec, sample rate, bit rate, duration, channels;
- for documents, page count, embedded fonts, software version.

Use automated tools like DROID, JHOVE, or MediaInfo to extract technical metadata consistently. Manual entry is error-prone.

### Record File Format Using Persistent Identifiers

File formats become obsolete, and the format identifier must remain resolvable over decades. Using a local or ad hoc format name fails this test.

Use:

- PRONOM persistent identifiers, the standard for format identification;
- MIME types as a secondary, less precise identifier;
- Library of Congress Sustainability of Digital Formats entries;
- format registry lookups rather than free-text format names.

A PRONOM ID like fmt/18 for PDF 1.3 remains meaningful even after the format is obsolete, enabling future migration decisions.

### Implement Fixity From The Start

Fixity metadata, checksums and hashes, is the foundation of digital preservation. It lets the institution detect when a file has changed or corrupted. Without it, silent bit rot goes undetected.

Practice:

- generate checksums at ingest, using SHA-256 or stronger;
- store checksums separately from the files they protect;
- schedule regular fixity checks against stored checksums;
- log every fixity check as a preservation event;
- respond to mismatches with restoration from backup.

Fixity is not a one-time action. It is an ongoing monitoring commitment that the administrative metadata must support.

### Document Rights And Access Comprehensively

Rights and access metadata determine who can view, download, reuse, or restrict a digital object. Incomplete rights metadata creates legal risk and suppresses legitimate use.

Record:

- copyright status and basis for the determination;
- rights holder or source of permission;
- licenses, such as Creative Commons, with the specific license;
- standardized rights statements, such as RightsStatements.org;
- access restrictions and their basis, donor, privacy, cultural sensitivity;
- embargo dates and conditions;
- reuse permissions and attribution requirements.

Do not leave rights to assumption. A clearly recorded rights status protects the institution and enables confident user access.

### Track Provenance And Source Relationships

Digital objects have histories: where they came from, who created them, what transformations they underwent. Provenance metadata makes that history traceable.

Record:

- the source physical or digital item, if the object is a derivative;
- the digitization or creation event, date, equipment, operator;
- the submitting agent or donor;
- prior repositories or custodians if known;
- relationships to parent collections and related objects.

Provenance supports both user understanding and preservation integrity. An object whose source and transformation history is lost is harder to trust and to migrate.

### Use PREMIS For Preservation Metadata

PREMIS is the international standard for preservation metadata and should underpin any serious digital preservation effort. It models the objects, events, agents, and rights involved in preservation actions.

Implement PREMIS to record:

- objects, the file, representation, and intellectual entity;
- events, ingest, fixity check, migration, normalization, access;
- agents, people, organizations, software that performed events;
- rights statements tied to the object.

PREMIS turns preservation from ad hoc actions into a documented, auditable lifecycle. Even a simplified PREMIS implementation beats unstructured notes.

### Capture Lifecycle Events As They Happen

Every significant action on a digital object, ingest, migration, fixity check, access grant, deletion, should be recorded as an event when it happens. Reconstructing events later is unreliable.

For each event record:

- the event type and outcome;
- the date and time;
- the agent, person, organization, or software;
- the object or objects affected;
- any tools or parameters used;
- linking to related events.

A complete event log is the evidence that preservation is actually happening, not just promised.

### Maintain Metadata Through Migrations

When files are migrated to new formats, the administrative metadata must be updated to reflect the new reality while preserving the history.

On migration:

- create a new technical metadata record for the derived file;
- record the migration event linking source and derived objects;
- preserve the original technical metadata as historical;
- update fixity for the new file;
- update access and rights if the migration changes them.

Migration without metadata update creates a gap between what the record says and what the file is, undermining trust.

### Ensure Administrative Metadata Is Itself Preserved

Administrative metadata is a digital object too, and it must be preserved alongside the content it describes. Losing the metadata loses the ability to manage the object.

Practice:

- store metadata in open, documented formats like XML or JSON;
- back up metadata separately from content;
- version metadata schemas and document migrations;
- include metadata in fixity checking;
- ensure metadata survives repository migrations.

Metadata that outlives its schema or platform becomes unreadable. Plan for metadata preservation as carefully as content preservation.

## Common Traps

### Treating Administrative Metadata As Optional

It is the metadata that enables management, access, and preservation. Skipping it makes objects unmanageable over time.

### Free-Text Format Names

Format names without persistent identifiers become unresolvable as formats obsolete. Use PRONOM or equivalent registries.

### Fixity As A One-Time Action

Checksums must be checked repeatedly over the object's life. Silent corruption otherwise goes undetected.

### Missing Or Vague Rights

Unclear rights suppress use and create legal risk. Record status, holder, license, and restrictions explicitly.

### No Provenance Or Source Record

An object without source and transformation history is harder to trust and to migrate. Capture provenance at ingest.

### Ad Hoc Preservation Notes Instead Of PREMIS

Unstructured notes cannot support audit or automation. Use PREMIS, even simplified, for preservation events.

### Events Recorded Late Or Not At All

Reconstructing lifecycle events later is unreliable. Capture events as they happen.

### Metadata Not Preserved Or Versioned

Metadata that outlives its schema or platform becomes unreadable. Preserve and version metadata alongside content.

## Self-Check

- Are the administrative metadata subtypes, technical, rights, source, preservation, access, all addressed?
- Is technical metadata captured at ingest using automated tools rather than manual entry?
- Are file formats recorded with persistent identifiers like PRONOM, not free-text names?
- Are fixity checksums generated at ingest, stored separately, and checked on a schedule?
- Are rights and access metadata comprehensive, covering status, holder, license, restrictions, and embargoes?
- Is provenance and source relationship metadata captured for every derivative object?
- Is PREMIS used, even in simplified form, to model preservation objects, events, agents, and rights?
- Are lifecycle events recorded as they happen, with type, date, agent, and outcome?
- Is administrative metadata updated through migrations while preserving historical records?
- Is the metadata itself preserved in open formats, backed up, versioned, and included in fixity checks?
