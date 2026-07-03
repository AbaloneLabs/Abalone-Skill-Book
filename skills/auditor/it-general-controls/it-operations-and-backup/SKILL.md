---
name: it-operations-and-backup.md
description: Use when the agent is testing IT operations controls, evaluating job scheduling and batch processing, assessing backup and recovery, testing incident and problem management, evaluating data integrity and availability controls, or deciding whether operations weaknesses affect the completeness and accuracy of system-generated data used in the audit.
---

# IT Operations and Backup

IT operations controls keep financial systems running correctly and recoverable. They cover batch job scheduling, interface processing, backup and restore, incident and problem management, and the integrity of data held in the systems. Operations is often treated as the least important IT general control because it feels like plumbing, but it is precisely where completeness and accuracy of system-generated data are won or lost: a failed batch job that drops transactions, an interface that silently loses records, or a restore from an untested backup can all produce financial statements that are materially incomplete or wrong. The discipline is to connect operations controls to the completeness and accuracy of the data the audit relies on.

## Core Rules

### Test batch and interface processing for completeness

Financial data is frequently moved between systems by scheduled batch jobs and interfaces (subledger to GL, payroll to GL, subsidiary to consolidation). Each is a completeness risk: if a job fails or an interface drops records, transactions vanish from the financial statements. For significant interfaces and batches:

- confirm jobs run on schedule and that failures are detected, investigated, and re-run;
- test reconciliation between source and target (record counts, control totals, amount reconciliations);
- confirm exception handling — records that fail validation are queued, resolved, and not silently dropped;
- review a period of job logs for failures and their resolution.

Unreconciled interfaces are a primary source of completeness misstatements; the reconciliation between systems is often the key detective control and should be tested as such.

### Confirm data integrity controls operate

Beyond movement of data, confirm controls that protect the integrity of data at rest:

- edit checks and validations on data entry (range, reasonableness, required fields);
- controls over direct database updates that bypass application validations (who can do them, are they logged);
- reconciliation between system data and physical reality (inventory counts, fixed asset physical verification);
- controls over data imports from external sources (bank files, price feeds) including validation and authorisation.

Data integrity is what makes system-generated reports trustworthy enough to use as audit evidence. Where integrity controls are weak, reports generated from the system cannot be assumed complete and accurate and must be independently corroborated.

### Test backup and, critically, restore

Backups are only useful if they can be restored. Many entities perform backups reliably but never test restoration, so when restoration is needed it fails. Confirm:

- backups cover the financial systems and occur at a frequency matched to the rate of data change and the tolerable data loss;
- backups are stored securely, including off-site or in geographically separate cloud regions, with access controlled;
- restoration is tested periodically, not just assumed, and the test includes the financial systems;
- recovery time and recovery point objectives are understood and consistent with the entity's ability to continue financial reporting.

A backup that has never been restored is an untested control. For audit purposes, the question is whether the entity could recover its financial records after a loss; confirm this with evidence of a successful test restore, not just a backup schedule.

### Evaluate incident and problem management for financial impact

Incidents (unexpected events) and problems (recurring underlying causes) are managed through a process that should capture, prioritise, and resolve them. For audit purposes, focus on incidents that could affect financial data:

- were there incidents during the period that caused data loss, corruption, or processing failure?
- how were they resolved, and was the resolution verified (e.g., reprocessing of dropped transactions)?
- were incidents that affected financial controls escalated to finance and the auditor?

An incident log that shows system outages or data issues during the period, with no evidence of financial impact assessment, is a gap. Confirm finance was involved in assessing incidents that touched financial systems.

### Test job scheduling controls and the handling of job failures

Scheduled jobs (depreciation runs, allocation posts, consolidation, period-close sequences) are where financial processing happens. Confirm:

- jobs are scheduled and initiated by authorised roles, not ad hoc by any user;
- job failures generate alerts that are monitored and acted upon;
- re-runs after failure are controlled so that transactions are neither duplicated nor dropped;
- period-close jobs run in the correct sequence and are signed off.

Job failures during period-close are a common source of cutoff and completeness errors; the failure handling is the control that prevents them from becoming misstatements.

### Connect operations controls to the completeness and accuracy assertion

The reason operations matters for the audit is the completeness and accuracy of the data used to produce the financial statements and the data used as audit evidence. Make the connection explicit:

- for each significant interface, identify the reconciliation or control total that confirms completeness, and test it;
- for each significant report used as audit evidence, confirm the underlying data integrity controls support relying on it;
- for each automated calculation, confirm the job that produces it runs correctly and that failures are caught.

An operations control with no identified effect on completeness or accuracy is being tested generically; tie it to a specific data risk to make the testing purposeful.

### Scope operations testing to material systems and flows

Not every job and interface matters equally. Focus operations testing on:

- interfaces that move material transaction volumes or values (revenue subledger to GL, payroll, consolidation);
- jobs that perform material calculations (depreciation, allocations, revenue recognition);
- systems whose data is used directly as audit evidence (trial balance, AR ageing, inventory);
- end-user computing files whose loss or corruption would be material.

Apply lighter procedures to low-materiality systems. Depth follows the materiality of the data and the extent of reliance.

### Assess the effect of operations weaknesses on the audit response

Where operations controls are weak — unreconciled interfaces, untested restores, unmonitored job failures — the data produced by the systems cannot be fully trusted. This forces:

- more independent reconciliation of system data to external or source evidence;
- broader substantive testing of completeness;
- reduced reliance on system-generated reports as audit evidence;
- in severe cases, data integrity procedures on the populations used.

Record each significant operations weakness and its effect on the audit plan, rather than leaving it as a standalone IT finding.

## Common Traps

- **Treating operations as plumbing** without connecting it to the completeness and accuracy of financial data and audit evidence.
- **Testing backup without testing restore**, accepting a backup schedule as evidence of recoverability.
- **Overlooking interface reconciliations** as the key completeness control between systems.
- **Accepting job schedules without testing failure handling**, missing the risk of duplicated or dropped transactions on re-run.
- **Missing direct database update paths** that bypass application validations and can corrupt data without trace.
- **Treating incident logs as IT records only**, without assessing whether incidents affected financial data and whether finance was involved.
- **Testing every system to the same depth**, wasting effort on immaterial jobs while under-testing the interfaces that move material data.
- **Letting operations weaknesses sit as IT findings** without reflecting them in expanded substantive testing and reduced reliance on system data.
- **Forgetting end-user computing files** (critical spreadsheets) whose backup, version control, and integrity are often weakest and most material.

## Self-Check

- For each significant interface or batch, did I test the reconciliation or control total that confirms completeness between source and target?
- Did I confirm data integrity controls — edit checks, controls over direct database updates, reconciliation to physical reality — support the system-generated data I rely on?
- Did I test restore, not just backup, with evidence of a successful test restore of the financial systems?
- Did I review the incident and problem log for events affecting financial systems, and confirm finance assessed the impact?
- Did I test job scheduling controls and failure handling, including controlled re-runs that avoid duplication or loss?
- For each operations control tested, did I connect it to a specific completeness or accuracy risk in the data?
- Did I scope operations testing depth to material systems, interfaces, and end-user computing files?
- Where operations controls are weak, did I reflect it in more independent reconciliation, broader substantive completeness testing, and reduced reliance on system-generated evidence?
