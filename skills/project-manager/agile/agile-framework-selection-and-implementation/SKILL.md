---
name: agile_framework_selection_and_implementation.md
description: Use when the agent is selecting an agile framework, implementing Scrum Kanban or SAFe, deciding between agile and predictive approaches, tailoring agile practices to a project context, or transitioning a team or organization to agile delivery methods.
---

# Agile Framework Selection and Implementation

Choosing an agile framework is presented as a lightweight decision, but it is a structural commitment that shapes how the team organizes, how work flows, how decisions are made, and how value is measured for the life of the engagement. The damage from a poor choice does not appear immediately; it appears as chronic friction, ceremonies that feel like waste, metrics that mislead, and a team that has adopted the vocabulary of agility without its discipline. The most common failure is not choosing the wrong framework, but choosing a framework for the wrong reasons and then mechanically imposing practices whose purpose no one understands.

The judgment problem is how to match a delivery approach to the actual problem characteristics, how to implement practices so their intent is preserved rather than hollowed out, and how to avoid the cargo-cult adoption that produces agile in form and waterfall in substance. Agents tend to default to whichever framework is fashionable, apply it uniformly regardless of context, and treat the ceremonies as the outcome rather than as means.

## Core Rules

### Match the Approach to Problem Characteristics, Not to Fashion

The choice between predictive, iterative, incremental, and agile approaches should be driven by the problem: how well-understood the requirements are, how stable the technology is, how much feedback is available during development, and how tolerant the stakeholders are of evolving scope. Well-understood, stable, low-feedback work suits predictive approaches; uncertain, evolving, high-feedback work suits agile. Choosing agile because it is current, for work that is actually well-defined and stable, imposes overhead without benefit. State the problem characteristics and derive the approach from them.

### Distinguish Frameworks by Their Core Mechanism

Scrum organizes around time-boxed iterations with committed cross-functional teams and prescribed ceremonies. Kanban organizes around continuous flow with work-in-progress limits and pull. SAFe scales agile across many teams with alignment and cadence structures. Each mechanism solves a different problem: Scrum for stable teams doing complex iterative work, Kanban for flow-based work with variable arrival, SAFe for large multi-team coordination. Selecting a framework without understanding its mechanism produces mismatched practice. Know what each framework is for before choosing.

### Preserve the Intent of Practices, Not Just Their Form

Every agile practice exists to solve a specific problem: the daily standup surfaces blockers, the retrospective improves the process, the demo creates feedback, work-in-progress limits expose bottlenecks. When practices are adopted without their intent, they become empty rituals that consume time without producing their benefit. For each practice you adopt, be able to state the problem it solves and the signal it is meant to produce. If a practice produces no signal, drop it or fix it.

### Implement the Whole System, Not Cherry-Picked Pieces

Agile frameworks are systems of mutually reinforcing practices. Adopting standups and demos without empowering the team, without a backlog that reflects real priorities, and without stakeholders who engage with iterations, produces the form without the function. The ceremonies become theater. Either commit to the system the framework prescribes, or deliberately tailor it and own the consequences of the tailoring. Do not silently drop the parts that are hard.

### Scale Deliberately, Not by Default

Single-team agile is hard enough. Scaling to multiple teams, whether through SAFe, LeSS, or a custom structure, adds coordination overhead, dependency management, and alignment complexity that many organizations underestimate. Scaling is justified only when the work genuinely requires multiple teams and the organization is prepared to invest in the coordination structures. Scaling a single team's worth of work across many teams creates coordination cost without throughput benefit. Scale because the work demands it, not because the org chart suggests it.

### Align Agile Roles With Real Authority

Agile frameworks define roles, product owner, scrum master, team, that carry specific authorities. The product owner prioritizes the backlog; the team determines how to do the work; the scrum master removes impediments. When these authorities are not genuinely granted, when the product owner has no real prioritization power or the team has no real say in how they work, the framework cannot function. Map framework roles to actual people with actual authority, and resolve the gaps before implementation.

### Define Done and Ready Explicitly

A shared, explicit definition of done prevents the ambiguity where work is "complete" to one person and "in progress" to another. A definition of ready ensures that work taken into an iteration is actually actionable. Without these, velocity becomes meaningless because "done" work varies in quality and completeness, and iterations stall on work that was never ready. Negotiate and document these definitions with the team and hold them.

### Measure Outcomes and Flow, Not Just Velocity

Velocity measures how much work the team completes, but it is easily gamed and says nothing about whether the right work is being done or whether value is being delivered. Complement velocity with flow metrics (cycle time, throughput, lead time), outcome measures (features delivered that users value, defects escaped), and health signals (team sustainability, technical debt trend). A team optimizing only velocity will inflate estimates and ship low-quality work. Measure what you actually want to improve.

## Common Traps

### Agile Because It Is Fashionable

The team adopts agile because it is the current default, for work that is actually well-defined and stable, and pays the overhead without the benefit. The trap is following fashion rather than problem fit. Derive the approach from the problem.

### Cargo-Cult Ceremonies

The team holds standups, demos, and retrospectives because the framework says to, but the standups are status reports, the demos are skipped, and the retrospectives produce no change. The trap is form without intent. Connect each practice to the problem it solves.

### Cherry-Picking the Easy Parts

The team adopts the ceremonies but not the empowerment, the backlogs but not the prioritization, the iterations but not the stakeholder engagement. The trap is that the visible practices are adopted and the hard cultural changes are avoided. Commit to the system or tailor deliberately.

### Scaling Before the Basics Work

The organization scales to SAFe or multi-team structures before a single team can reliably deliver. The trap is that scaling amplifies existing dysfunctions. Master single-team delivery first.

### Roles Without Authority

A product owner is named who cannot actually prioritize, or a team that cannot actually decide how to work. The trap is that the framework's mechanics depend on authorities that do not exist. Grant real authority to the roles.

### No Definition of Done

"Done" means different things to different people, so velocity is meaningless and quality varies. The trap is ambiguity disguised as flexibility. Define done explicitly and hold it.

### Optimizing Velocity at the Expense of Value and Quality

The team inflates estimates and ships thin work to keep velocity rising, while value delivered and quality decline. The trap is measuring activity instead of outcome. Measure flow, outcomes, and health, not just velocity.

## Self-Check

- [ ] Was the delivery approach selected based on problem characteristics (requirements clarity, technology stability, feedback availability, scope tolerance) rather than fashion?
- [ ] Can you state the core mechanism of the chosen framework and why it fits this problem?
- [ ] For each adopted practice, can you state the problem it solves and the signal it is meant to produce?
- [ ] Has the framework been implemented as a system, with the hard parts (empowerment, prioritization, stakeholder engagement) included rather than dropped?
- [ ] If scaling, is the scaling justified by genuine multi-team work requirements and backed by coordination investment, rather than by org-chart default?
- [ ] Do the agile roles carry real authority, with gaps between framework roles and actual authority resolved before implementation?
- [ ] Are there explicit, team-negotiated definitions of done and ready, consistently applied?
- [ ] Are flow metrics (cycle time, throughput, lead time), outcome measures, and health signals tracked alongside velocity, not velocity alone?
- [ ] Are ceremonies producing their intended signals, or have any become empty rituals that should be fixed or dropped?
- [ ] Is the team delivering value at a sustainable pace, rather than inflating metrics or accumulating technical debt?
