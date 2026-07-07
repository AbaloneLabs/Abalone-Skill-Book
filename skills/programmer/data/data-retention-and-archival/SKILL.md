---
name: data_retention_and_archival.md
description: Use when the agent is designing a data retention policy, archival strategy, or data lifecycle; deciding how long to keep data, when to move it to cheaper storage, when to delete it; handling GDPR/CCPA right-to-erasure and legal holds; managing table growth and query performance as data accumulates; designing time-series or log retention; or dealing with storage cost growth. Covers retention policy definition (legal, compliance, operational needs), tiered storage (hot/warm/cold), deletion vs archival, the right-to-erasure and data-subject rights, legal holds, and the performance and cost consequences of unbounded data accumulation.
---

# Data Retention And Archival

Data accumulates. Every transaction, every log line, every event, every user record adds to a store that, without a retention policy, grows without bound — and unbounded growth has compounding consequences. Storage cost rises linearly with volume. Query performance degrades as tables grow (more rows to scan, larger indexes, more cache pressure). Backup and restore times lengthen. Compliance risk increases (more data to breach, more data subject to regulation). And the cost of eventual deletion rises (more data to find, more references to sever, more systems to coordinate). The discipline of data retention is defining, for each class of data, how long it is kept, where it lives at each stage of its lifecycle, and when it is deleted or archived — and doing so before unbounded growth makes the problem acute. A retention policy is not just a cost-saving measure; it is a correctness, performance, and compliance obligation, and defining it late (after the table has billions of rows) means paying the cost of a migration that early planning would have avoided.

Agents tend to treat data as permanent ("we might need it someday"), to defer deletion decisions, and to handle compliance requests (right-to-erasure) reactively rather than designing for them. The judgment problem is recognizing that data has a lifecycle whose stages (hot, warm, cold, deleted) have different storage and access needs, that retention is driven by legal, compliance, and operational requirements that must be identified, and that deletion (especially under regulation) is a design problem requiring the data to be structured for findability and severability. This skill covers the discipline of data retention and archival: policy definition, tiered storage, deletion and the right-to-erasure, legal holds, and the performance and cost management that retention enables.

## Core Rules

### Define Retention Policy Per Data Class, Driven By Requirements

Different data classes have different retention needs, driven by legal, compliance, and operational requirements. Define the policy per class.

- **Identify the retention drivers: legal, compliance, operational, analytical.** Legal/regulatory requirements mandate minimum retention (financial records for 7 years, health records per HIPAA). Compliance requirements (GDPR's data minimization) mandate limits on retention. Operational needs (customer support, troubleshooting) want recent data accessible. Analytical needs (trends, ML training) may want long histories.
- **Define retention per data class, reconciling the drivers.** A single retention policy for all data oversimplifies; transactions, logs, user profiles, and analytics each have different needs. For each class, define the retention period (how long kept) and the disposition (archive or delete) at end-of-life, reconciling the often-conflicting drivers (keep for compliance vs delete for minimization).
- **Document the policy and the rationale.** A retention policy that exists only in someone's head is unenforceable. Document what is kept, for how long, where, and why; make the policy the basis for automated lifecycle enforcement.
- **Review the policy as requirements change.** Regulations evolve, business needs shift, and the policy must keep up. Review periodically; update the lifecycle enforcement to match.

### Use Tiered Storage (Hot/Warm/Cold) To Match Cost To Access Needs

Data's access pattern changes over its lifecycle — recent data is accessed frequently, old data rarely. Tiered storage matches the storage cost to the access frequency.

- **Hot storage: recent, frequently-accessed data on fast, expensive storage.** The data serving production queries (recent transactions, active user records) lives on fast storage (SSD, the primary database) optimized for low-latency access.
- **Warm storage: less-recent, occasionally-accessed data on medium-cost storage.** Data accessed sometimes (last year's records, support history) moves to cheaper storage (a separate database, object storage with an index) where queries are slower but cheaper.
- **Cold storage/archival: old, rarely-accessed data on cheap storage.** Data kept for compliance or rare analysis (records from years ago) moves to archival storage (object storage, tape) at minimal cost, accessible on demand but with high latency.
- **Automate the tier transitions.** Data should move from hot to warm to cold automatically based on age (a lifecycle rule), not manually. Define the transition points (after 30 days to warm, after 1 year to cold) and let the storage system enforce them.

### Design For Deletion And The Right-To-Erasure

Under regulations like GDPR and CCPA, users have the right to request deletion of their data (right-to-erasure, right to be forgotten). Designing for this is a structural problem, not a reactive one.

- **Know where all of a user's data lives.** A right-to-erasure request requires finding and deleting all of a user's data across all systems (primary database, backups, logs, analytics, caches, archival). Data scattered across systems without a map makes complete deletion nearly impossible. Maintain a data inventory or use a user identifier that allows finding all data.
- **Design data to be severable by user.** Data structured so a user's records can be found and deleted (by user id) supports erasure; data where a user's data is intermingled (aggregated into anonymous stats, embedded in shared records) may not need per-user deletion but must be designed to anonymize on erasure.
- **Handle backups and archival.** A user's data in backups or archival is not immediately deletable (backups are point-in-time snapshots). Design so that the data ages out of backups within a defined period (the backup retention), and document that erasure completes when the backups containing the data expire.
- **Distinguish deletion from anonymization.** True deletion removes the data; anonymization severs the link to the user (the data remains but is no longer personal data). For analytical data, anonymization (aggregating, removing identifiers) may suffice and preserve analytical value.
- **Support other data-subject rights: access, portability, rectification.** Regulations also grant the right to access one's data, receive a copy (portability), and correct inaccurate data. Designing for findability supports all these rights.

### Handle Legal Holds Exceptionally

A legal hold suspends the normal retention policy for specific data (relevant to litigation or investigation), requiring it to be preserved beyond its normal deletion date.

- **Implement legal hold as an exception to the retention policy.** Data under legal hold is not deleted or archived per the normal lifecycle; it is preserved until the hold is released. The hold mechanism must override the automated lifecycle enforcement.
- **Scope the hold precisely.** A hold that is too broad preserves more data than necessary (cost, risk); too narrow risks losing relevant data. Scope the hold to the specific data relevant to the matter (by user, date range, data type).
- **Track holds and their release.** A hold that is never released preserves data indefinitely. Track active holds; release them when the matter concludes; allow the data to proceed through its lifecycle.
- **Integrate holds with the deletion pipeline.** The deletion pipeline must check for active holds before deleting; a hold must prevent deletion reliably. Test this integration — a hold that fails to prevent deletion is a legal liability.

### Manage Performance And Cost Through Retention

Retention is a primary lever for query performance and storage cost. Actively managing it prevents the degradation that unbounded growth causes.

- **Bound table size via retention to maintain query performance.** A table with billions of rows is slower to query (more scanning, larger indexes) than one with millions. Retention (deleting or archiving old data) bounds the table size, maintaining performance as new data arrives.
- **Use time-based partitioning to enable efficient retention.** Partitioning data by time (daily or monthly partitions) allows dropping old partitions (instant deletion of a time range) rather than row-by-row deletion (slow, fragmentation-inducing). Time-series data and logs benefit especially.
- **Monitor storage cost and growth.** Track storage volume and cost by data class; alert on unexpected growth (a bug generating excess data, a missing retention policy). Storage cost that grows without bound is a signal to enforce or tighten retention.
- **Avoid keeping data "just in case."** The temptation to retain data speculatively ("we might need it for analysis") ignores the cost (storage, performance, compliance risk) and the availability of alternatives (sampling, aggregation, archival). Keep what is needed; archive or delete the rest.

## Common Traps

### No Retention Policy (Unbounded Growth)

Data kept indefinitely "just in case," growing storage cost, degrading query performance, and increasing compliance risk. Define a retention policy per data class.

### Retention Policy Not Enforced Automatically

A policy that exists on paper but is not implemented in the data lifecycle, so data accumulates despite the policy. Automate tier transitions and deletion.

### Right-To-Erasure Not Designed For

User data scattered across systems without a way to find and delete it all, making compliance requests slow, incomplete, or impossible. Map data locations; design for severability.

### Backups Ignored In Erasure

A right-to-erasure that deletes primary data but leaves the user's data in backups indefinitely, never truly completing. Design for backup expiration; document the completion timeline.

### Legal Hold Not Integrated With Deletion

A deletion pipeline that does not check for legal holds, deleting data under hold (legal liability), or a hold mechanism that fails to override the lifecycle. Integrate and test.

### Deletion As Row-By-Row On Huge Tables

Deleting old data row-by-row on a massive table, causing fragmentation and locking. Use time-based partitioning to drop old partitions efficiently.

### All Data On Hot Storage

Keeping all data (including rarely-accessed old data) on fast expensive storage, inflating cost. Use tiered storage; move cold data to cheap storage.

### Retention Driven Only By Cost (Ignoring Compliance)

Deleting data aggressively to save cost, violating legal retention requirements (records that must be kept for N years). Reconcile cost with legal and compliance drivers.

## Self-Check

- [ ] A retention policy is defined per data class (transactions, logs, user profiles, analytics), driven by identified requirements (legal minimums, compliance limits, operational needs, analytical value), reconciling conflicting drivers, documented with rationale, and reviewed as requirements change.
- [ ] Tiered storage (hot/warm/cold) matches storage cost to access frequency — recent frequently-accessed data on fast storage, older data on cheaper storage, archival data on minimal-cost storage — with automated tier transitions based on age.
- [ ] The system supports the right-to-erasure and other data-subject rights (access, portability, rectification): all of a user's data is findable across systems (data inventory or user identifier), data is structured to be severable by user, backups and archival are handled (erasure completes when they expire), and anonymization is used where deletion is not required.
- [ ] Legal holds are implemented as exceptions to the retention policy (preserving specific data beyond its normal deletion), scoped precisely, tracked to release, and integrated with the deletion pipeline (reliably preventing deletion of held data, tested).
- [ ] Table size is bounded via retention to maintain query performance, time-based partitioning enables efficient retention (dropping old partitions rather than row-by-row deletion), and storage cost and growth are monitored with alerting on unexpected growth.
- [ ] Deletion is designed to be efficient (partition drops, batch deletes) rather than row-by-row on huge tables, and the deletion pipeline handles dependencies (foreign keys, references, derived data in caches or analytics) to achieve complete deletion.
- [ ] Data is not kept speculatively ("just in case") — the retention policy reflects actual needs, and alternatives (sampling, aggregation, archival) are used where full-fidelity retention is not justified.
- [ ] The retention policy and its enforcement are documented and auditable, so compliance with legal and regulatory retention requirements can be demonstrated, and the system's data holdings can be accounted for.
