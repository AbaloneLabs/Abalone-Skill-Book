---
name: data_conversion_and_migration.md
description: Use when the agent is migrating accounting data between systems, converting a chart of accounts or master data, migrating open balances and transaction history, validating that migrated data reconciles to the source, or planning a cutover from an old accounting system to a new one.
---

# Data Conversion And Migration

Moving from one accounting system to another requires migrating data: chart of accounts, master data, open balances, and often transaction history. This is where system implementations most often fail, and where the failures are most expensive. A migration that drops transactions, mismaps accounts, corrupts master data, or carries over unreconciled balances produces a new system that cannot be trusted from day one. The damage is often invisible until the first close, when the new system's balances do not tie to anything and the team is left reconstructing history under time pressure.

Use this skill before migrating accounting data, converting a chart of accounts or master data, migrating open balances or history, validating migrated data, or planning a cutover. The goal is to prevent the agent from executing a migration that looks complete but carries over errors, drops transactions, or produces a new system whose balances cannot be reconciled or trusted.

## Core Rules

### Plan The Migration Scope And Strategy Deliberately

Migration is not all-or-nothing. Decide what moves, in what form, and with what level of history, based on need and risk.

Decide:

- whether to migrate full transaction history or only open balances;
- the cutoff date and the as-of point for migrated balances;
- whether parallel running in both systems is needed for a transition period;
- the treatment of closed periods, archived data, and historical reporting;
- the scope of master data migration versus recreation.

Migrating excessive history increases cost and risk. Migrating too little leaves the business unable to answer historical questions. Balance scope against need.

### Map The Old Structure To The New Structure Explicitly

Every account, customer, vendor, item, and dimension in the old system must map to a target in the new system. Implicit or ad hoc mapping produces gaps and misclassifications.

Build explicit mapping for:

- the old chart of accounts to the new chart of accounts;
- old customers, vendors, and items to new master records;
- old dimensions and segments to new reporting structures;
- old tax codes and payment terms to new equivalents;
- old currency and exchange rate handling to the new approach.

Document the mapping, have it reviewed, and reconcile the mapped totals to the source. A mapping that is not reviewed will contain errors that propagate into every migrated balance.

### Cleanse And Reconcile Before Migration

Migrating dirty data into a new system guarantees the new system inherits all the old problems. Cleanse and reconcile before migration, not after.

Before migration:

- reconcile all material accounts in the old system;
- clear or document aged reconciling items;
- resolve duplicate or obsolete master records;
- write off or document truly uncollectible receivables and payables;
- confirm fixed asset, inventory, and accrual subledgers tie to the GL;
- address known data quality issues.

A migration is an opportunity to start clean, but only if the cleansing is done before cutover. Migrating unreconciled balances transfers the problem and makes it harder to solve.

### Validate Migrated Data Against The Source

Migration is not complete until the migrated data is proven to reconcile to the source. Validation is the most important step and the one most often skipped.

Validate:

- trial balance totals in the new system tie to the old system at cutoff;
- each material account balance reconciles individually;
- open receivable and payable detail ties to aging and to control accounts;
- fixed asset and inventory subledgers tie to the GL;
- master data counts and key fields reconcile between systems;
- a sample of transactions, where history is migrated, traces correctly.

Document the validation with evidence. A migration without documented reconciliation cannot be trusted and will not satisfy an auditor.

### Handle Cutoff And Parallel Running Carefully

The cutover from old to new is the highest-risk moment. Transactions in flight at cutover must be handled deliberately.

Plan:

- the freeze period in the old system and the go-live in the new;
- the handling of transactions straddling the cutover date;
- the treatment of open sales and purchase orders;
- bank and cash positions at cutover;
- whether parallel running is needed to validate the first period.

A cutover without a clear freeze and handoff produces duplicate or missing transactions. Define the boundary and reconcile across it.

### Test The Migration End To End Before Cutover

A migration should be rehearsed in a test environment before the live cutover. First attempts almost always reveal mapping and data issues.

Test:

- run the full migration in a test environment;
- validate the test migration against the source;
- identify and resolve mapping and data errors;
- rehearse the cutover sequence and timing;
- confirm reporting and close work in the new system with migrated data.

Testing reveals problems when they are cheap to fix. Discovering them at live cutover is expensive and disruptive.

### Preserve The Audit Trail Across Systems

Migration must not destroy the audit trail. Transactions and balances in the new system should be traceable to their origin in the old system.

Preserve:

- references from migrated records to their source, such as old document numbers;
- the mapping documentation that explains how old structure became new;
- the validation evidence that proves reconciliation;
- access to the old system or archived data for historical inquiry;
- the cutoff and cutover documentation.

A new system whose balances cannot be traced to the old system creates an audit gap that persists for years.

### Acknowledge Framework And Professional Limits

Data migration must preserve framework-compliant accounting, and the new system's configuration must produce results consistent with the applicable reporting framework, tax, and regulatory requirements. Migration of balances involving revenue, leases, hedging, intercompany, or tax often requires professional accounting review to ensure the new system's treatment matches the old. Involve qualified accounting, tax, and IT professionals in the migration design and validation. Do not treat migration as a purely technical exercise; it is an accounting continuity decision.

## Common Traps

### Migrating Without Reconciliation

Moving balances without proving they tie to the source produces a new system that cannot be trusted.

### Implicit Or Unreviewed Mapping

Ad hoc account and master mapping contains errors that propagate into every migrated balance.

### Migrating Dirty Data

Carrying over unreconciled balances and duplicate masters transfers old problems into the new system.

### Skipping Validation

A migration without documented reconciliation cannot satisfy an auditor or the finance team.

### Poor Cutoff Handling

A cutover without a clear freeze and handoff produces duplicate or missing transactions.

### No Test Migration

Discovering mapping and data errors at live cutover is expensive and disruptive.

### Lost Audit Trail

A new system whose balances cannot be traced to the old creates a persistent audit gap.

### Treating Migration As Technical Only

Migration is an accounting continuity decision requiring professional review of framework, tax, and regulatory consistency.

## Self-Check

- Is the migration scope, history versus open balances, cutoff, and parallel running planned deliberately?
- Is there explicit, reviewed mapping for accounts, master data, dimensions, tax codes, and payment terms?
- Are source accounts reconciled, aged items cleared, duplicates resolved, and subledgers tied to the GL before migration?
- Is migrated data validated against the source, with trial balance, account, subledger, and master reconciliation documented?
- Is the cutover planned with a clear freeze, handoff, straddling transaction handling, and optional parallel running?
- Has the migration been tested end to end in a non-production environment with errors resolved before live cutover?
- Is the audit trail preserved across systems, with source references, mapping documentation, validation evidence, and old system access?
- Does the migration involve qualified accounting, tax, and IT professionals to confirm framework, tax, and regulatory continuity?
