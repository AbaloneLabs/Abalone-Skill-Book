---
name: operational-data-correction-and-reconciliation.md
description: Use when the agent is correcting operational data errors, reconciling records across systems, repairing duplicate or missing records, resolving mismatched statuses, validating financial or customer-impacting data, or closing data cleanup work.
---

# Operational Data Correction And Reconciliation

Data correction is operational recovery. A wrong status, duplicate record, missing approval, mismatched invoice, or broken integration can affect customers, money, controls, staffing, and reporting. Agents often recommend "clean up the data" without defining scope, evidence, authority, downstream repair, or reconciliation proof. This skill helps the agent correct operational data safely and verify that the correction actually resolves the impact.

## Core Rules

### Define the error and affected population

Start by describing the data defect precisely: missing value, wrong status, duplicate record, merged record, split identity, stale timestamp, incorrect amount, mismatched owner, bad category, failed integration, or incomplete migration. Identify affected records, systems, customers, time period, and confidence level.

If the affected population is unknown, define discovery steps and sampling. Do not assume the visible record is the only error when the cause may be systemic.

### Determine authority to correct

Not everyone who can edit a record should correct it. Define who has authority for customer identity, financial amounts, eligibility, approvals, inventory, employee records, regulated data, and audit evidence. Some corrections require finance, legal, compliance, privacy, or system-owner approval.

Emergency correction may be needed to prevent harm, but it still requires record of reason, approver or post-approval, scope, and review.

### Preserve evidence and prior state

Before correction, preserve enough evidence to explain what changed and why: prior value, new value, source record, timestamp, approver, affected systems, and correction method. For high-risk records, keep audit trail and change ticket.

Do not overwrite history in a way that makes disputes, audits, root-cause analysis, or customer remediation impossible.

### Reconcile across systems

Many data errors span CRM, ticketing, billing, warehouse, workforce, vendor, analytics, and finance systems. Define source of truth and reconcile downstream copies. If one system is corrected and another remains wrong, the error may reappear.

Use reconciliation checks: record counts, totals, status match, ID match, timestamp order, customer-level comparison, invoice total, inventory count, or sample review. The check should match the risk.

### Control bulk correction carefully

Bulk fixes can repair many records quickly, but they can also create broad new errors. Define selection criteria, test sample, approval, dry run where possible, backup or prior-state preservation, communication to affected teams, and post-run validation. For high-impact records, use staged correction rather than one irreversible update.

If the correction changes customer-facing status, financial amount, or eligibility, check whether customers, support, finance, or compliance need notification before or after the run.

### Assess customer, financial, and control impact

Data correction may require customer communication, refund, reissue, service recovery, billing adjustment, report restatement, audit evidence, quality review, or policy exception. Identify whether the wrong data caused an action, missed action, or wrong communication.

Correction of the record is not the same as correction of the consequence. If a customer was charged incorrectly or denied service, the downstream impact needs separate recovery.

### Prevent recurrence

After correction, identify why the error happened: bad validation, unclear field definition, manual entry, duplicate intake, integration failure, migration mapping, insufficient training, vendor feed, access issue, or automation rule. Decide whether prevention, detection, or response control should change.

If the same correction recurs, treat it as an operating defect, not routine cleanup.

### Close with proof

Closure should include corrected records, reconciled downstream systems, evidence preserved, impacted customers or reports handled, root cause or next investigation assigned, and monitoring for recurrence. For bulk corrections, sample after correction and confirm no unintended changes occurred.

Do not declare completion because a script ran or a spreadsheet was updated. Verify operational outcome.

## Common Traps

- Correcting one visible record when the defect may affect a population.
- Allowing anyone with edit rights to make high-impact corrections without authority.
- Overwriting prior state and destroying audit or dispute evidence.
- Fixing the source system but leaving downstream systems unreconciled.
- Treating record correction as complete while customer, billing, or control consequences remain.
- Repeating manual cleanup instead of fixing the cause.
- Closing bulk correction without sample review or unintended-change check.
- Running bulk fixes without dry run, staged rollout, backup, and post-run validation; letting corrected records re-enter old queues because downstream routing rules were not refreshed

## Self-Check

- Is the data defect described precisely by type, affected systems, records, customers, period, and confidence level?
- If the population is unknown, are discovery, sampling, and sizing steps defined?
- Is correction authority clear for identity, financial, eligibility, approvals, inventory, employee, regulated, and audit data?
- Are prior value, new value, source evidence, timestamp, approver, affected systems, and correction method preserved?
- Are source-of-truth rules and downstream reconciliation checks defined across all affected systems?
- For bulk correction, are selection criteria, dry run, staged rollout, prior-state preservation, approval, and post-run validation defined?
- Have customer, financial, service, report, audit, quality, and control consequences been assessed separately from record correction?
- Is recurrence prevention assigned through validation, definition, training, integration, vendor, access, or automation fixes?
- Does closure include correction proof, reconciliation, evidence, impact handling, root-cause follow-up, recurrence monitoring, and sample review?
- Have downstream queues, alerts, reports, and routing rules been refreshed so corrected records do not regress?
