---
name: callable_and_putable_bonds.md
description: Use when the agent is analyzing callable or putable bonds, assessing call and put premia, refinancing incentives, option-adjusted duration, embedded-option risk, or deciding whether a bond with early-redemption features is fairly priced and suitable.
---

# Callable And Putable Bonds

Many corporate and municipal bonds carry embedded options that let the issuer (callable) or the investor (putable) end the bond early. These options are not free features — they have value, and that value is paid for by one side. A callable bond is a regular bond plus a call option the investor has sold to the issuer. Ignoring the option turns a structurally inferior instrument into an apparently cheap, high-yielding bond.

Use this skill before answering questions such as "why does this bond yield more", "what is a callable bond", "should I buy a bond that can be called early", or "what is option-adjusted duration". The goal is to prevent the agent from comparing callable bonds to non-callable bonds on yield alone, and from missing that the investor has sold valuable optionality.

## Core Rules

### Frame The Bond As Regular Bond Plus An Option Position

This reframing makes the economics clear:

- Callable bond = non-callable bond − call option held by the issuer.
- Putable bond = non-callable bond + put option held by the investor.
- The "extra yield" on a callable bond is the premium the investor receives for selling the call; it is compensation, not a gift.

Because the investor has sold a call, the callable bond has negative convexity in the region where call becomes likely: upside is capped (the bond gets called) while downside continues. This is a real, asymmetric risk.

### Understand When Each Option Gets Exercised

Options are exercised when they are in the money for the holder.

- Issuer calls when rates fall enough that refinancing the bond cheaper is economic, after accounting for the call premium and transaction costs.
- Investor puts when rates rise enough (or credit deteriorates) that redeeming at par and reinvesting elsewhere is better than holding.

For a callable bond, the call is most likely in a falling-rate, improving-credit environment — exactly when the investor would most want the bond to keep rising. The investor gives up the best outcomes. For a putable bond, the put protects the investor in rising-rate or deteriorating-credit scenarios.

### Use Option-Adjusted Spread And Option-Adjusted Duration

For embedded-option bonds, nominal yield, nominal spread, and modified duration are misleading. Use the option-adjusted measures:

- OAS: the spread after removing the option's value. This is the true credit/liquidity compensation and the right basis for comparison with option-free bonds.
- Option-adjusted (effective) duration: sensitivity to parallel curve shifts after accounting for how cash flows change with rates.
- Effective convexity: reveals the negative-convexity region of a callable bond.

A callable bond's modified duration can be much longer than its effective duration, because part of the rate sensitivity is "absorbed" by the call. Quoting modified duration overstates the bond's price response to falling rates.

### Price The Call Premium And Refinancing Incentive

The call schedule specifies call prices (often declining from a small premium over par toward par). The issuer's decision compares:

- the present value of continuing coupon payments;
- the cost of refinancing at current market rates;
- the call premium paid;
- transaction and registration costs.

When market yields fall meaningfully below the coupon minus the call premium, call becomes highly likely. Investors should assume bonds trading above par with a near call will be called, and price them to the call date, not to maturity. Quoting yield-to-maturity on a bond certain to be called misrepresents the return.

### Assess Negative Convexity And Its Cost

Negative convexity is the defining risk of callable bonds. Near the call trigger:

- when rates fall, the bond's price rises toward the call price and then stops (call caps appreciation);
- when rates rise, the bond's price falls normally.

The investor has sold the upside of a falling-rate scenario. This is why callable bonds yield more than otherwise-identical non-callable bonds. The extra yield must be judged as fair compensation for the option sold, not as free income. In a volatile-rate environment, negative convexity is costly because the bond underperforms on large moves in either direction.

### Match The Instrument To The Rate View And Liquidity Needs

The suitability of callable versus putable bonds depends on the investor's view and constraints:

- Expect falling rates / want price appreciation: avoid callable bonds (call caps the gain); prefer non-callable or putable.
- Want higher current income and accept call risk: callable bonds pay for it, but model the call scenario.
- Concerned about rising rates or credit deterioration: putable bonds offer downside protection, for which the investor pays via lower yield.
- Need a known maturity cash flow: embedded options create uncertainty; a non-callable bullet bond is cleaner.

State the rate and credit view, then check whether the embedded option helps or hurts that view.

### Check Liquidity, Make-Whole, And Non-Standard Features

Embedded-option bonds carry additional considerations:

- Liquidity is often thinner, especially for older callable munis and small corporate issues; bid-ask can be wide.
- Make-whole calls (common in corporates) allow call at any time at a price designed to make call uneconomic except in extreme cases — these behave closer to non-callable and should not be conflated with fixed-schedule calls.
- Sinking funds, deferral features, and contingent events add further optionality that must be read in the indenture.

Do not assume all "callable" bonds have the same economics. Read the specific call structure.

## Common Traps

### Comparing Callable To Non-Callable Bonds On Yield Alone

The higher yield on a callable bond is the premium for the call option sold. Comparing coupons without OAS treats compensation for an asymmetric risk as free income.

### Quoting Yield-To-Maturity For A Bond Likely To Be Called

If market yields are below the coupon net of call premium, the bond will almost certainly be called. Yield-to-maturity is then fictional; yield-to-call is the relevant measure.

### Ignoring Negative Convexity

Callable bonds underperform in volatile-rate environments because upside is capped. Treating them like positive-convexity bonds hides a structural drag.

### Treating All Calls As The Same

Fixed-schedule calls with declining premia, make-whole calls, and par calls have very different exercise incentives. A make-whole callable corporate behaves almost like non-callable; a par-callable muni does not.

### Assuming The Call Will Not Happen Because It Has Not Yet

Call decisions follow rates and credit. A bond that has not been called for years can be called immediately when refinancing becomes economic. Past non-exercise is not a commitment.

### Overlooking Putable Bonds' Lower Yield As The Cost Of Protection

A putable bond's lower yield is the premium the investor pays for downside protection. It is only worth it if the protection is likely to be valuable; otherwise the investor has overpaid.

## Self-Check

- [ ] The bond is framed as a regular bond plus an embedded option, and the investor's position (short call for callable, long put for putable) is explicit.
- [ ] Option-adjusted spread and option-adjusted (effective) duration are used, not nominal spread and modified duration.
- [ ] The call/put exercise incentive is modeled against current market yields, and yield-to-call is used when call is likely.
- [ ] Negative convexity for callable bonds is identified and judged as a cost the higher yield must compensate.
- [ ] The instrument is matched to the investor's rate and credit view and to liquidity and known-maturity needs.
- [ ] Specific call structure (fixed-schedule, make-whole, par call, sinking fund) is read from the indenture rather than assumed.
- [ ] The conclusion avoids presenting a callable bond's higher yield as attractive without naming the option sold and the investor-specific rate view.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
