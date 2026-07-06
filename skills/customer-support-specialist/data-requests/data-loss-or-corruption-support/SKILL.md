---
name: data-loss-or-corruption-support.md
description: Use when the agent is handling customer reports of missing, deleted, overwritten, corrupted, duplicated, inaccurate, unavailable, or inconsistent data, including file loss, record corruption, failed sync, migration errors, import problems, backup restore questions, account history discrepancies, and data integrity incidents.
---

# Data Loss Or Corruption Support

Reports of lost or corrupted data are high-stress and high-risk. The customer may have lost work, records, money, compliance evidence, creative output, or business operations. Agents can make the problem worse by telling customers to retry, resync, reinstall, or overwrite state before evidence is preserved. This skill helps the agent handle data integrity issues with caution, evidence preservation, and clear escalation.

## Core Rules

### Treat data integrity as potentially high impact

Missing or corrupted data may be cosmetic, but it may also affect billing, legal records, customer records, medical or safety context, business reporting, inventory, orders, creative work, audit logs, or access rights. Start with impact, not assumptions.

Ask what data is missing or wrong, when it was last known good, who is affected, whether work is blocked, and whether the customer has deadlines or compliance obligations.

### Preserve evidence before suggesting fixes

Before advising sync resets, app reinstall, cache clearing, import retry, restore, merge, deletion, or manual edits, consider whether those actions could overwrite evidence or make recovery harder. Capture timestamps, object IDs, screenshots, error messages, device and app versions, recent actions, integrations, migration jobs, and affected records.

If the issue may be system-wide, route to incident or engineering quickly.

### Separate visibility from loss

Customers may think data is gone when it is hidden by filters, permissions, plan limits, device sync delay, archived state, region, account switch, or temporary outage. Investigate visibility and access causes without dismissing the report.

Do not tell the customer their data is safe until the underlying state is confirmed.

### Clarify recovery options and limits

Recovery may involve backup restore, point-in-time recovery, undelete, import rollback, audit log reconstruction, manual correction, engineering script, or no available recovery. Each option has tradeoffs: overwriting newer changes, partial recovery, downtime, privacy review, or cost.

Use approved language for data recovery guarantees. Never promise that data can be restored until the recovery owner confirms it.

### Protect other customers and shared records

In team accounts, shared documents, marketplaces, multi-tenant systems, or integrations, restoring one customer's data may affect other users or downstream systems. Verify ownership and authority before changing shared records.

If corruption crosses accounts or tenants, escalate as a potential incident and privacy or security risk.

### Communicate uncertainty with care

Customers reporting data loss need direct, calm updates. Say what is known, what evidence was collected, what is being checked, what actions the customer should avoid, and when the next update will come.

Avoid phrases like "just a display issue" unless confirmed. Avoid blame toward the customer before logs and product behavior are understood.

### Document for root cause and remediation

Record reproduction path, last-known-good state, suspected trigger, customer impact, data categories, recovery attempt, outcome, and remaining risk. Data integrity cases should feed bug triage, incident review, migration checks, backup validation, and support guidance.

If recovery fails, document the decision and customer-facing explanation carefully.

### Decide when to freeze change

If continued customer or system activity may overwrite logs, mutate records, trigger sync, resend imports, duplicate transactions, or destroy a recoverable state, ask the recovery owner whether a temporary freeze is needed. This may include pausing imports, avoiding manual edits, suspending automation, stopping retries, preserving the device state, or holding a migration job.

Do not recommend a freeze casually because it may block business operations. Explain the tradeoff and duration when approved.

### Coordinate customer workarounds with recovery

Customers often need to keep operating while recovery is investigated. A workaround such as manual reconstruction, CSV import, duplicate record creation, alternate system use, or re-entry of missing work may interfere with later restoration.

Tell the customer which workaround actions are safe, which should be avoided, and how to label temporary work so it can be reconciled later. If the team cannot advise safely, escalate before the customer invests hours in work that may be overwritten.

## Common Traps

- Telling the customer to retry, resync, reinstall, or reimport before preserving evidence.
- Assuming data is only hidden or filtered without confirming.
- Promising restoration because backups exist.
- Ignoring shared-account authority before restoring or editing records.
- Treating one report as isolated when it may signal a wider incident; overwriting newer data with an old restore without explaining tradeoffs
- Failing to tell the customer what actions to avoid while investigation is active; blaming user error too early
- Recording only "data missing" without object IDs, timestamps, last-known-good state, or impact; allowing imports, sync, automation, or manual edits to continue when they may overwrite evidence or recoverable data
- Suggesting temporary workarounds that later conflict with restore, deduplication, or reconciliation; closing after a workaround without feeding the integrity signal to product or engineering

## Self-Check

- Has customer impact been assessed across work, money, compliance, records, safety, reporting, inventory, orders, and deadlines?
- Is the last-known-good state recorded?
- Are timestamps, object IDs, screenshots, errors, device and app versions, integrations, migrations, and recent actions captured?
- Were potentially destructive customer actions avoided until evidence was preserved?
- Have visibility causes such as filters, permissions, archive state, plan limits, account switch, sync delay, or outage been checked?
- Are recovery options and tradeoffs explained without guarantees?
- Is shared-account authority verified before restore, correction, deletion, or merge?
- Has cross-account, multi-tenant, privacy, security, or incident risk been escalated?
- Does the customer know what to avoid and when to expect the next update?; is the case documented for root cause, bug triage, incident review, backup validation, and support guidance?
- Was a temporary freeze considered where ongoing activity could destroy evidence or recoverable state?; are customer workarounds coordinated with the recovery plan to avoid later conflicts?
