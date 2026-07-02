---
name: mbs_and_prepayment.md
description: Use when the agent is analyzing mortgage-backed securities, modeling prepayment risk, assessing extension and contraction risk, evaluating how interest rates, seasonality, and refinancing waves affect MBS cash flows and duration, or deciding suitability of agency and non-agency MBS.
---

# MBS And Prepayment

Mortgage-backed securities (MBS) package residential (or commercial) mortgages into bonds whose cash flows depend on whether homeowners refinance, move, or default. The defining feature is prepayment risk: homeowners can pay back principal early, and they tend to do so in ways that hurt the investor. This creates negative convexity — the bond's life shortens when rates fall (refinancing wave) and lengthens when rates rise (nobody refinances). An MBS is not a bond with a known maturity; it is a short option position on the path of interest rates and human behavior.

Use this skill before answering questions such as "are mortgage bonds safe", "what is prepayment risk", "why do MBS underperform when rates fall", or "how do MBS behave in different rate environments". The goal is to prevent the agent from treating MBS like corporate or government bonds, and from presenting the higher yield as free income rather than compensation for an embedded short option.

## Core Rules

### Frame MBS As A Bond Minus A Prepayment Option

An MBS is economically a non-callable bond in which the investor has sold homeowners a prepayment option. Homeowners prepay when it benefits them (rates fall, they refinance; they sell the house; they default and the insurer/guarantor pays). The investor receives the higher yield as compensation for having sold this option.

Because the option is held by economically rational actors, prepayment tends to happen at the worst time for the investor:

- Rates fall: homeowners refinance en masse; the MBS is called away (prepaid) just as the investor would enjoy price appreciation. The bond's effective duration collapses (contraction risk).
- Rates rise: nobody refinances; the MBS extends and behaves like a longer bond, locking the investor into a below-market coupon while the price falls with duration (extension risk).

This is negative convexity. The investor gives up upside and keeps downside. The extra yield is the premium for this asymmetry.

### Distinguish Agency From Non-Agency (Credit) Risk

The MBS market splits into two very different credit profiles:

- Agency MBS (issued/guaranteed by Ginnie Mae, Fannie Mae, Freddie Mac): credit/default risk is largely borne by the guarantor (US-government-backed for GNMA, conservatorship-supported for the GSEs). The investor's main risk is prepayment and rate risk, not default. The dominant analysis is prepayment modeling.
- Non-agency MBS (private-label, no guarantee): the investor bears credit/default risk directly. Analysis combines prepayment, default, and loss severity (recovery). These behaved catastrophically in 2008 and remain structurally riskier.

Do not analyze agency MBS for default risk as if it were non-agency, and do not assume non-agency MBS carry government backing. Identify the structure first.

### Model Prepayment Drivers, Not Just Rates

Prepayment is driven by more than the rate spread:

- Refinance incentive: the gap between the loan rate and current mortgage rates. The bigger the incentive, the faster prepayment.
- Housing turnover and seasonality: home sales drive prepayment; spring/summer see higher turnover.
- Burnout: after a pool has been through a refinance wave, the remaining borrowers are those who cannot or will not refinance (credit, equity, inertia), so the pool prepays slower in subsequent rate drops.
- Path dependence: prepayment depends on the rate path, not just the current rate. A pool that experienced falling rates prepays differently than one experiencing the same final rate via a different path.
- Credit and equity: borrowers with impaired credit or low/negative equity cannot refinance even when rates fall (lock-in effect).

Prepayment models (PSA, CPR, SMM) are projections, not certainties. They are calibrated on history and can be wrong when behavior, regulation, or technology (e.g., streamlined refi) changes.

### Separate Contraction Risk From Extension Risk

These are the two sides of prepayment risk and they harm the investor in opposite rate environments:

- Contraction risk (rates fall): prepayments accelerate; average life shortens; the investor receives principal back early and must reinvest at lower rates. Price appreciation is capped because the bond prepays near par.
- Extension risk (rates rise): prepayments slow; average life extends; the bond behaves like a longer-duration instrument and its price falls more. The investor is locked into a below-market coupon.

Both are manifestations of negative convexity. An MBS investor is short optionality in both directions. The higher yield compensates for this, but only if prepayment behavior matches the model.

### Understand How MBS Duration And Yield Behave Across Cycles

Because of prepayment, MBS effective duration is unstable:

- In falling-rate environments, duration shortens sharply (prepayment accelerates), capping gains relative to Treasuries.
- In rising-rate environments, duration lengthens (prepayment slows), magnifying losses relative to Treasuries.
- The OAS (option-adjusted spread) is the correct richness/cheapness measure, because nominal spread includes the cost of the prepayment option.

MBS tend to outperform Treasuries in stable, range-bound rate environments (they earn the OAS without much option exercise) and underperform in volatile environments (the short option position loses). The path of rates matters as much as the level.

### Account For Servicing, Guaranty Fees, And Pool Characteristics

MBS cash flows are reduced by servicing fees, guaranty fees, and credit losses (non-agency). Pool-specific characteristics drive prepayment:

- Loan age (seasoning): newer loans prepay slower; seasoned pools vary.
- Loan size: larger loans refinance faster (bigger savings).
- Original loan-to-value (LTV): higher-LTV loans have less refinance flexibility.
- Geographic concentration: pools concentrated in appreciating markets prepay faster via turnover.
- Coupon stack: the spread between the pool's coupon, the net coupon, and prevailing rates drives the refinance incentive.

Specified pools (particular characteristics) trade at premiums or discounts to generic TBA MBS based on their prepayment profiles.

### Match MBS To The Investor's Rate View And Liquidity Needs

MBS suit specific objectives and rate views:

- Stable-rate environment, income focus: MBS earn the OAS with limited option exercise; attractive carry.
- Falling-rate view: MBS underperform due to prepayment; avoid or hedge.
- Rising-rate view: MBS extend and underperform duration-matched Treasuries; the below-market coupon compounds the loss.
- Liquidity: agency MBS are among the most liquid fixed-income sectors; non-agency and specified pools are less liquid.

State the rate view and the role of the MBS in the portfolio. MBS are not a one-size income holding; their behavior is path-dependent and asymmetric.

## Common Traps

### Treating MBS Like Corporate Or Government Bonds

MBS have negative convexity and path-dependent cash flows. Applying standard duration/convexity or nominal-spread analysis without option adjustment produces wrong risk and value estimates.

### Presenting The Higher Yield As Free Income

The extra yield compensates for the prepayment option the investor sold. It is a premium for negative convexity, not a gift.

### Ignoring Extension Risk In Rising Rates

MBS extend when rates rise, behaving like longer bonds and underperforming. Investors who only think about prepayment in falling-rate terms miss half the risk.

### Assuming Prepayment Models Are Accurate

Prepayment behavior changes with regulation, technology, credit conditions, and borrower demographics. Models calibrated on history can be badly wrong in new environments.

### Conflating Agency And Non-Agency Credit Risk

Agency MBS are largely government-backed; non-agency MBS bear direct default and loss risk. Treating non-agency as government-backed, or analyzing agency for default risk, misprices the security.

### Overlooking Path Dependence

Two MBS with the same current coupon and rate can behave very differently based on the rate path they experienced (burnout). Point-in-time analysis misses this.

## Self-Check

- [ ] MBS are framed as a bond minus a prepayment option, with negative convexity, not as a standard bond.
- [ ] Agency (government-backed) and non-agency (credit-bearing) MBS are distinguished, and the analysis matches the structure.
- [ ] Prepayment drivers (refinance incentive, turnover, seasonality, burnout, path dependence, credit/equity) are modeled, not just the rate level.
- [ ] Contraction risk (falling rates) and extension risk (rising rates) are both addressed.
- [ ] Effective duration and OAS are used, not nominal spread and modified duration.
- [ ] Pool characteristics (seasoning, loan size, LTV, geography, coupon stack) are considered where relevant.
- [ ] MBS suitability is matched to a stated rate view and liquidity need, with the path-dependence caveat.
- [ ] The conclusion avoids presenting MBS yield as free income and references investor-specific rate view, horizon, and risk tolerance.
