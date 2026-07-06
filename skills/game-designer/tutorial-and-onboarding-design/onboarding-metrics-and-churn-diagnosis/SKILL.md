---
name: onboarding-metrics-and-churn-diagnosis.md
description: Use when the agent is instrumenting onboarding to measure where new players churn, diagnosing first-session drop-off points, deciding which onboarding steps to fix based on data, or evaluating whether onboarding changes actually improve retention or whether churn is being misdiagnosed.
---

# Onboarding Metrics and Churn Diagnosis

Onboarding is the most measurable part of a game's lifecycle because churn is concentrated in the first session, and the data exists to identify exactly where players leave — yet the data is easy to misread, and the wrong diagnosis produces fixes that do not improve retention. The judgment problem is that churn at a point does not uniquely identify the cause: a player who quits at a combat encounter may be leaving because the combat is hard, because the tutorial before it was confusing, because the game is not what the store page promised, or because their bus arrived. Agents tend to miss this because the metric (drop-off at step N) is clear while its cause is ambiguous, and because the team's preferred diagnosis is often the one that requires the least work to fix. The harm is onboarding "fixes" that address misdiagnosed causes and do not move retention, consuming the team's effort while the real churn drivers persist. This skill covers how to instrument onboarding, diagnose churn causes from data, and avoid the misdiagnosis trap. The agent has latitude in the metrics chosen, but the obligation to distinguish symptom from cause is not optional.

## Core Rules

### Instrument Funnel Steps Granularly Enough to Localize Churn

Churn diagnosis requires knowing not just that players leave the first session but where, specifically, they leave — which step, which encounter, which screen. The decision rule: instrument the onboarding funnel at a granular step level (each tutorial beat, each encounter, each menu), and produce a funnel report showing the drop-off at each step. Coarse funnel data (first-session completion rate) cannot localize the defect; only granular step-level instrumentation reveals where the churn concentrates, which is where the diagnosis must focus.

### Triangulate Churn Cause With Multiple Signals

A drop-off at a step has multiple possible causes, and no single signal distinguishes them, so the cause must be triangulated from several signals: the drop-off itself, the behavior preceding it (repeated failures, long pauses, menu exits), and qualitative follow-up where possible. The decision rule: for each churn point, gather behavioral context (what the player did before leaving) and, where feasible, qualitative context (survey, follow-up), and converge on a cause supported by multiple signals rather than a single metric. Single-signal diagnoses are guesses; triangulated diagnoses are evidence.

### Distinguish Difficulty Churn From Confusion Churn From Disinterest Churn

Churn at a point may be difficulty (the player could not proceed), confusion (the player did not know what to do), or disinterest (the player did not want to continue), and the fixes are entirely different: difficulty churn needs tuning, confusion churn needs clearer direction, disinterest churn needs a more engaging experience or a more accurate store page. The decision rule: classify each churn point by its cause type using the behavioral and qualitative signals, and apply the fix matched to the cause. Misclassifying confusion as difficulty, for example, produces a tuning fix that does not address the actual cause.

### A-B Test Onboarding Changes Against a Control

Onboarding fixes must be validated against a control, because retention fluctuates for many reasons and a perceived improvement may be noise. The decision rule: when changing onboarding, run an A-B test comparing the new version against the control, and ship the change only if retention measurably improves. Changes shipped without control comparison may be adopted on faith, attributing natural fluctuation to the fix, and the team continues "improving" onboarding without knowing whether any change helped.

### Watch Real Sessions, Not Just Aggregate Funnels

Aggregate funnel data shows where churn concentrates but not why, and watching real session recordings of players churning at the identified points reveals the cause in a way aggregates cannot. The decision rule: supplement funnel data with session recordings of players who churned at the diagnosed points, and observe what they did, where they struggled, and what preceded their exit. The recording often reveals a cause the aggregate hid — a confusing prompt, an unexpected failure — because the player's behavior is visible where the funnel is only a count.

### Re-Diagnose After Each Fix, Because Churn Migrates

Fixing one churn point often reveals or creates the next, because players who previously left at step N now proceed to step N+1 where a different churn cause awaits. The decision rule: after each onboarding fix, re-measure the funnel to confirm the targeted point improved and to identify the new churn concentration, and iterate. Treating onboarding as a single fix rather than an iterative diagnosis leaves the next churn point unaddressed, because churn migrates as earlier points are resolved.

## Common Traps

### Coarse Funnel Data That Cannot Localize the Defect

The team tracks first-session completion as the onboarding metric, sees it is low, and cannot determine where players are leaving, so they guess at fixes that do not target the actual churn points. The trap is that completion rate is the headline metric. The false signal is that onboarding is being measured. The harm is that the defect's location is invisible without granular step-level data, the fixes are aimed at guessed points, and the retention does not improve because the actual churn points were never identified or addressed, because the funnel was too coarse to localize the problem.

### Single-Signal Diagnosis That Confuses Symptom and Cause

The team sees drop-off at a combat encounter, diagnoses it as difficulty, and tunes the combat easier, but the churn persists because the actual cause was a confusing tutorial before the encounter that left players unprepared. The trap is that the drop-off location suggests an obvious cause. The false signal is that the diagnosis matches the location. The harm is that the fix addresses the symptom (drop-off at combat) not the cause (confusing tutorial), the churn does not improve, and the team concludes onboarding is unfixable or that the audience is wrong, when the real problem was a misdiagnosis that sent the fix to the wrong place.

### Misclassifying Confusion or Disinterest as Difficulty

The team, comfortable with tuning, classifies all churn as difficulty and tunes content easier, but the churn was caused by confusion (players did not know what to do) or disinterest (players did not want to), and the tuning does not help. The trap is that difficulty is the fixable cause the team is equipped to address. The false signal is that tuning is a concrete action. The harm is that the effort spent tuning does not move retention, the actual causes (unclear direction, unengaging content) persist, and the team's confidence in data-driven onboarding erodes because the fixes "did not work," when the real failure was classifying all churn under the one cause the team knew how to fix.

### Shipping Onboarding Changes Without Control Comparison

The team changes the onboarding based on a diagnosis, retention moves (up or down), and they attribute the movement to the change, but the movement was natural fluctuation and the change had no effect. The trap is that before-and-after comparison feels like measurement. The false signal is that retention changed after the fix. The harm is that the team adopts changes on faith, attributes noise to fixes, and continues "improving" onboarding without knowing what actually helps, accumulating changes that may collectively help or hurt with no way to distinguish, because no change was validated against a control.

### Ignoring Session Recordings in Favor of Aggregate Funnels

The team relies on funnel data alone, sees where churn concentrates, and fixes based on the counts, never watching what real churning players actually did, and misses causes the aggregates cannot show. The trap is that funnel data is clean and quantifiable. The false signal is that the data is comprehensive. The harm is that the aggregate hides the behavior that reveals the cause — the player who wandered, who re-read a prompt, who failed an unstated requirement — and the fix is aimed at the count rather than the cause, because the recordings that would have revealed the cause were never watched.

### Treating Onboarding as a Single Fix Rather Than Iterative Diagnosis

The team identifies one churn point, fixes it, declares onboarding improved, and stops, but the players who now proceed past the fixed point churn at the next point, which was never addressed. The trap is that fixing the identified point feels like completion. The false signal is that the targeted metric improved. The harm is that overall retention does not improve because churn migrated to the next point, the team believes the fix failed or onboarding is unsolvable, and the iterative diagnosis that would have caught the migration was never continued, because onboarding was treated as one fix rather than a sequence of diagnoses.

## Self-Check

- Is the onboarding funnel instrumented at granular step level, so churn can be localized to specific beats, encounters, and screens?
- For each churn point, am I triangulating the cause from multiple signals (drop-off, preceding behavior, qualitative follow-up)?
- Have I classified each churn point as difficulty, confusion, or disinterest, and matched the fix to the cause type?
- Are onboarding changes validated through A-B testing against a control, not adopted on before-and-after faith?
- Am I watching real session recordings of churning players, not relying solely on aggregate funnel counts?
- After each fix, am I re-measuring the funnel to confirm improvement and identify the next churn concentration?
- Did I resist classifying all churn as difficulty just because tuning is the fix I am equipped to make?
