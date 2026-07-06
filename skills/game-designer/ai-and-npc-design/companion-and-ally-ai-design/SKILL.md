---
name: companion-and-ally-ai-design.md
description: Use when the agent is designing AI companions and allies, deciding how allied NPCs assist the player, tuning companion competence and autonomy, or evaluating whether a companion reads as a helpful partner or as a liability that frustrates the player and undermines the core loop.
---

# Companion and Ally AI Design

A companion is the only NPC whose behavior the player cannot escape, and whose competence reflects directly on the player's own experience. The judgment problem is that companion AI must be helpful enough to feel like an ally but not so autonomous that it plays the game for the player, and the line between them is narrow and genre-dependent. Agents tend to miss this because the failure modes are opposite and the tuning target is a balance point, not an extreme: a companion that is too passive feels useless, a companion that is too active steals the player's agency, and both produce resentment. The harm is a companion the player wishes they could dismiss, that turns encounters into babysitting, or that makes the player feel redundant in their own game. This skill covers how to calibrate companion competence, design the companion's role relative to the player, and avoid the liability trap. The agent has latitude in the companion's character and abilities, but the obligation to make the companion a net positive to the player's experience is not optional.

## Core Rules

### Define the Companion's Role Relative to the Player's Core Loop

A companion must serve a defined role that complements the player's core loop — tanking, healing, providing utility, enabling combos, offering tactical information — rather than duplicating the player's function or operating on an unrelated axis. The decision rule: name the companion's role and confirm it fills a gap the player cannot fill alone, without overlapping the player's primary expression. A companion that competes with the player for the same role feels redundant; a companion that fills a complementary role feels essential.

### Calibrate Competence to Support, Not Replace

The companion's competence must be tuned so it handles its role adequately — keeping the player alive, completing its tasks, not requiring constant rescue — without executing the player's core fantasy for them. The decision rule: set the companion's competence at the level where the player still feels like the protagonist and the companion feels like support, and playtest specifically for whether the player feels upstaged or abandoned. The target is a companion the player is glad to have, not one they rely on completely or resent carrying.

### Make Companion Behavior Predictable Enough to Plan Around

The player must be able to anticipate the companion's behavior well enough to incorporate it into their tactics, because a companion whose actions are unpredictable cannot be coordinated with. The decision rule: ensure the companion's decision logic produces behavior the player can model and predict — it heals when health is low, it engages the nearest threat, it holds position when commanded — even if the underlying logic is complex. Unpredictable companions cannot be tactically integrated, and the player treats them as chaos rather than allies.

### Provide Player Direction Without Micromanagement

The player should be able to direct the companion at a high level — target priority, positioning, role emphasis — without needing to micromanage its every action, because micromanagement converts the companion into a second character the player must operate. The decision rule: offer a small set of high-level commands that shift the companion's behavior, and let the AI handle execution. The companion that requires constant command is a burden; the companion that cannot be directed is unresponsive; the balance is high-level control with autonomous execution.

### Ensure the Companion Does Not Create Failure States for the Player

A companion that can die, fail, or trigger unwanted aggro creates failure states the player did not choose and cannot fully control, converting the companion into a liability. The decision rule: define the companion's failure behavior (does it revive, does it retreat, does it draw enemies) and ensure its failure does not punish the player disproportionately or block progress. The worst companions are those whose death ends the encounter or whose misbehavior triggers consequences the player suffers, because the player is punished for AI they cannot fully control.

### Validate the Companion Across the Full Encounter Spectrum

A companion tuned for typical encounters may fail at the extremes — trivial encounters where it steals all the kills, or hard encounters where it dies instantly — and the extremes are where companion frustration concentrates. The decision rule: playtest the companion across trivial, typical, and extreme encounters, and tune so it is unobtrusive when not needed and reliable when essential. A companion that is fine in the average and broken at the extremes produces frustration precisely in the hardest, most memorable encounters.

## Common Traps

### The Useless Companion That the Player Carries

The companion is tuned too passively — it hangs back, it does not engage, it fails its role — and the player experiences it as dead weight that must be protected without contributing. The trap is that passive companions avoid the upstaging problem. The false signal is that the companion does not steal the player's kills. The harm is that the player resents the companion as a liability, the encounters feel harder because the companion is absent in practice, and the character that was meant to be an ally is experienced as a burden the player wishes they could dismiss, because the team avoided one failure mode by falling into the opposite one.

### The Overcompetent Companion That Plays the Game

The companion is tuned so aggressively — high damage, autonomous targeting, self-sufficient survival — that it clears encounters before the player engages, and the player feels redundant in their own game. The trap is that an effective companion feels like quality AI. The false signal is that encounters are easy and the companion is impressive. The harm is that the player's agency and core fantasy are stolen by the AI, the player disengages because their participation is unnecessary, and the game whose loop was built around the player's action becomes a spectator experience, because the companion crossed from support into replacement and no one noticed the player was no longer the protagonist.

### The Unpredictable Companion That Cannot Be Coordinated

The companion's decision logic produces behavior the player cannot model — it sometimes engages, sometimes flees, sometimes uses abilities at inscrutable times — and the player cannot incorporate it into tactics. The trap is that varied behavior feels lifelike. The false signal is that the companion acts autonomously. The harm is that the player treats the companion as random noise rather than an ally, tactics that should work fail because the companion behaves unexpectedly, and the companion that was meant to enable coordinated play instead prevents it, because the player cannot predict or rely on behavior they cannot model.

### The Micromanagement Burden

The companion requires constant direction — target selection, ability use, positioning — to function, and the player experiences it as a second character they must operate in real time. The trap is that direct control feels responsive. The false signal is that the player can make the companion do exactly what they want. The harm is that the cognitive load of operating two characters exceeds the player's bandwidth, the core loop's feel degrades because attention is split, and the companion that was meant to reduce the player's burden instead doubles it, because the AI was not autonomous enough to execute high-level direction.

### The Companion Whose Failure Punishes the Player

The companion can die or fail in ways that end the encounter, fail the mission, or draw catastrophic aggro, and the player is punished for AI behavior they could not fully prevent. The trap is that companion stakes feel dramatic. The false signal is that protecting the companion adds tension. The harm is that the player experiences the companion as a hostage whose mistakes the player pays for, the tension converts to frustration, and the player either avoids the companion entirely or plays defensively to protect it rather than engaging the core loop, because the companion's failure was designed as the player's failure.

## Self-Check

- Have I defined the companion's role as complementary to the player's core loop, filling a gap without duplicating the player's expression?
- Is the companion's competence calibrated to support the player without upstaging them or playing the game for them?
- Can the player model and predict the companion's behavior well enough to incorporate it into tactics?
- Does the companion accept high-level direction without requiring micromanagement of every action?
- Has the companion's failure behavior been designed so its mistakes do not disproportionately punish the player or block progress?
- Did I playtest the companion across trivial, typical, and extreme encounters, not just the average?
- Did I confirm the companion is a net positive the player is glad to have, rather than a liability they wish they could dismiss?
