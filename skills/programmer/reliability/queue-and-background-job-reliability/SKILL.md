---
name: queue_and_background_job_reliability.md
description: Use when the agent is designing or operating a background job system, task queue, or asynchronous worker pipeline — choosing a queue technology, modeling jobs and their lifecycle, handling retries and dead-letter queues, ensuring at-least-once or exactly-once processing, managing poison messages, dealing with worker crashes and restarts, ordering and visibility timeouts, or diagnosing stuck/duplicated/dropped jobs. Covers delivery semantics, idempotency for consumers, retry and DLQ strategy, visibility timeouts and lease management, backpressure and queue depth, and the failure modes (poison messages, lost jobs, duplicate processing) that background systems exhibit.
---

# Queue And Background Job Reliability

Background job systems decouple work from request handling, and their reliability rests on delivery semantics that are subtler than they appear. The central tension is between at-least-once delivery (a job is delivered at least once, but possibly more than once if a worker crashes or a network blips) and exactly-once processing (a job's effect happens exactly once, which requires the consumer to be idempotent because the delivery layer cannot guarantee exactly-once). Most queue systems provide at-least-once delivery, and the burden of exactly-once processing falls on the consumer: if a consumer is not idempotent, a redelivery produces duplicate effects (double charges, duplicate notifications, inconsistent state). The second central issue is the poison message — a job that always fails (malformed input, a bug, a dependency that will never succeed) and, without a dead-letter mechanism, is retried forever, consuming worker capacity and never completing. Background systems that do not handle these issues exhibit stuck queues, duplicated work, lost jobs, and workers that spin on poison messages.

Agents tend to assume "the queue handles reliability" without defining delivery semantics, to write non-idempotent consumers that break under redelivery, and to configure retries without a dead-letter escape. The judgment problem is recognizing that queue reliability is a contract between the delivery layer and the consumer, that the consumer's idempotency is what makes at-least-once delivery safe, that visibility timeouts and leases govern worker crash recovery, and that retries without bounds are a reliability hazard. This skill covers the discipline of queue and background job reliability: delivery semantics, consumer idempotency, retry and DLQ strategy, visibility timeouts and leases, backpressure, and the failure modes to design against.

## Core Rules

### Define And Match Delivery Semantics

The delivery guarantee of the queue (at-most-once, at-least-once, exactly-once) determines what the consumer must do to achieve correct processing. Mismatched assumptions cause lost or duplicated work.

- **Understand the queue's delivery guarantee.** Most systems (SQS, RabbitMQ, Kafka with commits, Redis queues) provide at-least-once delivery: a job is delivered at least once, but may be redelivered if the worker crashes before acknowledging, the connection drops, or a visibility timeout expires. Some (Kafka without commits, certain log systems) provide at-most-once. True exactly-once delivery is rare and usually approximated.
- **Design the consumer for the delivery guarantee.** If delivery is at-least-once (the common case), the consumer must be idempotent — processing the same job twice produces the same effect as once. If delivery is at-most-once, the consumer must tolerate lost jobs (or the system must accept the loss). Do not assume exactly-once unless the system genuinely guarantees it and you understand how.
- **Prefer at-least-once with idempotent consumers.** At-least-once delivery with idempotent processing achieves effectively-once processing (the effect happens once, even if the job is delivered multiple times). This is the most robust pattern; it does not depend on the delivery layer's exactly-once guarantees.
- **Make idempotency explicit and verifiable.** Idempotency is achieved by a unique job identifier checked against processed-job state (deduplication), or by designing the operation to be naturally idempotent (setting a state to a value, rather than incrementing it). Verify the idempotency holds under redelivery; a consumer assumed idempotent but not actually so will duplicate under redelivery.

### Manage Retries With A Dead-Letter Escape

Retries handle transient failures, but a job that always fails (a poison message) must eventually escape the retry loop via a dead-letter queue, or it consumes worker capacity forever.

- **Retry transient failures with backoff.** A job that fails due to a transient issue (a dependency briefly unavailable, a timeout) may succeed on retry. Use exponential backoff with jitter (see retries-timeouts-and-circuit-breakers) to avoid overwhelming the dependency and to spread retries over time.
- **Set a maximum retry count, after which the job is dead-lettered.** A job that fails after N retries (typically 3-10) is moved to a dead-letter queue (DLQ) for investigation, rather than retried forever. Without a DLQ, a poison message is retried indefinitely, consuming worker capacity and never completing.
- **Monitor and alert on the DLQ.** A DLQ with jobs in it indicates jobs that could not be processed — a bug, a malformed input, a dependency issue. Alert on DLQ depth; investigate and fix the cause; reprocess or discard the dead-lettered jobs deliberately.
- **Distinguish retriable from non-retriable failures.** A transient failure (timeout, dependency down) is retriable; a permanent failure (malformed input, validation error, a bug) is not — retrying it will never succeed. Do not retry permanent failures; dead-letter them immediately to avoid wasting retries.
- **Consider a delay queue for scheduled retries.** Rather than retrying immediately, a delay queue holds the job and re-delivers it after the backoff delay, freeing the worker in the meantime.

### Use Visibility Timeouts And Leases To Handle Worker Crashes

When a worker takes a job and crashes before completing it, the job must become visible again for another worker to pick up. Visibility timeouts and leases govern this recovery.

- **Set a visibility timeout longer than the expected job duration.** A visibility timeout hides a job from other workers while one processes it; if that worker does not acknowledge completion before the timeout, the job becomes visible again (redelivered). Set the timeout longer than the job's expected duration, so a normally-processing job is not redelivered prematurely.
- **Handle jobs that exceed the timeout (extend the lease or accept redelivery).** A long-running job may exceed the visibility timeout and be redelivered to another worker while the first is still processing — producing duplicate processing. Extend the lease (heartbeat) for long jobs, or ensure the consumer is idempotent so the duplicate is harmless.
- **Acknowledge completion only after the job's effects are durable.** A worker that acknowledges a job before its effects (database write, external call) are durable risks losing the job if it crashes between the acknowledge and the effect. Acknowledge after the effects are committed.
- **Do not acknowledge before processing (at-most-once risk).** A worker that acknowledges on receipt, then processes, risks losing the job if it crashes after acknowledging — the job is not redelivered because it was acknowledged. This converts the system to at-most-once delivery.

### Handle Ordering, Backpressure, And Queue Depth

Queues introduce ordering, backpressure, and depth considerations that affect correctness and reliability.

- **Decide whether ordering matters and use a queue that preserves it.** If jobs must be processed in order (state mutations for a single entity), use a queue with ordering (FIFO, partitioned by entity key) and ensure a single worker per partition. If ordering does not matter, a standard queue with parallel workers is faster.
- **Use queue depth as backpressure.** A growing queue depth signals the producers are outpacing the consumers. Use the depth to trigger scaling (add workers) or backpressure (slow producers, reject work) before the queue grows unboundedly and latency or memory becomes a problem.
- **Set a maximum queue depth to bound resource use.** An unbounded queue can consume all available memory or storage under sustained overload. Set a max depth and define behavior on overflow (reject new jobs, shed load, alert) rather than allowing the queue to grow until it fails.
- **Monitor consumer lag and processing latency.** The time a job spends waiting in the queue (lag) plus the processing time is the end-to-end latency. Monitor both; growing lag means consumers cannot keep up and the system is falling behind.

### Ensure Job Observability And Traceability

Background jobs run outside the request path, making their failures invisible without explicit observability.

- **Log job lifecycle events with identifiers.** Log job receipt, start, completion, failure, and dead-lettering, with the job ID, so the job's journey is traceable. A job that disappears (received but never completed or failed) is detectable from the logs.
- **Instrument job metrics.** Track jobs received, completed, failed, retried, dead-lettered, and the queue depth, by job type. These metrics reveal stuck queues, rising failure rates, and poison messages.
- **Propagate trace context into jobs.** A job triggered by a request should carry the trace context, so the job's processing can be correlated to the triggering request. See telemetry-pipeline-and-collection.
- **Provide a way to inspect and requeue jobs.** Operators need to inspect stuck or dead-lettered jobs, understand why they failed, and requeue them (or discard them) after fixing the cause. A DLQ without inspection or requeue tooling traps jobs that could be recovered.

## Common Traps

### Non-Idempotent Consumer Under At-Least-Once Delivery

A consumer that produces duplicate effects (double charge, duplicate notification) when a job is redelivered after a worker crash or visibility timeout. Make consumers idempotent via deduplication or naturally-idempotent operations.

### Retries Without A Dead-Letter Queue

A poison message retried forever, consuming worker capacity and never completing. Set a max retry count and dead-letter jobs that exceed it.

### Acknowledging Before Effects Are Durable

A worker acknowledging a job before its database write or external call is committed, risking job loss if it crashes between the acknowledge and the effect. Acknowledge after effects are durable.

### Visibility Timeout Shorter Than Job Duration

A job redelivered to another worker while the first is still processing, because the visibility timeout expired. Set the timeout longer than job duration, or extend the lease for long jobs.

### Retrying Non-Retrivable (Permanent) Failures

Retrying a job that fails due to malformed input or a validation error, wasting retries on a job that will never succeed. Distinguish retriable from non-retriable; dead-letter permanent failures.

### Unbounded Queue Depth

A queue growing without limit under overload, consuming memory or storage until it fails. Set a max depth with defined overflow behavior (reject, shed, alert).

### No Monitoring Of DLQ Or Queue Depth

Dead-lettered jobs accumulating unnoticed, or a growing queue depth indicating consumers cannot keep up, both undetected. Monitor and alert on DLQ depth and consumer lag.

### Lost Jobs (Received But Never Processed)

A job received by a worker that crashes without acknowledging, with no visibility timeout to redeliver it, so the job is lost. Use visibility timeouts to redeliver jobs from crashed workers.

## Self-Check

- [ ] The queue's delivery guarantee (at-most-once, at-least-once, exactly-once) is understood, and the consumer is designed to match it — at-least-once delivery (the common case) is paired with idempotent consumers (deduplication by job ID, or naturally-idempotent operations) to achieve effectively-once processing.
- [ ] Retries handle transient failures with exponential backoff and jitter, a maximum retry count moves perpetually-failing jobs to a dead-letter queue, the DLQ is monitored and alerted on, and non-retriable (permanent) failures are dead-lettered immediately rather than retried.
- [ ] Visibility timeouts are set longer than expected job duration (or leases are extended via heartbeat for long jobs), jobs are acknowledged only after their effects are durable, and jobs are never acknowledged before processing (which would risk loss on worker crash).
- [ ] Ordering is handled where it matters (FIFO or partitioned queue with single-worker-per-partition for ordered jobs), queue depth is used as backpressure (triggering scaling or load shedding), a maximum depth bounds resource use with defined overflow behavior, and consumer lag and processing latency are monitored.
- [ ] Job lifecycle is observable — receipt, start, completion, failure, retry, and dead-letter events are logged with job IDs, metrics track jobs received/completed/failed/retried/dead-lettered and queue depth by job type, and trace context is propagated into jobs for correlation.
- [ ] Operators can inspect and requeue dead-lettered or stuck jobs, so recoverable jobs are not permanently lost in the DLQ, and the cause of failures is diagnosable for fixing the underlying bug or input issue.
- [ ] The system has been tested for the failure modes: worker crash mid-job (job redelivered, idempotent consumer handles duplicate), poison message (dead-lettered after max retries, not retried forever), visibility timeout expiry (lease extended or duplicate handled), and sustained overload (backpressure engages, queue depth bounded).
- [ ] Idempotency has been verified under actual redelivery (a redelivered job produces no duplicate effect), not just assumed — because the gap between "assumed idempotent" and "actually idempotent" is where duplicate charges, duplicate notifications, and inconsistent state originate.
