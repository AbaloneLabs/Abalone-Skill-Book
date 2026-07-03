---
name: backlog_management_and_prioritization.md
description: Use when the agent is managing a product backlog, prioritizing and grooming work items, writing user stories and acceptance criteria, managing story points and estimation, handling backlog refinement sessions, or deciding what work to deliver in which iteration.
---

# Backlog Management and Prioritization

The backlog is the central instrument of agile delivery. It is where strategy meets execution: every item in the backlog represents a claim on the team's finite capacity, and the order and quality of those items determines whether the team delivers value or merely produces output. A poorly managed backlog is the single most common cause of agile failure, because it silently redirects effort away from valuable work without any visible defect. The team is busy, velocity looks healthy, and the product falls behind.

The judgment problem is how to keep the backlog a living, prioritized, ready-to-work instrument rather than a dumping ground; how to prioritize on value and risk rather than on who shouts loudest; and how to size and refine items so they are actionable without over-specifying. Agents tend to let the backlog accumulate untriaged items, prioritize by recency or seniority, and conflate activity with value.

## Core Rules

### Treat the Backlog as a Living Instrument, Not an Archive

A healthy backlog is refined continuously: items are added, clarified, sized, prioritized, split, merged, and removed. An unrefined backlog decays into a graveyard of stale wishes that no one will ever deliver but that consume attention whenever the list is reviewed. Establish a regular refinement cadence and treat backlog health as a first-class responsibility, not a spare-time activity. Prune ruthlessly: items that will never be done should be closed, not left to haunt the list.

### Prioritize on Value, Risk, and Dependency, Not on Loudness

Prioritization should be driven by a defensible model: business value, urgency, risk reduction, learning value, and dependency sequencing. Techniques like weighted shortest job first, cost of delay, or simple value-versus-effort matrices make priorities explicit and debatable. Prioritizing by who asked most recently or most loudly produces a backlog that serves the loudest stakeholder, not the product. Make the prioritization criteria visible so they can be challenged.

### Maintain a Single Backlog With Clear Ordering

There should be one ordered backlog that reflects the current priorities, not multiple competing lists maintained by different stakeholders. A single ordered list forces trade-off decisions to be explicit: adding something means something else moves down. Multiple lists allow every stakeholder to believe their work is top priority, which is resolved only by the team being overcommitted. One backlog, one ordering, owned by one product owner.

### Ensure Items Are Ready Before They Enter an Iteration

Work taken into an iteration should be "ready": clear, sized, with acceptance criteria, dependencies resolved, and the team confident it can be completed. Taking unready work into an iteration guarantees mid-iteration stalls, scope ambiguity, and incomplete work that corrupts velocity. A definition of ready makes the readiness test explicit. Refine items to ready before committing them.

### Write Items From the User's Perspective With Testable Acceptance Criteria

Backlog items should describe the value to the user or the outcome desired, not the implementation task. User stories, job stories, or feature descriptions frame the work in terms of who benefits and what they can then do. Each item needs acceptance criteria that are testable, so that "done" is unambiguous. Implementation-detail items produce work that is technically complete but value-opaque.

### Size Items to Enable Forecasting Without False Precision

Sizing, whether in story points or time, enables forecasting and capacity planning. The goal is relative consistency, not precision. Sizing should reflect effort, complexity, and uncertainty, and the team should converge on shared understanding through estimation practices like planning poker. False precision, sizing to the hour and treating it as a commitment, produces blame and gaming. Size to enable reasonable forecasting and acknowledge the uncertainty.

### Split Large Items Rather Than Carrying Them

Large items that span multiple iterations are hard to track, hard to estimate, and prone to incomplete delivery. Split them into smaller items that can be completed within an iteration and that each deliver value or a verifiable increment. A backlog full of large unsplit items produces lumpy delivery and unreliable velocity. Split aggressively; if an item cannot be split, that is a signal it is poorly understood.

### Manage Dependencies Explicitly in the Ordering

Some items must precede others: a foundation before the feature, an API before the consumer, a decision before the implementation. These dependencies must be reflected in the backlog ordering, or the team will attempt work that cannot complete and stall. Surface dependencies during refinement and sequence accordingly. Hidden dependencies are a primary cause of iteration failure.

## Common Traps

### The Backlog as a Dumping Ground

Every stakeholder adds items; no one removes or refines them. The backlog grows to hundreds of items, most stale, and refinement becomes hopeless. The trap is that adding feels productive and pruning feels like loss. Prune ruthlessly.

### Prioritization by Loudness or Seniority

The stakeholder who shouts loudest or has the most seniority gets their work first, regardless of value. The trap is that the backlog serves politics, not the product. Use explicit value-based prioritization.

### Multiple Competing Backlogs

Each stakeholder maintains their own list, all "top priority," and the team is overcommitted. The trap is that multiple lists hide the real trade-offs. Maintain one ordered backlog.

### Taking Unready Work Into Iterations

Items enter the iteration unclear, unsized, or with unresolved dependencies, and stall mid-iteration. The trap is the pressure to look busy overrides the discipline of readiness. Hold the definition of ready.

### Implementation-Detail Items

Items describe technical tasks rather than user value, so the team completes work whose value is unclear. The trap is that tasks are easier to write than value statements. Write from the user's perspective.

### False Precision in Sizing

Sizes are treated as hour-level commitments, so estimates become blame and gaming. The trap is mistaking precision for accuracy. Size for forecasting, acknowledge uncertainty.

### Large Items Carried Across Iterations

Big items never complete within an iteration, so velocity is unreliable and delivery is lumpy. The trap is that splitting feels like extra work. Split aggressively.

### Hidden Dependencies

Dependencies are not surfaced during refinement, so the team attempts work it cannot complete. The trap is that dependencies are discovered too late. Surface and sequence dependencies explicitly.

## Self-Check

- [ ] Is the backlog refined on a regular cadence, with stale items pruned rather than left to accumulate?
- [ ] Is prioritization driven by a visible value/risk/dependency model rather than by stakeholder loudness or seniority?
- [ ] Is there a single ordered backlog owned by one product owner, rather than multiple competing lists?
- [ ] Do items meet a definition of ready (clear, sized, acceptance criteria, dependencies resolved) before entering an iteration?
- [ ] Are items written from the user's perspective with testable acceptance criteria, not as implementation tasks?
- [ ] Is sizing used for forecasting with acknowledged uncertainty, not as hour-level commitment?
- [ ] Are large items split into iteration-completable pieces that each deliver value or a verifiable increment?
- [ ] Are dependencies surfaced during refinement and reflected in the backlog ordering?
- [ ] Can you defend the current top of the backlog on value and sequencing grounds, with explicit criteria?
- [ ] Would removing the bottom half of the backlog change what the team delivers in the next few iterations, or is it dead weight?
