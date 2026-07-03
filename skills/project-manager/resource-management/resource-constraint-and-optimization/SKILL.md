---
name: resource_constraint_and_optimization.md
description: Use when the agent is resolving resource constraints, optimizing resource use under scarcity, applying theory of constraints to a project, deciding tradeoffs when resources are insufficient, or reviewing whether scarce resources are being used where they add the most value.
---

# Resource Constraint And Optimization

Every project is constrained. The question is not whether constraints exist but how they are managed. When a project treats constraints as obstacles to power through, it produces burnout, defects, and missed commitments. When it understands which constraint actually limits throughput and optimizes around it, the same resources produce more value. The project manager must identify the binding constraint, avoid local optimization that does not help the whole, and make explicit tradeoffs when demand cannot be met.

Use this skill before resolving a resource shortage, optimizing resource use, applying constraint management, or deciding what to cut when resources are insufficient. The goal is to prevent the agent from optimizing the wrong part of the project and from treating scarcity as a motivation problem rather than a structural one.

## Core Rules

### Identify The Binding Constraint

Throughput is limited by the binding constraint, the scarcest resource that the rest of the project depends on. It may be a specialist, a licensed tool, an environment, a budget cap, or a decision maker. Work piles up before the constraint and starves after it.

Find the binding constraint by looking for where work waits, where queues build, and where delays propagate. The constraint is rarely where the most visible activity is; it is where the most waiting is.

Every improvement that does not address the binding constraint is a local optimization that does not improve the whole project.

### Subordinate Everything Else To The Constraint

Once the constraint is identified, organize the rest of the project around it. Do not produce work faster than the constraint can absorb, because it only builds inventory that waits and obscures the real pace. Schedule the constraint's work first and feed it continuously.

Subordination often feels counterintuitive because it means deliberately not maximizing the use of non-constrained resources. But resources that are not the constraint should have idle time, and that idle time is correct, not wasteful, because pushing more work through them only creates congestion.

### Elevate The Constraint When Subordination Is Insufficient

If subordinating the rest of the project to the constraint is not enough, elevate the constraint: add capacity, add a second specialist, license another tool seat, automate part of the work, or change the process.

Elevation usually costs money or time, so it should follow, not precede, exploitation and subordination. Many projects jump to adding resources before they have fully used the constraint they already have.

### Avoid Local Optimization

Optimizing a non-constraint resource feels productive but does not improve the project. Making a fast team faster when a slow review process is the constraint just builds more work waiting for review.

Before optimizing, confirm the target is the constraint. If it is not, the optimization adds cost without benefit and may even worsen congestion by feeding the constraint faster than it can absorb.

### Make Tradeoffs Explicit When Demand Exceeds Supply

When resources are genuinely insufficient to meet scope, schedule, and quality together, the gap is a tradeoff to be decided, not absorbed. Surface the options:

- reduce scope to fit the resources;
- extend the schedule to fit the resources;
- add resources, accepting cost and ramp-up;
- accept quality risk, knowingly;
- reallocate from lower-priority work.

Present the tradeoff to the decision maker. Absorbing the gap through pressure produces the same outcome, but later, with more damage.

### Optimize For The Whole, Not For Local Efficiency

Local efficiency metrics, such as keeping every person fully busy, can reduce overall throughput by creating inventory and congestion. Optimize for the flow of value through the whole system, even if it means some resources are sometimes idle.

This requires resisting the instinct to measure utilization as a virtue. A non-constraint resource at less than full utilization is often a sign of a well-managed system, not a problem to fix.

### Manage Buffers At The Constraint

Because the constraint determines throughput, protect it from starvation and disruption. Maintain a small buffer of ready work before the constraint so it never waits for input, and protect it from interruptions that steal its capacity.

Buffers at the constraint are intentional and governed, not accidental inventory. They exist to keep the most valuable resource continuously productive.

### Re-Evaluate As The Constraint Moves

Solving one constraint often moves the constraint elsewhere. The specialist is no longer the bottleneck, and now the testing environment is. Constraint management is ongoing, not a one-time fix.

After elevating a constraint, look for where the next constraint has emerged and repeat the cycle. A project that solves one constraint and stops optimizing will soon be limited by the new one.

## Common Traps

### Optimizing A Non-Constraint

Improving a resource that is not the bottleneck adds cost without improving throughput.

### Maximizing Local Utilization

Keeping every resource busy creates congestion and inventory that slow the whole.

### Powering Through Scarcity

Treating resource shortage as a motivation problem produces burnout and defects, not throughput.

### Jumping To Elevation Before Exploitation

Adding resources before fully using the existing constraint wastes money and ramp-up time.

### Hidden Constraints

The constraint may be a process, a decision maker, or an environment, not an obvious person or tool.

### Absorbing Gaps Silently

Absorbing insufficient resources through pressure produces slippage and attrition instead of decisions.

### One-Time Constraint Fix

Solving one constraint and stopping leaves the project limited by the next one.

### Inventory Mistaken For Progress

Work piled before the constraint looks like progress but is waiting and obscures the real pace.

## Self-Check

- [ ] The binding constraint is identified by where work waits and queues build.
- [ ] Non-constraint resources are subordinated to the constraint, including accepting idle time where appropriate.
- [ ] The constraint is fully exploited before elevation through added capacity is considered.
- [ ] Optimization targets the constraint, not local efficiency of non-constraints.
- [ ] Demand-supply gaps are surfaced as explicit tradeoffs rather than absorbed through pressure.
- [ ] The project is optimized for whole-system flow of value, not local utilization.
- [ ] Buffers protect the constraint from starvation and interruption.
- [ ] After a constraint is elevated, the next emerging constraint is identified and managed.
- [ ] Local optimization that does not improve the whole is avoided.
- [ ] Inventory waiting before the constraint is recognized as waiting, not as progress.
