---
name: startup_valuation.md
description: Use when the agent is valuing startups and early-stage companies, distinguishing pre-money and post-money valuation, comparing valuation methods across stages, assessing growth and risk and dilution, and judging whether a startup's valuation reflects realistic outcomes and market conditions.
---

# Startup Valuation

Startup valuation is fundamentally different from valuing a mature business. There is little or no revenue, no profits, no comparable multiples, and enormous uncertainty about whether the company will survive, grow modestly, or become a category winner. Methods that work for public companies (DCF, P/E) are useless or misleading. Instead, startup valuation is a negotiation between founders and investors that reflects stage, growth, market size, team, comparable rounds, and the balance of supply and demand for capital. Treating a startup valuation as a precise, DCF-justified number — or accepting the headline valuation without unpacking the terms and dilution — leads to overpaying for companies whose path to a return is unclear.

Use this skill before answering questions such as "how do I value a startup", "is this startup valuation fair", "what is pre-money versus post-money", or "how much should I invest at this valuation". The goal is to prevent the agent from applying mature-company valuation methods to startups, and from confusing a negotiated round price with intrinsic value.

## Core Rules

### Distinguish Pre-Money From Post-Money And Compute Ownership

The valuation headline must be paired with the investment to compute ownership:

- Pre-money valuation: the agreed value of the company before the new investment.
- Post-money valuation: pre-money plus the new investment.
- Ownership: investment / post-money valuation. A $5M investment at a $20M pre-money ($25M post-money) buys 20%.
- Option pool adjustment: if a new option pool is created and included in the pre-money, founders bear the dilution, effectively lowering their pre-money. Check whether the pool is pre- or post-money.

Always compute the post-money and the resulting ownership, and adjust for option pool sizing. The pre-money number alone is incomplete.

### Match The Valuation Method To The Stage

Valuation methods differ by stage because the available information differs:

- Pre-seed / seed: little revenue; valuation driven by team, market size, product vision, traction signals, and comparable rounds. Often set by negotiation and benchmark ("comparable seed rounds in this sector raise at $X"). Scorecard and venture capital (Berkus) methods attempt to systematize but are rough.
- Series A: early revenue and metrics (ARR, growth rate, unit economics); valuation driven by growth and market. Revenue multiples begin to apply but with wide dispersion.
- Growth stages (B/C+): meaningful revenue and metrics; valuation increasingly driven by revenue multiples, growth, and comparable public/private multiples.
- Pre-IPO / late stage: closer to public-market multiples, with discounts for illiquidity and lockup.

Applying a revenue multiple to a pre-revenue seed company, or a DCF to a growth-stage company, misuses the method. Match the method to the stage and the available data.

### Assess The Market Size And Path To A Return

A startup's value depends on whether it can grow into an outcome that returns the fund:

- TAM/SAM/SOM: total, serviceable, and obtainable market. A large market is necessary (but not sufficient) for a venture-scale return.
- Path to scale: how does the company grow revenue 10-100x? Is the growth path credible and capital-efficient?
- Return multiple required: a VC needs the company to return the fund (or a meaningful multiple) on winners. At a $100M post-money, the company must reach $1B+ for a 10x; at a $1B post-money, it must reach $10B+.
- Exit pathways: IPO, acquisition; what acquirers exist, and at what multiples?

A high valuation requires a proportionally large exit to generate venture returns. Buying at a $500M post-money means betting on a multi-billion outcome. Assess whether the market and path support the required exit.

### Evaluate Growth, Unit Economics, And Traction

Quantitative signals become more relevant at later stages:

- Growth rate: revenue growth (MoM, YoY); high growth supports higher multiples.
- Retention and cohorts: net revenue retention, churn, cohort expansion; retention quality predicts durability.
- Unit economics: gross margin, CAC, LTV, payback period; negative or uncertain unit economics require a clear path to improvement.
- Capital efficiency: revenue per dollar of capital burned; efficient growth supports higher valuations.

At seed, traction may be qualitative (LOIs, pilots, waitlists); at growth, it is quantitative. Match the rigor of the analysis to the stage, but never ignore the path to sustainable unit economics.

### Account For Dilution Across Rounds And The Exit Required

Ownership today is not ownership at exit; dilution reduces it:

- Future rounds: each subsequent round dilutes existing investors unless they exercise pro-rata rights.
- Option pool expansion: future hires and option grants dilute all holders.
- Down rounds: a lower-priced future round can impose anti-dilution adjustments and reset the cap table.
- Exit ownership: an investor owning 20% at Series A may own 5-10% at exit after dilution.

Model ownership at exit under dilution assumptions. The return depends on exit valuation × exit ownership, net of preferences. A 20% stake at a high valuation can deliver less than a 10% stake at a lower one if dilution and preferences differ.

### Judge The Realism Of The Valuation Against Comparables And Conditions

Valuations are set by negotiation and are sensitive to conditions:

- Comparable rounds: recent rounds in the same sector, stage, and geography; the most concrete benchmark.
- Market conditions: capital availability, risk appetite, and sector heat compress or expand valuations; valuations set in booms may not hold in downturns (down rounds).
- Term quality: a high valuation with heavy preferences or control terms may be economically worse than a lower valuation with clean terms (see term-sheet skill).
- "Mark-up" risk: buying at a valuation set by a hot market risks a down round if conditions change.

Benchmark the valuation against comparable rounds and adjust for term quality and market conditions. A valuation that looks high relative to comparables in a cool market is a warning; one that looks low in a hot market may be an opportunity.

### Separate Valuation From Probability Of Success

A startup valuation embeds an implicit probability of large outcomes:

- Power-law outcomes: most startups fail or return little; a few return 10-100x+. The valuation must be low enough that the winners, weighted by probability, cover the losers (see power-law skill).
- Expected value: EV = probability of success × success value − probability of failure × loss. A $20M valuation is attractive only if the probability-weighted exit value exceeds it meaningfully.
- Stage risk: earlier stages have higher failure rates but lower valuations; later stages have lower failure rates but higher valuations requiring larger exits.

Do not value a startup as if its success is certain. The valuation must leave room for the probability of failure and the power-law distribution of outcomes.

## Common Traps

### Applying Mature-Company Methods (DCF, P/E) To Startups

Startups lack the cash flows, comparables, and predictability these methods require. Use stage-appropriate methods.

### Confusing The Negotiated Round Price With Intrinsic Value

A round valuation is a negotiated price set by supply and demand, not a DCF-justified intrinsic value. It can be wrong and can reverse in down rounds.

### Ignoring Option Pool And Dilution Effects

Pre- versus post-money option pool and future-round dilution materially affect ownership and returns. Model ownership at exit, not just at investment.

### Overpaying In Hot Markets

Valuations set in capital-abundant booms often do not hold; later down rounds reset prices and dilute or wipe out earlier investors.

### Valuing Without Assessing The Required Exit

A high valuation requires a proportionally large exit for venture returns. Assess whether the market and path support the exit needed.

### Treating Success As Certain

Startup outcomes are power-law distributed. Valuation must account for the high probability of failure, not just the upside scenario.

## Self-Check

- [ ] Pre-money, post-money, ownership, and option pool adjustment are computed correctly.
- [ ] The valuation method matches the stage (seed negotiation/benchmarks; growth revenue multiples; late-stage public comps with illiquidity discount).
- [ ] Market size (TAM/SAM/SOM), path to scale, and the exit required to generate venture returns are assessed.
- [ ] Growth, retention/cohort, unit economics, and capital efficiency are evaluated appropriate to the stage.
- [ ] Dilution across future rounds, option pool expansion, and down-round risk are modeled to estimate exit ownership.
- [ ] The valuation is benchmarked against comparable rounds and adjusted for term quality and market conditions.
- [ ] Probability of success and power-law outcomes are factored into expected value, not assumed away.
- [ ] The conclusion avoids presenting a startup valuation as precise and references investor-specific fund requirements, stage, and risk tolerance.
