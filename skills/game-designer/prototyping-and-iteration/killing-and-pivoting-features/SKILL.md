---
name: killing-and-pivoting-features.md
description: Use when the agent is deciding whether to cut a feature, pivot a system, scope down a game, evaluate sunk cost versus future value, manage feature creep, or review whether a struggling feature should be fixed, reworked, or removed before it sinks the project.
---

# Killing and Pivoting Features

Killing a feature is one of the hardest judgments in game development, because by the time the question arises, the feature has sponsors, sunk cost, and emotional investment, and the default outcome is that it survives through sheer inertia and ships as dead weight. The judgment problem is that the case for keeping a feature is always concrete and visible (the work already done, the person who owns it, the slide where it was promised), while the case for killing it is abstract and invisible (the opportunity cost, the integration burden, the focus it drains from the core). Designers miss this because the cost of a bad feature is not the cost of building it; it is the ongoing cost of carrying it — testing it, fixing it, documenting it, explaining it to players, and dividing the team's attention — for the entire life of the product. The harm of failing to kill is bloated, unfocused games where the core is starved because the periphery was protected, and where the team's energy is spread so thin that nothing reaches its potential. The agent has freedom in what it cuts and how it pivots, but it must treat every feature as carrying an ongoing tax, evaluate features against the core fantasy rather than against their sunk cost, and recognize that the willingness to kill is what separates a focused game from a bloated one.

## Core Rules

### Evaluate Features Against the Core Fantasy, Not Against Their Sunk Cost

The question is never "how much have we spent on this feature"; it is "does this feature serve the core experience the game is trying to deliver." Sunk cost is irrelevant to the decision because it is gone regardless of what you choose, and weighting it keeps dead features alive. The rule is to define the game's core fantasy and pillars explicitly, and to test every struggling feature against them: if the feature does not advance the core, it is a candidate for cutting regardless of investment. Decision criterion: if this feature did not yet exist, would you build it now, given the core pillars and remaining budget? If no, the fact that it half-exists does not change the answer — it only makes the carrying cost real sooner.

### Count the Ongoing Tax, Not Just the Build Cost

A feature's cost is not the cost to build it; it is the cost to build it plus the cost to maintain, integrate, test, balance, document, support, and explain it for the entire life of the game. A mediocre feature that took a week to build can consume months of ongoing attention across art, QA, design, and community. The rule is to estimate the lifetime carrying cost of a feature, not just its initial cost, when deciding whether to keep it. Decision criterion: over the next year, how many person-weeks will this feature consume in maintenance, bug fixes, balancing, and player support? If the ongoing tax exceeds the value the feature adds to the core, the feature is a net loss every month it survives.

### Distinguish "Needs Fixing" From "Fundamentally Wrong"

Some struggling features are good ideas with implementation problems (fixable); others are ideas that cannot be made good at any investment (not fixable). The trap is to keep fixing the second kind under the belief that one more pass will work. The rule is to ask, for a struggling feature, whether any version of it — any tuning, any rework, any budget — would make it a meaningful contribution to the core. Decision criterion: have you tried a genuinely wide range of approaches, and does the feature remain weak across all of them? If so, the problem is the feature's premise, not its execution, and no amount of fixing will save it; cut it.

### Cut Early, When the Cost of Cutting Is Low

The cost of killing a feature rises monotonically as it integrates with other systems, acquires content, appears in marketing, and gathers stakeholders. A feature cut at the prototype stage costs almost nothing; the same feature cut in beta costs enormous rework, broken dependencies, and morale damage. The rule is to make kill decisions as early as the evidence allows, and to create explicit decision points — after prototype, after vertical slice, after first playable — where features are evaluated for survival. Decision criterion: at each milestone, is there a forced ranking or kill review, or do features survive by default? If survival is the default, dead features accumulate, and each one becomes more expensive to cut later.

### Prefer Cutting to Adding When the Game Is Unfocused

When a game feels bloated, confusing, or lacking identity, the instinct is often to add a unifying system or a new feature to tie things together. The correct response is almost always the opposite: cut until the core is clear, because bloat is a surplus problem, not a deficit problem. The rule is to diagnose unfocused games as having too many features competing for attention, and to resolve by subtraction toward the core, not by addition of more competing systems. Decision criterion: does the game have a clear, singular fantasy that every surviving feature serves? If not, the path to clarity is cutting the features that dilute it, not adding a feature to justify them.

### Manage the Human and Political Cost of Killing Honestly

Features have owners, and killing a feature kills someone's work, which produces real morale and political cost that teams often avoid by keeping features alive. The rule is to make kill decisions against pre-agreed criteria rather than against individuals, to be transparent about the reasoning, and to redirect the freed capacity toward the core rather than framing the cut as failure. Decision criterion: was the kill criterion agreed before the feature was built, so that the decision reads as process rather than personal? Features killed by shifting, post-hoc criteria erode trust and make future teams hide struggling features rather than surface them.

### Treat a Pivot as a New Bet, Not a Rescue of the Old One

A pivot — reworking a feature into something different — is often proposed as a way to save the sunk cost while changing direction. The judgment is that a genuine pivot is a new feature that must justify itself on its own merits, not a rescue that inherits the old feature's budget by default. The rule is to evaluate the pivoted feature as if it were a brand-new proposal: would you build it from scratch, given what you now know? Decision criterion: if the pivot were a new pitch today, would it be approved? If not, the pivot is sentimental rescue of sunk cost, and the team should cut cleanly and spend the budget on something that earns its place.

## Common Traps

### Sunk Cost Keeping a Dead Feature Alive

A feature has consumed months of work, so the team keeps it because cutting it would "waste" the investment, never recognizing that the investment is already gone and the only question is the future carrying cost. The trap is that sunk cost feels like a reason when it is only a feeling. The false signal is the visible, countable hours already spent; the harm is that the feature ships, drains ongoing attention, dilutes the core, and the original investment is wasted anyway — plus all the additional waste of carrying it.

### The "One More Pass" Loop on an Unfixable Feature

A feature is fundamentally weak, but the team believes one more tuning pass, one more art pass, one more rework will fix it, and this belief renews indefinitely because each pass produces marginal, hope-sustaining improvement. The trap is that incremental improvement feels like progress toward viability. The false signal is that the feature got slightly better; the harm is months of effort spent rescuing a premise that cannot be saved, while features that could have been great go unbuilt for lack of budget.

### Adding Features to Fix Bloat

The game feels unfocused, and the team's response is to add a unifying meta-system, a new mode, or a connective feature, when the actual problem is too many features competing. The trap is that addition feels like problem-solving. The false signal is that the new feature "ties it together"; the harm is that the bloat deepens, the core becomes even more diluted, and the team has spent more budget moving further from a focused game.

### Killing by Default Erosion Instead of Deliberate Decision

Rather than making an explicit kill decision, the team lets a struggling feature slowly lose resources, content, and attention until it ships half-baked, then blames the feature for failing. The trap is that avoiding the hard decision feels safer than making it. The false signal is that no explicit cut was made, so no one is responsible; the harm is a feature that consumed full build cost, ships in a state guaranteed to fail, disappoints players, and erodes trust in the team's judgment — worse than either cutting it or finishing it.

### The Sentimental Pivot That Inherits Unearned Budget

A feature is failing, and rather than cut it, the team pivots it into a different feature, but evaluates the pivot as a rescue rather than a new proposal, so it inherits budget and slot allocation it would never have earned on its own merits. The trap is that the pivot feels decisive and forward-looking. The false signal is that direction is changing; the harm is that sunk cost is laundered into a new feature that does not deserve its place, consuming the budget that a genuinely worthy new feature would have used.

### Protecting a Feature Because It Has an Owner

A feature survives not because it serves the core but because a respected team member built it and cutting it would feel like a rebuke, so the team protects the person by protecting the feature. The trap is that kindness to an individual becomes unkindness to the game. The false signal is team harmony; the harm is that the game carries dead weight, the team learns that struggling features are safe if they have a sponsor, and the core is starved to protect feelings — which ultimately produces a worse game and a worse outcome for everyone, including the person whose feature should have been cut.

## Self-Check

- Did I evaluate the struggling feature against the core fantasy and pillars, and would I build it from scratch today, rather than weighting sunk cost?
- Did I estimate the lifetime carrying cost (maintenance, integration, QA, balancing, support), not just the build cost, when judging whether to keep it?
- Have I determined whether the feature is fixable (implementation problem) or fundamentally wrong (premise problem) by testing a wide range of approaches?
- Did I make the kill decision at the earliest milestone the evidence allowed, rather than letting integration and content raise the cost of cutting?
- If the game feels unfocused, did I resolve it by cutting toward the core rather than adding a unifying feature to justify the bloat?
- Was the kill made against pre-agreed criteria to protect both the process and the feature's owner, rather than as a shifting post-hoc judgment?
- If proposing a pivot, did I evaluate the pivoted feature as a brand-new pitch that must earn its place, rather than as a rescue that inherits the old budget by default?
