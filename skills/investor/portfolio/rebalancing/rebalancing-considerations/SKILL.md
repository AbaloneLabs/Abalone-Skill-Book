---
name: rebalancing_considerations.md
description: Use when the agent is evaluating what to consider before rebalancing a portfolio, changing target allocation, trimming winners, adding to laggards, restoring asset mix, or adjusting investment exposure over time.
---

# Rebalancing Considerations

Rebalancing is the act of bringing a portfolio back toward an intended risk profile. It is not simply selling what went up and buying what went down. A rebalance decision should connect target allocation, investor goals, time horizon, risk tolerance, taxes, transaction costs, liquidity needs, and current market conditions.

Use this skill before answering broad questions such as "what should be considered when rebalancing", "how should a portfolio be rebalanced", "when should allocation be adjusted", or "what matters before trimming and adding positions". The goal is to prevent the agent from recommending mechanical trades without understanding why the target allocation exists.

## Core Rules

### Start With The Target Allocation

Rebalancing only makes sense relative to a target. The target may be formal or implicit, but the agent should make it explicit before discussing trades.

Ask:

- What is the intended allocation by asset class?
- Does the target reflect the investor's goals, time horizon, liquidity needs, and risk tolerance?
- Is the current target still appropriate?
- Are there sub-targets by geography, sector, factor, currency, duration, or account type?
- Which deviations are acceptable before action is needed?

If the target itself is wrong, rebalancing back to it can be harmful. First distinguish rebalancing from strategic allocation change.

### Measure Drift And Source Of Drift

Portfolio drift can come from price movement, contributions, withdrawals, dividends, currency movement, employer stock grants, private assets, or different accounts growing at different rates.

Measure:

- current weight versus target weight;
- absolute dollar difference;
- relative percentage drift;
- risk contribution, not only market value;
- concentration by single position and correlated exposures;
- taxable versus tax-advantaged account locations.

A position that is only slightly overweight by market value may contribute a large share of portfolio risk if it is volatile or highly correlated with other holdings.

### Match Rebalancing Method To Objective

There are several ways to rebalance:

- calendar-based review;
- threshold-based trades;
- directing new contributions to underweight assets;
- using dividends and interest;
- trimming overweight positions;
- tax-loss harvesting;
- gradual rebalancing over multiple trades;
- changing future purchase plans without selling.

Do not assume immediate selling is required. For taxable investors, using cash flows can reduce taxes and costs. For risk breaches, immediate action may be more appropriate.

### Consider Taxes, Costs, And Account Location

Rebalancing can create realized gains, losses, transaction costs, bid-ask spreads, fund redemption fees, and tax complexity. It can also improve after-tax outcomes when losses are harvested or asset location is optimized.

Check:

- taxable gains and losses;
- holding periods;
- wash-sale or similar rules where applicable;
- account type;
- fund distribution timing;
- transaction costs and spreads;
- liquidity of each holding;
- whether trades should occur in tax-advantaged accounts first.

Tax impact is part of the decision, but should not force the investor to keep a risk profile they cannot tolerate.

### Respect Liquidity And Cash Needs

Rebalancing should not accidentally consume emergency funds, near-term spending reserves, tax payments, tuition money, house down payments, or planned withdrawals. The right allocation for long-term capital may be wrong for cash needed soon.

Before trading, identify:

- cash reserve requirement;
- expected withdrawals;
- near-term liabilities;
- income needs;
- minimum account balances;
- settlement timing;
- currency needs.

If the investor will need cash soon, a rebalance that increases risk may be inappropriate even if it restores a long-term target.

### Distinguish Risk Control From Market Timing

Rebalancing can feel uncomfortable because it often sells winners and buys laggards. The reason should be risk control and discipline, not a prediction that yesterday's loser will outperform tomorrow.

The agent should not justify rebalancing with false precision. It should explain the risk profile being restored and the tradeoffs:

- reduced concentration;
- maintained diversification;
- controlled drawdown exposure;
- improved alignment with goals;
- potential opportunity cost if trends continue;
- possible tax cost.

### Review The Target After Major Life Or Goal Changes

Rebalancing assumes the investor's plan is still valid. Major changes may require a new target allocation instead:

- approaching retirement or a major withdrawal;
- job loss or income volatility;
- inheritance or concentrated stock grant;
- home purchase;
- new dependents;
- change in risk tolerance after a drawdown;
- change in debt, insurance, or emergency reserve;
- regulatory, tax, or account changes.

Do not rebalance mechanically when the investor's constraints have changed.

## Common Traps

### Rebalancing Without A Target

Selling and buying to "clean up" a portfolio is not rebalancing unless there is a target allocation or risk profile. Without a target, trades may reflect emotion or recent performance.

### Treating Every Drift As A Trade Signal

Small drifts may not justify costs and taxes. Thresholds should account for portfolio size, volatility, costs, and how much the drift changes risk.

### Ignoring Concentration Hidden Across Accounts

An investor may hold the same company, sector, country, factor, or employer exposure across brokerage, retirement, stock compensation, private funds, and business ownership. Rebalancing should consider the whole financial picture where possible.

### Buying Laggards Without Checking Thesis

Adding to underweight assets can be disciplined, but not if the asset is impaired or no longer fits the plan. Rebalancing is not a reason to ignore changed fundamentals.

### Letting Taxes Freeze The Portfolio

Avoiding taxes can lead to excessive concentration and risk. Sometimes paying tax is the cost of reducing unacceptable exposure.

### Forgetting Behavior Under Stress

A target allocation that the investor abandons in a downturn is too aggressive. Rebalancing should consider whether the investor can follow the plan during losses.

## Self-Check

- [ ] The target allocation or intended risk profile is explicit before recommending trades.
- [ ] Current drift is measured by weight, dollars, risk contribution, and concentration where relevant.
- [ ] The analysis distinguishes rebalancing from changing the strategic allocation.
- [ ] The proposed method considers thresholds, calendar review, cash flows, dividends, trimming, and gradual implementation.
- [ ] Taxes, account type, lots, transaction costs, spreads, liquidity, and fund distribution timing were considered.
- [ ] Cash reserves, near-term liabilities, withdrawals, settlement timing, and currency needs were checked.
- [ ] Whole-portfolio exposure across accounts, employer stock, private assets, and correlated risks was considered where possible.
- [ ] Underweight assets were checked for thesis impairment before adding.
- [ ] The recommendation explains risk-control purpose rather than pretending to time markets precisely.
- [ ] The conclusion avoids presenting a rebalance as suitable without investor-specific goals, time horizon, risk tolerance, and tax context.
