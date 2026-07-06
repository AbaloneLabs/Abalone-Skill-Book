---
name: interlocking-economy-and-progression-systems.md
description: Use when the agent is designing in-game economies, currency sinks and sources, progression curves, unlock gating, resource scarcity, inflation control, prestige systems, or balancing the feedback between player advancement and spending across RPG, survival, strategy, and live-service games.
---

# Interlocking Economy and Progression Systems

Economy and progression systems are the load-bearing skeleton of almost every game with long-term engagement, yet they are frequently designed as a loose collection of numbers rather than as an interlocking mechanism. The judgment problem is that an economy is a closed loop: every source creates a sink obligation, every reward changes the value of every other reward, and every progression gate shifts the difficulty and pacing of everything downstream. Designers tend to miss this because individual values look reasonable in isolation — a drop rate of 5%, a cost of 100 coins, a level cap of 50 — but the interactions between them produce inflation, dead currencies, trivialized difficulty, or progression walls that only become visible weeks into a live game. The harm is severe: a broken economy erodes player trust, trivializes the intended challenge, drives churn at the exact moment retention matters most, and in monetized games creates accusations of manipulative tuning even when the intent was benign. The freedom here is wide — there is no single correct curve — but the obligation is to model the system as a whole, stress-test its edge cases, and refuse to ship numbers that have only been validated by feel.

## Core Rules

### Model the Economy as Sources and Sinks, Not as a Price List

Every economy is defined by where value enters the system (sources: drops, quest rewards, daily login bonuses, crafting yields) and where it leaves (sinks: shop purchases, repairs, upgrades, fast travel, respec costs, consumable expenditure). A designer who lists prices without mapping the corresponding sources is designing half a system. The discipline is to build a source-and-sink ledger for every currency, then verify that under steady-state play the sink rate keeps pace with the source rate so that the currency retains meaning. When the ledger shows persistent surplus, the currency inflates and purchases become trivial; when it shows persistent deficit, the currency becomes a wall and players grind. The decision criterion: if a player plays optimally for one hour, how much of this currency do they gain, how much do they need to spend to keep up with intended progression, and does the remainder feel rewarding rather than hoarded or starving?

### Tie Progression to Player Capability Growth, Not to Raw Time

Progression should map to a change in what the player can do — new abilities, expanded options, increased mastery — not merely to a number going up. A level system that increases health and damage by 10% per level without changing available verbs is a treadmill: it feels like progress but changes nothing about how the game is played. The strong design ties each progression milestone to a meaningful capability unlock, so that advancement reshapes the player's relationship with the game's challenges. When you must use numerical scaling, ensure the encounters and economy scale in a way that preserves the relative challenge, otherwise higher levels become easier by accident. The decision criterion: after this progression step, can the player approach a problem they could not approach before, or are they merely doing the same thing with bigger numbers?

### Stress-Test the System at Multiple Player Profiles

An economy tuned for the average player will be broken for the extremes. The obsessive min-maxer will find every efficiency, hoard every resource, and trivialize the intended scarcity within days. The casual player who logs in twice a week will fall behind the intended curve, hit walls, and churn. The completionist will exhaust all content and demand more. The designer's obligation is to simulate or reason through at least three profiles — the efficient grinder, the average player, and the slow/lapsed player — and confirm that none of them hits a degenerate state: infinite accumulation, permanent starvation, or a cliff where the gap between reward and requirement becomes insurmountable. When a profile breaks, the fix is usually a non-linear sink or a catch-up mechanic, not a global rebalance that punishes the average player.

### Gate Content to Create Pacing, Not to Create Friction for Its Own Sake

Gating — level locks, currency requirements, quest prerequisites — exists to pace the player's experience and ensure they have the capability and context to enjoy what comes next. The trap is using gating as a substitute for content, stretching thin material by inserting arbitrary waits. Good gating asks "is the player ready for this?" and blocks until the answer is yes; bad gating asks "have they spent enough time?" and blocks regardless of readiness. The distinction matters because readiness-gating feels like the game is guiding the player, while time-gating feels like the game is withholding. When designing a gate, state explicitly what capability or knowledge the player should have by the time they pass it, and verify that the activities required to pass actually build that capability rather than merely consuming time.

### Build Currencies With Distinct, Non-Overlapping Jobs

A common failure is having multiple currencies that buy the same category of thing, which forces players to optimize across currencies and turns shopping into an accounting exercise. Strong economy design gives each currency a clear domain: soft currency for routine consumables, hard currency for premium items, a reputation currency for faction unlocks, a crafting currency for gear modification. When currencies overlap in purpose, the cheaper or more abundant one always wins and the others become vestigial. The decision criterion: can a player clearly answer "what do I spend this on, and why would I choose it over the alternatives?" If the answer is fuzzy, merge the currencies or sharpen their distinct roles. Dead currencies confuse players and signal sloppy design.

### Plan for Inflation and Deflation From Day One

Any live economy drifts. Sources that seemed balanced at launch accumulate as the player base matures; sinks that seemed adequate become trivial once players optimize. The designer must build in adjustable parameters — tunable drop rates, sink multipliers, periodic currency sinks tied to events, and prestige or reset mechanics that convert accumulated wealth into a fresh progression track. Prestige systems (New Game+, season resets, ascension mechanics) are not just replay features; they are economic control valves that drain accumulated value and restart the scarcity curve. The decision criterion: what mechanism exists to reset or rebalance the economy if inflation or deflation emerges post-launch, and can it be deployed without punishing players who have already invested?

## Common Traps

### Tuning Numbers in Isolation Without Modeling the Loop

A designer sets a drop rate, a shop price, and an upgrade cost by feel, each seeming reasonable, but never runs the combined math of "how many kills to afford this upgrade at this drop rate." The trap is that locally reasonable values produce globally broken economies — the upgrade that looked affordable takes forty hours of grinding because the drop rate and price were never multiplied together. The false signal is that each number "feels right" in a spreadsheet cell. The harm is a progression wall discovered only in late playtesting or, worse, after launch when players publicly document the grind.

### Using Exponential Costs With Linear Income

Progression systems frequently use exponential cost curves (each level costs 1.5x the last) while income grows linearly or not at all. The trap is that early progression feels fast and generous, lulling the designer into confidence, while mid-to-late progression becomes an asymptotic wall where each step takes dramatically longer. The false signal is the satisfying early-game pace. The harm is that players hit a difficulty cliff where advancement stops feeling rewarding and starts feeling punitive, driving churn at exactly the retention-critical mid-game.

### Creating a Currency With No Sink

A new currency is introduced for a feature — faction tokens, event badges, crafting shards — and the designer specifies how players earn it but not what they spend it on, or the spend options are so limited that players accumulate a surplus within days. The trap is that an unsunk currency inflates to worthlessness, becoming a number players ignore. The false signal is that players are "engaged" earning the currency. The harm is a dead system that clutters the UI, confuses new players, and requires a painful redesign or removal later.

### Letting One Optimal Strategy Dominate the Economy

The designer intends multiple viable playstyles, but one activity yields dramatically more reward per hour than alternatives, and the player base converges on it. The trap is that this single optimal activity becomes the de facto game, and all other content is abandoned. The false signal is that the dominant activity is popular, which looks like success. The harm is content attrition, player boredom, and a brittle meta where any nerf to the dominant strategy causes widespread backlash because everyone has invested in it.

### Ignoring the Interaction Between Real-Money and Soft Economies

In monetized games, the premium currency intersects with the soft economy through conversion, bundles, and time-savers. The trap is designing the soft economy in isolation and then bolting on monetization, which inevitably creates either pay-to-win outcomes (premium skips a wall that shouldn't exist) or accusations thereof even when the wall was unintentional. The false signal is that the two economies were "designed separately for clarity." The harm is reputational damage, player resentment, and regulatory scrutiny in jurisdictions that police dark patterns.

### Forgetting the Returnee and Late-Arrival Player

The economy is tuned for launch-day players progressing in lockstep, but six months later a new or returning player faces an economy where veterans have hoarded everything, prices (in player-driven markets) have inflated, and the intended early-game scarcity no longer functions because catch-up mechanics are absent. The trap is designing only for the synchronous launch cohort. The false signal is that the early game "worked at launch." The harm is that new player acquisition becomes retention-impossible, starving the game's long-term population.

## Self-Check

- Did I build a source-and-sink ledger for every currency, and can I state the steady-state surplus or deficit for an average hour of play?
- Did I verify that each progression milestone grants a meaningful capability change, not just a numerical increase, so advancement reshapes how the player engages with the game?
- Did I stress-test the economy against at least three player profiles (efficient grinder, average player, slow/lapsed player) and confirm none hits a degenerate accumulation, starvation, or insurmountable cliff state?
- Does every gate correspond to a stated readiness condition (capability or knowledge), rather than existing only to consume time?
- Does each currency have a distinct, non-overlapping job that a player can clearly articulate, with no vestigial or dead currencies?
- Is there a documented mechanism (tunable sinks, prestige reset, season structure) to counter post-launch inflation or deflation without punishing existing investment?
- If the game is monetized, have I traced how premium currency intersects with the soft economy and confirmed no unintentional pay-to-skip walls exist?
