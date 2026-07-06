---
name: mechanic-scope-and-complexity-budget.md
description: Use when the agent is deciding how many mechanics a game can support, allocating a complexity budget across systems, cutting mechanics that exceed the budget, or evaluating whether adding a new mechanic deepens the core loop or fragments player attention and polish resources.
---

# Mechanic Scope and Complexity Budget

Every game has a finite complexity budget: the total cognitive load a player can absorb and the total tuning surface a team can polish before ship. Each mechanic added consumes some of both, and a game that exceeds its budget becomes confusing to players and impossible to balance. The judgment problem is that adding mechanics always feels like adding value in the moment — each one is fun in isolation, each one was requested by someone — and agents tend to accumulate mechanics without tracking the cumulative cost, producing a game that is broad, shallow, and unfinishable. Agents miss this because the cost of a mechanic is not visible at the moment of addition; it appears later as tuning debt, tutorial bloat, and player overwhelm. The harm is a shipped game where no mechanic is polished because the budget was spread across too many, where players bounce off the complexity in the first session, and where post-launch balance is impossible because every change ripples through an overconnected system. This skill covers how to set and enforce a complexity budget, evaluate mechanic cost honestly, and cut ruthlessly. The agent has latitude in which mechanics to keep, but the obligation to treat complexity as a finite, tracked resource is not optional.

## Core Rules

### Set an Explicit Complexity Budget Before Adding Mechanics

A complexity budget is a stated limit on the number of load-bearing mechanics, the number of inter-system connections, and the total tutorial length the game can demand. The decision rule: before greenlighting mechanics, set these limits explicitly based on the target audience's tolerance and the team's tuning capacity, and treat each new mechanic as a charge against the budget that must be justified. Teams without an explicit budget accumulate mechanics until the game collapses under its own weight, because every individual addition seemed affordable.

### Cost Each Mechanic on Tuning Surface, Not Just Implementation

A mechanic's cost is not its implementation effort; it is the tuning surface it adds — the number of parameters, the number of interactions with other systems, the number of edge cases, and the ongoing balance burden. The decision rule: when proposing a mechanic, estimate its tuning surface honestly, including its interactions with every existing system, and charge that against the budget. A small mechanic that touches many systems can cost more tuning than a large self-contained one, and underestimating this is how teams ship unbalanced games.

### Require Each Mechanic to Deepen the Core Loop or Serve a Rest Beat

A mechanic earns its budget by either deepening the core loop (adding a meaningful choice or expression within it) or providing a necessary rest beat (variety that prevents fatigue). The decision rule: for each mechanic, state which role it plays, and cut any mechanic that does neither. Mechanics that exist because they are cool, or because a competitor has them, or because someone fought for them, consume budget without serving the experience, and their cost is paid by the mechanics that do matter.

### Track Cumulative Tutorial and Onboarding Cost

Each mechanic must be taught, and tutorial cost accumulates nonlinearly because players must also learn how mechanics combine. The decision rule: sum the teaching time for all mechanics, including combination teaching, and if the total exceeds the audience's patience for onboarding, cut mechanics until it fits. A game whose tutorial is two hours long because it must explain fifteen mechanics has exceeded its budget regardless of how good each mechanic is, because most players will quit before the teaching ends.

### Cut Mechanics Late Enough to Inform, Early Enough to Save Polish

There is a window to cut a mechanic: late enough that you have evidence it is not earning its budget, early enough that the saved polish budget can flow to the survivors. The decision rule: schedule a mechanic review at the point where prototypes are playable but before content lock, evaluate each against its budget charge and loop contribution, and cut the underperformers in that window. Cutting too early removes mechanics that would have proven out; cutting too late saves no polish budget and leaves dead systems in the shipped game.

### Enforce the Budget Against Feature Creep From All Sources

Feature creep comes from every direction — stakeholder requests, competitor parity, team excitement, player forum demands — and each source feels legitimate in isolation. The decision rule: route every proposed mechanic through the same budget evaluation, regardless of source, and require it to displace an existing mechanic if the budget is full. Teams that make exceptions for "strategic" or "must-have" additions find the budget meaningless, because the exceptions consume it anyway.

## Common Traps

### Adding Mechanics to Solve Core Loop Problems

When the core loop feels thin, the team adds adjacent mechanics — a crafting system, a base builder, a companion — to create breadth, instead of deepening the loop itself. The trap is that each new mechanic produces a burst of engagement that masks the underlying thinness. The false signal is a playtest spike whenever a new system is introduced. The harm is that the additions consume the complexity budget that should have gone to deepening the core, the shipped game is a constellation of shallow systems, and the core loop that was never finished remains the weak point, now buried under more systems that also need polish.

### Counting Mechanics Individually and Missing Interaction Cost

The team evaluates each proposed mechanic on its own merits and approves a dozen, each individually justified, without summing the interaction cost — the combinatorial tuning surface where each mechanic must balance against each other. The trap is that individual evaluation feels responsible and each approval is defensible. The false signal is that every mechanic passes its solo review. The harm is that the interaction surface grows quadratically, the tuning burden becomes unmanageable, the shipped balance is full of holes because no one could test the full combination space, and post-launch balancing becomes a game of whack-a-mole across an overconnected system.

### Retaining Mechanics Because of Sunk Implementation Cost

A mechanic was expensive to build, so the team keeps it in the shipped game even though playtests show it does not earn its budget, because cutting it feels like wasting the investment. The trap is that sunk cost feels like a real constraint. The false signal is that the mechanic exists and functions. The harm is that the dead mechanic continues to consume tutorial time, tuning effort, and player attention, dilutes the core loop, and confuses players about what the game is about, and the money spent building it is truly wasted because shipping it harms the game more than cutting it would have.

### Tutorial Bloat From Refusing to Cut

The team keeps every mechanic and tries to teach them all, producing a tutorial that is hours long, which the target audience abandons. The trap is that cutting a mechanic feels harder than writing more tutorial. The false signal is that the tutorial is comprehensive. The harm is catastrophic early churn: players who would have loved the core game never reach it because the onboarding demanded to reach it exceeded their patience, and the comprehensive tutorial that was meant to serve the mechanics instead buries them, converting a complexity problem into a retention disaster.

### Assuming More Mechanics Means More Depth

The team equates the number of mechanics with the depth of the game, and ships a broad feature list believing breadth is depth. The trap is that feature counts are easy to market and easy to measure. The false signal is that the game has "a lot to do." The harm is that depth comes from the interaction space within a small set of well-tuned mechanics, not from a large set of shallow ones, and a game with many underdeveloped mechanics is shallower than one with a few polished ones, because the player never reaches the interesting decisions in any of them before moving to the next.

## Self-Check

- Have I set an explicit complexity budget (mechanic count, interaction count, tutorial length) before adding mechanics, and is each new mechanic charged against it?
- When costing a mechanic, did I estimate its tuning surface and interaction burden, not just its implementation effort?
- Does each retained mechanic either deepen the core loop or provide a necessary rest beat, and have I cut those that do neither?
- Have I summed the teaching time for all mechanics including combinations, and confirmed it fits the audience's onboarding patience?
- Did I schedule a mechanic review in the window where prototypes are playable but content is not locked, and cut underperformers then?
- Does every proposed mechanic, regardless of source, pass through the same budget evaluation, displacing an existing mechanic if the budget is full?
- Did I resist retaining mechanics purely for sunk-cost reasons, recognizing that shipping a dead mechanic harms the game more than cutting it?
