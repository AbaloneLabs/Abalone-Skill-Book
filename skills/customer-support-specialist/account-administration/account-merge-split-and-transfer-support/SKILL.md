---
name: account-merge-split-and-transfer-support.md
description: Use when the agent is handling account merge, split, workspace transfer, tenant consolidation, account ownership transfer, organization migration, duplicate account cleanup, data movement, domain reassignment, or customer requests to combine or separate accounts where risks include data loss, unauthorized access, billing errors, broken integrations, audit trail confusion, privacy exposure, or irreversible changes made without stakeholder approval.
---

# Account Merge Split And Transfer Support

Merging, splitting, or transferring accounts is high-risk support work because it changes data boundaries. The request may sound administrative, but it can affect access, billing, ownership, integrations, audit history, subscriptions, legal entities, and privacy obligations. Agents often try to help quickly and miss that the requester may not have authority over both sides of the change. This skill helps the agent handle account structure changes conservatively and traceably.

Use this skill when customers ask to merge duplicate accounts, split a workspace, transfer ownership, move a tenant, consolidate organizations, reassign domains, migrate data, or clean up account structure after acquisition, rebrand, employee departure, or setup mistake. The agent should verify authority and clarify what can and cannot move.

## Core Rules

### Define the structural change precisely

Clarify whether the customer wants merge, split, transfer, ownership change, domain reassignment, workspace move, data export/import, user migration, billing consolidation, or duplicate cleanup. These are different operations with different risk.

Do not use "merge accounts" loosely. A merge may mean one login, one invoice, shared data, tenant consolidation, or just removing duplicates. Ask what outcome the customer expects before advising.

### Verify authority over every affected account

The requester must be authorized for each account, workspace, tenant, billing entity, or domain involved. Authority over one side is not enough. If the change touches another organization, subsidiary, former employer, reseller, or customer-owned tenant, follow the required approval path.

Be cautious with employee departures, company acquisitions, agency-client relationships, and disputed ownership. These cases often require additional verification, legal, privacy, or account management review.

### Explain what transfers and what does not

Account structure changes may or may not move users, files, settings, billing history, audit logs, integrations, API keys, custom fields, permissions, comments, analytics, subscriptions, support history, or deleted data. The agent should explain limitations before the customer approves.

Do not imply a seamless merge if the product only supports partial migration or manual reconstruction. Hidden limitations create support escalations after irreversible steps.

### Assess data privacy and visibility impact

Merging or moving accounts can expose data to new admins or users. Splitting accounts can remove visibility or break shared workflows. Domain reassignment can affect who can claim users. Review whether private, regulated, customer, employee, or third-party data could cross boundaries.

If data from one legal entity or customer organization will become visible to another, route to privacy, legal, or the appropriate governance owner.

### Check billing and subscription consequences

Account transfers can affect invoices, credits, renewal dates, plan tiers, seat counts, tax entity, discounts, entitlements, payment methods, and contract obligations. Billing history may not move with product data. Confirm billing impact before change.

If billing ownership differs from admin ownership, both may need involvement. Do not let a technical transfer create an invoice dispute.

### Plan sequencing and rollback limits

Some account changes are irreversible or hard to undo. Define steps, order, owner, expected downtime, customer approvals, backup or export options, and rollback limits. If rollback is impossible, say so clearly and obtain the required confirmation.

For complex moves, staged migration or customer-led export/import may be safer than direct merge. The safest path depends on data, urgency, and product capability.

### Protect integrations and identity systems

Transfers can break SSO, SCIM, webhooks, API tokens, billing integrations, support integrations, embedded links, automations, and domain verification. Ask which systems depend on the current account structure.

Before moving a domain or workspace, check whether users could be locked out or automations could run under the wrong owner.

### Document final state and customer confirmation

Record original state, requested outcome, affected accounts, authority evidence, limitations explained, approvals, actions taken, data or billing impact, and final state. Provide the customer with a clear confirmation of what changed and what remains their responsibility.

This record protects future agents from guessing after a complex structural change.

## Common Traps

- Treating "merge my accounts" as a clear request without defining the desired outcome.
- Accepting authority from one account owner when multiple accounts or entities are affected.
- Moving data in a way that exposes it to new admins or legal entities.
- Promising that all history, permissions, integrations, or billing records will transfer.
- Ignoring subscription, credit, tax, renewal, or invoice consequences.
- Performing irreversible changes without explaining rollback limits.
- Breaking SSO, domain verification, API tokens, or automations during transfer.
- Failing to document the original state and final state after a complex account change.

## Self-Check

- Is the requested change clearly classified as merge, split, transfer, domain reassignment, migration, or duplicate cleanup?
- Is the desired customer outcome stated in practical terms?
- Is authority verified for every affected account, tenant, workspace, billing entity, and domain?
- Are employee departure, acquisition, agency, reseller, or ownership dispute risks checked?
- Has the agent explained what data, settings, users, history, integrations, and billing records will or will not move?
- Are privacy and cross-entity visibility impacts reviewed?
- Are billing, subscription, renewal, tax, credit, and entitlement impacts checked or routed?
- Are sequencing, downtime, backups, and rollback limits clear?
- Are SSO, SCIM, domains, API tokens, webhooks, and automations considered?
- Is the original state, authority evidence, action, limitation, and final state documented?
