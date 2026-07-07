---
name: feature_engineering_and_feature_stores.md
description: Use when the agent is designing features for a machine-learning model, choosing raw signals versus derived features, encoding categorical or high-cardinality variables, handling missing values and time-based features, preventing train-serving skew, deciding whether to adopt a feature store, or managing feature definitions, point-in-time correctness, and reuse across training and serving. Also covers the failure mode where a model degrades in production because features were computed differently at train time versus serve time, or because a feature leaked future information.
---

# Feature Engineering And Feature Stores

Feature engineering is the work of transforming raw signals into the inputs a model actually learns from. It is where the most consequential and least visible mistakes in applied machine learning happen. Two failure modes dominate. The first is train-serving skew: a feature computed one way during training (in a batch job, with access to full historical context, computed over a clean dataset) and a different way at serving time (online, with only current data, computed over a noisier path). The model learns a function of the training-time feature and is then asked to predict from the serving-time feature, and the silent mismatch degrades production performance even though offline metrics looked fine. The second is feature leakage: a feature that encodes information unavailable at prediction time — a value populated only after the event, an aggregation that spans the prediction timestamp, a proxy so tightly correlated with the label that it makes the problem trivial — producing a model that aces validation and collapses in the real world.

Agents tend to treat feature construction as a local, notebook-level activity and defer the question of how those features will be reproduced in production. The harm appears as models that pass offline evaluation and degrade online, as features that cannot be recomputed at low latency, as duplicated and divergent feature logic maintained by separate teams, and as silent leakage that no one catches because the metrics look good. The judgment is to treat a feature as a contract between training and serving — a single definition, a single transformation, computed point-in-time-correct, reproducible offline and online — and to reach for a feature store only when the cost of divergence or duplication justifies it.

## Core Rules

### Treat Every Feature As A Train-Serving Contract

A feature is not a column in a dataframe; it is a definition plus a computation that must produce the same value in training and in serving. The most damaging bug in applied ML is a feature whose training and serving implementations quietly diverge. Design features with the explicit question: *how will this exact value be reproduced at prediction time, with the data and latency available then?*

- **Define each feature once and share the definition.** The same transformation code, or a shared registry of feature definitions, should drive both the training pipeline and the online serving path. Two implementations written separately will diverge.
- **Identify the serving path before committing to a feature.** If a feature requires data or computation unavailable online at the required latency, it is not a usable feature regardless of its offline value. Decide this before training on it.
- **Test for skew explicitly.** Compare feature distributions and per-feature values between training and serving on the same logical input. Skew detection should be a monitored, alertable check, not a one-time audit.

### Enforce Point-In-Time Correctness

A feature is point-in-time correct if, for a prediction made at time T, it uses only information that was knowable at or before T. Violating this is leakage, and it is the most common cause of implausibly-good offline metrics.

- **Join features by event time, not by current state.** When assembling a training row for a prediction at T, every feature must be computed from events with timestamps strictly before T. Joining against a current snapshot of a slowly-changing table leaks the future into the past.
- **Beware aggregations that span the prediction timestamp.** A "average spend over 30 days" feature that includes days after the prediction event is leaked. Window aggregations must end at or before T.
- **Audit features derived from downstream or post-hoc fields.** Any column populated only after the outcome is known (status flags set post-resolution, enrichment added later) is a leak. The fact that it exists in the training table does not make it available at prediction time.

### Choose Feature Types Deliberately

Different feature types carry different risks and require different handling. Do not apply the same encoding by default.

- **Categorical features:** cardinality drives the choice. Low-cardinality categoricals can be one-hot encoded; high-cardinality categoricals (user IDs, product IDs, URLs) need hashing, embeddings learned during training, or target encoding with strict leakage controls (out-of-fold targets only). Naive one-hot of a high-cardinality column explodes dimensionality and memorizes.
- **Numerical features:** decide between raw values, scaling/normalization (fit on training only), bucketization, and log transforms based on the distribution and the model family. Tree models tolerate raw scale; linear and neural models often require scaling.
- **Time and cyclical features:** time-of-day and day-of-week are cyclical; encode with sine/cosine transforms rather than raw integers, so 23:00 and 00:00 are close, not far apart.
- **Text and high-dimensional signals:** decide between handcrafted features, learned embeddings, and end-to-end representation, and be explicit about which path carries the leakage risk (a pretrained embedding is safe; an embedding fit on the full dataset including the test period is not).
- **Crossed and interaction features:** useful when domain knowledge suggests a specific interaction the model cannot easily learn, but each added feature is a new skew surface and maintenance burden.

### Handle Missing Values As A First-Class Decision

Missingness is information, not just noise to paper over. How you handle it affects both training and serving, and the handling must be consistent across both.

- **Distinguish missing-not-at-random from missing-at-random.** If the fact that a value is missing is itself predictive (a field a user never fills out), the missingness indicator is a feature in its own right; do not discard it by imputing.
- **Impute using training-set statistics only.** Mean/median/mode imputation must use training-set values, applied identically at serving; imputing with full-dataset statistics leaks distributional information.
- **Ensure the serving path can produce the same missingness.** If a feature is sometimes unavailable at serving for reasons that differ from training (a dependency is down, an enrichment times out), the model will see a different missingness pattern than it was trained on.

### Decide Whether A Feature Store Is Justified

A feature store is infrastructure that centralizes feature definitions, computes and stores feature values, and serves them consistently to both training and online inference. It is valuable when the cost it prevents is real, and expensive overhead when it is not. Do not adopt one by default.

- **Adopt when divergence or duplication is the actual pain.** Multiple teams recomputing the same features differently, training-serving skew recurring, or a need to share features across models and teams are real justifications. A single model with a single team rarely justifies the operational cost.
- **Prioritize the central property: one definition, two accesses.** The value of a feature store is that the same registered feature is materialized for batch training and fetched for online serving, eliminating skew by construction. If a proposed store does not provide this, it is a registry, not a feature store.
- **Account for the operational cost.** Feature stores add infrastructure (online and offline stores, a registry, ingestion pipelines, monitoring), and misconfigured point-in-time joins remain a leakage risk even within a store. The store is a tool for enforcing discipline, not a substitute for it.

### Manage Feature Lifecycle And Dependencies

Features are long-lived artifacts with dependencies, consumers, and downstream consequences. Treat them as versioned, owned entities, not as ad-hoc notebook output.

- **Version feature definitions and track lineage.** A change to a feature transformation is a change to the model's input; track which model version used which feature version, so a regression can be traced to a feature change.
- **Know who consumes each feature.** A feature shared across models means a change can break models you did not intend to affect. Shared features need owners and change management.
- **Deprecate deliberately.** A feature that is no longer used still incurs computation and storage cost and adds skew surface; prune features that are not contributing.

## Common Traps

- **Train-serving skew from dual implementations.** A feature computed in a Spark job for training and reimplemented in the online service by a different author, drifting silently as one side is updated. Share the definition; monitor for skew.
- **Point-in-time leakage through a current-state join.** Joining historical training rows against the current value of a slowly-changing table, smuggling future information into the past. Join by event time with an as-of constraint.
- **Aggregation windows that include the future.** A rolling feature whose window extends past the prediction timestamp, leaking the outcome period. Truncate windows at the prediction time.
- **High-cardinality one-hot encoding.** One-hot encoding a user-ID or URL column, producing a sparse, enormous, memorizing representation that does not generalize. Use hashing, embeddings, or carefully-controlled target encoding.
- **Target encoding without leakage control.** Replacing a categorical with its mean target value computed on the full dataset, directly leaking the label. Use out-of-fold or leave-one-out target means.
- **Imputation with full-dataset statistics.** Filling missing values using the mean computed across train and test, leaking distributional information. Fit imputation on training only.
- **Discarding missingness as a signal.** Imputing over a missing value that was itself predictive, throwing away information. Preserve a missing-indicator feature where missingness is meaningful.
- **Adopting a feature store to look modern.** Standing up feature-store infrastructure for a single-model, single-team project where the operational cost exceeds the value. Adopt when divergence or duplication is the real pain.
- **Treating the feature store as a leakage cure.** Assuming a feature store prevents leakage by existing, when point-in-time joins still must be configured correctly inside it. The store enforces discipline you choose to apply; it does not replace it.
- **Untracked feature changes breaking downstream models.** Modifying a shared feature definition without knowing which models consume it, silently regressing production models. Version features and track lineage and consumers.

## Self-Check

- For each feature, can I state the single shared definition that produces the same value in training and serving, and is skew between the two paths monitored rather than assumed absent?
- Is every feature point-in-time correct — for a prediction at time T, does it use only events and aggregations with timestamps strictly before T, with no current-state join smuggling in the future?
- Have I audited every feature for leakage from post-hoc fields, downstream-populated columns, or aggregation windows that span the prediction timestamp, rather than trusting that good metrics mean no leak?
- Are categorical encodings chosen by cardinality (low → one-hot, high → hashing/embedding/controlled target encoding), rather than default one-hot that explodes dimensionality?
- Is missing-value handling a deliberate decision — imputation fit on training only, missingness preserved as a feature where it is predictive — and is the serving path able to reproduce the same missingness pattern?
- If a feature store is adopted, is it justified by real divergence or duplication pain, does it provide one definition with batch and online access, and am I still configuring point-in-time joins correctly inside it rather than assuming the store prevents leakage?
- Are feature definitions versioned with lineage and consumers tracked, so a change can be traced and its impact on downstream models assessed before deployment?
- Did I verify the highest-risk cases — a leaked feature caught before training, a skew between train and serve values detected, a high-cardinality encoding replaced, a point-in-time join that would have smuggled the future — rather than only the clean single-team path?
