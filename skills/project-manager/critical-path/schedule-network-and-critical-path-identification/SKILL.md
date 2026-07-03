---
name: schedule_network_and_critical_path_identification.md
description: Use when the agent is building a schedule network diagram, computing earliest and latest start and finish dates through forward and backward pass analysis, identifying the critical path, or determining which tasks drive overall project duration.
---

# Schedule Network And Critical Path Identification

The critical path method is a computational technique, not a metaphor. It turns a list of activities and their dependencies into a network, then performs a forward pass to find the earliest each activity can occur and a backward pass to find the latest it can occur without delaying the project. The activities where earliest and latest coincide have zero float and form the critical path: the longest dependent chain that sets the project duration. The craft is in building the network honestly, performing the passes correctly, and reading the results without fooling yourself about what drives the end date. Agents tend to assign dates before building the network, to omit hidden dependencies, and to assume a single critical path when near-critical paths matter just as much.

Use this skill before constructing or auditing a schedule network, performing critical path analysis, or explaining what drives project duration. The goal is to prevent the agent from producing a network that looks computed but rests on incomplete logic or mistaken passes.

## Core Rules

### Build The Network Before Any Dates

A defensible critical path analysis begins with logic, not with a calendar. List every activity, then define the dependency relationships between them before any start or finish date is attached. The network is a model of what must precede what; dates are derived from it, not inputs to it. Dating first and justifying later produces a schedule that collapses on the first real interaction.

Represent dependencies explicitly as relationships between activities, and capture leads and lags deliberately rather than letting them hide inside inflated durations.

### Model All Four Dependency Types Correctly

Most dependencies are finish-to-start, where a successor begins after a predecessor ends, but the network often needs the others. Start-to-start lets activities begin together, often with a lag; finish-to-finish requires activities to complete together; start-to-finish is rare but real. Choosing the wrong type distorts the network: a finish-to-start used where a start-to-start with lag is correct injects false serial time and lengthens the apparent critical path.

Question every finish-to-start; many are actually start-to-start with a lag, and fixing them reveals real flexibility.

### Include External, Resource, And Decision Dependencies

A logic-only network assumes infinite resources and cooperative externals. Real projects depend on vendors, approvals, permits, shared specialists, and decisions from people outside the team. These must appear as constraints on the network, not as hopeful assumptions. An external approval modeled as zero-duration milestone with a finish-to-start link is still a dependency; ignoring it makes the network optimistic.

Resource dependencies are especially dangerous because they are invisible in a logic-only network and often create the real critical path once leveling is applied.

### Perform The Forward Pass To Find Earliest Dates

The forward pass moves from start to finish through the network, computing the earliest start and earliest finish of each activity by adding its duration to the earliest finish of its predecessors. Where multiple predecessors merge, take the latest earliest finish as the constraint. The forward pass yields the earliest the project can finish and the earliest each activity can occur assuming everything goes perfectly upstream.

Record earliest start and earliest finish for every activity; these are the inputs to float calculation.

### Perform The Backward Pass To Find Latest Dates

The backward pass moves from the project finish back to the start, computing the latest finish and latest start of each activity by subtracting its duration from the latest start of its successors. Where multiple successors branch, take the earliest latest start as the constraint. The backward pass yields the latest each activity can occur without delaying the project end.

The backward pass is where float is revealed: the gap between earliest and latest dates for each activity.

### Identify The Critical Path As Zero-Float Activities

Activities whose earliest and latest dates coincide have zero float: any delay delays the project. The chain of these activities is the critical path, and its total duration is the project duration. There can be more than one critical path, and the path can shift as the schedule changes. Always compute float rather than guessing the critical path by intuition, because non-obvious chains often turn out to be critical.

Identify and label the critical path explicitly so attention and protection focus there.

### Track Near-Critical Paths, Not Just The Critical One

A path with only a day or two of float is near-critical: a small delay makes it critical, and two near-critical paths slipping together can both become critical and stretch the project. List the top few paths by float and monitor them alongside the critical path. Treating only the current critical path as important is how projects get surprised by a path that was quietly catching up.

Recompute after every schedule update, because float shifts and the critical path can move.

### Resource-Level Before Trusting The Critical Path

A logic-only critical path assumes infinite resources. When shared resources are-leveled, the resulting resource-critical path is usually longer and is the honest forecast. Perform critical path analysis on the resource-leveled network, not the logic-only one, or you will understate duration and misplace the critical path.

## Common Traps

### Dating Before Building The Network

Assigning dates first and reverse-engineering logic produces a schedule that breaks on real dependencies. Build logic first.

### Defaulting Everything To Finish-To-Start

Overusing finish-to-start injects false serial time and hides real concurrency. Question each dependency type.

### Omitting External And Resource Dependencies

A logic-only network ignores vendors, approvals, and shared specialists, producing an optimistic duration. Model them as constraints.

### Guessing The Critical Path By Intuition

Intuition misses non-obvious critical chains. Compute float through forward and backward passes.

### Assuming A Single Critical Path

Multiple critical paths can coexist, and near-critical paths can overtake the current one. Track the top few paths.

### Ignoring Resource Leveling

The logic-only critical path understates duration once shared resources are-leveled. Analyze the leveled network.

### Stale Network After Changes

Float shifts and the critical path moves as the schedule changes. Recompute after every update.

### Merging Successors Incorrectly In The Passes

Taking the wrong predecessor or successor value at merge points corrupts every downstream date. Apply the max at merges in the forward pass and the min in the backward pass.

## Self-Check

- [ ] Is the network of activities and dependencies built before any calendar dates are assigned?
- [ ] Are dependency types chosen deliberately, with finish-to-start questioned where start-to-start with lag fits?
- [ ] Are external, approval, and resource dependencies modeled as explicit constraints?
- [ ] Does the forward pass compute earliest start and finish, taking the latest predecessor finish at merges?
- [ ] Does the backward pass compute latest start and finish, taking the earliest successor start at branches?
- [ ] Is the critical path identified as the zero-float chain rather than guessed by intuition?
- [ ] Are near-critical paths with small float listed and monitored alongside the critical path?
- [ ] Is critical path analysis performed on the resource-leveled network, not only the logic-only one?
- [ ] Are multiple critical paths acknowledged where they exist?
- [ ] Is the network recomputed after every schedule update so float and the critical path stay current?
