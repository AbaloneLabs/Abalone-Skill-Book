---
name: dependency_types_and_network_mapping.md
description: Use when the agent is mapping dependency networks, classifying dependency types including finish-to-start start-to-start finish-to-finish start-to-finish plus resource decision and external dependencies, or building a dependency model that exposes sequencing risk.
---

# Dependency Types And Network Mapping

A schedule is not a list of tasks with dates; it is a network of dependencies, and the shape of that network determines what the project can and cannot do. Most planning failures are dependency failures: work sequenced in the wrong order, a resource conflict treated as a finish-to-start link, an external dependency mistaken for an internal one, or a circular chain that quietly deadlocks execution. Agents tend to draw dependencies as simple before-after chains because that is what scheduling tools default to, then discover during execution that the real constraints, shared specialists, decisions, information, and external parties, were never modeled at all.

The judgment problem is how to classify dependencies by their true type so the model reflects reality, how to map the full network including the non-obvious constraint types, and how to build a dependency model that exposes sequencing risk rather than hiding it behind tidy finish-to-start arrows. The taxonomy matters because each dependency type demands different management: a resource dependency is solved by capacity, a decision dependency by governance, an external dependency by coordination and escalation.

## Core Rules

### Classify Each Dependency By Its True Type, Not The Default

Scheduling tools default to finish-to-start (FS), the predecessor must finish before the successor starts, but real projects use all four logical types. Finish-to-start is the strict sequence. Start-to-start (SS) with lag lets parallel work begin once a predecessor has started, useful when work can overlap. Finish-to-finish (FF) ties the completion of two tasks, common when integration requires both done. Start-to-finish (SF) is rare but real, for example a support rotation that cannot end until the next one begins. Choosing the wrong type either over-constrains the schedule (forcing strict sequence where overlap was possible) or under-constrains it (assuming overlap that is not safe). Model the logical relationship that actually exists, not the tool's default.

### Model Resource Dependencies Separately From Logical Ones

A resource dependency exists when two tasks compete for the same person, team, or asset and cannot run in parallel even though their logical relationship would allow it. These are not finish-to-start links; they are capacity constraints. Modeling them as logical FS hides the real cause and produces a schedule that looks feasible on paper but collapses when the shared specialist is double-booked. Represent resource dependencies explicitly, identify the constraining resources, and treat them as a distinct class so the schedule reflects what the people can actually do, not just what the logic permits.

### Capture Decision And Information Dependencies Explicitly

Many dependencies are not about tasks at all; they are about decisions and information. A design cannot proceed until an architecture decision is made; a build cannot start until requirements are stable; a test cannot run until test data is provided. These decision and information dependencies are easy to omit because they do not look like work, yet they are among the most common causes of delay. Map them explicitly: which decisions gate which work, which information feeds which task, and who owns resolving each. A dependency model that omits decisions and information is missing the constraints that most often bite.

### Treat External Dependencies As A Distinct, High-Risk Class

External dependencies, on vendors, customers, partner teams, platforms, regulators, or third parties, sit outside direct control and carry disproportionate risk. They deserve their own classification because their management is fundamentally different: you cannot assign or reassign them, you can only coordinate, contract, and escalate. Tag every external dependency, identify the external owner and the interface point, and track them with heightened scrutiny. An external dependency modeled as an ordinary internal link gets ordinary management, which is exactly what it does not need.

### Build The Network Before Assigning Dates

Sequence before dating. Map the full dependency network, all types, before attaching any calendar dates. Dating first forces artificial durations onto relationships that have not been understood, and produces a schedule that looks committed but is not executable. Once the network is mapped, the critical path, the slack, the constraining resources, and the external exposure become visible, and dates can be assigned with honesty about what the network actually permits.

### Identify And Resolve Circular Dependencies

A circular dependency, A depends on B depends on C depends on A, is a deadlock that will halt execution. These arise easily in complex networks, especially when information or decision dependencies are added, and they are often invisible in a tool that does not check for them. Run a cycle check on the network, identify any loops, and break them by restructuring the work: splitting a task, introducing an intermediate deliverable, or changing the sequence. A network with an unresolved cycle is not a plan; it is a trap waiting to spring.

### Distinguish Hard From Soft Dependencies

Not every dependency is absolute. A hard dependency is physical or logical: you cannot test code that has not been written. A soft dependency is preferential: it is better to do A before B, but they could overlap at some cost or risk. Distinguishing them matters because soft dependencies offer schedule flexibility that hard ones do not. Label dependencies by hardness so that when compression is needed, the soft links can be challenged and the hard ones respected. Treating all dependencies as hard produces an unnecessarily rigid plan; treating all as soft produces an unexecutable one.

### Map Dependencies At The Level That Reveals Risk

Too fine a dependency map drowns the model in links no one maintains; too coarse a map hides the constraints that matter. Map at the level where dependencies carry real schedule risk, typically the work package or deliverable level, and aggregate upward for reporting. The map must be detailed enough to expose the critical chain and the external exposure, but not so detailed that maintaining it costs more than it saves. Calibrate granularity to where decisions get made.

## Common Traps

### Defaulting Everything To Finish-To-Start

Letting the tool's default turn every relationship into strict sequence over-constrains the schedule and hides possible overlap. The trap is a plan that looks safe but is longer and more rigid than necessary. Model the true logical type.

### Modeling Resource Conflicts As Logical Links

Treating a shared-specialist constraint as an FS link hides the real cause and blocks the right solution, which is capacity management. The trap is a schedule that fails for reasons the model does not show. Represent resource dependencies separately.

### Omitting Decision And Information Dependencies

Skipping dependencies that are not tasks hides the constraints that most often cause delay. The trap is a complete-looking network that nonetheless deadlocks on an unmade decision. Map decisions and information explicitly.

### Treating External Dependencies As Ordinary Internal Links

Giving external dependencies the same handling as internal ones under-manages the highest-risk class. The trap is that the external party's slip becomes a surprise because it was not tracked with heightened scrutiny. Classify and tag them distinctly.

### Dating Before Mapping The Network

Assigning dates before understanding sequence produces a committed-looking schedule that is not executable. The trap is false precision built on an unmapped network. Sequence first, date second.

### Unresolved Circular Dependencies

A cycle in the network is an execution deadlock. The trap is that it is invisible until execution halts, then recovery is expensive. Run cycle checks and break loops during planning.

### Treating All Dependencies As Hard

Labeling preferential relationships as absolute removes schedule flexibility. The trap is an unnecessarily rigid plan that cannot be compressed when needed. Distinguish hard from soft.

### Over-Detailed Or Under-Detailed Dependency Maps

A map too fine to maintain goes stale; one too coarse hides real risk. The trap is a model that either wastes effort or misses the constraints that matter. Calibrate granularity to decision-making level.

## Self-Check

- [ ] Is each dependency classified by its true logical type (FS, SS, FF, SF) rather than defaulted to finish-to-start?
- [ ] Are resource dependencies modeled separately from logical links, with constraining resources identified?
- [ ] Are decision and information dependencies mapped explicitly, with owners assigned to resolve them?
- [ ] Are external dependencies tagged as a distinct high-risk class with identified external owners and interface points?
- [ ] Was the full dependency network mapped before any calendar dates were assigned?
- [ ] Has the network been checked for circular dependencies, with any cycles broken during planning?
- [ ] Are dependencies labeled as hard or soft so compression can challenge the right links?
- [ ] Is the dependency map maintained at a granularity that reveals risk without becoming unmaintainable?
- [ ] Can you identify, from the map, the critical chain, the constraining resources, and the external exposure?
- [ ] Has the model avoided treating all relationships as strict sequence where overlap or flexibility actually exists?
