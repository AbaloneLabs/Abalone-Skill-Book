---
name: model_evaluation_and_metrics.md
description: Use when the agent is evaluating a machine learning model's performance; choosing evaluation metrics (accuracy, precision, recall, F1, ROC-AUC, PR-AUC, RMSE, MAE, log loss, business/utility metrics); detecting overfitting or underfitting; designing cross-validation and train/validation/test splits; preventing data leakage (temporal, feature, target leakage); handling class imbalance; deciding on a baseline model; assessing whether a model is genuinely good or merely gaming a metric; aligning metrics with business objectives; or planning A/B tests and online evaluation after deployment. Also covers the failure modes of accuracy on imbalanced data, leakage-inflated scores, metric-business misalignment, overfitting to the validation set, and the gap between offline metrics and real-world impact. Use for classification, regression, ranking, or probabilistic models in any domain where a model must be judged before it is trusted.
---

# Model Evaluation And Metrics

Model evaluation is the judgment of whether a model is actually good enough to trust in the world, where "good" is defined not by a single number on a held-out dataset but by whether the model does what the business needs, generalizes beyond the data it was built on, and does not fail in the specific ways that cause harm. This is harder than it looks because a model can post excellent metrics and still be useless or dangerous: accuracy is meaningless on imbalanced data, a leakage-tainted validation set inflates every score, a metric that does not correspond to business value optimizes the wrong thing, and a model that wins offline can lose online where the distribution shifts. Evaluation is not the step where you "compute the number"; it is the step where you decide whether to trust the model, and that decision is only as good as the metric choice, the data split, and the alignment between the score and the real-world consequence.

Agents tend to under-invest here because the mechanical part is easy — call the library, print the accuracy — and a high number feels like success. The harm appears later and is severe: a fraud model with "99% accuracy" that catches no fraud (because 99% of transactions are legitimate); a model that leaked future information into training and collapses in production; a metric optimized to perfection that has no relationship to revenue or user outcomes; a model deployed on offline scores alone that underperforms the simple heuristic it replaced. The judgment problem is deciding, for each model, what "good" means in business terms, which metric actually measures that, how to construct an evaluation that reflects the real deployment conditions, and what evidence is required before the model is trusted beyond the notebook.

This skill covers metric selection, overfitting and leakage, splits and cross-validation, imbalance, baselines, metric-business alignment, and online evaluation. It is deliberately conservative: a model is not "good" because a number is high; it is good when the evaluation honestly reflects the deployment context and the metric tracks the outcome that matters.

## Core Rules

### Choose The Metric By The Business Cost Of Each Error Type, Not By Default

Every metric encodes an assumption about which errors matter and how much. Accuracy treats every error equally; precision cares about false alarms; recall cares about missed positives; a cost-weighted metric prices each error in dollars. Choosing a metric is choosing a theory of what the business can tolerate, and the default (accuracy) is usually wrong.

- **Accuracy is misleading under class imbalance.** If 99% of cases are negative, a model that always predicts "negative" is 99% accurate and useless. Never report accuracy alone on imbalanced problems; use it only when classes are roughly balanced and error costs are symmetric.
- **Precision vs recall reflects the false-positive/false-negative tradeoff.** Precision = of the positives predicted, how many were real (low precision = many false alarms). Recall = of the real positives, how many were caught (low recall = many misses). Which matters depends on the business: medical screening and fraud detection often prioritize recall (missing a case is costly); spam filtering and content moderation often prioritize precision (a false alarm buries legitimate content). F1 is the harmonic mean of the two, useful only when you genuinely want to balance them equally — which is often not the case.
- **Probabilistic metrics (log loss, Brier score, AUC) measure calibration and ranking, not just classification.** Use them when the predicted probability matters (risk scoring, ranking) or when the decision threshold is not fixed. AUC measures ranking quality across all thresholds, which is useful for comparing models but says nothing about performance at the threshold you will actually use.
- **Regression metrics encode the error cost differently.** RMSE penalizes large errors heavily (squared); MAE penalizes linearly; MAPE is relative. Choose by whether large errors are disproportionately bad (RMSE) or whether you care about average magnitude (MAE).
- **Prefer a business-aligned utility metric when one exists.** If the model drives revenue, cost, retention, or clicks, define a metric in those units (expected profit per decision, cost saved per caught case). A model that maximizes a proxy metric while minimizing business value is a failed model with a good number.

Write down, for the specific problem: what does a false positive cost, what does a false negative cost, and which metric reflects that? If you cannot answer the cost question, you do not yet know whether the model is good.

### Detect Overfitting And Underfitting With The Right Comparison

A model's quality is not its training score; it is its ability to generalize to data it has not seen. Overfitting (memorizing training data, failing on new data) and underfitting (too simple to capture the pattern, failing on both) are diagnosed by comparing performance across data splits, not by a single score:

- **The train/validation/test gap is the signal.** High training score with much lower validation score indicates overfitting — the model learned the training data's noise. Low scores on both training and validation indicate underfitting — the model lacks capacity or the features are inadequate. A small, consistent gap with good validation performance is the healthy state.
- **Overfitting to the validation set is real and insidious.** Every time you tune hyperparameters or select features by validation performance, you leak information from the validation set. Repeated selection over many configurations makes the validation score an optimistic estimate. This is why a held-out test set, touched only once at the end, exists.
- **Learning and validation curves diagnose the cause.** Plot performance against training-set size and model complexity. If more data would help (validation improving as training grows), the issue is data quantity; if more complexity helps (training and validation both low), the issue is underfitting; if complexity widens the gap, the issue is overfitting.
- **Regularization and simpler models are the standard remedies for overfitting**, but they are chosen against validation performance, which loops back to the need for a clean, once-used test set.

Do not report a single validation number as "the model's performance." Report the train/validation/test comparison and what it implies about generalization.

### Prevent Data Leakage — It Is The Most Common Cause Of Falsely Good Models

Data leakage is any way information from outside the training data influences the model in a way that will not be available at prediction time. It inflates every metric and produces models that look brilliant offline and fail in production. Leakage is subtle and comes in several forms:

- **Target leakage.** A feature that is a consequence of, or proxy for, the target (e.g., "post-surgery complication flag" predicting post-surgery mortality). The model learns to read the answer, which will not exist at prediction time. Inspect top features for ones that could not be known at prediction time.
- **Temporal leakage.** Using future data to predict the past, when the deployment predicts forward in time. Any split that shuffles time-series data, or any feature computed using the full dataset (aggregates over all time), leaks the future into the past. For time-ordered data, split and validate chronologically: train on the past, validate/test on the future.
- **Preprocessing leakage.** Fitting scalers, imputers, PCA, or feature selection on the full dataset before splitting, so the training data "knows" the validation distribution. Always fit preprocessing on the training fold only and apply it to validation/test, ideally inside a pipeline that prevents accidental refit.
- **Duplicate-row leakage.** The same entity or event appearing in both training and validation (e.g., two rows from the same user, or near-duplicate records), so the model memorizes an entity it will see again. Split by entity (group-aware splits) when rows are related.

The discipline: at every step, ask "could this information or this row have been available at the moment of prediction?" If not, it is leakage. A model whose validation score drops sharply under a leak-free, time-aware split was leaking.

### Split Data To Reflect The Deployment Condition, Not To Maximize Convenience

How data is split determines whether the evaluation reflects reality. A random shuffle split is the default in tutorials and the wrong choice for many real problems:

- **Random splits assume rows are independent.** This holds for some problems (independent customer records) and fails for others (time series, grouped data, spatial data). Splitting time series randomly leaks the future; splitting grouped data randomly lets the model memorize groups.
- **Time-based splits for temporal problems.** Train on data up to time T, validate/test on data after T, mirroring how the model will be used. This is the only honest split for forecasting, anomaly detection, and any problem where the future differs from the past.
- **Group-aware splits for related entities.** When multiple rows belong to one entity (a patient, a user, a session), split by group so the same entity does not appear in both train and test, preventing the model from memorizing entity-specific patterns.
- **Stratification for imbalance.** When classes are imbalanced, stratify the split to preserve the class ratio in each fold; an unstratified split may put all minority examples in one fold.
- **Cross-validation for small data, single holdout for large.** K-fold cross-validation uses data efficiently for small datasets but is expensive and can still leak if not time/group-aware. For large data, a single chronological or group-aware holdout is often cleaner and faster.

Match the split to the deployment: if the model will predict the future, evaluate on the future; if it will generalize to new entities, evaluate on unseen entities. A split that does not mirror deployment conditions measures the wrong thing.

### Handle Class Imbalance Deliberately In Both Training And Evaluation

Imbalanced data (one class rare) breaks default assumptions: accuracy is meaningless, the model can ignore the minority class and still score well, and naive training produces a model biased toward the majority. Imbalance must be handled in evaluation and, if needed, in training:

- **Evaluate with imbalance-aware metrics.** Precision, recall, F1, PR-AUC (preferred over ROC-AUC when the positive class is rare, because ROC-AUC can look optimistically high), and confusion-matrix inspection. Never report accuracy alone.
- **Use stratified and balanced splits.** Preserve the class ratio (or the desired ratio) across folds so validation reflects the deployment distribution.
- **Resampling and class weighting address training bias, not evaluation.** Oversampling the minority, undersampling the majority, or class-weighting the loss can help the model learn the minority class — but evaluate on the original, unrebalanced distribution, or the metric will not reflect reality.
- **Threshold tuning for the operating point.** A classifier outputs probabilities; the decision threshold trades precision for recall. Choose the threshold on validation data to hit the business-required recall or precision, then report performance at that threshold on the test set. The default 0.5 threshold is rarely the right operating point for imbalanced problems.

Decide, for the imbalanced problem: what minority-class recall is required, what false-positive rate is acceptable, and does the metric reflect that? An imbalanced problem evaluated by accuracy is not evaluated at all.

### Establish A Baseline Before Claiming A Model Is Good

A model's quality is relative: good compared to what? Without a baseline, a high metric may simply reflect an easy problem, and a complex model may be no better than a trivial alternative. Always compare against meaningful baselines:

- **Trivial baselines.** Always predict the majority class; always predict the mean (regression); predict from the last value (time series). A model that cannot beat these adds nothing.
- **Simple baselines.** A logistic regression on a few features; a single decision tree; a heuristic from domain rules. A complex model that barely beats a simple baseline may not justify its cost, opacity, and maintenance burden.
- **The incumbent.** Whatever system the model would replace (an existing model, a rule-based system, human decision-makers). The new model must beat the incumbent by enough to justify the switch, measured on the same data.

A model is "good" only relative to the alternatives. Report the baseline scores alongside the model's, and if the gap is small, question whether the model earns its complexity.

### Align The Metric With The Business Outcome — And Watch For Metric Gaming

A model optimizes exactly the metric it is trained and selected on, nothing more. If that metric does not correspond to the business outcome, the model will excel at the wrong thing. Worse, given enough iterations, a model will learn to exploit any gap between the metric and the true objective:

- **Proxy metrics diverge from true objectives.** Click-through rate is a proxy for engagement; engagement is a proxy for value. A model maximizing the proxy can produce clickbait, addiction, or churn. Trace each metric to the business outcome it stands for and check whether maximizing it still serves the outcome.
- **Goodhart's law in model selection.** Once a metric becomes the target of repeated tuning and selection, it ceases to be a good measure. Iterating against a validation metric over many configurations slowly overfits to that metric; the test set and, ultimately, online measurement are the guardrails.
- **Metric gaming is often invisible.** A model can exploit data artifacts (a leaky feature, a temporal pattern that will not persist, a labeling shortcut) to maximize the metric without learning the real signal. Inspect not just the score but what the model learned (feature importances, error analysis, slices where it fails).
- **Define success in business units where possible.** "Reduce false negatives by 30%" or "increase caught-fraud value by $X" is more aligned than "improve F1 by 0.05." Translate the metric to the decision it drives and the consequence of that decision.

Ask, for every model: if this metric goes to its theoretical best, does the business actually improve? If the answer is "not necessarily," the metric is a proxy that needs a guardrail — usually a business-aligned outcome measured online.

### Validate Online — Offline Metrics Are Necessary, Not Sufficient

Offline evaluation (on held-out historical data) is necessary but not sufficient, because production differs from history in ways that matter: the distribution shifts, the model's own predictions change user behavior, and some effects (cannibalization, latency, user experience) are invisible offline. The final validation is online:

- **Distribution shift.** The world changes; the data the model was trained on ages. A model that was good at deployment degrades as the distribution drifts. Plan for monitoring and retraining, not a one-time deployment.
- **A/B testing measures the real impact.** Compare the model against the incumbent on a randomized subset of real traffic, measuring the business outcome (not the offline metric). This catches effects offline evaluation cannot: user behavior change, interaction with other systems, and the gap between proxy and true objective.
- **Beware of novelty and placement effects.** A new model can perform differently simply because it is new (users react to change) or because of how it is exposed. Run tests long enough to separate real effect from transient reaction.
- **Monitor for degradation and unintended effects.** After deployment, track the outcome metric, the input distribution, and slices where the model might discriminate or fail. A model is not "done" at deployment; it is done when it continues to perform as intended under real conditions.

The principle: offline metrics earn the model the right to be tested online; they do not earn it the right to be trusted at scale. Treat deployment as the beginning of evaluation, not the end.

## Common Traps

### Reporting Accuracy On Imbalanced Data

Quoting 99% accuracy on a problem where 99% of cases are one class, implying the model is excellent when it learned nothing. Use precision, recall, F1, PR-AUC, and confusion-matrix inspection; never accuracy alone on imbalanced data.

### Target Or Temporal Leakage Inflating Every Score

A feature that encodes the target, or a time-shuffled split that lets the model see the future, producing validation scores far above what production will deliver. Audit features for predict-time availability and use chronological splits for temporal data.

### Preprocessing Fit On The Full Dataset

Fitting scalers, imputers, or feature selectors on all data before splitting, so validation data influences training preprocessing. Fit preprocessing inside the training fold only, ideally via a pipeline that prevents refit on validation.

### Optimizing A Proxy Metric That Diverges From The Business Outcome

Maximizing click-through rate, F1, or accuracy without checking whether improving it improves the real objective, producing a model that wins the metric and loses the goal. Trace the metric to the business outcome and validate online.

### No Baseline, So A Complex Model's "Good" Score Is Uninterpretable

Reporting a model's score without comparing to a trivial, simple, or incumbent baseline, so it is unclear whether the model adds value or the problem is just easy. Always report baselines alongside the model.

### Overfitting To The Validation Set Through Repeated Tuning

Selecting hyperparameters and features against the validation set over many iterations, slowly leaking validation information until the validation score is optimistic. Reserve a once-used test set and confirm with online measurement.

### Random Split For Time-Series Or Grouped Data

Shuffling rows for a temporal or grouped problem, so the model trains on the future or on the same entities it will be tested on. Use chronological splits for time and group-aware splits for related entities.

### Resampling The Evaluation Set Along With Training

Oversampling or undersampling before splitting, or evaluating on a rebalanced distribution, so the reported metric does not reflect the real deployment distribution. Resample for training; evaluate on the original distribution.

### Default 0.5 Threshold On An Imbalanced Classifier

Using the default probability threshold, which rarely matches the business's precision/recall requirement on imbalanced data. Tune the threshold on validation data to the required operating point and report performance there.

### Treating Offline Metrics As Proof Of Production Worthiness

Deploying on offline scores alone, assuming held-out historical performance guarantees real-world impact, then discovering distribution shift, user-behavior change, or proxy-true divergence in production. Validate online with A/B testing and monitor for degradation.

## Self-Check

- [ ] The metric was chosen against the business cost of each error type (false positive vs false negative), not defaulted to accuracy; for imbalanced problems, precision/recall/F1/PR-AUC and the confusion matrix are reported, and accuracy-alone is not used.
- [ ] Overfitting and underfitting were diagnosed via the train/validation/test score gap and learning curves, and the reported performance reflects generalization (validation/test), not the training score.
- [ ] Data leakage was audited: no target-leaking features (every top feature is available at prediction time), chronological splits for temporal data, group-aware splits for related entities, and preprocessing fit only within training folds (via a pipeline).
- [ ] The data split mirrors the deployment condition — chronological for future-prediction problems, group-aware for entity-generalization, stratified for imbalance — rather than a convenience random shuffle.
- [ ] Class imbalance is handled in both training (resampling/weighting) and evaluation (imbalance-aware metrics, original-distribution evaluation, threshold tuned to the required operating point on validation, not defaulted to 0.5).
- [ ] Meaningful baselines (trivial, simple, and the incumbent) are reported alongside the model, and the model's improvement over them justifies its complexity.
- [ ] The metric is traced to the business outcome, proxy divergence is acknowledged, and the model is inspected (feature importances, error slices) for gaming rather than trusted on the score alone.
- [ ] A held-out test set was reserved and used only once for final confirmation, guarding against validation-set overfitting from repeated tuning.
- [ ] Online validation is planned or completed: A/B testing against the incumbent on real traffic measuring the business outcome, with awareness of novelty/placement effects and a plan to monitor distribution shift and degradation after deployment.
- [ ] The highest-risk cases were verified — accuracy on imbalance, leakage-inflated scores, proxy-business misalignment, threshold at the operating point, and offline-to-online gap — not only the single best validation number.
