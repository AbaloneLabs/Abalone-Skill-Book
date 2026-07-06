---
name: economy-sinks-sources-and-balance.md
description: Use when the agent is designing a virtual economy, balancing currency sinks and sources, preventing inflation or deflation, setting prices for virtual goods, or evaluating whether an in-game economy will remain stable over the game's lifespan or collapse into hyperinflation.
---

# Economy Sinks, Sources, and Balance

A virtual economy is a closed system of currency flowing in (sources) and out (sinks), and its stability is an emergent property of whether those flows balance. The judgment problem is that economies are counterintuitive: a generous source feels rewarding in testing but produces inflation that makes all rewards worthless over time, and a strict sink feels punishing in testing but is what keeps rewards meaningful long-term. Agents tend to miss this because the imbalance only appears at scale and over time — the economy is stable for the first week and broken by month three — and because the metrics that reveal the problem (currency velocity, price drift) are not the metrics teams watch. The harm is an economy where currency becomes worthless (hyperinflation, players hoard because nothing is worth buying), or where currency is unobtainable (deflation, players cannot afford the rewards the game offers), either of which breaks the reward loop the economy exists to support. This skill covers how to model sources and sinks, detect imbalance early, and tune for long-term stability. The agent has latitude in the specific goods and prices, but the obligation to balance the flows is not optional.

## Core Rules

### Model Every Currency as Sources Minus Sinks Over Time

A currency's value is determined by the net flow: total created (sources: drops, quest rewards, sales) minus total destroyed (sinks: purchases, upgrades, taxes, fees) across the player population over time. The decision rule: for each currency, enumerate every source and sink, estimate the per-player flow rate at each progression stage, and confirm the net is slightly negative (mild sink pressure) so currency retains value. Teams that do not model the flows discover, post-launch, that a source they forgot (vendoring trash loot) floods the economy and inflates away every reward's value.

### Maintain Sink Pressure Slightly Above Source Pressure

A healthy economy has sinks slightly exceeding sources, producing mild scarcity that keeps currency meaningful and gives the player a reason to engage with the economy. The decision rule: tune so that the average player is slightly currency-hungry — able to afford the important things with effort, but unable to hoard to the point of indifference. Economies where sources exceed sinks inflate (currency worthless); economies where sinks vastly exceed sources deflate (currency unobtainable, players quit). The target is gentle scarcity, not abundance and not starvation.

### Design Sinks the Player Wants to Use, Not Punitive Taxes

A sink is only effective if the player chooses to use it, which means the sink must offer something the player values — a desirable item, a meaningful upgrade, a convenience, a cosmetic. The decision rule: design sinks as desirable purchases, not as forced taxes or penalties, because punitive sinks are avoided or resented and fail to remove currency. The strongest sinks are aspirational goals the player willingly grinds toward, which simultaneously removes currency and drives engagement.

### Account for the Whale and the Farmer in Flow Modeling

The average player flow is not what determines stability; the tails do. A small number of high-engagement players (whales) generate disproportionate currency, and a number of optimization-driven players (farmers, bots) exploit the highest-yield source relentlessly, and both can flood the economy regardless of the average. The decision rule: model the economy under adversarial assumptions — a fraction of players farming the most efficient source continuously — and confirm the sinks can absorb that flow. Economies balanced for the average player collapse when the tails exploit the sources.

### Separate Currencies by Timescale to Contain Inflation

A single currency serving both the moment-to-moment economy and the long-arc economy cannot be tuned for both, because the flows operate on different timescales. The decision rule: use separate currencies for different engagement tiers (a common currency for routine play, a rare currency for aspirational goals, a prestige currency for endgame), so inflation in one tier does not corrupt the others. A single currency that must serve all tiers inevitably inflates at the routine tier and starves the aspirational tier, because one flow rate cannot satisfy both.

### Detect Imbalance From Price Drift and Velocity, Not From Stock

The signal of economic imbalance is not how much currency players hold (stock) but how prices drift and how fast currency moves (velocity). The decision rule: instrument the economy with price indices for key goods, currency velocity (turnover rate), and sink-to-source ratios, and watch for drift — rising prices signal inflation, falling transaction volume signal deflation. Teams that watch only aggregate currency stock miss the imbalance, because hoarding masks inflation and spending masks deflation until the economy is already broken.

## Common Traps

### The Generous Source That Inflates the Currency Away

The team, wanting the game to feel rewarding, tunes sources generously — high drop rates, valuable vendor trash, repeatable rewards — and the currency inflates until nothing in the shop is worth buying because the player has more currency than the economy can absorb. The trap is that generous sources feel great in the first session. The false signal is strong early engagement and positive "feels rewarding" feedback. The harm is that within weeks the currency is worthless, rewards that should feel meaningful are trivial, the player has nothing to spend on, and the reward loop the economy supported collapses, because the team optimized for short-term feel over long-term stability.

### The Punitive Sink That Players Avoid or Resent

The team, recognizing inflation risk, adds aggressive sinks — repair costs, fast-travel fees, death penalties — that remove currency but that the player experiences as punishment, and the player avoids them or quits. The trap is that the sink mathematically balances the source. The false signal is that the spreadsheet balances. The harm is that punitive sinks fail in practice because players route around them (stop fast-traveling, stop dying, stop engaging), the currency the sink was meant to remove stays in the economy, and the players who cannot avoid the sink resent it and churn, because a sink the player avoids removes no currency and a sink the player resents removes players.

### Balancing for the Average Player and Ignoring the Tails

The team models the economy using the average player's flow rate, confirms it balances, and ships, only to find the economy floods because a minority of farmers and bots exploit the most efficient source far beyond the average. The trap is that the average is the intuitive modeling unit. The false signal is that the average balances on paper. The harm is that economies are governed by their tails, not their averages, and a small population farming the best source continuously generates currency at rates the average-based sinks cannot absorb, inflating the economy for everyone, because the team balanced for a representative player who does not determine the system's stability.

### Single Currency Asked to Serve All Timescales

The team uses one currency for routine purchases, aspirational goals, and endgame prestige, and cannot tune it for all three: the routine flow inflates it, the aspirational prices become trivial, and the endgame has no scarcity. The trap is that a single currency is simple to implement and communicate. The false signal is that the economy is unified and clean. The harm is that one flow rate cannot serve three timescales, the currency either inflates at the routine tier (trivializing aspirational goals) or is starved at the aspirational tier (frustrating routine play), and the economy that was meant to support engagement across the whole game instead breaks at whichever tier the single rate mismatches.

### Watching Currency Stock Instead of Price Drift

The team monitors how much currency players hold and concludes the economy is stable because the stock looks reasonable, while prices are drifting and velocity is collapsing, because hoarding masks the inflation. The trap is that stock is the visible metric. The false signal is that aggregate currency held looks bounded. The harm is that players hoarding currency (because nothing is worth buying) keeps the stock stable while the economy is already broken, the team does not detect the inflation until prices have drifted catastrophically, and by the time the drift is visible the economy requires painful retroactive fixes, because the team watched the wrong indicator and the imbalance compounded silently.

## Self-Check

- For each currency, have I enumerated every source and sink and confirmed the net flow is slightly negative across the player population over time?
- Is sink pressure tuned slightly above source pressure, producing gentle scarcity rather than abundance or starvation?
- Are my sinks desirable purchases the player wants to use, rather than punitive taxes the player avoids or resents?
- Did I model the economy under adversarial assumptions (farmers, bots, whales exploiting the best source), not just the average player flow?
- Have I separated currencies by timescale so routine, aspirational, and endgame economies do not corrupt each other?
- Am I monitoring price drift, currency velocity, and sink-to-source ratios, rather than aggregate currency stock?
- Did I avoid tuning sources generously for short-term feel at the cost of long-term inflation that trivializes all rewards?
