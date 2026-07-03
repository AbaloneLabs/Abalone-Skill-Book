---
name: data-integrity-audit.md
description: Use when the agent is auditing the completeness, accuracy, validity, and consistency of data in information systems, testing data quality controls, evaluating data lineage and reconciliation, assessing master data management, auditing data migration and interface integrity, or determining whether data relied upon for decisions, reporting, and automated controls is trustworthy.
---

# Data Integrity Audit

A data integrity audit determines whether data is complete, accurate, valid, and consistent throughout its lifecycle, so that it can be trusted for reporting, decisions, and automated processing. The central judgment problem is that data integrity is not a single property but a set of dimensions that can hold or fail independently, and integrity established at one point degrades unless controls maintain it. Agents frequently test a single dimension, accept record counts as proof of completeness, or audit the database while ignoring the interfaces and manual processes that corrupt data before and after it is stored.

## Core Rules

### Define the integrity dimensions relevant to the engagement

Data integrity is multi-dimensional. Establish which dimensions are in scope and define each operationally:

- **completeness** — all transactions or records that should exist are present, none omitted;
- **accuracy** — data values correctly represent the underlying facts;
- **validity** — data conforms to defined formats, ranges, and business rules;
- **consistency** — the same data agrees across systems, fields, and over time;
- **timeliness** — data is current and available when needed;
- **uniqueness** — no unintended duplication of records.

A conclusion that "data is accurate" does not imply completeness or consistency. Test and report each relevant dimension separately.

### Establish the authoritative source and data lineage

Before testing integrity, identify the system of record (the authoritative source) for each data domain and trace lineage: where data originates, how it transforms as it moves, where it is stored, and which downstream systems consume it. Integrity testing is meaningless without knowing which copy is authoritative and how the others should relate to it. Where multiple systems hold "the same" data with no designated master, inconsistency is guaranteed and the first finding is governance, not data.

### Test completeness independently of the system under audit

Completeness is the hardest dimension because the missing records are, by definition, absent. Do not rely solely on the audited system's own record count. Corroborate completeness through:

- reconciliation to an independent source (e.g., source documents, a feeder system, a third-party record);
- sequence checking for gaps in pre-numbered or sequential records;
- cut-off testing to confirm records fall in the correct period;
- comparison to expected volumes based on activity drivers.

A system that reports its own completeness is asserting, not demonstrating.

### Test accuracy against source, not against internal consistency

Accuracy means the data matches the underlying real-world fact, typically captured at the source. Testing that two fields in the same system agree tests internal consistency, not accuracy. To test accuracy, compare system data to the original source document, the originating transaction, or a third-party confirmation. Reconciliation between two derived fields can mask a shared upstream error.

### Validate data against business rules and constraints

Validity testing checks conformance to defined rules: format (date, email, code structure), range (values within plausible bounds), referential integrity (foreign keys resolve), and business logic (status transitions permitted, mandatory fields populated). Define the rule set from authoritative specifications, not from observed patterns in the data, because widespread invalid data can make the "pattern" look normal. Report both the rules violated and the population affected.

### Reconcile across systems and interfaces

Most data integrity failures occur at the boundaries between systems, not within them. Reconcile data across:

- source and target of each interface or migration;
- operational systems and reporting or analytics platforms;
- master data and transactional systems;
- internal records and external confirmations.

Identify where reconciliation breaks, the magnitude of discrepancy, and whether the difference is explained, investigated, and resolved or merely tolerated.

### Assess the controls that maintain integrity, not just current state

Point-in-time integrity is fragile; sustained integrity depends on controls. Evaluate:

- input validation and edit checks at data entry and interface points;
- reconciliation processes with defined tolerance, investigation, and correction;
- exception handling that prevents bad data from flowing unchecked;
- change controls that prevent schema or rule changes from corrupting existing data;
- access controls that prevent unauthorized modification or deletion.

Strong current data with weak maintenance controls predicts future integrity failure; report both.

### Evaluate data migration and interface integrity specifically

Data migrations and ongoing interfaces are high-risk integrity events. For migrations, assess whether the conversion preserved completeness, accuracy, and validity, whether reconciliation was performed between old and new systems, and whether exceptions were resolved before cutover. For ongoing interfaces, test reconciliation over a period, examine error and rejection logs, and confirm that failed records are captured and corrected rather than silently dropped.

### Consider the integrity needs of automated decisions and reporting

Where data drives automated decisions (eligibility, pricing, risk scoring) or external reporting, the integrity bar is higher because errors propagate at scale and are hard to detect downstream. Identify which data elements feed high-stakes automation or public reporting and apply more rigorous testing to those. A small integrity error in a field that drives automated benefit denial is far more consequential than the same error in a descriptive field.

### Report integrity findings by impact and affected population

Quantify each integrity finding by the number of records, the population percentage, the business processes affected, and the downstream consequences. A finding of "2% invalid status codes" is meaningless without context; "2% invalid status codes affecting 14,000 benefit determinations, of which an estimated 600 resulted in incorrect denials" is actionable. Always connect the data issue to its real-world effect.

## Common Traps

- **Single-dimension conclusions.** Reporting "data is accurate" when only completeness or consistency was tested.
- **Record-count completeness.** Accepting a system's own record count as proof of completeness without independent reconciliation.
- **Internal-consistency-as-accuracy.** Testing that two fields in the same system agree and calling it accuracy, when both inherit the same upstream error.
- **No designated system of record.** Testing integrity without establishing which system is authoritative, making inconsistencies uninterpretable.
- **Boundary blindness.** Auditing data within a system while ignoring interfaces and migrations where most corruption occurs.
- **Point-in-time-only assessment.** Reporting current data quality without evaluating the controls that sustain integrity over time.
- **Rule-from-data validation.** Deriving validity rules from observed patterns, normalizing widespread invalid data as the standard.
- **Silent-drop ignorance.** Failing to examine interface error and rejection logs where failed records disappear without correction.
- **Equal-weight findings.** Reporting integrity issues without quantifying affected records and downstream consequences.
- **Low-stakes over-focus.** Spending effort on descriptive fields while ignoring integrity of data that drives automated decisions or external reporting.

## Self-Check

- Have I defined and tested each relevant integrity dimension (completeness, accuracy, validity, consistency, timeliness, uniqueness) separately, rather than collapsing them?
- Have I established the authoritative system of record and traced data lineage before testing?
- Did I test completeness using independent sources or sequence checks, not the audited system's own record count?
- Did I test accuracy against original source documents or third-party confirmations, not merely internal field agreement?
- Are validity rules derived from authoritative specifications, not from observed data patterns that may normalize widespread errors?
- Have I reconciled data across systems, interfaces, and migrations, identifying and quantifying discrepancies?
- Did I assess the controls that maintain integrity over time (input validation, reconciliation, exception handling, change control, access control)?
- For data feeding automated decisions or external reporting, did I apply heightened integrity testing given the scale of potential propagation?
- Are findings quantified by affected records, population percentage, business processes impacted, and downstream consequences, not just stated as present?
