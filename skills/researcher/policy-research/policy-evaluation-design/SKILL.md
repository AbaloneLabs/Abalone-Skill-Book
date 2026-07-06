---
name: policy_evaluation_design.md
description: Use when the agent is evaluating a policy or program, choosing an impact evaluation design, establishing a counterfactual, attributing observed outcomes to the policy, or interpreting quasi-experimental evidence about what a policy caused.
---

# Policy Evaluation Design

Policy evaluation asks whether a policy or program caused the outcomes observed, and that question is fundamentally about the counterfactual, what would have happened without the policy. Because policies are rarely assigned at random, evaluators must construct a credible comparison, and the credibility of that comparison determines whether the evaluation can support a causal claim. When policy evaluation is designed weakly, three harms follow. Programs that do not work are continued or expanded because their evaluation could not distinguish effect from trend. Programs that work are killed because a weak design showed no effect. And vulnerable populations, who are often the subjects of policy experiments, bear the cost of evaluations that were never capable of answering the question. Policy evaluation is causal inference under difficult conditions, and it demands designs matched to the difficulty.

The agent should use this skill when designing an evaluation of a policy or program, choosing between experimental and quasi-experimental methods, constructing a comparison group, or interpreting evidence about policy impact. The goal is to keep the agent from treating before-after observation as evaluation, when credible policy evaluation requires a defensible counterfactual and a design that can rule out the obvious alternatives.

## Core Rules

### Define The Causal Question And The Counterfactual Precisely

Before choosing a design, define what is being evaluated and against what.

- The specific policy or program, including its components and dose.
- The population affected and the population of interest for inference.
- The outcomes the policy is meant to affect.
- The time horizon over which effects are expected.
- The counterfactual, what would have happened without the policy.

A vague question, such as did the reform work, cannot be evaluated. A precise question, such as what was the effect of the tuition subsidy on college enrollment among eligible students in the first two years, can be. Precision determines what design can answer it.

### Choose A Design That Can Identify The Causal Effect

The design must be capable of attributing outcomes to the policy. Match the design to the assignment mechanism and data available.

- Randomized designs, when the policy can be assigned randomly, such as lotteries or pilots.
- Regression discontinuity, when eligibility depends on a cutoff.
- Difference-in-differences, when a policy affects some units and not others over time.
- Synthetic control, when a single treated unit is compared to a weighted combination of others.
- Matching or propensity methods, when units can be compared on observed characteristics.
- Interrupted time series, when a policy changes at a clear point in a long series.

Each design has assumptions and conditions. State them, and check whether they hold. A design whose assumptions are violated cannot support the intended claim.

### Establish A Credible Comparison Group

The comparison group is the heart of non-randomized evaluation. It must approximate what the treated group would have looked like without treatment.

- Match on observed characteristics relevant to the outcome.
- Use units unaffected by the policy but similar in trend or level.
- Avoid comparison groups affected by other simultaneous changes.
- Test the plausibility of the parallel-trends or no-manipulation assumptions where relevant.

A comparison group that differs from the treated group in unobserved ways produces biased estimates. Invest in justifying the comparison, because the evaluation's credibility rests on it.

### Beware Of Selection Into The Policy

Policies are not assigned randomly in most cases. Units that adopt or receive a policy differ from those that do not.

- Voluntary programs attract different participants than non-participants.
- Policies target disadvantaged or high-need areas, which differ in trend.
- Rollout follows political or administrative logic correlated with outcomes.
- Anticipation of a policy changes behavior before implementation.

Selection biases estimates unless addressed by design. Identify the selection mechanism and choose a design that accounts for it.

### Rule Out Confounding Events And History

Outcomes change for many reasons besides the policy. A credible evaluation rules out alternatives.

- Other policies or economic changes occurring at the same time.
- Seasonal or cyclical patterns in the outcome.
- Shocks that affect the treated group differently.
- Changes in measurement or data collection.

Use designs that control for common trends, include relevant covariates, and conduct placebo or falsification tests on outcomes the policy should not affect. An effect that survives these tests is more credible.

### Define And Measure Outcomes That Match The Policy Intent

The outcomes measured must correspond to what the policy was meant to affect.

- Use outcomes directly targeted by the policy, not distant proxies.
- Measure outcomes at the right level, individual, household, school, region.
- Consider intermediate and final outcomes, and unintended outcomes.
- Use valid and reliable measures, and beware of changes in measurement over time.

An evaluation that measures the wrong outcomes can miss real effects or find spurious ones. Align measurement with the policy's theory of change.

### Assess Both Average And Distributional Effects

Average effects can hide important variation in who is helped or harmed.

- Examine effects across subgroups defined by need, income, region, or prior status.
- Look for heterogeneity that changes the policy's equity implications.
- Consider distributional effects, not only the mean.
- Beware of small subgroups where estimates are unstable.

A policy with a positive average effect that harms the most vulnerable is not a success by most criteria. Distributional analysis is central to policy evaluation, not optional.

### Account For Implementation And Take-Up

A policy's effect depends on whether it was actually implemented and used.

- Measure implementation fidelity and coverage.
- Measure take-up among the eligible population.
- Distinguish intent-to-treat effects from treatment-on-the-treated effects.
- Interpret null results in light of implementation, not just design.

A no-effect finding may reflect failure to implement, not failure of the policy. Implementation data are essential to interpret any result.

### Test Robustness And Report Sensitivity and connect Findings To Decision-Relevant Quantities

Causal estimates in policy evaluation depend on assumptions. Test how sensitive the conclusion is.

- Vary the comparison group or specification.
- Conduct placebo tests on unaffected outcomes or periods.
- Test assumptions such as parallel trends with pre-period data.
- Report how large an unobserved confounder would need to be to change the conclusion.

A conclusion that flips under reasonable alternatives is fragile and should be reported as such. Robustness testing is part of honest evaluation, not an extra.

Policy makers need results in usable form.

- Report effects in meaningful units, percentage points, currency, lives.
- Translate into costs and benefits where appropriate.
- Express uncertainty clearly, with intervals not just point estimates.
- Distinguish what the evaluation can and cannot say about scaling or generalizing.

An effect size buried in statistical machinery is less useful than a clearly communicated estimate with its uncertainty and limits. Evaluation serves decisions, and the reporting should reflect that.

## Common Traps

### Before-After As Evaluation

Observed change may reflect trend, not policy. Use a credible counterfactual.

### Weak Comparison Groups

A comparison group that differs in unobserved ways biases estimates. Justify the comparison.

### Ignoring Selection

Voluntary or targeted policies select units that differ. Address selection in design.

### Confounding Events

Simultaneous changes masquerade as policy effects. Rule out alternatives.

### Measuring Wrong Outcomes

Outcomes misaligned with policy intent miss or manufacture effects. Align measurement.

### Reporting Only Averages

Averages hide harms to subgroups. Examine distributional effects.

### Ignoring Implementation

No-effect may mean no implementation. Measure take-up and fidelity.

### Fragile Conclusions Presented As Solid

Estimates that flip under reasonable alternatives are not robust. Test and report sensitivity.

## Self-Check

- Is the causal question and counterfactual defined precisely, including policy, population, outcomes, and horizon?
- Does the chosen design, randomized or quasi-experimental, actually identify the causal effect under its assumptions?
- Is the comparison group credible, with its plausibility tested and justified?
- Is selection into the policy identified and addressed by the design?
- Are confounding events, history, and measurement changes ruled out through design and falsification tests?
- Do the measured outcomes match the policy's intent, at the right level and with valid measures?
- Are average and distributional effects across subgroups examined, including equity implications?
- Are implementation fidelity and take-up measured and used to interpret results?
- Is the conclusion tested for robustness to comparison group, specification, and unobserved confounding?
- Are findings reported in decision-relevant units with uncertainty and clear limits on what can be inferred?; for evaluations driving major policy or budget decisions, has an experienced policy evaluation methodologist or economist reviewed the design and analysis before conclusions are drawn?
