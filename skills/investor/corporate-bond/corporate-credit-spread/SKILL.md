---
name: corporate_credit_spread.md
description: Use when the agent is analyzing corporate-bond credit spreads, identifying spread drivers, using option-adjusted spread, interpreting spread compression and convergence, reading spreads as a leading macro indicator, or assessing credit risk premium versus compensation.
---

# Corporate Credit Spread

The credit spread is the extra yield a corporate bond offers over a risk-free government bond of similar maturity. It looks like a single number, but it is a composite of default risk, recovery risk, liquidity, taxes, and a risk premium that compensates investors for bearing all of these. Reading spread as "extra return" or as a pure default-probability signal leads to systematic errors in both direction and magnitude.

Use this skill before answering questions such as "are corporate spreads tight or wide", "is investment-grade credit attractively priced", "what does the spread mean", or "should I add credit exposure here". The goal is to prevent the agent from recommending credit based on a headline spread number without decomposing what the spread compensates and whether the compensation is adequate.

## Core Rules

### Define Which Spread Measure Is Being Used

Several "spreads" are quoted and they are not interchangeable.

- G-spread: yield over an interpolated government bond. Simple, but ignores curve shape and embedded options.
- I-spread: yield over the swap curve. Useful in markets where swaps are the benchmark.
- Z-spread (zero-volatility spread): constant spread over the entire government zero-coupon curve that prices the bond. Better for option-free bonds.
- Option-adjusted spread (OAS): the spread after stripping out the value of embedded options. This is the correct measure for callable/putable bonds and MBS.

For a callable bond, the nominal spread includes the option cost paid to the issuer. Only OAS reflects the true credit and liquidity compensation. Always confirm which measure a quoted spread uses before comparing across bonds.

### Decompose The Spread Into Its Components

A spread is not one risk. It bundles:

- expected loss (default probability × loss given default);
- liquidity premium (compensation for thinner trading, wider bid-ask, harder exit);
- risk premium (compensation for bearing credit and spread volatility, above expected loss);
- tax effects (some spread compensates for tax differences versus governments);
- technical/supply effects (issuance, index inclusion, dealer inventory).

The risk premium is often the largest component and the most variable. Spreads can be "tight" even when default expectations are low, because the risk premium has been compressed by demand. Ask whether the spread compensates for the volatility and tail risk, not just expected loss.

### Judge Whether The Spread Adequately Compensates The Risk

The central judgment is compensation adequacy, not the absolute spread level. Questions to ask:

- Is the spread wide or tight relative to its own history, and relative to the level of expected loss?
- Is the spread pricing an benign economy with no recession in the near term?
- What happens to the spread in a moderate recession, and does current carry cover the likely widening?
- Are defaults expected to stay low, or is the market pricing complacency?

Tight spreads in a late-cycle economy often reflect demand and low near-term defaults, not adequate compensation for the tail. Wide spreads in crisis often overcompensate and offer attractive forward returns — but only for investors who can hold through the widening.

### Read Spread Behavior Across The Quality Stack

Spreads behave differently by quality, and the shape of the spread curve across ratings is informative.

- IG spreads (AAA–BBB): narrower, more rate-driven, less default-sensitive; widen modestly in stress.
- Crossover BB: sensitive to the IG/HY boundary and to fallen-angel flows.
- BB–B: the core HY market; spread reflects real default risk and cyclical sensitivity.
- CCC+: distressed; spread is often a price quote, not a yield, because default is a material probability.

A flat spread curve across ratings (HY barely wider than IG) signals complacency. A steep spread curve (HY much wider than IG) signals stress or attractive risk pricing. Compare across the stack, not within one rating.

### Use Spreads As A Leading Indicator, With Humility

Credit spreads tend to widen before equities peak and recessions begin, because credit markets price deterioration earlier. But:

- spreads can stay tight for long periods during risk rallies;
- central-bank support can suppress spreads artificially;
- tight spreads are a warning, not a timing signal;
- widening can be liquidity-driven and reverse quickly.

Use spreads as one input to macro risk assessment, alongside equities, the yield curve, and real-time data. Do not convert a spread level into a precise market-call.

### Separate Convergence (Roll-Down) From Directional Spread Views

A bond's spread can change for two reasons: the issuer's credit improves (idiosyncratic), or the whole sector's spreads move (systemic). A third, often-overlooked source of return is spread convergence through roll-down the curve or credit migration:

- a bond purchased at a wider spread than its on-the-run peers may converge as it seasons;
- a rising star (HY upgraded to IG) experiences spread compression from forced buying.

Spread convergence strategies earn carry from these mechanical effects but assume stable systemic conditions. They lose in broad spread-widening events.

### Account For Liquidity And Optionality In The Spread

Part of any quoted spread is liquidity premium. Illiquid bonds trade wider than identical-credit liquid bonds. During stress, the liquidity component explodes and can dominate the credit component. An investor paying for "yield" may be getting paid for illiquidity they cannot easily exit. For bonds with embedded options, confirm the spread is OAS, not nominal, or the credit compensation is overstated.

## Common Traps

### Comparing Nominal Spreads Across Bonds With Different Optionality

A callable bond's nominal spread is wider partly because it compensates the investor for the issuer's call option. Comparing it to a non-callable bond's spread overstates its credit attractiveness.

### Treating Tight Spreads As A Buy Signal

Tight spreads mean low compensation, not safety. Buying credit when spreads are at cycle tights offers low cushion against widening and poor risk-adjusted forward returns.

### Equating Spread With Expected Excess Return

Spread is gross compensation. Expected excess return ≈ spread − expected losses − taxes − transaction costs − the cost of bearing spread volatility. In recessions, realized excess return is far below the starting spread.

### Ignoring The Risk Premium Component

Much of a spread is risk premium, not expected loss. When risk appetite collapses, the risk premium can widen dramatically even if default expectations are unchanged. Models that compare spread only to expected loss miss the largest swing factor.

### Reading One Quality Tier In Isolation

IG and HY spreads can diverge. A view on "credit" must specify quality, because the risk and compensation differ across the stack.

### Assuming Central-Bank Support Is Permanent

QE and credit-purchase programs compress spreads artificially. Spreads that look tight because of intervention can re-widen when support is withdrawn, independent of fundamentals.

## Self-Check

- [ ] The specific spread measure (G-, I-, Z-, or option-adjusted) is named, and OAS is used for bonds with embedded options.
- [ ] The spread is decomposed into expected loss, liquidity premium, risk premium, tax, and technical components rather than treated as a single number.
- [ ] Compensation adequacy is judged against expected loss, spread volatility, and recession scenarios, not against the absolute spread level or recent history alone.
- [ ] Spread behavior is compared across the quality stack (IG, crossover, core HY, distressed), not within one rating.
- [ ] Spread is used as a leading indicator with humility, acknowledging it can stay tight for long periods and is distorted by central-bank support.
- [ ] Convergence and roll-down return is separated from directional spread views, and its assumption of stable systemic conditions is stated.
- [ ] Liquidity premium and embedded-option effects are accounted for in the spread reading.
- [ ] The conclusion avoids recommending credit exposure based on a headline spread number without investor-specific horizon, cycle view, and risk disclosure.
