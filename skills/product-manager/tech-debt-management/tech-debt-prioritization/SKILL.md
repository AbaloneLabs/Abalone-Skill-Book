---
name: tech_debt_prioritization.md
description: Use when the agent is prioritizing tech debt items against each other and against feature work, sequencing debt paydown, deciding which debt to address in a given cycle, or building a debt roadmap that balances sustained velocity with product delivery.
---

# Tech Debt Prioritization

Once tech debt is triaged and its case can be made, the remaining challenge is prioritization: deciding which debt to address now, which to schedule, and which to leave, in a way that sustains velocity without starving the product of new value. This is where most debt programs fail in practice. Teams produce a long list of debt items, all categorized and impact-assessed, and then cannot decide what to actually do, because every item has a plausible case and the list exceeds available capacity. The result is either paralysis, where nothing gets addressed because everything seems important, or random selection, where whatever an engineer feels strongly about gets done while higher-impact debt waits.

Tech debt prioritization is a sequencing problem under constraint, and it shares logic with feature prioritization but with important differences. Debt work rarely delivers visible user value, so it competes for capacity against features that do, and it must be justified by the velocity, risk, and capability benefits it provides. The product manager's job is to apply disciplined prioritization to debt the same way it is applied to features, so that the highest-impact debt is addressed first and the overall debt trend moves in the right direction.

Use this skill before sequencing debt items, before allocating a cycle's capacity between debt and features, before building or updating a debt roadmap, or when a debt backlog has grown unmanageable. Ask: which debt items deliver the most benefit per unit of effort? Which are blocking specific planned work? Which can be deferred without growing risk? Is the overall debt trend improving, stable, or worsening, and does the prioritization reflect that?

## Core Rules

### Prioritize By Impact Divided By Effort

The core prioritization logic for debt is the same as for features: prioritize by impact relative to effort. High-impact, low-effort debt paydown should go first, because it removes significant harm cheaply. High-impact, high-effort debt should be scheduled deliberately, because it is worth doing but consumes capacity. Low-impact debt, regardless of effort, should generally wait or be handled opportunistically. This is obvious in principle and consistently violated in practice, because impact is harder to measure for debt than for features.

Assess each debt item's impact in concrete terms: how much velocity drag it removes, how much risk it mitigates, what it unblocks, and how much morale it restores. Assess effort honestly, including not just the fix but the testing, migration, and stabilization around it. Then sequence by the ratio. Resist the tendency to prioritize by urgency of complaint or by technical interest, both of which push lower-impact items ahead of higher-impact ones. The goal is the most benefit for the capacity spent.

### Prioritize Debt That Blocks Planned Work

Debt that stands in the way of planned feature work has a special priority, because addressing it is part of delivering the feature, not a competitor to it. If a planned feature will be slower, riskier, or impossible because of specific debt, paying that debt first is a prerequisite investment that makes the feature feasible. This debt should be prioritized ahead of debt that, while real, does not block anything specific.

Map debt items to planned and anticipated feature work. For each, ask whether the debt will impede the feature, and if so, how. Debt that blocks high-priority features becomes high-priority debt, sequenced just ahead of or alongside the feature. This alignment resolves the apparent conflict between debt and features, because the debt work is recognized as enabling the feature. It also ensures that debt paydown is directed where it has product payoff, not just code payoff.

### Balance Quick Wins Against Structural Investment

A debt program built only of quick wins, small fixes that remove localized harm, delivers visible progress but may leave structural debt untouched, allowing large risks to grow. A program built only of structural investment, large refactors or migrations, addresses root causes but consumes capacity for long stretches with little incremental visible progress, which erodes stakeholder support. A healthy program balances both.

Sequence a mix. Include quick wins that demonstrate progress and build momentum, and include structural investments that address the debt causing the most systemic harm. The balance depends on the debt profile: a team with accumulating structural debt needs more structural investment; a team with manageable structure but many localized issues benefits from quick wins. Avoid the extremes of only quick wins, which leaves big risks growing, and only structural work, which starves the product of features for too long.

### Sequence To Manage Dependencies

Debt items often depend on each other. Refactoring one component may be a prerequisite for safely refactoring another. A migration may need to happen before related debt can be addressed. Some fixes are cheaper if done together, because they touch the same code. Ignoring these dependencies leads to inefficient sequencing, where work is done in an order that requires rework or misses opportunities for combined effort.

Map dependencies among debt items and sequence accordingly. Identify prerequisites, where one fix enables or simplifies another. Identify bundling opportunities, where fixes in the same area should be done together to avoid loading context twice. Identify sequencing constraints, where a fix must wait for a migration or a feature to land. Dependency-aware sequencing extracts more value from the same capacity and avoids rework.

### Allocate Capacity Deliberately, Not By Leftover

The most common prioritization failure is treating debt as leftover work, addressed only when features finish early, which means it is almost never addressed because features rarely finish early. Debt must have dedicated capacity, allocated deliberately as part of the roadmap, not scavenged from spare time. Without dedicated allocation, debt loses to features every time, because features have visible value and advocates while debt has invisible value and fewer champions.

Agree on a debt capacity allocation and protect it. This might be a percentage of each cycle, a dedicated debt cycle periodically, or a set of debt items committed alongside features. The allocation should be visible in the roadmap and defended against feature pressure, because it represents an investment in sustained velocity that pays off over time. Adjust the allocation based on debt trend: higher when debt is growing and blocking, lower when debt is manageable.

### Track Debt Trend As A Health Metric

Prioritization should be guided by the overall debt trend, not just by individual items. If debt is growing, the prioritization is under-investing in paydown and the allocation should rise. If debt is stable or shrinking, the allocation may be sufficient or could be reduced to favor features. Without tracking trend, the team cannot tell whether its debt program is working, and prioritization becomes disconnected from the actual health of the codebase.

Establish a lightweight debt health metric. This could be the count or estimated effort of blocking debt items, the trend in velocity in debt-heavy areas, the incident rate attributable to debt, or a periodic engineering assessment. Track it over time and use it to adjust prioritization and allocation. A healthy debt program shows a stable or improving trend; a worsening trend signals that prioritization needs to change, not that the metric should be ignored.

### Revisit Prioritization As Context Changes

Debt prioritization is not a one-time exercise. The highest-priority debt changes as features are planned, as incidents reveal new risks, as the codebase evolves, and as the team learns more about the harm specific debt causes. A debt roadmap set once and followed rigidly will misallocate capacity as context shifts. Prioritization must be revisited regularly, the way feature prioritization is.

Schedule regular debt prioritization reviews, involving both product and engineering. Reassess impact as new information arrives, add newly identified debt, retire items that are no longer relevant, and adjust sequencing. This keeps the debt program responsive to reality rather than committed to a plan that has gone stale. The cost of regular review is small; the cost of following an outdated prioritization is capacity spent on debt that no longer matters while current risks go unaddressed.

## Common Traps

### Paralysis From An Oversized Backlog

A long list of plausible debt items with no sequencing decision. The trap is nothing getting addressed because everything seems important.

### Prioritizing By Complaint Or Interest

Selecting debt based on who feels strongly or what is technically engaging. The trap is lower-impact debt done before higher-impact debt.

### Debt As Leftover Work

Addressing debt only when features finish early. The trap is debt that almost never gets capacity because features rarely finish early.

### Only Quick Wins Or Only Structural Work

A program of small fixes that leaves systemic risk growing, or large refactors that starve features for too long. The trap is imbalance that serves neither velocity nor product.

### Ignoring Dependencies

Sequencing debt without regard for prerequisites and bundling. The trap is rework and missed efficiency from poor ordering.

### Rigid Roadmap

Following a debt plan set once, long after context changed. The trap is capacity spent on stale priorities while current risks wait.

## Self-Check

- [ ] Debt items were sequenced by impact divided by effort, not by complaint volume or technical interest.
- [ ] Debt that blocks planned feature work was prioritized as a prerequisite investment, not as a competitor to features.
- [ ] The debt program balances quick wins, for momentum, and structural investment, for systemic risk.
- [ ] Dependencies among debt items were mapped, and sequencing accounts for prerequisites and bundling.
- [ ] Debt has dedicated, protected capacity in the roadmap, not leftover capacity scavenged from features.
- [ ] A debt health metric is tracked over time and used to adjust allocation and prioritization.
- [ ] Prioritization is revisited regularly as features, incidents, and codebase evolution change what matters most.
- [ ] The overall debt trend is moving in the right direction, or the allocation is being adjusted to correct it.
- [ ] Both product and engineering participate in prioritization, bringing feature context and debt context together.
- [ ] No high-impact debt waits indefinitely while lower-impact debt is addressed, and no allocation is so rigid it cannot respond to changing risk.
