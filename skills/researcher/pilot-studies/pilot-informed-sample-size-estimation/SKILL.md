---
name: pilot_informed_sample_size_estimation.md
description: Use when the agent is using pilot data to estimate the main study sample size, deriving variance or recruitment and retention rates from a small pilot, setting effect size bounds from pilot estimates, or judging how much weight to place on unstable small-sample inputs when powering a definitive trial.
---

# Pilot Informed Sample Size Estimation

A pilot can inform the sample size of the main study, but only for quantities that are stable enough to estimate from a small sample, and only with explicit caution. The recurring failure is to take a noisy pilot estimate, a variance, a recruitment rate, or an effect size, and plug it directly into a sample size calculation as if it were the truth. Small samples produce unstable variance estimates, recruitment rates that swing widely with a few events, and effect estimates that are biased toward the extremes. When any of these feed an underpowered or overpowered main study, the consequence is a definitive trial that either cannot detect a real effect or wastes resources chasing an effect that was never there.

The judgment problem is that pilot data feels concrete and local, so it is over-trusted. Agents often prefer the pilot's own numbers over published estimates or expert elicitation because they seem more relevant, then ignore how wide the uncertainty around them really is. The harm this skill prevents is a main study sized on an illusion of precision, which can mislead funders, expose participants to a trial that cannot conclude, or set up a false expectation of effect. The agent has latitude in which inputs to use, but must quantify and hedge the instability of every pilot-derived input.

This skill offers methodological guidance for translating pilot data into defensible sample size estimates. It is not a substitute for statistical consultation, and sample size decisions based on misread pilot data can mislead the main study and waste participants' contributions.

## Core Rules

### Decide What The Pilot Can Legitimately Inform

Not every sample size input is well estimated by a pilot. Separate the inputs the pilot can usefully inform from those it cannot, and state this explicitly.

Usually reasonable to estimate from a pilot:

- recruitment and screen-fail rates, with wide intervals;
- retention and attrition rates;
- measurement variance and standard deviation, cautiously;
- missing data patterns and their direction;
- cost and burden per enrolled participant.

Usually unstable or misleading from a small pilot:

- the effect size of the intervention, which is biased and noisy;
- the minimum clinically or practically important difference, which is a judgment, not an estimate;
- subgroup effects, which a pilot cannot support;
- correlations assumed in clustered or repeated-measures designs, unless the pilot is sized for them.

Anchor the main study's target effect on the minimum meaningful difference and external evidence, not on the pilot's observed effect.

### Never Treat The Pilot Effect Size As The True Effect

The observed effect in a small pilot is one draw from a wide distribution. It is often inflated because pilots that show a large effect are more likely to proceed, and shrunken or reversed in others. Using it as the expected effect for the main study sets up a power calculation built on noise.

Instead, use the pilot effect only to set a plausible range or bound, and combine it with external evidence and the minimum meaningful difference. Report the pilot effect with its wide confidence interval and treat the interval, not the point estimate, as the informative quantity. If the pilot effect is implausibly large, suspect instability rather than breakthrough.

### Use Pilot Variance Cautiously And Adjust For Instability

Variance estimates from small samples are themselves highly variable and tend to be underestimated, which inflates apparent power. A standard deviation from a pilot of fifteen or thirty participants can be far from the truth, and using it naively can underpower the main study.

Apply caution explicitly:

- inflate the pilot standard deviation using a correction factor based on the pilot sample size;
- use the upper confidence limit of the variance rather than the point estimate for a conservative calculation;
- compare the pilot variance with published or external estimates and investigate discrepancies;
- run sensitivity analyses across a range of plausible variances;
- document which variance value was chosen and why.

A slightly larger main study sized on a conservative variance is far cheaper than a definitive trial that fails to detect a real effect.

### Estimate Recruitment And Retention With Intervals, Not Points

Recruitment and retention rates from a pilot are proportions based on few events, so their uncertainty is large. Reporting "we retained eighty percent" from a pilot of ten participants conveys false precision.

Estimate each rate as a proportion with a confidence interval, and use the lower bound for planning. Translate the rate into concrete numbers, such as participants per site per month and expected screen-fail ratio, and test whether the main study's timeline and sites can sustain the pessimistic bound. Build attrition into the sample size inflation rather than treating it as an afterthought.

### Separate Internal And External Pilots For This Purpose

An internal pilot is embedded in the main trial and its data may, under strict pre-specified rules, contribute to a re-estimation of variance or sample size. An external pilot stands alone and its data should not be pooled into the main analysis without strong justification. The sample size logic differs between them.

For an internal pilot, pre-specify the re-estimation method, whether blinded or unblinded, and the rules that govern it, to protect the type I error rate. For an external pilot, use its data only as planning input and keep its participants separate from the main analysis unless a justified, pre-specified pooling rule applies.

### Set Effect Size Bounds, Then Decide Power Against Them

Rather than powering against a single pilot-derived point estimate, define a range of effects of interest and examine power across it. This protects against over-committing to one noisy number.

Define:

- the minimum meaningful difference that would justify changing practice or belief;
- an optimistic bound informed by external evidence and the pilot's upper confidence limit;
- a pessimistic or null-adjacent bound below which the study would not be worth running.

Size the study to detect the minimum meaningful difference with adequate power, and report power across the range so decision-makers see the tradeoffs. Avoid sizing to the pilot's observed effect, which may sit anywhere in that range by chance.

### Propagate Pilot Uncertainty Into The Calculation

A sample size computed from point estimates hides the uncertainty in those estimates. Make the uncertainty visible by reporting how the required sample size changes as each pilot-derived input varies.

Produce a table or set of scenarios showing required sample size under, for example, the pilot variance point estimate, its upper confidence limit, and an external benchmark, crossed with low and high attrition assumptions. If small changes in inputs produce large changes in required sample size, treat the calculation as fragile and lean conservative.

### Document Every Assumption And Its Source

A defensible sample size calculation names where each number came from and why. This lets reviewers and future teams judge and update it.

For each input record:

- the value used and its source, whether pilot, external literature, expert elicitation, or regulatory guidance;
- the pilot sample size and any correction applied;
- the alternative values considered and why the chosen one was selected;
- the attrition assumption and its basis;
- the alpha, power, and design parameters, such as clustering or repeated measures.

### Plan For Re-Estimation When The Pilot Is Internal

If the pilot is internal, decide in advance whether and how sample size will be re-estimated as the main trial accrues, and protect against bias from that re-estimation.

Pre-specify the re-estimation trigger, the method, who is blinded, and how the overall type I error is controlled. Avoid unblinded effect-based re-estimation that inflates false positives. Prefer blinded variance re-estimation with an independent statistician, and document the rule in the statistical analysis plan before it is used.

## Common Traps

### Powering The Main Study On The Pilot's Observed Effect

The pilot effect is noisy and often inflated, so powering against it can produce an over- or under-powered trial. The trap is treating the local number as more trustworthy than external evidence. Anchor on the minimum meaningful difference and treat the pilot effect as one weak input.

### Using Pilot Variance As If It Were Exact

Small-sample variance is unstable and often too low, which silently inflates power. The trap is plugging the pilot standard deviation straight into the formula. Inflate it, use the upper confidence limit, and run sensitivity analyses.

### Reporting Recruitment Rates As Single Numbers

A retention or recruitment rate from a small pilot carries wide uncertainty. The trap is quoting eighty percent from ten participants as if it were precise. Report intervals and plan against the lower bound.

### Pooling External Pilot Data Into The Main Analysis

Combining external pilot participants with main-trial participants double-counts early adopters and biases estimates. The trap is doing this quietly to boost numbers. Keep them separate unless a pre-specified, justified pooling rule exists.

### Confusing The Meaningful Difference With The Estimated Effect

The minimum meaningful difference is a judgment about what matters, not something the pilot estimates. The trap is letting the pilot's observed effect redefine what difference the study targets. Set the meaningful difference from clinical, practical, or policy reasoning and external evidence.

### Ignoring Clustering Or Design Effects Not Captured By The Pilot

If the pilot was too small to estimate an intraclass correlation or repeated-measures correlation, assuming one from elsewhere can still mispower a clustered trial. The trap is carrying over a simple-variance pilot estimate into a complex design. Source design parameters explicitly and conservatively.

### Over-Trusting A Conveniently Large Pilot Effect

A big pilot effect feels like good news and lowers the apparent required sample size, which is tempting when resources are tight. The trap is accepting it uncritically. Large effects in small samples are exactly the ones most likely to be noise; scrutinize them hardest.

### Hiding The Fragility Of The Calculation

If the required sample size swings widely with plausible inputs, a single reported number misleads funders. The trap is presenting one clean figure. Show the range of scenarios and lean conservative where the calculation is fragile.

## Self-Check

- [ ] Has each sample size input been classified as legitimately pilot-informable versus unstable, with the effect size anchored on the minimum meaningful difference rather than the pilot's observed effect?
- [ ] Is the pilot effect reported with its wide confidence interval and used only as a bound or weak input, not as the expected effect?
- [ ] Has pilot variance been adjusted for small-sample instability, using a correction factor or upper confidence limit, with sensitivity analyses across a plausible range?
- [ ] Are recruitment and retention rates reported as proportions with intervals and translated into concrete per-site-per-month and screen-fail assumptions, planning against the lower bound?
- [ ] Is attrition built into the sample size inflation with a stated basis, rather than added as an afterthought?
- [ ] Is the distinction between internal and external pilot handled correctly, with internal-pilot re-estimation pre-specified, blinded where possible, and type I error controlled?
- [ ] Does the calculation report power across a range of effects of interest, not just a single pilot-derived point estimate?
- [ ] Is the fragility of the calculation visible, through a scenario table showing how required sample size changes as pilot-derived inputs vary?
- [ ] Is every assumption documented with its source, the pilot sample size, any correction applied, and the alpha, power, and design parameters?
- [ ] For high-stakes trials, costly interventions, or cases where pilot inputs are unstable or conflicting, has a biostatistician reviewed the sample size calculation, the variance corrections, and the sensitivity analyses before the main study is funded or launched?
