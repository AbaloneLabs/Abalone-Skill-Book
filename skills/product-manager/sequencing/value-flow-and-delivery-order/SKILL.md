---
name: value_flow_and_delivery_order.md
description: Use when the agent is deciding the order in which to deliver value to users, sequencing for cumulative customer benefit, choosing between thin vertical slices and horizontal layers, or arranging delivery so each release creates standalone value rather than requiring everything to finish.
---

# Value Flow And Delivery Order

Value flow is the question of how to arrange delivery so that value reaches users continuously throughout a program, rather than arriving all at once at the end. The central principle is the vertical slice: deliver a thin end-to-end piece of value that a user can actually experience, rather than a horizontal layer that is necessary but useless on its own. Done well, value flow means every release makes the product better for users, the team gets feedback continuously, and large programs produce incremental value rather than a single risky big-bang. Done poorly, the team spends months building foundations, platforms, and infrastructure, delivers nothing usable, and then attempts a risky integration at the end. Agents often sequence by architectural layers because that is how engineering thinks, producing horizontal delivery that creates no user value until everything is done.

The harm this skill prevents is the program that delivers nothing until the very end. Horizontal, layer-by-layer delivery feels efficient to engineering but produces zero user value for months, accumulates risk, and provides no feedback to correct course. When the integration finally happens, problems surface late and expensively, and the team has built on assumptions it never tested.

Use this skill before answering questions such as "how should we order delivery", "should we build the backend or frontend first", "how do we deliver value incrementally", or "what is a vertical slice". The goal is to prevent the agent from sequencing on architectural convenience rather than on continuous value delivery.

## Core Rules

### Deliver Vertical Slices That Create Standalone User Value

A vertical slice is a thin end-to-end piece of functionality that a user can experience and that delivers value on its own. It cuts through all the layers, from interface through logic to data, but it is narrow in scope. The alternative, a horizontal layer, completes one layer fully, the entire backend, or the entire data model, before any of it is usable. Vertical slices deliver value continuously; horizontal layers deliver value only when all layers are complete. Default to vertical slices for any work where user value is the goal.

The test of a vertical slice is whether a user could benefit from it if delivery stopped today. If the answer is no, because the slice depends on future work to be usable, it is not yet vertical. Reframe the work so that each delivered increment stands alone, even if its scope is small. A small standalone increment is more valuable than a large incomplete one.

### Sequence So Each Release Builds On The Last's Value

The order of vertical slices should be chosen so that each release compounds the value of the previous one, building toward the full vision through a sequence of valuable states. The first slice delivers the minimum valuable experience; each subsequent slice expands or deepens it. This creates a path where the product is continuously valuable, and where the team can stop at any point having delivered something real. Sequencing for compounding value also means that the most important capabilities come early, so that even a curtailed program delivers its core value.

Avoid sequencing that front-loads peripheral capabilities and back-loads the core. If the most important user value arrives only in the final release, the program has failed to deliver value flow, regardless of how many intermediate releases shipped. The core value should be reachable early and deepened over time.

### Use Horizontal Layers Only When Vertically Impossible

There are cases where vertical slicing is genuinely difficult or impossible: deep platform rebuilds, data migrations, infrastructure that has no user-facing surface, regulatory changes that must be complete to take effect. In these cases, horizontal work is legitimate, but it should be minimized and its risks acknowledged. The longer the program goes without delivering user value, the higher the risk that it builds the wrong thing or integrates poorly.

When horizontal work is unavoidable, find ways to inject vertical validation even within it. Can a small subset of users be migrated early to test the path? Can a feature flag expose the new platform to a slice of traffic? Can a read-only view of the new data model validate correctness before full cutover? These injections reduce the risk of pure horizontal delivery without requiring full vertical slices.

### Make Each Release Independently Deployable And Observable

A vertical slice is only valuable if it can actually reach users and if its effect can be observed. Sequence delivery so that each release is independently deployable, not dependent on future work to ship, and so that its impact on users and metrics is measurable. A slice that is built but cannot be deployed, or whose effect cannot be observed, provides neither value nor feedback. Deployability and observability are requirements of the slice, not afterthoughts.

This often means that instrumentation, feature flagging, and deployment infrastructure are part of the slice's definition of done, not separate work. Building the feature without the ability to ship or measure it delays both value and learning. Treat deployability and observability as first-class concerns of every increment.

### Avoid The Big-Bang Integration At The End

The most dangerous delivery pattern is the big-bang, where many streams of work proceed independently and integrate only at the end. The integration is where incompatibilities, missing contracts, and broken assumptions surface, all at once, under time pressure. Sequencing for value flow inherently avoids big-bang, because vertical slices integrate continuously as they are delivered. When multiple streams must run in parallel, build integration points throughout, not only at the end.

The cost of late integration is not only the integration work itself but the rework it triggers. Problems found at final integration may require changes to work that was believed complete, sometimes far upstream. Continuous integration through vertical slices finds these problems when they are cheap to fix, not when they are expensive.

### Let Feedback From Early Slices Reshape Later Ones

The purpose of delivering value continuously is not only to ship value but to learn from each release. Each vertical slice that reaches users generates feedback about whether the direction is right, what users actually do, and what should change. This feedback should reshape the subsequent slices, so that the program adapts based on reality rather than executing a fixed plan. A program that delivers vertical slices but ignores the feedback has gained deployability without adaptability.

Build in a review after each slice's release, where the feedback is assessed and the remaining sequence is updated. Some slices may be expanded based on positive reception; others may be cut because the feedback showed they were not valuable; new slices may emerge from insights the team did not have at the start. The sequence is a living plan, refined by each release.

## Common Traps

### Horizontal Delivery By Architectural Layer

Building backend, then frontend, then integration. The trap is zero user value for the entire build period and a risky final integration.

### Big-Bang Integration

Multiple streams integrating only at the end. The trap is all problems surfacing simultaneously, under pressure, when they are most expensive to fix.

### Slices That Cannot Deploy Or Be Measured

Features built without deployability or instrumentation. The trap is value and feedback both delayed, defeating the purpose of incremental delivery.

### Core Value Back-Loaded

Peripheral capabilities early, essential value last. The trap is a program that could be cancelled before delivering what mattered most.

### Ignoring Feedback From Released Slices

Delivering continuously but executing a fixed plan regardless. The trap is adaptability forfeited, with the team learning nothing from its own releases.

### Pretending Horizontal Work Is Vertical

Labeling layer work as slices when no user value is delivered. The trap is the appearance of value flow masking the risks of horizontal delivery.

## Self-Check

- [ ] Delivery is organized into vertical slices that create standalone user value, not horizontal layers.
- [ ] Each release compounds the value of the previous, with core capabilities reachable early.
- [ ] Horizontal work is used only when vertical slicing is genuinely impossible, and its risks are acknowledged.
- [ ] Each slice is independently deployable and its impact is observable through instrumentation.
- [ ] Integration happens continuously through slice delivery, not as a big-bang at the end.
- [ ] Feedback from each released slice is reviewed and reshapes the remaining sequence.
- [ ] The program delivers continuous value and could be stopped at any point having produced something real.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
