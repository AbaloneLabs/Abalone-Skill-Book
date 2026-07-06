---
name: support-tool-outage-and-manual-workaround.md
description: Use when the agent is handling support tool outages, degraded ticketing systems, CRM unavailability, chat platform failures, phone system issues, knowledge base outages, manual tracking, delayed updates, offline workarounds, or recovery after tool restoration where risks include lost customer requests, duplicate work, privacy exposure, weak audit trails, missed SLAs, or unsafe manual promises.
---

# Support Tool Outage And Manual Workaround

Support tool outages change the risk profile of customer service immediately. When ticketing, CRM, chat, phones, billing tools, identity tools, or knowledge bases are unavailable, agents may improvise in spreadsheets, personal notes, side chats, or memory. That can preserve service for a short time, but it can also lose requests, expose private data, break audit trails, and create promises that cannot be reconciled later. This skill helps the agent work during tool disruption without losing control.

Use this skill when support systems are down, degraded, slow, partially unavailable, or recovering. The agent should keep service moving only within safe manual boundaries and should plan reconciliation from the start.

## Core Rules

### Identify which capability is actually degraded

An outage may affect intake, assignment, customer lookup, identity verification, billing action, public replies, internal notes, knowledge access, file attachments, macros, phone routing, chat availability, analytics, or escalation. Do not treat all tool outages the same.

Name what can still be done safely and what must pause. If identity verification, account status, or billing records cannot be checked, some requests should not proceed even if the customer is waiting.

### Switch to approved manual processes

Use approved fallback tools, templates, secure storage, shared trackers, communication channels, and escalation routes. If no approved manual process exists, keep the workaround minimal and route to support operations or leadership.

Do not create ad hoc customer lists with sensitive data in personal documents, unapproved spreadsheets, or public chat. A manual workaround should be temporary, access-controlled, and reconcilable.

### Reduce scope during degraded operations

During tool failure, prioritize urgent, high-risk, time-sensitive, and low-dependency work. Defer actions that require unavailable systems, detailed account review, high-risk identity decisions, refunds, legal requests, or irreversible account changes unless an approved fallback exists.

Tell customers what can and cannot be done. It is better to set a conservative expectation than to make a manual promise the team cannot fulfill after recovery.

### Preserve minimal audit evidence

When systems are unavailable, capture only the information needed to reconnect the customer request later: contact method, customer identifier allowed by policy, time, issue category, urgency, promised follow-up, and owner. Avoid collecting excessive sensitive details in temporary trackers.

Record where the temporary evidence lives, who can access it, and how it will be deleted or archived after reconciliation. Manual records can become privacy liabilities if left behind.

### Communicate status without overexplaining internal failure

Customers need to know whether service is delayed, what action they should take, and when they will hear back. They do not need a detailed internal systems diagnosis. Use approved outage language and avoid blaming vendors or internal teams.

If the outage affects many customers, coordinate with incident communication owners. Frontline agents should not invent global status updates.

### Manage SLA and queue impact transparently

Tool outages distort response time, backlog, and productivity metrics. Track the outage window, affected queues, work paused, manual contacts, and expected recovery load. This prevents teams from interpreting degraded metrics as normal performance.

After restoration, plan backlog triage rather than simply releasing all held work at once. High-risk and oldest commitments may need priority.

### Reconcile after restoration

Recovery is not complete when the tool comes back. Enter or upload manual records, create missing tickets, update customer conversations, attach approved notes, close duplicates, remove temporary access, and delete unneeded temporary files according to policy.

Check for promises made during the outage. The team must know which customers were told to expect follow-up, refund, escalation, callback, or documentation.

### Review the fallback process

After the outage, inspect what worked and what failed. Did agents know the fallback? Was sensitive data protected? Were customers lost? Were managers able to see backlog? Did reconciliation take too long? Were there tool dependencies no one understood?

Use the findings to improve continuity plans, training, templates, permissions, and manual tracking.

## Common Traps

- Continuing normal support behavior even though verification or account data is unavailable.
- Creating unapproved spreadsheets or personal notes containing sensitive customer data.
- Promising refunds, access changes, or technical fixes that cannot be verified during the outage.
- Failing to track manual contacts, making reconciliation impossible.
- Giving customers detailed internal outage explanations that confuse or expose unnecessary information.
- Treating tool restoration as the end of the incident while manual records remain unreconciled.
- Letting temporary trackers persist after recovery with unnecessary personal data.
- Ignoring outage effects on SLA, backlog, and quality metrics.

## Self-Check

- Is the degraded capability identified precisely?
- Are unsafe actions paused when required data or verification tools are unavailable?
- Is the manual process approved, access-controlled, temporary, and reconcilable?
- Is the team collecting only minimal information needed for recovery?
- Are customers given clear expectations without excessive internal detail?
- Are SLA, queue, backlog, and manual workload impacts tracked?
- Is there an owner for recovery reconciliation?
- Are missing tickets, promised follow-ups, duplicates, and manual notes reconciled after restoration?
- Are temporary files or trackers deleted or archived according to policy?
- Has the outage produced improvements to fallback process, training, permissions, or tooling?
