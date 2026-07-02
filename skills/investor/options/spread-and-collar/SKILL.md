---
name: spread_and_collar.md
description: Use when the agent is evaluating options spread and collar strategies, assessing directional and volatility exposure, cost and defined-risk structure, payoff and breakeven analysis, early exercise and assignment risk, and how multi-leg structures alter risk relative to naked option buying or selling.
---

# Spread And Collar

Options spreads (vertical, horizontal, diagonal) and collars combine multiple option legs to create defined-risk, defined-reward payoffs. They can reduce cost, cap risk, or tailor directional and volatility exposure more precisely than single-leg options. But multi-leg structures introduce complexity: each leg has its own Greeks, the legs interact, early exercise and assignment risk differs from naked options, and the defined-risk label can mask the conditions under which the structure still loses money. Evaluating spreads and collars requires understanding the net payoff diagram, the net Greeks, the cost and capital efficiency, and the operational risks of managing multiple legs — not just the headline that the strategy is "defined risk."

Use this skill before answering questions such as "should I use a bull call spread", "how does a collar work", "what is the risk of an iron condor", or "are spreads safer than naked options". The goal is to prevent the agent from treating spreads as automatically safer or cheaper without analyzing the net payoff, net Greeks, and operational risks.

## Core Rules

### Map The Net Payoff Diagram And Breakeven

The payoff diagram is the definitive description of a spread or collar:

- Maximum profit and loss: spreads and collars define both the maximum gain and maximum loss; identify these precisely and the conditions (underlying price at expiration) under which each occurs.
- Breakeven: the underlying price at which the strategy breaks even; breakeven accounts for the net premium paid or received.
- Payoff shape: vertical spreads have linear (kinked) payoffs; iron condors and butterflies have tent-shaped payoffs; understand where the strategy makes and loses money.
- Probability of profit: the breakeven relative to the current price and implied volatility implies an approximate probability of profit; assess whether the risk-reward is favorable.

Draw or describe the full payoff diagram, not just the max profit/loss. The shape reveals where the strategy is vulnerable.

### Analyze Net Greeks And Volatility Exposure

Multi-leg structures have net Greeks that differ from single legs:

- Net delta: the directional exposure; a bull call spread is net long delta; an iron condor is approximately delta-neutral at inception.
- Net gamma: how delta changes as the underlying moves; spreads have lower gamma than naked options, meaning less delta drift but also less convexity.
- Net vega: volatility exposure; long-volatility structures (debit spreads, long straddles) benefit from rising implied volatility; short-volatility structures (credit spreads, iron condors) suffer from rising volatility.
- Net theta: time decay; debit spreads suffer time decay (negative theta); credit spreads benefit from time decay (positive theta).
- Pin risk near expiration: near expiration, ATM options have high gamma; spreads with short legs near the money face assignment uncertainty (pin risk).

Analyze the net Greeks to understand directional, volatility, and time-decay exposure. A credit spread is a short-volatility, positive-theta trade; a debit spread is a long-volatility, negative-theta trade. These are different bets.

### Assess Cost, Capital Efficiency, And Margin

Spreads affect capital efficiency and margin:

- Debit spreads (e.g., bull call spread): the net premium paid is the maximum loss; capital at risk is the debit paid; no margin required beyond the debit.
- Credit spreads (e.g., bull put spread): the net premium received; maximum loss is the spread width minus the credit; margin is required to cover the maximum loss.
- Iron condors and butterflies: defined-risk structures with margin equal to the widest spread leg; capital-efficient relative to naked selling.
- Collars: long stock, long put (downside protection), short call (finances the put); the short call may require no margin if covered by the stock; the structure is a hedged equity position.

Assess the capital required and margin treatment. Defined-risk spreads are more capital-efficient than naked selling, but credit spreads still tie up margin.

### Evaluate Early Exercise, Assignment, And Pin Risk

Multi-leg structures have specific operational risks:

- Early assignment (American options): short option legs can be assigned early, especially if deep ITM or around dividends; assignment of a short leg can create an unwanted stock position or disrupt the spread.
- Dividend-related early exercise: short calls around ex-dividend dates face early-exercise risk; short deep-ITM puts face early-exercise risk if the put's time value is below the carry.
- Pin risk near expiration: spreads with a short leg near the money at expiration face uncertainty about whether the short leg will be assigned; if assigned, the resulting position may be unhedged over the weekend.
- Leg risk: closing or adjusting one leg at a time exposes the investor to gap risk between leg executions; spread execution as a single package reduces leg risk.

Plan for early assignment, pin risk, and leg risk. American-style short legs, especially around dividends and expiration, require active management.

### Match The Structure To The Market View And Objective

Different structures fit different views:

- Directional (vertical spreads): bull call/put spreads for bullish views with defined risk and lower cost than naked calls; bear call/put spreads for bearish views.
- Volatility (straddles, strangles, iron butterflies): long straddles for large moves in either direction (long volatility); iron condors for range-bound markets (short volatility).
- Hedging (collars, protective puts): collars hedge a long stock position at near-zero net cost by selling upside (short call) to finance downside protection (long put).
- Income (credit spreads): credit spreads generate income in flat-to-favorable markets with defined risk.

Match the structure to the specific view (direction, volatility range, hedging need). Using a structure that does not match the view — e.g., an iron condor when a large move is expected — guarantees a poor outcome.

### Understand Tax Treatment And Constructive-Sale Rules

Spread and collar strategies have tax implications:

- Constructive sale (collars): a tight collar (strikes close enough) can be deemed a constructive sale under tax rules, triggering a realization of the underlying's gain; collar strikes must be spaced to avoid constructive-sale treatment if the goal is to defer gain.
- Section 1256 (broad-based index options): certain index options are subject to mark-to-market treatment with 60/40 capital gains treatment; equity options are not.
- Straddle rules: offsetting positions (straddles) can trigger loss-deferral rules; spread strategies may be subject to straddle rules.
- Wash-sale interaction: losses on options and the underlying can trigger wash-sale rules.

Tax rules (constructive sale, Section 1256, straddle rules, wash-sale) affect after-tax return. Tight collars can trigger unwanted realization; understand the rules before implementing.

## Common Traps

### Treating Defined-Risk As Risk-Free

Defined-risk means the maximum loss is bounded, not that loss is unlikely. Credit spreads can still realize the maximum loss, especially in fast-moving markets.

### Ignoring Net Volatility (Vega) Exposure

Credit spreads and iron condors are short volatility; rising implied volatility hurts them even if the underlying doesn't move. Debit spreads are long volatility.

### Overlooking Pin Risk And Early Assignment Near Expiration

Short legs near the money at expiration face assignment uncertainty; deep-ITM short legs around dividends face early-exercise risk. These disrupt the structure.

### Selecting A Structure That Does Not Match The Market View

Using an iron condor when a large move is expected, or a directional spread when volatility is the real bet, guarantees a poor outcome. Match structure to view.

### Forgetting Constructive-Sale And Straddle Tax Rules

Tight collars can trigger constructive-sale realization; spread straddles can trigger loss-deferral rules. Tax rules affect after-tax return.

### Underestimating Leg-Execution Risk

Closing legs separately exposes the investor to gap risk. Execute spreads as packages and plan management in advance.

## Self-Check

- [ ] The full net payoff diagram, maximum profit/loss, and breakeven are mapped, not just the headline max values.
- [ ] Net Greeks (delta, gamma, vega, theta) are analyzed for directional, volatility, and time-decay exposure.
- [ ] Cost, capital efficiency, and margin treatment (debit vs. credit spreads, iron structures, collars) are understood.
- [ ] Early-exercise, assignment, and pin risk (especially around dividends and expiration) are planned for.
- [ ] The structure is matched to the specific market view (direction, volatility range, hedging objective), not selected generically.
- [ ] Tax treatment (constructive sale for tight collars, Section 1256, straddle rules, wash-sale) is understood.
- [ ] Leg-execution risk is mitigated by package execution and advance management planning.
- [ ] The conclusion avoids presenting spreads as automatically safer, references the specific risk-reward and net Greeks, and includes appropriate risk disclosure and the need for options-trading experience.
