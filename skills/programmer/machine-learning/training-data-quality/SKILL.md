---
name: training_data_quality.md
description: Use when the agent is curating, labeling, or validating training data for a machine-learning model, splitting data into train/validation/test sets, addressing class imbalance or sampling bias, checking for data leakage between training and evaluation, defining a labeling process and adjudicating label noise, or auditing a dataset for demographic or distributional bias before training. Also covers the failure mode of a model that evaluates well but generalizes poorly because the data was leaked, biased, mislabeled, or unrepresentative, and the discipline of treating data quality as the primary determinant of model quality rather than an afterthought to model architecture.
---

# Training Data Quality

A machine-learning model is, more than anything, a function of its training data. The architecture and the optimizer matter, but they are second-order: a strong model trained on leaked, biased, mislabeled, or unrepresentative data will learn the wrong function, and no amount of architectural sophistication will recover. The judgment problem is that data quality failures are insidious — they often produce a model that evaluates well, because the same flaw that contaminates training also contaminates evaluation, so the metrics look great right up until the model meets reality. Data leakage is the canonical example: a feature that encodes the label (a timestamp that reveals the outcome, a field populated only after the event), present in both training and validation, yields a model that aces validation and collapses in production. Bias is another: a dataset drawn from one population generalizes poorly to another, and the validation set, drawn from the same population, will not warn you. The discipline is to treat data quality as the primary determinant of model quality, to design the data pipeline (collection, labeling, splitting) with the same rigor as the model, and to audit for leakage, bias, imbalance, and label noise before trusting any metric.

Agents tend to under-invest in data quality because the model-training step is where the visible work happens and where the metrics live. The harm appears as models that pass evaluation and fail in production, as biased models that perform well on average and badly on the populations that matter, and as the slow realization that the "good" metric was measuring the wrong thing. The judgment is to hold out data that the model has truly never seen (and that shares no contaminating signal with training), to design labeling with adjudication and measured inter-rater agreement, to address class imbalance deliberately rather than letting the majority class dominate, to audit for bias across the subgroups the model will serve, and to treat the dataset itself as a versioned, reviewed artifact rather than an ad-hoc dump. A model is only as good as its data; a metric is only as trustworthy as the split it was computed on.

## Core Rules

### Treat Data Quality As The Primary Determinant Of Model Quality

The single highest-leverage act in ML is improving the data, not the architecture. Time spent auditing, cleaning, balancing, and correctly labeling data almost always outperforms time spent tuning the model. Hold this priority explicitly, because the workflow naturally pulls attention toward the model.

- **Invest in data before the model.** A larger or cleaner dataset routinely beats a cleverer architecture on a worse dataset. Budget for data work as the primary cost, not the leftover.
- **Version and review the dataset like code.** A dataset is an artifact with a lineage (sources, filters, labels, version); treat it as something that can be audited, reproduced, and rolled back, not as an ad-hoc dump.
- **When metrics disappoint, suspect the data first.** A surprising result is more often a data problem (leakage, mislabeling, distribution shift) than a model problem; investigate the data before reaching for a new architecture.

### Prevent Data Leakage Between Training And Evaluation

Data leakage is the presence of information in the training data that would not be available at prediction time — most often the label itself, encoded in a feature. Leaked models evaluate well (because the leak is in validation too) and fail in production. Preventing leakage is the most important split-design discipline.

- **Hold out data the model has truly never seen, with no shared contaminating signal.** The test set must not share sources, preprocessing, or leak-prone features with training in a way that telegraphs the outcome.
- **Split by group when records are related.** If multiple rows relate to the same entity (user, session, image with augmentations), split by group so the model cannot memorize an entity present in both train and test. Random row-wise splitting leaks.
- **Beware features derived from the label or from future information.** A "days until churn" feature in a churn model, a timestamp that reveals the outcome, a field populated only after the event — these encode the answer. Audit every feature for whether it could be known at prediction time.
- **Do preprocessing fit on training only.** Scalers, imputers, and encoders must be fit on the training set and applied to validation/test; fitting on all data leaks distributional information.

### Design Labeling With Adjudication And Measured Agreement

Labels are the ground truth the model learns, and noisy labels cap model quality: a model cannot learn a function more precisely than its labels allow. Design labeling as a process with multiple labelers, measured agreement, adjudication of disagreements, and clear guidelines — not as a single person's subjective judgment.

- **Measure inter-rater agreement and treat low agreement as a signal.** If two competent labelers disagree often, the labeling guideline is ambiguous; clarify it before collecting more noisy labels.
- **Adjudicate disagreements rather than averaging them.** A disagreement resolved by a senior labeler or a panel produces a cleaner signal than averaging contradictory labels.
- **Sample-audit label quality continuously.** Even with good process, label noise accumulates; periodically re-audit a sample to catch drift in labeling quality.

### Address Class Imbalance Deliberately

When one class vastly outnumbers others, a model can achieve high accuracy by predicting the majority class always — which is useless for the minority class that usually matters. Address imbalance deliberately, and evaluate with metrics that reflect minority-class performance, not overall accuracy.

- **Choose a strategy matched to the problem** (resampling, class weighting, threshold tuning, focal loss, synthetic data), understanding each strategy's tradeoffs rather than applying one blindly.
- **Evaluate with metrics that expose minority-class performance.** Accuracy on imbalanced data is misleading; use precision, recall, F1, per-class metrics, and confusion matrices so minority-class failure is visible.
- **Set the decision threshold for the real-world cost tradeoff.** The default 0.5 threshold is rarely optimal; tune the threshold to the relative cost of false positives and false negatives in deployment.

### Audit For Bias Across The Populations The Model Will Serve

A model trained on an unrepresentative dataset performs well on average and badly on the subgroups that are underrepresented or historically disadvantaged. Bias audits are not optional for models that affect people; they are how you discover that "good average performance" hides harmful failure on a specific group.

- **Measure performance sliced by subgroup** (demographic, geographic, linguistic, device), not only in aggregate. Aggregate metrics hide disparate failure.
- **Examine the dataset's representativeness before training.** If a subgroup is scarce in the data, the model will likely fail on it; collect more data or apply fairness-aware techniques rather than shipping a known disparity.
- **Define and track fairness metrics appropriate to the harm.** Different harms call for different criteria (demographic parity, equalized odds, calibration); choose deliberately based on the context, and recognize the inherent tradeoffs between them.

### Make Splits Reflect The Deployment Distribution

A model is evaluated to predict how it will perform in deployment, so the evaluation set must reflect the deployment distribution. A validation set that is easier, cleaner, or differently composed than production data yields a metric that overstates real performance.

- **Reserve a holdout that matches production.** If production has a class balance, a noise level, or a temporal drift, the holdout should reflect it, or the metric will mislead.
- **Use time-based splits for temporal data.** For time-series or any data with temporal drift, split by time (train on the past, validate on the future) to simulate deployment; random splitting leaks the future into training.
- **Test on out-of-distribution samples where possible.** A model that performs on in-distribution data but fails on a shifted distribution is fragile; probe this before deployment.

## Common Traps

### Leakage That Inflates Validation Metrics

A feature derived from the label or from future information, present in both training and validation, producing a model that aces validation and collapses in production. Audit every feature for whether it could be known at prediction time; split by group for related records; fit preprocessing on training only.

### Optimizing The Model While The Data Is The Bottleneck

Reaching for architecture and tuning when the data is leaked, biased, mislabeled, or scarce, when improving the data would yield more. Treat data quality as the primary determinant; suspect the data first when metrics disappoint.

### Random Splitting Of Related Records

Splitting row-wise when rows relate to the same entity (user, session, augmented image), so the model memorizes entities present in both train and test and validation overstates real performance. Split by group.

### Mislabeled Data Capping Quality

Labels from a single subjective labeler with no adjudication or agreement measurement, capping model quality at the label noise level. Design labeling with multiple labelers, measured agreement, and adjudication.

### Accuracy On Imbalanced Data

Reporting accuracy on a dataset where one class dominates, masking a model that fails on the minority class that matters. Use per-class metrics, precision/recall/F1, and tune the decision threshold to real-world costs.

### Aggregate Metrics Hiding Subgroup Failure

Reporting only aggregate performance, hiding that the model performs well on the majority and badly on a disadvantaged subgroup. Measure performance sliced by subgroup and define fairness metrics appropriate to the harm.

### A Holdout That Does Not Match Production

A validation set cleaner, easier, or differently composed than production, so the metric overstates real performance. Reserve a holdout matching the deployment distribution; use time-based splits for temporal data.

## Self-Check

- [ ] Data quality is treated as the primary determinant of model quality: investment goes into data before architecture, the dataset is versioned and reviewed like code, and surprising metrics trigger data investigation before model changes.
- [ ] Data leakage is prevented: the test set shares no contaminating signal with training, splits are by group for related records, every feature is audited for whether it could be known at prediction time, and preprocessing is fit on training only.
- [ ] Labeling is designed as a process with multiple labelers, measured inter-rater agreement (low agreement triggers guideline clarification), adjudication of disagreements, and continuous sample-audits of label quality.
- [ ] Class imbalance is addressed deliberately with a strategy matched to the problem, evaluated with metrics that expose minority-class performance (per-class precision/recall/F1, confusion matrices), and a decision threshold tuned to the real-world cost tradeoff rather than defaulted to 0.5.
- [ ] Bias is audited across subgroups the model will serve: performance is measured sliced by demographic/geographic/linguistic/device dimensions, dataset representativeness is examined before training, and fairness metrics appropriate to the harm are defined and tracked with their tradeoffs recognized.
- [ ] Splits reflect the deployment distribution: the holdout matches production's class balance, noise level, and temporal drift; time-based splits are used for temporal data; and out-of-distribution samples are probed to test robustness.
- [ ] The highest-risk cases were verified — a leak-prone feature caught before training, a group-split that revealed real (lower) performance, a subgroup on which the model failed despite good aggregate metrics, and a temporal split that exposed drift — not only the clean balanced-dataset path.
- Does the output stay within the agent's scope, deferring final authority, licensed judgment, or specialist sign-off to the qualified person where the question exceeds the agent's competence?
