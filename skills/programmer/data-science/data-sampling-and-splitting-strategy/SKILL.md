---
name: data_sampling_and_splitting_strategy.md
description: Use when the agent is splitting data into train, validation, and test sets, choosing between random and temporal splits, designing cross-validation for small datasets, handling class imbalance via stratified or resampling strategies, avoiding group leakage when multiple rows share an entity (user, store, device, document), deciding sample sizes, or reviewing an evaluation protocol for honesty. Also covers the failure modes of random splits on time-series that leak the future into the past, group leakage where the same entity appears in train and test, validation-set overfitting from repeated tuning, unstratified splits that destabilize rare-class metrics, tiny test sets whose scores are dominated by sampling noise, and the recurring mistake of trusting a single holdout split whose score overstates real generalization.
---

# Data Sampling And Splitting Strategy

The evaluation protocol — how data is divided into train, validation, and test — determines whether a measured score tells you anything about real-world performance. The default split (shuffle the rows, take a random fraction for test) is correct only under assumptions that rarely hold: rows are independent, there is no temporal ordering, no entity spans multiple rows, and the test distribution matches deployment. Violate any and a clean-looking score becomes a lie. A random split on time-series trains on the future and tests on the past. A split that puts the same user in both train and test measures memorization, not generalization. Repeatedly tuning on a single validation set fits the validation set, leaving no honest estimate.

Agents miss these problems because the splitting step is a one-liner in most libraries and produces a number that looks decisive. The harm is overconfident models that underperform the moment they ship. A model evaluated on a split that shares users with training scores near-perfectly and collapses on truly unseen users. An imbalanced classification with an unstratified split has its tiny positive class land almost entirely in training, so test-set recall is based on three examples. A small test set produces a score whose confidence interval spans the entire decision range, yet is reported to two decimal places. The judgment problem is to design the split around the structure of the data and the deployment reality — respecting time, grouping, imbalance, and sample size — and to treat the test set as a scarce, once-used resource.

## Core Rules

### Match The Split To The Data Structure: Time, Groups, And Distribution

The first decision is what kind of split the data's structure demands, because the wrong kind leaks or tests the wrong population.

- **Time-series and any data with temporal ordering require a temporal split.** Train on the past, validate and test on the future, with the split point set by the deployment horizon. A random split lets the model train on data that post-dates the test period — the most damaging leakage in temporal data.
- **Grouped data require group-aware splits.** When rows share an entity (a user's sessions, a store's days), random splitting scatters the same entity across train and test, and the model memorizes entity effects rather than generalizing. Split by group (GroupKFold) so every entity lives in one set.
- **Match the test distribution to the deployment distribution.** If the model will score new markets or a future window, the test set should approximate that out-of-distribution reality (hold out entire markets or a forward time block). Random splits test interpolation; deployment is often extrapolation.
- **Choose the split kind before the fraction.** The kind (temporal, grouped, stratified, out-of-distribution) determines honesty; the fraction is secondary.

### Treat The Test Set As A Once-Used Resource

The test set's value is that the model has never influenced it; every look erodes that value.

- **Hold out a single test set, touched once at the end.** Use validation (and cross-validation) for all model selection, tuning, and feature iteration; reserve the test set for the final estimate of generalization.
- **Repeated tuning on the validation set is validation overfitting.** If you run hundreds of experiments and pick the best validation score, you have fit the validation set. Use nested cross-validation or a fresh holdout to estimate the cost of selection.
- **Do not retrain on train-plus-validation and report validation as the final score.** That conflates selection and evaluation. The final honest number comes from the untouched test set.
- **When the test set is consumed, acknowledge it.** Once used for a decision, it is no longer unbiased; plan a new holdout.

### Use Cross-Validation Honestly For Small Data, And Respect Its Assumptions

When data is too small for a reliable holdout, cross-validation extracts more honest estimates — but only if its structure respects the data.

- **Use k-fold CV for small i.i.d. datasets.** It reduces the variance of the performance estimate versus a single split and uses more data for training. Choose k (commonly 5 or 10) trading bias against variance and cost.
- **Use the right CV variant for the data structure.** Time-series needs rolling or expanding-window CV; grouped data needs GroupKFold; imbalanced classification needs StratifiedKFold. Plain k-fold on any of these leaks or destabilizes.
- **Nested CV when selection is part of the pipeline.** If you tune hyperparameters or select features, outer-fold evaluation with inner-fold selection gives an honest estimate including the cost of model selection.
- **Report the variance across folds, not only the mean.** A mean accuracy of 0.90 with folds ranging from 0.70 to 0.99 is far less stable than the headline suggests.

### Handle Class Imbalance In Both Splitting And Sampling

Imbalanced classes destabilize metrics and can be handled by stratification, resampling, or weighting, each with tradeoffs.

- **Stratify the split to preserve class ratios.** An unstratified split on a 1% positive class can leave the test set with almost no positives, making recall and precision meaningless. Stratified splitting keeps the ratio consistent across splits.
- **Choose resampling or weighting deliberately.** Oversampling, undersampling, synthetic generation (SMOTE), or class weighting each change what the model optimizes. Oversampling risks overfitting the minority; undersampling discards majority data; synthetic generation must never be applied before the split (it leaks across folds).
- **Match the metric to the imbalance.** Accuracy is meaningless under imbalance; use precision, recall, F1, PR-AUC, or cost-weighted metrics reflecting the business cost of each error type.
- **Never resample before splitting.** Generating or duplicating minority examples and then splitting scatters duplicated or synthetic rows across train and test, inflating scores. Resample inside the training fold only.

### Respect Sample Size In Both Splits And Score Interpretation

The amount of data bounds what can be learned and how precisely performance is measured; ignoring this produces confident nonsense.

- **Ensure each split is large enough for its purpose.** Training needs enough data to learn; validation and test need enough to estimate performance tightly. A test set of 50 examples produces a score with a confidence interval too wide to support a decision.
- **Compute the uncertainty of the evaluation.** A test accuracy of 0.92 on 100 examples has a 95% interval spanning roughly 0.85–0.96; the difference between 0.92 and 0.89 is not significant. Report intervals when the test set is small.
- **Scale model complexity to data volume.** A high-capacity model on a small dataset overfits; prefer simpler models or stronger regularization when data is scarce.
- **Power the comparison, not just the estimate.** Claiming model A beats model B requires the test sets and effect size to support that claim statistically; a 1% difference on a small test set is noise.

## Common Traps

### Random Split On Time-Series Data

Shuffling rows of a temporal dataset so the model trains on future periods and tests on the past, reporting accuracy unachievable in production. Use a temporal split (or rolling/expanding-window CV) that trains only on the past.

### Group Leakage From Shared Entities

Splitting rows independently when multiple rows belong to one user, store, or document, so the same entity appears in train and test and the model memorizes rather than generalizes. Split by group so each entity is wholly in one set.

### Validation-Set Overfitting From Repeated Tuning

Running many experiments against a single validation set and reporting the best score, having fit the validation set. Use nested cross-validation or a once-used test set for the final estimate.

### Unstratified Split Destabilizing A Rare Class

Splitting without stratification when the positive class is rare, leaving the test set with too few positives to estimate recall or precision. Stratify the split to preserve class ratios.

### Resampling Before Splitting

Oversampling or synthetically generating minority examples before the split, scattering duplicated or synthetic rows across train and test and inflating scores. Resample inside the training fold only.

### Tiny Test Set Reported As A Precise Score

Reporting a test-set score to two decimal places when the test set is small enough that the confidence interval spans the entire decision range. Compute and report the interval.

### Using Plain K-Fold On Grouped Or Temporal Data

Applying standard k-fold to grouped or time-series data, leaking across folds or across time. Use GroupKFold for grouped data and rolling/expanding-window CV for temporal data.

## Self-Check

- [ ] The split kind matches the data structure: temporal split (or rolling/expanding-window CV) for time-series, group-aware split (GroupKFold) for multiple rows per entity, and the test distribution approximates deployment.
- [ ] A single test set is held out and touched once at the end; all model selection, tuning, and feature iteration use validation or cross-validation, and the final score comes from the untouched test set.
- [ ] Repeated tuning is recognized as validation overfitting; nested cross-validation or a fresh holdout estimates the cost of selection, and the model is not retrained on train-plus-validation with validation reported as final.
- [ ] Cross-validation respects the data structure (StratifiedKFold for imbalance, GroupKFold for grouped data, rolling/expanding-window for time-series), nested CV is used when selection is part of the pipeline, and fold-to-fold variance is reported.
- [ ] Class imbalance is handled by stratified splits and a deliberate choice of resampling or weighting; resampling is applied inside the training fold only, and the metric matches the imbalance and the business cost of errors.
- [ ] Sample size is respected: each split is large enough, the uncertainty (confidence interval) is reported, model complexity is scaled to data volume, and comparisons are powered to support the claimed difference.
- [ ] The test set, once consumed for a decision, is acknowledged as no longer unbiased for the next decision, and a new holdout is planned when needed.
- [ ] The highest-risk cases were verified — a random split on time-series, group leakage, validation overfitting, an unstratified rare-class split, resampling before splitting, a tiny test set reported precisely, and plain k-fold on grouped or temporal data — not only the headline score.
