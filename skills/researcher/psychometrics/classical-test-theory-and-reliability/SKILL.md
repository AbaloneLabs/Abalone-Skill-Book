---
name: classical_test_theory_and_reliability.md
description: Use when the agent is estimating reliability, computing internal consistency or test-retest coefficients, interpreting standard error of measurement, applying the Spearman-Brown prophecy formula, or judging whether scores from a test are dependable enough for decisions.
---

# Classical Test Theory And Reliability

Classical test theory (CTT) frames every observed score as the sum of a true score and random error. The theory is simple; the judgment is not. Reliability is not a property of a test; it is a property of the scores a test produces in a particular population for a particular purpose. Agents frequently report a single reliability coefficient, treat it as a fixed attribute of the instrument, and conclude the scores are dependable. This misreads the theory. A coefficient computed on a homogeneous, high-ability sample will differ from one computed on a heterogeneous, mixed-ability sample, and a coefficient adequate for group-level description may be wholly inadequate for individual-level decisions. The discipline is to estimate, interpret, and report reliability in the context of the sample, the decision stakes, and the standard error of measurement.

Use this skill when estimating reliability, interpreting coefficients, or deciding whether scores support a given decision. The goal is to prevent the agent from presenting a reliability number as proof of dependability without considering sample dependence, the type of reliability estimated, and the precision needed for the intended use.

## Core Rules

### Treat Reliability As Sample-Dependent, Not Instrument-Fixed

A reliability coefficient describes the variance ratio in a specific sample. It is not transferable across populations.

Consider:

- the variability of the sample: restricted range shrinks reliability;
- the homogeneity of the group: homogeneous groups yield lower coefficients;
- the match between the norming sample and the current population;
- whether the reported coefficient comes from a comparable group.

Citing a published coefficient as evidence for a different population commits a transfer error. Re-estimate or justify comparability.

### Match The Reliability Type To The Source Of Error

Different reliability coefficients address different sources of measurement error. Selecting the wrong type produces a misleading estimate.

Match by:

- internal consistency for error from item sampling within one administration;
- test-retest for error from temporal instability;
- alternate or parallel forms for error from form differences;
- inter-rater reliability for error from scorer judgment.

Reporting Cronbach's alpha when the concern is temporal stability, or test-retest when the concern is internal coherence, answers the wrong question.

### Interpret Coefficients Against Decision Stakes, Not Cutoffs

Rules of thumb (e.g., alpha above .70) are screening heuristics, not adequacy standards. Adequacy depends on use.

Interpret by:

- group-level description tolerating lower reliability;
- individual clinical or high-stakes decisions requiring higher reliability;
- the cost of misclassification at the decision threshold;
- the standard error of measurement at the cutting score.

A coefficient of .80 may be ample for research means and dangerous for a diagnostic cutoff.

### Report The Standard Error Of Measurement

Reliability coefficients are abstract; the standard error of measurement (SEM) is concrete. SEM tells the user how much an individual score may wobble.

Report:

- the SEM in raw score units;
- confidence intervals around individual scores;
- the conditional SEM if precision varies across the score range;
- the practical width of the band relative to decision cutoffs.

A score of 30 with an SEM of 4 means the observed score could plausibly reflect a true score anywhere in a band around 30. Decisions near a cutoff made without this band are overconfident.

### Recognize The Limits Of Internal Consistency

Cronbach's alpha is the most reported reliability coefficient and the most over-interpreted. It is sensitive to test length and dimensionality.

Recognize:

- alpha rises with the number of items, inflating apparent reliability of long tests;
- high alpha does not imply unidimensionality;
- alpha can be high even when items measure different constructs;
- multidimensional scales require subscale-level coefficients.

A long, multidimensional test can yield alpha above .90 while measuring several things at once. Alpha reports average inter-item covariance, not coherence.

### Use The Spearman-Brown Formula When Length Changes

If a test is shortened or lengthened, reliability changes predictably. The Spearman-Brown prophecy formula estimates the new reliability.

Apply when:

- shortening a long test to reduce respondent burden;
- lengthening a brief test to raise reliability;
- comparing reliability across tests of different lengths;
- planning the number of items needed for a target reliability.

Assuming reliability is unchanged after dropping half the items overstates dependability.

### Distinguish Reliability From Validity And From Stability

Reliability, validity, and stability are related but distinct. Conflating them misleads.

Distinguish:

- reliability: consistency of scores;
- validity: correctness of interpretation;
- stability: consistency over time, which may reflect true change, not error.

A test can be perfectly reliable and measure the wrong thing, and a stable construct can show low test-retest if real change occurs between administrations.

### Account For Rater Error In Scored Responses

When humans score performance, the rater is part of the measurement instrument. Rater error contributes to unreliability.

Account for:

- inter-rater agreement and consistency statistics;
- rater training and calibration effects;
- rater severity or leniency drift;
- nested designs where raters score subsets of examinees.

Reporting only internal consistency for a rater-scored task ignores a major error source.

## Common Traps

### Citing Published Reliability As If Fixed

A coefficient from a manual applies to the manual's sample. Treating it as the instrument's reliability ignores sample dependence and misleads users about precision.

### Treating Alpha Above .70 As Adequate

The .70 cutoff is a research-screening heuristic. Applying it to high-stakes individual decisions tolerates error that can misclassify examinees near cutoffs.

### Reporting Coefficients Without SEM

A reliability coefficient without the standard error of measurement hides the score-level imprecision that actually matters for decisions.

### Confusing High Alpha With Unidimensionality

A long test of loosely related items can produce high alpha. Inferring coherence from alpha alone masks multidimensionality.

### Ignoring Restricted Range Effects

Computing reliability on a selected, homogeneous sample shrinks the coefficient and understates the test's precision in the broader population.

### Mismatching Reliability Type To Error Source

Using internal consistency to argue temporal stability, or test-retest to argue internal coherence, answers a question different from the one being asked.

### Overlooking Rater Error In Performance Assessments

For rater-scored tasks, internal consistency alone ignores rater severity, leniency, and drift, all of which inflate error.

## Self-Check

- Is reliability reported as sample-dependent, with the current sample described and compared to the norming group?
- Does the reliability type (internal consistency, test-retest, alternate forms, inter-rater) match the source of error under concern?
- Are coefficients interpreted against decision stakes rather than a generic cutoff?
- Is the standard error of measurement reported, with confidence intervals around individual scores?
- Is the effect of test length on alpha acknowledged, with Spearman-Brown used when length changes?
- Are multidimensional scales reported at the subscale level rather than with a single overall coefficient?
- Is reliability distinguished from validity and from temporal stability?
- For rater-scored tasks, is inter-rater reliability assessed alongside internal consistency?
- Is conditional precision considered if error varies across the score range?
- Are reliability claims limited to the population and purpose for which evidence exists?
