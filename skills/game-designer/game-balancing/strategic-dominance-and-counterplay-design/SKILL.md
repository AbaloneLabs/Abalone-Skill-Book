---
name: strategic-dominance-and-counterplay-design.md
description: Use when the agent is designing abilities, weapons, factions, decks, units, or matchups, evaluating whether a strategy is overpowered or degenerate, designing counters and hard/soft counters, or reviewing whether player choices remain meaningful under optimization and competitive pressure.
---

# Strategic Dominance and Counterplay Design

Strategic dominance is the failure mode where one strategy, build, faction, or option is so strong that rational players converge on it, and the game's space of meaningful choice collapses. Counterplay is the design property that prevents this: every strong option should have a response, and every response should itself be answerable, so that no single strategy is unbeatable and the meta remains alive. Designers miss the judgment here because in casual play, with imperfect execution and uncoordinated opponents, almost anything works, so dominance hides. It emerges only under optimization — when players min-max, share knowledge, and coordinate — and by then the design is shipped and the meta is degenerate. The judgment problem is that counterplay is not the same as balance: a game can be numerically balanced and still have no counterplay if every matchup is a coin flip or a stat check, and a game can have rich counterplay and still feel imbalanced if one option is slightly too strong. The harm of poor counterplay is that the game becomes solved: the community converges, variety dies, and the experience becomes repetitive for everyone, including the designers who wanted a living game. The agent has freedom in how it constructs matchups and abilities, but it must design against the optimizing player, not the casual one, and it must verify counterplay under pressure rather than assume it.

## Core Rules

### Design Every Strong Option With a Counter, and Every Counter With a Counter

The foundational rule of strategic depth is that power must be answerable. A dominant strategy exists whenever the best response to an option is to use that same option. The test is recursive: for every strong strategy S, there must be a counter C that beats S, and C must itself be answerable by some other strategy, so that the loop does not terminate at a single best move. Decision criterion: trace the matchup graph for your strongest options and confirm it contains cycles, not a single node that beats everything. A strategy with no losing matchup is, by definition, dominant and must be changed.

### Distinguish Hard Counters, Soft Counters, and Stat Checks

A hard counter nearly guarantees a win in a matchup (rock-paper-scissors), a soft counter shifts the odds meaningfully but allows outplay, and a stat check is a matchup with no counterplay where the higher number simply wins. Hard counters create rock-paper-scissors metas that feel arbitrary; stat checks feel unfair because the loser could not have done anything; soft counters are where skill expression lives. The rule is to prefer soft counters for core matchups, use hard counters sparingly to keep specific strategies honest, and eliminate stat checks entirely because they remove agency. Decision criterion: in a given matchup, can the losing player win through better decisions or execution? If not, it is a stat check and must be redesigned.

### Test for Dominance Under Optimization, Not Under Casual Play

Dominance is invisible in casual play because players do not optimize, so internal playtests of mixed skill will not reveal it. The rule is to test the design against the assumption that players will find and exploit the strongest option: hand the strongest build to your best tester, let them optimize it, and see if the opposing options can answer it. Decision criterion: have you stress-tested the design with a player whose explicit goal is to break it, using the strongest possible version of each strategy? If you tested only intended play, you have not tested for dominance.

### Avoid Coin-Flip Matchups Masquerading as Balance

A matchup that is 50/50 because both sides have no agency — two strategies that ignore each other and race — is not balanced, it is non-interactive. The trap is that the win rate looks even, so the team concludes the matchup is fine, but neither player is making meaningful decisions about the other. The rule is that balance must be interactive: a 50/50 matchup should arise from both players making choices that matter, not from two solitaire strategies whose outcomes average out. Decision criterion: in this matchup, do the players' decisions affect each other's outcome? If not, the evenness is meaningless.

### Ensure Counterplay Does Not Require Information the Player Cannot Have

A counter is only real if the player can identify the need for it in time to execute it. If the correct counter requires knowing the opponent's hidden strategy before it is revealed, the counterplay is theoretical, not practical. The rule is to design information flow so that counters are actionable: either reveal the relevant information early enough to respond, or make counters general enough to hold in multiple scenarios. Decision criterion: at the moment a player needs to counter, do they have the information to choose the right counter? If the answer is "they had to guess," the counterplay is a coin flip with extra steps.

### Watch for First-Mover and Snowball Advantages That Collapse Choice

In many games, the player who gains an early lead converts it into an insurmountable advantage, so the game is effectively decided in the opening and the rest is a formality. This is a counterplay failure: the losing player has no path back. The rule is to design comeback mechanics, diminishing returns on leads, or rubber-banding that keeps the game competitive without feeling unfair to the leader. Decision criterion: from a losing position, is there a realistic path to victory through superior play, or is the outcome locked? If locked, the game lacks counterplay from behind.

### Separate Balance From Counterplay in Diagnosis

When a strategy dominates, the fix is not always a number change. Sometimes the strategy has no counter because no counter was designed (a counterplay problem), and sometimes the counter exists but the numbers make it insufficient (a balance problem). Diagnosing wrong leads to endless nerfs that never fix the dominance. The rule is to ask: does a counter exist at any value of the parameters? If no counter exists at any value, the problem is counterplay and requires a design change; if a counter exists but loses, the problem is balance and requires a tuning change. Decision criterion: have you confirmed whether the dominance is structural or numeric before choosing the fix?

## Common Traps

### The Option That Beats Everything Slightly

A strategy is 55% against every other option, which sounds balanced because no matchup is a blowout, but because it wins every matchup it is strictly dominant and the meta collapses to it. The trap is that "slightly favored everywhere" looks moderate on a matchup chart but is categorical in effect, because players optimize toward any edge. The false signal is the absence of any 90/10 matchup; the harm is a homogeneous meta where the slight-favorite option is the only rational pick and everything else is dead.

### The Theoretical Counter That Is Not Actionable

The design doc says option B counters option A, so the team believes counterplay exists, but in practice the player facing option A cannot identify it as option A until it is too late to switch to option B. The trap is that the counter exists in the strategy space but not in the information space. The false signal is the matchup chart showing a favorable matchup for B; the harm is that the counter is never actually deployed and option A dominates in practice despite the paper counter.

### Balancing to Win Rate Instead of to Interaction

The team sees a 50% win rate and declares a matchup balanced, but the matchup is two non-interactive strategies racing past each other. The trap is that win rate is treated as the whole of balance when it is only one dimension. The false signal is the even number; the harm is that the game feels solitaire and unstrategic even though the spreadsheet says it is fair, and players disengage without the team understanding why.

### Snowball Design Mistaken for Rewarding Good Play

Early advantages compound into insurmountable leads, and the team calls this "rewarding skill," when it actually removes counterplay and decides games in the first minutes. The trap is that rewarding an early edge feels meritocratic. The false signal is that better players win, which is true but misleading; the harm is that the losing player has no meaningful decisions left, games feel decided early, and comebacks — the most exciting outcomes — become impossible.

### Nerfing the Symptom While the Dominant Strategy Adapts

The community complains about a specific overpowered build, the team nerfs it, and a different build from the same dominant strategy family immediately takes its place, because the underlying structural dominance was never addressed. The trap is treating each manifest build as the problem rather than the strategy space that produces them. The false signal is that the nerf "worked" because the complained-about build disappeared; the harm is endless whack-a-mole balance where the meta never diversifies because the root cause is structural.

## Self-Check

- For my strongest options, does the matchup graph contain cycles, or is there a node with no losing matchup?
- Have I classified my core matchups as hard counters, soft counters, or stat checks, and eliminated stat checks from competitive play?
- Did I test for dominance by handing the strongest optimized build to a skilled player trying to break the game, not just by observing intended play?
- In my even matchups, do the players' decisions actually interact, or are they non-interactive solitaire races averaging to 50%?
- At the moment a player needs to counter, do they have the information to choose the right counter, or must they guess?
- From a losing position, is there a realistic path to victory through superior play, or is the outcome effectively locked by early snowball?
- Before nerfing a dominant option, did I confirm whether the dominance is structural (no counter exists at any value) or numeric (a counter exists but loses), and chose the fix accordingly?
