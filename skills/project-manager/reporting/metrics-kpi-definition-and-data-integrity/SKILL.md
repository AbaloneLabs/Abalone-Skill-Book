---
name: metrics_kpi_definition_and_data_integrity.md
description: Use when the agent is defining project metrics and KPIs, establishing measurement frameworks, ensuring data quality and integrity for project reporting, selecting leading versus lagging indicators, avoiding metric manipulation and gaming, or designing a project measurement system that drives the right behaviors.
---

# Metrics, KPI Definition, and Data Integrity

Metrics are not neutral observations; they are instructions. Whatever a project measures and rewards becomes what the team optimizes for, sometimes at the expense of what actually matters. A well-chosen metric aligns behavior with project success; a poorly chosen one creates a target that, once hit, reveals it was measuring the wrong thing. Goodhart's law, when a measure becomes a target it ceases to be a good measure, is the central risk of project measurement, and it operates silently: the numbers improve while the underlying health deteriorates.

The judgment problem is how to choose metrics that reflect real project health and drive constructive behavior, how to combine leading and lagging indicators, and how to preserve the data integrity that makes metrics trustworthy. Agents tend to over-measure, choose metrics that are easy rather than meaningful, and ignore the gaming behaviors their metrics incentivize.

## Core Rules

### Choose Metrics That Reflect Outcomes, Not Just Activity

Every metric should be defensible against the question: "If this number improves, is the project genuinely better off?" Activity metrics (hours, tasks, lines of code) can improve while outcomes worsen. Outcome metrics (delivered value, defect rates, schedule variance, customer satisfaction) connect directly to project success. Prefer outcome metrics, and use activity metrics only as diagnostic context, never as headline measures of health.

### Balance Leading and Lagging Indicators

Lagging indicators (delivered scope, cost variance, final defect count) confirm what has already happened; they are accurate but late. Leading indicators (requirements volatility, early test results, team velocity trend, risk burn-down) predict what will happen; they enable early action. A measurement system of only lagging indicators reports history; one with leading indicators enables intervention. Balance both, and be explicit about which is which.

### Limit the Number of Headline Metrics

A dashboard with thirty metrics has no signal. Decision-makers can track a small number of headline metrics, typically five to seven, that together capture project health. Supporting metrics provide diagnostic depth when a headline metric signals trouble. Resist the pressure to add every measurable quantity to the headline view; more metrics dilute attention and obscure the signal.

### Define Each Metric Precisely With a Data Source and Owner

Every metric needs a precise definition: what exactly is being measured, the data source, the calculation, the cadence, and the owner responsible for its accuracy. Ambiguity in definition produces metrics that mean different things to different people and that drift over time. Document the definition, and when the definition changes, version it so historical comparison remains valid.

### Anticipate and Design Against Gaming

When a metric becomes a target, people optimize for the metric, sometimes by gaming it: inflating estimates to boost velocity, closing defects without fixing them to improve counts, reclassifying work to favorable buckets. Design metrics to be hard to game: use ratios rather than raw counts, cross-check related metrics for inconsistency, and rotate what is measured so no single number becomes the sole target. Watch for gaming as a signal that the metric is misaligned with the goal.

### Ensure Data Integrity Through Validation and Single Source

Metrics are only as trustworthy as their underlying data. Maintain a single authoritative source for each data type, validate inputs for completeness and accuracy, and reconcile metrics that draw on related data so they agree. When metrics from different sources disagree, trust collapses and decision-makers revert to anecdote. Invest in data integrity as the foundation of credible measurement.

### Connect Metrics to Decisions and Actions

A metric that drives no decision is overhead. For each metric, be able to state what decision or action it informs, what threshold triggers that action, and who is responsible for acting. If no one can answer what they would do differently based on the metric, it should not be a headline metric. Measurement exists to enable better decisions, not to fill reports.

### Review and Retire Metrics That Stop Adding Value

Metrics have a lifecycle. A metric that was once useful may become irrelevant as the project evolves. Periodically review the metric set: which are still informing decisions, which have been gamed into uselessness, and which no longer reflect current priorities. Retire or replace metrics that have stopped adding value. A measurement system that never changes accumulates dead metrics.

## Common Traps

### Activity Metrics Treated as Health Indicators

Hours, tasks, or velocity are reported as signs of health, while outcomes deteriorate unnoticed. The trap is that activity is easy to measure and feels like progress. Measure outcomes.

### Only Lagging Indicators

The measurement system reports only what has already happened, so action is always reactive. The trap is that lagging indicators are accurate and comfortable. Add leading indicators.

### Metric Overload

So many metrics are reported that none stand out, and decision-makers cannot identify what matters. The trap is that more metrics feel more rigorous. Limit headline metrics.

### Imprecise Definitions

A metric means different things to different people, so comparisons are invalid and trends are noise. The trap is that the name feels clear enough. Define precisely.

### Gaming the Target

The team optimizes for the metric by inflating estimates, closing defects prematurely, or reclassifying work, and the numbers improve while health declines. The trap is Goodhart's law. Design against gaming.

### Competing Data Sources

Different reports draw on different data, the numbers disagree, and trust collapses. The trap is that manual reporting feels controlled. Use a single source and validate.

### Metrics With No Decision Attached

A metric is reported that no one acts on, consuming effort without informing any decision. The trap is measurement for its own sake. Connect every metric to a decision.

### Dead Metrics Never Retired

The metric set accumulates measures that once mattered but no longer do, cluttering reports. The trap is that removing metrics feels like losing information. Retire metrics that stop adding value.

## Self-Check

- [ ] Does each headline metric reflect an outcome whose improvement means the project is genuinely better off?
- [ ] Is there a balance of leading indicators (predictive) and lagging indicators (confirmatory)?
- [ ] Are headline metrics limited to a number decision-makers can actually track, typically five to seven?
- [ ] Does each metric have a precise definition, documented data source, calculation, cadence, and owner?
- [ ] Have metrics been designed to resist gaming, using ratios, cross-checks, and rotation rather than raw counts?
- [ ] Is there a single authoritative data source for each metric type, with validation and reconciliation across related metrics?
- [ ] Can you state, for each metric, what decision or action it informs, the threshold that triggers action, and who acts?
- [ ] Are metrics periodically reviewed and retired when they stop informing decisions or reflecting priorities?
- [ ] Would an improvement in each headline metric correspond to genuine project improvement, or could it be gamed?
- [ ] Is the measurement system trusted by decision-makers, or have past inconsistencies eroded credibility?
