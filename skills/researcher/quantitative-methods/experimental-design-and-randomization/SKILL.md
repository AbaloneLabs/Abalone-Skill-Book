---
name: experimental_design_and_randomization.md
description: Use when the agent is designing an experiment, choosing between randomized and quasi-experimental designs, planning random assignment, blocking or stratifying, defining treatment arms and controls, planning blinding, or defending internal and external validity of a causal claim.
---

# Experimental Design And Randomization

An experiment is the strongest design for causal inference because random assignment, done well, breaks the link between treatment and confounders. But the strength of an experiment is not automatic. Weak randomization, broken blinding, differential attrition, contamination between arms, and post hoc adjustments can reintroduce the very bias that randomization was meant to remove. Researchers often treat "randomized" as a certificate of validity and stop scrutinizing the design.

Use this skill when designing an experiment, when planning randomization and blinding, when choosing controls and treatment arms, and when defending the causal claim the experiment is meant to support. The goal is to keep the agent from confusing random assignment with automatic internal validity, from under-specifying the randomization procedure, and from overclaiming causality when the design's assumptions are violated. The agent has latitude in design choice, but must justify how each threat to validity is addressed.

## Core Rules

### Match The Design To The Causal estimand

An experiment estimates a specific causal quantity, and the design must be built around it. Vague intentions to "test the effect" produce designs that estimate something other than what was claimed.

Define:

- the estimand, such as average treatment effect, treatment effect on the treated, or local average treatment effect;
- the population to which the estimate applies;
- the treatment, including dose, timing, and delivery;
- the comparator, including placebo, active control, or usual care;
- the outcome and its timing;
- the subgroups for which effects will be estimated.

The estimand dictates the randomization unit, the blocking, the analysis, and the population that can be generalized to.

### Randomize At The Right Unit And Document The Mechanism

Randomization is only as good as its implementation. The unit of randomization must match the unit of analysis and the unit at which treatment is delivered.

Decide and document:

- individual versus cluster randomization;
- the randomization sequence generation method;
- the allocation ratio and any restrictions;
- blocking and stratification factors;
- whether assignment is concealed until enrollment;
- who generates, who assigns, and who enrolls;
- how deviations and errors are recorded.

If the unit of assignment differs from the unit of analysis, the analysis must account for clustering. Convenience assignment labeled "random" is not randomization.

### Block And Stratify To Protect Precision And Balance

Simple randomization can produce unlucky imbalances, especially in smaller studies. Blocking and stratification protect against this and improve precision.

Plan:

- block sizes to prevent long runs of one arm;
- stratification on strong prognostic factors;
- minimization as an alternative in some settings;
- whether stratification factors will be adjusted for in analysis;
- the risk of predictability from small or fixed block sizes;
- the trade-off between complexity and feasibility.

Stratification that is not adjusted for in the analysis wastes its benefit. Stratification that is too granular can fragment the sample.

### Choose Controls And Comparators Deliberately

The comparator defines the effect being estimated. A weak or absent comparator turns an experiment into a before-after observation.

Consider:

- placebo control to blind for subjective outcomes;
- active control to compare against standard practice;
- waitlist control and its expectancy effects;
- usual care and its variability;
- no-treatment control and its ethical limits;
- dose-response arms to characterize the relationship;
- factorial arms to test multiple questions efficiently.

A comparator that drifts, contaminates, or is delivered differently across sites undermines the estimate.

### Blind Wherever Possible And Plan For Its Limits

Blinding reduces performance and detection bias. Where blinding is impossible, its absence must be acknowledged and mitigated.

Plan:

- participant blinding;
- provider or experimenter blinding;
- outcome assessor blinding, especially for subjective outcomes;
- analyst blinding through masked datasets;
- the integrity checks for blinding;
- the mitigations when blinding is infeasible, such as objective outcomes or independent assessors.

Unblinded subjective outcomes are a major threat to internal validity. If blinding is partial, state which parties were blinded.

### Anticipate And Address Threats To Internal Validity

Internal validity is the property that the observed difference is attributable to the treatment. Many threats can undo randomization.

Address:

- differential attrition and missing data;
- noncompliance with assigned treatment;
- contamination or spillover between arms;
- co-interventions that differ across arms;
- regression to the mean in selected populations;
- Hawthorne and experimenter effects;
- timing, seasonality, and historical events;
- changes in measurement during the study.

Plan the analysis to handle these, using intent-to-treat where appropriate and pre-specified secondary analyses for complier-average effects.

### Plan The Analysis To Honor The Design

The analysis must reflect how the study was conducted. Mismatches between design and analysis produce biased estimates.

Specify:

- intent-to-treat as the primary analysis for superiority trials;
- per-protocol and as-treated as secondary, with caveats;
- adjustment for stratification and blocking factors;
- handling of clustering if cluster-randomized;
- handling of repeated measures and time-to-event outcomes;
- multiplicity adjustment for multiple arms, outcomes, or time points;
- interim analyses and stopping rules;
- missing data methods, with sensitivity analyses.

An analysis that picks the most favorable estimate among several is not the planned analysis.

### Distinguish Internal From External Validity

A well-randomized study has strong internal validity but may have weak external validity. The two are different and must be argued separately.

For external validity, consider:

- the population enrolled versus the population of interest;
- the setting and its representativeness;
- the treatment as delivered versus as it would be used;
- the outcome measures and their relevance;
- the dose, timing, and intensity;
- contextual factors that moderate the effect.

Internal validity is necessary but not sufficient. A precise estimate of an effect in an unrepresentative sample may not generalize.

## Common Traps

### Treating Randomization As Automatic Validity

Randomization creates the potential for unbiased inference; it does not guarantee it once attrition, noncompliance, and contamination enter.

### Mismatching Unit Of Assignment And Unit Of Analysis

Cluster-randomized studies analyzed as if individually randomized produce falsely narrow intervals and inflated false positives.

### Weak Or Predictable Allocation

Assignments that can be predicted, such as from small fixed block sizes, allow enrollment to be manipulated and undo randomization.

### Inadequate Or Partial Blinding Left Unstated

Reporting "double-blind" without specifying who was blinded hides the real risk of bias.

### Choosing A Convenient Comparator

A no-treatment or waitlist comparator that produces expectancy effects can manufacture an apparent treatment effect.

### Ignoring Differential Attrition

If one arm loses more participants, the surviving comparison no longer reflects randomization and bias enters.

### Overclaiming External Validity

A clean internal estimate from a narrow, motivated sample does not license broad claims about effectiveness in practice.

## Self-Check

- [ ] Is the causal estimand defined, including population, treatment, comparator, outcome, and subgroups?
- [ ] Is the unit of randomization matched to the unit of analysis and treatment delivery, with clustering handled if needed?
- [ ] Is the randomization mechanism documented, including sequence generation, allocation concealment, and blocking or stratification?
- [ ] Are controls and comparators chosen deliberately to estimate the intended effect?
- [ ] Is blinding planned for all feasible parties, with mitigations where it is impossible?
- [ ] Are threats to internal validity, including attrition, noncompliance, contamination, and co-intervention, anticipated and addressed?
- [ ] Does the primary analysis honor the design, with intent-to-treat where appropriate and pre-specified secondary analyses?
- [ ] Are multiplicity, interim analyses, and missing data planned in advance?
- [ ] Is external validity argued separately from internal validity, with limits stated?
- [ ] Are deviations from the planned design reported transparently rather than hidden?
