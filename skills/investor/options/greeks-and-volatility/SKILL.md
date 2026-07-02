---
name: greeks_and_volatility.md
description: Use when the agent is evaluating options using the Greeks and volatility analysis, assessing delta, gamma, vega, theta, and rho exposures, implied versus historical volatility, volatility skew and term structure, and how these measures determine option pricing, risk, and the difference between being long or short volatility.
---

# Greeks And Volatility

The Greeks (delta, gamma, vega, theta, rho) and volatility (implied, historical, skew, term structure) are the analytical foundation of options. They describe how an option's price responds to the underlying, time, volatility, and rates. But the Greeks are not static numbers — they change as market conditions change (the Greeks themselves have sensitivities), and they are based on models (e.g., Black-Scholes) with assumptions that can fail. Treating the Greeks as fixed risk labels, or confusing implied volatility (a market price) with historical volatility (a realized statistic), leads to mispriced trades and misunderstood risk. Evaluating options requires understanding what each Greek measures, how they interact, and what volatility is really telling you.

Use this skill before answering questions such as "what do the Greeks mean", "is implied volatility high or low", "should I buy or sell volatility", or "how does gamma risk work". The goal is to prevent the agent from treating the Greeks as static or volatility as a simple number, and from confusing the price of volatility with its future realization.

## Core Rules

### Understand Each Greek And Its Limitations

Each Greek measures a specific sensitivity, with limitations:

- Delta: the option's price sensitivity to the underlying; also an approximate hedge ratio and probability-of-expiring-ITM proxy. Delta changes as the underlying moves (see gamma); it is a snapshot, not constant.
- Gamma: the rate of change of delta; high near the money and near expiration; gamma risk is the risk of delta changing rapidly, requiring rebalancing. Long gamma profits from large moves; short gamma loses in large moves.
- Vega: sensitivity to implied volatility (per 1% change); long options are long vega (benefit from rising IV); short options are short vega. Vega is highest for longer-dated ATM options.
- Theta: time decay per day; long options lose value over time (negative theta); short options benefit from time decay. Theta accelerates near expiration for ATM options.
- Rho: sensitivity to interest rates; generally small for short-dated options but relevant for long-dated options.

Understand what each Greek measures and that they are instantaneous sensitivities that change as conditions change. A delta hedge today is not a delta hedge tomorrow.

### Distinguish Implied From Historical Volatility

Implied and historical volatility are fundamentally different:

- Implied volatility (IV): the volatility implied by the option's market price, via a pricing model; it is the market's forward-looking expectation and a price, not a statistic. High IV means expensive options.
- Historical (realized) volatility: the actual volatility of the underlying over a past period; a statistic, not a price.
- IV versus HV comparison: comparing IV to HV (and to IV's own history) assesses whether options are expensive or cheap; IV above HV suggests options are rich (favor selling); IV below HV suggests options are cheap (favor buying).
- Volatility risk premium: the tendency for IV to exceed subsequent realized volatility; this is the premium option sellers collect over time, and the cost option buyers pay.

Do not confuse IV (a forward-looking price) with HV (a backward-looking statistic). The comparison between them, and IV's rank relative to its own history, drives whether options favor buying or selling.

### Analyze Volatility Skew And Term Structure

Volatility is not uniform across strikes and maturities:

- Volatility skew (smile): IV varies by strike; equity index options typically show a skew with higher IV for downside puts (crash protection demand) than upside calls. Skew reflects market pricing of downside risk.
- Term structure: IV varies by maturity; normally upward-sloping (longer-dated higher IV) but can invert (front-month higher) during stress when near-term uncertainty spikes.
- Skew and term structure as signals: steep skew indicates demand for downside protection; inverted term structure indicates near-term stress.
- Trading skew and term: some strategies (risk reversals, calendar spreads) trade the shape of skew or term structure rather than the level of volatility.

Skew and term structure reveal how the market prices risk across strikes and maturities. Analyze the shape, not just the level, of volatility.

### Understand Long Versus Short Volatility Positions

Volatility exposure (vega) determines how the position responds to IV changes:

- Long volatility (long vega): long options (long straddles, debit spreads, long calls/puts); benefit from rising IV and large realized moves; suffer from time decay (negative theta). Long volatility is a bet on movement exceeding the implied.
- Short volatility (short vega): short options (short straddles, credit spreads, iron condors); benefit from falling IV and range-bound markets (positive theta); suffer from rising IV and large moves. Short volatility is a bet on movement being less than implied.
- Gamma-vega interaction: long options are long both gamma and vega (benefit from moves and rising IV); short options are short both (suffer from moves and rising IV). This co-movement is why short volatility is dangerous in crashes.
- Volatility risk premium: structurally, IV tends to exceed realized volatility, favoring sellers over time — but with tail risk (crashes) that can wipe out years of premium.

Being long or short volatility is a fundamental position choice with different risk profiles. Short volatility collects premium but carries crash risk; long volatility pays premium but benefits from dislocation.

### Recognize Model Dependence And Greek Instability

The Greeks are model-based and can be unstable:

- Model assumptions: Black-Scholes assumes log-normal returns, constant volatility, and continuous trading; real markets violate these (fat tails, volatility clustering, gaps), causing model Greeks to misstate risk.
- Greek instability: gamma explodes near expiration for ATM options; vega and delta shift as IV changes; the Greeks are local approximations that degrade in large moves.
- Dynamic hedging assumption: delta-neutral hedging assumes continuous, costless rebalancing; real hedging incurs transaction costs and gap risk, so "delta-neutral" positions are not risk-free.
- Stress testing beyond Greeks: scenario analysis (large moves, volatility shifts) reveals risks that point-in-time Greeks understate.

The Greeks are useful approximations, not guarantees. Stress-test positions beyond the point-in-time Greeks, especially for short-gamma and short-vega positions.

### Use Greeks For Position Sizing And Risk Management

The Greeks inform risk management:

- Portfolio Greeks: aggregate delta, gamma, vega, and theta across the book to understand net exposures; manage net exposures to desired risk limits.
- Position sizing by Greeks: size positions by vega, gamma, or delta exposure, not just premium, to compare risk across structures.
- Hedging: use delta hedging to neutralize directional exposure, gamma scalping to monetize long-gamma positions, and vega hedging to neutralize volatility exposure.
- Stop-loss and adjustment: define in advance how positions will be managed if the underlying moves, IV changes, or time passes; avoid ad-hoc decisions under stress.

Use the Greeks to size, hedge, and manage positions systematically. Define management rules in advance rather than reacting under stress.

## Common Traps

### Treating The Greeks As Static

The Greeks change as the underlying, time, and volatility change. A delta hedge today is not a delta hedge tomorrow; gamma explodes near expiration.

### Confusing Implied With Historical Volatility

IV is a forward-looking price; HV is a backward-looking statistic. Comparing them drives buy/sell decisions.

### Ignoring Volatility Skew And Term Structure

IV varies by strike and maturity. The shape of skew and term structure carries information beyond the level.

### Underestimating Short-Volatility Tail Risk

Short volatility collects premium but can lose catastrophically in crashes (gamma-vega co-movement). The volatility risk premium is not free money.

### Trusting Model Greeks In Extreme Conditions

Black-Scholes assumptions fail in fat tails, gaps, and volatility clustering. Stress-test beyond point-in-time Greeks.

### Equating Delta-Neutral With Risk-Free

Delta-neutral positions still have gamma, vega, and theta risk; dynamic hedging incurs costs and gap risk. Delta-neutral is not risk-free.

## Self-Check

- [ ] Each Greek (delta, gamma, vega, theta, rho) is understood as an instantaneous, changing sensitivity, not a static label.
- [ ] Implied volatility (a forward-looking price) is distinguished from historical volatility (a backward-looking statistic), and their comparison drives the buy/sell decision.
- [ ] Volatility skew and term structure are analyzed for information beyond the level of IV.
- [ ] Long versus short volatility exposure (vega, and the gamma-vega interaction) is understood, including short-volatility tail risk.
- [ ] Model dependence (Black-Scholes assumptions) and Greek instability in extreme conditions are recognized; positions are stress-tested beyond point-in-time Greeks.
- [ ] The Greeks are used for portfolio aggregation, position sizing, hedging, and pre-defined risk management rules.
- [ ] Delta-neutral is not equated with risk-free; gamma, vega, theta, and dynamic-hedging costs are recognized.
- [ ] The conclusion references the specific Greek and volatility profile of the position, includes appropriate risk disclosure (especially for short volatility and short gamma), and notes the need for options-trading experience.
