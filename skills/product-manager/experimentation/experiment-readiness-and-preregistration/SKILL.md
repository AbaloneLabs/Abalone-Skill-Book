---
name: experiment_readiness_and_preregistration.md
description: Use when the agent is deciding whether an experiment is ready to run, defining hypotheses and success metrics before launch, preregistering test design, calculating required sample size and power, or preventing common experimental errors like peeking and multiple comparisons.
---

# Experiment Readiness And Preregistration

An experiment run carelessly produces a number that looks like knowledge and behaves like noise. The most dangerous experiments are not the ones that fail, but the ones that produce a misleadingly confident result from a flawed design: too small to detect the real effect, peeked at repeatedly until something appears significant, or measured against so many metrics that a false positive becomes near-certain. The judgment problem is that experimental rigor is decided before the test launches, not after, and once data starts flowing the temptation to reinterpret, peek, and mine for significance is overwhelming. Preregistration, the act of committing to hypothesis, metrics, sample size, and decision rules before launch, is the discipline that keeps an experiment honest, because it makes it impossible to quietly move the goalposts after seeing the score.

Use this skill before launching any A/B test or experiment, before defining success metrics, before calculating sample size, or before interpreting results. The goal is to prevent the agent from running underpowered tests that cannot detect real effects, from peeking at results and stopping early, or from running tests whose design guarantees unreliable conclusions.

## Core Rules

### State The Hypothesis And Decision Before Launch

An experiment without a pre-stated hypothesis and decision rule is exploration disguised as validation. Before any data is collected, commit to what is being tested, what outcome would confirm or refute it, and what action follows each result.

Preregister:

- the specific hypothesis and the change being tested;
- the primary success metric and its expected direction;
- the guardrail metrics that must not regress;
- the minimum detectable effect that matters;
- the decision rule: ship if X, hold if Y, kill if Z;
- the owner of the decision and the review date.

Without this, post-hoc rationalization will turn any result into a justification for whatever the team already wanted.

### Calculate Power And Sample Size Before Running

An underpowered experiment is worse than no experiment, because it will likely fail to detect a real effect and produce a false negative that kills a good idea, or produce a noisy positive that ships a bad one. Sample size must be calculated from the effect size that matters, not from convenience.

Calculate using:

- the minimum effect size that is practically meaningful, not the effect you hope for;
- the baseline rate and variance of the primary metric;
- the desired statistical power, typically 80% or higher;
- the significance threshold, typically 5%, adjusted for multiple comparisons;
- the expected exposure and traffic allocation.

If the required sample size is infeasible, the experiment as designed cannot answer the question and must be redesigned or abandoned.

### Guard Against Peeking And Early Stopping

Checking results repeatedly and stopping when they look good destroys the statistical guarantees of the test. Each peek inflates the chance of a false positive, and a test stopped at a random high point will not replicate. This is among the most common and damaging errors in experimentation.

Prevent peeking by:

- running to the pre-calculated sample size or fixed duration;
- using sequential methods only if designed for repeated looks;
- resisting pressure to call a test early when results look dramatic;
- communicating that early reads are exploratory, not decisive;
- locking the decision rule before exposure begins.

A test stopped at day three because it looks great is very often a false positive.

### Control The Multiple Comparisons Problem

Every additional metric, segment, and cut of the data increases the chance of finding a spurious significant result. Testing one primary metric at 5% significance has a 5% false-positive rate; testing twenty metrics guarantees near-certain false positives somewhere.

Control by:

- designating a single primary metric decided in advance;
- limiting guardrail metrics to a small predefined set;
- applying corrections, Bonferroni or false discovery rate, when multiple tests are run;
- treating segment analyses as exploratory unless pre-specified and powered;
- pre-declaring the full analysis plan.

If everything is tested, something will always appear significant by chance.

### Ensure Clean Randomization And Assignment

The validity of an experiment depends on assignment to control and treatment being truly random and stable. Bugs in assignment, cross-contamination between groups, or non-equivalent exposure corrupt the comparison.

Verify:

- assignment is random and deterministic per user;
- users stay in their assigned group across sessions, no flickering;
- no cross-contamination, network effects, or shared state between groups;
- exposure matches assignment, users actually experience their variant;
- the two groups are balanced in size and baseline characteristics.

A test with broken randomization produces confident nonsense.

### Define Guardrail Metrics To Prevent Harmful Wins

A change can lift the primary metric while harming something more important: latency, error rate, revenue from another segment, support load, or trust. Guardrail metrics define the unacceptable regressions that veto a win.

Define guardrails for:

- performance: latency, load time, error rate;
- business: revenue, retention, conversion in adjacent funnels;
- quality: support tickets, bug reports, satisfaction;
- fairness: impact across segments, no disparate harm;
- trust: privacy, security, data integrity.

A primary-metric win that trips a guardrail does not ship.

### Distinguish Practical From Statistical Significance

A result can be statistically significant and practically meaningless. With enough sample size, any tiny difference becomes significant. The question is whether the effect is large enough to justify the change.

Evaluate:

- the effect size and its confidence interval, not just the p-value;
- whether the effect exceeds the minimum meaningful threshold;
- the cost and risk of shipping versus the benefit captured;
- whether the effect is durable or likely novelty;
- the cumulative impact across affected users.

### Plan For Duration And Seasonality

Experiments run too briefly capture atypical periods: a holiday, an outage, a marketing spike. Duration must cover full cycles of normal behavior.

Plan duration by:

- running across complete weekly cycles to capture day-of-week variation;
- avoiding overlap with known anomalies, launches, holidays, outages;
- extending for low-traffic segments to reach adequate exposure;
- checking for novelty effects that fade after initial exposure;
- considering whether the effect needs time to materialize.

## Common Traps

### Underpowered Tests

Running experiments too small to detect the real effect, producing false negatives that kill good ideas.

### Peeking And Early Stopping

Checking repeatedly and stopping at a dramatic-looking point, inflating false positives.

### Multiple Comparisons

Testing many metrics and segments without correction, guaranteeing spurious findings.

### Post-Hoc Hypotheses

Formulating the hypothesis after seeing the data, turning exploration into false validation.

### Broken Randomization

Assignment bugs or contamination that make the comparison invalid.

### Ignoring Guardrails

Shipping a primary-metric win that harms performance, revenue, or trust.

### Statistical Over Practical Significance

Chasing p-values for effects too small to matter at scale.

### Short-Duration Noise

Running across an atypical period and mistaking anomaly for signal.

## Self-Check

- [ ] The hypothesis, primary metric, guardrails, minimum detectable effect, and decision rule are preregistered before launch.
- [ ] Sample size and power are calculated from the meaningful effect size, baseline variance, and desired power before the test runs.
- [ ] The test runs to its pre-calculated duration or sample size without peeking or early stopping.
- [ ] Multiple comparisons are controlled through a single primary metric, limited guardrails, and corrections where needed.
- [ ] Randomization and assignment are verified as clean, stable, and uncontaminated.
- [ ] Guardrail metrics cover performance, business, quality, fairness, and trust, and a guardrail trip vetoes a win.
- [ ] Practical significance, effect size and confidence interval, is evaluated alongside statistical significance.
- [ ] Test duration covers full weekly cycles and avoids known anomalies and novelty windows.
- [ ] Segment analyses are treated as exploratory unless pre-specified and powered.
- [ ] The decision owner and review date are defined before results are seen.
