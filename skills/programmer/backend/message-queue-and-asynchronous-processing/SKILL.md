---
name: message_queue_and_asynchronous_processing.md
description: Use when the agent is adding asynchronous processing to a backend service using a message queue or broker (Kafka, RabbitMQ, SQS/SNS, Redis Streams, ActiveMQ, NATS, Celery, Sidekiq, Spring @Async/listener), producing or consuming messages, choosing between a queue and a stream, designing consumers and workers, handling delivery semantics (at-least-once, at-most-once, exactly-once), acknowledgments and visibility timeouts, retries with backoff, poison messages and dead-letter queues, backpressure and consumer scaling, or diagnosing duplicate processing, lost messages, stuck/undelivered messages, consumer crashes mid-processing, queue backlog growth, or thundering-herd retry storms. Covers the at-least-once default and its consequences, idempotent consumers, acknowledgment discipline, and the boundary between async work and the request path.
---

# Message Queue And Asynchronous Processing

Asynchronous processing moves work off the request path onto a queue and a set of consumers, and the reason it is hard is that it changes the failure model. A synchronous call either succeeds or fails, visibly, within the request; an asynchronous message is delivered at least once, processed eventually, and may be redelivered after a crash, a timeout, or a deploy — so the consumer must be correct under duplication, partial failure, and ordering that is weaker than the application assumed. The dominant errors are all consequences of this: a consumer that is not idempotent double-charges on redelivery; a consumer that acknowledges before doing the work loses the message on a crash; a poison message that always fails is retried forever, blocking the queue; a retry without backoff turns a downstream outage into a self-inflicted denial of service. A queue makes a system more scalable and more fragile in new, invisible ways.

The judgment problem is not "how do I send a message" but "what delivery guarantees does this broker actually provide, is the consumer correct under redelivery and partial failure, what happens when the downstream is slow or down, and where does the queue's backlog show up." Agents tend to assume exactly-once delivery (almost no broker provides it; the default is at-least-once), to acknowledge on receipt rather than on completion (losing messages on crash), to retry without backoff or a cap (amplifying outages), and to treat the queue as a reliable work list without handling the poison-message case. Each produces a system that processes the happy path correctly and fails silently or catastrophically under exactly the conditions (crash, outage, redelivery) that queues exist to absorb.

## Core Rules

### Assume At-Least-Once Delivery; Make Every Consumer Idempotent

The single most important fact about message queues is that the overwhelming default delivery semantic is at-least-once: a message will be delivered at least once, and under failure conditions (consumer crash before ack, network blip, broker redelivery after a visibility timeout) it will be delivered more than once. Exactly-once delivery is offered by almost no broker in a meaningful, end-to-end sense (the few that claim it provide it only within their own transaction boundary, not across the consumer's side effects), and assuming it is the root cause of most duplicate-processing bugs. Treat every consumer as if it will receive the same message more than once, because under real failure conditions it will.

The defense is idempotency: processing the same message twice must have the same effect as processing it once. For a payment, this means an idempotency key deduplicates the charge. For a database write, a unique constraint or an upsert prevents a duplicate row. For an email send, a "sent" flag or a sent-log prevents a duplicate email. For a state transition, checking the current state before applying ("only if not already processed") prevents a double-transition. Design the consumer around a deduplication key (the message id, a business id, an idempotency key) stored where the side effect happens, and check it before applying the effect. A consumer that is not idempotent is a duplicate-side-effect bug waiting for the first redelivery — and redelivery is not rare, it is routine.

### Acknowledge After Processing, Not Before; Understand Visibility Timeouts

Acknowledgment (ack/nack/commit offset) tells the broker the message is done and can be removed (or, on nack, redelivered). The rule is to acknowledge after the work is complete and its side effects are durable, not when the message is received. Acknowledging on receipt and then crashing before processing loses the message permanently (the broker thinks it was delivered). Acknowledging after the database commit but before the email send loses the email if the process dies between them — so the side effects must be ordered so that the ack comes last, after all critical work, or the non-acknowledged side effects must be recoverable (see the outbox pattern below).

Visibility timeouts (SQS, RabbitMQ consumer timeouts) are the broker's mechanism for detecting a consumer that crashed without acking: after a message is delivered, it is invisible to other consumers for the visibility window; if not acked within the window, it becomes visible again and is redelivered. Set the visibility timeout longer than the maximum processing time (with margin), or the broker will redeliver a message that is still being processed, causing duplicate concurrent processing. If processing can exceed the timeout (a long job), extend it (SQS `ChangeMessageVisibility`, RabbitMQ heartbeats) or checkpoint progress so a redelivery resumes rather than restarts. A visibility timeout shorter than processing time is a duplicate-processing bug; an ack-before-processing is a lost-message bug.

### Bound Retries With Exponential Backoff, And Dead-Letter Poison Messages

When a message fails (the downstream is down, a transient error), the consumer should retry — but with discipline. Retry with exponential backoff and jitter (so a fleet of consumers does not retry in lockstep and hammer the downstream), a maximum retry count (so a message that will never succeed does not retry forever), and a dead-letter queue (DLQ) for messages that exhaust their retries. A poison message — one that always fails (a malformed payload, a bug in the consumer, a referentially-missing entity) — must be removed from the main queue and parked in a DLQ, where it can be inspected and handled manually, rather than blocking the queue forever.

The trap is the retry storm: a downstream outage causes every message to fail and retry, and without backoff the retries amplify the outage (the downstream comes back up to a wall of immediate retries and goes down again). With backoff and jitter, the retries spread out and the downstream recovers. With a retry cap and DLQ, poison messages stop consuming capacity. Set the retry policy deliberately: how many retries, what backoff curve, what total time-in-flight before DLQ, and who monitors and processes the DLQ (an unmonitored DLQ is where messages go to be silently lost). The full retry/DLQ strategy is covered in the messaging-area skills; the integration rule is that every consumer has an explicit retry policy and a DLQ, chosen rather than defaulted.

### Handle Ordering According To What The Application Actually Needs

Ordering is a property of the broker and the consumer topology, and assuming stronger ordering than the system provides is a subtle correctness bug. Most brokers guarantee ordering within a partition (Kafka partition, SQS FIFO message group, RabbitMQ queue with a single consumer) and do not guarantee ordering across partitions or with multiple concurrent consumers. A consumer fleet of N processing from one queue will process messages concurrently and out of order. If the application requires ordering (process event A before event B for the same entity), the messages must be routed to the same partition (by entity id, by key) and consumed by a single consumer per partition — which limits parallelism.

Decide what ordering the application actually needs. Often, per-key ordering (all events for one user/order/account in order) is what matters, and that is achievable by key-based partitioning with one consumer per partition. Global ordering (all events in order) is rarely needed and forces a single partition, killing parallelism. If events are independent (no ordering needed), use any partitioning for throughput. If events have a happens-before relationship only within a key, partition by key. And handle the case where a redelivery reorders: if message 2 is processed and acked but message 1 is redelivered after a crash, the consumer sees them out of order — idempotency and state-checking ("only transition from state X to Y") defend against this. The full ordering/partitioning discipline is in the messaging-area skills; the integration rule is to know the broker's ordering guarantee, partition for the needed ordering, and defend against redelivery reorder with idempotent state checks.

### Use The Transactional Outbox To Reliably Publish After A Database Write

A common requirement is "atomically update the database and publish a message" (e.g., on order creation, write the order and publish an OrderCreated event). The naive approach — write to the database, then publish to the broker — is not atomic: if the publish fails (broker down, network error), the database has the order but no event was published, and the system is inconsistent; if you publish first then write, the event exists for an order that was never created. Two-phase commit across the database and the broker is rarely available and usually too slow. The standard solution is the transactional outbox: write the event to an `outbox` table in the same database transaction as the order, then a separate process (a poller, or change-data-capture like Debezium) reads the outbox and publishes to the broker, tracking what has been published.

The outbox makes "database write + eventual publish" reliable: the write and the event-record commit atomically, so they always agree; the publisher retries publishing until it succeeds; and on crash/restart the publisher resumes from the outbox. This is the correct pattern for any "do X and publish an event about X" requirement, and it is the foundation of event-driven services that must not lose or invent events. Do not try to make the database-then-publish atomic with a distributed transaction; use the outbox and accept that publishing is eventual (which is fine, because the consumer is idempotent and handles redelivery). The full event-driven discipline is in the messaging-area skills; the integration rule is that publishing after a database write uses the outbox, not a best-effort publish.

### Bound The Queue And Apply Backpressure; Watch For Backlog Growth

A queue decouples producers from consumers, and the decoupling is a feature (the producer is not blocked by a slow consumer) and a hazard (if the producer is consistently faster than the consumer, the queue grows without bound, latency grows, memory fills, and eventually the queue or the consumers fail). Every queue needs to be bounded or monitored for backlog: if the production rate can exceed the consumption rate sustainably, you need more consumers (scale out), a faster consumer (optimize), or backpressure (slow the producer — reject requests, return 429, or block). An unbounded queue with a sustained rate mismatch is a slow-bleed failure that surfaces as ever-growing latency and an eventual OOM or queue-limit crash.

The discipline: monitor queue depth and consumer lag as primary metrics, alert on sustained growth (not transient spikes), and have a scaling strategy (auto-scale consumers on lag) and a backpressure strategy (what the producer does when the queue is full). Understand the latency implication of queueing: a message in a queue for 10 seconds adds 10 seconds to the end-to-end latency of the work, and a backlog of N messages at rate R adds N/R seconds — so a growing backlog is a growing latency problem even before it becomes a capacity problem. For latency-sensitive work, size consumers to keep the queue near-empty; for throughput-oriented work, a deeper queue is tolerable but still must be bounded. The queue is a shock absorber, not an infinite buffer.

### Choose Queue (Work Distribution) vs Stream (Event Log) For The Use Case

Brokers fall into two broad families with different semantics, and the choice shapes the whole design. A **queue** (RabbitMQ, SQS, ActiveMQ, Redis lists) is for work distribution: each message is consumed by one consumer and removed on ack; the goal is to distribute work across consumers and delete it when done. A **stream** or **log** (Kafka, Kinesis, Pulsar, Redis Streams) is an append-only event log: messages are retained (for hours to forever), multiple consumers read independently at their own offset, and the goal is to replay history and fan out to many independent consumers. A queue is right for "process this job once"; a stream is right for "this is a fact that happened, and several systems care about it, possibly including future systems that want to replay from the start."

The decision drives retention, consumer model, and replayability. A queue's messages are gone once acked — there is no replay, no second consumer that starts later and reads history. A stream's messages persist — a new consumer can start from the beginning and rebuild its view, and multiple consumers read independently. If you need replay (a new service that rebuilds a read model, an audit log, multiple downstream systems), use a stream. If you need one-shot work distribution (send this email, process this image), use a queue. Using a queue where replay is needed forces workarounds (a separate audit store); using a stream where one-shot work is needed adds retention and offset-management complexity. Match the broker family to the use case before committing.

## Common Traps

### Assuming Exactly-Once Delivery

Almost no broker provides end-to-end exactly-once; the default is at-least-once, and redelivery is routine. Make consumers idempotent; do not rely on the broker to deliver once.

### Acknowledging On Receipt Instead Of After Processing

Ack-then-crash loses the message permanently. Acknowledge after the work and its side effects are durable; order side effects so the ack is last.

### Non-Idempotent Consumer Double-Processing On Redelivery

A consumer with no deduplication that double-charges or double-inserts when a message is redelivered. Use an idempotency key / unique constraint / state check.

### Visibility Timeout Shorter Than Processing Time

The broker redelivers a message still being processed, causing duplicate concurrent processing. Set the timeout above max processing time; extend it for long jobs.

### Retry Without Backoff Amplifying An Outage

Immediate retries in lockstep hammering a recovering downstream. Use exponential backoff with jitter, a retry cap, and a DLQ for poison messages.

### Poison Message Retried Forever, Blocking The Queue

A message that always fails (malformed, buggy consumer) retried without a cap. Bound retries and dead-letter exhausted messages; monitor the DLQ.

### Database Write + Publish Not Atomic (Lost Or Invented Events)

Publish-after-write that loses the event on publish failure, or invents one on write failure. Use the transactional outbox so the write and event-record commit atomically.

### Unbounded Queue With A Sustained Rate Mismatch

Producers faster than consumers, backlog and latency growing without bound until OOM or queue-limit crash. Monitor depth/lag, scale consumers, apply backpressure.

### Assuming Ordering The Broker Does Not Provide

Concurrent consumers processing out of order, or cross-partition reordering on redelivery. Partition by key for per-key ordering; defend with idempotent state checks.

### Using A Queue Where Replay Is Needed (Or A Stream For One-Shot Work)

A queue's messages are gone once acked (no replay); a stream adds retention/offset complexity for one-shot work. Match the broker family (queue vs stream) to the use case.

## Self-Check

- [ ] Every consumer is designed for at-least-once delivery and is idempotent (deduplication key / unique constraint / upsert / state-check before applying the side effect), so redelivery does not cause duplicate side effects.
- [ ] Consumers acknowledge after processing and its durable side effects (not on receipt), visibility timeouts exceed maximum processing time (with margin or extension for long jobs), and ack ordering ensures no critical side effect is left un-acked.
- [ ] Retries use exponential backoff with jitter, a maximum count, and a dead-letter queue for poison messages; the DLQ is monitored and processed, not a silent loss.
- [ ] The broker's ordering guarantee is known, messages are partitioned by key where per-key ordering is needed, and redelivery reorder is defended against with idempotent state checks ("only transition from state X").
- [ ] "Database write + publish event" uses the transactional outbox pattern (event written in the same transaction, a separate publisher reliably publishes and tracks), not a best-effort publish that can lose or invent events.
- [ ] Queue depth and consumer lag are monitored as primary metrics with alerting on sustained growth, consumers auto-scale on lag, and producers apply backpressure (reject/429/block) when the queue is full; latency implications of backlog are understood.
- [ ] The broker family (queue for one-shot work distribution vs stream for retained, replayable event log) matches the use case, chosen before committing rather than defaulted.
- [ ] The async path was tested under the conditions that break it — consumer crash mid-processing, broker redelivery, downstream outage with retries, poison message, backlog growth — and no lost messages, duplicate side effects, or stuck queues appear.
