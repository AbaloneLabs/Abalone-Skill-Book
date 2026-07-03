---
name: relative_and_decomposition_estimation.md
description: Use when the agent is estimating work using relative sizing such as story points or T-shirt sizes, applying decomposition techniques to break down work before estimating, running planning poker, or deciding whether relative or absolute estimates better fit a given body of work.
---

# Relative And Decomposition Estimation

Relative estimation and decomposition are paired crafts. Relative sizing compares work to other work rather than to the clock, which is faster and often more accurate when absolute units are uncertain. Decomposition breaks large items into smaller, estimable pieces, because the biggest source of estimation error is size and ambiguity, not the choice of unit. Used together they let a team produce stable forecasts early, before enough is known to estimate in hours. Used badly, they produce story-point theater: numbers that look rigorous but are untethered from effort, drift over time, and get converted into false commitments. Agents tend to reach for relative sizing without anchoring it, and to estimate items that are far too large to estimate meaningfully.

Use this skill before sizing a backlog, running an estimation session, choosing between relative and absolute units, or diagnosing why velocity is unstable. The goal is to prevent the agent from producing relative estimates that are arbitrary and decomposition that is shallow.

## Core Rules

### Decide Relative Versus Absolute By Certainty And Purpose

Relative sizing (story points, T-shirt sizes) works best early, when scope is fuzzy and the team is comparing items it does not yet fully understand; it is fast, calibrates itself over time, and absorbs uncertainty. Absolute sizing (hours, days) works best when work is well understood, when the estimate must feed a cost model or contract, or when a single specialist will do the work and duration matters. The choice is not ideological; it depends on how much is known and what the estimate feeds.

State which you are using and why. Mixing them, or treating points as hidden hours, destroys both.

### Anchor Relative Estimates To A Reference Story

Relative estimation only works if the team shares a reference. Pick one well-understood, recently completed item as the baseline, assign it a value, and size everything else by comparison: about the same, bigger, much bigger. Without an anchor, points become arbitrary numbers that mean different things to different people. Re-anchor when the team changes or when the work type shifts significantly.

Keep the reference visible during every estimation session so comparisons stay grounded.

### Decompose Before Estimating Large Items

The dominant cause of estimation error is item size and ambiguity, not the estimation technique. An item that is large enough to contain unknown subtasks cannot be estimated honestly; the wide range of possible outcomes makes any single number misleading. Decompose large items into smaller pieces until each piece is small enough that the team can describe what done looks like and where the risk lies. Then estimate the pieces.

A common rule: if the team cannot agree, or if estimates span an order of magnitude, the item is too large and must be split.

### Run Planning Poker To Surface Disagreement

Planning poker is valuable not because of the cards but because simultaneous independent estimates expose disagreement that discussion would hide. When estimates cluster, confidence is high. When they diverge widely, the divergence is the finding: the team has different assumptions about scope, risk, or approach. Have the high and low estimators explain their reasoning before re-voting, then split the item if disagreement persists.

Do not average divergent estimates; averaging hides the uncertainty that the exercise was meant to reveal.

### Use A Non-Linear Scale To Force Choices

A Fibonacci-like or T-shirt scale (1, 2, 3, 5, 8, 13 or XS, S, M, L, XL) is non-linear on purpose. As items get larger, uncertainty grows faster than size, so the gaps between values widen. This forces the team to either commit to a big number that signals real uncertainty or to decompose the item. A linear scale lets large items be estimated with false precision.

Cap the scale and treat anything above the cap as too big to estimate; require decomposition instead.

### Track Velocity As A Range, Not A Number

Velocity is the sum of points completed per sprint, and it is useful for forecasting only when treated as a noisy range, not a target. Track a rolling average and its spread, and forecast using the range. A single velocity number presented as a commitment becomes a quota that distorts sizing and encourages inflating points to hit it.

Velocity is also only comparable within a team; points are not transferable across teams because the reference differs.

### Re-Estimate Only With Reason

Re-estimating every sprint wastes effort and breaks the calibration that makes velocity meaningful. Re-estimate when the reference has drifted, when the team composition changed substantially, when the work type changed, or when an item's understanding has fundamentally shifted. Otherwise leave old estimates alone and let actuals inform future sizing.

### Distinguish Effort, Complexity, And Risk In The Number

A story point conflates effort, complexity, and risk into one relative value, which is acceptable for forecasting but must be understood when interpreting. Two items with the same point value may differ: one is high-effort low-risk, the other is low-effort high-risk. When risk dominates, note it separately so it can be tracked rather than buried in the number.

## Common Traps

### Treating Points As Hidden Hours

If one point always equals four hours, the points add ceremony without value and invite false precision. Use hours directly, or commit to relative meaning.

### Estimating Without An Anchor

Unanchored relative estimates drift and become arbitrary. Establish and keep a visible reference story.

### Estimating Items That Are Too Large

Large items produce wild disagreement and meaningless averages. Decompose first, estimate second.

### Averaging Divergent Planning Poker Votes

Averaging hides the disagreement the exercise was designed to expose. Discuss and re-vote or split.

### Velocity As A Quota Or Target

When velocity becomes a goal, teams inflate points to hit it, destroying calibration. Forecast with a range.

### Comparing Points Across Teams

Points are team-specific because the reference differs. Cross-team point comparison is meaningless.

### Linear Scales With False Precision

A 1-to-10 scale lets large items be estimated precisely. Use a non-linear scale that forces decomposition.

### Re-Estimating Everything Each Sprint

Constant re-estimation breaks calibration and wastes time. Re-estimate only with a real reason.

## Self-Check

- [ ] Is the choice between relative and absolute estimation justified by certainty and purpose, and stated explicitly?
- [ ] Is there a visible, well-understood reference story anchoring all relative estimates?
- [ ] Are large items decomposed into estimable pieces before any number is assigned?
- [ ] Does planning poker surface and discuss disagreement rather than averaging it away?
- [ ] Is a non-linear scale used so large items are forced toward decomposition?
- [ ] Is velocity tracked and forecast as a range, never presented as a quota or target?
- [ ] Are points treated as team-specific and never compared across teams?
- [ ] Are old estimates left intact unless a real reason to re-estimate exists?
- [ ] Is risk that dominates an item's size noted separately rather than buried in the point value?
- [ ] When estimates span an order of magnitude, is the item split rather than forced to a number?
