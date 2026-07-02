---
name: sprint_and_milestone_planning.md
description: Use when the agent is planning a sprint or iteration, prioritizing a backlog, estimating team velocity and capacity, sequencing work with dependencies, sizing risk buffers, defining milestones and release criteria, or adjusting scope when capacity or priorities change mid-cycle.
---

# Sprint and Milestone Planning

Sprint and milestone planning is where estimates meet reality, and it fails in two predictable ways. The first is optimistic packing: the team commits to a sprint's worth of work based on raw capacity, ignoring interruptions, support load, meetings, dependencies, and the work that always appears, and then carries half of it forward every cycle. The second is plan-as-contract: the plan becomes a commitment that cannot be changed, so when reality diverges (a task is harder, a dependency slips, a priority changes), the team either silently overworks to hit the number or abandons the plan and operates without one. Both produce burnout, missed dates, and distrust between engineering and the rest of the org.

The judgment problem is that a plan is a hypothesis, not a promise, and its value is in forcing decisions about priority and sequencing, not in predicting the future precisely. The agent must plan with honest capacity (not theoretical capacity), sequence work to expose dependencies and the critical path early, build in buffer for the known-unknown work that always appears, define milestones in terms of verifiable outcomes rather than activity, and treat the plan as something to update when reality changes rather than something to defend. A plan that is never revised is a plan that has been abandoned; a plan that is revised transparently is a plan that is actually being used.

## Core Rules

### Plan to honest capacity, not theoretical capacity

The most common planning error is assuming 100% of engineer-time goes to planned sprint work. In reality, a large fraction goes to support, incidents, code review, meetings, context-switching, and the steady drip of small requests. Estimate capacity based on what the team historically completes, not on headcount times hours, and subtract known recurring loads (on-call rotation, support duty, standing meetings). A team that nominally has 10 engineer-weeks but historically completes 6 of planned work should plan to 6, not 10. Planning to theoretical capacity guarantees carryover every sprint and erodes trust in the plan.

### Prioritize by value and dependency, not by what is easy to estimate

Backlog ordering should reflect value (what matters most to users or the business) and dependency (what unblocks other work), with effort as a constraint, not as the primary sort. A common anti-pattern is to do the small, well-understood items first because they are easy to commit to, while the high-value, uncertain item that actually matters slips indefinitely. Sequence so that the highest-value work and the work on the critical path (the dependency that gates the most other work) are addressed early, when there is still time to absorb surprises. Effort informs sequencing only in that an item must fit the remaining capacity.

### Sequence to expose the critical path and dependencies early

Dependencies are where plans die. If task B depends on task A, and A is discovered to be hard in week 3 of a 4-week sprint, B cannot start and the plan slips. Sequence work so that dependencies and unknowns surface early: start the riskiest, most uncertain, or most dependency-laden work first, when there is time to react. Identify the critical path (the longest chain of dependent tasks that determines the milestone date) and protect it; a slip on the critical path slips the milestone, while a slip off it may not. Make dependencies explicit (internal and cross-team) and confirm they are available when needed, rather than assuming.

### Size risk buffer for the work that always appears

Every sprint includes unplanned work: incidents, urgent bugs, scope discovered mid-task, and requests that could not be predicted. This is not a failure of planning; it is a feature of real systems. Build buffer into the plan for it, sized by historical experience (if the team historically spends 25% of each sprint on unplanned work, reserve 25%). Distinguish buffer for known-unknowns (unplanned work we know will happen, sized historically) from buffer for unknown-unknowns (genuine surprises, which require contingency at the milestone level, not the sprint level). A plan with no buffer will carry over every sprint; a plan with honest buffer will mostly complete.

### Define milestones in terms of verifiable outcomes, not activity

A milestone like "migrate to the new auth system" is an activity description, not an outcome; it is impossible to verify when it is "done" because done is undefined. Define milestones as verifiable outcomes with explicit completion criteria: "all user-facing traffic authenticates via the new system, the old system is decommissioned, and the migration is documented." Verifiable criteria force clarity about what success looks like, prevent the milestone from creeping, and make it possible to honestly report progress. Every milestone and every significant task should have a definition of done that an outside observer could check.

### Treat the plan as a hypothesis to revise, not a contract to defend

A plan is the team's best prediction at planning time, and it will be wrong in some respect. When reality diverges (a task is harder, a dependency slips, a priority changes), the correct response is to revise the plan transparently and re-decide what fits the remaining capacity, not to silently overwork or to abandon planning. Build a mid-sprint check-in to compare actuals to the plan and adjust. Communicate changes (scope reduced, date moved) to stakeholders as they happen, not at the end. A plan that is never revised has been abandoned; a plan that is revised in the open is being used.

### Make carryover a signal, not a habit

Carrying unfinished work from sprint to sprint is sometimes legitimate (a dependency slipped) but often a symptom of over-commitment or unclear scope. Track carryover and investigate when it is chronic: if the team carries 30% every sprint, the plan is over-packed and capacity estimates are wrong. Do not let carryover become invisible by just rolling everything forward; each carried item should be a deliberate re-decision (still worth doing? still this priority?) rather than inertia.

## Common Traps

### Planning to theoretical capacity

Assuming 100% of time goes to planned work ignores support, incidents, and meetings. Plan to historically-observed completion, not headcount times hours, or carry over every sprint.

### Doing the easy, well-understood work first

Small items get done because they are easy to commit to, while the high-value uncertain work slips forever. Prioritize by value and dependency, with effort as a constraint.

### Discovering a dependency in week 3 of a 4-week sprint

Dependencies kill plans when discovered late. Sequence risky, uncertain, and dependency-laden work first, and make dependencies explicit and confirmed.

### No buffer for unplanned work

Every sprint includes incidents and surprises. A plan with no buffer carries over every cycle. Size buffer by historical unplanned-work ratio.

### Milestones defined as activities, not outcomes

"Build the reporting module" has no verifiable done. Define milestones as outcomes with explicit completion criteria an observer could check.

### Treating the plan as a contract that cannot change

When reality diverges, the team overworks silently or abandons the plan. Revise transparently, communicate changes, and treat the plan as a hypothesis.

### Chronic carryover treated as normal

If work carries over every sprint, the plan is over-packed. Track carryover and re-decide each carried item rather than rolling forward by inertia.

## Self-Check

- Is the sprint planned to historically-observed completion capacity (accounting for support, incidents, meetings, on-call), not theoretical headcount-times-hours?
- Is the backlog prioritized by value and dependency (critical path first), with effort as a constraint rather than the primary sort, so high-value work is not displaced by easy items?
- Are dependencies (internal and cross-team) made explicit, confirmed available, and sequenced so the riskiest or most dependency-laden work starts early enough to react?
- Is there buffer sized to the team's historical ratio of unplanned work, distinguishing known-unknown buffer (sprint level) from unknown-unknown contingency (milestone level)?
- Is every milestone defined as a verifiable outcome with explicit completion criteria (definition of done), rather than an activity description?
- Is there a mid-sprint check-in to compare actuals to the plan, and are revisions communicated to stakeholders as they happen rather than at the end?
- Is carryover tracked, and is chronic carryover investigated as a symptom of over-commitment rather than accepted as normal?
- When reality diverges from the plan, is the plan revised transparently and re-decided, rather than defended as a contract or silently abandoned?
