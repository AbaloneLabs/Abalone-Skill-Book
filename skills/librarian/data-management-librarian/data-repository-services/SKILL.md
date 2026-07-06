---
name: data_repository_services.md
description: Use when the agent is selecting, setting up, evaluating, or operating an institutional or domain data repository, choosing a repository platform such as Dataverse, DSpace, CKAN, or Invenio, defining collection scope and deposit policies, configuring metadata schemas and persistent identifiers, planning preservation and bit-level integrity, managing access controls for sensitive data, evaluating repository certification such as CoreTrustSeal, or advising researchers on where to deposit data.
---

# Data Repository Services

A data repository is the infrastructure that makes research data a persistent, citable, reusable scholarly asset. It is not just a storage system; it is a service with policies, preservation commitments, access controls, metadata, identifiers, and governance. Choosing or operating a repository involves decisions that determine whether data survives for decades, whether it can be found and cited, whether sensitive data is protected, and whether the institution meets funder and publisher expectations. The wrong choice, or a poorly operated repository, produces data that rots, identifiers that break, access that fails, or obligations the institution cannot meet.

The judgment problem is that repository selection and operation are often driven by convenience rather than fitness. An institution picks a platform because it is free or familiar, without assessing whether it can meet preservation, access, sustainability, or certification requirements. Depositors choose a repository based on ease of upload rather than long-term stewardship. And once a repository is running, the hard work, preservation planning, format migration, integrity checking, policy enforcement, sustainability funding, is easy to neglect because the data just sits there, until it does not. The agent's job is to evaluate repositories against real requirements, operate them with genuine stewardship commitments, and advise depositors on choices that serve the long-term interest of the data.

Use this skill when selecting, evaluating, setting up, or operating a data repository, defining deposit and preservation policies, configuring metadata and identifiers, planning access controls, pursuing certification, or advising researchers on where to deposit. The goal is to prevent the agent from choosing a repository on convenience alone, promising preservation the platform cannot deliver, neglecting sustainability and exit planning, mishandling sensitive data access, or treating a repository as storage rather than a stewardship service.

## Core Rules

### Match The Repository To The Data And The Requirements

Repository selection starts with requirements, not with platforms. Define what the data and its stakeholders need first.

Requirements to define:

- funder and publisher requirements for deposit location and identifiers;
- disciplinary norms and whether a domain repository is expected;
- data types, formats, and volumes the repository must handle;
- access needs, including open, restricted, and controlled access;
- preservation duration and level required;
- metadata and identifier expectations;
- sustainability and cost constraints.

Evaluate candidate repositories against these requirements. A generalist institutional repository, a domain repository, and a commercial data archive each serve different needs; the right choice depends on the data and its context.

### Define A Clear Collection Scope And Deposit Policy

A repository that accepts anything becomes unmanageable. Scope and policy define what the service is.

Scope and policy elements:

- what types of data and materials the repository accepts;
- what it does not accept, such as data without documentation or outside the institution;
- deposit eligibility, such as institutional affiliation or funder mandate;
- required documentation and metadata at deposit;
- maximum volume and format expectations;
- embargoes, access restrictions, and the conditions for them;
- retention and withdrawal policies.

A written, public deposit policy protects the repository from ad hoc decisions and sets clear expectations for depositors. Vague scope leads to inconsistent service and unmanageable collections.

### Commit To Genuine Preservation, Not Just Storage

Storage is not preservation. A repository must plan for long-term bit-level and logical preservation.

Preservation commitments:

- maintain multiple geographically distributed copies;
- perform regular fixity, or checksum, checks to detect corruption;
- plan for format migration and emulation as formats age;
- maintain a preservation policy defining levels of preservation offered;
- document preservation actions and provenance;
- plan for media and system migration over time;
- distinguish bit-level preservation from full logical preservation, and be honest about which is offered.

A repository that stores data on a single system with no fixity checking or migration plan is offering the illusion of preservation. Real preservation is active and planned.

### Assign Persistent Identifiers At Deposit

Persistent identifiers make data citable and findable over time. They are a core repository function.

Identifier practices:

- assign a persistent identifier, typically a DOI, at deposit or publication;
- ensure the identifier resolves to a stable landing page maintained by the repository;
- support versioning with clear identifier handling for dataset versions;
- register identifiers with resolvers such as DataCite or Crossref;
- maintain identifier resolution even if the dataset is withdrawn, by resolving to a tombstone record.

A DOI that resolves to a dead link or a moved page breaks the citation chain. Identifier resolution is an ongoing operational commitment.

### Configure Rich Metadata And Discovery

Metadata is how deposited data is found and understood. The repository must support rich, standards-based metadata.

Metadata configuration:

- adopt a standard schema appropriate to the content, such as DataCite, Dublin Core, DDI, or Darwin Core;
- define required and recommended fields;
- support controlled vocabularies and subject taxonomies for findability;
- expose metadata for harvesting via protocols such as OAI-PMH or APIs;
- index metadata for search within the repository and discovery layers;
- support linking to related publications, code, and other datasets.

Sparse or inconsistent metadata makes deposits invisible. Metadata quality is a core service metric.

### Manage Access Controls For Sensitive Data

Not all data can or should be open. The repository must support appropriate access controls.

Access control capabilities:

- support open access, embargoed access, and restricted access as distinct states;
- provide controlled-access mechanisms for sensitive data, such as request-and-approval workflows;
- maintain audit logs of who accessed restricted data and when;
- apply consistent access rules aligned with consent, ethics, and law;
- provide clear landing pages for restricted data explaining how to request access;
- avoid defaulting everything to open when sensitive data is involved.

A repository that only supports open access cannot ethically hold human-subjects data. Match access capabilities to the data you accept.

### Ensure Sustainability And Plan For Exit

A repository is a long-term commitment. Sustainability and exit planning are essential.

Sustainability considerations:

- identify ongoing funding for infrastructure, staffing, and preservation;
- plan for staff succession and knowledge transfer;
- budget for system upgrades and migrations over the repository's life;
- define an exit strategy if the repository must close or migrate;
- ensure data and metadata can be exported in standard formats;
- document commitments to depositors regarding retention and access.

Repositories that launch without sustainability plans often fail their depositors when funding ends. Plan for decades, not grant cycles.

### Pursue Certification Or Standards Alignment

Certification signals that a repository meets recognized stewardship standards. It matters for trust and funder compliance.

Certification and standards:

- evaluate alignment with CoreTrustSeal for trustworthy data repositories;
- consider ISO 16363 for rigorous audit where appropriate;
- document policies and practices to support certification;
- use certification as a framework for improvement, not just a badge;
- be honest about gaps and work toward closing them.

Certification is not required for every repository, but the frameworks it provides are valuable for self-assessment and for meeting funder expectations.

### Provide Genuine Depositor And User Support

A repository is a service. Support determines whether it is usable.

Support practices:

- provide clear deposit guidance, templates, and documentation requirements;
- offer curation review before publication;
- provide help with metadata, licensing, and sensitive data decisions;
- maintain responsive user support for depositors and data users;
- offer training and outreach to build deposit and reuse culture;
- collect feedback and improve the service over time.

A repository with no support shifts all burden to depositors and users, reducing deposit quality and reuse.

### Advise Depositors On Repository Choice

When advising researchers on where to deposit, match the data to the right repository, not just the institutional default.

Advising considerations:

- check funder and publisher requirements first;
- consider whether a domain repository better serves disciplinary reuse;
- evaluate whether the institutional repository can meet the data's preservation and access needs;
- assess volume, format, and sensitivity against repository capabilities;
- verify the repository assigns persistent identifiers and supports the needed license.

Defaulting every depositor to the institutional repository may fail domain norms or funder mandates. Advise based on the data's needs.

## Common Traps

### Choosing A Repository On Convenience Alone

Selecting a platform because it is free or familiar, without assessing fitness, leads to failed preservation and compliance. Match the repository to defined requirements.

### Promising Preservation The Platform Cannot Deliver

Claiming long-term preservation without fixity checking, migration planning, or multiple copies is dishonest. Be explicit about the preservation level offered.

### Vague Or Unenforced Scope And Policy

Accepting anything without a clear deposit policy creates unmanageable collections and inconsistent service. Define and enforce scope.

### Neglecting Sustainability And Exit Planning

Repositories without funding and succession plans fail their depositors. Plan for decades and define an exit strategy.

### Defaulting All Data To Open Access

Not all data can be open. A repository must support embargo, restricted, and controlled access for sensitive data.

### Broken Or Unmaintained Persistent Identifiers

DOIs that resolve to dead pages break the citation chain. Maintain identifier resolution, including tombstones for withdrawn data.

### Treating The Repository As Storage

A repository is a stewardship service with policies, preservation, metadata, and support. Storage alone is insufficient.

### No Depositor Support Or Curation

Shifting all burden to depositors reduces quality and reuse. Provide guidance, curation, and responsive support.

## Self-Check

- Was the repository selected against defined requirements, including funder, disciplinary, preservation, access, and sustainability needs, not convenience alone?
- Is there a clear, public collection scope and deposit policy defining what is accepted, required documentation, and access conditions?
- Does the repository offer genuine preservation, including multiple copies, fixity checks, migration planning, and a documented preservation level?
- Are persistent identifiers assigned at deposit and maintained to resolve to stable pages, including tombstones for withdrawn data?
- Is metadata rich, standards-based, indexed, and exposed for harvesting and discovery?
- Does the repository support open, embargoed, restricted, and controlled access appropriate to sensitive data, with audit logging?
- Is there a documented sustainability plan covering funding, staffing, upgrades, and an exit strategy with exportable data?
- Has the repository evaluated alignment with certification frameworks such as CoreTrustSeal, with honest gap assessment?
- Is genuine depositor and user support provided, including deposit guidance, curation review, and responsive help?
- When advising depositors, is the repository choice matched to the data's needs, funder requirements, and disciplinary norms rather than defaulting to the institutional option?
