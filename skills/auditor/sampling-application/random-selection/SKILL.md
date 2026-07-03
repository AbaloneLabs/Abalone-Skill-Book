---
name: random-selection.md
description: Use when the agent is applying random selection to an audit sample, generating random numbers and mapping them to population items, ensuring every item has an equal chance of selection, handling monetary-unit sampling, validating that the selection is genuinely random, or documenting the random selection method and seed to support reproducibility.
---

# Random Selection

Random selection gives every sampling unit in the population a known, non-zero chance of selection, and is the selection method that supports statistical sampling and statistical projection. Its strength is that it eliminates selection bias and produces a sample whose properties can be evaluated mathematically. Its demands are real: the population must be completely and accurately defined, the random method must be genuinely random (not pseudo-convenience), and the selection must be reproducible and documented. The discipline is to apply random selection rigorously where statistical conclusions are needed, and to recognise its limits where the population cannot be fully defined or where judgement-targeted selection would be more efficient.

## Core Rules

### Define the population and sampling unit completely before selecting

Random selection is only valid against a completely and accurately defined population. Before selecting:

- define the population (all items in scope for the period);
- define the sampling unit (individual transaction, line item, monetary unit);
- reconcile the population total (record count, monetary total) to the ledger or source;
- confirm no items are inappropriately excluded or duplicated.

A random sample from an incomplete or misdefined population inherits the population's defects; the randomness is real but the population is wrong. The population definition is the foundation, and its reconciliation is the completeness check.

### Use a genuinely random method, not a convenience approximation

Random selection requires a method that gives every unit an equal (or defined) chance of selection:

- use a random number generator (spreadsheet function, statistical software, dedicated tool) with a documented seed;
- map the random numbers to population items by a defined rule (record number, cumulative monetary value for monetary-unit sampling);
- avoid methods that approximate randomness (every nth item without a random start is systematic, not random; "picking haphazardly" is not random).

Document the method, the seed, and the mapping rule so the selection is reproducible. A selection that cannot be reproduced cannot be verified to be random.

### Choose between record-based and monetary-unit sampling deliberately

Two common forms of random selection serve different purposes:

- **Record-based (equal probability per item)**: every transaction or line item has an equal chance. Suitable when the risk is evenly distributed across items.
- **Monetary-unit sampling (MUS, probability proportional to size)**: every monetary unit has an equal chance, so larger items have a higher probability of selection. Suitable for tests of details where larger monetary amounts carry more risk, and it naturally emphasises high-value items while still allowing projection.

Choose the form that matches the risk structure. MUS is often efficient for monetary assertions because it concentrates on larger amounts; record-based is appropriate where the risk is per-transaction rather than per-dollar.

### Ensure every item has a chance of selection

A valid random sample requires that every sampling unit has a known, non-zero chance of selection. Watch for:

- items that the selection method cannot reach (e.g., zero-value items in MUS, which have no monetary units to select);
- items excluded by a filter that should have been in scope;
- items added to the population after the frame was defined.

For zero-value or negative items (which MUS cannot select directly), decide deliberately whether to include them via a supplementary procedure (they may indicate completeness or classification issues) or to exclude them with documented rationale. Silent exclusion of hard-to-select items is a coverage gap.

### Validate and document reproducibility

Random selection must be reproducible: another auditor, given the population, the method, the seed, and the mapping rule, should be able to generate the same sample. Document:

- the population definition and its reconciliation;
- the sampling unit;
- the random method and tool;
- the seed (for computational methods);
- the mapping rule;
- the resulting selected items.

Reproducibility is what makes the selection verifiable and defensible. A selection documented only as "25 items selected randomly" cannot be reviewed or confirmed.

### Combine random selection with 100% examination of key items

Random selection of the whole population is often inefficient when a few large or high-risk items dominate. Combine:

- 100% examination of key items (above a threshold, or high-risk by nature);
- random selection of the residual population.

This stratified approach removes sampling risk for the key stratum and uses random sampling efficiently for the remainder. Document the stratification and the selection method for each stratum.

### Use random selection where statistical projection is needed

Random selection is necessary (though not sufficient) for statistical sampling and statistical projection. If the conclusion requires:

- a quantified confidence level;
- statistical projection of misstatement with a defined precision;
- a defensible sample size derived from statistical formulae;

then random selection is required. Judgement-based or systematic selection cannot support statistical conclusions, regardless of how carefully performed. Match the selection method to the conclusion needed.

### Recognise where random selection is unnecessary or inefficient

Random selection is not always the best choice. It is unnecessary or inefficient where:

- the population is small enough to examine 100%;
- targeted selection of high-risk items (related-party, unusual, recent) is more efficient than random coverage;
- the conclusion does not require statistical projection (nonstatistical sampling with judgement-based selection is acceptable);
- haphazard selection is sufficient for a nonstatistical sample, provided bias is avoided.

Do not default to random selection where it adds design cost without proportional benefit. Reserve it for populations where its statistical properties are needed.

### Reconcile selected items to the population after selection

After selection, confirm the selected items exist in the population and that the selection reconciles:

- each selected item can be located in the population;
- the count of selected items matches the planned sample size (accounting for any methodology rules);
- no selected item was silently substituted.

This post-selection reconciliation catches errors in the selection process (a random number that mapped to a non-existent record, a duplicate selection) before they distort the sample.

## Common Traps

- **Selecting from an incomplete or unreconciled population**, so the randomness is real but the population is wrong.
- **Using a convenience approximation** (every nth item, haphazard picking) and calling it random, when it cannot support statistical conclusions.
- **Failing to document the method, seed, and mapping rule**, making the selection irreproducible and unverifiable.
- **Silently excluding zero-value, negative, or hard-to-select items**, creating coverage gaps, especially in MUS.
- **Using random selection where targeted or 100% examination would be more efficient**, adding cost without benefit.
- **Assuming random selection alone makes a sample statistical** — it is necessary but not sufficient; sizing and evaluation must also be statistical.
- **Not reconciling selected items to the population after selection**, missing mapping errors or duplicates.
- **Choosing record-based vs monetary-unit sampling without considering the risk structure**, using an inappropriate form for the assertion.
- **Treating reproducibility as optional**, when it is essential to verifiability and defence.

## Self-Check

- Is the population completely and accurately defined, with the sampling unit specified and the total reconciled to the ledger?
- Did I use a genuinely random method with a documented seed and mapping rule, not a convenience approximation?
- Is the form of random selection (record-based vs monetary-unit) appropriate to the risk structure and the assertion?
- Does every sampling unit have a known, non-zero chance of selection, with zero/negative/special items handled deliberately?
- Is the selection reproducible — could another auditor regenerate the same sample from the documented method, seed, and rule?
- Did I combine random selection with 100% examination of key items where a few large or high-risk items dominate?
- Am I using random selection because statistical projection is needed, or would nonstatistical selection be sufficient and more efficient?
- After selection, did I reconcile the selected items to the population to catch mapping errors or duplicates?
- Is the population, sampling unit, method, seed, mapping rule, and selected items documented so the selection is defensible?
