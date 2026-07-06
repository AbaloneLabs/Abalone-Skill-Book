---
name: quasi_experiment_selection_and_design.md
description: Use when the agent is choosing among quasi-experimental designs such as difference-in-differences, regression discontinuity, interrupted time series, or nonequivalent control group designs, matching the design to the causal question, or justifying a quasi-experiment when randomization is infeasible.
---

# Quasi Experiment Selection And Design

Random assignment is often impossible, unethical, or impractical, and quasi-experimental designs are the tools for drawing causal inferences without it. But quasi-experiments are not interchangeable, and none of them establishes causality as cleanly as randomization. Each design rests on specific assumptions and is vulnerable to specific threats. Choosing a difference-in-differences design when the parallel-trends assumption fails, or a regression discontinuity design when the running variable can be manipulated, produces a causal claim that looks rigorous but is biased. The skill is to match the design to the structure of the problem and to be explicit about the assumptions each design requires.

Use this skill when selecting a quasi-experimental design, justifying it over randomization, or assessing whether its assumptions hold. The goal is to prevent the agent from applying a trendy design whose assumptions the setting violates.

## Core Rules

### Justify Why Randomization Is Infeasible

Quasi-experiments are second-best when randomization is possible. The choice should be justified, not defaulted to.

Justify by:

- ethical constraints on randomizing the intervention;
- practical or political infeasibility;
- the intervention already having occurred;
- the cost or timeline of a randomized design.

If randomization is feasible and ethical, it is usually preferred; the quasi-experiment needs a reason.

### Match The Design To The Source Of Variation

Each quasi-experimental design exploits a different source of variation, and the design must fit how exposure to the intervention actually occurred.

Match by:

- difference-in-differences when a treated and comparison group are observed before and after, and parallel trends plausibly hold;
- regression discontinuity when treatment is assigned by a threshold on a running variable;
- interrupted time series when an intervention occurs at a point in a long series of observations;
- synthetic control when a single treated unit is compared to a weighted combination of untreated units;
- nonequivalent control group when groups are similar but not randomized.

The design must correspond to the real assignment mechanism, not to the analyst's preference.

### State And Assess The Core Identifying Assumption

Every quasi-experiment rests on an assumption that, if false, invalidates the causal estimate. This assumption must be named and examined.

Assess by design:

- difference-in-differences: parallel pre-treatment trends;
- regression discontinuity: continuity of potential outcomes at the threshold and no manipulation of the running variable;
- interrupted time series: no other concurrent changes and enough pre- and post-observations;
- synthetic control: the donor pool provides a credible counterfactual.

An assumption asserted but not examined is not a foundation.

### Check For Manipulation And Sorting

When assignment depends on a threshold or selection, individuals or units may sort around it, corrupting the design.

Check:

- in regression discontinuity, whether the density of the running variable jumps at the threshold;
- whether units could influence their treatment status;
- whether anticipation of the intervention changed behavior before it began;
- whether the threshold or cutoff was chosen to favor a result.

Manipulable assignment undermines the as-good-as-random claim at the threshold.

### Examine Pre-Treatment Parallel Trends

For difference-in-differences and related designs, the parallel-trends assumption is central and should be inspected, not assumed.

Inspect by:

- plotting pre-treatment trends for treated and comparison groups;
- testing for divergent pre-trends, while noting low power;
- considering whether events other than the intervention could differentially affect groups;
- using placebo or lead-lag analysis to check timing.

Divergent pre-trends suggest the comparison is not a valid counterfactual.

### Ensure Enough Observations For The Design

Several quasi-experimental designs are data-hungry. Underpowered designs produce imprecise or unstable estimates.

Verify:

- regression discontinuity needs sufficient observations near the threshold;
- interrupted time series needs enough points before and after, especially for seasonality;
- synthetic control needs a adequate donor pool and pre-intervention fit;
- difference-in-differences needs enough clusters if cluster-level treatment.

A regression discontinuity estimate from a handful of observations near the cutoff is too imprecise to support strong claims.

### Define The Estimand And Its Local Nature

Many quasi-experiments estimate a local effect, not an average effect for everyone. The estimand must be clear.

Define:

- regression discontinuity estimates the effect at the threshold, which may differ elsewhere;
- difference-in-differences estimates the effect on the treated, under assumptions;
- the population to which the local estimate generalizes;
- whether the local effect is policy-relevant.

A local average effect should not be narrated as the average effect for the whole population.

### Plan Sensitivity Analysis For Hidden Confounders

Quasi-experiments cannot rule out all confounding. Sensitivity analyses test how strong an unmeasured confounder would need to be to overturn the result.

Plan:

- bounds or sensitivity analysis for unmeasured confounding;
- placebo tests on outcomes that should not be affected;
- falsification tests on groups or periods that should show no effect;
- alternative comparison groups or specifications.

A result that survives rigorous sensitivity analysis is more credible than one never tested.

### Address Clustering And Standard Errors Appropriately

Treatment is often assigned at a group level while outcomes are measured individually. Incorrect standard errors produce false precision.

Address by:

- clustering standard errors at the level of treatment assignment;
- using appropriate methods when the number of clusters is small;
- reflecting serial correlation in time series designs;
- avoiding over-rejection from misspecified standard errors.

Individual-level standard errors in a cluster-treated design overstate significance.

## Common Traps

### Defaulting To A Trendy Design

Applying difference-in-differences or regression discontinuity because they are fashionable, when their assumptions fail, produces biased estimates.

### Unexamined Identifying Assumptions

Parallel trends, continuity, or no-manipulation assumed without inspection is no foundation at all.

### Ignorable Manipulation Of Assignment

Sorting around a threshold or anticipation of treatment corrupts the as-good-as-random claim.

### Underpowered Near A Threshold Or In A Series

Too few observations near a cutoff or too few time points produce unstable, uninterpretable estimates.

### Local Effects Narrated As Average Effects

A threshold or treated-group effect overgeneralized to the whole population misleads.

### Unmeasured Confounding Unaddressed

Quasi-experiments cannot rule out all confounders; ignoring this overstates certainty.

### Wrong Level Of Clustering

Individual standard errors under cluster-level treatment manufacture false precision.

## Self-Check

- Is the use of a quasi-experiment justified over randomization on ethical, practical, or feasibility grounds?
- Does the chosen design match the actual assignment mechanism and source of variation?
- Is the core identifying assumption named and empirically examined, not merely asserted?
- Is manipulation or sorting around the threshold or selection checked and ruled out?
- Are pre-treatment parallel trends inspected for difference-in-differences and related designs?
- Are there enough observations for the design's requirements (near the threshold, in the series, in the donor pool)?
- Is the estimand defined, including its local nature and the population it generalizes to?
- Are sensitivity analyses planned for unmeasured confounding, with placebo and falsification tests?
- Are standard errors clustered at the level of treatment assignment and robust to few clusters?
- Would a skeptic find the assumptions examined and the result tested, or only asserted?
