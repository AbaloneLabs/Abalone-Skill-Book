---
name: python_logging_and_observability.md
description: Use when the agent is adding logging or observability to Python code, configuring the logging module (loggers, handlers, formatters, levels), choosing between print and logging, structuring log records as JSON, deciding log levels (DEBUG/INFO/WARNING/ERROR/CRITICAL), adding context (extra, LoggerAdapter, structlog), handling exceptions in logs (exc_info, traceback), integrating tracing/metrics, or debugging why logs do not appear, appear twice, or leak secrets. Covers logger hierarchy, level selection, structured logging, exception logging, the print-vs-logging boundary, and the tradeoff between verbose diagnostics and noise/cost.
---

# Logging And Observability

Logging is how a running Python program tells you what it is doing and what went wrong. Python's `logging` module is flexible but configured by a hierarchy of loggers, handlers, and formatters that is easy to get wrong. The judgment problem is choosing what to log, at what level, with what context, and how to configure the system so the right records reach the right place without drowning the output in noise or leaking secrets.

Agents tend to use `print()` for diagnostics, log at INFO level indiscriminately, configure logging inside library code (breaking the caller's setup), or emit records that lack the context needed to diagnose a failure. The harm appears as logs that do not appear (because a library logged to an unconfigured logger), logs that appear twice (because a handler was added twice), verbose logs that nobody reads, and secrets written to files that ship to aggregators. The real work is logging structured records with context, configuring at the application boundary, choosing levels by audience, and treating logs as an observability surface with cost and privacy implications.

## Core Rules

### Use `logging`, Not `print`, For Anything That Runs In Production

`print()` writes to stdout unconditionally, with no level, no structure, no routing, and no way for the operator to suppress it. `logging` gives levels, handlers, formatters, and a hierarchy that operators configure.

- Use `logging` for any diagnostic output in code that runs outside a quick script.
- Reserve `print()` for CLI tools whose output is the product (e.g., a command that prints results to stdout for piping), and even there, log diagnostics to stderr via `logging`.
- The exception is a `__main__` block that reports a fatal error to the user; prefer `logging` configured to stderr, or a clean `print(..., file=sys.stderr)`.

### Configure Logging At The Application Boundary, Not In Libraries

A library should acquire a logger (`logger = logging.getLogger(__name__)`) and emit records, but it must not configure handlers, set levels on the root logger, or call `logging.basicConfig()`. Configuration is the application's job.

- Libraries: `logger = logging.getLogger(__name__)`; emit records; do not add handlers or call `basicConfig`.
- Applications: configure the root logger (or your package's logger) with handlers, formatters, and levels in `main()` or a setup function. Call `logging.basicConfig()` once, at the top.
- The trap is a library that calls `basicConfig()` and hijacks the application's logging setup, or a library that adds a handler to its logger that duplicates output when the application also configures it.

This separation lets the application decide where logs go (stdout, file, syslog, a remote aggregator) and at what verbosity, without the library fighting it.

### Choose Levels By Audience And Actionability

Log levels are a contract about who cares and what they should do. Logging everything at INFO makes the level meaningless.

- **DEBUG**: detailed diagnostic information for developers debugging a problem. Off in production.
- **INFO**: confirmation that things are working as expected, at coarse granularity (request started, job completed). Useful for tracing flow in production.
- **WARNING**: something unexpected happened, or a foreseeable problem is approaching, but the software is still working. Actionable but not urgent.
- **ERROR**: a serious problem; the software could not perform some function. Needs attention.
- **CRITICAL**: a severe error; the program itself may be unable to continue.

The common failure is logging normal flow at WARNING ("this is fine but I want to see it") or logging recoverable conditions at ERROR. Pick the level by what the reader should do: nothing (DEBUG), note it (INFO), investigate soon (WARNING), investigate now (ERROR), page someone (CRITICAL).

### Log Structured Records With Context, Not Just Messages

A log record like `"request failed"` is nearly useless for diagnosis. A structured record with fields (`method`, `path`, `status`, `duration_ms`, `user_id`, `error`) lets you search, filter, and correlate in an aggregator.

- Use `extra={...}` to attach structured fields, or use a structured logging library (`structlog`, `python-json-logger`) that emits JSON.
- Include identifiers that let you correlate: a request ID, a user ID, a trace ID. Without these, you cannot connect a log line to the request that produced it.
- Log the operation, the subject, the outcome, and the relevant values. `"deleted user 42"` is better than `"done"`.

Structured logs cost more to write but pay back massively in operability. Free-text logs are searchable only by substring; structured logs are queryable by field.

### Log Exceptions With `exc_info=True` And Context

When logging an exception, include the traceback. `logger.exception("...")` (equivalent to `logger.error("...", exc_info=True)` inside an `except` block) attaches the full traceback to the record.

- Always use `logger.exception()` or `exc_info=True` when logging inside an `except` block; the traceback is the most valuable diagnostic.
- Add context about what was being attempted: `"failed to process order %s"`, not just `"failed"`.
- Do not catch and log and then swallow silently unless that is the deliberate contract; usually the exception should propagate after logging, or be translated into a domain error.

### Do Not Log Sensitive Data

Logs are often shipped to aggregators, stored long-term, and read by many people. Secrets in logs are a security incident.

- Never log passwords, tokens, API keys, full credit card numbers, or personal data protected by regulation.
- Scrub or redact sensitive fields before logging. Many structured logging libraries support redaction filters.
- Be cautious with full request/response bodies, which may contain secrets in headers or payloads.

Treat every log line as something that may end up in a shared system. If you would not paste it into a public chat, do not log it.

### Understand Logger Hierarchy And Propagation

Loggers form a hierarchy by dotted name (`myapp`, `myapp.api`, `myapp.api.handlers`). A child logger propagates its records to its ancestors' handlers by default. This is why configuring the root logger captures everything, and why adding a handler to both a child and the root causes duplicate records.

- To prevent duplicates, either configure handlers only at the root (or top package) and let propagation deliver records, or set `propagate=False` on a child that has its own handler.
- Set levels per logger to control verbosity of subsystems: `logging.getLogger("myapp.db").setLevel(logging.WARNING)` to quiet a noisy subsystem.

### Integrate With Tracing And Metrics Where Applicable

For larger systems, logs are one pillar of observability alongside metrics (numeric aggregates) and distributed traces (request-scoped causality). Use OpenTelemetry or equivalent to correlate logs with traces via a trace ID, and emit metrics for rates and latencies rather than logging every event. Logging every request at INFO is often better replaced by a metric counter plus structured logs for errors and samples.

## Common Traps

### `print()` Instead Of `logging`

`print` has no level, routing, or structure. Use `logging` for anything in production code.

### Library Calling `basicConfig()`

A library that configures logging hijacks the application's setup. Libraries only acquire loggers and emit records; configuration is the application's job.

### Duplicate Logs From Handler Added Twice

Adding a handler to both a child logger and the root (with propagation on) logs each record twice. Configure handlers at one level or set `propagate=False`.

### Everything At INFO

Logging normal flow at INFO with no discrimination makes the level useless and floods logs. Reserve INFO for meaningful events; push detail to DEBUG.

### Logging Without Context

`"error"` or `"failed"` records cannot be diagnosed. Include the operation, subject, outcome, and identifiers.

### Forgetting `exc_info` On Exception Logs

Logging an exception without `exc_info=True` loses the traceback, the most valuable diagnostic. Use `logger.exception()`.

### Secrets In Logs

Passwords, tokens, and personal data written to logs become a security and compliance incident. Redact sensitive fields.

### Logs That Never Appear

A library logging to `getLogger(__name__)` with no configured handler emits nothing (or only WARNING+ to stderr in some setups). The application must configure logging for library records to surface.

## Self-Check

- [ ] Production code uses `logging`, not `print`; `print` is reserved for CLI tools whose stdout is the product.
- [ ] Libraries acquire loggers and emit records but do not configure handlers, set root levels, or call `basicConfig`; configuration lives at the application boundary.
- [ ] Log levels are chosen by audience and actionability: DEBUG for dev detail, INFO for meaningful flow events, WARNING for investigate-soon, ERROR for investigate-now, CRITICAL for page-someone.
- [ ] Records carry structured context (operation, subject, outcome, identifiers like request/trace/user ID), not just free-text messages.
- [ ] Exceptions are logged with `exc_info=True` / `logger.exception()` so the traceback is captured, plus context on what was attempted.
- [ ] No secrets (passwords, tokens, keys, regulated personal data) appear in log records; sensitive fields are redacted.
- [ ] Logger hierarchy and propagation are understood; handlers are configured at one level or `propagate=False` is set to avoid duplicate records.
- [ ] For larger systems, logs are correlated with traces (trace ID) and complemented by metrics for rates and latencies rather than logging every event.
