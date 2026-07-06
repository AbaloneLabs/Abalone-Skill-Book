---
name: data_validation_and_error_handling.md
description: Use when the agent is designing data validation rules for accounting data feeds, building error handling for failed transactions, configuring edit checks and field validations in an ERP, or evaluating the integrity of data imported from subledgers and external systems into the general ledger.
---

# Data Validation And Error Handling

Accounting data flows from many sources into the general ledger: subledgers, bank feeds, payroll systems, expense platforms, consolidation engines, and external integrations. Each feed is a potential entry point for errors, duplicates, missing values, and corrupted records. Data validation is the set of controls that prevents bad data from entering the ledger; error handling is the set of processes that manages the records that fail validation. Together they determine whether the general ledger reflects reality or a degraded approximation of it. A ledger fed by unvalidated data will contain errors that are expensive to find and correct, because once a wrong value is posted and flows into financial statements, the error propagates through every downstream report, reconciliation, and decision.

Use this skill before designing data validation, building error handling for failed transactions, configuring edit checks, or evaluating data feed integrity. The goal is to prevent the agent from building validation that is too permissive to catch real errors, error handling that silently drops failed records, or feeds that post without any independent integrity check.

## Core Rules

### Validate At The Boundary, Before Data Enters The Ledger

The most effective validation happens before data is posted, not after. Catch errors at the point of entry or import.

Validate at the boundary by:

- checking every imported record against validation rules before it is accepted;
- rejecting records that fail validation to a hold or error queue rather than posting them;
- running pre-posting edit checks on amount, account, date, and required fields;
- confirming that reference data, such as vendor and account, exists and is active;
- verifying totals and record counts against a control total from the source system.

Posting first and reconciling later is a weaker control. Validation at the boundary prevents the error from ever reaching the ledger.

### Define Validation Rules For Each Critical Field And Relationship

Validation must cover both field-level integrity and relational integrity. Define rules for each.

Field-level rules include:

- amount is numeric, non-negative where expected, and within a plausible range;
- date is valid, within the open period, and not in the future or distant past;
- account code exists in the chart of accounts and accepts postings;
- currency code is valid and matches the entity's reporting currency or a supported alternative;
- required dimensions, such as cost center, are present and valid.

Relational rules include:

- the debit and credit sides of an entry balance;
- the document type is consistent with the accounts used;
- the vendor or customer master record is active and not flagged;
- the tax code is valid for the jurisdiction and account;
- the intercompany pairing is valid and balanced across entities.

A feed validated only for field presence, not for relational correctness, will accept records that are individually plausible but collectively wrong.

### Use Control Totals To Confirm Completeness And Accuracy

Field-level validation confirms each record is well-formed; control totals confirm the feed as a whole is complete and accurate.

Use control totals by:

- comparing the record count and the sum of amounts from the source to the imported total;
- reconciling the hash or record count to detect missing or duplicate records;
- investigating any variance before posting, no matter how small;
- requiring sign-off on the control total reconciliation for high-risk feeds.

A feed that passes field validation but drops ten records due to a transmission error will post an incomplete ledger unless control totals catch the gap.

### Route Failed Records To A Managed Error Queue

Records that fail validation must not be silently discarded or posted anyway. They must be routed to a managed queue.

Design the error queue to:

- preserve the original record exactly as received, with the validation error identified;
- assign each error to an owner responsible for correction;
- age errors and escalate old ones that remain unresolved;
- prevent bulk clearing without correction of the underlying problem;
- track the resolution, whether correction and reprocessing or rejection and documentation.

A feed that drops failed records, or that posts them with default values substituted for invalid ones, corrupts the ledger silently. The error queue makes failure visible and manageable.

### Prevent Default Substitution That Masks Real Errors

Some systems substitute default values for invalid or missing data, such as a default account or a default cost center. This masks errors rather than fixing them.

Prevent harmful substitution by:

- disabling silent default substitution for critical fields such as account and amount;
- requiring explicit values for required fields, with no fallback default;
- flagging any record that relied on a default for review;
- reviewing the volume of default-dependent records as a data quality indicator.

A feed that posts thousands of records to a default suspense account because the real account was missing has not processed the data; it has hidden it.

### Handle Duplicates, Gaps, And Sequence Integrity

Feeds must be checked not only record by record but for sequence integrity. Duplicates and gaps indicate transmission or processing failures.

Check for:

- duplicate records, by reference number, hash, or a unique composite key;
- gaps in a sequence that should be complete, such as check numbers or invoice numbers;
- out-of-sequence records that may indicate retransmission or replay;
- duplicate posting of the same source document across feeds or periods.

A duplicate payment or a duplicate revenue entry often originates from a feed that was processed twice. Sequence and duplicate checks are the controls that catch this.

### Monitor Data Quality Trends Across Feeds

Data quality is not static. Monitor trends to detect degrading feeds before they corrupt the ledger.

Monitor:

- the error rate by feed and by rule;
- the volume of records relying on defaults or manual correction;
- the aging of unresolved errors;
- the recurrence of the same error type, which signals a source-system problem;
- changes in feed volume or structure that may indicate a source change.

A feed whose error rate is rising is a feed heading toward failure. Investigating the trend fixes the source; ignoring it guarantees a future ledger cleanup.

### Document The Validation Logic And Maintain It As Feeds Change

Validation rules must be documented and maintained. Feeds change, source systems are upgraded, and business rules evolve.

Maintain documentation that includes:

- each validation rule, its purpose, and its tolerance;
- the source of the rule, whether regulatory, policy, or operational;
- the version history and the effective dates of changes;
- the owner responsible for maintaining the rule;
- the testing evidence confirming a changed rule still catches the intended errors.

An undocumented validation rule that no one maintains will eventually stop catching the errors it was designed for, and no one will notice until audit.

### Acknowledge Framework And Professional Limits

Data validation and error handling protect the integrity of data reported under the applicable framework, but validation rules cannot by themselves establish that transactions are recorded in accordance with recognition, measurement, and disclosure requirements. Feeds involving revenue, leases, financial instruments, intercompany, and tax often require validation that reflects framework-specific logic. Confirm significant validation design decisions with qualified accounting professionals, and validate that validation supports framework-compliant reporting. Do not treat data validation as a purely technical control; it is a control over financial data integrity.

## Common Traps

### Post First, Reconcile Later

Posting unvalidated data and relying on later reconciliation is weaker than boundary validation.

### Field Validation Without Relational Checks

Records that are individually plausible but collectively wrong will pass field-only validation.

### No Control Totals

A feed that drops or duplicates records will post an incomplete or inflated ledger unless control totals catch it.

### Silent Discard Of Failed Records

Dropping or posting failed records with defaults corrupts the ledger without any visible signal.

### Default Substitution Masking Errors

Posting to default accounts hides missing data and creates a suspense balance that grows unnoticed.

### No Duplicate Or Sequence Checking

Duplicate payments and duplicate entries often originate from feeds processed twice without sequence checks.

### No Data Quality Monitoring

A degrading feed corrupts the ledger gradually until the error becomes material and visible.

### Undocumented And Unmaintained Rules and validation Treated As Purely Technical

Validation rules that no one maintains eventually stop catching the errors they were designed for.

Validation of revenue, lease, intercompany, and tax feeds requires framework-specific logic confirmed by professionals.

## Self-Check

- Is data validated at the boundary before posting, with failed records rejected to a hold rather than posted?
- Are validation rules defined for each critical field and for relational integrity, including balancing, document type, master record, and tax?
- Are control totals for record count and amount reconciled to the source before high-risk feeds are posted?
- Are failed records routed to a managed error queue with preservation, ownership, aging, escalation, and tracked resolution?
- Is silent default substitution prevented for critical fields, with any default reliance flagged and reviewed?
- Are duplicates, gaps, and sequence integrity checked to catch transmission and reprocessing failures?
- Are data quality trends, error rates, default reliance, aging, and recurrence monitored across feeds?
- Is validation logic documented, versioned, owned, and tested as feeds and source systems change?
- Does validation design reflect framework-compliant logic for revenue, leases, intercompany, and tax confirmed with qualified professionals?
