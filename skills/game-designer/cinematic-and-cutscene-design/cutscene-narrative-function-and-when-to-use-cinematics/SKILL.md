---
name: cutscene-narrative-function-and-when-to-use-cinematics.md
description: Use when the agent is deciding whether and where to use cinematic cutscenes in a game, evaluating whether a story beat should be playable or non-interactive, sequencing cinematics relative to gameplay, or reviewing whether cutscenes serve narrative function versus spectacle; trigger contexts include cutscene, cinematic, non-interactive scene, story beat, narrative pacing, when to use cinematics, playable versus scripted, dialogue scene, intro cinematic, ending cinematic, mid-game cutscene, exposition; important risks include overuse of cinematics, loss of player agency, pacing disruption, and cutscenes that pad rather than reveal.
---

# Cutscene Narrative Function And When To Use Cinematics

A cinematic is not a reward; it is a design instrument that trades player agency for authored control, and the agent is usually asked to decide whether, where, and how to use one while the game's narrative goals, the player's expectation of agency, and the production cost of non-interactive content are all in tension. The judgment problem is that cinematics can powerfully deliver moments the gameplay cannot — a character's interior turn, a scope shift, a consequence the player caused but cannot witness in play — and they can just as powerfully stall pacing, sever agency, and pad a runtime with spectacle the player endures rather than plays.

The agent tends to miss this because cutscenes are often requested by default ("we need an intro cinematic," "the ending should be a movie") rather than justified by function, and because the decision to make a beat non-interactive is treated as a production choice rather than a design one. The harm is a game where the player is repeatedly ejected from play to watch, where the authored story and the played story diverge, or where expensive cinematics crowd out the interactive systems that would have carried the moment better. Use this skill to slow the decision down enough to expose what each cinematic is actually for, then make the recommendation appropriately conservative when agency, pacing, and cost are at stake.

## Core Rules

### Justify Each Cinematic By Narrative Function, Not By Convention
Before approving a cinematic, state the function it serves that the gameplay cannot: establishing stakes the player could not witness, shifting scope (personal to world), delivering an emotional beat requiring authored timing, communicating a consequence of the player's actions that play cannot frame, or marking a structural boundary (act, chapter, ending). If the function can be served by play — by a scripted encounter, a dialogue, an environmental reveal — the cinematic is not justified. The decision rule: a cinematic is earned when it does something play cannot, and is indulgent when it does something play already does.

### Decide Playable Versus Non-Interactive Per Beat
Each story beat has a correct mode. Beats about player expression, choice, or skill should be playable; beats about revelation, timing, or consequence the player caused but cannot control should be cinematic. The agent should map the story arc beat by beat and assign each a mode, rather than defaulting to "cutscene at every plot point." A beat assigned cinematic that should be playable severs agency; a beat assigned playable that needs authored timing loses its impact. The trap is treating the mode as a production preference rather than a per-beat design decision.

### Sequence Cinematics To Protect Pacing And Flow
Cinematics disrupt the play loop, and their placement determines whether they energize or stall the session. The agent should place cinematics at natural breaks (chapter ends, location transitions, after a climactic encounter) rather than mid-action, keep them short relative to the play they interrupt, and never stack multiple cinematics back to back unless the stacking is itself the beat (an ending montage). A cinematic that interrupts a flow state to deliver exposition the player could have absorbed through play is a pacing defect, regardless of its quality.

### Preserve Continuity Between Authored And Played Story
The authored story the cinematic tells and the played story the player experienced must be continuous. The agent should ensure cinematics reflect the player's actual choices, state, and progression (who lived, what was chosen, what was built), use variable content where the player's state diverges, and avoid cinematics that assert facts contradicting the player's play. A cinematic that tells the player who their character is, when the player's play defined the character differently, breaks the contract between authored and interactive narrative.

### Respect Agency And Avoid The Ejection Trap
Every cinematic ejects the player from agency, and the accumulation of ejections is what makes a game feel like a movie with interruptions. The agent should count the ejections, measure the ratio of non-interactive to interactive time, and treat a high ratio as a design problem even when each cinematic is individually good. When a beat can be delivered through playable, controllable, or lightly-scripted means, prefer them; reserve full non-interactivity for moments that genuinely require authored control.

### Budget Cinematics Against Production And Localization Cost
Cinematics are among the most expensive content to produce and to localize (voice, facial animation, multiple language tracks, accessibility captioning and audio description). The agent should weigh the cost of a cinematic against its narrative return, prefer in-engine cinematics over pre-rendered where asset reuse is possible, and flag cinematics whose cost crowds out gameplay content the project needs more. A cinematic budget that consumes resources needed for core systems is a project risk, not just a creative choice.

### Plan For Skip, Accessibility, And Replay
Players skip, replay, and access cinematics differently. The agent should ensure cinematics are skippable on first and subsequent viewings, that critical gameplay information is never delivered only in a skippable cinematic, that captions and audio description are provided, and that replay does not force unskippable cinematics on the player. A cinematic that gates progress and cannot be skipped is an accessibility and replay defect.

## Common Traps

### The Default Cutscene At Every Plot Point
The team scripts a cinematic at every major plot point by convention, and the game becomes a sequence of interruptions. The trap is that each cinematic is individually justified. The false signal is that the story "needs" the cinematics. The harm is that the ratio of non-interactive time climbs, agency erodes, and the player experiences the story as something done to them rather than something they do.

### The Cinematic That Tells What Play Already Showed
A cinematic restates a revelation, a relationship, or a consequence the player already experienced through play, adding nothing and breaking flow. The trap is that the cinematic "reinforces" the beat. The false signal is that the beat feels important. The harm is redundant pacing and the implication that the authored version overrides the player's lived experience of the same moment.

### The Unskippable Cinematic Gating Progress
A cinematic cannot be skipped and must be watched to progress, punishing replay and excluding players who need to skip for accessibility or time. The trap is protecting the authored moment. The false signal is that the cinematic is essential. The harm is churn on replay, accessibility exclusion, and resentment toward a moment the team wanted to celebrate.

### The Cinematic Contradicting Player State
A cinematic asserts the character's personality, relationships, or choices in ways that contradict how the player played, breaking continuity between authored and played story. The trap is writing the cinematic in isolation from playstate. The false signal is that the cinematic reads well on the page. The harm is the player feels their choices did not matter, because the authored story overrode them.

### Spectacle Cinematics Padding Runtime
Expensive spectacle cinematics are used to pad perceived runtime or production value, consuming budget that core gameplay needed. The trap is that spectacle reads as quality in marketing. The false signal is that the game "looks cinematic." The harm is a hollow core loop surrounded by movies the player endures, and a project whose resources were misallocated toward non-interactive spectacle.

### Mid-Action Cinematics Breaking Flow
A cinematic triggers in the middle of an engaging play sequence to deliver exposition or a scripted beat, ejecting the player from flow. The trap is that the team wants the beat "right here." The false signal is that the moment is dramatic. The harm is the flow state is shattered, and the player resents the interruption more than they appreciate the content.

### Presenting Creative Preference As Design Necessity
The recommendation to add or cut a cinematic is often a judgment call, but the agent should not present a stylistic preference as a rule. State what is known (cost, ratio, function), what is inferred (pacing impact), and what is a creative choice, so the team can decide with the tradeoffs visible.

## Self-Check

- [ ] Is each cinematic justified by a narrative function the gameplay cannot serve, rather than by convention or production preference?
- [ ] Has the story arc been mapped beat by beat, with each beat assigned playable or non-interactive based on whether it is about player expression or authored revelation?
- [ ] Are cinematics sequenced at natural breaks, short relative to the play they interrupt, and never stacked unless stacking is the beat?
- [ ] Do cinematics reflect the player's actual choices, state, and progression, with variable content where playstate diverges?
- [ ] Has the ratio of non-interactive to interactive time been measured, and is a high ratio treated as a design problem even when each cinematic is good?
- [ ] Has the production and localization cost of each cinematic been weighed against its narrative return and against the gameplay content it crowds out?
- [ ] Are cinematics skippable on first and subsequent viewings, with no critical gameplay information delivered only in a skippable cinematic, and with captions and audio description provided?
- [ ] Does the output distinguish cinematics that serve the game from cinematics that serve marketing or runtime padding?
- [ ] Are the function, placement, and mode of each cinematic specific enough for the narrative and design teams to implement without re-deciding?
- [ ] Is uncertainty about pacing and player reception named, with the tradeoffs that would change the recommendation made explicit?
