---
name: etl_in_audit.md
description: Use when the agent is extracting, transforming, or loading data for audit analysis, pulling extracts from ERP or GL systems, mapping source fields to audit fields, cleaning and converting data types, dates, and currencies, reconciling transformed datasets to control totals, or building audit-ready datasets that must withstand inspection.
---

# ETL In Audit

Extract-transform-load (ETL) work in an audit is not a neutral data plumbing task. Every transformation step is a place where records can be silently dropped, amounts re-signed, dates shifted across periods, currencies mis-converted, or fields truncated, and each of these errors can change the conclusion of a substantive test or an analytic. Agents tend to treat the extract as a finished artifact once the row count looks plausible, but a row count match does not prove that the right records survived the right transformations with the right values. The harm this skill prevents is building an entire audit procedure on a dataset that looks complete but is internally wrong, which forces rework, undermines the workpaper, and can lead to an unsupported conclusion.

Use this skill before and during any data pull from an ERP, GL, subledger, or source system, and whenever data is reshaped into the schema used for audit analytics. The goal is an extract that is traceable, reconciled, reproducible, and defensible to a reviewer or inspector.

## Core Rules

### Know The Source System And Its Quirks Before Extracting

An extract is only as trustworthy as the agent's understanding of the system it came from. ERPs store the same business event in multiple tables, use different date semantics, and expose reporting views that quietly exclude voided, held, or future-dated items.

Before pulling data, confirm:

- which module, table, or report is the authoritative source;
- whether the report is a live view, a snapshot, or a batch-built cube;
- what filters or date parameters the report applies by default;
- whether amounts are stored signed, absolute, or with separate debit/credit columns;
- how the system represents voids, reversals, holds, and draft documents;
- whether multi-currency amounts are in transaction, functional, or reporting currency;
- the timezone and fiscal-calendar conventions of date fields;
- whether the export tool truncates long text fields or drops leading zeros.

### Build An Explicit Field Mapping And Keep It As Evidence

Implicit mapping is where most silent corruption happens. A field named "amount" in two systems may mean net, gross, base, or signed value. Document the mapping from source column to audit column and retain it as part of the workpaper.

For every mapped field record:

- source table and column name;
- source data type and example value;
- target audit field;
- transformation applied (none, cast, sign flip, multiply, lookup);
- handling of nulls, blanks, and zeros;
- known exceptions or unmappable values;
- who validated the mapping and when.

### Preserve Source Traceability End To End

Every row in the transformed dataset must point back to an immutable source record. Without traceability, an exception found later cannot be tied to the original transaction, and an inspector cannot re-perform the procedure.

Maintain traceability by:

- retaining a unique source key (document number, journal id, line id) on every row;
- keeping the raw extract unmodified alongside the transformed version;
- versioning the transformation script or query with a date and author;
- logging the extraction parameters (period, entity, filters, run time);
- storing the source system version or patch level at extract time;
- recording the row count and hash of the raw file.

### Reconcile To Control Totals At Every Stage

Reconciliation is not a final step; it is a gate between stages. Transform and reconcile, then transform again. A mismatch caught early is cheap; a mismatch found after analysis is expensive and casts doubt on all downstream work.

Reconcile at minimum:

- raw extract count and total to a system report or GL control account;
- post-cleaning count and total to the raw extract, with explained deltas;
- post-currency-conversion total to the source-currency total at the stated rate;
- final dataset count and total to the trial balance or subledger;
- entity and location subtotals to the consolidated total;
- period boundaries to the GL posting date cutoff.

### Handle Data Types, Dates, And Currency Deliberately

Type and format conversions are where numeric values are silently destroyed. A date stored as text in one locale, an amount read as text and summed to zero, or a currency converted at the wrong rate can all pass a row-count check and fail an audit.

Apply explicit rules:

- cast dates using an unambiguous format (ISO 8601) and validate the range;
- store amounts as fixed-decimal, never as float;
- separate sign from magnitude or standardize on a documented convention;
- convert currency at the rate appropriate to the assertion (transaction date, period-end, average);
- record the rate source and date for every converted amount;
- validate that no amount exceeds a sanity ceiling without explanation;
- flag rows where the currency code is missing or unrecognized.

### Clean And Transform Without Destroying Evidence

Cleaning improves usability but can erase the very exceptions the audit needs to find. Removing "duplicates" that are actually separate postings, or filtering "test" vendors that are real, destroys evidence and biases the result.

Before removing or altering any row:

- define what counts as a duplicate and check it against business rules;
- keep removed rows in a separate exception file, not deleted;
- justify each filter (e.g., why a status code excludes a record);
- never overwrite the original value; add a cleaned column instead;
- record the count and value of rows removed at each cleaning step;
- have a second person review any transformation that removes records.

### Document Reproducibility

An extract that cannot be reproduced is not audit evidence. The procedure must be re-runnable by another auditor with the same inputs and the same output.

Ensure reproducibility by:

- storing the exact query, script, or report parameters used;
- pinning software and library versions that affect parsing;
- recording the seed for any randomized transformation;
- keeping input and output files together with a manifest;
- noting any manual steps and who performed them;
- dating and signing the final extract.

### Manage Access, Sensitivity, And Freshness Of The Extract

Audit extracts often contain payroll, vendor banking, customer PII, or full GL detail, and a stale extract can quietly exclude late adjustments posted after the pull. Treat the extract as both a confidentiality risk and a point-in-time artifact.

Controls to apply:

- restrict extract files to the engagement team on a need-to-know basis;
- avoid emailing raw extracts; use the engagement's secure repository;
- record the extract timestamp and the GL "as-at" posting date;
- re-extract or roll forward if material late entries are discovered;
- purge intermediate files that are no longer needed once the final dataset is locked;
- confirm the extract reflects the correct adjusted or unadjusted trial balance basis.

## Common Traps

### Trusting The Row Count As Proof Of Completeness

A matching row count feels like strong evidence but only proves that the same number of rows arrived. It says nothing about whether the rows are the right rows, whether amounts survived transformation, or whether voided or held items were included or excluded consistently. Always reconcile value totals and key subtotals, not just counts.

### Letting The Export Tool Decide Formats

Spreadsheet and report exporters auto-detect types and often misread a numeric account code as a date, a leading-zero identifier as a number, or a large amount in scientific notation. These corruptions are invisible in a quick glance. Force explicit types on import and validate against the source.

### Converting Currency At Today's Rate For A Historical Population

Using the current exchange rate on a prior-period population produces totals that cannot reconcile to any ledger. Match the rate to the assertion: transaction-date rate for transaction testing, period-end rate for balance testing, weighted-average for income statement analytics.

### Dropping Rows Silently During Join Or Filter

An inner join or a filter on a status field can quietly remove records that have no match or an unexpected status, and the agent never sees them. Always compare pre-join and post-join counts and investigate every dropped row, especially high-value ones.

### Treating Null And Zero As Equivalent

A null amount, a zero amount, and a blank field mean different things in audit terms: unknown, none, and not captured. Collapsing them hides exceptions such as unposted journals or missing prices. Preserve the distinction and report each category separately.

### Overwriting Source Values When Cleaning

Replacing a messy source value with a clean one destroys the evidence of the original condition and makes the cleaning unauditable. Always keep the original column and write the cleaned value to a new column with a documented rule.

### Assuming One Extract Covers All Entities And Periods

A single report often defaults to one company code, one ledger, or one fiscal year. Pulling "all data" and getting only the default entity is a frequent and serious error. Confirm the entity, ledger, and period scope explicitly against the consolidation list.

### Skipping Reconciliation Because The Numbers Look Right

When a total is close to expected, agents often skip the formal reconciliation. Small unexplained differences are exactly where cutoff errors, missing locations, or double-counted intercompany live. Investigate every difference, even immaterial-looking ones, before relying on the data.

### Relying On An Extract Without Confirming Its As-At Basis

An extract pulled before the adjustments process looks different from one pulled after, and an unadjusted extract used to test an adjusted balance will never reconcile. Confirm whether the population is on an adjusted, unadjusted, or trial-balance-as-reported basis, and match it to the assertion.

### Treating Transformation As A One-Way, Unlogged Process

When data flows through several transformations with no log of intermediate states, an error discovered late cannot be traced to the step that caused it. Keep a log of each transformation stage with its input and output counts and totals, so any divergence can be localized and corrected without rebuilding the entire pipeline.

## Self-Check

- Is the authoritative source table or report identified and documented for every field?
- Does every row in the final dataset retain a unique key back to the source record?
- Has the dataset been reconciled by both count and value to a control total at each stage?
- Are currency conversions performed at rates appropriate to the specific assertion being tested?
- Are amounts stored as fixed-decimal, with sign convention documented and applied consistently?
- Are removed, filtered, or deduplicated rows preserved in a separate exception file with justification?
- Does the workpaper contain the exact query, parameters, versions, and run time needed to reproduce the extract?
- Have date fields been validated for range, format, and period-cutoff correctness?
- Are null, zero, and blank values distinguished and reported separately rather than collapsed?
- Has the entity, ledger, and period scope been confirmed against the full consolidation, not a system default?
- Has a second person reviewed any transformation that removes or alters records?
- Is there a log of each transformation stage with input and output counts and totals for traceability?
- Are access to and freshness of the extract controlled, with the extract timestamp and GL as-at date recorded?
- Are currency conversions reconciled back to the source-currency total at the stated rate and date?
- Has the raw, unmodified extract been retained alongside the transformed version for traceability?
