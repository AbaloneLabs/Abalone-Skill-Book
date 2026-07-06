---
name: operational-quality-control.md
description: Use when the agent is designing or reviewing operational quality controls, quality standards, quality review, defect prevention, quality gates, rework prevention, quality metrics, customer-impact review, or quality assurance for recurring operational work.
---

# Operational Quality Control

Operational quality is whether work is correct, complete, timely, safe, compliant, and usable by the recipient. It is not only whether a ticket was closed, an order was processed, or a task was marked done. Agents often add a review step at the end and call that quality control, while missing prevention, early detection, rework loops, customer impact, and the incentives that caused the defect.

Use this skill before designing quality checks, review gates, QA rubrics, sampling programs, defect tracking, rework controls, or operational quality metrics. The goal is to prevent superficial inspection and build quality into the operating system.

## Core Rules

### Define quality in observable terms

Do not use vague standards like "accurate" or "good customer experience" without observable criteria. Define what correct work looks like for each work type: required fields, correct decision, policy fit, documentation, timeliness, customer communication, safety, compliance, handoff quality, and absence of preventable rework.

Quality standards should distinguish severity. A typo, missing internal note, wrong payment action, privacy breach, unsafe instruction, or incorrect customer promise should not receive the same treatment. Severity drives review depth, escalation, remediation, and reporting.

### Separate prevention, detection, and correction

Quality control has three different jobs. Prevention reduces the chance of errors through process design, templates, validation, training, tool constraints, clearer inputs, and workload management. Detection finds errors through review, sampling, reconciliation, monitoring, audits, and exception reports. Correction fixes the work, remediates affected customers or stakeholders, and changes the system that produced the defect.

Do not rely on end-of-line inspection as the primary control. It is usually more expensive and may discover harm after the recipient has already been affected.

### Match controls to risk and reversibility

High-risk work needs stronger controls: money movement, account changes, privacy requests, safety actions, legal or compliance decisions, public communications, high-value customers, irreversible fulfillment, vendor penalties, and regulated processes. Low-risk, high-volume work may be better controlled through automation, sampling, or targeted review.

Control design should avoid two failures: under-controlling high-harm work and over-controlling low-risk work until staff create bypasses. The right control is proportional to harm, error likelihood, detectability, and reversibility.

### Build quality into upstream inputs

Many defects are created before the worker starts. Missing data, unclear policies, bad customer intake, incorrect master data, confusing tools, unrealistic time pressure, or poor handoff can make errors likely. Quality control should inspect the source of defects, not only the person who touched the final task.

If reviewers keep finding the same error, ask what upstream signal, validation, training, or tool constraint would prevent it. Repeated reminders are rarely enough.

### Track defects as learning data

Defect tracking should include defect type, severity, work type, process step, source, owner, detection method, customer impact, rework time, root cause, corrective action, and verification. Counting defects without classification creates noise.

Use categories that help decide action. For example, "agent error" is often less useful than "policy ambiguity," "missing intake field," "tool validation gap," "training gap," "volume pressure," or "handoff missing evidence."

### Protect the recipient during quality failures

When a defect affects a customer, employee, vendor, patient, member, account, shipment, or internal recipient, quality control must include remediation. Define who determines impact, whether notification is required, what correction is needed, whether compensation or apology applies, and how similar affected cases are found.

Internal scoring does not close the quality issue. The external harm must be corrected or consciously accepted by the right owner.

### Calibrate reviewers and standards

Quality review is only useful if reviewers apply standards consistently. Use shared rubrics, examples of pass and fail, calibration sessions, dispute processes, reviewer notes, and updates when policy changes. Monitor reviewer variance.

Inconsistent review creates distrust and poor coaching. Staff should understand what standard they are being held to and why it matters operationally.

### Balance productivity and quality

Quality can degrade when operations emphasize speed, backlog reduction, utilization, or handle time without a balancing measure. Pair productivity metrics with defect rate, severity, reopen rate, customer complaints, rework, audit findings, and staff workload.

Do not let quality work become invisible. Reviews, coaching, rework, and improvement time are part of capacity planning, not optional extra work.

### Close the loop through process improvement

Quality findings should feed SOP updates, tool changes, training, staffing, routing, intake design, automation, policy clarification, and vendor management. Each material defect pattern needs an owner and follow-up date.

If the same defect appears month after month, the quality program is measuring failure rather than improving the operation.

## Common Traps

- Defining quality as "no mistakes" without observable standards or severity levels.
- Adding only final review while ignoring prevention and early detection.
- Reviewing low-risk work heavily while high-risk exceptions rely on memory or informal approval.
- Blaming individual workers before checking intake, tools, workload, policy clarity, and handoffs.
- Treating internal QA scores as complete even when customers or downstream teams remain affected.
- Using defect counts without severity, customer impact, or root cause.
- Letting reviewers apply standards differently without calibration; measuring productivity separately from quality, causing staff to optimize speed at the expense of correctness
- Treating rework as normal operating load instead of a quality signal; closing quality issues without verifying that corrective actions changed the process

## Self-Check

- Are quality standards observable by work type, including accuracy, completeness, timeliness, policy fit, communication, safety, compliance, and handoff quality?
- Are defect severity levels defined and tied to review depth, escalation, remediation, and reporting?
- Does the control design include prevention, detection, and correction rather than only final inspection?
- Are controls proportional to risk, harm, likelihood, reversibility, and detectability?
- Are upstream inputs, tools, policies, handoffs, and workload pressure reviewed as possible defect sources?
- Are defects categorized by type, severity, source, step, detection method, impact, root cause, action, and verification?
- Is customer or stakeholder remediation defined when defects cause external harm?
- Are reviewers calibrated with rubrics, examples, dispute process, and policy updates?
- Are productivity metrics balanced with quality, rework, complaints, reopenings, and staff workload?
- Are recurring quality findings converted into process, tool, training, staffing, policy, or vendor improvements?
