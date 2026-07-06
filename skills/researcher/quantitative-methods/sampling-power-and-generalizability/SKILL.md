---
name: sampling_power_and_generalizability.md
description: Use when the agent is planning a quantitative sample, computing statistical power, choosing a sampling strategy, deciding sample size, handling nonresponse and missing data, or judging whether findings from a sample can generalize to a target population.
---

# Sampling Power And Generalizability

A quantitative study's conclusions depend on two things that are decided before data collection: who or what was sampled, and how many. A sample can be enormous and still misleading if it is biased, and it can be unbiased and still useless if it is too small to detect the effect. Agents often compute sample size as a formality, ignore the distinction between statistical and external generalizability, and treat missing data as a nuisance to gloss over. The result is studies that produce precise estimates of the wrong population, or underpowered studies that find nothing and conclude nothing exists. The discipline is to match the sampling strategy to the inferential target and to size the study for the question it actually asks.

Use this skill when planning samples, computing power, choosing sampling methods, or judging generalizability. The goal is to prevent the agent from producing studies that are confidently wrong or uselessly imprecise.

## Core Rules

### Match The Sampling Strategy To The Inferential Target

Different questions require different sampling. Statistical generalization to a defined population needs probability sampling; transferability or theory development may not.

Choose by target:

- descriptive population estimates require probability sampling and a clear frame;
- causal or explanatory questions require designs that support inference, not just any large sample;
- qualitative or theory-building work may use purposive sampling with stated logic;
- the sampling method must align with the claim type.

A convenience sample can support exploration but cannot support population estimates without strong assumptions.

### Define The Target Population And Sampling Frame Explicitly

Generalization requires knowing what population the sample is meant to represent and the frame from which it was drawn.

Define:

- the target population (who or what the claim is about);
- the sampling frame (the list or mechanism from which sample was drawn);
- the gap between frame and population;
- inclusion and exclusion criteria.

A sample drawn from a frame that misses part of the population generalizes only to the frame, not the population.

### Compute Power For The Smallest Meaningful Effect

Power analysis is only useful if it targets the effect that matters, not the effect that is easy to detect. Underpowered studies miss real effects; overpowered studies detect trivial ones.

Compute power for:

- the smallest effect size of practical or theoretical interest;
- the chosen significance level and desired power;
- the planned design and analysis, including clustering or repeated measures;
- expected attrition and missing data.

Powering for a large effect because it is convenient produces a study that cannot detect the effect you actually care about.

### Account For Design Features That Affect Power

Clustering, repeated measures, and covariates change the effective sample size and power. Ignoring them produces inaccurate power estimates.

Adjust for:

- clustering or nesting, which reduces effective sample size;
- repeated measures, which can increase power if correlated appropriately;
- covariates, which can increase power if predictive;
- the actual analysis planned, not a generic test.

A power analysis for individuals is wrong for a cluster-randomized design.

### Distinguish Statistical From External Generalizability

A study can have strong internal statistical generalization to its sample and weak generalization to a broader population. These are different claims.

Separate:

- statistical generalization: from sample to the population it was drawn from;
- external generalization: from that population to other populations or settings;
- the assumptions required to extend findings beyond the sampled population.

A representative sample of students does not automatically generalize to working adults.

### Anticipate And Plan For Nonresponse

Nonresponse biases results when those who respond differ from those who do not. Ignoring it produces falsely precise estimates.

Plan for:

- expected response rates and their impact on precision;
- the risk of nonresponse bias, not just its effect on sample size;
- follow-up strategies to reduce nonresponse;
- analysis of respondents versus nonrespondents where data allow.

A large sample with low and biased response is worse than a smaller sample with high response.

### Handle Missing Data With Appropriate Methods

Missing data are ubiquitous and their handling changes conclusions. Default approaches like complete-case analysis or pairwise deletion can bias estimates.

Handle by:

- understanding the missingness mechanism (MCAR, MAR, MNAR);
- using appropriate methods such as multiple imputation or full-information likelihood;
- reporting the amount and pattern of missingness;
- conducting sensitivity analysis for the missingness assumption.

Listwise deletion under MAR biases results; ignoring the mechanism hides the risk.

### Report Sampling Limitations Honestly

Every sample has limitations. Naming them lets the reader calibrate the conclusions.

Report:

- the gap between frame and population;
- response rates and nonresponse patterns;
- the amount and handling of missing data;
- deviations from the planned sampling design;
- the scope of generalization the sample supports.

### Consider The Unit Of Analysis And Clustering

Many studies collect data with nesting (students in classes, patients in clinics) that must be reflected in analysis. Ignoring it produces false precision.

Reflect by:

- identifying the unit of analysis and the unit of sampling;
- accounting for intraclass correlation in power and analysis;
- using multilevel models where appropriate;
- avoiding treating clustered observations as independent.

## Common Traps

### Convenience Samples Treated As Representative

A convenient sample generalizes only to itself unless representativeness is argued.

### Powering For Large Or Convenient Effects

Detecting only large effects produces studies that miss the meaningful smaller ones.

### Ignoring Clustering In Power And Analysis

Treating clustered data as independent inflates precision and false positives.

### Confusing Statistical With External Generalizability

A representative sample of one population does not transfer to all populations.

### Ignoring Nonresponse Bias

Low or biased response undermines estimates regardless of sample size.

### Default Missing-Data Methods

Listwise or pairwise deletion under MAR biases results silently.

### Overstating Generalization From Narrow Samples

Findings from students, one clinic, or one country do not universalize without argument.

## Self-Check

- Does the sampling strategy match the type of inference the study claims?
- Are the target population and sampling frame defined, with the gap between them noted?
- Is power computed for the smallest meaningful effect, not the easiest to detect?
- Does the power analysis account for clustering, repeated measures, and the planned analysis?
- Are statistical and external generalizability distinguished, with the latter argued not assumed?
- Is nonresponse anticipated, with its bias risk assessed, not just its effect on size?
- Is missing data handled by methods appropriate to its mechanism, with sensitivity analysis?
- Are sampling limitations, response rates, and missingness reported honestly?
- Is the unit of analysis and any clustering reflected in both power and analysis?
- Is the scope of generalization limited to what the sample actually supports?
