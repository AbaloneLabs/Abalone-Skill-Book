---
name: kpi_selection_and_hierarchy.md
description: Use when the agent is selecting key performance indicators, building a KPI hierarchy from north star to input metrics, choosing which metrics deserve to be KPIs versus supporting metrics, or structuring how metrics roll up to business outcomes.
---

# KPI Selection And Hierarchy

A KPI is a promise to pay attention. By calling a metric a key performance indicator, a team declares that this number, above all others, will be watched, explained, and acted upon. That promise is expensive, because attention is finite and every KPI competes for it. The failure mode is not having too few metrics, it is having too many KPIs, each diluting the others until none of them actually steers the team.

The harm this skill prevents is a dashboard full of numbers that together mean nothing. When everything is a KPI, nothing is, because there is no hierarchy to tell the team which movement matters most. A team that tracks forty metrics reacts to whichever one moved most recently, loses the connection between daily work and the business outcome, and optimizes locally while the product drifts globally. Selecting KPIs is the act of choosing what the organization will care about, and that choice requires structure.

Use this skill when defining a metric set for a team or product, when building a hierarchy from a north star down to input metrics, when deciding which metrics deserve KPI status, and when reviewing whether an existing KPI set still serves the business. The work is to choose few enough metrics to focus, structure them so daily work connects to outcomes, and reject vanity measures that flatter without informing.

## Core Rules

### Choose A North Star That Reflects Sustained Customer Value

The north star metric is the single outcome that best represents the product delivering lasting value to users. It is not revenue, which is a consequence, and it is not a vanity number like total signups, which can grow while value shrinks. A strong north star combines reach with depth, such as the number of users who completed a core value action in a period.

Test a candidate north star by asking whether sustained growth in it implies the product is genuinely working for users and the business. If the metric can grow while users are worse off, it is the wrong star. The north star should be hard to game precisely because it reflects real value, not activity.

### Distinguish KPIs From Supporting Metrics

A KPI is a metric the team commits to move and explain; a supporting metric is context that helps interpret the KPI. The distinction matters because KPIs command attention and accountability, while supporting metrics inform without competing for focus. Promoting too many metrics to KPI status fragments attention and erodes the signal.

Limit KPIs to the few metrics whose movement would change what the team does next. A useful rule is that if a metric's movement would not trigger a decision or a conversation, it is a supporting metric, not a KPI. Resist the political pressure to make every team's favorite number a KPI.

### Build A Hierarchy That Connects Inputs To Outcomes

A KPI set works only if daily work connects to the top metric through a legible hierarchy. The north star sits at the top, a small number of outcome KPIs sit beneath it, and input metrics that the team can directly influence sit beneath those. Each layer should explain how the layer above moves.

The hierarchy lets a team trace a drop in the north star down to the input that broke, and lets local work roll up to a visible outcome. Without this structure, teams optimize their own metric with no sense of whether it moves the business. Draw the tree explicitly and check that every input has a line to an outcome.

### Reject Vanity Metrics That Flatter Without Informing

Vanity metrics go up regardless of whether the product improves. Total registered users, total downloads, and gross page views can rise while retention and value fall, because they accumulate without reflecting quality. A vanity metric feels like progress but cannot guide a decision, because the same number is compatible with both success and failure.

Apply the decision test: if this metric moved, would it change what we do? If the answer is no, it does not belong in the KPI set. Total counts are especially dangerous because they almost never decline and therefore never signal trouble.

### Align KPI Selection With The Current Strategy Phase

The right KPIs depend on what the business needs most right now. A product in growth mode may prioritize activation and reach; a product fighting churn may prioritize retention and reactivation; a mature product optimizing margin may prioritize efficiency and cost per outcome. A KPI set frozen across phases misdirects effort toward yesterday's priority.

Review the KPI set against the current strategy at least each planning cycle. When the strategy shifts, the KPIs should shift with it, and the team should understand why. Keeping stale KPIs because they are familiar is a quiet way of optimizing for the past.

### Make Each KPI Owned And Explainable

Every KPI needs a named owner who is accountable for understanding its movement, not necessarily for moving it single-handedly. Ownership means the person can explain why the metric moved, what drove it, and what would move it next. A KPI without an owner is an orphan that nobody investigates when it drifts.

Explainability is the practical test of a good KPI. If nobody can decompose a metric movement into its drivers, the metric is too abstract to steer work. Prefer KPIs whose composition is understood well enough that a change can be diagnosed, not merely observed.

### Balance The Set Across Outcome Dimensions

A single KPI, even a good north star, captures only one dimension of health. A product can grow the north star while quality erodes, costs rise, or a key segment collapses. A robust KPI set balances the primary outcome with guardrails on the dimensions most likely to be sacrificed to hit it.

Include at least one metric per critical dimension: growth, retention, quality, and efficiency, adjusted to the business. The goal is not comprehensive coverage but resilient coverage, enough that a win on one KPI cannot hide a loss on another.

## Common Traps

### Calling Everything A KPI

When every metric on the dashboard is labeled a KPI, focus disappears and the team reacts to noise. The label should be scarce and meaningful. If the set has more than a handful of KPIs, it is really a list of metrics with no priority.

### Choosing A North Star That Can Grow While Value Falls

A north star based on raw activity or total counts can rise as the product degrades, because it rewards motion over value. Always check that sustained growth in the north star implies genuine user and business benefit.

### Breaking The Link Between Inputs And Outcomes

When input metrics are tracked without a line to outcomes, teams optimize locally with no evidence they move the business. Draw the hierarchy and verify each input rolls up, or the local work is unmoored.

### Keeping Stale KPIs Across Strategy Shifts

A KPI set that does not change when the strategy changes keeps the team optimizing for a priority that is no longer relevant. Review KPIs against strategy each cycle and retire the ones that no longer fit.

### Promoting Vanity Metrics For Comfort

Total counts and cumulative numbers feel reassuring because they only go up, but they cannot signal decline or guide a decision. Comfort is not information; exclude metrics that flatter without informing.

### Assigning KPIs Without Ownership

A KPI that nobody owns will not be explained when it moves, and movements will go unexamined until they become crises. Every KPI needs a named owner who can decompose and explain its movement.

## Self-Check

- [ ] A single north star metric is chosen that reflects sustained customer value, not revenue or a vanity count.
- [ ] The KPI set is small enough that each KPI would trigger a decision or conversation if it moved.
- [ ] A hierarchy connects the north star to outcome KPIs to input metrics, and every input has a line to an outcome.
- [ ] Vanity metrics that can grow while value falls are excluded from the KPI set.
- [ ] The KPI set was reviewed against the current strategy phase and updated where the strategy has shifted.
- [ ] Each KPI has a named owner who can explain its movement and its drivers.
- [ ] Each KPI is decomposable and explainable, so a movement can be diagnosed rather than merely observed.
- [ ] The set balances the primary outcome with guardrail KPIs on growth, retention, quality, and efficiency as relevant.
- [ ] Supporting metrics are distinguished from KPIs and are not competing for the same attention.
- [ ] No KPI is retained solely because it is familiar or comforting; each earns its place by informing decisions.
