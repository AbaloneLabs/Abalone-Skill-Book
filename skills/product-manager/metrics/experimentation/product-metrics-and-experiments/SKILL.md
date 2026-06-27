---
name: product_metrics_and_experiments.md
description: Use when the agent is choosing product metrics, designing an experiment, evaluating feature success, defining guardrails, interpreting product data, or deciding what evidence is needed after launch.
---

# Product Metrics And Experiments

Metrics can clarify product decisions, but they can also create false confidence. A product manager must choose metrics that reflect user value and business outcomes, instrument them correctly, interpret them with context, and protect against local optimization that harms the product.

Use this skill before defining success metrics, analytics events, experiment plans, launch readouts, dashboards, guardrails, or data-informed product decisions. The goal is to prevent the agent from choosing vanity metrics, ignoring data quality, over-reading weak experiments, or declaring success because a number moved without understanding why.

## Core Rules

### Start With The Decision The Metric Supports

Do not choose metrics before naming the decision. Metrics should help decide whether to build, ship, continue, roll back, iterate, scale, or stop.

Ask:

- What decision will this metric inform?
- What user or business outcome should change?
- What behavior would count as success?
- What behavior would count as harm?
- When will the metric be reviewed?
- Who owns the decision after review?

If no decision depends on the metric, it may be dashboard decoration.

### Distinguish Input, Output, And Outcome Metrics

Different metrics answer different questions.

Input metrics describe actions the team or system takes, such as messages sent, recommendations shown, or onboarding steps displayed. Output metrics describe immediate user behavior, such as clicks, completions, searches, exports, invites, or configuration changes. Outcome metrics describe meaningful user or business results, such as activation, retention, revenue, task success, trust, support reduction, or reduced manual work.

Use input and output metrics for diagnosis, but do not confuse them with outcomes. A higher click rate may be good, neutral, or harmful depending on whether it leads to user value.

### Define The Population And Denominator

Most metric mistakes are denominator mistakes. A conversion rate means little unless the eligible population, event start, event end, time window, exclusions, and counting method are clear.

Define:

- who is included;
- who is excluded;
- first-time versus returning users;
- user, account, session, device, or event counting;
- time window;
- exposure criteria;
- completion criteria;
- bot, test, staff, or duplicate filtering;
- handling of missing data.

Never accept "conversion improved" without asking conversion of whom, from what, to what, over which period.

### Pair Success Metrics With Guardrails

A metric can improve while the product gets worse. Guardrails protect against local optimization.

Examples:

- activation improvement with retention guardrail;
- faster checkout with refund or support guardrail;
- more notifications with unsubscribe or complaint guardrail;
- higher revenue with churn or trust guardrail;
- more content creation with quality or moderation guardrail;
- reduced support tickets with user success guardrail.

Guardrails should be chosen before launch, not after a surprising result.

### Instrument For Analysis Before Launch

Instrumentation should be designed with the product behavior. Events need consistent names, properties, timing, identity rules, and privacy review.

Check:

- event names are stable and descriptive;
- properties answer expected analysis questions;
- exposure is recorded separately from action;
- errors and drop-offs are tracked;
- account and user identity are resolved correctly;
- events avoid unnecessary personal data;
- dashboards distinguish fresh launch data from historical baselines;
- data quality can be validated.

If the team cannot trust the data, launch readouts become argument rather than learning.

### Choose Experiment Design Based On Risk And Signal

Not every product question needs an A/B test. Some need interviews, usability tests, cohort analysis, beta rollout, fake-door tests, concierge tests, holdout experiments, or simple operational monitoring.

Use experiments when:

- the change can be isolated;
- the metric can move in a reasonable time;
- traffic is sufficient;
- randomization is ethical and practical;
- risk is controlled;
- the team can act on the result.

Avoid experiments when the sample is too small, contamination is high, the effect would take months, or withholding the feature is unacceptable. Use staged rollout and qualitative learning instead.

### Interpret Results With Context

A result is not only a number. Check confidence, sample size, novelty effects, seasonality, segment differences, instrumentation bugs, concurrent launches, and long-term tradeoffs.

Ask:

- Is the effect large enough to matter?
- Is it consistent across key segments?
- Did guardrails move?
- Was the experiment long enough?
- Did users understand the feature?
- Are there delayed effects?
- Could external events explain the change?
- What qualitative feedback supports or contradicts the metric?

Do not declare product truth from a weak, short, or contaminated measurement.

### Protect Users And Trust

Metrics should not justify manipulative design. A product manager must distinguish genuine value from extraction, confusion, accidental clicks, lock-in, or dark patterns.

If a metric improves by making cancellation harder, hiding costs, increasing anxiety, or pushing unwanted actions, it is not healthy success. Include trust, consent, clarity, and user control in product evaluation.

## Common Traps

### Vanity Metrics

Page views, signups, clicks, downloads, or time spent can look impressive while saying little about user value. Use them only when tied to a decision and paired with stronger outcomes.

### Measuring Only The Happy Path

Track failures, abandonments, errors, retries, support contact, and reversals. A feature may increase completions only because users cannot escape the flow.

### Ignoring Segment Differences

An average improvement can hide harm to an important segment. Review new users, power users, paying customers, accessibility needs, regions, platforms, or high-value accounts where relevant.

### Moving Metrics After Seeing Results

Changing the success definition after launch creates false confidence. Record primary metrics and guardrails before evaluation.

### Treating No Significant Result As No Effect

An inconclusive experiment may mean insufficient sample, noisy metric, small effect, or poor design. It is not automatically proof that the feature has no value.

### Over-Instrumenting Sensitive Data

More data is not always better. Avoid collecting personal, private, or regulated information unless needed and governed.

## Self-Check

- [ ] Each metric is tied to a specific product decision.
- [ ] Input, output, and outcome metrics are distinguished.
- [ ] Population, denominator, exclusions, identity, time window, and counting method are defined.
- [ ] Success metrics have guardrails for retention, trust, quality, support, revenue, privacy, or user harm where relevant.
- [ ] Instrumentation captures exposure, action, errors, drop-offs, and needed properties without unnecessary personal data.
- [ ] Data quality, event naming, identity resolution, and dashboard interpretation were considered before launch.
- [ ] Experiment design matches traffic, risk, ethics, expected signal, and the team's ability to act on the result.
- [ ] Results are interpreted with sample size, confidence, seasonality, novelty, segment effects, and concurrent changes in mind.
- [ ] Qualitative evidence and user understanding are considered alongside metrics.
- [ ] The analysis avoids declaring success from vanity metrics or metric movement that harms user trust.
