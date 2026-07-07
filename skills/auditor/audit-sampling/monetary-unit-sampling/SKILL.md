---
name: monetary_unit_sampling.md
description: Use when the agent is applying monetary unit sampling or probability proportional to size sampling in a financial statement audit, designing MUS samples for tests of details, determining sample sizes and sampling intervals based on tolerable misstatement and expected misstatement, evaluating misstatements found in MUS including taintings and projection, or deciding when MUS is the appropriate sampling approach versus classical variables sampling.
---

# Monetary Unit Sampling

Monetary unit sampling (MUS), also called probability proportional to size (PPS) sampling, is a statistical sampling method widely used in tests of details for overstatement. It selects sampling units based on monetary value, so that larger-dollar items have a greater probability of selection. MUS is efficient for populations where few misstatements are expected and where the audit objective is to test for overstatement. The recurring failure is applying MUS without understanding its assumptions and limitations, or mis-evaluating the misstatements found by failing to account for tainting and projection correctly. MUS is a powerful tool, but its results, particularly when misstatements are found, require careful evaluation. Misapplying MUS, or using it where its assumptions do not hold, can produce misleading conclusions about whether the population is fairly stated.

Use this skill when applying monetary unit sampling in tests of details, when designing MUS samples, and when evaluating misstatements found using MUS. The goal is correct application of MUS, with appropriate sample sizes, proper handling of tainting and projection, and conclusions that reflect the method's assumptions and limitations.

## Core Rules

### Understand When MUS Is Appropriate

MUS is not appropriate for every sampling situation. It is efficient for testing overstatement in populations where few misstatements are expected, where the population has a large number of items, and where individual items are not extremely large relative to the sampling interval. It is less appropriate for testing understatement, for populations where many misstatements are expected, or where the objective requires classical variables techniques.

Determine appropriateness by:

- confirming the audit objective is to test for overstatement, which MUS handles efficiently;
- assessing whether few misstatements are expected, since MUS efficiency declines as expected misstatement rises;
- considering whether the population has many items, since MUS assumes a large population;
- recognizing that MUS, as typically applied, is less effective for understatement testing, which may require directional or classical approaches.

Using MUS where its assumptions do not hold produces inefficient or misleading results.

### Determine The Sample Size Based On Risk And Misstatement Parameters

MUS sample size is driven by the tolerable misstatement, the expected misstatement, the assessed risk of incorrect acceptance, and the population value. These parameters must be set deliberately, not defaulted.

Determine by:

- setting tolerable misstatement consistent with the overall materiality and performance materiality decisions;
- estimating expected misstatement based on prior experience and risk assessment;
- selecting the risk of incorrect acceptance consistent with the assessed risk;
- calculating the sample size using the appropriate formula or table, with the population value as an input.

Defaulting sample size to a round number, without basing it on the parameters, undermines the statistical basis of the method.

### Calculate The Sampling Interval And Select Items

In MUS, the sampling interval is the population value divided by the sample size. Selection proceeds by choosing a random start and then selecting every monetary unit at the interval, with the logical unit containing the selected monetary unit tested.

Calculate and select by:

- computing the interval as population value divided by sample size;
- selecting a random start within the first interval;
- identifying the logical unit containing each selected monetary unit;
- testing each selected logical unit in full, even if its value exceeds the interval;
- recognizing that items larger than the interval are selected with certainty and should be tested 100 percent.

Correct interval calculation and selection are essential to the statistical validity of the method.

### Evaluate Misstatements Using Tainting And Projection

When misstatements are found in MUS, they must be evaluated using the method's specific projection logic. Each misstated item carries a tainting, the percentage by which it is misstated, and the projected misstatement is the tainting applied to the interval.

Evaluate by:

- for each misstated item, calculating the tainting as the misstatement divided by the item's recorded value;
- for items smaller than the interval, projecting the tainting across the interval to estimate the population misstatement;
- for items larger than the interval, using the actual misstatement, since these items are tested 100 percent;
- summing the projected misstatements and comparing to tolerable misstatement, adjusted for sampling risk.

Failing to apply tainting and projection correctly produces a misleading estimate of population misstatement.

### Account For The Risk Of Incorrect Acceptance And The Precision Interval

MUS conclusions account for sampling risk through a precision interval or upper limit. Even if the projected misstatement is below tolerable, the upper limit, which accounts for sampling risk, must also be below tolerable for the population to be accepted.

Account by:

- calculating the upper limit on misstatement, which includes the projected misstatement plus an allowance for sampling risk;
- comparing the upper limit to tolerable misstatement;
- accepting the population only if the upper limit is below tolerable misstatement;
- where the upper limit exceeds tolerable, expanding the sample, adjusting the population, or qualifying the conclusion.

Comparing only the projected misstatement to tolerable, without the upper limit, understates the sampling risk.

### Handle Zero And Negative Balances Correctly

MUS, as typically applied, selects based on positive monetary value. Zero and negative balances are not selected by the standard method and require separate consideration, since they may be misstated in ways the sample would not catch.

Handle by:

- identifying zero and negative balances in the population;
- testing them through separate procedures, such as analytical review or targeted testing;
- considering whether the exclusion of zero and negative balances leaves a gap in coverage;
- documenting the separate procedures performed.

Ignoring zero and negative balances leaves a portion of the population untested.

### Consider The Effect Of Found Misstatements On The Conclusion

When misstatements are found, the conclusion must reflect both the projected amount and the increased sampling risk. Finding misstatements may indicate that the population is not as clean as expected, which can affect the risk assessment and the planned reliance.

Consider by:

- assessing whether the found misstatements indicate a control deficiency or a higher-than-expected error rate;
- considering whether the risk assessment and planned procedures remain appropriate;
- where the upper limit exceeds tolerable, deciding among expansion, adjustment, or qualification;
- documenting the evaluation and the basis for the conclusion.

Found misstatements are not merely projected; their implications for risk and controls must be considered.

## Common Traps

### Applying MUS Where Its Assumptions Do Not Hold

MUS assumes few misstatements, a large population, and an overstatement objective. Applying it elsewhere produces inefficient or misleading results.

### Defaulting Sample Size Without Parameter Basis

Sample size must be based on tolerable misstatement, expected misstatement, and risk. Defaulting to a round number undermines the statistical basis.

### Failing To Apply Tainting And Projection Correctly

Each misstatement's tainting must be projected across the interval. Incorrect projection produces a misleading estimate.

### Comparing Projected Misstatement Without The Upper Limit

The upper limit, accounting for sampling risk, must be below tolerable. Comparing only projected misstatement understates risk.

### Ignoring Zero And Negative Balances

Zero and negative balances are not selected by standard MUS and require separate procedures. Ignoring them leaves gaps in coverage.

### Treating Found Misstatements As Isolated

Found misstatements may indicate control deficiency or higher error rates. Their implications for risk and controls must be considered.

### Using MUS For Understatement Testing

MUS, as typically applied, is less effective for understatement. Understatement objectives may require directional or classical approaches.

## Self-Check

- Is MUS confirmed as appropriate for the objective, with few expected misstatements, a large population, and an overstatement focus?
- Is the sample size based on tolerable misstatement, expected misstatement, assessed risk, and population value, not defaulted to a round number?
- Is the sampling interval calculated correctly, with a random start and selection of the logical unit containing each selected monetary unit?
- Are items larger than the interval identified and tested 100 percent?
- Are misstatements evaluated using tainting and projection, with items smaller than the interval projected and items larger than the interval taken at actual misstatement?
- Is the upper limit on misstatement, accounting for sampling risk, calculated and compared to tolerable misstatement?
- Is the population accepted only if the upper limit is below tolerable misstatement?
- Are zero and negative balances identified and tested through separate procedures?
- Where misstatements are found, are the implications for risk, controls, and the conclusion considered and documented?
- Could an independent reviewer confirm that MUS is applied correctly, with appropriate sample sizes, proper tainting and projection, and conclusions reflecting the method's assumptions?
