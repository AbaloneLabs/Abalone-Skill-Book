---
name: requirements_traceability_and_scope_baseline.md
description: Use when the agent is maintaining traceability between requirements and delivered work, managing a requirements baseline, controlling scope changes during build, tracking requirement status, or ensuring delivered features match what was specified and approved.
---

# Requirements Traceability And Scope Baseline

A requirement that is written and then forgotten is a requirement that will not be delivered as intended. Between the approved spec and the shipped feature lies a gauntlet of design decisions, engineering tradeoffs, scope negotiations, testing choices, and forgotten edge cases, and without a thread connecting each requirement to the work that implements it, gaps appear silently. The judgment problem is that traceability and baselining feel like bureaucratic overhead, but they are the only mechanism that catches the slow drift where a feature ships missing a requirement, a requirement changes without re-approval, or scope expands without anyone noticing until the timeline breaks. The skill is making traceability lightweight enough to actually happen and rigorous enough to catch the drift that matters.

Use this skill before finalizing a requirements baseline, before tracking requirements through build, before approving a scope change, or before declaring a feature complete. The goal is to prevent the agent from treating requirements as write-only artifacts, from allowing silent scope drift, or from burying the team in traceability ceremony that no one uses.

## Core Rules

### Establish A Baseline Before Build Begins

A baseline is the agreed set of requirements that the build is accountable to. Without one, there is no stable reference against which to measure change, completion, or drift. The baseline is set when requirements are reviewed and approved, not when they are first drafted.

A useful baseline includes:

- the approved requirement set with version and date;
- the priority classification of each requirement;
- the explicit non-goals and out-of-scope items;
- the assumptions and dependencies current at approval;
- the success criteria and acceptance conditions.

Once baselined, changes are managed deliberately, not absorbed silently.

### Connect Each Requirement To The Work That Delivers It

Traceability is the bidirectional link between a requirement and the engineering, design, and test work that implements it. It is what lets you answer two questions: is every requirement being built, and is every piece of built work traceable to an approved requirement.

Build traceability by linking:

- each requirement to the design and engineering tasks that implement it;
- each requirement to the test cases that verify it;
- each delivered feature back to the requirement it satisfies;
- requirements to the acceptance criteria that define done.

The links do not need to be elaborate, but they must exist and be maintained. A requirement with no implementing work is a gap; work with no requirement is scope creep.

### Track Requirement Status Through The Lifecycle

A requirement is not binary, done or not. It moves through states, and tracking those states surfaces blockage and drift before they become failures.

Track status such as:

- approved and baselined;
- in design;
- in development;
- in review or QA;
- blocked or at risk;
- delivered and verified;
- deferred or descoped;
- changed since baseline.

Status tracking makes the health of the requirement set visible and forces decisions on blocked or drifting items.

### Control Changes Through A Deliberate Process

Once baselined, requirements will change, and that is healthy. The danger is uncontrolled change: a stakeholder adds scope in a side conversation, engineering quietly drops a requirement that proved hard, or a requirement mutates without anyone re-checking the impact. Change should be visible and decided, not absorbed.

For each change, capture:

- what is changing and why;
- the impact on scope, schedule, dependencies, and risk;
- who approved the change and at what level;
- the new baseline version and what it supersedes;
- communication to affected teams.

The goal is not to prevent change but to make it a conscious decision with visible consequences.

### Distinguish Scope Change From Clarification

Not every adjustment to a requirement is scope change. Much of what happens during build is clarification, resolving ambiguity in the original spec without changing its intent. Conflating the two either buries real scope changes or bureaucratizes harmless clarifications.

Distinguish:

- clarification: resolving ambiguity within the existing intent, no scope impact;
- minor change: small adjustment with negligible impact, lightweight approval;
- major change: alters scope, schedule, dependencies, or risk, full impact analysis;
- scope creep: addition that was never approved, must be caught and decided.

Right-size the process to the real impact of the change.

### Catch Scope Creep Early Through Coverage Checks

Scope creep accumulates when added work is not checked against the baseline. Regular coverage checks catch it before it breaks the timeline.

Run coverage checks by:

- periodically mapping delivered and in-progress work back to requirements;
- flagging work that traces to no approved requirement;
- flagging approved requirements with no implementing work;
- comparing actual scope trajectory to the baseline;
- surfacing the cumulative impact of approved changes.

### Define Completion Against Requirements, Not Against Activity

A feature is complete when its requirements are met and verified, not when the team stops working on it. Completion defined by activity, "we built the screens", misses whether the requirements were actually satisfied.

Define completion by:

- each must-have requirement verified against its acceptance criteria;
- non-functional requirements met, performance, security, accessibility;
- edge cases and error states handled as specified;
- traceability links confirming coverage;
- stakeholder sign-off against the baseline, not against impressions.

### Keep Traceability Lightweight Enough To Maintain

Heavy traceability matrices that no one updates are worse than none, because they create false confidence. The system must be simple enough to survive the pressure of delivery.

Keep it lightweight by:

- embedding traceability in the tools the team already uses, tickets, PRs, tests;
- automating links where possible;
- reviewing traceability as part of normal ceremonies, not as separate ceremony;
- focusing rigor on must-have and high-risk requirements;
- pruning stale links regularly.

## Common Traps

### Write-Only Requirements

Writing specs that are never referenced again, so drift goes undetected.

### No Baseline

Building against a moving target with no agreed reference, making completion and change undefinable.

### Silent Scope Drift

Allowing requirements to change through side conversations without re-approval or impact analysis.

### Conflating Clarification With Change

Bureaucratizing harmless clarifications or hiding real scope changes as clarifications.

### Activity-Based Completion

Declaring done based on effort expended rather than requirements verified.

### Heavy Unused Matrices

Traceability artifacts so burdensome that no one maintains them, creating false confidence.

### Coverage Gaps

Approved requirements with no implementing work, or work with no approved requirement, both undetected.

### Change Without Communication

Approving changes without informing affected teams, producing misaligned delivery.

## Self-Check

- [ ] A requirements baseline is established at approval with version, priority, non-goals, assumptions, and success criteria.
- [ ] Each requirement is linked bidirectionally to the design, engineering, and test work that implements and verifies it.
- [ ] Requirement status is tracked through the lifecycle, surfacing blocked, at-risk, and drifting items.
- [ ] Changes are controlled through a deliberate process capturing what, why, impact, approver, and new baseline version.
- [ ] Scope change is distinguished from clarification, with process right-sized to real impact.
- [ ] Regular coverage checks catch work without requirements and requirements without work.
- [ ] Completion is defined against verified requirements and acceptance criteria, not activity.
- [ ] Traceability is lightweight, embedded in existing tools, and maintained through normal ceremonies.
- [ ] Scope creep is caught early through cumulative impact review against the baseline.
- [ ] Changes are communicated to all affected teams, not approved in isolation.
