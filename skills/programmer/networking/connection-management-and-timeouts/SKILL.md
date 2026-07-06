---
name: connection_management_and_timeouts.md
description: Use when the agent is configuring connection pools, keep-alive, or TCP connection reuse, designing a timeout strategy across layers (connect, read, write, idle, request), handling backpressure when a downstream is slow, or diagnosing connection exhaustion and hangs. Also covers the failure mode of missing or overlong timeouts that cause cascading hangs, connection pool exhaustion under load, head-of-line blocking, backpressure ignored until threads and memory pile up, and the gap between "it works under light load" and "it survives a slow downstream."
---

# Connection Management And Timeouts

Connections and timeouts are the substrate of distributed-system reliability, and they fail in characteristic ways under stress. The judgment problem is that under light load everything works — connections open instantly, requests return quickly, pools have capacity — and the failures appear only when a downstream slows or fails, at which point the absence of timeouts and pool limits turns a slow dependency into a system-wide hang. A call to a downstream with no timeout waits forever; if that downstream slows, every thread and connection tied up waiting piles up until the pool is exhausted and the upstream stops serving entirely — a slow downstream has cascaded into an upstream outage. The discipline is to put a timeout on every network call (connect, read, write) so that a failing dependency is bounded not infinite, to size connection pools so they do not exhaust under realistic concurrency and to fail fast when the pool is full rather than queue unboundedly, to apply backpressure when a downstream cannot keep up so that load is shed at the edge rather than piling up inside, and to reuse connections efficiently (keep-alive, pooling) without the staleness and resource-leak problems that naive reuse introduces.

Agents tend to omit timeouts (or set them to defaults that are effectively infinite) and to size pools optimistically. The harm appears as cascading failures (a slow dependency hangs every caller), as connection exhaustion (the pool fills with waiting requests and new requests fail), as thread and memory blowup (unbounded queues behind a slow downstream), and as mysterious hangs that are hard to diagnose because nothing errored — everything is just waiting. The judgment is to treat every network call as one that can hang, to choose timeouts by layer and with awareness of the downstream's expected behavior, to bound every pool and queue, to implement backpressure that sheds load rather than buffering it, and to verify the behavior under a slow or failing downstream. A system without timeouts and backpressure works until a dependency slows; one with them degrades gracefully.

## Core Rules

### Put A Timeout On Every Network Call

Every network call — connect, read, write — must have a timeout, because without one a slow or failed downstream hangs the caller indefinitely. The question is never "should there be a timeout" but "what is the right value for each layer."

- **Set a connect timeout.** Connecting to a dead or slow endpoint should fail fast (typically seconds), not hang until the OS gives up (which can be minutes).
- **Set a read/write timeout matched to the operation.** A fast RPC needs a tight read timeout; a large upload needs a longer write timeout. Match the timeout to the realistic operation duration, with bounded headroom.
- **Set an overall request timeout that bounds the end-to-end call.** Individual operation timeouts do not bound the total if there are retries or multiple steps; an overall timeout ensures the caller is not committed indefinitely.
- **Choose timeouts with the downstream's behavior in mind.** A timeout shorter than the downstream's normal slow-case response produces false failures; one much longer allows a failing downstream to hang the caller. Know the downstream's latency distribution.

### Size Connection Pools And Fail Fast When Full

A connection pool bounds the resources committed to a downstream, and its size determines how much concurrency the system can sustain to that downstream. An undersized pool bottlenecks throughput; an oversized pool overwhelms the downstream. Either way, the pool must fail fast when full rather than queue unboundedly.

- **Size the pool for realistic concurrency to the downstream, within what the downstream can handle.** Too few connections bottleneck; too many overwhelm the downstream and waste resources.
- **Fail fast (or bound the wait) when the pool is exhausted.** A pool that queues unboundedly when full piles up memory and threads behind a slow downstream; failing fast (or waiting a bounded time) contains the problem.
- **Validate pooled connections before use.** Connections idle in a pool can be closed by the network, the downstream, or a middlebox; validate (or test-on-borrow) so a stale connection fails rather than corrupting a request.
- **Handle pool leaks.** A connection not returned to the pool leaks it; over time the pool exhausts. Use try/finally or RAII to guarantee return.

### Apply Backpressure To Shed Load Rather Than Buffer It

When a downstream cannot keep up, backpressure is the mechanism that propagates that slowness upstream so load is shed at the edge rather than piling up inside. Ignoring backpressure (buffering unboundedly) hides the problem until memory or threads exhaust.

- **Propagate backpressure rather than buffering.** When the downstream is slow, signal slowness upstream (a full pool, a rejected request, flow control) so the edge sheds load; do not buffer unboundedly and hide the problem.
- **Bound every queue.** A queue between stages should have a bound; when full, apply backpressure (block the producer, shed load) rather than growing without limit.
- **Prefer shedding at the edge to failing in the middle.** Rejecting a request at the edge (load shedding, rate limiting) is cleaner than accepting it and failing deep in the system after committing resources.
- **Make backpressure observable.** When backpressure activates, it should be visible (a metric, a log) so operators know the system is shedding load and why.

### Reuse Connections Efficiently Without Introducing Staleness

Connection reuse (keep-alive, pooling) avoids the cost of establishing connections repeatedly, but reuse introduces staleness and lifecycle problems that single-use connections do not have.

- **Use keep-alive and pooling to avoid reconnection cost.** Establishing a connection (especially with TLS) is expensive; reusing connections across requests dramatically reduces overhead.
- **Handle stale connections.** A pooled connection may have been closed by the downstream or a middlebox (a firewall that kills idle connections) without the pool knowing; validate before use or handle the resulting error with a reconnect-and-retry.
- **Manage idle connection lifecycle.** Idle connections consume resources on both ends; evict connections idle beyond a threshold, and cap the idle pool size.
- **Avoid the connection churn of too-aggressive eviction.** Evicting too eagerly forfeits the reuse benefit; balance staleness risk against reuse benefit.

### Design Timeouts As A Layered Strategy

Timeouts exist at every layer (client request, service RPC, database query, load balancer), and they must be coherent across layers. A lower layer with a longer timeout than a higher layer is wasted (the higher layer gives up first); a lower layer with no timeout leaves the higher layer as the only bound, which may not exist.

- **Make lower-layer timeouts shorter than higher-layer ones.** The database query should time out before the RPC, which should time out before the client request, so each layer has a chance to recover within the layer above.
- **Make retries respect the overall budget.** Retries multiply the time spent; ensure retries do not exceed the overall request timeout, or a retry storm extends the hang.
- **Coordinate with the load balancer's timeouts.** A load balancer that times out before the service leaves the service working and the client failed; align the balancer's timeout with the service's expected behavior.

### Document the Basis and the Reasoning

Every conclusion should be traceable to its evidence, assumptions, and alternatives considered. Record not only the outcome but the reasoning path: what was checked, what was ruled out, what uncertainty remains, and what would change the conclusion. Documentation that captures the basis allows another practitioner to review, reproduce, or challenge the work, and it prevents confident conclusions from becoming unrepeatable assertions. A decision made without a recorded basis cannot be audited, improved, or safely handed off.

## Common Traps

### Missing Or Effectively-Infinite Timeouts

A network call with no timeout or a default so long it is effectively infinite, so a slow or failed downstream hangs the caller indefinitely and cascades. Put a timeout on every call (connect, read, write, overall).

### Timeout Mismatched To The Operation

A timeout shorter than the downstream's normal slow-case (false failures) or much longer than needed (allows a failing downstream to hang the caller). Match the timeout to the realistic operation duration with bounded headroom.

### Connection Pool Exhaustion Under Load

A pool too small for realistic concurrency that bottlenecks, or one that queues unboundedly when full so memory and threads pile up behind a slow downstream. Size for realistic concurrency and fail fast (bounded wait) when full.

### Stale Pooled Connections Corrupting Requests

A pooled connection closed by the downstream or a middlebox, used without validation, causing a request to fail or corrupt. Validate before use (test-on-borrow) or handle the error with reconnect-and-retry.

### Backpressure Ignored Until Exhaustion

Unbounded buffering behind a slow downstream that hides the problem until memory or threads exhaust, rather than propagating backpressure to shed load at the edge. Bound every queue and propagate backpressure.

### Lower-Layer Timeout Longer Than Higher-Layer

A database query timeout longer than the RPC timeout, so the query runs after the caller has given up (wasted work) and the layering is incoherent. Make lower-layer timeouts shorter than higher-layer ones.

### Retries Exceeding The Overall Budget

Retries that multiply time spent beyond the overall request timeout, extending a hang into a retry storm. Make retries respect the overall timeout budget.

## Self-Check

- [ ] Every network call (connect, read, write) has a timeout, plus an overall request timeout bounding the end-to-end call, with values matched to the downstream's realistic latency distribution (not so tight as to false-fail, not so loose as to allow indefinite hanging).
- [ ] Connection pools are sized for realistic concurrency within what the downstream can handle, fail fast (bounded wait) when full rather than queuing unboundedly, validate connections before use (or handle stale-connection errors with reconnect-and-retry), and guarantee return via try/finally or RAII to prevent leaks.
- [ ] Backpressure is propagated (not buffered): every queue is bounded, when full the producer is blocked or load is shed, shedding happens at the edge rather than deep in the system, and backpressure activation is observable.
- [ ] Connections are reused efficiently (keep-alive, pooling) to avoid reconnection cost, with stale connections handled (validation or reconnect-and-retry), idle connections evicted beyond a threshold, and idle pool size capped — balanced against reuse benefit.
- [ ] Timeouts form a coherent layered strategy: lower-layer timeouts are shorter than higher-layer ones (database query < RPC < client request), retries respect the overall timeout budget, and load-balancer timeouts are aligned with the service's expected behavior.
- [ ] The highest-risk cases were verified — a slow downstream that was bounded by a timeout rather than hanging the caller, a pool that failed fast rather than exhausting, backpressure that shed load at the edge rather than piling up inside, and retries that stayed within the overall budget — not only the clean light-load path.
- Is the reasoning documented clearly enough that another practitioner could review the basis and reproduce the conclusion?
- Are assumptions, uncertainties, and confidence levels stated explicitly rather than buried in a confident-sounding conclusion?
