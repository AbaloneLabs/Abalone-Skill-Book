---
name: operational-acceptance-and-readiness-gates.md
description: Use when the agent is defining operational acceptance criteria, readiness gates, go-live acceptance, run-state entry criteria, operational risk signoff, support readiness, handover evidence, or conditions for an operations team to accept a project deliverable.
---

# Operational Acceptance And Readiness Gates

Operational acceptance is the point where a project stops being a project artifact and becomes something operations must run. A project can meet scope, budget, and schedule while still being unacceptable for run state because support, controls, training, monitoring, documentation, or ownership are incomplete. This skill helps the agent define readiness gates that protect the operating team from inheriting unsupported work.

## Core Rules

### Define what operations is accepting

Name the process, system, service, site, tool, vendor, policy, workflow, or capability that operations will own. Specify what changes for customers, employees, queues, staffing, systems, vendors, reporting, controls, and service levels.

Do not accept a vague handover such as "the project is live." Operations needs to know the run-state object and the responsibilities it inherits.

### Separate project completion from operational readiness

Project completion may mean deliverables were built. Operational readiness means the deliverables can be supported, monitored, improved, and controlled during normal work. Define separate gates for build complete, test complete, business ready, support ready, control ready, and operations accepted.

This distinction helps teams avoid pressure to accept work simply because the project timeline ended.

### Make acceptance criteria evidence-based

Acceptance should require evidence: signed process maps, test results, training completion, access setup, monitoring dashboards, support queue configuration, runbooks, escalation paths, control evidence, data validation, vendor readiness, and open-risk log. Evidence should be reviewed by the team that will own the work.

A checklist without evidence can become ceremony. A gate should be able to stop, delay, or narrow the transition.

### Include non-functional operating needs

Projects often focus on features and outputs while operations needs capacity, reliability, security, privacy, accessibility, maintainability, reporting, performance, auditability, recoverability, and supportability. Include these needs in acceptance criteria.

If non-functional needs are deferred, record owner, risk, deadline, compensating control, and whether operations accepts the temporary condition.

### Define defect and open-risk thresholds

Not every defect blocks acceptance, but some should. Classify open items by severity, customer impact, operational impact, workaround availability, control risk, and reversibility. Define which defects block transition, which can enter hypercare, and which are backlog items.

Avoid accepting many small unresolved items that together create large operating burden.

### Confirm support and escalation readiness

Before acceptance, support teams should know intake path, categories, ownership, severity rules, escalation contacts, response expectations, known issues, temporary workarounds, and communication templates. If support is not ready, customers and employees will discover the gaps first.

Support readiness includes after-hours and regional coverage where relevant.

### Ensure retained project knowledge transfers

Projects hold design context, decision history, assumptions, and tradeoffs. Operations needs enough of that context to diagnose issues later. Transfer architecture notes, design decisions, process assumptions, vendor commitments, configuration details, and known constraints.

Do not let key project knowledge disappear when contractors or project staff roll off.

### Record acceptance decision and conditions

The final acceptance decision should state accepted scope, excluded scope, evidence reviewed, open risks, conditions, owners, due dates, support model, hypercare plan, and escalation path. Conditional acceptance should have expiry and review date.

If operations rejects or delays acceptance, document the reason and required evidence to proceed.

### Protect operations from silent scope expansion

Projects sometimes add small responsibilities during testing, rollout, or stakeholder negotiation that were not in the original operating model. Before acceptance, compare original scope to actual delivered scope and the work users now expect operations to perform.

If scope expanded, update staffing, budget, service levels, controls, and ownership before accepting. Silent expansion becomes permanent unfunded work.

## Common Traps

- Treating go-live as proof that operations can run the result.
- Accepting a project deliverable without defining the run-state object and responsibilities.
- Using readiness checklists without evidence or authority to stop transition.
- Ignoring non-functional needs such as supportability, auditability, security, recovery, and reporting.
- Accepting many "minor" defects that create major operational burden together.
- Leaving support queues, categories, severity rules, and escalation paths undefined; assuming project staff will remain available after handover
- Losing design assumptions and decision history when vendors or contractors leave; recording acceptance informally in a meeting without conditions, owners, or due dates
- Pressuring operations to accept because the project budget or timeline ended; accepting hidden scope expansion without revising capacity, funding, controls, and service expectations

## Self-Check

- Is the process, system, service, site, vendor, tool, policy, workflow, or capability being accepted clearly defined?
- Are customer, employee, queue, staffing, system, vendor, reporting, control, and service-level changes explicit?
- Are project completion and operational readiness separated into meaningful gates?
- Does acceptance require evidence such as test results, training, access, monitoring, runbooks, controls, data validation, vendor readiness, and risk log?
- Can the gate stop, delay, narrow, or condition the transition?
- Are capacity, reliability, security, privacy, accessibility, maintainability, auditability, recovery, reporting, and supportability included?
- Are open defects classified by severity, impact, workaround, control risk, and reversibility?
- Are support intake, categories, ownership, severity, escalation, known issues, workarounds, templates, and coverage ready?
- Has project design context, assumptions, configuration, vendor commitment, and decision history transferred?
- Is acceptance recorded with scope, exclusions, evidence, open risks, conditions, owners, dates, support model, hypercare, and escalation path?; has actual delivered scope been compared with original scope and expected run-state workload?
