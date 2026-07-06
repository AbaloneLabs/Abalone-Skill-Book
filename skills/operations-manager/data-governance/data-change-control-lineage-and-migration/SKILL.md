---
name: data-change-control-lineage-and-migration.md
description: Use when the agent is planning or reviewing operational data changes, field changes, status changes, system migrations, data lineage, backfills, imports, exports, mapping changes, integration changes, or data model changes that affect operations.
---

# Data Change Control Lineage And Migration

Operational data changes can quietly reshape work. A renamed status, new field, changed mapping, imported file, or migration can alter routing, reporting, billing, automation, compliance evidence, and customer communication. Agents often treat data changes as technical implementation details. This skill helps the agent manage data changes with lineage, testing, rollback, and operational communication.

## Core Rules

### Identify what the data change touches

Start by naming the changed data element: field, status, category, ID, owner, timestamp, mapping, integration, report logic, automation input, import file, export file, or master record. Then identify every downstream use: queue routing, SLA calculation, billing, eligibility, customer messaging, quality review, reporting, forecasting, compliance evidence, vendor feeds, and audit logs.

Small changes can have wide consequences. A status value that looks internal may drive dashboards, customer notices, and automation triggers.

### Preserve lineage and transformation rules

Define where the data comes from, how it is transformed, which system owns it, which systems consume it, and how changes move between them. For migrations or backfills, document mapping rules, default values, excluded records, changed meanings, and known limitations.

Lineage should be understandable by operations, not only engineers. Operators need to know what the field means and whether they can trust it for decisions.

### Test with real operational scenarios

Test normal cases, edge cases, missing values, duplicates, historic records, manually corrected records, vendor records, high-risk customer records, and records crossing systems. Verify reports, automations, permissions, customer communications, and exception queues after the change.

Do not validate only that the data loaded. Validate that operations can use the changed data correctly.

### Control timing and freeze windows

Schedule data changes around operational calendars: payroll, billing, close, audit, renewal, peak volume, regulatory deadlines, incident recovery, and major launches. If the change must occur in a sensitive window, define extra monitoring, support, and rollback.

Communicate cutover timing, dual-running rules, and source of truth. If old and new data coexist, operators need to know which one to use and when.

### Plan rollback and correction

Some data changes are hard to reverse once downstream actions occur. Define rollback feasibility, correction scripts, record restoration, customer communication, manual reconciliation, and owner for cleanup. If rollback is impossible, use stronger testing and approval before change.

Preserve pre-change values where needed. They may be required for audit, dispute handling, or restoring records.

### Use parallel runs for high-risk transitions

For high-impact migrations or definition changes, consider parallel reporting or dual validation before full cutover. Compare old and new outputs, investigate differences, and agree which differences are expected. Parallel runs are especially useful when the change affects billing, compliance, customer communication, performance reporting, or automation.

Do not run old and new paths indefinitely. Define decision criteria and end date so dual-running does not become a permanent reconciliation burden.

### Govern approval and communication

Data changes should have business owner approval, technical owner, data owner, downstream stakeholder awareness, testing evidence, and change record. Communicate meaning changes to users, analysts, quality teams, support, and control owners.

If the change affects metrics, explain whether historical comparisons remain valid. A trend break should not be mistaken for performance change.

### Monitor after cutover

After implementation, watch error rates, missing values, rejected records, integration failures, report breaks, automation exceptions, customer contacts, manual corrections, and user questions. Set a stabilization period and exit criteria.

If issues appear, distinguish data load defects, mapping defects, user behavior, source-system problems, and old records that no longer fit the model.

## Common Traps

- Treating a field or status change as local when it drives downstream reports or automation.
- Migrating data without documenting mapping assumptions and excluded records.
- Testing technical load success but not operational use.
- Changing data during close, billing, audit, or peak service windows without extra controls.
- Allowing old and new sources of truth to coexist without rules.
- Failing to preserve pre-change values needed for disputes or audit.
- Running old and new data paths without criteria for comparison, decision, and retirement.
- Letting a trend break look like performance change; forgetting to brief downstream users on how their daily decisions should change after cutover

## Self-Check

- Are changed fields, statuses, IDs, mappings, integrations, reports, automations, imports, exports, and master records identified?
- Have downstream uses in routing, SLA, billing, eligibility, communication, quality, reporting, forecasting, compliance, vendors, and audit been mapped?
- Are lineage, transformation rules, source ownership, consumers, defaults, exclusions, and limitations documented?
- Has testing covered real operational scenarios, historic records, duplicates, missing values, vendors, permissions, reports, automations, and exception queues?
- Is timing aligned with operating calendars, freeze windows, cutover rules, and source-of-truth guidance?
- Are rollback, correction, restoration, reconciliation, customer communication, and cleanup ownership defined?
- For high-risk changes, is parallel validation or dual reporting defined with comparison criteria and end date?
- Are approvals, testing evidence, business meaning changes, user communication, and metric trend impacts recorded?
- Is post-cutover monitoring defined for errors, missing values, rejected records, reports, automations, customer contacts, and user questions?
- Have downstream users been told how their operating decisions change during and after cutover?
