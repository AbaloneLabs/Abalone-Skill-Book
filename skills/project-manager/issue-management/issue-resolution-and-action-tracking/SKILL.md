---
name: issue_resolution_and_action_tracking.md
description: Use when the agent is driving project issues to resolution, assigning owners and due dates, tracking resolution actions, deciding between a permanent fix and a temporary workaround, or reviewing whether open issues are actually progressing toward closure.
---

# Issue Resolution and Action Tracking

Triage decides what matters; resolution is the work of actually closing issues. The judgment problem is that issues, once logged and prioritized, often stall. They sit in the queue with an owner and a due date but no real progress, because the owner is overloaded, the root cause is hard, the fix requires resources that are not available, or the team has accepted a workaround as if it were a solution. A queue full of aged open issues is not a sign of careful work; it is a sign that resolution is not being driven. The skill is to convert each issue into concrete, owned, dated actions, distinguish a real fix from a temporary workaround, and track progress tightly enough to detect when an issue is drifting.

Agents tend to confuse activity with progress and workarounds with resolutions. The discipline here is to make resolution actions explicit and verifiable, to decide consciously when a workaround is acceptable and when it is not, and to close issues only when the underlying problem is genuinely resolved, not when the symptoms have been quieted.

## Core Rules

### Convert Each Issue Into Concrete Resolution Actions

An issue is not resolved by intent; it is resolved by specific actions. Break each issue into the discrete steps required to close it: investigate root cause, design the fix, implement, test, deploy, confirm. Each action should have a single owner, a due date, and a definition of done. An issue with no actions is waiting; an issue with actions but no owners or dates is also waiting. The action list is what turns a problem into managed work.

### Distinguish Root-Cause Fix From Workaround, and Decide Deliberately

A root-cause fix eliminates the problem; a workaround contains its symptoms without addressing the cause. Both are legitimate, but they are different decisions with different consequences. A workaround may be correct when the fix is too expensive, too slow, or too risky for now, but it leaves the problem in place and usually creates ongoing cost or technical debt. Make the choice explicit: is this issue being fixed or worked around, who decided, and what is the cost of leaving the cause unaddressed? Silent workarounds accumulate into fragile systems.

### Assign Owners Based on Authority and Capability, Not Availability

The owner of a resolution action should be the person with both the capability to do the work and the authority to make the necessary decisions. Assigning an issue to whoever is free, regardless of whether they can resolve it, guarantees delay. If the right owner is overloaded, that is a capacity problem to escalate, not a reason to misassign. Confirm the owner accepts the action and the due date; unaccepted assignments are not commitments.

### Set Realistic Due Dates Derived From the Work, Not From Wishful Targets

Due dates should reflect the actual effort and dependencies of the resolution, not a desired speed. A date set from optimism becomes a fiction that everyone ignores, which erodes the meaning of all dates in the queue. Estimate the actions, account for dependencies and the owner's other commitments, and set a date the owner genuinely expects to meet. If the needed date is earlier than the realistic one, that is a constraint to escalate, not a reason to write down a number no one believes.

### Track Progress Against Actions, Not Just Issue Status

A binary issue status of "open" or "in progress" hides whether anything is actually moving. Track progress at the action level: which actions are done, which are in flight, which are blocked, and which are overdue. Action-level tracking reveals a stalled issue long before its due date arrives, giving time to intervene. Review action progress in regular issue meetings and treat overdue actions as signals requiring attention, not as minor slippage.

### Define Closure Criteria Before Declaring an Issue Resolved

State what evidence will confirm the issue is resolved: a test passing, a metric returning to normal, a stakeholder confirming the problem is gone, a period of stable operation. Without predefined closure criteria, issues get closed when the symptoms fade, only to recur. Require the evidence before changing status to resolved, and for workarounds, define the monitoring that confirms the workaround is still holding. Closure on criteria, not on hope, prevents recurring surprises.

### Decide Explicitly When to Accept a Workaround as Permanent

Some issues will never get a root-cause fix because the cost exceeds the benefit, and that is a valid outcome, but it must be a conscious decision with a documented rationale and an owner for the ongoing workaround. Accepting a workaround by default, simply because the fix is hard, leaves a portfolio of latent problems. Review accepted workarounds periodically to confirm the cost-benefit still holds and to revisit whether a fix has become worthwhile.

### Close the Loop With the Reporter and Affected Stakeholders

Resolution is not complete until the people affected by the issue know it is resolved. Confirm closure with the reporter and relevant stakeholders, and record what was done. This closes the loop operationally and builds trust that raising issues leads to action. Issues closed silently leave stakeholders unsure whether their problem was addressed and less likely to report the next one.

## Common Traps

### Treating a Workaround as a Fix

Implementing a workaround and marking the issue resolved leaves the root cause in place, so the problem recurs or creates ongoing cost. The trap is that the workaround feels like progress and the symptom is quiet. Make the fix-versus-workaround decision explicit and track accepted workarounds as ongoing exposure.

### Issues With Owners and Dates but No Actions

An issue sits in the queue with an owner and due date but no concrete steps, so nothing happens. The trap is that the metadata looks like management. Break each issue into specific, owned, dated actions.

### Assigning to Whoever Is Available

Routing issues to free resources regardless of whether they can resolve them guarantees delay and rework. The trap is that assignment feels like progress. Assign based on authority and capability, and escalate capacity gaps rather than misassigning.

### Due Dates No One Believes

Optimistic dates get written down to look responsive, then missed, eroding the credibility of the whole queue. The trap is that ambitious dates feel committed. Set dates derived from realistic estimates, and escalate when the needed date is infeasible.

### Binary Status Hiding Drift

An issue marked "in progress" for weeks conceals that no actions are moving. The trap is that the status looks active. Track progress at the action level so stalled issues surface early.

### Closing on Symptom Relief

Issues get closed when symptoms fade, without confirming the root cause is addressed, then recur. The trap is that relief feels like resolution. Define and require closure evidence before changing status.

### Silent Closure

Issues are closed without informing the reporter or affected stakeholders, who remain unsure whether the problem was addressed. The trap is that silent closure avoids a conversation. Close the loop with the people affected to confirm resolution and sustain trust.

## Self-Check

- [ ] Is each open issue broken into concrete resolution actions with single owners and due dates?
- [ ] Is the fix-versus-workaround decision made explicitly, with rationale and tracked exposure for accepted workarounds?
- [ ] Are owners assigned based on authority and capability, with acceptance confirmed, rather than on availability?
- [ ] Are due dates derived from realistic estimates of the work and the owner's commitments?
- [ ] Is progress tracked at the action level, not just through binary issue status?
- [ ] Are overdue actions treated as signals requiring intervention rather than minor slippage?
- [ ] Are closure criteria defined and required as evidence before an issue is marked resolved?
- [ ] Are accepted permanent workarounds documented with rationale, owner, and periodic review?
- [ ] Is closure confirmed with the reporter and affected stakeholders, not done silently?
- [ ] Are recurring issues reviewed to check whether earlier closures were premature?
