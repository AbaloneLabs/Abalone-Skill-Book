---
name: preventive-and-detective-control-design.md
description: Use when the agent is designing operational controls, choosing preventive versus detective controls, adding approvals or checks, reviewing manual or automated controls, balancing control strength against speed, or deciding how to prevent, detect, correct, and evidence high-risk operational failures.
---

# Preventive And Detective Control Design

Controls are design choices that reduce the chance or impact of operational failure. Weak controls let high-risk work fail silently. Heavy controls can slow service, create bypasses, and push staff into informal workarounds. Agents often suggest "add approval" or "monitor more closely" without deciding what risk the control addresses and how it will operate. This skill helps the agent design controls that are targeted, usable, and verifiable.

## Core Rules

### Start With The Specific Risk

Define the failure the control should prevent, detect, or correct. Include the process step, trigger, impact, and current gap. A control without a clear risk becomes busywork.

Do not add controls because a process feels risky. Name the actual bad outcome and why current handling is insufficient.

### Choose The Control Type Deliberately

Preventive controls stop failure before it happens: required fields, access limits, approval gates, segregation of duties, validation rules, checklists, or automation. Detective controls find failure after it occurs: reconciliations, audits, exception reports, sampling, alerts, and reviews. Corrective controls restore the process after failure.

High-harm, irreversible failures usually need stronger preventive controls. Lower-risk or high-volume work may rely on detection and correction if prevention would be too costly.

### Design For Real Workflow

Controls must fit how work actually moves. Identify who performs the control, when, in what system, with what evidence, and what happens if the control fails. Include handoffs, shifts, vendors, time zones, and peak periods.

If a control is slow, unclear, or outside the workflow, staff will bypass it or perform it superficially.

### Preserve Segregation And Authority

For sensitive work, separate requester, preparer, approver, and reviewer where needed. Check access permissions, approval limits, conflict of interest, and override rules. The person who benefits from speed may not be the right person to approve exceptions.

When staffing makes segregation difficult, document compensating controls and residual risk acceptance.

### Balance Strength And Friction

Control strength should match risk. Over-controlling low-risk work creates delay and encourages bypass. Under-controlling high-risk work creates exposure. Consider volume, harm, reversibility, customer impact, compliance, and error likelihood.

Use risk-based sampling, thresholds, or automated validation where full review would be excessive.

Check whether the control changes customer or employee behavior. Extra evidence requests can reduce fraud but increase abandonment or complaints; extra approvals can improve accuracy but push urgent work into side channels. The design should anticipate these effects.

### Make Failure Paths Clear

Define what happens when the control detects an issue: hold work, return for correction, escalate, notify customer, open incident, start reconciliation, or trigger root cause review. A control that detects problems but has no response path only creates a report.

Include owner, deadline, and escalation for unresolved control exceptions.

### Evidence The Control

Controls need evidence that they operated: timestamp, reviewer, sample, approval note, exception report, system log, reconciliation result, or audit record. Evidence should be useful without creating unnecessary documentation burden.

If evidence is manual, check reliability. Manual evidence is easy to skip during peaks unless built into the workflow.

### Test Operating Effectiveness

A control can be well designed and still fail in practice. Test whether it operates consistently, catches the intended issue, is performed by the right owner, and creates timely response. Review exceptions, bypasses, overdue reviews, false positives, and false negatives.

Use test findings to adjust the control rather than blaming reviewers by default.

Test during normal and stressed conditions. Controls that operate during a quiet week may fail during peak volume, staff absence, system downtime, or vendor disruption. High-risk controls should be proven under the conditions where failure is most likely.

### Review Control Drift

Controls drift when tools change, teams reorganize, volume rises, or local shortcuts emerge. Revisit controls after incidents, audits, automation, policy changes, or repeated exceptions. Retire controls that no longer reduce risk.

Too many stale controls reduce respect for the control environment.

## Common Traps

- Adding an approval step without defining the risk it controls.
- Using detective controls for high-harm irreversible failures that need prevention.
- Designing controls around the official process while real work happens in side channels.
- Creating controls so slow that staff bypass them to meet service promises.
- Allowing the same person to request, approve, and review sensitive actions.
- Detecting exceptions without assigning a response path.
- Treating a policy statement as proof that the control operates; requiring evidence that is too burdensome to maintain during peak work
- Testing control existence but not operating effectiveness; keeping stale controls after the process or risk has changed

## Self-Check

- Is the control tied to a specific risk event and process step?
- Is the control type preventive, detective, corrective, or compensating by design?
- Does the control fit real workflow, systems, shifts, vendors, and handoffs?
- Are authority, access, and segregation of duties appropriate for the risk?
- Is control friction proportional to harm, likelihood, and reversibility?
- Is there a clear failure response path with owner and deadline?
- Is operating evidence defined and reliable?
- Has operating effectiveness been tested, not only design existence?
- Are bypasses, false positives, false negatives, and overdue exceptions reviewed?
- Is there a cadence to remove or redesign stale controls?
