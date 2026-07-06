---
name: cognitive-and-difficulty-accessibility.md
description: Use when the agent is designing difficulty curves, assist modes, save systems, onboarding and tutorials, information density, menu complexity, pause-anywhere behavior, or reviewing whether a game is completable by players with cognitive, attentional, memory, or learning differences.
---

# Cognitive and Difficulty Accessibility

Cognitive and difficulty accessibility governs whether a player's working memory, attention, reading level, processing speed, and tolerance for frustration can actually meet the demands the game places on them. Designers miss this domain more than any other because the demands feel invisible: the tutorial assumes you remember a mechanic introduced three hours ago, the menu buries a critical toggle four layers deep, the puzzle assumes you can hold seven variables in mind, and the difficulty curve assumes a tolerance for repeated failure that is far from universal. The judgment problem is that "challenge" and "access" are constantly in tension, and designers trained to value challenge tend to resolve that tension in favor of the intended difficulty, quietly excluding players with cognitive disabilities, attention disorders, low literacy, or simply less time. The harm is twofold: players who could love the game abandon it at a wall that was never essential to the experience, and the team believes the difficulty was "right" because their core audience cleared it. The agent has meaningful freedom in shaping difficulty, but the governing principle is that difficulty should be chosen by the player, not imposed by default, and that no player should be locked out of a game's content because the cognitive load was calibrated to an assumed norm.

## Core Rules

### Treat Difficulty as a Player Choice, Not a Designer Verdict

The single most important cognitive-accessibility decision is whether the player can shape the game's demands to match their capacity on a given day. This means granular, multi-axis difficulty rather than a single easy-normal-hard slider, and it means allowing the player to change difficulty mid-game without punishment. Decision criterion: can a player who hits a wall reduce the specific demand blocking them (enemy damage, resource scarcity, timing pressure, puzzle complexity) without being forced into a globally degraded experience or a stigmatized "story mode"? When difficulty is a verdict baked into the design, the game is accessible only to the players who already match it.

### Reduce Unnecessary Working Memory and Information Density

Players should not be required to hold more in mind than the core loop demands. Long ability lists, multi-step crafting trees, puzzles requiring simultaneous tracking of many variables, and menus that hide critical state all impose working-memory load that excludes players with memory or attention differences. The rule is to externalize state wherever possible: clear journals, objective trackers, in-world reminders, tooltips on hover, and the ability to re-read or re-watch tutorials. Decision criterion: if a player steps away for a week and returns, can they reconstruct what they were doing and how to do it without external guides? If not, the game is taxing memory it does not need to.

### Provide Clear, Retrievable Onboarding for Every Mechanic

Tutorials that fire once and never return are a cognitive-accessibility wall for players who learn slowly, who took a break, or who have attention differences and missed the prompt. Every mechanic should be re-teachable: a codex, a practice mode, a "remind me" prompt, or contextual hints that reappear on prolonged failure. Decision criterion: is there any mechanic in the game that a returning or struggling player cannot relearn from inside the game itself? Reliance on the player remembering a one-time popup is reliance on a capacity many players do not have on a given day.

### Allow Pause, Save, and Step-Away Freedom

Cognitive and attentional disabilities, caregiving responsibilities, and simply being an adult with interruptions all demand that the player can stop the game at any moment without penalty. This means pause-anywhere in single-player games, generous and player-initiated save systems, no punishment for leaving a session mid-encounter, and no live-service time pressure in content that is fundamentally single-player. Decision criterion: can a player walk away from the game at any instant and return later to the same state without losing progress or facing a penalty? Games that cannot be paused are inaccessible to a large class of players by design, not by accident.

### Align Reading Level and Language With the Broadest Audience

Text in games is routinely written at a reading level and density that excludes players with low literacy, cognitive disabilities, non-native speakers, and children. The rule is to write UI and critical-path text plainly, to keep narrative text optional or summarized, and to support localization and plain-language modes where feasible. Decision criterion: can a player with moderate reading difficulty understand every instruction the game requires them to act on? Flavor and lore can be rich; instructions and state must be clear.

### Avoid Punishing Failure Loops That Compound Cognitive Load

When failure resets long progress, repeats unskippable cutscenes, or forces re-traversal, the cost of failure becomes a cognitive and emotional tax that drives players away. The rule is to checkpoint generously, to make failure recoverable quickly, and to never couple cognitive difficulty (figuring out what to do) with motor or repetition difficulty (doing it over and over). Decision criterion: when a player fails, do they retry the interesting decision quickly, or do they repeat minutes of solved content? If the latter, the game is taxing patience and attention rather than skill.

### Offer Assists That Preserve the Experience, Not Degrade It

Cognitive assists — objective markers, puzzle hints, auto-manage options for complex systems, simplified controls — should be designed so that using them yields the same game, not a lesser one. The trap is that assists are built as "easy mode" that also strips story, depth, or endings. Decision criterion: when a player enables every cognitive assist, do they still reach the full ending and experience the full narrative? If assists gate content, they punish the players who need them most.

## Common Traps

### The Single Difficulty Slider That Moves Everything at Once

A designer ships easy-normal-hard, where easy lowers enemy health but also strips mechanics, speeds up timers, and removes content, conflating several independent demands under one knob. The trap is that a player who needs lower cognitive load does not necessarily want fewer mechanics, and a player who needs more forgiving combat does not want simpler puzzles. The false signal is that difficulty options exist; the harm is that the options bundle unrelated demands, so players cannot tune the specific wall they hit and instead get a globally mismatched experience.

### One-Shot Tutorials and the "You Should Know This by Now" Assumption

A mechanic is taught once in hour one and then assumed forever, with no in-game way to relearn it. The trap is that the designer and core testers played continuously and never forgot, so the assumption feels reasonable. The false signal is that the tutorial "worked" for the team; the harm is that returning players, players with memory differences, and players who skipped the popup are stranded with no recourse but external wikis, and many simply quit.

### Checkpoint Scarcity Disguised as Challenge

A designer places checkpoints far apart to make the game "tense," but the real effect is that every failure taxes the player's time and attention by forcing repetition of solved content. The trap is that the designer confuses repetition with difficulty and punishment with stakes. The false signal is that the game "feels hard" in a respected way; the harm is that players with limited time, attention differences, or lower frustration tolerance abandon the game, and the team attributes the churn to the audience rather than the design.

### Information Hidden in Menu Depth

Critical state — a buff, a quest step, a crafting recipe — is buried three menus deep, and the player is expected to remember it exists and where to find it. The trap is that the designer knows the menu by heart and never experiences the navigation cost. The false signal is that the information "is in the game"; the harm is that players with memory or attention differences cannot reliably surface it when they need it, and the game feels confusing rather than deep.

### Stigmatizing Assist Use

The game offers a hint mode or auto-manage but labels it "casual," disables achievements, or shows a banner announcing that the player is being helped. The trap is that the assist is framed as a lesser way to play, exploiting shame to keep players on the intended difficulty. The false signal is that the option exists and is therefore sufficient; the harm is that players who need the assist either endure the stigma or forgo it and fail, and in neither case has the game actually granted access.

## Self-Check

- Can the player adjust difficulty on multiple independent axes and change it mid-game without punishment, stigma, or content loss?
- Have I externalized state (journals, objective trackers, re-watchable tutorials) so that returning or struggling players can reconstruct what to do without external guides?
- Is every mechanic re-teachable from inside the game, rather than relying on a one-time tutorial popup?
- Can the player pause anywhere, save at any meaningful point, and step away without penalty in fundamentally single-player content?
- Is critical-path text written plainly enough for moderate reading difficulty, with rich lore kept optional?
- Do failure loops return the player to the interesting decision quickly, without repeating solved content or unskippable cutscenes?
- Do cognitive assists preserve the full ending and narrative rather than gating content behind their disablement?
