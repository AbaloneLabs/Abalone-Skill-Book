---
name: statistical_power_and_sample_size.md
description: Use when the agent is computing experiment sample size, running a power analysis, setting a minimum detectable effect, deciding experiment runtime, or judging whether a planned test is underpowered before launch.
---

# Statistical Power And Sample Size

Power analysis is the quantitative planning that decides whether an experiment can detect the effect it hopes for. A test that is underpowered is not merely slow; it is structurally incapable of answering the question it was built to answer, and its inconclusive result is then misread as evidence of no effect.

The judgment problem is that power feels like a technical detail, so teams skip it when a launch is urgent, or they set the minimum detectable effect to whatever number produces a feasible runtime rather than the smallest effect that would actually matter. The harm is double-edged. Underpowered tests kill valuable features because a non-significant result is treated as proof of failure, and they also waste weeks of traffic on experiments that could never have resolved. Agents tend to plug in optimistic baselines, ignore variance in the metric, forget the multiple-comparison penalty from secondary metrics, and never convert the required sample into a calendar runtime that the team can actually commit to.

Use this skill before approving an experiment runtime, before promising a readout date, and before telling leadership the test will answer the question. The goal is to prevent the launch of experiments that are mathematically doomed, and to force an honest conversation about effect size, traffic, and feasibility before any data is collected.

## Core Rules

### Set The Minimum Detectable Effect To What Matters, Not What Hopes

The minimum detectable effect (MDE) is the smallest change in the primary metric that would justify shipping the feature, and it is the most consequential and most abused input to a power calculation. Setting it to the effect you hope for, rather than the effect that is practically meaningful, produces an experiment that is either impossibly large or dangerously small.

Anchor the MDE to a business decision. Ask what movement in the metric would change behavior, what the cost of the feature is relative to that movement, and what the historical week-to-week noise in the metric looks like. A one percent relative lift might be meaningful on a high-volume conversion and meaningless on a low-volume retention metric. Write the MDE down with its rationale before touching a calculator, because the rationale is what gets challenged, not the number that falls out of it.

### Use The Real Baseline And The Real Variance

Power depends on the baseline rate or mean of the primary metric and on its variance, and both must come from real production data for the actual eligible population, not from a guess or from an aggregate dashboard. A baseline taken from all users when the experiment targets new users will be wrong, and wrong variance understates the sample required.

Pull the baseline and standard deviation from the same logging, the same time window, and the same population the experiment will use. Account for seasonality by sampling across several weeks, and account for the fact that some metrics are skewed or zero-inflated, which inflates variance and demands more traffic. If the metric is a ratio or a funnel step, use the correct variance formula for that metric type, not a generic proportion test.

### Pick Alpha And Power Deliberately

Alpha is the false-positive rate you accept, and power is one minus the false-negative rate. The conventional values of alpha 0.05 and power 0.8 are defaults, not laws, and they encode a tradeoff between the two kinds of error. Lowering alpha or raising power both demand more traffic.

Ask which error is more expensive for this decision. If shipping a harmful feature is costly, lower alpha. If killing a valuable feature is costly, raise power. For guardrail metrics, where you want to detect harm, consider a lower alpha to avoid missing regressions. State the chosen alpha and power in the brief with the reasoning, because the choice should be defensible to a skeptical reviewer, not inherited from a template.

### Convert Required Sample Into Calendar Runtime

Sample size is an abstraction; runtime is the commitment the team makes. Divide the required sample per arm by the daily eligible, exposed traffic per arm, and that is the minimum runtime. Then check it against reality: holidays, campaigns, code freezes, and known traffic dips all distort the experiment and may require extending runtime.

If the minimum runtime exceeds what is feasible, you have four honest options. Raise the MDE to the smallest effect you can still defend, choose a more sensitive metric, accept lower power and say so explicitly, or conclude that an A/B test is not the right method for this question and use a different design. The dishonest option is to run the test for the calendar time available and declare whatever comes back.

### Account For Multiple Comparisons Across Metrics And Segments

Every additional metric and every additional segment you test is another chance to find a false positive by luck. A panel of ten metrics at alpha 0.05 does not have a five percent false-positive rate overall; it has far more. The same applies to slicing results by country, device, or user cohort after the fact.

Control the family-wise error rate when you pre-declare multiple comparisons, using a correction such as Bonferroni or Holm, or designate one primary metric that carries the decision and treat the rest as secondary and exploratory. Pre-declare the segments you intend to report, and treat any segment that becomes interesting only after results are visible as hypothesis-generating, not decision-grade.

### Distinguish Continuous From Proportion Metrics

The sample size formula differs for continuous metrics, such as revenue per user or time on site, and proportion metrics, such as conversion rate or click-through rate. Continuous metrics depend on the standard deviation; proportion metrics depend on the baseline rate. Using the wrong formula produces a sample size that is off by an order of magnitude.

For continuous metrics, confirm the distribution is not so skewed that the mean is unstable, and consider a transformation or a winsorized metric if heavy tails dominate. For proportion metrics, confirm the baseline is not so close to zero or one that the normal approximation breaks down. In both cases, the power calculation is only as good as the inputs, and the inputs must come from real data on the real population.

### Re-Run Power When The Design Changes

Power is not a one-time calculation. If the eligibility rules narrow, the randomization unit changes, the primary metric is redefined, or the MDE is revised, the sample size changes too. Treat the power analysis as a living artifact that is re-run whenever the design moves.

This matters most when a team narrows eligibility to fix a problem mid-experiment, because the narrower population may no longer support the original runtime. Re-running power is not a sign of weakness; it is the discipline that prevents launching an experiment that quietly became underpowered.

## Common Traps

### Setting MDE To Whatever Fits The Calendar

When the required sample for a meaningful effect is too large, teams often raise the MDE until the runtime fits the schedule. The trap is that the experiment can now only detect an effect so large it would have been obvious without a test, so the test adds nothing and its inconclusive result is misread as no effect. Anchor MDE to the decision, then negotiate the calendar.

### Using An Optimistic Baseline

A baseline pulled from a peak week, or from all users instead of the eligible population, understates the required sample. The trap is that the power calculation looks clean but the experiment runs on a noisier population than assumed, so the effective power is far below the reported 0.8. Always use the real baseline for the real eligible population.

### Ignoring Metric Variance

Forgetting the standard deviation on a continuous metric, or assuming a proportion test applies to a skewed revenue metric, produces a sample size that is wrong by orders of magnitude. The trap is that the number looks plausible because it came out of a tool, but the tool was fed the wrong formula. Match the formula to the metric type.

### Forgetting The Multiple-Comparison Penalty

Declaring ten metrics and testing each at alpha 0.05 inflates the overall false-positive rate dramatically. The trap is that the experiment appears thorough while actually being a fishing expedition, and whichever metric wins gets reported as the result. Pre-declare one primary or apply a correction.

### Treating Calendar Runtime As A Substitute For Power

Running the test for as long as the schedule allows and declaring whatever comes back is not a power analysis. The trap is that the team feels disciplined because they ran it to completion, when the completion was arbitrary relative to the sample required. If the required sample is not feasible, say so and choose a different method.

### Skipping Power Because The Launch Is Urgent

Urgency is exactly when power analysis matters most, because an urgent feature cannot afford to wait for a test that will come back inconclusive. The trap is that skipping power feels like speed but actually wastes the entire experiment cycle on a result that could not resolve. A quick power check takes minutes and prevents weeks of wasted traffic.

### Misreading Non-Significance As No Effect

An underpowered test that returns a non-significant result has not shown the feature does not work; it has shown the test could not tell. The trap is reporting "no impact" and killing the feature, when the confidence interval may include large effects in both directions. Report the confidence interval and the MDE, not just the p-value.

## Self-Check

- [ ] The MDE is anchored to a business decision and its rationale is written down before any calculation.
- [ ] The baseline rate or mean comes from real production data for the actual eligible population, sampled across a representative window.
- [ ] The variance or standard deviation used in the calculation matches the metric type and is not assumed from a generic formula.
- [ ] Alpha and power were chosen with reasoning about which error is more expensive for this decision, not inherited from a template.
- [ ] The required sample was converted into a calendar runtime using real daily exposed traffic, and feasibility was checked honestly.
- [ ] When the required runtime is infeasible, one of the four honest options was chosen rather than running an arbitrary duration.
- [ ] Multiple comparisons across metrics and segments were controlled, with one primary or a stated correction.
- [ ] The correct sample-size formula was used for continuous versus proportion metrics.
- [ ] Power was re-run when eligibility, randomization unit, metric, or MDE changed during design.
- [ ] The brief states that a non-significant result from this test means the confidence interval was examined, not that the feature has no effect.
