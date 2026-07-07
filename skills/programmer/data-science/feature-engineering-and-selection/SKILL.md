---
name: feature_engineering_and_selection.md
description: Use when the agent is engineering features for a machine-learning model, encoding categorical variables (one-hot, ordinal, target/mean encoding, hashing, learned embeddings), handling missing values (imputation, missingness indicators, sentinel values), applying scaling and transforms (standardization, min-max, log, power), building interaction or ratio features, performing feature selection (filter, wrapper, embedded, regularization methods), or reviewing a feature pipeline for leakage and robustness. Also covers the failure modes of target leakage through outcome-derived features, mean-imputation that erases signal, high-cardinality encoding that explodes dimensionality, scale-sensitive models fed unscaled features, interactions that overfit, and the recurring mistake of building features using information that would not be available at prediction time.
---

# Feature Engineering And Selection

Feature engineering turns raw data into the inputs a model can learn from, and the central judgment problem is that every transformation is also a place where information about the outcome can leak into the training features in a way that will not exist at prediction time. A model trained on leaked features looks brilliant in cross-validation and collapses in production. Beyond leakage, the recurring failures are quieter: an encoding that turns a high-cardinality categorical into thousands of sparse columns; a missing-value strategy that imputes a constant and silently destroys the strongest signal (the fact that the value was missing); a tree ensemble fed a scaled feature where scaling does nothing, or a distance-based model fed unscaled features where one large-magnitude column dominates. The decisions that look mechanical — how to encode, impute, scale, and select — are where models succeed or fail.

Agents miss these problems because feature engineering is taught as a set of techniques and the pipeline runs without error, producing a model with reasonable cross-validation scores. The harm is models that do not generalize. A target-encoded categorical, fit on the full dataset, leaks the label and scores perfectly in validation yet is useless in production. A feature derived from a column updated after the outcome encodes the future. The judgment problem is to treat the feature pipeline as part of the model — subject to the same train/test discipline and leakage scrutiny — and to choose each transformation for a reason tied to the model and the data. This skill focuses on engineering and selection judgment; it complements the data-sampling-and-splitting skill (structural leakage prevention) and the statistical-pitfalls skill (general overfitting).

## Core Rules

### Make Every Feature Transformation Prediction-Time-Realistic, And Fit It Only On Training Data

The defining rule is that a feature must be computable at prediction time from information available then, and any learned transformation must be fit on the training split only:

- **Target leakage is the cardinal sin.** Any feature derived from the outcome — directly, through a proxy, or through a column updated after the event — leaks the label. "Average purchase in the last 30 days" is safe only if the window ends before the prediction point; "average purchase this month" leaks if the month contains the outcome.
- **Fit encoders, scalers, and imputers on training data only.** A mean used for imputation, a category mapping, or a scaler's mean and variance are parameters; fitting them on the full dataset leaks distributional information. Fit on the training fold, then transform validation and test with those learned parameters.
- **Beware features derived from columns updated after the event.** Operational databases are mutable; a "current status" column may have been rewritten after the outcome. Snapshot feature values as of prediction time.
- **When in doubt, simulate the prediction call.** Walk through how each feature would be computed for a brand-new record at scoring time. If the computation needs information you would not yet have, the feature leaks.

### Choose Encoding By Cardinality, Model Family, And Sample Size

The right choice depends on how many categories exist, how much data you have, and which model will consume the features:

- **One-hot encoding** is safe and interpretable for low-to-moderate cardinality, but explodes dimensionality for high-cardinality columns (user IDs, zip codes, SKUs), producing sparse, poorly-learned features that overfit.
- **Target/mean encoding** collapses high cardinality to one dimension by mapping each category to its outcome rate, but leaks the target unless done with out-of-fold smoothing and regularization. Use it for high cardinality, never fit on the full data.
- **Ordinal encoding** is appropriate only when the categories have a genuine order (low/medium/high); forcing an order onto nominal categories misleads scale-sensitive models.
- **Hashing and learned embeddings** serve high-cardinality or free-text features; embeddings capture similarity but need enough data to learn. Match the encoding to the model: trees tolerate ordinal/label encoding; distance-, regularization-, and neural-based models need one-hot, hashing, or embeddings.

### Treat Missingness As Signal, Not Just A Nuisance

Missing values are rarely random, and the pattern of missingness is often more predictive than the value itself:

- **Distinguish missingness mechanisms.** Missing completely at random is rare; missingness is usually correlated with the outcome (a field blank because the user is disengaged). Dropping rows with missing values biases the sample toward the non-missing population.
- **Pair imputation with a missingness indicator.** Imputing a constant or a mean hides the fact that the value was missing; adding a binary "was missing" column preserves the signal that often carries most of the predictive power.
- **Match the imputation to the model and the mechanism.** Mean/median imputation is a simple baseline but distorts variance; model-based imputation (k-NN, iterative) can preserve structure but must itself be fit on training data only. For trees, a sentinel value often suffices; for distance or linear models, it does not.
- **Never impute using the target.** Imputing a feature with the outcome (even indirectly) is leakage.

### Scale And Transform For The Model And The Distribution

Scaling matters for some models and is irrelevant for others; transforms matter when the distribution breaks the model's assumptions:

- **Scale-sensitive models require scaling.** Distance-based (k-NN, SVM, k-means), regularized linear (lasso, ridge), and neural models are dominated by large-magnitude features unless scaled. Tree-based models are scale-invariant and do not benefit.
- **Fit the scaler on training data only**, then apply to validation and test, exactly like any other learned transformation.
- **Transform skewed distributions when they break the model.** Log or power transforms (Box-Cox, Yeo-Johnson) tame heavy right-skew that distorts linear and distance models; they are usually unnecessary for trees. Beware log of non-positive values.
- **Match the transform to the phenomenon.** Counts and monetary amounts are often log-normal and benefit from log; bounded percentages do not. Do not apply transforms reflexively to every numeric column.

### Build Interactions And Ratios Deliberately, And Select Features To Control Overfitting

Interactions and ratios can capture structure a linear model misses, but each added feature is also a chance to overfit:

- **Add interactions that reflect a real mechanism.** A ratio (revenue per session) or a domain-specific combination is justified when it encodes a relationship the model cannot learn from raw columns alone (especially linear models). Avoid combinatorial explosions of all-pairs interactions on limited data.
- **Tree models discover interactions automatically.** For gradient-boosted trees and random forests, manual interactions rarely help; reserve them for linear or neural models.
- **Choose a feature-selection method by its bias and cost.** Filter methods (correlation, mutual information) are fast but ignore interactions; wrapper methods (recursive elimination, forward selection) are expensive and overfit the validation set if not nested; embedded methods (L1 regularization, tree importances) couple selection to the model and are a strong default.
- **Evaluate selection inside the cross-validation loop.** Selecting features on the full dataset, then cross-validating, leaks which features matter. Perform selection within each training fold (nested CV).

## Common Traps

### Target Leakage Through Outcome-Derived Features

Including a feature computed from the outcome, a post-outcome column, or a target-encoded mapping fit on the full data, producing implausibly high validation scores that vanish in production. Audit every feature against the prediction-time information set.

### Fitting Encoders, Imputers, Or Scalers On The Full Dataset

Computing imputation means, target-encoding maps, or scaler statistics across all rows, leaking distributional information. Treat every learned transformation as a model parameter fit on training data only.

### One-Hot Exploding A High-Cardinality Categorical

One-hot encoding a user ID, zip code, or SKU into thousands of sparse columns the model cannot learn from with limited data. Use target encoding (out-of-fold), hashing, or embeddings.

### Mean-Imputation Erasing The Missingness Signal

Replacing missing values with a mean or constant and dropping the fact that they were missing, discarding a signal that is often the strongest predictor. Pair imputation with a missingness indicator.

### Scale-Sensitive Model Fed Unscaled Features

Feeding a k-NN, SVM, regularized-linear, or neural model raw features of wildly different magnitudes, so one large column dominates the distance or gradient. Scale features for scale-sensitive models; trees are scale-invariant.

### Wrapper Selection Overfitting The Validation Set

Running recursive feature elimination or forward selection on the full dataset and reporting the resulting validation score, having used the validation set to choose features. Perform selection inside nested cross-validation.

### Interactions Built On Limited Data That Overfit

Adding a combinatorial set of interaction features to a small dataset, capturing noise. Add interactions that reflect a real mechanism; prefer tree models when data is limited.

## Self-Check

- [ ] Every feature is computable at prediction time from information available then; each input column's timestamp relative to the prediction point is known, and no feature is derived from the outcome, a post-outcome column, or a target-encoded mapping fit on the full data.
- [ ] All learned transformations (encoders, imputers, scalers, target-encoding maps) are fit on the training fold only and applied to validation and test.
- [ ] Categorical encoding matches cardinality and model family: one-hot for low cardinality, target encoding (out-of-fold) or embeddings/hashing for high cardinality, ordinal only for genuinely ordered categories.
- [ ] Missingness is treated as signal: the mechanism is considered, imputation is paired with a missingness indicator, and imputation never uses the target.
- [ ] Scaling and transforms are applied where the model or distribution requires them and omitted where irrelevant (trees); scalers are fit on training data only.
- [ ] Interaction and ratio features encode real mechanisms rather than combinatorial explosions, and tree models are preferred over manual interactions when data is limited.
- [ ] Feature selection (filter, wrapper, or embedded) is performed inside the cross-validation loop so held-out folds never influenced which features were chosen; wrapper methods use nested CV.
- [ ] The highest-risk cases were verified — target leakage, full-dataset fitting of transformations, high-cardinality one-hot explosion, erased missingness signal, unscaled scale-sensitive model, wrapper overfitting, and overfit interactions — not only the cross-validation score.
