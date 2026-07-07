---
name: project_integration_and_dependency_coordination.md
description: Use when the agent is coordinating dependencies across workstreams, teams, or sub-projects, integrating multiple project plans, managing interfaces between components, synchronizing cross-team delivery, or diagnosing why integration failed despite each team delivering its component on time.
---

# Project Integration And Dependency Coordination

Integration is the project manager's most distinct responsibility: ensuring that the separate workstreams, deliverables, and plans come together into a coherent whole that achieves the project's objectives. Individual teams can each deliver their component on time and on budget, and the project can still fail if the components do not integrate, if dependencies were not coordinated, or if interfaces were not defined. The judgment problem is that integration is invisible until it fails. Dependencies are often assumed rather than managed, interfaces are left implicit, and cross-team coordination is treated as each team's responsibility rather than a deliberate integration function. The skill is making dependencies explicit, defining interfaces, synchronizing delivery across teams, and managing integration as a first-class project activity.

Use this skill when coordinating multiple workstreams or teams, when integrating project plans, when managing component interfaces, when synchronizing cross-team delivery, or when integration has failed despite component-level success. The goal is to prevent the agent from leaving dependencies implicit, from assuming interfaces will work themselves out, from treating integration as automatic, or from discovering integration failures late when correction is most expensive.

## Core Rules

### [ ] Make Dependencies Explicit And Managed

Dependencies, where one team's work depends on another's output, are a primary source of integration risk. They must be identified, made explicit, assigned ownership, and actively managed. Implicit dependencies are discovered when they break, often late and expensively.

- [ ] Identify all cross-team and cross-workstream dependencies.
- [ ] Document each dependency: who depends on whom, on what, and by when.
- [ ] Assign ownership for managing each dependency.
- [ ] Track dependencies in regular coordination reviews.

### [ ] Define Interfaces Early And Formally

Interfaces, the points where components connect, must be defined early and formally, before teams build to incompatible assumptions. An interface definition specifies what each side provides and expects, in what format, and with what timing. Undefined interfaces are a leading cause of integration failure.

- [ ] Define interfaces between components before independent development begins.
- [ ] Specify what each side provides, expects, the format, and the timing.
- [ ] Document interfaces formally and version them.
- [ ] Validate interface definitions with all parties before building.

### [ ] Synchronize Delivery Across Teams

Teams working to independent schedules produce components that are ready at different times, delaying integration. Synchronize delivery through shared milestones, integration points, and aligned schedules so that components are ready to integrate when needed.

- [ ] Establish shared integration milestones across teams.
- [ ] Align team schedules around integration points.
- [ ] Define what "ready for integration" means for each component.
- [ ] Use integration milestones as synchronization points, not just deadlines.

### [ ] Plan Integration As A First-Class Activity

Integration is not something that happens automatically at the end; it is an activity that must be planned, resourced, and scheduled. Integration planning defines how components will be combined, tested, and verified, and allocates time and resources for it.

- [ ] Plan integration activities explicitly in the schedule.
- [ ] Allocate resources and time for integration, not just component development.
- [ ] Define integration test cases and acceptance criteria.
- [ ] Schedule integration early and incrementally, not only at the end.

### [ ] Integrate Incrementally, Not In A Big Bang

A big-bang integration, combining all components at the end, concentrates risk and makes defect isolation nearly impossible. Incremental integration, combining components in stages, surfaces defects early when they are easier to isolate and fix. Integrate as early and as often as feasible.

- [ ] Integrate components incrementally rather than in a single end-of-project event.
- [ ] Establish integration builds or test environments early.
- [ ] Integrate the riskiest interfaces first to surface problems early.
- [ ] Use each integration as a verification point before adding more components.

### [ ] Coordinate Across Teams Through Structured Mechanisms

Cross-team coordination requires structured mechanisms: integration meetings, dependency tracking, shared risk registers, and joint reviews. Relying on informal communication between teams loses information and misses dependencies. Formalize coordination.

- [ ] Establish regular cross-team integration and coordination meetings.
- [ ] Maintain a shared dependency and interface tracker.
- [ ] Use joint reviews at integration milestones.
- [ ] Maintain a shared risk register for integration risks.

### [ ] Assign Integration Ownership

Integration does not happen by itself; someone must own it. Assign clear ownership for integration, whether a dedicated integration manager, a lead, or the project manager, with the authority to resolve interface disputes and drive integration activities.

- [ ] Assign clear ownership for integration activities and decisions.
- [ ] Give the integration owner authority to resolve interface disputes.
- [ ] Ensure the owner tracks integration progress and risks.
- [ ] Avoid leaving integration as everyone's and therefore no one's responsibility.

### [ ] Test Integration Continuously

Integration testing verifies that components work together, not just that each works alone. Continuous integration testing, running integration tests as components evolve, catches interface defects early rather than at final integration.

- [ ] Define and run integration tests continuously as components evolve.
- [ ] Test interfaces specifically, not just end-to-end functionality.
- [ ] Automate integration tests where possible to run them frequently.
- [ ] Treat integration test failures as priority defects.

### [ ] Manage Interface Changes Through Control

Interfaces, once defined, become commitments that teams build to. Changes to interfaces must flow through change control and be communicated to all affected parties. Uncontrolled interface changes break integration silently.

- [ ] Treat interface definitions as baselined commitments.
- [ ] Route interface changes through formal change control.
- [ ] Communicate approved interface changes to all affected teams promptly.
- [ ] Track interface change impact across all dependent components.

### [ ] Learn From Integration Failures

When integration fails, analyze why: was the interface undefined, the dependency unmanaged, the integration unplanned? Use integration failures to improve the integration process for future workstreams and projects.

- [ ] Analyze the root cause of every integration failure.
- [ ] Identify whether the failure was interface, dependency, or process related.
- [ ] Feed lessons into improved integration planning.
- [ ] Capture integration lessons in the project's lessons learned.

## Common Traps

### [ ] Implicit Dependencies

Assuming dependencies will work out rather than identifying and managing them.

### [ ] Undefined Interfaces

Letting teams build to incompatible assumptions about how components connect.

### [ ] Independent Schedules

Teams delivering to unaligned schedules, delaying integration.

### [ ] Big-Bang Integration

Combining all components at the end, concentrating risk and complicating defect isolation.

### [ ] Integration As Afterthought

Failing to plan, resource, and schedule integration as a deliberate activity.

### [ ] No Integration Ownership

Leaving integration as everyone's responsibility and therefore no one's.

### [ ] Component-Only Testing

Testing each component in isolation without testing the interfaces between them.

### [ ] Uncontrolled Interface Changes

Changing interfaces without control, breaking integration silently.

## Self-Check

- [ ] Are cross-team dependencies identified, documented, owned, and actively managed?
- [ ] Are interfaces between components defined early and formally before development?
- [ ] Is delivery synchronized across teams through shared integration milestones?
- [ ] Is integration planned, resourced, and scheduled as a first-class activity?
- [ ] Is integration incremental rather than a single big-bang event?
- [ ] Are there structured coordination mechanisms across teams?
- [ ] Is clear ownership assigned for integration with authority to resolve disputes?
- [ ] Is integration testing continuous, including interface-specific tests?
- [ ] Are interface changes managed through formal change control?
- [ ] Are integration failures analyzed for root cause and fed into process improvement?
