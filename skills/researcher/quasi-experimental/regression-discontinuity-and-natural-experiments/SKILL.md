---
name: regression_discontinuity_and_natural_experiments.md
description: Use when the agent is applying regression discontinuity, instrumental variables, or natural experiment designs, checking continuity and manipulation at a cutoff, choosing bandwidth and polynomial, validating instruments, or interpreting local average treatment effects and their limits.
---

# Regression Discontinuity And Natural Experiments

Regression discontinuity and natural experiments are among the most credible non-randomized designs, but their credibility is conditional. Regression discontinuity works only if the running variable cannot be manipulated at the threshold and potential outcomes are continuous through it. Instrumental variables work only if the instrument is relevant, exogenous, and affects outcomes solely through the treatment. Natural experiments are only as good as the claim that exposure was as-good-as-random. Agents often apply these designs because they carry the prestige of credible causal inference, then skip the validation steps that earn that credibility. A regression discontinuity without a manipulation check, or an instrumental variable without exogeneity reasoning, is an observational study in a rigorous costume.

Use this skill when applying regression discontinuity, instrumental variables, or natural experiment designs. The goal is to prevent the agent from presenting these designs as credible without performing the validation their credibility requires.

## Core Rules

### Verify The Assignment Rule And Its Integrity

Regression discontinuity depends on treatment being determined by a threshold on a continuous running variable. The integrity of that rule is the foundation.

Verify by:

- confirming treatment was actually assigned by the threshold, with few exceptions;
- checking that the threshold was not chosen post hoc to produce a result;
- examining whether the rule was known and whether exceptions occurred;
- documenting any fuzziness around the cutoff.

A fuzzy or manipulated assignment rule weakens or breaks the design.

### Test For Manipulation Of The Running Variable

If units can influence their score relative to the threshold, sorting occurs and the as-good-as-random claim at the cutoff fails.

Test by:

- examining the density of the running variable for a discontinuity at the threshold;
- checking for bunching just above or below the cutoff;
- considering whether units had knowledge and ability to sort;
- investigating any density jump's cause.

A density discontinuity at the threshold is direct evidence of manipulation.

### Establish Continuity Of Potential Outcomes

Regression discontinuity assumes that units just above and below the threshold are comparable except for treatment. Observable characteristics should be continuous through the cutoff.

Establish by:

- testing for discontinuities in covariates at the threshold;
- checking that baseline characteristics do not jump;
- examining whether other outcomes that should not be affected are continuous;
- being alert to discontinuities driven by the running variable's construction.

A jump in covariates suggests the groups differ in ways other than treatment.

### Choose Bandwidth And Functional Form Carefully

The estimate near the threshold is sensitive to how wide a band is used and what functional form fits the relationship on each side.

Choose by:

- data-driven bandwidth selection with documented method;
- local linear or low-order polynomials, avoiding high-order overfitting;
- showing robustness to alternative bandwidths and specifications;
- not selecting the bandwidth that gives the most favorable result.

An estimate that flips sign or significance across reasonable bandwidths is not robust.

### Validate The Instrument For Instrumental Variables

Instrumental variables estimate a causal effect only if the instrument meets three conditions, two of which are testable and one of which must be argued.

Validate by:

- relevance: the instrument strongly affects the treatment (test via first stage);
- exogeneity: the instrument is unrelated to unmeasured confounders (argue substantively);
- exclusion restriction: the instrument affects the outcome only through the treatment (argue substantively);
- justifying that the exclusion restriction is plausible in this setting.

A weak or non-exogenous instrument produces biased, often wrong-signed estimates.

### Interpret The Local Average Treatment Effect Correctly

Instrumental variables estimate the effect on compliers, those whose treatment status changes with the instrument. This is a local effect, not the average effect.

Interpret by:

- recognizing the estimand is the complier average effect;
- noting that effects on always-takers and never-takers are not identified;
- assessing who compliers are and whether they represent the policy-relevant group;
- avoiding narration as the average treatment effect.

A local average effect on compliers may differ substantially from the effect on the whole population.

### Argue The As-Good-As-Random Claim For Natural Experiments

Natural experiments rest on the claim that exposure to the intervention was determined as if randomly. This must be argued and tested, not assumed.

Argue by:

- showing balance on observable characteristics across exposure;
- examining whether exposure could be influenced by factors related to the outcome;
- testing for selection into exposure where data allow;
- acknowledging limits to the as-good-as-random claim.

A natural experiment without a tested as-good-as-random claim is an observational study.

### Assess The Local And Context-Bound Nature Of Estimates

Both regression discontinuity and instrumental variables produce local estimates tied to a specific context, threshold, or complier population.

Assess:

- the population and setting the estimate applies to;
- whether the local effect generalizes to other thresholds or contexts;
- whether the policy-relevant question matches the local estimand;
- the limits of extrapolation.

An effect at a threshold or among compliers should not be narrated as a universal effect.

### Conduct Sensitivity And Falsification Tests

Credible designs are tested against alternative explanations. Robustness checks distinguish a credible estimate from a fragile one.

Conduct:

- placebo thresholds away from the true cutoff;
- outcomes that should not be affected by the treatment;
- alternative bandwidths, polynomials, or specifications;
- sensitivity to violations of the identifying assumptions.

A result that survives falsification tests is far more credible than one never tested.

## Common Traps

### Manipulable Running Variable

Sorting around a threshold breaks the as-good-as-random claim at the cutoff.

### Discontinuous Covariates

A jump in baseline characteristics suggests the groups differ beyond treatment.

### Bandwidth Chosen For Favorable Result

Selecting the bandwidth that gives significance is a form of specification search.

### Weak Or Non-Exogenous Instrument

A weak first stage or implausible exclusion restriction produces biased estimates.

### Local Effect Narrated As Average

Complier or threshold effects overgeneralized mislead about the population effect.

### Unargued As-Good-As-Random Claim

A natural experiment without tested balance is just an observational study.

### Missing Falsification Tests

A credible design that is never tested against placebos is asserted, not demonstrated.

## Self-Check

- Is the assignment rule verified, with exceptions and fuzziness documented?
- Is the running variable tested for manipulation via density discontinuity at the threshold?
- Are covariates and unaffected outcomes shown to be continuous through the cutoff?
- Are bandwidth and functional form chosen by documented methods and shown robust to alternatives?
- For instrumental variables, is relevance tested, and are exogeneity and the exclusion restriction argued?
- Is the local average treatment effect interpreted as complier-specific, not the population average?
- For natural experiments, is the as-good-as-random claim argued and tested via balance?
- Is the local and context-bound nature of the estimate acknowledged in interpretation?
- Are placebo thresholds, unaffected outcomes, and alternative specifications used to falsify the design?
- Would a skeptic find the identifying assumptions examined and the estimate tested, not merely asserted?
