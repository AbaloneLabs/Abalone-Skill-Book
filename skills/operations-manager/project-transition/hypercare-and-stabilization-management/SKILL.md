---
name: hypercare-and-stabilization-management.md
description: Use when the agent is planning or running hypercare, stabilization, early-life support, post-go-live issue management, transition command cadence, support exit criteria, defect burn-down, or steady-state handoff after a project goes live.
---

# Hypercare And Stabilization Management

Hypercare is not an indefinite emergency room for project leftovers. It is a temporary operating mode that protects users and customers while the new process, system, or service stabilizes and transfers into steady state. Agents often define hypercare as extra meetings and fast escalation, but miss scope, issue triage, quality thresholds, owner transfer, and exit criteria. This skill helps the agent run hypercare with discipline.

## Core Rules

### Define hypercare scope and purpose

State which users, services, systems, sites, processes, issue types, hours, and support channels are in hypercare. Also state what is out of scope and should go through normal support. Hypercare should focus on launch or transition risk, not become a universal bypass.

Define the purpose: protect service, identify defects, resolve adoption issues, stabilize workload, validate controls, or complete knowledge transfer.

### Establish issue triage and severity

Hypercare issues should be classified by customer or user impact, safety, compliance, revenue, data integrity, workaround availability, frequency, and operational burden. Severity should drive response time, owner, escalation, and communication.

Do not let all hypercare issues become urgent. Over-escalation burns out experts and hides the issues that truly threaten stabilization.

### Keep one issue source of truth

Use one issue log or system of record for defects, questions, workarounds, decisions, owners, status, severity, target date, dependencies, and closure evidence. Chat can support coordination but should not be the only record.

Duplicate tracking across project, support, and operations creates inconsistent status and missed follow-up.

### Assign owners across project and operations

During hypercare, some issues belong to the project team, some to operations, some to vendors, and some to product or IT. Define ownership rules and transfer criteria. The project team should not retain every issue by default, but operations should not inherit unresolved design defects without support.

Clarify who can approve workarounds, scope changes, communication, and risk acceptance.

### Monitor stabilization indicators

Track issue volume, severity trend, repeat questions, backlog, SLA, quality defects, user adoption, manual work, escalations, training gaps, control exceptions, customer complaints, and support load. Compare trends against exit criteria.

Stable does not mean no issues. It means remaining issues are understood, owned, and manageable through normal operations.

### Manage workarounds and temporary controls

Hypercare often uses manual workarounds, extra reviews, temporary reports, bridge calls, special queues, and direct expert access. Each should have owner, reason, risk, expiry, and retirement or formalization plan.

Temporary controls can protect service, but they also hide true cost and capacity if left in place.

### Communicate status and changes clearly

Users, support teams, managers, and leaders need different updates. Status should include current impact, top issues, owner, action, workaround, decision needed, next update, and expected change. Avoid saying an issue is fixed until users can verify the outcome.

If guidance changes during hypercare, update runbooks, FAQs, macros, and training materials immediately.

### Exit deliberately into steady state

Exit criteria may include issue volume threshold, no open critical defects, stable SLA, trained support, updated documentation, accepted residual risks, retired temporary channels, confirmed owners, and normal governance cadence. Exit should be signed off by operations, support, and project owners.

If exit criteria are not met, reduce hypercare carefully or extend it with a clear plan rather than simply declaring success.

### Prevent hypercare from masking capacity gaps

Extra project experts, bridge calls, manual checks, and leadership attention can make the new service appear stable even when normal operations cannot sustain it. Before exit, test whether the steady-state team can handle volume and decisions without temporary support.

If stability depends on exceptional staffing, document the gap and fund, reduce, automate, or accept the risk.

## Common Traps

- Starting hypercare without scope, purpose, and out-of-scope rules.
- Treating every post-go-live issue as urgent.
- Managing issues only in chat or meetings with no source of truth.
- Letting project teams hold all issues too long, or forcing operations to absorb unresolved design defects too early.
- Measuring stabilization by calendar time instead of issue trend and operational manageability.
- Leaving temporary bridge calls, manual reviews, and special queues active after they are no longer justified; communicating "fixed" before the affected users or process validate it
- Updating guidance verbally but not in runbooks, FAQs, macros, and training material; exiting hypercare without support readiness and owner transfer
- Treating residual risks as closed because the project team is leaving; mistaking stability under extraordinary project support for sustainable steady-state capacity

## Self-Check

- Is hypercare scope defined by users, services, systems, sites, processes, issue types, hours, channels, and exclusions?
- Is the purpose clear: service protection, defect discovery, adoption, workload stabilization, control validation, or knowledge transfer?
- Are severity rules based on impact, safety, compliance, revenue, data, workaround, frequency, and burden?
- Is there one source of truth for issues, decisions, owners, status, severity, dependencies, and closure evidence?
- Are ownership rules defined across project, operations, support, vendor, product, and IT?
- Are workaround, scope, communication, and risk-acceptance authorities clear?
- Are stabilization indicators tracked for volume, severity, repeats, backlog, SLA, quality, adoption, manual work, escalations, training, controls, complaints, and support load?
- Do temporary workarounds and controls have owner, reason, risk, expiry, and retirement or formalization plan?
- Are status updates audience-specific and tied to runbook, FAQ, macro, and training updates?
- Are exit criteria signed off by operations, support, and project owners with residual risks and normal governance in place?; has the team verified that steady-state staffing and governance can sustain the service without temporary hypercare support?
