---
name: numeric-tuning-and-parameter-balancing.md
description: Use when the agent is tuning damage, health, cooldowns, economy values, drop rates, XP curves, stat growth, weapon stats, or reviewing whether numeric parameters produce the intended player experience, pacing, and choice pressure across a game's systems.
---

# Numeric Tuning and Parameter Balancing

Numeric tuning is the art of choosing the constants that govern a game's feel, pacing, and strategic pressure — how much damage a weapon does, how long a cooldown lasts, how much gold a quest yields, how fast XP accrues. Designers miss the judgment here because the numbers feel arbitrary and individually small, so the temptation is to set them by feel, ship them, and patch later. The judgment problem is that numbers are not arbitrary: they are a coupled system where changing one value ripples through damage-per-second curves, time-to-kill, economy inflation, build viability, and the entire texture of player choice. A weapon that is 8% too strong does not feel 8% better; it becomes the only weapon anyone uses, and the rest of the arsenal becomes dead content. The harm of poor tuning is that the game's intended experience — the fantasy, the pacing, the meaningful choices — is silently replaced by a degenerate one, and the team cannot tell whether the design is wrong or merely mis-tuned, because they never separated the two. The agent has freedom in the specific values it chooses, but it must treat tuning as an empirical, instrumented discipline rather than guesswork, and it must understand that the right number is the one that produces the intended experience, not the one that looks reasonable on paper.

## Core Rules

### Define the Target Experience Before Choosing Any Number

Before tuning a value, state explicitly what player experience the number is meant to produce: time-to-kill, encounters-per-rest, sessions-to-max-level, gold-earned-per-hour relative to item cost. A number without a target experience is a guess, and guesses compound. The rule is to write down the desired feel in observable terms — "combat should last 6-10 seconds against a standard enemy," "the player should afford a weapon upgrade roughly every two hours" — and then tune toward that measurement. Decision criterion: if you cannot state what experience a parameter is supposed to create, you are not tuning, you are filling in a spreadsheet. Anchor every value to a felt outcome.

### Model the Coupled System, Not Individual Values in Isolation

Damage, health, armor, cooldowns, fire rate, and movement speed are not independent; they form a damage-per-second and time-to-kill system where changing one shifts the viability of every other. A 10% damage buff is also a 10% reduction in time-to-kill, a change in how many enemies can be fought before reload, and a shift in which builds clear content fastest. The rule is to maintain a model — a spreadsheet, a simulation, a DPS calculator — that shows the relationships, and to evaluate any change against the whole system, not the single cell. Decision criterion: when you change a number, can you predict its effect on time-to-kill, economy velocity, and build ranking? If not, you are tuning blind.

### Tune for the Range of Player Behavior, Not the Average

Players do not play at the average; they play at the extremes of skill, optimization, and min-maxing. A value balanced for the median player is trivially broken by a min-maxer and punishing for a casual player. The rule is to identify the relevant player percentiles — typically the casual floor, the engaged median, and the optimizing ceiling — and ensure the parameters produce a coherent experience across that range, often through difficulty options or scaling rather than a single flat value. Decision criterion: have you tested the parameters against both the player who ignores upgrades and the player who optimizes every slot? If the game collapses at either end, the tuning serves only the middle.

### Use Power and Cost Curves, Not Flat Numbers

Flat numbers (every weapon +10 damage) produce flat, boring progression and inevitable dominance. Curves — exponential or polynomial cost curves against linear or logarithmic power gains — produce meaningful tradeoffs where the cheapest option is not always best and the most expensive is not always worth it. The rule is to design the shape of progression explicitly: how power scales with level, how cost scales with power, and where the diminishing returns set in. Decision criterion: does the cost-to-power curve create genuine decisions, or is there always one obviously best upgrade path? If the latter, the curve is misshapen.

### Instrument and Measure Before and After Every Change

Tuning without telemetry is mythology. Every balance change should be informed by data — win rates, pick rates, completion times, economy balances, drop-rate outcomes across thousands of sessions — and verified by the same data after the change. The rule is to ship instrumentation before you need it, so that when a balance question arises the answer is queryable rather than anecdotal. Decision criterion: when someone asks "is this weapon too strong," can you answer with pick rate, win rate, and time-to-kill data, or do you have to argue from feel? Anecdotal tuning produces anecdotal balance.

### Make Changes Surgical and Isolated, Never Bundled

When you change five values at once and the game improves, you do not know which change helped, and you cannot reproduce or revert intelligently. The rule is to change one coupled variable at a time, measure, and then change the next. This is slower but produces understanding rather than luck. Decision criterion: if a patch makes the game worse, can you identify which change caused it? If you changed a bundle, you cannot, and you will over-correct in the next patch, oscillating the game's balance.

### Distinguish Design Problems From Tuning Problems

Sometimes the numbers are fine and the system is broken; sometimes the system is fine and the numbers are wrong. Conflating the two leads to endless re-tuning of a fundamentally flawed mechanic, or to redesigning a mechanic that only needed a value pass. The rule is to diagnose first: if no value of the parameter produces a good experience, the problem is the design, not the tuning. Decision criterion: have you tried a wide range of values for the parameter, and does the experience remain bad across all of them? If so, stop tuning and redesign.

## Common Traps

### Tuning to the Designer's Own Skill and Playstyle

The designer tunes the values against their own play, and because they are expert and biased toward a certain style, the values are calibrated to their body and preferences. The trap is that the designer is the worst possible tuning target — too skilled, too informed, too invested in the intended playstyle. The false signal is that the game "feels right" in internal play; the harm is that real players, playing differently and less optimally, experience a game tuned for someone who is not them, and the team cannot understand the discrepancy.

### The Single Dominant Strategy Emerging From a Small Numeric Edge

A weapon is 8% stronger than its peers, which sounds minor, but because players optimize, that 8% compounds into a 95% pick rate and the entire arsenal collapses to one option. The trap is that small numeric edges produce categorical strategic outcomes because players are ruthless optimizers, not gentle averagers. The false signal is that the edge "looks small" on the spreadsheet; the harm is dead content, homogeneous play, and a community that concludes the game is poorly balanced when the actual error was a few percentage points.

### Economy Inflation From Untuned Reward Curves

Gold and XP rewards are set early and never re-examined as content is added, so that a player who engages with side content out-levels the main path, trivializing it, or accrues so much currency that cost-based decisions lose all meaning. The trap is that rewards feel generous and generous feels good, so the inflation goes unnoticed until the economy is broken. The false signal is positive player sentiment about "feeling rewarded"; the harm is that the game's tension and progression collapse, and the late game becomes a trivial spending exercise.

### Buff-Only Patches That Inflate Power Over Time

Faced with underperforming options, the team buffs them rather than nerfing the overperformers, because buffs are popular and nerfs generate backlash. Over many patches, the entire power curve inflates upward, old content becomes trivial, and the numbers lose meaning. The trap is that each individual buff feels safe and player-friendly. The false signal is positive community reception per patch; the harm is long-term balance decay and a game where power creep has made the original design intent unrecognizable.

### Trusting Spreadsheets Over Playtest Feel

A model says the values are balanced, so the team ships them, but the model omitted a real interaction — a synergy, a movement tech, a stacking effect — that makes the on-paper balance wrong in practice. The trap is that the model feels authoritative because it is quantitative. The false signal is internal consistency of the spreadsheet; the harm is shipping numbers that are correct in the model and broken in the game, because the model did not capture the game.

## Self-Check

- Did I define, in observable experiential terms, what each tuned parameter is meant to produce before choosing its value?
- Did I model the coupled system (DPS, time-to-kill, economy velocity, build ranking) rather than tuning values in isolation?
- Have I tested the parameters against the range of player behavior, including both casual and min-max extremes, not just the median?
- Are progression and economy governed by explicit cost-to-power curves rather than flat numbers, and do those curves create real decisions?
- Is telemetry in place to measure win rates, pick rates, completion times, and economy state before and after every change?
- Are balance changes made one coupled variable at a time, so cause and effect are traceable and reversible?
- Have I distinguished whether a persistent problem is a tuning issue or a design issue by testing a wide range of values before redesigning?
