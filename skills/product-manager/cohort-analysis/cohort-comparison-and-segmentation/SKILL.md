---
name: cohort_comparison_and_segmentation.md
description: Use when the agent is comparing cohorts over time, segmenting cohorts by behavior or attribute, reading differences between cohorts, or guarding against over-segmentation and confounded cohort comparisons.
---

# Cohort Comparison And Segmentation

Comparing cohorts is where product analysis is most persuasive and most often wrong. A difference between two cohorts looks like evidence, but a difference can reflect a real behavioral change, a change in who the cohorts contain, a change in the environment, or pure noise from small samples. Segmentation, the natural next step, multiplies the comparisons and the risk: every new dimension splits the data into smaller cells, each more volatile than the last, until the analysis is a grid of dramatic numbers built on handfuls of users. The tools make comparison and segmentation effortless, which is exactly why the judgment around them is so often skipped.

The harm this skill prevents is reading meaning into differences that are not real, or missing confounders that explain the differences entirely. A team compares two cohorts, sees that one retains better, and credits a product change, when the cohorts actually differ in acquisition channel or device mix. Another team segments by five dimensions at once, finds a cell with a spectacular rate, and builds a strategy around eleven users. In each case the comparison felt rigorous because it was quantitative, but the difference was confounded or unstable, and the decision rested on an artifact.

Use this skill when comparing cohorts, segmenting within cohorts, reading differences between groups, or deciding how far to segment. The work is to ensure comparisons are valid, differences are not confounded, and segmentation stops before the cells become noise.

## Core Rules

### Confirm Cohorts Are Comparable Before Reading Differences

A difference between cohorts is meaningful only if the cohorts are comparable, meaning they differ only in the dimension you are studying. If one cohort came from a high-intent channel and the other from a low-intent channel, a retention difference may reflect the channel, not the product change you intended to study. Comparability is the precondition for interpretation, and it must be checked before the difference is read.

Before interpreting a difference, confirm the cohorts are similar in composition: acquisition source, device, geography, plan tier, and prior behavior. Where they differ, either control for the difference through segmentation or acknowledge that the comparison is confounded. A difference between non-comparable cohorts is a starting point, not a conclusion.

### Control For Confounders Through Targeted Segmentation

Confounders are the variables that differ between cohorts and also affect the outcome, and they are the main reason cohort comparisons mislead. Acquisition channel quality, device, geography, and plan tier all affect retention and conversion independently of any product change. If these differ between cohorts, the observed difference may be entirely due to the confounder.

Identify the plausible confounders for the outcome and segment the comparison by each, holding the confounder constant. Compare retention within a single acquisition channel, within a single device, within a single plan tier. If the cohort difference persists within each segment, it is more likely real; if it appears only in the aggregate, it was driven by composition.

### Segment By Dimensions That Plausibly Change Behavior

Segmentation is valuable only when the dimension could plausibly change the outcome. Segmenting by acquisition source, device, geography, plan tier, and first action is useful because these dimensions reflect real differences in user intent, capability, and experience. Segmenting by arbitrary attributes that have no behavioral meaning produces cells that differ by chance.

Choose segmentation dimensions based on a hypothesis about why behavior would differ. For each dimension, state the reason it might affect the outcome. If you cannot articulate a mechanism, the segmentation is exploratory and its results should be treated as hypotheses, not findings.

### Stop Segmenting Before Cells Become Noise

Every additional segmentation dimension splits each cohort into smaller cells, and small cells produce unstable, dramatic rates. A grid segmented by channel, device, geography, and plan tier can produce cells with a handful of users, each showing an extreme rate that is pure variance. Over-segmentation manufactures false patterns from noise.

Set a minimum cell size and suppress or flag any segment below it. Prefer a few well-chosen segmentation dimensions over many, and when a cell is small, report the sample size explicitly rather than charting the rate as if it were stable. The goal is interpretable comparison, not exhaustive subdivision.

### Distinguish Real Differences From Sampling Variance

A difference between cohorts or segments may be real, or it may be the normal variance of drawing different samples. Small cohorts and small segments are especially prone to large-looking differences that shrink or reverse with more data. Reading every difference as meaningful leads to chasing ghosts.

Assess whether a difference is larger than the expected variance for the sample sizes involved. Where possible, use confidence intervals or repeat the comparison across multiple time windows to see if the difference is stable. A difference that appears in one period and vanishes in the next is likely noise; a difference that persists is more likely real.

### Compare Differences Within Segments Before Summarizing

An aggregate difference between cohorts can hide that the difference runs in opposite directions across segments. Cohort A may retain better than cohort B overall, but worse within mobile users and better within desktop users, because the cohort mix shifted toward desktop. Summarizing only the aggregate conceals this structure and can lead to conclusions that are false for important segments.

After computing the aggregate difference, decompose it by the key segments and check whether the direction is consistent. If segments disagree, report the segment-level differences rather than a single summary, because the aggregate is an average of opposing effects and does not describe any actual group.

### Treat Observational Comparisons As Hypotheses, Not Causes

Cohort comparisons are observational, meaning users were not randomly assigned to cohorts, so differences are confounded by self-selection and pre-existing differences. A cohort that used a feature retains better, but the users who chose the feature may have been more motivated to begin with. The comparison generates a hypothesis about the feature; it does not prove the feature caused the retention.

Present cohort differences as correlations and use them to design experiments that isolate the causal effect. Where an experiment is not possible, use the strongest quasi-experimental method and state its assumptions. Acting on an observational difference as if it were causal leads to building for a lever that does not move the outcome.

## Common Traps

### Comparing Non-Comparable Cohorts

Cohorts that differ in composition, channel, device, or plan cannot be cleanly compared, because the difference may reflect composition rather than behavior. Confirm comparability or control for confounders before reading a difference.

### Reading A Confounded Difference As A Product Effect

When cohorts differ in a variable that affects the outcome, the observed difference may be entirely due to the confounder. Segment to hold confounders constant before attributing the difference to the product.

### Over-Segmenting Into Tiny Cells

Each segmentation dimension splits the data further, and small cells produce extreme, unstable rates. Set a minimum cell size and stop segmenting before the cells become noise.

### Treating Sampling Variance As A Real Difference

Small samples produce large-looking differences by chance, and reading every difference as meaningful leads to chasing ghosts. Check whether a difference exceeds expected variance and persists across periods.

### Summarizing An Aggregate That Hides Opposing Segments

An aggregate difference can average opposing segment-level effects, so the summary describes no actual group. Decompose by key segments and report the structure, not just the average.

### Inferring Causation From An Observational Comparison

Cohort comparisons are correlational because users self-select into cohorts. Present differences as hypotheses and test causation with experiments rather than acting as if the comparison proved the effect.

## Self-Check

- [ ] Cohorts are confirmed comparable in composition, channel, device, geography, and plan, before any difference is interpreted.
- [ ] Plausible confounders are identified and controlled through targeted segmentation that holds them constant.
- [ ] Segmentation dimensions are chosen because they plausibly affect behavior, with a stated mechanism for each.
- [ ] A minimum cell size is set, and segments below it are flagged or suppressed rather than charted as stable.
- [ ] Differences are checked against expected sampling variance and confirmed to persist across periods before being treated as real.
- [ ] Aggregate differences are decomposed by key segments to check for opposing directions before being summarized.
- [ ] Observational cohort differences are presented as correlational hypotheses, not as proven causal effects.
- [ ] No comparison is acted on as causal without an experiment or the strongest quasi-experimental alternative.
- [ ] Small-sample segments are reported with explicit sample sizes so dramatic rates are not mistaken for stable findings.
- [ ] The readout states comparability, confounders, segmentation dimensions, and cell sizes so the comparison is interpretable and auditable.
