---
name: tips_mechanics.md
description: Use when the agent is explaining Treasury Inflation-Protected Securities mechanics, analyzing principal adjustment and inflation indexing, comparing real versus nominal yields, assessing deflation floor protection, or evaluating tax treatment and suitability of inflation-linked government bonds.
---

# TIPS Mechanics

Treasury Inflation-Protected Securities (TIPS) are US government bonds whose principal is adjusted by the CPI. They are the cleanest way to contract for a real (inflation-adjusted) yield. But the mechanics — how the principal adjusts, how the coupon interacts with the inflation index, how deflation is handled, and how the tax works — are easy to get wrong. A common error is to treat TIPS as "bonds that pay more when inflation rises" without understanding that the contracted return is the real yield, and that the inflation adjustment is not free income but compensation for lost purchasing power.

Use this skill before answering questions such as "how do TIPS work", "should I buy TIPS", "what is the real yield on TIPS", or "do TIPS protect against deflation". The goal is to prevent the agent from describing TIPS imprecisely, and from recommending them without checking whether the real yield, deflation floor, and tax treatment fit the investor's situation.

## Core Rules

### Separate The Real Yield From The Inflation Adjustment

A TIPS has two return components:

- The real yield: the contracted return above inflation, set at auction and quoted as the "real yield." This is what the investor locks in regardless of inflation.
- The inflation adjustment: the principal (and therefore the coupon, which is a fixed rate applied to the adjusted principal) is indexed to CPI. This compensates for inflation so that the real yield is preserved.

The investor's realized nominal return is approximately real yield + realized inflation (with compounding). The real yield is the meaningful "price" of the bond; the inflation adjustment simply passes through realized CPI. A TIPS purchased at a 2% real yield delivers roughly 2% real return whatever inflation turns out to be. Do not describe the inflation adjustment as "extra return" — it preserves purchasing power, it does not create it.

### Understand The Principal Adjustment And Index Ratio

Each TIPS carries an index ratio that scales the principal by cumulative CPI since issuance. Mechanics:

- The coupon rate is fixed and low, applied to the inflation-adjusted principal.
- The adjusted principal rises with inflation (or falls with deflation) each day based on the CPI reference.
- At maturity, the investor receives the greater of the adjusted principal or the original par (the deflation floor, for the maturity payment).

Because the coupon is applied to the adjusted principal, both coupon payments and the final principal move with inflation. Quoted prices are in real terms; the dollar price equals the real price times the index ratio.

### Respect The Deflation Floor And Its Limits

TIPS provide a deflation floor at maturity: the investor receives at least original par, even if cumulative inflation since issuance is negative. This is genuine downside protection. But the floor has limits:

- The floor applies to the maturity principal payment, not to the coupon stream. Coupons can fall during deflation because they apply to a declining adjusted principal.
- The floor is per-issue, based on the bond's own issuance par. A seasoned TIPS bought at a high index ratio has less deflation protection than a newly issued one, because the adjusted principal can fall toward (but not below) original par, while the investor paid a price reflecting the higher ratio.
- Intra-life deflation followed by re-inflation still delivers the real yield; the floor mainly matters at maturity.

For an investor buying seasoned TIPS at a high index ratio, the deflation floor is less valuable than it appears. Check the index ratio and the implied downside.

### Compare TIPS To Nominal Treasuries Via Breakeven Inflation

The decision between TIPS and nominal Treasuries of the same maturity turns on breakeven inflation: the difference between the nominal yield and the TIPS real yield. Roughly:

- If realized inflation exceeds the breakeven, TIPS outperform.
- If realized inflation is below the breakeven, nominals outperform.

Breakeven inflation is a market-derived expectation plus risk premia, not a forecast. (See the dedicated breakeven-inflation skill for the decomposition.) Use the breakeven to frame the tradeoff, but do not treat it as a guaranteed forecast. The investor is implicitly taking a view on whether realized inflation will exceed the breakeven.

### Account For Tax Treatment, Especially Phantom Income

TIPS tax treatment is a common source of disappointment:

- The inflation adjustment to principal is taxable as ordinary income in the year it accrues, even though the investor does not receive it as cash until maturity or sale. This is "phantom income."
- Coupons are also taxable as ordinary income.
- In a tax-deferred account (IRA, 401(k)), phantom income is not an issue because tax is deferred until withdrawal.

For taxable investors in high brackets, phantom income can create a negative cash-flow return: the investor owes tax on inflation accruals while receiving only the small coupon. This makes TIPS far more efficient in tax-advantaged accounts. A TIPS held in a taxable account may deliver a poor after-tax real return when inflation is high.

### Match TIPS To The Investor's Objective

TIPS serve specific purposes, and the right structure depends on the goal:

- Liability matching against future real spending: long TIPS lock in a real yield for known future obligations (e.g., retirement real spending). The deflation floor and known real yield are attractive.
- Inflation hedging within a portfolio: TIPS diversify nominal-bond and equity exposure against inflation surprises.
- Short-term inflation protection: short TIPS or TIPS funds track near-term CPI with low duration.
- Speculation on real yields: buying long TIPS when real yields are high locks in real return; this is a rate view on real yields, not pure inflation protection.

State the objective. TIPS are excellent for real-liability matching and inflation hedging but are not a free inflation bet — the real yield at purchase determines the outcome.

### Consider Liquidity, Real-Yield Level, And Fund Versus Individual Bonds

- TIPS are less liquid than nominal Treasuries; spreads can be wider, especially off-the-run.
- The starting real yield dominates long-horizon outcomes. Buying long TIPS at a negative real yield locks in a negative real return to maturity; buying at a high positive real yield is historically attractive.
- Individual TIPS allow precise maturity and deflation-floor matching; TIPS funds provide diversification and liquidity but have fluctuating duration and no maturity date, so they do not lock in a real yield to a specific horizon.

For an investor who needs a known real cash flow at a specific date, an individual TIPS matching that date is cleaner than a fund.

## Common Traps

### Describing The Inflation Adjustment As Extra Return

The inflation adjustment preserves purchasing power; it does not create real wealth. The real yield is the contracted real return. Conflating the two overstates TIPS' attractiveness.

### Ignoring Phantom Income Tax In Taxable Accounts

Tax on inflation accruals that are not received as cash can make after-tax real return negative. TIPS are usually best held in tax-advantaged accounts.

### Treating TIPS Funds As Equivalent To Holding To Maturity

TIPS funds have no maturity and fluctuating duration; they do not lock in a real yield to a specific date. They are not a substitute for liability-matching with individual bonds.

### Buying Long TIPS At Negative Real Yields Without Realizing It

A negative real yield locks in a negative real return to maturity. The bond still "works" as inflation protection, but the investor is paying for the privilege.

### Overstating The Deflation Floor For Seasoned Bonds

The maturity floor protects original par, not the investor's purchase price at a high index ratio. A seasoned TIPS bought at a high ratio has less deflation protection than a new issue.

### Comparing TIPS Coupon To Nominal Coupon Directly

The TIPS coupon is a real rate applied to an adjusted principal. Comparing it to a nominal coupon without adjusting for inflation and the index ratio is meaningless.

## Self-Check

- [ ] The real yield is distinguished from the inflation adjustment, and the inflation adjustment is not described as extra return.
- [ ] The principal adjustment, index ratio, and coupon mechanics are explained correctly.
- [ ] The deflation floor is described with its limits, including reduced value for seasoned bonds bought at high index ratios.
- [ ] TIPS are compared to nominal Treasuries via breakeven inflation, with the tradeoff framed as a view on realized versus breakeven inflation.
- [ ] Phantom income tax is addressed, and TIPS are recommended for tax-advantaged accounts where appropriate.
- [ ] The TIPS structure (individual bond versus fund, short versus long) is matched to a stated objective such as real-liability matching, inflation hedging, or real-yield speculation.
- [ ] Starting real yield level is checked, and the implications of negative real yields are noted.
- [ ] The conclusion avoids presenting TIPS as a free inflation hedge and references investor-specific horizon, tax situation, and risk tolerance.
