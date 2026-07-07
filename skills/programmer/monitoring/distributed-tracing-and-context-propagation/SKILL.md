---
name: distributed_tracing_and_context_propagation.md
description: Use when the agent is implementing or evaluating distributed tracing (OpenTelemetry, Jaeger, Zipkin), propagating trace context (trace IDs, span IDs, baggage) across service boundaries and async/message boundaries, deciding sampling (head-based vs tail-based), instrumenting spans with the right attributes, diagnosing latency in a request that crosses many services, or ensuring traces connect end-to-end through message queues, batch jobs, and third-party calls. Also covers the failure modes of broken context propagation that fragments a trace, sampling that misses the slow or failing requests, spans that are too coarse or too fine, missing span attributes that make traces uninterpretable, and the recurring mistake of treating tracing as a backend concern when its value depends almost entirely on correct instrumentation and context propagation at every hop.
---

# Distributed Tracing And Context Propagation

In a distributed system, a single user request fans out across many services, databases, and queues, and when that request is slow or fails, the question "where did the time go, and where did it break?" cannot be answered by per-service metrics alone. Distributed tracing solves this by propagating a trace context (a trace ID and span IDs) through every hop, so the request's full path — and the latency and status of each segment — is reconstructable end to end. The judgment problem is that tracing's value depends almost entirely on two things that are easy to get wrong: context must propagate through every boundary (HTTP, message queue, async, batch, third-party), or the trace fragments and the end-to-end view is lost; and instrumentation must capture the right spans with the right attributes, or the trace is a chain of opaque boxes that cannot diagnose. A trace that breaks at a queue, that samples out the slow requests, or that lacks the attributes needed to localize a problem is no more useful than no trace at all.

Agents tend to under-invest here because tracing libraries make the easy case work — an HTTP call propagates context automatically — and the demo trace looks complete. The harm appears in the real, heterogeneous system. A request crosses a message queue and the trace breaks, because the queue producer did not inject context and the consumer did not extract it, so the consumer's work appears as an orphan with no link to the triggering request. A sampling policy samples out 99% of requests, including the rare slow one the team needed to see. A span wraps an entire service handler with no internal spans, so the trace shows "service B took 800ms" with no visibility into whether it was the database call, the downstream API, or the internal computation. A trace lacks the attributes (endpoint, user segment, error type) needed to search and localize, so finding the relevant trace among millions is impossible. The judgment problem is to propagate context through every boundary, sample intelligently (keeping the traces that matter), instrument spans at the right granularity with the right attributes, and treat tracing as an application-level responsibility, not a backend concern.

This skill covers context propagation across boundaries, sampling strategy, span instrumentation and attributes, trace-based diagnosis, and operational concerns. It complements the metrics-and-instrumentation skill (aggregate signals), the alerting-design skill (turning signals into alerts), and the connection-management skill (the network hops traces cross). Here the focus is end-to-end request-scoped observability through tracing.

## Core Rules

### Propagate Trace Context Through Every Boundary, Including Async And Message-Based Hops

A trace is only as complete as its weakest link: any boundary that drops context fragments the trace. Propagation must be explicit at every hop:

- **Propagate context over synchronous calls (HTTP, RPC).** Inject the trace context (trace ID, span ID, baggage) into outgoing request headers (W3C Trace Context, B3), and extract it on the receiving side to continue the trace. Most tracing libraries do this for common frameworks, but verify it is configured, not assumed.
- **Propagate context over message queues and async boundaries.** When publishing to a queue or triggering async work, inject the context into the message metadata; when consuming, extract it and continue the trace as a child span. This is the most common propagation failure: a queue breaks the trace because the producer and consumer did not handle context, and the consumer's work is orphaned.
- **Propagate context through batch and scheduled jobs.** A batch job triggered by a request (or that processes request-generated data) should link to the originating trace or start a trace that downstream work continues. Jobs that start fresh traces with no link lose the connection to their trigger.
- **Propagate context through thread, task, and coroutine boundaries.** Within a process, context must follow the logical request across async handoffs (thread pools, async/await, futures); a context lost at a thread boundary produces a broken span. Use the tracing library's context propagation primitives, not ad-hoc thread-local assumptions.
- **Handle third-party and external calls.** Calls to third-party APIs propagate context out (so the external call appears in the trace), even if the third party does not continue it; at minimum, the outgoing call's duration and status are captured as a span.

### Choose Sampling To Keep The Traces That Matter

Tracing everything is too expensive at scale; tracing 1% uniformly misses the rare events that tracing is most valuable for. Sampling strategy determines whether the traces you need are retained:

- **Head-based sampling decides at the start of the trace.** A fixed percentage of traces are sampled from the root; simple and cheap, but uniform — it samples fast and slow, successful and failing, at the same rate, and may miss rare slow or failing requests entirely.
- **Tail-based sampling decides at the end of the trace, based on its characteristics.** Keep all traces that are slow, errored, or high-value, and sample the rest. This retains the traces that matter for diagnosis (the slow request, the failure) while controlling cost. Tail-based sampling is more complex but far more valuable for diagnosis.
- **Do not sample out the interesting traces.** A 1% head-based sample that misses the 0.1% of failing requests defeats tracing's purpose for diagnosis. Use tail-based sampling (or a higher rate for errors/slow traces) to guarantee retention of the traces you will investigate.
- **Make sampling configurable and adaptive.** Traffic and diagnostic needs change; sampling rate and policy should be tunable without redeploying, so you can increase sampling during an incident or for a specific endpoint.

### Instrument Spans At The Right Granularity With Meaningful Attributes

A trace's diagnostic value depends on span granularity and the attributes captured. Too coarse hides the problem; too fine drowns it; missing attributes make it unsearchable:

- **Span the logical units of work, not every line of code.** A span per significant operation (a database query, a downstream call, a major computation step) localizes where time went. A span that wraps an entire handler with no internals shows "slow" but not why; a span per line adds overhead and noise.
- **Capture the key attributes on each span.** The endpoint, the operation, the status/error, the user or tenant where relevant, the resource identifier (which cache key, which query) — these make a span interpretable and searchable. A span with only a name and a duration is far less useful than one with attributes.
- **Mark errors and exceptions on spans.** An errored span should carry the error status and, where useful, the exception type and message, so error traces are findable and the failure is visible in the trace.
- **Use consistent span and attribute names.** Consistent naming (operation names, attribute keys) makes traces searchable and comparable across services; ad-hoc names produce a trace surface no one can query effectively.

### Make Traces Searchable And Usable For Diagnosis

A trace backend with millions of traces is useless without the ability to find the relevant ones. Searchability is a property of instrumentation:

- **Index and query by the attributes that localize.** Searching by service, endpoint, latency range, error status, and user/tenant is how investigations find the relevant trace; these attributes must be present and indexed. A trace that lacks searchable attributes is effectively unfindable.
- **Support service maps and dependency views.** Aggregate traces into a service dependency map (which services call which, at what latency and error rate) to reveal the system's topology and its hotspots. This view, derived from traces, is often the first diagnostic surface.
- **Link traces to logs and metrics.** A trace should link to the logs for its spans (via trace ID), and metrics should be derivable from or correlated with traces, so an investigation can move from aggregate signal (metric alert) to request-scoped detail (trace) to individual events (logs). This correlation is what makes observability three-dimensional.
- **Make traces accessible to the teams who need them.** Tracing locked to a platform team's tool, with no access for service teams, is underused; the teams who own services must be able to query and use traces for their own diagnosis.

### Handle Propagation Format And Version Compatibility

Trace context is carried in headers or metadata in a format that must be understood consistently across services, languages, and libraries:

- **Use a standard propagation format (W3C Trace Context).** Standard formats ensure context propagates across heterogeneous services (different languages, libraries, vendors); proprietary formats fragment when services use different libraries.
- **Handle multiple formats during migration.** Systems transitioning between formats (B3 to W3C, or vendor-specific to standard) must propagate both during the migration, or traces break at services still on the old format.
- **Verify end-to-end propagation, not assume it.** A misconfigured library, a custom HTTP client, or a queue integration can silently drop context. Periodically verify that a trace initiated at the edge completes end-to-end through every hop, including queues and async work.

## Common Traps

### Broken Context Propagation At A Queue Or Async Boundary

A message queue or async handoff that does not inject and extract trace context, orphaning the consumer's work from the triggering request and fragmenting the trace. Propagate context through every boundary, explicitly at queues and async hops.

### Uniform Sampling That Misses The Slow Or Failing Requests

A fixed head-based sample that samples out the rare slow or failing requests tracing is most valuable for. Use tail-based sampling to retain slow, errored, and high-value traces.

### Spans Too Coarse To Localize

A span wrapping an entire handler with no internal spans, showing "slow" but not whether the time was the database, a downstream call, or computation. Span the logical units of work (queries, calls, major steps).

### Missing Attributes That Make Traces Unsearchable

Spans with only a name and duration, lacking endpoint, operation, status, user, or resource identifiers, so the relevant trace among millions cannot be found. Capture the key attributes on each span.

### Treating Tracing As A Backend Concern

Assuming tracing "just works" because a backend is installed, when its value depends on correct context propagation and span instrumentation at every service hop. Tracing is an application-level responsibility at every boundary.

### Propagation Format Fragmentation

Services using different propagation formats (W3C vs B3 vs vendor-specific), so context is dropped at boundaries between them. Use a standard format and propagate both during migrations.

### Traces Disconnected From Logs And Metrics

A trace that cannot link to the logs for its spans or correlate with metrics, forcing investigations to jump between uncorrelated tools. Link traces to logs via trace ID and correlate with metrics for three-dimensional observability.

## Self-Check

- [ ] Trace context (trace ID, span ID, baggage) propagates through every boundary — synchronous calls (HTTP/RPC with standard headers), message queues and async work (injected into message metadata, extracted on consume), batch and scheduled jobs (linked to their trigger), thread/task/coroutine handoffs, and third-party calls (at minimum captured as outgoing spans).
- [ ] Sampling is chosen to retain the traces that matter: head-based for simplicity where acceptable, tail-based to guarantee retention of slow, errored, and high-value traces, with no policy that samples out the rare diagnostic requests, and sampling is configurable/adaptive without redeploy.
- [ ] Spans are instrumented at the right granularity (per logical unit of work — queries, downstream calls, major steps — not whole handlers, not per line), with key attributes (endpoint, operation, status/error, user/tenant, resource identifier) and consistent names.
- [ ] Traces are searchable and usable: indexed by localizing attributes, support a service dependency map, link to logs via trace ID and correlate with metrics, and are accessible to the service teams who need them for diagnosis.
- [ ] Propagation uses a standard format (W3C Trace Context), handles multiple formats during migrations, and end-to-end propagation is periodically verified (a trace from the edge completes through every hop, including queues).
- [ ] Errors and exceptions are marked on spans (status, exception type/message), so error traces are findable and the failure is visible in the trace.
- [ ] The highest-risk cases were verified — a trace that crossed a queue without breaking, a slow request retained under tail-based sampling, a span granular enough to localize a latency regression, and a trace findable by endpoint and error status — not only the clean single-service demo trace.
