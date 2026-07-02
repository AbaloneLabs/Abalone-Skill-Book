---
name: compliance_controls_in_business_process_design.md
description: Use when the agent is building preventive compliance controls into business workflows and system design, embedding approvals and segregation of duties into processes, designing controls that produce evidence by default, or reviewing process and system changes for compliance impact.
---

# Compliance Controls In Business Process Design

The strongest compliance controls are the ones built into the business process itself, so that compliant behavior is the path of least resistance and noncompliant behavior is difficult, visible, or blocked. When compliance lives in a separate layer of reviews, attestations, and after-the-fact audits, it depends on people remembering and choosing to comply, and it generates evidence only through manual effort. When controls are engineered into the workflow and the system, the process prevents violations before they happen, detects them automatically, and produces evidence as a byproduct of operation. The trap is treating compliance as something added on top of processes rather than designed into them, which produces friction, workarounds, and weak evidence. The discipline is to embed preventive and detective controls into process and system design, and to manage changes to those processes as compliance events.

Use this skill before designing or changing a business process or system, embedding compliance controls into workflows, designing approvals and segregation of duties, or reviewing a process change for compliance impact. The goal is to make the agent build controls into the work rather than around it.

## Core Rules

### Design Controls Into The Process, Not Around It

A control built into the process is enforced by the process; a control built around it depends on human discipline. Prefer the former.

Design so that:

- the compliant path is the default and the easiest path;
- prohibited actions are blocked or require explicit, documented override;
- required approvals are system-enforced, not optional;
- segregation of duties is built into roles and permissions;
- mandatory fields, checks, and validations prevent incomplete or noncompliant transactions;
- exceptions are routed for review rather than silently allowed.

If an employee can complete a noncompliant transaction simply by skipping a step, the control is not in the process. Make the process itself the control.

### Prefer Preventive Over Detective Controls

Preventive controls stop violations before they occur; detective controls find them after. Both matter, but preventive controls are stronger and cheaper over time.

Build preventive controls such as:

- system-enforced eligibility, screening, and blocking at the point of action;
- mandatory approvals with authority limits built into workflow;
- segregation of duties so no single person can both initiate and approve a high-risk action;
- hard stops on prohibited counterparties, geographies, or activities;
- validation rules that prevent incomplete or out-of-policy entries.

Use detective controls, such as monitoring, exception reporting, and data analytics, to catch what preventive controls miss and to test that preventive controls are working. Layer them deliberately.

### Make Compliance A Stakeholder In Process And System Design

Controls cannot be embedded if compliance is not at the design table. Make compliance a required stakeholder in process and system change.

Establish:

- compliance review as a required step in process design and system change;
- compliance sign-off as a condition for deploying changes to high-risk processes;
- compliance involvement in technology selection and configuration;
- a process-change intake so compliance sees proposed changes before they happen;
- collaboration between compliance, operations, and technology from the start.

A process designed without compliance and then sent for review can only be patched. Bring compliance in at design to shape the process.

### Build Evidence Production Into The Process

Controls that do not produce evidence are hard to prove. Design processes so that evidence is generated automatically as a byproduct of operation.

Build in:

- system logs of approvals, overrides, and decisions, with timestamps and user identity;
- automatic capture of screening results, exceptions, and dispositions;
- immutable records of transactions and their control states;
- audit trails that reconstruct who did what, when, and why;
- retention aligned to the longest applicable obligation.

A process that produces compliance evidence by default is far easier to defend in an exam or investigation than one that requires manual reconstruction. Engineer evidence in.

### Manage Segregation Of Duties And Access Deliberately

Segregation of duties and least-privilege access are foundational preventive controls that are often implemented loosely. Design them deliberately.

Design:

- separation of initiation, approval, recording, and reconciliation for high-risk activities;
- role-based access that grants the minimum permissions needed;
- authority limits tied to risk, with escalation for exceptions;
- periodic access reviews to remove excessive or stale permissions;
- controls over privileged and administrator access;
- joiner-mover-leaver processes that update access promptly.

Segregation that exists on paper but is bypassed through shared logins or excessive admin rights is not a control. Enforce it in the system.

### Review Process And System Changes As Compliance Events

A well-controlled process can be undone by an uncontrolled change. Treat changes as compliance events.

Govern changes by:

- requiring compliance review of process and system changes affecting high-risk activities;
- assessing the impact of changes on existing controls and evidence;
- testing changes before deployment to confirm controls still work;
- documenting the change, its approval, and its compliance impact;
- preventing uncontrolled changes through change management and configuration controls;
- monitoring after deployment for unintended effects.

A configuration change that quietly disables a validation rule can create a control gap that goes undetected for months. Control the change process.

### Calibrate Control Strength To Risk

Not every process warrants the same control strength. Apply controls proportional to risk to avoid friction and workarounds.

Calibrate by:

- risk-tiering processes based on the obligation, exposure, and harm potential;
- applying strong preventive controls to high-risk processes;
- applying lighter, risk-based controls to low-risk processes;
- reviewing the tiering when the process or risk changes;
- avoiding over-control that creates friction and encourages circumvention.

A process burdened with excessive controls for low risk will breed workarounds that defeat the controls. Match strength to risk.

### Design For Monitoring And Continuous Improvement

A process should be observable so that its control performance can be monitored and improved over time.

Design for:

- metrics and dashboards that show control performance in near real time;
- exception and override reporting that surfaces control stress;
- periodic control testing built into the operating rhythm;
- feedback loops from monitoring, testing, and incidents into process redesign;
- the ability to adjust controls quickly as risk changes.

A process that cannot be observed cannot be improved or defended. Build observability in from the start.

## Common Traps

### Controls Added On Top Of Processes

A separate compliance review layer depends on human discipline and generates manual evidence. Build controls into the process.

### Over-Reliance On Detective Controls

Finding violations after they occur is weaker and costlier than preventing them. Prefer preventive controls and layer detection.

### Compliance Excluded From Design

A process designed without compliance can only be patched later. Make compliance a design stakeholder.

### No Evidence By Default

Controls that require manual evidence collection are hard to prove and degrade over time. Engineer evidence production in.

### Loose Segregation And Access

Segregation on paper bypassed by shared logins or excessive rights is not a control. Enforce it in the system.

### Uncontrolled Changes

A configuration change can silently disable a control. Govern changes as compliance events.

### Over-Control Of Low-Risk Processes

Excessive controls breed workarounds. Calibrate control strength to risk.

### Unobservable Processes

A process that cannot be monitored cannot be improved or defended. Build in metrics and feedback loops.

## Self-Check

- Are controls designed into the process so the compliant path is the default and prohibited actions are blocked or routed for override?
- Are preventive controls preferred over detective ones, with detection layered to catch what prevention misses?
- Is compliance a required stakeholder in process and system design, with sign-off for high-risk changes?
- Does the process produce evidence automatically through logs, audit trails, screening dispositions, and retention aligned to obligations?
- Are segregation of duties, least-privilege access, authority limits, access reviews, and privileged-access controls enforced in the system?
- Are process and system changes reviewed, tested, documented, and controlled as compliance events?
- Is control strength calibrated to process risk tier, avoiding over-control that breeds workarounds?
- Is the process observable through metrics, exception reporting, testing, and feedback loops for continuous improvement?
- Could the controls be defended to a regulator as engineered into the work rather than added around it?
