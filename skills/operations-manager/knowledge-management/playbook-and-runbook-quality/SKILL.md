---
name: playbook-and-runbook-quality.md
description: Use when the agent is writing, reviewing, testing, or improving operations playbooks, runbooks, SOPs, standard work, incident procedures, escalation scripts, recovery guides, or repeatable operating instructions for frontline or support teams.
---

# Playbook And Runbook Quality

Playbooks and runbooks are used when consistency matters, often under time pressure. A weak runbook can look complete because it has many steps, but still fail when ownership, prerequisites, decision points, exceptions, evidence, and stop conditions are unclear. This skill helps the agent judge whether operational instructions are executable, safe, and maintainable rather than merely well formatted.

## Core Rules

### Define the operating scenario precisely

A runbook should state what situation it covers, what it does not cover, who should use it, what authority the user has, and when to escalate instead of continuing. Vague scope leads staff to apply the procedure to cases it was not designed to handle.

Include trigger conditions, severity, affected service, expected inputs, required tools, required access, and dependencies. If the procedure is only valid for a region, system version, customer segment, site, shift, or policy period, say so.

### Start with prerequisites and safety checks

Before action steps, identify prerequisites: permissions, system access, materials, data, approvals, communication channel, backup owner, business hours, safety conditions, and known constraints. For high-risk work, include stop conditions before the first irreversible action.

Runbooks should make it hard to skip safety, privacy, compliance, or customer-impact checks. If a user must choose between speed and control, the decision should be explicit and escalated.

### Write steps around decisions, not just actions

Good runbooks include decision points: if this evidence is present, do one path; if not, use another. They should state what evidence confirms success, what signals failure, and what the user should do when results are ambiguous.

Avoid long linear instructions that assume the happy path. Operations work often includes missing information, partial failures, system latency, requester confusion, vendor delays, and priority conflicts.

### Make ownership visible at each handoff

Every handoff should name the receiving role, required context, response expectation, and source of truth. "Notify the team" is too vague. The runbook should tell the user who owns the next action and what information must travel with the handoff.

If multiple teams are involved, include a coordination point so work does not split into parallel, conflicting actions. Incident, vendor, customer, finance, HR, IT, security, and facilities handoffs often need different evidence and communication style.

### Include communication guidance

Operational runbooks should cover what to tell affected users, leaders, vendors, customers, and internal teams. Communication should separate confirmed facts from assumptions, include current impact, next action, owner, timing, and next update, and avoid overpromising resolution.

For sensitive issues, include privacy and access boundaries. A runbook that tells staff to forward screenshots, customer data, payroll details, security findings, or employee information without limits creates secondary risk.

### Define rollback, recovery, and stop conditions

For actions that change systems, schedules, staffing, inventory, customer commitments, or physical operations, define rollback and recovery steps. If rollback is impossible, define approval before proceeding and the residual risk owner.

Stop conditions matter. Staff need to know when to pause, escalate, preserve evidence, call a specialist, notify leadership, or switch to business-continuity mode. Without stop conditions, agents may continue executing a procedure after it no longer fits the facts.

### Test the runbook with realistic users

Reviewing a runbook at a desk is not enough. Test it with someone close to the intended user, using realistic inputs, missing data, time pressure, and access constraints. Observe where they hesitate, ask clarifying questions, use side knowledge, or leave the runbook.

Update the runbook based on the test, not only on expert opinion. Experts often forget what novices cannot infer.

### Keep runbooks tied to change control

Runbooks must change when systems, forms, vendor contacts, policies, approvals, service levels, data fields, tooling, staffing models, or incident lessons change. Define owner, review trigger, effective date, and version history.

Do not let old screenshots, links, escalation contacts, or approval paths remain because the core text still looks correct. Operational failure often comes from small stale details.

## Common Traps

- Writing a procedure for the happy path while ignoring ambiguity, missing data, and partial failure.
- Treating many steps as proof of quality.
- Omitting prerequisites, permissions, tools, materials, access, or safety checks.
- Using vague handoffs such as "contact support" or "notify management."
- Failing to distinguish confirmed facts from assumptions in communication templates.
- Including sensitive information flows without privacy or access boundaries.
- Continuing a procedure after stop conditions should have triggered escalation; forgetting rollback or recovery for irreversible or customer-impacting actions
- Reviewing runbooks only with experts who already know the missing context; leaving stale links, contacts, screenshots, system names, and approval paths in place after process changes

## Self-Check

- Does the runbook state scenario, exclusions, intended user, authority, triggers, and escalation boundary?
- Are prerequisites, permissions, tools, materials, data, approvals, access, and safety checks explicit?
- Are decision points based on evidence rather than implied judgment?
- Does the runbook cover ambiguous results, missing information, partial failures, latency, and vendor delay?
- Are handoffs tied to named roles, required context, response expectations, and source of truth?
- Does communication guidance include impact, owner, action, timing, confidence, next update, and privacy limits?
- Are rollback, recovery, stop, evidence-preservation, and business-continuity conditions defined where relevant?
- Has the runbook been tested with realistic users, inputs, access constraints, and time pressure?
- Are stale screenshots, links, contacts, forms, fields, approvals, and tool names reviewed?
- Is the runbook tied to change control, version history, owner, effective date, and review triggers?
