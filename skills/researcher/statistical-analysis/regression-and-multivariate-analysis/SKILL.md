---
name: regression_and_multivariate_analysis.md
description: Use when the agent is specifying a regression or multivariate model, selecting predictors, interpreting coefficients, handling confounding and collinearity, checking model assumptions and fit, choosing between competing models, or defending whether the model supports causal or predictive claims.
---

# Regression And Multivariate Analysis

Regression models are the workhorses of quantitative research, and they are also the workhorses that throw the most riders. A researcher adds variables until the result looks good, interprets a coefficient as a causal effect, ignores collinearity and influential points, and reports a model that fits the sample but generalizes to nothing. Multivariate methods amplify these risks because their flexibility makes it easy to find a model that supports any preferred story.

Use this skill when specifying a model, selecting variables, interpreting coefficients, diagnosing problems, choosing among models, and defending the claim the model supports. The goal is to keep the agent from treating model output as ground truth, from confusing adjustment with causal identification, and from overfitting in the pursuit of fit. The agent has latitude in modeling strategy, but must justify each specification choice and report diagnostics honestly.

## Core Rules

### Specify The Model From Theory And Design, Not From Fit

A model encodes assumptions about how variables relate. Building it by adding and dropping predictors until fit improves produces a model tuned to sample noise.

Ground the model in:

- the substantive question and estimand;
- the causal or predictive structure hypothesized;
- known confounders that must be adjusted for;
- variables that should not be adjusted for, such as mediators or colliders;
- the functional form expected for each relationship;
- interactions that theory suggests;
- the level of measurement and distribution of each variable.

Document the reasoning behind each variable's inclusion or exclusion. A model built by p-value shopping is not a model of the phenomenon.

### Separate Causal From Predictive Goals

Regression can be used to estimate causal effects or to predict outcomes, and the two goals demand different choices and different validation.

Clarify:

- causal goals prioritize unbiased estimation of specific coefficients;
- predictive goals prioritize overall accuracy and calibration;
- confounders are essential for causal goals and may be optional for prediction;
- predictors that improve prediction can bias causal estimates, such as instruments or colliders;
- validation differs: out-of-sample prediction for prediction, design and identification for causation;
- reporting differs: coefficients and standard errors for causal, performance metrics for predictive.

A model that predicts well does not thereby identify causal effects, and a causal estimate is not judged by predictive fit.

### Identify And Address Confounding Properly

Adjustment is the most common response to confounding, but adjusting for the wrong variables can introduce or worsen bias.

Use a structural framework to decide:

- confounders, which should be adjusted for;
- mediators, which generally should not be adjusted for in a total effect analysis;
- colliders, which should not be adjusted for;
- instrumental variables, which should not be entered as covariates;
- proxies whose adjustment may help or hurt depending on structure;
- time-varying confounders affected by prior treatment, which need special methods.

A causal diagram or equivalent reasoning should guide adjustment decisions. Adjusting for everything available is not conservative; it is often wrong.

### Detect And Handle Collinearity

Collinearity inflates standard errors and makes coefficients unstable. It does not bias estimates, but it undermines their interpretability and precision.

Check:

- pairwise correlations among predictors;
- variance inflation factors or condition indices;
- whether collinearity is structural, such as polynomial terms, or accidental;
- whether predictors are proxies for the same underlying construct;
- the effect on the specific coefficient of interest, not just overall fit.

Responses include combining predictors, dropping redundant ones, using regularization, or reframing the question. Do not ignore collinearity because the model still fits.

### Diagnose Assumptions And Influential Points

A regression model assumes a great deal about its errors and functional form. These assumptions must be checked, not assumed.

Examine:

- linearity of relationships, with residuals or partial plots;
- homoscedasticity of residuals;
- normality of residuals for inference where assumed;
- independence of observations, with clustering handled;
- influential points, with leverage and Cook's distance;
- outliers and their effect on estimates;
- the link function for generalized models;
- overdispersion for count models.

A single influential point can reverse a coefficient. Report diagnostics and the effect of addressing problems.

### Interpret Coefficients In Their Proper Units And Context

A coefficient is a conditional association, and its meaning depends on the model, the variables, and the scale. Generic statements such as "X affects Y" overstate what the model shows.

Interpret:

- the units of the outcome and predictor;
- whether the coefficient is on a transformed scale, such as log or link;
- whether the association is conditional on other variables in the model;
- the uncertainty around the estimate;
- whether the interpretation is causal, predictive, or descriptive;
- the practical magnitude, not just the sign and significance.

A coefficient is not a verdict; it is a quantity that requires context to mean anything.

### Choose Among Models With Justified Criteria

Researchers often compare many models and report the best-fitting. This practice overfits and inflates significance.

Use principled criteria:

- pre-specify the model where possible;
- use theory and design to fix the structure;
- compare nested models with likelihood-based tests only when justified;
- use information criteria with awareness of their assumptions;
- use cross-validation or out-of-sample testing for prediction;
- penalize complexity to avoid overfitting;
- report the models considered, not just the chosen one.

Selecting on fit and reporting only the winner produces optimistic and irreproducible results.

### Validate The Model And Report Its Limits

A model fit to one sample may not generalize. Validation and honest reporting of limits protect against overclaiming.

Validate and report:

- in-sample versus out-of-sample performance for predictive models;
- calibration of predicted probabilities;
- sensitivity to influential points and alternative specifications;
- performance in relevant subgroups;
- the population and setting to which the model applies;
- known weaknesses and untestable assumptions;
- the distinction between association and causation the model can support.

A model is a conditional summary of one dataset. Its claims extend only as far as the design, assumptions, and validation allow.

## Common Traps

### Stepwise Selection Without Justification

Adding and dropping variables by p-value or fit criterion overfits and produces irreproducible models.

### Confusing Adjustment With Causal Identification

Adjusting for more variables does not make an estimate causal. Adjusting for mediators or colliders can increase bias.

### Ignoring Collinearity

Collinearity destabilizes coefficients and inflates uncertainty. A high R-squared does not mean the model is sound.

### Overreading A Single Coefficient

A coefficient is a conditional association with uncertainty, not a definitive effect. Sign and significance are not interpretation.

### Skipping Diagnostics

Assumptions about linearity, homoscedasticity, independence, and influence are checkable. Skipping the checks invalidates the inference.

### Reporting Only The Best-Fitting Model

Choosing among many models and reporting the winner produces optimistic results and hides the search.

### Claiming Prediction From In-Sample Fit

A model that fits its training data does not necessarily predict new data. Out-of-sample validation is required.

## Self-Check

- [ ] Is the model specified from theory, design, and a structural understanding of confounding rather than from fit?
- [ ] Is the goal, causal estimation or prediction, made explicit, with variables and validation chosen accordingly?
- [ ] Are confounders, mediators, colliders, and instruments handled correctly using a structural framework?
- [ ] Is collinearity diagnosed and addressed, with attention to the coefficient of interest?
- [ ] Are assumptions, including linearity, homoscedasticity, independence, and influence, checked with diagnostics?
- [ ] Are coefficients interpreted in proper units, scale, and context, with uncertainty and practical magnitude?
- [ ] Are model comparisons made with justified criteria, and are considered models reported?
- [ ] Is the model validated out-of-sample where prediction is claimed, with calibration checked?
- [ ] Are sensitivity analyses to influential points and alternative specifications reported?
- [ ] Are the limits of the model, including the association-versus-causation boundary, stated honestly?
