---
name: projected-misstatement.md
description: Use when the agent is projecting sample misstatement to the population, calculating projected misstatement and its precision interval, distinguishing statistical vs nonstatistical projection, evaluating whether projected misstatement exceeds tolerable misstatement, or documenting the projection method and the conclusion on the population.
---

# Projected Misstatement

Projected misstatement is the auditor's estimate of the total misstatement in the population, extrapolated from the misstatements found in the sample. It is the bridge between sample results and a conclusion about the population, and it is the basis for deciding whether the population is fairly stated. Its demands are real: the projection method must match the sampling method, statistical and nonstatistical projections differ fundamentally, and the conclusion must compare projected misstatement (and, for statistical sampling, the upper limit) against tolerable misstatement. The discipline is to project rigorously, to distinguish what the sample supports from what it does not, and to draw a defensible conclusion.

## Core Rules

### Match the projection method to the sampling method

Projection must be consistent with how the sample was selected:

- **Monetary-unit sampling (MUS)**: project by the taintings approach or the cell/block method appropriate to MUS; the projection and its precision have a defined statistical form.
- **Classical variables sampling (mean-per-unit, ratio, difference estimation)**: project using the method's formulae; each has its own projection and precision.
- **Nonstatistical sampling**: project by extrapolating the average misstatement rate (or amount) found in the sample to the population, without a statistical precision interval.

A mismatch between sampling method and projection method produces an invalid projection. Document which sampling method was used and apply the matching projection method.

### Distinguish statistical and nonstatistical projection fundamentally

The two approaches support different conclusions:

- **Statistical projection** produces a point estimate and a precision interval (sampling risk), allowing a conclusion at a defined confidence level. The upper misstatement bound is compared to tolerable misstatement.
- **Nonstatistical projection** produces a point estimate only; sampling risk is addressed qualitatively (the auditor judges whether the sample was large enough and the results consistent enough to support the conclusion). There is no statistical upper bound.

Do not present a nonstatistical projection with a precision interval — that precision does not exist. Do not present a statistical projection without its precision — the point estimate alone is not the conclusion.

### Project using the appropriate basis: by rate or by amount

Projection can be on different bases:

- **By item (count basis)**: project the proportion of items with misstatement to the population count; suitable when misstatement is a yes/no attribute.
- **By amount (monetary basis)**: project the monetary misstatement found to the population monetary total; suitable for tests of details of balances.
- **By tainting (MUS)**: project based on the percentage misstatement of each sampled monetary unit, which handles partial overstatements in a defined way.

Choose the basis that matches the assertion and the sampling method. Mixing bases (e.g., projecting a count rate as a monetary amount) produces a meaningless number.

### Compare projected misstatement to tolerable misstatement correctly

The conclusion depends on comparing the projection to tolerable misstatement, but the comparison differs by method:

- **Statistical sampling**: compare the upper misstatement bound (point estimate plus precision) to tolerable misstatement. If the upper bound is below tolerable, the population is acceptable; if above, it is not.
- **Nonstatistical sampling**: compare the projected misstatement to tolerable misstatement, and apply judgement about sampling risk. A projected amount below tolerable may still be unacceptable if sampling risk is high or results are inconsistent.

Comparing only the point estimate to tolerable misstatement, even in statistical sampling, ignores sampling risk and overstates the conclusion's strength.

### Handle the effect of misstatements found

The nature of misstatements found affects the projection and the conclusion:

- **A few isolated, specific errors** may support projection and a conclusion that the population is acceptable (if the upper bound is below tolerable).
- **A systematic or pervasive error pattern** suggests the population is not acceptable regardless of the projected amount, and may require expanding the sample, requesting correction, or modifying the opinion.
- **Anomalous errors** (clearly unusual, not representative) may be projected separately or excluded from projection with documented rationale, but the rationale must be defensible.

Do not mechanically project and conclude. Interpret the pattern of misstatement, not just its amount.

### Address uncorrected and projected misstatement in the conclusion

The conclusion integrates:

- specifically identified misstatements (factual errors found in the sample, on specific items);
- projected misstatement (the estimate for the unsampled population);
- uncorrected misstatement from other sources (other areas, prior periods).

The aggregate is compared to performance materiality (and tolerable misstatement for the specific population). A projected amount that is acceptable in isolation may, when aggregated with other uncorrected misstatements, become material. Evaluate the population in the context of the whole engagement.

### Document the projection method, inputs, and conclusion

The projection must be documented so it is reproducible and reviewable:

- the sampling method and the matching projection method;
- the sample results (items tested, misstatements found, taintings or amounts);
- the projection calculation (point estimate, and precision interval for statistical sampling);
- the comparison to tolerable misstatement (and the upper bound for statistical sampling);
- the conclusion and its basis.

A projection documented only as a number, without method and inputs, cannot be reviewed or defended.

### Recognise when projection is not appropriate

Projection assumes the sample is representative of the population. Projection may not be appropriate, or may need qualification, when:

- the population is not homogeneous (stratification was not applied but should have been);
- the misstatements are anomalous and not representative;
- the sample was judgement-targeted (non-representative by design), in which case projection to the population is not valid — the results apply to the items tested, not the population.

Do not project a judgement-targeted sample to the population. The whole point of targeted testing is that it is not representative; projecting it misrepresents the conclusion.

## Common Traps

- **Mismatching the projection method to the sampling method**, producing an invalid projection.
- **Presenting a nonstatistical projection with a precision interval**, or a statistical projection without its precision.
- **Comparing only the point estimate to tolerable misstatement**, ignoring sampling risk in statistical sampling.
- **Projecting a judgement-targeted sample to the population**, when targeted testing is not representative by design.
- **Mixing projection bases** (count rate projected as monetary amount, or vice versa), producing a meaningless number.
- **Mechanically projecting and concluding without interpreting the pattern of misstatement**, missing systematic or pervasive error.
- **Evaluating projected misstatement in isolation**, without aggregating with uncorrected misstatement from other sources.
- **Excluding anomalous errors from projection without a defensible rationale**, understating the projection; **Documenting only the projected number**, without method, inputs, and the comparison to tolerable misstatement

## Self-Check

- Is the projection method matched to the sampling method (MUS, classical variables, nonstatistical)?
- Am I presenting a statistical projection with its precision interval, or a nonstatistical projection without one — and not confusing the two?
- Is the projection basis (by item, by amount, by tainting) appropriate to the assertion and sampling method?
- For statistical sampling, did I compare the upper misstatement bound (not just the point estimate) to tolerable misstatement?
- Did I interpret the pattern of misstatement (isolated vs systematic vs anomalous), not just the projected amount?
- Did I aggregate projected misstatement with specifically identified and other uncorrected misstatement before concluding?
- If the sample was judgement-targeted, did I avoid projecting it to the population, since targeted testing is not representative?
- Is the projection method, inputs, calculation, comparison, and conclusion documented so the projection is reproducible and defensible?
