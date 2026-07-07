---
name: critical_path_and_dependency_management.md
description: Use when the agent is building or analyzing a project schedule network, identifying and managing the critical path, sequencing activities and managing dependencies, resolving resource leveling conflicts on the critical path, deciding where to add buffer or crash activities, or diagnosing why a project is slipping when individual activities appear on time.
---

# Critical Path And Dependency Management

A project schedule is a network of activities connected by dependencies, and the project's duration is determined not by the sum of activity durations but by the longest path through that network, the critical path. This is counterintuitive: teams often focus on activity durations, working to estimate and shorten individual tasks, while the real driver of project duration, the dependency structure and the critical path, receives less attention. The result is schedules that look detailed and are wrong, because they miss dependencies, misidentify the critical path, or allow non-critical work to consume resources needed on the critical path. The judgment problem is building schedules that reflect the real dependency structure, identifying and protecting the critical path, managing the resource and sequence decisions that affect it, and recognizing that schedule recovery requires acting on the critical path, not on activities that have float.

Use this skill when building or analyzing schedule networks, identifying the critical path, managing dependencies, leveling resources, deciding on crashing or buffer, or diagnosing schedule slippage. The goal is to prevent the agent from building schedules that ignore dependencies, from misidentifying or losing track of the critical path, from wasting effort shortening non-critical activities, and from adding buffer in the wrong places.

## Core Rules

### Build The Schedule As A Dependency Network, Not A Task List

A task list with durations is not a schedule; it is a list. A schedule is a network in which activities are connected by dependencies that determine when each can start and finish. Building the network, mapping which activities depend on which, is what transforms a list into a schedule that can predict project duration.

Build the network by:

- identifying dependencies between activities, finish-to-start, start-to-start, and others;
- mapping the full dependency structure, not just the obvious sequential relationships;
- distinguishing mandatory dependencies, inherent in the work, from discretionary ones, chosen sequencing;
- validating the network with the team to catch dependencies individuals did not see.

A schedule without a mapped dependency network cannot identify the critical path and cannot predict duration accurately.

### Identify And Track The Critical Path

The critical path is the longest path through the network, the sequence of dependent activities that determines the project's minimum duration. Activities on the critical path have no float; any delay on them delays the project. Identifying the critical path focuses management attention where it affects the schedule.

Identify and track by:

- computing the critical path through forward and backward pass analysis;
- tracking critical path activities closely, since delays there delay the project;
- recognizing that the critical path can shift as work progresses or conditions change;
- re-computing the critical path when the schedule changes, not assuming it is static.

A project managed without knowing its critical path manages all activities equally and therefore manages the decisive ones inadequately.

### Protect The Critical Path From Disruption

Because critical path activities have no float, disruptions to them flow directly to project duration. Protecting the critical path, ensuring its activities have resources, start on time, and are not delayed by avoidable issues, is the highest-leverage schedule management activity.

Protect by:

- prioritizing critical path activities for resources and attention;
- starting critical path activities on time rather than letting early delays accumulate;
- avoiding loading non-critical work onto resources needed for the critical path;
- monitoring critical path activities for emerging risks and acting before they delay;
- recognizing that a day saved on the critical path saves a day on the project.

Effort spent shortening a non-critical activity by a week may save nothing on the project; effort spent protecting the critical path by a day saves a day.

### Manage Dependencies Actively, Not Passively

Dependencies are not just lines on a network diagram; they are points where one activity's completion enables another's start, and they require active management. Unmanaged dependencies produce delays where a downstream activity cannot start because an upstream one slipped, or where a handoff fails because the dependency was not coordinated.

Manage actively by:

- tracking dependency completion, not just activity progress;
- coordinating handoffs between activities and teams;
- identifying external dependencies, on other projects, vendors, or approvals, and managing them explicitly;
- recognizing that discretionary dependencies can be relaxed to improve schedule where appropriate;
- surfacing dependency risks before they delay downstream work.

A dependency that is a line on a chart but not managed in execution will produce a delay that surprises no one who was watching.

### Distinguish Critical Path From Resource-Critical Sequences

The traditional critical path assumes unlimited resources, but real projects have resource constraints, and the resource-leveled schedule may have a different critical sequence, the resource-critical path, driven by resource availability rather than activity dependencies alone. Ignoring resource constraints produces a critical path that is theoretically correct and practically wrong.

Distinguish by:

- resource-leveling the schedule to reflect actual resource availability;
- identifying the resource-critical sequence, which may differ from the dependency critical path;
- recognizing that adding resources to a dependency-critical activity may not help if the resource-critical sequence constrains it;
- managing both dependency and resource constraints on the schedule.

A critical path computed without resource constraints predicts a duration the project cannot actually achieve.

### Use Buffer Deliberately At The Right Level

Buffer, schedule contingency, is essential because uncertainty is real, but where it is placed matters. Buffer distributed across activities, each padded a little, gets consumed by Parkinson's law, work expands to fill available time, and is unavailable when needed. Buffer concentrated at the project or milestone level, as in critical chain project management, remains available for the risks that materialize.

Use buffer deliberately by:

- concentrating buffer at milestone or project level rather than distributing it across activities;
- sizing buffer based on accumulated uncertainty, not arbitrary percentages;
- monitoring buffer consumption as a leading indicator of schedule health;
- avoiding the temptation to consume buffer early through Parkinson's law.

Buffer spread across activities disappears into expanded work; buffer held at the right level remains available when risks hit.

### Recover Schedule Through Critical Path Action

When a project slips, schedule recovery requires acting on the critical path, because only critical path changes reduce project duration. Shortening non-critical activities, adding resources to activities with float, or working harder on off-path work does not recover the schedule. Recovery means critical path action: crashing, fast-tracking, or scope reduction on the path.

Recover through critical path by:

- focusing recovery effort on critical path activities, where duration reduction helps;
- using crashing, adding resources to critical activities, where it is effective;
- using fast-tracking, running sequential activities in parallel, where dependencies allow;
- reducing scope on the critical path where crashing and fast-tracking are insufficient;
- recognizing that recovery effort off the critical path is wasted.

A recovery plan that works non-critical activities while the critical path slips is not recovering the schedule.

### Recognize When The Schedule Is Wrong And Replan

Schedules are predictions and they are wrong, sometimes slightly, sometimes substantially. The discipline is recognizing when the schedule has diverged enough from reality that it is no longer a useful predictor, and replanning rather than executing against a fiction. Clinging to a schedule that reality has invalidated produces reporting that no one believes and decisions based on false predictions.

Recognize and replan by:

- monitoring schedule divergence and recognizing when it exceeds the schedule's useful accuracy;
- replanning the schedule when assumptions, dependencies, or scope have changed substantially;
- communicating schedule changes and their causes transparently;
- avoiding the temptation to hide slippage through optimistic re-estimation.

A schedule that no one believes because it has diverged from reality is worse than no schedule, because it supports false decisions.

## Common Traps

### Task List Without Dependency Network

A list is not a schedule. Map dependencies to build a network that can predict duration.

### Lost Track Of The Critical Path

The critical path determines duration. Identify, track, and re-compute it as conditions change.

### Loading Non-Critical Work Onto Critical Resources

Non-critical work consuming critical resources delays the project. Protect critical path resources.

### Passive Dependency Management

Dependencies require active coordination. Track completion and coordinate handoffs.

### Ignoring Resource Constraints In Critical Path Analysis

The dependency critical path may differ from the resource-critical sequence. Resource-level the schedule.

### Distributed Buffer Consumed By Parkinson's Law

Buffer spread across activities disappears. Concentrate buffer at the right level.

### Recovery Effort Off The Critical Path

Only critical path action recovers schedule. Crashing non-critical work does not help.

### Clinging To An Invalidated Schedule

Schedules that diverge from reality support false decisions. Replan when divergence is substantial.

## Self-Check

- [ ] The schedule is built as a dependency network, not just a task list with durations.
- [ ] The critical path is identified, tracked, and re-computed as conditions change.
- [ ] Critical path activities are protected through resource priority and proactive risk management.
- [ ] Dependencies are managed actively, with handoffs coordinated and external dependencies tracked.
- [ ] Resource constraints are reflected through leveling, and the resource-critical sequence is identified.
- [ ] Buffer is concentrated at the appropriate level, not distributed and consumed across activities.
- [ ] Schedule recovery focuses on critical path action, not non-critical work.
- [ ] The schedule is replanned when divergence from reality makes it no longer useful.
- [ ] Buffer consumption is monitored as a leading indicator of schedule health.
- [ ] No recovery plan relies on shortening activities that have float.