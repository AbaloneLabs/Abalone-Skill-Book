---
name: breakeven_inflation.md
description: Use when the agent is interpreting breakeven inflation from TIPS and nominal Treasuries, decomposing expected inflation from inflation risk premium, comparing nominal versus real yields, or trading and hedging inflation views via breakeven positions.
---

# Breakeven Inflation

Breakeven inflation is the difference between the yield on a nominal Treasury and the real yield on a TIPS of the same maturity. It is widely quoted as "the market's inflation expectation," but it is not a clean forecast. It is a composite of expected inflation, an inflation risk premium, and liquidity premia that differ between the nominal and TIPS markets. Trading or judging inflation hedges on the headline breakeven number, without decomposing it, leads to misreading both the level and the signal.

Use this skill before answering questions such as "what is the market's inflation expectation", "is inflation priced in", "should I buy TIPS or nominals", or "how do I trade an inflation view". The goal is to prevent the agent from equating breakeven inflation with expected inflation, and from recommending breakeven trades without naming the premia and liquidity risks involved.

## Core Rules

### Define Breakeven Inflation Precisely

Breakeven inflation (BEI) = nominal yield − real (TIPS) yield, for matched maturities. It is the inflation rate at which an investor would be indifferent between holding the nominal Treasury and the TIPS to maturity.

- If realized CPI inflation over the period exceeds BEI, TIPS outperform.
- If realized inflation is below BEI, nominals outperform.

BEI is a spread, not a yield. It is derived from two separate instruments, each with its own market. Treat it as a relative-value measure, not a single security's quote.

### Decompose Breakeven Into Expectations Plus Premia

BEI is not pure expected inflation. It bundles:

- expected inflation: the market's genuine forecast of average CPI over the period;
- inflation risk premium: compensation investors demand for uncertainty about inflation (positive when inflation is feared, occasionally negative when deflation is feared);
- liquidity premium differential: TIPS are less liquid than nominals; the TIPS real yield includes a liquidity premium that inflates measured BEI. In stress, this premium swings wildly.

The inflation risk premium and the liquidity premium can be large and variable. A BEI of 2.5% might reflect 2.0% expected inflation plus a 0.5% risk premium, or 2.5% expected inflation with no premium. Models attempt to decompose these, but with uncertainty. Do not present BEI as a precise forecast.

### Use BEI For Relative Value, Not As A Forecast To Bet Against Casually

The decision between TIPS and nominals is a bet on whether realized inflation will exceed BEI. Frame it as such:

- Buying TIPS (long breakeven): profits if realized inflation exceeds the current BEI.
- Buying nominals (short breakeven): profits if realized inflation falls short of BEI.

Because BEI embeds a risk premium and liquidity effects, "BEI looks high/low" is not by itself a trade signal. The investor must have a view on whether realized inflation will deviate from BEI, and on whether the premia are mispriced. Many investors who "buy TIPS because inflation is rising" are buying after BEI has already risen, paying for inflation protection that is no longer cheap.

### Interpret BEI Across The Maturity Spectrum

BEI varies by maturity, and the term structure is informative:

- Short-dated BEI (1-2y): tracks near-term inflation prints and energy/food moves closely; noisy.
- Medium BEI (5y): reflects cyclical inflation views.
- Long BEI (10y, 30y): reflects anchored long-run inflation expectations; central-bank credibility matters most here.
- 5y5y forward BEI: the implied 5-year inflation starting 5 years forward, used to isolate long-run expectations from near-term noise.

A flat BEI curve suggests well-anchored expectations; a steeply rising or falling BEI curve signals shifting views about persistence. Compare across maturities rather than quoting a single point.

### Account For Liquidity And The TIPS/Nominal Basis

The TIPS market is smaller and less liquid than the nominal Treasury market. Consequences for BEI:

- In normal times, TIPS carry a liquidity premium that raises the real yield and thus lowers measured BEI relative to "true" expectations.
- In crises (e.g., 2008, March 2020), TIPS liquidity can deteriorate sharply, real yields spike, and BEI collapses — not because inflation expectations fell, but because the liquidity premium exploded. Nominal yields fell on safe-haven demand while TIPS cheapened.
- BEI can detach from inflation fundamentals during liquidity stress.

Any BEI-based trade or analysis must consider whether liquidity premia are distorting the reading. A sudden BEI drop in a crisis is often a liquidity artifact, not a deflation forecast.

### Distinguish Inflation-Level Risk From Inflation-Surprise Risk

BEI prices the expected path plus a premium for uncertainty. Two distinct risks:

- Inflation level: will average inflation be high or low? Long BEI (TIPS) hedges high inflation.
- Inflation surprise: will inflation deviate from the priced path? TIPS hedge positive surprises; nominals benefit from negative surprises.

An investor concerned about a specific inflation scenario (e.g., a supply shock, fiscal dominance) should check whether that scenario is already in BEI. If BEI has already risen to price the scenario, TIPS offer less marginal protection.

### Consider Tax, Account, And Implementation For BEI Trades

- Long BEI via individual TIPS vs short nominals: cleanest but capital-intensive and requires two legs.
- TIPS funds/ETFs: convenient but have duration and no maturity; they express a real-yield and inflation view without locking a breakeven to a date.
- Inflation swaps: institutional instrument for pure breakeven exposure; not accessible to most retail investors.
- Tax treatment: TIPS phantom income (see TIPS mechanics skill) affects after-tax breakeven for taxable investors.

Implementation choice affects how cleanly the trade expresses the inflation view and what residual risks (duration, liquidity, tax) remain.

## Common Traps

### Equating Breakeven Inflation With Expected Inflation

BEI includes a risk premium and a liquidity premium. Treating the headline number as the market's inflation forecast overstates precision and ignores the variable premia.

### Buying TIPS After BEI Has Already Risen

When inflation fears spike, BEI rises and TIPS become expensive. Buying then pays for protection that may already be priced; the investor needs inflation to exceed the new, higher BEI to profit.

### Ignoring Liquidity Distortion In Crises

BEI can collapse in crises because TIPS liquidity deteriorates, not because inflation expectations fell. Reading a crisis BEI drop as a deflation forecast is a classic error.

### Quoting One Maturity's BEI As "The" Inflation View

BEI varies by maturity. A 1-year BEI and a 30-year BEI reflect different things. Specify the maturity and consider the term structure and forwards.

### Treating BEI Trades As Pure Inflation Bets

Long/short breakeven trades carry duration, liquidity, and roll risk. A "pure" inflation view is hard to isolate and often contaminated by these residual risks.

### Forgetting Tax Effects On After-Tax Breakeven

Phantom income tax on TIPS raises the effective breakeven for taxable investors. The after-tax indifference point differs from the pre-tax BEI.

## Self-Check

- [ ] Breakeven inflation is defined as nominal minus real yield for matched maturities, and framed as the indifference point between TIPS and nominals.
- [ ] BEI is decomposed into expected inflation, inflation risk premium, and liquidity premium, not presented as a clean forecast.
- [ ] The TIPS-versus-nominals decision is framed as a bet on realized inflation versus BEI, with awareness of whether protection is already priced.
- [ ] BEI is interpreted across the maturity spectrum and via forwards (e.g., 5y5y), not as a single point.
- [ ] Liquidity distortion of BEI in crises is acknowledged, and crisis BEI drops are not misread as deflation forecasts.
- [ ] Inflation-level risk is distinguished from inflation-surprise risk, and the investor's specific scenario is checked against priced BEI.
- [ ] Implementation (individual TIPS, funds, swaps) and tax/account effects on after-tax breakeven are considered.
- [ ] The conclusion avoids presenting BEI as a precise forecast and references investor-specific inflation view, horizon, and risk tolerance.
