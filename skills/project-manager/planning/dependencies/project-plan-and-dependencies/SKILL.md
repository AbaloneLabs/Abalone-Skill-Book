---
name: project_plan_and_dependencies.md
description: Use when the agent is building or reviewing a project plan, sequencing work, estimating milestones, mapping dependencies, coordinating teams, or deciding whether a project schedule is realistic.
---

# Project Plan And Dependencies

A project plan is not a list of wishes with dates. It is a model of how work becomes deliverable under constraints. The project manager must understand task sequence, dependency chains, decision points, review cycles, capacity limits, critical path, and uncertainty. A plan that ignores dependencies creates avoidable surprises even when every individual task looks reasonable.

Use this skill before creating schedules, milestone plans, delivery roadmaps, dependency maps, resourcing plans, or execution trackers. The goal is to prevent the agent from producing a timeline that is tidy but not executable.

## Core Rules

### Break Work By Deliverable And Decision Point

Start from deliverables and decisions, then decompose into work. Avoid task lists that mix outcomes, meetings, reviews, and vague activities at the same level.

For each work item, define:

- deliverable or decision produced;
- owner;
- inputs required;
- dependencies;
- expected duration or effort range;
- review or approval needed;
- completion evidence;
- next work unlocked.

"Discuss integration" is not a deliverable. "Approve integration contract" or "complete integration spike and decision memo" is more actionable.

### Sequence Before Dating

Do not assign dates before knowing order. First map what must precede what.

Dependency types include:

- finish-to-start: one task must finish before another starts;
- start-to-start: one task can begin after another begins;
- external dependency: vendor, customer, legal, procurement, platform, partner;
- resource dependency: same person or team cannot do both at once;
- decision dependency: approval or tradeoff resolution required;
- information dependency: data, requirements, design, or test results needed.

After the sequence is understood, dates can be attached with more honesty.

### Identify The Critical Path

The critical path is the chain of work that determines the earliest possible completion. A delay on the critical path delays the project unless scope, resources, or sequence changes.

Ask:

- Which tasks cannot slip without moving the final date?
- Which approvals sit on the critical path?
- Which dependencies are outside direct control?
- Where is there slack?
- Which tasks look urgent but do not affect delivery?

Focusing only on busy tasks can hide the real schedule risk.

### Include Review, Rework, And Decision Time

Plans often assume work is accepted immediately. Real projects need review, feedback, revision, decision making, and sometimes escalation.

Include time for:

- design review;
- technical review;
- security and privacy review;
- legal approval;
- stakeholder feedback;
- customer validation;
- QA cycles;
- vendor response;
- content or translation review;
- training and support readiness.

If the plan has no rework allowance, it assumes perfect first attempts.

### Plan With Capacity, Not Only Duration

A task that takes two days of effort may take two weeks on the calendar if the owner is shared across projects. Capacity is often the real constraint.

Check:

- owner availability;
- holidays and leave;
- support rotations;
- parallel projects;
- specialist bottlenecks;
- review availability;
- time zones;
- vendor service levels;
- meeting load.

Do not assign every task as if every person is available full time.

### Use Milestones As Decision Gates

Milestones should represent meaningful state changes, not arbitrary calendar markers. Good milestones support decisions.

Examples:

- scope approved;
- design direction locked;
- technical feasibility proven;
- vendor contract signed;
- migration rehearsal complete;
- user acceptance testing passed;
- launch readiness approved;
- handoff complete.

Each milestone should have entry criteria, exit criteria, owner, and consequences if missed.

### Make Uncertainty Visible

Early plans are estimates. The project manager should show confidence level, unknowns, and where more discovery is required.

Use ranges when appropriate. Separate committed milestones from forecast milestones. Track assumptions that affect dates. If a date is externally fixed, name the tradeoffs required to meet it.

## Common Traps

### Backward Planning From A Desired Date Without Tradeoffs

Starting from a required date is fine. Pretending the date makes the work possible is not. If the date is fixed, scope, resources, quality, or risk acceptance must change.

### Treating Approval As Instant

Executive, legal, compliance, finance, and customer approvals often take longer than task execution. They need owners and due dates.

### Ignoring Shared People

Specialists such as designers, security reviewers, data engineers, legal counsel, or release managers can become bottlenecks across several projects.

### Confusing Effort With Elapsed Time

A task may have low effort but long elapsed time because of waiting, review, procurement, vendor response, or scheduling.

### Building A Plan With No Slack

No-slack plans turn every small issue into a delivery failure. Add contingency where uncertainty is real, especially on external dependencies.

### Updating Percent Complete Instead Of Reality

"80 percent done" can hide blocked work. Track completed deliverables, open decisions, risks, and next dependency, not only percentage.

## Self-Check

- [ ] Work is decomposed by deliverables and decision points, not vague activities.
- [ ] Each major work item has owner, inputs, dependencies, duration or effort range, review needs, and completion evidence.
- [ ] Dependencies are mapped before dates are assigned.
- [ ] Critical path, slack, external dependencies, and decision dependencies are visible.
- [ ] Review, rework, approval, testing, training, support, and handoff time are included.
- [ ] Capacity constraints, shared specialists, holidays, time zones, and parallel projects were considered.
- [ ] Milestones represent meaningful decision gates with entry and exit criteria.
- [ ] Uncertainty is visible through ranges, assumptions, open questions, and confidence level.
- [ ] Fixed deadlines are paired with explicit scope, resource, quality, or risk tradeoffs.
- [ ] The plan tracks real deliverable completion rather than only percent complete.
