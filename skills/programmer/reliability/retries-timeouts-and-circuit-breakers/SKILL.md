---
name: retries_timeouts_and_circuit_breakers.md
description: Use when the agent is calling external systems, third-party APIs, databases, message brokers, downstream microservices, cloud services, or any dependency that can fail, hang, slow down, or return errors; designing retry logic, timeout budgets, backoff and jitter, circuit breakers, fallbacks, or bulkheads; diagnosing cascading failures, retry storms, thundering herd, or live-lock; or deciding whether to retry, give up, fail fast, or shed load under partial outage. Also covers exponential backoff, retry budgets, idempotency of retries, timeout layering, half-open probing, and the difference between transient and permanent failures.
---

# Retries, Timeouts, And Circuit Breakers

Every external call is a dependency that can fail in more ways than it can succeed. It can refuse the connection, accept it then hang, return a transient error, return a permanent error, slow down to a crawl, or succeed after a delay so long that the caller has already given up. Reliability work is deciding, for each such call, when to try again, when to stop trying, when to stop the system from trying at all, and what to do instead. The recurring failure mode is not "we did not retry"; it is "we retried in a way that made the outage worse."

Agents tend to under-invest here because the happy path needs no resilience logic — the dependency responds quickly and the code works. The harm appears only under failure, and under failure the wrong resilience strategy is actively destructive: retries multiply load on an already-struggling dependency, missing timeouts turn a slow dependency into a thread-pool exhaustion, and a circuit breaker that never opens lets one failing service drag down every service that calls it. The judgment problem is treating each external call as a failure surface with a deliberate policy — retry how many times, with what backoff, until what timeout, opening which circuit, with what fallback — rather than sprinkling `retry(3)` and moving on.

This skill is about resilience to external-system failure. It complements the idempotency-and-race-safety skill, which covers making individual operations safe to repeat. Here the question is the policy around repeating and giving up on calls to other systems.

## Core Rules

### Distinguish Transient From Permanent Failures Before Retrying

Retrying is only correct for failures that might resolve on their own. Retrying a permanent failure wastes resources and delays the error the caller will eventually receive anyway. Classify each error before deciding to retry:

- **Transient (retryable).** Network timeouts, connection resets, temporary DNS blips, `503`/`429`/`502` with retry hints, brief capacity exhaustion. The next attempt may succeed.
- **Permanent (do not retry).** `400`/`401`/`403`/`404`, validation errors, schema mismatches, business-rule rejections, malformed requests. Retrying returns the same error.
- **Ambiguous (the dangerous middle).** A timeout or connection drop where the request may or may not have been received and processed. Here retry safety depends on idempotency, not on the error class.

Map every error your dependency can return into one of these buckets, explicitly. A blanket "retry on any non-2xx" retries permanent errors and burns capacity; a blanket "retry only on connection error" misses retryable `5xx`. The classification must match the real semantics of the dependency, including its documented retryable status codes and headers (for example `Retry-After`).

For ambiguous failures, the governing question is not "should I retry" but "is it safe to retry" — which is an idempotency question. A non-idempotent call that timed out may have already taken effect; retrying it duplicates the effect.

### Set Timeouts At Every Layer, Sized To The Caller's Budget

A missing timeout is an availability bug: the calling thread, connection, or coroutine will wait indefinitely, and under failure those waiting resources accumulate until the caller exhausts its pool and fails too. Every external call needs a timeout, and timeouts must be set at each layer with awareness of the layer above.

- **Per-call timeout.** How long this single attempt may take.
- **Operation timeout / budget.** How long the whole operation (all retries included) may take, bounded by the caller's own deadline.
- **Connect versus read timeouts.** A connection that never establishes and a response that never arrives are different failures; set them separately, because a fast connect timeout prevents slow-start hangs while a generous read timeout tolerates legitimately slow responses.

Size timeouts to the caller's deadline, not to the dependency's typical latency. If your request handler must respond in 2 seconds, the total budget for a downstream call (including retries) must be well under 2 seconds — otherwise your timeout and the upstream timeout race, and you either return late or waste retries that the caller will never see the result of. Propagate deadlines (context deadlines, request deadlines) end to end so a downstream call knows it has only 300ms left and does not start work it cannot finish.

### Use Backoff With Jitter, And Bound The Total Budget

When retrying, space the attempts out and randomize them. Retrying immediately, or on a fixed interval, synchronizes retries across many callers and creates a thundering herd: the dependency recovers and is immediately hit by every retried request at once, knocking it back down.

- **Exponential backoff.** Increase the delay between attempts (e.g., 100ms, 200ms, 400ms...). This gives the dependency time to recover and caps how quickly retries accumulate.
- **Jitter.** Add randomness to each delay (full jitter, equal jitter, or decorrelated jitter) so retries spread out instead of aligning. Jitter is not optional; without it, backoff still synchronizes.
- **A bounded total budget.** Cap both the number of retries and the wall-clock time spent retrying. Unbounded retry-with-backoff can run for minutes, long after the caller has given up.

Combine these into a policy: "up to 3 retries, exponential backoff with full jitter starting at 100ms, total deadline 2 seconds." The policy must respect the caller's deadline: if only 200ms remain, do not start a retry that needs 500ms.

### Make Retries Safe Through Idempotency

A retry is a second execution of the same request. It is only safe if executing the request twice has the same effect as executing it once. For reads and pure queries this is automatic; for writes and side-effecting calls it is not.

Before adding retries to any call that mutates state:

- Confirm the operation is idempotent by design, or
- Send an idempotency key so the server deduplicates, or
- Make the retry conditional on a durable marker that proves the first attempt did not complete.

Retrying a non-idempotent call after an ambiguous timeout (did the charge go through?) is how payments get doubled and orders get placed twice. If a call is not safely retryable and the failure is ambiguous, the correct action is usually to query the resulting state and reconcile, not to blindly retry.

### Use A Circuit Breaker To Stop Retrying A Known-Bad Dependency

Retries and timeouts handle individual failed calls. A circuit breaker handles a dependency that is failing systematically: it stops the system from even attempting calls that are overwhelmingly likely to fail, giving the dependency time to recover and protecting the caller from the cost of futile attempts.

A circuit breaker has three states and deliberate transitions:

- **Closed.** Calls go through normally. Failures are counted.
- **Open.** When failures exceed a threshold within a window, the breaker opens. Calls fail fast (or invoke a fallback) without hitting the dependency at all.
- **Half-open.** After a cooldown, the breaker lets a limited number of probe calls through. If they succeed, the breaker closes; if they fail, it re-opens.

Tune the breaker to the dependency's recovery characteristics: the failure threshold (how many failures before opening), the cooldown (how long to stay open before probing), and the half-open probe volume. A breaker that opens too easily causes unnecessary failures; one that never opens provides no protection. The goal is to detect sustained failure quickly and stop pouring requests into a broken dependency, while resuming traffic as soon as it recovers.

### Provide A Fallback For When The Call Cannot Succeed

When a call fails after all retries, or the circuit is open, the caller must do something deliberate. Options, in rough order of preference:

- **Serve stale or cached data** if correctness tolerates staleness (e.g., a cached config or product catalog).
- **Degrade gracefully** by returning partial results without the failing dependency's contribution.
- **Fail the specific feature** while keeping the rest of the system working (bulkhead the failure).
- **Fail the request** with a clear error, only when no degraded path is acceptable.

The worst fallback is no fallback: the failing dependency propagates its failure synchronously to every caller, which propagates it to every caller, cascading the outage across the system. Decide the fallback before the outage, not during it.

### Prevent Cascading Failure Through Bulkheads And Load Shedding

Resilience is not only about recovering individual calls; it is about preventing one failing dependency from consuming the resources of the caller. A slow dependency ties up threads, connections, and memory in the caller; if the caller exhausts its pool waiting on that dependency, it becomes unavailable for everything else, including calls that do not touch the failing dependency.

Mitigations:

- **Bulkheads.** Use separate connection/thread pools per dependency or per tenant, so exhaustion from one dependency does not starve others.
- **Timeouts (again).** The single most effective bulkhead is a timeout: a call that cannot hang forever cannot hold a resource forever.
- **Load shedding.** Under overload, reject or defer work deliberately (rate limiting, queue depth limits, admission control) rather than letting backlog grow unbounded.

The aim is that a failure in dependency A degrades feature A, not the whole system. Without bulkheads, partial failures become total failures.

### Decide When Fast Failure Beats Eventual Success

Retries and circuit breakers optimize for eventual success, but eventual success is not always the right goal. Sometimes failing fast is better:

- **When the caller's deadline is short.** If the upstream request will time out in 500ms, a 2-second retry budget is wasted work whose result no one will read.
- **When the failure is permanent.** Retrying a `401` only delays the error.
- **When the dependency is saturated.** Retrying into a dependency that is overloaded increases its load and prolongs the outage; failing fast (or opening a circuit) lets it recover.
- **When the user is better served by a fast, clear error than a slow, hopeful retry.** A checkout that fails in 1 second with "payment service unavailable, try again" is better than one that hangs for 30 seconds.

Choose the policy by the value of eventual success versus the cost of waiting. Not every call deserves a retry; some deserve an immediate, honest failure.

## Common Traps

### Retrying Without A Timeout

Wrapping a call in retry logic but leaving the per-call timeout unset (or very large). Each attempt can hang indefinitely, so the "retry" budget is unbounded and the calling thread is held for the entire outage. A retry policy without per-call timeouts is an availability hazard.

### Retrying On All Non-2xx Or All Exceptions

Catching every exception or retrying every error code, including `400`, `401`, `403`, and validation errors. This retries permanent failures, wasting capacity and delaying the inevitable error. Retry only the failure classes that can plausibly resolve.

### Fixed-Interval Retries Without Jitter

Retrying every 1 second exactly. When many callers time out simultaneously (e.g., after a dependency restart), they all retry on the same 1-second boundary, producing a synchronized spike that re-overloads the dependency. Jitter is required, not decorative.

### Infinite Or Very Long Retry Budgets

Configuring "retry until it works" or a multi-minute backoff. The caller's own deadline is almost always shorter, so the retries run long after the caller has given up, doing work whose result is discarded and consuming capacity on both sides. Bound the total budget to the caller's deadline.

### Retrying Non-Idempotent Calls After Ambiguous Failures

A payment or order-creation call times out (did it succeed?), and the code retries it unconditionally, creating a duplicate charge or order. Ambiguous failures on non-idempotent calls require reconciliation (query the result) or an idempotency key, not a blind retry.

### Timeouts That Exceed The Caller's Deadline

Setting a 10-second downstream timeout inside a request handler that must respond in 2 seconds. The downstream call can run far past the point where its result is useful, and the upstream timeout fires first, wasting the work. Align downstream budgets with the propagated deadline.

### A Circuit Breaker That Never Opens (Or Never Closes) and no Fallback, So Failure Propagates Synchronously

Configuring the failure threshold so high that the breaker never trips during real outages — providing no protection — or the cooldown so long that it never recovers even after the dependency is healthy. Tune the breaker against observed failure and recovery patterns, and verify it actually opens and closes under simulated failure.

A dependency fails, and because no fallback or degradation path exists, the failure propagates directly to the caller, which propagates to its callers, cascading the outage. Define the degraded behavior before you need it.

### One Shared Pool For All Dependencies and treating Retry Count As The Goal

A single thread or connection pool shared across all downstream calls. One slow dependency exhausts the pool, and calls to healthy dependencies also fail for lack of resources. Use bulkheads (separate pools per dependency) so one failure does not starve the others.

Optimizing for "more retries" or "higher success rate" without measuring the cost — added latency, load on the dependency, work whose results are discarded. A high retry success rate can still indicate a harmful policy if it comes at the cost of prolonged outages and wasted capacity. Optimize for system availability and honest failure, not for retry success.

## Self-Check

- [ ] Each error the dependency can return is classified as transient (retryable), permanent (do not retry), or ambiguous (retry-safety depends on idempotency), and the retry policy respects that classification.
- [ ] Every external call has a per-call timeout (connect and read set separately) and a total operation budget that fits within the caller's propagated deadline.
- [ ] Retries use exponential backoff with jitter, a bounded retry count, and a bounded total budget; no fixed-interval, unbounded, or deadline-exceeding retry loops remain.
- [ ] Retries on mutating or side-effecting calls are safe via idempotency by design, an idempotency key, or reconciliation — not blind retries after ambiguous failures.
- [ ] A circuit breaker (or equivalent) protects dependencies that fail systematically, with failure threshold, cooldown, and half-open probe tuned to observed recovery characteristics, and verified to actually open and close.
- [ ] A fallback or degradation path exists for when retries are exhausted or the circuit is open, so failure does not propagate synchronously and cascade.
- [ ] Bulkheads (separate pools per dependency) and load shedding prevent one slow or failing dependency from exhausting resources needed by healthy calls.
- [ ] Fast failure is chosen over eventual success where the caller's deadline is short, the failure is permanent, or the dependency is saturated and retrying would worsen the outage.
- [ ] Timeout and retry budgets were validated under simulated failure (slow dependency, partial outage, retry storm), not only against the happy path.
- [ ] The resilience policy optimizes for system availability and honest failure, not for retry count or superficial success rate.
