---
name: kelly_criterion.md
description: Use when the agent is sizing a position using the Kelly criterion or fractional Kelly, estimating edge and win probability for optimal bet sizing, accounting for estimation error and parameter uncertainty in Kelly, or judging whether a Kelly-derived size is survivable after transaction costs, correlation across bets, and behavioral tolerance for its volatility.
---

# Kelly Criterion

The Kelly criterion calculates the bet size that maximizes long-term wealth growth given a known edge. Mathematically elegant and theoretically optimal, it is also one of the most misused tools in investing. The problem is that Kelly assumes you know the true probabilities and payoffs, which investors never do; it produces extreme volatility and large drawdowns along the way; and full Kelly is intolerable for almost everyone. Agents often apply the formula to shaky estimates, take the result at face value, and ignore that estimation error and behavioral reality make full Kelly dangerous.

This skill is for using Kelly as a disciplined reference point, not a literal sizing rule.

## Core Rules

### Understand What Kelly Optimizes And At What Cost

Kelly maximizes the expected logarithm of wealth, which is the long-term growth rate. It is optimal in the very long run under known probabilities.

Know the tradeoffs:

- full Kelly maximizes growth but produces high volatility and deep drawdowns;
- the growth-optimal property holds only with exact inputs and infinite time;
- a bet above full Kelly has lower growth and higher risk, a dominated choice;
- a bet below full Kelly (fractional Kelly) trades some growth for far less volatility and drawdown.

Kelly is a ceiling on sensible size, not a target. Betting more than Kelly is mathematically dominated; betting a fraction of Kelly is usually the rational real-world choice.

### Demand Honest Estimates Of Edge And Probability

Kelly's output is only as good as its inputs, and the inputs are the hardest part.

Estimate:

- the probability of each outcome (win, loss, partial);
- the payoff ratio (how much you win versus lose);
- the edge, expected value per unit risked;
- the uncertainty in all of these estimates.

Most investors overestimate their edge. A Kelly size computed from an overestimated edge is an overbet. Be conservative and skeptical about the edge estimate, because the formula amplifies input errors into sizing errors.

### Apply Fractional Kelly To Handle Estimation Error

Because the true probabilities are never known, full Kelly is too aggressive. Fractional Kelly (half-Kelly, quarter-Kelly) is the standard practical adjustment.

Use fractional Kelly because:

- it reduces the impact of estimation error;
- it sharply reduces volatility and drawdown for a modest growth cost;
- it accommodates the gap between model and reality;
- it reflects that most real edges are smaller and noisier than estimated.

Half-Kelly gives about three-quarters of the optimal growth rate at half the volatility. For most investors, that trade is clearly worth it.

### Account For Multiple Simultaneous Bets And Correlation

Kelly for a single bet ignores that investors hold many positions at once. Correlated bets concentrate risk in ways single-bet Kelly misses.

Adjust:

- compute Kelly for the portfolio, not each bet in isolation;
- account for correlation among positions, which reduces effective diversification;
- cap total risk across simultaneous bets;
- recognize that many "independent" bets share hidden factors.

A portfolio of ten half-Kelly bets that are all correlated to the same factor is effectively one large, over-Kelly bet on that factor.

### Incorporate Transaction Costs And Taxes

Kelly assumes frictionless betting. Real trades incur costs that erode the edge.

Net out:

- transaction costs and bid-ask spreads;
- the cost of rebalancing and re-establishing positions;
- tax impact of realizing gains;
- slippage, especially for larger sizes.

An edge that supports full Kelly gross may support only fractional Kelly net. The net edge is what the formula should use.

### Respect Survivability And Behavioral Tolerance

Kelly drawdowns can be brutal. Even half-Kelly can produce drawdowns that cause investors to abandon the strategy at the worst time.

Check:

- the expected drawdown path at the chosen fraction;
- whether the investor can hold through the worst likely drawdown;
- whether leverage or funding is required and its risk;
- the point at which the strategy must be abandoned to avoid ruin.

A Kelly-optimal strategy that the investor quits after a 40% drawdown captures none of its long-run optimality. Size for the investor's actual behavioral tolerance, which usually means well below full Kelly.

### Treat Kelly As A Sanity Check, Not A Verdict

Kelly is most useful as a reference point: it tells you the direction and rough magnitude of sizing given an edge, and it warns against overbetting.

Use it to:

- confirm that a proposed size is below the Kelly ceiling;
- compare relative sizing across bets with different edges;
- detect overbetting when a position exceeds Kelly;
- frame the tradeoff between growth and volatility.

Do not treat the Kelly number as a precise prescription. It is a disciplined starting point modified by estimation error, correlation, cost, and behavior.

## Common Traps

### Full Kelly On Overestimated Edges

Most edges are smaller than believed. Full Kelly on an overestimated edge is an overbet that raises ruin risk.

### Ignoring Estimation Error

Kelly assumes known probabilities. Real probabilities are uncertain, and the formula amplifies that uncertainty into sizing.

### Single-Bet Kelly For A Portfolio

Computing Kelly per position ignores correlation. A portfolio of individually-Kelly-sized correlated bets is collectively overbet.

### Betting Above Kelly

Betting more than the Kelly fraction is mathematically dominated: lower growth and higher risk. It is never justified by the model.

### Ignoring Costs And Taxes

Gross-edge Kelly overstates sensible size. Net-of-cost edge is often a fraction of gross.

### Underestimating Drawdown Volatility

Full Kelly drawdowns are severe. Investors who cannot hold them capture none of the growth benefit.

### Treating Kelly As Precise

Kelly outputs are sensitive to small input changes. Presenting a precise Kelly fraction is false precision.

### Confusing Growth-Optimal With Goal-Optimal

Kelly maximizes long-run growth, not the probability of meeting a specific goal or avoiding a specific loss. A goal-based investor may rationally size below Kelly.

## Self-Check

- [ ] Kelly is used as a ceiling and reference point, with fractional Kelly (typically half or less) as the practical sizing rule.
- [ ] Edge, win probability, and payoff estimates are conservative and acknowledge uncertainty.
- [ ] Estimation error is handled by sizing below full Kelly, with the fraction reflecting confidence in the edge.
- [ ] Multiple simultaneous bets and their correlation are accounted for at the portfolio level, not per bet in isolation.
- [ ] Transaction costs, spreads, rebalancing, taxes, and slippage are netted out of the edge before sizing.
- [ ] The expected drawdown path at the chosen size is survivable and behaviorally tolerable, and no position exceeds the Kelly ceiling.
- [ ] Kelly is treated as a sanity check and relative-sizing tool, not a precise prescription.
- [ ] The recommendation flags that Kelly depends on unknowable inputs, that overbetting raises ruin risk, and that professional advice may be warranted for leveraged or concentrated sizing decisions.
