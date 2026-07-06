---
name: autocallable.md
description: Use when the agent is evaluating autocallable structured products, assessing the autocall mechanism and observation dates, barrier levels and downside risk, the conditional coupon, residual maturity and path dependence, issuer credit risk, and how these notes can deliver high headline yields while exposing the investor to significant equity downside.
---

# Autocallable

Autocallable notes are among the most popular structured products. They offer a high headline coupon (often 7-12%) paid conditionally on an underlying staying above a barrier, with the note automatically redeeming early (autocalling) if the underlying is above a level on observation dates. The appeal is high yield in a low-rate world. But the high coupon is compensation for selling downside protection: if the underlying falls below a barrier at maturity, the investor suffers the full equity loss. Autocallables are complex, path-dependent, and embed significant equity downside risk that is often obscured by the attractive coupon. Evaluating an autocallable requires understanding the autocall mechanics, the barrier, the conditional coupon, the path dependence, and the issuer risk — not just the headline yield.

Use this skill before answering questions such as "is this autocallable note a good investment", "how do autocallables work", "what is the risk of an autocall", or "why does this note pay 10%". The goal is to prevent the agent from treating the headline coupon as a safe yield, and from ignoring the conditional coupon, barrier, downside equity risk, path dependence, and issuer risk.

## Core Rules

### Understand The Autocall Mechanism And Observation Dates

The autocall feature defines the early-redemption structure:

- Autocall (early redemption): on periodic observation dates (quarterly or semi-annually), if the underlying is at or above the autocall level (often the initial level, 100%), the note automatically redeems early at par plus accrued coupon.
- Observation dates: the dates on which the autocall and coupon conditions are checked; more frequent observations increase the probability of autocall but also increase the chance of early redemption at par (limiting upside).
- Autocall probability: the note is likely to autocall if the underlying performs moderately well; the investor gets par plus coupon but loses the future coupon stream.
- Early redemption effect: autocalling returns principal early, requiring reinvestment at then-prevailing (possibly lower) rates; the high coupon is not perpetual.

The autocall mechanism means the note is likely to redeem early, ending the high-coupon stream. Understand the observation frequency, autocall level, and reinvestment risk upon early redemption.

### Analyze The Conditional Coupon And Its Triggers

The coupon is conditional, not guaranteed:

- Coupon condition: the coupon is paid on each observation date only if the underlying is at or above a coupon barrier (often the initial level or slightly below, e.g., 70-80% of initial).
- Coupon barrier versus autocall level: the coupon barrier is typically lower than the autocall level; the underlying can be below the autocall level (no early redemption) but above the coupon barrier (coupon paid), or below both (no coupon, no autocall).
- Memory feature: some notes have "memory" — missed coupons accumulate and are paid if the underlying later exceeds the barrier; notes without memory forfeit missed coupons permanently.
- High coupon as compensation: the high coupon reflects the risk that the underlying falls and the investor suffers equity losses at maturity; the coupon is the premium for selling downside protection.

The coupon is conditional and reflects the sale of downside protection. Assess the coupon barrier, memory feature, and the relationship between the coupon and the equity downside risk.

### Assess The Downside Barrier And Equity Loss Risk

The principal risk is the core risk:

- Downside barrier: at maturity, if the underlying is below a barrier (often 50-70% of initial), the investor suffers the full percentage loss of the underlying; principal is reduced proportionally.
- Barrier type: European barrier (observed only at maturity) versus American (continuous) barrier (observed at any point); American barriers are more likely to be breached and carry more risk.
- Knock-in feature: some notes require the barrier to be "knocked in" (breached at any point) for the equity loss to apply; if never breached, principal is protected at maturity.
- Downside scenario: in a significant market decline, the investor can lose a large portion of principal — comparable to holding the equity — while having received only the conditional coupon as offset.

The downside barrier determines the principal risk. Assess the barrier level, type (European vs. American, knock-in), and the loss scenario. The investor is selling downside protection; the coupon is the premium.

### Evaluate Path Dependence And The Interaction Of Features

Autocallables are path-dependent; the features interact:

- Path dependence: the outcome depends on the path of the underlying across all observation dates, not just the final level; the same final level can produce different outcomes depending on the path.
- Autocall ending the note: if the note autocalls early, the investor avoids subsequent barrier risk but loses future coupons; the outcome is par plus coupon.
- Survival to maturity with barrier breach: if the note does not autocall and the underlying is below the barrier at maturity, the investor suffers equity losses despite years of (possibly missed) coupons.
- Coupon-versus-loss asymmetry: the investor earns a finite coupon (e.g., 8% per year) but can suffer a large equity loss (e.g., 40-60%); the upside is capped (autocall at par) while the downside is significant.

The path-dependent interaction of autocall, coupon, and barrier creates outcomes ranging from early redemption at par plus coupon to material principal loss. Model scenarios across paths, not just endpoints.

### Assess Issuer Credit Risk And Secondary-Market Liquidity

Autocallables carry issuer and liquidity risk:

- Issuer credit risk: the note is an unsecured obligation of the issuer; if the issuer defaults, coupons and principal are at risk regardless of the underlying's performance.
- Issuer default correlation: worst case, the issuer defaults during a market decline (when the underlying is also down), compounding the loss.
- Secondary-market liquidity: secondary markets are thin and dealer-controlled; early sale often realizes a loss, especially if the underlying has declined.
- Pricing opacity: secondary-market prices are set by the issuer or dealer and are often unfavorable; the investor has limited price discovery.

Assess the issuer's credit quality and the poor secondary-market liquidity. The note must be held to autocall or maturity; early exit is costly.

### Compare To Direct Equity And Option Strategies

Autocallables can be compared to replication:

- Replication: an autocallable is roughly a combination of a bond, a short put (downside risk), and conditional digital coupons (short put options); the investor is selling options to the issuer.
- Direct equity plus bond: holding equity and a bond directly provides upside without the conditional-coupon complexity and without capping the upside at autocall.
- Covered call or put-selling: the investor could sell options directly (covered calls, cash-secured puts) to generate income with more transparency and control.
- Embedded cost: the issuer embeds structuring and distribution fees; the fair value at issuance is below par.

Compare the autocallable to direct equity, bond, and option strategies. The autocallable packages these with embedded fees and opacity; direct strategies offer transparency and control.

## Common Traps

### Treating The Headline Coupon As A Safe Yield

The coupon is conditional and compensates for selling downside protection. It is not a guaranteed yield.

### Underestimating The Downside Barrier Risk

If the underlying breaches the barrier at maturity, the investor suffers full equity losses. The finite coupon does not offset large losses.

### Ignoring Path Dependence

The outcome depends on the path across observation dates, not just the final level. Model scenarios across paths.

### Forgetting That Autocall Ends The High Coupon

Early redemption returns par plus coupon but ends the coupon stream, requiring reinvestment at prevailing rates.

### Overlooking Issuer Credit Risk And Default Correlation

The note is an unsecured obligation. Issuer default during a market decline compounds the loss.

### Underestimating Secondary-Market Illiquidity

Early sale often realizes a loss. The note must be held to autocall or maturity.

## Self-Check

- [ ] The autocall mechanism (observation dates, autocall level, early redemption, reinvestment risk) is understood.
- [ ] The conditional coupon (coupon barrier, memory feature, relationship to downside risk) is analyzed as compensation for selling downside protection.
- [ ] The downside barrier (level, European vs. American, knock-in) and the equity-loss scenario are assessed as the core principal risk.
- [ ] Path dependence and the interaction of autocall, coupon, and barrier are modeled across scenarios, not just endpoints.
- [ ] Issuer credit risk, default correlation with market declines, and secondary-market illiquidity are evaluated.
- [ ] The autocallable is compared to direct equity, bond, and option strategies (covered call, put-selling), and the embedded fees and opacity are recognized.
- [ ] The conclusion avoids presenting the headline coupon as safe yield, references the barrier, path dependence, issuer risk, and capped upside, and notes the investor-specific suitability and the need for structured-products experience.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
