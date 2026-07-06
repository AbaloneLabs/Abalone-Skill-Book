---
name: incident-recovery-and-normalization.md
description: Use when the agent is planning incident recovery, returning operations to normal, lifting containment, reconciling affected work, clearing incident backlog, validating service restoration, managing residual risk, or handing an incident from emergency response into stable operational ownership.
---

# Incident Recovery And Normalization

Recovery is not just making the immediate symptom stop. Operations must verify that service is restored, affected work is reconciled, customers or internal users are handled, temporary controls are lifted safely, and residual risks are owned. Agents often declare an incident done when a workaround is in place or the queue starts moving again. This skill helps the agent manage the transition from emergency response to stable operations without leaving hidden debt behind.

## Core Rules

### Define What Recovery Means

State the recovery target in operational terms: process restored, service level stable, backlog under threshold, data reconciled, customers notified, vendor action complete, physical site reopened, control restored, or workaround stable. Different incidents require different definitions.

Do not confuse containment with recovery. A paused workflow may stop harm, but the operation is not recovered until the paused work and affected parties are handled.

### Validate Restoration With Evidence

Use evidence to confirm restoration: successful transactions, queue movement, quality samples, system logs, customer confirmations, inventory counts, reconciliation reports, vendor confirmation, staff observations, and control checks. Do not rely only on "no new reports" when monitoring may be weak.

Define who validates recovery and what evidence is enough. High-risk incidents may require compliance, safety, security, finance, or quality signoff.

### Manage Backlog And Deferred Work

Incidents create deferred work: delayed cases, missed callbacks, unprocessed orders, manual reviews, quality checks, reconciliations, customer remediation, vendor follow-up, and documentation. Build a recovery queue with priority, owner, expected completion, and communication rules.

Avoid clearing backlog by skipping evidence, closing prematurely, or ignoring lower-visibility segments. Recovery must not create a second service failure.

### Lift Containment Deliberately

Containment actions should have removal criteria. Before lifting a pause, workaround, manual review, access restriction, or routing change, confirm the original risk is controlled and that staff know the current rule.

If a temporary workaround remains, assign owner, monitoring, expiration, and review. Temporary incident controls often become risky shadow processes if not managed.

### Reconcile Customer And Internal Commitments

Review what was promised during the incident: callbacks, refunds, replacements, manual reviews, executive updates, vendor actions, service credits, regulatory notices, or internal reports. Every promise needs owner and due date.

Do not rely on memory or chat. Use a tracked list so commitments survive shift changes and incident closure.

### Watch For Secondary Failures

Recovery can strain the operation. Backlog burn-down may increase defects, overtime, staff fatigue, customer confusion, or downstream bottlenecks. Monitor quality, rework, near breaches, staffing load, and customer complaints after service resumes.

If recovery actions create new risk, adjust priorities or communicate revised expectations.

### Handoff To Normal Ownership

When incident command ends, define who owns remaining work: process owner, queue lead, vendor manager, quality lead, compliance owner, finance, customer success, or project team. Include open risks, outstanding decisions, temporary controls, affected cases, and follow-up dates.

A handoff is not complete because a message was sent. The receiving owner must have enough context and authority to act.

### Capture Learning Without Delaying Recovery

Do not wait for a full postmortem to fix obvious stabilization gaps. If runbooks, access, customer messaging, or routing rules are clearly wrong, assign immediate corrective actions. Deeper root-cause analysis can follow.

Separate recovery actions from prevention actions. Both matter, but they have different urgency and owners.

### Declare Closure With Residual Risk Clear

Closure should state what is restored, what remains open, what risk is accepted, who owns follow-up, and whether further communication or post-incident review is planned. Closure without residual risk ownership is just a status label.

If customers or internal users still experience impact, use stabilized or workaround language rather than full resolution.

## Common Traps

- Calling the incident resolved when the visible symptom stops but deferred work remains.
- Using "no new reports" as proof of recovery without validating monitoring and samples.
- Clearing incident backlog by skipping quality, documentation, or control requirements.
- Leaving temporary workarounds in place with no owner or expiration.
- Forgetting customer, vendor, executive, or internal commitments made during the incident.
- Ending incident command before remaining work has normal owners.
- Failing to monitor secondary failures caused by recovery pressure; treating post-incident review as a reason to delay obvious recovery fixes
- Communicating full resolution when the operation is only stabilized by a workaround; losing affected-case lists, decision logs, or evidence during the transition to normal work

## Self-Check

- Is recovery defined in observable operational terms, not just symptom disappearance?
- Has restoration been validated with evidence appropriate to the incident risk?
- Are deferred work, backlog, reconciliation, remediation, and quality checks owned?
- Are containment actions lifted only after removal criteria are met?
- Are temporary workarounds assigned owner, monitoring, expiration, and review?
- Are all customer, vendor, executive, and internal commitments tracked to closure?
- Are secondary failures such as defects, overtime, fatigue, complaints, and downstream bottlenecks monitored?
- Has remaining work been handed off to normal owners with context and authority?
- Are immediate stabilization fixes separated from deeper prevention work?
- Does closure state restored scope, remaining risk, owners, and follow-up communication?
