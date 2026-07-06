---
name: permadeath-and-roguelike-progression.md
description: Use when the agent is designing permadeath systems, building roguelike progression and meta-progression, tuning run stakes and carryover, or evaluating whether permadeath creates engaging tension and meaningful retry or creates punishing, repetitive grind that churns players who cannot sustain the loss of progress.
---

# Permadeath and Roguelike Progression

Permadeath — the loss of the character or run on failure — is a powerful design that creates unmatched tension and stakes, and it is also one of the most punishing designs, capable of churning players who cannot sustain the repeated loss of progress. The judgment problem is that permadeath must create tension (the stakes that make each run matter), must provide meaningful progression across runs (so failure is not pure loss), and must balance the punishment against the willingness to retry, and agents tend to miss this because permadeath that is thrilling for the designer (who values the tension) can be punishing for players who experience it as repetitive grind, and because the meta-progression that makes permadeath sustainable is easy to underbuild. The harm is permadeath that churns players through repetitive loss, that lacks meaningful carryover, or that punishes in ways that exceed the willingness to retry. This skill covers how to design permadeath that creates engaging tension, build meta-progression that sustains retry, and balance the punishment. The agent has latitude in the system, but the obligation to make permadeath sustainable rather than purely punishing is not optional.

## Core Rules

### Ensure Each Run Is Varied Enough to Sustain Repetition

Because permadeath requires repeated runs, each run must be varied enough — through procedural generation, varied builds, emergent situations — that repetition does not fatigue, because runs that repeat the same content fatigue the player and the permadeath becomes a grind. The decision rule: ensure run variety (procedural elements, build diversity, emergent situations), and avoid runs that repeat identically. Repetitive runs fatigue, because the variety that would sustain repetition was absent.

### Provide Meta-Progression So Failure Is Not Pure Loss

Permadeath must provide meta-progression — progression that carries across runs, so each run contributes to long-term growth even on failure — so failure is not pure loss, and the player who fails still advances, sustaining the willingness to retry. The decision rule: implement meta-progression (unlocks, permanent upgrades, knowledge), ensure each run contributes to it, and avoid permadeath where failure advances nothing. Pure-loss permadeath churns, because the player who fails loses everything and advances nothing.

### Balance Run Length Against the Cost of Loss

Run length must be balanced against the cost of loss — shorter runs make the permadeath loss more tolerable, longer runs make it more punishing — because a run length that produces significant time investment before loss exceeds the willingness to retry for many players. The decision rule: calibrate run length to the loss tolerance (shorter for broader audiences, longer for hardcore), and avoid run lengths that make loss too costly. Over-long runs make loss too punishing, because the time investment exceeded the loss tolerance.

### Make the Tension of Permadeath Felt Without Being Crushing

Permadeath should create tension — the stakes that make each decision and encounter matter — without being crushing, and the tension must be calibrated so it enhances engagement rather than producing anxiety that drives players away. The decision rule: calibrate the permadeath tension to enhance engagement (stakes that matter) without crushing (anxiety that paralyzes), and avoid permadeath that is so punishing the tension becomes dread. Crushing permadeath drives players away, because the tension exceeded engagement into anxiety.

### Provide Player Knowledge and Skill Growth as Permanent Progression

Beyond mechanical meta-progression, permadeath games provide knowledge and skill growth — the player learns the systems, enemies, and strategies across runs — and this knowledge progression must be real (the systems must be learnable, the player must get better), so failure contributes to the player's mastery even without mechanical carryover. The decision rule: ensure the game's systems are learnable across runs (consistent rules, readable patterns), so the player's knowledge grows, and avoid systems that cannot be learned. Unlearnable systems prevent knowledge progression, because the player could not get better across runs.

### Offer Options to Modulate Permadeath for Different Audiences

Because permadeath's punishment exceeds some players' tolerance, offering options to modulate it — easier modes, partial permadeath, save-resume — broadens the audience while preserving the core permadeath experience for those who want it, and the options should be framed without stigma. The decision rule: offer permadeath modulation options for players who need them, frame them without stigma, and preserve the core experience as the default. Modulation-free permadeath excludes players who cannot sustain the punishment, because no option to modulate was provided.

## Common Traps

### Repetitive Runs That Fatigue

The team designs runs that repeat the same content — static layouts, fixed encounters, no variety — and the repetition fatigues the player across the multiple runs permadeath requires. The trap is that the run content is designed. The false signal is that the run is playable. The harm is that the player repeats identical content on every run, the repetition fatigues, the permadeath becomes a grind rather than a fresh challenge, and the player churns from fatigue, because the run variety was absent.

### Pure-Loss Permadeath Without Meta-Progression

The team designs permadeath without meta-progression — failure loses everything, advances nothing — and the player who fails has no carryover, and the willingness to retry erodes. The trap is that pure-loss permadeath has maximum stakes. The false signal is that the permadeath is punishing. The harm is that the player who fails loses all progress and advances nothing, the willingness to retry erodes with each pure loss, the player churns after repeated failures that contributed nothing, and the permadeath is unsustainable, because no meta-progression carried across runs.

### Over-Long Runs Making Loss Too Punishing

The team calibrates run length too long — significant time investment before the permadeath loss — and the loss becomes too punishing for the willingness to retry. The trap is that long runs feel substantial. The false signal is that the runs are epic. The harm is that the player invests significant time before losing it all, the loss exceeds the tolerance, the player who would retry shorter runs instead churns, and the permadeath is experienced as a time tax, because the run length was not balanced against the loss cost.

### Crushing Permadeath That Drives Players Away

The team calibrates permadeath so punishing that the tension becomes crushing anxiety, and players are driven away rather than engaged. The trap is that punishing permadeath has maximum stakes. The false signal is that the permadeath is hardcore. The harm is that the tension exceeds engagement into anxiety, the player experiences dread rather than thrill, the players who cannot sustain the crushing punishment leave, and the permadeath that should engage instead excludes, because the tension was not calibrated to enhance rather than crush.

### Unlearnable Systems Preventing Knowledge Growth

The team designs systems that cannot be learned across runs — inconsistent rules, unreadable patterns, arbitrary difficulty — and the player cannot grow in knowledge and skill, so failure does not contribute to mastery. The trap is that the systems are challenging. The false signal is that the game is hard. The harm is that the player cannot learn from failure (the systems are opaque), the knowledge progression that would sustain retry is absent, the player repeats runs without getting better, and the churn accelerates, because the systems were not learnable.

### Modulation-Free Permadeath Excluding Players

The team offers only full permadeath without modulation options, and players who cannot sustain the punishment are excluded. The trap is that pure permadeath preserves the experience. The false signal is that the game is uncompromising. The harm is that players who need modulation (easier modes, partial permadeath) have no option, they churn rather than endure the punishment, the audience that would engage with modulated permadeath is lost, and the game serves only the hardcore, because no modulation was offered.

## Self-Check

- Are runs varied enough (procedural elements, build diversity, emergence) to sustain repetition without fatigue?
- Is there meta-progression that carries across runs, so failure contributes to long-term growth rather than pure loss?
- Is run length balanced against the cost of loss, avoiding over-long runs that make loss too punishing?
- Is the permadeath tension calibrated to enhance engagement without crushing into anxiety that drives players away?
- Are the game's systems learnable across runs, so the player's knowledge and skill grow with each failure?
- Are modulation options offered for players who need them, framed without stigma, with the core experience preserved?
- Did I confirm the permadeath is sustainable and engaging rather than purely punishing and churning?
