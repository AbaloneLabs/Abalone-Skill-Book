---
name: market_depth_and_spreads.md
description: Use when the agent is assessing bid-ask spreads, trading volume, market depth, slippage, liquidity traps, or stress-scenario execution risk, and judging whether an asset can be traded at scale without excessive cost before forming an investment view or executing a trade.
---

# Market Depth And Spreads

Liquidity is the ability to trade an asset at scale without moving its price against you. It is easy to ignore in calm markets and devastating in stress. An investing agent often analyzes expected return and risk while treating liquidity as a given, assuming any position can be entered and exited at the quoted price. This assumption fails for small caps, emerging markets, bonds, alternatives, and even large positions in liquid assets during stress. Liquidity is itself a risk factor that affects realizable return, and underestimating it turns paper gains into trapped capital or forced losses.

Use this skill before answering questions such as "can I trade this position at scale", "how liquid is this asset", "what are the liquidity risks", or "how do I assess execution cost". The goal is to prevent the agent from treating liquidity as free and infinite, and to force it to measure spreads, depth, and slippage, anticipate liquidity traps, and stress-test execution before committing capital.

Liquidity risk can make an asset effectively untradeable exactly when exit is most needed. Conclusions should disclose the execution risk and investor-specific position sizing.

## Core Rules

### Measure Liquidity Across Multiple Dimensions

Liquidity is not a single number. It is the combination of several properties that determine realizable trading cost.

Dimensions:

- Tightness: the bid-ask spread; narrower is cheaper to trade round-trip.
- Depth: the volume available at or near the quote; deeper markets absorb larger orders with less price impact.
- Resiliency: how quickly the order book refills after a trade; resilient markets recover fast.
- Breadth: the range of participants and order flow; broad markets are more stable.
- Immediacy: how quickly a large order can be filled at a reasonable cost.

A market can be tight but shallow, cheap for small trades but catastrophic for large ones. Assess the combination, not just the headline spread.

### Separate Quoted Liquidity From Realizable Liquidity

The price you see is not the price you get at scale. Quoted liquidity often overstates what a real position can achieve.

Clarify:

- Quoted spreads and top-of-book depth reflect small-order liquidity.
- Real positions face price impact as larger orders consume successive layers of the order book.
- Average daily volume is a rough guide, but trading a large fraction of it moves the price significantly.
- Liquidity evaporates in stress, so calm-market depth overstates stress-market depth.

Estimate the slippage and market impact for the intended order size, not just the quoted spread. A position that looks cheap to enter can be expensive to exit.

### Stress-Test Execution Before Committing

Liquidity is most valuable when you need it most, and that is exactly when it disappears. Stress-test the exit before entering.

Ask:

- What is the estimated cost and time to liquidate the full position in normal conditions?
- What happens to spreads, depth, and slippage in a stress scenario, a market crash, a redemption wave, or a sector panic?
- Could the position become untradeable or gap-priced under stress?
- Is there a forced-seller dynamic, such as fund redemptions, that would coincide with the need to exit?
- Are there circuit breakers, trading halts, or settlement constraints that could trap the position?

An asset that is liquid in calm markets but illiquid in stress carries hidden tail risk. The relevant liquidity is the liquidity available when you need to sell, not when you do not.

### Account For Asset-Class Liquidity Differences

Liquidity characteristics differ sharply across asset classes, and applying equity-market assumptions elsewhere is dangerous.

Differences:

- Large-cap equities: generally deep and continuous, but still subject to gap and stress risk.
- Small-cap and micro-cap equities: thin, wide spreads, and gap-prone; large positions are hard to build and exit.
- Bonds: often trade over-the-counter with wide spreads, minimum lot sizes, and dealer markups; liquidity is far worse than equities.
- Emerging-market assets: shallower, more volatile, and subject to capital-flow and currency stress.
- Alternatives, private equity, real estate: illiquid by design, with long lockups and uncertain valuation.
- ETFs: can offer liquidity that exceeds the underlying, creating a liquidity illusion in stress.

Match the liquidity analysis to the asset class. Do not assume bond or alternative liquidity resembles equity liquidity.

### Watch For Liquidity Illusions And Mismatches

Some structures create apparent liquidity that fails under stress. Identify these traps.

Traps:

- ETF liquidity illusion: an ETF may trade liquidly while its underlying holdings do not; in stress, the ETF can disconnect or widen sharply.
- Open-end fund liquidity mismatch: funds offering daily redemption while holding illiquid assets can face gating or fire-sale liquidation.
- Dealer balance-sheet contraction: market-makers withdraw in stress, removing the liquidity that existed in calm markets.
- Concentrated market-making: if one or few dealers provide liquidity, their withdrawal gaps the market.
- Crowded trades: liquidity vanishes when everyone tries to exit the same position simultaneously.

Apparent liquidity in calm markets is not a promise of liquidity in stress. The structure of the market determines whether liquidity holds.

### Factor Liquidity Into Position Sizing And Time Horizon

Liquidity constraints should directly affect how large a position to take and how long it can be held.

Apply:

- Size positions so that entry and exit costs remain acceptable given expected return.
- For illiquid assets, demand a higher expected return to compensate for the liquidity discount.
- Match the holding period to the liquidity profile; illiquid assets require longer horizons.
- Build positions gradually and exit in tranches to reduce market impact.
- Maintain a liquidity reserve so that forced sales of illiquid positions are avoidable.

A position that is right on fundamentals but wrong on liquidity can still lose money through execution cost and forced timing.

### Incorporate Transaction Costs Into The Investment Thesis

Spreads, commissions, market impact, taxes, and funding costs reduce realizable return. An analysis that ignores them overstates performance.

Include:

- round-trip spread and commission cost;
- estimated market impact for the position size;
- tax treatment and timing;
- custody, fx, and settlement costs for foreign or complex assets;
- the cost of hedging or liquidity reserves if used.

For high-turnover or illiquid strategies, transaction costs can be the difference between a winning and losing approach.

## Common Traps

### Treating Quoted Spreads As The Trading Cost

Quoted spreads reflect small-order cost. Real positions face market impact that can be many times the spread.

### Assuming Calm-Market Liquidity Persists In Stress

Liquidity evaporates exactly when it is needed most. Calm-market depth overstates stress-market depth.

### Ignoring Asset-Class Liquidity Differences

Bonds, small caps, emerging markets, and alternatives are far less liquid than large-cap equities. Applying equity assumptions produces large errors.

### Falling For ETF Or Fund Liquidity Illusions

ETFs and open-end funds can offer liquidity that exceeds the underlying. In stress, the structure can fail and widen or gate.

### Sizing Positions Without Liquidity Constraints

A position that is right on fundamentals can lose money if it is too large to exit efficiently or if forced timing coincides with stress.

### Overlooking The Liquidity Discount

Illiquid assets should offer higher expected returns to compensate. Buying illiquidity without a discount is accepting risk without reward.

### Forgetting Transaction Costs In Return Estimates

Spreads, impact, taxes, and custody costs reduce realizable return. Ignoring them inflates expected performance.

## Self-Check

- [ ] Liquidity is measured across tightness, depth, resiliency, breadth, and immediacy, not just the headline spread.
- [ ] Quoted liquidity is distinguished from realizable liquidity at the intended position size.
- [ ] Execution is stress-tested for spreads, depth, slippage, gaps, and forced-seller dynamics before committing.
- [ ] Asset-class liquidity differences are accounted for rather than applying uniform equity assumptions.
- [ ] Liquidity illusions and mismatches in ETFs, funds, and dealer-dependent markets are identified.
- [ ] Position sizing and holding period reflect liquidity constraints and include a liquidity reserve.
- [ ] Transaction costs, spreads, market impact, taxes, and custody costs are incorporated into the return estimate.
- [ ] The analysis treats liquidity as a risk factor, not a free good.
- [ ] The possibility of the position becoming untradeable in stress is acknowledged.
- [ ] The conclusion discloses execution risk, flags investor position size and time horizon, and recommends professional advice for illiquid or complex assets.
