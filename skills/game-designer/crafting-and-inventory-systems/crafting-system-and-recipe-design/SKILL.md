---
name: crafting-system-and-recipe-design.md
description: Use when the agent is designing crafting systems, building recipe trees, balancing crafting inputs and outputs, or evaluating whether crafting deepens the gameplay loop or produces busywork, inventory bloat, and a disconnected subsystem that players ignore or resent.
---

# Crafting System and Recipe Design

Crafting systems promise depth — the player gathers, combines, and creates, engaging with the game's materials and economy — but they are also a primary source of busywork, inventory bloat, and disconnected subsystems that players ignore or resent. The judgment problem is that crafting must integrate with the core loop (not exist as a separate minigame), must produce outputs worth the input effort (not busywork recipes), and must be legible (so the player understands the system rather than wrestling with opaque recipes). Agents tend to miss this because crafting that looks deep on a recipe tree can be tedious in play (too many intermediate steps, trivial outputs), and because the temptation to add crafting because it is expected can produce a system disconnected from what the game needs. The harm is crafting that is ignored (because the outputs are not worth it), resented (because it is busywork), or bloating (because it fills the inventory with intermediate materials). This skill covers how to design crafting that deepens the loop, produces worthwhile outputs, and avoids the busywork and bloat traps. The agent has latitude in the recipes, but the obligation to make crafting serve the game is not optional.

## Core Rules

### Integrate Crafting With the Core Loop, Not as a Separate Minigame

Crafting must integrate with the core gameplay loop — producing outputs the loop uses, consuming inputs the loop generates, reinforcing the game's core activities — rather than existing as a separate minigame disconnected from the main play, because disconnected crafting feels like a different game the player must manage alongside the core. The decision rule: connect crafting's inputs to core activities (gathering from exploration, combat drops) and outputs to core needs (gear, consumables the loop uses), and avoid crafting that exists parallel to the loop. Disconnected crafting is ignored or resented, because it did not serve the core play.

### Ensure Crafting Outputs Are Worth the Input Effort

Each crafting recipe must produce an output worth the effort of gathering inputs and combining them — the crafted item should be valuable, useful, or unavailable otherwise — or the recipe is busywork that the player will ignore or resent. The decision rule: for each recipe, confirm the output's value justifies the input effort, and cut recipes whose outputs are trivially available or not worth the gathering. Recipes whose outputs are not worth the effort are busywork, because the player's time was not respected.

### Keep Recipe Trees Shallow Enough to Avoid Intermediate Bloat

Recipe trees with many intermediate steps — raw material processed into components processed into sub-assemblies processed into the final item — produce inventory bloat (stacks of intermediates) and tedium (many steps for one output), and should be kept shallow enough that the tree serves depth without bloat. The decision rule: limit recipe tree depth, avoid unnecessary intermediates, and ensure each step adds meaningful value rather than just processing. Deep intermediate trees bloat the inventory and tedious the player, because the steps added processing without value.

### Make the Crafting System Legible and Predictable

The crafting system must be legible — the player understands what inputs produce what outputs, where to find inputs, and what the results will be — through clear recipes, discoverable information, and intuitive combination, so crafting is about decision rather than guesswork. The decision rule: make recipes clear and discoverable, indicate input sources, preview outputs, and avoid opaque systems the player must wrestle with. Opaque crafting produces frustration, because the player could not understand or predict the system.

### Provide Meaningful Crafting Choices, Not Just Consumption

Crafting should offer meaningful choices — what to craft, what to prioritize, what to invest scarce materials in — rather than being pure consumption (gather everything, craft everything), so the crafting engages decision-making rather than just checklisting. The decision rule: introduce scarcity, tradeoffs, or branching recipes that force the player to choose, and avoid crafting where everything can be made without choice. Choiceless crafting is checklisting, because the player never decided, they just consumed.

### Avoid Crafting That Undermines Other Systems

Crafting must not undermine other systems — crafting that produces better gear than found loot undermines exploration, crafting that produces infinite consumables undermines economy — and the crafting outputs must be balanced against the other sources of those outputs. The decision rule: balance crafted outputs against found, purchased, and rewarded equivalents, and avoid crafting that makes other systems irrelevant. Crafting that undermines other systems makes them pointless, because the player crafts instead of engaging with the undermined system.

## Common Traps

### Disconnected Crafting as a Separate Minigame

The team designs crafting as a separate system disconnected from the core loop, with its own inputs and outputs that the core does not use or generate, and the crafting feels like a different game. The trap is that crafting is a self-contained system. The false signal is that the game has crafting depth. The harm is that the crafting does not reinforce the core loop, the player experiences it as a parallel chore, the engagement that integrated crafting would provide is absent, and the crafting is ignored or resented, because it was not connected to the core play.

### Recipes Whose Outputs Are Not Worth the Effort

The team designs recipes whose outputs are trivially available (buyable for cheap, commonly found) or not worth the gathering effort, and the player ignores the crafting because the outputs are not worth it. The trap is that the recipes add depth on paper. The false signal is that the crafting tree is large. The harm is that the player sees no reason to craft, the recipes are ignored, the gathering and crafting investment is wasted, and the crafting system exists without being used, because the outputs did not justify the inputs.

### Deep Intermediate Trees That Bloat and Tedious

The team designs recipe trees with many intermediate steps — raw to component to sub-assembly to final — and the intermediates bloat the inventory and the steps tedious the player. The trap is that deep trees feel sophisticated. The false signal is that the crafting has depth. The harm is that the inventory fills with intermediate stacks, the player spends time processing rather than playing, the final output required many steps of tedium, and the crafting that should have been engaging becomes a chore, because the depth was intermediate processing rather than meaningful choice.

### Opaque Crafting That Produces Frustration

The team designs a crafting system that is opaque — hidden recipes, unclear input sources, unpredictable outputs — and the player wrestles with the system rather than engaging with it. The trap is that discovery-based crafting feels exploratory. The false signal is that the crafting rewards experimentation. The harm is that the player cannot understand or predict the system, crafting becomes frustrating guesswork, the experimentation that should be enjoyable becomes tedious trial-and-error, and the player avoids the crafting, because it was not legible.

### Choiceless Crafting as Checklisting

The team designs crafting where everything can be made without meaningful choice — materials are abundant, recipes are linear, no tradeoffs — and the crafting is pure consumption checklisting rather than decision-making. The trap is that abundant crafting feels rewarding. The false signal is that the player can craft a lot. The harm is that the crafting involves no decisions, the player just gathers and crafts everything by checklist, the engagement that choices would provide is absent, and the crafting is a chore rather than a system, because there was nothing to decide.

### Crafting Undermining Other Systems

The team balances crafting such that crafted outputs are better than found loot or cheaper than purchased goods, and the crafting undermines exploration or economy. The trap is that powerful crafting feels rewarding. The false signal is that crafting is valuable. The harm is that the player crafts instead of exploring (undermining exploration) or instead of engaging the economy (undermining economy), the systems that crafting outcompetes become pointless, the game's variety collapses to crafting, and the undermined systems are wasted, because the crafting was not balanced against other sources.

## Self-Check

- Is crafting integrated with the core loop, consuming loop inputs and producing loop outputs, not a separate minigame?
- Does each recipe produce an output worth the gathering and crafting effort, without trivially available outputs?
- Are recipe trees shallow enough to avoid intermediate inventory bloat and processing tedium?
- Is the crafting system legible, with clear recipes, indicated sources, and predictable outputs?
- Does crafting offer meaningful choices (scarcity, tradeoffs, branches) rather than pure consumption checklisting?
- Is crafting balanced against found, purchased, and rewarded sources so it does not undermine other systems?
- Did I confirm crafting deepens the loop rather than existing as busywork the player ignores or resents?
