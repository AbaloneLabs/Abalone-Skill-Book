---
name: effect_size_and_power_analysis.md
description: Use when the agent is computing or interpreting effect sizes, planning sample size through power analysis, distinguishing types of effect size, judging whether a study was adequately powered, interpreting null results, or defending the practical importance and reproducibility of a finding.
---

# Effect Size And Power Analysis

Statistical significance tells you whether an effect was detectable; effect size and power tell you whether it matters and whether the study could find it. Researchers routinely report p-values without effect sizes, power studies after the fact to excuse null results, and sample sizes chosen by convenience. The consequence is a literature where "significant" effects are too small to matter, where "non-significant" effects are merely underpowered, and where replications fail because the original estimate was inflated.

Use this skill when computing or interpreting effect sizes, when planning sample size, when judging whether a study was adequately powered, and when interpreting null or small effects. The goal is to keep the agent from confusing detectability with importance, from running post hoc power analyses that restate the result, and from designing studies that cannot answer their own question. The agent has latitude in metrics and software, but must justify the effect size, the power, and the sample size against the substantive question.

## Core Rules

### Report Effect Sizes Alongside Every Significance Test

A p-value says nothing about magnitude. An effect size quantifies the size of the effect in a way that can be compared across studies and judged for practical importance.

Report:

- a standardized effect size where comparison across scales is useful;
- the raw effect in meaningful units where interpretation is local;
- the confidence interval around the effect size;
- the direction of the effect;
- the metric in which the effect is expressed;
- why this effect size metric was chosen.

A result reported only as "significant" or "non-significant" hides the quantity that matters most for interpretation and synthesis.

### Choose The Effect Size Metric For The Question

Different effect size metrics answer different questions, and the choice should be deliberate.

Common families:

- standardized mean differences, such as Cohen d or Hedges g, for group comparisons on a common scale;
- risk ratios, odds ratios, and risk differences for binary outcomes;
- correlation coefficients for association;
- R-squared or partial eta-squared for variance explained;
- raw units, such as points or millimeters, for clinical or practical interpretation;
- rate ratios and hazard ratios for time-to-event outcomes.

Each has assumptions, limits, and contexts where it is appropriate. Reporting the wrong metric can obscure or distort the finding.

### Interpret Effect Sizes Against Practical Importance, Not Generic Labels

Generic labels such as "small," "medium," and "large" are field- and context-dependent heuristics, not universal truths. An effect that is small in one context can be life-changing in another.

Judge importance against:

- the minimal clinically or practically important difference;
- the cost, risk, and effort of the intervention;
- the baseline level and the room for change;
- the population and stakes;
- comparisons with alternative interventions or effects;
- the cumulative effect over time or across people;
- the distributional consequences, not only the mean.

A "small" effect on a common and serious outcome can matter more than a "large" effect on a rare or trivial one.

### Plan Sample Size Through A Priori Power Analysis

Sample size should be chosen before data collection based on the effect the study is designed to detect, the desired power, and the significance level. Convenience samples produce studies that cannot answer their question.

Specify:

- the smallest effect size of interest, grounded in theory, prior evidence, or practical importance;
- the desired power, commonly 0.8 or higher, and its justification;
- the significance level and whether tests are one- or two-sided;
- the design, including groups, repeated measures, and clustering;
- the expected attrition and missing data;
- the analysis model and its assumptions;
- adjustments for multiplicity if relevant.

Document the calculation. A sample size without a stated detectable effect is a number, not a plan.

### Ground The Detectable Effect In Evidence, Not Optimism

The effect size used in power analysis determines everything. Plugging in an optimistic estimate from a flexible prior study sets up a study destined to fail or to overclaim.

Source the estimate carefully:

- prefer meta-analytic estimates over single studies;
- adjust for publication bias and inflation in published effects;
- consider the minimal important effect, not only the expected effect;
- account for differences in population, setting, and delivery;
- use pilot data cautiously, given its instability;
- run sensitivity analyses across plausible effect sizes.

A power analysis built on an inflated effect produces an underpowered study that will fail even if a real, smaller effect exists.

### Avoid Post Hoc Power Analyses That Restate The Result

Post hoc or "observed" power, computed from the observed effect and the actual sample, adds no information beyond the p-value and confidence interval. It is a restatement, not an independent assessment.

Instead, to interpret a non-significant result:

- examine the confidence interval and what effects it rules out;
- compare the upper bound to the minimal important effect;
- report the smallest effect the study could have detected with adequate power;
- discuss whether the study was designed to detect effects of practical importance;
- avoid claiming "no effect" when the interval includes important values.

If a study was underpowered, say so based on the a priori calculation, not a post hoc one.

### Distinguish Power To Detect From Power To Rule Out

A study can be designed to detect an effect or to rule one out. These are different inferential goals and require different planning.

Clarify:

- superiority designs test for an effect above a threshold;
- equivalence designs test whether effects fall within a margin;
- non-inferiority designs test whether an effect is not worse than a threshold;
- each requires different power calculations and different interpretations of null results;
- a non-significant superiority test is not evidence of equivalence unless designed for it.

A study designed for superiority cannot, by failing to reject, prove that the effect is absent or negligible.

### Account For Design Features That Affect Power

Power calculations that ignore clustering, repeated measures, attrition, and multiplicity overstate the effective sample and understate the required size.

Adjust for:

- clustering in cluster-randomized or multilevel designs;
- correlation in repeated measures and the design effect;
- attrition and the resulting reduction in analyzable sample;
- multiple primary outcomes and the correction that follows;
- interim analyses and the alpha spent;
- heterogeneity of treatment effects across subgroups.

A power calculation that treats clustered data as independent is optimistic by a factor that depends on the intraclass correlation.

## Common Traps

### Reporting Significance Without Effect Sizes

A p-value hides magnitude. Without an effect size and interval, readers cannot judge importance or compare across studies.

### Using Generic Small-Medium-Large Labels

Cohen's rough labels were never meant as universal cut-offs. Importance depends on context, stakes, and alternatives.

### Powering For An Inflated Prior Effect

Published effects are often exaggerated. Powering for them produces studies that miss real but smaller effects.

### Post Hoc Power To Explain Null Results

Observed power restates the p-value and adds nothing. It cannot rescue an underpowered study after the fact.

### Convenience Sample Sizes

Sample sizes chosen by time, budget, or habit rather than calculation produce studies of unknown sensitivity.

### Claiming No Effect From A Non-Significant Result

A non-significant superiority test is compatible with no effect, a small effect, or an underpowered study. It is not evidence of absence.

### Ignoring Clustering And Design Effects

Treating clustered or repeated data as independent inflates power and produces false precision.

## Self-Check

- [ ] Is an appropriate effect size reported with its confidence interval alongside every significance test?
- [ ] Is the effect size metric chosen to match the question, with raw and standardized forms considered?
- [ ] Is the effect interpreted against practical importance and context, not generic labels?
- [ ] Was sample size planned through an a priori power analysis with a justified detectable effect?
- [ ] Is the detectable effect grounded in prior evidence, meta-analysis, or minimal importance rather than optimism?
- [ ] Are post hoc power analyses avoided in favor of confidence interval interpretation for null results?
- [ ] Is the inferential goal, superiority, equivalence, or non-inferiority, matched to the power calculation?
- [ ] Are design features, including clustering, repeated measures, attrition, and multiplicity, accounted for in power?
- [ ] Are null results interpreted by what the confidence interval rules out, not by absence of significance?
- [ ] Is the practical importance and reproducibility of the effect defended, not merely its detectability?
