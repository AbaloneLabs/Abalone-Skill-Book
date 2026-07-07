---
name: sample_selection_methods.md
description: Use when the agent is choosing among audit sampling selection methods including random systematic haphazard and judgmental selection, determining which selection approach is appropriate for the sampling objective and population characteristics, evaluating whether selection methods produce representative samples, or justifying the selection method against the requirements of statistical and nonstatistical sampling.
---

# Sample Selection Methods

The method by which items are selected for testing is as important as the sample size. A sample that is selected for convenience, or that systematically excludes portions of the population, cannot support a conclusion about that population regardless of how large it is. The recurring failure is defaulting to judgmental or haphazard selection without considering whether it produces a representative sample, or applying systematic selection without checking for patterns in the population that could bias the results. The selection method must match the sampling objective and the population's characteristics, and it must be defensible against the charge that it was designed to avoid finding exceptions. A representative sample is the foundation on which projection and conclusion rest; without it, the sampling effort is wasted.

Use this skill when choosing a sample selection method, when determining whether a method produces a representative sample, and when justifying the selection method for statistical or nonstatistical sampling. The goal is a selection method that is appropriate, representative, and defensible.

## Core Rules

### Match The Selection Method To The Sampling Objective

Different sampling objectives call for different selection methods. Statistical sampling, which supports formal projection and quantified sampling risk, requires methods that give every item a known probability of selection. Nonstatistical sampling, which supports judgmental conclusions, permits more flexible methods but still requires that the sample be representative.

Match by:

- for statistical sampling, using random or systematic selection with a random start, which give every item a known probability of selection;
- for nonstatistical sampling, using haphazard or judgmental selection, provided the sample is designed to be representative;
- recognizing that the choice of method affects what conclusions can be drawn and how sampling risk is controlled;
- ensuring the method is documented and consistently applied.

The method must be chosen deliberately, not defaulted to convenience.

### Use Random Selection For Statistical Sampling

Random selection gives every sampling unit in the population an equal or known probability of selection. It is the foundation of statistical sampling and supports formal projection and quantified sampling risk. Random selection can be performed using random number generators or equivalent tools.

Use by:

- defining the sampling unit and the population clearly;
- generating random numbers and matching them to population items using a consistent mapping;
- ensuring every item has a chance of selection, with no systematic exclusion;
- documenting the random seed or method for reproducibility.

Random selection is the appropriate method when statistical projection and risk quantification are required.

### Use Systematic Selection With A Random Start

Systematic selection picks items at a fixed interval after a random start. It is efficient and, with a random start, can approximate random selection provided the population is not structured in a pattern that aligns with the interval.

Use by:

- calculating the interval as population size divided by sample size;
- selecting a random start within the first interval;
- picking every nth item thereafter;
- verifying that the population is not structured in a pattern that aligns with the interval, which could bias the sample.

Systematic selection is efficient and widely used, but the random start and pattern check are essential.

### Use Haphazard Selection For Nonstatistical Sampling

Haphazard selection picks items without a conscious bias, aiming for a sample that feels representative. It is appropriate for nonstatistical sampling but does not support formal projection. The auditor must ensure that haphazard selection is genuinely without pattern and does not systematically exclude difficult, old, or inconvenient items.

Use by:

- selecting items across the entire population, not just a convenient subset;
- avoiding patterns, such as always selecting from the top of a list or the same period each time;
- ensuring no systematic exclusion of items based on size, age, or location;
- documenting the selection approach and the items selected.

Haphazard selection requires discipline to avoid unconscious bias toward convenient items.

### Avoid Judgmental Selection That Undermines Representativeness

Judgmental selection directs testing to specific items based on risk, size, or other characteristics. It is appropriate for targeted testing of high-value or high-risk items but does not support a conclusion about the population as a whole. Judgmental selection must not be presented as if it supported projection.

Avoid by:

- using judgmental selection only for targeted testing of specific high-risk or high-value items;
- not projecting the results of judgmental selection to the broader population;
- combining judgmental selection of key items with random or systematic selection of the remainder where a population conclusion is needed;
- clearly documenting which items were judgmentally selected and why.

Judgmental selection has a role, but it must not be conflated with representative sampling.

### Ensure The Sample Is Representative Of The Population

Regardless of the method, the sample must be representative of the population. A sample that systematically excludes portions of the population, or that over-represents one segment, cannot support a conclusion about the whole.

Ensure by:

- defining the population completely and ensuring the sampling frame matches it;
- verifying that the selection method does not systematically exclude items based on size, age, location, or other characteristics;
- where the population is heterogeneous, considering stratification to ensure each segment is represented;
- comparing the sample's characteristics to the population to confirm representativeness.

A representative sample is the foundation of a defensible sampling conclusion.

### Handle Special Items And Strata Appropriately

Some populations contain key items, such as individually significant transactions, that should be tested 100 percent and not included in the sample. Other populations are heterogeneous and benefit from stratification into more homogeneous subpopulations. The selection method must account for these.

Handle by:

- identifying key items that warrant 100 percent testing and removing them from the sampling population;
- stratifying the remaining population into homogeneous strata where appropriate;
- applying the selection method within each stratum;
- combining the results across strata for the overall conclusion.

Proper handling of key items and strata improves both efficiency and the representativeness of the sample.

## Common Traps

### Defaulting To Judgmental Or Haphazard Selection Without Justification

Convenience-based selection does not support a population conclusion. The method must be chosen deliberately and matched to the objective.

### Applying Systematic Selection Without A Random Start

Without a random start, systematic selection can introduce bias, particularly if the population is structured.

### Ignoring Population Patterns That Align With The Interval

If the population is structured in a pattern that aligns with the systematic interval, the sample can be biased. A pattern check is essential.

### Systematically Excluding Difficult Or Inconvenient Items

Haphazard selection that avoids difficult, old, or remote items is not representative. Discipline is required to avoid unconscious bias.

### Presenting Judgmental Selection As Supporting Projection

Judgmental selection of high-risk items does not support projection to the population. Conflating the two misstates the conclusion.

### Failing To Define The Population Completely

A sample from an incomplete sampling frame cannot support a conclusion about the full population. The frame must match the population.

### Overlooking The Need For Stratification

Heterogeneous populations benefit from stratification. Applying a single method across strata of differing characteristics reduces efficiency and representativeness.

## Self-Check

- Is the selection method matched to the sampling objective, with statistical methods for projection and nonstatistical methods for judgmental conclusions?
- For statistical sampling, is random or systematic selection with a random start used, giving every item a known probability of selection?
- For systematic selection, is a random start used and is the population checked for patterns that align with the interval?
- For nonstatistical sampling, is haphazard selection genuinely without pattern and free of systematic exclusion of difficult or inconvenient items?
- Is judgmental selection used only for targeted high-risk or high-value items, with results not projected to the broader population?
- Is the sample representative of the population, with no systematic exclusion based on size, age, location, or other characteristics?
- Are key items identified for 100 percent testing and removed from the sampling population, with stratification applied where the population is heterogeneous?
- Is the population defined completely, with the sampling frame matching the population?
- Could an independent reviewer confirm that the selection method is appropriate, representative, and defensible against the charge of convenience-based selection?
- Is the method documented, including the approach, the random seed or start where applicable, and the items selected?
