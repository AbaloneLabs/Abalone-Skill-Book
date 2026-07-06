---
name: shared-inbox-and-request-routing.md
description: Use when the agent is managing shared inboxes, group queues, internal service routing, assignment rules, triage ownership, email-to-ticket flows, queue hygiene, or request visibility across an operations service team.
---

# Shared Inbox And Request Routing

Shared inboxes and group queues are useful only when they have ownership, triage rules, and visibility. Without discipline, they become hiding places for aging work, duplicate responses, missed handoffs, and informal priority decisions. This skill helps the agent design shared request routing so internal work moves to the right owner quickly and remains visible until closed.

## Core Rules

### Define the purpose of each queue

Name what the inbox or queue is for, what requests belong there, what requests do not, and who monitors it. A shared inbox should not be a general dumping ground for every unclear task.

If multiple channels exist, define which is source of truth. Email, chat, forms, and tickets should not create parallel untracked work.

### Assign triage ownership and cadence

Every shared queue needs a triage role, backup, coverage hours, and review cadence. Define how quickly new items are acknowledged, categorized, assigned, or returned. The triage owner should have authority to route work, not just observe it.

Coverage should include absences, holidays, shift handoffs, and peak periods. Shared ownership without duty ownership creates missed work.

### Make assignment rules explicit

Routing may depend on service type, priority, skill, region, customer group, sensitivity, system, asset, location, or workload. Define rules and exceptions. If manual judgment is needed, document the criteria.

Avoid assigning work only to whoever responds first. That rewards constant monitoring and can create uneven workload.

### Manage handoffs between queues

Requests often move from one internal service queue to another. Define when handoff is allowed, what context must transfer, who tells the requester, and whether the SLA clock resets, pauses, or continues. A handoff should not erase prior history or make the requester repeat evidence.

If misroutes are common, fix the intake or routing rules rather than normalizing queue-to-queue forwarding.

### Prevent duplicates and conflicting responses

Shared inboxes often create duplicate handling when multiple people answer the same request. Use ticket IDs, claim status, internal notes, response ownership, and source-of-truth records. Define when a thread becomes a ticket and when side conversations must be pulled back into the system.

For sensitive issues, restrict visibility and avoid forwarding long threads with unnecessary personal or customer data.

### Monitor aging and stuck work

Track new, unassigned, in-progress, waiting-on-requester, waiting-on-third-party, escalated, and closed states. Aging rules should trigger review before requests are forgotten. Waiting states need owner and next checkpoint.

Do not let "waiting" become a place where work disappears. The service team still owns follow-up or closure criteria.

### Manage queue hygiene

Close duplicates, merge related requests, correct misroutes, archive noise, and update request categories. Keep templates, tags, automations, and routing rules current. Poor queue hygiene distorts workload and service metrics.

If the queue is consistently noisy, improve intake rather than asking triage to compensate forever.

### Plan for surge and absence coverage

Shared queues fail during volume spikes, vacations, turnover, or incidents when everyone assumes someone else is watching. Define surge playbooks, temporary assignment rules, daily backlog review, and leadership escalation when the queue exceeds capacity. Coverage should be visible, not assumed.

### Protect metrics from inbox behavior and design routing for confidentiality and authority

Response time, resolution time, backlog, and SLA metrics depend on consistent status use. Define start and stop rules, pause rules, reopen handling, and what counts as resolution. Otherwise teams can look good while requesters wait.

Metrics should reflect requester experience, not only internal queue movement.

Not every shared queue member should see every request. Payroll, HR, medical, security, legal, disciplinary, financial, access, and customer-sensitive requests may need restricted routing, private notes, named approvers, and limited forwarding. Define who can view, assign, approve, and communicate on sensitive categories.

Authority matters as much as visibility. A triage owner who can see a request but cannot approve, reject, or escalate it must have a clear handoff path to someone who can make the decision.

## Common Traps

- Treating a shared inbox as owned because many people can see it.
- Letting chat, email, and tickets become competing sources of truth.
- Assigning work by fastest responder rather than clear routing criteria.
- Allowing multiple people to answer the same request inconsistently.
- Passing requests between queues without preserving history, owner, and SLA rules.
- Letting waiting states age without owner or checkpoint.
- Assuming normal shared inbox coverage during surges, vacations, or incidents; keeping noisy categories and outdated templates that distort metrics
- Measuring internal closure while the requester still lacks the outcome; giving broad queue visibility to sensitive requests without access rules or decision authority

## Self-Check

- Is the purpose, scope, exclusion, monitor, and source of truth clear for each shared queue?
- Are triage owner, backup, coverage hours, review cadence, and acknowledgement rules defined?
- Are assignment rules based on service type, priority, skill, region, sensitivity, workload, or other explicit criteria?
- Are duplicates, claim status, internal notes, thread-to-ticket conversion, and response ownership controlled?
- Are queue-to-queue handoffs governed by context, requester update, owner, and SLA treatment?
- Are sensitive threads restricted and unnecessary personal or customer data minimized?
- Are aging rules defined for new, unassigned, waiting, escalated, and stuck work?
- Is queue hygiene maintained through dedupe, merge, misroute correction, tag updates, and template review?
- Are surge, absence, turnover, and incident coverage rules defined for the shared queue?
- Do metrics reflect requester experience with clear start, stop, pause, reopen, and resolution rules?; are sensitive categories routed with appropriate visibility, approver authority, private notes, and forwarding limits?
