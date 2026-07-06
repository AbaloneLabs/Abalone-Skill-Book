---
name: hypothesis_testing_and_inference.md
description: Use when the agent is choosing a statistical test, checking its assumptions, interpreting p-values confidence intervals and Bayesian quantities, controlling error rates across tests, deciding between frequentist and Bayesian frameworks, or defending whether the chosen inference supports the substantive claim.
---

# Hypothesis Testing And Inference

Statistical inference is the bridge from a sample to a claim about the world, and it is the bridge where most quantitative research collapses. Researchers run a test, obtain a p-value, and declare a finding, often without checking assumptions, without correcting for multiplicity, and without distinguishing statistical from substantive significance. The machinery of inference is precise, but it licenses only the conclusions its assumptions and design support. Misusing it produces a literature of false positives and overconfident effects.

Use this skill when choosing a test, when checking assumptions, when interpreting p-values and intervals, when handling multiple tests, and when defending the inference. The goal is to keep the agent from ritualistic significance testing, from ignoring assumptions that license the test, and from overreading a p-value into a causal or practical claim. The agent has latitude in framework, but must justify the inferential chain from data to conclusion.

## Core Rules

### Choose The Test From The Design And The Data, Not From Habit

The right test is determined by the question, the design, the level of measurement, the distribution, the dependence structure, and the sample. Defaulting to a familiar test produces invalid inference when these differ.

Match the test to:

- the type of question, such as difference, association, or prediction;
- the number and independence of groups;
- the level of measurement of the variables;
- the distributional shape and presence of outliers;
- the dependence structure, including clustering and repeated measures;
- the sample size relative to the number of parameters;
- the planned estimand.

Document why this test, and name the assumptions it requires. A test chosen by habit is a test chosen without justification.

### Check Assumptions Before Trusting The Output

Every parametric test assumes something about the data. Violated assumptions can change both estimates and standard errors, sometimes dramatically.

Check:

- independence or correct modeling of dependence;
- normality of residuals where assumed;
- equality of variances where assumed;
- linearity where assumed;
- absence of influential outliers;
- adequate sample size for asymptotic results;
- correct link function for the model;
- missing data mechanism.

Use diagnostics, not assertions. Where assumptions fail, choose a robust alternative, transform with care, or use a method whose assumptions fit.

### Interpret P-Values As Continuous Evidence, Not Verdicts

A p-value is the probability of data at least as extreme as observed under a null hypothesis. It is not the probability that the hypothesis is true, nor the probability that the result is due to chance.

Interpret carefully:

- a small p-value is evidence against the null, not proof of a real effect;
- a large p-value is compatible with no effect, a small effect, or an underpowered study;
- the threshold of 0.05 is conventional, not magical;
- p-values are sensitive to sample size, so report effect sizes alongside;
- p-values from multiple tests are not independent evidence;
- p-values shift under optional stopping and analysis flexibility.

Report the exact p-value and the estimate with its interval, not just a significance flag.

### Report Confidence Intervals Alongside Point Estimates

A confidence interval expresses the precision of an estimate and the range of values compatible with the data. It carries more information than a p-value alone.

Use intervals to:

- show the plausible range of the effect;
- reveal whether an effect is precise enough to matter;
- expose whether the interval includes practically important values;
- communicate uncertainty honestly;
- support equivalence and non-inferiority reasoning.

Interpret intervals correctly: a 95 percent interval is one that would cover the true value in 95 percent of repeated samples, not a 95 percent probability that the true value lies in this particular interval under a frequentist framework.

### Control Error Rates Across Multiple Tests

Each additional test inflates the chance of a false positive somewhere in the analysis. Multiplicity is not optional bookkeeping.

Address:

- declare the family of tests in advance;
- choose a correction, such as Bonferroni, Holm, or false discovery rate control;
- distinguish confirmatory from exploratory tests;
- account for subgroup, outcome, and time-point multiplicity;
- account for model selection and covariate testing;
- report both unadjusted and adjusted results where helpful.

A family of fifty tests with no correction will almost certainly produce false positives. The correction must match the family.

### Distinguish Statistical Significance From Practical Importance

A result can be statistically significant and practically trivial, or practically important and statistically non-significant. These are different questions.

Report:

- the effect size with its interval;
- the minimal practically important difference;
- whether the interval lies entirely above or below that threshold;
- the cost, risk, and context that define practical importance;
- the uncertainty that remains.

Significance is a property of the test against a threshold; importance is a property of the effect in the world. Conflating them misleads readers and decisions.

### Choose Between Frequentist And Bayesian Frameworks Deliberately

The two frameworks answer different questions and require different inputs. The choice should be conscious and justified.

Consider:

- frequentist methods control long-run error rates and use p-values and intervals;
- Bayesian methods produce posterior distributions and require priors;
- priors should be justified and sensitivity-tested;
- Bayesian analysis can incorporate external evidence but is sensitive to prior choice;
- both frameworks require correct models and assumptions;
- mixing frameworks without thought produces incoherent inference.

State the framework and its rationale. Do not switch frameworks mid-analysis to obtain a preferred result.

### Pre-Specify The Analysis To Avoid Flexibility

Analytical flexibility, the freedom to choose among many defensible analyses, inflates false positives. Pre-specification constrains this freedom.

Pre-specify:

- the primary outcome and estimand;
- the primary model and covariates;
- the handling of missing data;
- the treatment of outliers and transformations;
- subgroup and sensitivity analyses;
- the decision rule for the primary test.

Deviations are legitimate when reported as such. Presenting a post hoc analysis as the planned one is a form of result-driven inference.

## Common Traps

### Ritualistic Significance Testing

Running a default test and reporting a p-value, without checking assumptions or reporting effect sizes, reduces inference to ceremony.

### Ignoring Multiplicity

Many tests with no correction manufacture false positives and inflate the apparent strength of findings.

### Equating Significance With Importance

A tiny effect can be significant in a large sample; a large effect can be non-significant in a small one. Report effect sizes and intervals.

### Treating A Large P-Value As Evidence Of No Effect

Failure to reject is compatible with no effect, a small effect, or an underpowered study. It is not proof of absence.

### P-Hacking Through Analysis Flexibility

Trying many analyses and reporting the one that crossed a threshold produces false positives dressed as findings.

### Violating Assumptions Silently

Parametric tests rely on assumptions. Ignoring diagnostics does not make violations disappear; it makes the inference invalid.

### Switching Frameworks For Convenience

Moving between frequentist and Bayesian approaches to obtain a preferred result is incoherent inference.

## Self-Check

- [ ] Is the test chosen to match the question, design, measurement level, distribution, and dependence structure?
- [ ] Are assumptions checked with diagnostics, and addressed through robust methods or justified transformations?
- [ ] Are p-values reported as continuous evidence with exact values, not as binary verdicts?
- [ ] Are confidence intervals reported alongside point estimates and interpreted for precision and practical range?
- [ ] Is multiplicity addressed with a declared family and an appropriate correction?
- [ ] Is statistical significance distinguished from practical importance, with effect sizes and minimal important differences?
- [ ] Is the inferential framework, frequentist or Bayesian, chosen deliberately with priors justified and sensitivity-tested?
- [ ] Is the primary analysis pre-specified, with deviations reported as exploratory or sensitivity analyses?
- [ ] Are missing data mechanisms considered and handled with appropriate methods?
- [ ] Are conclusions calibrated to the strength of the inference rather than to a single threshold crossing?
