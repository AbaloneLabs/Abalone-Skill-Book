---
name: investment_grade_vs_high_yield.md
description: Use when the agent is comparing investment-grade and high-yield corporate bonds, assessing credit ratings and default risk, evaluating recovery rates, analyzing spread and volatility differences, or deciding credit allocation across the business cycle.
---

# Investment Grade Versus High Yield

The split between investment-grade (IG) and high-yield (HY) corporate bonds is not a fine gradient — it is a regime boundary. Crossing from BBB- to BB+ changes the buyer base, the liquidity, the default expectation, the recovery behavior, and how the bond reacts to economic stress. Treating corporate bonds as a single asset class graded by yield leads to mispriced risk and nasty surprises in downturns.

Use this skill before answering questions such as "should I buy investment-grade or high-yield bonds", "how risky are junk bonds", "what credit quality should my bond allocation hold", or "how do IG and HY behave differently". The goal is to prevent the agent from recommending high yield as "higher income" without naming the equity-like drawdown and default risk, and from treating IG as risk-free.

## Core Rules

### Treat The IG/HY Boundary As A Structural Divide

The rating agencies draw a line at BBB-/Baa3 (IG) versus BB+/Ba1 (HY). This is not arbitrary cosmetic sorting — it has real structural consequences:

- Different investor bases: many insurers, pension funds, banks, and mandates are restricted to IG; HY is owned by dedicated credit funds, hedge funds, and retail.
- Different liquidity: IG trades in deeper markets; HY can seize in stress with wider bid-ask and lower volume.
- Different default and recovery profiles: HY defaults are far more frequent; recovery rates vary by seniority and sector.
- Different correlation to equities: HY total return is meaningfully equity-correlated, especially in stress; IG behaves more like rate-sensitive duration with a spread cushion.

A portfolio's behavior in a recession is determined heavily by how much HY it holds. State the IG/HY mix explicitly.

### Understand Rating Categories And Their Limits

Ratings summarize default probability over a horizon, but they lag, they disagree across agencies (split ratings), and they do not capture spread or liquidity.

- IG tiers (AAA, AA, A, BBB): default risk is low; the main risks are spread widening and rate risk. BBB is the largest IG bucket and the "fallen angel" risk zone.
- HY tiers (BB, B, CCC and below): BB is often "crossover" and behaves mildly; CCC is distressed and equity-like, with high default and high spread volatility.
- Ratings lag market pricing. Spread often moves before a downgrade. Use ratings as a starting screen, not as the decision.

Watch for "crossover" BB+ bonds that are IG in one agency and HY in another, and for fallen angels (IG downgraded to HY) which can suffer forced selling.

### Model Default And Recovery Together, Not Just Yield

Expected credit loss = default probability × loss given default (1 − recovery rate). A high yield that compensates for high default probability can still be inadequate if recovery is low.

- Historical average recovery for senior unsecured corporate debt is often around 40%, but it varies enormously by seniority, collateral, sector, and cycle.
- Distressed exchanges and pre-packaged bankruptcies can deliver recoveries far below historical averages.
- In deep recessions, defaults cluster and recoveries fall together — the worst time to be concentrated in low-quality credit.

Do not quote yield-to-maturity as expected return. Expected return ≈ yield − expected credit losses − taxes, adjusted for the probability of downgrade and forced selling.

### Match Credit Quality To Economic Cycle Sensitivity

IG and HY respond very differently to the cycle, and timing matters.

- Early/mid expansion: HY tends to outperform as spreads compress and default risk falls; the extra yield is rewarded.
- Late cycle: HY spreads often tighten to thin levels, reducing compensation for rising default risk; IG offers better risk-adjusted carry.
- Recession/stress: HY spreads blow out, defaults rise, and recovery falls; HY can draw down sharply like equities. IG widens too but typically less and recovers with rate cuts.

Holding HY through a recession is a real risk. The investor's ability to hold through drawdowns, and the time horizon, determine whether HY's extra yield is worth the cyclical risk.

### Separate Spread Risk From Interest-Rate Risk

A corporate bond's total risk has two components:

- Interest-rate (duration) risk: shared with government bonds; rises with maturity and falling rates.
- Spread (credit) risk: specific to the issuer and credit quality; rises as quality falls.

IG is dominated by duration risk. HY is dominated by spread risk and has low effective duration because of its shorter average maturity and higher coupon. This means:

- IG funds can lose from rising rates even with stable credit.
- HY funds can lose from widening spreads even with stable rates.
- The two are not substitutes; they hedge and expose different risks.

A "corporate bond allocation" must specify where on the duration-and-quality grid it sits.

### Check Covenants, Capital Structure, And Secured Status

Two bonds with the same rating can have very different loss outcomes based on where they sit in the capital structure and what protections they carry.

- Senior secured versus senior unsecured versus subordinated: recovery rises with seniority and collateral.
- Covenants (restrictions on additional debt, asset sales, payments): stronger covenants protect bondholders but have weakened over time in many markets.
- Change-of-control puts, incurrence tests, and maintenance covenants differ across issues.

For HY especially, reading the covenants and the capital structure is part of the work, not optional diligence.

### Consider Liquidity And Forced-Selling Dynamics

HY liquidity is thinner and more pro-cyclical. In stress:

- bid-ask widens dramatically;
- dealers provide less capital;
- fund outflows can force selling into a falling market (fire-sale dynamics);
- ETF discounts/premiums and gating can appear.

An investor who may need liquidity in a downturn should not hold concentrated low-quality credit. Liquidity risk compounds credit risk exactly when diversification is most needed.

## Common Traps

### Recommending High Yield As "More Income"

The extra yield exists because of higher default, recovery, downgrade, and liquidity risk. Presenting HY as a simple income boost hides equity-like drawdown risk.

### Quoting Yield-To-Maturity As Expected Return

YTM assumes no defaults, no downgrades, no early redemption, and no transaction costs. For HY, expected return is materially below YTM after credit losses.

### Treating BBB As Safe Because It Is "Investment Grade"

BBB is the lowest IG tier and the main source of fallen angels. In a recession, mass BBB-to-HY downgrades trigger forced selling and outsized losses even without default.

### Ignoring Fallen-Angel And Rising-Star Migration

Bonds crossing the IG/HY line suffer mechanical flows. Fallen angels are sold by IG mandates; rising stars are bought by newly eligible buyers. These flows create price dislocations independent of fundamentals.

### Assuming Diversification In A HY Fund Eliminates Default Risk

HY defaults cluster in recessions, and spreads correlate across issuers. Diversification reduces idiosyncratic default risk but not systemic spread-widening risk.

### Comparing IG And HY On Yield Alone

IG offers more duration risk; HY offers more spread risk. Comparing only coupons ignores that they are different risks suited to different objectives and cycles.

## Self-Check

- [ ] The IG/HY boundary is treated as a structural divide with different investor bases, liquidity, default, and correlation profiles.
- [ ] Default probability and recovery rate are combined into expected credit loss, and yield-to-maturity is not presented as expected return.
- [ ] Credit-quality choice is matched to the business cycle and to the investor's ability to hold through spread-widening drawdowns.
- [ ] Duration risk (IG-dominant) and spread risk (HY-dominant) are separated, and the position's risk drivers are named.
- [ ] Covenants, seniority, collateral, and capital-structure position are considered, especially for HY.
- [ ] Liquidity and forced-selling dynamics in stress are addressed, and the investor's liquidity needs are checked against the credit quality held.
- [ ] Fallen-angel, rising-star, and split-rating edge cases are considered.
- [ ] The conclusion avoids recommending a credit allocation without investor-specific horizon, cycle view, drawdown tolerance, and risk disclosure.
