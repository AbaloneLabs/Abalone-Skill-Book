---
name: mvp_metrics.md
description: Use when the agent is selecting, defining, or interpreting metrics for a minimum viable product, distinguishing learning metrics from vanity metrics, choosing the one metric that matters, instrumenting measurement before launch, avoiding premature optimization of the wrong numbers, or ensuring MVP metrics actually test the core hypothesis rather than flattering the team.
---

# MVP Metrics

The purpose of an MVP is to learn whether the core hypothesis holds, and the metrics chosen to evaluate it determine whether that learning is real or illusory. The wrong metrics, signups, page views, downloads, flatter the team while revealing nothing about whether the product delivers value, and a team optimizing the wrong number builds confidently toward the wrong destination. The judgment problem is choosing metrics that actually test the hypothesis, instrumenting them before launch so the data exists, resisting the pull of vanity metrics that feel good but teach nothing, and knowing when a metric is good enough to validate the hypothesis or bad enough to kill it. Agents tend to fail by defaulting to easy-to-measure numbers, by measuring many things instead of the one that matters, and by interpreting ambiguous metrics as validation.

Use this skill when selecting, defining, or interpreting metrics for an MVP. The goal is measurement that produces genuine learning about the core hypothesis, not numbers that comfort the team.

## Core Rules

### Start From The Hypothesis, Not From Available Metrics

The metrics that matter are determined by the hypothesis the MVP exists to test, not by what is easy to measure. Beginning from available metrics, because they are already instrumented, produces measurement of the wrong thing.

Start from the hypothesis by:

- stating the core hypothesis in one sentence, the specific claim about value the MVP tests;
- identifying what observable behavior would prove or disprove that claim;
- selecting metrics that capture that behavior directly;
- rejecting metrics that do not speak to the hypothesis, however easy to measure.

A metric that does not test the hypothesis is noise, regardless of how impressive it looks. The hypothesis is the source of truth for what to measure.

### Choose The One Metric That Matters

A common failure is measuring many metrics, each ambiguous, and interpreting the favorable ones as validation. The discipline is to choose one metric that matters, the single number that best tests the hypothesis, and let it govern the decision.

Choose the One Metric That Matters by:

- selecting the metric most directly tied to the core hypothesis;
- ensuring it captures value, not just activity;
- defining the threshold that would validate or refute the hypothesis in advance;
- letting this metric govern the decision, with other metrics as context.

One metric forces honesty. Many metrics invite cherry-picking. Choose the one that matters and commit to what it says.

### Distinguish Learning Metrics From Vanity Metrics

Vanity metrics, signups, downloads, page views, go up and to the right regardless of whether the product works, because they measure exposure rather than value. Learning metrics, retention, activation, payment, measure whether the product delivers what it promises.

Distinguish by:

- asking whether the metric could go up while the product fails; if yes, it is a vanity metric;
- favoring metrics that capture value delivered, such as retention and willingness to pay;
- treating activity metrics as diagnostic, not as validation;
- being suspicious of metrics that make the team feel good without informing decisions.

A vanity metric is one that cannot refute the hypothesis, because it goes up regardless. A learning metric is one that can prove the hypothesis wrong. Measure the latter.

### Measure Retention As The Primary Signal Of Value

For most products, retention is the metric that most directly tests whether the product delivers sustained value. A product that users try once and abandon has not validated its hypothesis, however many signed up.

Measure retention by:

- defining what counts as active use for the specific product;
- tracking retention over a meaningful timeframe, such as weekly or monthly cohorts;
- comparing retention to a benchmark that would indicate genuine value;
- treating high retention as the strongest signal that the hypothesis holds.

Signups measure interest; retention measures value. If users do not return, the value proposition has not been validated, whatever the signup numbers say.

### Instrument Measurement Before Launch

Metrics that are not instrumented before launch cannot be measured retroactively, and an MVP launched without measurement produces no learning, however it performs. Instrumentation is a launch requirement, not an afterthought.

Instrument by:

- defining all metrics and their calculation before launch;
- building the tracking and analytics needed to capture them;
- testing the instrumentation with synthetic data before real users arrive;
- ensuring the data will exist to answer the hypothesis, not hoping to reconstruct it later.

An MVP without instrumentation is a gamble, not an experiment. The measurement must exist from the first user.

### Define Success And Failure Thresholds In Advance

Ambiguous metrics invite motivated interpretation: a team that wants to proceed reads a marginal metric as success, and a team that fears the answer reads it as inconclusive. Defining thresholds in advance forces honesty.

Define thresholds by:

- stating, before launch, what metric value would validate the hypothesis;
- stating what value would refute it;
- defining the range that would require iteration rather than a verdict;
- committing to act on the result, not to reinterpret it.

A metric interpreted after the fact, with the desired outcome known, is not a test. Define what success and failure look like before the data arrives.

### Avoid Premature Optimization Of The Wrong Number

Once a metric is chosen, teams begin optimizing it, and if the metric is wrong, optimization accelerates progress toward the wrong destination. Changing the metric in response to learning is appropriate; optimizing a vanity metric because it is moving is not.

Avoid premature optimization by:

- resisting the urge to optimize any metric that does not test the hypothesis;
- treating early metrics as signals for learning, not targets for growth;
- being willing to change the metric if learning reveals it tests the wrong thing;
- focusing on understanding before optimization, and on optimization only after the hypothesis is validated.

A team optimizing signups builds a signup machine that delivers no value. Optimize the metric that matters, and only after it is proven to matter.

### Use Cohorts To Understand Behavior Over Time

Aggregate metrics hide the behavior that reveals value. A retention number averaged across all users conceals whether early users retained and recent users churned. Cohort analysis reveals the patterns that matter.

Use cohorts by:

- tracking metrics by signup cohort, such as weekly or monthly groups;
- comparing retention and engagement across cohorts to identify trends;
- detecting whether product changes improve or worsen cohort behavior;
- using cohort analysis to separate genuine improvement from aggregate noise.

Cohorts reveal whether the product is getting better for new users, which aggregate metrics conceal. Use them to understand behavior, not just to report numbers.

### Separate Activation From Retention

Activation, the user's first successful experience of value, and retention, their return to use again, are distinct signals, and conflating them muddies the learning. A product can activate well and retain poorly, or vice versa.

Separate by:

- defining activation as the specific action that indicates the user experienced the core value;
- measuring activation rate independently of retention;
- diagnosing whether problems lie in activation, retention, or both;
- addressing each with different interventions.

Activation measures whether users find value; retention measures whether they return for it. Diagnose problems at the right stage.

## Common Traps

### Measuring From Available Metrics, Not The Hypothesis

Easy-to-measure numbers rarely test the hypothesis. Start from the claim and select metrics that speak to it.

### Many Metrics Instead Of One

Many ambiguous metrics invite cherry-picking. Choose the One Metric That Matters and commit to it.

### Vanity Metrics Disguised As Learning

Signups, downloads, and page views go up regardless of value. Favor metrics that can refute the hypothesis.

### Ignoring Retention

Signups measure interest; retention measures value. Without return use, the hypothesis is not validated.

### Launching Without Instrumentation

Uninstrumented MVPs produce no learning. Build the measurement before launch.

### Interpreting Ambiguous Metrics As Validation

A marginal metric read as success is not a test. Define thresholds in advance and act on them.

### Optimizing The Wrong Number

Optimizing a vanity metric builds toward the wrong destination. Optimize what matters, after it is proven to matter.

### Aggregate Metrics Hiding Cohort Behavior

Averages conceal whether new users retain. Use cohorts to understand behavior over time.

## Self-Check

- [ ] Metrics are selected from the core hypothesis, not from what is easy to measure.
- [ ] One Metric That Matters is chosen and defined, with other metrics as context.
- [ ] Vanity metrics are identified and excluded from validation decisions.
- [ ] Retention is measured as the primary signal of value, with activation defined separately.
- [ ] All metrics are instrumented before launch, with tracking tested before real users arrive.
- [ ] Success and failure thresholds are defined in advance, with commitment to act on the result.
- [ ] No premature optimization of metrics that do not test the hypothesis.
- [ ] Cohort analysis is used to understand behavior over time, not just aggregate numbers.
- [ ] Activation and retention are measured and diagnosed separately.
- [ ] The metrics chosen can actually refute the hypothesis, not just flatter the team.
