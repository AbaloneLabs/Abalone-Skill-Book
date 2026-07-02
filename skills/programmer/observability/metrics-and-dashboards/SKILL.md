---
name: metrics_and_dashboards.md
description: Use when the agent is designing metrics, choosing metric types (counter, gauge, histogram, summary), instrumenting a service, building or reviewing dashboards, defining SLOs and their visualization, applying RED or USE methods, or diagnosing metric cardinality explosions and storage cost. Covers the metric types and when each fits, label cardinality as the dominant cost driver, dashboard design for incident response versus exploration, SLO visualization and error budgets, the RED (Rate, Errors, Duration) and USE (Utilization, Saturation, Errors) frameworks, avoiding metric explosion, naming conventions, and the discipline of instrumenting for the questions you will actually ask. Also use when metrics cost is growing without bound, when a dashboard is unreadable in an incident, when a counter resets or a histogram is mis-aggregated, or when SLOs are defined without error budgets.
---

# Metrics And Dashboards

Metrics are how a system observes itself at scale, and their central tradeoff is between information and cost. A metric is an aggregated number describing some aspect of the system over a window of time, and its value is that it is cheap to collect, store, and query at high cardinality of time — you can chart a counter across a year in milliseconds. Its cost is that aggregation discards detail: a histogram of request latency tells you the distribution, but a pre-aggregated average tells you almost nothing, and a metric labeled by user id tells you everything but bankrupts the storage. The recurring failure is designing metrics and dashboards without understanding this tradeoff, producing either metrics too aggregated to diagnose anything (averages hiding tail latency, counters without labels to slice by) or metrics so high-cardinality they explode storage cost (a label per user, per request path, per version, multiplying into millions of series). Dashboards inherit the same failure: a wall of charts optimized for looking comprehensive rather than for answering the question an on-call engineer actually has at 3am.

Agents tend to instrument by reflex (add a counter, add a histogram) without asking what question the metric answers or what cardinality it implies. The defects live in the choices: a latency metric recorded as an average instead of a histogram, so the tail is invisible; a label whose value space is unbounded (user id, URL with query parameters), so the series count grows without limit; a dashboard that shows fifty charts but no clear "is the system healthy?" signal; an SLO defined as a target with no error budget, so every violation is equally urgent and none is actionable. The judgment problem is treating metrics and dashboards as answers to specific, pre-identified questions — what is the system's health, where is the latency, who is affected — designed backward from those questions, with cardinality and aggregation chosen deliberately to balance information against cost.

This skill is about designing metrics and dashboards that answer real questions at sustainable cost. It complements the structured-logging skill (high-detail events) and the tracing skill (request-level causality); here the question is aggregated, time-series observability and its visualization.

## Core Rules

### Choose The Metric Type For The Question

The four metric types answer different questions, and choosing the wrong type loses information that cannot be recovered later.

- **Counter** monotonically increases (requests served, bytes sent, errors). Use for cumulative quantities; the interesting value is the rate of change (requests/sec), computed by differencing over time. Never use a counter for a value that can decrease; use a gauge.
- **Gauge** goes up and down (queue depth, memory in use, active connections). Use for instantaneous state. Sampled at intervals, so it captures a snapshot, not a complete history; rapidly-fluctuating gauges may miss extremes between samples.
- **Histogram** samples observations into buckets (request latency, response size) and enables computing percentiles (p50, p90, p99). Use for distributions where the average is misleading and the tail matters. The bucket boundaries determine the accuracy of percentiles — choose them for the values you care about.
- **Summary** (in some systems) computes percentiles client-side. Avoid in favor of histograms for aggregatable percentiles, since summary percentiles cannot be correctly averaged across instances — a histogram can be merged, a summary cannot.

The cardinal rule: if you care about a distribution (especially latency), use a histogram, never an average. An average hides the tail that determines user experience.

### Control Label Cardinality As The Dominant Cost Driver

Labels let you slice a metric (by route, by status code, by instance), and each unique combination of label values is a separate time series. Cardinality — the number of distinct series — is the dominant cost in metrics storage and query, and unbounded labels explode it.

- **Bound every label's value space.** A label with a bounded value space (a handful of status codes, a fixed set of routes, a known set of instances) is safe; a label with an unbounded space (user id, session id, full URL, exception message, IP address) produces a series per distinct value and grows without limit.
- **Multiply cardinalities, don't add them.** A metric with labels of cardinality 10, 20, and 100 has 20,000 series, not 130. Each label multiplies the series count; adding a high-cardinality label to an existing metric multiplies its entire series count.
- **Never label by unbounded identifiers.** User id, request id, session id, IP, full URL with query string, stack trace — these produce unbounded cardinality and must not be metric labels. Put them in logs and traces, where high detail is expected and cost is managed differently.
- **Monitor series count.** Track the number of active series; a sudden spike (a new label value, a deployment that introduced a new route) can multiply cost overnight. Alert on series-count growth.

### Design Dashboards For The Question, Not For Coverage

A dashboard's purpose is to answer a question fast — usually "is the system healthy, and if not, where?" — and its design should serve that question, not display every available metric.

- **Lead with the health signal.** The top of an incident dashboard should answer "is it healthy?" in one glance: the SLO status, the error rate, the key latency. An engineer under pressure should not hunt through fifty charts to learn whether to page.
- **Use the RED method for services.** For request-driven services, chart Rate (requests/sec), Errors (error rate or count), and Duration (latency, as a histogram with percentiles). These three answer "is the service handling traffic, correctly, and fast?" and are the core of service observability.
- **Use the USE method for resources.** For resources (CPU, disk, network, memory), chart Utilization (how busy), Saturation (how much queued), and Errors (device errors). These answer "is the resource overloaded or failing?"
- **Drill down, don't sprawl.** A dashboard that tries to show everything shows nothing well. Lead with the summary, then provide drill-downs (by route, by instance, by tenant) for diagnosis. Separate the "is it healthy" dashboard from the "what is wrong" exploration view.
- **Optimize for the on-call engineer at 3am.** A dashboard that is clear to its author in daylight may be opaque under incident stress. Favor obvious signals, clear thresholds, and minimal cognitive load.

### Define SLOs With Error Budgets, Not Bare Targets

A Service Level Objective is a target reliability (e.g., 99.9% of requests succeed with latency under 500ms over 30 days), and its power comes from the error budget — the remaining allowance of unreliability (0.1% of requests, in the example) that the system can "spend" before action is required.

- **Define the SLO in terms of user experience.** What does the user consider good? A latency threshold that bounds the experience, a success rate that bounds reliability. The SLO is about the user, not about internal metrics.
- **Compute the error budget and track its burn rate.** The error budget is the allowed unreliability over the window; the burn rate is how fast it is being consumed. A fast burn (even before the SLO is breached) is an early signal; budget exhaustion is the breach.
- **Make the error budget actionable.** When the budget is burning fast, the response is defined in advance (slow down feature work, freeze deploys, page). An SLO without a budget-driven response is a number, not a tool.
- **Distinguish SLOs from SLAs and indicators.** The SLI is the measurement (latency, error rate); the SLO is the target (99.9% under 500ms); the SLA is the contractual consequence of missing it. Do not conflate them.

### Instrument For The Questions You Will Actually Ask

Instrumentation should be driven by the questions the system will need to answer, not by what is easy to measure. The failure is instrumenting everything and answering nothing.

- **Identify the questions first.** What is the system's health? Where is latency? Who is affected? What changed? Then instrument to answer each, with the right metric type and labels.
- **Cover the four golden signals.** For any service: latency, traffic, errors, saturation. These answer the core health questions and should always be present.
- **Name metrics consistently.** A naming convention (unit in the name, consistent prefixes, lowercase with underscores) makes metrics findable and prevents the same quantity being measured under three names. Include the unit (`request_duration_seconds`, not `request_duration`).
- **Document what each metric means.** A metric whose meaning is unclear will be misread. Document the type, unit, labels, and what question it answers, so the on-call engineer interprets it correctly.

### Avoid Metric Explosion By Choosing The Right Tool Per Question

Not every question is a metrics question. Some are better answered by logs or traces, and forcing them into metrics causes cardinality explosions or information loss.

- **Metrics for aggregated, time-series health.** Is the system up, how fast, how error-prone, how loaded — these are metric questions, answerable across long time ranges cheaply.
- **Logs for individual events and detail.** What happened in this specific request, what was the error message, what was the user's input — these are log questions, where high detail per event is expected and volume is managed by sampling and retention.
- **Traces for request-level causality across services.** Where did this request spend its time, which service was slow, what did it call — these are trace questions.
- **Do not force high-detail questions into metrics.** A metric labeled by request id to "see each request" is a misuse that explodes cardinality; that detail belongs in logs or traces. Choose the tool for the question.

## Common Traps

### Averaging Latency Instead Of Histogramming

Recording latency as an average, hiding the tail that determines user experience and masking the slow requests that indicate the problem. Use a histogram; report percentiles; never average a distribution you care about.

### Unbounded Label Cardinality

Labeling by user id, session id, IP, full URL, or exception message, producing a series per distinct value and exploding storage cost. Bound label value spaces; put unbounded detail in logs and traces.

### Summary Percentiles Averaged Across Instances

Using summary metrics (client-side percentiles) and averaging them across instances, which is mathematically invalid and hides the true distribution. Use histograms, which can be merged correctly across instances.

### Dashboard That Shows Everything And Answers Nothing

A wall of charts optimized for coverage rather than for the on-call question, leaving the engineer to hunt for the health signal under stress. Lead with the health signal; use RED/USE; separate summary from drill-down.

### SLO Without An Error Budget

A reliability target with no error budget or burn-rate tracking, so every violation is equally urgent and none is actionable. Define the SLO, compute the budget, track the burn rate, and define the budget-driven response.

### Counter Used For A Decreasing Value

Using a counter for a quantity that can decrease (queue depth, memory), producing nonsense when the rate is computed. Use a gauge for values that go up and down; counters are monotonic.

### Misreading A Counter Reset

A counter that resets (process restart, instance replacement) appearing as a huge negative rate spike when differenced naively. Use a rate function that handles resets (treating decreases as resets), not naive differencing.

### Forcing High-Detail Questions Into Metrics

Labeling metrics by request id or stack trace to capture per-event detail, exploding cardinality for information that belongs in logs or traces. Match the tool to the question; metrics are for aggregated health, not individual events.

## Self-Check

- [ ] Each metric uses the type that fits its question: counters for monotonic cumulative quantities (with rate computed by reset-aware differencing), gauges for instantaneous up/down state, histograms for distributions where the tail matters (latency never averaged), and summaries avoided in favor of aggregatable histograms.
- [ ] Every label's value space is bounded (status codes, fixed routes, known instances), no label uses unbounded identifiers (user/session/request id, IP, full URL, stack trace), and the multiplicative series count is monitored for unexpected growth.
- [ ] Dashboards lead with the health signal (SLO status, error rate, key latency) answerable in one glance, use RED for services and USE for resources, separate the "is it healthy" view from the "what is wrong" drill-down, and are optimized for an on-call engineer under incident stress.
- [ ] SLOs are defined in terms of user experience, with computed error budgets, tracked burn rates (fast burn as an early signal), and pre-defined budget-driven responses — not bare targets where every violation is equally urgent.
- [ ] Instrumentation is driven by the questions the system will answer (the four golden signals — latency, traffic, errors, saturation — are always present), metrics are named consistently with units in the name, and each metric's type, unit, labels, and answered question are documented.
- [ ] High-detail questions (per-request events, error messages, causality across services) are routed to logs and traces rather than forced into metrics, and metrics are reserved for aggregated, time-series health answerable cheaply across long ranges.
- [ ] Counter resets (process restarts, instance replacements) are handled by reset-aware rate functions, not naive differencing that produces nonsense spikes.
- [ ] The metrics system's cost (series count, storage, ingestion) is monitored and bounded, with alerting on cardinality growth, so observability does not become an unbounded expense.
