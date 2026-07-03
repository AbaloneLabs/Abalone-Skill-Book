---
name: parametric_and_analogous_estimation.md
description: Use when the agent is estimating using parametric models and rates, building estimates from historical or analogy-based data, calibrating estimates against past projects, or deciding between bottom-up and top-down estimation approaches for a body of work.
---

# Parametric And Analogous Estimation

Parametric and analogous estimation are the two techniques that lean on data rather than judgment. Analogous estimation compares the work to a similar past effort and scales the result; parametric estimation applies a calibrated rate or model to a count of units. Both are fast and defensible when the underlying data is sound, and both are misleading when the analogy is weak or the parameters are stale. Agents tend to reach for a number that looks quantitative without checking whether the historical basis actually matches, or whether the rate has been validated against real outcomes. A parametric estimate built on an untested assumption is just a guess with a formula.

Use this skill before building a top-down or bottom-up estimate, applying a productivity rate, or using a past project as the basis for a new one. The goal is to prevent the agent from producing data-driven-looking estimates whose data foundation does not hold.

## Core Rules

### Match The Technique To Available Data

Analogous estimation suits early stages when only a comparable past project exists; it is fast, cheap, and roughly right when the analogy holds. Parametric estimation suits situations where a stable unit rate exists, such as hours per screen, cost per square meter, or lines per document. Bottom-up estimation, summing detailed work-package estimates, is the most accurate but the most expensive and only valid once decomposition exists. Choose by what data you actually have, not by what looks most rigorous.

State the technique and its data source, because the credibility of the estimate depends entirely on that basis.

### Validate The Analogy Before Scaling

Analogous estimation is only as good as the similarity between the past and present work. Before scaling a past result, compare the dimensions that drive effort: scope size, complexity, team skill, technology, domain familiarity, tooling, regulatory constraints, and environment. Where dimensions differ, adjust the estimate explicitly rather than assuming proportionality. A past project that looks similar but used a different technology or a more experienced team will mislead.

Document the comparison and the adjustments so the analogy can be challenged and revised.

### Calibrate Parameters Against Actuals

A parametric rate is only valid if it has been measured against real outcomes. A rate copied from a textbook or a vendor proposal is an assumption, not a parameter. Track actual effort and output, compute the real rate, and feed it back into the model. If actuals consistently diverge from the parameter, the parameter is wrong, the units are miscounted, or the model omits a driver such as complexity or integration.

Recompute parameters periodically; a rate that was right two years ago may no longer hold after tooling or team changes.

### Count The Right Units

Parametric estimation depends on counting the units the rate applies to, and miscounting is a common error. The units must be well-defined, consistently measurable, and actually correlated with effort. Counting screens is useful if screens drive effort; counting requirements may be less useful if their size varies wildly. Choose units that are predictive and that can be counted early and reliably.

Beware units that are easy to count but weakly correlated with effort; precision in counting does not rescue a weak driver.

### Decide Bottom-Up Versus Top-Down Deliberately

Top-down estimation starts from the whole, often analogous or parametric, and allocates downward; it is fast and good for early budgets but hides where the risk sits. Bottom-up starts from detailed packages and rolls up; it is accurate but expensive and only valid once the breakdown exists. A strong practice uses both: top-down to set a sanity bound, bottom-up to test it, and reconciliation to expose where they disagree. The gap between them is information about uncertainty.

Do not present a top-down number as if it were bottom-up precise.

### Separate The Model From Its Inputs

A parametric model has structure and inputs, and each input carries uncertainty. Distinguish the model's logic from the assumptions fed into it. When the estimate is challenged, the conversation should be about the inputs and their ranges, not the arithmetic. Make the inputs visible so they can be debated and updated.

A model that hides its inputs cannot be validated; one that exposes them can be improved.

### Adjust For Context Differences

Even a good parameter or analogy needs adjustment for the current context: team experience, new technology, regulatory burden, geographic distribution, concurrency, and tooling maturity. Apply explicit adjustment factors rather than hoping the base number absorbs the difference. Record the factors so they can be reviewed.

Context that differs significantly from the calibration basis can dwarf the precision of the underlying rate.

### Reconcile And Document The Basis Of Estimate

Every estimate should carry a basis-of-estimate note: technique used, data source, key assumptions, adjustment factors, and known limitations. This is what makes the estimate reviewable and what lets a future estimator learn from it. An estimate without a stated basis is an assertion.

## Common Traps

### Weak Analogy Treated As Strong

A past project that differs in technology, team, or complexity produces a confidently wrong estimate. Validate and adjust the analogy.

### Textbook Rates As If Measured

A productivity rate copied without validation is an assumption dressed as data. Calibrate against actuals.

### Counting Units That Do Not Predict Effort

Precise counting of a weakly correlated unit yields false precision. Choose predictive, countable units.

### Top-Down Presented As Bottom-Up Precision

A fast analogous or parametric number shown with detailed precision misleads stakeholders about certainty. Label the technique honestly.

### Stale Parameters

A rate correct years ago may no longer hold. Recompute parameters periodically against recent actuals.

### Hidden Model Inputs

A model whose assumptions are invisible cannot be challenged or improved. Expose the inputs and their ranges.

### Ignoring Context Adjustment

Failing to adjust for team, technology, or regulatory differences lets the base number carry error it cannot absorb. Apply explicit factors.

### No Basis Of Estimate

A number with no documented source, assumptions, or limitations is an assertion that cannot be reviewed. Write the basis.

## Self-Check

- [ ] Is the estimation technique chosen to match the data actually available, and is it stated?
- [ ] Has the analogy been validated against scope, complexity, team, technology, and environment, with adjustments documented?
- [ ] Are parametric rates calibrated against real actuals rather than copied from textbooks or proposals?
- [ ] Are the counted units well-defined, reliably countable, and genuinely correlated with effort?
- [ ] Are top-down and bottom-up estimates reconciled, with the gap treated as uncertainty information?
- [ ] Is a top-down number labeled as such rather than presented with bottom-up precision?
- [ ] Are model inputs and their ranges exposed so the estimate can be debated and updated?
- [ ] Are explicit adjustment factors applied for context differences in team, technology, regulation, and tooling?
- [ ] Are parameters recomputed periodically against recent actual outcomes?
- [ ] Does the estimate carry a basis-of-estimate note with source, assumptions, factors, and limitations?
