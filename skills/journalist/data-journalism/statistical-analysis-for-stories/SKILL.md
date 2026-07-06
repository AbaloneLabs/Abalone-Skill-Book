---
name: statistical-analysis-for-stories.md
description: Use when the agent is analyzing data to support a news story, choosing appropriate statistical methods, interpreting significance and effect size, avoiding common statistical errors, distinguishing correlation from causation, or presenting quantitative findings accurately and without overclaiming.
---

# Statistical Analysis For Stories

Statistics can reveal truths that no anecdote can, but they can also manufacture false certainty with the appearance of rigor. The harm this skill prevents is the publication of statistical claims that are wrong, misleading, or overclaimed, findings built on inappropriate methods, misread significance, ignored confounders, or effect sizes dressed up as meaningful. Journalists need not be statisticians, but they must understand enough to choose appropriate methods, recognize common errors, interpret results honestly, and know when to consult an expert. Statistical humility, the willingness to report what the numbers show and no more, is the core discipline.

## Core Rules

### Match the method to the question and the data
Different questions require different methods: rates for comparison, regression for controlling confounders, time-series for trends. Choose the method that fits the structure of the data and the question being asked, not the method that produces the most striking result. A sophisticated method applied to the wrong question produces precise nonsense.

### Always compare against an appropriate baseline
A number means nothing in isolation. "X percent of cases involve Y" requires knowing the rate of Y in a relevant comparison population. Choose the comparison deliberately, defend it, and test whether the finding holds under reasonable alternative comparisons. The choice of baseline can create or erase a finding.

### Distinguish statistical significance from practical importance
A result can be statistically significant, unlikely due to chance, yet too small to matter in practice. Report effect sizes and their real-world magnitude alongside significance. A tiny effect with a tiny p-value is still a tiny effect; conversely, an underpowered test may miss a large real effect.

### Separate correlation from causation, always
Two variables moving together does not establish that one causes the other. Confounders, reverse causation, and coincidence all produce correlation. Claim causation only with a plausible mechanism, control for likely confounders, and supporting evidence; otherwise report the association honestly and flag the uncertainty.

### Account for uncertainty and convey it honestly
Every estimate has uncertainty: confidence intervals, margins of error, the range of plausible values. Report the uncertainty, not just the point estimate. Presenting a single precise number when the true value could plausibly span a wide range misleads readers into false confidence.

### Beware multiple comparisons and the multiple-testing problem
Running many tests increases the chance of finding a "significant" result by accident. If you tested many comparisons, adjust for that, or treat striking results as hypotheses requiring confirmation rather than established findings. The more you sift, the more rigorously you must guard against false positives.

### Control for confounders rather than ignoring them
A relationship between two variables may be driven by a third. Identify likely confounders, age, income, geography, prior behavior, and use methods that adjust for them. An unadjusted association is a starting point, not a conclusion, and can reverse direction when confounders are accounted for.

### Submit analysis to expert review and publish methods
Have the analysis reviewed by an independent statistician or domain expert before publication, and publish the data and methods so others can reproduce and challenge the findings. Transparency and external review are what distinguish accountable data journalism from assertion with numbers attached.

## Common Traps

### The base-rate fallacy
Ignoring the underlying rate of an event leads to dramatic-sounding but misleading claims. A test with a small false-positive rate can produce mostly false positives when the condition is rare. Always consider the base rate when interpreting proportions and probabilities.

### Simpson's paradox
A trend in aggregate data can reverse in every subgroup. A pattern found overall may disappear or invert when the data is broken down by a relevant category. Test findings at multiple levels of aggregation before claiming they hold.

### Overfitting and p-hacking
Tuning a model or running analyses until something significant appears produces findings that do not generalize. Pre-register the primary question, resist tweaking until significance appears, and be skeptical of results that depend on specific, convenient analytical choices.

### Survivorship and selection bias
Analyzing only the cases that survived a selection process, successful companies, resolved cases, reported incidents, produces distorted conclusions about the whole. Understand what the dataset excludes and how that shapes the findings.

### Ecological fallacy
Drawing conclusions about individuals from group-level data, or vice versa, produces errors. A relationship observed at the aggregate level may not hold for individuals. Be explicit about the level of analysis and do not cross levels without justification.

### False precision
Reporting "32.7 percent" when the data and methods only support "about a third" conveys a precision the analysis does not have. Match the precision of presentation to the precision of the underlying estimate.

### Confusing absence of evidence with evidence of absence
Failing to find an effect is not the same as proving there is no effect, especially with small samples or noisy data. Distinguish "we found no effect" from "there is no effect," and report which you actually established.

## Self-Check

- [ ] Is the statistical method appropriate to the question and the structure of the data?
- [ ] Have I compared against a deliberate, defensible baseline, and tested alternative comparisons?
- [ ] Am I reporting effect sizes and practical importance, not just statistical significance?
- [ ] Have I separated correlation from causation, claiming causation only with mechanism, confounder control, and supporting evidence?
- [ ] Am I conveying the uncertainty in the estimates honestly, including ranges and margins of error?
- [ ] Did I account for multiple comparisons, or treat striking unconfirmed results as hypotheses rather than findings?
- [ ] Have I identified and adjusted for likely confounders, rather than reporting unadjusted associations as conclusions?
- [ ] Did I have the analysis reviewed by an independent expert, and are the data and methods published for scrutiny?
- [ ] Have I tested the findings at multiple levels of aggregation to guard against ecological fallacy and Simpson's paradox?
- [ ] Before publication, is every quantitative claim matched to what the analysis actually supports, with no overclaiming or false precision?
