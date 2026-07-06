---
name: unlock-and-gating-architecture.md
description: Use when the agent is designing how content and abilities are unlocked, choosing between level gates and skill gates, structuring ability unlocks to teach and empower, or evaluating whether a gating scheme respects player agency or railroads progression and causes frustration.
---

# Unlock and Gating Architecture

Gating is the architecture of what a player may access when, and it is the skeleton on which progression, exploration, and difficulty all hang. Done well, gating teaches mechanics in sequence, paces the reveal of the game's depth, and gives every unlock a moment of empowerment. Done poorly, gating railroads players, withholds tools they feel they need, and turns the game into a queue where the player waits for permission to have fun. The judgment problem is that every gate is a promise that something better lies beyond and also a frustration that the player cannot yet proceed, and agents tend to over-gate (because gates feel like structured design) or under-gate (because freedom feels generous), missing that the right gate type depends on what is being gated and why. Agents miss this because a gate that tests well in a guided playthrough hides the frustration it produces in free play, and because the difference between a teaching gate and a blocking gate is invisible in the data. The harm is a game where players feel managed rather than playing, or where they are overwhelmed by unlocked complexity with no sequence to absorb it. This skill covers how to choose gate types, sequence unlocks for teaching, and preserve agency. The agent has latitude in the gate layout, but the obligation to justify each gate's type and purpose is not optional.

## Core Rules

### Match Gate Type to What Is Being Gated

There are several distinct gate types and they serve different purposes: a level gate (reach level ten to enter this zone) gates by time investment; a skill gate (you need the double-jump to cross this gap) gates by capability and teaches; a knowledge gate (the player must understand a mechanic to proceed) gates by learning; a story gate (progress the narrative to unlock this system) gates by pacing. The decision rule: for each gate, name its type and confirm the type matches the purpose, because a level gate used where a skill gate is appropriate withholds a teaching opportunity, and a skill gate used where freedom is appropriate railroad's the player.

### Sequence Ability Unlocks to Teach, Then Empower, Then Combine

Each ability unlock is a teaching moment, and the sequence determines whether the player learns the game's depth or drowns in it. The decision rule: introduce abilities one at a time, give the player a safe space to learn each before combining, and only later present challenges that require combinations. Unlocking multiple abilities at once overwhelms; unlocking an ability with no immediate use confuses. The ideal unlock teaches the ability in isolation, then lets the player express mastery by combining it with prior tools.

### Prefer Soft Gates Over Hard Gates Where Agency Matters

A hard gate makes progression impossible without the key; a soft gate makes it difficult but possible with skill, allowing the determined player to sequence-break. The decision rule: use hard gates only where sequence-breaking would break the game (skipping a teaching moment, accessing content out of order), and use soft gates elsewhere to respect player agency and reward skill. A game of all hard gates feels like a corridor; a game of all soft gates feels unstructured. The mix is the design.

### Ensure Every Gate Has a Clear, Discoverable Key

A gate without a discoverable key is a dead end, and players interpret undiscoverable keys as bugs or as the game being unfair. The decision rule: for every gate, confirm the key is either obvious, telegraphed by environmental or narrative cues, or reachable by exploration that the game has taught the player to do. Gates whose keys require external knowledge (a wiki, a friend) fail the in-game contract and produce the worst kind of frustration — the player is stuck not because the challenge is hard but because the solution is hidden.

### Avoid Withholding Tools the Player Feels They Need

When a gate withholds a tool the player perceives as necessary for basic competence — a weapon to fight common enemies, a movement option to traverse standard terrain — the player feels artificially crippled and resents the gate. The decision rule: gate tools that expand expression and optionality, not tools that enable basic engagement with the core loop. Withholding an optional advanced combo is a tease; withholding the basic attack is a punishment, and players read the latter as the game refusing to let them play.

### Plan the Gate Map as a Graph, Not a Line

Most games are not linear; they are graphs of interconnected gates, and the graph's shape determines whether the game feels open or railroaded. The decision rule: draw the gate graph explicitly, identify the critical path and the optional branches, and confirm the graph offers meaningful choice rather than a line with cosmetic branches. A gate graph that is secretly a line — where every branch dead-ends back to the main path immediately — gives the illusion of freedom without the substance, and players detect the railroad.

## Common Traps

### Over-Gating Because Structure Feels Like Design

The team adds gates at every transition — level gates, story gates, currency gates — because each gate feels like structured, intentional design, and the result is a game where the player spends more time waiting for permission than playing. The trap is that gates are easy to add and read as progression systems. The false signal is that the progression is well-structured on paper. The harm is that the player experiences the gates as a series of locked doors, agency collapses, and the game feels like a guided tour rather than a space to explore, because the team confused restriction with structure and withheld the freedom that makes progression feel earned rather than granted.

### The Level Gate That Replaces Skill or Exploration

The team gates content behind a level requirement instead of behind a skill check or an exploration discovery, because level gates are easy to implement and require no design. The trap is that a number gate is simpler than designing a skill challenge. The false signal is that the gate functions and players eventually pass it. The harm is that the level gate teaches nothing, rewards only time, and replaces a moment of mastery or discovery with a moment of grinding, so the player crosses the gate no more skilled and no more engaged than before, and the content beyond the gate is consumed without the empowerment a well-designed gate would have provided.

### Withholding a Core Tool to Prolong Engagement

The team withholds a tool the player needs for basic competence — a fast-travel option, a key combat ability — until late in the game, to give the player something to look forward to, and the player spends the early game feeling crippled. The trap is that the withheld tool is a compelling unlock on the roadmap. The false signal is that the unlock is exciting when it finally arrives. The harm is that the early game, which is when retention is decided, is played with a deliberately handicapped kit, players churn before reaching the unlock that was meant to reward them, and the tool that would have made the core loop feel good is reserved for survivors rather than used to create them.

### The Undiscoverable Key That Becomes a Dead End

A gate's key is placed or telegraphed poorly, and players who reach the gate cannot find the key, interpret the dead end as a bug or a design failure, and quit or consult a wiki. The trap is that the key is obvious to the team that placed it. The false signal is that internal testers, who placed the key, find it instantly. The harm is that real players without the team's knowledge hit a wall, the frustration is disproportionate to the challenge because the obstacle is obscurity not difficulty, and the game earns a reputation for being obtuse or buggy, because a gate that cannot be passed from in-game information alone is a broken contract.

### The Cosmetic Branch That Pretends to Offer Choice

The gate graph is drawn with branches that look like player choice, but every branch converges immediately back to the critical path with no meaningful difference, so the choice is illusory. The trap is that a branching graph looks open in a design doc. The false signal is that the game has multiple paths. The harm is that players detect the railroad quickly — the branches lead nowhere distinct — and the agency they were promised is exposed as decoration, which breeds cynicism about the game's respect for the player, because the team offered freedom in the map's shape while denying it in the map's substance.

## Self-Check

- For each gate, have I named its type (level, skill, knowledge, story) and confirmed the type matches the purpose of what is being gated?
- Does the ability unlock sequence introduce tools one at a time, teach each in isolation, and only later require combinations?
- Have I used hard gates only where sequence-breaking would break the game, and soft gates elsewhere to preserve agency and reward skill?
- Is every gate's key discoverable through in-game cues, without requiring external knowledge from a wiki or friend?
- Am I gating tools that expand expression and optionality, rather than withholding tools the player needs for basic engagement with the core loop?
- Have I drawn the gate graph and confirmed it offers meaningful branching choice, not a line with cosmetic branches that converge immediately?
- Did I avoid over-gating transitions such that the player spends more time waiting for permission than playing?
