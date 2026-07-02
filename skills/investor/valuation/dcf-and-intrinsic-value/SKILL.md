---
name: dcf_and_intrinsic_value.md
description: Use when the agent is building or evaluating a discounted cash flow model, choosing a discount rate or growth assumption, handling terminal value, testing sensitivity, building scenarios, or understanding the limits of intrinsic value estimation before forming an investment view.
---

# DCF And Intrinsic Value

A discounted cash flow model estimates the value of a business as the present value of its expected future cash flows. In principle it is the most rigorous valuation method, because value ultimately comes from cash. In practice it is the most assumption-sensitive, and small changes in the discount rate, growth rate, or terminal value produce wildly different answers. An investing agent often builds a DCF, gets a number, and presents it as the intrinsic value, hiding the fact that the output is almost entirely determined by a few heroic assumptions. A DCF is not a truth machine; it is a structured way to expose what you are assuming.

Use this skill before answering questions such as "what is this company worth", "how do I build a DCF", "is this stock undervalued based on intrinsic value", or "what discount rate should I use". The goal is to prevent the agent from presenting a single point-estimate DCF as authoritative and to force it to justify each key assumption, run sensitivity and scenario analysis, respect the dominance of terminal value, and acknowledge the limits of intrinsic-value precision.

Intrinsic value estimation is inherently uncertain and model-dependent. A DCF is a tool for disciplined thinking, not a guarantee of future return. Conclusions should disclose the range of uncertainty and investor-specific context.

## Core Rules

### Justify The Discount Rate Explicitly

The discount rate, usually the weighted average cost of capital, has an outsized effect on DCF output. It must be chosen deliberately, not defaulted.

Consider:

- The risk-free rate and the equity risk premium in the current regime.
- The company's beta or systematic risk, adjusted for cyclicality and leverage.
- The cost of debt after tax, reflecting the actual capital structure.
- A margin of safety or additional risk premium for small, illiquid, or uncertain businesses.
- Whether the rate is real or nominal, matching the cash flow projections.

A discount rate that is too low inflates value; too high destroys it. Show the components and test how value changes across a plausible range. Do not present a single rate as obviously correct.

### Question The Growth Rate Assumptions

Growth assumptions drive the bulk of value in most DCFs, and they are the most over-optimistic input.

Test:

- Is the near-term growth rate consistent with the company's history, industry size, and competitive position?
- Does the growth rate imply the company eventually capturing an implausible share of the market or economy?
- Is growth driven by volume, price, or margin expansion, and is each plausible?
- Over what period does growth fade, and to what long-run rate?
- Is the growth rate above or below GDP, and why?

Growth cannot exceed the economy indefinitely. Any model with perpetual high growth is mathematically and economically suspect. Fade growth toward a sustainable long-run rate, and justify the fade period.

### Handle Terminal Value With Humility

Terminal value, the value of cash flows beyond the explicit forecast, typically dominates DCF output, often 70 percent or more. This makes the terminal assumptions the real drivers of the answer.

Approaches and cautions:

- Gordon growth terminal value requires a perpetual growth rate below the economy's long-run rate; even small changes swing the result enormously.
- Exit-multiple terminal value assumes a valuation multiple at the end of the forecast, which imports relative-valuation assumptions into a supposedly intrinsic method.
- Check what share of total value comes from terminal value; if it is very high, the explicit forecast is doing little work, and the result rests on the terminal assumption.
- Stress-test terminal value across a range of growth rates and exit multiples.

If terminal value is 80 percent of the total, you are not really valuing the explicit cash flows; you are valuing a perpetuity assumption. Acknowledge this and treat the output accordingly.

### Run Sensitivity Analysis On Every Key Input

Because DCF output is so assumption-sensitive, a single point estimate is misleading. Always show how value changes across plausible input ranges.

Build a sensitivity table:

- value across a grid of discount rates and growth rates;
- value across margin and capex assumptions;
- value across different terminal value approaches;
- the break-even assumptions required to justify the current market price.

Sensitivity analysis reveals which assumptions matter and exposes whether the conclusion is robust or hangs on one fragile input. If a small change in the growth rate flips the stock from cheap to expensive, the DCF is not providing a precise answer; it is showing that the valuation depends on an unknowable future.

### Build Multiple Scenarios, Not One Base Case

A single base-case forecast hides uncertainty. Construct several scenarios reflecting different futures.

Use:

- A bull case reflecting strong execution, high growth, and margin expansion.
- A base case reflecting consensus-like, realistic assumptions.
- A bear case reflecting competitive pressure, margin compression, and slower growth.
- Probability-weight the scenarios rather than relying on the base case alone.

Scenario analysis forces the agent to confront the range of outcomes and prevents false precision. The probability weights are themselves assumptions, but they make the uncertainty explicit.

### Match The Forecast Horizon To The Business Predictability

The appropriate forecast horizon depends on how predictable the cash flows are. Forcing a long explicit forecast on an unpredictable business produces noise.

Calibrate:

- Stable, mature businesses with predictable cash flows can support longer explicit forecasts.
- Young, high-growth, or cyclical businesses have less predictable cash flows; shorter explicit forecasts with wider scenario ranges are more honest.
- Some businesses, like early-stage tech or biotech, may not have meaningful near-term cash flows at all, making DCF speculative.

Do not pretend a 10-year explicit forecast of an unpredictable business is reliable. If predictability is low, the DCF should reflect that through wider ranges and lower confidence, not a false precise number.

### Acknowledge The Limits Of Intrinsic Value Precision

DCF produces a precise-looking number, but precision is not accuracy. Intrinsic value is a range, not a point.

Recognize:

- Small input changes produce large output changes, so the "intrinsic value" is better expressed as a range.
- The model reflects the analyst's assumptions, which may be wrong.
- Intrinsic value and market price can diverge for long periods; being "right" on value does not guarantee near-term return.
- DCF is most useful as a framework for disciplined thinking about what drives value, not as a price target.

Use the DCF to understand what assumptions are embedded in the current price and whether those assumptions seem reasonable, rather than to produce a single authoritative value.

## Common Traps

### Presenting A Single Point Estimate As Intrinsic Value

A DCF output is assumption-dependent. Presenting one number hides the uncertainty and overstates confidence.

### Defaulting The Discount Rate Without Justification

The discount rate dominates the result. Using a default like 10 percent without justifying the components produces an arbitrary answer.

### Assuming Unrealistic Perpetual Growth

Growth above the economy forever is impossible. Terminal growth rates must be conservative and justified.

### Letting Terminal Value Dominate Silently

When terminal value is 80 percent of the total, the explicit forecast is nearly irrelevant. Acknowledge this rather than hiding behind a detailed near-term model.

### Ignoring Sensitivity And Scenarios

A single base case hides how fragile the conclusion is. Sensitivity and scenario analysis are essential, not optional.

### Over-Precision On Unpredictable Businesses

A long detailed forecast of an unpredictable business is false precision. Match the horizon and ranges to predictability.

### Treating The DCF As A Price Target

Intrinsic value and market price diverge for long periods. A DCF is a thinking tool, not a guarantee of convergence.

## Self-Check

- [ ] The discount rate is justified by its components and tested across a plausible range.
- [ ] Growth rate assumptions are tested for plausibility, market-size limits, and a justified fade.
- [ ] Terminal value is handled with humility, and its share of total value is reported.
- [ ] Sensitivity analysis is run across all key inputs, showing how value changes.
- [ ] Multiple scenarios, bull, base, and bear, are built and probability-weighted.
- [ ] The forecast horizon matches the business's predictability, with wider ranges where uncertainty is high.
- [ ] Intrinsic value is expressed as a range, not a single precise point.
- [ ] The analysis identifies what assumptions are embedded in the current market price.
- [ ] The DCF is used as a disciplined thinking framework, not a price target.
- [ ] The conclusion discloses the uncertainty, avoids false precision, and flags investor horizon and professional advice.
