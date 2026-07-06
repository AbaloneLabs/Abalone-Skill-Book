---
name: save-system-and-checkpoint-placement.md
description: Use when the agent is designing save systems, placing checkpoints, tuning save frequency and granularity, or evaluating whether the save system respects player time and progress or produces lost progress, repeated content, and frustration through poorly placed checkpoints, autosave gaps, or punishing save scarcity.
---

# Save System and Checkpoint Placement

The save system is the guardian of the player's progress and time, and it is also one of the most underappreciated sources of frustration — through lost progress (saves too sparse), repeated content (checkpoints too far back), and the punishing save scarcity that turns failure into a tax on the player's time. The judgment problem is that save frequency must respect the player's time (so progress is not lost, content is not repeated) while serving the game's tension design (some games use save scarcity for stakes), and checkpoint placement must balance convenience against the engagement that well-spaced checkpoints provide. Agents tend to miss this because a save system that looks fine in design (checkpoints at logical points) can be miserable in play (too far apart, before unskippable cutscenes, after long approaches), and because the temptation to use save scarcity as difficulty produces time-punishment rather than engagement. The harm is players who lose progress to sparse saves, who repeat content due to distant checkpoints, and who experience failure as a time tax rather than a learning opportunity. This skill covers how to design save systems and checkpoint placement that respect player time while serving the game's tension. The agent has latitude in the system, but the obligation to respect the player's progress and time is not optional.

## Core Rules

### Match Save Frequency to Session Lengths and Player Time Respect

Save frequency must be matched to the player's session lengths and time investment — frequent enough that progress is not lost to interrupted sessions, that the player can stop without losing significant progress — because a save system that does not respect the player's time produces anxiety about stopping and lost progress. The decision rule: analyze the player's typical session lengths and progress increments, ensure saves capture progress at intervals that respect those, and avoid save gaps that risk significant loss. Save gaps that exceed session increments lose progress, because the frequency did not respect the time.

### Place Checkpoints to Minimize Repeated Content on Failure

Checkpoint placement must minimize the repeated content the player endures on failure — checkpoints should be close enough to the challenge that retry starts at the challenge, not before a long approach or unskippable content — because distant checkpoints punish failure with repetition rather than learning. The decision rule: place checkpoints immediately before challenges (not before long approaches), after completed sections (to confirm progress), and avoid checkpoints that force repeating content to retry. Distant checkpoints punish with repetition, because the retry did not start at the challenge.

### Use Save Scarcity Deliberately for Tension, Not as Default Difficulty

If save scarcity (limited saves, save points) is used to create tension, it must be a deliberate choice that serves the game's horror or survival fantasy, not a default difficulty approach that punishes all players with time loss, and the scarcity must be calibrated to produce tension without excessive punishment. The decision rule: if using save scarcity, define the tension purpose, calibrate the scarcity to produce tension without excessive repetition, and avoid scarcity as a default that punishes without purpose. Purposeless save scarcity punishes, because the scarcity served no tension end.

### Ensure Autosave Captures Progress Without Gaps

Autosave must capture the player's progress without gaps — saving after significant events, completions, and pickups — so the player who stops or crashes does not lose progress to an autosave gap. The decision rule: implement autosave at significant progress points (after encounters, after pickups, at area transitions), and avoid autosave gaps that risk losing recent progress. Autosave gaps lose progress, because the save did not capture the recent events.

### Never Force Repetition of Unskippable Content After Checkpoints

Checkpoints must never force the player to repeat unskippable content — cutscenes, dialogues, long approaches — on retry, because repeating unskippable content is pure time punishment that produces resentment rather than engagement. The decision rule: place checkpoints after unskippable content, make content skippable after first viewing, and avoid checkpoints that precede unskippable sections. Checkpoints before unskippable content punish, because the retry forced repetition the player could not skip.

### Provide Manual Save for Player Control Where Appropriate

Beyond autosave, the player should have manual save control where appropriate — to save before challenging sections, to preserve a state, to manage multiple saves — so the player has agency over their progress rather than being at the mercy of the autosave schedule. The decision rule: provide manual save where it serves the player's control (unless the game's tension design forbids it), and avoid relying solely on autosave where player control matters. Autosave-only systems remove player agency, because the player could not control their saves.

## Common Traps

### Sparse Saves That Lose Progress

The team spaces saves too far apart, and the player who stops or crashes loses significant progress to the gap, producing anxiety and resentment. The trap is that sparse saves serve tension. The false signal is that the saves are meaningful. The harm is that the player loses progress to save gaps, the anxiety about stopping undermines the experience, the player who loses progress may abandon the game, and the save system is resented, because the frequency did not respect the player's time.

### Distant Checkpoints That Punish With Repetition

The team places checkpoints far from challenges — before long approaches, after completed sections but before the next — and failure punishes the player with repeated content rather than learning. The trap is that distant checkpoints add stakes. The false signal is that the checkpoints are placed. The harm is that the player repeats content to retry the challenge, the failure is punished with time loss rather than teaching, the frustration of repetition accumulates, and the player may abandon the challenge, because the retry did not start at the challenge.

### Save Scarcity as Default Difficulty

The team uses save scarcity — limited saves, rare save points — as a default difficulty approach rather than a deliberate tension choice, and all players are punished with time loss. The trap is that scarcity feels challenging. The false signal is that the game has stakes. The harm is that the scarcity punishes without serving a tension purpose, the player loses progress and repeats content, the time punishment produces resentment, and the game excludes players who cannot tolerate the scarcity, because it was used as default rather than deliberate design.

### Autosave Gaps That Lose Recent Progress

The team implements autosave at too-coarse intervals, and the player who stops or crashes loses recent progress to the gap. The trap is that the autosave runs periodically. The false signal is that progress is saved. The harm is that the autosave did not capture recent events, the player loses the progress between the last save and the stop, the anxiety about gaps undermines the experience, and the autosave fails its purpose, because the intervals were too coarse.

### Checkpoints Before Unskippable Content

The team places checkpoints before unskippable cutscenes or dialogues, and the player who fails must repeat the unskippable content on every retry. The trap is that the checkpoint precedes the section. The false signal is that the checkpoint is placed. The harm is that the player repeats unskippable content on every retry, the repetition is pure time punishment, the resentment accumulates rapidly, and the player may abandon the section rather than endure the repeated unskippable, because the checkpoint forced repetition the player could not skip.

### Autosave-Only Systems Removing Player Agency

The team relies solely on autosave without manual save, and the player has no agency over their progress — cannot save before challenges, cannot preserve states, cannot manage multiple saves. The trap is that autosave-only is simpler. The false signal is that progress is automatically saved. The harm is that the player cannot control their saves, they cannot prepare for challenges or preserve states, the agency that manual save would provide is absent, and the player is at the mercy of the autosave schedule, because manual control was not provided.

## Self-Check

- Is save frequency matched to player session lengths, so progress is not lost to interrupted sessions?
- Are checkpoints placed immediately before challenges, minimizing repeated content on failure?
- If save scarcity is used, is it a deliberate tension choice calibrated to avoid excessive punishment, not default difficulty?
- Does autosave capture progress at significant points without gaps that risk losing recent events?
- Are checkpoints placed after unskippable content, with content made skippable, to avoid forced repetition?
- Is manual save provided for player control where appropriate, beyond autosave?
- Did I confirm the save system respects player time and progress rather than punishing with repetition and loss?
