---
name: data_retention_and_deletion_schedules.md
description: Use when the agent is setting retention periods, building deletion schedules, managing legal hold interactions, verifying deletion across backups, or deciding between anonymization and pseudonymization for end-of-life data.
---

# Data Retention And Deletion Schedules

Retention is where privacy obligations meet operational reality. Keeping data too long violates minimization and expands breach exposure; deleting it too soon can destroy evidence, breach legal obligations, or break a product. The hard part is not picking a number but building a system that actually deletes data on schedule, survives legal holds, and proves deletion across backups and replicas. A retention policy that nobody enforces is a liability, because it documents an unmet commitment.

Use this skill before setting retention periods, designing deletion workflows, responding to a deletion request, or interacting with legal hold. The goal is to make the agent build retention as an enforced, verifiable operational process rather than a policy on a shelf.

## Core Rules

### Set Retention Based On Purpose And Legal Drivers

Retention periods should be derived from the purpose of processing and from legal requirements, not from storage convenience or "we might need it."

For each data category, determine:

- the purpose the data serves and when that purpose ends;
- any legal minimum retention required by law (tax, employment, AML, healthcare records);
- any legal maximum or limitation period that discourages indefinite retention;
- business needs such as warranty, dispute defense, or audit cycles;
- the shortest period that satisfies all drivers.

Default to the shortest defensible period. Indefinite retention is almost never defensible and should require explicit justification.

### Document Retention In A Schedule Linked To The Inventory

A retention schedule is the operational expression of the policy. It must be specific, owned, and linked to the data inventory and RoPA.

Each schedule entry should record:

- the data category and the systems where it lives;
- the retention period and the start event (creation, last activity, relationship end);
- the legal or business basis for the period;
- the deletion method (hard delete, anonymization, archival);
- the owner accountable for enforcement;
- the review cadence.

Retention expressed as "as long as necessary" is not a schedule. State the period or the objective criteria used to compute it.

### Enforce Deletion Operationally

A retention schedule that depends on manual deletion will not be followed. Deletion must be engineered into systems and workflows.

Build enforcement by:

- automating deletion or anonymization jobs triggered by the start event;
- building deletion into data pipelines so downstream copies are also purged;
- using tombstones or hard deletes rather than soft deletes that quietly retain data;
- verifying that deletion propagates to caches, search indexes, logs, and warehouses;
- handling derived data such as scores, segments, and embeddings that depend on the deleted record.

If deletion only removes the primary record while copies persist in analytics, backups, and ML datasets, the data has not been deleted.

### Manage The Legal Hold Interaction

Legal hold suspends deletion to preserve evidence. The interaction between retention schedules and legal hold is a frequent source of error: data deleted despite a hold destroys evidence, while holds that never release cause indefinite retention.

Operate legal hold by:

- maintaining a current list of holds with scope, custodians, and matter reference;
- integrating hold flags into deletion jobs so held data is preserved;
- defining a release process with a trigger, review, and resumption of deletion;
- reviewing holds periodically to release those no longer needed;
- documenting the hold lifecycle so the preservation can be demonstrated.

A hold that was never formally released is indistinguishable from a decision to retain indefinitely.

### Verify Deletion Across Backups And Replicas

Backups are the graveyard of deletion commitments. Deleting a live record while it persists in backups means the data is not truly gone, and regulators expect a credible position on backup retention.

Address backups by:

- documenting backup retention and rotation cycles;
- defining how long deleted data may persist in backups before natural expiry;
- ensuring that restored data from backups re-applies deletion rules or holds;
- avoiding indefinite or unmanaged backup retention;
- considering encryption with key destruction as a supplementary measure where physical deletion is infeasible in the short term.

State the backup position honestly. Claiming deletion while relying on indefinite backups is not credible.

### Distinguish Anonymization From Pseudonymization

End-of-life data can be deleted or anonymized. The distinction between anonymization and pseudonymization determines whether the data remains in scope of privacy law.

Apply the distinction correctly:

- anonymization irreversibly prevents re-identification, taking the data out of scope of GDPR and similar laws;
- pseudonymization replaces direct identifiers but remains personal data because re-identification is possible, especially with a key;
- aggregation or tokenization is not automatically anonymization; re-identification risk must be assessed against motivated intruders and available auxiliary data.

Document the anonymization method and the re-identification analysis. Anonymization claimed without analysis is pseudonymization in disguise.

### Handle Deletion Requests Consistently With Schedules

Individual deletion rights interact with retention schedules. A deletion request does not always require immediate destruction if another legal basis for retention exists, but the response must be consistent and documented.

Reconcile by:

- honoring deletion where no overriding legal basis applies;
- retaining only where a specific legal obligation, legal hold, or defense basis requires;
- communicating clearly to the individual about what is deleted and what is retained and why;
- ensuring retention beyond a deletion request is the exception, not the default.

### Review Retention When Purposes Change

Retention is not static. When a purpose ends, a product is discontinued, or a legal driver changes, retention should be reassessed.

Trigger review on:

- product or feature retirement;
- end of a customer or vendor relationship;
- change in legal obligations;
- completion of a legal matter and release of hold;
- acquisition or divestiture.

## Common Traps

### Indefinite Retention Disguised As Policy

"We retain data as long as necessary for business purposes" is indefinite retention. Without a concrete period or criteria, it is a minimization and purpose limitation failure.

### Soft Deletes That Retain Data

Marking a record as deleted while keeping the underlying data leaves the data in scope and the deletion commitment unmet.

### Deletion That Stops At The Primary Database

Deleting a row in the primary database while copies persist in warehouses, caches, logs, and ML datasets means the data is not deleted.

### Ignoring Backup Reality

Pretending backups do not exist, or claiming immediate deletion while backups retain data for years, undermines the credibility of the entire program.

### Treating Pseudonymization As Anonymization

Tokenized or keyed data is still personal data. Treating it as out of scope because identifiers were replaced is a recurring and costly error.

### Legal Holds That Never Release

A hold applied for a matter and never released causes indefinite retention and accumulates risk long after the matter closes.

### Retention Driven By Storage Convenience

Keeping data because deletion is technically hard or because "we might want it later" is not a retention basis. The difficulty of deletion is a problem to solve, not a justification to retain.

## Self-Check

- Is each data category assigned a retention period derived from purpose and legal drivers, defaulting to the shortest defensible period?
- Is the retention schedule specific, owned, and linked to the data inventory and RoPA, with start events and deletion methods defined?
- Is deletion automated and engineered into systems, pipelines, caches, search indexes, and derived data?
- Is legal hold integrated into deletion jobs, with a documented release process and periodic review?
- Is the backup position documented honestly, including how long deleted data may persist and how restored data re-applies deletion?
- Are anonymization and pseudonymization correctly distinguished, with re-identification risk assessed and documented?
- Are individual deletion requests reconciled with retention schedules, retaining data only where a specific overriding basis exists?
- Is retention reviewed when purposes end, products retire, relationships end, or legal drivers change?
- Are soft deletes avoided or accompanied by eventual hard deletion of the underlying data?
- Can the organization demonstrate, with evidence, that deletion actually occurred across primary and secondary stores?
