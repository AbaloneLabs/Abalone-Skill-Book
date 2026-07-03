---
name: schedule_compression_and_recovery.md
description: Use when the agent is shortening a project schedule, recovering from slippage, fast-tracking or crashing activities, deciding schedule tradeoffs under a fixed deadline, or evaluating whether a compressed plan is still realistic.
---

# Schedule Compression And Recovery

When a project is behind or faces a fixed deadline, the temptation is to simply push dates or add pressure. Real compression and recovery require structural changes to logic, scope, resources, or quality. Each compression technique carries a cost, and pretending otherwise produces a plan that looks recovered but fails again. The project manager must choose techniques that fit the work, make their tradeoffs explicit, and avoid the false recovery of merely declaring an earlier date.

Use this skill before fast-tracking, crashing, recovering a slipped schedule, responding to a fixed deadline, or reviewing whether a compression plan is credible. The goal is to prevent the agent from producing a compressed schedule that ignores the cost, risk, and feasibility of the techniques applied.

## Core Rules

### Diagnose Before Compressing

Compression is a response to a specific problem. Before applying it, understand what is actually driving the schedule. Is the critical path the issue, or is it a resource bottleneck, an external dependency, or scope growth?

Compression applied to the wrong constraint wastes effort. If the binding constraint is a shared specialist, adding people elsewhere does not help. If the binding constraint is an external approval, internal fast-tracking does not help. Match the technique to the diagnosed cause.

### Understand Fast-Tracking

Fast-tracking means sequencing activities in parallel that were originally planned in series. It can shorten duration when activities are independent enough to overlap.

Fast-tracking increases risk because work proceeds with less complete information from predecessors, which increases rework, integration effort, and the chance of discarded work. It is most viable when activities have finish-to-start dependencies that can be relaxed to start-to-start with acceptable risk.

Assess for each overlap what information the later activity is assuming and what happens if that information changes. Fast-tracking without this analysis is gambling.

### Understand Crashing

Crashing means adding resources to critical path activities to finish them sooner, usually at additional cost. It works only when the work can absorb more resources, which is not always true.

Crashing is governed by the cost slope, the additional cost per unit of time saved, for each activity. Some activities crash cheaply, others cannot be crashed at all. Adding people to work that cannot be parallelized, such as a single sequential analysis or a fixed-duration cure, increases cost without reducing duration.

Crashing also carries onboarding cost, communication overhead, and quality risk that can offset the intended gain, famously summarized as the principle that adding people to a late project tends to make it later.

### Use Scope Reduction As A Compression Lever

Reducing scope is often the most reliable way to shorten a schedule, because it removes work from the critical path entirely. This may mean deferring features to a later phase, simplifying a deliverable, or dropping optional components.

Scope reduction should be a deliberate decision with stakeholder agreement, not a quiet cut. Present it as a tradeoff alongside fast-tracking and crashing so the sponsor can choose the least harmful combination.

### Make Tradeoffs Explicit

Every compression technique trades something. Fast-tracking trades risk and rework. Crashing trades cost and quality. Scope reduction trades delivered value. Quality reduction trades future defects and maintenance.

Present the tradeoffs together so the decision maker understands the full cost of hitting the date. A compressed plan that hides its tradeoffs is not a recovery, it is a deferred failure.

### Recompute The Critical Path After Compression

Compression changes the schedule network. An activity that was crashed may no longer be critical, and a previously non-critical path may become the new critical path. After any compression, recompute the critical path and confirm the end date actually moved.

Compression that targets non-critical work does not shorten the project. Always confirm the technique is applied to the current critical path.

### Distinguish Recovery From Denial

Recovery is a credible plan to regain a lost position through structural change. Denial is restating an earlier date without changing logic, scope, or resources. A recovery plan must be defensible: it must explain what will change, why it will work, what it costs, and what risk it adds.

If a recovery plan cannot answer these questions, it is not recovery. It is hope presented as a schedule.

### Plan For The Increased Risk

Compressed schedules have less margin and higher risk. Increase monitoring of the compressed activities, secure the resources they depend on, and prepare contingency for the rework or integration issues that fast-tracking and crashing tend to create.

## Common Traps

### Declaring An Earlier Date As Recovery

Changing a date without changing the work is not recovery. It is setting up the next slippage.

### Crashing Work That Cannot Parallelize

Adding people to sequential or fixed-duration work increases cost without saving time.

### Fast-Tracking Without Risk Analysis

Overlapping activities that depend on incomplete information increases rework and can extend duration.

### Compressing Non-Critical Work

Effort spent compressing activities with float does not move the end date.

### Ignoring Onboarding And Communication Overhead

New resources take time to become productive and increase coordination cost, which can offset compression gains.

### Hidden Quality Reduction

Cutting testing, review, or documentation to save time trades immediate schedule for future defects and rework.

### One Technique Applied Blindly

Reaching for the same compression lever regardless of context misses better options such as scope deferral.

### No Recomputation Of The Critical Path

Assuming the original critical path still applies after compression leads to effort wasted on the wrong activities.

## Self-Check

- [ ] The cause of slippage or the binding constraint was diagnosed before compression was applied.
- [ ] Fast-tracking is applied only where activities can overlap with acceptable risk, and the assumed information is identified.
- [ ] Crashing targets activities that can absorb resources, with cost slope considered.
- [ ] Scope reduction is offered as a deliberate tradeoff alongside other techniques.
- [ ] The tradeoffs of each technique, including cost, risk, quality, and value, are presented explicitly to the decision maker.
- [ ] The critical path is recomputed after compression to confirm the end date actually moved.
- [ ] The recovery plan explains what changes, why it works, what it costs, and what risk it adds.
- [ ] Increased monitoring and contingency are in place for compressed activities.
- [ ] Quality reduction, if any, is a conscious decision rather than a hidden side effect.
- [ ] Compression is not confused with merely restating an earlier date.
