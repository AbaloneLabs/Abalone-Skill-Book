---
name: incident-detection-triage-and-containment.md
description: Use when the agent is deciding whether an operational issue is an incident, assessing impact, triaging severity, containing customer or business harm, stabilizing a failing workflow, separating symptoms from confirmed facts, or choosing immediate protective actions before full recovery is known.
---

# Incident Detection Triage And Containment

Early incident handling is a judgment problem under uncertainty. The operation must decide whether a signal is noise, a local defect, a service-impacting incident, a safety or compliance event, or a wider business disruption before all facts are known. Agents often jump to explanation, apology, or root cause too soon. This skill helps the agent slow down enough to protect people, customers, records, and service while facts are still emerging.

## Core Rules

### Decide Whether This Is An Incident

Define the threshold that turns an issue into an incident. Triggers may include customer harm, missed service promises, safety risk, privacy or security exposure, financial loss, compliance deadline, critical system outage, vendor failure, high-volume defects, site disruption, or loss of operational control. A single severe case can be an incident even if volume is low.

Do not wait for perfect certainty when harm is active. If the signal may be high impact, classify provisionally and revise later. Provisional classification is safer than leaving a serious event in normal queue flow.

### Separate Confirmed Facts From Hypotheses

Create a current-state summary that separates what is confirmed, what is suspected, what is unknown, and what evidence is being gathered. Include timestamps, affected process, customer or internal impact, volume, geography, systems, vendors, teams, and current actions.

Avoid cause claims at detection time unless verified. A premature cause can misdirect containment, damage trust, and create bad external communication.

### Assess Impact Before Assigning Severity

Severity should reflect impact and urgency, not how loud the report is. Consider people affected, customer harm, safety, compliance, revenue, financial exposure, operational blockage, reputational risk, data sensitivity, executive or regulator visibility, reversibility, and whether harm is spreading.

Use the highest credible risk when facts are incomplete. If later evidence shows lower impact, downgrade deliberately and record why.

### Contain Harm Before Optimizing Recovery

Containment reduces further damage while the team investigates. Actions may include pausing a workflow, disabling a broken automation, holding outbound communication, freezing shipments, stopping approvals, routing work to manual review, isolating affected inventory, notifying a vendor, or narrowing service scope.

Containment has costs. Name what the containment protects, what it disrupts, who approves it, and how it will be lifted. Avoid permanent workarounds disguised as temporary containment.

### Preserve Evidence And Records

Incident triage must protect logs, case samples, communications, timestamps, queue snapshots, audit trails, screenshots, vendor notices, customer reports, and decision notes. Evidence is needed for recovery, customer remediation, compliance review, and root cause analysis.

Do not clean up the mess in a way that destroys the ability to understand it. If legal, privacy, security, safety, or compliance issues may apply, involve the appropriate owner before deleting, altering, or broadly sharing records.

### Define Immediate Ownership

Assign a triage owner even before the full incident command structure exists. The owner coordinates evidence gathering, impact assessment, containment actions, stakeholder updates, and escalation until incident command is assigned or the issue is downgraded.

Avoid shared ambiguity. A group chat is not an owner. Every open question should have a person or role responsible for the next update.

### Protect Normal Operations Deliberately

Early incident response can drain capacity from queues, service, quality review, and frontline support. Decide what normal work pauses, what remains protected, what priorities change, and how customer or internal expectations will be updated.

Do not silently let routine work fail because the incident is consuming key people. If normal operations will degrade, make that a managed decision.

### Escalate By Risk Surface

Escalate to the right owners based on the risk: safety, security, privacy, legal, compliance, finance, HR, facilities, vendor management, communications, product, engineering, customer success, or executive leadership. Escalation should include the decision needed, evidence, deadline, and current containment.

Do not escalate only for visibility. Escalation without a clear ask creates noise and slows triage.

### Timebox Reassessment

Set a reassessment time as soon as the incident is opened. Early facts change quickly. Decide when severity, scope, containment, owner, and communication will be reviewed again.

If the event is downgraded, close the loop with records and stakeholders. If it grows, move into formal command and recovery.

## Common Traps

- Waiting for full root cause before declaring an incident while harm continues.
- Treating a low-volume event as low severity even though each case carries safety, legal, financial, or privacy risk.
- Letting the loudest stakeholder define severity instead of using impact and urgency.
- Communicating suspected cause as fact during early triage.
- Starting recovery actions before containing the source of ongoing harm.
- Pausing a workflow without naming who approved it, what it protects, and when it will be reviewed.
- Cleaning up queues, messages, or records in a way that destroys evidence; leaving triage ownership in a group chat with no accountable owner
- Pulling key staff into incident work without deciding what normal service will slow or pause; escalating broadly without a specific decision, evidence packet, and deadline

## Self-Check

- Is the incident threshold explicit, and has the issue been provisionally classified where risk is credible?
- Are confirmed facts, hypotheses, unknowns, and evidence needs separated?
- Does severity reflect impact, urgency, spread, reversibility, and high-risk obligations?
- Are containment actions defined with owner, approval, disruption, protected outcome, and review point?
- Are logs, timestamps, samples, communications, and decision records preserved?
- Is there a named triage owner with responsibility for next updates and open questions?
- Has the impact on normal operations been managed rather than left implicit?
- Are the right safety, security, privacy, legal, compliance, finance, vendor, or executive owners involved where relevant?
- Does every escalation include the decision needed, evidence, deadline, and current containment?
- Is there a timeboxed reassessment of severity, scope, containment, ownership, and communication?
