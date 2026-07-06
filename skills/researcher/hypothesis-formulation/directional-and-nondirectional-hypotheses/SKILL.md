---
name: directional_and_nondirectional_hypotheses.md
description: Use when the agent is choosing between a directional and a nondirectional hypothesis, deciding whether to predict the sign of an effect, selecting a one-tailed versus two-tailed test, justifying a directional prediction from theory, or avoiding post-hoc switches in sidedness that inflate the false-positive rate.
---

# Directional And Nondirectional Hypotheses

Whether a hypothesis predicts the direction of an effect is one of the most consequential and most abused decisions in study design. A directional hypothesis (and its one-tailed test) commits in advance to a specific sign of the effect, while a nondirectional hypothesis (and its two-tailed test) allows an effect in either direction. The judgment problem is that agents often treat this choice as a free optimization, picking whichever form yields significance, rather than as a substantive theoretical commitment that changes the meaning of the test, the critical region, and the error rate. The blind spots are well documented. Choosing one-tailed after glimpsing the data roughly doubles the effective alpha in the observed direction. Choosing one-tailed when an effect in the opposite direction would be scientifically interesting hides a real finding behind a test that cannot detect it. And conflating a directional research prediction with the statistical form of the test leads to conclusions the analysis never supported. The harm is reproducibility failure: published "significant" effects that do not replicate because the sidedness was chosen to fit the sample, and missed discoveries because a two-tailed question was tested with a one-tailed lens.

This skill covers the disciplined choice and justification of directional versus nondirectional hypotheses and the corresponding one-tailed and two-tailed tests. It is methodological guidance for matching the form of the hypothesis to the theory and the question; it is not a substitute for statistical consultation, preregistration, or a registered analysis plan. Wrongly chosen sidedness can produce misleading conclusions, so for high-stakes or confirmatory designs, consult a methodologist or statistician before locking the form.

## Core Rules

### Let The Research Question And Theory Dictate Directionality

The first decision is whether existing theory, prior evidence, or practical interest supports a specific direction. If a credible body of theory or replicated evidence predicts that the effect can only plausibly go one way, a directional hypothesis is justified. If an effect in either direction would be meaningful, surprising, or actionable, a nondirectional hypothesis is the honest choice. Do not begin from the desired p-value; begin from what the field already knows and what the study is trying to learn. The directionality is a claim about the state of knowledge, and it should be defensible to a skeptic.

### Match The Statistical Test To The Hypothesis Form

Directionality of the hypothesis and the sidedness of the test must correspond exactly. A directional hypothesis (greater than, or less than) pairs with a one-tailed test that places the entire critical region in the predicted tail. A nondirectional hypothesis (not equal) pairs with a two-tailed test that splits alpha across both tails. Mismatching them, such as stating a nondirectional hypothesis but running a one-tailed test, produces a result whose reported alpha does not match the question asked. Confirm the form of the hypothesis and the form of the test agree before computing anything.

### Justify A Directional Prediction With Theory, Not With The Data

A one-tailed test concentrates alpha in one tail, making it easier to reach significance in that direction but unable to detect an effect in the opposite direction. This concentration is defensible only when an opposite-direction effect is either impossible by mechanism or of no practical interest, and that justification must come from theory or prior evidence established before this study. If the only basis for the directional prediction is the current sample, the prediction is not a hypothesis but a description, and the one-tailed test is invalid. Write the theoretical justification down so it can be audited.

### Fix Sidedness Before Data Analysis And Document It

The credibility of a directional or nondirectional claim depends on the choice being made a priori. Record the hypothesis form, the test sidedness, and the justification in a protocol or preregistration before data are collected or inspected. This protects against the most common abuse, which is observing the direction of the effect and then choosing the sidedness that minimizes the p-value. If the choice was not pre-specified, the conservative and defensible default is the two-tailed test.

### Understand The Tradeoff In Power And Detectability

Directional and nondirectional forms trade different risks. A one-tailed test has more power to detect an effect in the predicted direction because it concentrates alpha there, but it has zero power to detect an effect in the opposite direction; such an effect, however large, will not be flagged as significant and may be misread as no effect. A two-tailed test can detect effects in either direction but requires a larger effect or sample to reach significance in any one direction, because alpha is split. Weigh the value of detecting an unexpected reverse effect against the value of extra power in the predicted direction, and choose the form whose risk profile matches the stakes.

### State The Implication Of An Opposite-Direction Result

Before running the test, decide and document what an effect in the non-predicted direction would mean for the one-tailed case. If you choose one-tailed, you are declaring that an opposite-direction effect is either impossible or irrelevant; if it occurs, the correct interpretation is not "significant in the other direction" (the test cannot say that) but a signal that the directional assumption was wrong and the study should be reconceived. Failing to plan for this leaves the team to improvise a misleading interpretation when the data contradict the prediction. Make the contingency explicit.

### Keep The Research Prediction And The Statistical Test Distinct

A directional research prediction and a one-tailed statistical test are related but separable. You may hold a directional research hypothesis and still choose a two-tailed test to remain sensitive to surprises, or you may have no directional prediction and still report directional effect estimates with confidence intervals. Do not collapse the two. State the research prediction in plain language, state the statistical test and its sidedness separately, and confirm that the conclusion you draw is licensed by the test actually performed, not by the prediction alone.

### Align Sidedness Across Primary, Secondary, And Equivalence Goals

When a study has multiple goals, the sidedness choices must be coherent. A primary superiority question might be one-tailed, but a secondary safety question is often two-tailed because harm in either direction matters. An equivalence or non-inferiority goal uses a different framework entirely, with hypotheses reversed and sidedness defined by the margin. Mixing sidedness inconsistently across goals, or switching forms for the same outcome, inflates the error rate and confuses readers. Specify the form for each hypothesis and keep them aligned with their distinct inferential goals.

## Common Traps

### Choosing One-Tailed After Seeing The Effect Direction

The trap is inspecting the data, noticing the effect is positive, and then declaring a one-tailed test to halve the p-value. The harm is a large inflation of the false-positive rate, because the decision is conditional on the sign of the observed effect. The mechanism is that the effective alpha in the chosen direction becomes roughly double the nominal level. Fix sidedness a priori, and if it was not fixed, use two-tailed.

### Using One-Tailed To Reach Significance While Ignoring Reverse Effects

The trap is adopting a one-tailed test purely for power while an effect in the opposite direction would be scientifically important. The harm is a real reverse effect going undetected and being misreported as no effect, because the one-tailed test has no critical region in that tail. The mechanism is that concentrating alpha sacrifices all sensitivity to the other direction. Reserve one-tailed tests for cases where an opposite effect is truly impossible or irrelevant.

### Conflating A Directional Prediction With A One-Tailed Test

The trap is assuming that holding a directional research belief obligates a one-tailed test, or that a two-tailed test contradicts a directional prediction. The harm is an inflexible pairing that either wastes power or blinds the study to surprises. The mechanism is confusing the scientific prediction with the statistical procedure. Treat them as distinct choices and justify each on its own merits.

### Switching Sidedness Mid-Analysis

The trap is running a two-tailed test, finding it nonsignificant, and then switching to one-tailed to cross the threshold, or vice versa to soften a result. The harm is that the reported alpha no longer reflects the actual error rate, and the conclusion is a product of the analyst's path rather than the data. The mechanism is optional stopping on the test form. Pre-specify sidedness and treat any change as a deviation to be reported.

### Misreading A One-Tailed Test When The Effect Reverses

The trap is observing a large effect opposite to the prediction and trying to interpret it as "significant in the other direction" under a one-tailed test. The harm is a conclusion the test cannot support, since the one-tailed critical region lies entirely in the predicted tail. The mechanism is applying two-tailed logic to a one-tailed result. Recognize the opposite effect as evidence the directional assumption failed, and reconceive the analysis rather than reinterpreting it.

### Splitting Alpha Without Justification In Two-Tailed Tests

The trap is defaulting to a two-tailed test that splits alpha equally when the two directions are not equally important, reducing power for the direction that matters. The harm is a missed detection of the effect of primary interest because half the alpha was allocated to a direction no one cares about. The mechanism is uncritical use of the conventional form. When asymmetry is justified by theory, consider a directional test, and document why.

### Inconsistent Sidedness Across Outcomes

The trap is using one-tailed for the primary outcome and two-tailed for secondary outcomes, or changing forms between the protocol and the report, without a coherent rationale. The harm is an error rate that is impossible to audit and conclusions that appear cherry-picked. The mechanism is uncoordinated choices across the analysis plan. Specify the form for every hypothesis up front and keep the rationale consistent.

### Reporting Direction Without A Directional Test

The trap is stating "the treatment increased the outcome" as a confirmed directional conclusion when the test was two-tailed and significant, without checking that the effect estimate and confidence interval support the direction. The harm is an overstatement, because a two-tailed significant result licenses rejecting the null but the direction and magnitude still need explicit reporting. The mechanism is sliding from a nondirectional test to a directional claim. Always pair the test with the effect estimate, its sign, and its confidence interval.

## Self-Check

- [ ] Is the choice between directional and nondirectional driven by theory, prior evidence, and the research question rather than by the desired p-value?
- [ ] Does the sidedness of the statistical test exactly match the form of the hypothesis (one-tailed with directional, two-tailed with nondirectional)?
- [ ] If a one-tailed test is planned, is there a documented a priori theoretical justification, and is an opposite-direction effect genuinely impossible or irrelevant?
- [ ] Was the sidedness fixed before data analysis and recorded in a protocol or preregistration, with two-tailed as the default when it was not pre-specified?
- [ ] Has the power tradeoff been weighed, acknowledging that one-tailed gains power in one direction but has zero power to detect an effect in the other?
- [ ] Is there an explicit, pre-specified plan for how to interpret an effect in the non-predicted direction under a one-tailed test?
- [ ] Are the directional research prediction and the statistical test stated separately, and is the conclusion licensed by the test actually performed?
- [ ] Are sidedness choices coherent across primary, secondary, safety, and equivalence goals, with each form justified for its inferential purpose?
- [ ] Are effect estimates, signs, and confidence intervals reported alongside the test so that any directional claim is explicitly supported?
- [ ] For high-stakes, confirmatory, or regulated designs, has a methodologist or statistician reviewed and approved the sidedness decisions before analysis?
