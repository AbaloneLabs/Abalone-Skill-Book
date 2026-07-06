---
name: difference_in_differences_and_panel_designs.md
description: Use when the agent is applying difference-in-differences or panel data designs, checking parallel trends, choosing control groups, handling staggered treatment timing, dealing with two-way fixed effects pitfalls, or interpreting dynamic treatment effects over time.
---

# Difference In Differences And Panel Designs

Difference-in-differences is one of the most widely used quasi-experimental tools, and also one of the most misapplied. Its appeal is simplicity: compare the change in a treated group to the change in a comparison group, and attribute the difference to the intervention. But this rests on the parallel-trends assumption, which is strong, often untestable, and frequently violated. More recently, it has become clear that the standard two-way fixed effects estimator behaves badly under staggered treatment timing and heterogeneous effects, producing estimates that can be wrong in sign. Agents who run a difference-in-differences model without checking trends, without considering timing, and with the default estimator are producing causal claims on an unstable foundation.

Use this skill when applying difference-in-differences or panel designs, selecting comparison groups, or handling staggered timing. The goal is to prevent the agent from producing a clean two-way fixed effects result that hides assumption failures and estimator problems.

## Core Rules

### Make The Parallel-Trends Assumption Central And Inspect It

Difference-in-differences identifies the causal effect only if, absent treatment, treated and comparison groups would have followed parallel trends. This is the load-bearing assumption and must be examined.

Inspect by:

- plotting outcomes for treated and comparison groups over the pre-period;
- testing for differential pre-trends, while acknowledging low power;
- considering whether any factor other than treatment could differentially trend;
- using lead indicators to check for anticipation effects.

Parallel trends cannot be proven; it can only be scrutinized and made plausible or implausible.

### Choose A Credible Comparison Group Deliberately

The comparison group is the counterfactual. A poorly chosen group invalidates the design regardless of the estimator.

Choose by:

- similarity to the treated group in relevant characteristics and pre-trends;
- absence of spillovers from the treatment;
- stability of composition over the period;
- not being affected by other concurrent changes.

A convenient comparison group with divergent pre-trends is not a counterfactual.

### Handle Staggered Treatment Timing Explicitly

When units adopt treatment at different times, the standard two-way fixed effects estimator can combine already-treated with not-yet-treated units in misleading ways, especially with heterogeneous effects.

Address by:

- recognizing the two-way fixed effects pitfalls under staggered timing;
- using estimators designed for staggered adoption and heterogeneous effects;
- constructing the correct comparison for each cohort and timing;
- being transparent about which units serve as controls at each point.

Default two-way fixed effects under staggeration can produce negative weights and wrong-signed estimates.

### Examine Dynamic And Event-Study Effects

A single average treatment effect hides how the effect evolves over time. Event-study specifications reveal the dynamics and test the design.

Examine by:

- estimating leads and lags of treatment to trace the effect over time;
- checking that pre-treatment leads are near zero, supporting parallel trends;
- observing when the effect appears and how it grows or fades;
- distinguishing immediate from delayed effects.

Significant pre-treatment leads suggest either anticipation or a failed parallel-trends assumption.

### Guard Against Composition Changes

If the composition of treated or comparison groups changes over time, trends may reflect who is in the group rather than the intervention.

Guard by:

- using balanced panels where feasible;
- examining entry and exit of units over time;
- weighting or restricting to consistent units;
- checking whether results hold under a stable sample.

Composition-driven trends confound the treatment effect.

### Account For Clustering And Serial Correlation

Panel data have correlated observations over time and within groups. Standard errors that ignore this overstate precision.

Account by:

- clustering standard errors at the level of treatment assignment, not the individual observation;
- addressing serial correlation in long panels;
- using appropriate methods when the number of clusters is small;
- recognizing that too few clusters cause over-rejection.

Difference-in-differences with unclustered standard errors is a classic source of false positives.

### Consider Whether Treatment Effects Are Heterogeneous

If effects differ across units or over time, the average from a standard estimator may not represent any meaningful causal quantity.

Consider:

- whether effects plausibly differ by unit characteristics or treatment timing;
- using estimators that handle heterogeneity rather than imposing constancy;
- reporting group-specific or time-specific effects where informative;
- avoiding the assumption that one average captures the policy-relevant effect.

An average of positive and negative effects can be near zero and mask both.

### Test The Design With Placebo And Falsification Checks

A credible difference-in-differences survives tests on outcomes and groups where no effect should appear.

Test by:

- placebo outcomes that should not be affected by the treatment;
- placebo treatment dates before the actual intervention;
- groups that should not be affected;
- alternative comparison groups.

A result that appears only for the true treatment, true date, and affected groups is more credible.

### Be Transparent About External Validity

Difference-in-differences estimates the effect on the treated in a specific context. Generalization is a separate claim.

Clarify:

- the estimate applies to the treated units and period studied;
- whether the context generalizes to other settings or times;
- whether the effect depends on the specific dose or implementation;
- the limits of extrapolation.

A policy effect in one state or year does not automatically transfer to others.

## Common Traps

### Parallel Trends Assumed Not Inspected

The central assumption treated as given, without plotting or testing, undermines the whole estimate.

### Default Two-Way Fixed Effects Under Staggeration

The standard estimator can combine groups incoherently under staggered timing and heterogeneous effects.

### Inadequate Comparison Group

A convenient control with divergent pre-trends is not a counterfactual.

### Unclustered Or Wrong-Level Standard Errors

Ignoring clustering or serial correlation manufactures false precision.

### Ignored Pre-Treatment Leads

Significant leads suggest anticipation or failed parallel trends but are often overlooked.

### Composition Changes Confounding Trends

Shifting group membership produces trends unrelated to the intervention.

### Local Effect Overgeneralized

A treated-context effect narrated as a universal policy impact overclaims.

## Self-Check

- Is the parallel-trends assumption central, inspected via plots and pre-trend tests, and made plausible?
- Is the comparison group chosen for credibility, similarity, and absence of spillovers?
- Is staggered treatment timing handled with appropriate estimators, not default two-way fixed effects?
- Are dynamic effects examined with event-study leads and lags, and are pre-leads near zero?
- Are composition changes guarded against with balanced panels or stable samples?
- Are standard errors clustered at the treatment-assignment level and robust to serial correlation?
- Is treatment-effect heterogeneity considered, with estimators that handle it?
- Are placebo outcomes, dates, and groups used to falsify the design?
- Is the external validity of the treated-context estimate acknowledged as a separate claim?
- Would a methodological skeptic find the assumptions examined and the estimator appropriate?
