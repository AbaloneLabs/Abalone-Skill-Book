---
name: marketing_experimentation_and_ab_testing.md
description: Use when the agent is designing A/B tests and marketing experiments, deciding what to test and how, determining sample size and significance, avoiding false conclusions from tests, or reviewing whether an experiment's results are trustworthy and actionable.
---

# Marketing Experimentation And AB Testing

Experimentation is how marketing separates what works from what we think works. Without it, decisions rest on opinion, HiPPO authority, and survivorship bias, and the team optimizes for activity rather than outcome. With it, decisions rest on evidence and the program compounds learning. But experimentation is easy to do badly and bad experiments are worse than none, because they produce confident wrong answers. The judgment problem is that a valid experiment requires careful design: a clear hypothesis, a meaningful metric, adequate sample size, control of confounders, and honest interpretation. The most common failure is running underpowered tests, peeking at results, or declaring winners on noise, producing decisions that feel data-driven but are not. The skill is designing experiments whose results can be trusted and acted upon.

Use this skill before designing an A/B test or experiment, before interpreting test results, before deciding what to test, or when an experiment's conclusion seems too good or contradictory. The goal is to prevent the agent from running invalid experiments, from declaring winners on noise, or from testing trivial changes while ignoring strategic questions.

## Core Rules

### [ ] Start With A Hypothesis, Not A Test Idea

A good experiment tests a hypothesis about cause and effect: "If we change X, then metric Y will change because Z." Starting from a test idea without a hypothesis produces tests without clear interpretation. The hypothesis forces you to specify what you expect and why, which makes the result interpretable.

- [ ] Write a hypothesis with a clear cause, expected effect, and rationale.
- [ ] Define what result would confirm or refute the hypothesis.
- [ ] Ensure the hypothesis connects to a decision you will make.
- [ ] Reject test ideas that have no underlying hypothesis.

### [ ] Choose A Primary Metric That Reflects The Goal

The primary metric must directly reflect the business goal of the test, not a proxy that is easy to move. Testing for click-through when the goal is revenue optimizes for the wrong thing. Choose the metric that the decision will be based on, and resist changing it after results come in.

- [ ] Define one primary metric tied to the business goal.
- [ ] Distinguish primary metric from secondary and guardrail metrics.
- [ ] Resist optimizing for easy-to-move proxies over hard-to-move outcomes.
- [ ] Do not change the primary metric after the test starts.

### [ ] Calculate Sample Size Before Starting

An underpowered test cannot detect a real effect and produces inconclusive or misleading results. Sample size depends on the baseline rate, the minimum detectable effect, and the desired statistical power. Calculating it before starting tells you how long the test must run.

- [ ] Calculate required sample size based on baseline, MDE, power, and significance.
- [ ] Estimate how long the test must run to reach that sample.
- [ ] Do not conclude before the sample size is reached.
- [ ] Accept that some effects are too small to detect feasibly.

### [ ] Run The Test Long Enough To Capture Cycles

Marketing data varies by day of week, season, and cohort. A test that runs for two days may capture only weekdays and mislead. Tests must run long enough to capture full cycles and reach the required sample size, whichever is longer.

- [ ] Run for at least one full weekly cycle, often two or more.
- [ ] Account for seasonality and anomalies during the test window.
- [ ] Do not stop early for apparent wins or losses.
- [ ] Pre-commit to a minimum duration before starting.

### [ ] Guard Against Peeking And P-Hacking

Checking results repeatedly and stopping when they look good inflates false positives. Testing many variants or metrics and reporting the significant ones is p-hacking. Both produce winners that do not hold up. Pre-commit to analysis rules and resist the temptation to peek.

- [ ] Pre-commit to when and how results will be analyzed.
- [ ] Do not stop the test early based on interim results.
- [ ] Correct for multiple comparisons when testing many variants or metrics.
- [ ] Report all tests run, not only the ones that won.

### [ ] Control Confounders Through Randomization

Valid experiments require that the only systematic difference between groups is the treatment. Randomization distributes confounders evenly; without it, differences may be due to audience, timing, or selection bias rather than the treatment. Ensure randomization is real and not compromised.

- [ ] Randomize assignment to control and treatment groups.
- [ ] Verify that groups are balanced on key variables.
- [ ] Watch for selection bias, e.g., only part of the audience is exposed.
- [ ] Control for time, season, and external events that affect both groups.

### [ ] Distinguish Statistical Significance From Practical Significance

A result can be statistically significant and practically meaningless, or practically important but not yet statistically confirmed. Significance tells you the effect is real; effect size tells you whether it matters. Both must be considered in the decision.

- [ ] Report effect size and confidence interval, not just p-values.
- [ ] Compare the effect to the minimum meaningful difference.
- [ ] Consider the cost and risk of implementing the change.
- [ ] Do not ship a significant but trivial change, nor ignore a meaningful non-significant one.

### [ ] Use Guardrail Metrics To Catch Harm

A test can move the primary metric while harming others: conversion up but retention down, revenue up but complaints up. Guardrail metrics monitor for unintended harm and prevent local optimization that damages the whole. Define them before the test.

- [ ] Define guardrail metrics that must not degrade, e.g., retention, load time, NPS.
- [ ] Monitor guardrails alongside the primary metric.
- [ ] Do not ship a change that wins on primary but breaks guardrails.
- [ ] Investigate guardrail movements even when the primary metric wins.

### [ ] Prioritize Tests By Expected Value And Learning

Not all tests are worth running. Prioritize by the expected value of the decision and the learning the test provides, not by ease of implementation. Testing button colors is easy but low-value; testing pricing or positioning is hard but high-value. Allocate experimentation capacity to where it matters.

- [ ] Estimate the value of the decision the test informs.
- [ ] Prioritize tests that unlock strategic learning, not just incremental wins.
- [ ] Balance quick wins with high-stakes strategic experiments.
- [ ] Stop testing trivial changes once the obvious optimizations are done.

### [ ] Document And Share Results, Including Failures

Experimentation builds organizational knowledge only if results are documented and shared, including tests that failed. A library of past experiments prevents re-testing the same things and spreads learning. Hoarding or hiding results wastes the investment.

- [ ] Document every experiment: hypothesis, design, results, decision.
- [ ] Share results broadly, including null and negative results.
- [ ] Maintain an experiment library to prevent duplication.
- [ ] Use past results to inform future hypotheses.

## Common Traps

### [ ] Testing Without A Hypothesis

Running tests with no clear expectation, making results uninterpretable.

### [ ] Proxy Metric Optimization

Optimizing click-through or opens when the goal is revenue or retention.

### [ ] Underpowered Tests

Concluding from samples too small to detect the effect.

### [ ] Peeking And Early Stopping

Stopping tests when results look good, inflating false positives.

### [ ] P-Hacking

Testing many variants or metrics and reporting only the winners.

### [ ] Compromised Randomization

Confounders that make differences attributable to something other than the treatment.

### [ ] Significant But Trivial

Shipping changes that are statistically significant but practically meaningless.

### [ ] Local Optimization Harm

Winning on the primary metric while harming guardrails or the broader system.

## Self-Check

- [ ] Does each test start with a hypothesis specifying cause, effect, and rationale?
- [ ] Is the primary metric tied to the business goal, not a convenient proxy?
- [ ] Was sample size calculated before starting, and is the test run to completion?
- [ ] Does the test run long enough to capture weekly cycles and reach required sample?
- [ ] Are analysis rules pre-committed, with no peeking or early stopping?
- [ ] Is assignment randomized and verified balanced across groups?
- [ ] Are effect size and practical significance reported alongside statistical significance?
- [ ] Are guardrail metrics defined and monitored to catch unintended harm?
- [ ] Are tests prioritized by expected value and strategic learning, not just ease?
- [ ] Are all results, including failures, documented and shared to build organizational learning?
