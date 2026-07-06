---
name: data_validation.md
description: Use when the agent is validating the completeness and accuracy of audit data extracts, reconciling record counts and monetary totals to control figures, performing field-level validation, detecting truncation and encoding corruption, handling nulls and blanks, logging exceptions, or signing off on data integrity before relying on audit analytics.
---

# Data Validation

Data validation is the gate between an extract and the audit procedures that depend on it. An analytic run on unvalidated data produces confident-looking results that may be wrong in ways the auditor never checks: truncated identifiers, re-encoded characters that split one vendor into two, sign errors that turn debits into credits, or nulls silently dropped before aggregation. Agents often validate superficially, confirming that the file opened and the row count is plausible, then proceed as if the data were trustworthy. The harm this prevents is building substantive conclusions, sampling frames, and risk ratings on a dataset whose integrity was never actually established, which forces rework and can produce a conclusion that cannot be defended under review or inspection.

Use this skill immediately after receiving or transforming any audit data extract and before any analytic, sample, or reconciliation relies on it. The goal is a documented, exception-tracked, signed-off statement that the data is complete and accurate enough to support the intended procedures.

## Core Rules

### Validate Completeness And Accuracy As Separate Properties

Completeness means every record that should be present is present. Accuracy means the records that are present hold the correct values. A dataset can be complete but inaccurate, or accurate but incomplete, and both must be tested independently.

Test completeness by reconciling:

- total record count to the source report or control total;
- monetary total to the GL or subledger control account;
- count and total by entity, location, and currency;
- count and total by period, week, or posting date bucket;
- first and last document or sequence numbers to expected ranges;
- presence of all expected entity and ledger codes.

Test accuracy by checking:

- field-level value ranges and types;
- sign and magnitude of amounts;
- date validity, range, and period assignment;
- referential integrity to master data;
- duplicate detection on keys that should be unique;
- cross-field consistency (e.g., net plus tax equals gross).

### Reconcile Counts And Totals, And Explain Every Difference

A reconciliation that ties exactly is reassuring; a reconciliation with a small unexplained difference is a warning. Small differences are where cutoff errors, missing entities, duplicate intercompany, and excluded status codes hide. Never accept "it's close enough" without a documented cause.

For each reconciliation capture:

- the source figure and where it came from;
- the extract figure and how it was computed;
- the difference, signed and in value and count;
- the explanation for the difference;
- whether the difference is acceptable for the intended procedure;
- who reviewed and accepted the explanation.

### Perform Field-Level Validation Against Business Rules

Row-level data can look fine in aggregate while being wrong at the field level. Validate each critical field against the rules it must obey, not just against whether it is populated.

Typical field-level checks:

- amount is numeric, non-null, and within a sane ceiling;
- date is a valid calendar date within the engagement period;
- currency code exists in the currency master;
- account number exists in the chart of accounts;
- document number matches an expected format and length;
- sign of the amount matches the document type;
- tax and discount fields reconcile to the line amount;
- no identifier has lost leading zeros or been truncated.

### Detect Truncation, Encoding, And Format Corruption

Export and import steps corrupt data in predictable ways, and the corruption is usually invisible in a quick preview. A vendor name cut at 30 characters, a euro sign turned into mojibake, or an account code read as a date all change joins and groupings.

Screen for corruption by:

- checking max field length against the source schema;
- scanning for replacement characters and encoding artifacts;
- verifying numeric fields contain no text or whitespace;
- confirming date fields parse under the declared locale;
- checking for identifiers stored in scientific notation;
- comparing distinct counts of key fields to the source;
- sampling rows and eyeballing them against the source report.

### Handle Nulls, Blanks, And Zeros Explicitly

These three states mean different things and collapsing them hides exceptions. A null amount may mean an unposted journal; a blank vendor may mean a master-data gap; a zero may be a genuine zero-value posting. Each must be counted, classified, and decided on.

For each critical field report:

- count of null values;
- count of blank or empty-string values;
- count of zero values;
- whether each category is expected or an exception;
- the action taken (keep, exclude with reason, or correct with evidence).

### Log Every Exception And Track It To Resolution

Validation is only useful if its findings are acted on. An exception that is noticed but not logged is lost; an exception that is logged but not resolved becomes a silent assumption in the results.

For each exception record:

- a unique exception id;
- the field and record affected;
- the rule that was violated;
- the observed and expected values;
- the impact on completeness or accuracy;
- the resolution and who approved it;
- whether the row was kept, corrected, or excluded.

### Sign Off On Data Integrity Before Analysis Begins

Data integrity should be a formal gate, not an afterthought. No analytic, sample, or conclusion should be built on data that has not been validated and signed off. The sign-off records who accepted the data and on what basis.

The sign-off should state:

- the dataset name, version, and extract timestamp;
- the reconciliations performed and their results;
- the exceptions found and their resolutions;
- any residual limitations or caveats;
- the procedures the data is considered reliable enough to support;
- the preparer and reviewer signatures and dates.

### Re-Validate After Any Refresh Or Rollover

Extracts get refreshed when late adjustments arrive or when the period rolls forward. A validation done once does not carry over to a new version of the file. Treat each refresh as a new dataset and re-run the full validation suite.

Re-validate on:

- receipt of a refreshed or re-extracted file;
- rollover to a new period or new fiscal year;
- change of source system version or patch;
- addition of a new entity, location, or currency to scope;
- any correction applied to the dataset itself.

## Common Traps

### Treating A Successful File Open As Validation

The fact that a file opened without error proves only that it is readable, not that its contents are correct or complete. Many corrupt extracts open cleanly because the corruption is in the values, not the file structure. Always run field-level and reconciliation checks beyond opening the file.

### Reconciling Only The Grand Total

A grand total can tie even when an entity is missing and another is double-counted, because the errors offset. Reconcile at the entity, currency, and period level so that offsetting errors are exposed rather than hidden inside a single matching number.

### Accepting Small Differences As Immaterial

Small unreconciled differences are often the only visible symptom of a real problem such as a cutoff shift or an excluded status code. Investigate the cause of every difference before deciding it is immaterial; do not assume immateriality to avoid the work.

### Collapsing Null, Blank, And Zero Into One Bucket

Treating all three as "empty" hides the distinctions that matter for audit. An unposted journal with a null amount, a missing price shown as blank, and a genuine zero-value posting are different facts and must be reported separately.

### Validating Only The Fields Used In The Current Test

Agents often validate only the columns the immediate analytic needs, leaving the rest untested. When the analytic later expands, the unvalidated fields cause silent failures. Validate all critical fields up front, even those not used in the first procedure.

### Failing To Detect Truncation Because The Preview Looks Fine

A spreadsheet preview shows the first rows, which are often short enough to escape truncation. Truncation usually appears only in longer values deeper in the file. Check maximum field lengths and distinct counts against the source, not just a preview.

### Logging Exceptions But Never Resolving Them

An exception log that accumulates open items is evidence that validation was performed but not completed. Every exception must be resolved, and the resolution must be reflected in the dataset and the sign-off, not left as an open note.

### Signing Off Based On Trust Rather Than Evidence and validating Once And Assuming It Stays Valid

A sign-off that says "data reviewed and acceptable" without the underlying reconciliations and exception log is not a validation; it is an assertion. The sign-off must reference the specific evidence that supports it.

Data does not stay valid just because it was valid once. A colleague may re-sort, a refresh may overwrite, or a filter may be reapplied, each of which can reintroduce the very errors the first validation caught. Treat the validated file as a controlled artifact and re-validate after any change.

## Self-Check

- Have completeness and accuracy each been tested independently, not just one or the other?
- Does every reconciliation capture source figure, extract figure, difference, explanation, and reviewer?
- Are reconciliations performed at entity, currency, and period level, not only at the grand total?
- Has every critical field been validated against its business rules, not merely checked for population?
- Have truncation, encoding, and format corruption been screened using lengths and distinct counts?
- Are null, blank, and zero values counted and classified separately for each critical field?
- Is there an exception log with a unique id, rule violated, resolution, and approver for every finding?
- Does the sign-off state which procedures the data is reliable enough to support, with caveats?
- Has the full validation suite been re-run after any refresh, rollover, or correction to the dataset?; are the validation evidence and sign-off retained as part of the workpaper, not just held in memory?
- Would an independent auditor be able to reproduce the validation from the documentation alone?; is the validated file treated as a controlled artifact, with re-validation triggered after any refresh or edit?
