---
name: backpressure_and_load_shedding.md
description: Use when the agent is designing backpressure for overloaded systems, managing queue depths, choosing load-shedding strategies, prioritizing traffic under load, signaling overload to clients, or preventing retry storms and cascading failures under surge traffic.
---

# Backpressure and Load Shedding

Every system has a maximum sustainable throughput, and exceeding it does not produce graceful slowdown—it produces collapse. Once a server is saturated, queues grow, latency explodes, memory fills, and the system spends more effort managing backlog than doing useful work, until it stops serving anyone. Backpressure and load shedding are the mechanisms that keep a saturated system serving some traffic well rather than all traffic badly. They are the difference between a system that degrades linearly under overload and one that falls off a cliff.

The judgment problem is detecting overload early (before it cascades), choosing what to shed and what to keep, sizing queues and buffers so they absorb bursts without hiding problems, and signaling overload to clients so they cooperate rather than retry and amplify the load. The agent should not equate "add a queue" with resilience; an unbounded queue is a memory leak that delays failure rather than preventing it.

This skill applies whenever you are designing how a server, worker, pipeline, or client behaves under load that exceeds its capacity.

## Core Rules

### Detect overload by the right signal, and detect it early

The signal you monitor determines how early you react:

- **Queue length / concurrency**: the most direct signal. When the number of in-flight requests or queued items exceeds a threshold, the system is approaching saturation. React before latency explodes.
- **Latency**: rising p99 latency (especially tail latency growing faster than the median) indicates queueing. But latency is a lagging signal; by the time it spikes, saturation has begun.
- **Resource utilization**: CPU, memory, connection pool exhaustion, thread pool saturation. Useful but indirect; a system can be saturated at 60% CPU if bottlenecked on something else (locks, I/O).
- **Little's Law**: concurrency = throughput × latency. If latency rises while throughput stops rising, the system is saturated and additional load only inflates queues.

Monitor queue depth and concurrency as the primary early signal, with latency and utilization as confirmation.

### Bound queues explicitly; unbounded queues are a hazard

Every queue, buffer, channel, and connection pool must have a bound. An unbounded queue absorbs load until memory is exhausted, at which point the process dies—taking down everything, not just the overloaded path. Decide the bound based on how much burst you want to absorb versus how quickly you want to shed excess:

- **Small bound**: sheds quickly, protects the system, but offers little burst absorption. Good for latency-sensitive paths.
- **Large bound**: absorbs bursts, but lets latency grow as items wait in the queue. Good for throughput-oriented batch paths.

When a bounded queue is full, choose the rejection policy: drop newest, drop oldest, block the producer (backpressure), or reject with an error. The choice depends on whether freshness or completeness matters.

### Apply backpressure to producers, not just buffers

Backpressure means telling the producer to slow down, so the system does not accumulate work it cannot process:

- **Blocking backpressure**: the producer blocks (synchronously or asynchronously) when the consumer cannot keep up. This propagates pressure all the way back to the client, which must then slow down. Effective but can tie up producer resources.
- **Reactive/stream backpressure**: in streaming protocols (Reactive Streams, gRPC flow control), the consumer signals demand and the producer only sends what is requested. This is the cleanest form of backpressure for pipelines.
- **Client-side backpressure**: the server signals overload (via 429/503 with Retry-After, or flow control) and the client reduces its request rate.

The goal is that pressure propagates to the source rather than accumulating as unbounded queue.

### Shed load deliberately by priority

When you cannot serve everything, decide what to drop:

- **Priority-based shedding**: serve high-priority traffic (payments, critical writes) and shed low-priority traffic (analytics, batch, prefetch). Define priority classes and enforce them at the admission point.
- **Tenant fairness**: under overload, avoid letting one noisy tenant consume all capacity. Per-tenant quotas or fair queuing prevent starvation of well-behaved tenants.
- **Freshness-based shedding**: for time-sensitive data, drop the oldest queued items (they are likely stale) and keep newer ones, rather than processing a backlog of stale work.
- **Request type**: shed expensive operations (search, export) before cheap ones (reads), because one expensive request costs as much as many cheap ones.

Weak choice: shedding randomly or first-come-first-served until capacity is full, so a burst of low-priority traffic starves critical requests. Strong choice: explicit priority classes with admission control.

### Signal overload to clients so they cooperate

When shedding load, tell the client how to cooperate:

- Return `429 Too Many Requests` (per-client overload) or `503 Service Unavailable` (server-wide overload) with a `Retry-After` header.
- Use `503` with `Retry-After` for capacity-driven shedding and `429` for per-client rate limiting; clients treat them differently.
- Provide load-shedding feedback in protocols that support it (gRPC `UNAVAILABLE` with backoff, HTTP/2 flow control).
- Include jitter in any retry guidance to avoid synchronized retry storms when the overload clears.

### Prevent retry storms and cascading failures

Load shedding interacts with retries. If every shed request is retried immediately by every client, the retry traffic can exceed the original load and prevent recovery:

- Require clients to use exponential backoff with jitter on overload responses.
- Consider load shedding at the retry layer too: a client that is already backing off should not pile on retries.
- Watch for cascading failures: when one service sheds load, its callers may also overload as they retry or fail over. Design shedding with the call graph in mind, so a downstream shed does not cause upstream collapse.

### Distinguish sustained overload from transient bursts

The response should match the cause:

- **Transient burst**: absorb with a bounded queue and let it drain. No shedding needed if the burst is shorter than the queue depth.
- **Sustained overload**: a queue only delays failure; you must shed load or add capacity. Sustained overload requires shedding (or autoscaling), not larger queues.

A common error is sizing queues for sustained overload, which just delays the collapse and makes recovery harder.

## Common Traps

### Unbounded queues

A queue without a limit absorbs load until the process runs out of memory and dies, taking down the whole system. Unbounded queues hide overload until it is catastrophic.

### Detecting overload too late (via latency or CPU)

By the time latency spikes or CPU hits 100%, queues are already deep and recovery is hard. Queue depth and concurrency are earlier, more actionable signals.

### Shedding randomly or FCFS

First-come-first-served admission lets a burst of low-priority traffic block critical requests. Random shedding wastes capacity on low-value work. Shed by priority.

### Retry storms after shedding

If clients retry shed requests immediately and in sync, the retry wave can be larger than the original overload and prevent recovery. Require jittered backoff.

### One noisy tenant starving others

Without per-tenant admission control, a single client generating surge traffic can consume all capacity. Fair queuing or per-tenant quotas prevent this.

### Confusing 429 and 503

Using `429` (per-client rate limit) for server-wide capacity shedding misleads clients into thinking only they are affected, prompting retries that worsen overload. Use `503` for capacity shedding.

### Larger queues as a fix for sustained overload

Increasing queue depth for a sustained overload problem only delays the collapse and increases latency for every queued request. Sustained overload needs shedding or capacity, not buffering.

### Backpressure that deadlocks

Blocking backpressure, if the producer and consumer share a resource (e.g., the same thread pool), can deadlock: the producer blocks waiting for the consumer, which cannot run because the producer holds the resource. Ensure backpressure paths do not create circular waits.

## Self-Check

- Is overload detected from early signals (queue depth, concurrency) rather than lagging ones (latency, CPU) alone?
- Does every queue, buffer, channel, and connection pool have an explicit bound, with a defined rejection policy when full?
- Is backpressure propagated to producers (blocking, reactive flow control, or client signaling) rather than absorbed as unbounded queue?
- Is load shedding priority-based, protecting critical traffic and shedding low-priority or expensive requests first?
- Are noisy tenants prevented from starving others via per-tenant quotas or fair queuing?
- Does the system signal overload to clients with the correct status (503 for capacity, 429 for per-client) and a Retry-After with jitter?
- Is the retry behavior of clients designed to avoid synchronized retry storms (exponential backoff with jitter)?
- Have you considered cascading failures across the call graph, so shedding in one service does not collapse its callers?
- Is the queue depth sized for transient bursts, with a separate shedding/autoscaling strategy for sustained overload?
- Have you confirmed backpressure paths cannot deadlock (no circular waits on shared resources)?
