---
name: data-import-migration-and-bulk-operation-support.md
description: Use when the agent is supporting data imports, CSV uploads, bulk edits, migrations, exports for migration, batch jobs, field mapping, deduplication, validation errors, partial import failures, rollback requests, or customer data movement where risks include data loss, duplicate records, privacy exposure, irreversible changes, weak backup guidance, unclear source of truth, or support approving bulk operations without understanding downstream impact.
---

# Data Import Migration And Bulk Operation Support

Bulk data work can change many records faster than a customer or support team can inspect manually. A CSV upload, migration, batch edit, or deduplication job may create duplicates, overwrite fields, expose data, trigger automations, break reports, or become impossible to fully reverse. Agents often treat import errors as formatting problems and miss the larger data-governance and business-process risk. This skill helps the agent support bulk data movement with caution and evidence.

Use this skill when customers ask about imports, migrations, CSV validation, bulk edits, batch jobs, field mapping, deduplication, rollback, partial failures, or moving data between systems. The agent should help diagnose safely without taking ownership of customer data decisions unless authorized.

## Core Rules

### Clarify the data operation and business goal

Identify whether the customer is importing new data, updating existing records, merging duplicates, migrating from another system, exporting for backup, deleting records, bulk assigning ownership, or changing field values. Ask what business outcome they need and which records are affected.

Do not start with file formatting alone. The operation's intent determines source of truth, backup need, mapping, validation, and rollback risk.

### Confirm source of truth and ownership

Before advising bulk changes, know which system is authoritative, who owns the data, and who approved the change. If two systems disagree, do not assume the upload file is correct. Customer stakeholders may need to resolve data ownership first.

Support can explain product behavior, but the customer usually owns whether the data should be changed. Avoid making business data decisions for them.

### Protect backups and rollback expectations

Bulk operations should have a backup or export where available, a sample test, and clear rollback limits. Some operations may be irreversible or only partially reversible. Explain this before the customer proceeds.

Do not imply support can restore data unless the product and policy actually support that restoration. If rollback requires engineering or is not guaranteed, say so.

### Validate mapping and field behavior

Field names, required values, date formats, IDs, picklists, relationships, ownership fields, permissions, and locale settings can cause unexpected results. Ask for safe examples of failing rows, validation errors, and mapping choices, not full sensitive files unless approved.

Explain overwrite behavior clearly. Updating blank values, duplicate keys, or external IDs can have surprising consequences.

### Use sample testing before broad execution

For risky imports or migrations, advise testing with a small representative sample where the product supports it. The sample should include normal records, edge cases, duplicates, missing optional fields, special characters, and permission-sensitive records.

Testing only one perfect row does not validate the real import. A representative sample catches mapping and validation issues before broad damage.

### Consider automation and notification side effects

Bulk changes can trigger emails, webhooks, workflows, billing changes, assignments, reports, integrations, or customer-visible notifications. Ask whether automations should be paused or whether stakeholders should be notified before the operation.

Do not advise rerunning imports repeatedly without checking downstream side effects. Duplicate notifications or records can create customer-facing problems.

### Handle partial failures deliberately

When an import partially succeeds, preserve the success and failure report, identify which records changed, and avoid immediate blind reruns. Determine whether failed rows can be corrected and retried without duplicating successful rows.

A partial failure is a reconciliation problem, not just an error message. The customer needs to know what changed and what remains pending.

### Plan customer communication for visible changes

Bulk operations may change what end users see, receive, or can access. Ask whether admins need to notify users before ownership changes, new records, deleted records, or corrected data appear. Support should flag customer-visible impact even when the technical import succeeds.

### Protect sensitive data in files and logs

Import files may contain personal data, financial details, health data, student data, credentials, or confidential business information. Use approved upload channels and redaction. Avoid asking customers to email full files unless policy allows.

If a file contains secrets or unnecessary sensitive fields, advise removing them before sharing or uploading.

## Common Traps

- Treating import support as a CSV formatting issue while ignoring business data impact.
- Assuming the customer's file is the source of truth.
- Promising rollback or restoration that the product cannot guarantee.
- Testing only a perfect sample row before a large import.
- Overlooking overwrite behavior, duplicate keys, blank values, and locale formats.
- Rerunning a partial import and creating duplicates.
- Completing a bulk change without considering whether affected users need notice.
- Ignoring automations, emails, webhooks, reports, and integrations triggered by bulk changes; collecting full sensitive files through unsafe channels

## Self-Check

- Is the bulk operation type and business goal clear?
- Is source of truth and data owner identified?
- Has the customer approved the data change rather than only asked for technical help?
- Are backup, export, rollback, and irreversibility limits explained?
- Are field mappings, required values, IDs, duplicate keys, dates, locale, and overwrite behavior checked?
- Has a representative sample test been considered before broad execution?
- Are automation, notification, integration, billing, and reporting side effects considered?
- Are partial successes and failures reconciled before retry?
- Has user or stakeholder communication been considered for visible data changes?
- Are files, logs, and examples shared through approved channels with sensitive data minimized?; would the customer understand exactly what changed, what failed, and what risk remains?
