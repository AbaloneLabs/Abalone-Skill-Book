---
name: energy_and_oil.md
description: Use when the agent is evaluating energy and crude oil investments, analyzing supply and demand drivers, OPEC policy and shale economics, inventory and seasonality, refining margins and crack spreads, or deciding how to gain exposure to oil via futures, equities, or funds.
---

# Energy And Oil

Crude oil is the most heavily-traded commodity and a major macro input, but its price is set by a complex balance of supply (OPEC decisions, shale economics, geopolitical disruptions), demand (economic growth, transport, seasonality), and inventories. Oil investing is also layered with structural complications: most investors cannot store crude, so they use futures (with roll yield), equities (with operational and corporate risk), or funds (with structural tracking issues). Treating oil as a simple directional bet on "the oil price" — without understanding the curve, the drivers, and the vehicle — leads to losing money even when the directional view is correct.

Use this skill before answering questions such as "should I invest in oil", "how do I get exposure to oil prices", "what drives oil prices", or "are oil stocks a good way to play oil". The goal is to prevent the agent from conflating spot oil price moves with investor returns, and from ignoring OPEC, shale, inventory, and refining-margin dynamics.

## Core Rules

### Model Supply: OPEC, Shale, And Geopolitics

Oil supply is managed and disrupted, not purely market-driven:

- OPEC+ production policy: OPEC and allies (notably Russia) coordinate production cuts or increases to manage price; decisions can move price sharply and are political as well as economic.
- Shale (tight oil) economics: US shale is short-cycle and responsive to price; when prices rise, shale drilling ramps quickly, capping price spikes; when prices fall, rigs drop. Shale has made supply more elastic than in the pre-shale era.
- Spare capacity: OPEC spare capacity buffers disruptions; low spare capacity raises price sensitivity to outages.
- Geopolitical disruptions: war, sanctions, sabotage, and regime risk in producing regions cause supply shocks.
- Decline rates: natural production decline requires continuous investment to maintain output; underinvestment (e.g., during prolonged low prices) tightens future supply.

Supply analysis must combine OPEC policy, shale responsiveness, geopolitical risk, and the investment cycle. A static supply assumption misses the dynamic response.

### Model Demand: Growth, Transport, And Energy Transition

Oil demand is driven by economic activity and transport, with structural shifts:

- Economic growth and industrial activity: oil demand correlates with GDP, especially in emerging markets where consumption per capita is rising.
- Transport: gasoline, diesel, jet fuel, and shipping are the largest demand segments; sensitive to mobility, trade, and travel.
- Seasonality: gasoline peaks in summer (driving season); heating oil in winter; hurricanes can disrupt both supply and demand.
- Energy transition: EV adoption, efficiency, and policy (fuel standards, carbon pricing) create structural headwinds to long-term demand growth, though timing is uncertain.
- Price elasticity: demand is relatively inelastic short-term (people need fuel) but responds to sustained high prices over time.

Demand analysis must separate cyclical (growth, season) from structural (transition) forces. A cyclical demand surge does not negate the long-term transition headwind.

### Track Inventories And The Balance

The price of oil reflects the balance between supply and demand, visible in inventories:

- Inventories (crude and products): build when supply exceeds demand, draw when demand exceeds supply; the market watches weekly inventory data closely.
- Days of forward cover: inventory relative to demand; low cover raises price sensitivity to disruptions.
- Storage capacity and contango: when storage fills and the curve is in contango, physical holders earn roll but face storage cost and capacity limits.
- Strategic reserves: government stockpiles (SPR in the US) can be released to cushion supply shocks.

Inventory data is the real-time read on the supply-demand balance. A tightening balance (falling inventories) supports price; a loosening balance pressures it.

### Understand The Futures Curve, Roll Yield, And Contango/Backwardation

Most investors gain oil exposure via futures, where the curve shape determines roll yield (see the contango/backwardation skill):

- Backwardation (near prices > far prices): rolling futures earns positive roll yield; long investors are paid to roll. Reflects tight near-term supply.
- Contango (near < far): rolling futures incurs negative roll yield; long investors pay to roll. Reflects oversupply and storage economics.
- Long-term futures funds: the return is spot change plus roll yield; in persistent contango, a fund can lose money even if spot eventually rises.

Oil futures returns diverge sharply from spot price changes because of roll yield. Never equate "oil went up" with "an oil futures fund made money" without accounting for the curve and rolls.

### Separate Upstream, Midstream, And Downstream Equity Exposure

Oil equities are not pure oil-price exposure; they segment by value chain:

- Upstream (E&P): exploration and production; most direct leverage to the oil price, but with cost structure, basin quality, and balance-sheet risk.
- Oilfield services: provide drilling and services; leveraged to activity levels, not directly to price.
- Midstream (pipelines, storage, terminals): fee-based, less price-sensitive, more volume- and contract-driven; MLP structures (see infrastructure skill).
- Downstream (refining): margin-driven (crack spread between crude input and refined output), inversely exposed to crude cost in some scenarios.
- Integrated majors: combine segments, smoothing exposure but diluting pure-price leverage.

Refining margins (crack spreads) are a distinct profit driver from the crude price. A refiner can thrive when crude falls (cheaper input, stable product prices) and suffer when crude rises. Choose the segment that matches the thesis.

### Choose The Right Vehicle And Understand Tracking Error

Oil exposure vehicles have very different risk and tracking profiles:

- Futures-based oil ETFs/funds: provide futures exposure but incur roll yield, management fees, and structural tracking error versus spot; long-only futures funds in contango systematically underperform spot.
- Equity exposure (E&P, integrated, services): adds corporate, operational, ESG, and management risk; leveraged but not pure.
- Direct physical: impractical for crude (storage, regulation); not a realistic retail option.
- Options and structured products: tactical, with volatility and time decay.

Match the vehicle to the thesis. A pure oil-price view is best expressed via futures (accepting roll); a long-term energy view may favor equities (accepting corporate risk). Do not assume an oil ETF tracks the oil price.

### Account For Volatility, Geopolitics, And Tail Risk

Oil is among the most volatile major assets:

- Price swings of 20-50% in months are common, driven by OPEC surprises, geopolitical events, and demand shocks.
- Tail risks: supply outages (war, sanctions), demand collapses (pandemic, recession), and negative pricing anomalies (WTI going negative in 2020 due to storage saturation).
- Position sizing must reflect this volatility; leveraged oil positions can be wiped out in normal-range moves.

Oil is not a stable income asset. Size positions for volatility, and avoid leverage that cannot survive a normal oil drawdown.

## Common Traps

### Equating Spot Oil Price Moves With Futures Fund Returns

Futures funds incur roll yield. In contango, a fund can lose money while spot rises. Always account for the curve and rolls.

### Treating Oil Stocks As Pure Oil-Price Exposure

Oil equities add corporate, operational, cost, ESG, and management risk. They are leveraged equities, not oil.

### Ignoring OPEC And Shale Supply Elasticity

OPEC can cut to support price; shale can ramp quickly to cap it. Static supply assumptions miss the managed and elastic nature of supply.

### Confusing Refining Margins With The Crude Price

Refiners profit from the crack spread, not the crude price. They can thrive when crude falls and suffer when it rises.

### Overlooking Energy Transition As A Long-Term Demand Headwind

Cyclical demand strength does not negate the structural transition. Long-horizon investors must weigh stranded-asset and demand-decline risk.

### Underestimating Volatility And Tail Risk

Oil routinely moves 20-50% in months and has hit extreme tail events (negative pricing). Sizing and leverage must survive normal-range volatility.

## Self-Check

- [ ] Supply analysis covers OPEC+ policy, shale elasticity, spare capacity, geopolitical disruption, and the investment/decline cycle.
- [ ] Demand analysis separates cyclical (growth, season) from structural (energy transition) forces.
- [ ] Inventories and the supply-demand balance are tracked as the real-time price driver.
- [ ] The futures curve, roll yield, and contango/backwardation are accounted for; futures fund returns are not equated with spot moves.
- [ ] Equity exposure is segmented (upstream, services, midstream, downstream, integrated) and matched to the thesis; refining margins are distinguished from crude price.
- [ ] The exposure vehicle (futures fund, equity, options) is chosen with awareness of tracking error, roll, and corporate risk.
- [ ] Volatility, geopolitical tail risk, and sizing reflect oil's normal-range swings and extreme events.
- [ ] The conclusion avoids presenting oil as a stable directional bet and references investor-specific horizon, volatility tolerance, and risk capacity.
