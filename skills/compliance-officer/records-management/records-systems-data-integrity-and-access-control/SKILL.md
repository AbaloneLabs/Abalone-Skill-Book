---
name: records-systems-data-integrity-and-access-control.md
description: Use when the agent is designing or governing a records system, setting access controls and audit trails for regulated records, ensuring the integrity and authenticity of records over their retention life, or preparing records to serve as admissible evidence in legal or regulatory proceedings.
---

# Records Systems, Data Integrity, and Access Control

A record that cannot be proven authentic, complete, and unaltered has limited value as evidence and may fail to satisfy regulatory recordkeeping requirements. Agents often focus on storing records and overlook whether the system preserves their integrity, who can access and modify them, and whether an audit trail can demonstrate that the record produced in a proceeding is the same record that was originally created. The judgment problem is designing records systems that maintain integrity and authenticity over long retention periods, controlling access so that records cannot be altered without detection, and ensuring the system can produce admissible evidence when called upon.

This skill applies to records management, IT, compliance, and legal operations governing electronic and physical records systems. Recordkeeping, evidentiary, and electronic-signature requirements differ by jurisdiction, sector, and record type. Verify the applicable integrity and admissibility standards and consult counsel for systems holding records likely to be litigated or examined by regulators.

## Core Rules

### Design for Integrity, Authenticity, and Completeness as System Properties

Record integrity is not achieved by hoping users do not alter files; it is a property of the system. A credible records system maintains integrity through controls such as write-once or append-only storage for records of record, versioning that preserves the complete history of any changes, checksums or hashing that can detect alteration, and timestamps from a trusted source. Authenticity, the ability to prove the record is what it claims to be and originated where it claims, requires metadata capturing origin, creation time, author, and chain of custody. Completeness requires that the record and its associated metadata are preserved together, because a record stripped of its metadata may be inadmissible or meaningless. Design these properties into the system at the outset, because retrofitting integrity controls onto a system that allowed free modification is often impossible.

### Enforce Least-Privilege Access With Meaningful Segregation

Access to records should follow least privilege: each user can access only the records necessary for their role, and the ability to create, read, modify, and delete is granted separately and narrowly. For sensitive record categories such as personnel files, safety incident records, investigation files, and customer personal data, apply heightened restrictions and segregation of duties so that no single user can both alter a record and conceal the alteration. Broad access and shared administrative accounts defeat integrity, because actions cannot be attributed to individuals and alterations cannot be detected. Review access rights periodically and remove access promptly on role change or departure.

### Maintain Audit Trails That Can Prove the Record's History

An audit trail is the evidence that a record has not been tampered with. It should capture who accessed, created, modified, or deleted each record, when, and what changed, in a log that is itself protected from alteration. The audit trail must be sufficient to reconstruct the record's history and to demonstrate that the version produced in a proceeding is authentic. Systems that allow modification without logging, or whose logs can be edited by administrators, cannot support integrity claims. Test the audit trail by attempting to alter a record and confirming the action is captured and attributable.

### Preserve Readability and Format Viability Over the Retention Life

A record that cannot be opened or read at the end of its retention period is functionally lost, even if the bits are preserved. Format obsolescence, dependency on specific software versions, link rot, and the loss of context all threaten long-term readability. For records with long retention, choose durable formats, migrate records as formats and systems change, preserve the metadata and context needed to interpret the record, and periodically verify that archived records can actually be opened and rendered. A backup that cannot be restored is not a preservation strategy; test restoration on a defined cadence.

### Manage Electronic Signatures and Workflow Evidence Appropriately

Where records are signed or approved through electronic workflows, the signature evidence and the workflow history are part of the record. Different signature types carry different evidentiary weight, from simple e-signatures to advanced and qualified signatures backed by certificates. Match the signature type to the legal and evidentiary requirements of the record type, and preserve the signature validation evidence, the certificate chain, and the workflow audit trail alongside the signed record. A signed record whose validation evidence has expired or been discarded may be difficult to authenticate years later.

### Prepare for Litigation and Regulatory Production Before It Happens

The test of a records system is whether it can produce relevant records quickly, completely, and with integrity when litigation or a regulatory inquiry arises. This requires that records are organized and searchable, that legal holds can be applied across the system, that export and production tools preserve metadata, and that the system can demonstrate chain of custody for produced records. Organizations that discover, only after litigation begins, that their records are scattered across unsanctioned tools, that search does not work, and that metadata is stripped on export face expensive and risky production. Design the system for the production scenario, not only for storage.

## Common Traps

### Storage Without Integrity Controls

A system that allows users to modify or overwrite records freely cannot support integrity or authenticity claims. Integrity must be a system property enforced through write controls, versioning, and hashing.

### Shared Accounts and Unattributable Actions

When actions cannot be attributed to individuals, alterations cannot be detected and accountability is impossible. Eliminate shared credentials for any system holding records of record.

### Audit Trails That Administrators Can Alter

An audit log that can be edited or deleted by an administrator provides no integrity assurance. Protect logs with segregation of duties and append-only or write-once storage.

### Preserving Bits but Losing Readability

A record in an obsolete format, or a backup that cannot be restored, is functionally lost. Plan format migration, context preservation, and restoration testing across the retention life.

### Stripping Metadata on Export

Producing records without their metadata undermines admissibility and completeness. Export and production tools must preserve the metadata and chain-of-custody evidence.

### Discovering System Gaps During Litigation

Learning that records are in unsanctioned tools, search is broken, or metadata is lost only when production is demanded turns a records problem into a litigation crisis. Design for production from the outset.

## Self-Check

- Did I design the system for integrity, authenticity, and completeness through write controls, versioning, hashing, and metadata capture, rather than relying on user behavior?
- Did I enforce least-privilege access with segregation of duties for sensitive record categories, and review and revoke access promptly on role change or departure?
- Does the audit trail capture all access and modifications in a tamper-protected log, and have I tested it by attempting an alteration?
- Did I plan for long-term readability through durable formats, migration, context preservation, and periodic restoration testing?
- Did I match electronic-signature types to the record's legal and evidentiary requirements and preserve the validation evidence and workflow history?
- Did I design the system for litigation and regulatory production, including searchability, legal-hold integration, metadata-preserving export, and chain-of-custody demonstration?
- Did I confirm the applicable recordkeeping, evidentiary, and admissibility standards with counsel for systems holding records likely to be litigated or examined?
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
