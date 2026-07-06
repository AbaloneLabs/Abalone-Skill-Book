---
name: progression-curve-and-pacing.md
description: Use when the agent is shaping a progression curve, deciding the rate at which players unlock abilities and content, managing the tension between rapid early rewards and sustained long-term engagement, or evaluating whether a progression pace causes burnout, boredom, or churn.
---

# Progression Curve and Pacing

A progression curve is the schedule by which a game doles out new capabilities, content, and rewards over time, and it governs whether a player feels a constant forward pull or a grinding slog. The judgment problem is that the curve must be fast enough to hook in the first session and slow enough to sustain engagement for the game's intended lifespan, and these pull in opposite directions — a curve tuned for instant gratification exhausts its rewards early and leaves nothing to look forward to, while a curve tuned for longevity feels stingy in the session that matters most, the first one. Agents tend to miss this because they tune the curve by feel in isolation rather than as a whole-system property, and because the metrics that signal a healthy curve (session length, retention) lag the design decisions that produce them by weeks. The harm is a game that either front-loads all its satisfaction and dies after three days, or starves its players and dies in the first hour. This skill covers how to shape a curve across timescales, manage reward density, and detect pacing failure from behavior. The agent has latitude in the specific rewards and rates, but the obligation to design the curve as a deliberate whole is not optional.

## Core Rules

### Design the Curve Across Three Timescales Simultaneously

A progression curve operates on the moment-to-moment timescale (rewards within a session), the session-to-session timescale (what changes between play sessions), and the long-arc timescale (the trajectory across the full game lifespan). The decision rule: shape all three explicitly and ensure they reinforce rather than contradict each other, because a curve that is satisfying moment-to-moment but offers nothing session-to-session produces a game that is fun for one night and forgotten. Each timescale must have its own reward cadence, and the player must feel forward motion on all three.

### Front-Load Enough to Hook, Then Transition to Sustain

The first session must deliver enough novelty and reward to hook, but the hook rate cannot sustain the full game without exhausting the reward inventory. The decision rule: design an explicit transition point where the curve shifts from rapid early rewards to a slower sustaining pace, and ensure the transition does not feel like a wall. The common failure is a curve that is either uniformly fast (exhausts rewards, nothing left to earn) or uniformly slow (never hooks). The art is the shaped transition, not a single rate.

### Match Reward Density to the Player's Attention Budget

Reward density — how often something meaningful happens — must match how much attention the player is paying. Early, when the player is learning and engaged, dense rewards reinforce learning; later, when the player is competent and the loop is familiar, the same density becomes noise and the rewards must become larger, rarer, or more varied to register. The decision rule: plan reward density to decrease in frequency but increase in significance over the curve, so that the diminishing novelty of small rewards is offset by the growing significance of larger ones.

### Avoid the Grind Wall and the Reward Cliff

Two structural failures define a bad curve. A grind wall is a sudden spike in required effort with no proportional reward increase, which players experience as a demand to pay or quit. A reward cliff is a sudden drop in reward rate where the player feels the game stopped giving, often at endgame when the designed content ends and only repetition remains. The decision rule: scan the curve for both failure modes and smooth them — flatten walls by spacing requirements, fill cliffs by adding aspirational long-term goals — because each produces churn at a predictable point that telemetry will confirm.

### Use Variable Reward Scheduling for Long-Term Engagement

Predictable fixed rewards satiate; variable rewards (in timing, magnitude, or type) sustain engagement far longer because they exploit the same scheduling effects that govern attention. The decision rule: for the long-arc curve, introduce variability in reward timing and content so the player cannot fully predict when the next meaningful payoff arrives, while keeping the average rate fair. Purely deterministic curves produce a flat engagement profile; well-tuned variability produces the sustained pull that drives long-term retention.

### Detect Pacing Failure From Behavior, Not From Player Statements

Players are poor at reporting pacing problems directly — they say "it got boring" without identifying that the reward cliff at hour eight caused it. The decision rule: instrument the curve with behavioral telemetry (session length over time, drop-off points, time-to-first-meaningful-reward, progression velocity) and read pacing health from where players stall or quit, then map those points to curve features. Behavioral drop-off localizes the pacing defect far more precisely than sentiment, which is diffuse.

## Common Traps

### The Uniform Curve That Is Either Too Fast or Too Slow

The team applies a single reward rate across the whole game, either because tuning one number is simple or because consistency feels fair, and the result is a curve that fails at one end. The trap is that a uniform rate is easy to defend and easy to tune. The false signal is that the curve is clean and predictable. The harm is that a uniform fast rate exhausts the reward inventory early and leaves the late game empty, while a uniform slow rate never hooks and the player quits before the sustaining pace can engage them, because the curve needed to be shaped — fast then slow — not flat.

### The Grind Wall Inserted to Extend Playtime

Late in development the team realizes the game is too short, and rather than adding content, they inflate the requirements at a progression gate to stretch playtime, creating a grind wall. The trap is that inflating numbers is fast and cheap compared to building content. The false signal is that the reported playtime increases. The harm is that the grind wall is the single most reliable churn driver in progression design, players experience it as a cynical demand for their time or money, review sentiment sours precisely at the wall, and the extended playtime comes at the cost of the goodwill that would have driven word of mouth.

### The Reward Cliff at Endgame

The designed progression ends at the level cap or final content, and beyond it there is nothing to earn, so players who reach endgame face a reward cliff and quit. The trap is that the designed curve was the whole plan and endgame was an afterthought. The false signal is that the curve is complete up to the cap. The harm is that the most engaged players — the ones who reached endgame — are exactly the players the game should retain as evangelists and live-ops participants, and the reward cliff discards them, converting the most valuable cohort into churn at the moment they were most invested.

### Tuning the Curve by Internal Feel Without Behavioral Data

The team tunes the progression rate based on how it feels during internal play, where the testers are paid to continue regardless of reward, and ships a curve that feels fine to them but churns real players. The trap is that internal feel is the available signal during development. The false signal is that internal testers complete the progression without complaint. The harm is that internal testers tolerate pacing that real players will not, because they are not paying with their own limited leisure time, so the curve that passed internal review produces drop-off in the wild that the team did not predict, because they tuned to the wrong audience's patience.

### Copying a Successful Game's Curve Without Its Context

The team copies the progression curve of a hit game — its level cap, its reward cadence, its gate structure — assuming the curve is what made it successful, without matching the content volume, audience, and loop that the curve was designed for. The trap is that a proven curve feels safe to replicate. The false signal is that the copied curve is "industry standard." The harm is that a curve is tuned to a specific content inventory and audience; transplanted without its context, it produces walls and cliffs against the team's actual content, because the curve that fit a game with a thousand hours of content produces starvation in a game with twenty.

## Self-Check

- Have I shaped the progression curve explicitly across moment-to-moment, session-to-session, and long-arc timescales, rather than tuning a single rate?
- Is there a deliberate transition from hook-rate early rewards to a sustaining pace, designed so it does not feel like a wall?
- Does reward density decrease in frequency but increase in significance over the curve, matching the player's shifting attention?
- Have I scanned the curve for grind walls (effort spikes without reward) and reward cliffs (sudden reward drops, especially at endgame) and smoothed both?
- Does the long-arc curve use variable reward scheduling to sustain engagement, rather than purely deterministic payoffs?
- Am I reading pacing health from behavioral telemetry (drop-off points, progression velocity) rather than from player sentiment statements?
- Did I tune the curve against my actual content inventory and audience, not by copying a hit game's curve without its context?
