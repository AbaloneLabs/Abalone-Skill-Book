---
name: conversion_optimization_and_attribution.md
description: Use when the agent is optimizing conversion rates, attributing conversion credit across touchpoints, prioritizing which funnel step to improve, or deciding whether a conversion gain is real or an artifact of windowing and segmentation.
---

# Conversion Optimization And Attribution

Optimizing conversion is not the same as moving a number. A rate can rise because the product improved, because the audience shifted, because the window changed, or because a segment with different behavior entered the funnel. Without separating these, a team celebrates a gain that is partly or wholly an artifact, then scales an intervention that does not reproduce the result. Attribution adds another layer: when conversion depends on multiple touchpoints, deciding which one deserves credit is a model of belief, not a measurement, and the model determines where effort and budget go.

The harm this skill prevents is optimizing the wrong thing with confidence. A team improves a step, sees the overall rate rise, and credits the improvement, when the rise came from a higher-intent acquisition channel. A marketing team uses last-touch attribution, credits the final ad, and underfunds the earlier touchpoints that actually drove the conversion. In each case the decision felt data-driven because a number moved, but the link between the action and the outcome was assumed or modeled, not established.

Use this skill when prioritizing which funnel step to improve, when attributing conversion credit across touchpoints, and when validating that an observed conversion gain is real rather than an artifact. The work is to prioritize by leverage, attribute with awareness of model assumptions, and validate gains before scaling them.

## Core Rules

### Prioritize Steps By Leverage, Not By Drop-Off Alone

The step with the largest drop-off is not always the step most worth improving. Leverage depends on the size of the drop, the feasibility of moving it, and the downstream effect on the rest of the funnel. A step with moderate drop-off that is easy to fix and unlocks conversion for many users may be higher leverage than the biggest drop, which may be structurally hard to move.

Estimate leverage as the product of the addressable drop, the realistic improvement, and the value of the converted users. A huge drop that you can move by one percent is lower leverage than a moderate drop you can move by twenty percent. Prioritize by expected impact, not by the raw drop-off rate.

### Separate Real Gains From Window And Segmentation Artifacts

A conversion rate can rise without any product improvement. A longer conversion window captures more eventual converters and inflates the rate. A shift in the audience mix toward higher-intent users lifts the average even though no user's experience changed. Before treating a gain as real, rule out these artifacts, or the team will scale an intervention that does not reproduce the result.

Hold the window, the population definition, and the segmentation constant when comparing rates over time. If the gain disappears under a consistent definition, it was an artifact. A real gain survives a change in how it is measured; an artifact depends on the measurement.

### Evaluate Attribution Models By Their Assumptions

Attribution assigns credit for a conversion across the touchpoints that preceded it, and every model encodes assumptions that determine the answer. Last-touch gives all credit to the final interaction and ignores everything that built intent earlier. First-touch does the reverse and undervalues the closer. Position-based and data-driven models spread credit by rules or learned weights. None of these is the true causal decomposition, because the true decomposition is not observable.

Choose an attribution model based on the decision it informs and the biases you can tolerate, and never present its output as ground truth. The model is a lens, and each lens emphasizes a different part of the journey. Know which lens you are using and what it hides.

### Compare Attribution Across At Least Two Models

Because attribution output depends on the model, a conclusion that holds under only one model is fragile. Run the same conversion data through at least two attribution models, typically last-touch and first-touch, or last-touch and a data-driven model. If the credit distribution flips between models, the attribution is not robust enough to drive a budget decision.

Where models agree, you can act with more confidence. Where they disagree, the disagreement itself is information: it tells you the touchpoints are interdependent and that simple credit assignment will mislead. Treat model disagreement as a signal to investigate the journey, not to pick the model that confirms your preference.

### Validate Conversion Gains With A Counterfactual

A conversion improvement observed after a change is the weakest evidence that the change caused it, because time carries confounders such as seasonality, concurrent changes, and audience shift. The credible evidence is a counterfactual: what would conversion have been without the change. Without a comparison group, the gain is a before-and-after story.

Where possible, validate gains with an experiment or holdout that isolates the change. Where experimentation is not possible, use the strongest available quasi-experimental method and state its assumptions. A gain that survives a clean comparison is real; a gain observed only in a before-and-after is a hypothesis.

### Account For Interaction Effects Between Steps

Funnel steps are not independent. Improving one step can change the composition of users who reach the next step, sometimes lowering the downstream conversion rate even though the improved step converted more users. A team that optimizes each step in isolation can move local rates up while the overall funnel moves down, because the steps interact.

Evaluate optimization at the funnel level, not only the step level. After improving a step, check whether the users it newly delivers convert downstream at a comparable rate, or whether they dilute the pool. The goal is overall conversion, not a high rate at any single step.

### Distinguish Volume Optimization From Rate Optimization

Improving the conversion rate and improving the volume of conversions are different goals that can pull in opposite directions. A change that raises the rate by excluding low-intent users may lower total conversions, because fewer users enter. A change that widens entry may lower the rate while raising volume. Decide which outcome matters before optimizing, because the two metrics can move differently.

State whether the goal is rate, volume, or the value of converted users, and optimize against that goal. Reporting only the rate can hide that volume fell, and reporting only volume can hide that efficiency dropped.

## Common Traps

### Prioritizing The Biggest Drop-Off Without Considering Leverage

The largest drop is not always the highest-leverage improvement, because it may be structurally hard to move. Prioritize by expected impact, accounting for feasibility and downstream effect, not by the raw drop-off rate.

### Treating A Rate Gain As Real Without Ruling Out Artifacts

A longer window or an audience shift can lift the conversion rate with no product improvement. Hold the definition constant when comparing rates, or the gain may be an artifact of measurement.

### Trusting A Single Attribution Model As Ground Truth

Attribution output is assumption-laden, and any single model hides the touchpoints it undervalues. Compare across at least two models before acting, and treat disagreement as a signal to investigate.

### Optimizing Steps In Isolation And Missing Interactions

Improving one step can change who reaches the next step and lower downstream conversion, so local optimization can reduce overall funnel performance. Evaluate changes at the funnel level, not only per step.

### Confusing Rate Optimization With Volume Optimization

Raising the rate can lower volume, and raising volume can lower the rate. Decide which outcome matters before optimizing, and report both so a gain in one does not hide a loss in the other.

### Crediting A Change Based On Before-And-After Alone

A conversion rise after a change is weak evidence of causation, because time carries confounders. Validate with a counterfactual or experiment before scaling the change.

## Self-Check

- [ ] Funnel steps are prioritized by leverage, combining addressable drop, realistic improvement, and converted-user value, not by raw drop-off alone.
- [ ] Conversion gains are checked against window, population, and segmentation artifacts by holding the definition constant across comparisons.
- [ ] The attribution model in use is named, and its assumptions and blind spots are understood before its output drives decisions.
- [ ] Attribution is compared across at least two models, and model disagreement triggers investigation rather than selection of the preferred result.
- [ ] Conversion gains are validated with a counterfactual, experiment, or the strongest quasi-experimental method, not accepted from before-and-after alone.
- [ ] Optimization is evaluated at the funnel level, and interaction effects between steps are checked so local gains do not reduce overall conversion.
- [ ] The goal of rate versus volume versus converted-user value is stated explicitly, and both rate and volume are reported so a gain in one does not hide a loss in the other.
- [ ] No conversion gain is scaled until it survives a consistent measurement definition and, where possible, a clean comparison.
- [ ] Touchpoint credit is presented as model-dependent, not as ground truth, in any readout that informs budget or effort allocation.
- [ ] A real gain is distinguished from an artifact by confirming it persists under unchanged window, population, and segmentation.
