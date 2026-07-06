---
name: incident-watchlist-and-affected-customer-tracking.md
description: Use when the agent is creating or reviewing incident watchlists, affected customer lists, ticket tags, impact tracking, proactive outreach lists, escalation logs, customer promise tracking, or operational records during outages, bugs, degraded service, data incidents, fulfillment incidents, or other customer-impacting events.
---

# Incident Watchlist And Affected Customer Tracking

Incident tracking is not clerical bookkeeping. It decides which customers receive updates, which promises must be kept, which harms are understood, and which patterns reach the incident team. Poor tracking misses silent impact, duplicates work, exposes private information, or leaves customers without follow-up after the technical issue is fixed. This skill helps the agent build watchlists that support reliable incident response.

## Core Rules

### Define the affected-customer criteria

Before collecting names, define what makes a customer affected: failed workflow, exposed error, time window, region, plan, device, integration, order state, payment attempt, data object, account permission, or support report. Separate confirmed affected, likely affected, possibly affected, and explicitly unaffected groups.

Without criteria, the list becomes a mix of guesses, loud complaints, and duplicate tickets. Criteria should change when the incident scope changes, but old assumptions must be recorded.

### Use multiple signals

Inbound tickets rarely show full impact. Combine support contacts, telemetry, status checks, incident logs, billing events, order events, monitoring alerts, social reports, customer success notes, community posts, and account manager reports when available.

Track the source and confidence for each entry. A high-value customer report may be urgent, but it is not automatically representative of all impact.

### Preserve time windows and state

Incidents evolve. Record when the customer was affected, what action failed, whether they retried, whether a workaround was used, whether the issue is still active, and whether recovery was confirmed.

For transactional issues, state matters: payment authorized but not captured, order created but not fulfilled, cancellation requested but not processed, export generated but incomplete, or message queued but not delivered. These differences shape follow-up and remediation.

### Track promises and ownership

Every customer promise should have an owner, due date, channel, and status. Promises include next update, refund review, manual fix, data restoration, engineering review, account-manager contact, or executive response.

Do not let incident closure erase customer obligations. A technical resolution may still require refunds, credits, replacement orders, data correction, complaint follow-up, or proof of recovery.

### Protect privacy and access

Incident lists often contain account identifiers, contact details, order IDs, payment issues, medical or safety context, legal complaints, or security-sensitive facts. Limit fields to what responders need, restrict access, and avoid copying sensitive customer data into uncontrolled spreadsheets or chat threads.

When lists must be shared across teams or vendors, redact unnecessary details and use approved systems.

### Keep the watchlist operational

The list should help teams act. Include priority, segment, impact type, current status, next action, owner, last update, and escalation flag. Remove duplicates and link related tickets.

If the list grows large, group by impact pattern and priority. A raw pile of ticket links is hard to use during a live response.

### Reconcile before closing

Before incident closeout, compare the watchlist against known impact signals. Identify customers who need proactive outreach, customers who recovered without contact, customers awaiting a promise, and customers whose status remains uncertain.

Closure should include a plan for orphaned cases and delayed follow-up.

### Use lifecycle states instead of loose notes

Every tracked customer should move through defined states such as reported, under review, confirmed affected, workaround provided, waiting on customer, waiting on internal team, waiting on vendor, recovered, remediation pending, follow-up sent, or closed. The exact labels can differ by organization, but the meaning must be stable.

Loose notes like "checking," "handled," or "probably fine" are hard to audit during a live incident. Lifecycle states help teams see bottlenecks, count unresolved impact, and avoid double outreach. They also make shift handoff safer because the next agent can tell what is truly done.

### Retain evidence for later review

Some incident lists become evidence for SLA credits, refunds, regulatory questions, legal complaints, customer success reviews, or product postmortems. Preserve the basis for affected status and remediation decisions. Do not keep more private data than needed, but do keep enough to explain why a customer was included, excluded, credited, or contacted.

## Common Traps

- Treating the loudest tickets as the full affected population.
- Mixing confirmed, likely, possible, and unaffected customers without labels.
- Losing the time window that explains why one customer is affected and another is not.
- Tracking customer names but not the failed workflow or current recovery state.
- Forgetting promises made during the incident once service is restored; copying sensitive customer, payment, security, or legal data into open documents
- Creating duplicate entries across tickets, account-manager notes, and incident chats; leaving no owner for proactive outreach or manual remediation
- Keeping an incident watchlist that is too unstructured to drive action; using vague states such as "handled" without showing whether recovery, remediation, and follow-up are complete
- Deleting or failing to preserve the evidence needed for later SLA, refund, audit, or postmortem review; closing the incident without reconciling unresolved customer cases

## Self-Check

- Are affected, likely affected, possibly affected, and unaffected criteria defined?
- Are time windows, regions, plans, workflows, integrations, and customer states captured?
- Does the watchlist combine support contacts with telemetry, logs, billing or order events, social reports, customer success, and account-team signals where available?
- Is each entry labeled with source and confidence?
- Are failed actions, retry status, workaround use, recovery confirmation, and current state recorded?
- Does every customer promise have an owner, due date, channel, and status?
- Are privacy and access limits appropriate for the data being tracked?
- Are duplicates merged or linked?
- Can the list drive prioritization, escalation, outreach, and remediation?; was the watchlist reconciled before incident closure?
- Does each customer have a clear lifecycle state rather than a vague note?; is enough evidence retained to justify affected status, exclusion, remediation, and follow-up decisions?
