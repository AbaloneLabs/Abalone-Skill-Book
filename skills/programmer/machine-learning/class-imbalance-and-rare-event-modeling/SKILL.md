---
name: class_imbalance_and_rare_event_modeling.md
description: Use when the agent is training or evaluating a classifier on imbalanced data or rare events (fraud, churn, defect, disease, anomaly, failure prediction), choosing between resampling (oversampling, undersampling, SMOTE), class weighting, or threshold tuning, deciding whether to frame a rare event as supervised classification or unsupervised anomaly detection, calibrating predicted probabilities versus tuning a decision threshold, or selecting precision-recall and cost-sensitive metrics over accuracy. Also covers the failure mode of reporting accuracy on a skewed label distribution, of SMOTE applied before splitting or across leakage boundaries, of tuning the threshold on the test set, of confusing a calibrated probability with a decision, and of treating a rare-event problem as ordinary balanced classification when the rarity itself changes the correct objective and metric.
---

# Class Imbalance And Rare Event Modeling

In most real-world classification problems the class that matters is rare. Fraud is a fraction of a percent of transactions; churn is a minority of users; the defect, the disease, the failure, the intrusion — each is a small slice of a much larger normal population. The judgment problem is that imbalance breaks the defaults that make ordinary classification feel easy. The default metric (accuracy) is useless, because predicting "normal" always scores well. The default threshold (0.5) is rarely the right operating point, because the costs of the two error types are almost never equal. The default training objective pushes the model toward the majority class, because that is what minimizes average loss. And the rarity of the positive class means that every estimate about it — its precision, its recall, the value of a synthetic sample — rests on very few examples and is therefore noisy and easily over-optimistic. The discipline is to recognize that an imbalanced problem is a different problem, to choose metrics that reflect the real cost of each error type, to handle the imbalance deliberately in training (if at all) while always evaluating on the true distribution, and to decide whether the task is even best framed as supervised classification or as anomaly detection.

Agents tend to under-invest here because the mechanical workflow still runs: the model trains, it produces a high accuracy number, and a SHAP plot or a confusion matrix can be generated. The harm appears when the number is consumed. A fraud model reported at "99% accuracy" that catches no fraud is the canonical failure, but the subtler harms are worse: a model that looks good on PR-AUC but whose probabilities are uncalibrated, so the downstream system that consumes them makes wrong decisions; a SMOTE pipeline that synthesized minority examples using neighbors from across the train/test boundary, inflating validation scores with leakage; a threshold tuned on the test set to maximize F1, producing an operating point that will not generalize; a rare-event model trained as classification when the positive class is so scarce and so shifting that an unsupervised anomaly framing would have been more robust. The judgment is to treat the decision threshold, the probability calibration, the metric, and the framing as four separate decisions, each tied to the business cost of errors, and to evaluate everything on an untouched, original-distribution test set.

This skill covers resampling and its pitfalls, class weighting, the threshold-versus-calibration distinction, precision-recall and cost-sensitive evaluation, and the anomaly-detection framing for rare events. It complements the model-evaluation-and-metrics skill (which covers metrics broadly) and the training-data-quality skill (which covers imbalance in data terms); here the focus is the specific judgment of modeling and evaluating rare positive classes.

## Core Rules

### Never Judge An Imbalanced Classifier By Accuracy — Use The Metric That Reflects Error Cost

Accuracy on imbalanced data is not merely a weak metric; it is actively misleading, because the majority-class baseline (always predict "negative") achieves a high score while being useless. The metric must encode the relative cost of the two error types, and that cost is a business decision, not a statistical default.

- **Precision answers "when the model alarms, how often is it right?"** High precision means few false alarms; it matters when acting on a positive prediction is expensive or disruptive (a manual fraud review, a content takedown, an intrusive medical follow-up). Low precision erodes trust and overwhelms reviewers with false alarms.
- **Recall answers "of the real positives, how many did we catch?"** High recall means few misses; it matters when missing a positive is costly or dangerous (missed fraud, missed disease, missed safety failure). Low recall means the rare event slips through, which is often the whole failure mode the model exists to prevent.
- **F1 is the harmonic mean of precision and recall, useful only when you genuinely weight them equally — which is often not the case.** Do not default to F1 just because it is a single number; it hides the precision/recall tradeoff that the decision actually depends on.
- **Prefer PR-AUC over ROC-AUC when the positive class is rare.** ROC-AUC can look optimistically high on imbalanced data because the many true negatives inflate the curve; PR-AUC is far more sensitive to performance on the minority class and is the honest summary of a rare-event model's ranking quality.
- **Prefer a cost-weighted or business-aligned metric when error costs are known.** If a false negative costs $1000 and a false positive costs $10, define expected cost per decision and optimize that. A model that maximizes F1 while minimizing business value is a failed model with a tidy number.

Write down, for the specific problem: what does a false positive cost, what does a false negative cost, and which metric reflects that ratio? If you cannot answer the cost question, you do not yet know whether the model is good.

### Treat Resampling As A Training Bias Fix, Never As An Evaluation Change

Resampling (oversampling the minority, undersampling the majority, or synthesizing new minority examples with SMOTE/ADASYN) changes what the model sees during training so it does not collapse to the majority class. It does not change the world the model will be deployed in. The most common and damaging error is to resample the whole dataset before splitting, so synthetic or duplicated minority rows leak into the validation set, inflating every metric.

- **Resample inside the training fold only.** Oversampling or SMOTE must be applied after the train/validation split, fit on the training data, and never touch the validation or test set. The cleanest way to guarantee this is to put the resampler inside a pipeline that is fit per cross-validation fold.
- **Always evaluate on the original, unrebalanced distribution.** A metric computed on a rebalanced test set does not reflect deployment and is not a valid estimate of production performance. The whole point of evaluation is to measure the world as it is.
- **Understand each resampler's failure mode before choosing it.** Random oversampling duplicates minority rows, which can cause the model to overfit those exact examples. Undersampling discards majority data, which can throw away useful signal. SMOTE and its variants synthesize new minority points by interpolating between neighbors, which assumes the minority class lies on a smooth manifold — an assumption that fails for categorical features, for minority clusters, and across the decision boundary with the majority class.
- **Beware SMOTE across leakage boundaries.** SMOTE uses nearest neighbors; if a minority point's neighbors include rows that belong in the validation set (because resampling ran before the split), the synthetic point leaks information. Per-fold resampling prevents this.
- **Consider whether resampling is needed at all.** Class weighting (penalizing minority-class errors more in the loss) often achieves what resampling achieves without duplicating or fabricating data, and it composes cleanly with tree models, linear models, and neural nets. Try class weighting before reaching for SMOTE.

### Tune The Decision Threshold On Validation, Report At That Threshold On Test

A probabilistic classifier outputs a score; the decision comes from thresholding that score. The default threshold of 0.5 is almost never the right operating point for an imbalanced problem, because it ignores the asymmetry between false-positive and false-negative cost. Threshold tuning is a separate, deliberate step — and it must be done on validation data, not on the test set.

- **Choose the threshold to hit a business-required recall or precision, evaluated on validation data.** If the requirement is "catch at least 90% of fraud," sweep the threshold on the validation set to find the point that achieves that recall, then read off the resulting precision. The threshold is the operating point; the metric at that point is the honest performance.
- **Never tune the threshold on the test set.** Selecting the threshold that maximizes F1 (or any metric) on the test set is optimization on the evaluation data — it overfits the threshold to test-set noise and overstates real performance. Tune on validation, report once on test.
- **Report performance at the chosen threshold, not the area under a curve alone.** PR-AUC summarizes ranking quality across all thresholds, but the system will operate at one threshold; the precision and recall at that operating point are what the business experiences.
- **Re-tune the threshold when the cost ratio or the class balance changes.** A threshold tuned for one deployment context is wrong for another; if the false-negative cost rises or the base rate shifts, re-derive the operating point on fresh validation data.

### Separate Probability Calibration From Threshold Tuning — They Solve Different Problems

Threshold tuning sets the operating point; calibration sets whether the predicted probability itself is trustworthy. These are different decisions, and conflating them produces a model whose decisions are right but whose scores are wrong (or vice versa). Many downstream systems consume the probability, not the binary decision, and an uncalibrated probability misleads them even when the thresholded decision is acceptable.

- **Calibration means the probability reflects true frequency.** A calibrated model that says "10% chance of fraud" is right about 10% of the time. Tree models, naive Bayes, and models trained on rebalanced data are often poorly calibrated out of the box.
- **Resampling and class weighting distort calibration.** Oversampling the minority or weighting its loss changes the implicit prior the model learns, so the raw output no longer reflects the true base rate. If the downstream system needs calibrated probabilities, recalibrate (Platt scaling, isotonic regression) on a held-out set after training, or avoid resampling in favor of post-hoc threshold tuning on the original distribution.
- **Decide whether the consumer needs the probability or the decision.** If only the binary decision is consumed, calibration matters less and threshold tuning dominates. If a risk score, a ranking, or a downstream expected-value calculation consumes the probability, calibration is essential and resampling must be accounted for.
- **Validate calibration explicitly (reliability diagrams, expected calibration error)** wherever the probability is consumed, not only the decision.

### Decide Between Supervised Classification And Anomaly Detection Based On Rarity And Stability

When the positive class is extremely rare (a handful of examples) or when the nature of the rare event shifts over time (new fraud patterns, novel defects, zero-day intrusions), supervised classification may be the wrong framing entirely. A classifier needs enough labeled positives to learn what they look like, and it learns only the positive patterns it has seen. Anomaly detection learns the normal distribution and flags deviations, which generalizes to unseen rare events at the cost of less specific explanations.

- **Use supervised classification when there are enough labeled positives and the positive pattern is relatively stable.** If you have thousands of labeled fraud cases and the fraud modus operandi changes slowly, a classifier with proper imbalance handling is appropriate.
- **Use anomaly detection when positives are too scarce to learn, or when novel positives are expected.** When new failure modes will appear that were never in training, a model that learns "normal" and flags deviations (isolation forest, one-class SVM, density-based methods, autoencoder reconstruction error) catches the unseen, at the cost of less precise attribution.
- **Hybrid approaches are common and often best.** A classifier for known rare events paired with an anomaly detector for the unknown; tune each on its own metric and combine their alerts deliberately rather than averaging scores blindly.
- **Frame the choice against the business question.** "Catch the fraud we know about" favors classification; "flag anything unusual for review" favors anomaly detection. The framing is a product decision, not just a modeling one.

## Common Traps

### Reporting Accuracy On Imbalanced Data

Quoting 99% accuracy on a problem where 99% of cases are negative, implying excellence when the model learned only the majority class. Use precision, recall, F1, PR-AUC, and confusion-matrix inspection; never accuracy alone on imbalanced data.

### Resampling Before Splitting (SMOTE Leakage)

Applying oversampling or SMOTE to the whole dataset before the train/validation split, so synthetic or duplicated minority rows appear in both, inflating validation scores with leakage. Resample inside the training fold only, ideally via a per-fold pipeline.

### Evaluating On A Rebalanced Test Set

Computing the metric on a distribution that was oversampled or undersampled to balance, so the reported precision/recall does not reflect the real deployment base rate. Always evaluate on the original, unrebalanced distribution.

### Tuning The Threshold On The Test Set

Sweeping the threshold to maximize F1 on the test set, then reporting that F1 as if it were an honest estimate, when the threshold was optimized against test noise. Tune on validation; report once on test at the chosen operating point.

### Confusing A Calibrated Probability With A Decision

Treating a model's raw output score as a trustworthy probability when resampling or class weighting has distorted the implicit prior, so downstream expected-value calculations are wrong. Decide whether the consumer needs calibrated probabilities; if so, recalibrate on held-out data or avoid resampling.

### Defaulting To SMOTE Without Understanding Its Assumptions

Applying SMOTE blindly to categorical features, to minority clusters, or across the train/test boundary, generating synthetic points that do not reflect real minority structure and that leak information. Understand the interpolation assumption; prefer class weighting first; resample per-fold.

### Using ROC-AUC On A Rare Positive Class

Reporting ROC-AUC as the headline metric when the positive class is rare, because the many true negatives make the curve look good and hide poor minority-class performance. Prefer PR-AUC, and report precision and recall at the operating threshold.

### Framing A Novel-Rare-Event Problem As Classification

Building a supervised classifier for a problem where positives are too scarce or too shifting to learn (new fraud patterns, zero-day defects), so the model can only catch what it has seen and misses the novel. Consider anomaly detection or a hybrid when positives are scarce or unstable.

## Self-Check

- [ ] The model is judged by a metric that reflects the business cost of each error type (precision, recall, F1 only when equally weighted, PR-AUC preferred over ROC-AUC for rare positives, or an explicit cost-weighted/business metric), and accuracy is not reported alone on the imbalanced distribution.
- [ ] Resampling (oversampling, undersampling, SMOTE/ADASYN) is applied inside the training fold only — never before the split — ideally via a per-fold pipeline, and the resampler's assumptions (SMOTE's manifold interpolation, categorical handling, neighbor leakage) were considered before it was chosen.
- [ ] Evaluation is performed on the original, unrebalanced distribution, so the reported precision/recall reflects the real deployment base rate rather than an artificially balanced one.
- [ ] The decision threshold is tuned on validation data to hit a business-required recall or precision (not defaulted to 0.5, not tuned on the test set), and performance is reported at that operating point on a once-used test set.
- [ ] Probability calibration is treated as a separate decision from threshold tuning: where the downstream system consumes the probability, calibration is validated (reliability diagram, expected calibration error) and the distortion from resampling/class-weighting is accounted for or corrected via recalibration.
- [ ] The framing choice between supervised classification and anomaly detection was made deliberately against the rarity and stability of the positive class (classification for sufficient stable positives, anomaly detection for scarce or novel positives, hybrid where appropriate), not defaulted to classification.
- [ ] Class weighting was considered before reaching for resampling, and the tradeoff between oversampling (overfitting duplicates), undersampling (discarding signal), and synthesis (assumption violations) was weighed for the specific dataset.
- [ ] The highest-risk cases were verified — a SMOTE pipeline checked for cross-fold leakage, a threshold confirmed tuned on validation not test, an uncalibrated probability caught before it misled a downstream expected-value calculation, and a novel-rare-event problem recognized as needing anomaly detection rather than classification — not only the clean balanced-metric path.
