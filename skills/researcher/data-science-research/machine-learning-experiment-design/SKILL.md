---
name: machine_learning_experiment_design.md
description: Use when the agent is designing a machine learning experiment, splitting data for training and evaluation, choosing baselines and metrics, controlling for randomness, or ensuring that reported model performance reflects genuine capability rather than leakage or overfitting.
---

# Machine Learning Experiment Design

Machine learning experiments are easy to run and easy to get wrong. Because the same data can be sliced many ways and because random variation is large, a model can appear to outperform a baseline through leakage, lucky splits, metric shopping, or overfitting to the validation set. When ML experiment design is loose, three harms follow. Models are published or deployed with performance that does not hold up, wasting effort and misleading the field. Real improvements are missed because the experiment could not detect them. And in deployed systems, inflated performance estimates lead to over-trust in models that fail on real data, sometimes with serious consequences. ML experiment design is a discipline of making performance estimates mean what they claim to mean.

The agent should use this skill when designing an ML experiment, setting up train, validation, and test splits, choosing evaluation metrics and baselines, running ablations, or interpreting claims about model performance. The goal is to keep the agent from treating a high score as evidence of a good model, when a high score without rigorous design is often evidence of a flawed experiment.

## Core Rules

### Separate Training, Validation, And Test Strictly

The integrity of the test set determines whether performance estimates are meaningful.

- Reserve a test set touched only once, for final evaluation.
- Use a validation set for model selection and tuning.
- Ensure no information from the test set leaks into training or selection.
- Prevent leakage through preprocessing, feature selection, or hyperparameter tuning done on the wrong data.

Leakage is the most common cause of inflated performance. Any step that uses test data, directly or indirectly, contaminates the estimate. Audit the full pipeline for leakage, not just the model fitting.

### Prevent Data Leakage Across Groups And Time

Standard random splits fail when data have grouping or temporal structure.

- Group leakage, when records from the same patient, user, or site appear in both train and test.
- Temporal leakage, when future data trains a model meant to predict the past.
- Spatial leakage, when nearby or correlated observations span the split.
- Duplicate or near-duplicate records crossing the split.

Use group-aware or time-aware splits that respect the structure. A model that performs well on random splits but poorly on grouped or forward splits has learned to recognize groups, not to generalize.

### Choose Metrics That Match The Decision

A single metric rarely captures what matters. Choose metrics aligned with the actual use.

- For classification, consider accuracy, precision, recall, F1, and calibration, not accuracy alone.
- For imbalanced data, accuracy is misleading; use metrics sensitive to the minority class.
- For probabilistic outputs, assess calibration, not just discrimination.
- For ranking, use ranking metrics relevant to the use case.
- For regression, consider error in meaningful units and its distribution.

Reporting only the metric where the model looks best is metric shopping. Pre-specify the primary metric, report a panel, and be honest about where the model is weak.

### Establish Appropriate Baselines

A model's performance is meaningful only relative to baselines.

- Simple baselines, such as majority class, mean prediction, or linear models.
- Strong baselines, prior state-of-the-art or well-tuned classical methods.
- The baseline the model is meant to replace, in deployment contexts.

A complex model that beats a weak baseline but not a strong one is not an advance. A model that beats a strong baseline by a tiny, noisy margin may not be a real improvement. Choose baselines that make the comparison meaningful.

### Control Randomness And Report Variance

ML experiments involve randomness in splitting, initialization, sampling, and training. A single run is an anecdote.

- Set and report random seeds.
- Run multiple seeds or splits and report mean and variance.
- Report confidence intervals or significance for key comparisons.
- Distinguish real differences from noise.

A reported improvement within the noise of a single run is not an improvement. Variance reporting is what separates a real effect from a lucky seed.

### Guard Against Overfitting To The Validation Set

Repeatedly tuning on the same validation set leads to overfitting to it, even without touching the test set.

- Limit the number of model selection decisions.
- Use nested cross-validation for unbiased performance estimation when tuning heavily.
- Reserve the test set for a single final evaluation.
- Be honest about how many configurations were tried.

A model selected after hundreds of validation trials has partly memorized the validation set. The test estimate is the check on whether the selection generalizes.

### Run Ablations To Understand What Drives Performance

A model's score does not explain why it works. Ablations do.

- Remove or swap components to see what contributes.
- Test the effect of data, features, and architecture separately.
- Compare to simpler variants to isolate gains.
- Identify whether performance comes from the method or from the data.

Without ablations, it is unclear whether an improvement comes from the proposed innovation or from incidental choices. Ablations make the contribution honest.

### Test On Out-Of-Distribution And Realistic Conditions

In-distribution test performance overstates real-world performance.

- Evaluate on data from different sites, times, or populations.
- Test robustness to shifts in distribution.
- Assess performance on subgroups that may be underrepresented.
- Consider adversarial or edge cases relevant to deployment.

A model with high in-distribution accuracy that collapses on shifted data is not deployment-ready. Out-of-distribution evaluation reveals the brittleness that deployment will expose.

### Report Honestly And Completely and pre-Register Or Pre-Define The Evaluation Plan

Report what was done, not just what worked.

- Report all metrics, not only the best.
- Report failed approaches and negative results.
- Disclose the number of experiments run and selection performed.
- Provide code, data, and configuration for reproducibility.

Selective reporting inflates the apparent performance of methods across the field. Complete reporting lets readers calibrate trust.

To prevent post-hoc metric and split selection, define the evaluation plan in advance.

- Specify primary metric, splits, baselines, and success criteria.
- Commit to the plan before running final evaluations.
- Report deviations and their reasons.

Pre-definition is the strongest defense against the slow drift toward favorable choices. Even an internal pre-registration, shared with collaborators, disciplines the analysis.

## Common Traps

### Test Set Leakage

Any use of test data in training or selection inflates performance. Audit the pipeline.

### Random Splits On Grouped Or Temporal Data

Random splits overestimate performance on grouped or time-series data. Use structure-aware splits.

### Metric Shopping

Reporting only favorable metrics misleads. Pre-specify and report a panel.

### Weak Baselines

Beating a weak baseline is not an advance. Use strong baselines.

### Single-Run Results

One run is an anecdote. Report variance across seeds and splits.

### Overfitting To Validation

Heavy tuning overfits the validation set. Use nested methods and a held-out test.

### Skipping Ablations

A score without ablations does not explain the gain. Run them.

### Ignoring Distribution Shift and selective Reporting

In-distribution accuracy overstates real performance. Test out-of-distribution.

Reporting only successes inflates the field. Report failures too.

## Self-Check

- Are training, validation, and test sets strictly separated, with the test set used only once?
- Is the pipeline audited for leakage through preprocessing, features, grouping, time, or duplicates?
- Are metrics chosen to match the decision, pre-specified, and reported as a panel rather than only the best?
- Are baselines appropriate, including simple and strong comparators?
- Is randomness controlled through seeds, multiple runs, and variance reporting?
- Is overfitting to the validation set guarded against through limited selection and nested methods?
- Are ablations run to identify what drives performance?
- Is the model evaluated out-of-distribution and on relevant subgroups and edge cases?
- Is reporting complete, including failed approaches, number of experiments, and reproducibility materials?
- Is the evaluation plan pre-defined or pre-registered to prevent post-hoc favorable choices?; for models intended for deployment or high-stakes decisions, has an experienced ML researcher or methodologist reviewed the experiment design and leakage audit before performance claims are made?
