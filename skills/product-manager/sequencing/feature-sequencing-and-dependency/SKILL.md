---
name: feature_sequencing_and_dependency.md
description: Use when the agent is sequencing features or initiatives, mapping dependencies between work items, deciding what to build first to unblock or validate, or ordering delivery so that value compounds and the team is never blocked by unfinished prerequisites.
---

# Feature Sequencing And Dependency

Sequencing is the decision about what to build first, what depends on what, and how to order work so that the team is never blocked, value compounds, and early delivery validates direction. It is not the same as prioritization, which decides what is worth doing; sequencing decides the order in which worth-doing work should be executed given dependencies, risk, and learning. Done well, sequencing produces a flow where each deliverable enables or validates the next, and where the team always has valuable work available. Done poorly, it produces teams blocked by missing prerequisites, value back-loaded so late it never arrives, or foundations built speculatively that never get used. Agents often sequence by static priority value, ignoring that dependencies and learning change the optimal order.

The harm this skill prevents is the team that is always blocked or always building the wrong thing first. A high-value feature that depends on unfinished platform work cannot be delivered; speculative foundation work that precedes validated demand wastes capacity; value back-loaded behind months of prerequisites arrives too late to inform anything. Good sequencing is what turns a prioritized list into executable flow.

Use this skill before answering questions such as "what should we build first", "how do we sequence these features", "what are our dependencies", or "why is the team always blocked". The goal is to prevent the agent from sequencing on value alone without considering dependencies, risk, and learning.

## Core Rules

### Map Dependencies Before Sequencing

You cannot sequence intelligently without knowing what depends on what. A feature that looks high-value may be blocked by unfinished foundational work; two features may contend for the same scarce resource; one initiative may unblock several others. Map the dependency graph before sequencing, so that the order respects the real structure of the work rather than an idealized view. Dependencies include technical prerequisites, data dependencies, design dependencies, integration dependencies, and organizational dependencies on other teams.

Distinguish hard dependencies, where work genuinely cannot proceed without the prerequisite, from soft dependencies, where proceeding is possible but costly or risky. Hard dependencies constrain the sequence; soft dependencies inform it but allow parallel work. Conflating the two either over-constrains the plan or ignores real blockers.

### Sequence To Unblock And Enable, Not Only To Deliver Value

The highest-leverage sequencing decisions often favor work that is not the highest static-value item but that unblocks or enables many other items. A platform capability that five features depend on may be worth building early even if its standalone value is modest, because it unlocks the flow of all five. Conversely, a high-value feature that depends on nothing may be safely deferred if other work creates more optionality. Consider enablement value alongside delivery value when ordering work.

This is especially true for shared infrastructure and foundational capabilities. Building them early, when they unblock multiple streams, accelerates everything that follows. Deferring them creates a bottleneck where many teams wait on one piece of unfinished foundation. The sequencing of foundations often determines the throughput of the entire organization.

### Sequence To Validate Assumptions Early

Many initiatives rest on assumptions that, if wrong, invalidate the work. Sequencing should front-load the work that tests the riskiest assumptions, so that the team learns whether the direction is sound before committing to the dependent work. Delivering a small slice that validates the core hypothesis, then expanding if it holds, is faster and safer than building the full feature on an untested assumption and discovering the assumption was wrong at the end.

Identify the riskiest assumption behind each initiative and ask what smallest piece of work would test it. Sequence that piece early, even if it is not the highest-value deliverable, because it reduces uncertainty for everything that follows. This is the core of learning-driven sequencing: order work to maximize the rate at which risky assumptions are resolved.

### Avoid Speculative Foundation Built Before Validated Demand

The mirror image of validating early is the trap of building foundational capabilities speculatively, before any validated demand for them exists. A platform built for future features that may never materialize, an abstraction generalized before multiple concrete cases exist, an infrastructure scaled for usage that has not arrived: all are speculative foundation that consumes capacity now for uncertain future benefit. Sequence foundations to follow validated demand, not to precede it.

The exception is when a foundation is a hard prerequisite for any validation at all. If no learning is possible without the foundation, build the minimum foundation needed to validate, and no more. Generalize and scale only after demand is proven. This discipline prevents the common pattern of over-built foundations that delay value delivery without accelerating it.

### Balance Parallel And Sequential Work Against Capacity

Sequencing is not only about order; it is about what runs in parallel and what runs in sequence, given finite capacity. Too much parallel work overloads the team and delivers nothing; too much sequential work underuses capacity and slows delivery. The right balance keeps the team fully utilized on valuable work without spreading effort so thin that nothing completes. Sequence work that must be serial, parallelize work that is independent, and manage the contention for shared resources.

Pay attention to bottlenecks, the scarce resources that limit throughput. Sequencing around a bottleneck, so that it is never idle and never overloaded, often improves overall throughput more than optimizing any individual stream. The constraint of the system determines the optimal sequencing, not the value of individual items.

### Re-Sequence When Learning Changes The Picture

Sequencing is not set at the start; it evolves as the team learns. An assumption that validated early may open or close downstream options; a dependency may resolve earlier or later than expected; a new initiative may enter with higher priority. Build in regular re-sequencing based on what has been learned, so that the order continues to reflect the current best understanding rather than a plan made under earlier uncertainty.

The cadence of re-sequencing should match the rate of learning. A team learning fast may re-sequence every few weeks; a team in steady delivery may re-sequence quarterly. The trigger is that the assumptions underlying the current sequence have changed enough to warrant a different order.

## Common Traps

### Sequencing By Static Value Alone

Ordering by priority score without considering dependencies. The trap is high-value work that cannot proceed because prerequisites are unfinished.

### Ignoring Enablement Value

Deferring foundational work that unblocks many streams. The trap is many teams blocked on one piece of unfinished foundation.

### Back-Loading All Validation

Building full features before testing assumptions. The trap is discovering wrong assumptions only after full investment.

### Speculative Foundation Before Demand

Over-building platforms and abstractions for future use. The trap is capacity consumed now for benefit that never materializes.

### Too Much Parallel Work

Spreading effort across many initiatives so nothing completes. The trap is many things in progress and nothing delivered.

### Static Sequencing Under Changing Learning

Refusing to re-sequence when assumptions change. The trap is an order that no longer reflects the best current understanding.

## Self-Check

- [ ] Dependencies, hard and soft, are mapped before sequencing decisions are made.
- [ ] Enablement value, work that unblocks other work, is weighed alongside delivery value in ordering.
- [ ] The riskiest assumptions behind each initiative are identified, and work that tests them is sequenced early.
- [ ] Foundations follow validated demand, with only the minimum built ahead to enable validation.
- [ ] Parallel and sequential work are balanced against capacity and managed around bottlenecks.
- [ ] Sequencing is revisited on a cadence tied to the rate of learning.
- [ ] The team is never blocked by missing prerequisites that could have been sequenced earlier.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
