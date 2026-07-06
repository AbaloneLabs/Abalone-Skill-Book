---
name: experiment_metrics_and_guardrails.md
description: Use when the agent is choosing metrics for an experiment or feature launch — selecting a primary metric, defining supporting/secondary metrics, designing guardrail metrics to catch regressions, detecting cannibalization or displacement effects, handling network/spillover effects in metrics, choosing between count vs rate vs ratio metrics, or defining when a launch decision is supported by the metric set. Also covers the failure modes of a single north-star metric that hides regressions elsewhere, funnel metrics that move by shifting users between steps, ratio metrics whose denominator shifts independently, guardrails set too loose to catch real harm, and the recurring mistake of optimizing a local metric that cannibalizes the global one it is supposed to proxy.
---

# Experiment Metrics And Guardrails

A metric is a model of what you care about, and every metric is wrong in some way. The job of metric design is to choose metrics whose wrongness does not lead you astray — to pick a primary metric that captures the goal, supporting metrics that explain how the goal moved, and guardrail metrics that catch the regressions the primary metric cannot see. The judgment problem is that a single number, however carefully chosen, hides everything it does not measure, and a feature that "wins" its primary metric can quietly lose on latency, retention, error rate, or a neighboring metric it cannibalized. Teams that watch one north-star metric ship features that look successful and erode the product; teams that watch a well-designed metric set catch the tradeoffs before they ship.

Agents tend to miss these problems because picking a metric feels obvious ("we want more engagement, so measure engagement"), and a moving-upward primary metric feels like success. The harm is in what the metric does not capture. A feature increases clicks (the primary metric) by making buttons look clickable, but decreases time-on-task and retention (not measured) because users click, get frustrated, and leave. A feature increases search CTR by surfacing more sensational results, cannibalizing conversion (the metric that actually pays). A ratio metric improves because its denominator collapsed, not because its numerator grew. A guardrail set at "no more than 5% regression" lets a 4.9% latency regression ship, accumulating across features until the product is slow. The judgment problem is to design the metric set — primary, supporting, guardrails — as a system that exposes tradeoffs rather than hiding them, and to read the set together rather than acting on the one number that moved.

This skill covers primary/supporting metric selection, guardrail design, cannibalization, ratio-metric pitfalls, and network-effect handling. It complements the ab-test-design skill (experiment validity and statistics) and the statistical-pitfalls skill (interpretation errors).

## Core Rules

### Choose A Primary Metric That Captures The Goal, Not A Proxy For It

The primary metric is the decision criterion: did this feature achieve what we intended? Its quality determines whether shipping decisions serve the goal or a distortion of it:

- **The primary metric should map directly to the intended user or business outcome.** If the goal is "help users complete tasks faster," the primary metric is task completion time, not clicks (a proxy that can move the wrong way). If the goal is revenue, measure revenue, not a proxy like add-to-cart that can cannibalize returns.
- **Beware proxies that diverge from the goal under optimization.** Click-through rate is a proxy for engagement; engagement is a proxy for value. Optimizing the proxy hard enough (sensational headlines, dark patterns) moves the proxy while harming the goal. The closer the metric is to the true outcome, the safer it is to optimize.
- **Prefer absolute and business-aligned units.** "Revenue per user per day" or "successful task completions per week" tracks the outcome; a normalized rate or ratio can move for reasons unrelated to the goal (see ratio pitfalls below).
- **One primary metric drives the decision.** Multiple co-primary metrics invite cherry-picking the winner. If two outcomes matter equally, define a single composite or a clear decision rule upfront.

### Add Supporting Metrics That Explain How The Primary Moved

A primary metric tells you whether the goal moved; supporting metrics tell you why, and whether the movement is the mechanism you intended:

- **Funnel and driver metrics localize the change.** If the primary (e.g., checkout completion) moved, the supporting metrics (add-to-cart rate, cart-to-checkout rate, payment-error rate) show which step drove it. A primary that moved for an unintended reason (a payment bug that happened to reduce errors) is a different decision than one that moved by the intended mechanism.
- **Supporting metrics catch "movement by redistribution."** A primary can improve because users shifted between segments or steps rather than because the overall improved. Supporting metrics reveal whether the gain is real or compositional.
- **Do not let supporting metrics become decision criteria.** They explain; they do not decide. Acting on whichever supporting metric happens to be significant is multiple testing (see the ab-test-design skill).

### Design Guardrails That Catch The Regressions The Primary Cannot See

Guardrails are the metrics that, if they regress, block the launch even if the primary improved. They exist because no primary metric captures all the harm a feature can do:

- **Cover the harm categories the feature could plausibly cause.** Performance (latency, error rate, crash rate), reliability (uptime, timeout rate), business health (retention, revenue, support tickets), and user experience (page load, abandonment). A feature that improves the primary but spikes latency or errors should not ship.
- **Set thresholds tight enough to catch real harm.** A guardrail at "no more than 5% regression" lets four 1% regressions ship per quarter and compound. Set thresholds against the practical cost of the regression, not a round number; for latency and errors, even small regressions matter.
- **Distinguish hard and soft guardrails.** A hard guardrail (e.g., error rate must not increase) blocks launch; a soft guardrail (e.g., a minor latency increase) triggers investigation but may be acceptable with justification. Decide which is which upfront.
- **Include long-term and lagging guardrails.** Some harms (retention, churn, support volume) lag the feature by weeks. A guardrail set measured only during the experiment window can miss them; plan a post-launch follow-up for lagging metrics.

### Detect Cannibalization And Displacement Effects

A common failure mode is a feature that improves a local metric by cannibalizing a global one — the gain is real locally and a loss globally:

- **Cannibalization within the funnel.** A feature increases clicks on a new element by drawing them from another element; total clicks are unchanged or down. Measure the total (the global metric), not just the new element's local metric.
- **Cannibalization across products or surfaces.** A new feature captures demand that would have converted elsewhere; the platform's total conversion is unchanged but the feature "looks successful." Measure the global outcome, not just the feature's surface.
- **Displacement in time.** A feature pulls demand forward (a flash sale increases today's sales at the expense of tomorrow's); weekly or monthly totals reveal what daily metrics hide. Measure over a long enough window to catch temporal displacement.
- **The test: does the global metric move, or only the local one?** If only the local metric moved, the feature redistributed rather than created value. Always pair a local/feature metric with its global counterpart.

### Handle Ratio Metrics Carefully — Denominators Can Move Independently

Ratio metrics (CTR = clicks / impressions; conversion rate = conversions / sessions) are common and treacherous, because either the numerator or the denominator can move, and the ratio can change for reasons unrelated to the goal:

- **A ratio can improve because the denominator shrank.** CTR goes up because impressions went down (fewer, more-targeted impressions) even if clicks are flat. Always report numerator and denominator alongside the ratio.
- **Decide whether to optimize the ratio or the absolute.** Maximizing CTR can reduce total clicks (by showing fewer impressions); maximizing total clicks can reduce CTR. Which serves the goal depends on the business — decide explicitly, do not let the metric's form decide for you.
- **Beware ratio metrics with rare denominators.** A conversion rate where the denominator is tiny per user is high-variance and easily distorted; an absolute count or a per-active-user rate may be more stable.

### Account For Network Effects And Spillover In Metric Design

In settings with network effects (two-sided markets, social, referrals), the no-interference assumption of A/B testing breaks, and standard user-level metrics mislead:

- **Treatment can spill to control.** A feature that increases rider demand affects driver availability for control riders; a social feature changes what control users see via their treated friends. User-level metrics then mix direct and indirect effects.
- **Use the right unit of randomization and analysis.** Cluster-randomized (by city, school of users, or geo) experiments keep interference within clusters; analyze at the cluster level. The metric must be defined and aggregated at the randomization unit.
- **Measure global outcomes, not just per-user.** In a market, total matches or total revenue may be the right metric; per-user metrics can mask market-level effects (e.g., each rider gets fewer matches even as total matches rise).

## Common Traps

### Optimizing A Proxy That Diverges From The Goal

Maximizing a proxy metric (clicks, CTR, engagement) hard enough that it moves the proxy while harming the true outcome (value, revenue, retention). Choose the primary metric as close to the true outcome as feasible.

### A Single North-Star Metric Hiding Regressions

Watching only the primary metric and shipping features that win it while eroding latency, errors, retention, or a neighboring metric. Design guardrails across plausible harm categories.

### Guardrail Thresholds Too Loose To Catch Harm

Setting guardrails at round-number tolerances (5%, 10%) that let real regressions ship and compound. Set thresholds against practical cost; treat latency and errors strictly.

### Movement By Redistribution, Not Creation

A primary metric that improved because users or demand shifted between segments, steps, or time periods, with no global gain. Pair local metrics with global counterparts to detect cannibalization.

### Ratio Metric Improving Via A Shrinking Denominator

A ratio (CTR, conversion rate) that improved because its denominator fell, not because its numerator grew. Always report numerator and denominator; decide ratio vs absolute explicitly.

### Supporting Metrics Treated As Decision Criteria

Acting on whichever supporting or segment metric happened to be significant, which is multiple testing and ships noise. The primary metric decides; supporting metrics explain.

### Missing Lagging Guardrails

Measuring only short-window metrics and missing harms (retention, churn, support volume) that appear weeks after launch. Plan post-launch follow-ups for lagging metrics.

### User-Level Metrics Under Network Effects and composite Metric With Hidden Component Swings

Using user-level metrics in a two-sided market or social setting where treatment spills to control, mixing direct and indirect effects. Use cluster randomization and analyze at the cluster level.

A single composite "score" that improved while one component collapsed and another spiked, hiding the real change. Inspect components, not just the composite.

## Self-Check

- [ ] The primary metric maps directly to the intended outcome (not a diverging proxy), is in absolute/business-aligned units where possible, and is the single decision criterion (with a defined rule if two outcomes matter).
- [ ] Supporting metrics (funnel steps, drivers) are included to explain how the primary moved and to catch movement-by-redistribution, but are not used as decision criteria.
- [ ] Guardrails cover plausible harm categories (latency, errors, crashes, retention, revenue, support volume), with thresholds set against practical cost (strict for latency/errors), and hard vs soft guardrails distinguished upfront.
- [ ] Lagging guardrails (retention, churn) have a post-launch follow-up plan, since the experiment window may be too short to reveal them.
- [ ] Cannibalization and displacement are checked: local/feature metrics are paired with global counterparts (total clicks, total conversion, weekly/monthly totals) to confirm the feature created value rather than redistributing it.
- [ ] Ratio metrics report numerator and denominator alongside the ratio, and the decision explicitly chooses ratio vs absolute optimization rather than letting the metric's form decide.
- [ ] In settings with network effects (markets, social), randomization and analysis are at the cluster/geo level and metrics are aggregated at the randomization unit, with global outcomes measured alongside per-user metrics.
- [ ] Composite metrics are inspected at the component level, not trusted as a single score.
- [ ] The highest-risk cases were verified — a proxy diverging from the goal, a primary win with an unguarded regression, a ratio improving via a shrinking denominator, cannibalization hiding behind a local gain, a loose guardrail letting harm ship, and user-level metrics under interference — not only the primary metric moving up.
