---
name: statistical_sampling_software.md
description: Use when the agent is using statistical sampling software for audit, performing attribute, monetary-unit, or variables sampling, defining sampling parameters such as confidence level and tolerable and expected error, generating defensible samples, documenting parameters and selection method, or evaluating sample results against precision and tolerable limits.
---

# Statistical Sampling Software

Statistical sampling software turns a set of judgment calls into a defensible sample, but only if the inputs are chosen deliberately and the outputs are documented. The same tool that produces a clean, random sample can also produce a meaningless one when the confidence level, tolerable error, or expected error is set to whatever the software defaults to rather than to what the risk actually requires. Agents often treat the software as authoritative because it produces precise-looking numbers, but the precision of the output is only as good as the relevance of the parameters and the integrity of the population fed in. The harm this prevents is a sample that looks statistically rigorous yet tests the wrong population, uses parameters that do not match the assessed risk, or cannot be reproduced or defended under review.

Use this skill whenever statistical sampling software is used to plan, generate, or document an audit sample, and whenever parameters are set or results are evaluated. The goal is a sample whose method, parameters, and selection are transparent, reproducible, and tied to the specific risk and assertion being tested.

## Core Rules

### Match The Sampling Approach To The Audit Objective

Different objectives call for different sampling methods, and choosing the wrong method undermines the whole test. Attribute sampling suits tests of control; monetary-unit sampling suits substantive tests where misstatement amount matters; variables sampling suits substantive tests of account balances.

Match method to objective by:

- using attribute sampling for yes/no control deviations;
- using monetary-unit sampling for overstatement of balances;
- using variables (classical) sampling for substantive dollar testing;
- confirming the method addresses the relevant assertion;
- avoiding a method just because the software makes it easy to run.

### Set Parameters Deliberately, Not By Default

The parameters drive the sample size and the conclusion, and the software's defaults reflect no knowledge of this engagement's risk. Every parameter must be chosen for a reason and linked to the risk assessment.

For each sample, justify:

- confidence level and why it is appropriate to the risk;
- tolerable error or deviation rate and how it was set;
- expected error rate and the basis for the estimate;
- population size and recorded value;
- whether the population is stratified and on what basis;
- the definition of an error or deviation;
- the sampling unit and how it maps to the population.

### Confirm The Population Before Sampling

Statistical sampling assumes a complete and accurate population. Sampling software cannot detect that the population is missing an entity or excludes voids; it will simply produce a clean sample of an incomplete frame.

Before generating the sample:

- reconcile the population to the GL or control total;
- confirm completeness of entities, currencies, and periods;
- confirm the treatment of voids, manuals, and deleted items;
- remove items not subject to testing, with documented reasons;
- verify the sampling unit field is unique and populated;
- confirm the population matches the assertion being tested.

### Control The Random Seed And Selection Method

Reproducibility depends on a controlled seed and a documented selection method. A sample that cannot be regenerated cannot be defended, because no one can confirm which items were selected.

Record for every sample:

- the software name and version;
- the exact selection method (random, systematic, MUS);
- the random seed or starting point used;
- the population file and its version or hash;
- the date and operator who generated the sample;
- the full list of selected items with their keys.

### Stratify When It Materially Improves Efficiency

Stratification can focus effort on the items that carry the most risk, but it must be applied transparently and consistently. Poorly documented stratification can bias the sample toward or away from large items.

When stratifying:

- state the stratification variable and cut points;
- confirm every population item falls into exactly one stratum;
- document the sample size and method within each stratum;
- ensure high-value or risky items receive appropriate coverage;
- reconcile the sum of strata to the total population;
- avoid strata so small they cannot support a sample.

### Document The Sampling Plan Before Selecting

The plan should be fixed before selection so that the results cannot be reframed after the fact. Changing parameters after seeing the results undermines the statistical validity of the conclusion.

The documented plan should include:

- the objective and assertion;
- the population definition and reconciliation;
- the method and all parameters;
- the definition of an error;
- the expected sample size;
- the decision rule for evaluating results;
- the preparer and reviewer sign-off.

### Evaluate Results Against The Pre-Set Decision Rule

The conclusion follows from comparing the evaluated result to the tolerable limit using the rule fixed in the plan. Re-running the sample or relaxing the rule because the result was unfavorable destroys the test's integrity.

When evaluating:

- compute the deviation rate or projected misstatement;
- compute the upper limit or precision per the method;
- compare the upper limit to the tolerable error;
- distinguish anomalous errors from projected errors;
- apply the decision rule exactly as documented;
- record the conclusion and its basis.

### Retain The Full Evidence Package

A defensible sample is more than the list of items tested. The evidence package must let a reviewer or inspector reproduce the entire process from population to conclusion.

Retain:

- the population file and its reconciliation;
- the sampling plan with parameters and sign-off;
- the software version, seed, and selection log;
- the list of selected items and their test results;
- the evaluation computation and conclusion;
- any exceptions and their disposition.

### Define An Error Before Testing Begins

The definition of an error drives the evaluation, and an ambiguous definition lets the auditor reclassify findings after the fact. Fix the error definition in the plan so that every deviation is scored consistently and cannot be argued away.

The error definition should specify:

- what constitutes a deviation for a controls test;
- what constitutes a misstatement for a substantive test;
- whether timing or classification differences count as errors;
- how partial errors are measured and weighted;
- whether errors caused by the auditor are excluded;
- the treatment of errors that are later corrected by management.

## Common Traps

### Accepting Software Defaults For Parameters

Default confidence and error rates are generic and bear no relation to this engagement's risk. A sample sized on defaults can be far too small for a high-risk area or wastefully large for a low-risk one. Always set parameters to the assessed risk.

### Sampling An Unreconciled Population

Statistical software will happily sample a population that is missing an entity or excludes a document class. The sample looks valid but tests an incomplete frame. Always reconcile and confirm completeness before sampling.

### Changing Parameters After Seeing Results

Adjusting confidence, tolerable error, or expected error once the results are known is a form of result shopping that invalidates the test. Lock the plan before selection and apply the decision rule as written.

### Confusing Attribute And Variables Results

Treating a deviation rate from an attribute sample as if it were a dollar misstatement, or vice versa, produces a conclusion that does not match the method. Keep the evaluation consistent with the sampling method used.

### Losing Reproducibility By Omitting The Seed

Without the seed and method, the selected items cannot be regenerated, and the sample cannot be defended. Record the seed, method, software version, and population version every time.

### Over-Reliance On Precision In The Output

The software prints precise-looking limits, but that precision is meaningless if the parameters were wrong or the population was incomplete. Judge the sample by the quality of inputs, not the apparent precision of outputs.

### Stratifying Without Documenting The Basis

Undocumented stratification can quietly over- or under-weight large items and bias the conclusion. Always state the stratification variable, cut points, and per-stratum method.

### Treating Zero Errors As Automatic Comfort

A clean sample does not prove the population is free of error; it only proves that errors, if present, are unlikely to exceed the tolerable rate at the chosen confidence. State the conclusion in those statistical terms, not as certainty.

## Self-Check

- Is the sampling method matched to the specific assertion and objective, not chosen for convenience?
- Is every parameter (confidence, tolerable error, expected error, population) justified and tied to risk?
- Has the population been reconciled and confirmed complete before the sample was generated?
- Are the software name, version, selection method, and random seed recorded for reproducibility?
- Is the sampling plan documented and signed off before selection, with a fixed decision rule?
- If stratified, are the variable, cut points, and per-stratum method documented and reconciled to the total?
- Is the evaluation performed using the pre-set decision rule, without adjusting parameters after the fact?
- Are anomalous errors distinguished from projected errors in the evaluation?; is the conclusion stated in statistical terms relative to tolerable limits, not as absolute certainty?
- Does the retained evidence package allow a reviewer to reproduce the process from population to conclusion?; would an inspector accept this sample as defensible based on the documentation alone?
- Is the error definition fixed in the plan and applied consistently to every item tested?; are zero-error and high-error results evaluated against the plan's decision rule without relaxing parameters?
