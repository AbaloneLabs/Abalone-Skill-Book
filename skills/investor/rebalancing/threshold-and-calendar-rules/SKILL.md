---
name: threshold_and_calendar_rules.md
description: Use when the agent is choosing or evaluating rebalancing rules, setting drift tolerance bands, deciding between calendar-based and threshold-based rebalancing, sizing bands by volatility and cost, or judging how rebalancing frequency, transaction costs, taxes, and behavioral discipline interact to keep a portfolio near its target risk profile.
---

# Threshold And Calendar Rules

Rebalancing rules determine when a portfolio is brought back toward its target. The two dominant families are calendar rules (rebalance on a fixed schedule) and threshold rules (rebalance when drift exceeds a band). Both can work, and both can fail. The judgment problem is that the "best" rule depends on portfolio size, volatility, transaction costs, taxes, and the investor's discipline, and agents often pick a rule by intuition or backtest fit rather than by reasoning about these tradeoffs.

This skill is for designing or evaluating the rebalancing trigger itself, complementing broader rebalancing judgment.

## Core Rules

### Tie The Rule To The Risk Profile, Not To Convenience

The purpose of rebalancing is to keep the portfolio near its intended risk. The rule should trigger when risk has drifted enough to matter, not merely when a date arrives.

Ask:

- how much weight drift changes portfolio risk;
- which sleeves are volatile enough to breach risk quickly;
- whether the rule catches risk concentration, not just dollar drift;
- whether correlated sleeves drift together and amplify risk.

A calendar rule that lets a volatile sleeve drift for a year can allow large risk shifts. A threshold rule set on dollar weight can miss risk concentration. Design the rule around risk, then express it in actionable terms.

### Choose Calendar, Threshold, Or Hybrid Deliberately

Each family has distinct properties.

Calendar rules:

- simple and predictable;
- lower monitoring burden;
- may allow large drift between dates for volatile portfolios;
- can be combined with a threshold override for risk breaches.

Threshold rules:

- tighter risk control;
- require monitoring and may trade more in volatile, whipsawing markets;
- need well-calibrated bands to avoid overtrading;
- better for large portfolios where drift matters in dollars.

Hybrid rules (calendar review with threshold-triggered trades) often capture the benefits of both. Match the rule type to the portfolio's volatility, size, and the investor's monitoring capacity.

### Calibrate Bands To Volatility, Cost, And Size

A single 5% band is not appropriate for every portfolio. Bands should reflect how fast each sleeve drifts and how much trading costs.

Set bands considering:

- the volatility of each sleeve (more volatile needs tighter or more frequent review);
- transaction costs and bid-ask spreads;
- tax friction in taxable accounts;
- portfolio size (small accounts may have fixed-cost floors);
- the correlation structure, since correlated drift amplifies risk.

Tighter bands improve risk control but raise cost and churn. Wider bands lower cost but allow more risk drift. There is no free lunch; the band is a tradeoff.

### Account For Asymmetric And Correlated Drift

Drift is not symmetric. A 90% equity rally shifts risk far more than a 10% bond move. Correlated sleeves drift together.

Consider:

- relative (percentage) versus absolute (point) bands, and which catches the relevant drift;
- risk-weighted bands that trigger on risk contribution, not dollar weight;
- the effect of several sleeves breaching at once in a trend;
- whether a single breach or a portfolio-level breach should trigger action.

Risk-aware bands catch what dollar bands miss.

### Incorporate Costs And Taxes Into The Trigger

A rebalance is only worth doing if the expected benefit exceeds its cost. The rule should reflect this.

Estimate:

- the round-trip cost of the trade;
- the tax cost of realizing gains;
- the expected risk reduction or rebalancing premium from the trade;
- whether directing cash flows or dividends can achieve the rebalance without selling.

For taxable investors, threshold rules that trigger tax-costly sales may underperform cash-flow rebalancing. Cost-aware rules avoid churn that destroys after-tax value.

### Build In Discipline And Avoid Market Timing

The rule's value comes from mechanical discipline. If the investor overrides it based on views ("equities will keep rising, so don't sell"), the rule becomes market timing and loses its benefit.

Require:

- a pre-commitment to follow the rule;
- a documented exception process for genuine thesis changes;
- separation of rebalancing (restore target) from tactical views (separate budget);
- review of whether the investor actually followed the rule in past stress.

A rule the investor ignores in a crash is just decoration.

### Review The Rule Periodically

Rebalancing rules should be revisited as the portfolio, costs, and the investor's situation change.

Review:

- whether realized drift and breaches match expectations;
- whether costs and taxes are eroding the benefit;
- whether the target allocation itself is still right;
- whether life changes (retirement, withdrawal, inheritance) require a new rule.

## Common Traps

### One-Size-Fits-All Bands

A fixed 5% band applied to every portfolio ignores volatility, cost, and size. Bands should be calibrated.

### Calendar Rules That Allow Excessive Drift

Annual rebalancing of a volatile portfolio can let risk drift far before the trigger fires.

### Threshold Rules That Overtrade In Choppy Markets

Tight bands in volatile, mean-reverting markets cause whipsaw and cost drag with little risk benefit.

### Ignoring Tax And Cost In The Trigger

A rule that triggers frequent taxable sales can destroy after-tax value even while looking good pre-tax.

### Treating Rebalancing As A Return Source

Rebalancing can add a small premium in mean-reverting markets and cost in trending markets. It is primarily risk control. Overselling its return benefit leads to disappointment and abandonment.

### Overriding The Rule Based On Views

Skipping a rebalance because "this sleeve will keep going" converts discipline into market timing and typically underperforms.

### Measuring Drift Only In Dollars

Dollar drift can hide risk concentration. A volatile sleeve that is modestly overweight by dollars can dominate portfolio risk.

## Self-Check

- [ ] The rule is tied to risk drift, not only to calendar convenience or dollar weight.
- [ ] Calendar, threshold, or hybrid choice is justified by volatility, size, cost, and monitoring capacity.
- [ ] Bands are calibrated to each sleeve's volatility, transaction cost, tax friction, and correlation structure.
- [ ] Relative, absolute, and risk-weighted band options were considered, with correlated drift accounted for.
- [ ] Round-trip cost, tax, and expected benefit were incorporated into the trigger, and cash-flow rebalancing was considered to reduce selling.
- [ ] The rule includes a pre-commitment to discipline and a documented exception process, separating rebalancing from tactical views.
- [ ] The rule is scheduled for periodic review against realized drift, cost, and life changes.
- [ ] The recommendation frames rebalancing primarily as risk control, acknowledges cost and tax drag, and notes that investor-specific circumstances and professional tax and financial advice may be warranted.
