---
name: compliance_remediation_and_control_redesign.md
description: Use when the agent is planning remediation and control redesign after a systemic compliance failure, redesigning preventive and detective controls, building and retesting control effectiveness, integrating redesign with systems and policy, or governing multi-year remediation programs and their reporting.
---

# Compliance Remediation And Control Redesign

When a compliance failure reveals that a control did not work, the organization faces a redesign challenge, not a patch. A weak control redesigned weakly will fail again, often more expensively. Yet remediation programs frequently produce controls that look improved on paper, added approvals, longer checklists, more training, while leaving the underlying design flaw intact: the control still depends on the same human discretion that was bypassed, still cannot detect the conduct it is meant to catch, and still produces no evidence of operation. Control redesign is an engineering discipline. It requires understanding why the old control failed, designing the new one to fail safe where possible, integrating it into systems rather than relying on memory, and retesting it under conditions that approximate real pressure. A remediation program that does not produce genuinely better controls is a program that has not remediated.

Use this skill when planning remediation after a systemic compliance failure, when redesigning preventive or detective controls, when building and retesting control effectiveness, when integrating redesign with systems and policy, or when governing multi-year remediation programs. The goal is to make the agent treat control redesign as a rigorous, system-embedded, effectiveness-driven engineering exercise rather than a documentation update.

## Core Rules

### Diagnose The Design Failure Before Redesigning

You cannot fix a control you do not understand. Before redesigning, diagnose precisely how and why the existing control failed, because the failure mode determines the fix.

Diagnose by identifying:

- whether the control was preventive or detective, and which it needed to be;
- whether the failure was design, the control could not have caught the conduct; implementation, the control existed but was not operating; or bypass, the control operated but was circumvented;
- whether the control depended on human discretion that was compromised;
- whether the control produced evidence of operation or operated invisibly;
- whether the control's threshold or logic was appropriate to the risk;
- whether the control was integrated into the workflow or existed as a separate manual step easily skipped;
- whether segregation of duties was present or absent;
- whether the control scaled to the volume and complexity of the activity.

A control that failed because it relied on the same person who had incentive to bypass it cannot be fixed by adding a checklist. The redesign must change who performs the control, how it is enforced, or how the conduct is structurally prevented.

### Prefer Structural And System-Embedded Controls Over Manual Ones

The hierarchy of control reliability runs from system-enforced, structural controls down to manual, discretionary ones. Remediation should move designs up that hierarchy wherever feasible.

Aim, in order of preference, for:

- structural controls that make the conduct impossible or very difficult, such as system-enforced segregation of duties, hard stops, and mandatory fields;
- automated detective controls that flag the conduct in real time or near-real time, such as transaction monitoring and exception reporting;
- system-embedded approvals that cannot be bypassed without an override that is itself logged and reviewed;
- independent review controls performed by someone without incentive to approve;
- manual controls with strong evidence, such as documented dual review;
- manual controls relying on individual discretion, the least reliable and the most common failure point.

A redesigned control that remains a manual checklist dependent on the same actor has not been redesigned; it has been relabeled. Push toward structural and automated prevention wherever the risk and systems allow.

### Design Controls To Fail Safe And Produce Evidence

A well-designed control fails safe, meaning that if it cannot operate, the underlying transaction does not proceed, and it produces evidence of its operation as a byproduct rather than as an afterthought.

Design for:

- fail-safe behavior, where a control failure or timeout blocks the transaction rather than allowing it;
- default-deny logic, where absence of approval means no action, rather than default-allow;
- complete evidence capture, where the control logs who, what, when, and the basis, automatically;
- exception handling, where overrides are themselves controlled, logged, and reviewed;
- immutability of the evidence trail, so it cannot be altered after the fact;
- timeliness, so detection happens before harm compounds.

A control that produces no evidence of operation cannot be tested, audited, or trusted. Evidence should be a design requirement, captured by the system, not reconstructed by the operator.

### Integrate Redesign With Systems, Policy, And Process

A control does not exist in isolation. It must fit the system that runs the transaction, the policy that defines the standard, and the process that people follow. Redesign that changes one without the others creates gaps.

Integrate by:

- embedding the control in the relevant system configuration, workflow, or application;
- updating the policy and procedure to match the redesigned control, so documentation is not stale;
- updating training so people understand the new control and their role in it;
- aligning job descriptions, authority matrices, and approval workflows;
- coordinating with IT, security, finance, and the business owners whose systems are affected;
- planning the change management, including testing, rollout, and fallback;
- ensuring the control survives system upgrades, migrations, and vendor changes.

A control that lives only in a procedure document, while the system allows the conduct, is not a control. The system configuration is the real control; the document describes it.

### Define Effectiveness With Operational Metrics

Effectiveness is not whether the control was implemented but whether it achieves its purpose under real conditions. Define effectiveness in operational terms that can be measured.

Define:

- the specific risk the control is meant to prevent or detect;
- the metric that shows prevention or detection, such as blocked transactions, detected exceptions, or time-to-detection;
- the population and sampling for testing;
- the threshold for declaring effectiveness;
- the testing approach, including attempts to bypass the control;
- the period over which effectiveness must hold;
- the owner of effectiveness monitoring and the reporting cadence.

Test the control adversarially. Have someone attempt to commit the original conduct, or a variant, and confirm the control catches or blocks it. A control that has never been tested under pressure is unproven.

### Retest Periodically And After Change

Controls degrade. Systems change, personnel turn over, volumes grow, and bad actors adapt. A control that was effective at launch may fail a year later if it is not monitored and retested.

Build a retesting regime that:

- schedules periodic effectiveness testing based on risk;
- triggers retesting after system changes, migrations, or process redesigns;
- monitors operational metrics continuously, not only at test points;
- investigates exceptions and near-misses as early signals;
- reviews override and bypass logs for patterns;
- feeds results back into continuous improvement;
- escalates control degradation before it becomes a failure.

Treat the control as a living system requiring maintenance, not a one-time deliverable. The absence of detected exceptions may mean the control works or that it is not actually screening; distinguish the two through testing.

### Govern The Remediation Program As A Multi-Year Effort

Systemic remediation is rarely quick. Redesigning controls, integrating them into systems, and proving effectiveness over time is a multi-quarter or multi-year program that needs governance.

Govern by:

- a program structure with clear sponsorship, ownership, and milestones;
- integration with the broader compliance program and risk assessment;
- reporting to leadership and the board on progress, evidence, and residual risk;
- budget and resource commitments that match the scope;
- dependency management across IT, business, and external vendors;
- risk acceptance decisions documented at the right level;
- a defined exit criteria for the program, not indefinite remediation.

A remediation program without governance drifts, loses funding, and quietly stops. Sustained sponsorship and transparent reporting on evidence-based progress are what carry it to genuine completion.

### Coordinate Redesign With Disclosure And Regulator Commitments

Where the failure has been disclosed or is under a resolution, the redesign may be subject to regulator commitments, reporting, and certification. The redesign must satisfy those commitments as well as the operational need.

Coordinate:

- the specific commitments made in any settlement, order, or disclosure;
- the reporting and certification obligations and their timelines;
- consistency between the redesigned control and what was represented to the regulator;
- the evidence the regulator will expect to see;
- the independence of any testing the regulator requires;
- the consequences of failing to meet commitments.

A redesigned control that does not satisfy a regulator commitment, or that contradicts what was represented, creates breach risk. Align the redesign with the external obligations from the start.

## Common Traps

### Redesigning The Same Weakness With More Steps

Adding approvals or checklists to a control that failed because of discretionary dependence does not fix the design flaw. Move up the control hierarchy toward structural and automated prevention.

### Manual Controls Disguised As System Controls

A control that lives in a procedure while the system permits the conduct is not a control. Embed the control in the system configuration.

### No Evidence Of Operation

A control that produces no log cannot be tested, audited, or trusted. Design evidence capture as a byproduct of operation.

### Closing On Implementation Without Effectiveness Testing

Configuring the control and declaring victory, without testing whether it catches or blocks the conduct, treats an untested hypothesis as a result. Test adversarially and over time.

### Ignoring System And Process Integration

Changing the control without updating policy, training, workflow, and system configuration creates gaps. Integrate across all layers.

### One-Time Testing Assuming Permanence

Controls degrade with change and time. Build periodic and event-triggered retesting with continuous metric monitoring.

### Ungoverned, Under-Resourced Remediation

A multi-year effort without sponsorship, budget, and reporting drifts and stalls. Govern the program with sustained commitment and transparent progress reporting.

### Misalignment With Regulator Commitments

A redesign that does not meet settlement or disclosure commitments, or that contradicts representations, creates breach risk. Align with external obligations throughout.

## Self-Check

- Has the design failure been diagnosed precisely, identifying whether it was design, implementation, or bypass, and whether it relied on compromised discretion?
- Does the redesign move up the control hierarchy toward structural, system-embedded, automated prevention rather than adding manual steps?
- Is the control designed to fail safe, with default-deny logic, complete automatic evidence capture, controlled exceptions, and immutable logs?
- Is the redesign integrated across system configuration, policy, procedure, training, job descriptions, and change management, with coordination across IT and business owners?
- Is effectiveness defined by operational metrics, sampling, threshold, adversarial bypass testing, and a demonstration period, with a named monitoring owner?
- Is periodic and event-triggered retesting built in, with continuous metric monitoring, exception and near-miss investigation, and escalation of degradation?
- Is the remediation governed as a multi-year program with sponsorship, ownership, milestones, budget, dependency management, board reporting, and defined exit criteria?
- Is the redesign coordinated with disclosure, settlement, and certification commitments, with consistency between the control and regulator representations?
- Does the redesign survive system upgrades, migrations, and vendor changes, with the control truly embedded rather than documented?
- Does the program produce genuinely better controls verified under real pressure, rather than improved documentation?
