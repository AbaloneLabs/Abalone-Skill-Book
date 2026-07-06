---
name: sink-and-source-balance-and-inflation-control.md
description: Use when the agent is designing in-game economy sinks and sources, currency flow ledgers, reward rates, repair and upkeep costs, crafting consumption, auction house taxation, prestige resets, or reviewing whether a live economy retains currency value over months without inflating to worthlessness or starving players into grind.
---

# Sink and Source Balance and Inflation Control

A game economy is a closed system in which every unit of currency that enters must eventually leave, or it accumulates until it loses all meaning, and the discipline of balancing the two is what separates an economy that holds its value for years from one that collapses within weeks. The judgment problem is that sources and sinks are designed by different people at different times for different reasons — a content designer adds a reward to make a quest feel generous, a systems designer adds a repair cost to create stakes — and no single person owns the aggregate flow that determines whether the currency inflates or deflates. Designers miss the inflation risk because individual values look reasonable and because the problem is invisible in the short term: a player who earns slightly more than they spend feels rewarded, not endangered, and the surplus only becomes catastrophic when compounded across the player base over months. The harm is severe and arrives late: an inflated currency trivializes the intended progression, erodes the value of every reward the team carefully tuned, makes new content trivial for veterans and impossible to balance, and in monetized games undermines the real-money economy that funds the game. The opposite failure — a deflated economy where sinks exceed sources — produces starvation, grind, and churn at the moment retention matters most. Agents tend to err by tuning sources for generosity and feel without modeling the corresponding sink obligation, by treating live economies as static rather than drifting, or by adding sinks reactively and punitively after inflation appears rather than designing them in from the start. The freedom here is wide — many balanced configurations exist — but the obligation is to model the economy as a flow system with a ledger, to stress-test it across player profiles and time horizons, and to build in control mechanisms before launch rather than after.

## Core Rules

### Build a Source-and-Sink Ledger for Every Currency Before Tuning Any Number

Before setting a single drop rate or price, construct a ledger that lists every source (where currency enters: drops, quest rewards, dailies, vendor sales, event rewards) and every sink (where it leaves: purchases, repairs, upgrades, fast travel, respecs, taxes, consumables) for each currency, with the rate of each under intended play. The ledger is the instrument that makes the economy legible: it reveals whether sources exceed sinks (inflation) or sinks exceed sources (starvation) under the design's own assumptions. The discipline is to maintain this ledger as a living document, updated whenever a source or sink changes, and to refuse to ship a currency whose ledger does not balance under at least the average player profile. The decision criterion: for each currency, can I state the net flow per hour of intended play, and is the resulting surplus or deficit intentional and bounded?

### Tune the Steady-State Surplus to Be Small and Rewarding, Not Large

A healthy currency generates a small, consistent surplus for the average player — enough that play feels rewarding and accumulation is visible, but not so much that the currency loses scarcity. The trap is the belief that more generous is always better: a large surplus feels great in the first session and destroys the economy by the tenth. The discipline is to target a surplus that allows the player to afford intended purchases on the intended schedule (a meaningful upgrade every few hours, a major purchase every few sessions) without accumulating enough to trivialize all future costs. The decision criterion: after twenty hours of intended play, can the average player afford the items the design intends them to have at that point, and are there still purchases that require saving or choice? When everything is affordable immediately, the surplus is too large.

### Match Recurring Sinks to Recurring Sources, Not to One-Time Events

The most robust sinks are recurring and scale with player activity: repair costs that scale with combat, fast-travel fees that scale with travel, crafting consumption that scales with production, auction-house taxes that scale with trading. These sinks self-balance because they activate in proportion to the activities that generate currency. The trap is relying on one-time sinks (a single expensive upgrade, a one-time unlock) that drain a surge of currency once and then never again, leaving the ongoing source flow to inflate the economy afterward. The decision criterion: does the sink recur at a rate proportional to the source it offsets, or is it a one-time drain that leaves the steady-state flow unbalanced? Prefer recurring, activity-proportional sinks; reserve one-time sinks for pacing milestones, not for economic balance.

### Model Multiple Player Profiles, Especially the Optimizer

An economy balanced for the average player will be broken at the extremes. The optimizer (or min-maxer) will find the highest-yield source, repeat it, and accumulate currency far faster than the average, trivializing scarcity within days. The casual player will earn slowly, fall behind the intended curve, and hit walls. The completionist will exhaust sinks and hoard. The discipline is to simulate the economy under at least three profiles — the efficient optimizer, the average player, and the slow or lapsed player — over a multi-week or multi-month horizon, and to confirm none reaches a degenerate state: unbounded accumulation, permanent starvation, or a cliff where the gap between income and required spend becomes insurmountable. The decision criterion: under the optimizer profile, how long until the currency is trivialized, and is there a mechanism to prevent or delay that?

### Use Prestige, Season, and Reset Mechanics as Inflation Control Valves

Any live economy drifts, and the most powerful tool for resetting accumulated value is a structured reset: prestige systems that convert accumulated wealth into a fresh progression track, season structures that introduce a new currency or rebalanced old one, ascension mechanics that drain hoards in exchange for permanent (non-economic) benefits. These are not merely replay features; they are economic control valves that drain the surplus the steady state could not absorb. The discipline is to design at least one such mechanism into the economy from the start, sized to the expected accumulation rate, and to ensure it rewards rather than punishes the reset so that players participate willingly. The decision criterion: if inflation emerges six months post-launch, what mechanism drains the excess without confiscating player investment, and is it already built?

### Distinguish Between Earned and Purchased Currency in the Inflation Model

In monetized economies, premium currency purchased with real money can enter the soft economy through conversions, bundles, and time-savers, and this flow is not bounded by playtime the way earned currency is. A premium-currency injection can flood the soft economy and cause inflation that play-balance cannot address. The discipline is to model the soft economy both with and without premium injections, to identify where premium spending creates soft-currency surpluses, and to ensure that the soft sinks can absorb realistic premium-driven flows without inflating. The decision criterion: under a high-spending player profile, does the premium-to-soft conversion path generate soft-currency accumulation that breaks the sink balance, and if so, is the conversion rate or the soft sink adjusted to compensate?

## Common Traps

### Sources Tuned for Feel Without a Corresponding Sink

A content designer increases a quest reward to make the quest feel satisfying, or adds a login bonus to boost retention, without adding or scaling any sink to absorb the new currency. The trap is that the change is locally correct and globally inflationary, and because no one owns the aggregate flow, the imbalance accumulates across many such decisions. The false signal is that players report the rewards "feel good." The harm is a currency that inflates silently until purchases become trivial, at which point the team cannot identify which of dozens of generous decisions caused it.

### Exponential Costs With Linear Income

Progression uses an exponential cost curve (each tier costs a multiple of the last) while income grows linearly or not at all. The trap is that early progression feels fast and generous, building designer confidence, while mid-to-late progression becomes an asymptotic wall where each step takes dramatically longer than the last. The false signal is the satisfying early pace. The harm is a difficulty cliff where advancement stops feeling rewarding and starts feeling punitive, driving churn at the retention-critical mid-game and producing public complaints about grind.

### One-Time Sinks That Drain Once Then Leave the Flow Unbalanced

A major sink — an expensive endgame upgrade, a one-time cosmetic — drains a large sum once, after which the recurring source flow has no offsetting recurring sink and the currency begins to inflate. The trap is that the one-time sink created the illusion of balance in the period being measured, then ceased to function. The false signal is that "the sink worked" in the short window. The harm is post-purchase inflation that emerges only after players have passed the one-time drain, by which point the imbalance is entrenched.

### Ignoring the Optimizer Profile

The economy is balanced for the average player, but the optimizer finds the highest-yield activity, repeats it efficiently, and accumulates currency an order of magnitude faster than intended, trivializing all scarcity within days. The trap is that the optimizer is a minority, so average-player metrics look healthy while a visible minority demonstrates that the economy has no real scarcity. The false signal is that "most players are fine." The harm is that the optimizer's behavior becomes public knowledge, the dominant strategy spreads, and the average player base converges on the inflated economy, at which point the balance is broken for everyone.

### Adding Sinks Punitively After Inflation Appears

When inflation is detected post-launch, the team adds aggressive new sinks — raised repair costs, new taxes, devalued rewards — to drain the surplus. The trap is that these reactive sinks punish all players, including those who did not hoard, and are perceived as the game "taking away" what players earned, generating backlash. The false signal is that the sinks "address the inflation." The harm is player resentment and churn from players who feel penalized for the design's earlier imbalance, and the loss of trust that makes future economy changes harder to deploy.

### Premium Currency Injection Inflating the Soft Economy

In a monetized game, premium currency is convertible into soft currency or soft-currency-equivalent items, and the conversion rate was set without modeling its effect on the soft sink balance. The trap is that high-spending players inject soft currency far faster than play would generate it, inflating the soft economy in a way that play-balance cannot address and that disproportionately affects the spending cohort. The false signal is that the conversion "monetizes well." The harm is soft-currency inflation among spenders that distorts the economy, creates pay-to-progress outcomes, and undermines the soft economy's role as the free-to-play progression path.

## Self-Check

- Have I built and maintained a source-and-sink ledger for every currency, and can I state the net flow per hour of intended play with the resulting surplus or deficit intentional and bounded?
- Is the steady-state surplus small enough that play feels rewarding without trivializing future purchases, so that after twenty hours the average player can afford intended items but still faces meaningful economic choices?
- Are the major sinks recurring and activity-proportional rather than one-time drains that leave the steady-state flow unbalanced after they are consumed?
- Have I modeled the economy under at least three profiles — optimizer, average, and slow/lapsed — over a multi-month horizon, confirming none reaches unbounded accumulation, permanent starvation, or an insurmountable cliff?
- Is there a built-in reset or prestige mechanism that can drain accumulated value post-launch without confiscating player investment, sized to the expected accumulation rate?
- Have I modeled the soft economy both with and without premium-currency injections, confirming that realistic spending does not inflate the soft economy beyond its sink balance?
- If inflation or deflation emerges post-launch, is the response designed to avoid punishing players who did not cause the imbalance, rather than reactive punitive sinks?
