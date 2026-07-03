---
name: experiment_design_and_statistics.md
description: Use when the agent is designing A/B or multivariate tests, calculating sample size and duration, defining metrics and guardrails, interpreting statistical significance and power, or reviewing whether an experiment will produce trustworthy and actionable results.
---

# Experiment Design And Statistics

Experiments are how marketing separates real effects from noise. They fail when they are underpowered and declare noise a winner, when they stop early on a flattering interim result, when they measure many metrics and report the one that won, or when they test changes too small to matter. Good experimentation is humble about uncertainty, disciplined about design, and honest about what a result does and does not prove.

Use this skill before designing a test, choosing metrics, calculating sample size, or interpreting results. The goal is to prevent the agent from running experiments that produce confident but wrong conclusions.

## Core Rules

### Start With A Hypothesis And A Primary Metric

Every test should answer a specific question. Define it before collecting data.

Define:

- the hypothesis and the change being tested;
- the primary metric that determines success;
- the expected direction and size of effect;
- the guardrail metrics that must not be harmed;
- the decision the result will inform.

Testing without a predefined primary metric invites cherry-picking the winner after the fact. The hypothesis and metric are set first.

### Calculate Sample Size And Duration Before Launching

A test that ends too early or too small cannot distinguish signal from noise. Size it correctly.

Calculate based on:

- the baseline conversion rate of the metric;
- the minimum effect size worth detecting;
- the statistical power, typically eighty percent or higher;
- the significance threshold, typically five percent;
- the expected traffic and its variability over time.

A test that ends when it looks good is not a test; it is a story. Predetermine the sample or duration and honor it.

### Honor The Predetermined Duration, Do Not Peek

Stopping early on a significant interim result inflates false positives. Resist peeking.

Avoid:

- checking significance daily and stopping when it crosses;
- ending when the result looks good or bad;
- extending until significance appears;
- running many short tests instead of fewer powered ones.

Interim significance bounces. A test that stops the first time it crosses the threshold will often have stopped on noise. Run to the planned duration.

### Account For Multiple Comparisons

The more metrics and segments you check, the more likely one wins by chance. Control for this.

Control by:

- predefining one primary metric;
- treating secondary metrics as exploratory;
- applying corrections when testing many variants;
- being honest that segment wins are often noise;
- replicating surprising segment effects before acting.

If you test twenty segments, one will be significant by chance. Multiple comparisons without correction manufacture false insights.

### Distinguish Significance, Power, And Effect Size

These three are different and each matters. Conflating them misleads.

Understand:

- significance as confidence the effect is real, not zero;
- power as the chance of detecting a real effect if it exists;
- effect size as whether the effect is large enough to matter;
- that a significant tiny effect may not be worth shipping;
- that a non-significant result is not proof of no effect.

A statistically significant result that moves conversion by a rounding error is not worth the complexity. Judge importance by effect size, not just the p-value.

### Use Guardrails To Catch Harm Elsewhere

A win on the primary metric can hide harm elsewhere. Define guardrails.

Set guardrails for:

- downstream metrics such as revenue per visitor and retention;
- latency, error rate, and technical performance;
- segments that may be harmed even if the average wins;
- long-term effects not visible in the test window;
- business outcomes such as returns, complaints, and LTV.

A test that lifts clicks but harms revenue is a loss. Guardrails catch the harm the primary metric hides.

### Beware Novelty And Seasonality Effects

Some effects fade or are artifacts of timing. Account for them.

Account for:

- novelty effects where a change performs well then regresses;
- seasonality and promotional periods that skew results;
- day-of-week and time-of-day patterns;
- external events during the test window;
- the need to run across a full cycle, often one or more weeks.

A change tested only over a holiday weekend may not generalize. Run across representative time periods.

## Common Traps

### Underpowered Tests

Tests with too little traffic declare noise as winners and miss real effects.

### Peeking And Early Stopping

Stopping when a result looks good inflates false positives.

### Multiple Comparisons Without Correction

Testing many metrics or segments and reporting the winner manufactures false insights.

### Confusing Significance With Importance

A significant tiny effect is not worth shipping; effect size matters.

### No Guardrails

A primary-metric win can hide harm to revenue, retention, or other outcomes.

### Ignoring Novelty And Seasonality

Short or unrepresentative test windows produce results that do not hold.

### No Hypothesis Or Primary Metric

Testing without a predefined question invites cherry-picking after the fact.

## Self-Check

- [ ] The test has a predefined hypothesis, primary metric, expected effect, and decision it informs.
- [ ] Sample size and duration are calculated from baseline rate, minimum detectable effect, power, and significance.
- [ ] The test runs to its predetermined duration without early stopping on interim significance.
- [ ] One primary metric is predefined, and secondary metrics are treated as exploratory.
- [ ] Multiple comparisons across metrics, variants, or segments are controlled or acknowledged.
- [ ] Statistical significance is interpreted alongside power and effect size.
- [ ] Guardrail metrics are defined and checked to catch downstream or segment harm.
- [ ] The test window accounts for novelty, seasonality, and day-of-week patterns.
- [ ] Surprising segment results are treated as hypotheses to replicate, not confirmed wins.
- [ ] The conclusion states what the result does and does not prove, with uncertainty acknowledged.
