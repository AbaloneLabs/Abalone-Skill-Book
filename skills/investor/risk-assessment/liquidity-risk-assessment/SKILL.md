---
name: liquidity_risk_assessment.md
description: Use when the agent is evaluating the ability to convert holdings to cash without large price impact, bid-ask spread and market depth, funding and margin liquidity, redemption and lockup risk in funds, liquidity horizons for bonds and private assets, forced-sale and fire-sale scenarios, or reviewing whether a portfolio can meet obligations and withdrawals under stress when markets freeze and buyers disappear.
---

# Liquidity Risk Assessment

Liquidity risk is the risk that a position cannot be sold, or can only be sold at a large discount, when the need or desire to sell arises. It is the risk that matters most precisely when everything else goes wrong, because in a crisis the buyers disappear, bid-ask spreads blow out, and assets that looked easy to trade become impossible to exit at any reasonable price. The judgment problem is that liquidity is invisible in calm markets and devastating in stressed ones. Agents routinely measure liquidity by recent trading volume and tight spreads, which are precisely the conditions that vanish first, and they conflate funding liquidity (ability to borrow or meet margin) with market liquidity (ability to sell). They also underestimate how fast liquidity drains when many investors head for the same exit. The skill is assessing liquidity under stress, not under normal conditions, and ensuring the portfolio can meet its obligations when markets freeze.

This skill is for evaluating and managing liquidity risk with honest awareness of how it behaves in crises.

## Core Rules

### Distinguish Market Liquidity From Funding Liquidity

Liquidity risk has two distinct forms that interact dangerously in crises. Confusing them leads to blind spots.

Market liquidity:

- the ability to sell an asset at or near its marked price;
- measured by bid-ask spread, market depth, time to liquidate at fair value;
- depends on the presence of buyers and market-makers, which evaporate in stress.

Funding liquidity:

- the ability to meet cash obligations, margin calls, and collateral demands;
- measured by access to credit, cash reserves, and the terms of leverage;
- depends on lenders and prime brokers, who withdraw credit precisely when it is needed.

A portfolio can have ample market liquidity but fatal funding liquidity (heavy leverage with sudden margin calls), or ample funding but frozen market liquidity (cash-rich but holding illiquid assets). Assess both independently and together, because their joint failure is the classic crisis dynamic.

### Measure Liquidity Under Stress, Not In Calm Markets

The defining error is using normal-condition metrics to judge stressed-condition liquidity. Tight spreads and high volume today say almost nothing about tomorrow's crisis.

Stress the measurement by:

- using crisis-period bid-ask spreads and depths, not recent averages;
- estimating the price impact of selling a meaningful fraction of the position, not a round lot;
- modeling the time to liquidate at fair value versus at fire-sale prices;
- accounting for the fact that liquidity is endogenous: your own selling worsens the market.

A bond that trades at a one-bp spread in normal times can gap to a 5% bid-ask in a freeze. Size positions so that stress-condition liquidation is feasible, not just normal-condition trading.

### Assign Liquidity Horizons To Every Position

Different assets require different times to exit at fair value. A liquidity horizon is the time needed to sell a position without materially moving the price.

Estimate horizons:

- cash and large-cap equities, government bonds: hours to days;
- investment-grade corporate bonds: days to weeks;
- high-yield and emerging-market bonds: weeks;
- small-cap and micro-cap stocks: days to weeks depending on size;
- private equity, real estate, infrastructure, private credit: months to years, often with lockups;
- structured and bespoke products: potentially untradeable in stress.

Match the portfolio's liquidity horizons to its obligations. If liabilities or redemption needs require cash in days, the portfolio cannot hold assets that take months to sell.

### Layer Liquidity To Meet Obligations And Redemptions

A sound portfolio holds a ladder of liquidity sufficient to meet expected and stressed obligations without forced fire-sales.

Structure:

- a tier of immediate liquidity (cash, money market, short Treasuries) for near-term needs and surprises;
- a tier of secondary liquidity (liquid bonds, large-cap equities) sellable in days to weeks;
- a strategic illiquid tier (private assets, real estate) held only with a horizon and buffer that forbids forced exit.

The size of the liquid tiers should cover operating needs, planned withdrawals, and a stressed margin of safety. Illiquid assets are acceptable only when the investor can hold them through any plausible stress without needing to sell.

### Account For Redemption And Lockup Risk In Pooled Vehicles

Funds, partnerships, and structured products impose their own liquidity terms that may not match the underlying assets. This creates a maturity transformation risk.

Examine:

- redemption frequency, notice periods, and gates that can suspend withdrawals;
- lockups and holding periods that prevent exit;
- the mismatch between a fund's redemption terms and the liquidity of its holdings (open-end funds holding illiquid assets are especially dangerous);
- side-pocketing and suspension powers the manager can invoke in stress.

An open-end fund offering daily redemption while holding private or thin corporate credit is a liquidity accident waiting to happen. Read the offering documents, not just the recent redemption experience.

### Model Forced-Sale And Fire-Sale Scenarios

The worst liquidity outcomes occur when selling is forced and many sellers act at once. Model these paths explicitly.

Scenario elements:

- a margin call or collateral demand that forces immediate liquidation;
- a wave of redemptions that forces a fund to sell its most liquid (not worst) assets, leaving a concentrated illiquid rump;
- a market freeze where the only bids are far below fair value;
- the feedback loop where falling prices trigger more forced selling.

If a plausible scenario forces a fire-sale of the portfolio's best assets at the worst prices, the structure is unsound regardless of how it looks in calm markets.

### Recognize That Liquidity Is Correlated And Endogenous

Liquidity is not a property of an asset in isolation. It depends on the behavior of other investors and on your own actions.

Understand:

- liquidity is correlated across assets: in a crisis, everything illiquid freezes together;
- liquidity is endogenous: large or urgent selling worsens the market for the seller;
- crowded trades are the least liquid in a unwind, because everyone runs for the same exit;
- the assets that felt most liquid in the rally (because volume was high) can be the most trapped in the reversal.

Size positions and structure the portfolio assuming liquidity will be worst exactly when you need it most, not average across regimes.

## Common Traps

### Measuring Liquidity By Recent Normal-Condition Spreads

Tight recent spreads and high volume vanish first in stress. Normal-condition metrics systematically overstate crisis liquidity.

### Conflating Market And Funding Liquidity

A portfolio can be cash-rich but hold illiquid assets, or hold liquid assets but face fatal margin calls. Both forms must be assessed.

### Maturity Transformation In Funds

Open-end funds offering fast redemption while holding illiquid assets create a structural liquidity mismatch that breaks under redemptions.

### Holding Illiquid Assets Without A Liquidity Buffer

Private assets are acceptable only when the investor can hold them through any stress. Forcing a sale in a freeze destroys value.

### Ignoring Crowded-Trade Unwinds

The most popular trades are the least liquid when sentiment reverses, because everyone sells at once and buyers vanish.

### Assuming The Marked Price Is Realizable

Mark-to-model or stale prices for illiquid assets overstate the value achievable in an actual sale. The real price is what a buyer pays in stress.

### Underestimating Correlated Liquidity Drain

In a crisis, liquidity across asset classes dries up together. Diversification across illiquid assets does not provide liquidity diversification.

### Forgetting The Cost Of Holding Too Much Liquidity

Holding excessive cash eliminates liquidity risk but incurs the opportunity cost of foregone returns and inflation erosion. Liquidity must be sized to need, not maximized.

## Self-Check

- [ ] Market liquidity and funding liquidity are assessed separately and jointly, recognizing that their joint failure is the classic crisis dynamic.
- [ ] Liquidity is measured under stressed conditions (crisis spreads, depths, price impact of sizeable sales), not by recent normal-condition averages.
- [ ] Each position is assigned a realistic liquidity horizon, and the portfolio's horizon profile is matched to its obligations and redemption needs.
- [ ] Liquidity is layered (immediate, secondary, strategic illiquid tiers) with buffers sufficient to meet expected and stressed obligations without forced fire-sales.
- [ ] Redemption terms, lockups, gates, and the mismatch between fund redemption terms and underlying asset liquidity are examined in offering documents.
- [ ] Forced-sale and fire-sale scenarios, including margin calls, redemption waves, and feedback loops, are modeled to find the portfolio's liquidity breaking points.
- [ ] The correlation and endogeneity of liquidity are acknowledged, with sizing that assumes liquidity will be worst when needed most and that crowded trades unwind badly.
- [ ] The recommendation states that liquidity cannot be guaranteed in all conditions, that stressed markets can produce losses far beyond model estimates, that holding liquidity involves opportunity cost, and that professional risk and operational expertise may be warranted for complex or leveraged portfolios.