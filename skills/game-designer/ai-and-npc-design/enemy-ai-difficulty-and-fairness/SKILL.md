---
name: enemy-ai-difficulty-and-fairness.md
description: Use when the agent is designing enemy AI for combat, tuning enemy difficulty and aggression, deciding how enemies telegraph attacks and respond to player behavior, or evaluating whether enemy AI provides a fair challenge or reads as cheap, unfair, or exploitable.
---

# Enemy AI Difficulty and Fairness

An enemy's difficulty is not its stat block; it is the player's experience of fighting it, and that experience is governed by fairness far more than by numbers. The judgment problem is that players accept hard enemies that are fair and reject easy enemies that are cheap, and the difference between fair and cheap is a design property — telegraphing, reaction windows, counterplay — that agents often sacrifice to hit a difficulty target. Agents tend to miss this because difficulty is easy to tune by numbers (raise health, raise damage) while fairness is hard to design (the enemy must be readable, beatable, and consistent), and because a numbers-tuned hard enemy passes a difficulty check while failing a fairness check. The harm is enemies that players call "spongy" or "cheap," combat that players avoid or exploit, and a difficulty curve that produces churn at the encounters that were meant to be the game's highlights. This skill covers how to design enemy behavior for fairness, tune difficulty without sacrificing readability, and avoid the cheap trap. The agent has latitude in enemy design, but the obligation to make difficulty fair is not optional.

## Core Rules

### Make Every Attack Telegraphed and Therefore Dodgeable

A fair enemy broadcasts every attack before it lands, through animation, audio, or visual cues, giving the player a reaction window to respond. The decision rule: for every enemy attack, define the telegraph (what cue, how early) and the reaction window (how long the player has), and confirm a skilled player can consistently avoid it on reaction. Un-telegraphed attacks are cheap by definition, because the player cannot respond to information they were not given, and cheapness reads as unfair regardless of the damage value.

### Ensure Every Enemy Has Discoverable Counterplay

A fair enemy can be defeated by a strategy the player can discover and execute, not only by stats that exceed it. The decision rule: for each enemy, define its counterplay (a weakness, a stun condition, a spacing exploit, a combo opening) and ensure the counterplay is discoverable through observation and executable with skill. Enemies with no counterplay are defeated only by out-statting them, which reduces combat to a numbers check and removes the skill expression that makes combat engaging.

### Tune Difficulty by Behavior Complexity Before Stats

The first lever for increasing difficulty should be behavioral — more attack types, better spacing, adaptive responses — and stats (health, damage) should be the last lever, because behavioral difficulty is fair (the player can learn and counter it) while stat difficulty is often cheap (the enemy is a sponge or one-shots). The decision rule: when raising difficulty, add behavioral complexity first and reserve stat increases for cases where behavior alone cannot reach the target. Spongy enemies are the canonical symptom of stat-first tuning, and players reject them as padding.

### Make Enemy Behavior Consistent With Its Readable Model

The player infers an enemy's behavior model from its appearance and telegraphs — a big enemy is slow and heavy, a small enemy is fast and fragile — and the behavior must be consistent with that model or it reads as a betrayal. The decision rule: define each enemy's readable model and ensure its behavior, speed, and telegraphs are consistent with it. An enemy that looks slow but attacks instantly, or looks fragile but tanks damage, breaks the player's inferred model and reads as cheap, because the player's reasonable expectations were violated.

### Provide Difficulty Options That Preserve Fairness at Every Tier

When offering difficulty tiers, each tier must remain fair at its level — the easy tier must be readable and counterplayable, the hard tier must be readable and counterplayable, just with tighter windows and more aggression. The decision rule: tune each difficulty tier by adjusting the fairness-preserving parameters (reaction windows, enemy aggression, error tolerance), not by adding cheapness (one-shots, sponginess, un-telegraphed attacks). A hard mode that achieves difficulty through cheapness is not a harder fair game; it is an unfair game, and players correctly reject it.

### Adversarially Playtest for Exploits and Cheese

Players will find the exploit — the safe spot, the loopable pattern, the stunlock — that trivializes the enemy, and once found, the exploit becomes the meta and the designed combat is bypassed. The decision rule: adversarially playtest each enemy to find exploits, and either fix them or design the enemy to punish them. An enemy with a known unpatched exploit ceases to function as designed, because the player base converges on the cheese and the carefully-tuned combat is never experienced.

## Common Traps

### Stat-First Tuning That Produces Spongy or One-Shot Enemies

The team raises difficulty by inflating enemy health and damage, producing enemies that take too long to kill (spongy) or kill the player instantly (one-shot), both of which read as cheap. The trap is that stat tuning is fast and measurable. The false signal is that the enemy is "harder" by the numbers. The harm is that spongy enemies pad playtime without adding engagement, one-shot enemies feel unfair regardless of telegraphing, and the difficulty that was achieved through stats is experienced as tedium or frustration rather than challenge, because the team reached the difficulty target by sacrificing the fairness that makes difficulty feel earned.

### The Un-Telegraphed Attack That Reads as Cheap

An enemy attack has no readable telegraph or a telegraph shorter than human reaction time, so the player is hit by attacks they could not have avoided, and reads the enemy as cheap. The trap is that fast attacks feel challenging. The false signal is that the attack increases difficulty. The harm is that the player cannot improve against the attack through skill (because it is unreactable), so the only response is to memorize or cheese, the combat ceases to be a skill expression and becomes a memorization tax, and the enemy that was meant to be a challenge is instead experienced as unfair, because the information needed to respond fairly was withheld.

### The Enemy With No Counterplay, Only a Stat Check

An enemy has no discoverable weakness or strategy, so it can only be defeated by having higher stats than it, and combat against it reduces to a numbers check. The trap is that a stat-check enemy is simple to design. The false signal is that the enemy functions as a gate. The harm is that the player cannot express skill against the enemy, the encounter is won or lost at the stat level rather than the play level, players who lack the stats hit a wall and grind, and the combat that was meant to be the game's skill expression is replaced by a progression gate, because no counterplay was designed for the player to discover and execute.

### Inconsistent Behavior That Breaks the Readable Model

An enemy behaves inconsistently with its appearance and telegraphs — a hulking enemy with instant attacks, a fragile-looking enemy that absorbs damage — and the player's inferred model is violated, reading the enemy as cheap or buggy. The trap is that each behavior was tuned in isolation. The false signal is that each behavior is balanced individually. The harm is that the player cannot build a reliable model of the enemy, every encounter feels like the rules changed, and the enemy that was meant to be readable is instead unpredictable in a way that reads as unfair, because the consistency that lets the player learn and counter was never enforced across the enemy's behaviors.

### The Unpatched Exploit That Becomes the Meta

An enemy has an exploitable pattern — a safe position, a loopable animation, a stunlock — that adversarial players discover and share, and the exploit becomes the dominant strategy, bypassing the designed combat entirely. The trap is that the exploit is invisible in normal play. The false signal is that typical testers engage the enemy as designed. The harm is that the player base converges on the cheese, the carefully-tuned combat is never experienced, the enemy ceases to function as a challenge and becomes a routine, and the game develops a reputation for being cheesable, because the exploits were not found and closed through adversarial playtesting.

### Difficulty Tiers That Add Cheapness Instead of Fairness

The hard mode achieves its difficulty by adding un-telegraphed attacks, one-shot damage, or sponginess, rather than by tightening reaction windows and increasing aggression within the fairness framework. The trap is that cheapness is an easy way to raise numbers. The false signal is that hard mode is measurably harder. The harm is that hard mode is unfair rather than harder, skilled players who wanted a fair challenge reject it, the difficulty option that was meant to serve the hardcore audience instead alienates them, and the game earns a reputation for having a hard mode that is cheap rather than a hard mode that is fair, because the team confused numerical difficulty with fair difficulty.

## Self-Check

- Does every enemy attack have a readable telegraph and a reaction window within which a skilled player can consistently respond?
- Does every enemy have discoverable, executable counterplay, so it can be defeated by strategy and skill, not only by stats?
- When raising difficulty, did I add behavioral complexity before inflating stats, reserving stat increases for cases where behavior cannot reach the target?
- Is each enemy's behavior consistent with the readable model its appearance and telegraphs imply?
- Do my difficulty tiers preserve fairness at every level by tightening windows and aggression, not by adding cheapness?
- Did I adversarially playtest each enemy for exploits and cheese, and fix or punish the exploits I found?
- Did I confirm the difficulty feels earned (fair and hard) rather than padded (spongy) or unfair (one-shot, un-telegraphed)?
