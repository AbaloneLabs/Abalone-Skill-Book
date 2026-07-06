---
name: regression_modeling_and_confounding.md
description: Use when the agent is building a regression or multivariate model, selecting predictors, addressing confounding and multicollinearity, interpreting coefficients, checking model assumptions and fit, or deciding which model best answers a research question without overfitting.
---

# Regression Modeling And Confounding

Regression is the workhorse of quantitative analysis, and it is also where confounding, overfitting, and misinterpretation hide most easily. A coefficient is not an effect just because the model produced it; it depends on which variables were included, how they were measured, whether the assumptions hold, and whether the model answers the question being asked. Agents tend to throw all available variables into a model, read off significant coefficients, and present them as causal effects. The discipline is to build models that correspond to the inferential question, to address confounding substantively rather than mechanically, and to interpret coefficients for what they actually represent.

Use this skill when building regression or multivariate models, selecting covariates, diagnosing confounding, or interpreting model output. The goal is to prevent the agent from producing models that fit the data but answer the wrong question.

## Core Rules

### Build The Model To Answer A Specific Question

A model is a tool for a question, not a dump of variables. Different questions require different specifications.

Define first:

- the estimand: what quantity does the study want to estimate?
- whether the goal is prediction, association, or causal explanation;
- which coefficient or contrast answers the question;
- the time frame and population the model targets.

A model built without a clear estimand produces coefficients no one knows how to interpret.

### Address Confounding Through Design And Substance, Not Just Covariates

Adding covariates does not automatically remove confounding. Some covariates help, some introduce bias, and some are irrelevant.

Address confounding by:

- identifying confounders from theory and prior knowledge, not data availability;
- including variables that are causes of both exposure and outcome;
- avoiding mediators and colliders that open biasing paths;
- considering design-based approaches (stratification, matching, instrumental variables) where appropriate.

Mechanical covariate adjustment can worsen bias if it conditions on a mediator or collider.

### Distinguish Confounders, Mediators, And Colliders

These have opposite implications for adjustment. Adjusting for the wrong type introduces the bias it was meant to remove.

Classify each variable:

- confounder: a common cause of exposure and outcome; adjust to reduce bias;
- mediator: on the causal path from exposure to outcome; adjusting blocks the effect of interest;
- collider: a common effect of exposure and outcome (or their causes); adjusting induces bias;
- proxy or independent predictor: may improve precision without addressing confounding.

Drawing a causal diagram clarifies which variables belong in the model.

### Check For And Handle Multicollinearity

High correlation among predictors inflates variance and makes coefficients unstable and uninterpretable, even if the model fits well.

Detect and address:

- examine correlations and variance inflation factors;
- determine whether collinearity reflects redundant measurement or a structural relationship;
- combine, drop, or reparameterize variables where justified;
- report that coefficient instability is due to collinearity, not a substantive null.

A non-significant coefficient under high collinearity does not mean the variable has no effect.

### Validate Model Assumptions

Every regression rests on assumptions about functional form, distribution, and error structure. Violated assumptions invalidate inference.

Check:

- linearity or correct functional form of continuous predictors;
- independence of errors and appropriate correlation structure;
- homoscedasticity or use of robust standard errors;
- distributional assumptions relevant to the model;
- influence of outliers and high-leverage points.

Report assumption diagnostics and sensitivity of conclusions to violations.

### Assess Fit And Predictive Performance Separately From Inference

A model can fit the data well and still answer the wrong question, or fit poorly and still estimate an effect of interest. Fit is not validation of the substantive claim.

Assess appropriately:

- goodness-of-fit and residual diagnostics for model adequacy;
- predictive performance (cross-validation, out-of-sample error) for prediction goals;
- the distinction between in-sample fit and generalization;
- overfitting when many predictors are used relative to events or observations.

High R-squared does not validate a causal interpretation.

### Avoid Overfitting And Specification Search

Trying many model specifications and reporting the best-fitting or most significant one inflates false positives and produces non-replicable results.

Guard by:

- pre-specifying the primary model based on theory;
- limiting specification search and labeling it exploratory;
- using cross-validation or hold-out samples for prediction;
- reporting sensitivity to reasonable alternative specifications.

A model chosen because it gave significance is unlikely to replicate.

### Interpret Coefficients In Their Proper Scale And Context

A coefficient's meaning depends on the variable's scale, the link function, and the other variables in the model. Misreading it is a common error.

Interpret by:

- stating the scale of the predictor and outcome;
- for nonlinear models, translating coefficients to meaningful quantities (odds ratios, risk differences, marginal effects);
- noting that a coefficient is conditional on the other variables in the model;
- avoiding causal language unless the design and assumptions support it.

A regression coefficient is an association conditional on the model, not necessarily an effect.

### Handle Interactions And Nonlinearity Deliberately and report The Model Transparently

Interactions and nonlinear terms change the interpretation of main effects and should be included based on theory, not discovered by search.

Include when:

- theory predicts effect modification;
- the functional form is known to be nonlinear;
- the interaction is pre-specified as a hypothesis;
- and then interpret the model as a whole, not the main effects alone.

Adding interactions until one is significant is multiplicity in disguise.

A reader should be able to understand what was estimated and why. Opaque modeling hides inferential choices.

Report:

- the estimand and why this model estimates it;
- which variables were included and why, with a causal diagram where relevant;
- the handling of missing data and assumptions;
- diagnostics, sensitivity analyses, and alternative specifications;
- software and version.

## Common Traps

### Throwing In All Available Variables

Mechanical covariate adjustment can introduce bias via mediators or colliders.

### Adjusting For Mediators Or Colliders

Conditioning on the wrong variable type creates the bias it was meant to remove.

### Reading Coefficients As Causal Effects

A conditional association is not a causal effect without design and assumptions.

### Ignoring Multicollinearity

Unstable coefficients under collinearity are misread as null effects.

### Specification Search For Significance

Reporting the best of many tried models inflates false positives and fails to replicate.

### High Fit Mistaken For Validity

R-squared measures fit, not whether the model answers the question or is causal.

### Unreported Assumption Violations

Inference from a model with violated assumptions is unreliable and misleading.

## Self-Check

- Is the model built to estimate a specific, stated estimand rather than to absorb all variables?
- Are confounders identified from theory, with mediators and colliders excluded from adjustment?
- Has a causal diagram or equivalent reasoning clarified which variables belong in the model?
- Is multicollinearity checked, and are unstable coefficients interpreted accordingly?
- Are model assumptions (form, independence, variance, distribution, influence) checked and reported?
- Is fit or predictive performance assessed separately from the substantive inference?
- Is the primary model pre-specified, with any specification search labeled exploratory?
- Are coefficients interpreted in their proper scale and context, without unsupported causal language?
- Are interactions and nonlinear terms theory-driven, not search-driven?
- Is the model reported transparently, including estimand, variables, diagnostics, and alternatives?
