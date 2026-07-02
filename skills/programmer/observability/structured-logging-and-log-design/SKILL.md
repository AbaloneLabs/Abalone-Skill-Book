---
name: structured_logging_and_log_design.md
description: Use when the agent is adding, reviewing, or refactoring application logging — choosing log levels, deciding what to log and what to omit, structuring log fields, adding request or trace correlation ids, instrumenting errors and exceptions, shipping logs to an aggregation backend, or auditing existing logs for cost, privacy, and usefulness. Covers structured vs unstructured logging, log level discipline, PII and secret redaction, correlation and context propagation, log volume and cost, log/metric/trace boundaries, sampling, and making logs useful in production incidents rather than only in local debugging. Also use when a log line might leak personal data, credentials, tokens, or excessive user content, or when logs are too noisy, too sparse, or unsearchable during an outage.
---

# Structured Logging And Log Design

A log line is a permanent, searchable, shared record of something that happened in a running system. It is not a print statement you delete later. Once it ships to an aggregation backend it is stored, indexed, queried, retained per policy, possibly shown to auditors or attackers, and billed by the gigabyte. Every log line is simultaneously a debugging aid, an audit record, a monitoring input, a privacy surface, and a cost. The judgment problem is not "what do I want to see while developing" but "what does this line do for someone debugging a 3am incident, what does it risk leaking, and what does it cost to store and search at scale."

Agents tend to get logging wrong in both directions. They log too much (every branch, every variable, debug-level noise shipped to production that drowns the signal and inflates the bill), or too little (only fatal errors with no context, so an incident report says "request failed" with no way to find which request or why). They log the wrong shape (free-text strings no one can query, stack traces without request ids, errors at INFO level, secrets accidentally serialized into a struct dump). And they conflate logging with metrics and tracing, dumping high-cardinality per-request data into logs when it belongs in a trace. The harm is concrete: incidents that take hours instead of minutes because the logs cannot be correlated; compliance incidents because a token was logged; surprise cloud bills because every request logged a kilobyte of JSON; and alert fatigue because ERROR level is used for things that are not errors.

This skill focuses on what to log, at what level, in what shape, and what to keep out. It is distinct from the monitoring/alerting skill (which covers turning signals into actionable alerts) and the error-handling skill (which covers error type design and propagation). Here the question is: if I write this line, will it help production and hurt nothing.

## Core Rules

### Decide The Purpose Of Each Log Line Before Writing It

Every log line should serve one of a small number of purposes, and you should know which:

- **Operational debugging.** "What did this request do, and where did it diverge from the happy path?" Needs context (request id, user/tenant, key inputs, outcome) and should be at a level you can turn off in normal production.
- **Audit / compliance.** "Who did what, when, to which record?" Must be durable, tamper-evident as required, and often retained on a separate, longer policy. These are not debug logs.
- **Business / domain events.** "An order was placed, a payment succeeded, a user was created." Often the input to downstream metrics and analytics; structured and stable.
- **Error / fault reporting.** "Something failed, here is what and why." Needs the exception, the context, and enough to reproduce or classify.

A line that serves none of these — "logging to see what's going on" — is noise. If you cannot say which purpose a line serves and who reads it, it probably should not exist or should be DEBUG that is off by default.

### Structure Logs As Queryable Fields, Not Free Text

A log line should be a structured record (JSON or the backend's native format) with named fields, not a formatted string. The difference is operational: structured fields can be filtered, grouped, and aggregated ("show me all errors where `tenant=acme` and `endpoint=/charge` in the last hour"); free text can only be grep'd, and grep does not survive schema drift or localization.

Conventions that make structured logs useful:

- **Consistent field names across the service** (`request_id`, `user_id`, `tenant_id`, `route`, `status`, `latency_ms`). Ad-hoc names (`reqId` in one line, `requestID` in another) defeat correlation.
- **Stable, enumerated values for categorical fields** (status enums, error codes). Free-form strings cannot be aggregated.
- **Numbers with units in the name** (`latency_ms`, `size_bytes`) so no one misreads milliseconds as seconds.
- **A stable, low-cardinality message template** plus variable fields, rather than interpolating everything into the message. Many backends index the template and let you group by it.

The strong log is a row in a table; the weak log is a sentence in a novel.

### Use Log Levels To Mean Something, And Enforce It

Log levels only work if every author agrees what each level means. A common, enforceable contract:

- **ERROR** — something failed in a way that needs attention; the operation did not complete and a human may need to act. If no one would ever act on it, it is not ERROR.
- **WARN** — something unexpected or degraded happened, but the operation recovered or fell back. Worth noticing in aggregate; not necessarily worth a page.
- **INFO** — high-level lifecycle and business events (request handled, job started, user created). The default production level; each line should be meaningful at volume.
- **DEBUG / TRACE** — diagnostic detail for development and incident investigation. Off in normal production, turned on temporarily when diagnosing.

The failure modes are symmetric: ERROR used for recoverable warnings trains everyone to ignore errors; INFO used for per-loop-iteration detail drowns the real events; DEBUG left on in production ships noise at INFO cost. Pick the contract, document it, and lint against violations (e.g., no ERROR without an exception or a recovery action).

### Propagate Context So Lines Can Be Correlated

A single user request fans out across many log lines, often across services. Without correlation, those lines are unconnected dots. Propagate context on every line:

- **A request id / trace id generated at the edge** and carried through every downstream call (HTTP header, metadata, context). Every log line in that request's path includes it.
- **A span id** for sub-operations within the request, so you can see which step inside the request emitted a line.
- **User, tenant, session, and route** where relevant and safe, so you can filter by who and what.
- **A causation / correlation id for async work** so a background job triggered by a request can be traced back to it.

Correlation ids are the single highest-leverage logging decision. A service whose logs cannot be grouped by request is nearly useless during an incident. Generate the id at the entry point, propagate it through every call (including async and queue boundaries), and attach it to every line.

### Keep Secrets, PII, And Excessive Content Out Of Logs

Logs are a persistent, shared, often poorly-access-controlled surface. Anything written to a log should be assumed to eventually reach people and systems the author did not anticipate. Rules:

- **Never log secrets.** Passwords, API keys, tokens, session cookies, authorization headers, private keys, full credit card numbers. Redact or omit entirely; masking the middle of a token is not safe if the ends are enough to use it.
- **Minimize PII.** Email, phone, government ids, addresses, health or financial data. Log opaque ids instead of the raw value where possible, and follow the data-retention and data-minimization rules of your jurisdiction and product.
- **Beware struct dumps and debug formatting.** `Debug`/`toString` on a request, response, or config object can serialize fields the author never meant to log, including secrets and PII. Review what serialized output actually contains.
- **Do not log full user-supplied content by default.** Request bodies, form fields, and uploaded content can contain anything the user typed, including other people's data. Log a size and a type, not the content, unless there is a specific, reviewed reason.
- **Apply redaction at the logging boundary**, not by hoping each call site remembers. A structured logger that scrubs known-sensitive keys is more reliable than every author.

Treat every log line as a potential data-leak vector. If a field would be a security incident in a breach, it is a security incident in a log.

### Bound Volume And Cost; Logging Is Not Free

Log ingestion, indexing, and retention are billed and bounded. A service that logs 2KB of JSON per request at 10k requests/second ships ~20MB/s — gigabytes per day, terabytes per month, and most of it is never read. Rules:

- **Log events, not iterations.** INFO per request is usually right; INFO per loop iteration or per cache lookup is almost never right.
- **Use sampling for high-frequency diagnostic lines.** Log 1 in N, or log only when latency exceeds a threshold, rather than every occurrence.
- **Keep DEBUG/TRACE off in production by default** and enable temporarily per service or per tenant when diagnosing.
- **Distinguish hot-path logs from cold-path logs.** Move per-request detail to distributed traces (which are sampled by design) rather than forcing it through the log pipeline.
- **Review retention.** Audit logs may need years; debug logs may need days. One retention policy for everything is either too expensive or too short.

A logging change that multiplies volume is a cost and operability change, not just a code change. Estimate the volume at production scale before merging.

### Separate Logs From Metrics And Traces

The three pillars of observability overlap but are not interchangeable, and using the wrong one degrades all three:

- **Metrics** answer "how often, how much, how fast, at what rate?" They are pre-aggregated, low-cardinality, cheap to store, and the right home for counters, histograms, and rates. Do not compute "error rate" by counting log lines; emit a metric.
- **Logs** answer "what happened, in detail, for this specific event?" They are high-detail, higher-cost, and the right home for the context around a specific request or error.
- **Traces** answer "where did the time go, across which components, for this request?" They are sampled, span-structured, and the right home for per-request timing and causality across services.

Putting high-cardinality per-request data into logs (to derive metrics later) is expensive and lossy; putting detailed context into metrics is impossible; putting cross-service causality into logs duplicates what traces do better. Use each pillar for its strength, and link them (trace id in the log, exemplars on the metric) so they reinforce rather than duplicate.

### Make Logs Survive And Help During The Incident

The test of a logging design is not "does it look good in development" but "does it help during a production incident." Ask, for each significant path:

- **If this fails, will the logs tell me which request, which user, which tenant, and why?** If the error log has no request id and no context, it will not.
- **Can I filter to the failing request's lines across services?** Requires propagated correlation ids.
- **Does the error log include the exception type, message, and stack (or equivalent), plus the inputs that triggered it?** Not "request failed," but "request `abc` to `/charge` failed: `CardDeclined` for customer `123`."
- **Is the log at the right level so the signal is not buried?** A real error hidden in a flood of INFO will be missed.

Design logs as if you will be the one reading them at 3am with a customer waiting.

## Common Traps

### Logging The Whole Request Or Response Object

Dumping the full request/response/config struct "for debugging." It inflates volume, leaks PII and secrets (often via fields the author forgot), and is rarely useful in aggregate. Log the fields you specifically need; if you need the whole object, that is what DEBUG under a flag is for, with redaction.

### ERROR For Things No One Acts On

Marking recoverable or expected conditions as ERROR because "it might be important." This trains operators to ignore ERROR and breaks any alert keyed on error rate. If the code recovered and no human needs to act, it is WARN or INFO, not ERROR.

### Free-Text Logs That Cannot Be Queried

`log("User " + name + " did " + action + " and got " + result)`. No field can be filtered or aggregated; the same event logged differently by different authors never groups. Use structured fields with a stable template.

### Logging Tokens, Passwords, Or Authorization Headers

Directly or via struct dumps, headers, or "request context" objects. Tokens in logs are a credential leak to everyone with log access and a compliance incident. Redact known-sensitive keys at the logger; never trust every call site to remember.

### No Request Id, So Lines Cannot Be Correlated

Each log line is an island. During an incident there is no way to see all the lines for the failing request across services. Generate and propagate a request/trace id at the edge and attach it to every line.

### Per-Iteration Or Per-Call INFO In A Hot Path

INFO inside a loop, a per-row mapper, or a cache lookup, shipped to production. Volume explodes and the meaningful events are buried. Move it to DEBUG, sample it, or emit a metric instead.

### Deriving Metrics From Log Counts

Counting log lines to compute error rate, request rate, or latency. It is lossy, expensive, and breaks when log levels change or sampling is applied. Emit a real metric counter or histogram.

### Swallowing Errors Silently Or Logging-And-Continuing

Catching an exception, logging it, and continuing as if nothing happened — or worse, catching and logging at DEBUG so no one sees it. If the error matters, it must be visible at the right level and handled deliberately; logging is not a substitute for a decision.

### Inconsistent Field Names Across Services

`userId` here, `user_id` there, `uid` elsewhere. Cross-service correlation and dashboards break. Agree on a field-name convention and enforce it.

### Log Level Used As A Tag Instead Of A Severity

Using levels to categorize ("INFO = business, DEBUG = technical") rather than to express severity. This defeats level-based filtering: turning down the level to reduce noise hides real severity information. Levels express "how bad / how visible"; categories are fields.

## Self-Check

- [ ] Each log line has a stated purpose (operational debugging, audit/compliance, business event, or error/fault), and lines serving none of these were removed or moved to DEBUG-off-by-default.
- [ ] Logs are structured records with consistent field names across the service and across services (`request_id`, `user_id`, `tenant_id`, `route`, `status`, `latency_ms`), stable enumerated categorical values, and unit-suffixed numbers — not free-text strings.
- [ ] Log levels follow an enforced contract: ERROR only for failures needing action, WARN for recovered degradation, INFO for meaningful events at volume, DEBUG/TRACE off in normal production — and no ERROR exists that no one would act on.
- [ ] A request/trace id is generated at the edge and propagated through every call (including async and queue boundaries) and attached to every log line, so any request's path can be correlated across services.
- [ ] No secrets (passwords, tokens, keys, cookies, auth headers) or unnecessary PII are logged, directly or via struct/header dumps; sensitive keys are redacted at the logging boundary, and user-supplied content is logged as size/type rather than raw body.
- [ ] Log volume was considered at production scale: events are logged rather than iterations, high-frequency diagnostics are sampled or DEBUG-gated, and per-request detail that belongs in traces is not forced through the log pipeline.
- [ ] Metrics (rates, counts, histograms) are emitted as metrics, not derived from log counts; traces carry cross-service timing and causality; logs carry per-event detail — and the three are linked by shared ids.
- [ ] Retention matches purpose: audit logs retained per policy, debug logs short-lived; a single blanket retention is not applied to everything.
- [ ] For each significant failure path, the logs would actually help at 3am: the error line includes the request id, user/tenant, endpoint, exception type and message, and the triggering inputs — not just "request failed."
- [ ] Struct dumps, `Debug`/`toString` output, and "request context" objects were reviewed for fields that leak secrets or PII before being allowed into a log line.
