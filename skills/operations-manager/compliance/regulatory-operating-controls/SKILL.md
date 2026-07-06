---
name: regulatory-operating-controls.md
description: Use when the agent is designing or reviewing operational controls for regulatory requirements, regulated workflows, mandatory checks, approval thresholds, access restrictions, regulated customer communications, deadline controls, audit trails, or monitoring needed to keep recurring operations compliant.
---

# Regulatory Operating Controls

Regulatory controls must operate inside the daily workflow, not sit beside it. A requirement that depends on memory, informal review, or heroic compliance effort will fail during volume spikes, turnover, or tool changes. Agents often recommend "ensure compliance" without designing the control, evidence, owner, and failure path. This skill helps the agent make regulatory controls practical and auditable.

## Core Rules

### Link Control To Requirement

Identify the specific regulatory or policy requirement the control supports. State what failure it prevents or detects, who is protected, and what harm occurs if it fails. If the requirement is unclear, get specialist interpretation before designing the operating control.

Avoid generic controls that do not map to an obligation or risk.

### Place Controls At The Right Workflow Point

Controls should operate where the risk occurs: intake, eligibility check, identity verification, approval, customer notice, transaction processing, data access, quality review, closure, retention, or reporting. A late control may find issues but not prevent harm.

For high-risk irreversible work, use prevention. For reversible lower-risk work, detection and correction may be sufficient.

### Define Owner And Evidence

Every control needs an operator, reviewer, evidence owner, frequency, and record location. Evidence should show who did what, when, on which item, with what result, and what happened to exceptions.

Do not rely on someone saying they checked. Regulated controls need durable evidence.

Include proof of timeliness where deadlines matter. A correct approval, notice, or review may still fail the control if the timestamp shows it occurred after the regulated action or deadline.

### Account For Exceptions

Regulated work often has edge cases: missing information, urgent customer harm, system outage, protected customer status, legal hold, or conflicting requirements. Define who can approve exceptions, what evidence is needed, and when the exception expires.

Uncontrolled exceptions are a common source of compliance failure.

Classify exceptions by whether they are allowed by rule, require specialist approval, or are not permitted. Some regulated steps cannot be waived by operations, even during pressure or customer escalation.

### Include Access And Segregation

Check who can view, edit, approve, export, delete, or override regulated records. Access should match role and need. Segregation of duties may be required for approvals, payments, complaints, investigations, or customer data handling.

Review access after role changes, vendor changes, and incidents.

### Monitor Control Health

Use exception reports, overdue reviews, sampling, audit findings, access reports, missed deadline lists, and quality results to monitor whether controls operate. Include thresholds that trigger escalation.

A control that is only checked annually may fail for months before anyone notices.

Monitor both execution and timeliness. A review completed late, an approval after action, or a notice sent after deadline may not satisfy the obligation even though the control record exists.

### Protect Documentation Quality

Regulated decisions often need explanation, evidence, and consistent language. Templates can help, but staff need judgment on when templates are insufficient. Documentation should support later review without exposing unnecessary sensitive data.

Poor notes can be as risky as no notes if they misstate facts or create unsupported commitments.

### Plan For Operational Pressure

Controls are most likely to fail during peak volume, outages, staffing gaps, vendor delays, or leadership pressure. Define how the control operates during disruption and what compensating control applies if normal workflow is unavailable.

Do not let "emergency" become an informal compliance bypass.

Pre-approve the emergency path where possible. Staff should know which controls remain mandatory, which can be modified, who approves the modification, and what evidence is required afterward.

### Test And Update Controls

Test whether the control catches the intended failure, produces usable evidence, and is followed by staff. Update controls after regulatory changes, audit findings, incidents, product changes, or process redesign.

Testing should include sample cases and exception cases, not only standard work.

When testing finds gaps, decide whether the issue is design failure, execution failure, evidence failure, or interpretation failure. Each requires a different owner and remediation path.

## Common Traps

- Designing a control without tying it to a specific obligation or failure.
- Placing the check too late to prevent irreversible harm.
- Relying on undocumented human memory for regulated requirements.
- Allowing exceptions without approval, evidence, and expiration.
- Giving broad access because it is convenient.
- Monitoring controls too infrequently for the risk.
- Using templates that create misleading or unsupported records; letting peak workload or outages bypass compliance steps informally
- Testing only easy cases and missing edge cases; failing to update controls when requirements or workflows change

## Self-Check

- Is each control tied to a specific obligation and failure mode?
- Is the control placed where it can prevent or detect the risk in time?
- Are operator, reviewer, evidence, frequency, and record location defined?
- Are exceptions governed by approval, evidence, expiration, and review?
- Are access and segregation of duties appropriate?
- Are control health indicators and escalation thresholds defined?
- Does documentation support review without overexposing sensitive information?
- Are continuity or disruption modes covered by compensating controls?
- Has operating effectiveness been tested with standard and exception cases?
- Is there a trigger to update the control after regulatory or process change?
