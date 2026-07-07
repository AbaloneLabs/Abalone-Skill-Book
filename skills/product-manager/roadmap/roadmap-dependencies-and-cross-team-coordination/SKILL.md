---
name: roadmap_dependencies_and_cross_team_coordination.md
description: Use when the agent is planning a roadmap with cross-team dependencies, coordinating platform and product teams, managing sequencing across squads, resolving dependency bottlenecks, or designing a roadmap that accounts for shared infrastructure and partner timelines.
---

# Roadmap Dependencies And Cross-Team Coordination

A roadmap that looks clean within one team is often impossible across several. Most meaningful product work crosses team boundaries: a feature needs a platform change, a launch needs marketing and sales enablement, a migration depends on an external partner, and a data initiative needs engineering, analytics, and legal to converge. The judgment problem is that dependencies are invisible on a single-team roadmap and lethal when ignored. A roadmap that does not model who must deliver what for whom, and when, will produce a cascade of slips that each team blames on the other. Coordinating dependencies is not project management overhead; it is the core act of making a multi-team roadmap executable.

Use this skill before finalizing a roadmap that spans more than one team, before committing to dates that depend on other teams, before resolving contention for shared platform or specialist resources, and before presenting a cross-team plan as realistic. The goal is to prevent the agent from producing a roadmap that is internally tidy but externally impossible, or that hides the handoffs where execution will actually break.

## Core Rules

### Map Dependencies Before Committing To Dates

Dependencies must be understood before any date is credible. A feature owned by team A that needs an API from team B is not schedulable until team B's capacity and priorities are known.

Map dependency types:

- technical: API, platform, infrastructure, data pipeline, shared service;
- design: UX research, design system component, accessibility review;
- go-to-market: pricing, packaging, sales enablement, support readiness;
- compliance: legal, privacy, security, regulatory review;
- data: instrumentation, analytics, reporting, ML model;
- external: partner, vendor, integration, certification.

For each dependency, identify the providing team, the consumer, the needed-by date, and the negotiation status.

### Distinguish Hard And Soft Dependencies

Not every dependency is equally binding. Hard dependencies block start or finish; soft dependencies affect quality or efficiency but not feasibility.

Classify:

- hard blocking: work cannot begin or ship without it;
- hard sequencing: must complete in a defined order;
- soft: preferred but workaround exists;
- informational: teams should be aware but no constraint.

Treating all dependencies as hard paralyzes planning; treating all as soft produces slips. Be precise.

### Negotiate Capacity With Providing Teams, Not Assume It

The most common roadmap failure is assuming a providing team will deliver on the consumer's timeline without checking their roadmap. Capacity is finite and already committed.

For each cross-team dependency:

- confirm the providing team has accepted the work into their roadmap;
- agree on the needed-by date and their committed delivery date;
- identify contention with their other commitments;
- surface the tradeoff if your request displaces their other work;
- escalate early if the dates do not reconcile.

A dependency is only real when the providing team has acknowledged and sequenced it.

### Identify And Protect Shared Bottlenecks

Certain resources are bottlenecks across many teams: security review, legal, design systems, data engineering, release management, a specific platform team. These specialists are where cross-team plans collide.

Manage bottlenecks by:

- mapping which teams need the same scarce resource and when;
- sequencing demand to avoid simultaneous contention;
- negotiating reserved capacity or SLAs for critical shared functions;
- front-loading bottleneck work to unlock parallel progress;
- escalating persistent contention to a level that can arbitrate.

Ignoring bottlenecks guarantees that the critical path runs through a queue the team does not control.

### Align Roadmap Cadences Across Teams

Teams on different planning cadences cannot coordinate cleanly. If team A plans monthly and team B quarterly, dependencies land in mismatched windows.

Align by:

- synchronizing planning cycles where dependencies are dense;
- establishing cross-team dependency reviews before each cycle;
- creating a shared view of committed cross-team work;
- defining a fast-track path for urgent interdependencies;
- holding regular dependency standups for active cross-team initiatives.

Cadence misalignment is a silent cause of slips that looks like individual team failure.

### Build In Buffer For External And Partner Dependencies

External dependencies, partners, vendors, certifications, are the least controllable and most likely to slip. A roadmap that schedules them as reliably as internal work is fragile.

For external dependencies:

- add explicit buffer proportional to the partner's reliability;
- define fallback plans if the partner slips;
- set early warning checkpoints, not just final deadlines;
- negotiate contractual milestones where possible;
- avoid making the entire critical path depend on a single external entity.

### Make The Dependency Graph Visible And Maintained

A dependency map built once and never updated decays immediately. The map must be a living artifact reviewed in every planning cycle.

Maintain by:

- keeping a single source of truth for cross-team dependencies;
- reviewing status, risk, and slip probability each cycle;
- flagging dependencies that have gone silent or stale;
- recording resolution to build institutional memory;
- surfacing chronic dependency failures as systemic issues.

### Plan For Dependency Failure

Even mapped dependencies slip. The roadmap should encode what happens when a dependency fails, not assume it will not.

For critical dependencies define:

- the trigger that declares a dependency at risk;
- the decision owner for responding;
- the fallback, descope, defer, workaround, or escalate;
- the communication path to affected teams and stakeholders.

A roadmap with no failure plan converts every slip into a crisis.

## Common Traps

### Assuming Providing Teams Will Deliver

Treating another team's capacity as available because you need it, without confirming acceptance, is the most common cross-team failure.

### Single-Team Roadmap Thinking

Optimizing one team's roadmap while ignoring the handoffs produces local efficiency and global slip.

### Treating All Dependencies As Equal

Inflating every preference to a hard dependency paralyzes planning; deflating real blockers produces slips.

### Ignoring Shared Bottlenecks

Security, legal, design systems, and platform teams are where plans collide. Unmanaged contention is guaranteed slip.

### Cadence Mismatch

Teams planning on different cycles cannot reconcile dependencies and slip in predictable, avoidable ways.

### No Buffer On External Work

Scheduling partner and vendor work as reliably as internal work guarantees fragility on the least controllable part of the plan.

### Stale Dependency Maps

A dependency graph built once decays within a cycle and becomes misleading rather than helpful.

### No Failure Plan

Assuming dependencies will not slip leaves the team unprepared and reactive when they inevitably do.

## Self-Check

- [ ] Dependencies are mapped, with type, providing team, consumer, needed-by date, and negotiation status before dates are committed.
- [ ] Hard blocking dependencies are distinguished from soft and informational ones.
- [ ] Providing teams have accepted and sequenced the work into their own roadmaps, not been assumed into compliance.
- [ ] Shared bottleneck resources are identified, sequenced, and have contention managed or escalated.
- [ ] Planning cadences are aligned or reconciled where dependencies are dense.
- [ ] External and partner dependencies carry buffer, early warning checkpoints, and fallback plans.
- [ ] A single living dependency source of truth is reviewed and updated each planning cycle.
- [ ] Each critical dependency has a defined failure response: trigger, owner, fallback, and communication path.
- [ ] The roadmap models cross-team handoffs rather than presenting a single-team view as complete.
- [ ] Chronic dependency failures are surfaced as systemic issues rather than blamed on individual teams.
