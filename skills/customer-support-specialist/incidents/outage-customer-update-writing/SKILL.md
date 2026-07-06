---
name: outage-customer-update-writing.md
description: Use when the agent is writing or reviewing customer updates during outages, degraded service, incidents, status-page posts, support ticket replies, social updates, workaround notices, estimated restoration messages, or incident communications where uncertainty and customer impact must be handled carefully.
---

# Outage Customer Update Writing

Outage communication is not just explaining that something is broken. It shapes customer decisions while facts are incomplete, teams are under pressure, and public trust is fragile. Weak updates overstate certainty, hide the practical impact, create inconsistent expectations, or force customers to keep asking for basic information. This skill helps the agent write incident updates that are accurate, useful, and disciplined.

## Core Rules

### Lead with customer impact

Start from what customers experience: unavailable login, delayed orders, missing messages, failed payments, slow reports, broken exports, partial data visibility, or intermittent errors. Internal component names are useful only when they help customers decide what to do.

If the customer cannot tell whether they are affected, the update is incomplete. Name affected products, regions, channels, plans, time windows, and workflows when known. If scope is still being investigated, say that plainly and state what signal is being checked.

### Match certainty to evidence

Do not write as if the cause, affected scope, or restoration time is known unless the incident owner has confirmed it. Use separate language for "we are investigating," "we have identified," "we are applying a fix," "we are monitoring," and "resolved." Each status creates a different expectation.

Avoid false precision. An estimated restoration time should include confidence and a next-update commitment. If there is no reliable estimate, give the next update time and any safe customer action instead of inventing a deadline.

### Give the next useful action

Customers need to know whether to retry, wait, use a workaround, pause an operational process, contact their users, preserve evidence, or expect follow-up. Provide only workarounds that have been validated and are safe for the affected customer segment.

If a workaround has tradeoffs, explain them. A workaround that risks duplicate orders, duplicate charges, data loss, or security exposure can create a second incident.

### Keep channels consistent

Status page, tickets, chat, phone scripts, social posts, in-app banners, account manager notes, and executive updates should not tell different stories. If a public update must be shorter, it should still align with the same facts and next-update time.

Use a single source of truth and make clear who can approve new language. During incidents, well-meaning agents often copy stale updates because the latest guidance is scattered.

### Protect sensitive and speculative details

Do not disclose security-sensitive internal architecture, customer-specific exposure, vendor credentials, exploit details, or unconfirmed root cause. Avoid naming a third-party provider publicly unless approved and useful to customers.

Privacy and security incidents require extra caution. If customer data, account access, safety, payments, or legal obligations may be involved, coordinate with the incident owner, security, privacy, legal, or compliance before sending broad statements.

### Show ownership without overpromising

Acknowledge the disruption and own the communication. Do not hide behind passive language or blame another team. At the same time, do not promise compensation, timelines, root cause, data integrity, or permanent prevention until those decisions are approved.

Strong ownership means clear updates, honest limits, and follow-through. It does not mean making commitments the response team cannot keep.

### Update on a cadence

State when the next update will arrive, even if the investigation is still active. If there is no material change, say so and keep the cadence. Silence makes customers assume the issue is ignored.

For long incidents, vary updates based on customer usefulness: known impact, mitigations, remaining affected groups, expected next milestone, and whether customers should keep workarounds in place.

### Adapt by audience and consequence

Different customers use outage information for different decisions. An individual user may need to know whether to retry later. An administrator may need to warn employees. An enterprise owner may need impact records for an SLA review. A merchant may need to pause fulfillment. A regulated customer may need evidence for its own incident process.

Do not send every audience the same thin status sentence when their operational consequence differs. Keep facts consistent, but tailor the level of detail, channel, and next step. If account teams or customer success managers will add context, make sure their version does not drift from the approved incident facts.

## Common Traps

- Writing about internal systems while customers still cannot tell what is affected.
- Giving an estimated restoration time because customers are asking, not because evidence supports it.
- Saying "resolved" before monitoring confirms customer-facing recovery.
- Publishing one message publicly while ticket replies use different facts or timelines.
- Offering a workaround that can create duplicate transactions, lost data, or security risk.
- Over-apologizing without giving concrete next steps or next update timing; naming a vendor, root cause, or security detail before approval
- Treating intermittent degradation as minor because the service is not fully down; forgetting customers in different regions, plans, devices, or channels may see different impact
- Sending the same update to end users, admins, merchants, enterprise owners, and regulated customers despite different consequences; letting old incident macros remain in use after the status changes

## Self-Check

- Does the update describe customer-visible impact before internal cause?
- Are affected products, regions, channels, workflows, plans, and time windows stated when known?
- Is uncertainty labeled clearly without fake precision?
- Are investigation, identified, fixing, monitoring, and resolved states used accurately?
- Is any restoration estimate supported, or is there a clear next-update time instead?
- Are workarounds validated and safe for the affected segment?
- Are status page, ticket, social, chat, phone, in-app, and account-team messages aligned?
- Have sensitive security, privacy, vendor, and root-cause details been withheld unless approved?
- Does the message show ownership without unauthorized promises about compensation, prevention, or timelines?
- Is there a concrete cadence for the next customer update?; is the update tailored for audiences whose operational decisions differ while staying consistent with approved facts?
