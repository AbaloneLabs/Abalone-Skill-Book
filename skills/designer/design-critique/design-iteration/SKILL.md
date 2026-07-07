---
name: design_iteration.md
description: Use when the agent is iterating on a design, planning revision rounds, deciding what to change between versions, managing design debt, choosing what to refine versus explore versus freeze, or determining when a design is done, and must decide iteration scope, convergence versus divergence, when to stop exploring, and how to avoid both premature convergence on a weak solution and endless iteration that never ships.
---

# Design Iteration

Iteration is the engine of design quality, but it is also the engine of design paralysis. The same act of revising the work can either converge a weak idea into a strong solution or spin forever, polishing details while the core remains untested. The judgment problem is not whether to iterate, which is given, but what kind of iteration to do at each point: when to diverge and explore alternatives, when to converge and refine a chosen direction, what to freeze so the team can move forward, and when to declare the design done. Agents tend to fail by converging too early on the first plausible solution, by iterating on details while the underlying structure is still wrong, by treating every round as refinement when the work actually needs exploration, and by never declaring done, so the design drifts and the ship date slips.

Use this skill when planning or performing design iterations, deciding what to change between versions, managing rounds of revision, or determining when a design is ready. The goal is iteration that improves the core before the detail, balances exploration with convergence, and reaches a defensible definition of done.

## Core Rules

### Match The Iteration Mode To The Stage: Diverge Before You Converge

Iteration has two fundamentally different modes. Divergent iteration generates alternatives: it widens the space, tests different approaches, and resists committing to one direction. Convergent iteration refines a chosen direction: it deepens detail, resolves edge cases, and polishes. The failure is applying the wrong mode. Diverging when the team needs to ship produces endless options; converging when the core is still uncertain locks in a weak solution and refines it beautifully.

Use divergent iteration when:

- the core problem or approach is still unclear;
- the first solution may not be the best;
- the team has time and the stakes of a wrong choice are high.

Use convergent iteration when:

- a direction is chosen and validated;
- the structure is sound and the work is about detail and robustness;
- the ship date requires commitment.

Recognize which mode the work needs at each round, and do not drift into polishing a direction that was never properly explored.

### Fix The Core Before The Detail

A pervasive failure is iterating on surface details while the underlying structure remains wrong. Adjusting spacing, color, and microcopy on a flow whose steps are in the wrong order produces a polished version of a broken design. Each iteration should first ask whether the core, the structure, hierarchy, flow, and primary interactions, is correct, and only then move to detail. Detail iteration on an unsound core is wasted effort that must be redone once the core changes.

Sequence iteration as:

- structure and flow first;
- hierarchy and primary interactions next;
- states, edge cases, and error handling;
- visual detail and polish last.

If a core change is still plausible, defer the polish, because polish creates attachment that makes core changes psychologically harder.

### Define What Done Means Before You Iterate

Without a definition of done, iteration has no stopping condition and drifts until external pressure, usually a deadline, forces a stop. Define done as concretely as possible before iterating: which user tasks must succeed, which states must be handled, which consistency constraints must hold, which open questions must be resolved. Done is not "it feels finished"; it is a checkable set of conditions. With done defined, each iteration has a target, and the team can recognize completion rather than oscillating.

Define done by stating:

- the user tasks that must be completable without blockers;
- the states that must be designed, including empty, loading, error, and edge;
- the consistency and platform constraints that must hold;
- the open questions that must be resolved or explicitly deferred.

Revisit the definition as you learn, but keep it explicit so iteration converges rather than wanders.

### Avoid Premature Convergence On The First Plausible Solution

The first solution that works feels inevitable, especially after effort invested in it. But the first solution is rarely the best, and converging on it skips the exploration that often produces a stronger answer. Before committing to a direction, generate at least one or two genuine alternatives, even rough ones, to confirm the chosen direction is better rather than merely first. Premature convergence is the most common reason designs feel adequate but never excellent.

Pressure-test convergence by asking:

- what one or two other approaches were considered and set aside;
- why the chosen direction is better, not just earlier;
- what would have to be true for an alternative to win;
- whether the alternatives were real or strawmen.

Real comparison, even quick, justifies convergence; skipping it leaves the team wondering if they settled.

### Avoid Endless Iteration That Never Ships

The opposite failure is never converging. Each round reveals new possibilities, new edge cases, new polish opportunities, and the design can always be improved. But a design that never ships helps no user. Recognize when iteration has shifted from improving the core to diminishing returns, where each round changes less and risks more. At that point, freeze the remaining items as known design debt and ship, rather than holding the entire release for marginal gains.

Signals to stop iterating:

- changes are now in detail, not structure;
- each round introduces as many new issues as it resolves;
- the core user tasks succeed and the defined done conditions are met;
- further delay costs more than the remaining improvements are worth.

Shipping with documented, prioritized debt is healthier than holding for perfection that never arrives.

### Manage Design Debt Explicitly

Iteration that defers issues must record them, or they become invisible and resurface as regressions or surprises. Maintain a design debt list: known compromises, deferred states, inconsistencies, and edge cases not yet handled. Each item should note the risk and the condition under which it should be addressed. Design debt is not a failure; untracked design debt is. Explicit debt lets the team ship knowingly and plan future rounds.

### Iterate Against Evidence, Not Opinion

Each iteration should be informed by something more reliable than the designer's or reviewer's hunch: usability findings, analytics, support data, prototype test results, or at minimum a clearly stated user consequence. Iterating purely on opinion produces oscillation, where each round reverses the last based on who spoke most recently. Anchor iterations to evidence so each round moves toward a better user outcome rather than reacting to the loudest feedback.

### Preserve What Is Working While Changing What Is Not

Iteration often focuses on what to change and forgets to protect what is already working. A round that fixes one problem can quietly break another, because the change was not checked against the whole. Before changing, note what must be preserved: consistency with other surfaces, existing successful flows, established patterns. After changing, verify the preserved properties still hold.

## Common Traps

### Premature Convergence

Committing to the first plausible solution skips exploration and often settles for adequate instead of strong. Generate and compare real alternatives before converging.

### Polishing A Broken Core

Iterating on detail while the structure is wrong wastes effort that must be redone. Fix the core before the surface.

### No Definition Of Done

Without explicit done conditions, iteration drifts until a deadline forces a stop. Define done in checkable terms before iterating.

### Endless Iteration

Always-improving designs that never ship help no one. Recognize diminishing returns, record debt, and ship.

### Wrong Mode For The Stage

Diverging when convergence is needed delays shipping; converging when exploration is needed locks in a weak solution. Match mode to stage.

### Iterating On Opinion Alone

Rounds driven by hunch rather than evidence oscillate without improving the user outcome. Anchor iterations to evidence.

### Untracked Design Debt

Deferred issues that are not recorded become invisible and resurface later. Maintain an explicit debt list with risks and revisit conditions.

### Breaking What Worked

A change that fixes one problem can break a working flow or consistency. Note what to preserve and verify it after each change.

## Self-Check

- [ ] The iteration mode, divergent or convergent, matches the stage and certainty of the work.
- [ ] Structure, flow, and hierarchy are addressed before visual detail and polish.
- [ ] A concrete, checkable definition of done was defined before iterating and is being tracked.
- [ ] At least one or two genuine alternatives were considered before converging on a direction.
- [ ] Diminishing returns were recognized, remaining items recorded as debt, and the design shipped rather than held for perfection.
- [ ] Design debt is tracked explicitly with risk and revisit conditions for each item.
- [ ] Iterations are anchored to evidence or clearly stated user consequences, not opinion alone.
- [ ] What is already working was identified and verified to still hold after each change.
- [ ] The team can explain why the current direction is better, not just earlier, than alternatives.
- [ ] Iteration improved the core user outcome rather than merely reacting to the most recent feedback.
