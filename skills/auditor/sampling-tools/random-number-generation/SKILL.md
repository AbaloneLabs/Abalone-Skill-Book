---
name: random_number_generation.md
description: Use when the agent is generating random numbers for audit sampling, ensuring reproducible random selection, controlling and documenting the random seed, setting up systematic selection with a fixed interval and start point, avoiding selection bias, or documenting the randomization method so a sample can be regenerated and defended.
---

# Random Number Generation

Random number generation is the mechanism that makes an audit sample defensible, but it is also where bias and non-reproducibility quietly enter. A sample generated with an unrecorded seed, a non-uniform generator, or a start point chosen by eye can never be regenerated, and a sample that cannot be regenerated cannot be shown to be unbiased. Agents often reach for whatever random function is at hand, assume it is "random enough," and move on, treating the selection step as trivial compared to the testing. The harm this prevents is a sample whose selection method is opaque, biased, or unreproducible, which can invalidate the entire test and leave the conclusion unsupported if the selection is ever challenged.

Use this skill whenever random or systematic selection is used to draw an audit sample, and whenever the selection method must be documented or defended. The goal is a selection that is uniformly random or systematically unbiased, fully reproducible, and documented well enough that another auditor could regenerate the identical sample.

## Core Rules

### Use A Documented, Reproducible Random Source

Not all random functions are suitable for audit sampling. Spreadsheet volatile functions, unseeded library calls, and human-chosen numbers are not reproducible and are not defensible. Use a source whose output can be regenerated from a recorded seed.

Acceptable practice includes:

- a seeded pseudo-random generator with the seed recorded;
- a published random number table with the start position recorded;
- a system audit module that logs the seed and selection;
- a documented algorithm with version and parameters;
- rejection of any source that changes on recalculation.

### Record The Seed And Selection Parameters

Reproducibility requires that every input that affects the selection is captured. Without the seed, the same software and the same population will produce a different sample each time.

For every random selection, record:

- the random seed or table start position;
- the generator or function name and version;
- the population file and its version or hash;
- the sample size and selection method;
- the mapping between random numbers and population items;
- the operator and date of selection.

### Ensure Every Unit Has A Known Selection Probability

A defensible sample requires that every population item has a known, non-zero chance of selection. Methods that silently skip certain items, or that give some items no chance, bias the sample.

Verify that:

- every population item is eligible for selection;
- no filter removes items after the frame is fixed;
- systematic intervals do not align with a population cycle;
- monetary-unit selection gives larger items proportionally higher odds;
- items with zero or negative values are handled deliberately;
- the selection probability of each item is known and documented.

### Set Up Systematic Selection Carefully

Systematic selection is efficient but carries a specific risk: if the population has a repeating pattern whose period matches the interval, the sample becomes biased. The interval and start point must be chosen to avoid this.

For systematic selection:

- compute the interval as population size divided by sample size;
- choose the start point randomly within the first interval;
- confirm the population is not sorted by a cyclical field;
- shuffle or randomize the population order if a cycle is suspected;
- record the interval and start point for reproducibility;
- verify the selected count equals the planned sample size.

### Avoid Selection Bias At Every Step

Bias can enter through the generator, the population ordering, the handling of edge cases, or the auditor's discretionary substitutions. Each must be controlled explicitly.

Guard against bias by:

- never substituting a selected item for a more convenient one;
- handling items that cannot be located through a documented rule;
- not re-running the selection because the result looked unfavorable;
- randomizing population order before systematic selection;
- treating every selected item as mandatory to test;
- escalating any inability to test a selected item as a scope limitation.

### Document The Method So The Sample Can Be Regenerated

The documentation is the proof that the selection was random and unbiased. It must be detailed enough that another auditor, given the same population and parameters, regenerates the identical sample.

The documentation should include:

- the selection method and its parameters;
- the seed or start point;
- the population version and ordering;
- the mapping from random numbers to items;
- the full list of selected items with keys;
- any deviations and their documented handling.

### Reconcile The Selected Sample To The Population

After selection, confirm that the sample is a valid draw from the intended population. This catches errors where the wrong column was used, the population changed, or the selection over- or under-shot.

Reconcile by:

- confirming every selected item exists in the population;
- confirming no selected item is duplicated;
- confirming the sample size matches the plan;
- confirming the sample total is plausible relative to the population;
- confirming no out-of-scope items were selected;
- investigating any mismatch before testing begins.

### Lock The Population Before Generating Numbers

Random numbers are only meaningful relative to a fixed population. If the population changes between generation and testing, the selection no longer corresponds to the frame that was sampled.

Lock the population by:

- fixing the population file and recording its hash;
- generating the sample only after the population is locked;
- re-generating the sample if the population is refreshed;
- documenting the population version tied to each sample;
- preventing edits to the population during selection.

### Handle Unavailable Or Untestable Selected Items Formally

Sometimes a selected item cannot be located, has been destroyed, or falls outside the testing scope. The way these cases are handled determines whether the sample stays unbiased. An ad hoc substitution reintroduces bias; a documented rule preserves defensibility.

Handle such items by:

- treating the inability to test as a finding, not an inconvenience;
- attempting to obtain the item through formal request;
- documenting the reason the item cannot be tested;
- applying a pre-defined rule (e.g., treat as a deviation, or replace with the next random item);
- never choosing a replacement by judgment or convenience;
- escalating persistent unavailability as a scope or limitation issue.

## Common Traps

### Using A Volatile Or Unseeded Random Function

Spreadsheet functions that recalculate on every open produce a different sample each time and cannot be reproduced. Always use a seeded, stable generator and record the seed.

### Forgetting To Record The Seed

Without the seed, the most rigorous generator is useless for reproducibility. A sample that cannot be regenerated cannot be shown to be unbiased. Record the seed every time, without exception.

### Aligning The Systematic Interval With A Population Cycle

If invoices are sorted by day of week or by branch, and the interval matches that cycle, the sample becomes biased toward one slice. Randomize the order or confirm no cycle before using systematic selection.

### Substituting Selected Items For Convenience

Replacing a hard-to-test selected item with an easier one introduces exactly the bias that randomization was meant to remove. Test every selected item or document a formal scope limitation.

### Re-Running The Selection Until The Sample Looks Good

Generating sample after sample until a "nice" set appears is selection bias dressed up as rigor. Run the selection once, record the seed, and accept the result.

### Ignoring Items With Zero Or Negative Values

In monetary-unit sampling, zero and negative items get no selection probability and must be handled deliberately, often tested separately. Ignoring them silently excludes a part of the population.

### Assuming Random Means Representative

A random sample is unbiased in expectation, but any single draw can over- or under-represent a segment by chance. Do not claim the sample is representative; claim it is unbiased and evaluate accordingly.

### Losing The Link Between Numbers And Population Items and editing The Population After The Sample Is Drawn

If the mapping from random numbers to population rows is not recorded, the selected items cannot be verified. Keep the mapping explicit and tied to a stable key.

Sorting, filtering, or re-extracting the population after selection breaks the correspondence between the random numbers and the items they selected. Lock the population before generating numbers and never edit it afterward without regenerating the sample.

## Self-Check

- Is the random source seeded, stable, and reproducible rather than volatile or human-chosen?
- Are the seed, generator version, population version, and operator recorded for every selection?
- Does every population item have a known, non-zero selection probability under the method used?
- For systematic selection, is the interval computed correctly and the start point randomized?
- Has the population been checked for cyclical ordering that could bias systematic selection?
- Are selected items treated as mandatory, with no convenience substitutions?
- Was the selection run once and accepted, without re-running for a favorable result?
- Are zero, negative, or edge-case items handled deliberately and documented?
- Does the documentation allow another auditor to regenerate the identical sample?; has the selected sample been reconciled to the population for existence, duplicates, and size?
- Was the population locked and hashed before the random numbers were generated?; are selected items that cannot be tested handled through a documented rule rather than ad hoc substitution?
