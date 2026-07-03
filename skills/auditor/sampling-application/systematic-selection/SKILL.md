---
name: systematic-selection.md
description: Use when the agent is applying systematic selection to an audit sample, calculating the sampling interval and selecting every nth item, choosing a random start, handling ordered or sorted populations, evaluating whether a pattern in the population could bias systematic selection, or documenting the systematic method and its safeguards.
---

# Systematic Selection

Systematic selection selects every nth item from a population after a random start, where n is the sampling interval (population size divided by sample size). It is widely used because it spreads the sample evenly across the population, is simple to apply, and ensures coverage of all parts of the population. Its distinctive risk is that if the population has a periodic pattern that aligns with the interval, the sample can be biased. The discipline is to apply systematic selection where the population has no aligning pattern (or after a deliberate sort), to use a genuine random start, and to recognise that systematic selection is generally nonstatistical unless the population is in random order.

## Core Rules

### Calculate the interval from a defined population and sample size

Systematic selection requires a defined interval:

- interval = population size / sample size;
- the population must be completely defined and counted (or its monetary total known for monetary-unit systematic sampling);
- the sample size must be determined before selection.

The interval is the structural backbone of the method. An undefined or miscounted population produces an incorrect interval, which distorts the selection. Reconcile the population count or total before calculating the interval.

### Use a genuine random start within the first interval

The starting point determines which items are selected. To avoid selection bias:

- select the start randomly from within the first interval (e.g., between 1 and n);
- use a random number generator, not a convenient or round number;
- document the start and how it was chosen.

A non-random start (e.g., always starting at item 1, or at a round number) defeats the purpose of the random start and can bias the sample toward a particular segment of the population. The random start is what makes systematic selection defensible.

### Check for and address periodic patterns in the population

The central risk of systematic selection is that a periodic pattern in the population aligns with the interval, causing every selected item to share a characteristic:

- transactions processed in batches of a size that divides the interval;
- a recurring cycle (weekly, monthly) whose length aligns with the interval;
- a sort order that clusters similar items at a regular cadence.

Before applying systematic selection, examine the population structure. If a pattern exists and aligns with the interval:

- change the interval slightly, or choose a different sample size;
- sort the population randomly before selection;
- use random selection instead.

Silent alignment of the interval with a population pattern is a bias that is invisible in the selected items but distorts the sample.

### Consider the effect of sort order on coverage

Systematic selection spreads the sample evenly across the population in the order presented. The sort order therefore shapes coverage:

- sorted chronologically, the sample covers the full period evenly (often desirable);
- sorted by value, the sample covers the value range evenly;
- sorted by a meaningful attribute (location, customer, account), the sample covers that attribute evenly, which may or may not be desired.

Choose the sort order deliberately. Random sort order makes systematic selection approximate random selection; a meaningful sort order turns systematic selection into a form of stratified coverage. An accidental or unconsidered sort order can bias the sample.

### Recognise the statistical status of systematic selection

Systematic selection is generally treated as nonstatistical, because the items are not independent (selecting one determines the others). However:

- if the population is in random order, systematic selection approximates random selection and can be treated as statistical;
- if the population is sorted or patterned, the sample is nonstatistical.

Match the conclusion to the statistical status. If statistical projection is required, use random selection, or justify treating systematic selection as statistical based on the population being in random order.

### Handle the end of the population and partial intervals correctly

When the population size is not an exact multiple of the interval, the last interval is partial:

- the standard approach is to wrap around (continue counting from the start of the population), so the sample size is maintained;
- document the wrap-around so it is clear and reproducible;
- confirm the final count of selected items matches the planned sample size.

Failing to handle the partial interval can leave the sample short or create confusion about which items were selected. The wrap-around is the conventional, clean solution.

### Document the method for reproducibility

Systematic selection must be reproducible. Document:

- the population definition and its reconciliation;
- the sampling unit;
- the sample size and the calculated interval;
- the random start and how it was chosen;
- the sort order of the population;
- the resulting selected items (including any wrap-around).

Another auditor, given the population in the documented sort order, the interval, and the start, should be able to regenerate the same sample.

### Combine with stratification or 100% examination of key items

Systematic selection of the whole population is often combined with:

- 100% examination of key items (large, high-risk);
- stratification of the residual population, with systematic selection within each stratum.

This concentrates effort on the highest-risk items and uses systematic selection to spread coverage within each stratum. Document the stratification and the selection method per stratum.

## Common Traps

- **Using a non-random start** (item 1, a round number, a convenient position), defeating the random start safeguard.
- **Failing to check for periodic patterns** that align with the interval, creating invisible bias.
- **Applying systematic selection to a sorted population** without recognising the sort order shapes coverage, or treating it as random when it is not.
- **Treating systematic selection as statistical without confirming the population is in random order**.
- **Not handling the partial interval at the end of the population**, leaving the sample short or selection ambiguous.
- **Calculating the interval from an unreconciled population count or total**, distorting the selection.
- **Assuming even coverage equals unbiased coverage** — even coverage is only unbiased if the population has no aligning pattern.
- **Failing to document the start, interval, and sort order**, making the selection irreproducible.

## Self-Check

- Did I calculate the interval from a fully defined and reconciled population and a pre-determined sample size?
- Did I use a genuine random start within the first interval, documented and reproducible?
- Did I examine the population for periodic patterns that could align with the interval, and address any found?
- Is the sort order of the population deliberate, and do I understand how it shapes coverage?
- Am I treating the sample as statistical or nonstatistical consistent with whether the population is in random order?
- Did I handle the partial interval at the end of the population (typically via wrap-around) to maintain the sample size?
- Is the method (population, unit, size, interval, start, sort order, selected items) documented so the selection is reproducible?
- Did I combine systematic selection with 100% examination of key items or stratification where appropriate?
