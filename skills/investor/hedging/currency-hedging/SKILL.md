---
name: currency_hedging.md
description: Use when the agent is managing foreign exchange exposure in a portfolio, deciding whether and how much to hedge currency risk, using FX forwards, futures, swaps, or hedged share classes, evaluating hedging cost, basis risk, interest rate differentials, and the strategic versus tactical role of currency hedging for international investments.
---

# Currency Hedging

International investing brings currency exposure: a foreign asset's return in the home currency depends on both the asset's local return and the currency movement. Currency hedging uses forwards, futures, swaps, or hedged share classes to reduce that FX exposure. The judgment problem is that currency hedging is not free, not perfect, and not always desirable. Agents often treat "hedged" as obviously safer, when in fact hedging has an ongoing cost, basis and tracking risk, and changes the portfolio's risk profile in ways that may or may not suit the investor.

This skill is for deciding whether, how much, and how to hedge currency exposure.

## Core Rules

### Decide Whether Currency Is A Risk Or A Diversifier

Currency exposure is not automatically bad. For some investors it adds diversification; for others it is unwanted volatility.

Assess:

- the investor's base currency and spending currency;
- the horizon (currency volatility matters less over very long horizons);
- whether currency diversifies or concentrates the portfolio's risk;
- the correlation of the foreign currency to the foreign asset and to the rest of the portfolio.

For a long-horizon investor, currency moves tend to mean-revert and may add little long-term risk while providing diversification. For a short-horizon investor or one spending in the home currency, unhedged FX can add meaningful volatility. The decision depends on the investor, not on a general rule.

### Separate Strategic From Tactical Currency Decisions

Strategic currency hedging is a standing policy that reflects the investor's long-term risk profile. Tactical currency hedging is a view on currency direction.

Distinguish:

- strategic hedge ratio set from goals, horizon, and base currency;
- tactical deviations as separate, budgeted bets with a thesis and exit;
- the tracking error a tactical currency view adds;
- the risk of mixing the two and calling every impulse "strategic."

A common error is to toggle the hedge ratio based on currency views while calling it strategic. Strategic hedging should be stable; tactical currency views should be explicit and bounded.

### Quantify The Hedging Cost

Currency hedging is not free. The cost is largely driven by interest rate differentials (covered interest parity), plus transaction and roll costs.

Estimate:

- the interest rate differential between the two currencies, which is the primary hedging cost or carry;
- the transaction cost and bid-ask of the hedging instrument;
- the rollover cost as hedges renew;
- the cumulative drag over the holding period.

Hedging a high-yield currency into a low-yield one incurs a positive cost (you give up the yield differential). Hedging a low-yield currency into a high-yield one can have negative cost but reflects the rate difference, not free money. The cost is real and persistent.

### Account For Basis Risk And Imperfect Hedging

A currency hedge rarely matches the exposure perfectly. Mismatches create basis risk.

Check:

- the exact currency exposure of the underlying assets (multiple currencies within a fund);
- the hedge's currency, notional, and tenor versus the exposure;
- cross-currency basis and deviations from covered interest parity, especially in stress;
- the tracking error of hedged share classes or funds.

A global equity fund holds dozens of currencies; a single-currency hedge covers only one. Hedged funds approximate the hedge but have tracking error. Basis risk means the hedge reduces, not eliminates, FX exposure.

### Match The Hedge Horizon To The Investment Horizon

Currency hedges roll and must be renewed. The horizon of the hedge should match the investment.

Decide:

- the tenor of forwards or futures (monthly, quarterly);
- the rollover frequency and its cost;
- whether to use rolling forwards or a hedged fund that handles it internally;
- the gap risk if a hedge lapses.

Short-tenor hedges need frequent rolling and incur more transaction cost; long-tenor or fund-based hedges reduce roll burden but may have less flexibility.

### Consider Partial Hedging

Full hedging removes all currency exposure at full cost. Partial hedging is often the practical compromise.

Choose:

- a hedge ratio that balances volatility reduction and cost (often 50% for diversified equity);
- different ratios for different asset classes (often hedge bonds more than equity, since currency adds unwanted volatility to stable bond returns);
- whether to hedge only the largest currency exposures;
- the interaction with the investor's home bias and spending currency.

A common disciplined approach is to hedge foreign bonds heavily (currency volatility dominates bond returns) and foreign equity partially (currency diversifies equity risk over long horizons).

### Reconcile With Implementation Realities

Retail and institutional investors have different tools and costs.

Consider:

- availability of hedged share classes or ETFs versus direct forwards;
- the cost and minimum size of direct FX hedging;
- tax treatment of currency gains and hedging instruments;
- the operational burden of rolling and monitoring hedges.

For most individual investors, hedged funds are the practical tool, accepting their tracking error and embedded cost. Direct hedging is cost-effective only at scale.

## Common Traps

### Assuming Hedged Is Always Safer

Hedging reduces currency volatility but adds cost and basis risk. For long-horizon investors, unhedged currency may diversify at no net cost.

### Ignoring The Interest Rate Differential Cost

The primary hedging cost is the rate differential, not the transaction fee. It is persistent and can be material.

### Mixing Strategic And Tactical Hedging

Toggling the hedge ratio based on currency views, while calling it strategic, destroys the discipline of both.

### Treating A Single-Currency Hedge As Complete

A global fund holds many currencies. A single-currency hedge leaves the others exposed.

### Over-Hedging For Long Horizons

For very long horizons, currency mean-reverts and adds little risk. Full hedging incurs cost for little long-term benefit.

### Trusting Hedged Funds To Be Perfect

Hedged funds have tracking error, embedded cost, and basis risk. They reduce, not eliminate, FX exposure.

### Forgetting Cross-Currency Basis In Stress

In crises, covered interest parity can break and hedging costs spike or hedges fail to perform, exactly when stress matters.

### Hedging Currency But Ignoring Home-Currency Asset Risk

An investor who fully hedges currency but holds foreign assets still bears the foreign asset's local-market risk. Hedging currency is not hedging the asset.

## Self-Check

- [ ] The decision treats currency as a risk or a diversifier based on the investor's base currency, horizon, and correlations, not as automatically bad.
- [ ] Strategic hedge ratio is separated from tactical currency views, with tactical deviations budgeted and bounded.
- [ ] Hedging cost, including the interest rate differential, transaction cost, and rollover drag, is quantified.
- [ ] Basis risk, cross-currency basis, multi-currency exposure, and hedged-fund tracking error are accounted for.
- [ ] Hedge tenor and rollover frequency match the investment horizon, with gap risk addressed.
- [ ] A partial hedge ratio is chosen deliberately, often differing by asset class (more for bonds, less for equity).
- [ ] Implementation realities (hedged funds versus direct forwards, cost, tax, operations) are reconciled with the investor's scale.
- [ ] The recommendation flags that hedging has persistent cost and basis risk, that currency outcomes are uncertain, and that professional advice may be warranted for multi-currency portfolios.
