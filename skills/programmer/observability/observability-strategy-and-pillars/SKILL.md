---
name: observability_strategy_and_pillars.md
description: Use when the agent is designing an observability strategy, deciding what to instrument and with which pillar (logs, metrics, traces), balancing observability cost against coverage, defining production-readiness criteria, drawing the line between alerting and observability, or planning debug-driven instrumentation for a new system. Covers the three pillars and their distinct roles, correlation across pillars via shared identifiers, the cost of cardinality and retention in each, production-readiness review for observability, the boundary between alerting (action) and observability (investigation), debug-driven instrumentation (instrument for the questions you will ask), and the discipline of building observability into the system rather than bolting it on. Also use when observability cost is growing without bound, when an incident cannot be diagnosed from existing telemetry, when a service ships without adequate observability, or when deciding whether a signal should be an alert or just a metric.
---

# Observability Strategy And Pillars

Observability is the ability to ask new questions about a system from the outside, without shipping new code to answer them, and it is a property of the system's design rather than a tool you buy. The three pillars — logs, metrics, and traces — are the telemetry through which questions are answered, and each answers a different class of question at a different cost. The recurring strategic failure is treating the pillars as interchangeable or as a checklist ("we have logs and metrics, so we are observable"), when in fact the questions a system can answer are determined by how the pillars are combined, correlated, and scoped to the system's actual failure modes. A service with mountains of unstructured logs but no metrics cannot tell you it is degrading until a user complains; a service with dashboards but no traces cannot tell you which dependency caused the latency spike; a service with all three but no correlation between them forces the engineer to manually reconcile "the metric spiked at 14:03, which trace was that, what did its logs say." Observability is the integration, not the parts.

Agents tend to add telemetry reactively — a log here, a metric there, a trace when a library enables it — without a strategy for what questions the telemetry must answer or how the pillars combine. The defects live in the gaps and the cost: the question that no pillar can answer (the "unknown unknown" that observability exists to surface); the cardinality explosion in metrics or the volume explosion in logs that makes observability itself a major cost center; the alert that fires on every wiggle of a metric because alerting and observability were conflated; the service that passes code review but ships without the telemetry needed to operate it, because observability was "later." The judgment problem is treating observability as a first-class design concern — what will we need to know, how will we know it, what will it cost, and how do the pillars combine to answer it — built into the system, not bolted on after an incident.

This skill is about observability as a strategy across the pillars. It complements the structured-logging, metrics, and tracing skills (each pillar in depth); here the question is how the pillars fit together, what observability costs and how to bound it, and how to design a system that can be operated.

## Core Rules

### Assign Each Pillar Its Role, And Combine Them Deliberately

The three pillars answer different questions at different cost, and a complete observability strategy uses each for what it does best, combined via correlation.

- **Metrics answer "what is the aggregated health, over time."** Is the system up, how fast, how error-prone, how loaded — cheap to collect and query across long ranges, but aggregated (detail is lost). Use metrics for dashboards, alerting thresholds, and trend analysis.
- **Logs answer "what happened, in detail, per event."** The specific error message, the input that triggered it, the sequence of events — high detail per event, but high volume and cost. Use logs for diagnosis when you need the specifics a metric cannot carry.
- **Traces answer "where did this request go, and where did it spend time or fail."** Request-level causality across services — which dependency was slow, which hop failed. Use traces for diagnosing cross-service latency and failures.
- **Combine them via correlation.** The power is in moving between pillars: a metric spike linked to a representative trace (exemplar), a trace linked to its logs (trace id in every log line), a log linked back to the metric context. Without correlation, each pillar is an island and the engineer manually reconciles them under incident stress.

### Instrument For The Questions You Will Actually Ask (Debug-Driven Instrumentation)

Observability exists to answer questions, and the right instrumentation is driven by the questions the system will face, not by what is easy to emit. The failure is instrumenting everything and answering nothing.

- **Identify the questions before the incident.** What is the system's health? Where is the latency? Who is affected? What changed? What did this specific request do? Map each question to the pillar and instrumentation that answers it.
- **Instrument the failure modes, not just the happy path.** The telemetry that matters most is what tells you why the system is failing: error rates by type, latency tails, dependency health, saturation signals. Happy-path metrics (request count) without failure-path metrics (error breakdown, tail latency) leave you blind to the problems that page you.
- **Cover the four golden signals.** Latency, traffic, errors, saturation — for any service, these answer the core health questions and must be present. They are the minimum, not the maximum.
- **Re-evaluate after each incident.** Every incident that was hard to diagnose reveals a question the telemetry could not answer; add the instrumentation that would have answered it. Observability evolves with the system's actual failure modes.

### Correlate Across Pillars Via Shared Identifiers

The multiplier on observability is correlation: the ability to move from a metric, to a trace, to the logs of a specific request, without manual reconciliation. This requires shared identifiers threaded through all pillars.

- **A request id (and trace id) in every log line.** Every log for a request carries the id that links it to the trace and to other logs of the same request. Set this in logging context once per service; do not rely on developers to remember.
- **Trace id as an exemplar on metrics.** Where the metrics system supports it, attach a representative trace id to a metric sample, so a spike can be linked to the trace that caused it.
- **Span id in logs of that span.** A log within a span carries the span id, linking it to the exact operation in the trace. This lets you pivot from "this span was slow" to "what did the logs say at that span."
- **Correlation is set up once, system-wide.** It is infrastructure, not a per-feature decision. A service that ships without trace-id-in-logs is a service whose incidents are harder to diagnose than they should be.

### Bound The Cost Of Each Pillar Deliberately

Observability telemetry is itself a major system, with storage and ingestion costs that can rival the system being observed. Unbounded, it grows until it is the largest line item; the cost must be designed, not suffered.

- **Metrics cost is driven by cardinality.** Each unique label combination is a series; unbounded labels (user id, request id) explode series count. Bound label value spaces; monitor series count; route high-cardinality detail to logs, not metrics.
- **Logs cost is driven by volume and retention.** Every log line is stored, indexed, and retained; verbose logging at high throughput is expensive. Sample (especially debug/trace levels), set retention by value (operational logs longer, debug shorter), and avoid logging in hot loops.
- **Traces cost is driven by sampling rate and span count.** Every span is collected and stored; 100% sampling at high volume is prohibitive. Sample (head or tail-based), span meaningfully, and report asynchronously.
- **Treat observability cost as a first-class metric.** Track the cost and volume of each pillar; a sudden spike (a new high-cardinality label, a new verbose log path, a raised sample rate) is a budget event. Do not let observability become an unbounded expense.

### Define Production-Readiness Criteria For Observability

A service is not production-ready if it cannot be operated — if its health cannot be assessed, its failures cannot be diagnosed, and its saturation cannot be detected. Observability is a production-readiness gate, not an afterthought.

- **Require the golden signals before launch.** Latency, traffic, errors, saturation — a service without these cannot be monitored or alerted on, and is not ready to receive production traffic.
- **Require dashboards that answer "is it healthy."** An on-call engineer must be able to assess the service's health at a glance, without ad-hoc queries. A service without a health dashboard is a service whose incidents start with "is it even up?"
- **Require alerting on the SLOs, not on every metric.** Alerts should fire on user-visible degradation (SLO burn), not on internal wiggles. A service that pages on every metric spike suffers alert fatigue; a service with no alerts learns about problems from users.
- **Require correlation infrastructure.** Trace id in logs, exemplars on metrics — the service must integrate with the correlation infrastructure, or its incidents cannot be cross-pillar diagnosed.
- **Review observability in code review and design review.** A change that adds a new failure mode without instrumenting it is incomplete; a new service without the golden signals is not launchable. Make observability a review criterion, not a hope.

### Draw The Line Between Alerting And Observability

Alerting and observability are related but distinct, and conflating them produces alert fatigue (too many alerts) or blind spots (no alerts on what matters). The line is actionability.

- **Alerting is for action.** An alert should fire when a human must act — the system is failing in a way that requires intervention. Alerts should be tied to user-visible degradation (SLO burn, error spikes, saturation), high-signal, and actionable.
- **Observability is for investigation.** Metrics, logs, and traces exist to help understand the system, including when an alert fires. Not every interesting metric should be an alert; most should be dashboards for investigation.
- **Do not alert on every metric.** A threshold on every chart produces noise: alerts on transient spikes, on metrics that do not require action, on symptoms whose cause is already alerted elsewhere. Alert on the SLO and the few saturation/error signals that demand action; leave the rest as observability.
- **Do not rely on alerts for observability.** A service with alerts but no dashboards tells you when something is wrong but not what; pair every alert with the observability to diagnose it.

## Common Traps

### Treating The Pillars As A Checklist Rather Than A Combined Strategy

"We have logs and metrics, so we are observable," without correlation or coverage of the actual failure modes. Combine the pillars via shared identifiers; instrument for the questions you will ask; cover the failure paths.

### Instrumenting The Happy Path, Not The Failure Modes

Metrics for request count without error breakdown or tail latency, leaving the system blind to the problems that page. Instrument errors, latency tails, dependency health, and saturation — the failure-path telemetry that matters in incidents.

### No Correlation Across Pillars

Metrics, logs, and traces that cannot be linked, forcing manual reconciliation under incident stress. Thread trace id and request id through all pillars; set up correlation once, system-wide.

### Unbounded Observability Cost

High-cardinality metrics, verbose logs at high volume, or 100% trace sampling, growing cost without bound until observability is the largest line item. Bound cardinality, sample logs and traces, set retention by value, and track observability cost as a metric.

### A Service Shipping Without Production-Readiness Observability

A service that passes code review but lacks the golden signals, a health dashboard, or SLO-based alerting, so it cannot be operated. Make observability a launch gate and a review criterion.

### Alerting On Every Metric

Thresholds on every chart, producing alert fatigue from transient spikes and non-actionable symptoms. Alert on user-visible degradation (SLO burn) and the few actionable signals; leave the rest as dashboards.

### Relying On Alerts For Observability

Alerts that tell you something is wrong but no dashboards or traces to diagnose it, so every alert becomes a manual investigation. Pair every alert with the observability needed to diagnose its cause.

### Observability Bolted On After An Incident

Adding telemetry reactively after a hard-to-diagnose incident, rather than building it into the system's design. Instrument for the failure modes at design time; re-evaluate and add after each incident, but start from a strategy.

## Self-Check

- [ ] Each pillar is assigned its role (metrics for aggregated health over time, logs for per-event detail, traces for request-level causality across services) and the three are combined via correlation (shared trace/request ids, exemplars) rather than treated as a checklist.
- [ ] Instrumentation is driven by the questions the system will face (the four golden signals — latency, traffic, errors, saturation — are always present; failure modes are instrumented, not just the happy path), and is re-evaluated after each incident to close the gaps that made diagnosis hard.
- [ ] Correlation is set up once, system-wide: every log line carries the trace id and request id (via logging context), metrics carry trace exemplars where supported, and span ids link logs to their spans — so an engineer can pivot across pillars without manual reconciliation.
- [ ] The cost of each pillar is bounded and tracked: metrics cardinality is controlled (no unbounded labels, series count monitored), logs are sampled and retained by value, traces are sampled (head or tail-based) and spanned meaningfully, and observability cost is a first-class metric with alerting on growth.
- [ ] Production-readiness criteria require observability before launch: the golden signals, a health dashboard answering "is it healthy" at a glance, SLO-based alerting, and integration with the correlation infrastructure — and observability is a code-review and design-review criterion, not an afterthought.
- [ ] The line between alerting and observability is clear: alerts fire only when a human must act (user-visible degradation, SLO burn, actionable saturation/errors), most metrics are dashboards for investigation, and every alert is paired with the observability needed to diagnose it.
- [ ] The strategy is built into the system at design time (instrumenting the failure modes and questions identified up front) rather than bolted on after incidents, while still evolving as real incidents reveal new questions the telemetry must answer.
- [ ] The observability strategy is reviewed against the system's actual failure modes — the questions an incident will ask — rather than against a generic checklist, so the telemetry can answer the unknown unknowns that observability exists to surface.
