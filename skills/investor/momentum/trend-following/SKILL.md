---
name: trend_following.md
description: Use when the agent is designing or evaluating a trend-following strategy using moving averages, breakout signals, or time-series momentum, assessing the risk of false signals and whipsaw, managing position sizing and stop-loss discipline, understanding the return distribution and drawdown profile of trend following, and weighing the behavioral and cost challenges of a systematic trend approach.
---

# Trend Following

Trend following is a systematic strategy that seeks to capture sustained price movements by identifying and riding established trends, typically using moving average crossovers, breakout signals, or time-series momentum. The thesis is that trends exist due to slow information diffusion, behavioral biases, and institutional flows, and that a disciplined, rule-based approach can capture a portion of them while controlling risk with exits. The judgment problem is that trend following is psychologically brutal and statistically uneven. The strategy generates many small losses from false signals and whipsaws in choppy markets, and makes its returns from a few large, sustained trends. Most investors understand the logic, implement it, and then abandon it after a string of whipsaw losses, precisely when the strategy is working as designed. The discipline is to execute the rules consistently, size positions for the drawdown path, and survive the inevitable dry spells to capture the rare large trends.

This skill is for designing and maintaining trend-following strategies with discipline about false signals, risk control, and behavior.

## Core Rules

### Choose Signals With Awareness Of Their Tradeoffs

Trend-following signals differ in responsiveness and noise. The choice of signal shapes the strategy's behavior.

Compare signal types:

- moving average crossovers (e.g., 50-day over 200-day), which are simple and robust but lag, entering trends late and exiting late;
- breakout signals (e.g., entering on new highs over a lookback period), which capture moves early but generate many false signals in range-bound markets;
- time-series momentum (going long if the asset is up over a lookback), which is academically grounded and captures the core trend effect;
- the lookback period, which trades off responsiveness (shorter) versus robustness (longer).

No signal is optimal in all environments. The choice should reflect the investor's time horizon, the asset's characteristics, and tolerance for whipsaw. Robustness across parameter variations is more important than optimizing on a single backtest.

### Anticipate And Accept Whipsaw And False Signals

The defining cost of trend following is whipsaw: false signals that generate small losses in choppy, range-bound markets. These are not failures; they are the cost of capturing trends.

Understand:

- the majority of individual signals will be losers, since trends are rare relative to chop;
- the strategy's positive expectancy comes from a small number of large winning trends that outweigh the many small losses;
- whipsaw clusters in low-volatility, range-bound markets where trends fail to develop;
- the hit rate (percentage of winning trades) is often below 50%, which is psychologically difficult;
- the investor must accept the whipsaw as the price of admission, not try to eliminate it.

Attempting to filter out all false signals usually also filters out the large trends, destroying the strategy's edge. Acceptance of whipsaw is central to trend following.

### Implement Strict Risk Control And Position Sizing

Because individual trades have low hit rates and trends are rare, risk control and position sizing determine survival.

Apply:

- a defined risk per trade (e.g., risking a small percentage of capital on each signal), so that a string of losses is survivable;
- stop-loss orders or trailing stops to exit losing trades and let winners run;
- volatility-based position sizing, so that more volatile assets take smaller positions to equalize risk;
- a maximum portfolio risk limit, accounting for correlation across simultaneous positions;
- the recognition that trend following's edge is in the distribution (fat-tailed winners), not in the average trade.

The strategy survives long enough to capture the large trends only if position sizing prevents any single trade or cluster of losses from being catastrophic.

### Understand The Return Distribution And Drawdown Profile

Trend-following returns are not normally distributed. They have negative skew in many individual trades but positive skew at the portfolio level over time, with most returns coming from rare large wins.

Characterize:

- the low hit rate and the distribution of trade outcomes (many small losses, few large gains);
- the drawdown profile, including extended periods of flat or negative returns during trendless markets;
- the positive skew at the portfolio level, where a few strong-trend years produce the bulk of long-term returns;
- the interaction with volatility, since trend following tends to do well in high-volatility, trending environments and poorly in low-volatility, choppy environments.

The investor must be prepared for extended dry spells and must size the allocation so that drawdowns do not force abandonment.

### Diversify Across Markets And Timeframes

Trend following benefits from diversification across uncorrelated markets and timeframes, since trends occur at different times in different assets.

Diversify:

- across asset classes (equities, bonds, commodities, currencies), since trends are idiosyncratic;
- across geographies and sectors within equities;
- across timeframes (combining faster and slower signals), to capture trends of different durations;
- the benefit is that whipsaw in one market is offset by trends in another, smoothing portfolio returns.

A diversified trend-following portfolio has a far more tolerable drawdown path than a single-market strategy, which is essential for the behavioral sustainability of the approach.

### Maintain Behavioral Discipline Through Rules And Pre-Commitment

The greatest threat to trend following is the investor's own behavior. The strategy requires taking signals after losses, holding losers until stopped, and letting winners run, all of which are emotionally difficult.

Build defenses:

- a fully systematic, rule-based approach that removes discretion from signal execution;
- pre-commitment to the rules, including taking every signal and not second-guessing exits;
- an understanding that the strategy is designed for the long run, not for any individual trade;
- regular review of process adherence, not of short-term performance;
- acceptance that underperformance in trendless markets is expected, not a sign of failure.

The investor who deviates from the rules, skips signals after losses, or cuts winners early will capture the whipsaw cost without the trend benefit, guaranteeing poor results.

## Common Traps

### Optimizing Signals On A Single Backtest

Over-fitting parameters to historical data produces a strategy that fails out of sample. Robustness across parameters matters more than peak backtest performance.

### Trying To Eliminate Whipsaw

Filtering out all false signals also filters out the large trends. Whipsaw is the cost of the edge.

### Poor Position Sizing

Risking too much per trade or ignoring correlation leads to ruin during clusters of losses. Sizing is survival.

### Cutting Winners Early

Trend following depends on rare large wins. Taking profits early removes the positive skew that drives returns.

### Abandoning The Strategy After Losses

Quitting after a string of whipsaws captures the cost and misses the subsequent trends. Behavioral discipline is the strategy.

### Neglecting Diversification

A single-market trend strategy has an intolerable drawdown path. Diversification across markets and timeframes is essential.

### Confusing Trend Following With Prediction

Trend following does not forecast direction; it reacts to established trends. Treating it as prediction leads to discretion and deviation.

## Self-Check

- [ ] The signal type (moving average, breakout, time-series momentum) and lookback period are chosen with awareness of responsiveness-versus-robustness tradeoffs and robustness across parameters.
- [ ] Whipsaw and false signals are anticipated and accepted as the cost of capturing trends, not treated as failures to be eliminated.
- [ ] Strict risk control is implemented, with defined risk per trade, stop-losses, volatility-based sizing, and portfolio-level risk limits accounting for correlation.
- [ ] The return distribution (low hit rate, fat-tailed winners, extended dry spells) and drawdown profile are understood, and the allocation is sized to survive them.
- [ ] The strategy is diversified across asset classes, geographies, sectors, and timeframes to smooth returns and reduce drawdown depth.
- [ ] Behavioral discipline is maintained through systematic rules, pre-commitment, process-focused review, and acceptance of trendless-market underperformance.
- [ ] The investor does not cut winners early, skip signals after losses, or deviate from the rules based on discretion.
- [ ] The guidance flags that trend following can suffer extended losses, that whipsaw is normal, that past performance does not guarantee future results, that the strategy requires behavioral resilience, and that professional advice may be warranted for systematic strategies.
