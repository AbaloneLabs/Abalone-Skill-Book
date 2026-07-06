---
name: genre-conventions-and-innovation-choices.md
description: Use when the agent is deciding which genre conventions to honor versus subvert, positioning a game within or against an established genre, managing player expectations for a known category, evaluating the risk of innovating on load-bearing mechanics, or deciding whether a novel mechanic is a differentiator or a friction source.
---

# Genre Conventions and Innovation Choices

A genre is a contract: a set of mechanics, controls, pacing expectations, and failure modes that players have learned and use to predict how a new game will behave. Honoring conventions lets players transfer their existing skill, lowering the cost of entry; subverting them creates distinction and identity, but at the cost of friction and the risk of rejecting the very audience the genre attracts. The judgment problem is that conventions and innovations are not good or bad in themselves — every convention was once an innovation — and the designer's task is to decide, for each element, whether to honor, refine, or reject it, with full awareness of what that choice costs. Agents tend to miss this because the two failure modes pull in opposite directions: slavish adherence produces a derivative game with no reason to exist, while reflexive innovation produces a game that fights its own audience. The deeper trap is that innovation is culturally rewarded in design discourse, so agents over-index on novelty and under-weight the cost of making players relearn what they came expecting. The harm is a game that is either invisible or inaccessible, and in both cases the team is surprised because they optimized for the wrong signal. This skill covers how to classify conventions by load-bearing weight, budget innovation toward differentiators rather than friction, and manage the expectation contract with the player. The agent has latitude to position anywhere on the spectrum, but the obligation to make each choice consciously is binding.

## Core Rules

### Classify Each Convention as Load-Bearing, Expected, or Cosmetic

Not all conventions carry equal weight, and the cost of subverting them varies accordingly. Load-bearing conventions are the mechanics that define the genre's core skill and identity — the aiming in a shooter, the build order in an RTS, the party composition in a party RPG — and subverting them rejects the genre itself. Expected conventions are the standard arrangements players assume but that can be reworked without destroying identity — checkpoint structure, save systems, control schemes. Cosmetic conventions are surface presentation — HUD layout, color coding, terminology — that can be changed freely. The decision rule: when considering innovation, target expected and cosmetic conventions for novelty and treat load-bearing conventions as fixed unless you are consciously making a different game. Subverting a load-bearing convention is not innovation within a genre; it is genre migration, and it must be scoped as such.

### Budget Innovation Toward the Differentiator, Not Across Every System

A game cannot innovate everywhere without becoming an unplayable pile of unfamiliar systems, because the player's cognitive budget for learning new rules is finite. The decision rule: choose one or two systems where the game will be genuinely novel — its differentiators — and make every other system conventional and instantly legible, so the player's learning effort is spent on the innovation, not wasted on relearning conventions that did not need reinventing. A game that innovates on combat, movement, economy, and UI simultaneously overwhelms players and ensures none of the innovations is mastered or appreciated. Concentrated novelty is memorable; diffuse novelty is exhausting.

### Honor the Expectation Contract for Failure and Recovery

Players approach a genre with a model of what failure costs and how recovery works, and violating that model without warning produces acute frustration. The decision rule: if you change the failure or recovery model — permadeath in a genre that normally respawns, checkpoint loss in a genre that normally saves freely — signal it explicitly, early, and repeatedly, and make it a marketed feature rather than a surprise. Unsignaled changes to the failure contract feel like bugs or cruelty, because the player's genre-trained reflexes produce the wrong response and they are punished for following their training. The failure contract is the most emotionally loaded convention; innovate here only with intent and communication.

### Subvert Conventions to Deliver a Specific Fantasy or Theme, Not for Novelty's Sake

Innovation is justified when it serves a design purpose — enabling a fantasy, expressing a theme, solving a real problem with the convention — and is unjustified when it exists only to be different. The decision rule: for each subversion, state the player-facing benefit it produces that the convention could not. If the only answer is "it's more original," the subversion is vanity and the player pays the learning cost for no reward. The strongest subversions are motivated ones: a horror game removes the minimap because the fantasy is disorientation; a heist game removes save-scumming because the fantasy is commitment. Unmotivated subversion reads as contrarianism and trains players to distrust the design.

### Test Innovations Against the Genre's Core Audience, Not General Players

An innovation that delights players unfamiliar with the genre may alienate the genre's core audience, who have refined expectations and value the conventions precisely because they enable high-level play. The decision rule: when validating an innovation, test it with players who are fluent in the genre, because they will detect whether the innovation enhances or breaks the genre's depth. General players will tolerate almost any change in a short session; genre experts will tell you whether your innovation survives a hundred hours. Innovating without consulting the core audience produces a game that newcomers bounce off (because the genre is inherently demanding) and experts reject (because the innovation undermined what they loved).

### Recognize When You Are Making a Different Genre and Re-Scope Accordingly

Sometimes a series of justified subversions collectively transform a game into a different genre than the one it was positioned in, and continuing to develop it as the original genre produces a confused product. The decision rule: if your load-bearing innovations have changed the core skill the game demands, honestly reclassify the game and reset audience expectations, marketing, and onboarding to match the new genre. Clinging to the original genre label while shipping a different game's mechanics guarantees that every player measures the game against expectations it never intended to meet, and the game is reviewed for what it is not.

## Common Traps

### Innovation Theater on Non-Load-Bearing Systems

The team innovates aggressively on UI, save systems, and menus — the cheap-to-change systems — while leaving the core loop conventionally identical, then markets the game as innovative. The trap is that surface innovation is visible in screenshots and demos and earns press coverage. The false signal is that reviewers call the game "fresh." The harm is that the core experience is derivative, players who came for innovation find none where it matters, and the game is remembered as gimmicky rather than genuinely novel, because the innovation never touched the loop where players spend their time.

### Subverting a Load-Bearing Convention Without Scope Adjustment

The team removes or reverses a genre-defining mechanic — real-time combat in a turn-based series, permadeath in a casual genre — but continues to scope, pace, and onboard as if the convention were still present. The trap is that the change was easy to implement and felt bold in prototype. The false signal is that the mechanic works in isolation. The harm is that the entire surrounding design — level pacing, economy, tutorial — was built for the old convention and now mismatches the new one, producing a game whose parts were designed for different rules and never reconcile.

### Novelty That Punishes Genre Fluency

An innovation changes a convention that genre experts rely on for high-level play — altering the input buffer, removing animation canceling, changing the resource economy — and the change makes the experts' hard-won skill counterproductive. The trap is that the change is invisible to new players and thus passes general playtests. The false signal is that newcomers perform fine. The harm is that the genre's most dedicated advocates, whose word-of-mouth sustains the game's long tail, find their mastery invalidated and depart, and the game loses the audience that would have carried it for years.

### Conventionalism Masquerading as Polish

The team, afraid of risk, ships every system as the genre standard, then justifies the absence of innovation as "respecting the genre" or "polish over novelty." The trap is that conventional games are easy to build and easy to defend in review. The false signal is high production values and smooth playtests. The harm is that the game offers no reason to exist alongside the established leaders of the genre, who already execute the conventions better, and the game is ignored not because it is bad but because it is redundant — a fate worse than failure because it teaches the team the wrong lesson about risk.

### Ignoring the Audience's Genre Literacy Assumption

The game assumes players are fluent in the genre and omits onboarding for conventions, but the actual audience includes many players new to the genre who are stranded, or conversely the game over-tutorials conventions the audience already knows, insulting and boring the experts. The trap is that the team calibrated onboarding to their own fluency level. The false signal is that internal testers, who know the genre, find the pacing fine. The harm is that the wrong audience is either overwhelmed or alienated, and the onboarding never serves the players who actually arrive, because the team never asked who those players would be.

## Self-Check

- For each convention I plan to subvert, have I classified it as load-bearing, expected, or cosmetic, and confirmed that load-bearing subversions are scoped as genre migration rather than tweaks?
- Have I concentrated innovation on one or two differentiating systems while keeping the rest conventional and legible, rather than innovating diffusely across every system?
- If I changed the failure or recovery model, did I signal it explicitly and early to players, and market it as a feature rather than letting it surprise them?
- For each subversion, can I state the specific player-facing benefit it delivers, confirming it is motivated by fantasy or theme rather than novelty for its own sake?
- Did I validate the innovations with genre-fluent players who can assess whether the changes enhance or undermine the genre's depth, rather than relying only on general playtesters?
- If my innovations collectively changed the core skill the game demands, have I honestly reclassified the genre and reset marketing and onboarding to match?
- Have I avoided innovation theater on cheap surface systems while leaving the core loop derivative, and confirmed the game offers a reason to exist alongside genre leaders?
