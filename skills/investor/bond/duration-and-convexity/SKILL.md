---
name: duration_and_convexity.md
description: Use when the agent is analyzing bond interest-rate sensitivity, estimating price changes from yield moves, comparing modified versus effective duration, assessing convexity, constructing duration hedges, or deciding bond maturity and positioning across a rate cycle.
---

# Duration And Convexity

Duration is not "how long until the bond matures." It is the price sensitivity of a fixed-income cash flow to a change in yield. Treating duration as a maturity proxy, or ignoring convexity, leads to systematic errors in estimating how much a bond, fund, or portfolio will move when rates change.

Use this skill before answering questions such as "how much will this bond fund fall if rates rise", "what duration should I hold", "how do I hedge interest-rate risk", or "what is the difference between modified and effective duration". The goal is to prevent the agent from quoting a single duration number as if it fully describes rate risk, and to prevent naive linear approximations from being treated as exact.

## Core Rules

### Understand Which Duration Measure Is Being Used

Several numbers are all called "duration" but answer different questions. Using the wrong one produces wrong conclusions.

- Macaulay duration: the weighted-average time to receive cash flows, in years. Useful conceptually, not directly for price impact.
- Modified duration: the percentage price change for a small parallel yield change, assuming cash flows do not change. Good for option-free bonds.
- Effective (option-adjusted) duration: accounts for embedded options that change cash flows as rates move. Required for callable bonds, MBS, and any bond whose cash flows are rate-dependent.
- Key-rate / partial duration: sensitivity to specific points on the yield curve, not just parallel shifts.

For a callable bond or mortgage-backed security, modified duration can be badly wrong because the cash flows themselves shift with rates. Insist on effective duration for anything with embedded optionality.

### Treat Duration As A Linear Approximation With Limits

Duration gives the first-derivative (linear) price response. It is accurate only for small yield changes. For larger moves, the linear estimate diverges from the true price because the price-yield relationship is curved.

- For a yield decrease, duration understates the price gain.
- For a yield increase, duration overstates the price loss (bonds fall less than the linear rule predicts, when convexity is positive).

Always pair duration with convexity when estimating moves of more than a few tens of basis points, or when comparing bonds with very different convexity profiles.

### Use Convexity To Correct The Estimate And To Compare Bonds

Convexity is the second-derivative term. A rough price-change approximation is:

percentage price change ≈ (-duration × yield change) + (½ × convexity × yield change²)

Convexity matters more when:

- yield moves are large;
- the bond is long-duration (convexity scales with maturity squared);
- the bond has embedded options (callable bonds have negative convexity in some rate ranges);
- comparing two bonds with similar duration but different curvature.

Positive convexity is valuable: it means the bond gains more when rates fall and loses less when rates rise. Investors generally pay for positive convexity through lower yield. Negative convexity (callable bonds, MBS in refinance windows) means price appreciation is capped while downside continues — this is a real risk, not a curiosity.

### Separate Parallel-Shift Risk From Curve-Shape Risk

A single duration number assumes the entire yield curve moves in parallel. In reality, the curve can steepen, flatten, or invert. Two portfolios with identical effective duration can behave very differently if the curve twists.

Ask:

- Where on the curve is the exposure concentrated (bullet versus barbell)?
- Is the investor expressing a view on the level of rates, the slope, or the curvature?
- What happens if short rates move but long rates do not, or vice versa?

Use key-rate durations or scenario analysis (e.g., +50bp parallel, bear-steepener, bull-flattener) rather than relying on one number.

### Match Duration To The Liability Or Objective

The "right" duration depends on what the bond position is for. There is no universally correct duration.

- Matching a known future liability: choose duration close to the time horizon to immunize against rate moves.
- Income generation with low volatility: shorter duration reduces price swings but caps yield.
- Expressing a rate view: lengthen duration if falling rates are expected, shorten if rising rates are expected — but this is a directional bet, not free income.
- Holding to maturity: price volatility matters less if the investor can truly hold, but reinvestment risk and opportunity cost remain.

Do not recommend long duration as if it is simply "more yield" without naming the rate risk being taken.

### Distinguish Price Risk From Reinvestment Risk

These move in opposite directions along the rate cycle.

- Rising rates: bond prices fall (price risk), but coupons and maturities can be reinvested at higher yields (reinvestment benefit).
- Falling rates: bond prices rise, but reinvestment income declines.

Short duration minimizes price risk but maximizes reinvestment risk. Long duration does the opposite. The investor's time horizon and cash needs determine which risk matters more. Duration matching and immunization explicitly balance these two.

### Translate Fund Duration Into Realistic Loss Estimates

A common request is "what happens to my bond fund if rates rise 1%". A reasonable first estimate is:

approximate price change ≈ -effective duration × 1%

But this must be qualified:

- it ignores convexity and curve-shape changes;
- fund duration changes over time and is reported with a lag;
- corporate and emerging-market funds also carry spread risk that can dominate rate risk in stress;
- the yield income earned during the rate move partially offsets price loss over the holding period.

Always state the time frame. Over a one-year horizon, a fund's total return is roughly yield minus the duration-driven price change. Over longer horizons, yield compounds and price moves matter less.

## Common Traps

### Quoting Modified Duration For Callable Bonds Or MBS

Modified duration assumes fixed cash flows. For securities whose cash flows change with rates, only effective duration is meaningful. Using the wrong measure can understate or reverse the sign of the true sensitivity.

### Ignoring Negative Convexity

Callable bonds and MBS can exhibit negative convexity, meaning the price-yield curve bends the wrong way: upside is capped (the bond gets called or prepaid) while downside continues. Treating these like ordinary positive-convexity bonds hides a structural risk.

### Treating "Low Duration" As "Safe"

Short-duration bonds have low rate risk, but they still carry credit risk, reinvestment risk, and inflation risk. A short-duration high-yield fund can lose far more from spread widening than from duration.

### Assuming The Yield Curve Moves In Parallel

Most real rate moves reshape the curve. A duration number that assumes parallel shifts can be correct on average and wrong in the specific scenario that occurs.

### Confusing Yield With Return On Long Bonds

A long-duration bond may offer a high yield, but most of the expected return can be wiped out by a modest rate rise over the investor's actual holding period, which may be far shorter than maturity.

### Forgetting That Hedging Costs Money And Adds Basis Risk

Duration hedges (e.g., short Treasury futures, pay-fixed swaps) reduce rate risk but introduce basis risk, margin requirements, rolling costs, and counterparty considerations. A hedge that is imperfect can leave residual exposure or create new risks.

## Self-Check

- [ ] The correct duration measure (modified versus effective/option-adjusted) is used for the security type, especially for callable bonds and MBS.
- [ ] Convexity is included when estimating price moves beyond small parallel shifts, and its sign is checked (positive versus negative).
- [ ] The analysis distinguishes parallel-shift risk from curve-shape risk and uses key-rate durations or scenarios where relevant.
- [ ] Duration is matched to a stated objective or liability horizon, not chosen as a generic "more yield" decision.
- [ ] Price risk and reinvestment risk are both addressed, and the tradeoff between short and long duration is made explicit.
- [ ] Fund-duration loss estimates include convexity, spread risk, yield offset over the holding period, and the time frame of the estimate.
- [ ] Any duration hedge names its basis risk, rolling cost, margin impact, and residual exposure.
- [ ] The conclusion avoids presenting a duration choice as suitable without investor-specific horizon, cash needs, and rate-view tolerance.
