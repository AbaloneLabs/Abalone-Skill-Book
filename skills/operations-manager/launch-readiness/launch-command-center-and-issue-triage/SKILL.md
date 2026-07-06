---
name: launch-command-center-and-issue-triage.md
description: Use when the agent is setting up or operating a launch command center, war room, issue triage process, launch monitoring cadence, incident routing, severity model, decision log, status reporting, or cross-functional launch response.
---

# Launch Command Center And Issue Triage

A launch command center is not valuable because many people are in one channel. It is valuable when signals are detected early, issues are classified consistently, owners are assigned quickly, and decisions are recorded. Agents often design a launch room as a communication hub but miss triage criteria, escalation authority, noise control, customer impact tracking, and fatigue management. This skill helps the agent run launch response as an operational control.

## Core Rules

### Define the command center mission and window

State what the command center is responsible for: launch monitoring, issue triage, customer-impact response, operational coordination, decision logging, escalation, rollback recommendation, or stakeholder updates. Also state what it does not own, so normal teams do not dump unrelated work into the launch channel.

Define start time, end time, coverage hours, handoff plan, time zones, and criteria for extending or closing the command center. A launch room that never formally closes becomes a noisy substitute for normal operations.

### Establish roles before issues arrive

Typical roles include command lead, operations lead, product or engineering lead, support lead, communications owner, customer-impact owner, data or dashboard owner, vendor contact, decision logger, and executive liaison. The exact roles depend on launch risk, but each role needs authority and backup.

Avoid inviting everyone without role clarity. A crowded channel can slow decisions when no one knows who owns triage.

### Use a severity and priority model

Define severity by customer impact, employee impact, safety, compliance, revenue, service interruption, data integrity, reputational risk, volume affected, and workaround availability. Priority should reflect urgency and response order. Do not let the loudest stakeholder set severity.

Predefine examples for launch-specific issues: blocked onboarding, incorrect eligibility, payment failure, fulfillment delay, high support contact rate, missing inventory, broken workflow, inaccurate reporting, or harmful messaging.

### Create a single issue record

Every launch issue should have one source of truth with issue ID, description, detection source, severity, impact, owner, status, next action, decision needed, workaround, customer communication, timestamps, and closure evidence. Chat is useful for coordination, but it should not be the only record.

Duplicate issue reports should be linked, not handled independently. Conflicting updates create confusion and waste response capacity.

### Triage signal from noise

Launch periods create many signals: customer complaints, internal questions, dashboard anomalies, social posts, sales escalations, vendor notices, system alerts, and anecdotal reports. Define what qualifies as an issue, what needs monitoring, and what belongs to normal operations.

Do not dismiss anecdotes too quickly, but do require enough evidence to determine impact and owner. Early weak signals can matter; unfiltered noise can paralyze the team.

### Set status cadence and decision rules

Define how often the command center reviews issues, who attends, what format is used, when leadership is updated, and what decisions require escalation. Status updates should include current impact, trend, owner, next action, blocker, risk, ETA confidence, and decision ask.

Avoid meetings that only read ticket lists. Use the cadence to make decisions, remove blockers, and align communication.

### Coordinate customer and internal communication

Issue triage must connect to communication. Decide who can message customers, frontline teams, sales, support, vendors, executives, and affected internal groups. Communication should separate confirmed facts from hypotheses and avoid promising fixes before response owners confirm them.

If the issue affects safety, privacy, legal, contractual, or regulated obligations, route communication through the right owner before broad distribution.

### Manage fatigue and transition to steady state

Launch response can burn out key people. Define shift coverage, backup decision makers, rest windows, and handoff notes. As issue volume stabilizes, move work into normal incident, support, defect, vendor, or process-management channels.

Closure should include open issues transferred, owners confirmed, monitoring adjusted, communication archived, and retrospective inputs captured.

## Common Traps

- Creating a launch room without mission, scope, closure criteria, or decision authority.
- Inviting many stakeholders but assigning no triage owner or decision logger.
- Letting seniority, anxiety, or customer visibility override severity criteria.
- Handling launch issues only in chat with no issue record or closure evidence.
- Treating duplicate reports as separate incidents instead of linking them.
- Dismissing early weak signals because they are anecdotal, or chasing every anecdote as a crisis.
- Running status meetings that produce updates but no decisions; allowing unapproved customer or executive communication to outrun facts
- Keeping the command center open so long that normal ownership never resumes; ignoring fatigue, time zones, shift handoffs, and backup authority

## Self-Check

- Is the command center mission, scope, start, end, coverage, extension, and closure criteria clear?
- Are command, operations, product, support, communication, customer-impact, data, vendor, logging, and executive-liaison roles assigned with backups where needed?
- Is severity based on impact, safety, compliance, revenue, data, reputation, volume, and workaround availability?
- Are launch-specific severity examples defined?
- Does every issue have a source-of-truth record with ID, impact, owner, status, action, workaround, communication, timestamps, and closure evidence?
- Are duplicate reports linked and conflicting updates controlled?
- Are issue, monitor-only signal, and normal-operation work distinguished?
- Does the status cadence drive decisions, blocker removal, escalation, and communication alignment?
- Are customer, frontline, vendor, executive, and sensitive communications owned and fact-bounded?
- Are fatigue coverage, handoffs, steady-state transition, and retrospective capture planned?
