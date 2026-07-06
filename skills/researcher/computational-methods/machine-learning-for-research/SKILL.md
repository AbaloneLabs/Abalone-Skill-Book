---
name: machine-learning-for-research.md
description: Use when the agent is applying machine learning to a research question, choosing between prediction and inference goals, preventing data leakage and overfitting, evaluating models with appropriate cross-validation, interpreting model results without causal overreach, or ensuring reproducibility of the full modeling pipeline.
---

# Machine Learning For Research

Machine learning offers powerful tools for prediction, pattern discovery, and high-dimensional analysis, but importing it into research without methodological care produces confident-looking but unreliable results. The central distinction agents miss is between prediction and inference. A model that predicts well is not a model that explains, and a strong predictive feature is not necessarily a cause. Compounding this, machine learning pipelines have many degrees of freedom, feature engineering, hyperparameters, model selection, that can be tuned to the data in ways that look impressive on the held-out set but fail to generalize. The judgment problem is to keep the research goal, prediction or inference, explicit, to prevent data leakage and overfitting through disciplined validation, and to interpret results within the limits of what the model and the evaluation actually demonstrate. A machine learning study that reports high accuracy without honest validation, or that slips from prediction into causal language, produces results that will not replicate and claims the model cannot support.

Use this skill when applying machine learning in research. The goal is to prevent the agent from confusing prediction with inference, from leaking information across the train-test boundary, and from overclaiming what the evaluation demonstrates. The agent has freedom in model and architecture, but the goal, validation, and interpretation must be disciplined and reported.

## Core Rules

### Keep The Goal, Prediction Or Inference, Explicit

Prediction and inference are different goals with different methods and different valid claims. Conflating them is the root of most ML misuse.

Keep explicit by:

- prediction: the goal is accurate out-of-sample forecasting, and the model is a black box that need not explain;
- inference: the goal is understanding relationships, and the model must be interpretable and causally defensible;
- matching the model class and evaluation to the goal;
- avoiding causal language for predictive models and predictive boasts for inferential ones.

A high-accuracy predictive model does not reveal causes; a well-identified inferential model may predict poorly. The goal must govern every later choice.

### Prevent Data Leakage Rigorously

Data leakage, information from the test or validation set reaching training, inflates performance and produces models that fail in deployment.

Prevent by:

- separating train, validation, and test sets before any preprocessing or feature selection;
- fitting preprocessing, imputation, and feature engineering only on training data;
- using nested cross-validation when selection happens within cross-validation;
- checking for temporal, spatial, or group-based leakage that splits ignore.

Leakage is often subtle: scaling on the full dataset, or splitting by row when data are clustered by patient or site. Disciplined separation is essential because leakage silently invalidates every performance number.

### Guard Against Overfitting And The Multiplicity Problem

Modern ML pipelines have many tunable choices. Tuning them to the validation set overfits to it.

Guard by:

- holding out a true test set touched only once for final evaluation;
- nested cross-validation for honest performance when tuning heavily;
- pre-registering or freezing the pipeline before final evaluation where possible;
- recognizing the garden of forking paths in feature and model choice.

With enough choices, a researcher can find a specification that performs well by chance. Honest evaluation requires that the final test be genuinely unseen by the modeling process.

### Evaluate With Metrics And Splits Matched To The Problem

Accuracy is rarely the right metric alone, and random splits are rarely the only relevant split.

Evaluate by:

- metrics matched to the problem: precision, recall, AUC, calibration, not just accuracy;
- class imbalance handled through appropriate metrics and resampling;
- splits that reflect the deployment or scientific question: temporal, grouped, or spatial;
- performance under distribution shift where relevant.

Accuracy on a 99-percent-negative problem is meaningless. Random splits that put the same patient in train and test overstate performance. The evaluation must match the inferential target.

### Establish An Appropriate Baseline

A model's performance is meaningful only against a baseline. Without one, it is unclear whether the model adds anything.

Establish by:

- simple baselines: majority class, linear models, or existing domain rules;
- comparing the complex model against these baselines on the same splits;
- justifying complexity only when it improves on simpler approaches;
- reporting the baseline alongside the model's performance.

A deep model that beats a trivial baseline by a hair may not justify its complexity, its opacity, or its overfitting risk. Baselines keep claims honest.

### Interpret Models Within Their Limits

Interpretability tools reveal aspects of a model, not ground truth about the world.

Interpret by:

- distinguishing model interpretation from substantive explanation;
- recognizing that feature importance reflects the model and data, not causation;
- using multiple interpretation methods, as each has different assumptions;
- validating interpretations against domain knowledge and held-out cases.

SHAP values or attention weights describe how the model reaches its predictions, not why the phenomenon behaves as it does. Leaping from model interpretation to causal or substantive claim is overreach.

### Avoid Causal Overreach

Correlation, even high-performing correlation, is not causation. ML models optimized for prediction do not identify causes.

Avoid by:

- not interpreting predictive features as causal without an identification strategy;
- using causal inference or experimental methods when causation is the goal;
- acknowledging confounding and selection in the training data;
- reserving causal claims for designs that support them.

A model that predicts readmission from prior utilization is not showing that utilization causes readmission. Conflating prediction with causation is a frequent and serious error.

### Ensure Full Reproducibility

ML research must be reproducible: the same code, data, and environment must yield the same results.

Ensure by:

- versioned code, fixed random seeds, and recorded software environments;
- documented data splits, preprocessing, and hyperparameters;
- availability of code and, where possible, data or data statements;
- clear reporting of the full pipeline, not just the final model.

An ML result that cannot be reproduced is a claim, not a finding. The many degrees of freedom make reproducibility especially important and especially often neglected.

## Common Traps

### Prediction Confused With Inference

Treating a high-accuracy predictive model as if it explained or identified causes.

### Data Leakage

Letting test-set information reach training inflates performance and produces models that fail to generalize.

### Overfitting To The Validation Set

Tuning many choices against validation, then reporting validation performance as honest.

### Wrong Metric Or Split

Accuracy on imbalanced data or random splits on clustered data misrepresent real performance.

### No Baseline

Reporting model performance without a simple baseline hides whether the complexity adds value.

### Interpretation As Explanation

Treating feature importance or attention as causal or substantive truth about the world.

### Causal Claims From Predictive Models

Asserting causation from correlation-based models without an identification strategy.

## Self-Check

- Is the goal, prediction or inference, stated explicitly and matched to model and evaluation?
- Is data leakage prevented through disciplined separation of preprocessing and splits?
- Is overfitting controlled with a held-out test set and nested cross-validation where tuning is heavy?
- Are evaluation metrics and splits matched to the problem, including imbalance and grouping?
- Is an appropriate simple baseline established and compared on the same splits?
- Are interpretability tools used within their limits, distinct from substantive explanation?
- Are causal claims avoided unless an identification strategy supports them?
- Is the full pipeline reproducible, with code, seeds, environment, and data documented?
- Is complexity justified by improvement over simpler baselines?
- Does the reporting distinguish what the evaluation demonstrates from stronger claims?
