---
name: liability_driven_allocation.md
description: Use when the agent is constructing an allocation to match or immunize a defined set of liabilities or future cash-flow obligations, pension or annuity or endowment LDI, matching duration and convexity of assets to liabilities, deciding the funded ratio and surplus risk, or reviewing how interest-rate and inflation sensitivity of assets should mirror the liability profile.
---

# Liability-Driven Allocation

Liability-driven allocation (LDI) structures the asset side to match the characteristics of a defined set of liabilities or future obligations, rather than to maximize return in isolation. The central idea is that risk is measured relative to the liabilities, not in absolute terms: surplus volatility (assets minus liabilities) matters more than asset volatility alone. The judgment problem is that LDI is powerful for institutions with well-defined liabilities (pensions, insurance, endowments with spending rules) but is frequently misapplied by individuals whose "liabilities" are fuzzy, inflation-linked, and uncertain in timing. Agents tend to either over-engineer duration matching for vague household goals, or dismiss LDI entirely and ignore the real benefit of thinking in surplus terms. The skill is knowing when liability matching genuinely reduces risk and when it is a false precision exercise.

This skill is for deciding whether and how to align assets with liabilities, with honest awareness of where the method applies and where it breaks down.

## Core Rules

### Define The Liabilities Before Matching Assets

LDI begins with the liabilities. If the obligations are not well-defined, no amount of asset-side sophistication will help. Characterize the liability stream precisely.

For each liability, capture:

- the amount, in nominal or real terms;
- the timing and whether dates are fixed or contingent;
- the duration and convexity of the liability cash flows;
- inflation sensitivity (nominal versus real obligations);
- certainty of the obligation (a pension annuity is far more certain than an estimated retirement spending need);
- growth or discount assumptions used to value the liability.

A pension with actuarially determined annuity payments is a genuine liability suitable for LDI. A rough estimate of future living costs is an aspiration, not a liability, and treating it as one invites false precision.

### Measure Risk As Surplus, Not Asset Volatility

The defining shift in LDI is that the relevant risk is the volatility of the surplus (assets minus liabilities), not the volatility of assets alone. A portfolio that is volatile in absolute terms but moves with the liabilities may have low surplus risk.

Compute:

- the funded ratio (assets divided by liabilities) and its trajectory;
- surplus volatility under interest-rate and inflation shocks;
- the economic value of the surplus under different discount rates;
- tracking error of assets versus a liability-mimicking benchmark.

A portfolio that looks risky to an absolute-return investor can be low-risk to a liability-matching investor, and vice versa. Always carry the analysis through to the surplus.

### Match Duration And Convexity, Not Just Asset Class

The dominant risk in LDI is usually the interest-rate sensitivity of the liabilities. Long-dated liabilities have long duration, and their present value rises when rates fall. Assets must mirror this to stabilize the surplus.

Match:

- the duration of assets to the duration of liabilities;
- the convexity profile, especially when liabilities have option-like features (early retirement, lump-sum options);
- the key-rate duration along the curve, not just a single point estimate;
- the inflation linkage (nominal bonds for nominal liabilities, real-return assets for inflation-linked liabilities).

A duration mismatch is the most common source of surplus volatility. A short-duration asset portfolio funding long-duration liabilities swings into deficit whenever rates fall.

### Separate The Hedging Sleeve From The Return-Seeking Sleeve

Most LDI designs split the portfolio into two parts with different jobs. Conflating them destroys both.

Define:

- the hedging (liability-matching) sleeve, sized to cover essential liabilities, invested in duration-matched high-quality bonds;
- the return-seeking (growth) sleeve, sized to close any funding gap or build surplus, invested in growth assets;
- the allocation between them based on the funded ratio, risk tolerance, and the cost of shortfall.

Once a liability is well-funded, the marginal benefit of taking more risk to over-fund it is usually lower than the benefit of securing it. The glide path typically increases the hedging share as the funded ratio rises.

### Respect The Difference Between Institutions And Households

LDI is well-suited to institutions with precise, contractual liabilities. It is harder to apply to individuals, whose obligations are softer and more uncertain.

For households, distinguish:

- hard liabilities (a fixed mortgage, a known tuition bill, a pension bridge to Social Security) that genuinely warrant matching;
- soft liabilities (estimated retirement spending) that are better handled with goals-based or floor-and-upside thinking than strict immunization;
- human capital, which is itself a bond- or equity-like asset on the household balance sheet.

Do not impose pension-grade duration matching on a retiree whose spending estimate could be off by 30%. Match what is real; manage the rest with judgment.

### Account For Inflation Linkage Explicitly

Many real-world liabilities are inflation-linked (retirement living costs, wages, medical costs). Matching them with nominal bonds creates hidden real shortfall risk.

Address:

- whether the liability is nominal or real;
- the share of real-return assets (TIPS, inflation-linked bonds, real assets, equities) appropriate to the inflation sensitivity;
- the inflation beta of the liability and the matching assets;
- the cost and availability of inflation-linked instruments.

A nominal-bond hedge that immunizes against rate moves but ignores inflation can leave the real surplus badly exposed.

### Plan For Funding Gaps And Surpluses

LDI must handle the case where assets do not cover liabilities. The response to a deficit differs from the response to a surplus.

For a deficit:

- assess whether return-seeking risk is justified to close the gap, given the cost of failure;
- consider contributions, benefit changes, or longer working life;
- avoid taking so much risk that a market downturn widens the gap catastrophically.

For a surplus:

- consider de-risking to lock in the surplus (a surplus is real wealth only if secured);
- weigh the cost of over-hedging versus the benefit of stability;
- decide whether surplus belongs to the sponsor, the beneficiaries, or future spending.

The funded ratio, not the asset return, should drive the de-risking decision.

## Common Traps

### Applying Pension-Grade LDI To Fuzzy Household Goals

Strict duration immunization assumes liabilities are known precisely. Imposing it on a retiree's spending estimate, which is highly uncertain, produces false precision and unnecessary complexity.

### Ignoring The Surplus View

Optimizing asset volatility while ignoring how assets move relative to liabilities misses the entire point of LDI. The surplus is the risk that matters.

### Duration Mismatch Between Assets And Liabilities

Funding long-duration liabilities with short-duration assets creates large surplus swings when rates move. The mismatch, not the asset volatility, is the danger.

### Matching Nominal Assets To Real Liabilities

Immunizing against rate risk with nominal bonds while the liability is inflation-linked leaves the real surplus exposed to inflation surprises.

### Conflating Hedging And Return-Seeking

Mixing growth assets into the hedging sleeve, or bonds into the growth sleeve, blurs both jobs and defeats the risk separation that makes LDI work.

### Taking Excessive Risk To Close A Funding Gap

Doubling down on equities to fix a deficit can widen the gap in a downturn. The risk taken to close a gap must be survivable if it fails.

### Over-Hedging A Surplus And Forfeiting Growth

Locking every dollar into the hedging sleeve when the funded ratio is high sacrifices the growth that could fund aspirational goals. De-risking should be deliberate, not reflexive.

### Assuming Static Liability Cash Flows

Liabilities change with actuarial assumptions, inflation, longevity, and benefit decisions. A duration match set once and never revisited drifts out of alignment.

## Self-Check

- [ ] The liabilities are defined with amounts, timing, duration, convexity, inflation linkage, and certainty, and the analysis distinguishes hard contractual obligations from soft estimates.
- [ ] Risk is measured as surplus volatility and funded-ratio movement, not asset volatility alone, and a liability-mimicking benchmark or surplus view is used.
- [ ] Asset duration and convexity are matched to liability duration and convexity, including key-rate and inflation profiles, not just broad asset-class labels.
- [ ] The portfolio is split into a hedging sleeve and a return-seeking sleeve with distinct jobs, sized by funded ratio and the cost of shortfall.
- [ ] The design respects the difference between institutional liabilities (precise, contractual) and household goals (uncertain), avoiding false precision where liabilities are soft.
- [ ] Funding gaps and surpluses are handled deliberately, with survivable risk to close gaps and considered de-risking to secure surpluses.
- [ ] Inflation linkage is addressed explicitly, with real-return assets used where liabilities are inflation-sensitive.
- [ ] The recommendation states that LDI reduces relative risk but does not guarantee liabilities will be met, that actuarial and market assumptions are uncertain, and that for pension, insurance, or complex institutional contexts professional actuarial and investment advice is warranted.