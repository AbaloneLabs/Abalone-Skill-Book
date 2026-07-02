---
name: implied_vs_realized_volatility.md
description: Use when the agent is comparing implied volatility to realized volatility, assessing the volatility risk premium, evaluating option selling or buying strategies, interpreting IV rank or percentile, or deciding whether options are cheap or expensive. Covers IV-RV spread, variance risk premium, vol-of-vol, and the structural reasons implied vol tends to exceed realized vol.
---

# Implied Versus Realized Volatility

Implied volatility (IV) is the market's forward expectation of volatility, embedded in option prices. Realized volatility (RV) is the volatility that actually occurred. These are two different quantities, and the gap between them, the volatility risk premium, is one of the more persistent features of options markets. On average, implied volatility exceeds realized volatility, which is why systematic option selling has historically earned a premium, and also why naive option buying tends to lose money over time even when the directional view is correct.

Agents tend to treat IV as a forecast and to assume options are fairly priced. But IV is a consensus expectation that is systematically biased upward, because option buyers pay for insurance and sellers demand compensation for bearing tail risk. The judgment problem is deciding when the IV-RV spread offers a tradeable edge, when it reflects fair compensation for risk that will materialize, and how to avoid the false dichotomy of "IV is high so sell" or "IV is low so buy."

This skill applies to evaluating option strategies, the variance risk premium, IV rank and percentile, and the relative richness of options. It is not investment advice; the volatility risk premium is not guaranteed, option selling carries tail risk that can exceed the premium collected, and implied vol can spike far beyond any historical realized level.

## Core Rules

### Understand The Structural Source Of The Volatility Risk Premium

Implied vol generally exceeds realized vol because option sellers require compensation for bearing risks that option buyers wish to shed: crash risk, gap risk, and the inability to hedge during dislocations. This is analogous to an insurance premium. Buyers accept overpaying on average because the protection matters precisely in states where they need it. Sellers accept the negative skew because they collect premium over time.

Recognize that the premium is compensation for risk, not free money. It exists because sellers occasionally take large losses when realized vol spikes above implied. Any strategy that harvests the premium must survive those episodes. The long-run average positive spread is real, but it is earned by bearing tail risk that materializes infrequently and severely.

### Compare IV To RV Over A Matching Horizon

The IV-RV comparison is only meaningful when the horizons match. An option's IV reflects the market's expectation over the option's life; comparing it to 20-day historical realized vol is mismatched if the option expires in 90 days. Match the realized window to the option's remaining life, or use forward realized vol over the option's term when evaluating after the fact.

Use the same annualization and measurement conventions for both. A common error is comparing a 30-day IV to a 10-day RV and drawing conclusions about richness. Align windows, and when assessing the premium historically, compare IV at entry to realized vol over the subsequent matching period.

### Use IV Rank And Percentile With Distributional Awareness

IV rank (current IV relative to its own 52-week range) and IV percentile (percentage of days IV was lower than current) measure whether IV is high relative to its recent history. They are useful for context but are not standalone signals. High IV rank means IV has fallen before; it does not mean it will fall now, and low IV rank does not guarantee a rise.

Interpret these metrics with awareness of the underlying distribution and regime. IV can stay elevated for long periods during sustained stress, and can remain depressed for years in calm markets. A "high IV rank" in a structural vol regime may simply be the new normal. Use rank and percentile as one input alongside the absolute IV level, the IV-RV spread, and the fundamental drivers of vol.

### Distinguish Directional Vol From Pure Vol Exposure

A long or short option position combines volatility exposure with directional (delta) exposure unless delta-hedged. An option buyer who is right about vol increasing but wrong on direction may still lose if the delta drag exceeds the vega gain. To isolate the vol view, delta-hedge or use spread structures that neutralize direction.

Clarify whether the thesis is about volatility level, volatility direction, or the shape of the vol surface, and structure the trade to express that specific view. Mixing a directional view with a vol view in one position makes it impossible to know which drove the result. Pure vol trades (delta-hedged, or via vol swaps and VIX derivatives) isolate the vol component.

### Account For Vol-Of-Vol And Path Dependence

Implied volatility itself is volatile, and option pricing assumes a specific evolution of vol over the option's life. Path matters: a delta-hedged option's profit depends on how vol is realized path-by-path (gamma scalping), not just on average realized vol. High vol-of-vol increases the dispersion of option outcomes and the cost of dynamic hedging.

For dynamic hedging strategies, model the path of realized vol and the frequency of re-hedging, not just the average. For static positions, recognize that early exercise, early assignment, and barrier breaches depend on the path, not the terminal value. The gap between theoretical vol value and realized P&L is often a path effect.

### Recognize That Selling Vol Is Short Convexity

Short vol positions have negative convexity: they earn small gains most of the time and suffer large losses in tails. This produces a return profile that looks excellent in backtests dominated by calm periods and catastrophic in the few tail events. The Sharpe ratio of short vol is misleadingly high because it hides skew.

Size short vol positions for the tail loss, not the average gain. A short vol strategy that doubles the portfolio's tail exposure is dangerous even if its average return is attractive. Stress-test against historical vol spikes (2008, 2015, 2020, 2024) and against levels beyond history. The premium is only worth harvesting if the strategy can survive the episode that ends many short-vol players.

### Evaluate Whether The Current IV-RV Spread Compensates For Risk

The average IV-RV spread is positive, but it varies, and at any given time the spread may be too small to compensate for the tail risk of selling, or too compressed to make buying attractive. Evaluate the current spread relative to its own history and relative to the prevailing risk environment. A thin spread in a fragile market may not pay for the risk.

Demand a margin: only sell vol when the spread is wide enough to compensate for the probability and magnitude of a vol spike. Conversely, buying vol as a hedge may be rational even at a negative expected value if it protects against catastrophic loss. The decision is not purely about the average premium; it is about risk-adjusted compensation in the investor's specific portfolio context.

## Common Traps

### Treating IV As An Unbiased Forecast

IV is a consensus expectation biased upward by the risk premium. The trap is assuming options are fairly priced and that IV equals expected realized vol. The systematic overpricing is the premium sellers harvest and buyers pay.

### Selling Vol Based Only On "IV Is High"

High IV rank does not mean vol will revert down. The trap is selling into a regime where elevated vol is justified and persistent, then taking a large loss when it spikes further. Assess the spread and the fundamental drivers, not just the rank.

### Ignoring Negative Convexity In Short Vol

The attractive steady returns of short vol hide occasional catastrophic losses. The trap is sizing by average return and Sharpe rather than by tail loss. Short vol must be sized for the spike.

### Comparing Mismatched IV And RV Horizons

Comparing a 90-day IV to a 20-day RV produces meaningless conclusions. The trap is acting on a richness signal that is an artifact of mismatched windows. Align the horizons.

### Mixing Directional And Vol Views

An unhedged option position conflates delta and vega. The trap is attributing a vol-driven loss or gain to direction, or vice versa. Isolate the view with appropriate structuring.

### Assuming The Variance Risk Premium Is Constant

The premium varies by asset, regime, and market stress. The trap is applying a single historical average to all conditions. Evaluate the current spread, not just the long-run average.

### Underestimating Gap Risk For Option Sellers

Sellers of naked options face theoretically unlimited loss (calls) or very large loss (puts) on gaps. The trap is modeling vol as continuous and missing the jump risk. Gap risk is exactly what the premium compensates, and what sellers must survive.

## Self-Check

- [ ] The volatility risk premium is understood as compensation for bearing tail risk, not free money, and the strategy is sized to survive vol spikes.
- [ ] IV and RV are compared over matching horizons with consistent annualization.
- [ ] IV rank and percentile are interpreted with regime and distributional awareness, not as standalone sell/buy signals.
- [ ] The vol view is isolated from directional exposure via delta-hedging or appropriate spreads where a pure vol thesis is intended.
- [ ] Vol-of-vol and path dependence are considered for dynamic and barrier-dependent positions.
- [ ] Short vol positions are sized for tail loss and stress-tested against historical and beyond-historical vol spikes, not just average return.
- [ ] The current IV-RV spread is evaluated relative to its history and the risk environment, with a margin demanded before selling.
- [ ] Negative convexity of short vol is acknowledged; Sharpe ratios are not relied upon to judge tail-exposed strategies.
- [ ] Gap and jump risk for option sellers is explicitly considered.
- [ ] The conclusion is probabilistic and notes the premium is not guaranteed, vol can spike beyond history, and option selling carries catastrophic tail risk; it is not personalized advice.
