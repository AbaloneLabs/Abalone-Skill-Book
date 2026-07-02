---
name: risk_per_trade.md
description: Use when the agent is sizing positions by risk per trade, setting stop-loss levels and position sizes from a fixed risk budget, using R-multiples to define and compare trades, managing simultaneous positions and aggregate risk, or judging whether per-trade risk limits account for correlation, liquidity, gap risk, and the investor's total risk capacity.
---

# Risk Per Trade

Risk-per-trade sizing defines position size by the amount willing to be lost if the stop is hit, rather than by dollar allocation. It is the foundation of disciplined trading: every position has a defined risk (often called one R), a stop, and a target that justifies the risk. The judgment problem is that risk-per-trade is only as good as its assumptions, and agents routinely treat the stop distance as fixed and certain when in reality stops gap, correlation concentrates loss across simultaneous trades, and "1% risk" across many positions can add up to a portfolio-wrecking aggregate.

This skill is for sizing by risk per trade while respecting the gaps between theory and reality.

## Core Rules

### Define Risk Per Trade From The Stop, Not From Allocation

Risk-per-trade sizing starts with the exit, not the entry. The size is derived so that hitting the stop loses the planned risk amount.

Compute:

- the entry price and the stop level (where the thesis is wrong);
- the per-unit risk (entry minus stop, adjusted for direction);
- the position size that makes total risk equal the planned risk budget;
- the resulting dollar allocation and its share of portfolio.

This inverts the usual logic. First decide where you are wrong and how much you will lose there, then size to that, rather than buying a dollar amount and hoping the stop works.

### Set The Risk Budget From Total Capacity, Not Per Trade

The per-trade risk must fit within the investor's total risk capacity and aggregate risk limits.

Establish:

- total risk capacity (how much the portfolio can lose without ruining the plan);
- aggregate open risk across all simultaneous positions;
- a per-trade cap (often a small fraction of capital);
- a total-open-risk cap that accounts for correlation.

A common error is setting 1% per trade and holding 30 positions, creating 30% open risk that, if stops hit together, is catastrophic. Per-trade limits must roll up to an aggregate limit.

### Account For Correlation Across Simultaneous Positions

Independent per-trade risk assumes positions are uncorrelated. In reality, correlated positions stop out together in a market shock.

Adjust:

- group positions by shared factor (market, sector, currency, theme);
- cap risk per factor or theme, not only per position;
- assume correlation rises toward one in a crash when sizing aggregate risk;
- reduce total open risk when many bets share a driver.

Twenty "1% risk" tech longs are not 20 independent bets; they are one large bet on tech with gap risk on all of them.

### Define Targets And R-Multiples Before Entry

Risk-per-trade discipline pairs the risk (1R) with a target that justifies taking it. The reward-to-risk ratio (target distance over stop distance) frames whether the trade is worth it.

For each trade, state:

- the risk in R (the planned loss if stopped);
- the target in R (the planned gain if right);
- the reward-to-risk multiple;
- the probability implied by the entry to make the trade positive expectancy.

A trade risking 1R to make 1R needs a high win rate; a trade risking 1R to make 3R can be profitable with a lower win rate. Define the R-multiple before entry, not after.

### Account For Gap And Slippage Risk

Stops are not guarantees. Markets gap through stops, especially on news, opens, and illiquid names, and the realized loss exceeds the planned risk.

Build in:

- the realistic worst case if the stop gaps, not the clean stop level;
- wider buffers or smaller sizes for gap-prone (earnings, news, thin) positions;
- the use of options or hedges where gap risk is severe;
- position sizes that survive the gap scenario, not just the clean stop.

A 1% planned risk can become a 5% loss on a gap. Size so the gap scenario is survivable.

### Incorporate Liquidity And Exit Reality

Risk-per-trade assumes you can exit at the stop. In illiquid names, the exit moves the market against you.

Check:

- average volume and the size of your position relative to it;
- bid-ask spread and its widening under stress;
- the realistic fill if the stop triggers in a fast market;
- whether the position can be exited within the risk budget at all.

Large positions in thin markets have hidden risk: the act of selling worsens the price, so the realized stop is far worse than the planned one.

### Reconcile Risk-Per-Trade With Portfolio Goals

Risk-per-trade is a trading tactic. It must serve the portfolio's goals, risk capacity, and time horizon, not override them.

Verify:

- that aggregate trading risk fits within strategic risk capacity;
- that frequent stop-outs do not create death-by-a-thousand-cuts drawdown;
- that the strategy's edge, after costs, justifies the risk taken;
- that the investor can sustain a string of losses without abandoning discipline.

A technically correct risk-per-trade plan that bleeds the portfolio slowly or that the investor quits after a losing streak has failed its purpose.

## Common Traps

### Per-Trade Limits Without Aggregate Caps

1% per trade across many correlated positions creates aggregate risk far beyond capacity when stops hit together.

### Assuming Stops Fill Cleanly

Stops gap in fast markets. Realized loss routinely exceeds planned risk on news, opens, and illiquid names.

### Ignoring Correlation

Independent per-trade sizing assumes uncorrelated bets. Correlated positions stop out together, concentrating loss.

### Sizing From Allocation, Not Stop

Buying a dollar amount and then placing a stop produces arbitrary risk. Size from the stop and the risk budget.

### No Defined Target Or R-Multiple

A trade without a defined target and reward-to-risk ratio cannot be evaluated for positive expectancy. Hope is not a plan.

### Death By A Thousand Cuts

Many small losses that each respect the per-trade limit can still produce a large aggregate drawdown over time.

### Overtrading Relative To Edge

If the strategy's net edge is small, even disciplined risk-per-trade produces net losses after costs. Sizing discipline cannot rescue a negative-edge strategy.

### Ignoring Liquidity At The Exit

A position that cannot be exited at the planned stop has hidden risk that surfaces only when the stop triggers.

## Self-Check

- [ ] Position size is derived from the stop and the planned risk budget, not from a dollar allocation.
- [ ] Per-trade risk fits within total risk capacity and an aggregate open-risk cap.
- [ ] Correlation across simultaneous positions is accounted for, with factor- or theme-level risk caps.
- [ ] Each trade has a defined target and R-multiple (reward-to-risk) stated before entry.
- [ ] Gap and slippage risk are incorporated, with sizes that survive the gap scenario, not just the clean stop.
- [ ] Liquidity, average volume, bid-ask, and realistic exit fills are checked for each position.
- [ ] Aggregate trading risk fits strategic capacity, and the strategy's net edge after costs justifies the risk.
- [ ] The recommendation flags that stops can gap, that correlated losses can exceed aggregate limits, that trading involves substantial risk of loss, and that professional advice may be warranted for active strategies.
