---
name: causal_inference_and_attribution.md
description: Use when the agent is drawing causal conclusions from evidence, interpreting associations, choosing between causal inference frameworks, planning instrumental variable or natural experiment analyses, building directed acyclic graphs, judging whether a design supports causal claims, or deciding how strongly to word a causal statement.
---

# Causal Inference And Attribution

Causation is the most overclaimed conclusion in research. A correlation, a difference in means, a regression coefficient, a qualitative observation of sequence, or a statistically significant association can all be real and still tell the researcher nothing about what would happen if something were changed. Yet policy, treatment, and theory decisions depend on causal answers. The central judgment problem is not how to compute an effect estimate but how to decide whether a given estimate, produced by a given design under a given set of assumptions, licenses a causal claim at all, and how strong that claim may be.

Use this skill before attributing an outcome to a cause, before choosing an identification strategy, before interpreting a coefficient as an effect, before writing causal language into a conclusion, and before appraising someone else's causal claim. The goal is to keep the agent from converting a well-measured association into a causal assertion the design cannot support, and to keep it from refusing to make a defensible causal claim when the evidence genuinely warrants one. The agent has freedom to choose designs and frameworks, but must make the identifying assumptions explicit and name what would have to be true for the causal claim to hold.

## Core Rules

### Distinguish Association From Causation At The Outset

Association is what is observed in the data: two variables move together. Causation is a claim about an intervention: if this variable were set to a different value, the outcome would change. These are different quantities, and a design that estimates one well may estimate the other poorly.

Clarify which question is being asked:

- Is the goal description, prediction, or causal explanation?
- Does the decision depend on what happens if something is changed, or only on knowing who tends to have what?
- Is the target a population average effect, an effect on the treated, an effect on a specific subgroup, or a mechanism?
- Is the treatment binary, continuous, multi-valued, or time-varying?

A predictive model can be excellent and causally useless. A causal estimate can be correct and predict poorly. Name the target before choosing the method.

### Know The Conditions A Causal Claim Requires

A defensible causal claim rests on conditions that are stronger than statistical significance. The classical criteria include temporality, the cause preceding the effect; association, the cause and outcome covarying; and non-spuriousness, the association not being explained by a common cause. Modern frameworks make these conditions precise.

In the potential outcomes framework, a causal effect is defined as the difference between outcomes under different treatment states for the same unit. Because only one outcome is observed per unit, causal inference is fundamentally a missing data problem. Estimation requires assumptions about how the observed units stand in for the unobserved counterfactuals.

Identify the assumptions the design relies on:

- exchangeability, that treated and untreated are comparable on relevant factors conditional on observed covariates or in expectation;
- positivity, that every unit has a nonzero probability of receiving each treatment level;
- consistency, that a unit's observed outcome under a treatment equals the potential outcome under that treatment;
- no interference, that one unit's treatment does not affect another's outcome, unless the design accounts for spillover;
- correct measurement of treatment, outcome, and confounders.

State these assumptions explicitly and judge how plausible each is in the specific context.

### Encode Assumptions In A Directed Acyclic Graph

A directed acyclic graph makes the causal structure and the identifying assumptions visible and checkable. It forces the researcher to declare which variables cause which, before seeing which adjustments produce the desired result.

Use a graph to:

- declare the hypothesized causal structure;
- identify the set of variables that must be adjusted for to estimate a total effect;
- identify variables that must not be adjusted for, such as mediators and colliders, because conditioning on them introduces bias;
- expose unmeasured confounding paths;
- reason about whether an instrumental variable is valid.

Drawing the graph is a judgment act, not a clerical step. Two researchers with different causal beliefs will draw different graphs and reach different conclusions from the same data. The graph should be defensible from domain knowledge, not reverse-engineered from the estimate it produces.

### Match Identification Strategy To The Design Available

Different designs license causal claims under different assumptions, and some are far stronger than others.

Consider, in roughly increasing strength of assumptions:

- randomized experiments, where random assignment provides exchangeability by design;
- natural experiments and quasi-experiments, including regression discontinuity, interrupted time series, and difference in differences, where treatment assignment is as-if random around a threshold or over time;
- instrumental variable designs, where an instrument affects treatment but not the outcome except through treatment;
- matching, weighting, and regression adjustment on observed confounders, which depend on the assumption of no unmeasured confounding;
- structural and mechanistic reasoning, which depends on strong theory.

No observational method removes confounding in general. Each replaces randomization with an assumption that may or may not hold. The strength of the causal claim should track how plausible the identifying assumption is, not how sophisticated the estimator looks.

### Scrutinize Instrumental Variables And Natural Experiments

These designs are powerful but fragile, and their assumptions are easy to state and hard to verify.

For instrumental variables, the instrument must be relevant, strongly associated with treatment; exogenous, not correlated with the unobserved confounders; and exclusive, affecting the outcome only through treatment. The exclusion restriction cannot be tested directly and is often the point of failure. The estimate is a local average effect on compliers, not a population average effect, and extrapolating it is a separate assumption.

For regression discontinuity, the design is credible only if units cannot precisely sort around the threshold and if the functional form near the cutoff is modeled defensibly. For difference in differences, the parallel trends assumption is central and untestable in the treatment period; pre-trends are suggestive, not proof. For interrupted time series, other events coinciding with the intervention are a constant threat.

Name the specific assumption each design leans on and what evidence supports or undermines it.

### Be Honest About Unmeasured Confounding

In observational research, the single largest threat is a cause of both treatment and outcome that was not measured. No amount of covariate adjustment for observed variables can address an unmeasured confounder, and adding many observed covariates can create a false sense of control.

Consider:

- sensitivity analyses that show how strong an unmeasured confounder would need to be to overturn the conclusion;
- whether the observed covariates are plausible proxies for the important unobserved ones;
- whether selection into treatment is driven by factors also related to the outcome;
- whether the magnitude of the estimated effect is plausible given plausible confounding.

A causal claim from observational data should report a sensitivity analysis or explicitly acknowledge that unmeasured confounding is an unresolvable threat.

### Calibrate Causal Language To The Evidence

The words used to describe a finding imply a level of causal commitment, and readers act on those words. Calibrate language to what the design can support.

- Randomized experiments with high adherence and low attrition can support causal verbs.
- Strong quasi-experiments can support cautious causal language with stated assumptions.
- Observational studies with adjustment for observed confounders should hedge, naming the unmeasured confounding threat.
- Cross-sectional associations should use associational language only.
- Mechanistic or theoretical arguments should be labeled as hypotheses.

Words such as causes, leads to, increases, reduces, drives, and produces imply intervention. Words such as is associated with, is related to, predicts, and correlates with do not. A single verb choice can convert a defensible association into an indefensible causal claim.

### Separate The Effect Estimate From Its Interpretation

An estimated coefficient is a number. Its causal meaning depends on the assumptions that connect it to a counterfactual. Two researchers can agree on the number and disagree on whether it is a causal effect.

Report:

- the estimand, the causal quantity being targeted;
- the identification strategy and its assumptions;
- the estimator and its properties;
- the estimate with its uncertainty;
- the population and setting to which it applies;
- the assumptions under which it generalizes.

Do not present an estimate as a causal effect without stating the assumptions that license that interpretation.

## Common Traps

### Calling An Adjusted Association A Causal Effect

Adjusting for observed covariates reduces bias only for those covariates. It does not remove unmeasured confounding, and it does not convert an observational design into an experiment. Calling the result a causal effect hides the central untestable assumption.

### Conditioning On A Mediator And Calling The Result A Total Effect

Adjusting for a variable on the causal pathway between treatment and outcome estimates a different quantity, a controlled or natural direct effect, not the total effect. Reporting it as the total effect understates the true effect and answers a different question than intended.

### Conditioning On A Collider And Creating Bias

Adjusting for a variable affected by both treatment and outcome, or by their causes, can induce a spurious association. Collider bias is invisible to researchers who select on the collider without recognizing it, as when studying only hospitalized patients or only survey responders.

### Treating A Strong Instrument As Proof Of Validity

A strong instrument addresses relevance, not exogeneity or exclusion. The hardest assumptions, exclusion and exogeneity, cannot be verified from the data. An instrument that predicts treatment well can still violate the exclusion restriction and produce a biased causal estimate.

### Overgeneralizing A Local Average Effect

Instrumental variable estimates apply to compliers, the units whose treatment status changes with the instrument. Applying the estimate to always-takers, never-takers, or a different population is an extrapolation that requires its own justification.

### Confusing Statistical Significance With Causal Strength

A small p-value means the association is unlikely under the null. It says nothing about whether the association is causal, how large the causal effect is, or whether it generalizes. Causal conclusions require design and assumptions, not significance alone.

### Reverse Causation In Cross-Sectional Designs

When treatment and outcome are measured at the same time, the direction of causation is unidentified. A plausible theory in one direction is not evidence against the reverse direction, and panel or longitudinal designs are needed to establish temporal order.

### Assuming Randomization Solves Everything

Randomization provides exchangeability at assignment, but nonadherence, attrition, and spillover can break the link between assignment and the effect of interest. Intention-to-treat and per-protocol estimates answer different questions, and both have assumptions.

## Self-Check

- Is the estimand explicitly defined as a causal quantity, and is it distinguished from the association observed in the data?
- Are the identifying assumptions, including exchangeability, positivity, consistency, and no interference, stated and judged for plausibility in this context?
- Is the causal structure encoded in a graph or equivalent explicit statement, and is it defensible from domain knowledge rather than reverse-engineered?
- Is the identification strategy matched to the design available, and is its central assumption named and scrutinized?
- For instrumental or quasi-experimental designs, are relevance, exclusion, parallel trends, or no-sorting assumptions examined rather than assumed?
- Is unmeasured confounding addressed through sensitivity analysis or explicit acknowledgment rather than ignored?
- Does the causal language track the strength of the design, avoiding causal verbs for designs that only support association?
- Are mediators, colliders, and post-treatment variables handled correctly rather than adjusted for indiscriminately?
- Is the effect estimate separated from its causal interpretation, with the estimand, assumptions, estimator, and generalization conditions reported?
- Is the population and setting to which the causal claim applies stated, and is extrapolation beyond it labeled as such?
