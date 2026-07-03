---
name: sample-size-determination.md
description: Use when the agent is determining sample size for tests of details or tests of controls, evaluating the factors that drive sample size (confidence, tolerable misstatement or deviation rate, expected misstatement or deviation, population size), justifying a judgemental sample size, or deciding whether a sample size is large enough to support the intended conclusion.
---

# Sample Size Determination

Sample size is the point where sampling theory meets audit judgement. Too small a sample cannot support the intended conclusion and leaves the opinion exposed; too large a sample wastes effort and signals that the risk assessment was not used to focus the work. The discipline is to derive sample size from the factors that actually drive it — the desired confidence, the tolerable and expected error rates or amounts, and the population characteristics — rather than defaulting to a fixed number, and to justify the size against the conclusion it must support.

## Core Rules

### Derive sample size from the factors that drive it, not from habit

For tests of details, sample size is driven by:

- **Desired confidence level** (inverse of sampling risk): higher confidence requires a larger sample.
- **Tolerable misstatement**: the maximum misstatement the auditor will accept. A smaller tolerable misstatement requires a larger sample.
- **Expected misstatement**: the misstatement the auditor anticipates. A higher expected misstatement requires a larger sample (to distinguish expected from tolerable).
- **Population size**: relevant for small populations; for large populations its effect is limited.
- **Population variability and stratification**: more variable populations, or effective stratification, affect size.

For tests of controls, sample size is driven by:

- **Desired confidence level**.
- **Tolerable deviation rate**: the maximum rate of control failure the auditor will accept.
- **Expected deviation rate**: the rate anticipated; a higher expected rate requires a larger sample.
- **Population size and frequency of control operation**.

State each factor explicitly for every sample. A sample size chosen without reference to these factors is a guess, not a determination.

### Calibrate tolerable misstatement or deviation rate to the specific risk

Tolerable misstatement (details) and tolerable deviation rate (controls) are not constants; they should reflect the risk and materiality of the specific population. A lower tolerable amount or rate — appropriate for a high-risk, material population — drives a larger sample. A higher tolerable amount — appropriate for a lower-risk population — permits a smaller sample. Do not apply a blanket tolerable rate across all populations; calibrate it to the risk assessment so that sample size follows risk.

### Set expected misstatement or deviation rate from evidence, not optimism

The expected rate is a forecast of what the sample will find, and it materially affects size. Set it from:

- prior-year results in this area;
- walkthrough observations and control assessment;
- analytical procedures indicating unusual patterns;
- known issues or changes in the period.

An expected rate set unrealistically low (because the auditor hopes the area is clean) produces a sample too small to detect the actual error rate, leading to a failed sample and re-testing. An expected rate set too high inflates the sample unnecessarily. Use evidence, and where uncertainty is high, set a higher expected rate and accept a larger sample.

### Apply the relationship: tolerable vs expected drives size

The sample size is sensitive to the gap between tolerable and expected error. When the gap is narrow (expected close to tolerable), the sample must be large to distinguish them. When the gap is wide (expected far below tolerable), the sample can be smaller. Two implications:

- If you expect few errors and find many, the sample was undersized relative to reality; the conclusion is unreliable and the population may be misstated.
- Setting expected error conservatively (not optimistically) protects against this, at the cost of a larger sample.

Understand and document this relationship so the size is defensible against the result.

### Account for population size correctly

For large populations, population size has limited effect on sample size in statistical sampling — a sample of a given size provides essentially the same assurance whether the population is 10,000 or 1,000,000. For small populations, population size matters more and the finite population correction reduces the required sample. Do not inflate sample size because a population is large; do not under-size because a population is small. Apply the population-size factor per the methodology.

### Justify judgemental (nonstatistical) sample sizes against the same logic

Nonstatistical sample sizes are not formula-driven, but they are not arbitrary. Justify a judgemental size by reference to the same factors: the risk of the area, the tolerable amount or rate, the expected error, the population variability, and the assurance needed. A nonstatistical sample of a fixed number (e.g., 25) is defensible only if the factors support that number for this population; applying 25 everywhere regardless of risk is not. Document the judgement so the size can be challenged.

### Adjust size for stratification and for key-item testing

Where the population is stratified (high-value items examined 100%, residual sampled), the sample size applies to the residual stratum and can be smaller because the highest-risk items are fully covered. Make the stratification explicit and size each stratum appropriately: 100% for the key-item stratum, a risk-based sample for the residual. Effective stratification is one of the most powerful ways to reduce sample size without reducing assurance.

### Re-size when circumstances change

If, during testing, the expected error rate proves higher than planned, or a population characteristic differs from assumption, the original sample size may no longer support the intended conclusion. Re-size and extend the sample, or switch to a substantive approach that does not rely on the sample. Do not cling to the original size when the assumptions underlying it have been disproven.

### Ensure the size supports the intended conclusion

The ultimate test of sample size is whether it can support the conclusion drawn. For a high-risk, material assertion, the sample must be large enough that finding no errors (or few) genuinely supports reliance, and that finding errors would be detected. For a lower-risk assertion, a smaller sample may suffice. Ask, for each sample: if the population were materially misstated, would this sample size have a reasonable chance of detecting it? If not, the size is too small regardless of what the formula produced.

## Common Traps

- **Defaulting to a fixed sample size** (e.g., always 25) regardless of risk, tolerable error, or expected error.
- **Setting expected error optimistically low** to keep the sample small, then finding more errors than expected and being unable to conclude.
- **Applying a blanket tolerable rate across populations** instead of calibrating to the specific risk and materiality.
- **Ignoring the tolerable-vs-expected relationship** and being surprised when a narrow gap requires a large sample.
- **Inflating sample size for large populations** under the false belief that bigger populations always need bigger samples.
- **Justifying judgemental sizes without reference to the driving factors**, making them indefensible.
- **Failing to stratify**, sampling homogeneously across a population with a few very large items, which both wastes effort and under-covers the large items.
- **Clinging to the original sample size when actual errors exceed expected**, instead of extending the sample or changing approach.
- **Choosing a size that cannot realistically detect a material misstatement**, leaving the conclusion exposed if challenged.

## Self-Check

- For each sample, did I state the driving factors — confidence, tolerable misstatement or deviation rate, expected error, population size, variability?
- Did I calibrate tolerable misstatement or deviation rate to the specific risk and materiality of this population, not apply a blanket rate?
- Did I set the expected error rate from evidence (prior years, walkthroughs, analytics), and conservatively where uncertainty is high?
- Do I understand and document the tolerable-vs-expected relationship and its effect on the size I chose?
- Did I apply the population-size factor correctly — not inflating for large populations, not under-sizing for small ones?
- If the sample is nonstatistical, did I justify the size against the same driving factors rather than using a fixed default?
- Did I stratify where the population has a few large items, examining key items 100% and sizing the residual sample separately?
- If actual results diverge from the expected error assumption, did I re-size or change approach rather than clinging to the original size?
- Does the sample size give a reasonable chance of detecting a material misstatement if one exists — the ultimate test of adequacy?
