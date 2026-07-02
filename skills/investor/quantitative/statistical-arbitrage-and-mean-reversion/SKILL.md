---
name: statistical_arbitrage_and_mean_reversion.md
description: Use when the agent is evaluating pair trading, statistical arbitrage, mean-reversion strategies, cointegration-based spreads, z-score entry rules, or assessing when a spread will revert versus when it has broken permanently. Covers pairs trading assumptions, cointegration stability, half-life, spread breakdown risk, and the danger of averaging into a diverging spread.
---

# Statistical Arbitrage And Mean-Reversion

Statistical arbitrage and mean-reversion strategies bet that a relationship between prices will return to its historical norm after diverging. The appeal is clear: the trades appear market-neutral, the edge seems mathematically grounded, and the entry rule (a z-score threshold) feels objective. The danger is that the entire premise rests on the assumption that the historical relationship is stable. When that relationship breaks, the strategy that was "mean-reverting" becomes a losing position that the rules keep adding to, because each wider divergence looks like a better entry.

Agents tend to under-price the risk of relationship breakdown. A cointegration test on history says the spread was stable in the past; it does not say it will remain stable. Companies are acquired, restructured, delisted, or fundamentally altered; pairs that cointegrated for years can diverge permanently. The judgment problem is deciding when a mean-reversion trade has genuine edge, when the spread has broken for fundamental reasons, and how to control risk so a single broken relationship cannot destroy the portfolio.

This skill applies to pair trading, stat-arb strategy evaluation, cointegration analysis, z-score and half-life estimation, and spread risk management. It is not investment advice; statistical relationships in markets break without warning, mean-reversion strategies suffer tail losses, and historical cointegration does not guarantee future stability.

## Core Rules

### Confirm Cointegration, Not Just Correlation

Correlation between two price series is not sufficient for pairs trading. Two assets can be highly correlated because they share a trend yet diverge without bound. Cointegration is the relevant property: a stable long-run equilibrium relationship such that the spread between the series is stationary (mean-reverting). Only cointegrated pairs support a mean-reversion trade.

Test cointegration formally (Engle-Granger or Johansen) and confirm the spread passes stationarity tests. Even then, treat the result as evidence about the past, not a guarantee. Cointegration can break, and tests have low power in short samples. Require both statistical evidence and an economic reason the two assets should be tied (same industry, shared inputs, dual-class shares, index arbitrage).

### Demand An Economic Driver Behind The Relationship

A statistical relationship without an economic basis is fragile. Strong pairs have a structural reason to move together: companies in the same commodity-linked business, a parent and subsidiary, an ADR and its home listing, two share classes of one company. When the economic link is clear, divergence is more likely to be temporary (a mispricing) than permanent (a regime change).

Weak pairs are those selected purely by statistical fit during backtesting, with no obvious reason they should track. These are the most likely to break. Prefer pairs where you can articulate why the spread should revert and what would cause it to break.

### Estimate Half-Life And Match It To Holding Period

The half-life of mean reversion is how long, on average, a deviation takes to halve. It determines the trade's expected holding period and turnover. A spread with a half-life of two days is a high-frequency trade; a half-life of two months is a different strategy with different costs and capital requirements.

Estimate half-life and ensure the strategy's rebalancing and holding assumptions match it. If the half-life is long, the trade ties up capital and incurs financing costs for extended periods. If half-life estimates are unstable or very long, the mean-reversion edge is weak and the trade is uneconomic after costs.

### Set Hard Stop Losses For Spread Breakdown

The defining risk of mean-reversion is the spread that never reverts. Rules that scale into a widening spread (each z-score increment adds size) are the most dangerous: they turn a statistical anomaly into a portfolio-destroying loss if the relationship has broken. Averaging down into a permanently diverging pair is how many stat-arb desks blew up.

Define, in advance, the spread level at which the thesis is declared broken and the position is closed at a loss. This stop must be based on recognizing that the relationship may have changed, not merely on pain tolerance. Without a hard exit for breakdown, a mean-reversion strategy has unbounded risk on the downside it claims doesn't exist.

### Monitor For Structural Change In The Relationship

Cointegration is not eternal. Corporate actions (mergers, spinoffs, bankruptcies, index changes), regulatory shifts, business model changes, and idiosyncratic shocks can sever a previously stable link. A spread that has diverged far may reflect a fundamental break, not a mispricing.

Monitor for news and structural events in the underlying names, not just the spread's z-score. If a fundamental change has occurred (one company gained a major contract, lost a key patent, faces litigation), the historical relationship may no longer apply. Re-test cointegration on rolling windows and reduce or exit when stability deteriorates.

### Account For Costs, Financing, And Shorting Constraints

Stat-arb involves both long and short positions, which introduces borrowing costs, short squeeze risk, and margin requirements. The apparent edge often shrinks dramatically after accounting for stock loan fees, bid-ask spreads, commissions, and the capital tied up. Many historically profitable pairs become uneconomic for non-institutional traders once full costs are included.

Model all costs: commissions, spreads, short rebate (often negative for hard-to-borrow names), margin interest, and capital opportunity cost. A strategy with a small statistical edge per trade can be net-negative after costs, especially with high turnover. Hard-to-borrow names can be recalled or subject to buy-ins that force closure at adverse prices.

### Size For Diversification Across Many Pairs

A single pair can break and produce a large loss. The economic justification for stat-arb is the law of large numbers across many independent pairs: individual breakdowns are absorbed by the portfolio. Concentration in a few pairs defeats this and exposes the strategy to idiosyncratic relationship failure.

Ensure the portfolio holds many pairs with low correlation between their breakdown risks. Avoid pairs that share a common driver (all in one sector, all dependent on one commodity), because a regime shift can break them simultaneously. Diversification across pairs, sectors, and relationship types is the structural risk control.

## Common Traps

### Trading Correlated But Non-Cointegrated Pairs

High correlation tempts traders into pairs that are not cointegrated. The trap is that correlated trending assets can diverge without bound, producing unbounded losses on a "mean-reversion" trade. Always confirm cointegration and spread stationarity.

### Averaging Into A Broken Spread

Scaling into a widening spread feels disciplined (each level is a "better entry") but is catastrophic if the relationship has broken. The trap is that the strategy's own logic prevents recognizing breakdown. Hard stops based on structural change are essential.

### Overfitting Pair Selection To History

Selecting the pairs that cointegrated best over a backtest period introduces selection bias; those pairs may have been the luckiest, not the most robust. The trap is deploying many overfit pairs that fail forward. Use out-of-sample selection and require economic logic.

### Ignoring Shorting Costs And Constraints

The short leg incurs borrow costs, can be hard to locate, and can be bought in. The trap is modeling the strategy as if shorts are free and always available. Hard-to-borrow and short squeeze risk can turn a paper profit into a realized loss.

### Assuming Half-Life Is Stable

Half-life estimates are noisy and regime-dependent. The trap is sizing and timing the trade on a point estimate that may be far off. Use confidence intervals and stress-test for slower reversion than expected.

### Concentrating In One Sector Or Theme

Pairs in the same sector share breakdown risk. A regulatory or commodity shock can break them all at once. The trap is counting them as diversified when they share a common driver. Diversify across uncorrelated relationship types.

### Believing Market-Neutral Means Risk-Free

Stat-arb is market-neutral to directional beta but carries relationship-breakdown risk, factor risk, liquidity risk, and short squeeze risk. The trap is sizing as if the only risk is waiting for reversion. Size for the tail risk of broken relationships.

## Self-Check

- [ ] Cointegration (not just correlation) is confirmed statistically, and the spread is shown to be stationary.
- [ ] An economic driver explains why the pair should track, and what would break the link is articulated.
- [ ] Half-life is estimated and the strategy's holding period, turnover, and costs are consistent with it.
- [ ] A hard stop defines when the relationship is declared broken; the strategy does not scale indefinitely into a diverging spread.
- [ ] Structural change (corporate actions, fundamentals, news) is monitored, not just the z-score.
- [ ] All costs are modeled: commissions, spreads, borrow costs, margin, and capital opportunity cost.
- [ ] The portfolio is diversified across many pairs with low common breakdown risk, not concentrated in one sector or theme.
- [ ] Shorting constraints (hard-to-borrow, recall, buy-in risk) are considered.
- [ ] Pair selection avoids backtest overfitting; out-of-sample robustness is checked.
- [ ] The conclusion is probabilistic and notes relationships can break without warning and historical cointegration does not guarantee future stability; it is not personalized advice.
