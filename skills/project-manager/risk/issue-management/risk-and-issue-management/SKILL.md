---
name: risk_and_issue_management.md
description: Use when the agent is identifying project risks, managing issues, deciding escalation, reviewing mitigation plans, preparing a risk register, or checking whether delivery threats are being handled before they become surprises.
---

# Risk And Issue Management

Risk management is not pessimism. It is the discipline of seeing uncertainty early enough to act. An issue is a risk that has already happened or is happening now. A project manager must distinguish risks from issues, warnings from facts, and mitigation from vague hope. The point is not to list every possible problem; it is to protect delivery, quality, trust, and decision making.

Use this skill before creating a risk register, reviewing project health, preparing escalation, managing blockers, or deciding whether a project is still on track. The goal is to prevent the agent from producing status that looks calm while real delivery threats remain unnamed.

## Core Rules

### Separate Risk, Issue, Assumption, And Dependency

Use precise categories:

- Risk: something uncertain that may affect the project.
- Issue: something already affecting the project.
- Assumption: something believed true but not yet proven.
- Dependency: something the project needs from another person, team, system, vendor, or decision.

Blurring categories weakens action. "Vendor may be late" is a risk. "Vendor missed the agreed delivery date" is an issue. "Vendor supports the required API" is an assumption. "Vendor contract approval" is a dependency.

### Describe Impact In Project Terms

A risk statement should connect cause, event, and impact. Avoid vague entries like "legal risk" or "resource issue".

Use a structure:

- Because of [condition], [event] may happen, causing [impact].

Examples:

- Because security review has not started, launch approval may be delayed, causing the public release date to slip.
- Because migration volume is unknown, the cutover may exceed the maintenance window, causing rollback or customer downtime.
- Because the same designer supports three projects, feedback cycles may stretch, causing implementation to wait.

This makes mitigation easier.

### Assign Owners And Trigger Dates

Every material risk or issue needs an owner. Ownership means watching the signal, driving mitigation, and escalating when needed. It does not always mean personally solving the problem.

For each item, define:

- owner;
- impact;
- probability or urgency;
- mitigation;
- contingency;
- trigger condition;
- review date;
- escalation path.

Risks without owners become meeting decoration.

### Distinguish Mitigation From Contingency

Mitigation reduces probability or impact before the risk happens. Contingency is what the team will do if the risk happens.

Example:

- Risk: vendor delivery may be late.
- Mitigation: confirm delivery milestones twice weekly and request early test package.
- Contingency: reduce launch scope or switch to manual import if package is not received by a trigger date.

"Monitor closely" is not mitigation by itself unless paired with trigger and action.

### Escalate With Options

Escalation should not be a vague alarm. Bring the decision maker a clear problem, impact, options, recommendation, and deadline.

Useful escalation includes:

- current fact pattern;
- decision needed;
- impact of no decision;
- options with tradeoffs;
- recommendation;
- date by which action is needed;
- owner for next step.

Escalation is not failure. Late escalation is usually worse than early escalation.

### Watch For Silent Risks

Some risks do not announce themselves through blockers. They hide in ambiguity, quality, morale, governance, and assumptions.

Check:

- unclear decision rights;
- unapproved scope growth;
- low test coverage;
- unreviewed security or legal concerns;
- stakeholder disengagement;
- team burnout;
- vendor silence;
- data quality uncertainty;
- missing support readiness;
- fragile manual process;
- single point of failure.

If the status is green but these signals exist, the project may not be healthy.

### Keep Risk Review Alive

A risk register created once is not useful. Risks change as work progresses. Some are retired, some become issues, and some new risks appear.

At regular review:

- close risks that no longer apply;
- convert realized risks into issues;
- update probability, impact, and owner;
- check mitigation progress;
- add new risks from recent work;
- confirm trigger dates;
- escalate overdue decisions.

## Common Traps

### Listing Risks Without Actions

A long list of worries is not risk management. Prioritize the material risks and define action.

### Keeping Everything Green

Green status can be politically comfortable but operationally dangerous. If a project has unmanaged critical risks, it should not be reported as simply healthy.

### Escalating Too Late

If leadership hears about a risk only when the date is already missed, they cannot help with tradeoffs.

### Treating Dependency Delay As Someone Else's Problem

Dependencies still affect the project. The project manager must track, communicate, and escalate dependency risk even when another team owns the work.

### Ignoring Quality Risks

Schedule and budget are visible, but quality, security, adoption, support readiness, and maintainability can create larger failure after launch.

### No Trigger Conditions

Without trigger dates or thresholds, teams keep hoping. A trigger turns uncertainty into a decision point.

## Self-Check

- [ ] Risks, issues, assumptions, and dependencies are separated rather than mixed together.
- [ ] Material risks are written with cause, event, and project impact.
- [ ] Each important risk or issue has owner, probability or urgency, impact, mitigation, contingency, trigger, review date, and escalation path.
- [ ] Mitigation and contingency are distinct.
- [ ] Escalations include facts, impact, options, recommendation, decision needed, and deadline.
- [ ] Silent risks such as unclear authority, scope growth, quality, burnout, legal, security, support, vendor silence, and data uncertainty were reviewed.
- [ ] Dependency risks are tracked even when another team owns the dependent work.
- [ ] Trigger dates prevent indefinite monitoring without action.
- [ ] The risk register is updated as risks retire, become issues, or change severity.
- [ ] Project health status reflects unmanaged risk, not only whether tasks are currently moving.
