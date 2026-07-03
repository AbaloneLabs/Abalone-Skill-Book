---
name: population_and_period_definition.md
description: Use when the agent is defining the population for audit testing, verifying population completeness and reconciliation to the general ledger, defining the period under audit, handling cutoff and boundary items, or ensuring the sample population matches the assertion being tested and the conclusion being drawn.
---

# Population And Period Definition

Every audit conclusion is a conclusion about a population. If the population is wrong, the conclusion is wrong, regardless of how well the testing was performed. Agents frequently extract a list of transactions, sample from it, and project the result, without confirming that the list represents the complete population for the assertion and period being tested. A sample drawn from an incomplete or mis-scoped population cannot support any conclusion about the whole, and a population that excludes the risky items (voided, manual, late, or adjusted entries) produces a clean result that proves nothing. Population definition is the foundation of defensible testing, and it is the step most often skipped.

Use this skill before extracting any population for testing, before sampling, and whenever defining the period or boundary of a testing application. The goal is a complete, reconciled, assertion-appropriate population that genuinely supports the intended conclusion.

## Core Rules

### Define The Population To Match The Assertion And Conclusion

The population must correspond to the assertion being tested and the conclusion the auditor intends to draw. A mismatch between population and conclusion invalidates the result.

Match population to assertion:

- for completeness, the population must include all items that should have been recorded, often requiring a different source than the ledger itself;
- for existence or occurrence, the population is the recorded items, tested for validity;
- for accuracy, the population is the recorded items, tested for correct amounts;
- for cutoff, the population spans the period boundary on both sides;
- for valuation, the population is the balances, tested against evidence of value.

State the assertion and the conclusion before extracting. If the population does not cover the assertion, the conclusion cannot be drawn from it.

### Verify Population Completeness Before Sampling

A sample is only as good as the population it is drawn from. Before sampling, confirm the population represents the entire set of items relevant to the objective for the period.

Verify completeness by:

- identifying the source system and extraction date;
- confirming the period filters and cutoff;
- reviewing included and excluded transaction statuses;
- checking entity, location, and currency filters;
- reconciling the population total to the general ledger or a control total;
- confirming record count and monetary total;
- determining the treatment of voided, reversed, manual, and late items;
- checking for duplicate handling;
- obtaining data owner confirmation.

If the population cannot be reconciled, either fix the extract or document the limitation and reconsider whether the conclusion is possible. Sampling cannot cure a missing population.

### Reconcile The Population To An Independent Control Total

Reconciliation is the single most important completeness check. It confirms that the extracted population ties to an independently maintained total.

Reconcile:

- the monetary total of the population to the general ledger balance for the account;
- the record count to a system-generated count;
- the population to a sub-ledger or subsidiary record where one exists;
- period subtotals to monthly or periodic control totals;
- the population across locations or entities to consolidated totals.

Investigate any unreconciled difference before testing. An unreconciled population may be missing items, include duplicates, or cover the wrong period, any of which invalidates the conclusion.

### Define The Period And Handle Boundary Items

The period under audit must be defined explicitly, and items at the boundary require deliberate treatment. Cut-off errors and period misclassification are common and material.

Define and handle:

- the start and end dates of the population extract;
- transactions dated near the period boundary on both sides;
- items recorded in the period but belonging to another period;
- items belonging to the period but recorded later;
- reversing entries and accruals that cross the boundary;
- items in transit, in process, or unmatched at period end.

For cutoff-sensitive assertions, extract a window on both sides of the boundary and test items within it. A population that ends exactly at period end without a look-back or look-forward window misses cutoff manipulation.

### Include The Risky Items, Do Not Exclude Them

Populations are often extracted in a way that quietly excludes the riskiest items. Voided, reversed, manual, adjusted, and late entries are exactly where errors and fraud concentrate, and they are exactly what template extracts tend to omit.

Ensure the population includes:

- voided and reversed transactions, which may indicate processing errors or manipulation;
- manual journal entries, which bypass automated controls;
- adjusting and top-side entries, which may reflect override;
- late and post-period-end entries, which may be cut-off manipulation;
- entries to suspense, clearing, or temporary accounts;
- transactions with non-standard statuses or flags.

Confirm with IT or the data owner that no items are excluded by the extract logic. An extract that silently drops these items produces a clean but meaningless result.

### Confirm The Sampling Unit Matches The Testing Objective

The sampling unit is the item from which the sample is drawn. It must match the objective, or the conclusion will not follow from the result.

Choose the sampling unit to fit the objective:

- individual transactions, for testing authorization or accuracy of processing;
- monetary units, for testing overstatement of a balance;
- customer or vendor balances, for testing receivable or payable existence;
- line items within an account, for testing classification;
- document or record, for testing that a control was performed.

A mismatch, such as sampling customers to test transaction accuracy, produces a result that does not support the claimed conclusion. State the sampling unit and confirm it aligns with the assertion.

### Document The Population Definition For Reproducibility

The population definition must be documented so another auditor could reproduce the extract and reach the same population.

Document:

- the source system, table, and extraction method;
- the filters, including period, entity, location, currency, and status;
- the reconciliation to the control total and the resolution of any difference;
- the treatment of boundary, voided, manual, and late items;
- the sampling unit and its alignment with the assertion;
- the data owner confirmation and extraction date.

A population referenced only as "the list from the system" cannot be reproduced or defended. The definition is the evidence that the population was complete and appropriate.

## Common Traps

### Sampling From An Unreconciled Population

If the extract does not tie to the general ledger or a control total, the population may be incomplete or wrong, and no conclusion follows from the sample.

### Excluding Voided, Manual, Or Late Items

Template extracts often drop the riskiest items. A population that excludes voided, manual, adjusted, or late entries tests a cleaned subset and misses where errors concentrate.

### Mismatching Population To Assertion

Sampling recorded items to test completeness, or sampling customers to test transaction accuracy, produces a conclusion the population cannot support.

### Ending The Extract Exactly At Period End

A population with no boundary window misses cutoff manipulation. For cutoff-sensitive assertions, extract on both sides of the period end.

### Trusting The Extract Without IT Confirmation

Extract logic can silently exclude items through status filters or join conditions. Confirm with IT or the data owner that no items are dropped.

### Documenting Only The Sample, Not The Population

A sample documented without the population definition cannot be reproduced or defended. The population definition is the foundation, not a footnote.

### Drawing A Conclusion Beyond The Population

A conclusion about the whole ledger cannot be drawn from a sample of a subset. State the conclusion to match the population actually tested.

## Self-Check

- Is the population defined to match the assertion being tested and the conclusion intended, with the alignment stated explicitly?
- Is population completeness verified, including source system, period filters, status filters, and reconciliation to a control total?
- Is the population reconciled to the general ledger or an independent control total, with any difference investigated and resolved?
- Is the period defined explicitly, with boundary items handled through a window on both sides for cutoff-sensitive assertions?
- Does the population include the risky items, voided, reversed, manual, adjusted, and late entries, rather than excluding them through extract logic?
- Is the sampling unit confirmed to match the testing objective, so the conclusion follows from the result?
- Is the population definition documented for reproducibility, including source, filters, reconciliation, and data owner confirmation?
- Is the stated conclusion limited to the population actually tested, without over-extending to items or periods not covered?
