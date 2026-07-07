---
name: elixir_telemetry_and_observability.md
description: Use when the agent is instrumenting an Elixir/Phoenix/OTP application for observability (Telemetry events, :telemetry.execute, Telemetry.Metrics, reporters like TelemetryMetricsPrometheus/StatsD, structured logging with Logger, distributed tracing with OpenTelemetry/Spandex), deciding what to measure (Phoenix request events, Ecto query events, Oban job events, VM metrics), attaching Telemetry handlers, configuring log levels and metadata, correlating traces across services, or is diagnosing "no metrics appear", "Telemetry handler crashes silently", "logs too noisy / missing context", "trace context lost across processes/HTTP", or observability gaps. Covers Telemetry events and handlers, metrics (counter/gauge/summary/distribution), structured logging, distributed tracing and context propagation, what to instrument, and the pitfalls of handler crashes, cardinality, and missing correlation.
---

# Telemetry And Observability In Elixir

Elixir's observability is built on `:telemetry` (a unified event-emission standard that Phoenix, Ecto, Oban, and the BEAM itself emit), `Telemetry.Metrics` (aggregations into counter/gauge/summary/distribution), the `Logger` (structured logging), and OpenTelemetry/Spandex (distributed tracing). Agents emit Telemetry events but forget to attach a handler (so nothing is measured), let a handler crash silently (a crashing handler is detached and future events are lost — handlers must be defensive), create high-cardinality metrics (a metric tagged by `user_id` or `request_id` explodes the series count and overwhelms the backend), lose trace context across a process boundary or HTTP call (context is propagated via process dictionary/headers; a raw `Task.async` or a missing header drops it), or log without the metadata (request_id, user_id) needed to correlate. The judgment problem is to emit and attach Telemetry deliberately, to choose metrics with bounded cardinality, to log structured context, and to propagate trace context across boundaries.

Agents emit without attaching, crash handlers, create high-cardinality metrics, or lose trace context. The remedy is defensive handlers, bounded-cardinality metrics, structured logging, and explicit context propagation.

## Core Rules

### Emit And Attach Telemetry Deliberately; Handlers Must Be Defensive

`:telemetry.execute([:my_app, :request, :stop], measurements, metadata)` emits an event; *nothing happens* unless a handler is attached with `:telemetry.attach`/`attach_many` or via `Telemetry.Metrics` reporters. Libraries (Phoenix, Ecto, Oban) emit standard events; attach to those rather than re-instrumenting. Critical rule: a handler that *raises* is detached by Telemetry and silently stops receiving events — handlers must be defensive (rescue, log the error, do not raise). Use `Telemetry.Metrics` (a higher-level aggregation layer) and a reporter (Prometheus, StatsD) to define metrics over events declaratively, rather than hand-rolling handlers for common cases.

- `:telemetry.execute` emits; attach a handler (`:telemetry.attach`) or use `Telemetry.Metrics` + a reporter.
- Attach to library-emitted events (Phoenix/Ecto/Oban) rather than re-instrumenting.
- Handlers must be defensive (a raise detaches the handler; future events lost).

### Choose Metrics With Bounded Cardinality (Counter/Gauge/Summary/Distribution)

`Telemetry.Metrics` defines metric types: `counter` (monotonic count), `last_value`/`gauge` (current value), `sum` (monotonic sum), `summary` (percentiles), `distribution`/`histogram` (bucketed). Tag metrics to slice them, but **cardinality is the killer**: a metric tagged by `user_id`, `request_id`, `email`, or any unbounded value creates one series per distinct value, exploding memory and overwhelming the backend (Prometheus especially). Tag only by low-cardinality dimensions (`endpoint`, `status_code`, `method`, `route`, `tenant_id` if bounded). For "per-user" insight, use logs/traces (which handle high cardinality), not metrics. Buckets for distributions should match the SLO-relevant ranges (e.g., HTTP latency buckets around p95/p99 thresholds).

- Metric types: counter/last_value/sum/summary/distribution — choose by what you measure.
- Tag only low-cardinality dimensions (`route`, `status`, `method`); never `user_id`/`request_id`/`email`.
- Per-user/per-request insight belongs in logs/traces, not metrics; set SLO-relevant histogram buckets.

### Log Structured Context (request_id, user_id, latency)

Logger supports structured metadata (`Logger.info("msg", request_id: rid, user_id: uid)`), and Phoenix injects `request_id` automatically via `Plug.RequestId`. Configure Logger to emit JSON in production (a JSON formatter) so logs are machine-parseable and correlatable. Include correlation IDs (`request_id`, a `trace_id`/`span_id` from tracing, `user_id`) in every log line so a request's logs can be found across services and processes. Set log levels deliberately (`:info` production, `:debug` dev/staging); avoid logging at `:debug` in production (volume/noise). Be careful logging sensitive data (PII, secrets) — redact or omit. Attach metadata via `Logger.metadata/1` for process-bound context (a GenServer's identity).

- Structured metadata (`request_id`, `user_id`, `trace_id`) in every log; JSON format in production.
- Phoenix `Plug.RequestId` for correlation; set levels deliberately (`:info` prod, `:debug` dev).
- Redact PII/secrets; use `Logger.metadata` for process-bound context.

### Propagate Trace Context Across Process And HTTP Boundaries

Distributed tracing (OpenTelemetry/Spandex) follows a request across services via a trace context (trace_id/span_id) propagated through headers (`traceparent`/`tracestate` for W3C, `b3` for Zipkin) on HTTP calls and through the process dictionary on the BEAM. Context is *lost* at boundaries unless propagated: a raw `Task.async`/`spawn` does not inherit the calling process's context (use `Task.async_stream` with tracing or explicit context passing); an outbound HTTP call (Tesla/Finch/HTTPoison) must inject the headers (via the OpenTelemetry/Tesla middleware); a message sent to another node does not carry context. Use the library's propagation (OpenTelemetry's `Tracer.with_span`, the Tesla/Phoenix/Ecto instrumentation) rather than manual headers. Verify end-to-end traces span services; a broken trace is usually a missing propagation middleware.

- Trace context propagates via headers (HTTP) and process dictionary (BEAM); it's lost at raw spawn/HTTP boundaries.
- Use library instrumentation (OpenTelemetry Tesla/Phoenix/Ecto middleware), not manual headers.
- `Task.async` does not inherit context; use traced task patterns or explicit passing.

### Decide What To Instrument Based On SLOs And Failure Modes

Instrument what matters for SLOs and debugging, not everything. Standard events to capture: Phoenix request latency/count/error rate (per route), Ecto query latency/count (per query/Repo), Oban job success/failure/duration (per queue), VM metrics (scheduler usage, memory, process count, GC), and business-specific events (orders processed, payments). Tie metrics to SLOs (p99 latency < X, error rate < Y%) so alerts fire on user-visible degradation, not raw resource usage. Use traces for slow-request diagnosis (which span in the request was slow), logs for individual-event detail, and metrics for aggregate trends. Each signal has a role; do not try to do everything with one.

- Instrument SLO-relevant events (request latency/error, query latency, job success, VM health, business events).
- Metrics for trends/alerts, traces for slow-request diagnosis, logs for individual detail.
- Tie alerts to SLOs (user-visible degradation), not raw resource noise.

## Common Traps

### Emitting Telemetry Without Attaching A Handler

Nothing is measured. Attach via `:telemetry.attach` or `Telemetry.Metrics` + reporter.

### Handler Crash Detaching Itself

A raise detaches the handler; future events lost. Handlers must rescue.

### High-Cardinality Metrics

Tagging by `user_id`/`request_id` explodes series. Tag low-cardinality dimensions; use logs/traces for per-user.

### Trace Context Lost Across Task/HTTP

Raw `Task.async` or missing HTTP headers drop the trace. Use library instrumentation/middleware.

### Logging Without Correlation IDs

Logs can't be tied to a request. Include `request_id`/`trace_id`/`user_id` metadata.

### Logging Secrets/PII

Tokens/passwords in logs leak. Redact; structure logs to omit sensitive fields.

### Over-Instrumenting (Noise)

Measuring everything overwhelms backends and obscures signal. Instrument SLO-relevant events.

### Broken Distributed Traces

A missing propagation middleware splits a trace across services. Verify end-to-end spans.

## Self-Check

- [ ] Telemetry events are emitted *and* a handler attached (`:telemetry.attach` or `Telemetry.Metrics` + reporter); library-emitted events (Phoenix/Ecto/Oban) are reused.
- [ ] Handlers are defensive (rescue errors; a raise would detach the handler and lose future events).
- [ ] Metrics use bounded-cardinality tags (`route`, `status`, `method`), never `user_id`/`request_id`/`email`; per-user/per-request detail is in logs/traces.
- [ ] Logs are structured (JSON in production) with correlation metadata (`request_id`, `trace_id`, `user_id`); levels are set deliberately; PII/secrets are redacted.
- [ ] Trace context is propagated across HTTP (library middleware injecting headers) and process boundaries (traced task patterns, not raw `spawn`); end-to-end traces span services.
- [ ] Instrumentation targets SLO-relevant events (request/query/job latency and errors, VM health, business events); alerts fire on user-visible degradation.
- [ ] The three signals (metrics, logs, traces) are used for their respective roles (trends, detail, slow-path diagnosis), not redundantly.
- [ ] The observability setup has been considered under handler crashes, cardinality, context loss, sensitive data, and SLO alignment, and remains effective.
