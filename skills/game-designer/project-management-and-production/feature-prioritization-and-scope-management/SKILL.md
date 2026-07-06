---
name: feature-prioritization-and-scope-management.md
description: Use when the agent is prioritizing game features against a fixed budget, cutting scope to hit a ship date, evaluating which mechanics are core versus nice-to-have, managing feature creep, or defending scope decisions to stakeholders and the team.
---

# Feature Prioritization and Scope Management

Scope is the variable that kills more games than any technical failure, and the judgment problem is that every individual feature looks affordable while the aggregate is impossible. The recurring failure is that prioritization is done feature by feature in isolation, each one defended on its own merits, and the team never makes the hard comparison that reveals the game cannot contain all of them. Agents miss this because cutting feels like loss, the features being cut are often someone's passion, and the cost of keeping a feature is invisible until late when integration, polish, and QA reveal that the game is over-scoped by a factor no schedule can absorb. This skill covers how to prioritize against the player experience rather than feature counts, how to cut early enough that cutting is cheap, and how to defend scope decisions with reasoning the team can accept rather than resent. The designer has authority over what is core but must exercise it against real budget and schedule constraints, not aspirational ones.

## Core Rules

### Prioritize Against the Core Player Experience, Not Against a Feature List

The first question of prioritization is not which features are good but which features are load-bearing for the experience the game is trying to deliver. Define the core loop and the emotional promise in a few sentences, and then evaluate every candidate feature by whether it serves that core or competes with it. A feature can be excellent in isolation and still be a cut candidate if it dilutes the core, draws resources from the moments that matter, or adds breadth the player will not value. The discipline is to rank features by their contribution to the core experience, not by their individual quality or by who advocates for them. When two features serve the same player need, one is redundant; when a feature serves no need in the core promise, it is scope debt regardless of how well-designed it is.

### Cut Early, Because the Cost of a Feature Compounds Over the Project

A feature kept in the plan at the start of a project is not the same cost as a feature kept at the end. Early in development, cutting is cheap: no art is final, no code is integrated, no marketing has promised it. Late in development, cutting is expensive but keeping is catastrophic, because the feature has consumed polish time, QA cycles, and integration attention that the core needed. The rule is that the cheapest cut is the one made before the feature is built, and the most expensive cut is the one made during cert when the team is exhausted. This means prioritization must be aggressive and continuous, not a one-time exercise at the start. Treat the feature list as something to shrink every milestone, not something that only grows.

### Make Scope a Visible, Negotiated Constraint, Not an Ambient Guilt

When the game is over-scoped, the team feels it as stress and guilt but rarely sees it as a decision that can be reversed. The rule is that scope must be made explicit: a list of what is in, what is out, and what is at risk, reviewed regularly against the remaining budget. When scope is invisible, each discipline absorbs the overload silently and produces a worse version of everything. When scope is visible, over-commitment becomes a named problem that leadership must resolve by cutting, not a private failure each team member tries to survive. The designer's job is to keep the scope document honest, including the features that are nominally in but realistically will not fit.

### Distinguish Core, Supporting, and Trimmable Layers for Every System

For each system or feature, identify which layer each component occupies. The core layer is what the player must experience for the game to deliver its promise; it cannot be cut without changing what the game is. The supporting layer makes the core work well; it can be reduced but not removed without degrading the experience. The trimmable layer is polish, breadth, and alternate paths; it is the first thing to cut under pressure and the last thing to add when time allows. The value of this layering is that it converts a panicked "cut something" into a structured "cut the trimmable layer of system X first." When every component is treated as equally essential, no structured cut is possible and the team cuts randomly under deadline.

### Evaluate the True Cost of a Feature, Including Hidden Carrying Costs

A feature's cost is not just the time to build it. It includes integration with every system it touches, the QA surface it adds, the localization of its text, the tutorial and onboarding needed to teach it, the accessibility considerations it raises, and the ongoing maintenance through every future patch. The trap is that advocates quote the build cost and hide the carrying cost. The rule is that any feature added to scope must be costed against its full lifetime burden, and the team must explicitly accept that burden. When a feature is cheap to build but expensive to maintain, that asymmetry must be named, because the maintenance cost is what survives into ship and live-ops.

### Defend Cuts With Player-Facing Rationale, Not Just Schedule Pressure

A cut justified only by "we don't have time" breeds resentment and invites the cut feature to creep back, because the implication is that the feature was good and only the schedule failed. A cut justified by "this feature competed with the core loop for the player's attention, and removing it makes the core stronger" is a design decision the team can rally behind. The rule is that scope decisions should be framed in terms of what they do for the player experience, not just what they do for the schedule. This is not spin; it is the honest framing, because a well-prioritized cut genuinely does improve focus. When a cut is purely schedule-driven, say so plainly rather than disguising it as a design choice.

### Build a Buffer for the Unknowns That Will Become Features

No plan survives contact with development, and a portion of the budget must be reserved for the features that emerge as necessary only after prototyping reveals a gap. The rule is to leave explicit slack rather than committing the entire budget to known features, because a fully-committed plan has no room to absorb the discoveries that make the game actually work. The slack is not laziness; it is the resource that lets the team respond to playtest findings without cannibalizing the core. When the plan has no buffer, every new necessity becomes a cut to something already promised, and the team enters a death spiral of traded commitments.

## Common Traps

### Keeping a Feature Because Someone Already Built a Prototype

A programmer or artist builds a prototype of a feature on their own initiative, it works, and the team keeps the feature in scope because the work is already done. The trap is that the sunk cost of the prototype is treated as a reason to commit to the full feature, ignoring that the prototype is a fraction of the real cost of integration, polish, and QA. The false signal is the visible, working prototype, which feels like a de-risked feature. The harm is that the feature absorbs the budget that the core needed, justified by work that has already been spent and cannot be recovered.

### Equating Feature Count With Game Value

The team measures progress by the number of features shipped, and stakeholders reward breadth, so the plan accumulates features to look substantive. The trap is that a game with many shallow features often delivers a weaker experience than a game with few deep ones, but the feature count metric cannot detect the difference. The false signal is the growing list of checked-off features. The harm is that polish is spread thin across everything, no single system reaches excellence, and the shipped game feels broad but unsatisfying.

### Cutting Quality Instead of Cutting Scope

Under deadline pressure, the team keeps every feature but ships each one at reduced quality, telling itself that breadth with rough edges beats a smaller polished game. The trap is that players experience quality far more viscerally than breadth, and a rough feature actively damages the perception of the whole game. The false signal is that nothing was cut, so the plan appears intact. The harm is that the game ships with the same scope but worse reviews, because the compromised quality is what players actually feel, and the breadth they were protecting is invisible next to the jank.

### Deferring the Hard Cut Until It Is Most Expensive

The team knows a feature is at risk but keeps it in the plan on the grounds that things might improve, and the cut is made only when the schedule forces it late in the project. The trap is that late cuts are the most expensive possible cuts: art is final, code is integrated, marketing has shown the feature, and the team is exhausted. The false signal is the hope that the schedule will recover. The harm is that the late cut wastes all the work invested after the feature was already doomed, and the team morale collapses precisely when it is most needed.

### Prioritizing by Who Shouts Loudest

Scope decisions get made by political pressure: the stakeholder who complains most or the discipline lead with the most charisma gets their features protected, while quieter but more central features get cut. The trap is that political prioritization optimizes for internal satisfaction rather than player experience, and the resulting game serves the team's preferences, not the audience's. The false signal is the absence of internal conflict, which feels like alignment. The harm is that the core experience is quietly eroded to protect politically-favored additions, and nobody notices until playtests reveal the game no longer delivers its promise.

## Self-Check

- Is every in-scope feature evaluated against its contribution to the core player experience, with redundant or competing features identified as cut candidates regardless of individual quality?
- Are cuts being made continuously and early, with a shrinking feature list each milestone, rather than deferred until late when they are most expensive?
- Is scope maintained as a visible, reviewed document distinguishing in, out, and at-risk features, rather than existing as ambient stress the team absorbs silently?
- For each system, have I classified components into core, supporting, and trimmable layers so that pressure produces structured cuts rather than random ones?
- Does each feature's recorded cost include integration, QA, localization, onboarding, accessibility, and maintenance, not just the build estimate?
- Are scope cuts framed in player-facing rationale where the cut genuinely improves focus, and stated honestly as schedule-driven where that is the real reason?
- Does the plan contain explicit buffer for features that will emerge as necessary during prototyping, rather than committing the entire budget upfront?
