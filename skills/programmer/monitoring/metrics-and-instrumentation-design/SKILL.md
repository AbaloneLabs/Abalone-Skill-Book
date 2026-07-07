---
name: metrics_and_instrumentation_design.md
description: Use when the agent is designing or reviewing application instrumentation and metrics — choosing what to measure (RED, USE, business metrics), selecting metric types (counter, gauge, histogram, summary), defining labels and cardinality, deciding collection interval and aggregation, building dashboards, or ensuring metrics support alerting and capacity planning. Also covers the failure modes of unbounded label cardinality exploding storage, metrics that measure activity instead of outcomes, missing units or dimensions that make data uninterpretable, pre-aggregating away the ability to diagnose, and the recurring mistake of instrumenting everything and drowning in low-signal metrics while the few metrics that matter are absent or misnamed.
---

# Metrics And Instrumentation Design

Instrumentation is the foundation of observability: the metrics you emit are the only signals you will have when something goes wrong, and the metrics you fail to emit are blind spots no dashboard or alert can fill. The judgment problem is that instrumenting "everything" produces a flood of low-signal metrics that drown the few that matter, while instrumenting naively produces metrics that are technically present but useless for diagnosis — a counter with no labels to slice by, a latency metric pre-aggregated to an average that hides the tail, a label whose unbounded cardinality explodes storage, a metric named "requests" with no unit or scope that no one can interpret. Good instrumentation is selective (it measures what matters for the service's health and its users), well-typed (each metric is the right type for what it represents), well-labeled (sliced by the dimensions needed to localize a problem, with cardinality bounded), and designed for the alerting and diagnosis it will support, not for the appearance of coverage.

Agents tend to under-invest here because emitting a metric is easy (increment a counter, record a duration) and the dashboard looks full. The harm appears at diagnosis time. An outage strikes and the team cannot tell which subset of users or requests are affected, because the metrics were not labeled by the relevant dimensions. A latency regression is reported as "average latency up 10%" with no visibility into whether it is the p50, p95, or p99, or which endpoint, because the metric was pre-aggregated to an average. A label on user ID or request URL pushes cardinality into the millions and the metrics backend degrades or bills explode. A business metric (conversion, active users) is absent, so the team monitors infrastructure health while the user-facing outcome degrades unnoticed. The judgment problem is to choose metrics that measure outcomes and health, type and label them for diagnosis, bound cardinality, and preserve the resolution needed to localize problems — treating instrumentation as a designed system, not an afterthought.

This skill covers what to measure (RED/USE/business), metric types, labels and cardinality, aggregation and resolution, and designing for alerting and dashboards. It complements the alerting-design skill (turning metrics into alerts), the slo-and-error-budget skill (the metrics that define reliability targets), and the distributed-tracing skill (request-scoped observability). Here the focus is the metrics themselves — what to emit, how to type and label them, and how to keep them useful.

## Core Rules

### Measure Outcomes And Health, Not Just Activity

The first decision is what to measure. Instrumenting every internal function produces noise; the goal is metrics that reflect user impact and system health:

- **Start with the RED method for request-driven services.** Rate (requests per second), Errors (failed requests), and Duration (latency) per endpoint or service are the core signals of user-facing health. Every user-facing service should have these.
- **Add the USE method for resources.** Utilization, Saturation, and Errors for resources (CPU, memory, disk, network, connection pools, queues) reveal when a resource is the bottleneck. USE catches capacity and saturation problems RED cannot.
- **Include business and outcome metrics.** Infrastructure health is a proxy for user impact; measure the outcomes directly (active users, conversions, successful operations, queue depth of user-visible work) so a degradation in the actual product is visible, not only a degradation in infrastructure.
- **Measure what the user experiences, from where they experience it.** Client-side or edge metrics (observed latency, error rates from the user's perspective) catch problems server-side metrics miss, because the user's experience includes network, client, and edge effects.

### Choose The Right Metric Type For What You Are Measuring

Metric type determines what you can compute from the metric later. Choosing the wrong type loses information permanently:

- **Counter for monotonically increasing values.** Use counters for cumulative totals (total requests, total errors, total bytes sent). Counters support rate calculations (requests/sec) over time windows. Do not use a counter for a value that can decrease.
- **Gauge for a value that goes up and down.** Use gauges for instantaneous snapshots (current queue depth, active connections, memory in use, temperature). Gauges reflect current state, not cumulative totals.
- **Histogram for distributions (latency, sizes).** Use histograms to capture the distribution of values across many observations, enabling percentile calculations (p50, p95, p99). This is essential for latency, where the tail (p99) behaves very differently from the average. Never reduce latency to an average at collection time — you lose the tail permanently.
- **Summary as an alternative for quantiles on the client side.** Summaries compute quantiles client-side, avoiding the histogram's bucket-boundary issues, but are not aggregatable across instances (you cannot meaningfully average pre-computed quantiles). Prefer histograms for distributed systems where aggregation across instances is needed.

### Label By The Dimensions Needed To Localize, With Bounded Cardinality

Labels (dimensions) are what let you slice a metric to localize a problem — by endpoint, status code, region, error type. They are also the primary source of metric explosion:

- **Label by the dimensions you will diagnose by.** If an outage investigation will ask "which endpoint? which region? which error type?", those must be labels, or the metric cannot answer. Anticipate the slicing questions and label for them.
- **Bound label cardinality strictly.** Each unique combination of label values is a separate time series; high-cardinality labels (user ID, request URL with parameters, session ID) explode the time series count, degrading the backend and inflating cost. Never put unbounded identifiers (user ID, full URL) in labels; bucket or omit them.
- **Prefer a small set of low-cardinality labels over many high-cardinality ones.** Endpoint name, HTTP status class, region, and error category are low-cardinality and high-value; user ID and request ID are high-cardinality and belong in logs or traces, not metrics.
- **Move request-scoped detail to tracing or logging.** The specific user, the specific request ID, the specific parameters — these are request-scoped and high-cardinality; capture them in traces and logs (where they are indexed per-request), not in metrics (where they explode time series).

### Preserve Resolution: Do Not Pre-Aggregate Away Diagnosability

How you aggregate at collection time determines what you can diagnose later. Pre-aggregating to a summary statistic destroys the detail needed to investigate:

- **Do not average latency at collection time.** An average hides the tail; a regression in the p99 (affecting a subset of users) is invisible in an average that barely moves. Emit a histogram and compute percentiles at query time.
- **Choose histogram buckets to resolve the percentiles you care about.** Bucket boundaries determine which percentiles are accurate; buckets that are too coarse cannot resolve p99, and too many fine buckets waste storage. Choose buckets that resolve the SLO-relevant percentiles.
- **Choose collection interval to match the phenomenon.** A 60-second interval smooths over sub-minute spikes; a 10-second interval reveals them but costs more. Match the interval to the phenomena you need to detect (fast for acute spikes, coarser for trend analysis).
- **Keep raw counts, derive rates at query time.** Store cumulative counters; compute rates (per-second) at query time. Pre-computed rates lose the ability to re-window or reconcile across scrape intervals.

### Name And Document Metrics So They Are Unambiguous

A metric's name and its unit and scope must be clear, or the metric will be misread:

- **Include the unit in the name or convention.** "request_duration" is ambiguous (seconds? milliseconds?); "request_duration_seconds" or a documented unit convention removes ambiguity. Mismatched units are a common cause of wrong dashboards and alerts.
- **Name by what is measured, with consistent conventions.** Consistent naming (a prefix for the service, a clear noun for the metric, a unit suffix) makes metrics discoverable and comparable; ad-hoc names produce a metrics surface no one can navigate.
- **Document what each metric means and its labels.** A metric without documentation is guessed at; record what it measures, its unit, its labels and their value sets, and what a change means. Documentation is what makes a metrics surface usable to anyone but its author.

### Design Instrumentation For Alerting And Dashboards, Not Coverage

Instrumentation exists to support decisions — alerts, dashboards, capacity planning. Design it for those uses:

- **Instrument for the alerts you will write.** If an alert will fire on error rate over a window, the error metric must exist at the right resolution and with the right labels. Design backward from the alerts and SLOs (see the slo-and-error-budget and alerting-design skills).
- **Make dashboards answer the likely questions.** A dashboard should answer "is the service healthy, and if not, where?" — RED/USE per service, business metrics, resource saturation. Avoid dashboards that are a wall of unrelated metrics with no diagnostic flow.
- **Avoid vanity metrics and duplicate coverage.** A metric that no alert or dashboard uses, or three metrics measuring the same thing differently, add noise and cost. Prune instrumentation that does not serve a decision.

## Common Traps

### Measuring Activity Instead Of Outcomes

Instrumenting every internal function and counting requests, while the business outcome (conversion, successful operations, user-visible failures) is unmeasured, so the team monitors activity while the product degrades. Measure outcomes and user-facing health (RED, USE, business metrics).

### Unbounded Label Cardinality Exploding Storage

Putting user ID, full URL, or session ID in labels, creating millions of time series that degrade the backend and inflate cost. Bound cardinality; move request-scoped detail to traces and logs.

### Pre-Aggregating Latency To An Average

Recording average latency at collection time, permanently hiding the tail, so a p99 regression affecting a subset of users is invisible. Emit histograms and compute percentiles at query time.

### Missing Units, Scope, Or Documentation

A metric named "requests" or "latency" with no unit, scope, or documentation, so it is misread or unusable by anyone but its author. Include units, use consistent naming, and document each metric and its labels.

### Missing Dimensions Needed To Localize

A metric with no labels (or the wrong labels), so an outage cannot be sliced by endpoint, region, or error type to localize the affected subset. Label by the dimensions you will diagnose by.

### Instrumenting Everything And Drowning In Noise

Emitting every possible metric, so the few that matter are buried and the backend is overloaded. Be selective; instrument for the alerts, SLOs, and diagnostic questions that matter, and prune what serves no decision.

### Wrong Metric Type For The Value

Using a counter for a value that decreases, a gauge for a cumulative total, or a summary where cross-instance aggregation is needed. Match the metric type to what is being measured and how it will be aggregated.

## Self-Check

- [ ] Metrics cover outcomes and health (RED for request-driven services, USE for resources, business/outcome metrics, and client/edge observed metrics), not only internal activity.
- [ ] Each metric uses the correct type (counter for monotonic totals, gauge for instantaneous values, histogram for distributions like latency), and latency is never pre-aggregated to an average — histograms preserve the tail for percentile computation.
- [ ] Labels cover the dimensions needed to localize problems (endpoint, status, region, error type) while cardinality is bounded (no user ID, full URL, or session ID in labels; request-scoped detail moved to traces/logs).
- [ ] Resolution is preserved: histograms with buckets that resolve SLO-relevant percentiles, collection interval matched to the phenomena, cumulative counters with rates derived at query time, no pre-aggregation that destroys diagnosability.
- [ ] Metrics are named with units and consistent conventions, and each metric and its labels are documented (what it measures, unit, label value sets, what a change means).
- [ ] Instrumentation is designed for the alerts, SLOs, and dashboards it supports (designed backward from the decisions), with vanity and duplicate metrics pruned, not emitted for the appearance of coverage.
- [ ] The highest-risk cases were verified — an outage that could be localized by labels, a p99 regression visible in a histogram, a high-cardinality label caught before explosion, and an outcome metric that caught a degradation infrastructure metrics missed — not only the wall of green dashboards.
