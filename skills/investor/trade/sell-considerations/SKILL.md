---
name: stock_sell_considerations.md
description: Use when the agent is evaluating what to consider before selling a stock, trimming an equity position, exiting a thesis, taking profits, cutting losses, or comparing hold versus sell decisions.
---

# Stock Sell Considerations

Selling is not just the reverse of buying. A sell decision can be driven by thesis failure, valuation, risk control, liquidity needs, taxes, opportunity cost, portfolio rebalancing, or behavioral pressure. The programmer-like mistake for an investing agent is to reduce the decision to "price went up" or "price went down". Both can be irrelevant without a current thesis and portfolio context.

Use this skill before answering broad questions such as "what should be considered before selling a stock", "should an investor take profits", "when should a losing stock be sold", or "how should a position be trimmed". The goal is to make the agent examine the reason for selling, what changed, what remains true, and what the proceeds will do next.

## Core Rules

### Rebuild The Current Thesis

Before deciding to sell, restate the original thesis and the current thesis. They may differ. The investor may have bought for growth, then kept holding for tax reasons, dividends, habit, or identity. A sell review should make that shift visible.

Ask:

- Why was the stock originally bought?
- Which assumptions have been confirmed, weakened, or disproven?
- What is the thesis from today forward?
- What evidence would justify continuing to hold?
- What evidence would make holding irrational?

Do not treat the purchase price as the anchor for the decision. The relevant question is whether holding the stock today is better than the available alternatives after costs and taxes.

### Separate Price Movement From Thesis Change

A falling price is not automatically a reason to sell, and a rising price is not automatically a reason to hold. Price movement matters because it may change valuation, risk, and portfolio weight. It is not the whole argument.

Possible reasons to sell after a decline:

- thesis broken;
- downside worse than originally understood;
- balance sheet risk increased;
- better information shows prior analysis was wrong;
- opportunity cost is high;
- position size still too large for risk.

Possible reasons not to sell after a decline:

- thesis intact;
- valuation now compensates for risk;
- position size is controlled;
- selling would only lock in panic without improving the portfolio.

Possible reasons to sell after a gain:

- valuation now requires unrealistic assumptions;
- position became too concentrated;
- risk-reward worsened;
- thesis played out;
- tax or cash planning favors trimming.

### Compare Holding To Rebuying

A useful test is: if the investor had cash today, would they buy this position at its current size and price? If not, ask why they continue to hold it.

This does not mean every non-buy is an immediate sell. Taxes, transaction costs, liquidity, diversification, and gradual rebalancing matter. But the rebuying test exposes inertia and anchoring.

### Consider Position Size And Concentration

Selling can be about risk control even when the business is performing well. A winning stock can grow into a portfolio-dominating position. The investor may still like the company but need to reduce concentration.

Check concentration by:

- single position weight;
- sector and industry exposure;
- factor exposure such as growth, value, momentum, or duration sensitivity;
- geography and currency;
- employer or income correlation;
- tax lot and account type;
- liquidity under stressed market conditions.

Do not frame every sale as a full exit. Trimming, staged selling, hedging, or rebalancing may better match the reason.

### Include Taxes And Frictions Without Letting Them Rule

Taxes can materially affect selling decisions, but tax avoidance can become a trap if it preserves an unacceptable risk. Consider realized gains, losses, holding periods, account type, wash-sale rules where applicable, transaction costs, spreads, and liquidity.

The agent should distinguish:

- pre-tax investment decision;
- after-tax portfolio decision;
- cash-flow or liability need;
- tax-loss harvesting;
- rebalancing trade.

Do not give tax-specific advice without the applicable jurisdiction and investor facts. Instead, flag tax as a required consideration and recommend professional review where appropriate.

### Define What Happens To Proceeds

Selling creates cash or replacement capital. A sell recommendation is incomplete unless it considers the next use of proceeds.

Possible uses:

- rebalance to target allocation;
- buy a better opportunity;
- reduce concentration;
- increase emergency liquidity;
- meet a planned cash need;
- pay debt;
- hold cash while waiting for clarity.

Selling into cash may reduce one risk and create another, such as inflation risk, reinvestment risk, or opportunity cost. The agent should not treat "sell" as the end of the decision.

### Watch Behavioral Bias

Sell decisions are vulnerable to loss aversion, anchoring, regret avoidance, overconfidence, tax fixation, and the desire to be proven right. The agent should name emotional or behavioral pressures when they appear.

Common signals:

- refusing to sell until "back to break-even";
- selling winners too early only because gains feel fragile;
- holding losers because selling feels like admitting a mistake;
- doubling down to defend the original thesis;
- ignoring new evidence because the investor identifies with the company.

The corrective action is not emotionless investing. It is writing explicit rules before stress and applying them consistently.

## Common Traps

### Selling Only Because Of A Headline

Headlines can matter, but the agent must ask whether the event changes long-term cash flows, risk, valuation, or probability distribution. A temporary controversy and a permanent impairment are different.

### Holding Because Of Yield Alone

A dividend yield can be attractive, but it may also reflect price decline, payout risk, or business deterioration. Check free cash flow, balance sheet, payout ratio, cyclicality, and management policy.

### Ignoring Tax Lots

Different lots may have different gains, losses, holding periods, and account implications. A trim decision may be better implemented by selecting lots deliberately rather than selling indiscriminately.

### Treating Sell Discipline As Market Timing

Selling because risk-reward changed is not the same as predicting the next month. A disciplined exit can be based on valuation, thesis failure, concentration, or need for liquidity.

### Making All-Or-Nothing Decisions

Full exit, partial trim, staged sale, covered risk reduction, or simply no new buying are different choices. Match the action to the reason.

### Overlooking the edge case or exception

The typical or textbook scenario is analyzed thoroughly, but the unusual case is skipped. The trap is that the standard path is well-handled while the exception silently produces the wrong outcome, because the agent stopped at the common case and never tested the boundary.

## Self-Check

- [ ] The original thesis and current forward-looking thesis were restated.
- [ ] The analysis separates price movement from actual thesis change.
- [ ] Holding was compared against the alternative of buying the same position today.
- [ ] Concentration, position size, correlation, and portfolio role were reviewed.
- [ ] The decision considers trim, staged sale, or hold alternatives rather than only full exit.
- [ ] Taxes, lots, account type, transaction costs, spreads, and liquidity were considered without letting tax avoidance override risk.
- [ ] The planned use of sale proceeds is explicit.
- [ ] Behavioral biases such as anchoring, loss aversion, regret avoidance, and overconfidence were checked.
- [ ] The sell reason is tied to risk-reward, thesis, portfolio fit, or liquidity rather than a headline alone.
- [ ] The conclusion avoids presenting a sell action as suitable without investor-specific goals, constraints, and tax context.
