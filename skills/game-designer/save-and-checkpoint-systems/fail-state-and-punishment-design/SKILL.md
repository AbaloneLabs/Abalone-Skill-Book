---
name: fail-state-and-punishment-design.md
description: Use when the agent is designing failure states and consequences, tuning death penalties and respawn rules, deciding what the player loses on failure, or evaluating whether failure teaches and motivates retry or punishes and discourages, producing churn at difficulty spikes and resentment toward the game.
---

# Fail-State and Punishment Design

Failure is inevitable in most games, and what happens when the player fails — what they lose, what they repeat, how the game frames the failure — shapes whether failure teaches and motivates retry or punishes and discourages. The judgment problem is that failure consequences must teach (so the player learns and improves), must be proportionate (so the cost does not exceed the willingness to retry), and must respect the player's time (so failure does not become a time tax), and agents tend to miss this because punishing failure feels weighty and consequential while producing churn, and because the line between stakes and punishment is a judgment that depends on the player. The harm is players who churn at difficulty spikes because the failure cost exceeded their willingness to retry, and who experience failure as punishment rather than learning. This skill covers how to design fail-states that teach and motivate, calibrate consequences proportionately, and avoid the punishment-that-churns trap. The agent has latitude in the consequences, but the obligation to make failure teach rather than punish is not optional.

## Core Rules

### Make Failure Teach What Went Wrong and What to Do Differently

Failure must teach — through clear feedback on what went wrong, what the player could have done, and how to improve — so the player learns from the failure and can retry with new understanding, rather than failing without learning and repeating the same mistakes. The decision rule: for each failure state, ensure the failure provides actionable feedback (what went wrong, what to try differently), and avoid failures that punish without teaching. Failures without teaching produce repeated mistakes, because the player did not learn what to change.

### Calibrate Failure Consequences to Not Exceed Willingness to Retry

Failure consequences — lost progress, repeated content, resource loss — must be calibrated so the cost does not exceed the player's willingness to retry, because a cost that exceeds the willingness produces churn rather than retry. The decision rule: for each failure consequence, assess whether the cost is proportionate to the player's willingness to retry, and reduce consequences that would drive churn. Disproportionate consequences churn, because the cost exceeded the willingness to retry.

### Respect Player Time in Failure Recovery

Failure recovery must respect the player's time — quick retry, minimal repeated content, no excessive re-traversal — because failure that costs significant time becomes a time tax that produces resentment and churn rather than learning. The decision rule: ensure failure recovery is quick (nearby respawn, minimal repetition), and avoid recovery that costs significant time. Time-heavy recovery churns, because the time tax exceeded the player's tolerance.

### Distinguish Stakes From Punishment in Consequence Design

There is a difference between stakes (consequences that give the failure meaning and the success weight) and punishment (consequences that exist to hurt the player), and fail-state design should aim for stakes rather than punishment, because stakes motivate while punishment discourages. The decision rule: for each consequence, ask whether it creates stakes (meaningful weight) or punishment (hurt for its own sake), and favor stakes. Punishment-as-design discourages, because the consequence existed to hurt rather than to add meaning.

### Provide Recovery Paths So Failure Is Not a Dead End

Failure should not be a dead end — the player should always have a path to recover, retry, or proceed — because a failure with no recovery produces a stuck player who churns rather than a learning player who retries. The decision rule: ensure every failure state has a recovery path (retry, alternate route, resource regen), and avoid fail-states that trap the player without recovery. Dead-end failures trap and churn, because the player had no path forward.

### Frame Failure as Part of Learning, Not as Shame

The framing of failure — the messaging, the tone, the presentation — should frame it as part of learning (a normal step in mastery) rather than as shame (a judgment on the player), because shame-framed failure discourages while learning-framed failure motivates. The decision rule: frame failure with learning-oriented messaging (encouragement, feedback), and avoid shame-framed failure (mockery, condescension). Shame-framed failure discourages, because the player was judged rather than taught.

## Common Traps

### Failure Without Teaching, Producing Repeated Mistakes

The team designs failure that punishes without teaching — no feedback on what went wrong, no indication of what to change — and the player repeats the same mistakes without learning. The trap is that the failure has consequences. The false signal is that the failure has weight. The harm is that the player does not learn from the failure, they repeat the same approach and fail again, the frustration of unteaching failure accumulates, and the player churns at the failure point, because the failure did not teach what to change.

### Disproportionate Consequences That Drive Churn

The team calibrates failure consequences that exceed the player's willingness to retry — severe progress loss, long repetition, heavy resource cost — and the player churns rather than retries. The trap is that severe consequences feel weighty. The false signal is that the failure has stakes. The harm is that the cost of retrying exceeds the player's willingness, the player abandons the challenge rather than paying the cost, the churn at the failure point undermines retention, and the game loses the player who would have continued with proportionate consequences, because the consequence was disproportionate.

### Time-Heavy Recovery That Taxes the Player

The team designs failure recovery that costs significant time — long re-traversal, repeated content, slow respawn — and the failure becomes a time tax that produces resentment. The trap is that slow recovery adds stakes. The false signal is that the failure has weight. The harm is that the player pays a time cost on every failure, the repetition produces resentment, the player who would retry with quick recovery instead churns, and the failure is experienced as a time tax rather than a learning opportunity, because the recovery did not respect the time.

### Punishment Masquerading as Stakes

The team designs consequences that are punishment (hurt for its own sake) but frames them as stakes, and the punishment discourages rather than motivates. The trap is that punishment feels consequential. The false signal is that the failure has weight. The harm is that the consequence exists to hurt rather than to add meaning, the player experiences it as punitive rather than motivating, the discouragement accumulates, and the player churns, because punishment was mistaken for stakes.

### Dead-End Failures That Trap the Player

The team designs failure states with no recovery path — the player is stuck, cannot retry, cannot proceed — and the failure becomes a dead end that traps the player. The trap is that the failure is severe. The false signal is that the failure has consequences. The harm is that the player has no path forward, they are trapped at the failure point, the frustration of a dead end accumulates, and the player churns because they cannot recover or proceed, because no recovery path was provided.

### Shame-Framed Failure That Discourages

The team frames failure with shame — mocking messages, condescending tone, judgmental presentation — and the player is discouraged rather than motivated. The trap is that shame-framed failure feels challenging. The false signal is that the game does not coddle. The harm is that the player experiences the failure as a judgment, the discouragement accumulates, the player who would retry with learning-framed failure instead churns, and the failure is experienced as shame rather than as a step in mastery, because the framing was judgmental rather than educational.

## Self-Check

- Does each failure state provide actionable feedback that teaches what went wrong and what to do differently?
- Are failure consequences calibrated to not exceed the player's willingness to retry, avoiding churn?
- Does failure recovery respect the player's time, with quick retry and minimal repeated content?
- Have I distinguished stakes (meaningful weight) from punishment (hurt for its own sake) and favored stakes?
- Does every failure state have a recovery path, avoiding dead-ends that trap the player?
- Is failure framed as part of learning (encouragement, feedback) rather than as shame (mockery, judgment)?
- Did I confirm failure teaches and motivates retry rather than punishing and discouraging churn?
