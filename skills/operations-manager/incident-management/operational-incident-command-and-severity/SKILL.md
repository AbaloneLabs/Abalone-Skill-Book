---
name: operational-incident-command-and-severity.md
description: Use when the agent is setting up incident command, assigning incident roles, deciding severity levels, coordinating cross-functional response, structuring command cadence, managing decision authority, or reviewing whether an operational incident has the right leadership, escalation, and control structure.
---

# Operational Incident Command And Severity

Incident command gives a disrupted operation a temporary decision system. Without it, teams duplicate work, chase symptoms, debate severity, give inconsistent updates, and leave critical decisions unowned. Agents often recommend escalation or a status meeting without defining who commands, who decides, who communicates, and how severity changes. This skill helps the agent structure operational incident response so authority, cadence, and accountability are clear.

## Core Rules

### Establish A Single Incident Lead

Name one incident lead or commander responsible for coordination, decision cadence, role assignment, and escalation. The lead does not need to solve every technical or operational detail, but they must keep the response coherent.

Avoid co-command unless roles are explicitly separated. Two leaders giving instructions can create conflicting priorities, duplicate outreach, and unsafe workarounds.

### Assign Core Incident Roles

Define who owns operations recovery, customer or stakeholder communication, evidence and timeline logging, frontline instructions, vendor coordination, risk or compliance review, executive updates, and post-incident handoff. Smaller incidents may combine roles, but the responsibilities still need names.

Role assignment should reflect authority and expertise. Do not make the person with the most context carry every role if they also need to diagnose or execute recovery.

### Use Severity Criteria, Not Mood

Severity should be based on impact, urgency, risk, spread, customer or internal harm, compliance exposure, safety, financial impact, operational blockage, and reversibility. Define criteria enough that teams can classify consistently.

Severity should be revisited. Downgrading is not failure if evidence improves; upgrading is not embarrassment if impact grows. Record the reason for severity changes.

### Define Decision Rights

Incident response often requires decisions that normal process does not: pause work, override routing, approve overtime, use vendor emergency support, issue customer remediation, trigger business continuity, notify regulators, or accept residual risk. Name who can make each decision.

Do not let urgency expand authority silently. High-risk decisions involving legal, privacy, safety, finance, employment, contractual terms, or external statements need the right owner.

### Set A Command Cadence

Create a cadence for standups, updates, decision checkpoints, and handoffs. The cadence should match severity and pace of change. Severe active incidents may need frequent check-ins; stabilized incidents may need scheduled updates.

Each cadence point should answer: what changed, what is still unknown, what decisions are needed, what is blocked, what changed for stakeholders, and when the next update occurs.

### Maintain A Decision And Timeline Log

Track incident start, detection, severity changes, containment, key decisions, customer or internal impacts, communications, vendor actions, recovery milestones, and open risks. The log should distinguish facts from assumptions and be stored where the response team can use it.

The log is not bureaucracy. It prevents repeated questions, supports handoffs, and becomes the source for recovery review, customer communication, and corrective action.

### Manage Cross-Functional Boundaries

Operational incidents often involve support, product, engineering, facilities, finance, compliance, legal, security, vendors, and customer-facing teams. Name each function's role and expected output. If a function is consulted only for awareness, say so.

Avoid broad meetings where many people listen but no one owns decisions. Bring the right people for the decision at hand.

### Protect Responders And Capacity

Incident command should control workload. Decide who is pulled into response, who maintains normal operations, whether shifts or on-call need relief, and when responders need rest. Long incidents create fatigue and mistakes.

Do not rely on the same experts indefinitely. Build handoffs and backups as the incident extends.

### Define Exit From Incident Command

Incident command should end when the operation is stable enough for normal ownership or a recovery workstream. Define exit criteria: harm contained, service restored or workaround stable, owners assigned, communication complete, residual risks accepted, and post-incident review scheduled.

Closing command too early leaves recovery fragmented. Keeping command too long consumes capacity and muddies accountability.

## Common Traps

- Calling a meeting an incident response without assigning a single incident lead.
- Letting the most knowledgeable person become commander, communicator, investigator, and executor at once.
- Setting severity based on anxiety, executive attention, or customer volume alone.
- Failing to revisit severity as impact evidence changes.
- Making high-risk decisions outside the authority of operations because the incident feels urgent.
- Running status meetings that do not produce decisions, owners, or next update times.
- Keeping timeline and decisions scattered across chat threads; inviting many functions without clarifying what decision or output is needed from them
- Burning out responders by treating long incidents as continuous emergency mode; closing incident command before recovery ownership, communication, and residual risk are clear

## Self-Check

- Is there one incident lead responsible for command, cadence, escalation, and coherence?
- Are core roles assigned for recovery, communication, logging, vendor coordination, frontline direction, and risk review?
- Are severity criteria based on impact, urgency, risk, spread, and reversibility rather than mood?
- Are severity changes recorded with reasons?
- Are decision rights clear for pauses, overrides, overtime, remediation, continuity, and high-risk external actions?
- Does the command cadence produce changed facts, decisions needed, blockers, owners, and next update time?
- Is there a usable incident timeline and decision log?
- Are cross-functional participants tied to specific responsibilities or decisions?
- Has responder fatigue and normal-operation capacity been managed?
- Are exit criteria from incident command defined before closure?
