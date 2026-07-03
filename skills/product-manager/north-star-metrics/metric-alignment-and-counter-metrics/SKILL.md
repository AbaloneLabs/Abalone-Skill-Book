---
name: metric_alignment_and_counter_metrics.md
description: Use when the agent is aligning teams around shared metrics, preventing local optimization that harms the whole, defining counter-metrics and guardrails, or ensuring that metric-driven teams do not game targets or sacrifice long-term value for short-term movement.
---

# Metric Alignment And Counter Metrics

When multiple teams optimize metrics simultaneously, the interaction of their efforts can produce outcomes no one intended. A team that improves its metric may harm another team's metric, or may move its number in a way that damages the product overall. Metric alignment is the discipline of ensuring that the metrics teams pursue combine to move the product in the intended direction, rather than canceling each other or causing harm. The central tool is the counter-metric: a paired metric that detects when optimization of the primary metric has begun to destroy value. Done well, alignment and counter-metrics keep metric-driven teams honest and pointed at genuine value. Done poorly, teams game their targets, sacrifice long-term health, or optimize past the point where further movement harms the product. Agents often set metrics without counter-metrics, leaving nothing to catch the moment optimization becomes destructive.

The harm this skill prevents is metric-driven self-harm. A team dutifully hitting its target can degrade retention, trust, quality, or long-term value, because the primary metric captures only one dimension of success and is silent on the others. Without counter-metrics, the damage is invisible until it becomes a crisis.

Use this skill before answering questions such as "how do we align teams on metrics", "what counter-metrics should we use", "are our metrics causing harm", or "how do we stop teams from gaming targets". The goal is to prevent the agent from assigning metrics without the guardrails that keep optimization constructive.

## Core Rules

### Pair Every Primary Metric With A Counter-Metric

Any metric, optimized hard enough, eventually conflicts with another dimension of value. Engagement metrics conflict with user wellbeing; acquisition metrics conflict with quality; speed metrics conflict with reliability; revenue metrics conflict with retention. The counter-metric is the paired measure that detects when optimization of the primary metric has crossed into harm. For every primary metric a team owns, identify the dimension most likely to be sacrificed, and track a counter-metric that would reveal the sacrifice.

The counter-metric is not a second target to optimize; it is a guardrail that triggers investigation when it moves the wrong way. If engagement rises but satisfaction or retention falls, the counter-metric signals that the engagement gain may be coming at the cost of value. The pair together describes success more completely than either alone.

### Align Team Metrics So They Combine Constructively

When each team optimizes its own metric in isolation, the combined effect can be neutral or harmful. A growth team optimizing acquisition may flood the product with low-quality users who damage the experience for existing users, whose retention an engagement team is trying to protect. The metrics are individually rational but collectively destructive. Alignment requires designing the set of team metrics so that pursuing them in combination moves the product toward the North Star rather than in conflicting directions.

Map the interactions between team metrics. Where two metrics conflict, either realign one team's metric, set a shared metric that both contribute to, or establish explicit tradeoff rules. Ignoring the interactions guarantees that local optimization will produce global friction.

### Detect Gaming Through Divergence Between Metric And Outcome

When a metric becomes a target, gaming follows: the number moves without the underlying value moving. The signal of gaming is divergence between the metric and the outcomes it is supposed to represent. Engagement rises but retention falls; tickets closed rises but satisfaction falls; features shipped rises but adoption does not. Counter-metrics catch this divergence, because they measure the outcome the primary metric is supposed to produce. Where metric and outcome diverge, the metric has stopped measuring what it was meant to measure, and optimization of it has become counterproductive.

Investigate divergence rather than explaining it away. The temptation is to attribute a falling counter-metric to external factors while crediting a rising primary metric to the team's effort. This asymmetry hides gaming. Hold both movements to the same standard of explanation.

### Prevent Short-Term Optimization From Sacrificing Long-Term Value

Many metrics can be moved quickly in ways that borrow from the future. Revenue can rise this quarter through aggressive upselling that increases next year's churn. Engagement can rise through features that addict users while eroding trust. Quality metrics can be hit by cutting scope that would have prevented future incidents. The pattern is movement now paid for by value later. Counter-metrics that lag, such as retention, trust, and technical debt indicators, catch this borrowing before it compounds.

Build lagging counter-metrics into the metric set even though they are slow, because they are the only signals that reveal whether short-term gains are real or borrowed. A metric set consisting only of fast-moving metrics will systematically optimize against the future.

### Distinguish Metrics To Optimize From Metrics To Monitor

Not every metric should be a target. Some metrics are worth tracking as monitors, to detect problems, without being assigned to a team for optimization. Conflating the two categories causes harm: metrics that should only be monitored become targets, leading teams to game them, while metrics that should be targets are only watched, leading to inaction. Be explicit about which metrics are optimization targets, which are guardrails, and which are purely diagnostic.

The rule of thumb is to optimize few metrics and monitor many. A team should own a small number of primary metrics with counter-metrics; the broader set of product metrics should be watched for anomalies without becoming local targets. This preserves focus while maintaining awareness.

### Revisit Metrics When They Stop Predicting Or Representing Value

Metrics decay. A metric that once represented value can stop doing so as the product, market, or behavior changes. Engagement measured one way may cease to correlate with retention as the product evolves; an acquisition metric may stop predicting lifetime value as the customer mix shifts. A metric set that is never revisited will gradually misdirect effort, because the metrics no longer mean what they once did. Build a cadence for reviewing whether metrics still represent and predict what they are supposed to.

The signal that a metric has decayed is the same divergence that signals gaming: the metric moves without the expected outcome following. Whether the cause is gaming or genuine change in the metric's meaning, the response is the same: investigate and, if necessary, replace the metric.

## Common Traps

### Primary Metric Without A Counter-Metric

Optimizing a single dimension with nothing to catch harm. The trap is dutiful optimization that degrades every dimension the metric does not capture.

### Locally Rational Metrics That Conflict Globally

Each team optimizing its number without considering interactions. The trap is collective effort that cancels out or causes harm.

### Explaining Away Divergence

Crediting metric gains to effort while blaming counter-metric declines on external factors. The trap is hidden gaming that persists until damage is severe.

### Fast Metrics Without Lagging Guardrails

Tracking only quickly-moving metrics. The trap is systematic optimization against long-term value, because no signal reveals the borrowing.

### Treating Every Metric As A Target

Turning diagnostic metrics into optimization goals. The trap is gaming of metrics that were meant only to detect problems.

### Never Revisiting Decaying Metrics

Keeping a metric set fixed as the product changes. The trap is effort misdirected by metrics that no longer represent or predict value.

## Self-Check

- [ ] Every primary metric is paired with a counter-metric that detects when optimization has crossed into harm.
- [ ] Team metrics are designed so that pursuing them in combination moves the product toward the North Star, not in conflicting directions.
- [ ] Divergence between a metric and its intended outcome triggers investigation rather than convenient explanation.
- [ ] Lagging counter-metrics, retention, trust, debt, are tracked to catch short-term gains borrowed from long-term value.
- [ ] Metrics are explicitly categorized as optimization targets, guardrails, or diagnostic monitors, and few are targets.
- [ ] A cadence exists for reviewing whether metrics still represent and predict value, and decaying metrics are replaced.
- [ ] Gaming is detected through metric-outcome divergence, not through trust in the team's intentions.
