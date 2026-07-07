---
name: physics-driven-emergent-gameplay-and-design-intent.md
description: Use when the agent is designing physics-driven or emergent gameplay systems, deciding what player expression a physics simulation should enable, managing the relationship between simulation depth and authored intent, or reviewing whether physics-driven systems produce intended solutions versus exploitable chaos; trigger contexts include physics-driven gameplay, emergent gameplay, immersive sim, simulation depth, systemic design, physics puzzle, physics interaction, emergent solution, systemic interaction, authored versus emergent, physics exploitation, creative solutions, simulation game design; important risks include emergent chaos overriding intent, exploitable physics, solutions the team did not anticipate, and depth that masks shallowness.
---

# Physics-Driven Emergent Gameplay And Design Intent

Physics-driven gameplay promises that players will invent solutions the designers never imagined, and it delivers on that promise in ways that are both the genre's glory and its greatest design risk, because the same emergence that produces creative play produces exploitable chaos, unintended solutions, and outcomes that bypass the authored intent the game was built around. The agent is usually asked to design or review a physics-driven system while the intended player expression, the acceptable solution space, and the boundary between emergence and exploitation are under-specified. The judgment problem is that emergent systems are powerful precisely because they are not fully controllable, and the designer must steer them through constraints, affordances, and systemic coherence rather than through scripted authorship.

The agent tends to miss this because emergence is celebrated as inherently good, and because the failure modes (trivializing encounters, sequence-breaking, economy-breaking exploits) look like features until they undermine the game. The harm is systems where the physics enables solutions that bypass all authored challenge, where emergent interactions produce unfair or absurd outcomes, or where the promise of depth masks a shallow core loop. Use this skill to slow the design down enough to expose what emergence should and should not enable, then make the recommendation appropriately conservative when intent, fairness, and exploit-resistance are at stake.

## Core Rules

### Define The Player Expression The Physics Should Enable
Before building any physics-driven system, state the player expression it should enable: creative problem-solving, systemic experimentation, physical mastery, sandbox play, or emergent storytelling. The agent should derive the simulation depth from this expression (an immersive sim needs deep interactivity; a linear action game needs shallow, readable physics) and build toward enabling that expression. Physics without a defined expression target produces depth without purpose.

### Design Affordances That Steer Toward Intended Solutions
Emergence is steered through affordances — what the physics makes easy, obvious, and rewarding to do. The agent should design objects and systems whose affordances point toward intended solution families (flammable objects near fire sources, breakable barriers near heavy objects, conductive materials near electricity), so players discover intended emergence naturally. Affordances that point nowhere produce confusion; affordances that point everywhere produce noise.

### Decide The Acceptable Solution Space Explicitly
For each physics-driven challenge, decide what solutions are acceptable, what are tolerated, and what must be prevented. The agent should map the intended solution space (the creative approaches the team wants to reward), the tolerated space (unintended but harmless solutions), and the forbidden space (solutions that trivialize, break economy, or sequence-break). A challenge whose solution space is undefined will be solved in ways the team did not anticipate and may not want.

### Build Systemic Coherence So Interactions Are Predictable
Emergent systems feel fair when their interactions are coherent: fire burns flammables, water conducts electricity, heavy objects break fragile ones, consistently and everywhere. The agent should ensure systemic properties (flammable, conductive, fragile, buoyant) are applied consistently across all objects, so players can reason about and predict interactions. Incoherent systems, where fire burns here but not there, feel arbitrary and prevent the learning that emergent play requires.

### Anticipate And Bound Exploitable Interactions
Physics-driven systems will be exploited, and the team must anticipate and bound the exploits that matter. The agent should identify physics interactions that could trivialize challenge (infinite loops, damage stacking, physics-launching past gates), decide which exploits are acceptable emergent play and which break the game, and bound the breaking ones through constraints, immune states, or design. Unbounded physics exploitation can trivialize an entire game's challenge.

### Separate Emergent Depth From Authored Shallowness
A physics-driven game can use emergence to mask a shallow authored core, and the team must decide honestly whether the depth is real. The agent should assess whether the emergent interactions are genuinely deep (many meaningful combinations) or shallow (a few repeated patterns dressed as depth), and ensure the core loop rewards sustained engagement. Emergence that masks shallowness produces a game that feels deep for an hour and empty thereafter.

### Test With Creative Players Before Shipping
Emergent systems must be tested with players who will push, break, and exploit them, not just players who play as intended. The agent should run playtests with creative and adversarial players, catalog the solutions and exploits they discover, and decide which to embrace, steer, or bound. A system tested only by intended-path players ships with its exploit space entirely unexamined.

## Common Traps

### Emergence Celebrated As Inherently Good
The team treats emergence as a virtue and does not steer it, and the physics produces chaos that overrides intent. The trap is that emergence is the genre's appeal. The false signal is that players are "being creative." The harm is trivialized challenge, broken economies, and a game whose authored design is bypassed by unbounded emergence.

### Affordances That Point Everywhere Or Nowhere
Objects have no clear affordances, so players cannot discover intended solutions, or everything interacts with everything, producing noise. The trap is that full interactivity feels rich. The false signal is that the simulation is deep. The harm is players cannot reason about the system, intended solutions are never found, and the emergence feels random rather than learnable.

### Unintended Solutions Trivializing Authored Challenge
Players discover physics solutions that bypass the intended challenge entirely — launching over gates, looping damage, stacking to skip sections. The trap is that the solutions are creative. The false signal is that emergence is working. The harm is the authored game is trivialized, the difficulty curve collapses, and the challenge the team built is skipped by the physics they enabled.

### Incoherent Systemic Properties
Systemic properties apply inconsistently (fire burns this wood but not that wood), and players cannot predict interactions. The trap is that special cases feel handcrafted. The false signal is that the world is detailed. The harm is emergent play becomes guesswork, because the coherence that lets players learn and reason is absent.

### Exploits That Break Economy Or Progression
Physics interactions enable infinite-resource loops, duplication, or progression skips, and the economy or progression breaks. The trap is that the exploit is clever. The false signal is that it rewards creativity. The harm is the game's balance and economy collapse for players who discover the exploit, and the intended progression is undermined.

### Depth Masking Shallowness
The emergent surface is broad but the core is shallow, and players exhaust the meaningful combinations quickly. The trap is that the system feels deep initially. The false signal is that there are many interactions. The harm is the game feels deep for a session and empty thereafter, because the emergence was breadth without depth.

### Untested Exploit Space
The system is tested only by intended-path players, and the exploit space ships unexamined. The trap is that QA passed the intended solutions. The false signal is that the challenges are completable. The harm is day-one exploits, trivialized challenge, and a community that discovers the shallowness or breakability the team never looked for.

### Presenting Emergence As Neutral Depth
Emergent design decisions are often judgment calls, but the agent should not present emergence as if it were inherently good or neutral. State what is known (the expression target, the solution space, the exploit surface), what is inferred (player creativity), and what is a design judgment about acceptable exploitation, so the team decides with the tradeoffs visible.

## Self-Check

- [ ] Is the player expression the physics should enable defined and used to derive simulation depth?
- [ ] Do object and system affordances steer players toward intended solution families without pointing everywhere or nowhere?
- [ ] Is the acceptable solution space mapped — intended, tolerated, and forbidden — for each physics-driven challenge?
- [ ] Are systemic properties (flammable, conductive, fragile, buoyant) applied consistently so interactions are predictable and learnable?
- [ ] Are exploitable interactions anticipated and bounded, with a decision on which exploits are acceptable emergent play and which break the game?
- [ ] Has the team honestly assessed whether the emergent depth is real or masks a shallow core loop?
- [ ] Has the system been tested with creative and adversarial players, with discovered solutions and exploits cataloged and decided upon?
- [ ] Does the output distinguish emergence that serves design intent from emergence that overrides it?
- [ ] Are the expression target, affordances, solution space, and exploit boundaries specific enough for the team to implement without re-deciding?
- [ ] Is uncertainty about player creativity and exploit discovery named, with the tradeoffs that would change the recommendation made explicit?
