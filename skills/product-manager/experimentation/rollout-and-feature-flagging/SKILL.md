---
name: rollout_and_feature_flagging.md
description: Use when the agent is planning a staged rollout, configuring feature flags, deciding rollout percentages and escalation gates, managing kill switches, or coordinating gradual exposure with monitoring and rollback.
---

# Rollout And Feature Flagging

Rollout is the controlled delivery of a change to real users, and feature flags are the mechanism that makes control possible. This is not the statistical test of whether a feature works; it is the operational craft of exposing it safely, watching for harm, and being able to reverse course when something breaks. A good rollout protects users from a broken feature and protects the team from an irreversible mistake.

The judgment problem is that rollout is usually treated as a deployment detail rather than a risk-management decision. Agents tend to flip a flag to 100 percent because the build passed, leave flags on forever because no one dares remove them, and discover too late that the kill switch was never wired to anything that works. The harm is incidents that affect every user at once, flags that accumulate into unmanageable complexity debt, and rollbacks that fail under the exact stress conditions when they are needed most.

Use this skill before configuring a feature flag, setting rollout percentages, defining escalation gates, planning a kill switch, or coordinating a launch with support and go-to-market teams. The goal is to prevent big-bang exposure of unvalidated change, flag sprawl that obscures system behavior, and rollback paths that have never been exercised.

## Core Rules

### Choose Gradual Rollout Over Big Bang For Anything Risky

A big-bang release exposes every user at once and gives no early warning. Gradual rollout exposes a small cohort first, watches for failure, and expands only when the system and the experience hold up. The tradeoff is speed versus blast radius, and for anything that touches money, trust, safety, or core flows, the blast radius must be small until evidence justifies expanding it.

Decide based on the cost of a defect. A cosmetic change on a low-traffic page can ship broadly; a change to checkout, authentication, pricing, data migration, or notifications should start at a small percentage of users, often internal or beta first. The first cohort should be the one that surfaces problems cheapest, which may be staff, a single region, or a tolerant segment.

Define the expansion rule in advance: what must be true to go from 1 percent to 10 percent to 50 percent to 100 percent. Without a rule, expansion becomes a feeling, and feelings expand toward 100 percent regardless of evidence.

### Use The Right Flag Type For The Job

Feature flags are not one thing. Conflating their purposes causes both operational confusion and experiment contamination. Distinguish at least four types.

Release flags decouple deploy from release and let code sit dormant until it is turned on. Ops flags control runtime behavior for incidents, capacity, or kill switches and must be fast to toggle. Experiment flags assign users to variants for A/B testing and must integrate with the randomization and analysis pipeline. Permission flags gate entitlement by plan, role, or feature access and are often long-lived by design.

The type determines the lifecycle. A release flag should die soon after full rollout. An ops flag should live but be tested. An experiment flag should die after the decision. A permission flag is a product configuration, not technical debt. Mislabeling them is how temporary release flags become permanent clutter.

### Target Cohorts And Percentages Deliberately

Percentage rollout and cohort targeting determine who sees what. The first decision is whether the percentage is of all users, of an eligible subset, or of new users only, because these produce very different risk profiles and very different metrics.

Targeting internal users, a single region, beta opt-ins, or a specific plan first lets you catch defects in a population that can absorb them. Targeting only new users is common for onboarding changes but means you will not see effects on existing users until much later, if ever. Be explicit about who is excluded and why, because a silent exclusion can hide the segment most likely to break.

Use consistent hashing so a user stays in the same bucket across requests and devices; otherwise users flip between variants and the experience is incoherent. When combining targeting with percentage, confirm the percentage applies to the targeted population, not to global traffic.

### Define Escalation Gates And What To Monitor At Each Step

Each rollout step should have explicit gates: conditions that must hold before expanding. Gates are the structural defense against the tendency to keep going because nothing has exploded yet.

Monitor at three levels. System health includes error rates, latency, crash rates, resource use, and infrastructure saturation. Product health includes the primary metric, guardrails, and funnel completion. User signal includes support volume, complaints, app-store reviews, and qualitative feedback. A gate should require all three to be within threshold, not just the first.

Define what out-of-threshold means and who acts. A spike in errors at 5 percent should pause expansion and trigger investigation, not proceed to 50 percent on the assumption it will smooth out. Write the thresholds down before launch, when you are calm, not during an incident when you are not.

### Plan The Kill Switch And Verify Rollback Before Launch

A kill switch is worthless if it does not work under the conditions that make you need it. The most dangerous moment to discover that rollback is broken is during an incident caused by the feature you are trying to roll back.

Before launch, verify that disabling the flag actually removes the feature, that dependent state reverts cleanly, that caches and queues do not serve stale treated behavior, and that the rollback path has been tested in an environment that resembles production. For changes that mutate data or schema, plan whether rollback restores prior state or merely stops new mutation, because those are different promises.

Decide who has authority to flip the kill switch and under what conditions, and make sure that person is reachable during the launch window. A switch that requires three approvals and a deploy pipeline is not a kill switch; it is a process.

### Manage The Interaction Between Flags And Experiments

Flags and experiments interact, and uncoordinated interactions produce confounded results. Two experiments that overlap can each attribute the other's effect to themselves, and a rollout flag that changes mid-experiment can shift the population under measurement.

Decide whether experiments are mutually exclusive (a user can be in only one) or overlapping (a user can be in several). Mutual exclusion keeps results clean but limits throughput; overlapping maximizes traffic but requires interaction analysis. Document the layering so that a future analyst can reconstruct who was exposed to what.

When a feature is both rolling out and being experimented on, separate the rollout exposure from the experiment assignment. Rolling out to 50 percent while also running a 50/50 experiment on that 50 percent is valid but must be modeled explicitly, or the numbers will not mean what they appear to mean.

### Keep Flag Hygiene And Schedule Cleanup

Every flag is a conditional branch that future engineers must reason about. Flags that never retire accumulate into complexity debt: more test combinations, more cognitive load, more paths that are never exercised, and more places where a stray toggle causes an incident.

For each flag, record an owner, a purpose, an expected retirement date, and a removal condition. When the rollout reaches 100 percent and the decision is final, remove the flag and the dead code behind it rather than leaving it "just in case." Schedule periodic flag audits to find flags that met their condition but were never cleaned up.

The exception is ops and permission flags, which are meant to persist, but even those should be documented, tested, and reviewed so they do not rot into a configuration nobody understands.

### Coordinate Rollout With Support And Go-To-Market

A rollout is not only an engineering event. Support teams need to know what is launching, to whom, and what to tell users who notice the change. Go-to-market teams need to align messaging, pricing, and sales enablement with who can actually access the feature.

If marketing announces a feature that 2 percent of users can see, the other 98 percent experience a broken promise. If support is surprised by an influx of contacts about a new behavior, the first hour of the incident is wasted on confusion rather than resolution. Share the rollout plan, the cohort definition, the expected timeline, and the rollback conditions with the teams that face users, before the flag goes live.

## Common Traps

### Big-Bang Release To Save Time

Shipping to 100 percent at once feels faster because it skips the staged steps, but it trades a small scheduling cost for a large blast radius. The trap is that the time saved is visible and the risk taken is invisible until something breaks for everyone simultaneously.

### Leaving Release Flags On Forever

A release flag that reached 100 percent and was never removed becomes a permanent conditional that no one understands. The trap is that "it works, do not touch it" feels safe while the flag silently complicates every future change and every future test in that code path.

### Untested Kill Switch

Teams assume rollback works because the flag exists, but the flag may not fully revert behavior, may not flush caches, or may require a deploy that is too slow during an incident. The trap is confidence based on existence rather than verification, discovered at the worst possible moment.

### Expanding On Vibe Rather Than Gates

Without pre-defined gates, expansion drifts toward 100 percent because nothing has obviously failed yet. The trap is that early small problems are rationalized as noise, and the team reaches full exposure before the noise becomes a clear signal.

### Confounding Rollout With Experiment Assignment

Mixing a live rollout with an experiment without modeling the overlap produces metrics that look precise but are biased. The trap is that the analysis reports a clean lift while the population under measurement was silently shifting.

### Flag Sprawl From No Retirement Discipline

Each new flag feels small, but unretired flags compound into a combinatorial explosion of untested configurations. The trap is that complexity grows invisibly until a release is blocked by interactions no one can fully reason about.

### Surprising Support And GTM

Engineering launches on its own schedule while support and marketing learn about the change from user complaints. The trap is treating rollout as a technical event when its first consequences are human and reputational.

### Inconsistent Hashing Causing Variant Flipping

If a user is not consistently bucketed, they may see treatment on one request and control on the next, producing an incoherent experience and contaminated metrics. The trap is that the flag system reports stable assignment while users actually flip.

## Self-Check

- [ ] The rollout starts with a small, defect-tolerant cohort and expands only on defined gates, not on schedule pressure.
- [ ] Each flag is labeled by type (release, ops, experiment, permission) with a lifecycle matching that type.
- [ ] Percentage and cohort targeting are explicit about the population denominator and use consistent hashing.
- [ ] Escalation gates cover system health, product metrics, and user signal, with thresholds written before launch.
- [ ] The kill switch has been verified to actually revert behavior, including caches, queues, and dependent state.
- [ ] The interaction between rollout flags and concurrent experiments is documented and modeled, not assumed clean.
- [ ] Every release and experiment flag has an owner, purpose, retirement condition, and scheduled cleanup.
- [ ] Support and go-to-market teams have the rollout plan, cohort definition, timeline, and rollback conditions before launch.
- [ ] No flag is left at a partial percentage indefinitely without a reason and a review date.
- [ ] Rollback authority and the conditions to invoke it are defined and reachable during the launch window.
