---
name: statistical_inference_and_p_values.md
description: Use when the agent is running inferential statistics, interpreting p-values and confidence intervals, deciding statistical significance, avoiding p-hacking and multiple-testing errors, choosing estimation versus hypothesis testing, or writing up quantitative results honestly.
---

# Statistical Inference And P-Values

Statistical inference is where data becomes a claim, and it is also where most quantitative research goes wrong in ways that are invisible in the final report. A p-value does not measure the probability that a hypothesis is true, a significant result does not mean an effect is important, and a non-significant result does not mean no effect exists. Agents routinely overinterpret p-values, run many tests and report only the significant ones, and treat estimation as secondary to binary significance decisions. The discipline is to use inference as a tool for calibrated uncertainty, not as a machine for producing publishable "findings."

Use this skill when running or interpreting inferential statistics, managing multiple comparisons, choosing between testing and estimation, or writing up quantitative results. The goal is to prevent the agent from producing statistically flavored overclaims and from mistaking significance for truth.

## Core Rules

### Understand What A P-Value Does And Does Not Mean

Misinterpretation of the p-value is the root of most inference errors. It must be used for what it is.

Recall:

- a p-value is the probability of data at least as extreme as observed, assuming the null is true;
- it is not the probability that the null is true;
- it is not the probability that the result is due to chance;
- it does not measure the size or importance of an effect;
- a small p-value does not confirm the research hypothesis.

State inference in terms compatible with what the statistic actually supports.

### Report Effect Sizes And Confidence Intervals, Not Just P-Values

Significance without magnitude is uninformative. An effect can be statistically significant and trivially small, or non-significant and meaningfully large.

Always report:

- the effect size estimate with its confidence interval;
- the practical or theoretical meaning of the magnitude;
- the precision of the estimate, not just whether it crossed a threshold;
- the direction of the effect.

Estimation frames the result as "how large, with what uncertainty" rather than "significant or not."

### Control The Familywise Or False-Discovery Error Rate

Running many tests inflates the chance of false positives. Uncontrolled multiplicity is a primary route to spurious findings.

Address multiplicity by:

- identifying all tests run, not only those reported;
- pre-specifying the primary analysis;
- correcting for multiple comparisons where many are tested;
- distinguishing confirmatory from exploratory analyses;
- reporting all comparisons, significant or not.

Selective reporting of significant results from many tests is p-hacking regardless of intent.

### Distinguish Statistical Significance From Practical Importance

A threshold p-value says nothing about whether an effect matters. Conflating the two produces both overclaims and underclaims.

Separate by:

- defining what magnitude would be practically or theoretically meaningful in advance;
- comparing the effect size to that benchmark, not to the p-value;
- noting when a significant effect is too small to care about;
- noting when a non-significant effect is consistent with a meaningful magnitude.

### Avoid Optional Stopping And Data-Driven Analysis Choices

Peeking at data and deciding whether to continue, or choosing analysis options based on which gives significance, inflates false positives.

Pre-specify and lock:

- the sample size or stopping rule;
- the primary outcome and analysis;
- covariates, transformations, and exclusions;
- the handling of outliers and missing data.

Any data-dependent choice should be reported as exploratory and subjected to sensitivity analysis.

### Interpret Non-Significance Correctly

A non-significant result is not evidence of no effect. With low power or imprecise estimates, a meaningful effect may simply be undetected.

Interpret by:

- examining the confidence interval, not just the p-value;
- checking whether the interval is consistent with a meaningful effect;
- distinguishing "no evidence of an effect" from "evidence of no effect";
- the latter requires the interval to exclude meaningful magnitudes.

Equivalence or non-inferiority questions require designs and analyses built for them.

### Check Model Assumptions Before Trusting Output

Every inferential procedure rests on assumptions. Output from a violated model is misleading regardless of its p-value.

Check:

- distributional assumptions (normality, variance structure);
- independence and correlation structure;
- linearity or functional form;
- homoscedasticity where assumed;
- the impact of any violation on the conclusions.

Report assumption checks and sensitivity to violations, not just the final statistic.

### Separate Confirmatory From Exploratory Analysis

Confirmatory analyses test pre-specified hypotheses; exploratory analyses search for patterns. Conflating them inflates false positives and overclaims.

Label clearly:

- which analyses were pre-specified and confirmatory;
- which were generated by examining the data and are exploratory;
- that exploratory findings are hypothesis-generating, not confirmed;
- the implications for how strongly results can be stated.

### Preregister Or Lock The Analysis Plan Where Possible

The credibility of inferential results depends on the analysis preceding the data. Preregistration or an internal locked plan provides that evidence.

Lock in advance:

- hypotheses, primary outcomes, and analysis;
- sample size and stopping rules;
- exclusions and handling of missing data;
- the boundary between confirmatory and exploratory.

Without a prior plan, readers cannot distinguish prediction from post-hoc explanation.

## Common Traps

### Treating P-Values As Truth Probabilities

A small p-value is not the probability the hypothesis is true; misreading it inflates confidence.

### Significance Without Effect Size

Reporting only p-values hides whether the effect is meaningful or trivially small.

### Uncontrolled Multiple Testing

Running many tests and reporting the significant ones manufactures false positives.

### Optional Stopping

Peeking and continuing until significance inflates the false-positive rate.

### Non-Significance Read As No Effect

Absence of a detected effect is not evidence of absence, especially with low power.

### Unchecked Model Assumptions

Inferential output from a model with violated assumptions is unreliable.

### Exploratory Findings Presented As Confirmatory

Post-hoc patterns labeled as hypothesis tests overclaim what the data support.

## Self-Check

- Is the p-value interpreted as evidence against the null, not as the probability the hypothesis is true?
- Are effect sizes and confidence intervals reported alongside any significance test?
- Is multiplicity addressed through pre-specification, correction, or clear labeling of exploratory tests?
- Are analysis choices (sample size, outcomes, covariates, exclusions) locked before or independent of results?
- Is a non-significant result interpreted via its confidence interval, not declared "no effect"?
- Are model assumptions checked and the sensitivity of conclusions to violations reported?
- Are confirmatory and exploratory analyses clearly separated and labeled?
- Is the practical or theoretical importance of the effect assessed against a pre-defined magnitude?
- Is the analysis plan preregistered or internally locked where possible?
- Are all tests run reported, not only those that reached significance?
