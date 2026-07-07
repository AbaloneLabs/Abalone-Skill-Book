---
name: missing_data_and_imputation.md
description: Use when the agent is handling missing data in an analysis, diagnosing the missingness mechanism, choosing between deletion and imputation methods, specifying multiple imputation or full-information likelihood, running sensitivity analyses for missing-not-at-random data, or reporting how missing data were treated so the analysis remains defensible and transparent.
---

# Missing Data And Imputation

Missing data are the rule in real research, not the exception, and they are where analyses most often become quietly invalid. The damage is not the missingness itself but the unexamined assumption that it can be ignored. Researchers default to dropping incomplete cases because it is easy, impute a mean because it fills the cell, and proceed as if the resulting dataset were complete and representative. Each of these defaults is a hidden assumption about why the data are missing, and when that assumption is wrong, the estimates are biased, the standard errors are wrong, and the conclusions inherit an error that no p-value will reveal. The judgment problem is to diagnose why data are missing, choose a method whose assumptions match that diagnosis, and report the handling so transparently that a reader can judge whether the missingness undermines the claim.

Use this skill when diagnosing missingness, choosing a handling strategy, implementing imputation or likelihood methods, assessing sensitivity to untestable assumptions, and reporting missing-data treatment. The goal is to keep the agent from applying a default method without diagnosing the mechanism, from using approaches known to bias estimates, from treating imputed data as if it were observed, and from hiding the proportion and pattern of missingness from the reader. The agent has latitude in method and software, but the link between missingness mechanism, method choice, and reported uncertainty must be explicit.

## Core Rules

### Quantify And Characterize Missingness Before Choosing A Method

The first step is description, not remedy. The amount, location, and pattern of missingness determine how serious the problem is and which methods are even eligible.

Map the missingness:

- the proportion missing per variable, per record, and per key subgroup;
- whether missingness is concentrated in a few variables or spread across many;
- the pattern: monotone in longitudinal data, intermittent, or item-level within waves;
- whether missingness correlates with observed variables, which informs the mechanism;
- whether missingness on a variable relates to that variable's own unobserved value.

A dataset with five percent scattered missingness is a different problem from one with forty percent missingness concentrated among the sickest participants. Characterizing the pattern is what makes the later method choice defensible.

### Diagnose The Missingness Mechanism As A Reasoned Argument

The taxonomy of missingness mechanisms is not a label to be assigned by software; it is an assumption about reality that the researcher must argue, because it cannot be fully verified from the data at hand.

Reason about:

- missing completely at random, where missingness is independent of all values, observed or not;
- missing at random, where missingness depends only on observed data, so conditioning on observed variables can render it ignorable;
- missing not at random, where missingness depends on the unobserved value itself, which is never fully testable.

The diagnosis rests on substantive knowledge, the pattern of missingness, and auxiliary variables, not on a single test. State the assumed mechanism explicitly, because every valid method is valid only under a specific mechanism, and the most dangerous errors come from assuming ignorability that does not hold.

### Recognize Why Naive Approaches Fail

Listwise deletion, pairwise deletion, and single-value imputation such as mean or last-observation-carried-forward are still common because they are easy, and each can badly bias estimates or distort uncertainty.

Understand the failures:

- listwise deletion is unbiased only under missing completely at random; otherwise it silently drops a non-representative subset and shrinks the sample;
- pairwise deletion produces parameter estimates from different subsamples, yielding inconsistent and often non-positive-definite covariance matrices;
- mean imputation attenuates variance and correlations, biasing every downstream estimate;
- last-observation-carried-forward invents data in longitudinal studies and can manufacture or hide change;
- single imputation treats filled-in values as real, understating standard errors and producing false precision.

These methods are not conservative defaults; they are known sources of bias. Using them without justification is not a neutral choice.

### Match The Method To The Diagnosed Mechanism

Modern methods are valid under specific mechanisms, and using one outside its assumptions imports bias. The choice must follow the diagnosis.

Match deliberately:

- multiple imputation is valid under missing at random, given an adequate imputation model that includes the analysis variables and predictors of missingness;
- full-information maximum likelihood uses all available data under a missing at random assumption and avoids explicit imputation;
- inverse probability weighting reweights observed cases to represent the missing, under a correct model for the missingness probability;
- methods for missing not at random require explicit selection or pattern-mixture models whose untestable assumptions must be probed.

No method rescues data missing for reasons tied to the unobserved value without assumptions that cannot be verified. The method's validity is conditional on assumptions the researcher must state and defend.

### Build The Imputation Model To Support The Analysis

For multiple imputation, the imputation model is not a side detail; it determines whether the imputed values preserve the relationships the analysis will estimate.

Ensure the imputation model:

- includes all variables used in the analysis, including the outcome, exposure, and interactions;
- includes auxiliary variables that predict missingness or the missing values, to make the missing at random assumption more plausible;
- preserves the structure of the data, including multilevel nesting, skew, and bounds;
- reflects the relationships among variables, so imputed values are congenial with the analysis model;
- is run with enough imputations that the imputation-induced noise is small relative to the estimate's uncertainty.

An imputation model that omits the outcome or key predictors produces imputed values that distort the very associations the study estimates. Imputation and analysis models must be compatible.

### Account For Imputation Uncertainty, Do Not Hide It

Imputed values are not measurements; they are draws from a model, and treating them as observed understates uncertainty and inflates significance.

Preserve uncertainty by:

- pooling estimates across imputations using the established combination rules for point estimates and variance;
- separating within-imputation variance from between-imputation variance in the total uncertainty;
- reporting the fraction of missing information, not only the proportion of missing cells;
- checking that the number of imputations is adequate for the stability of standard errors.

A single imputed dataset analyzed as if complete produces confidence intervals that are too narrow and p-values that are too small. The point of multiple imputation is to carry the imputation uncertainty into the inference.

### Run Sensitivity Analyses For The Untestable Assumption

Because the missingness mechanism cannot be fully verified, a defensible analysis probes how the conclusions change under alternative mechanisms, especially missing not at random.

Probe sensitivity by:

- varying the assumed relationship between missingness and the unobserved value;
- using selection models, pattern-mixture models, or delta-adjustment to tilt imputed values;
- reporting how large an MNAR departure would need to be to change the substantive conclusion;
- comparing results under deletion, MAR-based imputation, and plausible MNAR scenarios.

If a small, plausible departure from the MAR assumption reverses the finding, the conclusion is fragile and must be reported as such. Sensitivity analysis is the honest response to an assumption that cannot be confirmed.

### Report Missingness Handling Transparently

Readers cannot judge a claim if they cannot see how much data were missing and what was done about it. Omitting this hides a load-bearing assumption.

Report:

- the amount and pattern of missingness per variable and per analysis;
- the assumed mechanism and the reasoning behind it;
- the method chosen and why it matches that mechanism;
- the imputation model, auxiliary variables, and number of imputations;
- the fraction of missing information and sensitivity analysis results.

A methods section that says missing data were handled by multiple imputation, with no further detail, conceals every assumption that determines whether the estimate is trustworthy.

## Common Traps

### Default Listwise Deletion

Dropping incomplete cases without diagnosing the mechanism is valid only under missing completely at random and silently biases the sample otherwise.

### Mean Or Single Imputation

Filling cells with a single value attenuates variance, distorts correlations, and produces false precision by ignoring imputation uncertainty.

### Assuming Missing At Random Without Argument

Treating MAR as a default rather than a defended assumption hides the untestable condition on which the entire analysis rests.

### Imputation Model Omitting The Outcome

Imputing predictors without the analysis outcome distorts the associations the study is designed to estimate.

### Treating Imputed Values As Observed

Analyzing a single imputed dataset as if complete understates uncertainty and inflates significance.

### Ignoring The Multilevel Structure

Imputing clustered or longitudinal data with a flat model destroys the within-cluster correlation that the analysis depends on.

### Skipping Sensitivity Analysis

Failing to probe how conclusions shift under missing not at random leaves a fragile finding presented as robust.

### Undisclosed Missingness Handling

Reporting results without the amount, pattern, mechanism assumption, method, and sensitivity hides the assumptions that determine credibility.

## Self-Check

- Is the amount and pattern of missingness quantified per variable, record, and key subgroup before any method is chosen?
- Is the missingness mechanism diagnosed as a reasoned argument, with the assumed mechanism stated explicitly?
- Are naive approaches such as listwise deletion, mean imputation, and last-observation-carried-forward avoided or explicitly justified?
- Is the chosen method, whether multiple imputation, full-information likelihood, or weighting, matched to the diagnosed mechanism?
- Does the imputation model include all analysis variables, the outcome, interactions, and auxiliary predictors of missingness?
- Does the imputation preserve the data's structure, including nesting, skew, bounds, and longitudinal correlation?
- Is imputation uncertainty propagated into the final estimates through proper pooling and the fraction of missing information?
- Is a sensitivity analysis run to test how conclusions change under plausible missing-not-at-random departures?
- Are the amount, pattern, mechanism assumption, method, imputation model, number of imputations, and sensitivity results reported transparently?
- Would a reader be able to judge whether the missingness undermines the study's central claim?
