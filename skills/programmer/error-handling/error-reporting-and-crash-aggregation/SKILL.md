---
name: error_reporting_and_crash_aggregation.md
description: Use when the agent is setting up error reporting, crash aggregation, or exception tracking (Sentry, Bugsnag, Rollbar, Crashlytics); deciding what errors to report and at what severity; grouping and deduplicating errors; managing error volume and noise; handling PII and sensitive data in crash reports; setting up error-driven workflows and SLAs; or diagnosing from aggregated error data. Covers what to capture in a crash report, error grouping and fingerprinting, volume management (sampling, rate limiting, noise filtering), severity and routing, source maps and symbolication, PII scrubbing, and the operational workflow of triaging and resolving aggregated errors.
---

# Error Reporting And Crash Aggregation

A single error in isolation is a data point; thousands of errors, aggregated and grouped, are a signal that reveals the health of a system and the experience of its users. Error reporting and crash aggregation systems (Sentry, Bugsnag, Rollbar, Crashlytics) collect errors and crashes from production, group them by cause (so 10,000 occurrences of the same null-pointer are one issue, not 10,000), and present them prioritized by impact (how many users, how often, how severe). The value is operational: instead of discovering errors from user reports or log searching, the team sees aggregated errors in near-real-time, triages the highest-impact ones, and tracks resolution. The failure modes are characteristic: a system that reports everything drowns in noise (every handled exception, every expected 404, every transient retry), burying the real bugs; a system that reports too little misses critical issues; a system that includes PII or secrets in crash reports creates a privacy incident atop the bug; and a system without grouping turns a single widespread bug into thousands of identical issues that cannot be triaged.

Agents tend to either over-report (every exception, including handled and expected ones, creating noise) or under-report (only uncaught crashes, missing handled-but-significant errors), to neglect grouping and deduplication, and to include sensitive data in crash reports without scrubbing. The judgment problem is recognizing that error reporting is a signal-extraction problem (separating real bugs from expected errors and noise), that grouping and deduplication are what make aggregated data triageable, and that crash reports are data payloads that must respect privacy and security boundaries. This skill covers the discipline of error reporting and crash aggregation: what to capture, grouping and fingerprinting, volume management, severity and routing, PII scrubbing, and the triage workflow.

## Core Rules

### Report Errors That Represent Real Bugs, Not Expected Conditions

The signal-to-noise ratio of error reporting determines its usefulness. Report what indicates a bug; suppress what is expected.

- **Report unexpected errors (uncaught exceptions, unhandled promise rejections, crashes).** These indicate bugs the team did not anticipate; they are the primary signal. Ensure uncaught errors at every boundary (request handlers, event handlers, background jobs, async callbacks) are reported.
- **Report handled exceptions that indicate a real problem, not those that are part of normal flow.** A caught "not found" exception in a lookup is expected flow (return 404); a caught "connection refused" exception in a critical path is a real problem. Report the latter; do not report the former as an error (log it at info, or not at all).
- **Distinguish error severity: fatal (crash, data corruption), error (a real bug affecting users), warning (a degraded condition), info (expected but notable).** Report fatals and errors to the aggregation system; warnings and info to logs. Sending everything to the error system dilutes the signal.
- **Do not report expected HTTP errors (404, 400 validation) as crashes.** A 404 on a missing resource is expected application behavior, not a bug. Report 500s and unexpected status codes; let expected errors flow through normal logging.

### Capture Enough Context To Diagnose, Without Over-Capturing

A crash report without context (just the stack trace) often cannot be diagnosed; a report with too much context is bloated and risks privacy. Capture the right context.

- **Capture the stack trace (with filenames, line numbers, and function names), the error message, and the error type.** These identify where and what went wrong. Ensure stack traces are preserved through async boundaries (callbacks, promises, futures) where they are often lost.
- **Capture request and user context: the request URL, method, parameters (sanitized), user ID (not PII), session ID, and the triggering operation.** This context shows what the user was doing when the error occurred, enabling reproduction. See telemetry-pipeline-and-collection for context propagation.
- **Capture environmental context: the application version, the runtime version, the OS/device (for client crashes), and recent actions or breadcrumbs (what the user did before the error).** Breadcrumbs (recent navigation, API calls, user actions) are especially valuable for client-side crashes where the stack trace alone is insufficient.
- **Do not capture excessive data: full request bodies, entire database rows, large payloads.** These bloat reports and risk including sensitive data. Capture identifiers and summaries, not full payloads; the team can fetch full data if needed during diagnosis.

### Group And Deduplicate Errors Effectively

A single bug causing 10,000 errors must appear as one issue, not 10,000. Grouping and deduplication (fingerprinting) make aggregated data triageable.

- **Group errors by their root cause, typically the stack trace and error type.** Errors with the same stack trace and error type are one issue, regardless of when or for whom they occurred. The aggregation system's fingerprinting (based on the stack trace, with normalization of variable parts like line numbers or IDs) groups them.
- **Customize fingerprinting where the default is wrong.** Some errors need custom grouping: an error whose message varies ("user X not found") should group by the error type and stack, not the message; an error with different stacks but the same semantic cause may need custom grouping. Configure the fingerprint to match the grouping that makes sense for triage.
- **Handle stack trace normalization.** Stack traces with minor variations (different line numbers across versions, minified variable names in client code) should still group. Source maps (for minified JavaScript) and symbolication (for compiled binaries) restore the original names and lines, enabling correct grouping.
- **Track issue lifecycle: new, resolved, ignored, regression.** An issue that is resolved should not reappear as new; if the same error recurs after a fix, it is a regression (the fix did not work). The aggregation system tracks this lifecycle; use it to avoid re-triaging resolved issues and to catch regressions.

### Manage Volume To Preserve Signal

Error reporting systems have volume limits (ingestion limits, cost) and signal limits (too many issues overwhelm triage). Manage volume actively.

- **Sample or rate-limit high-volume errors.** An error occurring millions of times (a widespread client crash, a tight error loop) should be sampled (report 1% with a count) or rate-limited (report the first N per minute), not reported every occurrence. The aggregation system tracks the total count even when sampling.
- **Filter noise at the source.** Known-irrelevant errors (a third-party script failing, a bot triggering expected 404s, a deprecated browser) can be filtered before reporting, reducing volume and noise. Document the filters so they are understood and reviewed.
- **Use the aggregation system's deduplication.** The system's grouping means a recurring error is one issue with a count, not N reports. Rely on this; do not re-report an error the system is already tracking.
- **Monitor error volume and budget.** Track the volume of errors reported; alert on spikes (a sudden increase indicates a new widespread issue) and on budget exhaustion (approaching ingestion limits). Volume that grows without bound is a sign of missing filters or a degrading system.

### Scrub PII And Secrets From Crash Reports

Crash reports are data payloads sent to a third-party (or internal) system; they must not contain PII, secrets, or sensitive data.

- **Scrub or redact sensitive fields before reporting.** Request parameters, headers, and payloads may contain PII (emails, phone numbers), secrets (passwords, tokens, API keys), or regulated data (credit card numbers, health information). Configure scrubbing rules to redact these before the report leaves the application.
- **Do not send full request bodies or headers by default; send sanitized summaries.** A full request body is likely to contain sensitive data; a summary (the endpoint, sanitized parameter keys, response status) provides context without the risk. Expand to full body only for specific debugging, with scrubbing.
- **Be especially careful with client-side crash reports.** Client crashes (browser, mobile) may capture user input, screen content, or device data that includes PII. Configure client SDKs to scrub before sending; review what client SDKs capture by default.
- **Review the error reporting system's data handling and compliance.** The system (Sentry, Bugsnag, etc.) stores the reports; ensure its data handling (location, retention, access) meets the application's compliance requirements (GDPR, HIPAA, SOC 2). Do not send regulated data to a system not certified to handle it.

### Establish An Error-Driven Workflow

Error reporting is valuable only if there is a workflow to triage and resolve the issues it surfaces. Establish the workflow.

- **Triage new issues regularly (daily or more often).** New error issues should be reviewed, prioritized by impact (users affected, frequency, severity), and assigned for resolution or marked as known/ignored. An untriaged backlog loses value as issues age.
- **Set resolution targets by severity.** A fatal error affecting many users warrants immediate action; a minor error affecting few can wait. Define SLAs for resolution by severity, matching the urgency to the impact.
- **Use error data to drive prioritization.** The aggregation system shows which errors affect the most users or occur most frequently; prioritize these. An error affecting 10,000 users is higher priority than one affecting 10, regardless of the code's perceived complexity.
- **Close the loop: verify fixes resolve the issues.** After deploying a fix, verify the error issue's occurrence drops to zero (or the regression flag does not fire). A fix that does not reduce occurrences did not work; investigate further.

## Common Traps

### Over-Reporting (Every Exception Including Expected Ones)

Reporting handled exceptions that are part of normal flow (404s, validation errors), drowning the signal in noise. Report unexpected errors and real problems; let expected errors flow through logging.

### Under-Reporting (Only Uncaught Crashes)

Reporting only uncaught crashes, missing handled-but-significant errors (a caught database failure, a degraded operation). Report handled errors that indicate real problems.

### No Grouping Or Poor Fingerprinting

Errors not grouped by root cause, so one bug appears as thousands of issues that cannot be triaged. Configure fingerprinting; normalize stack traces.

### PII Or Secrets In Crash Reports

Request bodies, headers, or payloads containing PII or secrets, sent unscrubbed to the error system. Scrub sensitive fields before reporting.

### Stack Traces Lost Across Async Boundaries

Errors in callbacks, promises, or futures reported without the original stack trace, preventing diagnosis. Preserve stack traces through async boundaries.

### No Source Maps Or Symbolication

Minified client code or compiled binaries reported without source maps or symbols, showing meaningless names and lines. Deploy source maps; symbolicate reports.

### Volume Not Managed (Sampling, Filtering)

High-volume errors reported every occurrence, exhausting ingestion limits and overwhelming triage. Sample or rate-limit; filter known noise.

### No Triage Workflow (Backlog Accumulates)

Issues reported but not triaged or resolved, accumulating until the backlog is unmanageable. Triage regularly; set resolution SLAs; close the loop on fixes.

## Self-Check

- [ ] Error reporting captures unexpected errors (uncaught exceptions, crashes) and handled errors that indicate real problems, while expected conditions (404s, validation errors) flow through normal logging rather than being reported as crashes — preserving signal-to-noise ratio.
- [ ] Each crash report captures enough context to diagnose (stack trace with async-boundary preservation, error message and type, request/user/environment context, breadcrumbs for client crashes) without over-capturing (full payloads, database rows) that bloats reports and risks sensitive data.
- [ ] Errors are grouped and deduplicated by root cause (stack trace and error type, with custom fingerprinting where the default is wrong, stack trace normalization, source maps for minified code, and symbolication for compiled binaries), so one bug appears as one issue regardless of occurrence count.
- [ ] Volume is managed: high-volume errors are sampled or rate-limited (with total count tracked), known noise is filtered at the source (with filters documented), the aggregation system's deduplication is relied upon, and error volume and budget are monitored with alerting on spikes.
- [ ] PII and secrets are scrubbed from crash reports (sensitive fields redacted before reporting, full request bodies/headers avoided in favor of sanitized summaries, client SDKs configured to scrub, and the error system's data handling reviewed for compliance).
- [ ] Issue lifecycle is tracked (new, resolved, ignored, regression), so resolved issues do not reappear as new and regressions (recurrence after a fix) are caught.
- [ ] An error-driven workflow is established: new issues are triaged regularly and prioritized by impact (users affected, frequency, severity), resolution targets are set by severity, error data drives prioritization, and fixes are verified to reduce occurrences (closing the loop).
- [ ] Error reporting is integrated with the broader observability stack — errors are correlated with traces and metrics (via trace IDs and context — see telemetry-pipeline-and-collection), deploys and releases are tracked (so a spike can be correlated with a release), and the error system complements rather than duplicates logging and alerting.
