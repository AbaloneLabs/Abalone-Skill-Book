---
name: vol_surface_and_term_structure.md
description: Use when the agent is interpreting the volatility surface, analyzing skew and smile, reading the VIX futures term structure (contango versus backwardation), estimating roll yield, or trading calendar and term-structure spreads in options or vol. Covers implied vol skew, term structure shape, roll cost of VIX futures, and how contango and backwardation signal market stress and create structural drag or opportunity.
---

# Vol Surface And Term Structure

The volatility surface is the three-dimensional structure of implied volatilities across strikes and maturities. It is not flat: out-of-the-money puts typically carry higher IV than calls (skew), and short-dated options often price different vol than long-dated ones (term structure). These shapes are not curiosities; they encode the market's joint expectations about downside risk, vol persistence, and event timing. Reading the surface, and especially the VIX futures term structure, is essential for anyone trading vol, options spreads, or vol-linked products.

Agents tend to flatten vol analysis to a single number ("the VIX is at 18") and miss that the shape carries most of the information. A flat VIX reading can accompany a steeply backwardated term structure (stress) or deep contango (calm with roll drag). The judgment problem is reading the surface and term structure as a coherent picture of market expectations, understanding the roll mechanics that contango and backwardation impose, and avoiding the trap of trading spot vol when the tradeable instruments are futures with their own dynamics.

This skill applies to analyzing vol skew and smile, the VIX futures curve, roll yield, calendar spreads, and term-structure-based vol strategies. It is not investment advice; term structure can shift abruptly, contango is not guaranteed, and VIX derivatives have structural behaviors that can produce large losses.

## Core Rules

### Read Skew As A Price On Downside Risk

Volatility skew, the higher IV of out-of-the-money puts relative to calls, reflects the market's willingness to pay for downside protection. Steep skew signals strong demand for puts (fear of gaps and crashes); flat or inverted skew signals complacency or demand for upside calls. Skew is itself informative and is distinct from the overall vol level.

Interpret skew in context: steep put skew in a low-VIX environment can warn of hidden tail concern; flat skew in a high-VIX environment may indicate the elevated vol is expected to be symmetric. Track skew relative to its own history, not just its absolute level. Changes in skew often precede or accompany regime shifts.

### Distinguish Term Structure States And Their Meaning

The VIX futures term structure is the curve of futures prices across expiration months. In contango, distant futures are pricier than near ones, reflecting the expectation that current low spot vol will rise toward a long-run average; this is the normal state in calm markets. In backwardation, near futures exceed distant ones, reflecting current stress expected to mean-revert down; this occurs during volatility spikes.

The state of the term structure is a powerful signal. Persistent contango indicates calm; transition to backwardation indicates stress. The depth and persistence of backwardation often marks risk-off extremes that historically coincide with market bottoms, though timing such extremes is unreliable. Read the curve's shape, not just the spot VIX, because the same spot level can accompany very different curves and very different forward expectations.

### Quantify The Roll Yield From Term Structure

Because VIX futures converge to the VIX spot at expiry, a futures curve in contango produces a negative roll yield for long holders: as each futures contract rolls down the curve toward expiry, it loses value relative to the next contract, all else equal. This is structural drag. In backwardation, the roll is positive for long holders, but backwardation typically coincides with already-elevated spot and realized losses.

Estimate the roll yield explicitly as the annualized difference between near and next contracts relative to price. Long-vol positions held through contango bleed steadily; this is why long-vol ETPs and funds decay over time even if spot VIX is unchanged. Any long-vol thesis must overcome the roll cost, not just be right about vol direction. Conversely, short-vol positions earn the roll in contango but face unlimited risk in spikes.

### Match The Tradeable Instrument To The Thesis

Spot VIX is not directly tradeable. The tradeable instruments are VIX futures, options on those futures, and ETPs derived from them, each with its own term structure, roll, and decay dynamics. A thesis about spot vol must be translated into the specific instrument whose payoff reflects that thesis after rolls and costs.

Clarify what the position actually exposures you to: a long VIX call spread, a long front-month future, a long mid-curve future, or a short ETP each behaves differently. A view that "vol will rise" can still lose money on a long futures position if the rise occurs in spot but the futures curve adjusts. Structure the trade so the instrument's payoff matches the thesis horizon and mechanism.

### Use Calendar And Diagonal Spreads To Trade Term Structure

Term-structure trades express a view on the relative pricing of different maturities. A calendar spread (long one maturity, short another) isolates the shape of the curve from the overall vol level. In options, this trades the difference in IV between two expirations.

Design these spreads to express a specific term-structure view (e.g., near-dated vol is overpriced relative to far-dated, or vice versa) while managing the Greeks, especially vega and gamma differences. Calendar spreads can be low-cost but have complex risk; the near leg has high gamma and the far leg high vega, so the position's behavior changes as expiration approaches. Understand how the spread's risk profile evolves, not just its initial setup.

### Recognize That The Surface Moves As A System

Skew, term structure, and the overall vol level move together but not in lockstep. A vol spike usually steepens skew and flattens or inverts term structure; a vol decline usually flattens skew and steepens contango. But the relationships are regime-dependent and can diverge. Treating any one dimension in isolation misses the joint dynamics.

Analyze the surface as a coherent system: what does the combination of level, skew, and term structure imply about the market's expectations? Divergences (e.g., rising vol with flattening skew) can signal specific regimes or positioning effects worth investigating. The surface is a picture; read the whole picture.

### Account For Event Risk In Short-Dated Vol

Short-dated options and front-month futures are most sensitive to imminent events (earnings, central bank meetings, elections). The term structure around such events can show bumps or inversions that reflect event pricing, not general vol expectations. Reading these as pure term-structure signals misinterprets event-driven demand.

Identify scheduled events within the horizon of the instruments being analyzed and separate event-driven term-structure effects from the baseline curve. A steep contango broken by a single elevated expiry often reflects an upcoming event, not a structural view.

## Common Traps

### Trading Spot VIX As If It Were Directly Tradeable

Spot VIX cannot be held; only derivatives can. The trap is forming a thesis about spot and executing in futures or ETPs whose returns diverge from spot due to roll and decay. Always translate the thesis into the actual instrument's payoff.

### Ignoring Contango Drag On Long-Vol Positions

Long-vol ETPs and futures bleed in contango. The trap is holding long vol expecting spot mean reversion while the roll cost erodes capital even if vol is unchanged. Quantify and overcome the roll.

### Reading Only The Spot VIX Level

The spot number hides the curve. The trap is concluding "vol is low, buy" or "vol is high, sell" without examining contango/backwardation and skew. The shape carries the forward-looking information.

### Assuming Backwardation Marks The Exact Bottom

Backwardation signals stress and has historically appeared near vol peaks, but it can persist or deepen. The trap is buying risk immediately upon seeing backwardation. Use it as a condition, not a timing trigger.

### Misreading Event-Driven Term-Structure Bumps

A single elevated expiry from an upcoming event is not a structural curve signal. The trap is treating it as backwardation or contango. Separate event effects from the baseline curve.

### Underestimating Calendar Spread Complexity

Calendar spreads have evolving Greek profiles. The trap is setting them up for the initial thesis and ignoring how gamma/vega change as expiration nears. Model the spread through its life.

### Treating Skew As Static

Skew shifts with regime and positioning. The trap is assuming a fixed "normal" skew. Track skew relative to its own history and watch for divergences from the vol level.

## Self-Check

- [ ] Skew is read as a price on downside risk and tracked relative to its own history, not just its absolute level.
- [ ] The term structure state (contango vs backwardation) is identified and interpreted for what it signals about stress and forward expectations.
- [ ] Roll yield from contango/backwardation is quantified, and long-vol positions are assessed net of roll drag.
- [ ] The tradeable instrument's payoff is confirmed to match the thesis horizon and mechanism, not the spot VIX level.
- [ ] Calendar and diagonal spreads are designed to isolate term-structure views with their evolving Greek profiles understood.
- [ ] The surface is analyzed as a coherent system of level, skew, and term structure, including divergences.
- [ ] Scheduled events within the horizon are identified and separated from baseline term-structure signals.
- [ ] The analysis does not treat spot VIX as directly tradeable or ignore roll mechanics.
- [ ] Contango drag and backwardation risk are explicitly weighed in any vol-position recommendation.
- [ ] The conclusion is probabilistic and notes term structure can shift abruptly and VIX derivatives carry structural loss risk; it is not personalized advice.
