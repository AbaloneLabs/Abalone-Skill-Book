---
name: balance-tuning-and-iteration-methodology.md
description: Use when the agent is tuning game balance through iteration, designing a balance methodology and feedback loop, or evaluating whether balance iteration converges on good balance or oscillates endlessly, chases outliers, and over-tunes to the point of flattening the strategic variety that makes the game interesting.
---

# Balance Tuning and Iteration Methodology

Balance tuning is the iterative refinement of a game's numbers and systems toward good balance, and it is a methodology problem as much as a design problem, because without a sound iteration methodology the tuning oscillates endlessly (overcorrecting each change), chases outliers (tuning to edge cases), or over-tunes (flattening the variety that makes the game interesting into uniform viability that is also uniform blandness). The judgment problem is that balance iteration must converge (each change moves toward balance, not away), must be informed by the right evidence (data and expert play, not just complaints), and must preserve variety (balance for multiple viable options, not uniform sameness). Agents tend to miss this because the tuning that feels responsive (big swings) often oscillates, and because perfect balance (all options equal) can produce a boring game where no option is distinct. The harm is balance that never converges, that chases outliers, or that flattens the game's variety into blandness. This skill covers how to design a balance iteration methodology that converges and preserves variety, and avoid the oscillation, outlier-chasing, and over-tuning traps. The agent has latitude in the tuning, but the obligation to converge on good balance is not optional.

## Core Rules

### Iterate With Small, Evidence-Informed Changes

Balance changes should be small (so the effect can be measured and overcorrection avoided) and evidence-informed (based on data and expert assessment, not guesswork or complaints), because small evidence-informed changes converge on balance while large reactive changes oscillate around it. The decision rule: iterate with small, evidence-informed changes, measure the effect, and avoid large reactive swings. Large reactive swings oscillate, because the overcorrection moved away from balance.

### Define Balance Criteria Before Tuning

Balance criteria — what "balanced" means for this game (win rates, pick rates, viability thresholds) — must be defined before tuning, because without defined criteria the tuning has no target and no stopping point, and the iteration chases a vague sense of "feels right" that never converges. The decision rule: define balance criteria before tuning, measure against them, and avoid tuning without a target. Targetless tuning does not converge, because the criteria were not defined.

### Tune to the Center, Not to Outliers

Balance tuning should tune to the center (the typical experience, the majority of play) not to outliers (the edge cases, the top-tier exploits), because tuning to outliers distorts the center (nerfing a strategy that is only broken at top tier harms the typical players), and the center is where most play occurs. The decision rule: tune to the center, address outliers with targeted solutions, and avoid outlier-driven global changes. Outlier-driven tuning distorts the center, because the edge case drove the global change.

### Preserve Strategic Variety, Do Not Flatten to Uniformity

Balance should preserve strategic variety (multiple distinct viable options) not flatten to uniformity (all options equal and interchangeable), because perfect balance can produce a boring game where no option is distinct, and the variety of distinct viable options is what makes the game interesting. The decision rule: balance for distinct viable options, and avoid flattening variety. Flattening balance produces blandness, because the distinctiveness was removed.

### Use a Feedback Loop With Measurement and Verification

Balance iteration requires a feedback loop — make a change, measure the effect, verify against criteria, adjust — because without measurement and verification the tuning is guesswork that cannot converge, and the loop closes the gap between change and confirmed effect. The decision rule: use a measurement-and-verification feedback loop, and avoid change-without-measure tuning. Change-without-measure tuning does not converge, because the effect was not verified.

### Know When to Stop Tuning and Ship

Balance iteration must know when to stop — when the criteria are met, when further tuning produces diminishing returns or harm — because endless tuning (chasing perfect balance that does not exist) delays the ship and risks over-tuning, and good enough balance that ships is better than perfect balance that never does. The decision rule: define stopping criteria, recognize diminishing returns, and know when to ship. Endless tuning delays and over-tunes, because the stopping point was not recognized.

## Common Traps

### Large Reactive Swings Oscillating

The team makes large reactive balance swings, and the tuning oscillates. The trap is that the swings are responsive. The false signal is that the change is significant. The harm is that the large swings overcorrect, the overcorrection moves away from balance as much as toward, the next swing overcorrects again, the tuning oscillates around balance without converging, the players endure constant whiplash, and the balance never settles, because the changes were not small and measured.

### Targetless Tuning Not Converging

The team tunes without defined balance criteria, and the tuning does not converge. The trap is that the balance feels off. The false signal is that the tuning is ongoing. The harm is that the targetless tuning chases a vague sense of right, the criteria for balanced are not defined, the tuning has no target and no stopping point, the iteration wanders without converging, the team tunes endlessly, and the balance never reaches a defined good state, because the criteria were not defined.

### Outlier-Driven Tuning Distorting the Center

The team tunes to outliers, and the center is distorted. The trap is that the outliers are broken. The false signal is that the exploits are addressed. The harm is that the outlier-driven tuning nerfs strategies that are only broken at top tier, the typical players (the center) who used those strategies normally are harmed, the center experience is distorted by edge-case tuning, the majority play is worsened to fix the minority exploit, and the balance serves the outlier over the center, because the tuning was not to the center.

### Flattening Variety Into Uniform Blandness

The team over-tunes toward uniform balance, and the variety flattens into blandness. The trap is that all options are viable. The false signal is that the balance is fair. The harm is that the over-tuning makes all options equal and interchangeable, the distinctiveness that made options interesting is removed, the strategic variety collapses into uniform sameness, the game becomes balanced but boring, and the players lose the variety that made the game engaging, because the balance flattened the variety.

### Change-Without-Measure Tuning Not Converging

The team tunes without measurement and verification, and the tuning does not converge. The trap is that the changes are informed. The false signal is that the tuning is responsive. The harm is that the change-without-measure tuning is guesswork, the effect of each change is not verified, the feedback loop is open (changes without confirmed effects), the tuning cannot distinguish what worked from what did not, the iteration wanders, and the balance does not converge, because the effect was not measured.

### Endless Tuning Delaying and Over-Tuning

The team tunes endlessly chasing perfect balance, and the ship delays and the balance over-tunes. The trap is that the balance can be better. The false signal is that the tuning is thorough. The harm is that the endless tuning chases a perfect balance that does not exist, the ship is delayed, the over-tuning flattens variety and introduces new imbalances, the good-enough balance that would have shipped is passed, and the game suffers from tuning that did not know when to stop, because the stopping point was not recognized.

## Self-Check

- Are balance changes small and evidence-informed, with effects measured to avoid overcorrection?
- Are balance criteria (win rates, viability thresholds) defined before tuning begins?
- Is tuning directed at the center (typical play) rather than outliers (edge cases, top-tier exploits)?
- Does balance preserve strategic variety (distinct viable options) rather than flattening to uniformity?
- Is a measurement-and-verification feedback loop used to close the gap between change and confirmed effect?
- Is there a recognized stopping point, avoiding endless tuning that delays and over-tunes?
- Did I confirm the balance iteration converges on good balance while preserving variety rather than oscillating and flattening?
