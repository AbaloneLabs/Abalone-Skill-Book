---
name: operational-readiness-for-product-launch.md
description: Use when the agent is assessing operational readiness for a product, service, process, site, market, or feature launch, including staffing, capacity, support model, tooling, SOPs, controls, customer impact, vendor readiness, service levels, failure modes, and launch risk.
---

# Operational Readiness For Product Launch

Operational readiness for launch is not proof that a product or project is finished. It is proof that the operating system can absorb the change without uncontrolled service failure, confused ownership, hidden manual work, or unsupported promises. Agents often focus on a launch checklist and miss whether frontline teams, support queues, vendors, controls, reporting, and recovery paths are ready. This skill helps the agent evaluate launch readiness as an end-to-end operating commitment.

## Core Rules

### Define what is launching and what changes operationally

Name the launch object, affected customers or users, start date, rollout shape, locations, channels, systems, policies, vendors, and internal teams. Then identify what will be different after launch: new demand, new process steps, new exception types, new data, new controls, new service promises, new failure modes, or changed handoffs.

Do not accept "same as today plus volume" without testing the assumption. A small product change can create a large operational change if it affects eligibility, billing, identity, fulfillment, support, scheduling, or compliance.

### Translate launch assumptions into workload

Readiness depends on expected volume, contact rate, defect rate, manual intervention rate, escalation rate, cancellation or return rate, training need, vendor workload, and peak timing. Convert product, marketing, sales, or project assumptions into operational demand by queue, role, shift, location, and skill.

If assumptions are weak, state the range and define monitoring triggers. Launch plans often fail because demand uncertainty was treated as a single confident number.

### Validate process and handoff readiness

Map the launch workflow from customer or user trigger through intake, fulfillment, support, exception handling, escalation, reporting, and closure. Confirm owner, system of record, required data, decision rules, service levels, and handoff criteria at each step.

Pay special attention to edges between product, sales, marketing, customer support, operations, finance, legal, compliance, vendors, IT, and field teams. Launch defects often appear in the gaps where no single team owns the full path.

### Check staffing, skills, and coverage

Readiness requires enough trained people at the right time, not just nominal headcount. Check staffing plan, shift coverage, backup roles, surge support, training completion, access permissions, language or region coverage, manager coverage, and fatigue risk during launch week.

For launches with uncertain volume, define surge thresholds and who can authorize temporary staffing, overtime, queue deflection, scope reduction, or launch slowdown.

### Confirm tooling, data, and controls

Tools should be configured before launch: ticket categories, forms, routing, dashboards, macros, runbooks, access rights, monitoring alerts, fulfillment systems, inventory records, and reporting fields. Data should support triage, service levels, compliance, billing, and post-launch learning.

Controls should not be postponed unless the risk is accepted by the right owner. Manual workarounds need owner, duration, quality check, audit trail, and retirement plan.

### Prepare for known failure modes

Identify likely failures: bad eligibility, unclear messaging, volume spike, fulfillment delay, vendor miss, data mismatch, access failure, payment issue, customer confusion, quality defect, safety issue, or regulatory concern. Define detection signal, owner, response, escalation, communication, and stop condition.

Launch readiness is stronger when the team knows what would make them pause, slow, roll back, or change scope.

### Align service promises with operational capability

Marketing, sales, product, and leadership promises must match what operations can deliver. If the launch promises same-day service, custom handling, 24/7 support, immediate refunds, guaranteed delivery, or no downtime, check whether staffing, tooling, vendors, and controls support it.

If capability is not ready, revise the promise, narrow the audience, phase the rollout, or obtain explicit risk acceptance.

### Define readiness evidence and signoff

Readiness should be evidenced by completed tests, trained users, system configuration, volume model, runbooks, support paths, vendor confirmation, dashboard validation, control review, and open-risk log. Signoff should state what is ready, what is not ready, accepted risks, decision owner, and next review date.

Do not let a green project status hide unresolved operational risks. The launch decision should make tradeoffs visible.

## Common Traps

- Treating product completion as operational readiness.
- Accepting volume assumptions without translating them into queues, shifts, skills, and vendors.
- Ignoring manual intervention rates and exception demand.
- Mapping the happy path but not support, escalation, billing, cancellation, return, and failure paths.
- Counting headcount without checking training, access, coverage, backup roles, and fatigue.
- Launching with temporary workarounds but no owner, quality control, or retirement plan.
- Letting external promises exceed operational capability; treating unresolved risks as minor because each team owns only a small part
- Using signoff as ceremony instead of evidence-based risk acceptance; forgetting dashboards, categories, macros, and reporting fields until after launch

## Self-Check

- Is the launch object and operational change clearly defined by customer, user, channel, site, system, process, vendor, and policy impact?
- Are volume, contact, defect, manual work, escalation, return, cancellation, and peak assumptions translated into workload by queue, role, shift, and skill?
- Are uncertain assumptions expressed as ranges with monitoring triggers?
- Does the workflow cover intake, fulfillment, support, exceptions, escalation, reporting, and closure?
- Are owners, systems of record, data needs, service levels, and handoff criteria defined at each step?
- Are staffing, training, access, surge support, shift coverage, region or language needs, and fatigue risks addressed?
- Are tools, forms, routing, dashboards, macros, runbooks, permissions, inventory, and reporting fields ready?
- Are known failure modes tied to detection, owner, response, communication, escalation, and stop condition?
- Do service promises match operational capability, or is scope, rollout, promise, or risk acceptance adjusted?
- Does signoff show evidence, open risks, accepted tradeoffs, decision owner, and next review date?
