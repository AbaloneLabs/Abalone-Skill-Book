---
name: audit_sampling_and_projection.md
description: Use when the agent is designing audit samples, deciding sample sizes, choosing statistical or judgmental sampling methods, selecting items, evaluating deviations or misstatements, projecting sample errors to a population, or deciding whether test results support a population-level conclusion.
---

# Audit Sampling And Projection

Audit sampling is the application of procedures to less than the whole population so the auditor can reach a conclusion about the whole. Done well, it is efficient and defensible. Done casually, it produces a confident-looking conclusion that the evidence never supported. The most common failure is not arithmetic; it is a mismatch between what was sampled, how it was selected, and what the auditor later claims the result proves. A targeted selection of high-risk items cannot support an error-rate conclusion, and a random sample drawn from an incomplete population cannot support anything at all.

Use this skill before designing a sample, choosing a selection method, deciding sample size, evaluating deviations, projecting errors, or writing a conclusion that extrapolates from a sample to a population. The goal is to make the sampling design, the projection logic, and the limits of the conclusion explicit and defensible.

## Core Rules

### Decide What Conclusion The Sample Must Support

Sampling design follows from the conclusion required. A sample built to test whether a control operated (attributes sampling) is designed differently from a sample built to test whether an account balance is not materially misstated (variables sampling or monetary unit sampling). Before selecting anything, state the objective.

For each sampling application, document:

- the assertion or control objective being tested;
- the population to which the conclusion will apply;
- whether the conclusion is attribute-based or monetary;
- the sampling approach, statistical or non-statistical;
- the tolerable deviation rate or tolerable misstatement;
- the expected deviation or error rate;
- the acceptable level of sampling risk;
- the confidence level if statistical sampling is used.

If the objective is unclear, the sample size and selection method cannot be justified.

### Define The Population And Verify Completeness

A sample is only as good as the population it is drawn from. Before sampling, confirm the population represents the entire set of items relevant to the objective for the period under audit.

Verify:

- the source system and extraction date;
- the period filters and cutoff;
- included and excluded transaction statuses;
- entity, location, and currency filters;
- reconciliation of the population total to the general ledger or control total;
- record count and monetary total;
- treatment of voided, reversed, manual, and late items;
- duplicate handling;
- data owner confirmation.

If the population cannot be reconciled, either fix the extract or document the limitation and reconsider whether a sampling conclusion is possible. Sampling cannot cure a missing population.

### Separate Statistical From Non-Statistical Sampling

Both statistical and non-statistical sampling can be valid, but they support different claims and require different discipline.

Statistical sampling uses probability-based selection so sampling risk can be measured. It requires:

- a random or systematic selection method with a defined selection probability;
- a defined sampling unit;
- a sample size derived from risk, tolerable rate, and expected rate;
- an evaluation that compares results to a computed precision or confidence interval;
- a documented projection method.

Non-statistical, judgmental sampling relies on auditor judgment for selection and evaluation. It is appropriate when the objective is to identify specific issues, test high-risk items, or corroborate other evidence. It does not, by itself, support a quantified error-rate conclusion for the whole population.

Do not label a haphazard selection as statistical. Do not imply that a judgmental sample supports a confidence interval it was never designed to support.

### Justify Sample Size Against Risk And Tolerable Limits

Sample size should respond to risk, tolerable deviation or misstatement, and the expected error rate. A larger sample is needed when risk is high, tolerable error is low, or errors are expected.

Factors that should increase sample size:

- higher inherent or control risk;
- lower tolerable deviation rate or tolerable misstatement;
- higher expected error rate;
- greater need for assurance;
- larger and more heterogeneous population;
- history of prior exceptions;
- fraud risk or management override;
- significant manual intervention or estimates.

Factors that may support a smaller sample include low risk, stable and automated processes, strong corroborating evidence, and a history of clean results. Document the reasoning, not only the number.

### Treat High-Value And High-Risk Items Deliberately

Many populations contain a few items that are individually significant. These should usually be examined entirely rather than sampled, because the failure of a single large item can be material on its own.

Consider separately testing:

- items above a defined monetary threshold, such as performance materiality;
- unusual, related-party, or manually adjusted transactions;
- late-period and cutoff-sensitive items;
- items flagged by analytics or prior-year issues;
- transactions involving estimates or management judgment.

If high-value items are tested 100 percent and the remainder is sampled, state that the conclusion applies to the sampled residual population, not to the items examined separately.

### Define The Exception Or Deviation Before Testing

The auditor must decide what counts as a deviation before looking at the items. Defining exceptions after seeing the results invites biased evaluation.

Define:

- the attribute or compliance criterion;
- what constitutes a pass;
- what constitutes a deviation;
- how missing evidence is treated;
- whether timing, authorization, completeness, accuracy, or another attribute is tested;
- whether a single item can have multiple deviations.

If evidence is missing for a selected item, treat it as a deviation or perform alternative procedures. Do not silently replace it with a clean item, because that biases the sample toward favorable results.

### Evaluate Results And Project Errors

Once testing is complete, evaluate deviations or misstatements against the tolerable limit and project the results where the method supports projection.

For attribute sampling, compare the observed deviation rate, adjusted for sampling risk, to the tolerable deviation rate. For monetary unit or variables sampling, project the misstatement to the population and compare the projected error, plus sampling risk allowance, to tolerable misstatement.

Projection methods include:

- ratio projection, which scales sample errors by the ratio of population to sample value;
- difference projection, which scales the average error per item to the population count;
- mean-per-unit estimation;
- monetary unit projection based on the tainting of selected dollars.

Document the method, the inputs, and the result. If errors are projected, also assess whether the errors are anomalous, in which case they should not be projected but should be considered individually.

### Decide Whether To Extend Testing

When exceptions exceed expectations, the auditor should not simply accept or reject the population. Consider the cause, whether errors are systemic, and whether additional procedures are needed.

Options include:

- requesting management explanation and corroborating it;
- testing additional items to refine the estimate;
- targeting the subpopulation where errors clustered;
- asking management to investigate the population;
- qualifying the conclusion or escalating;
- concluding the control or balance is not supported.

Do not quietly expand the sample until the error rate looks acceptable. Any expansion should be documented with its own rationale and result.

### State The Limits Of The Conclusion

A sampling conclusion should describe exactly what it covers and what it does not. Readers should not be left to infer that the whole population was examined or that the result guarantees no error.

State:

- the population and period;
- the sampling method and size;
- the tolerable limit used;
- the observed and projected error;
- the sampling risk allowance;
- whether the conclusion is attribute-based or monetary;
- any anomalous items treated separately;
- any limitations from an incomplete population or unavailable evidence.

## Common Traps

### Drawing A Random Sample From A Non-Random Subset

If the extract excludes canceled, manual, or late items, the sample is drawn from a cleaned population and the conclusion does not cover the risky items. Reconcile the population before sampling.

### Projecting Errors From A Targeted Sample

Targeted or judgmental selection is useful for finding problems, but the error rate it produces cannot be projected to the population. Do not multiply a targeted sample's error rate by the population size.

### Replacing Missing Or Difficult Items

Swapping out a selected item because its evidence is missing biases the sample. Treat missing evidence as a deviation or perform alternative procedures and document the choice.

### Defining Exceptions After Seeing The Results

Redefining what counts as an error to make the results look better is not evaluation; it is rationalization. Define the deviation criterion before testing.

### Confusing Tolerable Misstatement With Expected Error

Tolerable misstatement is the maximum error the auditor will accept. Expected error is the rate anticipated before testing. Using the wrong value in the sample size formula produces a sample that is either too small or wastefully large.

### Ignoring Anomalous Errors In Projection

Some errors are isolated and do not reflect the population, such as a single misclassified transaction caused by a one-time system change. These should be evaluated individually rather than projected, and the decision should be documented.

### Claiming A Population Conclusion Without Stating Sampling Risk

A statistical sample supports a conclusion only within a confidence interval. Stating the point estimate without the precision or sampling risk allowance overstates the reliability of the result.

### Treating Sampling As A Box To Tick

If the sample size, method, and evaluation are copied from a template without considering risk, tolerable limits, and expected error, the sampling work is decoration. The design must respond to the specific objective.

## Self-Check

- Does the sampling application state the assertion or control objective, the population, the period, and whether the conclusion is attribute-based or monetary?
- Has the population been reconciled to a control total and checked for completeness before sampling?
- Is the sampling method labeled correctly as statistical or non-statistical, and does the claimed conclusion match what that method can support?
- Is the sample size justified by risk, tolerable deviation or misstatement, expected error, and confidence level?
- Are high-value and high-risk items identified and tested separately where individually significant?
- Is the exception or deviation criterion defined before testing, including how missing evidence is treated?
- Is the projection method documented, and are anomalous errors evaluated individually rather than projected?
- When exceptions exceed expectations, is the response documented rather than silently adjusting the sample?
- Does the stated conclusion include the population, method, tolerable limit, observed and projected error, sampling risk allowance, and any limitations?
- Could an experienced auditor reproduce the sampling logic and reach the same conclusion from the documentation?
