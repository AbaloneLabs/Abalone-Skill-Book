---
name: third-party-integration-sync-issue-support.md
description: Use when the agent is supporting third-party integrations, data sync, marketplace apps, connector failures, CRM or payment integration issues, OAuth app connections, sync delays, duplicate records, mapping errors, vendor coordination, or cross-system troubleshooting where risks include blaming the wrong system, exposing customer data to vendors, changing mappings without approval, missing rate limits, or failing to preserve customer ownership of connected systems.
---

# Third Party Integration Sync Issue Support

Third-party integration issues live between systems, vendors, and customer configuration. A sync failure may come from credentials, permissions, field mapping, API limits, vendor outage, customer data shape, duplicate rules, network failures, or product regression. Agents often blame the other vendor or escalate internally without enough cross-system evidence. This skill helps the agent coordinate integration troubleshooting while preserving ownership, privacy, and customer trust.

Use this skill when handling connector failures, CRM sync issues, payment integrations, marketplace apps, OAuth app connections, duplicate records, delayed sync, mapping errors, missing data, vendor coordination, or integration disconnects. The agent should identify the failure boundary and avoid unsupported changes to customer systems.

## Core Rules

### Map the integration path

Identify the systems involved, direction of sync, object types, fields, trigger, schedule, authentication method, connector owner, and expected result. A sync from product to CRM is different from CRM to product, bidirectional sync, import, export, or webhook-based update.

Do not troubleshoot "integration broken" as one category. The path determines evidence and risk.

### Determine where the failure occurs

Separate connection failure, authentication failure, permission issue, mapping error, validation failure, rate limit, duplicate conflict, vendor outage, customer data issue, product bug, and delayed processing. Ask what changed recently: credentials, permissions, fields, workflow, app version, vendor plan, API limits, or data volume.

Use evidence from both systems when possible: timestamps, record IDs, sync job IDs, error messages, connector logs, vendor status, and affected objects.

### Protect customer data during vendor coordination

When involving a vendor or partner, share only the data allowed by customer agreement and policy. Do not send customer records, personal data, payment details, screenshots, or logs to a third party without authorization and redaction.

If the customer must open a vendor ticket, help them understand what safe evidence to provide. Support should not become an unauthorized data broker.

### Avoid unapproved mapping and data changes

Field mappings, deduplication rules, sync direction, overwrite behavior, and object ownership can materially change customer data. Do not edit them casually to test a theory unless the customer authorizes the change and rollback is understood.

If a mapping change could overwrite data, create duplicates, trigger automation, or affect billing, escalate or advise testing in a safe environment where available.

### Watch rate limits and volume effects

Sync issues often appear when volume grows, bulk edits run, retries stack up, or a connected app hits API limits. Check whether the customer recently imported records, launched a campaign, changed workflow, or increased usage.

Do not recommend repeated manual sync runs without understanding rate limits and duplicate risk. More retries can worsen the problem.

### Communicate shared responsibility clearly

Integration support often requires actions by the customer, third-party vendor, and internal team. State who owns each step and what evidence each party needs. Avoid blaming the vendor before boundary evidence exists.

If the issue is outside supported scope, explain the boundary while still offering safe next steps such as documentation, logs, or vendor contact path.

### Preserve records and reconciliation

Sync failures can leave partial updates, duplicate records, missing fields, or inconsistent states. Before advising cleanup, understand which system is source of truth and whether audit history matters. Bulk correction can cause more damage than the original failure.

For serious data divergence, recommend a reconciliation plan with backup, sample validation, owner approval, and post-fix checks.

### Feed recurring issues back to product and partnerships

Repeated connector failures may indicate poor documentation, brittle mapping, vendor API change, missing monitoring, product limitation, or partner support gap. Capture patterns by vendor, connector, object, error code, and customer segment.

Do not treat recurring integration pain as isolated support tickets if the marketplace or product experience needs improvement.

## Common Traps

- Blaming the third-party vendor before locating the failure boundary.
- Troubleshooting without knowing sync direction, object, trigger, and source of truth.
- Sharing customer data or logs with vendors without authorization and redaction.
- Changing field mappings or sync rules without customer approval and rollback plan.
- Running repeated manual syncs that worsen rate limits or duplicate records.
- Treating partial data divergence as simple cleanup without reconciliation plan.
- Failing to tell the customer which party owns which next step.
- Missing recurring connector patterns that should go to product or partnerships.

## Self-Check

- Are systems, sync direction, objects, fields, trigger, schedule, and auth method mapped?
- Is the failure classified by connection, permission, mapping, validation, rate limit, vendor outage, data issue, or product bug?
- Are timestamps, record IDs, sync job IDs, connector logs, vendor status, and errors captured?
- Is customer data shared with vendors only through approved, redacted, authorized paths?
- Are mapping, overwrite, dedupe, and sync-direction changes approved and reversible?
- Are rate limits, bulk jobs, retries, and recent volume changes considered?
- Is source of truth clear before cleanup or reconciliation?
- Are customer, vendor, and internal next steps separated with owners?
- Are recurring connector issues routed to product, documentation, or partnerships?
- Would the customer see the support response as accountable without unsupported vendor blame?
