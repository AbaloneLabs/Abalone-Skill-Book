---
name: hypothesis_formulation_and_testing.md
description: Use when the agent is writing hypotheses, deciding between null and alternative hypotheses, choosing one-tailed versus two-tailed tests, mapping hypotheses to study designs, planning confirmatory versus exploratory tests, or interpreting whether evidence supports, rejects, or fails to decide a claim.
---

# Hypothesis Formulation And Testing

A hypothesis is a commitment made before evidence is examined. Many researchers treat it as a decorative sentence near the end of an introduction, but its real function is inferential: it fixes the claim, the contrast, the direction, the test, and the decision rule that the study must honor. Once data are collected, a poorly formed hypothesis cannot be repaired by a clever statistic. The damage shows up as inflated error rates, post hoc reframing, and conclusions that the design never licensed.

Use this skill before locking a hypothesis into a protocol, preregistration, proposal, or analysis plan, and again when interpreting results against that plan. The goal is to keep the agent from writing hypotheses that are unfalsifiable, untestable with the chosen design, or quietly mutated after results appear. The agent has freedom to choose frameworks, but the logical chain from construct to testable statement to decision rule must be explicit and internally consistent.

## Core Rules

### State The Hypothesis Before Looking At The Data

A confirmatory hypothesis is a pre-commitment. Its value comes from the fact that it constrained the design, the measures, the sample, and the analysis before any result was observed.

Before data collection or before unblinding, fix:

- the exact construct and population;
- the comparison or contrast;
- the predicted direction if direction is claimed;
- the primary outcome and its scale;
- the test and the decision threshold;
- the role of covariates and subgroups;
- which analyses are exploratory.

If the hypothesis is written after inspecting the data, it is no longer a test of that hypothesis. Label it exploratory and do not present p-values as confirmatory.

### Make The Hypothesis Falsifiable

A useful hypothesis can be wrong. If every possible outcome is compatible with the hypothesis, it explains nothing and tests nothing.

Check that the hypothesis:

- predicts a specific pattern, difference, relationship, or absence;
- has at least one imaginable observation that would count against it;
- is not so vague that any result can be reinterpreted as support;
- is not tautological or true by definition;
- distinguishes the proposed mechanism from plausible rivals.

A hypothesis that "improves outcomes" without naming the outcome, comparator, or direction is a topic, not a hypothesis.

### Align Hypothesis, Design, And Test

The hypothesis dictates what design can test it. A causal hypothesis needs a design that supports causal inference. A hypothesis about distribution needs measurement of that distribution. A hypothesis about mediation needs temporal ordering and the right measures.

Verify the chain:

- claim type implied by the hypothesis;
- design that can produce that claim type;
- variables the design actually observes;
- test whose assumptions the data meet;
- inference the test licenses;
- language used to report it.

A mismatch anywhere in this chain invalidates the conclusion. A correlation test cannot adjudicate a causal hypothesis; a cross-sectional design cannot adjudicate a temporal-ordering hypothesis.

### Choose Null And Alternative Deliberately

The null and alternative are not formalities. They encode what counts as the reference state and what kind of departure would matter.

Decide:

- whether the null is "no effect," "no difference," "equivalence," or "non-inferiority";
- whether the alternative is two-sided or one-sided;
- whether a minimal clinically or practically important effect should anchor the test;
- whether the test is for superiority, equivalence, or non-inferiority;
- how the choice interacts with ethics, stakes, and prior evidence.

A one-sided test is legitimate when only one direction is meaningful and was specified in advance; it is not a device for halving the p-value of an already-observed result.

### Predefine Direction And Avoid Quiet Switching

Directional hypotheses carry responsibility. If the hypothesis predicts an increase, a large decrease is evidence against the hypothesis, not evidence for a different version of it.

Lock in:

- the predicted sign of the effect;
- the outcome variable the sign applies to;
- the comparator;
- whether opposite-direction findings will be reported and how.

Switching from two-sided to one-sided, or reversing the predicted direction after seeing the sign of the estimate, doubles the nominal error rate and is a form of result-driven inference.

### Distinguish Confirmatory, Exploratory, And Post Hoc Tests

Not every test in a paper carries the same evidential weight, and readers deserve to know which is which.

Classify each test:

- confirmatory, planned before data and tied to the primary hypothesis;
- exploratory, planned to search for patterns without a strict commitment;
- post hoc, conceived after seeing the data;
- sensitivity, used to check robustness of the primary result.

Report each class honestly. Confirmatory tests license stronger claims; exploratory and post hoc tests generate hypotheses for future work. Multiplicity adjustments differ across these classes.

### Plan For Multiplicity And Error Control

Every additional test inflates the chance of a false positive somewhere in the analysis. This is not optional bookkeeping; it determines whether a "significant" result means anything.

Address:

- number of primary and secondary outcomes;
- number of groups, time points, and subgroups;
- interim analyses and stopping rules;
- covariates and model selections tried;
- correction method, such as Bonferroni, Holm, or false discovery rate control;
- family over which correction is applied;
- whether correction is required for exploratory analyses.

A family of forty tests with no correction will produce false positives almost guaranteed. Plan the family before running it.

### Define The Decision Rule, Not Just The Threshold

A p-value is not a verdict. The decision rule says what the team will conclude given the evidence, and it should be set in advance.

Specify:

- primary outcome and test;
- significance threshold and why;
- effect size required for practical importance;
- confidence interval interpretation;
- handling of multiplicity;
- handling of missing data and deviations;
- what a null result will mean;
- what an inconclusive result will mean.

Without a decision rule, the team will rationalize whatever number appears. With one, the team can report results honestly even when they are disappointing.

## Common Traps

### Writing The Hypothesis After The Result

Drafting a hypothesis that matches the observed finding turns a confirmatory study into a story. It destroys error control and inflates the published record with chance patterns.

### Using One-Sided Tests To Reach Significance

Choosing a one-sided test because the two-sided p-value was 0.07 is result-driven inference. One-sided tests must be justified by design and intent, not by the data.

### Treating Failure To Reject As Proof Of No Effect

A non-significant p-value is compatible with no effect, with a small effect, or with an underpowered study. It is not evidence of absence unless the study was designed for equivalence.

### Hypothesizing After Results Are Known

Reframing an unexpected finding as "what we expected all along" is HARKing. It makes the literature look more coherent than the evidence warrants.

### Confusing Statistical Significance With Practical Importance

A tiny effect can be statistically significant in a huge sample, and a large effect can be non-significant in a small one. Report effect sizes and confidence intervals, not just p-values.

### Ignoring Multiplicity Across Outcomes And Subgroups

Running many tests and reporting only the significant ones manufactures false positives. The family of tests must be declared and corrected.

### Mutating The Primary Outcome

Switching the primary outcome to whichever looked best, often rationalized as a "better measure," is a form of result selection. The primary outcome is fixed in the protocol.

## Self-Check

- [ ] Is each hypothesis stated before data collection or unblinding, with date-stamped evidence?
- [ ] Is every hypothesis falsifiable, with at least one imaginable observation that would count against it?
- [ ] Does the chosen design actually support the claim type the hypothesis implies?
- [ ] Are null and alternative hypotheses, including direction and equivalence framing, specified in advance?
- [ ] Is the predicted direction locked, with no switching after seeing the sign of the effect?
- [ ] Are confirmatory, exploratory, post hoc, and sensitivity tests labeled distinctly?
- [ ] Is multiplicity addressed with a declared family and a planned correction?
- [ ] Is the decision rule defined, including thresholds, effect sizes, and meaning of null or inconclusive results?
- [ ] Are effect sizes and confidence intervals reported alongside any significance test?
- [ ] Are deviations from the planned hypothesis reported transparently rather than hidden?
