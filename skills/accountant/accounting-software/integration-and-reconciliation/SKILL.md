---
name: integration_and_reconciliation.md
description: Use when the agent is integrating accounting software with other systems, reconciling data flowing between integrated systems, handling integration exceptions and synchronization failures, or assessing whether integrated systems produce consistent and reliable financial data.
---

# Integration And Reconciliation

Modern accounting does not happen in one system. The general ledger receives data from payroll, banking, expense management, CRM, e-commerce, point of sale, and operational systems. Each integration is a point where data moves, and each point of movement is a point where data can be lost, duplicated, transformed incorrectly, or delayed. Integration without reconciliation produces a ledger that looks current but contains silent gaps and errors. The discipline is to ensure that what left the source system arrived in the GL completely, correctly, and in the right period.

Use this skill before integrating systems, designing reconciliation between integrated systems, handling integration exceptions, or assessing whether integrated data is reliable. The goal is to prevent the agent from building integrations that move data without verifying completeness, or that produce a ledger whose balances cannot be tied back to their source systems.

## Core Rules

### Design Integration Around A Reconciliation Point

Every integration should have a defined point where the data received is reconciled to the data sent. Without this, errors accumulate silently.

For each integration define:

- the source system and the data being sent;
- the receiving system and how the data is posted;
- the reconciliation point, daily, per period, or per batch;
- the expected matching criteria, such as count, amount, or transaction ID;
- the owner responsible for investigating exceptions.

An integration without a reconciliation point is an open loop. Open loops fail quietly until a material discrepancy surfaces at close.

### Reconcile Volume, Amount, And Identity

A complete reconciliation checks more than totals. It checks that the right transactions arrived in the right form.

Reconcile:

- the count of transactions sent versus received;
- the total amount sent versus received;
- the identity of transactions, through IDs or hashes, to detect duplicates and drops;
- the posting date and period to detect timing shifts;
- the account or dimension mapping to detect misclassification.

A reconciliation that checks only totals can hide a dropped transaction offset by a duplicated one. Check volume and identity, not just amount.

### Handle Exceptions And Failed Transfers Explicitly

Integrations fail. Records are rejected, mappings break, and source data is malformed. How exceptions are handled determines whether the integration is reliable.

Handle exceptions by:

- logging every rejected or failed transfer with a reason;
- routing exceptions to a defined owner for resolution;
- preventing silent drops where failed records disappear without trace;
- reprocessing corrected records without creating duplicates;
- aging and escalating unresolved exceptions.

An integration that silently drops failed records produces a ledger that is incomplete in ways no one can see. Make failures visible and actionable.

### Control For Duplicate And Missing Transactions

Duplicates and missing transactions are the two most common integration failures. Both distort the ledger.

Control by:

- using unique transaction identifiers to detect duplicates;
- implementing idempotent processing so re-running a batch does not double-post;
- reconciling counts to detect missing transactions;
- monitoring for gaps in sequence or timestamp;
- investigating any reconciliation break promptly.

A reconciliation that nets to zero despite a missing and a duplicate transaction is a false comfort. Identity-level reconciliation catches what amount-level reconciliation misses.

### Manage Timing And Period Cutoff

Integrated data must land in the correct period. Timing differences between source and GL create cutoff errors.

Manage timing by:

- defining the cutoff rule for each integration, such as posting date or transaction date;
- handling time zone differences between systems;
- managing the period-end freeze so late-arriving data posts to the correct period;
- reconciling in-transit items at period end;
- documenting the expected lag between source and GL posting.

A payroll or sales integration that posts a day late can shift material amounts between periods. Define and test the cutoff.

### Monitor Integration Health Continuously

Integration health is not a close activity; it is a daily control. Waiting until close to discover integration failures is too late.

Monitor:

- daily reconciliation status for each integration;
- exception counts and aging;
- synchronization lag between systems;
- error logs and failed transfers;
- changes in source system format or volume.

Automate alerts for reconciliation breaks or exception thresholds. A healthy integration is one whose status is visible every day, not only at month-end.

### Document The Integration Architecture And Data Flow

Integrated environments become unintelligible without documentation. Finance, IT, and auditors need to understand how data moves.

Document:

- the source and destination of each integration;
- the data fields and transformations applied;
- the reconciliation point and criteria;
- the exception handling and owner;
- the cutoff and timing rules;
- the effect on the audit trail.

An undocumented integration is understood only by the person who built it. When that person leaves, the integration becomes a black box.

### Acknowledge Framework And Professional Limits

Integration design must preserve framework-compliant accounting and a complete audit trail. Integrations involving revenue, tax, payroll, intercompany, or regulated data often carry framework, tax, or regulatory requirements that must be reflected in the integration logic. Confirm significant integration designs with qualified accounting, tax, and IT professionals, and validate that integrated posting produces framework-compliant results with a traceable audit trail. Do not treat integration as a purely technical concern; it is an accounting data integrity decision.

## Common Traps

### Integration Without A Reconciliation Point

An open-loop integration fails quietly until a material discrepancy surfaces at close.

### Amount-Only Reconciliation

Checking only totals hides dropped and duplicated transactions that net to zero.

### Silent Exception Drops

Failed records that disappear without logging or resolution produce an incomplete ledger.

### Duplicate Or Missing Transactions

Lack of unique identifiers or idempotent processing creates duplicates; lack of count reconciliation hides missing items.

### Poor Period Cutoff

Integrations that post late shift material amounts between periods.

### Close-Only Monitoring

Waiting until close to discover integration failures is too late to resolve them cleanly.

### Undocumented Architecture

An undocumented integration becomes a black box when its builder leaves.

### Treating Integration As Technical Only

Integration is an accounting data integrity decision requiring professional review of framework, tax, and regulatory consistency.

## Self-Check

- Does each integration have a defined reconciliation point with matching criteria and an owner?
- Does reconciliation check volume, amount, and transaction identity, not just totals?
- Are exceptions logged, routed, aged, and resolved without silent drops?
- Are duplicates and missing transactions controlled through unique identifiers, idempotent processing, and count reconciliation?
- Is period cutoff defined and tested, with in-transit items reconciled at period end?
- Is integration health monitored daily with alerts for breaks and exception thresholds?
- Is the integration architecture documented, including sources, transformations, reconciliation, exceptions, cutoff, and audit trail?
- Does the design involve qualified accounting, tax, and IT professionals to confirm framework, tax, and regulatory compliance and a traceable audit trail?
