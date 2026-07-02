---
name: var_and_expected_shortfall.md
description: Use when the agent is estimating or interpreting Value at Risk or Expected Shortfall, choosing confidence levels and time horizons, comparing parametric, historical, and Monte Carlo VaR methods, or judging the limitations of VaR and ES including tail risk beyond the threshold, model risk, fat tails, and the danger of treating a probabilistic risk number as a maximum loss.
---

# VaR And Expected Shortfall

Value at Risk (VaR) and Expected Shortfall (ES, also called Conditional VaR) are the standard probabilistic measures of downside risk. VaR answers "how bad can the bad outcomes get at a given confidence level," while ES answers "given that we breach the VaR threshold, how bad is the average of the worse outcomes." The judgment problem is that these are statistical estimates with deep limitations: they depend on method and assumptions, they say nothing about the tail beyond the threshold, and they are routinely misread as "the maximum we can lose," which is precisely the error that breeds false confidence before a crisis.

This skill is for using VaR and ES with honest awareness of what they do and do not say.

## Core Rules

### Understand What Each Measure Does And Does Not Say

The two measures answer different questions, and confusing them is dangerous.

Know:

- VaR(X%, T) is the loss that is exceeded with probability X% over horizon T. It says nothing about how much worse the losses beyond it are.
- ES (or CVaR) is the expected loss given that the VaR threshold is breached. It captures the average severity in the tail and is a coherent risk measure, unlike VaR.

VaR tells you where the tail begins; ES tells you how bad the tail is on average. Use ES when tail severity matters, because a portfolio can have identical VaR but very different ES.

### Choose Confidence Level And Horizon Deliberately

The two parameters determine the number entirely. Different choices produce non-comparable figures.

Decide:

- confidence level (95%, 99%, 99.5%), higher means rarer but more uncertain estimates;
- time horizon (1 day, 10 days, 1 month), matched to the holding and liquidation period;
- whether the horizon accounts for the time to reduce or hedge the position;
- consistency across portfolios and periods for comparability.

A 99% one-day VaR and a 95% one-month VaR are not comparable. Always state both parameters with the number.

### Pick The Method With Its Tradeoffs

Three main methods, each with strengths and weaknesses.

Parametric (variance-covariance):

- fast and simple;
- assumes a distribution (often normal), which underestimates fat tails;
- poor for nonlinear positions (options, convexity).

Historical simulation:

- uses actual past returns, captures some fat tails;
- only as good as the sample, blind to scenarios not in the history;
- assumes the past is representative of the future.

Monte Carlo:

- flexible, can model fat tails, path dependence, and nonlinearities;
- only as good as the assumed distribution and model;
- computationally heavier and model-risk-laden.

Match the method to the portfolio. Normal-assumption parametric VaR is dangerous for option-laden or fat-tailed portfolios.

### Do Not Treat VaR As A Maximum Loss

The most dangerous misuse is reading VaR as "the most we can lose." It is a threshold, not a cap. Losses beyond VaR happen, and in the very crises VaR is meant to warn about, they can be far worse.

Always pair VaR with ES, and report the tail beyond the threshold. Make explicit that losses worse than the VaR level occur with the stated probability, and that the worst of those can be multiples of the VaR.

### Account For Fat Tails And Non-Normality

Financial returns are not normal. They have negative skew and excess kurtosis, and crises produce moves that normal models deem virtually impossible.

Address:

- use methods that capture fat tails (historical with crisis data, Student-t, extreme value theory);
- stress the assumption by comparing normal VaR to fat-tailed VaR;
- report skew and kurtosis alongside VaR;
- recognize that parametric normal VaR systematically understates tail risk.

A normal VaR that says a 2008-style move is a once-in-10,000-years event is a model failure, not reassurance.

### Acknowledge Model Risk And Estimation Error

VaR and ES are estimates with wide confidence intervals, especially in the tails where data is sparse.

Be explicit about:

- the standard error of the estimate, which is large for high confidence levels;
- sensitivity to the sample period and method;
- the breakdown of relationships (volatility, correlation) in crises;
- the gap between model and reality for unprecedented scenarios.

Report VaR as a range with caveats, not a precise point. Two methods can produce materially different numbers for the same portfolio.

### Incorporate Liquidity And Path Effects

Standard VaR assumes positions can be liquidated at the horizon. In a crisis, liquidity evaporates and the realized loss exceeds the mark.

Adjust:

- use a longer horizon for less liquid positions;
- add liquidity horizons per asset class (the Basel approach);
- model funding and margin stress for leveraged positions;
- account for bid-ask widening and market impact in a sell-off.

A 10-day VaR that assumes clean liquidation underestimates the loss for an illiquid position that takes weeks to sell into a falling market.

### Use VaR As One Input, Not The Verdict

VaR and ES are useful for comparing portfolios, setting limits, and flagging concentrations. They are not sufficient as a sole risk measure.

Combine with:

- stress testing and scenario analysis;
- drawdown and worst-path analysis;
- factor and concentration decomposition;
- qualitative judgment about regime and structural risks.

No single number captures portfolio risk. VaR is a complement to, not a replacement for, broader risk judgment.

## Common Traps

### Reading VaR As The Maximum Loss

VaR is a threshold, not a cap. Losses beyond it occur, and in crises they can be catastrophic.

### Normal-Assumption VaR For Fat-Tailed Portfolios

Parametric normal VaR systematically understates tail risk and can declare extreme moves nearly impossible.

### Comparing VaRs With Different Parameters

A 99% VaR and a 95% VaR, or one-day and one-month horizons, are not comparable. Parameters must be stated and matched.

### Ignoring Expected Shortfall

Two portfolios with identical VaR can have very different tail severity. ES captures what VaR hides.

### Blindness To Scenarios Outside The Sample

Historical VaR only sees crises already in the data. The next crisis may have a new shape.

### Assuming Liquidation At The Horizon

Standard VaR ignores the liquidity gap. Illiquid positions realize losses far beyond the horizon's mark.

### False Precision In The Tail

High-confidence VaR estimates have huge standard errors. Presenting them as precise invites overconfidence.

### Method Shopping

Choosing the method that produces the lowest VaR to make risk look small is a form of self-deception.

## Self-Check

- [ ] VaR and ES are both reported, with ES used to capture tail severity beyond the VaR threshold.
- [ ] Confidence level and time horizon are stated explicitly and matched across comparisons.
- [ ] The method (parametric, historical, Monte Carlo) is chosen with its tradeoffs understood, and normal-assumption VaR is avoided for fat-tailed or nonlinear portfolios.
- [ ] The number is explicitly framed as a threshold, not a maximum loss, with the tail beyond it described.
- [ ] Fat tails, skew, and kurtosis are addressed, and fat-tailed methods are compared to normal VaR.
- [ ] Model risk, estimation error, sample sensitivity, and relationship breakdown are acknowledged, and VaR is reported as a range.
- [ ] Liquidity horizons, funding and margin stress, and liquidation difficulty are incorporated for illiquid or leveraged positions.
- [ ] VaR and ES are used alongside stress testing, drawdown analysis, and qualitative judgment, not as the sole risk measure.
- [ ] The recommendation avoids presenting VaR as a safety guarantee, acknowledges that extreme losses can exceed any model, and notes that professional risk expertise may be warranted for complex or leveraged portfolios.
