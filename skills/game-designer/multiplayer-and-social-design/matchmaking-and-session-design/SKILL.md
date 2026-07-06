---
name: matchmaking-and-session-design.md
description: Use when the agent is designing matchmaking systems, skill rating algorithms, queue design, party and group formation, session length and structure, leaver handling, reconnect systems, or reviewing whether players find appropriately matched games quickly and whether session pacing sustains engagement without burnout or churn.
---

# Matchmaking and Session Design

Matchmaking is the invisible system that determines whether a multiplayer game feels fair, and session design is the structure that determines whether players stay for one more match or leave for the night, and both are routinely underestimated because their effects are systemic and delayed rather than visible in any single match. The judgment problem is that matchmaking is a multi-objective optimization with no single correct solution: it must minimize queue time, maximize match fairness, respect party composition, account for connection quality, and populate a queue whose size changes by orders of magnitude across the day, and every improvement to one objective tends to worsen another. Designers miss the tradeoffs because the metrics that are easy to measure (queue time) are not the metrics that determine retention (perceived fairness and match quality), and because the player's experience of a match depends on factors the matchmaking cannot see (mood, recent streak, who they are partied with). The harm is severe: poor matchmaking produces streaks of one-sided matches that feel unfair regardless of the rating math, drive churn at the exact moments retention is fragile, and in ranked play generate widespread perceptions that the system is rigged. Poor session design produces games that are either too short to feel meaningful or too long to fit into a player's available time, and both failures manifest as declining session counts that the team struggles to attribute to their root cause. Agents tend to err by optimizing for the measurable (queue time) at the expense of the meaningful (match quality), by treating skill rating as a solved problem rather than a model with assumptions, or by designing sessions for the enthusiast while the casual majority churns. The freedom here is real — many viable matchmaking philosophies exist — but the obligation is to make the tradeoffs explicit, to measure the player-facing outcomes rather than only the system-facing ones, and to recognize that a fast queue into a bad match is worse than a slow queue into a good one.

## Core Rules

### Make the Tradeoff Between Queue Time and Match Quality Explicit

Every matchmaking system balances queue time against match quality, and the correct balance is a deliberate decision, not an emergent property of the algorithm. A system that prioritizes fast queues will frequently produce mismatched games that feel unfair; a system that prioritizes tight skill bands will produce long queues that feel tedious. The discipline is to decide, for each queue and population level, what queue time is acceptable at what match-quality cost, and to communicate that decision to the team rather than letting the algorithm's defaults determine it. The decision criterion: at peak population and at low population, what is the target queue time, and what match-quality sacrifice is the system allowed to make to hit it? When the tradeoff is unspecified, the system will silently optimize for queue time because it is the metric that surfaces first, and match quality will erode without anyone choosing it.

### Measure Perceived Fairness, Not Just Rating Convergence

The metric that determines whether players stay is not the skill-rating difference between teams but the player's perception that the match was fair, and the two diverge systematically. A match with a tight rating spread can still feel unfair if it is one-sided, if the player is on a losing streak, or if the match's decisive moment felt arbitrary. The discipline is to instrument player-facing outcomes — win-rate distribution, match-side balance, streak frequency, post-match survey or behavior — and to treat a population whose perceived fairness is declining as a problem even if the rating math looks correct. The decision criterion: across the player base, is the win rate converging toward the intended distribution, and are streaks of one-sided matches within tolerable bounds? When rating convergence looks good but players report or behave as though matches are unfair, the model's assumptions are wrong even if its math is correct.

### Design Session Length to Match Player Availability and Game Rhythm

Session length is a design decision with retention consequences: a match that is too short feels inconsequential and fails to build engagement; a match that is too long excludes players with limited time and produces dropouts when real life interrupts. The discipline is to identify the target player's available session window (the fifteen-minute break, the hour-long evening) and to design the core session unit to fit within it with margin, including queue time, matchmaking, and post-match overhead. The decision criterion: can a player complete a satisfying session unit within their typical available window, and does the session length match the cognitive and emotional rhythm the game intends (the quick competitive burst, the sustained cooperative arc)? When the session is longer than the player's window, leavers and dropouts are a design consequence, not a player failure.

### Handle Leavers and Reconnects as a System, Not as Punishment

Players leave matches for many reasons — real-life interruption, frustration, poor connection, genuine disconnection — and the system's response determines whether a leaver becomes a permanent churn or a returning player. The discipline is to distinguish causes where possible (intentional quit versus disconnect), to provide reconnect paths that restore the player to their match, to backfill positions where the design allows, and to apply penalties that deter habitual leaving without punishing the player who had a genuine interruption. The decision criterion: when a player disconnects, can they rejoin, is their team made whole, and is the penalty proportionate to whether the leave was voluntary? Punitive-only systems that ban or heavily penalize all leavers treat genuine disconnections as misconduct and convert recoverable situations into permanent churn.

### Account for Party and Premade Composition in Match Quality

Players in voice-coordinated parties perform above their individual ratings, and a match that treats a five-player premade as five independent players of equivalent rating will systematically mismatch against teams of solo players. The discipline is to apply a party-size or coordination adjustment to the effective rating of premades, to match premades against premades where population allows, and to recognize that the solo-player experience against coordinated teams is a primary driver of perceived unfairness. The decision criterion: does the system adjust for the performance advantage of coordination, and does it avoid systematically pitting solo players against full premades? When solo players repeatedly face coordinated teams, the resulting one-sided matches are a matchmaking design failure, not a skill discrepancy.

### Design Ranked and Casual Queues With Different Objectives

Ranked and casual play serve different player needs and require different matchmaking philosophies. Ranked play prioritizes fairness and rating integrity: tight skill bands, penalties for leaving, no backfill that would compromise the rating, and a visible skill ladder that players invest in. Casual play prioritizes speed and accessibility: looser skill bands, faster queues, backfill to keep matches full, and no rating stakes that punish experimentation. The trap is applying the same philosophy to both, which either makes casual play too rigid or compromises ranked integrity. The decision criterion: for each queue, is the objective (fairness and stakes, or speed and accessibility) clearly defined, and does the matchmaking serve that objective without importing the other queue's constraints?

## Common Traps

### Optimizing Queue Time Until Match Quality Collapses

Under pressure to reduce queue-time metrics, the matchmaking widens its skill band progressively until matches populate quickly but are systematically mismatched. The trap is that queue time is the metric that surfaces in dashboards and gets optimized, while match-quality erosion appears later and in different metrics (retention, win-rate skew, complaint volume). The false signal is that "queues are fast." The harm is a player base that gets into games quickly but experiences them as unfair, producing churn that the fast-queue metric cannot explain and that the team attributes to other causes.

### Treating Skill Rating as Ground Truth Rather Than a Model

The rating algorithm is treated as an accurate measurement of skill, when it is a statistical model with assumptions that fail at the edges: new players whose rating is uncertain, smurfs whose rating is wrong, players whose skill varies by role or time of day, and populations where the rating distribution is skewed. The trap is that matchmaking decisions are defended with "the math is correct" while the model's assumptions are violated by the actual population. The false signal is that the rating system is "proven" in other games. The harm is systematic mismatches at the edges of the distribution that the team cannot diagnose because they trust the model over the player experience.

### Streaks of One-Sided Matches Driving Perceived Unfairness

The rating spread in a match is tight, but several matches in a row resolve one-sidedly — due to snowball mechanics, early advantages, or simple variance — and the player experiences a streak of stomps or beatdowns that feels rigged regardless of the underlying fairness. The trap is that each individual match is defensible on rating grounds while the sequence is destroying the player's perception of the system. The false signal is that per-match fairness metrics are within bounds. The harm is churn driven by the streak experience, which the per-match metrics are blind to and which the team fails to connect to the matchmaking.

### Sessions Longer Than the Player's Available Window

The match or session unit is designed for the enthusiast with unlimited time, but the majority of players have bounded windows — a commute, a lunch break, a limited evening — and sessions that exceed the window produce dropouts, leavers, and abandoned matches. The trap is that the session length feels right to the team, who play in long sessions, while the casual majority cannot complete one. The false signal is that the session is "the right length for the game." The harm is leaver rates that the team attributes to player behavior when they are a design consequence, and churn from players who cannot fit the game into their lives.

### Punitive-Only Leaver Systems That Convert Disconnects to Churn

A player disconnects due to a network drop or real-life interruption, and the system applies a leaving penalty — a timeout, a rank deduction, a ban — that treats the involuntary leave as misconduct. The trap is that the penalty deters habitual leavers but also punishes and alienates players who had a genuine interruption, converting a recoverable disconnect into a permanent departure. The false signal is that "leaver rates declined" after the penalty. The harm is that the decline includes players who would have returned but were pushed away, and the system's rigidity manufactures churn from situations that reconnect and backfill could have resolved.

### Solo Players Systematically Matched Against Premades

The matchmaking treats parties as the sum of their members' ratings without adjusting for the coordination advantage, and solo players are frequently placed against voice-coordinated teams that outperform their nominal rating. The trap is that the rating math looks balanced while the actual match is one-sided, because coordination is an unmodeled variable. The false signal is that "ratings are equal." The harm is that solo-queue players — often the majority of the population — experience systematic unfairness, conclude the game is unplayable solo, and churn, hollowing out the player base that premades depend on.

## Self-Check

- Have I made the queue-time-versus-match-quality tradeoff explicit for each queue at peak and low population, rather than letting the algorithm's defaults determine it?
- Am I measuring player-perceived fairness (win-rate distribution, streak frequency, post-match behavior) in addition to rating convergence, and treating declining perceived fairness as a problem even when the math looks correct?
- Does the core session unit fit within the target player's available time window including queue and overhead, and does its length match the game's intended cognitive and emotional rhythm?
- Does the leaver system distinguish voluntary quits from disconnects, provide reconnect and backfill paths, and apply penalties proportionate to cause rather than punishing all leavers uniformly?
- Does the matchmaking adjust for the coordination advantage of premade parties and avoid systematically pitting solo players against coordinated teams?
- Are ranked and casual queues designed with distinct objectives — fairness and stakes versus speed and accessibility — without importing one queue's constraints into the other?
- Have I recognized that the skill-rating system is a model with assumptions, and am I validating those assumptions against the actual population rather than defending mismatches with "the math is correct"?
