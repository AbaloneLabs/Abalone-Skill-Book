---
name: bias_and_fairness_in_ml.md
description: Use when the agent is assessing bias or fairness in a machine learning system, choosing a fairness definition, measuring disparities across groups, auditing data and model for bias, or deciding what fairness requires for a given application.
---

# Bias And Fairness In ML

Machine learning systems can perpetuate, amplify, or create unfairness, because they learn from data that reflect historical inequities, are measured against metrics that ignore groups, and are deployed in contexts where errors carry different costs for different people. When bias and fairness are handled superficially, three harms follow. A model is declared fair because it satisfies one mathematical criterion while violating another that matters more for the affected group. Harms to vulnerable groups are buried under aggregate performance. And fairness is treated as a checkbox rather than a continuing obligation, so bias re-emerges as the model and data evolve. Fairness in ML is not a single metric to optimize; it is a set of value-laden decisions that must be made explicitly and revisited.

The agent should use this skill when assessing a model for bias, choosing fairness metrics, auditing data or predictions, or advising on whether a system is fair enough for its use. The goal is to keep the agent from reducing fairness to a comforting number, when fairness is a contextual judgment that requires understanding who is affected, how, and what the tradeoffs are.

## Core Rules

### Recognize That Bias Can Enter At Every Stage

Bias is not only in the model. It can enter anywhere in the pipeline.

- Historical bias in the data reflecting past discrimination.
- Representation bias when some groups are underrepresented.
- Measurement bias when features or labels are measured differently across groups.
- Aggregation bias when one model fits some groups worse than others.
- Deployment bias when the system is used in ways that differ from its design.

Audit each stage, not just the final predictions. A model can be mathematically neutral on biased data and still produce biased outcomes.

### Define The Protected Groups And Harms Relevant To The Application

Fairness is always relative to specific groups and specific harms. Define them.

- Which groups are at risk, by race, gender, age, disability, language, or other attributes.
- What harms matter, false denials, false arrests, misdiagnosis, higher error rates, stereotyping.
- Which decisions the model informs and their consequences for affected individuals.
- The legal, ethical, and social context that constrains what is acceptable.

Fairness without specified groups and harms is abstract and useless. The first step is to make the stakes concrete.

### Choose A Fairness Definition Deliberately

There are many mathematical fairness criteria, and they often conflict. None is universally correct.

- Demographic parity, equal positive rates across groups.
- Equal opportunity, equal true positive rates.
- Equalized odds, equal error rates across groups.
- Calibration within groups, equal reliability of scores.
- Individual fairness, similar individuals treated similarly.

Each criterion encodes a different value and is appropriate in different contexts. Choosing one is a value judgment, not a technical default. State which is chosen, why, and what it sacrifices. Be aware that some criteria are provably incompatible in non-trivial settings.

### Measure Disparities Across Groups

Fairness assessment requires measuring how outcomes and errors distribute across groups.

- Report performance metrics per group, not only in aggregate.
- Report error types, false positives and false negatives, per group.
- Report calibration per group where relevant.
- Examine intersectional groups, not only single attributes, since harms can concentrate at intersections.

Aggregate fairness can hide harms to smaller or intersectional groups. Disaggregated reporting is the minimum for honest assessment.

### Audit The Data For Representation And Label Bias

The data shape what the model can learn. Audit them.

- Are all relevant groups adequately represented?
- Are labels proxies that encode bias, such as arrest as a proxy for crime?
- Are features measured consistently across groups?
- Are historical outcomes in the data themselves discriminatory?

A model trained on biased data will reproduce that bias, regardless of the algorithm. Data audit is often more important than model tuning for fairness.

### Understand The Tradeoffs Between Fairness And Other Objectives

Fairness interventions often trade off against accuracy, and different fairness criteria trade off against each other.

- Imposing a fairness constraint may reduce overall accuracy.
- Satisfying one criterion may violate another.
- The right balance depends on the stakes and values at play.
- Hiding the tradeoff makes the decision less legitimate, not more.

Make tradeoffs explicit and involve affected stakeholders where possible. A silent tradeoff imposed by the developer is less defensible than an explicit one.

### Consider The Decision Threshold And Its Differential Impact

Many systems threshold a score into a decision. The threshold affects groups differently.

- A single threshold can produce different error rates across groups with different score distributions.
- Group-specific thresholds can equalize error rates but may be legally or ethically constrained.
- The choice of threshold is a policy decision, not just a technical one.

Examine how threshold choices affect each group, and justify the threshold policy. The threshold is often where fairness is won or lost in practice.

### Test For Robustness Of Fairness Over Time And Shift

Fairness achieved at one moment can degrade as data and conditions shift.

- Monitor group-level performance after deployment.
- Re-audit when the population, environment, or use changes.
- Beware of feedback loops where predictions shape future data.
- Plan for re-evaluation, not one-time certification.

A model certified fair at launch can become unfair as the world changes. Fairness is an ongoing obligation, not a one-time check.

### Engage Affected Communities And Domain Experts and document Fairness Choices And Limitations

Fairness is not purely technical. Those affected understand the harms in ways data cannot.

- Consult affected communities about what harms matter.
- Involve domain experts who understand the application context.
- Consider participatory approaches to defining fairness.
- Be transparent with affected groups about how the system works.

Technical fairness metrics without community input can optimize the wrong thing. Lived experience identifies the harms that matter.

Record the fairness analysis so it can be reviewed and revisited.

- Which groups and harms were considered.
- Which fairness criteria were chosen and why.
- What disparities were found and what was done.
- What limitations and unresolved risks remain.

Documentation makes fairness decisions auditable and supports accountability. It also forces the team to be explicit about choices that might otherwise be implicit.

## Common Traps

### Satisfying One Criterion And Declaring Fairness

Criteria conflict. Meeting one may violate another that matters more.

### Aggregate Fairness Hiding Subgroup Harms

Averages can mask harms to smaller or intersectional groups. Disaggregate.

### Ignoring Data Bias

A neutral model on biased data reproduces bias. Audit the data.

### Treating Fairness As A Checkbox

Fairness degrades over time and shift. It is ongoing, not one-time.

### Hiding Tradeoffs

Fairness trades off against accuracy and against other criteria. Make tradeoffs explicit.

### Defaulting To A Single Threshold

Thresholds affect groups differently. Examine and justify the threshold policy.

### Technical Metrics Without Community Input

Metrics alone can optimize the wrong harm. Engage affected groups.

### No Documentation Of Fairness Choices

Undocumented choices cannot be audited. Record the analysis.

## Self-Check

- Has each stage of the pipeline, from data to deployment, been audited for sources of bias?
- Are the protected groups and specific harms relevant to the application defined explicitly?
- Is the fairness criterion chosen deliberately, with its rationale and sacrifices stated, and the incompatibility among criteria acknowledged?
- Are outcomes, error types, and calibration reported per group, including intersectional groups?
- Has the data been audited for representation, label, and measurement bias?
- Are tradeoffs between fairness, accuracy, and other criteria made explicit and justified?
- Is the decision threshold examined for differential impact across groups?
- Is fairness monitored over time and re-audited under distribution shift and feedback loops?
- Have affected communities and domain experts been engaged in defining what fairness requires?
- Are fairness choices, findings, and limitations documented for review and accountability?; for systems affecting significant rights, opportunities, or safety, has a fairness specialist, domain expert, or affected-community representative reviewed the assessment before deployment?
