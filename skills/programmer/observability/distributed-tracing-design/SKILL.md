---
name: distributed_tracing_design.md
description: Use when the agent is instrumenting a service for distributed tracing, designing span and trace models, propagating trace context across service and protocol boundaries, choosing a sampling strategy, correlating traces with logs and metrics, reasoning about tracing overhead, or debugging why a trace is incomplete or broken across services. Covers the trace and span model, context propagation (W3C Trace Context, B3) and the damage of broken propagation, sampling strategies (head-based, tail-based) and their tradeoffs, correlating traces with logs and metrics via trace id, the overhead of tracing and how to bound it, tracing across asynchronous and message-driven boundaries, the difference between tracing and logging and when each fits, and the discipline of instrumenting the boundaries that matter. Also use when a trace is missing spans, when context is lost across a hop, when sampling hides the rare errors that matter, or when tracing overhead is too high.
---

# Distributed Tracing Design

Distributed tracing follows a single request as it crosses service boundaries, and its value is causality at scale: when a user-facing request is slow or fails, the trace shows which of the dozen services it touched was responsible, without guessing. The model is a tree of spans — each span a unit of work in one service, parented to the span that called it — and the trace is the complete tree. The recurring failure is broken causality: the trace that starts in the frontend, loses its context at the first service-to-service hop, and arrives at the backend as an orphan span with no connection to the request that caused it. A tracing system with broken propagation produces fragments, not traces, and fragments answer none of the questions tracing exists to answer. The second failure is sampling that hides the interesting events: a sampling rate tuned for cost discards the rare slow request or the rare error, leaving exactly the cases you need to investigate untraced.

Agents tend to treat tracing as "add the library and it works," because the library handles the easy case (a synchronous call within one process). The defects live at the boundaries the library does not automatically handle: a service-to-service call where the context is not propagated in the headers; an asynchronous handoff (a message queue, a scheduled task) where the context must be carried in the message and re-established by the consumer; a third-party client or database driver that does not participate in tracing, creating a gap in the span tree; a sampling decision made at ingestion that drops the rare tail events. The judgment problem is treating tracing as a property of the whole request path — every boundary must propagate context, every service must create and parent spans correctly, and sampling must preserve the events that matter — and instrumenting the boundaries deliberately rather than assuming the library covers them.

This skill is about designing distributed tracing that produces complete, useful traces. It complements the structured-logging skill (event detail) and the metrics skill (aggregated health); here the question is request-level causality across services and how to capture it end to end.

## Core Rules

### Propagate Context Across Every Boundary, Or The Trace Is Broken

A trace is only as useful as it is complete, and completeness depends on context propagation: the trace id and the current span id must travel with the request across every hop, so each downstream service can create a child span parented to the caller. A single hop that drops the context orphans everything downstream.

- **Use a standard propagation format.** W3C Trace Context (`traceparent`/`tracestate` headers) is the modern standard; B3 is a common predecessor. Use the standard rather than a custom format, so heterogeneous services and libraries interoperate.
- **Propagate at every inter-service boundary.** Synchronous HTTP/gRPC calls, asynchronous message queues, background job schedulers, database or cache clients that support tracing — each must carry the context in its headers or message metadata, and the receiver must extract and continue it. Audit each boundary; a single un-instrumented hop breaks the tree.
- **Handle asynchronous and message-driven propagation explicitly.** When work is handed off via a queue or a scheduled task, the context must be serialized into the message and re-established by the consumer, often across process and time boundaries. This is never automatic; instrument it deliberately.
- **Verify propagation end to end.** A trace that reaches the deepest service with the correct parent chain is the only proof propagation works. Test with a request that traverses the full path and inspect the resulting trace for gaps or orphans.

### Model Spans To Reflect Causality, Not Code Structure

A span represents a logical unit of work, and the span tree represents causality (which work caused which). The model should reflect how the request flowed, not how the code is organized.

- **Create a span for each meaningful unit of work.** A service entry, an outbound call, a significant internal operation. Too few spans hide where time went; too many spans drown the structure in noise and add overhead. Aim for the granularity that shows the causality without clutter.
- **Parent spans correctly.** A span's parent is the span that caused it — the caller's current span, extracted from the propagated context. Incorrect parenting (a span parented to the wrong span, or to nothing) distorts the tree and breaks the causality story.
- **Name spans meaningfully.** A span name should identify the operation (`GET /users/{id}`, `db.query users`) so the trace is readable. Avoid high-cardinality names (a span per URL with ids); use templated routes and put specifics in span attributes.
- **Annotate with attributes that aid diagnosis.** The trace should let you answer "what was this request, and what was special about it?" — annotate spans with relevant attributes (user id class, cache hit/miss, retry count, error type), bounded in cardinality.

### Choose Sampling To Preserve The Events That Matter

Tracing every request is too expensive at scale, so traces are sampled — but sampling that discards the interesting events defeats the purpose. The sampling strategy must preserve slow requests, errors, and rare paths.

- **Head-based sampling** decides at the trace's start whether to sample, propagating the decision to all services. Cheap and simple, but blind to the outcome — it samples the same fraction of fast/successful and slow/failed requests, so rare errors and tail latencies are under-represented or absent.
- **Tail-based sampling** decides after the trace completes, based on its characteristics (latency, error status, specific attributes). More expensive (the trace must be assembled before deciding), but it can sample 100% of errors and slow requests while sampling few fast ones — preserving exactly the events that matter for diagnosis.
- **Match the strategy to the goal.** For routine health and throughput analysis, head-based sampling is cost-effective. For diagnosing rare errors and tail latency, tail-based sampling (or 100% sampling at low volume) is necessary. Do not use head-based sampling for a problem that needs the tail.
- **Beware sampling hiding the problem.** If the issue is rare (a 0.1% error rate, a p99.9 latency), a 1% head sample may capture zero examples. Either raise the rate, use tail-based sampling for errors/slow traces, or fall back to logs/metrics that capture all events.

### Correlate Traces With Logs And Metrics Via Shared Identifiers

A trace is most powerful when correlated with the logs and metrics of the same request, so you can move between the aggregated view, the request-level causality, and the event detail. The correlation is via shared identifiers.

- **Put the trace id (and span id) in every log line for the request.** A log with the trace id links to the trace; a trace with the span id links to the logs of that span. This lets you pivot from "this trace was slow" to "what did the logs say at the slow span."
- **Put the trace id in metrics where feasible** (exemplars), so a metric spike can be linked to a representative trace. Not all metrics systems support this, but where they do, it bridges aggregated and request-level views.
- **Make the identifiers available throughout the request.** The trace id should be in the logging context (MDC, context logger) so every log includes it without manual threading. Set this up once per service; do not rely on developers remembering to add it.

### Bound Tracing Overhead, Especially On The Hot Path

Tracing is not free: creating spans, propagating context, and reporting traces all cost CPU, memory, and network. Uncontrolled, tracing overhead can rival the work being traced.

- **Sampling is the primary overhead control.** The fewer traces sampled, the lower the overhead; but see the sampling rule — do not sample away the events you need.
- **Avoid span explosion.** A span per loop iteration or per cheap operation creates enormous overhead and unreadable traces. Span meaningful units, not every function call.
- **Make reporting asynchronous and batched.** Span reporting should not block the request; buffer and send asynchronously, with backpressure so a slow collector does not stall the service.
- **Measure tracing overhead.** Tracing CPU and latency overhead should be a tracked metric; if it grows, investigate (too many spans, too high a sample rate, a reporting bottleneck).

### Trace Across The Hard Boundaries: Async, Queues, And External Calls

The boundaries that break traces are not the easy synchronous calls; they are the asynchronous handoffs, message queues, and external (database, third-party) calls where context propagation is not automatic.

- **Message queues require explicit context in the message.** The producer writes the trace context into the message metadata; the consumer extracts it and continues the trace, parenting its work to the producer's span. Without this, every consumed message is an orphan trace.
- **Scheduled and delayed work carries context forward in time.** A task scheduled now and executed later must carry the context that existed at scheduling, so the later execution is parented correctly. This spans process restarts and long delays; handle it deliberately.
- **Database and cache clients may or may not support tracing.** Where they do, enable it; where they do not, accept the gap (the call appears as time in the parent span without a child), and consider whether the gap hides a problem (a slow query invisible in the trace).
- **Third-party calls are a context boundary.** You propagate context to your own services; calls to third parties typically start a new trace from their perspective. Capture the outbound call as a span in your trace (so you see the time and outcome), even if the third party's internal trace is separate.

## Common Traps

### Broken Context Propagation At A Service Boundary

A service-to-service call that does not carry the trace context in its headers, orphaning the downstream service's spans and breaking the causality tree. Propagate context at every boundary using a standard format; verify end to end.

### Asynchronous Handoff Dropping The Trace

A message queue or scheduled task where the context is not carried in the message, so the consumer's work appears as an unrelated orphan trace. Serialize context into messages and re-establish it in consumers, explicitly.

### Head-Based Sampling Hiding Rare Errors And Tail Latency

A fixed head sampling rate that captures few or zero examples of the rare errors or p99 latency you need to diagnose. Use tail-based sampling for errors and slow traces, or raise the rate, for problems that need the tail.

### Span Explosion From Over-Instrumentation

A span per loop iteration or cheap operation, creating massive overhead and unreadable traces. Span meaningful units of work; reserve span creation for operations whose causality or timing matters.

### Orphan Spans From Incorrect Parenting

A span parented to the wrong span or to nothing, distorting the tree and breaking the causality story. Extract the parent from the propagated context and parent correctly; verify the tree shape.

### High-Cardinality Span Names

Span names that include ids or full URLs, producing unbounded name cardinality and unreadable aggregates. Use templated route names; put specifics in bounded span attributes.

### Logs Without Trace Ids, Breaking Correlation

Log lines that lack the trace id, so a slow trace cannot be linked to the logs of the request. Put the trace id (and span id) in every log line via logging context, set up once per service.

### Tracing Overhead Rivaling The Work Traced

Uncontrolled span creation, too-high sampling, or synchronous reporting making tracing a significant fraction of request cost. Sample appropriately, span meaningfully, report asynchronously, and measure overhead.

## Self-Check

- [ ] Context (trace id, span id) is propagated across every service boundary using a standard format (W3C Trace Context), including synchronous calls, and the propagation is verified end to end (a full-path request produces a complete trace with no gaps or orphans).
- [ ] Asynchronous and message-driven boundaries (queues, scheduled tasks, delayed work) carry context in the message and re-establish it in the consumer, parenting correctly across process and time boundaries — these hard boundaries are instrumented explicitly, not assumed.
- [ ] Spans model causality (each span parented to the span that caused it), are created at meaningful granularity (not per loop iteration, not missing significant operations), are named with templated routes (not high-cardinality ids), and are annotated with bounded attributes that aid diagnosis.
- [ ] The sampling strategy matches the goal: head-based for routine health and throughput, tail-based (or elevated/100% at low volume) for diagnosing rare errors and tail latency — and the strategy does not sample away the events the investigation needs.
- [ ] Traces correlate with logs (trace id and span id in every log line via logging context) and, where supported, with metrics (exemplars), so you can pivot between aggregated health, request-level causality, and event detail.
- [ ] Tracing overhead is bounded and measured: sampling controls volume, spans are meaningful (not exploded), reporting is asynchronous and batched with backpressure, and tracing CPU/latency overhead is a tracked metric that is investigated if it grows.
- [ ] Database, cache, and third-party calls are captured as spans in the trace (showing time and outcome) even where the called system's internal trace is separate, so the trace shows where time was spent across all dependencies.
- [ ] The instrumentation is reviewed for the specific failure modes (broken propagation, async orphans, sampling gaps, span explosion, missing correlation) rather than assumed to work because a library was added.
