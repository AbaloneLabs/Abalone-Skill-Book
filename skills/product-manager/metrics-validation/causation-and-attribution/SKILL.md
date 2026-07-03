---
name: causation_and_attribution.md
description: Use when the agent is attributing an outcome to a cause, distinguishing correlation from causation in product data, evaluating attribution models, or deciding whether a metric change was actually caused by a product change versus a confounder.
---

# Causation And Attribution

Product data is overwhelmingly observational. A number went up after a launch, a cohort that used a feature retained better, a campaign preceded a spike in signups. Each of these is a coincidence in time, and coincidence is not causation. The product manager's hardest analytical job is resisting the temptation to read causality into patterns that are merely correlated, because acting on a false causal claim wastes effort on a lever that does nothing.

The harm this skill prevents is building and investing based on a story that the data does not actually support. A team attributes a retention gain to a feature that had no effect, doubles down on it, and is surprised when scaling the feature does not scale the gain. A marketing team credits a campaign for growth that was driven by a seasonal shift, then misallocates budget. In each case the decision felt data-informed because a number was cited, but the causal link was assumed, not established.

Use this skill whenever you are tempted to say a change caused an outcome, whenever an attribution model assigns credit across touchpoints, and whenever a metric movement is about to justify a decision. The default posture is skepticism: assume a relationship is correlational until something isolates the cause.

## Core Rules

### Demand A Counterfactual Before Accepting Causation

Causation requires a comparison to what would have happened otherwise. A retention rate of forty percent means nothing on its own; it means something only against the rate you would have seen without the change. The counterfactual is the entire foundation of a causal claim, and observational data rarely provides one cleanly.

Before accepting that a change caused an outcome, ask what the comparison group is. If there is no comparison group, the claim is a before-and-after story, which conflates the change with everything else that happened over time, including seasonality, growth, and unrelated improvements. A credible causal claim needs a defensible counterfactual, not just a before and after.

### Look For Confounders Before Trusting A Relationship

A confounder is a third variable that drives both the supposed cause and the outcome. Users who complete onboarding retain better, but the motivation that made them complete onboarding may be what actually drives retention. The behavior and the outcome share a cause, and neither causes the other. This is the most common structure in product data.

For any causal claim, list the plausible confounders: pre-existing motivation, acquisition channel, plan tier, device, prior experience, timing. If a confounder could explain the relationship, the claim is not established. The question is not whether a confounder exists, but whether the analysis has ruled out the ones that matter.

### Recognize Selection Effects In Behavioral Comparisons

When users self-select into a behavior, comparing those who did it to those who did not is biased by the selection itself. The users who adopted a feature are different from those who did not, usually in ways related to the outcome. Comparing their retention measures the difference in who they are as much as the effect of the feature.

Selection effects cannot be removed by adding more users or more segments, because the bias is in who chose. They can only be addressed by random assignment, by finding a natural experiment, or by acknowledging the comparison as correlational. A behavioral cohort comparison is a hypothesis, not a result.

### Prefer Experiments For Causal Claims

Randomized assignment is the cleanest way to isolate a causal effect, because it breaks the link between user characteristics and exposure to the change. An experiment with a proper control group produces a causal estimate that observational analysis cannot match. When the stakes are high and the change is testable, the experiment is the right tool.

When an experiment is not possible, use the strongest available alternative: holdout groups, difference-in-differences, regression discontinuity, or interrupted time series. Each has assumptions and limitations, and none is as clean as randomization. State the assumptions, and present the conclusion with the appropriate uncertainty.

### Evaluate Attribution Models By Their Assumptions

Attribution models assign credit for a conversion across the touchpoints that preceded it, but every model encodes assumptions that determine the answer. Last-touch attribution gives all credit to the final click and ignores everything before. First-touch does the reverse. Position-based and data-driven models spread credit according to rules or learned weights. None of these is the true causal decomposition, because the true decomposition is not observable.

Choose an attribution model based on the decision it informs and the biases you can tolerate, and never present its output as ground truth. Compare results across at least two models; if the conclusion flips depending on the model, the attribution is not robust enough to drive a decision. Attribution is a model of belief about causality, not a measurement of it.

### Separate Within-User Change From Between-User Difference

A metric can move because individual users changed behavior, or because the mix of users changed, or both. A retention improvement driven by a shift toward higher-quality acquisition channels is not a product improvement, it is a population change. Confusing the two leads to crediting the product for what the audience did.

Decompose movements into within-user and between-user components where possible. If the average moved but no individual user's behavior changed, the cause is composition, not the product. This distinction is essential before attributing a metric change to anything the team did.

### Time-Order The Cause And Effect, But Do Not Stop There

A cause must precede its effect, so temporal ordering is a necessary condition for causation. But temporal ordering is not sufficient, because many things precede an outcome without causing it. Confirming that the change came first rules out reverse causation but not confounding.

Use timing as a filter, then apply the harder checks. A launch followed by an improvement is consistent with causation, but so is a launch timed before a seasonal peak. Sequence alone never establishes causation; it only removes one alternative explanation.

## Common Traps

### Reading Before-And-After As Causation

A metric that improves after a change is the weakest possible evidence of causation, because time carries many confounders. Seasonality, growth, concurrent changes, and regression to the mean can all produce the pattern. Always ask what the comparison group is before treating a before-and-after as proof.

### Treating A Correlated Behavior As A Causal Lever

Users who do a behavior often have better outcomes, but that does not mean pushing other users to do the behavior will replicate the outcome. The behavior may be a marker of an underlying trait, not a cause of the outcome. Building for the marker instead of the cause wastes effort.

### Trusting Attribution Output As Ground Truth

Attribution models produce precise-looking numbers that are actually assumption-laden estimates. Treating model output as the real causal decomposition leads to confident misallocation of credit and budget. Always know which model produced the numbers and what it assumes.

### Ignoring Regression To The Mean

An extreme metric value is likely to be followed by a less extreme one, purely by chance. A team that intervenes after a bad week will often see improvement that has nothing to do with the intervention. Compare against the expected regression, not against the extreme point.

### Confusing Population Shift With Product Effect

When the user mix changes, average metrics move even if no individual behavior changed. A higher average retention after a channel shift may reflect better users, not a better product. Segment before attributing an average movement to a product change.

### Overclaiming From A Single Experiment

One experiment shows an effect under specific conditions, not a universal law. Extrapolating a single result to all users, all markets, or all timeframes ignores the context in which the experiment ran. Generalize cautiously and re-test where the context differs.

## Self-Check

- [ ] A defensible counterfactual or comparison group is identified before any causal claim is made.
- [ ] Plausible confounders were listed and the analysis addresses the ones most likely to matter.
- [ ] Behavioral comparisons are recognized as selection-biased and presented as hypotheses, not causal results.
- [ ] Where stakes are high and the change is testable, an experiment or the strongest quasi-experimental alternative is used.
- [ ] Any attribution model's assumptions are known, and conclusions are checked across at least two models before driving a decision.
- [ ] Metric movements are decomposed into within-user change and between-user composition shift before being attributed to the product.
- [ ] Temporal ordering is confirmed but not treated as sufficient evidence of causation.
- [ ] Before-and-after patterns are explicitly distinguished from causal estimates and not presented as proof.
- [ ] Regression to the mean is considered as an alternative explanation for improvement after a low point.
- [ ] Causal claims are stated with the appropriate level of uncertainty, and a single experiment is not over-generalized beyond its context.
