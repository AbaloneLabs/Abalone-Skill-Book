---
name: observability_and_slo_design.md
description: Use when the agent is designing observability or monitoring for a service, instrumenting metrics logs and traces (the three pillars), defining SLOs SLIs and error budgets, deciding what to alert on, choosing between symptom-based and cause-based alerting, avoiding cardinality explosions in metrics, designing dashboards that are actually used, preventing alert fatigue and alerts that cry wolf, or distinguishing monitoring of known unknowns from observability of unknown unknowns. Covers the observability and SLO judgment problem — what to measure, what to alert on, how to bound cardinality, and how to make signals actionable — distinct from network and infrastructure topology.
---

# Observability And SLO Design

Monitoring and observability are spoken of as if they were the same, and the difference is not academic. Monitoring is the instrumentation of the known unknowns — the questions you knew to ask in advance, encoded as metrics, thresholds, and dashboards. Observability is the capacity to answer the unknown unknowns — the questions you could not have predicted, asked after the system is already behaving strangely, by exploring high-cardinality, high-context signals. A system can be heavily monitored and have no observability: a hundred dashboards, a thousand alerts, and no way to ask "why is this one user's requests failing right now" without shipping a new metric and waiting for it to populate. The failure shows up exactly when it hurts — during an incident, when the question is novel and the existing instrumentation cannot answer it.

Agents tend to reach for the metrics-first, dashboard-heavy pattern that the tooling makes easy: instrument every endpoint with latency and count, build a dashboard per service, set a threshold alert on error rate, and declare the service observable. Each step is defensible in isolation and insufficient in aggregate. A dashboard nobody looks at detects nothing; an alert that fires on every transient spike trains the team to ignore it; a metric with unbounded labels (one per user, per request path, per container) explodes the storage cost until the metrics backend refuses to ingest; an error-rate alert tells you that something is wrong but not what, where, or for whom. The discipline of observability is to design the signals so that they are actionable — so that an alert means a human must act, a dashboard answers a recurring question, and the trace and log fabric can answer the question nobody predicted. Without that discipline, observability becomes a cost center that produces noise and no insight.

This skill is about what to measure, what to alert on, and how to keep the signal fabric usable as the system grows. It is distinct from network and infrastructure topology (`network-and-security-groups`), from how infrastructure is delivered (`infrastructure-as-code-design`), and from workload configuration (`kubernetes-deployment-design`). Those decide what runs. This skill decides how you know whether it is running correctly, and how you find out when it is not.

## Core Rules

### Instrument All Three Pillars, And Know What Each One Is For

Metrics, logs, and traces are not interchangeable, and a service that has only one of them is blind in the dimensions the others cover. Each pillar answers a different class of question, and the strength of observability comes from their correlation.

- **Metrics answer aggregate questions over time.** How many requests, at what latency, with what error rate, trending how. Metrics are cheap to store and query at aggregate, which is why they are the basis for alerting and SLOs — but they are low-context, and a metric spike tells you *that* something changed, not *why*. Instrument the standard RED metrics (Rate, Errors, Duration) for every service, and the USE metrics (Utilization, Saturation, Errors) for every resource.
- **Logs answer detailed questions about specific events.** A log line carries the context a metric cannot — the request id, the user, the parameters, the error message. Logs are expensive to store and search, which is why they should be structured (machine-parseable fields, not free text) and sampled judiciously under high volume. The log is what you reach for when a metric tells you something is wrong and you need the specifics.
- **Traces answer questions about a request's path across services.** A trace follows a single request through every service it touched, with timing and context for each span, which is the only signal that localizes a latency problem to the service and the hop that caused it. Without distributed tracing, a cross-service latency regression is diagnosed by guessing. Trace at least a sample of requests, and ensure trace context (traceparent, baggage) propagates through every hop including asynchronous and queue-based boundaries.
- **Correlate the pillars through shared identifiers.** A metric spike should link to the logs and traces for the affected window; a trace should carry the ids that join to log lines; a log line should carry the trace id. The value of three pillars is multiplicative when they join, and merely additive when they do not. Propagate a request id and trace id through every boundary.

### Define SLOs From The User's Experience, And Let Error Budgets Govern Releases

A Service Level Objective is a promise about the service's reliability expressed as a measurable target, and it is the mechanism that turns "we want it to be reliable" into an engineering decision. Without an SLO, reliability is an unaccountable aspiration; with one, it is a budget that can be spent.

- **Define the SLI from the user's perspective, not the system's.** A good Service Level Indicator measures what the user experiences: the fraction of requests that succeeded within an acceptable latency, not the CPU utilization of a server. "99% of requests return a successful response within 500ms" is an SLI; "CPU below 80%" is an internal metric that may or may not relate to user experience. Choose indicators that would be wrong to violate — if the SLI is met but users are unhappy, the SLI is wrong.
- **Set the SLO target based on what users actually need and what the system can sustain.** A 99.99% availability target is roughly 52 minutes of downtime per year and demands engineering effort, redundancy, and operational maturity that most services cannot justify. A 99% target is roughly 7 hours of downtime per month and is trivially met by a robust single-region design. Match the target to the requirement, and do not inherit "five nines" from aspiration — every nine costs an order of magnitude more.
- **Use the error budget as a release and risk governor.** The error budget is the complement of the SLO: if the SLO is 99.9% success over a window, the budget is the 0.1% of requests that may fail. When the budget is healthy, the team can ship aggressively and take risks; when the budget is depleted, the team must stabilize and stop shipping changes that risk further failure. This converts reliability from a political argument into a quantitative one, and it is the single most valuable output of an SLO program.
- **Review and recalibrate SLOs against reality.** An SLO that is always met with huge margin is too loose and is hiding capacity for faster feature delivery; an SLO that is always violated is either too tight or reflects a system that cannot meet its promises. Recalibrate on a cadence so the targets track both user expectations and system capability.

### Alert On Symptoms, Not On Causes

The most common alerting failure is alerting on causes — CPU is high, disk is filling, a queue is long — rather than on symptoms — users are seeing errors, requests are slow. The distinction determines whether alerts are actionable.

- **A symptom is a user-visible degradation; a cause is an internal condition that may or may not produce one.** Alerting on symptoms means the alert fires when users are actually affected, which is when a human must act. Alerting on causes means the alert fires whenever an internal metric crosses a line, regardless of whether users notice, which produces alerts that are usually ignorable — and ignorable alerts train the team to ignore all alerts, including the ones that matter.
- **Page on symptoms, investigate with causes.** The on-call engineer should be paged when the SLO is burning (errors or latency breaching the user-facing target), not when CPU crosses 80%. The internal metrics — CPU, memory, queue depth, saturation — are the investigation surface used to find *why* the symptom is occurring, not the trigger. Use dashboards and lower-urgency alerts for causes; reserve paging for symptoms.
- **Make every alert actionable and owned.** An alert that fires and for which the response is "there is nothing to do" is a false positive by definition, regardless of whether the underlying condition is real. Every alert must have a documented response, an owner, and a threshold chosen so that it fires only when action is needed. Alerts without a runbook are noise waiting to become an incident.
- **Avoid alerting on absolute thresholds without context.** "Latency above 500ms" ignores that latency is higher during peak load, higher for certain endpoints, and naturally variable. Prefer alerts on SLO burn rate (how fast the error budget is being consumed) over absolute thresholds, because burn rate normalizes for the target and the window.

### Bound Metrics Cardinality, Or The Backend Becomes Unusable Or Unaffordable

Cardinality is the number of distinct label combinations a metric can take, and it is the silent killer of metrics backends. A metric labeled by user id, request id, or container id has unbounded cardinality, and unbounded cardinality means storage and query cost grow without limit until ingestion fails.

- **Never put high-cardinality values in metric labels.** User ids, request ids, session ids, container names, and full URLs are data for logs and traces, not labels for metrics. A metric with a label per user has as many series as users, and a spike in users becomes a spike in series that the backend cannot store or query. Put aggregates in metrics; put identities in logs and traces.
- **Bound every label set deliberately.** For each label, ask how many distinct values it can take across the lifetime of the metric, and multiply across all labels to estimate the series count. If the product is unbounded or very large, drop or bucket the label — group status codes into classes, bucket latency rather than recording per-value, cap the number of labeled endpoints.
- **Prefer histograms over per-bucket counters for distributions.** Latency should be recorded as a histogram with defined buckets (or a summary with quantiles), not as one counter per observed value. Histograms aggregate into a bounded series count while still supporting percentile queries; per-value counters explode.
- **Monitor the metrics backend's own cardinality and cost.** A metrics system that becomes expensive or slow is often the first sign of a cardinality leak — a new label added in a release that quietly multiplied the series count. Track series count and ingestion cost as operational metrics, and investigate spikes.

### Design Dashboards And Alerts For Use, Not For Coverage

A dashboard that exists because "we should have a dashboard for this" is a dashboard nobody will look at, and it consumes attention and screen space that could go to a dashboard that answers a real question. The same applies to alerts: coverage is not value.

- **Every dashboard should answer a specific recurring question.** "Is the service healthy?" "Is the SLO at risk?" "Where is latency being spent?" A dashboard without a question is decoration. Build dashboards for the questions on-call engineers and service owners actually ask, and retire dashboards that no longer answer a live question.
- **Dashboards should lead with the user-facing signal.** The top of the dashboard is the SLO, the error rate, the latency — the symptoms. The supporting internal metrics — CPU, saturation, queue depth — are below, as context for investigation. A dashboard that leads with CPU and buries the error rate is organized for the system, not for the user.
- **Alerts that cry wolf destroy the alerting system.** An alert that fires frequently and is usually ignorable trains the team to mute or ignore the channel, so that when a real alert fires it is missed. Aggressively tune or remove alerts that produce noise; a smaller set of high-signal alerts is strictly more valuable than a large set of low-signal ones. Measure alert quality — the fraction of alerts that led to action — and cull the rest.
- **Distinguish monitoring (known unknowns) from observability (unknown unknowns) in what you build.** Dashboards and threshold alerts handle the known unknowns — the failures you predicted. Tracing, structured logs, and ad-hoc query capability handle the unknown unknowns — the novel failures you did not predict. A system with only dashboards can answer the questions you thought to ask in advance; a system with rich traces and logs can answer the question you discover at 3am. Invest in both, and recognize that the second is what you reach for during the incidents that matter.

## Common Traps

### Dashboards Built For Coverage That Nobody Actually Uses

A dashboard per service, full of every available metric, created because "we should have one," and opened by no one. Each dashboard consumes attention and maintenance and answers no question. Build dashboards for specific recurring questions, lead with the user-facing signal, and retire the ones that no longer serve a question.

### Alerts On Causes (CPU, Disk) That Cry Wolf

The alert fires whenever CPU crosses a line, regardless of whether users are affected, so the team learns to ignore it — and then misses the alert that mattered. Page on symptoms (SLO burn, user-facing errors and latency); use internal metrics as investigation context, not paging triggers.

### Unbounded Cardinality From User-Id Or Request-Id Labels

A metric labeled by user or request id creates one series per user or per request, and the metrics backend's cost and query time grow until ingestion fails or the bill becomes untenable. Keep high-cardinality values in logs and traces; bound every metric label set deliberately.

### An SLO Inherited From Aspiration Rather Than Need

"Five nines" copied from a template, with no analysis of what users need or what the system can sustain, producing either a target that is always violated (and thus ignored) or one met with such margin that it hides room for faster delivery. Set SLOs from user need and measured capability, and recalibrate against reality.

### One Pillar Without The Others, With No Correlation

Metrics without traces and logs can show *that* latency spiked but not *where* or *why*; logs without traces show individual events but not the request's path; traces without metrics cannot alert. Instrument all three and join them through shared ids, so a signal in one leads to the others.

### Alerting On Absolute Thresholds Without SLO Context

"Latency above 500ms" fires during normal peak load and stays silent during a real regression on a low-volume endpoint. Alert on SLO burn rate, which normalizes for the target and the window, rather than on absolute thresholds that ignore context.

### Treating Observability As A Cost Center Rather Than A Capability

Metrics, logs, and traces are seen as overhead to be minimized, so sampling is aggressive, retention is short, and the system cannot answer novel questions during incidents. Observability is the capability that determines how fast incidents are resolved; under-investing in it is paid for in every future outage's duration.

## Self-Check

- [ ] All three pillars are instrumented — RED metrics (Rate, Errors, Duration) for services and USE metrics (Utilization, Saturation, Errors) for resources, structured logs carrying request and trace ids, and distributed traces sampled across every hop including asynchronous and queue boundaries — and they are correlated through shared identifiers so a signal in one leads to the others.
- [ ] SLIs are defined from the user's perspective (fraction of requests succeeding within acceptable latency, not internal CPU or disk), SLO targets are set from user need and measured system capability rather than aspiration, and error budgets govern release cadence and risk-taking (ship aggressively when the budget is healthy, stabilize when it is depleted).
- [ ] Paging alerts fire on symptoms (SLO burn rate, user-facing errors and latency), not on causes (CPU, disk, queue depth), every alert has a documented response and an owner, and alerts that produce no action are tuned or removed so the channel retains signal.
- [ ] No metric carries high-cardinality labels (user id, request id, container id, full URL), every label set was bounded by estimating lifetime series count, latency is recorded as histograms rather than per-value counters, and the metrics backend's series count and ingestion cost are monitored for cardinality leaks.
- [ ] Dashboards answer specific recurring questions, lead with the user-facing signal (SLO, errors, latency) with internal metrics as supporting context below, and dashboards that no longer answer a live question are retired rather than accumulated.
- [ ] Alert quality is measured (the fraction of alerts that led to action), low-signal alerts are culled in favor of a smaller high-signal set, and the system distinguishes monitoring of known unknowns (dashboards, threshold alerts) from observability of unknown unknowns (traces, structured logs, ad-hoc query) with investment in both.
- [ ] SLOs are reviewed and recalibrated on a cadence against actual user satisfaction and system capability, so targets that are always met with large margin are tightened and targets that are always violated are either corrected or acknowledged as beyond current capability.
- [ ] Sampling, retention, and query capability for logs and traces are sufficient to answer novel questions during incidents (not aggressively minimized as overhead), recognizing that observability is the capability that determines incident resolution time.
