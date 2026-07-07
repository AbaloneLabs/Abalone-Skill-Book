---
name: async_and_concurrent_error_propagation.md
description: Use when the agent is handling errors in asynchronous or concurrent code — propagating errors from async tasks, futures, promises, goroutines, callbacks, or background threads; deciding between awaiting/collecting errors vs fire-and-forget; handling errors in fan-out/fan-in, parallel task groups, and pipelines; managing error context across thread/task boundaries; or diagnosing swallowed errors, lost rejections, and unhandled promise rejections. Covers error propagation across async boundaries, structured concurrency and error collection, the "lost error" failure modes (swallowed exceptions, unhandled rejections, detached task errors), error context preservation, cancellation as an error, and the discipline of ensuring no async error is silently lost.
---

# Async And Concurrent Error Propagation

In synchronous code, error propagation is straightforward: an exception propagates up the call stack until caught, and if uncaught, the program fails visibly. In asynchronous and concurrent code, this simplicity breaks down. An error in an async task, a goroutine, a callback, or a background thread does not propagate to the code that started it — the starter has often moved on, and the error has nowhere to go. The result is the characteristic failure mode of async error handling: the lost error. An error occurs in a detached task, is not caught or propagated, and is silently swallowed; the task fails, but no one knows, the failure is not reported, and the system continues in an inconsistent state (a background job that silently stops processing, a write that silently fails, a computation that silently produces no result). The complement is the unhandled rejection: an error in a promise or future that is never awaited or caught, surfacing as an unhandled rejection warning (or, worse, nothing at all) and indicating a task whose failure is unobserved. The discipline of async error propagation is ensuring that no error in any async or concurrent context is silently lost — that every task's failure is either propagated to its starter, collected by a structured concurrency primitive, or explicitly reported.

Agents tend to start async work without planning how its errors propagate (fire-and-forget), to await or join tasks without collecting their errors, and to lose error context (stack traces, causal chains) across async boundaries. The judgment problem is recognizing that async error propagation is not automatic and must be designed, that structured concurrency (task groups, scopes) is the primitive that makes propagation manageable, and that the lost error and unhandled rejection are the failure modes to design against. This skill covers the discipline of async and concurrent error propagation: propagation across boundaries, structured concurrency, lost-error prevention, context preservation, cancellation, and fan-out/fan-in error handling.

## Core Rules

### Never Fire-And-Forget: Propagate Or Report Every Async Error

An async task whose error is neither propagated to its starter nor explicitly reported is a lost error. Ensure every task's failure is observed.

- **Do not start an async task and ignore its result (fire-and-forget) unless the task's failure is truly irrelevant.** A fire-and-forget task that fails silently loses the error; if the task's work mattered (a write, a notification, a computation), the loss causes inconsistency. Propagate the task's error to its starter, or report it (to an error system — see error-reporting-and-crash-aggregation).
- **Await or join tasks and handle their errors.** When you start a task and await it, its error propagates to you (like a synchronous call); handle it (recover, log, propagate further). Awaiting is the mechanism that connects an async task's failure to its starter.
- **For tasks you cannot await (truly background), install an error handler that reports.** A background task (a periodic job, a detached worker) that runs independently of any starter needs an error handler at its top level that catches all errors and reports them (to logging, an error system). The handler is the safety net that prevents silent loss.
- **Fail loudly on unhandled rejections.** A promise rejection or future error that is never caught is an unhandled rejection — an unobserved failure. Configure the runtime to fail (or warn loudly) on unhandled rejections, so they are not silently lost. Treat unhandled rejections as bugs to fix, not warnings to ignore.

### Use Structured Concurrency To Manage Task Lifetimes And Errors

Structured concurrency (task groups, scopes, nurseries) ties child tasks to a parent scope, ensuring all children complete (or fail) before the parent proceeds, and propagating child errors to the parent. It is the primitive that makes async error propagation manageable.

- **Use structured concurrency (task groups, scopes) instead of detached tasks.** A task group starts child tasks, waits for all to complete, and propagates any child's error to the parent. Child errors are not lost; they propagate through the group. This is far safer than detached tasks whose errors have no path.
- **When a child fails, decide the group's behavior: fail-fast (cancel siblings, propagate) or collect-all (wait for all, aggregate errors).** Fail-fast cancels the remaining children on the first failure and propagates the error (suitable when the operation is all-or-nothing). Collect-all waits for all children and aggregates their errors (suitable when partial results are useful). Choose the behavior matching the operation's semantics.
- **Ensure cancellation propagates to children.** When a parent is cancelled (or fails and triggers fail-fast), its children must be cancelled too, so they do not continue running pointlessly. Structured concurrency propagates cancellation; detached tasks do not (they run to completion regardless).
- **Bound the task group's lifetime.** A task group's scope defines when its children must complete; the parent does not proceed until they do. This prevents task leaks (children running indefinitely after the parent has moved on) that plague unstructured concurrency.

### Preserve Error Context Across Async Boundaries

Errors that cross async boundaries (callback to caller, task to awaiter, thread to joiner) often lose context — the stack trace, the causal chain, the surrounding state. Preserve this context for diagnosis.

- **Preserve stack traces across async boundaries.** In many runtimes, an error thrown in an async task loses the starter's stack (the error's stack starts at the task, not the call that started it). Use runtimes or libraries that preserve async stack traces (Node's async hooks, languages with async stack trace support), or capture the starter's stack explicitly for inclusion in the error.
- **Wrap errors with causal context when propagating.** When an error propagates across a boundary, wrap it with the context of the boundary (what operation was being attempted, what inputs) to aid diagnosis. An error "connection refused" is less useful than "failed to fetch user profile for user X: connection refused to dependency Y."
- **Propagate trace and request context (trace ID, user ID) into async tasks.** An error in a background task should carry the trace context of the request that started it, so the error can be correlated to the request. See telemetry-pipeline-and-collection.
- **Beware error-cause chains that span boundaries.** An error caused by another error (a wrapping exception with a cause) should preserve the full chain across boundaries, so the root cause is visible. Losing the cause chain hides the original failure.

### Handle Errors In Fan-Out/Fan-In And Parallel Pipelines

When work is parallelized (fan-out to multiple tasks, fan-in to collect results), error handling must account for multiple tasks that may each fail.

- **Collect errors from all parallel tasks, not just the first.** A fan-out that returns the first error and ignores the rest loses information (other tasks may have failed too, with related or distinct errors). Use a task group that collects all errors (or at least the first with structured cancellation of the rest).
- **Decide whether partial failure is acceptable.** A fan-out that processes 100 items in parallel may tolerate 5 failures (process the 95, report the 5) or may require all-or-nothing (fail if any fails). Match the error handling to the operation's semantics.
- **Aggregate errors meaningfully.** Multiple errors from a fan-out should be aggregated into a composite error (an "aggregate exception," "multi-error") that reports all failures, not collapsed into a single generic error that loses the individual failures.
- **Handle the fan-in (result collection) for errors.** The fan-in step (collecting results from parallel tasks) must handle tasks that errored (no result) vs succeeded (a result), distinguishing them and propagating the errors appropriately.

### Treat Cancellation As A Distinct Error Category

Cancellation (a task aborted because it is no longer needed, or because its parent failed) is a distinct kind of error that should be handled differently from a failure.

- **Distinguish cancellation from failure.** A cancelled task did not fail; it was aborted (its result is no longer needed, often because a sibling failed or the user cancelled). Handling cancellation as a failure (logging it as an error, retrying it) is incorrect; it should be handled as a clean abort (release resources, exit quietly).
- **Propagate cancellation cooperatively.** A cancelled task should check for cancellation (cancellation tokens, abort signals) at cancellation points (await, loop iterations) and exit promptly. A task that ignores cancellation runs pointlessly, wasting resources.
- **Use cancellation to implement fail-fast in task groups.** When one child fails, the group cancels the others (they receive cancellation, not failure); the parent receives the original failure. This is the mechanism behind structured concurrency's fail-fast.
- **Do not conflate cancellation with timeout.** A timeout is a failure (the task took too long); cancellation is an abort (the task is no longer needed). They may both use the same mechanism (a signal) but have different semantics and handling.

## Common Traps

### Fire-And-Forget (Lost Errors)

Starting an async task and ignoring its result, so its failure is silently lost. Propagate the error (await) or report it (install a handler).

### Unhandled Promise Rejection

A promise or future whose error is never caught, surfacing as an unhandled rejection or nothing. Await and catch; fail loudly on unhandled rejections.

### Detached Tasks Without Error Handlers

Background tasks started detached, with no error handler, so failures are lost. Use structured concurrency, or install a top-level error handler that reports.

### Stack Trace Lost Across Async Boundary

An error whose stack trace starts at the task, not the starter, preventing diagnosis of the propagation path. Preserve async stack traces; capture the starter's stack.

### Error Cause Chain Lost

A wrapping error that loses the original cause when crossing a boundary, hiding the root failure. Preserve the cause chain.

### Fan-Out Losing Errors

A parallel fan-out that returns the first error and ignores the rest, or collapses multiple errors into one, losing information. Collect and aggregate all errors.

### Cancellation Treated As Failure

A cancelled task handled as a failure (logged as an error, retried), when it should be a clean abort. Distinguish cancellation from failure; handle cancellation cooperatively.

### Task Group Without Cancellation Propagation

A task group that fails fast but does not cancel siblings, so they run pointlessly. Ensure cancellation propagates to children on failure or parent cancellation.

## Self-Check

- [ ] No async task is fire-and-forget unless its failure is truly irrelevant — every task's error is either propagated to its starter (via await/join) or explicitly reported (via a top-level error handler), and unhandled rejections fail loudly rather than being silently lost.
- [ ] Structured concurrency (task groups, scopes, nurseries) is used instead of detached tasks, with a defined behavior on child failure (fail-fast with sibling cancellation, or collect-all with error aggregation), cancellation propagated to children, and task lifetimes bounded by the scope.
- [ ] Error context is preserved across async boundaries — stack traces (via runtime support or explicit capture), causal context (wrapping with the operation and inputs), trace/request context (trace ID propagated into tasks), and the full error-cause chain — so propagated errors are diagnosable.
- [ ] Fan-out/fan-in and parallel pipelines collect errors from all tasks (not just the first), decide whether partial failure is acceptable (process successes, report failures, or fail all), aggregate multiple errors meaningfully (composite error), and handle the fan-in's distinction between errored and successful tasks.
- [ ] Cancellation is treated as a distinct error category (a clean abort, not a failure), handled cooperatively (tasks check for cancellation at cancellation points and exit promptly), used to implement fail-fast in task groups, and not conflated with timeout.
- [ ] Background tasks (periodic jobs, detached workers) have a top-level error handler that catches all errors and reports them (to logging and the error system), so their failures are observed even though they have no starter to propagate to.
- [ ] The system has been tested for async error failure modes: a failing task in a group (error propagates or aggregates correctly), a cancelled task (exits cleanly, not treated as failure), a fire-and-forget task (error is reported, not lost), and an unhandled rejection (fails loudly, detected).
- [ ] Error propagation does not introduce deadlocks or hangs — a task awaiting another's result handles the case where that task errors (does not wait forever), and error-handling code (catch blocks, finally blocks) does not itself throw unhandled errors that re-lose the original.
