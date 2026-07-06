---
name: statistical_pitfalls_and_misinterpretation.md
description: Use when the agent is interpreting data analysis results, reading or writing a statistical claim, deciding whether an observed pattern reflects a real effect, summarizing a dataset, fitting and interpreting a model, or drawing a conclusion from metrics. Also covers the failure modes of confusing correlation with causation, Simpson's paradox, survivorship bias, p-hacking and multiple testing, data snooping/dredging, overfitting and in-sample evaluation, regression to the mean, base-rate neglect, the Texas-sharpshooter fallacy, and the recurring mistake of presenting a single summary statistic or significant p-value as definitive when the analysis is vulnerable to several silent errors at once.
---

# Statistical Pitfalls And Misinterpretation

A statistic is a summary of data, and every summary discards information in ways that can mislead. The judgment problem is that a correctly computed statistic can still support a wrong conclusion, because the conclusion depends on assumptions the computation does not check: that the sample represents the population, that the comparison is apples-to-apples, that the effect is not an artifact of how the data was selected or analyzed. Two variables can move together without one causing the other; an effect that holds in every subgroup can reverse in aggregate; the survivors in a dataset can teach a lesson that fails the population that did not survive; a "significant" result can be the one-in-twenty coincidence that multiple testing guarantees. These pitfalls do not announce themselves; the analysis looks clean, the p-value looks decisive, and the conclusion is wrong in a way that is hard to detect after the fact.

Agents tend to miss these problems because statistical tooling produces precise outputs and a clean number feels like an answer. The harm is decisions built on misread data. A feature ships because a metric "improved significantly," unaware that the team tested fifteen metrics and one was bound to move. A policy is set on a correlation that reverses under confound control. A strategy copies the traits of successful companies, missing that the same traits appeared in failed companies erased by survivorship. A model is trusted because it fits the data it was tuned on. The judgment problem is to read every statistical claim as a chain of assumptions — about sampling, comparison, confounding, multiplicity, and generalization — and to check the chain, not just the final number. This skill is deliberately conservative: a single analysis is rarely definitive, and confidence should scale with how many of these pitfalls have been ruled out, not with the precision of the output.

This skill covers the core interpretation pitfalls that recur across data work. It complements the ab-test-design skill (experiment-specific validity), the experiment-metrics skill (metric design), and the model-evaluation skill (ML-specific evaluation).

## Core Rules

### Never Read Correlation As Causation Without A Causal Argument

Two variables moving together is a statistical fact; one causing the other is a causal claim that requires more than co-movement. Conflating them is the most common statistical error:

- **Correlation has many non-causal explanations.** A may cause B; B may cause A; a confounder C may cause both; the correlation may be coincidence; the correlation may be induced by a common trend (both rising with time) or selection. The data alone cannot distinguish these.
- **Causation requires a causal design or argument.** Randomized experiments establish causation by breaking confounding. Without randomization, causation requires a plausible mechanism, control of likely confounders, temporal ordering (cause before effect), and ideally natural experiments or instrumental variables. Observational correlation plus "it makes sense" is not causation.
- **Control for confounders, and beware residual confounding.** In observational data, adjust for known confounders — but unmeasured confounders remain, and they are the rule, not the exception. A "controlled" analysis that ignores an obvious confounder is no better than the uncontrolled one.
- **State causation claims at the strength the design supports.** "Users who do X retain better" is an observation; "doing X causes better retention" requires an experiment or a strong quasi-experimental design. Match the language to the evidence.

### Watch For Simpson's Paradox — Aggregation Can Reverse The Effect

A trend that appears in aggregate can reverse in every subgroup, or vice versa, when subgroup composition differs. This is Simpson's paradox, and it makes aggregate conclusions point the wrong way:

- **Always slice by meaningful subgroups.** Before trusting an aggregate effect, check whether it holds within the segments that matter (by platform, cohort, severity, user type). If the effect reverses or vanishes in segments, the aggregate is misleading.
- **Composition drives the reversal.** Simpson's paradox occurs when one group is over-represented in one condition and has a different baseline. The aggregate is a weighted mix that can flip the direction of the within-group effect.
- **Decide which level is the decision-relevant one.** Sometimes the aggregate is right (when the composition is the real phenomenon); sometimes the within-group is right (when composition is a confound). Decide based on the question, not by picking the level that supports the preferred conclusion.

### Account For Survivorship And Selection Bias

A dataset of "what survived" teaches lessons that fail the full population, because the survivors differ systematically from those who did not make it into the data:

- **Ask what is missing and why.** Survivors (successful companies, returned surveys, completed funnels, non-crashed planes) are in the data; failures are not. Conclusions drawn only from survivors are biased toward the traits that correlate with survival, which may be irrelevant or harmful to the broader population.
- **Selection changes everything downstream.** Any analysis on a non-randomly-selected subset (users who opted in, customers who responded, records that passed a filter) describes that subset, not the population. Generalizing to the population requires accounting for the selection mechanism.
- **Look for the censoring.** In time-to-event data, subjects who have not yet experienced the event are not "failures" — they are censored. Treating censored subjects as failures (or dropping them) biases survival and conversion estimates.

### Correct For Multiple Testing, Or Accept That You Will Find "Significance" By Chance

Every test carries a false-positive probability, and running many tests guarantees false positives:

- **The family-wise error rate compounds.** At α=0.05, one in twenty true-null tests is expected to appear significant. Twenty metrics, ten segments, and a few time windows yield hundreds of tests — and many "significant" results that are pure chance.
- **Pre-specify the primary analysis and correct the rest.** Decide the main question and its test upfront; treat additional analyses as exploratory or apply a correction (Bonfieroni is conservative, Benjamini-Hochberg controls false-discovery rate). An uncorrected "we found a significant subgroup" is very likely noise.
- **Beware data snooping / dredging.** Slicing, transforming, and testing the same data until something is significant — then reporting that test as if pre-specified — produces results that will not replicate. The honest report states how many tests were run.

### Separate In-Sample Fit From Out-Of-Sample Generalization

A model or pattern that fits the data it was built on will almost always look good; the question is whether it generalizes:

- **In-sample fit is optimistic.** Any flexible enough model can fit the noise in its training data; the more parameters or the more tuning, the more it overfits. In-sample accuracy, R², or low error tell you about fit, not generalization.
- **Evaluate on held-out data the model never saw.** A clean train/validation/test split (or cross-validation, used correctly) estimates generalization. If the held-out performance is much worse than in-sample, the model overfit.
- **Beware fitting to the validation set.** Repeatedly tuning on the validation set until it improves leaks validation information; the held-out test set, touched once at the end, is the guardrail. (See the model-evaluation skill.)
- **Prefer simpler models unless complexity earns its keep.** A complex model that barely beats a simple one on held-out data is rarely worth the opacity and overfitting risk; the simple model is more likely to generalize.

### Beware Regression To The Mean

Extreme observations tend to be less extreme on re-measurement, because extremity is partly luck. This produces illusory "effects" wherever selection on extremeness is followed by re-measurement:

- **Selected-extreme groups regress.** The top performers this period will, on average, be less extreme next period — not because they got worse, but because their extremity included a lucky component. The bottom performers will, on average, improve.
- **Do not attribute regression to an intervention.** "We punished the worst performers and they improved; we rewarded the best and they declined" is regression to the mean, not evidence that punishment works and reward harms. A control group establishes whether the intervention did anything.
- **Always compare to a control when judging interventions on extreme groups.** Without a comparison, regression to the mean mimics a treatment effect.

### Respect Base Rates And Avoid Neglecting Them

A test's or model's predictive value depends on the base rate of what it predicts, not just its accuracy. Ignoring base rates produces wildly wrong posterior estimates:

- **A precise test on a rare event produces mostly false positives.** A 99%-accurate test for a condition with 0.1% prevalence has far more false positives than true positives. The positive predictive value depends on prevalence.
- **Always combine the likelihood with the prior.** Bayes' rule is not optional for interpretation: the probability of the condition given a positive result depends on both the test's accuracy and the base rate. State the base rate when reporting a detection or classification result.
- **Beware the prosecutor's fallacy.** Confusing P(evidence | innocent) with P(innocent | evidence) — a rare match is not the same as a low probability of innocence, unless the base rate is accounted for.

### Distinguish Statistical Significance From Magnitude And Importance

A significant p-value says "unlikely under the null," not "important" or "large":

- **Significance scales with sample size.** With enough data, any tiny difference is statistically significant; with little data, a large real effect is not detected. Significance is a threshold, not a measure of effect size.
- **Report and read effect sizes and confidence intervals.** The magnitude of the effect and its uncertainty are what matter for decisions; the p-value alone hides both. A significant result with a tiny effect size is usually not actionable.
- **A non-significant result is not "no effect."** It is "we did not detect an effect" — which may mean there is none, or that the study was underpowered. The confidence interval distinguishes "no effect" (tight interval around zero) from "inconclusive" (wide interval spanning meaningful effects).

## Common Traps

### Presenting Correlation As Causation

Claiming A causes B from co-movement without an experiment or a causal argument, leading to interventions that do not work. Establish causation with design and confound control; match language to evidence.

### Simpson's Paradox Hiding A Reversal

Trusting an aggregate effect that reverses within meaningful subgroups, deciding in the wrong direction. Slice by subgroup and check whether the effect holds within segments.

### Survivorship Bias In The Sampled Data

Drawing lessons from survivors (successful cases, completed funnels, returned surveys) that fail the full population including the missing failures. Ask what is missing and why; account for selection.

### Uncorrected Multiple Testing Producing "Significant" Noise

Running many tests and reporting the significant ones, guaranteeing false positives by chance. Pre-specify the primary analysis; correct or treat the rest as exploratory.

### Data Snooping Until Something Is Significant

Slicing and transforming the same data until a significant result appears, then reporting it as pre-specified. State how many analyses were run; treat post-hoc findings as hypothesis-generating.

### In-Sample Fit Reported As Generalization

Trusting a model's fit to its training data as evidence it will work on new data, when flexible models overfit. Evaluate on held-out data; reserve a once-used test set.

### Regression To The Mean Read As An Intervention Effect and base-Rate Neglect In Detection Or Classification

Attributing the natural regression of extreme groups to a punishment or reward, without a control. Use a control group when judging interventions on selected-extreme subjects.

Treating a precise test's positive result as high-probability-of-the-condition without accounting for a low base rate, producing mostly false positives reported as true. Combine likelihood with prior; report positive predictive value.

### Equating Significance With Importance and reading A Non-Significant Result As "No Effect"

Acting on a statistically significant but practically tiny effect (or dismissing a large effect as "not significant" in an underpowered study). Report effect sizes and confidence intervals; distinguish significance from magnitude.

Interpreting failure to detect as evidence of absence, when a wide confidence interval means the study was inconclusive. Read the interval: tight around zero is "no effect"; wide is "we don't know."

## Self-Check

- [ ] No causal claim is made from correlation alone; causation is supported by a randomized experiment, a quasi-experimental design with confound control, or a plausible mechanism plus temporal ordering — and the language matches the strength of the design.
- [ ] Aggregate effects were checked within meaningful subgroups to rule out Simpson's paradox; if the effect reverses or vanishes in segments, the aggregate is not trusted, and the decision-relevant level is chosen by the question.
- [ ] The dataset's selection mechanism is identified (what is missing and why: survivors, opt-ins, censored subjects, filtered records), and conclusions are scoped to the sampled population or adjusted for selection.
- [ ] Multiple testing is accounted for: a primary analysis is pre-specified, additional tests are corrected (Bonferroni/Benjamini-Hochberg) or labeled exploratory, and the number of analyses run is reported to prevent data snooping.
- [ ] Model and pattern performance is reported on held-out data the analysis did not tune on, with a once-used test set guarding against validation leakage; in-sample fit is not presented as generalization.
- [ ] Interventions judged on selected-extreme groups are compared against a control to separate real effects from regression to the mean.
- [ ] Detection, classification, and test results combine the likelihood with the base rate (positive predictive value), avoiding base-rate neglect and the prosecutor's fallacy.
- [ ] Effect sizes and confidence intervals are reported alongside p-values; statistical significance is distinguished from practical magnitude, and a non-significant result is read as "not detected" (with the interval inspected) rather than "no effect."
- [ ] The highest-risk cases were verified — correlation-as-causation, Simpson's reversal, survivorship, uncorrected multiplicity, in-sample overfit, regression to the mean, base-rate neglect, and significance-without-magnitude — not only the single clean summary statistic.
