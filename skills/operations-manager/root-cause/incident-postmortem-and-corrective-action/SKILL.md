---
name: incident-postmortem-and-corrective-action.md
description: Use when the agent is conducting an operational incident postmortem, writing a post-incident review, defining corrective and preventive actions, assigning CAPA owners, validating incident learning, or turning incident evidence into durable operational changes.
---

# Incident Postmortem And Corrective Action

An incident postmortem should turn disruption into safer operations. It should not be a blame document, a polished narrative, or a long action list that nobody verifies. Agents often summarize what happened and propose generic training, communication, or monitoring without connecting actions to evidence. This skill helps the agent produce postmortems that explain impact, decisions, causes, corrective actions, and verification.

## Core Rules

### State Purpose And Scope

Define why the postmortem is being performed and what event it covers. Include incident dates, affected processes, customers or internal users, systems, vendors, severity, and recovery status. State what is out of scope if separate reviews exist for security, legal, safety, or technical root cause.

The postmortem should answer the operational question: what must change so recurrence or impact is less likely?

### Build A Factual Timeline

Use the incident log, timestamps, communications, queue data, vendor notices, customer reports, and recovery actions to build a factual sequence. Include detection, severity changes, containment, decisions, communications, recovery milestones, and handoffs.

Separate facts from interpretation. If a timeline point is approximate or disputed, label it.

### Review Impact And Response Quality

Assess customer or internal impact, service breaches, financial exposure, compliance or safety concerns, employee workload, backlog, quality drift, and recovery cost. Also review response quality: detection speed, escalation, command structure, communication, containment, and recovery.

Postmortems should not focus only on the original trigger. Response gaps often determine how much harm occurred.

### Identify Causes And Contributing Conditions

Use evidence to distinguish triggering cause, enabling conditions, detection failure, response failure, and recovery friction. Include system design, workload, training, tools, data quality, vendor behavior, policy ambiguity, incentives, and ownership gaps where relevant.

Avoid assigning blame to individuals without examining the system around the behavior and involving the right management or HR process where necessary.

### Design Corrective And Preventive Actions

Each action should be specific, owned, dated, and tied to a cause or response gap. Good actions change process, control, tooling, access, staffing, training, monitoring, vendor management, policy, runbooks, or service promises. Weak actions only say "be more careful" or "communicate better."

Separate immediate containment, corrective action, preventive action, and longer-term improvement. They may have different owners and timelines.

Prioritize actions by risk reduction, feasibility, dependency, and time to effect. A small control that prevents repeat customer harm this week may matter more than a large redesign that will not land for a quarter. Keep long-term fixes visible, but do not let them delay practical protection.

### Define Verification Of Effectiveness

Corrective action is incomplete until someone checks whether it worked. Define evidence: audit sample, reduced defect rate, successful drill, updated access control, queue metric, training assessment, vendor SLA improvement, control test, or incident recurrence monitoring.

Set the verification date and owner. Closing an action when the change is implemented is weaker than closing it when effectiveness is shown.

### Manage Risk Acceptance

Not every risk can be eliminated immediately. If the organization accepts residual risk, record the risk, reason, owner, duration, monitoring, and review date. Risk acceptance should be explicit, not hidden in an unresolved action.

High-risk residual exposure may require leadership, legal, compliance, safety, security, finance, or customer owner approval.

### Communicate Learning Appropriately

Share the right level of postmortem learning with affected teams. Frontline staff need changed procedures and warning signs. Leaders need risk, actions, owners, and investment decisions. Customers or partners may need a carefully approved summary where applicable.

Do not overshare sensitive legal, privacy, security, personnel, or customer information in broad learning documents.

### Track Actions To Closure

Use an action tracker with owner, due date, status, dependency, verification method, and overdue escalation. Postmortem quality is proven by follow-through, not by the document.

Recurring overdue actions should be escalated as operating risk.

Review closed actions against fresh evidence after the next comparable demand period, shift, launch, or peak. Some fixes only look complete until the same operating condition returns.

## Common Traps

- Writing a polished narrative that does not define what must change.
- Omitting detection, escalation, communication, and recovery failures from analysis.
- Treating the trigger as the only root cause.
- Ending causes at human error or vendor fault without examining system conditions and controls.
- Creating action items that are vague, ownerless, or not tied to a cause.
- Closing corrective actions when they are implemented without verifying effectiveness.
- Hiding residual risk in deferred actions rather than recording acceptance; sharing sensitive postmortem details too broadly
- Letting action owners miss dates without escalation; repeating postmortems for similar incidents without comparing prior actions and recurrence

## Self-Check

- Is the incident scope, severity, affected operation, and review purpose clear?
- Does the timeline separate facts, estimates, and disputed points?
- Are impact and response quality both reviewed?
- Are triggering causes, enabling conditions, detection gaps, response gaps, and recovery friction distinguished?
- Are corrective and preventive actions specific, owned, dated, and tied to causes?
- Are immediate containment, corrective action, preventive action, and longer-term improvement separated?
- Is effectiveness verification defined for every material action?
- Is residual risk explicitly accepted by the right owner where it remains?
- Is learning communication tailored and sensitive information protected?
- Are action items tracked through verification, not only implementation?
