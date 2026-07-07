---
name: data_quality_assurance_and_cleaning.md
description: Use when the agent is setting up data entry and validation rules, checking data quality during or after collection, detecting outliers, diagnosing missing data patterns, deciding whether to exclude or correct data points, or documenting data cleaning so the analysis remains reproducible and unbiased.
---

# Data Quality Assurance And Cleaning

Data cleaning is where raw measurements become analyzable data, and it is also where researchers quietly introduce bias, manufacture findings, or destroy the ability to reproduce results. The danger is not that cleaning happens; some cleaning is almost always necessary. The danger is that cleaning is treated as housekeeping: a series of unrecorded judgment calls about what counts as an error, what counts as an outlier, what to impute, and what to drop. Each such call, when made after seeing the results or without documentation, shifts the dataset toward the researcher's expectations. A dataset that has been "cleaned" without a recorded plan is a dataset whose relationship to the original measurements is unknown, and any inference drawn from it inherits that uncertainty without acknowledging it.

Use this skill when building data collection systems, writing data entry and validation procedures, checking data quality, handling outliers and anomalies, diagnosing missingness, deciding on exclusions and corrections, and documenting the cleaning pipeline. The goal is to keep the agent from cleaning toward a result, from confusing statistical anomaly with substantive error, from deleting inconvenient observations, and from leaving the cleaning process untraceable. The agent has latitude in tools and thresholds, but every transformation that changes the analytic dataset must be deliberate, justified, and recoverable.

## Core Rules

### Prevent Errors At Entry Rather Than Cure Them After

The cheapest and least biased correction is the one never needed. Quality assurance built into collection beats quality control applied afterward, because entry-time constraints prevent errors that later cleaning can only guess at.

Build prevention into:

- constrained response formats and range checks in electronic forms;
- required fields, type validation, and skip logic that reflect the instrument;
- double data entry or verification for high-stakes manual transcription;
- automated consistency checks across related fields at capture time;
- immediate flagging of out-of-range or logically impossible values for resolution while context is fresh.

A value caught at entry can be corrected against the source. A value caught months later in a spreadsheet often cannot be reconstructed and is either dropped or guessed, both of which inject uncertainty. Design the collection instrument so that many errors cannot occur.

### Separate Cleaning From Analysis And Predefine It Where Possible

Cleaning choices made while looking at results are contaminated choices. If the analyst knows which exclusion moves the finding toward significance, the exclusion is no longer neutral.

Protect the integrity of cleaning by:

- writing a data cleaning plan before unblinded analysis where feasible;
- preregistering exclusion rules, outlier thresholds, and recoding logic when the design allows;
- executing cleaning in a scripted, version-controlled pipeline rather than by hand editing;
- freezing an analytic dataset before hypothesis testing begins;
- documenting every deviation from the plan and the reason for it.

Hand editing a spreadsheet is never acceptable as the cleaning record, because it leaves no auditable trail. The cleaning pipeline must be reproducible: another researcher running the same script on the raw data must obtain the same analytic dataset.

### Distinguish Four Kinds Of Anomaly Before Acting

Not every strange value is an error, and not every error should be removed. Conflating anomaly, error, outlier, and influential point leads to bad decisions.

Classify each flagged observation:

- entry or transcription error, where the recorded value is wrong relative to the source;
- measurement error, where the instrument captured something other than the construct;
- legitimate extreme value, where the observation is real and meaningful and belongs in the analysis;
- influential point, where the value is correct but disproportionately drives the estimate.

The response differs by type. Errors are corrected against the source or, if unrecoverable, set to missing and handled through missing-data methods. Legitimate extremes are retained and analyzed with methods robust to them. Removing a legitimate extreme because it is inconvenient is a form of result-driven cleaning that distorts the population being described.

### Decide Outlier Treatment By Substantive And Statistical Logic Together

Outlier detection is not a mechanical rule applied to a boxplot. A value can be statistically extreme and substantively ordinary, or statistically ordinary and substantively impossible.

Judge outliers against:

- the known plausible range of the variable from theory and measurement;
- the distributional context, including expected skew and heavy tails;
- whether the value is extreme on one variable or a multivariate outlier given the joint pattern;
- the influence of the value on the specific estimate of interest, not just on overall fit;
- whether removal, winsorizing, transformation, or robust estimation best preserves representativeness.

Report which observations were affected, the rule used, and a sensitivity analysis showing the estimate with and without the treatment. An outlier rule applied silently and differently across analyses is a route to hidden bias.

### Diagnose Missing Data As A Pattern Before Choosing A Method

Missingness is information. The pattern of what is missing, and why, determines whether the analysis is biased and which remedy is valid. Treating all missingness the same is the most common cleaning error.

Diagnose:

- the amount of missingness per variable, per record, and per key subgroup;
- whether missingness is concentrated or scattered, monotone in longitudinal data, or item-level;
- the likely mechanism: missing completely at random, missing at random given observed data, or missing not at random depending on the unobserved value itself;
- whether missingness is related to the outcome, to treatment, or to auxiliary variables;
- whether the missingness was planned, such as matrix sampling, or unplanned.

The mechanism is not testable in full, so the diagnosis is a reasoned argument supported by data, not a certificate. The handling method must be compatible with the assumed mechanism, and the assumption must be stated and, where possible, stress-tested.

### Make Exclusion Decisions Explicit And Defensible

Dropping observations reduces the data and can change who the sample represents. Exclusions must be traceable to predefined criteria, not to the desirability of the result.

For each exclusion rule, record:

- the criterion and the substantive or procedural reason for it;
- whether the rule was prespecified or added after seeing data;
- the number and characteristics of excluded observations;
- the effect of exclusion on sample composition and on the estimate;
- an analysis including the excluded observations where feasible, as a sensitivity check.

Exclusions applied differently to treatment and control, to responders and nonresponders, or to high and low scorers are not neutral cleaning; they are analytic choices that shape the inferential population. The flow from raw collected data to the final analytic sample should be diagrammed and reported.

### Preserve Provenance And Reproducibility Through Documentation

A cleaned dataset without documentation is a black box. The reader must be able to see how raw data became analytic data and judge whether the transformations were defensible.

Document:

- the source and version of the raw data;
- every transformation, recode, derivation, and unit conversion, with rationale;
- the handling of each variable with missingness, outliers, or errors;
- the software, package versions, random seeds, and scripts used;
- the mapping from raw to analytic variables, including renamed and combined fields;
- the date and authorship of each cleaning step.

Reproducibility means that the analytic dataset can be regenerated from the raw data by rerunning the pipeline. If cleaning was done by hand in a proprietary tool with no script, the dataset is not reproducible and its findings are not independently checkable.

### Balance Cleaning Rigor Against The Risk Of Introducing Bias

More cleaning is not always better. Aggressive correction of every anomaly can remove real signal, flatten genuine variation, and steer the data toward an expected distribution. The aim is fidelity to the phenomenon, not tidiness.

Balance by:

- preferring correction over deletion where the source allows recovery;
- preferring retention with robust methods over removal of legitimate extremes;
- avoiding transformations chosen because they normalize a distribution and improve results;
- checking that cleaning did not differentially affect groups, conditions, or outcomes;
- reporting the magnitude of change introduced by cleaning, not only that it occurred.

A dataset that has been scrubbed until it looks clean may have been scrubbed until it no longer represents the population. The standard is not cleanliness but defensible fidelity to what was measured.

## Common Traps

### Cleaning Toward A Desired Result

Adjusting exclusions, outliers, or transformations after seeing outcomes steers the data toward significance and is a form of analysis flexibility that inflates false positives.

### Treating Outliers As Errors By Default

Removing statistically extreme but legitimate values distorts the distribution and biases estimates, especially when removal is uneven across groups.

### Silent Hand Editing

Correcting values directly in a spreadsheet without a script or audit trail makes the cleaning irreproducible and hides judgment calls from reviewers.

### Ignoring The Missingness Mechanism

Applying listwise deletion or mean imputation regardless of why data are missing can introduce substantial bias when missingness depends on the unobserved value or on the outcome.

### Confusing Data Quality With Data Tidiness

Reshaping data for convenience, renaming variables, and merging files are necessary, but they are not the same as verifying that values are correct and representative.

### Undocumented Derivations

Computed variables, unit conversions, and recodes that are not written down become invisible assumptions that later researchers cannot verify or challenge.

### Differential Cleaning Across Groups

Applying stricter exclusions, different outlier rules, or more imputation to one condition or subgroup than another changes the comparison being estimated.

### Overcleaning Until Signal Disappears

Aggressively removing anomalies can strip genuine heterogeneity and heavy tails that are real features of the population, producing precise estimates of the wrong quantity.

## Self-Check

- Are entry-time validation, range checks, and consistency checks built into collection to prevent errors before they are recorded?
- Is the cleaning plan separated from unblinded analysis, preregistered where feasible, and executed in a reproducible scripted pipeline?
- Is every flagged anomaly classified as entry error, measurement error, legitimate extreme, or influential point, with the response matched to the type?
- Are outlier rules justified by substantive and statistical logic together, with affected observations reported and a sensitivity analysis provided?
- Is the missingness pattern diagnosed by amount, mechanism assumption, and subgroup, and is the handling method compatible with that mechanism?
- Are exclusion criteria explicit, prespecified where possible, counted, characterized, and tested for their effect on composition and estimates?
- Is there a documented flow from raw data to the analytic sample that another researcher could follow?
- Are all transformations, recodes, derivations, unit conversions, software versions, seeds, and scripts recorded for reproducibility?
- Has cleaning been checked for differential effects across groups, conditions, and outcomes?
- Does the cleaned dataset preserve fidelity to the measured phenomenon rather than merely looking tidy, and are the limits introduced by cleaning acknowledged?
