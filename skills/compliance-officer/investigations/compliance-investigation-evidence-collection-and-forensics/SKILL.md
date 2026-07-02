---
name: compliance_investigation_evidence_collection_and_forensics.md
description: Use when the agent is collecting documents and data for a compliance investigation, issuing preservation or litigation holds, preserving email and chat, conducting digital forensics on devices and accounts, maintaining chain of custody, or handling electronic evidence that may be used in discipline, disclosure, or enforcement.
---

# Compliance Investigation Evidence Collection And Forensics

Evidence is perishable. Email is deleted on retention schedules, chat messages vanish when an account is disabled, system logs roll over, and a subject who learns of an investigation may alter or destroy records. The reliability of every later finding, disciplinary action, and regulatory submission depends on whether evidence was collected completely, preserved defensibly, and handled with a documented chain of custody. A conclusion built on incomplete or contaminated evidence cannot survive challenge, and a sloppy collection can itself become a spoliation claim or an obstruction exposure.

Use this skill before collecting documents or data in an investigation, when issuing preservation holds, when engaging forensic analysts, when imaging devices or extracting mailbox content, when handling evidence that may go to a regulator or court, or when an existing collection has gaps or custody questions. The goal is to make the agent treat evidence as a defensible, reproducible record rather than a set of files gathered ad hoc.

## Core Rules

### Issue And Confirm The Preservation Hold Before Collection

The first act of evidence handling is to stop routine deletion. A litigation or preservation hold suspends retention schedules and instructs custodians to preserve relevant material. A hold that is issued but not confirmed, or that covers the wrong systems, is little better than no hold.

A defensible hold includes:

- the custodians and accounts in scope;
- the systems and data sources covered, including email, chat, shared drives, cloud storage, HR systems, finance systems, and personal devices if used for work;
- the time period covered;
- the instruction to suspend deletion and auto-purge;
- the prohibition on alteration or deletion;
- acknowledgement tracked per custodian;
- periodic re-issuance for long matters;
- release when the hold is no longer needed.

Confirm the hold took effect. Check that automated retention policies, mailbox auto-delete, and backup rotation are actually paused for the affected systems. A hold letter that the IT system ignores is a spoliation risk.

### Define The Collection Universe From The Allegation

Collect too little and the investigation is inconclusive; collect indiscriminately and the review becomes unmanageable and expensive. Define the collection universe deliberately from the factual questions.

Map the universe by identifying:

- the custodians likely to hold relevant material;
- the systems and repositories they use;
- the time window around the conduct;
- the transaction or record types implicated;
- the communication channels in play, including personal devices, messaging apps, and personal email if permitted;
- third-party-held data such as vendor or bank records;
- system logs, access records, and audit trails;
- backups and archives that may hold deleted material.

Document the rationale for inclusion and exclusion. If a custodian or system is excluded, record why, so a later challenge cannot claim a selective collection.

### Use Forensically Sound Collection Methods

How data is collected affects whether it can be trusted. Copying files through normal interfaces can alter metadata, miss deleted items, and break the ability to prove integrity. Forensic methods preserve the evidence as it existed.

For forensic soundness:

- use write-blockers or forensic imaging for devices;
- collect mailboxes through server-side export rather than PST drag-and-drop where possible;
- capture hash values, such as MD5 or SHA-256, at collection to prove integrity;
- preserve original metadata, including created, modified, and accessed dates;
- work from copies, never the original, for analysis;
- document the tool, version, operator, date, and method for each collection step;
- image the full device where feasible rather than selecting files, since context and deleted material may matter;
- capture volatile data, such as memory or live system state, only where relevant and by qualified personnel.

For cloud and SaaS sources, understand the provider's export limitations, preservation options, and logging. Some platforms do not preserve edits or deletions in a way that survives, so act before the platform's own retention expires.

### Maintain Chain Of Custody From The First Touch

Chain of custody is the documented record of who handled the evidence, when, where, why, and what was done to it. Gaps invite challenges that the evidence was altered or fabricated.

For each item or data set, record:

- a unique identifier;
- description, including device, account, or file set;
- source and location;
- date, time, and method of collection;
- collecting individual;
- hash or other integrity value;
- transfer log showing each handoff;
- storage location and access controls;
- any processing or analysis performed and by whom;
- return or disposition.

Physical devices, hard drives, USB media, and printed documents need the same discipline as digital files. Store originals in secured, access-controlled locations, and log every access. A custody break, even an innocent one, can be fatal to the evidence's weight.

### Handle Processing And Review Defensibly

Raw collections are usually too large to review directly. Processing reduces, filters, and organizes data, but each processing step must be transparent and reproducible.

Document:

- deduplication method and thresholds;
- date and custodian filters applied;
- search terms or analytics used, with validation of recall and precision;
- technology-assisted review protocols where used;
- exceptions and items set aside;
- the review platform and its handling of metadata;
- any privilege screening and the log of withheld items.

Avoid over-filtering that silently discards relevant material, and avoid under-filtering that buries the key evidence. Test search terms against known relevant documents to confirm they capture what they should. A search that misses the smoking gun because of a term typo is a collection failure dressed up as a review efficiency.

### Preserve And Analyze The Right Artifact Types

Different allegations turn on different evidence types. Match the forensic analysis to the factual questions.

Common artifact types and their value:

- email and calendar: approvals, intent, and timing;
- chat and messaging: informal direction and coordination;
- financial and ERP records: transactions, overrides, and approvals;
- expense systems: reimbursement patterns and policy breaches;
- access and authentication logs: who did what and from where;
- system configuration and change logs: control bypasses and tampering;
- deleted-file recovery and unallocated space: destruction or hidden activity;
- browser and cloud-sync history: exfiltration and off-channel communication;
- mobile device data: location, messages, and app activity;
- backups and snapshots: state at a point in time.

Coordinate with IT and security early. They control the systems and logs, but they may not know what is relevant without specific guidance, and they may not preserve volatile data unless asked promptly.

### Coordinate With Legal On Privilege, Privacy, And Cross-Border Limits

Evidence collection sits at the intersection of privilege, privacy law, employment law, and cross-border restriction. Collecting data without considering these can waive privilege, breach privacy law, or create its own violation.

Address:

- whether collection is directed by counsel to support privilege;
- applicable data protection law, including employee monitoring notice and works council requirements;
- cross-border transfer restrictions when data moves between jurisdictions;
- employee expectations of privacy on personal devices or accounts;
- legal process required to compel third-party records;
- privilege screening before production to regulators or litigation;
- minimization of personal data not relevant to the inquiry.

In several jurisdictions, imaging an employee's device or reading personal communications without a legal basis is itself a violation. Do not assume the company owns all data on a company-issued device or that monitoring notice covers every collection.

### Plan For Production And Disclosure From The Outset

Evidence collected for an internal investigation often ends up before a regulator, auditor, court, or external party. Collection choices made early determine what can later be produced cleanly.

Consider:

- the likely production formats and whether metadata will be required;
- the need for a privilege log and consistent redaction;
- the chain of custody documentation that will accompany production;
- the ability to authenticate records, including who can testify to collection;
- retention and disposition after the matter closes;
- consistency between what was collected, what was reviewed, and what is produced.

A collection that cannot be reproduced or explained undermines production. Regulators and courts assess not only the evidence but the process that produced it.

## Common Traps

### Issuing A Hold But Not Confirming It Took Effect

A hold letter sent to custodians does not stop automated deletion if the retention engine keeps running. Confirm with IT that the systems are actually preserving data.

### Collecting From The Obvious Sources Only

Limiting collection to the subject's mailbox misses shared drives, chat platforms, personal devices, finance systems, and logs where the real evidence often lives. Map the universe from the allegation, not from convenience.

### Breaking Chain Of Custody Through Casual Handling

Letting a manager copy files to a personal drive, or reviewing originals without logging access, creates custody gaps. Handle evidence through controlled processes from the first touch.

### Over-Filtering And Losing Relevant Material

Aggressive deduplication or narrow search terms can silently discard the key document. Validate filters against known relevant items before relying on them.

### Ignoring Privacy And Cross-Border Limits

Imaging devices or reading communications without a legal basis can breach privacy law and create a new violation. Coordinate with counsel on monitoring notice, works council rules, and transfer restrictions.

### Treating Backups As A Substitute For Live Preservation

Backups are point-in-time and may not capture the relevant window or may be overwritten on a cycle. Do not assume a backup exists when you need it; preserve live data first.

### Failing To Capture Volatile Or Platform-Specific Data

Chat platforms and cloud apps may not retain edits or deletions, and logs roll over quickly. Act before the platform's retention expires, and capture volatile data promptly where relevant.

## Self-Check

- Has a preservation hold been issued, confirmed as effective against automated deletion, acknowledged by custodians, and periodically re-issued for long matters?
- Is the collection universe defined from the factual questions, with custodians, systems, time window, and data types documented, and exclusions justified?
- Are collection methods forensically sound, using imaging or server-side export, capturing hash values, preserving metadata, and working from copies?
- Is chain of custody documented for every item from collection through storage, processing, and analysis, with unique identifiers, handoff logs, and access controls?
- Are processing steps, including deduplication, filters, search terms, and analytics, documented and validated against known relevant documents?
- Has the forensic analysis targeted the artifact types that answer the factual questions, including logs, deleted-file recovery, and off-channel communications where relevant?
- Have privilege, privacy, employee monitoring notice, works council, and cross-border transfer limits been coordinated with counsel before collection?
- Is the collection planned for likely production, with metadata, privilege log, authentication, and retention considered from the outset?
- Are IT and security engaged early with specific preservation guidance, including volatile and platform-specific data?
- Can the collection be reproduced and explained if challenged by a regulator, auditor, or court?
