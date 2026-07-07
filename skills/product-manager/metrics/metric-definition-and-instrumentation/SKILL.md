---
name: metric_definition_and_instrumentation.md
description: Use when the agent is defining product metrics, choosing what to measure, designing event tracking and instrumentation, avoiding vanity metrics, establishing metric definitions and governance, or deciding whether a metric is trustworthy enough to drive decisions.
---

# Metric Definition And Instrumentation

A metric is only as good as the definition behind it and the instrumentation that produces it. Most metric problems are not analytical; they are definitional and technical. Two teams arguing about whether activation is 40% or 60% are usually using different definitions of activation, computed from different event streams, with different filters, and neither number is wrong, they are just measuring different things. The judgment problem is that a metric looks like a number but is actually a contract: a precise definition, a reliable data pipeline, a known set of exclusions, and a documented ownership. Treating metrics as self-evident quantities rather than engineered artifacts produces dashboards that mislead, decisions that contradict each other, and a culture that loses trust in its own data.

Use this skill before defining a new metric, before instrumenting events, before trusting a dashboard number, or before resolving metric discrepancies between teams. The goal is to prevent the agent from proposing metrics that cannot be measured reliably, from using vanity metrics that cannot drive decisions, or from building instrumentation whose silent errors corrupt every downstream analysis.

## Core Rules

### Define The Metric As A Precise Contract

A metric name is not a definition. "Active user" can mean logged in, performed an action, opened the app, or generated revenue. The definition must be unambiguous enough that two analysts computing it independently get the same number.

A complete metric definition specifies:

- the numerator and denominator with exact event or state conditions;
- the unit of measurement, user, account, session, device;
- the time window, daily, weekly, rolling 28-day;
- the filters and exclusions, internal users, test accounts, bots;
- the cohort or segment scope;
- the threshold or condition that qualifies an event;
- the source system and event of record.

If the definition cannot be written in one precise paragraph, the metric is not yet defined.

### Start From The Decision, Not The Data

A metric exists to inform a decision. Defining metrics by what data is easy to collect produces vanity metrics that go up and tell you nothing. Start from the question the metric must answer.

For each metric, specify:

- the decision it will inform;
- the action that follows if it moves;
- the threshold or direction that triggers action;
- who owns the decision and reviews the metric;
- how often it is reviewed.

If no decision changes based on the metric, it is decoration. A metric that cannot change a decision is a vanity metric regardless of how impressive it looks.

### Distinguish Leading, Lagging, And Diagnostic Metrics

Different metrics serve different roles, and confusing them produces bad decisions. Lagging metrics confirm what happened; leading metrics predict what will happen; diagnostic metrics explain why.

Classify and combine:

- lagging: revenue, churn, retention, the outcomes you are trying to move;
- leading: activation, engagement depth, feature adoption, early predictors;
- diagnostic: funnel drop-off, error rate, segment breakdown, the why behind movement.

Managing only lagging metrics means learning too late to act. Managing only leading metrics risks optimizing proxies that do not connect to outcomes.

### Instrument Events To Match The Definition

Instrumentation is where definitions meet reality, and where most silent corruption occurs. An event that fires on the wrong action, fires twice, or fails to fire on edge cases makes the metric wrong in ways no dashboard will reveal.

Instrument carefully:

- define each event's trigger condition precisely;
- handle edge cases, app backgrounding, offline, retries, duplicate fires;
- include the properties needed for segmentation and filtering;
- version events and properties so schema changes are traceable;
- test instrumentation in staging against the definition before trusting it;
- document the event of record when multiple systems could log it.

Unvalidated instrumentation is the single largest source of metric distrust.

### Separate Raw Counts From Rates And Norms

A raw count going up often means nothing because the base grew. Total signups rising is expected if traffic rose; the question is the rate. Conversely, a rate can hide problems in a shrinking base.

Always pair:

- counts with the denominator they depend on;
- rates with the absolute volumes behind them;
- period-over-period change with the base size;
- aggregate metrics with segment breakdowns.

A metric presented without its denominator or base is incomplete and often misleading.

### Establish Metric Governance And Ownership

Metrics drift without ownership. Definitions change silently, filters get added, segments get redefined, and suddenly year-over-year comparisons break. Governance prevents this decay.

Govern by:

- assigning a single owner accountable for each metric's definition;
- maintaining a metric catalog with definitions, owners, and change history;
- requiring documented review before a definition changes;
- versioning definitions so historical comparisons remain valid;
- communicating definition changes to all consumers.

### Validate The Metric Before Trusting It

Before a metric drives decisions, validate that it behaves as expected. A new metric should be stress-tested against known scenarios.

Validate by:

- checking it against known segments and historical events;
- confirming it moves in expected directions for known changes;
- comparing it to an independent measure of the same concept;
- inspecting the underlying events for anomalies and duplicates;
- having a second analyst reproduce the number from raw data.

A metric that has never been validated is a hypothesis about a number, not a measurement.

### Make The Metric's Limitations Explicit

Every metric has blind spots. Honest metric design states what the metric does not capture so consumers do not over-rely on it.

Document:

- what the metric excludes or ignores;
- known biases in the population it covers;
- edge cases where it misleads;
- the confidence or noise level around it;
- complementary metrics needed to complete the picture.

## Common Traps

### Vanity Metrics

Total signups, page views, and raw counts that go up regardless of effort cannot drive decisions and create false comfort.

### Undefined Definitions

Using a metric name without a precise contract guarantees that teams measure different things and argue past each other.

### Silent Instrumentation Errors

Events that fire wrong, double-fire, or miss edge cases corrupt the metric invisibly and destroy trust in data.

### Data-First Definition

Defining metrics by available data rather than by the decision they inform produces numbers that cannot change anything.

### Ignoring The Denominator

Presenting counts without bases, or rates without volumes, hides the real story and enables misleading narratives.

### Definition Drift

Silent changes to filters, segments, or thresholds break historical comparisons and erode trust.

### Single-Metric Tunnel Vision

Optimizing one metric in isolation produces Goodhart effects where the metric improves and the outcome does not.

### Unvalidated Trust

Using a metric for decisions before confirming it measures what it claims invites systematic error.

## Self-Check

- [ ] Each metric has a precise written definition covering numerator, denominator, unit, window, filters, scope, and source.
- [ ] The metric is tied to a specific decision, action, threshold, owner, and review cadence.
- [ ] Leading, lagging, and diagnostic metrics are distinguished and combined rather than conflated.
- [ ] Instrumentation is defined to match the definition, with edge cases, properties, versioning, and staging tests.
- [ ] Raw counts are paired with their denominators and rates with their absolute volumes.
- [ ] A metric owner and catalog exist, with versioned definitions and documented change communication.
- [ ] The metric was validated against known scenarios and independently reproduced before driving decisions.
- [ ] The metric's exclusions, biases, edge cases, and complementary metrics are documented.
- [ ] The metric resists Goodhart effects by being paired with counter-metrics and outcome checks.
- [ ] No vanity metric that cannot change a decision is presented as a key result.
