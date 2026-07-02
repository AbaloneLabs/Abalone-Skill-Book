---
name: hedging_with_futures.md
description: Use when the agent is evaluating futures for hedging equity, commodity, currency, or interest-rate exposure, assessing beta and cross hedging, basis risk, hedge ratios, rolling, liquidity, and the gap between a theoretically perfect hedge and the residual risks that remain in real hedging programs.
---

# Hedging With Futures

Futures are a primary tool for hedging exposure to equities, commodities, currencies, and interest rates. The appeal is capital efficiency and the ability to reduce or transfer risk without selling the underlying. But hedging with futures is not risk elimination — it exchanges price risk for basis risk (the imperfect correlation between the futures and the hedged exposure), introduces rolling and liquidity demands, and can itself create losses if the hedge and the underlying diverge. A "hedged" position that ignores basis risk, hedge-ratio drift, rolling costs, and liquidity needs can leave the investor exposed to exactly the risk they sought to mitigate. Evaluating a futures hedge requires understanding the hedge ratio, basis risk, the cross-hedge mismatch, and the operational demands of maintaining the hedge.

Use this skill before answering questions such as "should I hedge my portfolio with futures", "how do I hedge currency exposure", "what is basis risk", or "how do I calculate a hedge ratio". The goal is to prevent the agent from treating futures hedging as risk elimination, and from ignoring basis risk, hedge-ratio drift, rolling, and liquidity demands.

## Core Rules

### Understand That Hedging Exchanges Price Risk For Basis Risk

A futures hedge does not eliminate risk; it transforms it:

- Basis risk: the risk that the futures price and the hedged exposure do not move together perfectly; the residual risk after hedging. Basis = spot − futures (or futures − spot, by convention).
- Cross hedge: when the hedging instrument differs from the exposure (e.g., hedging a small-cap portfolio with S&P 500 futures), the imperfect correlation creates cross-hedge basis risk.
- Perfect hedge assumption: a perfect hedge requires the futures and the exposure to be identical and perfectly correlated; in practice, hedges are imperfect and leave residual basis risk.
- Basis risk realization: if the hedge and the exposure diverge (basis widens), the hedge loses money even if the hedging decision was correct; basis movements can create unexpected gains or losses.

Frame hedging as risk transformation, not risk elimination. Quantify and accept the basis risk; do not assume the hedge is perfect.

### Calculate The Hedge Ratio Appropriately

The hedge ratio determines how many futures contracts to use:

- Naive hedge ratio: a 1:1 hedge (one dollar of futures per dollar of exposure); simple but ignores beta and correlation differences.
- Beta-adjusted (minimum-variance) hedge ratio: for equity hedges, adjust the number of contracts by the beta of the portfolio relative to the futures underlying; HR = beta × (portfolio value / futures notional).
- Regression-based hedge ratio: for cross hedges, regress the exposure's returns on the futures returns; the slope coefficient is the minimum-variance hedge ratio; the R-squared indicates hedge effectiveness.
- Hedge ratio drift: the optimal hedge ratio changes over time as beta and correlations shift; periodic rebalancing is needed to maintain the hedge.

Calculate the hedge ratio using beta adjustment or regression, not a naive 1:1 ratio. Recognize that the ratio drifts and requires rebalancing.

### Assess Cross-Hedge Mismatch And Effectiveness

Cross hedges introduce mismatch:

- Underlying mismatch: hedging a non-index portfolio with index futures; the portfolio's sector, factor, or stock-specific exposure is not hedged.
- Factor and tracking risk: the hedge neutralizes market (beta) risk but leaves factor, sector, and idiosyncratic risk; a portfolio can lose value relative to the hedge if its factors underperform.
- R-squared and effectiveness: the regression R-squared indicates how much of the exposure's variance the hedge explains; low R-squared means poor hedge effectiveness.
- Currency cross hedge: hedging one currency exposure with a different (but correlated) currency's futures; correlation breaks can leave residual FX risk.

Assess the cross-hedge mismatch via beta, correlation, and R-squared. A low-effectiveness cross hedge may not justify the cost and complexity.

### Manage Rolling And Its Cost Impact

Futures hedges require rolling:

- Roll schedule: contracts expire and must be rolled to maintain the hedge; rolling incurs transaction costs and potential roll yield (positive or negative).
- Roll cost: in contango, rolling incurs negative roll yield (a cost); in backwardation, rolling generates positive roll yield; the term structure affects the cost of carrying the hedge.
- Roll timing and slippage: rolling during illiquid periods or crowded rolls incurs slippage; rolling is an ongoing operational demand.
- Hedge maturity matching: ideally, match the hedge maturity to the hedging horizon to minimize rolling; but liquidity is often best in the front month, creating a trade-off.

Rolling is an ongoing cost and operational demand. Factor roll yield (contango drag or backwardation benefit) and transaction costs into the hedge economics.

### Evaluate Liquidity, Margin, And Cash-Flow Demands

Hedging with futures creates cash-flow demands:

- Margin and marking-to-market: futures are marked-to-market daily; adverse moves require posting variation margin immediately, even if the underlying hedge is working; this creates cash-flow timing risk.
- Initial margin: capital must be posted as initial margin; this capital is tied up and not earning the underlying's return.
- Margin calls and liquidity: large adverse moves can trigger margin calls that strain liquidity; if the investor cannot meet margin calls, the hedge may be liquidated at a loss, precisely when it is needed.
- Cash-flow mismatch: the hedge gains are realized in the futures account (cash), while the underlying loss is unrealized (or realized later); the timing mismatch can create funding strain.

Futures hedging creates margin and cash-flow demands that can strain liquidity, especially during volatility spikes. Ensure sufficient liquidity to maintain the hedge through adverse moves.

### Decide Between Short-Term And Long-Term Hedging Objectives

Hedging objectives differ by horizon:

- Short-term tactical hedge: hedging a specific near-term risk (e.g., around an event, earnings, or a volatile period); futures are cost-effective for short-term, tactical hedging.
- Long-term strategic hedge: hedging exposure over a long horizon (e.g., persistent currency hedge on international assets); rolling costs and basis risk accumulate over time; long-term hedging may be better served by other instruments (forwards, swaps) or by reducing the exposure.
- Over-hedging and under-hedging: over-hedging (more than 100% of exposure) creates speculative short exposure; under-hedging leaves residual risk; match the hedge to the objective.

Match the hedging instrument and horizon to the objective. Futures are well-suited to short-term tactical hedging; long-term strategic hedging must account for cumulative roll costs and basis risk.

### Consider Tax, Accounting, And Regulatory Treatment

Hedging has tax and accounting implications:

- Tax treatment of hedging gains/losses: futures hedging gains and losses may have different tax treatment than the underlying; mark-to-market (Section 1256 for certain US futures) versus realization for the underlying can create timing mismatches.
- Hedge accounting: for entities using hedge accounting, documentation and effectiveness testing are required to match hedge gains/losses with the underlying in financial statements.
- Regulatory and reporting: large futures positions trigger reporting; position limits may constrain hedge size.
- Straddle and wash-sale rules: offsetting positions may trigger straddle or wash-sale rules.

Tax and accounting treatment affects the after-tax and reported outcome of hedging. Understand the treatment and documentation requirements.

## Common Traps

### Treating Hedging As Risk Elimination

Hedging exchanges price risk for basis risk. Cross hedges leave residual risk; basis movements create unexpected gains or losses.

### Using A Naive 1:1 Hedge Ratio

Beta and correlation differences mean a 1:1 ratio over- or under-hedges. Use beta-adjusted or regression-based hedge ratios.

### Ignoring Hedge-Ratio Drift

Beta and correlations shift over time. The hedge ratio must be rebalanced periodically to remain effective.

### Underestimating Margin And Cash-Flow Demands

Daily marking-to-market and margin calls can strain liquidity during volatility spikes, forcing hedge liquidation at the worst time.

### Forgetting Roll Costs In Long-Term Hedges

Rolling incurs transaction costs and roll yield (contango drag). Long-term hedging costs accumulate and may exceed the benefit.

### Over-Hedging And Creating Speculative Exposure

Hedging more than 100% of the exposure creates speculative short position. Match the hedge to the objective.

## Self-Check

- [ ] Hedging is framed as risk transformation (price risk to basis risk), not risk elimination.
- [ ] The hedge ratio is calculated using beta adjustment or regression (minimum-variance), not a naive 1:1 ratio.
- [ ] Cross-hedge mismatch (underlying, factor, tracking risk) is assessed via beta, correlation, and R-squared.
- [ ] Hedge-ratio drift is recognized, and a rebalancing plan is in place.
- [ ] Rolling costs (transaction costs, roll yield, contango drag) and operational demands are factored into hedge economics.
- [ ] Margin, marking-to-market, margin-call, and cash-flow/liquidity demands are understood, and sufficient liquidity is ensured to maintain the hedge through adverse moves.
- [ ] The hedging horizon and objective (short-term tactical vs. long-term strategic) match the instrument choice, and over-hedging is avoided.
- [ ] Tax, accounting (hedge accounting), and regulatory/reporting treatment is understood, and the conclusion includes appropriate risk disclosure and the need for futures-trading experience.
