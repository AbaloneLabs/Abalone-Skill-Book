---
name: bottleneck-and-throughput-control.md
description: Use when the agent is analyzing production bottlenecks, throughput constraints, line balancing, work-in-progress buildup, cycle time, takt, queueing, constraint management, throughput improvement, or production flow where risks include optimizing non-bottleneck work, hiding starvation or blocking, increasing WIP, worsening quality, relying on misleading averages, or improving local productivity while reducing system output.
---

# Bottleneck And Throughput Control

Throughput is a system property, not the sum of everyone working faster. A line can be full of busy people and still fail because one constraint controls output. Agents often suggest improving the most visible task, adding labor, or pushing more work into the system without identifying the real bottleneck. This skill helps the agent inspect production flow and make throughput decisions that improve total output without increasing chaos.

Use this skill when reviewing bottlenecks, cycle time, WIP, line balance, production queues, throughput loss, station utilization, or process improvement in a production environment. The agent should focus on the constraint and the flow of work, not isolated productivity.

## Core Rules

### Identify the actual constraint

The bottleneck is the step that limits total output under current conditions. It may be a machine, operator skill, inspection, material delivery, changeover, tooling, testing, packaging, software system, quality release, or downstream storage. Identify it with evidence: queue buildup before the step, starvation after it, cycle time, downtime, utilization, and output history.

Do not assume the loudest complaint is the bottleneck. A visible busy station may be a symptom of an upstream or downstream constraint.

### Measure flow, not only utilization

High utilization can be harmful when it creates queues, long lead times, fatigue, or blocked work. Track throughput, cycle time, WIP, queue age, downtime, changeover loss, rework, first-pass yield, and customer due date performance. A station can look efficient while the system slows.

Use averages carefully. Average cycle time can hide frequent stoppages, long tails, or mix changes. Look at variance and the conditions under which flow breaks.

### Protect the constraint

Once the constraint is known, ensure it is not waiting for material, tools, instructions, maintenance, quality decisions, or operator coverage. Keep it working on the highest-priority, ready work. Avoid starving it with poor staging or blocking it with downstream congestion.

Improving non-bottleneck steps may be useful for quality or safety, but it will not increase throughput unless the constraint moves. Make that distinction explicit.

### Control work-in-progress

Pushing more work into the system often increases WIP, confusion, damage, search time, and lead time without increasing output. Use WIP limits, staging rules, visual controls, and release discipline to keep flow manageable.

When WIP builds, ask why: downstream blockage, batch size, quality hold, material mismatch, staffing, machine downtime, or schedule resequencing. WIP is evidence of a flow problem.

### Consider product mix and changeover effects

Throughput changes with product mix, complexity, batch size, cleaning, tooling, setup, inspection, and packaging. A line that meets output targets on one mix may fail on another. Compare performance by mix before declaring a general bottleneck.

Scheduling similar work together may reduce changeover, but it may also delay urgent orders or create inventory imbalance. Evaluate the tradeoff.

### Avoid throughput gains that damage quality or safety

Faster output is not a success if defects, rework, injuries, near misses, or customer failures rise. Check whether people skip checks, bypass guards, shorten curing or test times, reduce cleaning, or hide rework to hit rate targets.

Throughput improvement should preserve required quality and safety controls. If a control is too slow, redesign it rather than bypassing it.

### Use experiments before permanent changes

For proposed changes such as staffing shifts, layout changes, batch size changes, tooling, automation, or schedule rules, run controlled tests when possible. Define expected effect, measurement, duration, and rollback. Production systems are interconnected; a local fix can create a downstream constraint.

Document what changed and what was learned. Without a clear experiment, teams may argue from anecdotes.

### Reassess after the bottleneck moves

When one constraint improves, another will control the system. Reassess flow instead of continuing to optimize the old bottleneck. This is normal and should be expected.

Throughput management is continuous. The goal is not to eliminate every constraint; it is to manage the current constraint deliberately.

## Common Traps

- Improving a non-bottleneck station and expecting total output to rise.
- Treating high utilization as proof of good flow.
- Adding work-in-progress to "keep everyone busy" while increasing lead time and confusion.
- Ignoring product mix, batch size, changeover, and inspection differences.
- Speeding work by bypassing quality or safety controls.
- Using average cycle time while ignoring variance, downtime, and long-tail delays.
- Making permanent layout or staffing changes from one anecdotal shift.
- Continuing to optimize yesterday's bottleneck after the constraint moved.

## Self-Check

- Is the current bottleneck identified with queue, starvation, cycle time, downtime, and output evidence?
- Are throughput, WIP, cycle time, queue age, yield, and due date performance reviewed together?
- Is the constraint protected from waiting on materials, tools, people, instructions, and downstream space?
- Are non-bottleneck improvements labeled as quality, safety, or future-capacity work rather than immediate throughput gains?
- Are WIP limits and release rules preventing uncontrolled buildup?
- Is product mix and changeover effect included in the analysis?
- Are quality and safety metrics checked alongside throughput?
- Are proposed changes tested with clear measurement and rollback where practical?
- Has the team reassessed whether the bottleneck moved after improvement?
- Would the change improve total system output, not just local productivity?
