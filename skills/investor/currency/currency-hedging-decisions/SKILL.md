---
name: currency_hedging_decisions.md
description: Use when the agent is deciding whether to hedge currency exposure in international equity or bond portfolios, evaluating the cost and basis risk of hedging, choosing a hedge ratio, or selecting between hedged and unhedged funds. Covers currency hedging, hedge ratio, forward hedging cost, interest rate differential, basis risk, hedging equities versus bonds, hedging emerging markets, and the tradeoffs of partial versus full hedging over different horizons.
---

# Currency Hedging Decisions

Hedging currency is a risk transfer with a cost, not a free improvement. The judgment problem is that investors treat hedging as obviously good (it reduces volatility) without weighing its cost, its horizon-dependence, its basis risk, and the fact that it changes the portfolio's behavior in ways that may or may not suit the objective. Agents default to full hedging for bonds and no hedging for equities as rules of thumb, missing that the right hedge ratio depends on the investor's base currency, horizon, view on the cost, and the specific asset. The skill is making the hedging decision deliberately — choosing a hedge ratio, understanding what it costs and what it gives up, and recognizing where hedging fails.

This skill is for deciding whether and how much to hedge currency exposure. It is not personalized financial advice; hedging costs, instruments, and tax treatment vary by jurisdiction and investor.

## Core Rules

### Decide Hedging By Asset Class, Because Currency Risk Means Different Things To Each

The volatility contribution of currency differs enormously across asset classes, and so does the case for hedging it.

Reason by asset class:

- **bonds** — currency volatility is comparable to or larger than bond volatility, so unhedged currency can dominate a foreign bond's risk and often double its volatility; hedging is usually appropriate because the investor wants the bond's yield, not a currency bet;
- **equities** — equity volatility is several times currency volatility, so currency contributes a smaller share of total risk, and unhedged exposure can diversify over long horizons; partial or no hedging is often defensible;
- **alternatives and real assets** — currency sensitivity depends on the asset's pricing and revenue currency; evaluate case by case.

Apply a different hedge ratio to each sleeve. One policy across all assets mis-hedges at least one of them.

### Price The Cost Of Hedging Through The Interest Rate Differential

Currency hedging via forwards is not free; its cost is embedded in the forward points, which reflect the interest rate differential between the two currencies under covered interest rate parity.

Understand the cost:

- hedging a lower-yielding currency into a higher-yielding base currency incurs a cost (the yield forgone);
- hedging a higher-yielding currency into a lower-yielding base currency can produce a yield pickup;
- over time, persistent hedging costs can materially reduce the return of a hedged allocation relative to unhedged;
- the cost shows up as a difference between hedged and unhedged fund yields, not as an explicit fee.

Model the net yield and return of the hedged exposure, not the gross. A hedged international bond fund can yield noticeably less than its unhedged counterpart purely because of the hedge cost.

### Match The Hedge Ratio To The Horizon And Objective

The benefit of hedging (reduced short-term volatility) and its cost (forgone diversification and possible return drag) trade off differently across horizons.

Match ratio to horizon:

- for short horizons and liability-matching objectives, full hedging reduces volatility and aligns the asset with the base-currency obligation;
- for long horizons and growth objectives, partial or no hedging may capture currency diversification that emerges over decades;
- for intermediate horizons, partial hedging (for example 50 percent) can capture much of the volatility reduction while retaining some diversification.

There is no universally correct hedge ratio. The right ratio depends on what the asset is for and over what horizon.

### Recognize Basis Risk And Hedging Imperfection

Hedges are rarely perfect. The currency exposure being hedged may not match the hedging instrument exactly, producing basis risk — the residual exposure when the hedge and the underlying diverge.

Sources of basis risk:

- daily hedging versus monthly or quarterly (the hedge resets less often than the asset moves);
- hedging a basket currency exposure with a single forward, or vice versa;
- emerging market currencies where forwards are costly, illiquid, or subject to delivery restrictions;
- hedged funds that hold multiple currencies and hedge a representative basket, leaving residual exposure to individual currency moves.

A "hedged" fund is not a guarantee of zero currency exposure. Understand the hedging mechanics and the residual basis risk, especially for multi-currency funds and emerging markets.

### Distinguish Hedging From Currency Speculation

Hedging reduces an existing exposure to align the portfolio with the base currency. Speculation adds currency exposure to express a directional view. These are different activities with different risk profiles.

Keep them separate:

- a hedging decision should be driven by the portfolio's risk and objective, not by a forecast that a currency will fall;
- if the investor has a specific currency view, express it deliberately and sized as a position, not by quietly adjusting the hedge ratio;
- avoid letting a currency forecast disguise itself as a hedging policy, because currency forecasts are notoriously unreliable.

Mixing the two leads to oversized, unintentional currency bets presented as risk management.

### Evaluate Hedging Emerging Market Currency Specially

Emerging market currencies present hedging challenges that differ from developed markets: higher volatility, restricted deliverability, higher hedging costs, and the risk of large discrete devaluations or capital controls.

Assess EM hedging:

- EM currency hedging can be expensive and imperfect, sometimes negating the yield advantage of EM debt;
- some EM currencies cannot be hedged efficiently or at all through standard forwards;
- capital controls and peg regimes can produce gap risk that no hedge fully covers (see the peg-devaluation skill);
- the unhedged EM currency may itself be the source of return (carry) or loss (devaluation), and the decision is consequential.

For EM, the hedging decision is often a close call between cost, basis risk, and the currency's role in the portfolio, and should be made explicitly rather than by default.

### Choose The Right Vehicle And Monitor It

Hedged and unhedged funds, and currency overlay strategies, differ in how they hedge, how often, and at what cost. Selecting the vehicle matters as much as the decision to hedge.

Evaluate vehicles:

- the fund's hedging frequency and methodology (daily, monthly, basket versus single-currency forwards);
- the embedded hedging cost reflected in the fund's yield and tracking error;
- the fund's track record in different currency regimes, including periods of sharp base-currency moves;
- the tax treatment of any hedging gains or losses within the fund.

A hedged fund that hedges infrequently or expensively can deliver a poor approximation of the intended exposure. Verify the mechanics, not just the label.

## Common Traps

### Treating Hedging As Free Volatility Reduction

Hedging has a cost embedded in the forward points. The volatility reduction is real but not free.

### Applying One Hedge Ratio To All Asset Classes

Bonds and equities warrant different hedging because currency contributes very different shares of their risk.

### Ignoring The Interest Rate Differential Cost

Hedging low-rate currencies into higher-rate base currencies drags returns. Model net, not gross.

### Assuming A Hedged Fund Has Zero Currency Exposure

Basis risk, hedging frequency, and basket approximations leave residual exposure. A hedged fund is an approximation, not a guarantee.

### Disguising A Currency Forecast As A Hedging Decision

Adjusting the hedge ratio based on a currency view is speculation, not risk management, and currency forecasts are unreliable.

### Hedging Emerging Markets Mechanically

EM hedging is costly, imperfect, and sometimes infeasible. The decision is consequential and should be made explicitly.

### Selecting A Hedged Fund By Label Alone

Hedging frequency, methodology, cost, and tracking error differ across funds. Verify the mechanics.

## Self-Check

- [ ] Hedging was decided by asset class, with the case for bonds (usually hedge) distinguished from the case for equities (often partial or none).
- [ ] The cost of hedging was priced through the interest rate differential and forward points, and net hedged yield and return were modeled.
- [ ] The hedge ratio was matched to the horizon and objective, with full hedging for short horizons and liability matching, and partial or no hedging considered for long-horizon growth.
- [ ] Basis risk and hedging imperfection (frequency, basket approximation, EM deliverability) were recognized, and a hedged fund was treated as an approximation.
- [ ] Hedging was kept distinct from currency speculation, with any directional currency view expressed as a deliberate, sized position rather than a quiet hedge-ratio change.
- [ ] Emerging market currency hedging was evaluated for cost, basis risk, deliverability, and the currency's role in the portfolio, not defaulted mechanically.
- [ ] The hedging vehicle was selected on methodology, frequency, embedded cost, tracking error, and tax treatment, not on label alone.
- [ ] The conclusion flags that hedging costs and instruments vary by jurisdiction, that currency movements are unpredictable, that past performance does not guarantee future results, and that this is not personalized financial advice.
