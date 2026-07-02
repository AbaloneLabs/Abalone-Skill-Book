---
name: relative_strength.md
description: Use when the agent is constructing or evaluating a relative strength or cross-sectional momentum strategy that ranks and selects assets based on comparative performance, analyzing the effects of ranking methodology and lookback period on returns, managing turnover and transaction costs from rotation, assessing drift and rebalancing frequency, and weighing the behavioral and cost challenges of a ranking-based momentum approach.
---

# Relative Strength

Relative strength, or cross-sectional momentum, is a strategy that ranks a universe of assets by their past performance and invests in those that have outperformed their peers, rotating capital toward strength. Unlike trend following (time-series momentum, which looks at an asset's own past), relative strength compares assets to each other, buying the winners and selling the losers within the group. The thesis is that relative outperformance persists due to behavioral biases, institutional flows, and slow information diffusion, and that systematically rotating toward strength captures this effect. The judgment problem is that relative strength strategies generate significant turnover and transaction costs from frequent rotation, suffer from drift between rebalancing dates, and are prone to underperformance when market leadership rotates sharply. Investors who implement the strategy without accounting for turnover costs, rebalancing frequency, and the behavioral pain of buying what has already risen will find the theoretical premium consumed by implementation.

This skill is for constructing and maintaining relative strength strategies with discipline about ranking, turnover, cost, and drift.

## Core Rules

### Choose A Ranking Methodology And Lookback Period Deliberately

The way assets are ranked and the lookback period used for momentum significantly affect the strategy's behavior and returns.

Decide:

- the ranking metric (e.g., total return over the lookback, risk-adjusted return, or price change), since each captures different aspects of strength;
- the lookback period (commonly 6-12 months for cross-sectional momentum), which trades off signal strength against noise;
- whether to exclude the most recent month (to avoid short-term reversal effects that can contaminate medium-term momentum);
- the universe being ranked (e.g., sectors, countries, asset classes, or individual stocks), since broader universes offer more diversification but more turnover.

The methodology should be grounded in evidence and robust to parameter variation, not optimized to a single historical period. Sensitivity testing across lookbacks and metrics is essential.

### Manage Turnover And Transaction Costs

Relative strength strategies are turnover-intensive, because rankings change and the portfolio rotates. Transaction costs can consume a large portion of the gross premium.

Control costs:

- estimate the strategy's annual turnover and the resulting transaction costs (commissions, spreads, market impact);
- choose a rebalancing frequency that balances signal capture against cost (monthly, quarterly);
- consider a band or buffer around the ranking cutoff, so that small ranking changes do not trigger trades;
- use liquid, low-cost instruments (ETFs, futures) to minimize execution costs;
- account for taxes in taxable accounts, since frequent rotation generates short-term gains.

A relative strength strategy with a gross premium of 2% can have a net premium near zero if turnover and costs are high. Cost-adjusted backtesting, not gross backtesting, is the relevant analysis.

### Address Drift Between Rebalancing Dates

Between rebalancing dates, the portfolio's exposures drift as prices change. Drift can either help or hurt, depending on whether held winners continue to outperform.

Manage drift:

- recognize that infrequent rebalancing allows more drift, which can dilute the momentum signal;
- balance the cost of more frequent rebalancing against the benefit of maintaining exposure;
- consider partial rebalancing or threshold-based adjustments to control drift without full rotation;
- monitor the portfolio's effective ranking drift, not just its holdings.

The right rebalancing frequency depends on the tradeoff between maintaining the signal and controlling cost. There is no universally optimal frequency; it depends on the universe, costs, and signal strength.

### Diversify The Universe To Reduce Concentration Risk

Relative strength strategies can concentrate in a few strong assets, especially in narrow universes. Diversification across the ranked universe reduces idiosyncratic risk.

Apply:

- hold multiple top-ranked assets (e.g., top quartile or top third), not just the single winner;
- diversify across sectors, countries, or asset classes within the ranked universe;
- apply position limits to prevent any single asset from dominating;
- consider the correlation structure of the held assets, since holding several highly correlated "winners" offers less diversification.

A diversified relative strength portfolio captures the cross-sectional momentum effect with less single-asset risk and a smoother return path.

### Prepare For Leadership Rotation And Underperformance

Relative strength underperforms when market leadership rotates sharply, because the strategy holds yesterday's winners while new leaders emerge. These periods are inherent to the strategy.

Understand:

- sharp regime changes (e.g., growth-to-value, or a sector rotation) can cause rapid underperformance as held winners fall and new winners rise;
- the strategy's lag in adapting to new leadership is a feature of its medium-term focus, not a bug;
- underperformance can persist for months during extended rotation periods;
- the behavioral challenge of holding assets that are falling while new leaders rise is significant;
- the strategy requires a long horizon and pre-commitment to capture the momentum premium through cycles.

The investor who abandons relative strength during a rotation period captures the underperformance and misses the recovery when new trends establish.

### Combine With Other Factors To Smooth Returns

Relative strength (momentum) can be combined with other factors to reduce its sharp edges and improve consistency.

Consider:

- combining momentum with value, since the two are negatively correlated and value tends to do well when momentum crashes;
- adding quality as a defensive anchor within the momentum portfolio;
- using relative strength within a multi-factor framework to capture momentum alongside other premia;
- the benefit of diversification across factor returns, smoothing the overall portfolio.

Factor combination does not eliminate momentum's risks, but it can reduce the severity of drawdowns and make the strategy more behaviorally sustainable.

## Common Traps

### Ignoring Turnover And Transaction Costs

Relative strength is turnover-intensive. Gross premia overstate net returns; cost-adjusted analysis is essential.

### Optimizing The Lookback On A Single Backtest

Over-fitting the ranking period to historical data produces a fragile strategy. Robustness across parameters matters more.

### Holding Too Few Assets

Concentrating in one or two top-ranked assets raises idiosyncratic risk. Diversification across the ranked universe is safer.

### Neglecting Drift Between Rebalancing

Infrequent rebalancing allows exposure drift that can dilute the signal. The frequency must balance cost and signal maintenance.

### Abandoning During Leadership Rotation

Relative strength underperforms when leadership rotates. Quitting then captures the loss and misses the recovery.

### Confusing Relative Strength With Prediction

The strategy reacts to established relative outperformance; it does not forecast. Treating it as prediction invites discretion and deviation.

### Overlooking Tax Inefficiency

Frequent rotation generates short-term gains. Tax efficiency matters in taxable accounts.

## Self-Check

- [ ] The ranking methodology and lookback period are chosen deliberately, grounded in evidence, and robust across parameter variations.
- [ ] Annual turnover and transaction costs (commissions, spreads, impact, taxes) are estimated, and the strategy is evaluated on a net, cost-adjusted basis.
- [ ] Rebalancing frequency balances signal maintenance against cost, with buffers or thresholds to reduce unnecessary rotation.
- [ ] Drift between rebalancing dates is monitored and managed, with awareness of its effect on exposure.
- [ ] The portfolio holds multiple top-ranked assets with position limits, diversifying across the universe to reduce concentration risk.
- [ ] The investor is prepared for leadership rotation and underperformance, with a long horizon and pre-commitment to the strategy.
- [ ] Relative strength is considered as part of a multi-factor approach to smooth returns and reduce drawdown severity.
- [ ] The guidance flags that relative strength can underperform for extended periods, that turnover costs can erode premia, that past performance does not guarantee future results, and that professional advice may be warranted for systematic rotation strategies.
