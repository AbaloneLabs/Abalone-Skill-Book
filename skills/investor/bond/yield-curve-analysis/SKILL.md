---
name: yield_curve_analysis.md
description: Use when the agent is interpreting the Treasury yield curve, assessing normal versus inverted versus flat shapes, discussing term premium, using the curve as a recession or stagflation indicator, or forming duration and curve-positioning strategy from curve shape.
---

# Yield Curve Analysis

The yield curve is the set of yields across maturities for bonds of the same credit quality, most commonly US Treasuries. Its shape encodes market expectations about growth, inflation, and monetary policy, plus a term premium for holding longer maturities. But the curve is a noisy, composite signal, and reading it as a simple recession oracle leads to overconfident and often wrong conclusions.

Use this skill before answering questions such as "what does an inverted yield curve mean", "is a recession coming based on the curve", "should I buy long bonds when the curve is inverted", or "how should I position duration given the curve shape". The goal is to prevent the agent from reducing a rich, ambiguous signal to a single headline rule, and to prevent curve-based trades from being presented as low-risk predictions.

## Core Rules

### Decompose What The Curve Is Pricing

The yield at each maturity is not a pure forecast. It blends three things:

- expected average future short rates over the bond's life (the expectations component);
- a term premium: the extra yield investors demand for accepting interest-rate risk at long maturities;
- liquidity, supply, and technical effects from issuance, central bank buying, regulation, and safe-asset demand.

A flat or inverted curve can reflect falling rate expectations, a compressed term premium, or both. Before interpreting the shape, ask which forces are driving it. A curve inversion caused by a collapsing term premium (heavy safe-asset demand) carries a different message than one caused by sharply falling rate expectations.

### Choose The Right Spread For The Question

Different parts of the curve answer different questions, and they do not always agree.

- 2s10s (2-year versus 10-year): the classic recession-watch spread, but noisy and affected by front-end policy.
- 3-month versus 10-year: the spread the historical recession literature relies on most; often inverts earlier and later than 2s10s.
- 2s5s, 5s10s: isolate near-term policy expectations versus longer-term growth/inflation views.
- Real yield curve (from TIPS) versus nominal: separates inflation expectations from real-rate expectations.

Do not cite "the curve is inverted" without specifying which spread, and do not treat disagreement between spreads as meaningless — it is informative.

### Treat The Curve As A Probability Signal, Not A Guarantee

An inverted curve has historically preceded most US recessions, but:

- the lead time varies widely (months to over a year);
- some inversions are not followed by recession;
- the curve can re-steepen during the recession itself as the front end falls;
- causation is debated — inversion may reflect expected easing rather than causing slowdown.

State the signal probabilistically. Avoid "a recession will begin on date X because the curve inverted." The honest framing is that curve inversion raises the odds of weaker growth ahead, with an uncertain and variable lag.

### Account For Quantitative Easing And Term Premium Distortion

Central bank asset purchases, quantitative tightening, regulation-driven bank demand, foreign reserve demand, and pension liability-driven buying all compress or distort term premium. In such regimes the curve may be flatter or more inverted than a "pure" expectations reading would suggest, weakening its predictive content. A curve that is flat because term premium is structurally compressed is not the same signal as a curve that is flat because the market expects aggressive rate cuts.

### Use Curve Shape To Inform, Not Replace, Strategy

Curve shape can inform positioning, but it should be combined with the investor's objective and risk tolerance.

- Steepening curve: long-duration bonds may underperform if the back end rises; barbell versus bullet choices matter.
- Flattening/inverted curve: holding long bonds sacrifices little yield while taking more rate risk; the risk/reward of extending duration worsens.
- Bull steepener (front falls faster): benefits front-end roll and reinvestment.
- Bear flattener (front rises faster): hurts short-duration income strategies.

Do not present a curve-based trade as a free lunch. Curve trades (steepeners, flatteners) can lose money even when the eventual shape change is correct, due to carry, path, and timing.

### Separate Nominal, Real, And Breakeven Signals

The nominal curve mixes real-rate expectations and inflation expectations. Decomposing it reveals more:

- TIPS real yields show the real cost of capital across maturities.
- Breakeven inflation (nominal minus real) shows the inflation path the market prices.
- A flat nominal curve with rising breakevens and falling real yields points toward stagflation risk — weak real growth with sticky inflation.

Stagflation is a regime where the nominal curve alone misleads: it may look flat or inverted while real yields are deeply negative and inflation expectations are unanchored.

### Consider Currency And Sovereign Differences

Yield curves differ across countries. Comparing them requires adjusting for currency hedging costs, inflation differentials, credit quality, and central bank regimes. A "steep" curve in an emerging market may reflect default and inflation risk, not growth optimism. Do not read foreign curves through a US-centric lens.

## Common Traps

### Treating One Spread As "The" Curve

Headlines usually quote 2s10s, but the recession signal is stronger and more consistent in the 3-month/10-year spread. Conclusions can flip depending on which spread is chosen.

### Confusing Inversion With Immediate Recession

Inversion is a leading indicator with a long, variable, and uncertain lag. Acting as if recession starts the day after inversion leads to premature defensive positioning and missed returns.

### Ignoring Term Premium

If term premium is near zero or negative due to QE and safe-asset demand, the curve's shape says less about growth expectations than in normal regimes. Reading it the same way overweights the signal.

### Chasing The Curve Trade Without Carry

Steepeners and flatteners have carry and roll-down. A curve trade can be directionally right and still lose money if the move is slow and carry is negative. Path matters.

### Reading A Foreign Curve Like The US Curve

US Treasury recession-prediction literature does not transfer cleanly to other sovereigns with different inflation histories, central bank credibility, and default risk.

### Overreacting To Small Daily Moves

The curve shifts daily from supply, data, and positioning. Investment strategy should respond to material, persistent changes in shape, not noise.

## Self-Check

- [ ] The specific spread being discussed (2s10s, 3m10y, 5s30s, real versus nominal) is named, not vaguely "the curve."
- [ ] The interpretation separates expectations from term premium and notes QE/QT and safe-asset-demand distortions where relevant.
- [ ] Recession signals are stated probabilistically with variable lead times, not as deterministic predictions.
- [ ] Stagflation risk is assessed using real yields and breakevens, not the nominal curve alone.
- [ ] Any curve-positioning strategy accounts for carry, roll-down, path risk, and the investor's duration objective.
- [ ] Foreign-curve analysis adjusts for currency, inflation, credit, and central-bank differences.
- [ ] The conclusion avoids presenting curve-based trades as low-risk predictions and references investor-specific horizon and risk tolerance.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
