---
name: first-session-hook-and-onboarding.md
description: Use when the agent is designing a game's first session and onboarding, planning what a new player experiences in the opening minutes, deciding how to teach without lecturing, or evaluating whether the first session hooks the player or loses them to confusion, boredom, or a front-loaded tutorial.
---

# First Session Hook and Onboarding

The first session is where retention is decided: players form their judgment of a game within minutes, and most who will churn do so in the first session, long before they reach the systems the team spent years building. The judgment problem is that the first session must simultaneously teach, hook, and respect the player's time, and these goals conflict — a thorough tutorial teaches but bores, a front-loaded spectacle hooks but confuses, and a hands-off approach respects agency but loses players who cannot figure out what to do. Agents tend to miss this because the team experiences the opening as fresh and exciting (it is the game they built), while a new player experiences it as a stream of unfamiliar systems demanding attention, and because the metrics that reveal first-session failure (early churn) are lagging. The harm is a game whose opening loses the majority of players before they reach the experience that would have retained them. This skill covers how to design the first session for hook and teaching, manage the tutorial tradeoff, and validate the opening against real new players. The agent has latitude in the opening's content, but the obligation to make the first session retain is not optional.

## Core Rules

### Deliver a Meaningful, Satisfying Action in the First Minutes

The player must perform something satisfying — a successful action, a moment of agency, a taste of the core fantasy — within the first few minutes, or the session has not hooked them and the rest of the onboarding is wasted on a player already disengaging. The decision rule: identify the earliest moment the player can perform a core satisfying action, and engineer the opening to deliver it as early as possible, before or alongside teaching. Openings that delay satisfaction behind lengthy tutorial or cutscenes lose players who never reach the payoff, because the hook was deferred past the patience window.

### Teach Through Play, Not Through Text or Lecture

Players learn mechanics by doing them, not by reading about them, and tutorial delivered as text walls or extended lectures is skipped, ignored, or resented. The decision rule: convert every tutorial beat into a required action the player performs in a safe context, so learning is doing, and reduce text to the minimum needed to prompt the action. Tutorials that tell rather than show produce players who cannot perform the mechanic because they never practiced it, because the teaching medium did not match the learning medium.

### Front-Load Only What the Next Ten Minutes Require

The opening should teach only what the player needs to engage the immediate next stretch, not everything the game will eventually require, because front-loading the full mechanic set overwhelms and is forgotten before it is used. The decision rule: scope the opening's teaching to the mechanics the first session uses, and defer the rest to contextual teaching when each becomes relevant. Front-loaded comprehensive tutorials teach mechanics the player does not use for hours, which are forgotten and must be re-taught, wasting the opening on content that does not stick.

### Provide Clear, Immediate Direction Without Removing Agency

The new player must always know what to do next — where to go, what to try — without the game seizing control and doing it for them, because clear direction reduces confusion while preserved agency sustains engagement. The decision rule: for each moment of the opening, confirm the player has a clear next goal (an objective marker, an environmental cue, a prompt) and the agency to pursue it their way. Openings that provide no direction lose players to confusion; openings that direct too tightly lose players to the feeling of being railroaded.

### Respect the Player's Time by Removing Unskippable Friction

Unskippable cutscenes, slow walking sections, extended dialogue, and forced pacing in the opening tax the player's time before they are committed, and the player who feels their time being wasted in the first minutes leaves. The decision rule: make all opening narrative and pacing skippable or compressible, and ensure no unskippable element exceeds the patience of a player deciding whether to continue. Openings that hold the player captive through unskippable content signal disrespect for their time and convert the uncommitted into churn.

### Validate the Opening With Genuine New Players Continuously

The opening's quality is measurable only with players who have never seen the game, because experienced testers and developers carry knowledge that masks the confusion real new players face. The decision rule: test the opening continuously with genuine new players throughout development, observe where they stall, quit, or express confusion, and revise iteratively. Openings validated only with experienced testers ship with confusion the team could not see, because the knowledge gap that produces the confusion was never present in testing.

## Common Traps

### Delaying the Hook Behind a Front-Loaded Tutorial

The team front-loads a comprehensive tutorial before the player reaches any satisfying action, believing the player must understand the systems before they can enjoy them, and the player churns during the tutorial before reaching the hook. The trap is that thorough teaching feels responsible. The false signal is that the tutorial is comprehensive. The harm is that the player never reaches the satisfying action that would hook them, the comprehensive tutorial is delivered to an audience that is leaving, and the retention that depended on the first session is lost because the hook was deferred behind teaching the player did not stay to receive.

### Teaching by Telling Instead of by Doing

The tutorial delivers mechanics as text or voiceover explanation, the player reads or hears it, and then is expected to perform the mechanic without having practiced it, and fails. The trap is that text tutorials are easy to author and review. The false signal is that the mechanic is explained clearly. The harm is that the player cannot perform a mechanic they only read about, they conclude the game is hard or the controls are bad, and the teaching that was meant to enable play instead produced failure, because the explanation did not become competence without practice.

### Front-Loading the Full Mechanic Set

The opening teaches every mechanic the game will use, so the player is prepared for all eventualities, and the result is an overwhelming deluge of systems the player cannot absorb and will not use for hours. The trap is that comprehensive teaching feels thorough. The false signal is that the player has been introduced to everything. The harm is that the player is overwhelmed, retains little of what was taught, and must be re-taught mechanics later when they become relevant, wasting the opening on content that did not stick, because the teaching was scoped to the game's full needs rather than the session's immediate needs.

### No Direction That Loses Players to Confusion

The team, committed to respecting agency, provides minimal direction in the opening, assuming the player will explore and discover, and the player does not know what to do and quits. The trap is that hands-off design feels respectful. The false signal is that the opening is unintrusive. The harm is that the new player, without the team's knowledge of the game, cannot determine the goal or the path, confusion converts to frustration, and the player leaves in the first minutes not because the game is bad but because they could not figure out what to do, because direction was withheld in the name of agency.

### Unskippable Opening That Wastes the Player's Time

The opening forces the player through unskippable cutscenes, slow walking, or extended dialogue before they reach interactive gameplay, and the uncommitted player feels their time being held hostage and leaves. The trap is that the opening's pacing is part of the artistic vision. The false signal is that the opening is cinematic and impactful. The harm is that the player who has not yet decided to invest resents the time tax, the unskippable content signals disrespect, and the player churns before the opening completes, because the team prioritized the vision over the player's freedom to engage at their pace.

### Validating the Opening Only With Experienced Testers

The team tests the opening with developers and experienced testers who know the game, and they navigate it smoothly, masking the confusion that genuine new players will face, and the opening ships with defects the team could not see. The trap is that experienced testers are the available pool. The false signal is that the opening tests well. The harm is that real new players stall and quit at points the experienced testers passed without trouble, the confusion is invisible to anyone who knows the game, and the opening that passed internal review fails the audience it was built for, because it was never tested by anyone who did not already know it.

## Self-Check

- Does the opening deliver a meaningful, satisfying core action within the first few minutes, before or alongside teaching?
- Is every tutorial beat converted into a required action the player performs, with text reduced to the minimum prompt?
- Is the opening's teaching scoped to what the first session uses, with the rest deferred to contextual teaching?
- Does the player always have a clear next goal while retaining the agency to pursue it their way?
- Are all opening narrative and pacing elements skippable or compressible, with no unskippable element exceeding patience?
- Am I testing the opening continuously with genuine new players, observing where they stall or quit?
- Did I avoid front-loading the full mechanic set, recognizing that comprehensive early teaching overwhelms and does not stick?
