---
name: contextual-teaching-and-skill-scaffolding.md
description: Use when the agent is designing how mechanics are taught contextually throughout a game, planning skill scaffolding that introduces complexity gradually, deciding when to teach advanced techniques, or evaluating whether a game's teaching cadence builds competence or dumps complexity and leaves players overwhelmed or underprepared.
---

# Contextual Teaching and Skill Scaffolding

Teaching is not a one-time event in the opening; it is a continuous process that spans the whole game, because a game's depth emerges as mechanics combine and advanced techniques become relevant, and each new layer must be taught when the player is ready to absorb it. The judgment problem is that the right moment to teach a technique is when the player needs it — not before (forgotten) and not after (frustrated) — and identifying that moment requires understanding the player's evolving competence, which is something agents often guess rather than measure. Agents tend to miss this because the teaching cadence looks fine when the designer plays (they already know everything), and because the failure modes are opposite and easy to misdiagnose: a player who fails may need more teaching or may be overwhelmed by too much. The harm is players who reach advanced content without the skills to engage it, or who are taught techniques they never use and forget, either of which produces churn at the points where the game should be most engaging. This skill covers how to sequence teaching to the player's readiness, scaffold skill acquisition, and detect when teaching misfired. The agent has latitude in the teaching content, but the obligation to teach at the moment of need is not optional.

## Core Rules

### Teach Each Mechanic at the Moment It Becomes Necessary

A mechanic taught before it is needed is forgotten by the time it becomes relevant; a mechanic needed before it is taught produces frustration. The decision rule: for each mechanic, identify the first situation where the player must use it to progress, and place the teaching immediately before that situation, so the teaching and the need are adjacent in time and the player practices while the knowledge is fresh. Teaching divorced from need — whether early or late — fails to produce competence, because learning requires immediate application to consolidate.

### Scaffold Complexity by Introducing, Then Combining, Then Mastering

Skill acquisition proceeds in stages: introduce a mechanic in isolation, let the player master it, then combine it with prior mechanics, then present challenges requiring fluent combination. The decision rule: structure the teaching cadence so each mechanic passes through introduction, combination, and mastery before the next is introduced, rather than dumping multiple mechanics and immediately requiring their combination. Skipping the isolation stage overwhelms; skipping the combination stage leaves the player unable to integrate; the scaffold is the sequence.

### Use the Environment and Encounters as the Teacher, Not Text

The strongest teaching is embedded in the level or encounter design — a space that can only be traversed with the new mechanic, an enemy that punishes the old approach and rewards the new — so the player learns by attempting rather than by reading. The decision rule: for each teaching moment, design the environment or encounter to require the mechanic, so the player discovers it through play, and use text only to confirm what the play taught. Teaching that relies on text the player may skip produces players who never learned, because the teacher was ignorable.

### Provide Safe Practice Before High-Stakes Application

A newly-taught mechanic should be practiced in a low-stakes context — a safe space, a forgiving enemy, a retryable challenge — before it is required in a high-stakes situation where failure is costly. The decision rule: for each mechanic, insert a practice opportunity between the teaching and the high-stakes requirement, so the player builds competence without punishment. Requiring a freshly-taught mechanic in a high-stakes situation immediately produces failures the player attributes to the game being unfair, because they were not given the practice to develop the skill.

### Detect Teaching Failure From Behavioral Stuck Points

When teaching misfires, the player stalls at the point where the unlearned or undertaught mechanic is required, and this stall is visible in behavior — repeated failures, long pauses, abandonment — before it appears in feedback. The decision rule: instrument the teaching-critical moments with behavioral telemetry (attempt counts, time-to-proceed, abandonment rate), and when players stall at a specific point, diagnose whether the preceding teaching was insufficient and revise. Waiting for players to report teaching problems finds them too late, because the players who needed the teaching already left.

### Re-Teach After Gaps to Counter Forgetting

Players forget mechanics they have not used, especially across play sessions or after long stretches of other content, and a mechanic taught in hour two and required in hour twenty must be re-taught or the player will fail. The decision rule: identify mechanics with long gaps between teaching and use, and provide a refresher — a contextual prompt, a brief reminder, a low-stakes re-introduction — at the point of resumed use. Assuming the player remembers a mechanic from hours ago produces failures attributable to forgetting, not to difficulty, because the teaching was not reinforced at the point of need.

## Common Traps

### Teaching Mechanics Long Before They Are Needed

The team teaches a mechanic in an early tutorial that is not used until much later, assuming early preparation is efficient, and the player has forgotten it by the time it becomes relevant. The trap is that early comprehensive teaching feels thorough. The false signal is that the player has been introduced to the mechanic. The harm is that the player fails when the mechanic is finally required because they forgot it, the failure is attributed to difficulty rather than to the teaching gap, and the comprehensive early teaching was wasted because learning without immediate application does not consolidate, because the teaching and the need were separated by too much time and content.

### Dumping Multiple Mechanics and Requiring Immediate Combination

The team introduces several mechanics in quick succession and immediately presents a challenge requiring their combination, skipping the isolation and mastery stages, and the player is overwhelmed. The trap is that rapid introduction feels efficient. The false signal is that all mechanics have been taught. The harm is that the player cannot combine mechanics they have not individually mastered, the challenge produces frustration rather than engagement, and the player concludes the game is too hard or the controls are bad, because the scaffold that would have built competence was collapsed into a dump.

### Text-Based Teaching the Player Skips

The team delivers teaching through text or dialogue that the player can skip or dismiss, and many players skip it, then fail at the mechanic because they never received the teaching. The trap is that text teaching is easy to author. The false signal is that the teaching is present in the game. The harm is that the skippable teaching reaches only the players who chose to read it, the players who skipped are unprepared, and the failures that result are attributed to difficulty rather than to missed teaching, because the teacher was designed to be ignorable and was ignored.

### Requiring a Fresh Mechanic in a High-Stakes Situation

The team teaches a mechanic and immediately requires it in a punishing encounter, with no practice opportunity, and the player fails repeatedly while learning under pressure. The trap is that immediate high-stakes application feels challenging. The false signal is that the encounter is engaging. The harm is that the player fails for lack of practice rather than lack of skill, the failures feel unfair because the game demanded competence it did not allow to develop, and the player churns at the teaching moment that should have built confidence, because the safe practice stage was skipped.

### Ignoring Behavioral Stuck Points Until Feedback Arrives

The team does not instrument teaching-critical moments, assumes the teaching worked because no one complained, and discovers at launch that players stall at specific points where the teaching was insufficient. The trap is that absence of feedback feels like success. The false signal is that no one reported a teaching problem. The harm is that the players who needed better teaching left without reporting, the stuck points are invisible without behavioral telemetry, and the teaching defects ship undetected, discovered only in launch churn and negative reviews that reference "confusing" or "too hard" sections that were teaching failures, not difficulty.

### Assuming the Player Remembers Across Long Gaps

The team teaches a mechanic early, uses it much later across a long gap, and does not re-teach, assuming the player remembers, and the player has forgotten and fails. The trap is that the mechanic was taught once. The false signal is that the teaching is complete. The harm is that the player fails at the resumed use because the memory decayed across the gap, the failure feels like the game springing a forgotten requirement, and the player who would have succeeded with a refresher instead churns, because the teaching was not reinforced at the moment of resumed need.

## Self-Check

- Is each mechanic taught immediately before the first situation where the player must use it, so teaching and need are adjacent?
- Does the teaching cadence scaffold each mechanic through isolation, combination, and mastery before the next is introduced?
- Are teaching moments embedded in environment and encounter design that require the mechanic, rather than in skippable text?
- Is there a safe, low-stakes practice opportunity between teaching a mechanic and requiring it in a high-stakes situation?
- Am I instrumenting teaching-critical moments with behavioral telemetry to detect stuck points where teaching was insufficient?
- For mechanics with long gaps between teaching and use, have I planned refreshers at the point of resumed need?
- Did I avoid dumping multiple mechanics and immediately requiring combination, recognizing that mastery requires staged scaffolding?
