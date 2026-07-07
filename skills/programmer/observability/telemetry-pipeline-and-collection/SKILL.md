---
name: telemetry_pipeline_and_collection.md
description: Use when the agent is designing or implementing telemetry collection — instrumenting code for metrics, logs, and traces; choosing collection agents and pipelines (OpenTelemetry, Fluentd, Vector, Beats); routing, sampling, enriching, or filtering telemetry; managing telemetry volume and cost; correlating traces/logs/metrics; or building the pipeline that moves observability data from services to a backend. Covers instrumentation strategy, the metrics/logs/traces triad and when each applies, sampling and volume control, context propagation and correlation, pipeline architecture, and telemetry-as-data tradeoffs (cardinality, retention, cost).
---

# Telemetry Pipeline And Collection

Telemetry — the metrics, logs, and traces a system emits — is the raw material of observability, and the pipeline that collects, processes, and routes it is infrastructure with its own design tradeoffs. The central tension is volume versus value: a system that instruments everything at full fidelity produces a firehose of data that is expensive to store, slow to query, and hard to search for the signal within, while a system that instruments sparingly lacks the data to diagnose problems. A naive pipeline that ships every log line and every trace to a backend scales linearly with traffic and cost, and at high volume the telemetry itself becomes a reliability and cost problem — a busy service generating gigabytes of logs per minute can saturate its own network or exhaust its backend's ingestion capacity. The disciplined approach treats telemetry as data with a lifecycle: instrument strategically, sample to control volume, propagate context to correlate signals, and architect the pipeline to handle volume, cost, and failure gracefully.

Agents tend to instrument reactively (add logging everywhere when debugging, never remove it), to ship full-fidelity telemetry without sampling, and to treat the pipeline as a black box that "just sends data to the backend." The judgment problem is recognizing that telemetry is a data system with volume, cost, and reliability implications, that the three signal types (metrics, logs, traces) serve different purposes and have different volume profiles, and that the pipeline must be designed for sampling, correlation, and graceful degradation just like any other data infrastructure. This skill covers the discipline of telemetry collection and pipeline design: instrumentation strategy, the signal triad, sampling and volume control, context propagation, pipeline architecture, and the cost/cardinality/retention tradeoffs.

## Core Rules

### Instrument With The Three Signal Types Strategically

Metrics, logs, and traces serve different purposes, have different volume and cost profiles, and complement each other. Understanding when to use each is the foundation of instrumentation.

- **Metrics: aggregated, numeric, low-volume, for dashboards and alerting.** Metrics (counters, gauges, histograms) are pre-aggregated numbers (request count, error rate, latency percentile) that are cheap to store and fast to query. Use them for system-level health, dashboards, and alerting. They do not tell you what happened in a specific request, but they show aggregate behavior.
- **Logs: discrete events with context, for debugging specific occurrences.** Logs are timestamped records of events (a request, an error, a state change) with structured fields. Use them to understand what happened in a specific case. They are higher-volume than metrics and must be sampled or filtered at scale to control cost.
- **Traces: request-scoped journeys across services, for understanding distributed behavior.** A trace follows a single request through all the services it touches, with spans for each operation. Use them to understand where time is spent and where failures occur in a distributed request. Traces are the highest-volume signal per request and require sampling.
- **Instrument all three, at the right granularity for each.** A system with only metrics cannot debug specific failures; a system with only logs cannot alert on aggregate health; a system with only traces lacks cheap dashboards. Use all three, each at the granularity appropriate to its volume profile.

### Control Volume Through Sampling And Filtering

Telemetry volume scales with traffic, and uncontrolled volume is a cost and reliability problem. Sampling and filtering are the mechanisms that keep volume manageable while preserving signal.

- **Sample traces head-based or tail-based.** Head-based sampling (decide at request start whether to trace, e.g., 1% of requests) is simple and cheap but misses rare, important traces (errors, slow requests). Tail-based sampling (decide after the request completes, keeping all errors and slow requests) preserves important traces but is more complex. Use tail-based sampling where the infrastructure supports it.
- **Sample logs at high volume.** A service generating millions of log lines needs sampling (keep 1%, or keep all errors and sample info logs) or filtering (drop known-noisy log categories) to control volume. Always keep error and warning logs at full fidelity; sample debug and info logs.
- **Do not sample metrics.** Metrics are pre-aggregated and low-volume; sampling them loses fidelity without saving much. Emit metrics at full fidelity; sample the high-volume signals (logs, traces).
- **Filter at the source where possible.** Filtering telemetry before it leaves the service (dropping health-check requests, known-noisy endpoints) is cheaper than filtering in the pipeline. Filter early to reduce pipeline load.
- **Make sampling configurable and dynamic.** Sampling rates should be tunable at runtime (via config or feature flag) so volume can be adjusted during incidents (increase trace sampling to debug) or to control cost (decrease sampling when volume spikes).

### Propagate Context For Correlation Across Signals And Services

The value of telemetry multiplies when signals can be correlated — a metric spike linked to the traces of the requests that caused it, a log line linked to the trace of its request, a trace following a request across service boundaries. Correlation requires context propagation.

- **Propagate trace context (trace ID, span ID) across service calls.** A trace that follows a request through multiple services requires each service to propagate the trace context (via headers like W3C Trace Context). Without propagation, each service's spans are orphaned and the distributed journey is invisible.
- **Include the trace ID in log records.** A log line with the trace ID can be correlated to the trace of its request, linking the discrete event to the request journey. This is the key integration between logs and traces.
- **Include relevant context (request ID, user ID, tenant ID) in all signals.** Shared identifiers across metrics (labels), logs (fields), and traces (attributes) allow pivoting from one signal to another during investigation.
- **Use standard context propagation formats.** W3C Trace Context, Baggage, and OpenTelemetry's semantic conventions ensure context propagates across different libraries, languages, and backends. Avoid proprietary formats that break correlation.

### Architect The Pipeline For Volume, Failure, And Cost

The telemetry pipeline — the agents, collectors, and transport that move data from services to the backend — is infrastructure that must handle volume, tolerate failure, and control cost, like any data pipeline.

- **Use a local collector/agent on each host, forwarding to a central pipeline.** A local agent (Fluentd, Vector, OpenTelemetry Collector) buffers telemetry from the service, handles retries, and forwards to a central pipeline or backend. This decouples the service from the backend and provides a buffer against backend unavailability.
- **Buffer and retry to tolerate backend unavailability.** If the backend is down or slow, the pipeline should buffer telemetry locally (on disk) and retry, rather than dropping it or blocking the service. The buffer prevents telemetry loss during backend outages.
- **Do not let telemetry pipeline failure affect the service.** Telemetry collection must not block or crash the service it observes. Use async emission, bounded queues (with drop-oldest on overflow), and circuit-breaking against a failing backend, so a telemetry problem does not become a service outage.
- **Control cardinality to control cost.** High-cardinality labels (user ID, request ID) on metrics explode the storage cost (each unique label combination is a separate time series). Keep metric labels low-cardinality (service, endpoint, status code); put high-cardinality data in logs and traces where it is expected.
- **Plan retention by signal type.** Metrics (low-volume, high-query) can be retained long (months to years); logs and traces (high-volume) are retained shorter (days to weeks) to control cost. Tiered retention (hot for recent, cold/archived for older) balances queryability and cost.

### Enrich And Process In The Pipeline

The pipeline can enrich telemetry (add metadata, normalize formats) and process it (filter, sample, route to different backends) before storage, centralizing logic that would otherwise be spread across services.

- **Enrich telemetry with infrastructure metadata.** The pipeline can add metadata the service does not have (cloud region, instance type, Kubernetes pod, deployment version) by inspecting the source, providing consistent attribution across all telemetry from a source.
- **Route different signals to different backends.** Metrics to a time-series database, logs to a log backend, traces to a tracing backend — the pipeline routes each signal type to the appropriate store, allowing best-of-breed backends.
- **Centralize sampling and filtering logic.** Sampling and filtering decisions can be made in the pipeline (tail-based sampling in the collector) rather than in each service, allowing consistent policy across services and easier adjustment.
- **Normalize formats and schemas.** Services emitting different log formats or metric conventions can be normalized in the pipeline to a consistent schema, simplifying querying and dashboarding.

## Common Traps

### Shipping Full-Fidelity Telemetry Without Sampling

Sending every log line and every trace to the backend, scaling cost and volume linearly with traffic until the backend is saturated or the bill is unmanageable. Sample traces and logs; keep metrics full-fidelity.

### No Context Propagation Across Services

Orphaned traces and logs that cannot be correlated across service boundaries, making distributed behavior invisible. Propagate trace context (W3C Trace Context) and include trace IDs in logs.

### High-Cardinality Metric Labels

Labels like user ID or request ID on metrics, exploding the number of time series and the storage cost. Keep metric labels low-cardinality; put high-cardinality data in logs and traces.

### Telemetry Pipeline Blocking Or Crashing The Service

Synchronous telemetry emission or unbounded queues that block the service when the backend is slow or down, turning an observability problem into an outage. Use async emission, bounded queues with drop, and circuit-breaking.

### No Buffering Against Backend Unavailability

A pipeline that drops telemetry when the backend is unavailable, losing the data needed to diagnose the outage. Buffer locally and retry.

### Over-Instrumentation Never Removed

Debug logging added during an incident and never removed, inflating volume permanently. Review and prune instrumentation; make sampling configurable.

### One Signal Type To The Exclusion Of Others

A system with only metrics (cannot debug specifics), only logs (cannot alert on aggregates), or only traces (lacks cheap dashboards). Instrument all three at appropriate granularity.

### Sampling That Misses Important Events

Head-based trace sampling that misses errors and slow requests, or log sampling that drops errors. Use tail-based sampling for traces; always keep errors at full fidelity.

## Self-Check

- [ ] All three signal types are instrumented at appropriate granularity — metrics (low-volume, aggregated) for dashboards and alerting, logs (discrete events) for debugging specifics, traces (request journeys) for distributed behavior — none to the exclusion of the others.
- [ ] Volume is controlled through sampling (tail-based for traces to preserve errors and slow requests, sampling for high-volume logs with errors kept at full fidelity, no sampling of low-volume metrics), filtering at the source, and dynamically configurable sampling rates.
- [ ] Context (trace ID, span ID, request ID, user/tenant ID) is propagated across service calls using standard formats (W3C Trace Context, OpenTelemetry semantic conventions), and trace IDs are included in log records so signals can be correlated during investigation.
- [ ] The pipeline uses local collectors/agents that buffer and retry to tolerate backend unavailability, does not block or crash the service (async emission, bounded queues with drop, circuit-breaking), and decouples the service from the backend.
- [ ] Metric labels are low-cardinality (service, endpoint, status code) to control time-series explosion and cost; high-cardinality data (user ID, request ID) is in logs and traces where it belongs.
- [ ] Retention is planned by signal type (metrics long, logs/traces shorter) with tiered storage (hot/cold) balancing queryability and cost.
- [ ] The pipeline enriches telemetry with infrastructure metadata, routes signals to appropriate backends, centralizes sampling/filtering logic, and normalizes formats for consistent querying.
- [ ] Instrumentation is reviewed and pruned over time (debug logging removed, noisy categories filtered), and the pipeline's volume, cost, and drop rates are themselves monitored so a telemetry problem is detected before it becomes a service or cost problem.
