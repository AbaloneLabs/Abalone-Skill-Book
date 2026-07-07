---
name: machine_learning_for_prediction.md
description: Use when the agent is using machine learning for prediction in research, designing train validation and test splits, diagnosing overfitting, selecting models for predictive performance, weighing interpretability against accuracy, or guarding against the error of treating predictive success as causal or explanatory understanding.
---

# Machine Learning For Prediction

Machine learning excels at prediction, and that strength is also its trap. A flexible model can fit complex patterns and achieve impressive accuracy on held-out data, which looks like understanding but is not. Prediction answers what will happen; explanation and causation answer why. Researchers trained on inferential statistics reach for ML, report high accuracy, and then slip into causal or mechanistic claims that the model was never built to support and cannot justify. The second trap is overfitting dressed as success: a model tuned until it wins on the validation set has learned the validation set, not the phenomenon, and its reported performance is an optimistic fiction. The judgment problem is to keep prediction honest, to separate the predictive goal from explanatory claims, to validate in a way that survives contact with new data, and to report the limits of what a black-box model can reveal.

Use this skill when building predictive models, splitting data for training and evaluation, diagnosing overfitting, choosing among algorithms, trading interpretability against accuracy, and reporting predictive results. The goal is to keep the agent from confusing predictive performance with causal understanding, from leaking information across data splits, from selecting models on a single validation pass, and from presenting a tuned pipeline as if it were reproducible without the full training procedure. The agent has latitude in algorithms and architectures, but the discipline of validation, the honesty about goals, and the reproducibility of the pipeline must each be enforced.

## Core Rules

### Make The Predictive Goal Explicit And Separate It From Causal Claims

Prediction and explanation are different enterprises that demand different variables, validation, and interpretation. Conflating them is the most damaging ML error in research.

Clarify:

- whether the goal is to predict an outcome accurately, or to explain or cause it;
- for prediction, variables are chosen for their contribution to accuracy, including ones that would bias causal estimates, such as colliders or instruments;
- for explanation, the coefficients or importances of a predictive model are not causal effects;
- predictive success does not reveal mechanism, importance in the real world, or what would happen under intervention;
- the claim must match the goal: a predictive model supports forecasts, not causal conclusions.

A model that predicts readmission well tells you who is at risk, not why, and not what intervention would help. State the goal up front and keep the claim inside it.

### Enforce Strict Separation Of Train, Validation, And Test

Information leakage across splits inflates performance and manufactures false confidence. The test set must remain untouched until the final reported evaluation.

Prevent leakage by:

- splitting at the right unit, so repeated measures, clusters, or time-correlated records from one source do not appear in both train and test;
- performing all preprocessing, feature selection, and imputation fitting within the training fold only, then applying learned parameters to validation and test;
- holding out a true test set used exactly once, after all model selection is complete;
- using nested cross-validation when model selection and performance estimation both draw on the data;
- guarding against temporal leakage, where future information reaches training features.

A model that peeks at the test set during tuning has learned the test set. Performance estimated on contaminated splits does not generalize and will not replicate.

### Diagnose Overfitting Rather Than Assume Its Absence

Overfitting is the default failure of flexible models, and it hides behind good training performance. The signal of overfitting is the gap between training and out-of-sample performance, not the absolute level of either.

Diagnose by:

- comparing training performance to validation and test performance, with large gaps flagging overfitting;
- using cross-validation that respects the data structure to estimate honest performance;
- watching for instability of selected features or hyperparameters across folds;
- checking performance on relevant subgroups, not only on the aggregate;
- recognizing that a model tuned to a single validation set has partially overfit that set.

A model with near-perfect training accuracy and modest test accuracy has memorized, not learned. The relevant question is how the model performs on data it has never influenced, including data from a different time or site.

### Choose Models By Honest Criteria And The Bias-Variance Tradeoff

Model choice for prediction should balance bias and variance against the data, the noise, and the cost of complexity. Selecting the most flexible model because it wins one validation round ignores the tradeoff.

Choose by:

- matching model capacity to the amount and noise level of the data, since scarce noisy data rewards simpler models;
- penalizing complexity to avoid chasing noise, through regularization or simpler model families;
- evaluating with metrics that match the predictive goal, such as calibration and discrimination for probabilities, not only accuracy;
- comparing against a sensible baseline, such as a regularized regression, to confirm the complexity is earning its keep;
- reporting the full set of models considered, not only the winner.

The model that wins a tuning competition on one validation set is often not the model that generalizes best. Simplicity, stability, and calibration matter as much as the headline metric.

### Weigh Interpretability Against Accuracy Deliberately

In many research and applied contexts, a model must be understood, audited, or trusted, not merely accurate. The interpretability-accuracy tradeoff is a real decision, not an obstacle to be optimized away.

Weigh by:

- whether stakeholders need to understand, contest, or act on individual predictions;
- whether the cost of an opaque error, in fairness, safety, or liability, exceeds the accuracy gain;
- whether post-hoc explanations suffice, recognizing their fragility and non-uniqueness;
- whether a simpler transparent model achieves adequate performance for the use case;
- the regulatory or ethical demand for explainability in the deployment context.

A black-box model with two percent higher accuracy may be unacceptable where decisions affect people and must be justified. Interpretability is a first-class requirement, not a consolation prize.

### Validate Generalization Across Context, Time, And Population

A model validated on one dataset may fail on another that differs in population, setting, or time. External validity is the real test of a predictive model.

Validate beyond the source by:

- testing on data from a different site, period, or population before claiming generalization;
- checking calibration drift, since prediction systems degrade as the world changes;
- examining performance across subgroups for fairness and equity, not only on average;
- monitoring performance after deployment and defining a retraining or retirement trigger;
- reporting the population and conditions to which performance is known to apply.

A model that performs well in one hospital or one year is not thereby a general tool. Generalization is demonstrated by external testing, not asserted from internal validation.

### Make The Entire Pipeline Reproducible

A predictive result is only as credible as the pipeline that produced it, and ML pipelines are complex enough to be irreproducible by default.

Reproduce by:

- versioning data, code, features, hyperparameters, and model artifacts;
- fixing random seeds and recording software and library versions, since ML results are sensitive to these;
- automating the full path from raw data to reported metric, so it can be rerun exactly;
- documenting every preprocessing, selection, and tuning step, including those that failed;
- sharing or containerizing the environment so others can regenerate the result.

A model whose reported accuracy cannot be reproduced because the tuning script was lost, the data drifted, or the library updated is a claim without evidence. Reproducibility is part of the result, not an afterthought.

## Common Traps

### Predictive Success Read As Causal Understanding

High accuracy does not reveal mechanism, importance, or the effect of intervention; treating it as if it does overclaims what the model shows.

### Information Leakage Across Splits

Letting validation or test data influence training, through preprocessing, feature selection, or repeated tuning, inflates performance and destroys honest estimation.

### Overfitting Hidden By Good Training Metrics

A model that fits training data near-perfectly but generalizes poorly has memorized noise; the training-test gap, not the training score, reveals it.

### Selecting On A Single Validation Pass

Tuning until one validation set is beaten partially overfits that set, so reported performance is optimistic and will not replicate.

### Ignoring The Bias-Variance Tradeoff

Using the most flexible model regardless of data size and noise chases signal that is not there and produces unstable, non-generalizing models.

### Sacrificing Interpretability Silently

Choosing an opaque model for a small accuracy gain in a context that demands audit or fairness ignores the real cost of unexplainable decisions.

### Overclaiming External Validity

Strong performance on one dataset does not generalize to other populations, sites, or times without external testing and drift monitoring.

### Irreproducible Pipelines

Reported accuracy from a pipeline whose data, code, seeds, and versions are not recorded cannot be checked and is not a reliable result.

## Self-Check

- Is the predictive goal stated explicitly and kept separate from causal or explanatory claims?
- Are train, validation, and test splits enforced at the correct unit, with all preprocessing and selection fit within training folds and the test set used exactly once?
- Is overfitting diagnosed by the training-to-out-of-sample gap, cross-validation that respects structure, and stability across folds, rather than assumed absent?
- Is model choice guided by the bias-variance tradeoff, appropriate metrics, a sensible baseline, and reported alternatives, not by the single validation winner?
- Is the interpretability-accuracy tradeoff weighed against the need for audit, fairness, and trust in the deployment context?
- Has the model been tested on external data from different populations, sites, or times, with calibration drift and subgroup performance examined?
- Are fairness and equity across subgroups checked, not only aggregate performance?
- Is the full pipeline, including data, code, features, hyperparameters, seeds, versions, and failed attempts, versioned and reproducible?
- Are the population and conditions to which performance is known to apply reported honestly?
- Is the reported performance one that another researcher could regenerate from the recorded pipeline?
