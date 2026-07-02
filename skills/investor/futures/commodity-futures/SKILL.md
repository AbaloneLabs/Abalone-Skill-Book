---
name: commodity_futures.md
description: Use when the agent is evaluating commodity futures investments, assessing seasonality and inventory cycles, supply and demand drivers, roll yield and term structure, contango and backwardation dynamics, and the specialized tax treatment that makes commodity futures returns differ from other asset classes.
---

# Commodity Futures

Commodity futures offer exposure to energy, metals, agriculture, and livestock — assets that can diversify a portfolio and hedge inflation. But commodity futures returns are not commodity spot returns; they are driven by the interaction of spot price, roll yield (contango or backwardation), and collateral return, and they are highly cyclical, seasonal, and supply-shock-driven. A commodity futures position can lose money even when the spot price rises (due to contango roll drag), and the tax treatment (e.g., Section 1256 in the US) differs from equities and bonds. Evaluating commodity futures requires understanding the specific commodity's supply-demand dynamics, the term structure, the roll mechanics, and the tax treatment — not just a forecast of the commodity's price direction.

Use this skill before answering questions such as "should I invest in commodity futures", "how do commodity futures returns work", "what drives oil or gold futures", or "are commodities a good inflation hedge". The goal is to prevent the agent from equating commodity futures returns with spot-price changes, and from ignoring roll yield, seasonality, supply shocks, and tax treatment.

## Core Rules

### Decompose Commodity Futures Return Into Spot, Roll, And Collateral

Commodity futures returns have three components:

- Spot return: the change in the commodity's spot price.
- Roll return (roll yield): the gain or loss from rolling expiring contracts; depends on the term structure (contango = negative roll yield; backwardation = positive roll yield).
- Collateral return: the interest earned on the cash backing the futures position (often assumed at the money-market rate).
- Total return ≈ spot return + roll return + collateral return.

Roll return can dominate total return in range-bound markets. A long position in steep contango loses to roll even if the spot is flat. Do not equate futures return with spot return.

### Analyze Supply And Demand Drivers For Each Commodity

Each commodity has distinct supply-demand dynamics:

- Energy (oil, gas): OPEC+ production decisions, shale economics, geopolitical supply disruptions, inventory levels, refining capacity, seasonal demand (driving season, winter heating).
- Metals (gold, copper, industrial metals): gold driven by real rates, central-bank demand, safe-haven flows; industrial metals driven by industrial production, construction, and inventory cycles.
- Agriculture (grains, oilseeds, soft commodities): weather-driven supply (droughts, frosts), planting decisions, global demand, trade policy; highly seasonal and weather-sensitive.
- Livestock (cattle, hogs): feed costs, biological production cycles, disease outbreaks, consumer demand.

Each commodity's price is driven by commodity-specific factors. Do not treat "commodities" as a homogeneous asset class; analyze the specific commodity's supply-demand balance.

### Assess Inventory, Seasonality, And Storage Economics

Inventory and seasonality shape term structure:

- Inventory levels: high inventories suggest ample supply and favor contango (cost of carry dominates); low inventories suggest scarcity and favor backwardation (convenience yield dominates).
- Seasonality: agricultural commodities have planting, growing, and harvest seasonality; energy has seasonal demand (summer driving, winter heating); seasonality affects both spot prices and term structure.
- Storage economics: for storable commodities, the cost of storage, insurance, and financing sets the maximum contango; storage constraints can cause extreme dislocations (e.g., WTI negative prices).
- Convenience yield: the benefit of holding the physical commodity in scarcity; high convenience yield drives backwardation and positive roll yield.

Inventory levels, seasonality, and storage economics determine the term structure and thus the roll yield. Assess the inventory cycle and storage conditions, not just the spot forecast.

### Evaluate Contango, Backwardation, And Roll Yield Impact

Term structure is central to commodity futures returns:

- Contango drag: many commodities are structurally in contango (cost of carry), creating persistent negative roll yield that erodes long-only index returns; the spot can rise while the futures position loses to roll.
- Backwardation benefit: backwardation creates positive roll yield; long positions benefit even in flat spot markets; energy markets in supply tightness often exhibit backwardation.
- Curve shape changes: the curve shifts between contango and backwardation with the inventory cycle; a long position's roll yield changes as the curve reshapes.
- Index roll drag: broad commodity indices (e.g., the S&P GSCI) suffer structural roll drag because they are long-only and weighted toward often-contangoed energy; index returns underperform spot returns over time.

The term structure determines whether a long position earns or pays to roll. Assess the curve shape and its impact on total return, and recognize the structural roll drag in long-only commodity indices.

### Understand The Role Of Commodities In A Portfolio

Commodities can serve specific portfolio roles:

- Inflation hedge: commodities, particularly energy and industrial metals, tend to do well during inflation surprises and supply shocks; they can hedge inflation that harms stocks and bonds.
- Diversification: commodities have historically had low correlation to stocks and bonds, improving portfolio diversification; however, correlations can rise during risk-off episodes.
- Supply-shock hedge: commodities benefit from adverse supply shocks (geopolitical, weather) that hurt equities; this is a distinct diversification benefit.
- Cyclical and volatile: commodities are highly cyclical and volatile; they can suffer multi-year drawdowns and should be sized modestly.

Commodities can diversify and hedge inflation, but they are volatile and cyclical. Size the allocation modestly and understand the cyclical risk.

### Consider Tax Treatment And Access Vehicles

Commodity futures have specialized tax and access considerations:

- Section 1256 (US): broad-based index futures and certain commodity futures are subject to mark-to-market treatment with 60% long-term / 40% short-term capital gains treatment, regardless of holding period; this is more favorable than ordinary income but differs from equities.
- Commodity ETFs/ETNs: many commodity ETFs hold futures and have their own tax treatment; ETNs have different structures; K-1 versus 1099 reporting varies by vehicle.
- Roll drag in ETFs: commodity futures ETFs incur the same roll drag as direct futures; the ETF's return reflects spot, roll, and expenses.
- Direct futures versus vehicles: direct futures require margin accounts and expertise; ETFs and mutual funds provide easier access but add expense ratios and may have tracking error from roll mechanics.

Understand the tax treatment (Section 1256, ETF/ETN structures) and the access vehicle's roll mechanics and expenses. The choice of vehicle affects after-tax return and tracking.

### Distinguish Direct Commodity Exposure From Commodity Equities

Commodity equities are not the same as commodity futures:

- Commodity equities (oil majors, miners, agribusiness): exposed to the commodity price but also to equity-market risk, management, operational, and balance-sheet factors; equity returns diverge from commodity returns.
- Operational leverage: commodity producers have operational leverage; their equity can be more volatile than the commodity.
- Correlation: commodity equities correlate more with the broad equity market than the commodity itself, reducing diversification benefit.
- Choice of exposure: for pure commodity exposure, futures are more direct; for equity exposure to commodity-producing companies, equities are appropriate.

Distinguish direct commodity exposure (futures) from commodity-equity exposure. They have different risk-return profiles and diversification characteristics.

## Common Traps

### Equating Commodity Futures Return With Spot-Price Change

Roll yield and collateral return are material components. Contango drag can cause losses even when spot rises.

### Treating Commodities As A Homogeneous Asset Class

Each commodity has distinct supply-demand, inventory, and seasonal dynamics. Analyze the specific commodity.

### Ignoring Structural Roll Drag In Long-Only Indices

Broad commodity indices suffer persistent roll drag from contango. Index returns underperform spot over time.

### Confusing Commodity Equities With Direct Commodity Exposure

Commodity equities carry equity-market, management, and operational risk; their returns diverge from the commodity. Futures are more direct.

### Overlooking Seasonality And Inventory Cycles

Seasonality and inventory levels shape both spot prices and term structure. Ignoring them misses key drivers.

### Forgetting Specialized Tax Treatment

Section 1256 (60/40 treatment) and ETF/ETN structures affect after-tax return. Vehicle choice matters.

## Self-Check

- [ ] Commodity futures return is decomposed into spot return, roll yield, and collateral return, not equated with spot change.
- [ ] The specific commodity's supply-demand drivers (energy, metals, agriculture, livestock) are analyzed.
- [ ] Inventory levels, seasonality, storage economics, and convenience yield are assessed for their effect on term structure.
- [ ] Contango drag and backwardation benefit are evaluated, and structural roll drag in long-only indices is recognized.
- [ ] The portfolio role (inflation hedge, diversification, supply-shock hedge) is understood, and the allocation is sized modestly given cyclical volatility.
- [ ] Tax treatment (Section 1256, ETF/ETN structures, K-1 vs. 1099) and access-vehicle roll mechanics and expenses are understood.
- [ ] Direct commodity exposure (futures) is distinguished from commodity-equity exposure.
- [ ] The conclusion references the specific commodity's term structure and roll-yield implication, includes appropriate risk disclosure, and notes the need for futures/commodity-trading experience.
