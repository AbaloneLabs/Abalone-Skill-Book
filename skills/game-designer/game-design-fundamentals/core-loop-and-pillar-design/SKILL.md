---
name: core-loop-and-pillar-design.md
description: Use when the agent is defining a game's core loop, establishing design pillars, deciding which mechanics are load-bearing versus peripheral, aligning systems with the central fantasy, cutting features that dilute focus, or evaluating whether a prototype's moment-to-moment action supports the game's stated identity.
---

# Core Loop and Pillar Design

The core loop is the smallest cycle of action, feedback, and reward that a player repeats throughout a game, and the design pillars are the two-to-five declarative commitments that every system and level is supposed to serve. Together they are the spine of a game: when coherent, every feature reinforces the same experience; when incoherent, the game accumulates systems that fight each other and the player cannot say why it feels scattered. The judgment problem is that core loops and pillars are deceptively easy to write and hard to honor. An agent can produce a plausible-sounding loop in minutes, but that loop is worthless if it is not load-bearing — if the real engagement comes from a different cycle the team never named, or if the pillars are aspirational language the mechanics do not enforce. Agents tend to miss this because they treat loops and pillars as documents to deliver rather than constraints to test against, and because feature creep is rewarded short-term by the feeling of "more game." The harm is a game that is broad but shallow, where no single experience is polished because resources spread across competing loops, and where playtesters describe the game in terms the team never intended. This skill covers how to define a loop that is genuinely central, write pillars that function as decision filters, and use them to cut features. The agent has latitude in what the loop and pillars are, but the obligation to make them binding is not optional.

## Core Rules

### Define the Core Loop as the Smallest Satisfying Cycle, Not a Feature List

The core loop is not a list of everything the player can do; it is the minimal cycle that, repeated, produces the game's primary satisfaction. A loop has three to five verbs, a clear input-to-feedback-to-reward chain, and a reason the player wants to restart it immediately after it completes. The decision criterion: if you removed everything outside this cycle, would the game still be fun for thirty minutes? If yes, you have found the loop; if no, you have listed features, not identified the engine of engagement. When a team cannot articulate the loop in one sentence without conjunction-stacking ("you explore and fight and craft and build and trade"), the game has no center and will be polished nowhere.

### Make Pillars Decision Filters, Not Adjectives

A pillar is useful only if it can reject a feature. "Fun," "immersive," and "epic" are not pillars because no feature fails them — they are adjectives that approve everything and constrain nothing. A functional pillar is a declarative constraint that, when applied to a proposed feature, yields a clear keep, cut, or modify. The decision rule: for each pillar, name one feature it would forbid. If you cannot, the pillar is decoration. Strong pillars are often phrased as tensions the team has chosen to resolve in a specific direction ("every encounter must be winnable without violence" rejects mandatory combat puzzles), and they gain their power precisely because they foreclose options.

### Distinguish Load-Bearing Mechanics from Peripheral Ones

In any game, a small number of mechanics carry the majority of the experience, and the rest exist to support, vary, or contextualize them. The judgment problem is that teams treat all shipped mechanics as equally important, which dilutes polish across everything. The decision rule: identify the one to three load-bearing mechanics — the ones whose removal would collapse the game's identity — and allocate disproportionate iteration budget to them. Peripheral mechanics should be cheap to implement, robust, and optional; they should never consume the polish budget that the core requires. When a peripheral mechanic demands as much tuning as a core one, it is either secretly core (promote it) or scope creep (cut it).

### Ensure the Loop Produces Compounding or Escalating Payoff

A core loop that returns the same reward every cycle produces satiation: the player tires of it after the novelty fades. A durable loop either compounds (each cycle's output feeds the next, producing growth — upgrades that unlock new strategies) or escalates (the challenge or stakes rise, producing tension — tougher enemies, higher bets). The decision rule: trace the output of one loop cycle and confirm it changes the input of the next. If the loop is circular with no accumulation — same fight, same reward, same state — it will not sustain a full game and must be wrapped in a progression system that supplies the missing compounding. Loops that feel repetitive in testing usually lack this payoff trajectory, not better moment-to-moment action.

### Use Pillars to Resolve Cross-Feature Conflict Before It Becomes Code

When two features pull in opposite directions — a stealth pillar versus a spectacle combat system, a "read everything" pillar versus a fast-travel system — the conflict is cheaper to resolve at the pillar level than at the implementation level. The decision rule: when a feature conflict arises, return to the pillars and ask which feature the pillar prioritizes; do not compromise by building both poorly. Compromise features, built to satisfy competing pillars, are the most expensive per unit of fun because they serve no one fully. The agent's job is to surface these conflicts early, name the tradeoff explicitly, and let the pillar decide rather than allowing silent drift toward a muddy middle.

### Validate the Loop in a Vertical Slice Before Scaling Content

A core loop that feels good in a prototype may not survive the addition of progression, economy, narrative, and content volume, because those systems can starve the loop of its reward or bury it under friction. The decision rule: before greenlighting full content production, validate the loop inside a vertical slice that includes the real progression and economy, not just the moment-to-moment action. Teams that scale content on an unvalidated loop discover, at enormous cost, that the loop breaks once wrapped in the full game — the economy inflates the reward, the progression trivializes the challenge, or the narrative interrupts the rhythm. The slice is the proof that the loop is load-bearing under load.

## Common Traps

### The Kitchen-Sink Loop That Names Everything as Core

The team, unwilling to prioritize, declares that exploration, combat, crafting, building, trading, and social systems are all "core," producing a loop document that is actually a feature inventory. The trap is that calling everything core feels inclusive and avoids hard prioritization conversations. The false signal is that the document is comprehensive and the team agrees on it. The harm is that no mechanic receives the concentrated polish a true core requires, the budget is fragmented, and the shipped game is competent at many things and exceptional at none, which is rarely enough to find an audience.

### Pillars Written as Marketing Copy

The pillars are phrased in the language of the game's eventual box quote — "a living, breathing world" — which is evocative but cannot reject a single feature. The trap is that aspirational language reads beautifully in a pitch deck and earns approval from stakeholders who do not test it against decisions. The false signal is that the pillars sound compelling and everyone nods. The harm is that during development, every feature can claim alignment with some generous reading of the pillars, so nothing is ever cut, scope balloons, and the pillars never do the filtering work they were created for.

### Keeping a Loved Mechanic That Serves No Pillar

A mechanic is prototyped, the team enjoys it, and it survives into shipping even though it serves none of the pillars and competes for the player's attention with the core loop. The trap is that fun in isolation feels like sufficient justification, and cutting something the team loves feels like cruelty. The false signal is that internal playtests produce smiles. The harm is that the orphan mechanic consumes polish budget, confuses players about what the game is about, and dilutes the core loop's dominance — the player never settles into the central rhythm because a side system keeps interrupting.

### The Loop That Does Not Compound

The loop is satisfying for the first hour but produces identical rewards every cycle, so by hour three the player has seen everything the loop offers and disengages. The trap is that the moment-to-moment feel is excellent, so the loop passes short playtests. The false signal is strong first-session telemetry and positive early impressions. The harm is catastrophic long-term retention: the team blames the lack of "content" and produces more of the same loop, when the real defect is the absence of a progression layer that makes each cycle meaningfully different from the last.

### Adding Systems to Fix a Core Loop Problem

When the core loop feels thin, the team adds adjacent systems — a crafting menu, a base-building layer, a companion system — hoping that breadth will substitute for depth. The trap is that new systems create a burst of engagement that masks the underlying thinness. The false signal is that each new system produces a spike in playtest interest. The harm is that the additions dilute the core loop's centrality, multiply the tuning surface, and defer the real work of making the core loop itself richer, so the shipped game is a collection of shallow systems orbiting a loop that was never finished.

## Self-Check

- Can I state the core loop as the smallest satisfying cycle of three to five verbs, and would removing everything outside that cycle still leave a fun thirty-minute game?
- For each design pillar, can I name at least one feature it forbids, confirming the pillar functions as a decision filter rather than an adjective?
- Have I identified the one to three load-bearing mechanics and allocated disproportionate iteration budget to them, rather than spreading polish evenly across all shipped mechanics?
- Does the loop's output feed back into its next cycle through compounding or escalation, so that repeated cycles are meaningfully different rather than identical?
- When feature conflicts arose, did I resolve them at the pillar level rather than building compromised versions of both, and did I name the tradeoff explicitly?
- Was the core loop validated inside a vertical slice that includes the real progression and economy, before scaling into full content production?
- Have I cut or refused loved mechanics that serve no pillar, rather than retaining them because they were fun in isolation?
