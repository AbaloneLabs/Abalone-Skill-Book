---
name: correlation_analysis.md
description: Use when the agent is measuring how assets move together, estimating correlation or covariance for diversification, checking whether diversifiers hold up in crises, analyzing tail dependence, time-varying correlation, or correlation breakdown during stress, or evaluating how reliable a diversification benefit actually is.
---

# Correlation Analysis

Correlation is the backbone of diversification, and it is also the number that most often misleads. Agents routinely cite a single average correlation, treat it as stable, and then watch diversification vanish in exactly the crash it was supposed to cushion. The judgment problem is that correlation is regime-dependent, nonlinear in the tails, and estimated with noise, so a number that looks reassuring in a spreadsheet can be worthless when it matters most.

This skill is for the moment before trusting a diversification claim. The goal is to prevent the agent from equating "low average correlation" with "real protection."

## Core Rules

### Never Trust A Single Average Correlation

A pairwise correlation averaged over years hides the structure that matters. Two assets can have low average correlation and still move together in every crisis.

Always decompose:

- correlation in calm versus stressed markets;
- correlation over rolling windows, not one fixed period;
- correlation by decade or regime, to see how it drifts;
- the dispersion of correlation, not only its mean.

A diversifier with low average correlation but high stressed correlation is a false diversifier. Report the stressed number, not the average.

### Examine Tail Dependence, Not Only Linear Correlation

Linear correlation captures the middle of the distribution and misses the tails. Many assets are weakly correlated on ordinary days and tightly linked on extreme down days.

Use tail-focused measures where possible:

- conditional correlation in the worst return buckets;
- co-exceedance, how often both assets fall together in extreme moves;
- tail dependence coefficients (upper and lower);
- rank-based measures (Spearman, Kendall) that are less sensitive to outliers in the center.

The relevant question for risk control is usually "what happens in the worst weeks," not "what is the average relationship." Tail dependence answers the first; average correlation answers the second.

### Treat Correlation As Time-Varying

Correlations shift with the dominant risk factor. Stocks and bonds diversify in a growth-shock regime and fall together in an inflation-shock regime.

Check:

- what macro factor currently drives the correlation;
- how the correlation changed across past regime shifts;
- whether the current regime is the one the sample was drawn from;
- leading indicators of a regime change (rates, inflation, volatility, credit spreads).

A correlation assumption valid for one regime is dangerous in another. Name the regime and test the alternative.

### Account For Estimation Error And Noise

Correlation estimates are noisy, especially over short windows and for volatile assets. A small sample can produce a reassuring number by chance.

Consider:

- the sample length and whether it spans multiple cycles;
- confidence intervals around the estimate;
- the effect of a few outlier observations;
- shrinkage or robust estimators that reduce noise.

Do not present a point estimate as if it were exact. Two assets with "0.2 correlation" and "0.4 correlation" may be statistically indistinguishable.

### Distinguish Correlation From Causation And Common Factors

Low correlation between two assets can hide a shared exposure to a third factor. When that factor moves, both fall.

Ask:

- what common risk factors drive both assets (rates, dollar, growth, credit, volatility);
- whether the low correlation is structural or incidental;
- what happens if the common factor reverses.

Factor-based diversification (balancing risk factors, not just asset labels) is usually more robust than label-based diversification.

### Stress The Diversifier In Its Worst Case

The real test of a diversifier is its behavior when the core portfolio is already stressed. Test the joint distribution, not the marginal.

Run:

- historical crisis replays (2008, 2020, rate shocks, inflation spikes);
- conditional analysis where the core portfolio is down materially;
- scenario shocks to the shared factors;
- correlation-to-one stress, where diversification partially or fully fails.

If the diversifier fails in the same scenarios that hurt the core, it adds complexity without protection.

### Connect Correlation To Portfolio Impact, Not Just Statistics

A correlation number is only useful insofar as it changes portfolio risk. Translate it.

Compute:

- the portfolio volatility with and without the diversifier;
- the expected drawdown reduction in stress;
- the diversification benefit net of the diversifier's own risk and cost;
- whether the benefit justifies the complexity, fees, and liquidity cost.

A diversifier that lowers portfolio volatility by a tiny amount while adding cost and opacity is not worth it, regardless of its attractive correlation.

## Common Traps

### Citing Average Correlation As If It Were Stable

The single most common error. Average correlation hides the stressed-correlation breakdown that destroys diversification in crashes.

### Assuming Stock-Bond Diversification Is Permanent

Stocks and bonds have diversified for much of the recent era, but they have fallen together in inflation and rate-shock regimes. Treating the relationship as a law invites a nasty surprise.

### Ignoring Tail Dependence

Low linear correlation with high tail dependence means the diversifier helps on ordinary days and fails on the worst ones, the opposite of what is needed.

### Overfitting To One Historical Sample

A correlation estimated from one calm decade will not hold in a different regime. Backtests built on it are optimistic.

### Counting Many Weakly-Correlated Equity Sleeves As Diversification

Emerging markets, small-caps, sector equity, and private equity may all be equity beta with correlations that spike to near one in a crash. Label diversity is not risk diversity.

### Confusing Low Correlation With Positive Expected Return

A diversifier can be uncorrelated and still lose money on its own. Diversification reduces risk; it does not create return. A bad diversifier can drag on the portfolio while failing in stress.

## Self-Check

- [ ] Correlation is reported with stressed, rolling, and regime decomposition, not a single average.
- [ ] Tail dependence or conditional-in-stress correlation was examined, not only linear correlation.
- [ ] The current regime and the factor driving the correlation were identified, and an alternative-regime test was run.
- [ ] Estimation error, sample length, and confidence around the correlation estimate were acknowledged.
- [ ] Common hidden factors behind the assets were identified and stress-tested.
- [ ] The diversifier was tested in scenarios where the core portfolio is already stressed, including correlation-to-one.
- [ ] The correlation finding was translated into portfolio impact (volatility, drawdown, net of cost), not left as a raw statistic.
- [ ] The analysis avoids presenting diversification as reliable without flagging regime risk, tail dependence, and that historical correlations may not hold, and that professional advice may be warranted for complex portfolios.
