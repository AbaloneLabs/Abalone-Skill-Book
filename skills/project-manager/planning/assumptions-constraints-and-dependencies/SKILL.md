---
name: assumptions_constraints_and_dependencies.md
description: Use when the agent is identifying and managing project assumptions, constraints, and dependencies, documenting what the plan relies on being true, resolving cross-project dependencies, or diagnosing why hidden assumptions or unmanaged dependencies caused project failure.
---

# Assumptions Constraints And Dependencies

Every project plan rests on things believed to be true but unverified (assumptions), things that limit the solution space (constraints), and things the project needs from outside itself (dependencies). These three are the hidden foundation of the plan, and when they are unexamined, they become the causes of failure. Assumptions that turn out false invalidate the plan; constraints that are ignored produce infeasible solutions; dependencies that are unmanaged produce delays waiting on others. The judgment problem is that these elements are invisible in normal status reporting and surface only when they break, by which time recovery is costly. The most common failure is proceeding on unvalidated assumptions, treating constraints as flexible, and leaving dependencies to hope. The skill is surfacing, documenting, validating, and actively managing assumptions, constraints, and dependencies throughout the project.

Use this skill before finalizing a plan, before committing to dates, before identifying what could go wrong, or when a project is failing due to causes no one surfaced earlier. The goal is to prevent the agent from proceeding on unexamined assumptions, from ignoring hard constraints, or from leaving dependencies unmanaged.

## Core Rules

### [ ] Surface Assumptions Explicitly And Document Them

Assumptions are the beliefs the plan depends on but has not verified: that a vendor will deliver on time, that a regulation will not change, that a resource will be available. Left implicit, they are invisible risks. Surfacing and documenting them makes them examinable and manageable.

- [ ] List every assumption the plan depends on.
- [ ] Document each assumption with its basis and impact if false.
- [ ] Review assumptions with stakeholders to surface additional ones.
- [ ] Treat the assumption log as a living document throughout the project.

### [ ] Validate Critical Assumptions Early

Not all assumptions matter equally. Critical assumptions, those whose falsity would seriously harm the project, must be validated early through research, confirmation, or testing. Validating early prevents building a plan on a false foundation.

- [ ] Prioritize assumptions by impact if false.
- [ ] Validate critical assumptions before the plan depends on them.
- [ ] Convert validated assumptions into confirmed facts.
- [ ] Treat unvalidated critical assumptions as risks.

### [ ] Distinguish Hard Constraints From Soft Preferences

Constraints are limits the project must operate within: a fixed date, a fixed budget, a regulatory requirement, a technology standard. Hard constraints cannot be changed; soft preferences can. Confusing the two produces plans that either violate real constraints or over-constrain on mere preferences.

- [ ] Identify all constraints: time, cost, scope, technology, regulatory, organizational.
- [ ] Classify each as hard (immutable) or soft (negotiable).
- [ ] Design the solution within hard constraints.
- [ ] Challenge and negotiate soft constraints where they limit value.

### [ ] Manage The Iron Triangle Of Constraints Explicitly

Time, cost, and scope form the classic constraint triangle: changing one affects the others. Treating all three as fixed when reality demands change produces failure. The project manager must make the tradeoffs explicit and negotiate which constraint flexes.

- [ ] Recognize which of time, cost, scope is the priority and which can flex.
- [ ] Make tradeoffs explicit when one constraint must change.
- [ ] Negotiate constraint changes with stakeholders rather than absorbing them silently.
- [ ] Avoid pretending all three are fixed when they are not.

### [ ] Identify Internal And External Dependencies

Dependencies are relationships where one piece of work depends on another. Internal dependencies are within the project or team; external dependencies are on other teams, vendors, or organizations. External dependencies are riskier because they are outside the project's control and must be actively managed.

- [ ] Map dependencies between tasks, teams, vendors, and external parties.
- [ ] Distinguish internal dependencies (controllable) from external (influence only).
- [ ] Identify the critical path dependencies that most affect schedule.
- [ ] Document who owns each dependency and the handoff criteria.

### [ ] Manage External Dependencies Proactively

External dependencies fail silently and surface as delays. The project manager cannot control them but must manage them through early engagement, clear agreements, regular check-ins, and contingency. Waiting passively for an external dependency to resolve is a common cause of slippage.

- [ ] Engage dependency owners early and secure their commitment.
- [ ] Define clear handoff criteria and dates.
- [ ] Check in regularly on external dependency progress, not just at due dates.
- [ ] Build contingency for external dependency failure.

### [ ] Resolve Circular And Conflicting Dependencies

Dependencies can be circular (A depends on B which depends on A) or conflicting (multiple dependencies with incompatible requirements). These create deadlock that prevents progress. Resolving them requires restructuring work, negotiating, or escalating.

- [ ] Detect circular dependencies that create deadlock.
- [ ] Restructure work to break circular dependencies.
- [ ] Identify and resolve conflicting dependency requirements.
- [ ] Escalate dependencies that cannot be resolved at the project level.

### [ ] Integrate Assumptions, Constraints, And Dependencies With Risk

Assumptions that may be false, constraints that may conflict, and dependencies that may fail are all sources of risk. These elements should feed directly into risk management, not be tracked separately. Integration ensures they receive active risk response.

- [ ] Convert critical unvalidated assumptions into risks.
- [ ] Identify risks arising from constraint conflicts.
- [ ] Treat external dependency failures as risks with responses.
- [ ] Keep assumption, constraint, and dependency logs aligned with the risk register.

### [ ] Review And Update Throughout The Project

Assumptions, constraints, and dependencies are not static; they evolve as the project progresses and the environment changes. New ones emerge; existing ones resolve or fail. Regular review keeps them current and prevents stale assumptions from misleading the plan.

- [ ] Review assumptions, constraints, and dependencies at regular intervals.
- [ ] Update the logs as new ones emerge and existing ones change.
- [ ] Validate that previously confirmed assumptions still hold.
- [ ] Communicate changes to stakeholders who depend on them.

### [ ] Communicate Dependencies And Constraints To Stakeholders

Stakeholders often hold assumptions about what the project can deliver and when, and unspoken constraints and dependencies create false expectations. Communicating them transparently aligns expectations and surfaces negotiation opportunities. Transparency prevents later disappointment.

- [ ] Communicate key assumptions, constraints, and dependencies to stakeholders.
- [ ] Explain how they affect schedule, cost, and scope.
- [ ] Surface where stakeholder action could resolve a constraint or dependency.
- [ ] Avoid letting stakeholders operate on different assumptions than the project.

## Common Traps

### [ ] Implicit Assumptions

Proceeding on beliefs that are never surfaced, documented, or validated.

### [ ] Critical Assumptions Left Unvalidated

Building the plan on assumptions whose falsity would be fatal, without checking them.

### [ ] Constraint Confusion

Treating hard constraints as flexible or soft preferences as immutable.

### [ ] All-Three-Fixed Illusion

Pretending time, cost, and scope are all fixed when reality requires tradeoffs.

### [ ] Passive Dependency Management

Waiting for external dependencies to resolve rather than actively managing them.

### [ ] Circular Deadlock

Dependencies that create circular waits, paralyzing progress.

### [ ] Separate Tracking From Risk

Managing assumptions and dependencies separately from risk, leaving them unaddressed.

### [ ] Stale Logs

Assumption and dependency logs created once and never updated as reality changes.

## Self-Check

- [ ] Are assumptions surfaced explicitly, documented, and reviewed with stakeholders?
- [ ] Are critical assumptions validated early before the plan depends on them?
- [ ] Are hard constraints distinguished from soft preferences and respected?
- [ ] Is the iron triangle of time, cost, and scope managed with explicit tradeoffs?
- [ ] Are internal and external dependencies mapped with owners and handoff criteria?
- [ ] Are external dependencies managed proactively with engagement and contingency?
- [ ] Are circular and conflicting dependencies detected and resolved?
- [ ] Do assumptions, constraints, and dependencies feed into integrated risk management?
- [ ] Are the logs reviewed and updated regularly as the project and environment evolve?
- [ ] Are key dependencies and constraints communicated transparently to stakeholders?
