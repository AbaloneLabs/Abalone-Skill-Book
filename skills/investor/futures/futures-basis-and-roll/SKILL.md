---
name: futures_basis_and_roll.md
description: Use when the agent is evaluating futures basis, roll yield, and term structure, assessing convergence, cost of carry, contango and backwardation, and how the shape of the futures curve — not just the spot price — determines the return from holding futures over time.
---

# Futures Basis And Roll

A futures position's return is not just the change in the spot price. It also includes basis (the difference between futures and spot), convergence as the contract approaches expiration, and roll yield (the gain or loss from rolling expiring contracts into later months). The shape of the futures curve — contango (upward-sloping) or backwardation (downward-sloping) — determines whether rolling generates a positive or negative return independent of spot price movement. Investors who focus only on the spot price forecast, ignoring basis, convergence, and roll, will misunderstand futures returns and can lose money even when correctly forecasting the spot direction. Evaluating futures requires reading the entire term structure, not just the front-month price.

Use this skill before answering questions such as "how do futures returns work", "what is roll yield", "what is the difference between contango and backwardation", or "why did my futures position lose money when the spot rose". The goal is to prevent the agent from equating futures returns with spot-price changes, and from ignoring basis, convergence, and the roll-return impact of term structure.

## Core Rules

### Decompose Futures Return Into Spot, Basis, And Roll

Total futures return has multiple components:

- Spot return: the change in the underlying's spot price.
- Basis: the difference between the futures price and the spot price; basis converges to zero as the contract approaches delivery (convergence).
- Roll return (roll yield): the gain or loss from rolling an expiring contract into the next month; depends on the shape of the term structure.
- Collateral return: the interest earned on the cash collateral backing the futures position (relevant for fully-funded index returns).

Total return = spot return + roll return + collateral return (approximately). Roll return can be positive or negative and can dominate total return in range-bound markets. Do not equate futures return with spot return.

### Understand Convergence And Basis Risk

Basis converges, but not always perfectly:

- Convergence: at delivery, futures and spot prices converge (basis goes to zero) because of arbitrage; a position held to delivery realizes the spot return plus the initial basis.
- Cash settlement versus physical delivery: cash-settled contracts converge to an index or reference price; physical-delivery contracts converge to the deliverable spot; convergence mechanics differ.
- Basis risk: for hedgers, basis risk is the risk that the futures price and the hedged exposure do not move together perfectly; imperfect convergence or a mismatch between the hedge and the exposure leaves residual risk.
- Convergence in stressed markets: convergence can fail or weaken in stressed or illiquid markets (e.g., WTI going negative in April 2020 due to storage constraints).

Convergence is the mechanism that ties futures to spot, but basis risk remains for hedgers, and convergence can fail under stress. Understand the settlement mechanism and delivery conditions.

### Analyze Contango, Backwardation, And Roll Yield

Term structure determines roll yield:

- Contango (upward-sloping curve): later contracts are more expensive than near contracts; rolling from a cheaper expiring contract to a more expensive later contract incurs a cost (negative roll yield). Common when there is a cost of carry (storage, insurance, financing) or when supply exceeds near-term demand.
- Backwardation (downward-sloping curve): later contracts are cheaper than near contracts; rolling from an expensive expiring contract to a cheaper later contract generates a gain (positive roll yield). Common when near-term demand exceeds supply or when there is a scarcity premium.
- Roll yield impact: in contango, roll yield is a persistent drag that can exceed spot gains; in backwardation, roll yield is a persistent tailwind. Roll yield can dominate total return in range-bound markets.
- Structural tendency: many commodities are structurally in contango (cost of carry), eroding long-only index returns; energy markets move between contango and backwardation with supply-demand cycles.

The shape of the curve is as important as the spot forecast. A long futures position in steep contango loses to roll even if the spot is flat; a position in backwardation gains from roll even if the spot is flat.

### Distinguish Cost Of Carry From Supply-Demand Drivers

Term structure has two main drivers:

- Cost of carry: for storable commodities, contango reflects the cost of storing, insuring, and financing the commodity (the arbitrage that prevents the curve from being too flat); this is a structural drag on long positions.
- Supply and demand (convenience yield): backwardation reflects a scarcity premium — near-term demand exceeds supply, and holders of the physical commodity earn a "convenience yield" that makes spot more valuable than future delivery; this benefits long futures positions.
- Financial assets (equities, bonds, currencies): for financial futures, cost of carry includes interest rates and (for equities) dividends; the forward price reflects interest minus dividend yield.

Understand whether the curve shape reflects cost of carry (a structural drag) or supply-demand scarcity (a roll tailwind). The driver determines whether the roll yield is persistent or cyclical.

### Evaluate Roll Mechanics And Their Impact On Returns

Roll mechanics affect realized returns:

- Roll timing: contracts are rolled on a schedule (e.g., 5 days before expiration for indices); the roll moves a large notional from one contract to the next, and crowded rolls can affect prices.
- Roll frequency: more frequent rolling (front-month) captures more roll yield (positive or negative) but incurs more transaction costs; less frequent rolling (longer-dated contracts) captures less roll yield but lower costs.
- Roll slippage: rolling in illiquid contracts or during crowded roll periods incurs slippage.
- Index roll methodology: commodity index funds follow defined roll schedules; these schedules are known and can be front-run, affecting index returns.

Roll mechanics — timing, frequency, slippage, and index methodology — affect realized returns. Understand the roll schedule and its impact.

### Assess Liquidity And Contract Specifications

Contract specifications and liquidity matter:

- Contract size and tick value: the notional value per contract and the minimum price increment; these determine position granularity and transaction costs.
- Liquidity: front-month contracts are most liquid; deferred months are less liquid with wider spreads; rolling into illiquid contracts incurs costs.
- Position limits and reporting: exchange position limits and large-trader reporting affect large positions.
- Margin and leverage: futures are leveraged; margin requirements determine capital efficiency but also amplify losses; margin calls can force liquidation.

Understand contract specifications, liquidity (especially in deferred months), position limits, and margin/leverage implications. Illiquid deferred months and leverage amplify risk.

## Common Traps

### Equating Futures Return With Spot-Price Change

Total futures return includes roll yield and collateral return, not just spot change. Roll can dominate in range-bound markets.

### Ignoring Roll Drag In Contango

Long positions in contango incur persistent negative roll yield that can exceed spot gains. Many commodity indices suffer structural roll drag.

### Overlooking Backwardation As A Roll Tailwind

Backwardation generates positive roll yield that benefits long positions even when spot is flat. Scarcity-driven backwardation is a tailwind.

### Forgetting Basis Risk In Hedging

Basis risk — the imperfect correlation between futures and the hedged exposure — leaves residual risk even in a "hedged" position.

### Assuming Convergence Is Guaranteed

Convergence can weaken or fail under stress (storage constraints, illiquidity, delivery dislocations). Cash and physical settlement differ.

### Underestimating Leverage And Margin-Call Risk

Futures are leveraged; adverse moves trigger margin calls that can force liquidation at unfavorable prices. Leverage amplifies losses.

## Self-Check

- [ ] Total futures return is decomposed into spot return, roll yield, and collateral return, not equated with spot change.
- [ ] Basis, convergence, and the settlement mechanism (cash vs. physical delivery) are understood, including convergence-failure risk under stress.
- [ ] Contango (negative roll yield) and backwardation (positive roll yield) are analyzed, and the term structure's impact on return is assessed.
- [ ] The driver of curve shape (cost of carry vs. supply-demand scarcity/convenience yield) is distinguished.
- [ ] Roll mechanics (timing, frequency, slippage, index methodology) and their impact on realized returns are understood.
- [ ] Contract specifications, liquidity (especially deferred months), position limits, and margin/leverage implications are assessed.
- [ ] The conclusion references the specific term structure and its roll-yield implication, not just the spot forecast, and includes appropriate risk disclosure regarding leverage and the need for futures-trading experience.
