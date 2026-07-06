---
name: outsourcing-scope-and-control-design.md
description: Use when the agent is defining outsourced work scope, retained controls, decision rights, approval authority, service boundaries, data access, escalation rules, quality controls, exception handling, and operating responsibilities between an organization and an external provider.
---

# Outsourcing Scope And Control Design

Outsourcing fails when the contract names a service but the operating boundary is unclear. The provider may do the visible task, while the organization keeps hidden approvals, exception decisions, customer recovery, control evidence, or rework. This skill helps the agent define outsourced scope and retained control so work can move outside the organization without losing accountability.

## Core Rules

### Define included and excluded work

State exactly which tasks, channels, customer groups, regions, languages, systems, hours, volumes, and exception types are in scope. Also state what is excluded and how excluded work is routed. Ambiguous boundaries create disputes and service gaps.

Use examples. A provider may handle standard requests but not VIP customers, regulated cases, refunds above a threshold, safety issues, legal complaints, custom quotes, or suspected fraud.

### Separate execution authority from decision authority

A provider may execute tasks but not own decisions such as policy exceptions, financial adjustments, account termination, access approval, customer compensation, disciplinary action, compliance interpretation, or safety signoff. Define which decisions are retained and how the provider obtains approval.

If the provider must wait for internal approvals, design the approval SLA and backup path. Otherwise the provider will miss service levels for reasons outside its control.

### Design controls for risk level

Controls should match the harm of failure. Low-risk work may use sampled review and simple dashboards. High-risk work may require pre-approval, dual control, audit logs, restricted access, certification, call or ticket review, reconciliation, and exception reporting.

Do not overload low-risk work with heavy controls, but do not rely on trust for privacy, financial, safety, legal, or customer-impacting decisions.

### Define data and system access boundaries

List the data, systems, tools, reports, shared folders, communication channels, and customer information the provider needs. Grant least-privilege access that still allows timely service. Define access approval, removal, periodic review, device rules, data storage, screenshots, exports, and subcontractor access.

If the provider needs data to solve cases, do not design a process that forces them to ask internal teams for every lookup. That creates shadow channels and delay.

### Specify handoffs and escalation paths

Handoffs should include trigger, required evidence, channel, owner, response expectation, status update, and closure responsibility. Escalations should distinguish operational urgency, policy decision, quality concern, customer risk, security issue, and provider performance problem.

Define what the provider should do while waiting. A case should not disappear because it crossed the outsourcing boundary.

### Keep accountability with the organization

The provider performs work, but the organization remains accountable for customers, employees, compliance, and brand. Retained owners should monitor performance, resolve policy questions, review quality, update knowledge, and decide service changes.

Do not outsource accountability along with labor. If internal owners cannot govern the work, the scope is not ready.

### Make service levels operationally complete

SLAs should include volume assumptions, priority definitions, response and resolution rules, pause rules, dependencies, quality measures, rework, escalation, backlog, and reporting. A speed-only SLA encourages fast bad work.

Include what happens when the organization delays approvals, system access, training updates, or policy decisions. Shared dependencies should not be counted as provider failure without context.

### Define change control for scope and rules

Operations change after outsourcing: products change, policies change, volume shifts, systems update, and customer expectations move. Define how process changes, knowledge updates, script changes, access changes, and SLA revisions are approved and communicated.

Without change control, providers keep using old instructions or improvise based on incomplete messages.

## Common Traps

- Defining outsourcing scope by a broad function name rather than tasks, channels, exceptions, and boundaries.
- Forgetting explicit exclusions and routing for out-of-scope work.
- Letting providers execute actions that require retained policy, financial, legal, safety, or access authority.
- Creating approval dependencies without approval SLAs or backup owners.
- Applying the same control level to low-risk and high-risk work.
- Granting broad system access because detailed access design is inconvenient.
- Designing handoffs that transfer work but not ownership, evidence, or status responsibility; measuring speed while ignoring quality, rework, customer impact, and dependency delays
- Assuming the provider will learn changes through informal messages; treating provider mistakes as isolated failures when scope, access, rules, or controls were unclear

## Self-Check

- Are included tasks, channels, groups, regions, languages, systems, hours, volumes, and exceptions explicit?
- Are excluded work and routing paths defined with examples?
- Are execution authority and decision authority separated for policy, finance, access, safety, compliance, compensation, and customer-impact decisions?
- Are internal approval SLAs and backup paths defined for retained decisions?
- Do controls match risk level without overburdening low-risk work or under-controlling high-risk work?
- Are data, system, tool, report, folder, channel, export, device, and subcontractor access boundaries clear?
- Do handoffs include trigger, evidence, channel, owner, response expectation, status update, and closure responsibility?
- Does the organization retain accountable owners for quality, policy, knowledge, service changes, and compliance?
- Do SLAs cover priority, pause rules, dependencies, quality, rework, escalation, backlog, and shared responsibility?
- Is change control defined for process, policy, script, access, knowledge, system, and SLA changes?
