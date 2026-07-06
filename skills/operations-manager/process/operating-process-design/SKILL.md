---
name: operating-process-design.md
description: Use when the agent is designing or reviewing an operating process, recurring workflow, standard operating procedure, intake path, approval flow, queue, internal service, back-office routine, or repeatable team process that must produce reliable outcomes under real workload and exception pressure.
---

# Operating Process Design

An operating process is not a tidy list of ideal steps. It is the way real people, systems, controls, inputs, decisions, and exceptions produce repeatable outcomes under pressure. Agents often describe the happy path and miss the parts that decide whether the process works: triggers, ownership, handoffs, invalid inputs, waiting states, controls, evidence, failure recovery, and how staff know what to do when reality does not match the document.

Use this skill before designing an SOP, intake queue, approval process, fulfillment routine, support process, finance operation, vendor workflow, internal service, or cross-functional operating path. The goal is to make the work reliable enough that a capable team can execute it consistently without relying on private memory or heroic coordination.

## Core Rules

### Define the process boundary and purpose

Start with the trigger, input, output, customer or recipient, and done state. A process should begin when a recognizable event occurs and end when an observable result is delivered, recorded, and accepted by the next owner. Avoid vague boundaries such as "handle requests" or "manage issues."

Clarify what the process is meant to optimize: speed, accuracy, compliance, customer experience, cost, safety, consistency, auditability, or learning. If the purpose is unclear, the design will accumulate steps that feel reasonable but do not serve the operating goal.

### Map the real workflow, including waiting

Document the actual path work takes: systems, queues, messages, approvals, reviews, data entry, handoffs, waiting states, and rework loops. Waiting time is part of the process even if no one is actively working. Include where work can stall and how staff know it is stalled.

For each major step, name input, owner, action, decision point, tool, output, evidence, and next owner. A process that says "team reviews" without naming who reviews, what they check, and how they record completion is not operationally usable.

### Design intake quality before downstream work

Bad inputs create downstream rework and inconsistent decisions. Define required fields, valid formats, evidence, requester authority, priority information, and rejection or clarification paths. Make incomplete intake visible instead of letting it become hidden work.

Do not overburden intake with unnecessary fields. Required information should be tied to a decision, control, routing need, or service promise. If staff routinely bypass intake requirements, either the requirements are wrong or enforcement is weak.

### Assign ownership, not just participation

Every step needs an accountable role. Participation is not ownership. The owner is responsible for completing the step, making or escalating the decision, updating status, and ensuring the next handoff works.

Clarify who owns backlog monitoring, blocked work, exception approval, customer or stakeholder communication, record updates, quality review, and process improvement. Shared responsibility may be appropriate for collaboration, but accountability should not be shared so broadly that no one acts.

### Build exception paths explicitly

Most process failures occur when the case is unusual. Identify missing data, duplicate requests, urgent requests, policy exceptions, customer complaints, system outages, vendor delays, failed approvals, quality defects, fraud risk, safety risk, privacy issues, and conflicting priorities.

For each important exception, define who decides, what evidence is required, what can be done immediately, what must be escalated, how the exception is recorded, and whether the normal service promise changes. Exceptions should not depend on whoever is most persuasive in chat.

### Match controls to risk

Controls prevent errors, fraud, unsafe actions, privacy exposure, compliance failure, and inconsistent treatment. They can also slow work and invite bypasses if applied indiscriminately. Use stronger controls for high-harm, irreversible, regulated, financial, security-sensitive, or customer-impacting steps.

Controls may include required fields, validation, approval thresholds, segregation of duties, audit logs, sampling review, reconciliation, access permissions, automated checks, and exception reporting. The control should address a named risk; otherwise it may be friction masquerading as discipline.

### Make status and aging visible

The process should show what is new, in progress, blocked, awaiting approval, awaiting customer or vendor response, completed, reopened, and overdue. Status should include owner and next action, not only a broad stage name.

Visibility exists to support intervention. A dashboard that shows aging without owner, cause, and decision needed can become passive reporting. Define who reviews status, how often, and what threshold triggers action.

### Preserve evidence and auditability

The process should produce records that prove what happened: request, input, decision, approval, communication, exception, completion, and any customer or stakeholder commitment. Evidence should be easy to find by the next owner, reviewer, auditor, or incident responder.

Do not let critical decisions live only in private messages. If a decision matters for customer trust, cost, compliance, quality, or safety, it belongs in the system of record or an approved evidence location.

### Test the process before scaling

Run the process against realistic cases, including edge cases and failures. Test with actual users, tools, roles, and volumes where possible. Look for unclear ownership, missing inputs, handoff delays, double entry, control bottlenecks, training gaps, and status ambiguity.

Scale only after the process works in practice. A process designed in a meeting may look complete while failing the first time a real exception appears.

## Common Traps

- Writing a happy-path SOP that omits invalid inputs, blocked work, exceptions, and recovery paths.
- Confusing policy with process. Policy states what should be true; process explains how people make it true.
- Naming teams instead of accountable owners for decisions, backlog, communication, and quality.
- Treating waiting time, approvals, vendor response, and customer follow-up as outside the process.
- Requiring intake fields that staff cannot explain or do not use for decisions.
- Adding manual approvals for low-risk work while leaving high-risk steps uncontrolled.
- Building a process around a tool's default workflow rather than the operating outcome needed; letting key decisions and exception approvals live in chat instead of the system of record
- Measuring only completed volume while ignoring aging, blocked work, rework, defects, and customer impact; launching the process without testing realistic edge cases and handoffs

## Self-Check

- Are trigger, input, recipient, output, done state, and process purpose clearly defined?
- Does the workflow map tools, queues, waiting states, approvals, handoffs, rework loops, and evidence?
- Are intake requirements tied to routing, decision, control, service promise, or risk?
- Is each step owned by an accountable role rather than a vague team?
- Are exception paths defined for missing data, duplicates, urgency, policy deviations, outages, vendor delays, complaints, and risk-sensitive cases?
- Do controls match named risks without creating unnecessary bypass-prone friction?
- Is status visible with owner, aging, blocked reason, next action, and review cadence?
- Are decisions, approvals, communications, and exceptions recorded in an auditable place?
- Has the process been tested with realistic cases, edge cases, real tools, and actual users?
- Can a trained person execute the process without relying on private tribal knowledge?
