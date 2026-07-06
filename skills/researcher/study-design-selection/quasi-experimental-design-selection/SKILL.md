---
name: quasi_experimental_design_selection.md
description: Use when the agent is selecting among quasi-experimental designs such as nonequivalent control group, interrupted time series, or regression discontinuity, deciding how to approximate causal inference when randomization is not feasible, or assessing the assumptions and threats each quasi-experimental option can and cannot address.
---

# Quasi-Experimental Design Selection

When randomization is not feasible or ethical but the research question is still causal, quasi-experimental designs are the bridge. They exploit natural variation, cutoffs, or time structure to approximate the balance that randomization would provide. The difficulty is that each quasi-experimental option addresses a different set of threats and rests on different assumptions, and agents often pick one by familiarity rather than by analyzing which design best neutralizes the confounding structure of the specific problem. The result is a causal claim propped up by a design whose assumptions the setting violates.

The harm this skill prevents is causal inference built on a design whose identifying assumptions do not hold, producing confident but wrong conclusions that drive policy or practice. A nonequivalent control group study with diverging baseline trends, an interrupted time series with confounded concurrent events, or a regression discontinuity design with manipulation around the cutoff can each yield estimates that look rigorous but are biased in unknown directions. The agent has freedom to choose among quasi-experimental options, but must justify the choice against the specific threats present and must state the assumptions on which the estimate rests.

## Core Rules

### Reserve Quasi-Experimental Designs For Causal Questions Without Randomization

Quasi-experimental designs exist to approximate causal inference when random assignment is impossible. They are not a generic upgrade for observational studies, and they are overkill for purely descriptive or associational questions. Use them when the claim is causal, randomization is infeasible or unethical, and a credible source of exogenous variation or comparison exists.

Confirm before selecting:

- the claim is causal, not merely descriptive or associational;
- randomization is genuinely infeasible, not merely inconvenient;
- a plausible comparison, cutoff, or time structure is available to exploit;
- the design can be implemented with the data that can actually be obtained.

If none of these hold, a quasi-experimental design cannot rescue the causal claim.

### Match The Design To The Structure Of The Treatment Assignment

Different quasi-experimental designs fit different ways the treatment is distributed. Selecting the wrong family for the assignment mechanism produces a design that cannot identify the effect.

Match structure to design:

- nonequivalent control group, when treated and untreated groups exist but were not randomized;
- interrupted time series, when a treatment is introduced to a whole unit at a known time and repeated observations exist before and after;
- regression discontinuity, when treatment assignment depends on a cutoff on a continuous score;
- difference-in-differences, when a treatment affects one group or time and a parallel comparison exists;
- synthetic control, when a single treated unit is compared to a weighted combination of untreated units.

The assignment mechanism dictates which designs are even candidates.

### Scrutinize The Parallel Trends Assumption In Difference-In-Designs

Many quasi-experimental designs, especially difference-in-differences and nonequivalent control group designs with pre-post data, rest on the assumption that treated and comparison units would have followed parallel paths absent treatment. If baseline trends differ, the estimate absorbs that divergence.

Assess:

- whether enough pre-period observations exist to observe trends;
- whether pre-treatment trends are actually parallel, not just assumed;
- whether any event coincides with treatment that differentially affects one group;
- whether composition of the groups changes over time.

Parallel trends is an assumption about an unobserved counterfactual and must be argued with evidence, not asserted.

### Guard The Cutoff In Regression Discontinuity

Regression discontinuity identifies the effect at the cutoff by comparing units just above and just below. Its validity collapses if the cutoff is not sharp, if units can manipulate their score to land on one side, or if other changes occur exactly at the cutoff.

Verify:

- whether assignment is sharp or fuzzy, and model accordingly;
- whether units can precisely sort around the cutoff, which violates local randomness;
- whether the running variable is measured before treatment, not manipulated after;
- whether the bandwidth and functional form are chosen without peeking at the result;
- whether the estimate is local to the cutoff and not generalized as an average effect.

A manipulated or fuzzy cutoff turns a credible design into an observational comparison.

### Rule Out Confounding Events In Interrupted Time Series

An interrupted time series attributes a change in level or slope to the intervention, but any contemporaneous event can produce the same pattern. The design's credibility depends on ruling out alternative explanations for the break.

Check:

- whether other interventions, policy changes, or shocks coincide with the time of treatment;
- whether measurement of the outcome changed at the same time;
- whether the series is long enough to distinguish the break from noise and seasonality;
- whether a comparison series without the treatment shows no comparable break.

A break in the series is evidence of change, not proof the treatment caused it, unless rivals are excluded.

### Assess Selection Bias In Nonequivalent Control Group Designs

When groups are not randomized, they may differ systematically before treatment. Nonequivalent control group designs rely on the assumption that, conditional on observed covariates or matching, the groups are comparable. This is only as good as the covariates and the matching.

Evaluate:

- whether the covariates capture the meaningful sources of selection into treatment;
- whether overlap in covariate distributions is sufficient for credible comparison;
- whether matching, weighting, or adjustment is appropriate to the selection mechanism;
- whether unmeasured confounders plausibly remain and in which direction.

Residual selection bias is the central threat and must be bounded, not waved away.

### Predefine The Outcome, Timing, And Analysis Plan

Quasi-experimental designs are vulnerable to choices that can move the result, including the window, the bandwidth, the functional form, and the set of covariates. Predefining these protects the inferential strength.

Lock in advance:

- the primary outcome and its measurement window;
- the pre- and post-periods or the bandwidth around a cutoff;
- the functional form and any covariate set, chosen without reference to the estimate;
- the handling of lags, leads, and seasonality;
- sensitivity analyses that vary the key choices.

A result that appears only under one favorable specification cannot carry a strong causal claim.

### State And Stress-Test The Identifying Assumptions

Every quasi-experimental estimate rests on assumptions that cannot be tested directly but can be probed indirectly. The credibility of the claim depends on how well these probes hold.

Probe:

- placebo tests on outcomes or groups that should not be affected;
- pre-treatment parallel trends and leads that should show no effect;
- sensitivity to bandwidth, window, functional form, and covariate choice;
- bounds on how strong unmeasured confounding would need to be to overturn the result.

The estimate is only as credible as the assumptions survive these tests.

## Common Traps

### Choosing The Design By Familiarity

Selecting difference-in-differences or regression discontinuity because it is familiar, regardless of the assignment structure, produces a design that cannot identify the effect in the given setting.

### Asserting Parallel Trends Without Evidence

Difference-in-differences rests on parallel counterfactual trends. Assuming rather than demonstrating them, especially with few pre-periods, hides the design's main vulnerability.

### Allowing Cutoff Manipulation In Regression Discontinuity

If units can sort around the cutoff, local randomness fails and the estimate reflects selection, not treatment. Manipulation testing is mandatory, not optional.

### Confounding The Interrupted Time Series Break

Attributing a level or slope change to the treatment while another event coincides is the classic failure of this design. A comparison series is the strongest defense.

### Treating Covariate Adjustment As Removing Selection Bias

Matching or adjusting on observed covariates handles only measured differences. Unmeasured selection persists and often dominates, so the claim must acknowledge it.

### Generalizing The Local Estimate

Regression discontinuity estimates the effect at the cutoff, not the average effect everywhere. Treating it as a population average effect overstates its scope.

### Fishing Across Specifications

Trying multiple bandwidths, windows, or covariate sets and reporting the favorable one inflates false positives and manufactures a result that will not replicate.

## Self-Check

- [ ] Is the claim causal, and is randomization genuinely infeasible, justifying a quasi-experimental rather than randomized or purely observational design?
- [ ] Does the selected design family match the actual structure of treatment assignment, rather than the agent's familiarity?
- [ ] For difference-in-differences or nonequivalent control designs, is the parallel trends or comparability assumption supported with pre-period evidence, not merely asserted?
- [ ] For regression discontinuity, has cutoff manipulation been tested, and is the running variable measured before treatment?
- [ ] For interrupted time series, have contemporaneous confounding events and measurement changes been ruled out, ideally with a comparison series?
- [ ] Are the primary outcome, timing, bandwidth, functional form, and covariate set predefined rather than chosen after seeing the estimate?
- [ ] Are the identifying assumptions stated explicitly and probed with placebo tests, leads, sensitivity analyses, or confounding bounds?
- [ ] Is the estimate reported with its correct scope, such as local-to-cutoff or treatment-on-the-treated, without overgeneralizing to an average population effect?
- [ ] Is the causal language calibrated to the strength of the assumptions, with no upgrade to a definitive causal claim the design cannot support?
- [ ] For high-stakes, contested, or assumption-sensitive cases, is expert causal-inference or methodological consultation sought before the design and claim are finalized?
