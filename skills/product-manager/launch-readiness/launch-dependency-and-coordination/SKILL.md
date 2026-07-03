---
name: launch_dependency_and_coordination.md
description: Use when the agent is coordinating launch dependencies across teams, sequencing dependent work, managing the critical path of a launch, or resolving cross-team blockers that threaten a launch date.
---

# Launch Dependency And Coordination

A launch is rarely the work of one team. It is a chain of dependent work across product, engineering, design, marketing, legal, security, data, and external partners, and the launch date is determined not by the average progress but by the slowest link in that chain. Dependency and coordination management is the work of finding that link, keeping it moving, and knowing when a slip there moves the whole launch.

Agents miss this because a launch plan often lists workstreams as if they were independent. They track each team's progress, report green across the board, and discover two weeks out that team C was waiting on team A, which was waiting on a legal review that never started. The harm is that the critical path was invisible until it broke, the launch slips late and expensively, and the teams that delivered on time are held back by the one that was never coordinated. The opposite failure is over-coordination, where every dependency is escalated and the launch drowns in status meetings while the actual work stalls.

Use this skill before answering broad questions such as "how do we coordinate this launch", "what are our dependencies", "what is the critical path", "who is blocking us", or "how do we sequence the work across teams". The goal is to prevent the agent from treating a launch as parallel workstreams and forgetting that the dependencies, not the averages, decide the date.

## Core Rules

### Map Dependencies Before Scheduling The Date

A launch date set before the dependencies are mapped is a wish. The real date is a function of the longest chain of dependent work, and that chain is invisible until someone traces it. Map the dependencies first, then derive a realistic date, then negotiate from evidence.

For each workstream, identify what it depends on: another team's deliverable, an external review, a vendor, a migration, a legal or security sign-off, a piece of infrastructure. Draw the chain, find the longest path, and treat that as the binding constraint on the date. A launch scheduled against anything less than the mapped critical path will slip, and the slip will be discovered late.

### Identify And Protect The Critical Path

The critical path is the chain of dependencies whose total duration sets the earliest possible launch date. Work on the critical path cannot slip without slipping the launch; work off the path can absorb some delay. Most coordination failure comes from not knowing which is which.

Identify the critical path explicitly, and protect it aggressively. Prioritize resources, unblock issues, and escalate risks on the critical path before off-path work, because a day saved off the path does not move the date and a day lost on the path does. Make the critical path visible to everyone, so effort goes where it changes the outcome.

### Name A Single Owner For Each Dependency

A dependency owned by everyone is owned by no one. Each cross-team dependency needs a single named owner who is accountable for it being delivered, and that owner is a person, not a team. "Engineering will handle the migration" is not ownership; "Priya owns the migration, due Thursday" is.

Assign one owner per dependency, and confirm that owner has accepted it. Track dependencies by owner, not by team, because accountability dissolves the moment it is assigned to a group. When a dependency slips, there must be one person to talk to, not a meeting to convene.

### Surface Blockers Early And Escalate With A Proposed Path

Blockers are normal in a launch; late surprises are not. The coordination discipline is to surface blockers as soon as they appear and to escalate them with enough context and a proposed resolution that the right person can act fast.

Establish a rhythm for surfacing blockers: a standing risk review in the final weeks, a single source of truth for the dependency list, and an expectation that blockers come with impact, options, and a recommended owner. Escalating a blocker without a proposed path forces the receiver to solve it from scratch, which costs the time the escalation was meant to save.

### Distinguish Hard Sequencing From Parallelizable Work

Not all dependencies are hard. Some work must happen in sequence, such as a migration before a feature that depends on it, but much work can proceed in parallel if the teams coordinate on interfaces early. Misclassifying parallelizable work as sequential needlessly extends the timeline, and misclassifying sequential work as parallel creates integration failures late.

For each dependency, ask whether it is a hard sequence or whether the teams could proceed in parallel against an agreed interface. Where interfaces can be defined early, parallelize. Where the work genuinely cannot start until another finishes, sequence it honestly and protect that point on the critical path.

### Plan For The Latest Safe Delay Point

Every launch has a point of no return: the latest moment at which a slip can be absorbed without cascading into external commitments like announcements, partner coordination, or customer promises. Knowing that point in advance lets the team decide rationally whether to push through or slip, rather than discovering it under pressure.

Identify the latest safe delay point for the launch, and the external commitments that harden after it. As that point approaches, the decision to launch or slip must be made deliberately against the critical path, not by default. Slipping after external commitments are locked turns a schedule problem into a trust problem.

### Coordinate External Partners And Vendors Explicitly

External dependencies behave differently from internal ones. A vendor cannot be pulled into a war room, a partner has their own roadmap, and a review by an external party runs on their timeline, not yours. External dependencies are routinely underestimated and are a frequent cause of late slips.

Identify external dependencies early, confirm their timelines in writing, and build buffer for the parts you cannot control. Treat external commitments as constraints on the critical path, and have a contingency for when they slip, because the leverage you have over an external party at the eleventh hour is close to zero.

## Common Traps

### Scheduling The Date Before Mapping Dependencies

A date set without the dependency map is aspirational, and the gap between aspiration and reality is discovered late, when slips are expensive and external commitments are already locked.

### Losing Track Of The Critical Path

Treating all workstreams as equally important hides the chain that actually decides the date, so effort goes to off-path work while the binding constraint slips unnoticed.

### Ownership Diffused Across A Team

A dependency assigned to "engineering" or "the platform team" has no owner, and when it slips no one is accountable, which guarantees it slips again.

### Blockers Surfaced Late

Blockers held until a status meeting, or escalated without context, cost the launch days that could have been saved by early, actionable escalation.

### Misclassifying Sequence And Parallelism

Treating parallelizable work as sequential extends the timeline needlessly, while treating sequential work as parallel produces integration failures in the final weeks.

### Missing The Latest Safe Delay Point

Without a defined point of no return, the team defaults to pushing forward under pressure, and a rational slip decision becomes a forced launch that fails.

### Underestimating External Dependencies

Vendors and partners run on their own timelines and cannot be escalated the way internal teams can. Treating them like internal dependencies is a reliable way to slip late.

## Self-Check

- [ ] Dependencies were mapped before the launch date was set, and the date reflects the critical path rather than aspiration.
- [ ] The critical path is identified, visible, and protected, with resources and escalation prioritized there over off-path work.
- [ ] Each cross-team dependency has a single named owner who has accepted accountability, not ownership diffused across a team.
- [ ] Blockers are surfaced early through a standing rhythm and escalated with impact, options, and a recommended owner.
- [ ] Hard sequencing is distinguished from parallelizable work, and interfaces are defined early to enable parallelism where possible.
- [ ] The latest safe delay point is identified, along with the external commitments that harden after it.
- [ ] External partners and vendors are identified early, their timelines confirmed in writing, and buffer built for parts outside your control.
- [ ] A single source of truth tracks dependencies, owners, status, and risks rather than scattered status updates.
- [ ] The launch or slip decision near the point of no return is made deliberately against the critical path, not by default.
- [ ] Coordination scales to the launch tier, with enough structure for a major release and minimal ceremony for a minor one.
