---
name: contango_and_backwardation.md
description: Use when the agent is analyzing commodity futures curve structure, assessing contango versus backwardation, modeling roll yield and cost of carry, relating curve shape to supply and demand and inventory conditions, or understanding how the term structure drives futures returns independent of spot price moves.
---

# Contango And Backwardation

For commodity futures investors, the shape of the futures curve is often more important than the direction of the spot price. A long futures position earns (or loses) roll yield as it rolls from expiring to next-month contracts, and this roll yield can dominate total return. Investors who are right about the direction of commodities but wrong about the curve structure routinely lose money. Treating commodity futures as a simple spot-price bet — without understanding contango, backwardation, and roll yield — is the most common and costly error in commodity investing.

Use this skill before answering questions such as "why did my commodity fund lose money when oil rose", "what is roll yield", "what is the difference between contango and backwardation", or "how do commodity futures returns work". The goal is to prevent the agent from conflating spot price changes with investor returns, and from ignoring the term structure that drives futures performance.

## Core Rules

### Define Contango And Backwardation Precisely

The futures curve plots futures prices against expiration dates:

- Contango: near-term futures prices are lower than longer-dated prices; the curve slopes upward. The market expects prices to rise, or storage and financing costs (cost of carry) push forward prices above spot.
- Backwardation: near-term futures prices are higher than longer-dated prices; the curve slopes downward. The market expects prices to fall, or tight near-term supply creates a scarcity premium for immediate delivery.

The curve shape reflects supply, demand, inventory, storage capacity, and the cost of carry. It is not a forecast to bet against casually; it is the market's priced view of the balance.

### Understand Roll Yield As A Return Component

A long-only futures investor must roll positions before expiration, selling the expiring contract and buying the next:

- In backwardation: the investor sells the expiring (higher-priced) contract and buys the next (lower-priced) contract, capturing a positive roll yield. The investor is paid to maintain the position.
- In contango: the investor sells the expiring (lower-priced) contract and buys the next (higher-priced) contract, incurring a negative roll yield. The investor pays to maintain the position.

Roll yield is a structural return component independent of spot price moves. Over time, persistent backwardation rewards long investors; persistent contango penalizes them. A commodity futures fund's total return ≈ spot return + roll yield + collateral return (interest on margin).

### Compute Total Futures Return, Not Just Spot

The investor's return is not the spot price change. It is:

total return ≈ spot price change + roll yield + collateral yield

- A commodity can rise in spot terms while a long futures fund loses money, if contango roll drag exceeds the spot gain.
- Conversely, a commodity can fall in spot terms while a long futures fund makes money, if backwardation roll yield exceeds the spot loss.

This is why commodity index returns diverge so sharply from spot prices over time. Always decompose futures returns into spot, roll, and collateral components.

### Relate Curve Shape To Fundamentals

Contango and backwardation are not random; they reflect physical market conditions:

- Backwardation signals tight near-term supply, low inventories, and a scarcity premium; producers and consumers pay up for immediate delivery. Common when supply is constrained or demand is strong.
- Contango signals oversupply, high inventories, and ample storage; the cost of carry (storage, insurance, financing) pushes forward prices above spot up to the carry ceiling. Common in gluts.
- Full carry: contango cannot exceed the cost of storing the commodity (else arbitrage), bounding how steep contango can get for storable goods.
- Non-storable commodities (electricity, some agricultural): curve behavior differs because storage arbitrage does not apply.

Read the curve as a real-time signal of the physical balance. A move into deeper backwardation suggests tightening; a move into contango suggests loosening.

### Distinguish Curve Trading From Directional Trading

Investors can express different views via the curve:

- Long-only directional: long the front month, exposed to spot and front-end roll; simplest but most exposed to roll drag in contango.
- Curve/roll trades: long one maturity and short another (calendar spread), isolating the curve shape change from the spot level; used to express views on tightening (long near/short far) or loosening (short near/long far).
- Roll-down strategies: positions designed to capture positive roll-down as a contract moves toward expiration along a downward-sloping (backwardated) curve.

A pure directional view and a curve view are different trades. An investor bullish on a commodity should still check whether the curve (roll yield) supports or erodes the position.

### Account For Cost Of Carry And Storage Constraints

The cost of carry bounds curve behavior for storable commodities:

- Storage cost: physical storage, insurance, financing; sets the maximum contango (full carry).
- Storage capacity limits: when storage fills (as in the 2020 WTI negative-price event), contango can blow out and futures can go negative as physical holders pay to exit delivery obligations.
- Convenience yield: the benefit of holding physical inventory (for production or sale) creates backwardation; high convenience yield reflects scarcity.

Cost of carry and convenience yield explain why curves differ across commodities and across time. Oil, gold, grains, and natural gas have very different curve behaviors due to storability and storage economics.

### Apply To Commodity Funds And Index Construction

Most commodity exposure comes via index funds, whose construction affects roll drag:

- Front-month rolling indexes (e.g., simple oil ETFs): most exposed to front-end roll; high drag in contango.
- Optimized/roll-optimized indexes: roll into further-out months to reduce drag; reduces but does not eliminate contango cost.
- Dynamic roll and enhanced indexes: actively choose roll timing to capture roll yield; can outperform static indexes but add complexity and tracking differences.
- Diversified commodity indexes: blend many commodities, smoothing individual curve effects but diluting exposure.

Understand the index methodology before buying a commodity fund. Two "oil" funds with different roll strategies can have very different returns in the same spot environment.

## Common Traps

### Equating Spot Price Moves With Futures Fund Returns

Futures returns include roll yield. A fund can lose in contango while spot rises, or gain in backwardation while spot falls. Always decompose.

### Buying Long-Only Commodity Funds In Persistent Contango

Contango roll drag is a structural headwind. Long-only funds in contangoed markets systematically underperform spot, sometimes dramatically.

### Ignoring The Curve When Taking A Directional View

A bullish spot view is undermined by a contango curve (negative roll) and reinforced by backwardation (positive roll). Check the curve before sizing the position.

### Treating The Curve As A Forecast To Fade

The curve embeds supply, demand, inventory, and carry information. Fading it casually, without a fundamental view on why the balance will shift, is a bet against informed pricing.

### Assuming All Commodity Funds Are The Same

Roll methodology, index construction, and commodity mix drive large return differences. Read the methodology, not just the name.

### Forgetting Storage And Capacity Constraints

Contango is bounded by cost of carry for storable goods, and storage saturation can cause extreme events (negative prices). Curve behavior depends on storability.

## Self-Check

- [ ] Contango and backwardation are defined precisely, and curve shape is read as a signal of the physical supply-demand-inventory balance.
- [ ] Roll yield is computed and included as a return component; total futures return is decomposed into spot, roll, and collateral.
- [ ] The investor's return is not equated with spot price change; the impact of contango drag or backwardation benefit is quantified.
- [ ] Curve shape is related to fundamentals (tightness, oversupply, cost of carry, convenience yield, storage constraints).
- [ ] Directional and curve/roll trades are distinguished, and the curve's support or drag on a directional view is checked.
- [ ] Cost of carry, storage capacity, and non-storability differences across commodities are accounted for.
- [ ] Commodity fund/index methodology (front-month, optimized, dynamic roll, diversified) is read before purchase.
- [ ] The conclusion avoids presenting commodity futures as a spot-price bet and references investor-specific horizon, curve view, and risk tolerance.
