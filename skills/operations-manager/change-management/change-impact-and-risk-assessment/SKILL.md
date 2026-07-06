---
name: change-impact-and-risk-assessment.md
description: Use when the agent is assessing the impact, blast radius, operational risk, stakeholder exposure, control implications, go/no-go criteria, or approval basis for a process, tool, staffing, policy, vendor, workflow, service, or operating model change before it is implemented.
---

# Change Impact And Risk Assessment

Operational change often looks smaller than it is because the visible request is usually a new step, tool setting, staffing adjustment, policy update, vendor change, or workflow tweak. The real risk sits in what the change touches: queues, handoffs, customer promises, controls, exception paths, reporting, training, and staff workload. This skill helps the agent avoid treating change impact as a generic pros-and-cons exercise. The agent should make the operating consequences explicit before recommending, approving, scheduling, or communicating a change.

## Core Rules

### Define the change in operational terms

Start by stating what will be different after the change, not only what artifact will be delivered. Name the current process, the future process, the affected locations or teams, the roles whose daily work changes, and the first date when the change affects live work. A change is not fully described until the agent can explain who will do something differently, what decision or task moves to another place, what data or system state changes, and what customers, employees, vendors, or regulators may experience differently.

Separate the stated purpose from the proposed method. A request such as "move intake to a new form" may be trying to reduce rework, improve audit evidence, route work faster, standardize eligibility checks, or lower staffing effort. The impact assessment should test whether the method actually serves the purpose and whether another path would reduce risk.

### Map the operating surface, not only direct users

Identify every surface that may be affected: request intake, eligibility rules, queue priority, staffing coverage, training, quality review, approvals, exception handling, customer communication, data entry, reporting, billing, compliance evidence, vendor dependencies, facilities or assets, and incident response. Include downstream consumers who may not touch the changed process directly but rely on its output.

Look especially for hidden work. Changes often create manual reconciliation, duplicate entry, informal coaching, increased supervisor approvals, more customer questions, temporary spreadsheets, or extra handoff meetings. These loads may not appear in the implementation plan but can decide whether the change succeeds.

### Tier risk by consequence and reversibility

Do not rely only on probability labels. Classify the change by blast radius, consequence, reversibility, detectability, customer visibility, legal or compliance exposure, safety exposure, and ability to pause. A low-likelihood failure that affects payroll, regulated records, critical service access, safety, or large customer populations may require stronger controls than a common but easily reversed inconvenience.

Ask what happens if the change is wrong for one hour, one day, one week, and one reporting cycle. The answer should shape pilot size, approval level, monitoring, rollback readiness, and communications. Reversible changes can move faster if detection is strong. Irreversible or hard-to-detect changes need more evidence before rollout.

### Compare normal, edge, and exception paths

Assess impact across the whole operating pattern, not only the clean case. Check peak volume, low staffing, new hires, regional holidays, system downtime, urgent escalations, VIP or vulnerable customers, regulated cases, unusual pricing, backdated work, reopens, corrections, refunds, and vendor delays. Many changes pass a happy-path review and fail when an exception path is common enough to matter.

The agent should ask whether the new process preserves the old safeguards for exceptions or replaces them with assumptions. If a workaround is required, name who can use it, when, how it is recorded, and how it is retired.

### Verify evidence quality before accepting estimates

Impact estimates should be tied to evidence: volume history, queue aging, defect rates, staffing rosters, training completion, incident records, audit findings, complaint themes, system logs, vendor SLAs, and frontline observations. Treat one anecdote, a stale dashboard, or a manager's memory as a lead, not a basis for approval.

Where evidence is incomplete, make the uncertainty visible. State what is known, what is assumed, what could change the decision, and how uncertainty will be reduced. A good assessment can still recommend action under uncertainty, but it should not hide the uncertainty inside confident language.

### Decide mitigation, owner, and residual risk

For each material risk, define a prevention control, detection control, response owner, and time limit. Mitigation should be operationally real: staffing coverage, cutover checks, quality sampling, supervisor review, system alerts, approval gates, temporary capacity, customer scripts, vendor confirmation, or rollback criteria. "Monitor closely" is not a control unless it says who monitors what signal at what frequency and what they do when the signal breaches.

If the organization accepts residual risk, name the accountable decision maker and the reason. Residual risk should not be accepted by silence. It should be tied to urgency, expected benefit, alternatives considered, and the cost of delay.

### Turn the assessment into a decision basis

End with a clear recommendation: proceed, pilot, defer, redesign, escalate, or reject. Include go/no-go criteria, required approvals, preconditions, monitoring signals, communications, support coverage, and rollback readiness. The assessment should be useful to someone who was not in the discussion and needs to understand why the change was allowed or blocked.

## Common Traps

- Treating implementation effort as impact. A change can be easy to configure and still create high operational risk if it changes controls, queues, customer promises, or exception handling.
- Asking only the project owner who is affected. Project owners often see planned users and miss downstream teams, quality reviewers, finance, compliance, vendors, and customer-facing staff.
- Using a risk matrix without operational meaning. Red-yellow-green labels are weak unless they connect to approval level, rollout size, monitoring, rollback, and support coverage.
- Ignoring temporary states. Migration periods, dual-running, partial adoption, and mixed old-new cases can create more risk than the final steady state.
- Assuming training solves design flaws. If the change requires people to remember many exceptions, override confusing defaults, or reconcile bad data manually, training may only hide a brittle process.
- Forgetting reporting and evidence. A changed workflow can break metrics, audit trails, reconciliation, or performance targets even when frontline execution still works.
- Letting urgency erase ownership. Fast changes still need accountable approval, known residual risk, and a named person watching the first live results.

## Self-Check

- Is the change described in terms of what work, decision, data, control, or customer experience will actually change?
- Have direct users, downstream teams, customers, vendors, reporting consumers, and control owners been considered?
- Are risks tiered by consequence, reversibility, detectability, blast radius, and sensitivity, not only by rough probability?
- Have normal, peak, low-staffing, exception, error-correction, and regulated cases been assessed?
- Are impact estimates tied to current evidence, with assumptions and unknowns called out separately?
- Does each material risk have a prevention or detection control, response owner, and escalation path?
- Is any accepted residual risk explicit, justified, and owned by someone with authority?
- Does the final recommendation include proceed, pilot, defer, redesign, escalate, or reject with concrete go/no-go criteria?
