---
name: null_and_alternative_hypothesis_construction.md
description: Use when the agent is constructing null and alternative hypotheses, pairing H0 with H1 or Ha, ensuring hypotheses are mutually exclusive and exhaustive, choosing one-sided versus two-sided alternatives, operationalizing a predicted relationship, linking hypotheses to variables and study design, or aligning alpha effect size and power before running an inferential test.
---

# Null And Alternative Hypothesis Construction

A hypothesis is the load-bearing structure of a study. The null hypothesis (H0) and the alternative (H1 or Ha) are not decorative labels; they define exactly what evidence the study collects, what test it runs, what conclusion it can support, and what error it is willing to risk. The judgment problem is that agents routinely write hypotheses that are vague, unpaired, or misaligned with the actual design, and then run a statistical test that answers a different question than the one posed. The blind spots are subtle. A hypothesis that names no measurable variable cannot be tested. A null and alternative that overlap cannot yield a decisive result. An alternative that was chosen after peeking at the data is not a hypothesis at all but a description disguised as one. The harm is real and cumulative: false positives presented as discoveries, non-reproducible claims that waste follow-up studies, and interventions adopted on the strength of a test that never actually addressed the research question.

This skill covers the disciplined construction of H0 and H1 before data are analyzed. It is methodological guidance for framing testable, well-paired hypotheses; it is not a substitute for statistical consultation, IRB or ethics review, or a registered analysis plan. Wrongly framed hypotheses can lead to misleading conclusions, so for high-stakes, novel, or regulated designs, consult a methodologist or statistician before locking the hypotheses.

## Core Rules

### Make The Null And Alternative Mutually Exclusive And Exhaustive

The pair must partition the parameter space so that every possible outcome falls under exactly one hypothesis and the two can never both be true. For a mean comparison, a clean pair is H0 stating the difference equals zero versus H1 stating the difference does not equal zero. Test the pair for overlap: if a parameter value could satisfy both H0 and H1, the test cannot adjudicate between them, and the result will be uninterpretable. Also test for a gap: if some plausible parameter value satisfies neither, the framework is incomplete and a reviewer will rightly reject the logic. State each hypothesis in terms of the same population parameter so the exclusivity is visible on the page.

### Write Each Hypothesis In Terms Of Measurable Population Parameters

A hypothesis is only as good as the operationalization behind it. Translate the conceptual claim into a statement about a specific parameter, such as a mean, proportion, difference, correlation, regression coefficient, or rate ratio, with the population and the units named. "The intervention reduces anxiety" is a research idea, not a statistical hypothesis; "the mean GAD-7 score in the treatment group is lower than in the control group at 12 weeks" is a statistical hypothesis because it names the measure, the groups, the direction, and the time point. If you cannot point to the variable and the parameter, the hypothesis is not yet testable.

### Decide One-Sided Versus Two-Sided Before Seeing The Data

The form of the alternative determines the critical region and the p-value, and it must be fixed a priori. A two-sided alternative (not equal) splits alpha across both tails and is the default when an effect in either direction would be meaningful or surprising. A one-sided alternative (greater than, or less than) concentrates alpha in one tail and is justified only when an effect in the opposite direction is either impossible by theory or of no practical interest. Document the rationale at the design stage and, ideally, in a preregistration. Switching to one-sided after observing the direction of the effect is a form of p-value manipulation that inflates the false-positive rate.

### Tie Every Hypothesis To Specific Variables And The Study Design

The hypothesis must match the design that will test it. Identify the independent variable, the dependent (outcome) variable, the unit of analysis, and the comparison group, and confirm the design can deliver them. A hypothesis about a causal effect requires randomization or a credible identification strategy; a hypothesis about association does not. A hypothesis phrased at the individual level cannot be tested with aggregate data, and vice versa. Before finalizing, walk from the hypothesis to the variables to the measurement instrument to the analysis, and confirm each link holds, because a break at any link invalidates the test.

### Align Alpha, Effect Size, And Power To The Hypothesis

The hypothesis, the significance level (alpha), the minimally important effect size, and the desired power form one system and must be set together. Decide what effect size would be scientifically or practically meaningful, not merely what is detectable. Confirm the sample size yields adequate power (commonly 0.80 or higher) to detect that effect at the chosen alpha, given the planned test and the one- or two-sided form. A hypothesis tested with insufficient power will fail to reject H0 whether or not the effect is real, producing an inconclusive study dressed up as a negative result. State these quantities explicitly and justify them.

### Distinguish The Research Hypothesis From The Statistical Hypothesis

The research hypothesis is the scientific prediction in plain language; the statistical hypothesis is the formal statement about parameters that the test evaluates. Keep both, and keep them linked, because they answer different questions. A study can reject H0 and still fail to support the intended research claim if the parameter tested was a proxy, if the effect was in the predicted direction but trivially small, or if confounding breaks the inferential chain. Write the research hypothesis, then derive the statistical hypothesis from it, then confirm that rejecting H0 would actually constitute evidence for the research claim.

### Specify The Hypothesis Before Data Collection Or In A Preregistration

The credibility of a hypothesis test depends on the hypotheses being fixed in advance. Record the hypotheses, the variables, the test, the alpha, and the sidedness in a protocol or preregistration before data are collected or analyzed. This protects against hypothesizing after the results are known (HARKing) and against flexible analysis choices that erode the meaning of the p-value. If the design is exploratory, label it as such and present the hypotheses as generated rather than confirmed, because the inferential standard differs.

### Plan For Multiple Hypotheses And Correct Accordingly

When a study tests several hypotheses, the chance of at least one false positive rises above the nominal alpha. Identify all hypotheses and comparisons up front, including subgroup and secondary outcomes, and decide on a correction strategy such as Bonferroni, Holm, or false discovery rate control. A hypothesis that is one of many but is reported as if it were the sole test carries an inflated error rate. State the family of tests and the correction in the analysis plan so the reported alpha matches the actual error control.

## Common Traps

### Writing A Null And Alternative That Overlap Or Leave A Gap

The trap is phrasing H0 and H1 so that some parameter values satisfy both or neither, which makes the test logically unable to decide. This happens when the hypotheses use different parameters, vague verbs, or implicit qualifiers. The harm is a result that is technically significant yet meaningless, because the test never truly adjudicated between two exclusive states. Force the pair onto a single parameter axis and confirm every value falls under exactly one hypothesis.

### Using A Vague Research Statement As The Statistical Hypothesis

The trap is treating "treatment improves outcomes" as the testable H1. The harm is that without a named measure, parameter, and direction, any result can be reinterpreted as support, and the test chosen later may not match the claim. The mechanism is that vagueness licenses post-hoc flexibility. Always reduce the claim to a statement about a specific measurable parameter before any analysis.

### Switching To One-Sided After Seeing The Effect Direction

The trap is observing that the effect is positive and then declaring a one-sided test to halve the p-value. The harm is a dramatic inflation of the false-positive rate, because the decision is conditional on the data. The mechanism is that the effective alpha becomes roughly double the nominal level for effects in the chosen direction. Fix sidedness a priori and document it; if it was not fixed, default to two-sided.

### Hypothesizing After The Results Are Known (HARKing)

The trap is generating a hypothesis from the observed data and presenting it as if it had been predicted, which makes a chance finding look like a confirmation. The harm is a literature filled with non-reproducible claims, because the "confirmed" hypothesis was constructed to fit noise. The mechanism is that post-hoc hypotheses fit the sample by definition and have no out-of-sample validity. Preregister hypotheses, and present unregistered findings as exploratory.

### Testing A Hypothesis The Design Cannot Support

The trap is writing a causal hypothesis and then testing it with a cross-sectional or observational design that cannot rule out confounding, or phrasing an individual-level hypothesis and testing it with aggregate data. The harm is a confident causal or individual-level claim that the data cannot bear. The mechanism is a mismatch between the inferential demand and the design's capacity. Map the hypothesis to the design and downgrade the claim to match what the design can deliver.

### Conflating Rejecting H0 With Proving The Research Claim

The trap is equating a small p-value with confirmation of the broader scientific story. The harm is overclaiming, because rejecting H0 only rules out the null, it does not establish the magnitude, the cause, or the practical importance of the effect. The mechanism is that significance is sensitive to sample size and can occur for trivial effects. Always pair the test with an effect size estimate, a confidence interval, and a judgment of practical importance.

### Ignoring Multiplicity Across Many Tests

The trap is running many hypothesis tests and reporting the significant ones without correction. The harm is an inflated family-wise error rate and a crop of false discoveries. The mechanism is that with enough tests, some will be significant by chance alone. Declare the full family of tests in advance and apply a correction matched to the inferential goal.

### Underpowering The Test And Reading Non-Rejection As No Effect

The trap is running an underpowered study, failing to reject H0, and concluding the effect does not exist. The harm is a false negative presented as evidence of no effect, which can suppress further work on a real phenomenon. The mechanism is that non-rejection in a low-power test is consistent with both no effect and a real effect the study could not detect. Report power and effect sizes, and phrase non-rejection as inconclusive rather than as proof of no effect.

## Self-Check

- [ ] Are H0 and H1 stated in terms of the same population parameter, and is every possible parameter value covered by exactly one of them (mutually exclusive and exhaustive)?
- [ ] Does each hypothesis name a measurable variable, parameter, population, and time point, so it could be tested against collected data?
- [ ] Was the choice of one-sided versus two-sided alternative fixed before data analysis, documented, and justified by theory or practical interest rather than by the observed effect direction?
- [ ] Does the hypothesis map cleanly to the independent variable, dependent variable, unit of analysis, and comparison group that the study design actually provides?
- [ ] Are alpha, the minimally meaningful effect size, and the target power stated together, and does the sample size yield adequate power for the planned test?
- [ ] Is the research hypothesis distinguished from the statistical hypothesis, and would rejecting H0 genuinely constitute evidence for the research claim?
- [ ] Were the hypotheses fixed before data collection or analysis, ideally in a preregistration, with any post-hoc hypotheses labeled as exploratory?
- [ ] If multiple hypotheses or comparisons are planned, is the family of tests declared and an appropriate multiplicity correction specified?
- [ ] Are the planned conclusions paired with effect sizes and confidence intervals, and is non-rejection framed as inconclusive rather than as proof of no effect?
- [ ] For high-stakes, novel, regulated, or uncertain designs, has a domain expert, methodologist, or statistician reviewed the hypotheses and analysis plan before data are collected?
