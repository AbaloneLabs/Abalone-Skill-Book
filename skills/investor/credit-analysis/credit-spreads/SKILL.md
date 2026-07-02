---
name: credit_spreads.md
description: Use when the agent is interpreting credit spreads, spread types, risk premia, spread convergence or widening, or using spreads as recession and risk-off signals before forming a view on credit markets or portfolio positioning.
---

# Credit Spreads

A credit spread is the yield premium a corporate bond offers over a comparable risk-free government bond. It is the market's price of credit risk, and it is one of the most information-rich indicators in finance. An investing agent often reads a spread as simply "higher yield for more risk," missing that spreads compensate for default loss, liquidity, and a risk premium that varies with the cycle, and that the direction and shape of spreads across ratings and maturities carry powerful signals about the economy and market regime. Spreads are not just a number; they are a barometer.

Use this skill before answering questions such as "what do credit spreads tell us", "are spreads signaling a recession", "is high-yield attractively priced", or "how do I read the credit market". The goal is to prevent the agent from reading spreads superficially and to force it to decompose spread into its components, interpret widening and compression, read the term structure and rating dispersion, and use spreads as a leading macro and risk signal.

Credit spread analysis is a market-reading skill, not a default forecast. Spreads can stay tight or wide for long periods, and signals can be early. Conclusions should disclose uncertainty and investor-specific context.

## Core Rules

### Decompose The Spread Into Its Components

A credit spread is not pure default compensation. It bundles several components.

Components:

- Expected loss: default probability times loss given default, the actuarial credit risk.
- Liquidity premium: compensation for the bond's lower liquidity versus government bonds.
- Risk premium: additional compensation investors demand for bearing uncertain credit risk, which varies with risk appetite.
- Tax and technical factors: supply, demand, index inclusion, and tax effects.

The same spread can be adequate or inadequate depending on the mix. In calm markets, the risk premium compresses and spreads tighten beyond what expected loss justifies. In stress, the risk premium explodes and spreads widen far beyond expected loss. Understanding which component is moving explains the spread action.

### Interpret Widening And Compression Through The Cycle

Spread direction signals where the credit cycle and risk appetite stand.

Patterns:

- Compression: spreads tighten as liquidity abundant, defaults low, and risk appetite high; typical of expansion and late-cycle complacency.
- Bottoming and turning: spreads stop tightening and begin to widen, often an early warning of credit deterioration.
- Widening: spreads rise as default expectations, risk aversion, or liquidity stress increase; a classic risk-off and recession-warning signal.
- Peak and compression after stress: spreads reach extremes during panic, then compress as conditions stabilize and value buyers return.

The turn from compression to widening is one of the earliest reliable signals that the credit cycle is deteriorating, often preceding equity declines and recessions.

### Read The Rating Dispersion For Stress Signals

How spreads behave across the rating spectrum reveals where stress is concentrated.

Analyze:

- Investment-grade versus high-yield spreads: high-yield widening faster than investment-grade signals rising default concern at the risky end.
- Within high-yield, BB versus CCC: CCC widening faster signals deep distress and a flight to quality within junk.
- Cross-sector dispersion: sectors with widening spreads signal idiosyncratic or structural stress.

Uniform compression suggests broad risk appetite. Rising dispersion, where the weakest credits widen most, is an early stress signal even before aggregate spreads move much.

### Use The Term Structure For Recovery And Refinancing Signals

The shape of spreads across maturities carries information about default timing and refinancing risk.

Interpret:

- Upward-sloping spread curves suggest default risk is back-loaded; the market expects near-term stability.
- Inverted spread curves, where short spreads exceed long, signal near-term refinancing or default stress.
- Kinks in the curve at specific maturities can pinpoint where the market sees refinancing pressure.

A steep spread curve is not always bullish; it can reflect near-term safety with longer-term concern. An inverted curve is a strong near-term stress warning.

### Compare Spreads To Expected Loss For Value

Spreads compensate for expected loss plus a risk premium. Judging value requires comparing spread to expected loss.

Assess:

- Does the spread exceed expected loss by an adequate margin?
- Is the risk premium compressed, offering little cushion, or elevated, offering value?
- How does the current spread compare to history through similar cycle phases?

When spreads are tight and barely exceed expected loss, credit offers little compensation for risk. When spreads are wide and far exceed expected loss, there may be value, provided the default and recovery assumptions are right.

### Use Spreads As A Leading Macro And Risk Signal

Credit spreads are among the earliest indicators of economic and market stress, often leading equities and the official data.

Apply as signals:

- Sustained high-yield spread widening is a classic recession warning.
- Sharp, disorderly widening signals acute risk-off and potential systemic stress.
- Compression after stress signals stabilization and re-risking.
- Divergence between tightening equity volatility and widening credit spreads can warn that credit sees risk equities do not.

Use spreads alongside, not instead of, other indicators. A single spread move is a hint; confirmation across ratings, sectors, and other risk indicators strengthens the signal.

### Distinguish Fundamental From Technical Spread Drivers

Spreads move for fundamental and technical reasons, and confusing them leads to wrong conclusions.

Drivers:

- Fundamental: changing default expectations from deteriorating credit quality or macro conditions.
- Technical: supply and demand, fund flows, dealer positioning, index rebalancing, and forced selling.
- Liquidity: bid-ask widening during stress can make quoted spreads appear wider than true credit risk.

A spread widening driven by forced selling and liquidity withdrawal may reverse quickly, while one driven by fundamental default deterioration may persist. Identify the driver before acting.

## Common Traps

### Treating Spreads As Pure Default Compensation

Spreads bundle expected loss, liquidity, and a risk premium. Reading them as pure default compensation ignores the cycle-driven risk premium.

### Reading A Single Spread Number Without Decomposition

The same spread level can be adequate or inadequate depending on the mix of components. Decompose before concluding.

### Ignoring Rating And Sector Dispersion

Aggregate spreads can mask severe stress in weak credits or sectors. Dispersion reveals where the real risk is.

### Confusing Fundamental And Technical Widening

Technical, flow-driven widening can reverse; fundamental widening persists. Identify the driver before acting.

### Treating Tight Spreads As Safety

Tight spreads reflect low current default expectations and abundant liquidity, not the absence of risk. They can be a complacency signal at cycle peaks.

### Using Spreads In Isolation As A Recession Timer

Spreads are a leading signal but can be early and noisy. Use them alongside other indicators for confirmation.

### Overlooking The Term Structure

The shape of spreads across maturities carries refinancing and default-timing signals that aggregate spreads hide.

## Self-Check

- [ ] The spread is decomposed into expected loss, liquidity premium, risk premium, and technical factors.
- [ ] Widening and compression are interpreted through the credit cycle and risk appetite.
- [ ] Rating and sector dispersion are analyzed for where stress is concentrated.
- [ ] The spread term structure is read for near-term refinancing and default-timing signals.
- [ ] The spread is compared to expected loss to judge whether compensation is adequate.
- [ ] Spreads are used as a leading macro and risk signal, with confirmation from other indicators.
- [ ] Fundamental and technical spread drivers are distinguished before acting.
- [ ] Tight spreads are not treated as proof of safety.
- [ ] The analysis recognizes that spread signals can be early and noisy.
- [ ] The conclusion discloses uncertainty, avoids treating spreads as a precise recession timer, and flags investor horizon and professional advice.
