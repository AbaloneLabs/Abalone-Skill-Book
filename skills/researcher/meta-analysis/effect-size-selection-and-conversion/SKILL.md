---
name: effect_size_selection_and_conversion.md
description: Use when the agent is choosing an effect size metric for a meta-analysis, converting among standardized mean differences, odds ratios, risk ratios, and correlation coefficients, combining studies with different outcome scales, or deciding which effect estimate to extract and pool.
---

# Effect Size Selection And Conversion

A meta-analysis pools effect sizes, and the choice of effect size metric is the single most consequential decision after study inclusion. The wrong metric, an inconsistent metric across studies, or a flawed conversion can make a synthesis meaningless or actively misleading. Agents often reach for whatever statistic a study reports, or a default metric they know, without checking whether it is comparable across studies or appropriate to the outcome type. Pooling incompatible effect sizes produces a precise-looking number that answers no coherent question.

Use this skill when selecting an effect size metric, extracting effect estimates from heterogeneous studies, converting among metrics, or deciding whether studies can be pooled at all. The goal is to prevent the agent from combining effect sizes that do not measure the same thing.

## Core Rules

### Match The Metric To The Outcome Type And Question

Effect size metrics are not interchangeable; each answers a specific question and suits specific data. Choosing a metric is choosing the question the meta-analysis answers.

Match by outcome:

- binary outcomes: odds ratio, risk ratio, risk difference, depending on the inferential question;
- continuous outcomes on a common scale: mean difference;
- continuous outcomes on different scales: standardized mean difference;
- time-to-event: hazard ratio;
- association between two continuous measures: correlation coefficient;
- diagnostic accuracy: sensitivity, specificity, diagnostic odds ratio.

The metric determines interpretability. A pooled standardized mean difference answers a different question than a pooled risk ratio, and the two cannot be narrated the same way.

### Ensure Conceptual Comparability Before Pooling

Even with the same metric, studies must measure the same construct for the pool to be meaningful. A standardized mean difference across different outcome instruments is only valid if the instruments capture the same underlying construct with similar reliability.

Check comparability:

- do the outcomes measure the same construct?
- are the measurement instruments sufficiently similar?
- is the direction of effect consistent across studies (higher is better vs. worse)?
- are the comparison groups defined the same way?
- is the time point of measurement comparable?

If studies measure conceptually different outcomes, pooling them yields an average of nothing in particular.

### Convert Effect Sizes Using Defensible Methods

When studies report different metrics, conversion is sometimes possible, but every conversion adds assumptions and potential error. Conversions should be explicit and sourced.

Common conversions include:

- between standardized mean difference and odds ratio via known transformations;
- from a t or F statistic to a standardized mean difference;
- from a correlation to a standardized mean difference and vice versa;
- from means and standard deviations to a standardized mean difference.

Record the formula or source for each conversion, and flag conversions that rest on strong assumptions (e.g., normality, equal variances). Do not silently transform a reported statistic into the metric you wanted.

### Handle Outcome Direction And Sign Consistently

A frequent and serious error is pooling effect sizes with inconsistent direction, so that a beneficial effect in one study and a harmful effect in another both appear as positive numbers. This manufactures a false impression of consistency.

Standardize by:

- defining the favorable direction before extraction;
- recoding all effects so that positive always means the same direction of outcome;
- documenting any study where the direction was reversed;
- checking the forest plot for studies that sit on the unexpected side.

Direction errors are easy to make and hard to detect in a table; the forest plot is the check.

### Prefer Adjusted Over Unadjusted Estimates Judiciously

Studies may report crude and adjusted effect estimates. The choice affects the pool and its interpretation.

Decide and document:

- whether to extract adjusted or unadjusted estimates, and why;
- consistency across studies in what covariates were adjusted for;
- whether mixing adjusted and unadjusted estimates is defensible;
- the impact of different adjustment sets on comparability.

Mixing estimates with very different adjustment sets can produce heterogeneity that is really methodological, not substantive.

### Account For Unit Of Analysis And Clustering

Many studies report effects from clustered or multi-arm designs. Ignoring clustering or double-counting arms distorts the pooled effect and its precision.

Handle by:

- splitting shared comparison groups in multi-arm trials to avoid double-counting;
- adjusting for clustering using design effects or intraclass correlation when reported;
- ensuring each independent unit of analysis contributes once to a given comparison;
- documenting how combined intervention arms or multiple comparators were handled.

### Extract Variance And Precision, Not Just Point Estimates

A meta-analysis weights studies by precision, so the variance or standard error of each effect is essential. A point estimate without its variance cannot be properly pooled.

Ensure for each study:

- the standard error, confidence interval, or variance is extracted or computable;
- the sample sizes feeding the effect are recorded;
- the basis for the precision estimate is documented;
- missing precision is handled by a pre-specified rule (e.g., derive from p-value, contact authors, exclude).

Pooling point estimates by simple averaging ignores precision and is not a meta-analysis.

### Decide On Random- Versus Fixed-Effects A Priori

The choice of model is an inferential decision tied to the assumed population of studies. It should be justified before results are seen.

Consider:

- fixed-effect assumes one true effect shared by all studies;
- random-effects assumes a distribution of true effects;
- the presence of known clinical or methodological diversity usually favors random-effects;
- the choice changes weights, confidence intervals, and prediction.

Do not switch models because one gives a more favorable or significant result.

### Pre-Specify How To Handle Zero-Event And Sparse-Data Studies

Binary outcomes with zero events in one or both arms require special handling, and the method changes the result, especially with sparse data.

Pre-specify:

- continuity correction rules and magnitude;
- whether to use methods designed for sparse data or rare events;
- handling of studies with zero events in both arms;
- sensitivity analyses for the continuity correction choice.

The treatment of zero-event studies can swing a pooled odds ratio substantially, so the rule must precede the data.

## Common Traps

### Pooling Incompatible Outcome Scales

Standardized mean differences across instruments measuring different constructs produce an average that has no referent.

### Silent Effect Size Conversion

Transforming a reported statistic into a desired metric without recording the method hides assumptions that may not hold.

### Inconsistent Direction Coding

Mixing favorable and unfavorable directions so contradictory studies appear to agree is a serious and common error.

### Ignoring Precision In Pooling

Averaging point estimates without weighting by precision is not meta-analysis and overweights small noisy studies.

### Switching Models For A Favorable Result

Choosing fixed- or random-effects based on which gives significance is p-hacking at the synthesis level.

### Unhandled Clustering Or Multi-Arm Double-Counting

Ignoring shared groups or clustered designs inflates precision and can shift the pooled estimate.

### Post-Hoc Continuity Corrections

Choosing how to handle zero-event studies after seeing the result lets the method drive the conclusion.

## Self-Check

- Is the effect size metric matched to the outcome type and the inferential question?
- Do all pooled studies measure the same construct with comparable instruments and direction?
- Is every effect size conversion documented with its formula, source, and assumptions?
- Are all effects recoded to a consistent favorable direction before pooling?
- Is the choice of adjusted versus unadjusted estimates consistent and justified?
- Are multi-arm and clustered designs handled to avoid double-counting and precision inflation?
- Does every study contribute a variance or standard error, with missing precision handled by rule?
- Is the fixed- versus random-effects choice justified a priori?
- Are zero-event and sparse-data studies handled by a pre-specified method with sensitivity analysis?
- Does the forest plot reveal no studies sitting on an unexpected side due to direction errors?
