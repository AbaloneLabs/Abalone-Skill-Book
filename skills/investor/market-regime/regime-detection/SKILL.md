---
name: regime_detection.md
description: Use when the agent is detecting shifts in market regime, interpreting changes in volatility correlation and dispersion, reading leading economic indicators and central bank signals, or judging whether market structure has changed before adjusting portfolio risk or exposure.
---

# Regime Detection

A market regime is a persistent state defined by the dominant driver of returns, the level of volatility, the pattern of cross-asset correlations, and the prevailing policy and liquidity backdrop. Regimes matter because the same portfolio can thrive in one and fail in another. An investing agent often treats the recent regime as permanent, projecting today's correlations and factor behavior indefinitely. This is the core error. Regimes shift, sometimes abruptly, and the shift invalidates strategies built for the previous regime.

Use this skill before answering questions such as "has the market regime changed", "why are my diversifiers failing", "what indicators signal a regime shift", or "how should positioning adapt to a new regime". The goal is to prevent the agent from assuming regime stability and to force it to monitor the indicators, correlations, and structural signals that mark transitions, while avoiding the opposite trap of declaring a regime change on every data wobble.

Regime detection is inherently uncertain and backward-looking indicators lag. Acting on a suspected shift carries real risk, especially if the call is premature. Conclusions should disclose confidence, lag risk, and investor-specific constraints.

## Core Rules

### Define What A Regime Is And Why It Matters

A regime is more than bull or bear. It is the combination of the return driver, volatility level, correlation structure, and policy backdrop that determines which strategies work.

Regime dimensions:

- Growth regime: expansion, slowdown, recession, recovery.
- Inflation regime: disinflation, stable, reflation, stagflation.
- Policy regime: easing, neutral, tightening, quantitative expansion or contraction.
- Liquidity regime: abundant, normal, tight, stress.
- Volatility regime: low and stable, elevated, crisis.
- Correlation regime: stock-bond diversifying, stock-bond correlated, cross-asset panic.

The same asset can behave differently across regimes. Long-duration bonds diversify equities in a disinflationary growth shock but fall alongside equities in an inflation or rate-shock regime. Knowing the regime tells you which assumptions still hold.

### Monitor Leading Indicators, Not Just Coincident Data

Regime shifts are visible first in leading indicators, then in markets, then in lagging confirmations like employment and GDP. Relying only on coincident data guarantees a late response.

Leading inputs to watch:

- yield curve shape and changes;
- credit spreads and lending standards;
- purchasing manager indices and new orders;
- jobless claims and hiring surveys;
- earnings revisions and guidance;
- housing permits and building activity;
- commodity and input price trends;
- central bank communication and balance-sheet direction.

No single leading indicator is reliable. Look for agreement across several, and weight those with the best historical track record for the specific transition you suspect.

### Track Volatility As A Regime Signal

Volatility regimes are sticky. A shift from low and stable to elevated volatility often marks a regime change, and volatility clustering means the new regime can persist.

Interpret:

- Realized versus implied volatility, and the volatility risk premium.
- The VIX or equivalent level and term structure; inversion can signal stress.
- Volatility-of-volatility and skew as signs of hedging demand.
- Dispersion, how much individual stocks move differently, which rises in transitions.

Low volatility is not safety. It can reflect complacency and hidden leverage that unwinds violently. Elevated volatility can mark the end of a regime and the start of another.

### Watch Correlation And Dispersion Changes

Correlations shift by regime, and a change in the correlation structure is one of the clearest regime signals.

Key patterns:

- In calm risk-on regimes, correlations within and across asset classes can be low, rewarding stock-picking.
- In stress, correlations converge toward one as nearly everything falls together, destroying diversification.
- Stock-bond correlation can flip sign across inflation regimes, changing whether bonds hedge equities.
- Factor correlations change; defensive and quality factors may decouple from growth and speculative factors.

If your diversifiers suddenly move with the assets they were meant to offset, the regime may have changed even if the headline index looks stable.

### Read Central Bank Signals And The Policy Reaction Function

The policy regime is defined not only by current rates but by the central bank's reaction function, how it responds to incoming data. A change in the reaction function is a regime shift.

Ask:

- Is the central bank reacting to inflation, employment, financial stability, or growth?
- Has the balance between inflation and growth concerns shifted?
- Is forward guidance credible, and are markets pricing the same path?
- Is liquidity being added or drained through balance-sheet policy?
- Are there cross-border policy divergences affecting currency and flows?

A central bank that pivots from inflation-fighting to financial-stability support has changed the regime for risk assets, even before the data confirms it.

### Assess Market Structure And Positioning

Market structure, the composition of participants, leverage, and liquidity, shapes how regimes express themselves and how violently transitions occur.

Structural factors:

- ETF and passive ownership share and its effect on correlations and flows.
- Derivatives positioning, dealer gamma, and options-driven flows.
- Hedge fund leverage and crowding in popular trades.
- Retail participation and zero-commission behavior.
- Market-maker inventory and liquidity provision.

A regime shift into a market that is crowded and leveraged will be sharper than one into a market that is under-owned and lightly positioned. Positioning data, though imperfect, helps gauge the fuel for or against a transition.

### Confirm With Multiple Independent Signals

A robust regime call requires agreement across independent signal types. One indicator wobbling is noise; several aligning across growth, inflation, policy, volatility, and correlation is evidence.

Build a checklist:

- Do leading indicators agree on direction?
- Is volatility persistently elevated or suppressed?
- Have correlations shifted in a regime-consistent way?
- Has the policy reaction function changed?
- Is positioning consistent with the old or new regime?
- Are cross-asset signals, equities, credit, rates, currency, commodities, consistent or conflicting?

Name the conflicts. A regime call with conflicting signals should carry low confidence and smaller position changes.

## Common Traps

### Treating The Current Regime As Permanent

The most common error is projecting today's correlations, factor returns, and policy backdrop indefinitely. Regimes shift, and strategies optimized for the last regime often fail in the next.

### Declaring A Regime Change On One Data Point

A single weak print or one volatility spike is not a regime shift. Regimes are persistent states confirmed by multiple signals over time.

### Relying Only On Lagging Confirmations

GDP and unemployment confirm regime shifts long after markets have moved. Waiting for confirmation guarantees acting late.

### Assuming Diversifiers Always Diversify

Correlations rise in stress and flip across inflation regimes. A portfolio diversified for one regime can concentrate risk in another.

### Overfitting To Recent Volatility Behavior

Low realized volatility can breed leverage that unwinds violently. Treating low volatility as proof of safety ignores the fragility it can create.

### Ignoring The Policy Reaction Function

The same data can produce opposite market outcomes depending on how policy responds. A central bank that eases into weakness supports risk assets; one that tightens into weakness does not.

### Confusing Noise With Structural Change

Not every correlation wobble or flow reversal is a regime shift. Distinguish a persistent state change from temporary dislocation.

## Self-Check

- [ ] The regime is defined across growth, inflation, policy, liquidity, volatility, and correlation dimensions, not labeled from price alone.
- [ ] Leading indicators are monitored, not only coincident or lagging data.
- [ ] Volatility level, term structure, and dispersion are interpreted as regime signals.
- [ ] Correlation and dispersion changes are checked for diversification breakdown or factor rotation.
- [ ] The central bank reaction function and balance-sheet direction are assessed for regime implications.
- [ ] Market structure, leverage, and positioning are considered for transition violence.
- [ ] The regime call is confirmed by multiple independent signals, and conflicts are named.
- [ ] The analysis distinguishes a persistent state change from temporary noise.
- [ ] Lag risk is acknowledged, since leading indicators can be early and confirmations late.
- [ ] The conclusion states confidence, avoids false precision, and flags investor horizon, risk tolerance, and professional advice.
