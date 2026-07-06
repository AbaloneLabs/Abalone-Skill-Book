---
name: heterogeneity_and_subgroup_analysis.md
description: Use when the agent is interpreting statistical heterogeneity in a meta-analysis, deciding whether to pool, planning and running subgroup or meta-regression analyses, diagnosing the source of inconsistency, or avoiding post-hoc fishing across subgroups.
---

# Heterogeneity And Subgroup Analysis

Heterogeneity is the central interpretive problem in meta-analysis. When studies disagree, the pooled estimate is an average over genuinely different effects, and a single number can hide that the studies do not tell the same story. Agents frequently suppress heterogeneity to report a clean pooled effect, or chase it with post-hoc subgroup analyses until something becomes significant. Both failures produce confident-looking conclusions that do not survive scrutiny. The honest work is to detect heterogeneity, diagnose its source, and report what the average can and cannot tell you.

Use this skill when interpreting a forest plot, assessing heterogeneity statistics, planning subgroup or meta-regression analyses, or deciding how to report an inconsistent body of studies. The goal is to prevent the agent from either hiding real disagreement or manufacturing false explanations for it.

## Core Rules

### Quantify Heterogeneity, Do Not Just Test It

A single p-value for heterogeneity is a weak basis for judgment, especially with few studies. Quantify the magnitude and its uncertainty.

Report:

- the test statistic for heterogeneity and its limitations (low power with few studies);
- I-squared with its confidence interval, not just a point value;
- tau-squared to express the variance of true effects;
- the prediction interval where possible, to show the spread of true effects.

Thresholds for "low," "moderate," and "substantial" heterogeneity are guides, not rules. Interpret magnitude in the context of the outcome and the effects involved.

### Decide Whether Pooling Is Meaningful

High heterogeneity can mean that a single pooled estimate misrepresents the evidence. Pooling should be justified, not automatic.

Ask:

- are the studies similar enough in population, intervention, and outcome to share a true effect?
- does the spread of effects cross the null in both directions?
- would a reader be misled by a single average?
- is a narrative or stratified synthesis more honest than one number?

It is legitimate to present a pooled estimate and still warn that it averages over important differences, but only if those differences are then investigated.

### Pre-Specify Subgroup And Meta-Regression Analyses

Subgroup analyses chosen after seeing results are a major source of false positives. Each additional subgroup multiplies the chance of a chance finding.

Pre-specify:

- which subgroups are hypothesized to modify the effect, with a rationale;
- the direction expected for each;
- whether the analysis is exploratory or confirmatory;
- a small number of candidate moderators to limit multiplicity.

A subgroup that emerges from the data can be reported, but only as hypothesis-generating, not as a confirmed modifier.

### Test Subgroups By Between-Group Difference, Not Within-Group Significance

A common error is to declare a subgroup effect because the effect is significant in one subgroup and not in another. The correct test is whether the subgroups differ from each other.

Evaluate:

- the interaction test between subgroup and effect;
- the overlap of confidence intervals between subgroup estimates;
- whether the difference is plausible in magnitude, not just statistically flagged.

Non-overlap of individual subgroup significance is not evidence of moderation.

### Use Meta-Regression Cautiously And With Enough Studies

Meta-regression can examine continuous moderators, but it is easily over-interpreted, especially with few studies where the analysis is underpowered and prone to spurious associations.

Guard by:

- requiring an adequate number of studies per moderator;
- limiting the number of covariates relative to studies;
- checking for collinearity among moderators;
- treating observational meta-regression associations as ecological, not individual-level.

A meta-regression slope across study-level covariates does not establish an individual-level causal relationship.

### Distinguish Statistical From Clinical Heterogeneity

Two studies can be statistically consistent yet clinically too different to pool meaningfully, or statistically heterogeneous yet clinically coherent. Both dimensions matter.

Assess:

- clinical diversity (populations, interventions, doses, settings, outcomes);
- methodological diversity (designs, risk of bias, measurement);
- statistical heterogeneity (the numeric inconsistency).

Statistical homogeneity does not override clinical diversity; it may simply reflect low power to detect it.

### Investigate Heterogeneity Through Sensitivity Analysis

Sensitivity analyses test how robust the pooled estimate is to decisions about which studies and methods to include.

Run sensitivity analyses by:

- excluding high risk-of-bias studies;
- excluding each study one at a time (leave-one-out);
- varying the effect metric or model;
- excluding studies with extreme effects or suspected outliers;
- re-running with alternative zero-event handling.

If the conclusion flips under a defensible sensitivity analysis, the conclusion is fragile and should be reported as such.

### Report The Range And Prediction, Not Only The Mean

A pooled effect with a narrow confidence interval can still imply a wide spread of true effects across settings. The prediction interval communicates this.

Report:

- the pooled mean and its confidence interval;
- the predicted range of true effects in a new similar study;
- the proportion of studies favoring each direction, if relevant.

A reader planning to apply the finding needs to know the expected spread, not just the average.

### Avoid The Temptation To Explain Away Heterogeneity With One Study

When heterogeneity is high, it is tempting to find one outlier, remove it, and declare the evidence consistent. This is defensible only if the removal is justified on methodological grounds and disclosed.

Justify any exclusion by:

- a documented methodological flaw, not the size of its effect;
- pre-specified exclusion rules where possible;
- showing the result with and without the study;
- resisting the impulse to remove the most inconvenient rather than the most flawed study.

## Common Traps

### Suppressing Heterogeneity For A Clean Pooled Effect

Reporting a single average while burying high I-squared misleads readers about how consistent the evidence is.

### Post-Hoc Subgroup Fishing

Running many subgroups until one is significant manufactures false positives and should be labeled exploratory at best.

### Inferring Moderation From Within-Subgroup Significance

Significance in one subgroup and not another does not show the subgroups differ; the interaction test does.

### Over-Reading Meta-Regression With Few Studies

Ecological slopes from a handful of studies are underpowered and easily spurious.

### Equating Statistical Homogeneity With Clinical Comparability

Low detected heterogeneity may reflect low power, not genuine similarity of studies.

### Removing An Outlier For Convenience

Excluding the most inconvenient study without methodological justification is cherry-picking.

### Ignoring The Prediction Interval

A narrow confidence interval around the mean can hide a wide spread of true effects that matters for application.

## Self-Check

- Is heterogeneity quantified with I-squared, tau-squared, and a prediction interval, not just a p-value?
- Has the meaningfulness of pooling been justified in light of the heterogeneity?
- Are subgroup and meta-regression analyses pre-specified with rationales and expected directions?
- Are subgroup effects judged by between-group interaction, not within-group significance?
- Is meta-regression limited to a defensible number of moderators relative to the number of studies?
- Are clinical and methodological diversity assessed alongside statistical heterogeneity?
- Do sensitivity analyses test robustness to risk of bias, outliers, model, and metric?
- Is the predicted range of true effects reported alongside the pooled mean?
- Is any study exclusion justified methodologically and shown with and without the study?
- Are post-hoc findings clearly labeled exploratory rather than confirmatory?
