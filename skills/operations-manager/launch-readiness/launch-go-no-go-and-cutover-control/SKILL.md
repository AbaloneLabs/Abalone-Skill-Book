---
name: launch-go-no-go-and-cutover-control.md
description: Use when the agent is preparing or reviewing a launch go/no-go decision, cutover plan, phased rollout, readiness gate, launch hold, rollback, contingency plan, acceptance of launch risk, or operational decision to proceed, delay, narrow scope, or revert.
---

# Launch Go No Go And Cutover Control

The go/no-go decision is where launch pressure can override operational judgment. Teams may confuse "we want to launch" with "the operation can safely launch," especially after deadlines, marketing commitments, or executive visibility have built momentum. This skill helps the agent structure the launch decision, cutover, and rollback so proceeding, delaying, narrowing scope, or reverting are explicit choices based on evidence.

## Core Rules

### Define decision authority before the meeting

Name who can approve go, no-go, partial go, delayed go, rollback, or scope reduction. The decision owner should have authority to accept operational risk and the credibility to resist schedule pressure. Contributors should provide evidence, not silently assume someone else will object.

Separate recommendation from approval. Operations, product, engineering, support, legal, compliance, finance, vendors, and customer-facing teams may each own readiness evidence, but the final decision must be traceable.

### Use gate criteria that can actually stop launch

Readiness gates should include measurable criteria for must-have processes, staffing, tools, support, controls, training, vendor readiness, communications, monitoring, data, and rollback. A gate that cannot stop or change the launch is only a status update.

Classify issues as launch blockers, launch constraints, accepted risks, and post-launch follow-ups. Define the threshold for each before reviewing the final risk list.

### Review residual risk, not only open tasks

A launch can have all tasks closed and still carry unacceptable risk. Review unresolved assumptions, weak test evidence, manual workarounds, limited coverage, vendor uncertainty, unclear customer messaging, control gaps, known defects, and recovery limits.

For each residual risk, identify impact, likelihood, detection method, owner, mitigation, acceptance authority, expiry, and trigger for reassessment. Accepted risk should not disappear after launch.

### Make cutover steps operationally executable

Cutover plans should include sequence, timing, owner, dependency, system state, communication, customer or user impact, validation evidence, and hold point. The plan should be granular enough for execution but not so detailed that it becomes unreadable during pressure.

Include pre-cutover checks, launch-window roles, command channel, issue logging, status cadence, validation checks, and post-cutover stabilization. If the launch spans time zones, sites, or shifts, define handoff.

### Control freezes and change windows

During launch, unrelated changes can create noise and mask root causes. Define code freeze, process freeze, configuration freeze, vendor change freeze, communication freeze, or inventory freeze where appropriate. Also define who can approve urgent exceptions.

Do not freeze so broadly that legitimate incident response is blocked. The freeze should protect the launch, not paralyze the operation.

### Define rollback and roll-forward options

Rollback should not be a vague promise. Define what can be reversed, what cannot, how long rollback remains possible, who approves it, what data or customer impact must be reconciled, and what communication is required. Sometimes roll-forward with mitigation is safer than rollback; make that tradeoff explicit.

If no rollback exists, the go/no-go decision must recognize that the launch is irreversible or costly to unwind.

### Use phased rollout deliberately

Phasing can reduce risk if each phase has learning goals, success criteria, monitoring, support capacity, and stop rules. Phasing is weak if it merely spreads confusion over time or launches to a group that does not represent the real operating complexity.

Choose pilot audiences, regions, sites, or customer groups based on what risk needs to be tested, not only political convenience.

### Capture the decision record

The final decision should record decision, scope, time, owner, evidence, blockers resolved, accepted risks, conditions, communication plan, rollback triggers, and next review. This protects teams from later ambiguity and helps post-launch review.

If a launch proceeds over operational objections, document the objection and the owner accepting the risk.

## Common Traps

- Letting the calendar, marketing date, or executive preference become the real go/no-go criterion.
- Holding a readiness meeting after the decision has already been made informally.
- Using gate criteria that cannot change the launch decision.
- Treating closed tasks as proof that residual risk is acceptable.
- Accepting risks without owner, mitigation, trigger, expiry, and authority.
- Writing a cutover plan that lacks owners, validation checks, hold points, and shift handoffs.
- Freezing changes so broadly that incident response is slowed, or so loosely that launch signals are noisy; claiming rollback exists without testing what data, customer commitments, or process changes cannot be reversed
- Running a pilot that does not test the real operational risk; failing to record who accepted the risk when operations recommended delay or scope reduction

## Self-Check

- Are go, no-go, partial go, delay, scope reduction, and rollback authorities named?
- Are recommendation contributors separated from final approval owner?
- Can readiness gate criteria actually stop, narrow, or delay launch?
- Are issues classified as blockers, constraints, accepted risks, and post-launch follow-ups?
- Are residual risks reviewed for impact, likelihood, detection, mitigation, owner, authority, trigger, and expiry?
- Does the cutover plan include sequence, timing, owner, dependency, communication, validation, hold point, and handoff?
- Are launch-window roles, command channel, issue log, status cadence, and post-cutover checks defined?
- Are freezes and urgent exception paths calibrated to protect launch without blocking response?
- Are rollback and roll-forward options specific about reversibility, approval, data reconciliation, customer impact, and communication?
- Is the decision record clear on scope, evidence, accepted risks, objections, conditions, triggers, and next review?
