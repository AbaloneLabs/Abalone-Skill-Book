---
name: portfolio_hedging.md
description: Use when the agent is hedging an equity or multi-asset portfolio using index puts, futures, inverse ETFs, or options overlays, deciding how much portfolio risk to hedge, evaluating hedge cost, basis risk, rollover drag, and effectiveness, or judging whether a hedge protects the intended risk within an acceptable cost and time horizon.
---

# Portfolio Hedging

Portfolio hedging uses instruments (index puts, futures, inverse ETFs, options spreads) to offset part of a portfolio's downside. The idea is simple; the execution is full of traps. Hedges cost money continuously, they rarely match the portfolio exactly (basis risk), they expire or roll and must be renewed, and they often fail in the very crash they were bought for if the portfolio and the hedge diverge. The judgment problem is that a hedge that feels protective can be an expensive drag in calm markets and an ineffective partial cover in the crash that matters.

This skill is for deciding whether and how to hedge a portfolio, with honest accounting of cost and effectiveness.

## Core Rules

### Define The Risk Being Hedged

A hedge without a defined risk is just a speculative position. Start by naming exactly what you are protecting against.

Specify:

- the portfolio exposure to hedge (equity beta, a concentrated position, a sector);
- the scenario (a general market decline, a sharp crash, a slow bear market);
- the horizon over which protection is needed;
- the magnitude of loss to offset.

Different risks call for different hedges. A general equity decline is hedged with index puts or futures; a single-stock concentration may need a collar or exchange fund; a slow bear market is poorly served by short-dated puts that expire before the decline.

### Choose The Instrument By Its Properties

Each hedging instrument has a distinct cost, basis, and behavior.

Index puts:

- clean downside protection with capped further loss;
- expensive, with continuous premium decay (theta);
- basis risk if the portfolio differs from the index;
- need rolling as they expire.

Futures and short exposure:

- lower explicit cost than puts;
- no premium decay but unlimited downside if wrong;
- basis risk versus the portfolio;
- margin and funding requirements.

Inverse and leveraged ETFs:

- convenient but suffer daily-reset drift over time;
- poor for multi-period hedging due to compounding effects;
- high internal cost.

Options spreads (put spreads, collars):

- cheaper than outright puts by selling some upside or downside;
- capped protection, not full insurance;
- reduce cost at the price of a protection gap in the tail.

Match the instrument to the horizon, the basis tolerance, and the cost budget.

### Quantify And Budget The Hedge Cost

Hedging is insurance, and insurance costs money every day you hold it. The cost compounds and can rival the loss it prevents over time.

Estimate:

- the annualized premium or carry cost;
- the expected drag on returns in calm markets;
- the cumulative cost over the holding period;
- the break-even decline at which the hedge pays off;
- the opportunity cost of capital tied up in the hedge.

A hedge that costs 2% per year to hold drags returns persistently. Over a decade of calm markets, that is a 20%+ cumulative drag. Decide whether the protection justifies the known ongoing cost.

### Measure Basis Risk Honestly

The hedge and the portfolio are rarely identical. The gap is basis risk, and it is where hedges fail.

Assess:

- the portfolio's beta to the hedge instrument;
- differences in sector, style, cap, and country exposure;
- how the basis behaves in calm versus stressed markets;
- the historical tracking error between portfolio and hedge.

A portfolio of small-cap value hedged with large-cap index puts may find the hedge rises in a crash driven by large-cap tech while the portfolio falls for different reasons. Basis risk turns a "perfect hedge" into a partial, unpredictable one.

### Plan Rollover And Time Decay

Most hedges expire or drift and must be renewed. The rollover plan is part of the hedge.

Address:

- the tenor that balances cost and protection (short-dated is cheaper per unit time but rolls more often and risks gaps);
- the rollover schedule and its cost;
- time decay (theta) that erodes option value daily;
- the risk of being unhedged during a roll gap.

Short-dated puts are cheap but expire constantly, risking a gap between expirations. Long-dated puts cost more upfront but need fewer rolls. The tenor decision is a cost-versus-continuity tradeoff.

### Decide Partial Versus Full Hedging

Full hedging eliminates the risk but also the return and costs the most. Partial hedging is usually the practical choice.

Decide:

- the hedge ratio (what fraction of the exposure to offset);
- whether to hedge the whole portfolio or a specific concentrated sleeve;
- whether a dynamic hedge (increasing in risk-off) suits the investor;
- the tradeoff between cost and protection at each ratio.

A 50% hedge halves the downside and the cost. Many investors are best served by hedging only their most dangerous concentration, not the whole portfolio.

### Reconcile Hedging With Strategy And Goals

Hedging must fit the investor's strategy, not fight it. A long-term equity investor who hedges away all equity risk has, at cost, converted to bonds.

Check:

- whether de-risking by selling or reallocating is cheaper than hedging;
- whether the hedge aligns with the time horizon and goals;
- whether the investor can sustain the cost drag through calm periods;
- whether the hedge is a temporary tactical measure or a permanent overlay.

Sometimes the right answer is to reduce the position, not to buy an expensive hedge to keep it. Hedging makes sense when selling is impossible (taxes, lockups, restrictions) or when temporary protection is needed around a known event.

## Common Traps

### Hedging Without A Defined Risk

Buying puts "for protection" without naming the risk, horizon, or magnitude is speculation dressed as insurance.

### Ignoring The Ongoing Cost Drag

Hedges cost money every day. Over years of calm markets, the cumulative drag can exceed the loss the hedge was meant to prevent.

### Basis Risk Hidden By A Clean Label

An index put does not perfectly hedge a portfolio that differs from the index. The gap appears in the crash.

### Daily-Reset Inverse ETFs For Long Horizons

Inverse and leveraged ETFs drift over time due to compounding and are poor multi-period hedges despite their convenience.

### Rollover Gaps And Time Decay

Short-dated hedges expire and must roll, risking gaps and bleeding theta. Without a rollover plan, protection lapses.

### Over-Hedging Into A Return Drag

Hedging away most of the risk also hedges away most of the return, at a net cost. The investor ends up with bond-like returns and bond-like risk but higher fees.

### Hedging When Selling Is Cheaper

If the position can be sold or trimmed, that is often cheaper and cleaner than buying ongoing hedges to keep it.

### Trusting The Hedge In A Novel Crisis

Basis and correlation can shift in a new type of crisis, so the hedge underperforms its historical behavior exactly when needed.

## Self-Check

- [ ] The specific risk, scenario, horizon, and magnitude being hedged are defined.
- [ ] The instrument (puts, futures, inverse ETF, spread, collar) is chosen for its cost, basis, and behavior properties.
- [ ] The annualized cost, return drag, break-even decline, and opportunity cost are quantified and budgeted.
- [ ] Basis risk between the portfolio and the hedge is measured in calm and stressed markets.
- [ ] Rollover schedule, tenor, and time decay are planned, with gap risk addressed.
- [ ] The hedge ratio (partial versus full) is chosen deliberately, with concentrated sleeves prioritized.
- [ ] Hedging is reconciled with strategy and goals, and selling or reallocating is considered as a cheaper alternative where feasible.
- [ ] The recommendation flags that hedges cost money continuously, that basis risk can reduce effectiveness in crises, and that professional derivatives expertise may be warranted for complex overlays.
