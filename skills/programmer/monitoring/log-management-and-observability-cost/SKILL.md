---
name: log_management_and_observability_cost.md
description: Use when the agent is designing or reviewing a logging strategy at scale — choosing log levels, structured logging fields, sampling and verbosity for production, hot vs cold log tiers, retention windows, or correlating logs with traces and metrics. Also covers the failure modes of verbose INFO/DEBUG logging inflating ingestion and storage cost until the observability bill dwarfs the service, PII and secrets leaking into log streams, logs that cannot be correlated to a trace or a metric so investigations jump between disconnected tools, and the recurring mistake of logging everything at INFO in production because it feels safe while it silently creates cost, noise, and a retention liability.
---

# Log Management And Observability Cost

A log line is the most expensive string in your system. Unlike a metric (a few bytes per series) or a sampled trace (one per many requests), a log line is emitted per event, ingested per byte, indexed per field, stored for the retention window, and searched at query time — and every one of those steps is priced. The judgment problem is that logging feels free at write time (it is one `log.info` call) and ruinous at scale (a chatty service at a thousand requests per second, logging ten INFO lines each, produces almost a billion lines a day). The same instinct that makes developers log liberally in development — "more visibility is safer" — is the instinct that, in production, turns the observability bill into the largest line item after compute, drowns the meaningful lines in noise, and creates a retention surface full of data that must not be kept (PII, secrets) yet is hard to delete once indexed.

Agents tend to treat logging as a coverage problem rather than a cost-and-signal problem. The harm appears in three forms. First, cost: a service promoted from staging to production with DEBUG or verbose INFO still on ingests orders of magnitude more data than its quieter peers, and because log ingestion is usage-priced, nobody notices until the monthly bill spikes. Second, signal loss: when everything is logged at INFO, the lines that matter (the error, the boundary crossing, the business event) are buried among routine per-request chatter, so investigations grep through millions of lines and the useful signal is effectively absent. Third, liability: logs accumulate PII, tokens, and request bodies that regulations require you to protect or purge, but a log stream indexed by arbitrary fields is nearly impossible to selectively redact or delete after the fact. The judgment problem is to treat each log line as a deliberate, costed decision: log structured events at the right level, sample the routine, retain by tier, protect what is sensitive, and tie every line to its trace and metric context so logs are the third dimension of observability, not an isolated firehose.

This skill covers logging strategy at scale — structured logging, levels and sampling, tiering and retention, PII and security, cost control, and correlation with traces and metrics. It complements the metrics-and-instrumentation skill (aggregate signals), the distributed-tracing skill (request-scoped correlation), and the alerting-design skill (which warns explicitly against alerting on individual log lines). Here the focus is the logs themselves: what to emit, at what level, for how long, at what cost, and how to keep them useful rather than ruinous.

## Core Rules

### Log Structured Events, Not Prose For Humans

A log line should be a machine-parseable record with a stable schema, not a free-text sentence. Structured logging (a timestamp, a level, a message, and named fields) is what makes logs queryable, aggregatable, and correlatable; free-text logs are greppable but not analyzable at scale.

- **Emit fields, not interpolated strings.** `{"event":"checkout_failed","order_id":"...","reason":"payment_declined","duration_ms":120}` is queryable by event, reason, and duration; `"Checkout failed for order ... because payment declined"` is only greppable by literal substring. Prefer a structured sink (JSON, logfmt) over `printf`.
- **Use a stable, bounded set of event names and field keys.** Consistent event names (`request_completed`, `db_query`, `auth_failed`) and field keys make logs aggregatable across services; ad-hoc field names produce a schema no one can query. Treat the log schema as an interface, not a scratchpad.
- **Put the trace and span IDs on every line.** A log line without a trace ID cannot be correlated to its request; this is the single most valuable field. Inject the trace context (see the distributed-tracing skill) into every structured record so logs, traces, and metrics join on a shared key.
- **Reserve the human-readable message as a label, not the payload.** The `message` field is for scanning; the fields carry the data. Do not bury data inside a sentence that must be parsed back out.

### Choose Levels And Sampling Deliberately, Not Liberally

Log level is a cost-and-noise control, and the default in production should be sparse. The trap is treating INFO as "safe" and DEBUG as "harmless" — at production volume neither is.

- **Default production to WARN and above, plus a small set of meaningful INFO events.** Routine per-request activity (handled successfully, no anomaly) should not be an INFO line per request; that is what metrics are for. Reserve INFO for business and lifecycle events (startup, shutdown, a batch completed, a config reloaded) that are individually meaningful and low-volume.
- **Sample the routine, keep the exceptional.** For high-volume events you do want visibility into, sample (log one in N, or log on error) rather than emitting all. A 1% sample of successful requests preserves trend visibility at 1% of the cost; errors should be logged unsampled.
- **Never ship DEBUG to production ingestion by default.** DEBUG is for local and staging diagnosis. If it must be dynamically enabled in production (for an incident), gate it behind a runtime flag with a short time-to-disable, so it does not linger and accrue cost.
- **Let the level reflect actionability, not curiosity.** ERROR means something is wrong and likely needs attention; WARN means something unusual happened that may need investigation; INFO means a meaningful event occurred. If a level would never prompt action or analysis, it is noise at that level.

### Tier Storage Hot-To-Cold And Set Retention By Value

Not all logs are equally valuable over time, and pricing reflects that: hot, indexed, fast-query storage is expensive; cold, compressed, slow-query archive is cheap. Matching retention to value is how you keep cost proportional to usefulness.

- **Keep recent logs hot for investigation, age them to cold.** The logs you search during an incident are recent (hours to days); keep those in fast, indexed storage. Older logs are rarely queried and should move to cheaper compressed archive, accepting slower query for far lower cost.
- **Set retention by the value and the obligation, not by default infinity.** A 90-day hot window serves most incident investigation; a year of cold archive satisfies most audit needs; infinite retention serves almost nothing and accumulates cost and liability. Define the window per log class and enforce deletion at its end.
- **Separate security/audit logs from application logs.** Audit logs (access, auth, change) may have regulatory retention obligations and should be stored in a dedicated, protected sink with its own retention; mixing them with chatty application logs complicates both retention and access control.
- **Re-evaluate retention against actual query patterns.** If no one has queried logs older than 14 days in a year, a 365-day hot window is pure waste. Trim retention to observed need.

### Treat PII And Secrets As A First-Class Log Risk

Logs are a data exfiltration and compliance surface. A log stream that captures request bodies, headers, tokens, or user identifiers becomes a store of sensitive data that is broad-access, long-retained, and hard to redact — the opposite of how sensitive data should be handled.

- **Never log secrets, tokens, credentials, or full request bodies by default.** Authentication headers, API keys, passwords, and payment data must be stripped before logging. A logger that dumps the full request is a leak waiting to be discovered in an audit.
- **Classify and redact PII at the logging boundary.** Email, phone, government IDs, and other personal data should be hashed, truncated, or omitted in logs, not emitted raw. Redaction belongs in the logging pipeline (or the logger itself), not in a hope that consumers will handle it.
- **Restrict access to logs that contain any sensitive data.** If logs must carry identifiers for correlation, scope access to the teams and roles that need it; broad read access to a PII-bearing log stream is a privacy incident.
- **Assume logs will be retained longer than intended.** Retention drifts upward; design as if a log line, once written, may persist for years. If you would not want it stored for years, do not write it.

### Correlate Logs With Traces And Metrics, Do Not Isolate Them

Logs are most valuable when they are the detail layer of a three-signal system: a metric tells you something is wrong (aggregate), a trace tells you where in one request (request-scoped), and a log tells you what happened at a specific point (event detail). Logs that lack the shared keys to join these layers force investigations to jump between disconnected tools.

- **Put trace ID, span ID, and a correlation/request ID on every log line.** This is the join key to traces; without it, a log cannot be placed in its request's context.
- **Emit metrics from the same events you log, where sensible.** A checkout handler that logs `checkout_failed` should also increment a `checkouts_failed_total` counter, so the aggregate signal and the event detail derive from one source of truth.
- **Make dashboards deep-link to logs and traces.** An alert or dashboard panel should link to the relevant log query or trace view, so an investigation flows from signal to detail without manual context-switching.
- **Do not duplicate in logs what metrics already provide.** If a metric already captures request rate and error rate, logging one line per request to "also have it" is redundant cost; log the exceptional and the contextual, and let metrics carry the aggregate.

## Common Traps

### Logging At INFO Or DEBUG In Production By Default

A service shipped with verbose INFO or DEBUG enabled ingests orders of magnitude more data than its peers, and because ingestion is usage-priced the cost is invisible until the bill arrives. Default production to sparse WARN-plus and sampled meaningful events; gate DEBUG behind a runtime flag.

### Free-Text Logs That Cannot Be Queried Or Aggregated

Interpolated string logs that are only greppable by literal substring, so "how many checkouts failed by reason X in the last hour" cannot be answered without parsing prose. Emit structured fields with a stable schema so logs are queryable and aggregatable.

### Verbose Logging Inflating The Observability Bill

Treating log volume as a non-issue until ingestion and storage become the largest observability cost, often dwarfing the monitored service. Treat each line as costed: sample the routine, keep the exceptional, tier the storage, and trim retention to observed need.

### PII And Secrets Leaking Into The Log Stream

Logging request bodies, headers, tokens, or raw user identifiers, creating a broad-access, long-retained store of sensitive data that violates privacy obligations and is nearly impossible to redact after indexing. Strip secrets and redact PII at the logging boundary.

### Logs Disconnected From Traces And Metrics

Log lines lacking trace IDs or correlation IDs, so an investigation cannot move from a metric alert to the trace to the specific log event, forcing disconnected tool-hopping. Put the shared join keys on every record.

### Retention Set To Infinity By Default

Keeping all logs hot and forever because deletion feels risky, accumulating cost and a retention liability for data that is never queried and may be regulated. Set retention by value and obligation, tier hot-to-cold, and enforce deletion.

### Alerting On Individual Log Lines Instead Of Aggregate Signals

A single ERROR log triggers a page, when errors are expected within the budget and the page should fire on the rate or error-budget burn (see the alerting-design skill). Log the event; alert on the aggregate.

### Logging Everything "Just In Case" And Drowning The Signal

Emitting every internal step at INFO so the meaningful lines are buried among routine chatter, making the log surface effectively unsearchable during an incident. Log the exceptional and the contextual; let metrics carry the routine aggregate.

## Self-Check

- [ ] Logs are structured records with a stable schema (timestamp, level, event name, named fields, trace ID, span ID), not free-text prose that can only be grepped — and trace/correlation IDs are present on every line so logs join to traces and metrics.
- [ ] Production defaults to sparse logging (WARN and above, plus a small set of low-volume meaningful INFO events); routine per-request success is represented by metrics, not an INFO line per request, and DEBUG is gated behind a runtime flag rather than shipped by default.
- [ ] High-volume routine events are sampled (one in N, or on error) rather than emitted in full, preserving trend visibility at a fraction of the cost, while errors and exceptional events are logged unsampled.
- [ ] Storage is tiered hot-to-cold (recent logs in fast indexed storage, older logs in cheap compressed archive), and retention is set by value and obligation per log class with deletion enforced — not default infinity.
- [ ] No secrets, tokens, credentials, or full request bodies are logged by default; PII is classified, hashed, truncated, or omitted at the logging boundary; and access to any log stream carrying identifiers is scoped to the roles that need it.
- [ ] Audit/security logs are separated from application logs into a dedicated, protected sink with their own retention, rather than mixed into the chatty application stream.
- [ ] Logs correlate with traces and metrics: trace ID and span ID on every line, metrics emitted from the same events where sensible, and dashboards/alerts deep-link to the relevant log query and trace view.
- [ ] No alert fires on an individual log line; alerts fire on aggregate rate or error-budget burn, with the log providing event detail behind the signal.
- [ ] The highest-risk cases were verified — a verbose level caught before production ingestion, a structured field that answered an aggregation query free-text could not, a PII field redacted before it was written, and a retention window trimmed to observed query need — not only the clean low-volume development path.
