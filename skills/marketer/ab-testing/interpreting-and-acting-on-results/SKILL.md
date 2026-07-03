---
name: interpreting_and_acting_on_results.md
description: Use when the agent is interpreting experiment results, deciding whether to ship or kill a variant, analyzing segment effects, writing test readouts, communicating uncertainty to stakeholders, or reviewing whether conclusions drawn from a test are justified by the evidence.
---

# Interpreting And Acting On Results

The end of a test is where the most damage is done. A result is declared a winner when it was noise, a non-significant result is declared proof of no effect, a segment win is over-interpreted, or a winning variant is shipped without checking the guardrails. Interpreting results requires discipline because the pressure to declare victory is constant and the tools make false confidence easy. Honest interpretation protects the business from acting on illusions.

Use this skill before declaring a test outcome, shipping a variant, writing a readout, or advising a decision based on experiment results. The goal is to prevent the agent from turning ambiguous evidence into confident wrong action.

## Core Rules

### Read The Result Against The Predefined Criteria

The test was designed with a primary metric and a threshold. Honor them.

Check:

- the primary metric against the predefined significance and effect size;
- whether the test reached its planned sample and duration;
- the guardrail metrics for harm;
- the direction against the hypothesis;
- the confidence interval, not just the point estimate.

If the primary metric was set in advance, judge by it. Changing the metric after seeing results is cheating the test.

### Distinguish Significant, Inconclusive, And No-Effect

These three outcomes are different and demand different actions.

Interpret:

- significant as evidence the effect is likely real, pending effect size;
- inconclusive as the test could not tell, not as proof of no effect;
- no-effect as only provable with a tight confidence interval around zero;
- the confidence interval to see the range of plausible effects.

An inconclusive test means you do not know. Declaring no effect from an underpowered test is a common and costly error.

### Check Segments, But Do Not Overfit

Segment results can reveal where a change helps or hurts, but they multiply comparisons and invite false findings.

Use segments to:

- confirm the average effect holds across key segments;
- flag where a change helps one group and harms another;
- generate hypotheses for future tests;
- avoid acting on a segment win as if it were proven.

A segment that wins while the average is flat is often noise. Treat segment findings as hypotheses to replicate, not as confirmed truths.

### Examine Effect Size And Confidence Interval

Whether an effect matters depends on its size and the uncertainty around it.

Examine:

- the point estimate of the effect;
- the confidence interval showing the plausible range;
- whether the lower bound still justifies shipping;
- the cost and risk of implementing the change;
- the business value of the expected uplift.

A significant result with a tiny effect may not be worth shipping. A result with a wide interval may include near-zero effects. Decide on the range, not just the midpoint.

### Verify Guardrails Before Shipping

A win on the primary metric can hide harm. Check guardrails before acting.

Verify:

- downstream metrics such as revenue, retention, and quality;
- technical metrics such as latency and error rate;
- segments that may be harmed;
- long-term indicators not visible in the test window;
- any metric that would make the change a net negative.

Shipping a click win that harms revenue is a loss. Guardrails exist to prevent this; check them every time.

### Communicate Uncertainty Honestly

Stakeholders want certainty. Provide honest uncertainty instead.

Communicate:

- the point estimate and the confidence interval;
- what the result does and does not prove;
- the assumptions and limits of the test;
- the remaining unknowns;
- the recommended action and its risks.

Overstating certainty creates false confidence that collapses later. Honest uncertainty builds trust and leads to better decisions.

### Decide Ship, Kill, Or Iterate Based On Evidence

A test should produce a decision and a next step, not just a verdict.

Decide:

- ship if the primary metric wins, guardrails hold, and effect size justifies it;
- kill if the change harms guardrails or clearly underperforms;
- iterate if the result is promising but inconclusive or partial;
- replicate if the result is surprising or high-stakes;
- document the learning regardless of outcome.

A test that ends without a clear decision and documented learning wasted its run. Every test should update what the team knows.

## Common Traps

### Declaring Noise A Winner

Treating an interim or underpowered significant result as a confirmed win.

### Inconclusive As No-Effect

Interpreting a test that could not tell as proof that there is no effect.

### Overfitting Segments

Acting on a segment win as proven when it is often noise from multiple comparisons.

### Ignoring Effect Size

Shipping any significant result regardless of whether the effect matters.

### Skipping Guardrails

Shipping a primary-metric win that harms downstream or business metrics.

### Overstating Certainty

Presenting ambiguous results as definitive to satisfy demand for a clear answer.

### No Decision Or Learning

Ending a test with a verdict but no documented decision or insight.

## Self-Check

- [ ] The result is judged against the predefined primary metric, significance threshold, and effect size.
- [ ] Inconclusive results are distinguished from proven no-effect, using the confidence interval.
- [ ] Segment effects are checked but treated as hypotheses to replicate, not confirmed truths.
- [ ] Effect size and confidence interval are examined, not just significance.
- [ ] Guardrail metrics are verified before shipping, including downstream and technical metrics.
- [ ] Uncertainty is communicated honestly, including what the result does and does not prove.
- [ ] The decision to ship, kill, iterate, or replicate is stated with its rationale.
- [ ] Surprising or high-stakes results are replicated before driving hard-to-reverse decisions.
- [ ] The learning from the test is documented regardless of whether it won or lost.
- [ ] The readout states the remaining unknowns and recommended next steps.
