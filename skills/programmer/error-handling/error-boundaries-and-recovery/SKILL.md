---
name: error_boundaries_and_recovery.md
description: Use when the agent is deciding where to place error boundaries, choosing recovery strategies after a failure, surfacing errors to users, logging and alerting on failures, implementing partial recovery, or isolating error state so a failure in one component does not propagate system-wide.
---

# Error Boundaries and Recovery

When a component fails, the failure does not stay local unless you deliberately contain it. An unhandled exception in a renderer can blank a whole page, a failing background job can poison a shared queue, and a corrupted cache entry can cascade into errors across every request that touches it. Error boundaries are the seams where you catch, contain, and decide what to do with a failure so that its blast radius is bounded. Recovery is the decision about what happens after the catch: retry, fall back, degrade, or surface the error. The catch is the easy part; the hard part is making sure the boundary is in the right place and that the recovery is correct rather than a silent swallow.

The judgment problem is placing boundaries at the granularity that matches the fault tolerance you want, choosing a recovery strategy that preserves correctness, deciding what the user and the operators should see, and ensuring that contained state does not leak across the boundary. The agent should not equate "wrap everything in try/catch" with resilience; a catch that swallows an error without recovery is worse than letting it propagate, because it hides the failure.

This skill applies whenever you are designing how failures propagate through a system, where they are caught, and what the system does afterward.

## Core Rules

### Place boundaries at the granularity of acceptable failure isolation

An error boundary should enclose exactly the unit whose failure you are willing to tolerate independently:

- **Per-feature/per-widget**: in a UI, a boundary around each panel means one failing panel does not blank the page. Too coarse (one boundary for the whole page) loses isolation; too fine (a boundary per line) adds noise.
- **Per-request/per-task**: in a server, a boundary around each request ensures one request's failure does not crash the process or poison shared state for others.
- **Per-job/per-message**: in a worker, a boundary around each message prevents one bad message from killing the consumer and blocking the queue.
- **Per-subsystem**: in a larger system, a boundary around a non-critical subsystem (recommendations, notifications) lets the core continue when it fails.

Decide the unit of independent failure first; the boundary goes around that unit.

### Choose a recovery strategy that matches the failure and the requirement

Catching an error is not recovery; recovery is what you do next. Match the strategy to the situation:

- **Retry**: for transient failures (network blip, lock contention), with bounded attempts and backoff. Not appropriate for permanent failures (validation, permission).
- **Fallback/degradation**: serve a cached, default, or reduced result when the primary path fails. Requires that the fallback is correct for the context (see graceful-degradation skill).
- **Partial recovery**: complete the parts of the operation that can succeed and report which parts failed, rather than failing the whole.
- **Propagate**: for failures the boundary cannot meaningfully handle, let them propagate to a boundary that can—but do so deliberately, with the error typed so the outer boundary can act.
- **Fail fast/terminate**: for failures that indicate corrupt state where continuing is unsafe (violated invariant, unrecoverable resource loss), fail fast rather than continuing in a broken state.

Weak choice: a catch-all that logs and returns a default for every error regardless of type. Strong choice: recovery keyed to the error type and the operation's requirements.

### Never swallow errors silently

A catch that ignores the error (`catch (e) {}` or `except: pass`) is one of the most damaging patterns in software. It hides bugs, masks outages, and makes failures impossible to diagnose. Every catch must do at least one of: recover meaningfully, re-throw, or log/alert at an appropriate level. If you genuinely cannot act on an error, log it with context so it is at least visible; silent swallowing guarantees the failure will recur undiagnosed.

### Surface errors to users honestly and actionably

When a failure reaches the user, the message should be:

- **Honest**: do not claim success when the operation failed, and do not show a generic "something went wrong" when a specific, actionable message is possible.
- **Actionable**: tell the user what they can do (retry, check input, contact support) rather than only what went wrong.
- **Free of internals**: no stack traces, error codes that map to internal paths, or other users' data.
- **Consistent with the recovery**: if the system has already fallen back, the user should see the degraded experience, not an error for the original failure.

### Log and alert with enough context to diagnose, without flooding

The boundary is often the right place to log, because it is where the failure is contextualized:

- Include the operation, the inputs (sanitized of secrets), the error type and message, and a correlation/trace ID.
- Log at a level matching severity: transient retried failures at debug/info; user-facing failures at warn/error; invariant violations at error/fatal.
- Alert on patterns, not individual logs: a single retried timeout is not an incident; a sustained spike in boundary-caught failures is. Rate-limit and dedupe alerts so operators are not flooded during an outage.

### Isolate state across the boundary

A boundary that catches an error must also ensure no corrupted state escapes:

- **Roll back partial mutations** before recovering, so the next request does not see a half-applied change.
- **Reset shared/cached state** that may have been corrupted by the failing operation.
- **Do not let failed operations poison shared resources** (queues, caches, connection pools). A bad message that crashes a handler should be quarantined, not retried forever.

### Distinguish boundary handling from letting errors propagate

Not every failure should be caught at the nearest boundary. Sometimes the right move is to let an error propagate to a boundary that has enough context to recover:

- A low-level I/O error should often propagate to the request boundary, which knows whether the operation is retriable, rather than being caught and swallowed locally.
- Type your errors so boundaries can decide: catch and recover for expected/recoverable errors, let unexpected errors propagate to a top-level safety net.

The decision is per-error-type, not a blanket policy.

## Common Traps

### Silent swallowing

`catch (e) {}` hides failures, masks bugs, and makes diagnosis impossible. Every catch must recover, re-throw, or log with context.

### Boundary too coarse or too fine

One boundary around everything provides no isolation; a boundary around every line adds noise and obscures real failures. Match the boundary to the unit of acceptable independent failure.

### Recovery that is wrong for the error type

Retrying a permanent validation error forever, or falling back to an unsafe default for a money-critical path, applies a strategy that does not fit the failure. Recovery must be keyed to error type.

### Showing users raw internals

Stack traces, internal error codes, or SQL fragments in user-facing messages leak information and look unprofessional. Surface honest, actionable, sanitized messages.

### Alerting on every caught error

A boundary that alerts on every single caught failure floods operators during an outage and trains them to ignore alerts. Alert on patterns and spikes, with rate limiting.

### Letting corrupted state escape the boundary

Catching an error but leaving a half-applied mutation or a poisoned cache entry means the next operation fails too. Roll back and reset state as part of recovery.

### Catching and rethrowing without adding value

`catch (e) { log(e); throw e; }` at every layer adds noise without changing behavior. Catch where you can recover or where you have unique context; let errors propagate otherwise.

### Retrying bad messages forever

A worker boundary that catches an error and requeues the message can poison the queue with a message that always fails. Quarantine poison messages after N attempts.

## Self-Check

- Are error boundaries placed at the granularity of acceptable independent failure (per-feature, per-request, per-message, per-subsystem), not too coarse or too fine?
- Does each catch apply a recovery strategy (retry, fallback, partial recovery, propagate, fail fast) matched to the error type and the operation's requirements?
- Are there any silent catches (`catch (e) {}`) that swallow errors without recovery, re-throw, or logging?
- Do user-facing error messages remain honest, actionable, and free of internal details?
- Do boundary logs include enough context (operation, sanitized inputs, error type, trace ID) to diagnose without flooding operators?
- Are alerts based on patterns and spikes with rate limiting, rather than every individual caught error?
- Does recovery roll back partial mutations and reset potentially corrupted shared state before continuing?
- Are poison messages or recurring failures quarantined rather than retried forever?
- Are errors typed so boundaries can distinguish recoverable from unrecoverable failures and act accordingly?
- Have you confirmed that failures caught at a boundary do not let corrupted state escape to affect subsequent operations?
