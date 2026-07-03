---
name: dependency_driven_sequencing_and_critical_chain.md
description: Use when the agent is sequencing work driven by dependencies, identifying dependency-driven critical chains, resolving circular or conflicting dependencies, or deciding where to concentrate schedule protection and buffer based on the true constraint structure.
---

# Dependency Driven Sequencing And Critical Chain

Sequencing is where a plan becomes executable or fictional. When work is sequenced by convenience or by who shouts loudest, the schedule ignores the dependency structure that actually governs when things can happen. Critical chain and critical path analysis exist to reveal the true constraint: the sequence of dependencies, including resource dependencies, that determines how fast the project can possibly complete. Agents tend to sequence by duration or by team preference, identify a critical path by longest task chain, and then add buffer everywhere or nowhere. The result is a plan that protects the wrong tasks and leaves the real constraint exposed.

The judgment problem is how to sequence work so the dependency structure drives the order, how to identify the true critical chain including resource constraints, how to resolve circular and conflicting dependencies, and how to concentrate schedule protection where the constraint actually is rather than spreading it thin. Sequencing is a constraint-analysis discipline, not a task-ordering exercise.

## Core Rules

### Sequence From The Dependency Structure, Not From Task Duration Or Preference

The right sequence is dictated by what must precede what, not by which task is longest or which team wants to start first. Build the sequence by walking the dependency network: for each piece of work, what are its hard predecessors, what does it enable, and where can it overlap. Only after the dependency-driven sequence is established should you consider duration and capacity. Sequencing by duration produces a plan that looks optimized but violates dependencies; sequencing by preference produces a plan that reflects politics rather than feasibility. The dependency structure is the authority.

### Identify The Critical Chain Including Resource Constraints

The critical path, defined by logical dependencies alone, often misses the real constraint: a shared resource that serializes work the logic would allow in parallel. Critical chain analysis adds resource dependencies to logical ones and finds the longest sequence through both, which is the true determinant of project duration. A plan that optimizes the logical critical path while ignoring the resource-critical chain protects the wrong sequence and still slips on the resource bottleneck. Identify the critical chain, the combined logical-plus-resource longest path, and treat that as the schedule's spine.

### Concentrate Buffer On The Critical Chain, Not Scattered Across Tasks

Traditional scheduling pads every task with its own safety margin, which gets wasted through student syndrome and Parkinson's law, work expands to fill the time, and the slack never accumulates where it is needed. Critical chain practice instead strips individual task padding and consolidates protection into project and feeding buffers placed where they protect the critical chain. Concentrate buffer at the points that guard the constraint, and manage buffer consumption as the primary schedule health signal. Buffer scattered everywhere protects nothing; buffer concentrated on the critical chain protects the project.

### Resolve Circular Dependencies By Restructuring, Not By Ignoring Them

A circular dependency, where work loops back on itself, is an execution deadlock. It cannot be sequenced; it must be restructured. Break the cycle by splitting a task into parts, introducing an intermediate deliverable that decouples the loop, changing the sequence, or reframing the dependency as iterative rather than strictly sequential where the work genuinely allows. Ignoring a cycle, or assuming the team will figure it out during execution, guarantees a stall. Resolve circular dependencies during planning, when restructuring is cheap, rather than during execution, when it is expensive.

### Resolve Conflicting Dependencies By Surfacing The Tradeoff

Conflicting dependencies arise when two constraints pull in opposite directions: a logical dependency says A before B, but a resource or external dependency effectively requires B before or alongside A. Do not resolve these silently by picking one. Surface the conflict, name the tradeoff, and make a deliberate choice: which constraint is hard and must be honored, which can be relaxed, and what the consequence of the choice is. Conflicting dependencies are decision points, and hiding them in the schedule guarantees that the wrong one gets resolved by accident during execution.

### Use Feeding Buffers To Protect The Critical Chain From Non-Critical Slippage

Non-critical work feeds into the critical chain at merge points, and slippage on a feeding path can consume critical-chain slack even though the feeding path had float. Place feeding buffers where non-critical paths join the critical chain, so that delay on a feeding path absorbs into its buffer rather than propagating onto the constraint. Without feeding buffers, the critical chain is exposed to slippage from paths that were supposed to have slack. Protect the merge points, because that is where non-critical delays become critical.

### Manage The Constraining Resource As A Shared Asset

When the critical chain runs through a constraining resource, a shared specialist, a rare environment, or a single tool, that resource must be managed deliberately. Schedule it to avoid context switching and multitasking, which destroy its throughput, and prioritize its work by what protects the critical chain. A constraining resource that is allowed to multitask across many tasks will lengthen every one of them and extend the project. Treat the constraint as the project's pacemaker and protect its focus.

### Re-Examine The Critical Chain As Conditions Change

The critical chain is not static. When tasks slip, scope changes, resources shift, or dependencies resolve differently than planned, the constraint can move to a different part of the network. Re-run the critical chain analysis when significant changes occur, because protecting a chain that is no longer critical wastes effort while the new critical chain goes unprotected. The constraint moves; the plan's attention must move with it.

## Common Traps

### Sequencing By Duration Or Team Preference

Ordering work by what is longest or who is loudest ignores the dependency structure. The trap is a plan that violates dependencies and stalls during execution. Sequence from the network.

### Identifying The Critical Path While Ignoring Resource Constraints

Optimizing the logical longest path misses the resource bottleneck that actually determines duration. The trap is protecting a chain that is not the real constraint. Find the critical chain including resources.

### Scattering Buffer Across Every Task

Padding each task individually lets the slack get wasted and never accumulate where needed. The trap is a plan with no usable protection despite appearing padded. Concentrate buffer on the critical chain.

### Ignoring Circular Dependencies

Assuming the team will resolve a cycle during execution guarantees a stall. The trap is a deadlock discovered too late to restructure cheaply. Resolve cycles during planning.

### Silently Picking A Side In Conflicting Dependencies

Resolving a conflict without surfacing the tradeoff lets the wrong constraint win by accident. The trap is a schedule that looks consistent but violates a hard constraint. Make the choice deliberately.

### No Protection At Merge Points

Letting non-critical feeding paths join the critical chain without buffer exposes the constraint to slippage from paths that had float. The trap is critical-chain delay from supposedly non-critical work. Use feeding buffers.

### Multitasking The Constraining Resource

Allowing the bottleneck resource to context-switch destroys throughput and lengthens the project. The trap is that every task on the constraint runs slower. Protect the constraint's focus.

### Treating The Critical Chain As Static

Protecting a chain that is no longer critical while the real constraint has moved wastes effort. The trap is attention fixed on the wrong part of the network. Re-examine when conditions change.

## Self-Check

- [ ] Is the work sequence derived from the dependency structure rather than from task duration or team preference?
- [ ] Has the critical chain been identified including resource constraints, not just the logical critical path?
- [ ] Is schedule buffer concentrated on the critical chain and at feeding merge points rather than scattered across every task?
- [ ] Have circular dependencies been detected and resolved by restructuring during planning, not deferred to execution?
- [ ] Are conflicting dependencies surfaced as explicit tradeoffs with a deliberate choice about which constraint is hard?
- [ ] Are feeding buffers placed where non-critical paths join the critical chain to absorb feeding slippage?
- [ ] Is the constraining resource managed as a shared asset, protected from multitasking and scheduled to protect the chain?
- [ ] Is the critical chain re-examined when tasks slip, scope changes, or resources shift, so attention tracks the real constraint?
- [ ] Can you identify, from the plan, the single sequence that determines project duration and where its protection sits?
- [ ] Has the plan avoided adding safety margin to every task in a way that wastes slack and leaves the constraint exposed?
