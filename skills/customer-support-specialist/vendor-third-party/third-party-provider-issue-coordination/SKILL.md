---
name: third-party-provider-issue-coordination.md
description: Use when the agent is coordinating customer support around third-party provider failures, vendor outages, payment processor issues, carrier delays, identity provider problems, marketplace platform defects, integration failures, outsourced support dependencies, or partner-owned incidents that affect the customer's outcome.
---

# Third Party Provider Issue Coordination

Third-party failures are still customer-facing failures. Support teams may not control the provider, but they do control evidence collection, escalation, communication, expectation setting, and follow-through. Poor coordination creates circular blame, stale updates, duplicated customer effort, and missed remediation. This skill helps the agent manage vendor-dependent issues without pretending to control what they cannot.

## Core Rules

### Separate customer ownership from vendor control

Be clear internally about what the organization owns, what the provider owns, and what the customer needs. Do not tell the customer simply to "contact the vendor" if the organization sold, configured, recommended, billed, or operationally depends on that vendor.

Customer ownership means giving a clear path, tracking the case, and communicating known limits. It does not mean guaranteeing a fix from a provider outside your control.

### Gather provider-ready evidence

Vendors often need specific artifacts: timestamps, transaction IDs, logs, error codes, screenshots, request IDs, carrier scans, payment authorization IDs, identity-provider traces, affected accounts, API payload examples, region, device, and reproduction steps.

Collect evidence once and structure it. Repeatedly asking customers for the same information because the vendor escalation packet was weak damages trust.

### Map escalation channels and SLAs

Know how to escalate to the provider: portal, account manager, emergency contact, partner Slack, email alias, carrier claim, payment processor dispute path, or integration support. Track provider SLA, response time, escalation level, and next checkpoint.

If provider escalation has no reliable timeline, communicate a checkpoint rather than inventing one.

### Avoid public blame while staying honest

Customers deserve truthful information, but public blame rarely helps. Say that the issue depends on a partner or provider when approved and useful, but keep focus on impact, current action, and next update.

Do not expose contract details, provider defects, private communications, or unverified claims. If legal, security, payment, or regulated services are involved, use approved language.

### Maintain contingency options

Look for safe alternatives: manual processing, alternate carrier, alternate payment method, temporary feature flag, account-level workaround, delayed fulfillment, customer self-service step, or escalation to an internal owner.

Every workaround needs a risk check. Manual fixes can create duplicate charges, privacy exposure, inconsistent records, or unsupported commitments.

### Keep a single customer timeline

Vendor issues often split across internal tickets, provider cases, customer tickets, and account-team notes. Maintain one customer-facing timeline that captures reports, evidence sent, provider responses, promised updates, workarounds, and final resolution.

The customer should not have to reconstruct the history across teams.

### Close with confirmation and learning

When the provider resolves the issue, confirm whether the customer's outcome is fixed. Then capture vendor response quality, escalation gaps, missing evidence, repeated defects, and contract or tooling improvements.

Recurring vendor issues should become operational risk signals, not isolated support annoyances.

### Escalate business-critical dependency risk

Some vendor issues are ordinary support dependencies, while others threaten revenue recognition, regulated commitments, customer safety, enterprise SLAs, data integrity, or launch readiness. Mark cases that have business-critical consequences and route them through the appropriate operational owner, not only the normal vendor ticket queue.

Support should also identify whether the vendor issue reveals a single point of failure. If there is no backup provider, manual process, or customer workaround for a critical dependency, the support record should make that risk visible to operations, product, procurement, or leadership.

### Respect vendor communication boundaries

Provider contracts may restrict what can be shared publicly, what data can be sent, or who can contact the provider. Follow approved channels even when a customer is pressuring for speed. If an exception is needed, escalate rather than bypassing contract, security, or privacy controls.

## Common Traps

- Deflecting to the vendor even when the customer bought the experience from your organization.
- Promising a vendor timeline that has not been confirmed.
- Escalating with incomplete evidence and then making the customer collect more later.
- Naming or blaming the provider publicly without approval.
- Losing track of provider case IDs, response times, and next checkpoints; offering manual workarounds without checking duplicate, privacy, billing, or data-integrity risk
- Letting internal, vendor, and customer timelines diverge; closing the customer case when the vendor responds, not when customer impact is resolved
- Ignoring recurring provider failures because each case looks small; failing to flag vendor issues that threaten safety, regulated commitments, enterprise SLAs, revenue, or data integrity
- Bypassing approved vendor channels and data-sharing limits under customer pressure; forgetting to update macros, routing, or escalation packets after learning what evidence the provider needs

## Self-Check

- Is customer ownership clear even where vendor control is limited?
- Are provider responsibilities, internal responsibilities, and customer next steps separated?
- Does the escalation packet include timestamps, IDs, logs, screenshots, errors, affected accounts, environment, and reproduction details as needed?
- Are provider channel, SLA, escalation level, owner, case ID, and next checkpoint tracked?
- Is vendor language truthful but not speculative, contractual, or publicly blameful?
- Have workarounds been checked for duplicate actions, privacy, billing, data, and commitment risk?
- Is there one customer-facing timeline across internal and vendor records?
- Are promised customer updates tracked while waiting on the provider?
- Is customer recovery confirmed after provider resolution?; are recurring provider issues fed into operations, contract review, tooling, or escalation improvements?
- Has business-critical dependency risk been routed beyond the ordinary vendor ticket if needed?; are vendor communication, contract, security, and privacy boundaries respected?
