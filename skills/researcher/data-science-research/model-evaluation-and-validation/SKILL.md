---
name: model_evaluation_and_validation.md
description: Use when the agent is evaluating or validating a machine learning model, choosing evaluation protocols, interpreting performance metrics, assessing calibration and error analysis, or deciding whether a model is reliable enough for its intended use.
---

# Model Evaluation And Validation

A model's reported performance is not its real performance. The number on a benchmark depends on the data it was measured on, the metric chosen, the calibration of its outputs, and the kinds of errors it makes. When model evaluation is shallow, three harms follow. Models are declared successful based on aggregate metrics that hide catastrophic failures on important subgroups. Decision-makers trust probabilistic outputs that are poorly calibrated, leading to bad decisions. And models are deployed into conditions unlike the ones they were evaluated on, where they fail silently. Model evaluation is the process of building an honest picture of how a model behaves, and that picture must be broader than a single score.

The agent should use this skill when evaluating a model, choosing metrics and protocols, conducting error analysis, assessing calibration, or deciding on deployment readiness. The goal is to keep the agent from equating a good aggregate score with a good model, when a model is only as trustworthy as the breadth and honesty of its evaluation.

## Core Rules

### Evaluate On Data That Represents The Intended Use

A model is only as good as its performance on the data it will actually face.

- Evaluate on data from the deployment distribution, not just the training distribution.
- Include data from different sites, times, populations, and conditions.
- Test on subgroups that will be affected by deployment.
- Anticipate distribution shift and drift over time.

A model that excels on a curated benchmark may fail on the messier data of real use. Match evaluation data to the intended use, and report where the match is uncertain.

### Look Beyond Aggregate Metrics To Subgroup Performance

Aggregate metrics hide where the model fails. Always examine subgroups.

- Performance by demographic, clinical, geographic, or behavioral subgroup.
- Performance across the range of input difficulty.
- Performance on rare but important cases.
- Whether some groups bear disproportionate error or harm.

A model with strong average performance that fails on a minority group is not acceptable for many uses. Subgroup evaluation is central to trustworthy assessment, especially for high-stakes decisions.

### Assess Calibration, Not Just Discrimination

For models that output probabilities or confidence, calibration matters as much as discrimination.

- Do predicted probabilities match observed frequencies?
- Are the model's confident predictions actually reliable?
- Does calibration hold across subgroups, or is the model overconfident for some?
- How does calibration affect downstream decisions that threshold the output?

A model with good discrimination but poor calibration produces confident wrong answers that decision-makers trust too much. Calibration is a first-class evaluation target for probabilistic models.

### Conduct Systematic Error Analysis

Knowing where a model fails is as important as knowing its average performance.

- Examine the highest-error cases and look for patterns.
- Identify systematic failure modes, not just random errors.
- Look for failure on edge cases, rare inputs, and out-of-distribution data.
- Categorize errors by type, false positives, false negatives, nonsensical outputs.

Error analysis reveals whether failures are tolerable and random or dangerous and systematic. It guides whether to deploy, refine, or reject the model.

### Choose Evaluation Protocols That Match The Decision

Different decisions need different evaluation protocols.

- Single held-out test for a one-time deployment decision.
- Repeated or cross-validation for stable performance estimates.
- Temporal validation for time-dependent use.
- Prospective validation on new data collected after model development.

A protocol appropriate for a research benchmark may be inadequate for a deployment decision. Match the protocol to the stakes and the use.

### Evaluate Against The Right Baselines And Thresholds

Performance is relative. State what the model is compared to.

- Simple baselines and existing methods.
- Human performance where relevant.
- Thresholds of acceptable performance for the use case.
- The cost of errors, not just their rate.

A model that beats a weak baseline but falls short of the performance needed for safe use is not deployment-ready. Connect performance to the thresholds and costs that define success in the application.

### Test Robustness And Failure Modes

Models fail in characteristic ways under stress. Test them.

- Robustness to input perturbations and noise.
- Behavior on missing or corrupted inputs.
- Performance under distribution shift.
- Failure gracefully or catastrophically.

A model that fails catastrophically on slightly shifted inputs is risky to deploy. Robustness testing reveals the conditions under which the model becomes unreliable.

### Validate Before And Monitor After Deployment

Evaluation does not end at deployment. Plan for ongoing validation.

- Validate prospectively before full deployment.
- Monitor performance on real data after deployment.
- Detect distribution drift and performance decay.
- Have thresholds and processes for retraining or rollback.

A model validated once and never monitored will degrade silently. Pre-deployment validation and post-deployment monitoring together constitute trustworthy evaluation.

### Report Performance Honestly And Contextually and connect Evaluation To The Decision To Deploy

Performance reporting should let others judge the model's reliability.

- Report aggregate and subgroup performance.
- Report calibration and error analysis, not just scores.
- Report the data, metrics, and protocols used.
- State the conditions under which performance holds and breaks.

Selective or context-free reporting overstates reliability. Honest, contextual reporting lets decision-makers calibrate trust.

Evaluation serves a decision. Make the connection explicit.

- Is performance sufficient for the intended use, given error costs?
- Are the failure modes acceptable for the stakes?
- Are the subgroups affected acceptably?
- What monitoring and safeguards are needed?

Evaluation that does not inform the deploy-or-not decision has not done its job. The final step of evaluation is a judgment about suitability for use, grounded in the evidence gathered.

## Common Traps

### Trusting Aggregate Scores

Averages hide subgroup failures. Always examine subgroups.

### Ignoring Calibration

Good discrimination with poor calibration produces confident wrong answers. Assess calibration.

### Skipping Error Analysis

Average performance hides systematic failure modes. Analyze errors.

### Evaluating Only On In-Distribution Data

Benchmark performance overstates real performance. Test on deployment-like data.

### Weak Or Missing Baselines

Performance without comparison is meaningless. Use strong baselines and relevant thresholds.

### Assuming Robustness

Models fail under shift and perturbation. Test robustness explicitly.

### Deploy-And-Forget

Performance decays after deployment. Validate prospectively and monitor.

### Context-Free Reporting

Scores without context overstate reliability. Report data, metrics, protocols, and limits.

## Self-Check

- Is the model evaluated on data representing the intended deployment distribution, including different sites, times, and conditions?
- Is subgroup performance examined, not just aggregate metrics?
- Is calibration assessed for probabilistic outputs, across subgroups?
- Is systematic error analysis conducted to identify failure modes?
- Does the evaluation protocol match the decision the evaluation serves?
- Is performance compared to appropriate baselines, human performance, and acceptable thresholds?
- Is robustness to perturbation, missing data, and distribution shift tested?
- Is prospective validation planned before and monitoring planned after deployment?
- Is performance reported honestly with data, metrics, protocols, subgroup results, calibration, and error analysis?
- Does the evaluation culminate in an explicit judgment about suitability for the intended use?; for models intended for high-stakes or safety-critical deployment, has an experienced ML evaluator or domain expert reviewed the evaluation and deployment readiness before launch?
