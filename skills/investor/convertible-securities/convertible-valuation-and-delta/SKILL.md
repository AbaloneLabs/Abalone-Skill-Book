---
name: convertible_valuation_and_delta.md
description: Use when the agent is valuing a convertible bond, computing its delta and sensitivity to the underlying stock, assessing implied volatility and option value, comparing theoretical value to market price, or understanding how a convertible's price responds to equity, volatility, credit, and interest rate changes.
---

# Convertible Valuation And Delta

Valuing a convertible bond requires modeling an instrument whose value depends simultaneously on equity, volatility, credit, and interest rates. Unlike a straight bond, whose value is driven by rates and credit, or a stock option, whose value is driven by the underlying price and volatility, a convertible's value is a function of all of these, interacting through the embedded conversion option and the bond floor. The practical output of valuation is not a single number but a profile of sensitivities: how much the convertible's price moves when the stock moves (delta), when volatility changes (vega), when rates or credit shift, and how that curvature (gamma) evolves. Investors who quote a theoretical value without these sensitivities, or who rely on a simple yield calculation, miss how the convertible actually behaves and where its risks and opportunities lie.

Use this skill when valuing convertibles, computing Greeks, assessing whether a convertible is rich or cheap relative to its model value, or explaining why a convertible's price moved differently than expected. The goal is to prevent the agent from presenting a point-estimate valuation without sensitivities, from ignoring the credit and rate inputs that move the bond floor, and from confusing a convertible's delta with a stock's beta.

## Core Rules

### Use A Model That Captures Both The Bond And The Option

A convertible's value cannot be captured by discounting cash flows alone (which ignores the option) or by a pure equity option model (which ignores the floor). Appropriate valuation uses models that combine the two:

- Binomial or trinomial lattice models that embed the bond's cash flows and credit spread while valuing the conversion option at each node.
- Finite difference or Monte Carlo methods for more complex features (resets, path-dependent calls).

The model must incorporate the issuer's credit spread (hazard rate or spread over risk-free), the risk-free rate, the stock's volatility and dividend yield, and the convertible's full term structure (coupons, calls, puts, resets). A model that omits any of these produces a theoretical value that diverges systematically from the market.

### Compute And Interpret Delta Correctly

Delta measures how much the convertible's price changes for a small change in the underlying stock:

- Deep in-the-money convertibles have delta near 1 (move like stock).
- At-the-money convertibles have delta around 0.5-0.6, with positive convexity.
- Busted convertibles have delta near 0 (move like bonds, insensitive to small stock moves).

Delta is not constant; it changes with the stock price (gamma) and with time and volatility. An investor estimating P&L from a stock move using a single delta value will be wrong for larger moves because gamma and the shifting regime alter the sensitivity. Delta also defines the hedge ratio for arbitrageurs: to delta-hedge, short delta times the conversion ratio in shares per bond.

### Assess Gamma, Convexity, And Optionality

Gamma measures how delta changes as the stock moves, reflecting the convertible's convexity:

- At-the-money convertibles have the highest gamma; their delta changes rapidly, making them valuable when volatility is high.
- Deep in-the-money and busted convertibles have low gamma; they behave more linearly.

Convexity is the key advantage of convertibles: they participate in upside (delta rises toward 1) while the floor limits downside (delta falls toward 0). An investor who ignores gamma misjudges how the convertible's risk profile evolves as the stock moves, and an arbitrageur who ignores gamma cannot manage a dynamic hedge.

### Decompose Value Into Bond Floor, Option Value, And Spread

Breaking the theoretical value into components reveals what drives the price and where mispricing may lie:

- Bond floor: present value of cash flows at the credit-adjusted discount rate.
- Option value: value of the embedded conversion option, sensitive to volatility and time.
- Cheapness or richness: the difference between market price and theoretical value, often expressed as an implied volatility spread or a price gap.

If a convertible trades below its bond floor, either the market expects severe credit deterioration, the model's credit input is too tight, or there is a genuine mispricing. If it trades at a low implied volatility relative to the stock's realized or option-implied volatility, the embedded option may be cheap.

### Stress The Credit And Rate Inputs, Not Just The Stock

The bond floor moves with credit spreads and interest rates, yet analysts often focus on equity scenarios and treat credit and rates as fixed:

- Widening credit spreads drop the floor, sometimes dramatically for lower-rated issuers.
- Rising rates lower the floor, though the equity component can offset if the stock rises.
- Stock declines and credit widening often coincide for leveraged issuers, compounding the floor's erosion.

Run valuations across credit spread and rate scenarios, not just stock price scenarios, especially for convertibles of lower-rated issuers where the floor is credit-sensitive.

### Reconcile Model Value With Market Price And Liquidity

A theoretical value is only useful when compared to the market price and to the realities of trading:

- Bid-ask spreads on convertibles can be wide, especially for smaller issues; a model "cheapness" of 1-2 points may be inside the spread.
- Convertible liquidity varies; large positions cannot always be established or exited at mid-market.
- New issue pricing often reflects distribution dynamics rather than pure model value.

Do not conclude a convertible is "cheap" based on model value alone without accounting for liquidity, the reliability of inputs (especially volatility and credit), and the cost of carrying a position.

## Common Traps

### Quoting A Single Theoretical Value Without Sensitivities

A point estimate without delta, gamma, vega, and credit sensitivity is not actionable. The value of valuation is the risk profile, not the number.

### Ignoring The Credit Input In The Bond Floor

Treating the discount rate as risk-free or using a stale credit spread understates floor sensitivity and overstates downside protection for weaker issuers.

### Confusing Convertible Delta With Stock Beta

Delta is an option sensitivity that changes with the stock; beta is a regression coefficient. Applying stock-style beta thinking to a convertible misestimates its equity sensitivity.

### Over-Relying On Implied Volatility Without Validation

Implied volatility backed out of a convertible price can be distorted by credit, liquidity, or model error. Cross-check against listed option implied vol and realized vol before concluding the option is cheap or rich.

### Treating Model Cheapness As Tradeable Without Liquidity

A model gap inside the bid-ask spread, or in an illiquid issue, is not exploitable. Account for transaction costs and position size limits.

### Stressing Only Equity Scenarios

Credit and rate moves can dominate the floor's behavior, especially in distress. A valuation that fixes credit and rates misses compounding risks.

## Self-Check

- Does the valuation use a model that captures both the bond floor and the conversion option, with credit spread, rate, volatility, and dividend inputs?
- Is delta computed and interpreted across regimes, with the understanding that delta changes with the stock (gamma) and is not constant?
- Is gamma and convexity assessed, recognizing that at-the-money convertibles carry the most optionality?
- Is the theoretical value decomposed into bond floor, option value, and cheapness/richness, with implied volatility cross-checked against market alternatives?
- Are credit spread and interest rate scenarios stressed, not just equity scenarios, especially for lower-rated issuers?
- Is the model value reconciled with the market price, accounting for bid-ask spreads, liquidity, and the reliability of inputs?
- Is the convertible's behavior understood as regime-dependent, with the analysis covering in-the-money, at-the-money, and busted cases?
- Does the conclusion avoid presenting valuation as a single number and instead convey the sensitivity profile that drives risk and opportunity?
