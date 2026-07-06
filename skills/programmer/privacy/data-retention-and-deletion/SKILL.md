---
name: data_retention_and_deletion.md
description: Use when the agent is defining how long personal data is kept, building deletion or anonymization pipelines, implementing user deletion or "right to be forgotten" requests, handling data across primary stores, replicas, backups, logs, analytics warehouses, exports, caches, and derived datasets; deciding retention periods per data category; reconciling deletion with legal hold or financial record-keeping obligations; verifying that a deletion actually reaches every copy; or reviewing a system for compliance with retention and erasure obligations under GDPR, CCPA/CPRA, or similar regimes. Also covers the gap between logical deletion and physical deletion, backup restoration reintroducing deleted data, derived and aggregated data, deletion verification, and the architectural cost of retrofitting deletion into a system never designed for it.
---

# Data Retention And Deletion

Retention and deletion are the back end of the data lifecycle, and they are where privacy obligations meet architectural reality. The promise a system makes when it collects data — "we keep this for X purpose and delete it when done" — is easy to state and remarkably hard to honor, because data does not live in one place. A user's record exists in the primary database, in read replicas, in nightly backups, in streaming pipelines, in an analytics warehouse, in exported CSVs, in a CRM, in logs, in caches, in feature stores, and in derived aggregates. A deletion that removes the row from the primary table and declares victory has deleted one copy of perhaps a dozen, and the other eleven are now an unmanaged personal-data store you forgot you had. The defining retention failure is not refusing to delete; it is deleting incompletely and believing the obligation is met.

Agents tend to under-invest here because deletion is invisible in feature work and the architecture rarely supports it. The happy path is "delete the user, the row goes away, the user is gone." The defects live in the copies the happy path does not touch: a backup restored three months later that reintroduces the deleted user; an analytics table keyed by user id that retains every event; a derived "user segment" that persists the relationship; a log line that captured the email; a third-party processor that was never notified. The judgment problem is treating retention and deletion as a system-wide data-flow property that must be designed in, enumerated across every store, and verified — not as a `DELETE FROM users WHERE id = ?` that concludes the task.

This skill is about making retention limits and deletion real across a distributed data footprint. It complements the PII skill (which covers minimization, purpose, and what to collect in the first place) and the anonymization skill (which covers transforming data so it is no longer personal). Here the question is: when data must stop existing, how do you make it actually stop existing everywhere.

## Core Rules

### Inventory Every Store Before Promising Deletion

You cannot delete from a store you have not enumerated. The first task in any retention or deletion design is a complete inventory of where each data category lives and how it moves. For each category of personal data, list:

- **Primary store and all replicas** (read replicas, standby, cross-region copies).
- **Backups and snapshots** (full, incremental, PITR / point-in-time-recovery logs), and their retention windows.
- **Logs and observability** (application logs, access logs, audit logs, error trackers) that captured the data.
- **Analytics and data warehouse** (event tables, fact tables, materialized views, feature stores).
- **Search and secondary indexes** (Elasticsearch, search indices, denormalized join tables).
- **Caches** (Redis, Memcached, CDN, process-local caches).
- **Exports, dumps, and shared artifacts** (CSVs sent to partners, data shared with subprocessors).
- **Derived and aggregated data** (segments, scores, profiles, model training sets derived from the raw data).
- **Third-party processors** (CRM, analytics vendors, email providers, AI/LLM services) that received the data.

A deletion design that has not enumerated these is a design that will miss at least one. The inventory is the foundation; without it, "we deleted the user" is a hope, not a fact.

### Tie Retention Periods To Purpose, And Make Expiry Automated

Data should live only as long as its stated purpose requires, and the end of that period must be enforced by automation, not by someone remembering. Indefinite retention is both a liability and a signal that no one decided.

- **Define a retention period per data category, tied to the purpose** stated at collection. Transactional data is kept for the transaction and the legal/financial record-keeping window; analytics data is kept for the analytics horizon; debug logs for days, not years.
- **Automate deletion or anonymization at expiry.** Scheduled jobs, TTLs, or lifecycle policies that move data to deletion. Retention that depends on manual action is retention that will be ignored.
- **Distinguish "active" from "archived" retention.** Data may move from hot storage to cold archive to deletion; each transition should be automated and time-bounded.
- **Reconcile with legal hold.** Some data must be retained longer than its purpose due to litigation, regulatory, or financial obligations (e.g., tax records). Legal hold is an exception that overrides deletion, documented per case, not a reason to retain everything forever "just in case."

The strongest designs make deletion the default state and retention the deliberate exception. Anything kept beyond its purpose is a documented decision, not an accident of never having deleted it.

### Make Deletion Reach Every Copy, Including Backups

The hardest deletion problem is the indirect copy. Deleting the primary row is easy; deleting the row from the backup that will be restored in three months is not physically possible without rewriting the backup. The honest approach is to enumerate the copy types and handle each:

- **Live stores (primary, replicas, caches, search, analytics).** Delete or anonymize directly, and verify the deletion propagated (respecting replication lag).
- **Backups and snapshots.** You generally cannot rewrite backups. The standard approach is to let existing backups age out on their retention schedule, while ensuring the deleted record is purged from any backup that is restored. Document this explicitly: "deletion is effective in live systems immediately; backups containing the record expire on their normal schedule (X days)." For sensitive categories, consider encryption with per-user or per-category keys so that destroying the key renders the backup data unreadable (crypto-shredding).
- **Logs and event streams.** These are append-only and often cannot be surgically edited. Prevent PII from entering them in the first place (see the PII skill); where it has, plan for redaction or short retention.
- **Derived data.** A segment, score, or aggregate derived from the user's data may need deletion or recomputation. Decide whether the derived artifact is personal data (it usually is, if keyed to the user) and treat it accordingly.
- **Third-party processors.** Forward the deletion to every processor that holds the data, and confirm completion. Maintain the recipient list as part of the inventory.

A deletion that does not account for backups and derived data is incomplete. State the policy for each copy type explicitly rather than implying total physical deletion that the architecture cannot deliver.

### Design For "Right To Be Forgotten" And Access Requests From The Start

Most privacy regimes give individuals the right to access, correct, export, and delete their data. These are obligations, and they are expensive to retrofit into a system that was not designed to answer "where is this user's data." Design for them at the start:

- **A user-data map keyed by user id.** For any user, the system can enumerate every store that holds their data, so an access or deletion request can fan out to all of them. Without this, a deletion request becomes a multi-team archaeological dig.
- **Idempotent, verifiable deletion jobs.** A deletion that partially fails and cannot be re-run safely leaves the user half-deleted. Make deletion operations idempotent and produce a verifiable result (every targeted store reports zero rows for the user).
- **Access and export.** The right of access and data portability require producing the user's data in a usable format. This is only feasible with the data map above.
- **Timeliness.** Regimes impose deadlines (e.g., 30 days). A manual, multi-week process per request does not scale and risks missing the deadline; automate the fan-out.

The architectural cost of supporting these rights after launch is far higher than designing for them at the start. If the system cannot answer "delete this user everywhere," that is an architectural gap, not a future task.

### Distinguish Logical Deletion, Anonymization, And Physical Deletion

"Delete" is ambiguous, and the choice affects what the system can do later and what the obligation actually requires.

- **Logical deletion (soft delete).** Mark the record inactive or tombstoned, keep the data. Useful for undo, audit, and referential integrity, but the data is still personal data and still under retention obligations. A soft-deleted record that lives forever is not deletion.
- **Anonymization.** Irreversibly transform the data so it no longer relates to an identifiable individual (see the anonymization skill). If successful, the result is no longer personal data and obligations largely fall away. This is often the right end-state for analytics data you want to keep.
- **Physical deletion.** Remove the data so it cannot be recovered. The strongest guarantee for "right to be forgotten," subject to the backup caveat above.
- **Crypto-shredding.** Encrypt data with a per-user key and destroy the key to render the data unreadable. A practical way to achieve effective deletion for backups and distributed stores where surgical row deletion is infeasible.

Choose deliberately per category. Soft-deleting everything and calling it deletion leaves permanent liability; physically deleting data you need for audit breaks compliance the other way. Match the mechanism to the obligation.

### Verify Deletion Actually Worked

Deletion that is not verified is an assertion. Build verification into the process:

- **Post-deletion checks.** After a deletion job runs, query every store in the inventory and confirm zero rows (or zero re-identifiable artifacts) for the user. A store that still returns data is a gap.
- **Reconciliation across replicas and lag.** Account for replication lag; verify after the lag window, not immediately.
- **Audit trail of the deletion itself.** Record that a deletion was requested, executed, and verified — without retaining the deleted personal data in the audit record (record the fact of deletion, not the data).
- **Periodic sweeps.** Run retention sweeps on a schedule and verify they delete what they should, alerting on anomalies (a category that is not aging out, a store that is growing instead of shrinking).

A deletion pipeline without verification will silently miss stores, and you will learn about it from a regulator or a breach, not from your own monitoring.

## Common Traps

### Deleting The Primary Row And Declaring Done

Removing the user from the primary table and treating the deletion as complete, while the data persists in replicas, backups, logs, analytics, exports, caches, and third-party processors. Enumerate every store; deletion must reach all of them or be explicitly scoped per copy type.

### Backups That Silently Reintroduce Deleted Data

A backup restored months later that brings back deleted users, because the deletion never reached the backup and no policy handled it. Use crypto-shredding or documented backup-expiry-on-schedule; never assume a restored backup is clean.

### Indefinite Retention With No Policy

Storing personal data with no retention rule and no deletion mechanism, so it accumulates forever across every store. Every category needs a retention period tied to purpose and an automated path to deletion or anonymization.

### Soft Delete Treated As Compliance With Erasure

Marking records inactive (soft delete) and treating the erasure obligation as met, while the data remains fully present and under obligation. Soft delete is a state, not deletion; the data must eventually be anonymized or physically deleted.

### Derived And Aggregated Data Forgotten

Deleting raw user events but leaving derived segments, scores, profiles, or model training sets keyed to the user, which remain personal data. Treat derived artifacts as personal data and delete or anonymize them too.

### No Data Map, So Deletion Requests Become Manual Archaeology

A deletion request that requires engineers to hunt across every service and store to find a user's data, missing the regulatory deadline and likely missing stores. Build a user-data map keyed by user id so requests fan out automatically.

### Logs And Analytics That Cannot Be Surgically Edited and third-Party Processors Never Notified

PII that entered append-only logs or event streams and now cannot be removed without rewriting the whole store. Prevent PII from entering these stores; where it has, rely on short retention or redaction.

Deleting the user internally but never forwarding the deletion to the CRM, analytics vendor, or email provider that holds a copy. Maintain the processor list and forward deletion requests with confirmation.

### Deletion That Cannot Be Verified and legal Hold Used As An Excuse To Retain Everything

A deletion job that runs with no post-checks, so a store it silently missed remains populated indefinitely. Verify deletion across every store after it runs, and alert on anomalies.

Invoking "we might need it for legal reasons" to retain all data indefinitely, when legal hold is a per-case exception, not a blanket policy. Document specific holds; retain the rest per purpose.

## Self-Check

- [ ] A complete inventory exists of every store holding each personal-data category (primary, replicas, backups, logs, analytics, search, caches, exports, derived data, third-party processors), and the deletion design was built from this inventory rather than from the primary table alone.
- [ ] Each data category has a retention period tied to its collected purpose, with automated deletion or anonymization at expiry; indefinite retention does not exist except as a documented legal-hold or record-keeping exception.
- [ ] Deletion reaches every live store and is verified across replicas (accounting for lag); backups are handled by documented expiry-on-schedule or crypto-shredding, with the policy stated explicitly rather than implying impossible physical deletion.
- [ ] Derived and aggregated data (segments, scores, profiles, training sets) keyed to a user is treated as personal data and deleted or anonymized, not overlooked.
- [ ] The system supports access, correction, export, and erasure requests from a user-data map keyed by user id, with idempotent, verifiable deletion jobs that fan out to every store and meet regulatory deadlines.
- [ ] Logical deletion (soft delete), anonymization, physical deletion, and crypto-shredding are distinguished and chosen deliberately per category — soft-deleted data is not treated as erased, and data needed for audit is not physically destroyed.
- [ ] Deletion is verified by post-deletion checks across every store in the inventory, with an audit trail recording the fact of deletion (not the deleted data), and periodic retention sweeps alert on stores that are not aging out.
- [ ] Third-party processors that hold the data are enumerated and forwarded deletion requests with confirmation of completion.
- [ ] For binding obligations, the retention and deletion design was checked against the applicable regime (GDPR, CCPA/CPRA, etc.) and a qualified reviewer, not relied on as legal advice.
