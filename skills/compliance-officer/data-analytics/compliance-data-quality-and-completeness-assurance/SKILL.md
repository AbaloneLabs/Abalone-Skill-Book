---
name: compliance_data_quality_and_completeness_assurance.md
description: Use when the agent is assessing compliance data quality, validating population completeness, reconciling data across systems, documenting data lineage, or ensuring that monitoring, testing, and reporting rest on reliable and complete underlying data.
---

# Compliance Data Quality And Completeness Assurance

Every compliance monitoring system, dashboard, KRI, and control test depends on data. When the data is incomplete, inconsistent, or poorly understood, every conclusion built on it is unreliable. The danger is that poor data quality rarely surfaces on its own; dashboards still render green, reports still generate, and tests still produce numbers. The failure appears only later, when a regulator asks why a population was missing transactions, when a known case was not caught, or when two reports show different totals for the same metric. Data quality and completeness assurance is the discipline of proving that the data supporting compliance work is fit for purpose before relying on it, and of maintaining that proof as systems change.

Use this skill before relying on data for monitoring, testing, metrics, or reporting; when reconciling populations across systems; when documenting lineage; or when investigating why a metric or alert behaves unexpectedly. The goal is to make the agent treat data as a control dependency that must be validated, reconciled, and governed, not as a given.

## Core Rules

### Treat Data Quality As A Prerequisite, Not An Afterthought

Before building analytics, metrics, or tests on a data source, confirm the source is fit for the intended use. A beautiful dashboard built on unreliable data is worse than no dashboard, because it creates false confidence.

For each data source assess:

- completeness: does it contain all the records and fields it should;
- accuracy: do the values correctly represent the underlying transactions or events;
- timeliness: is the data current enough for the intended use;
- consistency: do related fields and records agree within and across sources;
- validity: do values conform to expected formats, ranges, and reference data;
- uniqueness: are records duplicated in ways that would distort counts or analytics;
- integrity: are relationships between records and entities preserved.

State explicitly which dimensions matter most for the intended use. For population completeness in control testing, completeness is paramount; for real-time alerting, timeliness may dominate. Fit for purpose is the standard, not perfection.

### Prove Population Completeness Through Independent Reconciliation

Completeness is the dimension most often assumed and most often wrong. A population that looks complete within one system may exclude transactions that flowed through another.

Reconcile the population by:

- comparing record counts and totals to an independent control source, such as a general ledger, settlement system, or authoritative register;
- reconciling across the period, not only at a point in time;
- identifying and explaining all variances;
- confirming that voided, reversed, deleted, manual, and exception items are included or intentionally excluded with rationale;
- confirming that all in-scope systems, channels, and entities feed the population;
- testing for records that should be present but are not, such as by tracing a sample from the source system forward into the population.

If the population cannot be reconciled to an independent source, document the limitation and avoid conclusions that the data cannot support. An unreconciled population is a material limitation, not a minor caveat.

### Document Data Lineage End To End

Lineage is the map of where data originates, how it moves, how it is transformed, and where it is consumed. Without lineage, errors are nearly impossible to diagnose and changes are risky.

Document for each data feed:

- the originating system and record;
- each transfer, transformation, aggregation, and filtering step;
- the jobs, schedules, and dependencies that move the data;
- the reference data and lookup tables used;
- the consuming systems, reports, and analytics;
- the owners of each stage;
- known limitations, assumptions, and manual interventions.

Lineage should allow a value in a report to be traced back to its source record. When lineage is opaque, a change upstream can silently break downstream analytics with no one noticing until a case is missed.

### Reconcile Across Systems And Resolve Entity Resolution Challenges

Compliance data often spans multiple systems that use different identifiers for the same party, account, or transaction. Without reconciliation, the same activity can be split or duplicated, and relationships can be missed.

Address:

- mapping of identifiers across systems, such as customer IDs, account numbers, and transaction references;
- entity resolution where the same party appears under different identifiers or name variants;
- handling of mergers, acquisitions, reorganizations, and system migrations that change identifiers;
- reconciliation of reference data such as risk ratings, classifications, and hierarchies;
- timing differences between systems that can cause transient mismatches.

Entity resolution failures are a common reason that aggregation-based detection misses structuring or network patterns. Test entity resolution against known relationships to confirm it works.

### Validate Data After Every System Or Feed Change

Data quality is not a one-time certification. Systems, feeds, transformations, and reference data change continuously, and each change can silently degrade quality.

Re-validate after:

- source system upgrades or configuration changes;
- data feed, ETL, or pipeline changes;
- reference data updates;
- system migrations, consolidations, or decommissions;
- mergers, acquisitions, or divestitures;
- changes to business processes that affect what is captured;
- changes to retention, archiving, or purge schedules.

Maintain a calendar of known change events and trigger validation accordingly. A feed that broke during a migration and was never noticed can create a months-long blind spot.

### Establish Data Quality Controls And Monitoring

Data quality should be controlled and monitored continuously, not only checked at project start. Build controls around the data.

Implement:

- automated checks for record counts, totals, nulls, duplicates, and validity at each load;
- reconciliation controls that compare feeds to source or control totals;
- exception reporting for records failing quality checks;
- a process to investigate and resolve data quality exceptions;
- monitoring of data quality metrics themselves, such as completeness and reconciliation rates over time;
- ownership and escalation for persistent quality issues.

Treat data quality failures as control failures. A persistently failing reconciliation is a signal that something is broken upstream and that downstream reliance is at risk.

### Understand And Disclose Limitations

No data source is perfect. The integrity of compliance work depends on understanding and disclosing limitations rather than hiding them.

Disclose:

- known gaps in coverage, such as channels or entities not captured;
- fields that are unreliable, defaulted, or frequently missing;
- lags between event and data availability;
- assumptions made in transformations or entity resolution;
- periods affected by known data issues;
- the effect of these limitations on conclusions drawn from the data.

A report or test that silently relies on incomplete data will be challenged. A report that discloses its limitations and scopes its conclusions accordingly is far more defensible.

### Assign Data Ownership And Accountability

Data quality degrades when nobody owns it. Assign clear ownership for each data domain and feed.

Define:

- the business owner accountable for the accuracy and completeness of the source data;
- the data steward responsible for quality monitoring and issue resolution;
- the technical owner of the pipeline and transformations;
- the consumer owner responsible for flagging when data no longer meets needs;
- escalation paths for unresolved quality issues.

Without ownership, data quality issues fester until they cause a visible failure. Ownership ensures that quality is maintained as a routine discipline.

## Common Traps

### Assuming A Population Is Complete Because It Looks Populated

A populated dataset can still miss entire channels or segments. Reconcile to an independent source.

### Building Analytics Before Validating The Underlying Data

Dashboards render regardless of data quality, creating false confidence. Validate fitness for purpose first.

### Opaque Lineage That Prevents Diagnosis

Without documented lineage, upstream changes break downstream analytics silently. Map lineage end to end.

### Ignoring Entity Resolution Across Systems

Different identifiers for the same party split or hide relationships. Test resolution against known cases.

### No Re-Validation After System Or Feed Change

A migration can break a feed for months unnoticed. Maintain a change calendar and re-validate.

### Hiding Data Limitations To Keep Reports Clean

Undisclosed gaps mislead users and invite challenge. Disclose limitations and scope conclusions.

### No Owner For Data Quality

Unowned data quality issues persist until they cause failure. Assign business, steward, technical, and consumer owners.

## Self-Check

- Has each data source been assessed for completeness, accuracy, timeliness, consistency, validity, uniqueness, and integrity against its intended use before reliance?
- Is population completeness proven through independent reconciliation of counts and totals across the period, with variances explained and excluded segments justified?
- Is data lineage documented from origin through each transformation to consumption, with owners, dependencies, reference data, limitations, and manual interventions recorded?
- Are identifiers mapped and entity resolution tested across systems, including mergers, migrations, reference data, and timing differences?
- Is data quality re-validated after source upgrades, feed or pipeline changes, reference data updates, migrations, M&A, process changes, and retention schedule changes?
- Are data quality controls and monitoring in place, including automated checks, reconciliations, exception reporting, issue resolution, and quality metric trends?
- Are known coverage gaps, unreliable fields, lags, assumptions, affected periods, and their effect on conclusions disclosed rather than hidden?
- Is ownership assigned for each data domain covering business owner, data steward, technical pipeline owner, and consumer owner, with escalation for unresolved issues?
- Are data quality failures treated as control failures with investigation and resolution rather than tolerated anomalies?
- Can a value in a report or alert be traced back through documented lineage to its source record?
