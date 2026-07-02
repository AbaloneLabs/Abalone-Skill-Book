---
name: message_queue_design_and_delivery.md
description: Use when the agent is designing or reviewing asynchronous communication between services, building event-driven or pub/sub architectures, choosing or configuring a message broker or queue (Kafka, RabbitMQ, SQS, NATS, Redis Streams, etc.), defining delivery semantics (at-most-once, at-least-once, exactly-once), handling message ordering and partitioning, designing consumers and consumer groups, implementing retries, dead-letter queues, and poison-message handling, applying backpressure and flow control, evolving message schemas, or deciding between a queue and a stream/log. Also covers idempotent consumer design, the myth of broker-provided exactly-once, message size and latency tradeoffs, and the failure modes of assuming the broker handles ordering, deduplication, and failure the way the application needs.
---

# Message Queue Design And Delivery

A message queue decouples producers from consumers so they can scale, fail, and evolve independently. That decoupling is powerful, but it shifts a specific set of hard problems onto the application: what happens when a message is delivered twice, delivered out of order, delivered after a long delay, or never delivered at all. The queue does not solve these problems for you; it exposes knobs (acknowledgement, retries, ordering guarantees, dead-letter routing) that you must configure and design around. Most production failures in event-driven systems come from assuming the queue's defaults match the application's needs.

Agents tend to under-invest here because the happy path is trivial — publish, consume, ack — and the broker's documentation promises "at-least-once delivery" or "ordering per partition" that sounds sufficient. The harm appears only under failure: a consumer that crashes after the side effect but before the ack causes duplicate processing; a retry storm against a poison message exhausts the queue; ordering assumptions break when partitions change; a schema change silently breaks consumers that were not deployed; backpressure is absent and the producer overwhelms the consumer's database. The judgment problem is deciding, for each message flow, what delivery semantics and ordering the business requires, how failures are contained, and how the consumer is made safe against the reality of redelivery — then verifying the broker configuration actually delivers what was assumed.

This skill focuses on queue and delivery design. It complements the idempotency-and-race-safety skill (which covers making an individual operation safe to repeat) and the retries/timeouts/circuit-breakers skill (which covers the policy around calling external systems). Here the question is the architecture of asynchronous messaging: semantics, ordering, partitioning, failure handling, and consumer design.

## Core Rules

### Choose The Delivery Semantic Deliberately, And Know What The Broker Actually Provides

Delivery semantics are the foundation; everything else depends on them. There are three, and they have very different costs and failure modes:

- **At-most-once.** A message is delivered zero or one times; it may be lost. Cheap and fast (no ack, no persistence, or fire-and-forget). Suitable only for loss-tolerant telemetry, metrics, or best-effort signals. Never use it for anything where a lost message causes business harm.
- **At-least-once.** A message is delivered one or more times; it is not lost, but it may be redelivered. This is what most real brokers provide when configured with acknowledgement and persistence. It is the practical default for business messages — but it makes **duplicate delivery a first-class concern** that the consumer must handle.
- **Exactly-once.** A message is delivered exactly one time. In practice, true end-to-end exactly-once across an arbitrary consumer with side effects is not achievable by the broker alone; what brokers offer is effectively-once (deduplication within the broker, or transactional produce-consume that prevents redelivery under specific conditions) which still requires the consumer's side effects to be idempotent or transactional with the message acknowledgement.

The crucial judgment: assume **at-least-once** as the real-world baseline and design the consumer to be safe under redelivery, regardless of what the broker markets. "Exactly-once" features (Kafka transactions, transactional sessions, SQS deduplication) reduce redelivery windows but do not remove the obligation to make side effects idempotent, because crashes, retries, and redeployments still occur outside those windows. Decide the semantic per flow: which messages can tolerate loss (at-most-once), which require no-loss-but-tolerate-duplication (at-least-once), which require effectively-once (at-least-once plus idempotent consumer).

### Design Consumers To Be Idempotent Under Redelivery

Because at-least-once is the real baseline, every consumer must answer: "what happens if this message is processed twice?" If the answer is "the side effect happens twice" (a second charge, a second email, a second inventory decrement), the consumer is broken. Idempotency is not optional; it is the contract the consumer owes the system.

Make consumers idempotent by:

- **A durable deduplication key.** Derive a stable key from the message (producer-supplied message id, event id, or a composite of source + source-event-id) and record processed keys in a store the consumer checks before processing. Reject or no-op duplicates.
- **Conditional side effects.** Perform the side effect only if the current state allows it (e.g., a database conditional update from `pending` to `done`), so a second application is a no-op.
- **External idempotency keys.** For calls to external systems (payment providers, email), send the provider's idempotency key so the provider deduplicates.
- **Transactional acknowledgement.** Commit the side effect and the message acknowledgement in the same transaction (or use the outbox pattern) so the consumer never performs the effect without also marking the message done.

The deduplication store must itself be reliable and scoped correctly: a key scoped too narrowly (per-consumer-instance) misses duplicates across instances; too broadly (global) may falsely reject distinct messages. Scope the key to the semantic operation — same producer, same logical event.

### Make Ordering Guarantees Explicit And Match Them To The Requirement

Ordering is not free and not global. Most brokers guarantee ordering only within a partition/key, not across the whole topic, and even per-partition ordering breaks under certain failure or scaling conditions. Decide what ordering each flow requires:

- **No ordering required.** Independent events (clicks, individual metric points). Highest throughput, easiest parallelism.
- **Per-key ordering.** Events for the same entity (same order id, same account) must be processed in order, but events across entities can be parallel. Achieved by partitioning on the entity key so all events for that key land on the same partition and consumer.
- **Global ordering.** All events across the topic in order. Requires a single partition, which destroys parallelism and throughput; rarely the right choice and usually a sign the design should be rethought.

For per-key ordering, the partitioning key is the critical decision: it must group all events that must be ordered together, and it must be stable. Beware that ordering can still be violated by retries (a failed message retried after later messages were processed), by consumer concurrency within a partition, or by partition rebalancing. Where strict ordering matters, combine partitioning with a consumer that processes one message per key at a time and handles retries without skipping ahead.

### Partition For Parallelism And Affinity, Not Randomly

Partitioning serves two purposes and they must be chosen together:

- **Parallelism.** More partitions allow more consumers to process in parallel. Underprovisioned partitions cap throughput; the consumer group cannot scale beyond the partition count.
- **Affinity.** Partitioning by key keeps related messages together, enabling per-key ordering and stateful processing (a consumer handling one key holds its state locally).

The key and partition count are hard to change later (rekeying redistributes all messages and breaks ordering during migration), so choose deliberately. Common mistakes: too few partitions (throughput ceiling), too many (overhead and tiny per-partition throughput), keying on a high-cardinality field that spreads a single entity across partitions (breaks ordering), or keying on a low-cardinality field that creates hot partitions (one key dominates and its partition's consumer is overloaded while others are idle). Aim for enough partitions to carry peak load with headroom, keyed so that each entity's events are co-located and no single key dominates.

### Handle Failure With Retries, Dead-Letter Queues, And Poison-Message Strategy

Messages fail. A consumer that crashes, throws, times out, or depends on a failing downstream will fail to process some messages. The system must contain that failure rather than let it block the queue or loop forever.

- **Bounded retries with backoff.** Retry transient failures a limited number of times with backoff and jitter (see the retries skill). Unbounded retries on a persistent failure loop forever and waste capacity.
- **Dead-letter queue (DLQ).** After retries are exhausted, route the message to a DLQ rather than discarding it or blocking the main queue. The DLQ is where unrecoverable messages wait for human or automated intervention.
- **Poison-message handling.** A message that always fails (malformed, references missing data, triggers a consumer bug) will be retried forever unless bounded retries and a DLQ are in place. Detect poison messages (high failure count, parse errors) and quarantine them quickly.
- **DLQ is operational, not a trash can.** A DLQ with no monitoring or process becomes a silent graveyard of lost messages. Monitor DLQ depth, alert on growth, and define who inspects and resolves (replay after fix, discard as invalid, or route elsewhere).

Decide, per message type, the retry policy, the DLQ destination, and the operational process. A queue with no DLQ strategy turns every persistent failure into either an infinite loop or silent data loss.

### Apply Backpressure And Flow Control So Producers Do Not Overwhelm Consumers

Decoupling does not mean the producer can run arbitrarily far ahead of the consumer. If the producer publishes faster than the consumer can process, messages accumulate, latency grows, memory/disk fills, and eventually the system fails — often catastrophically (broker disk full, consumer database overwhelmed, oldest messages dropped under retention limits).

Flow-control options, in order of robustness:

- **Consumer-driven pacing.** The consumer pulls or acknowledges at its own rate, and the producer is throttled or batched to match (request/response, credit-based flow control, bounded queues).
- **Bounded queues with explicit overflow policy.** When the queue exceeds a depth, apply a deliberate policy: block the producer, shed load (drop low-priority messages), or sample. Never let the queue grow unbounded with no policy.
- **Retention limits understood.** Brokers enforce retention by time or size; if the consumer falls behind beyond retention, messages are silently dropped. Know the retention and monitor consumer lag against it.

Backpressure must be a designed property, not an emergent failure. The default question: what happens if the producer runs at full speed and the consumer is down or slow? If the answer is "the queue grows until something breaks," the design is incomplete.

### Distinguish A Queue From A Stream, And Choose The Right Primitive

"Message queue" is used loosely, but queues and streams/logs are different primitives with different guarantees and use cases, and choosing the wrong one causes recurring pain.

- **Queue (RabbitMQ queues, SQS, ActiveMQ).** Messages are delivered to a consumer and removed on ack. Work-distribution primitive: each message is processed once by one consumer. Good for command/task dispatch, RPC-over-messaging, transient work items. Once consumed and acked, the message is gone; replay is not inherent.
- **Stream / log (Kafka, Kinesis, Pulsar, NATS JetStream).** An append-only, retained, partitioned log; consumers track their own offset. Event primitive: many consumers can independently read the same history, replay from any point, and the data persists for a retention window. Good for event sourcing, audit, analytics fan-out, and any case where multiple independent consumers need the same events or where replay matters.

Choose based on the need: if you need work distribution with message deletion on consumption, a queue fits; if you need durable history, multiple independent consumers, replay, or event sourcing, a stream fits. Forcing one to behave like the other (simulating a log with a queue by copying messages, or simulating a queue with a stream by seeking to the tail) produces fragile systems.

### Evolve Message Schemas Without Breaking Consumers

Messages are a contract between producer and consumer, and consumers may be deployed at different times, owned by different teams, and running older versions. Naive schema changes break consumers silently: a renamed field, a changed type, or a removed field causes consumers to drop messages, misparse them, or fail.

Schema evolution rules:

- **Consumers must tolerate unknown fields.** Forward and backward compatibility require consumers to ignore fields they do not recognize rather than fail.
- **Make additive changes.** Add new optional fields with defaults; do not rename or remove existing fields; do not change field types incompatibly.
- **Use a schema registry and a versioned format** (Avro, Protobuf, JSON Schema) so producers and consumers negotiate compatibility, and incompatible changes are caught at deploy time rather than at runtime.
- **Version the message or event type** when a breaking change is unavoidable, and run old and new consumers in parallel during migration.
- **Communicate changes to consumer teams** and coordinate deployment order when needed; a producer cannot assume all consumers are up to date.

Treat the message schema as a public API. Breaking it is a production incident spread across every consumer.

### Make The System Observable And Reconcilable

Asynchronous systems fail silently in ways synchronous systems do not: a message is published, no one confirms it was consumed, and the missing effect is discovered much later. Observability and reconciliation are not optional.

- **Monitor producer rate, consumer rate, and consumer lag.** Lag growing unbounded is the leading indicator of consumer failure or overload.
- **Monitor retry counts, DLQ depth, and DLQ age.** These reveal failing messages and operational neglect.
- **Trace message flow end-to-end** with correlation ids propagated from producer to consumer to downstream, so a missing effect can be traced through the pipeline.
- **Make effects reconcilable.** For high-stakes flows, a periodic reconciliation job compares what should have happened (source of truth) against what did (downstream effect) and repairs gaps. Reconciliation is the safety net beneath idempotency and delivery guarantees, because every layer occasionally drops something.

## Common Traps

### Assuming The Broker Provides Exactly-Once Delivery

Reading "exactly-once" in the broker's feature list and designing the consumer without idempotency. Broker exactly-once features reduce redelivery within specific conditions but do not cover all crash, retry, and redeployment scenarios. Assume at-least-once and make consumers idempotent regardless.

### Redelivery Causes Duplicate Side Effects

A consumer that charges, emails, or decrements inventory without deduplication, so a redelivered message performs the effect twice. Every consumer of an at-least-once queue must deduplicate by a stable key or make its side effect conditional.

### Unbounded Retries On A Poison Message

Retrying a message that always fails (malformed payload, missing reference, consumer bug) forever, blocking the partition or burning capacity. Bound retries and route persistent failures to a DLQ quickly.

### Ordering Assumed Across Partitions Or Despite Retries

Assuming messages are processed in order globally, or that per-key ordering holds even when a failed message is retried after later messages were processed. Ordering is per-partition and can be violated by retry timing; combine partitioning with single-message-per-key processing where strict order matters.

### Hot Partitions Or Too-Few Partitions

Keying on a low-cardinality field so one key dominates and its partition's consumer is overloaded, or provisioning too few partitions so the consumer group cannot scale to peak load. Choose key cardinality and partition count for both balance and parallelism.

### No Backpressure, So The Queue Grows Until It Breaks

Producer publishes freely, consumer falls behind, and the queue grows until the broker runs out of disk, retention drops the oldest messages, or latency becomes unacceptable. Design flow control and monitor consumer lag against retention.

### Breaking Schema Changes That Silently Break Consumers

Renaming a field, changing a type, or removing a field without coordination, so deployed consumers drop or misparse messages. Use additive, backward-compatible changes and a schema registry.

### DLQ As A Silent Graveyard

Routing failed messages to a DLQ with no monitoring or process, so they accumulate unaddressed and effectively become data loss. Monitor DLQ depth and age, alert on growth, and define the resolution process.

### Queue Used Where A Stream Is Needed (Or Vice Versa)

Using a transient queue where durable history and multiple independent consumers are required (so replay and fan-out are impossible), or using a stream where simple work distribution with deletion is needed (so the system carries unnecessary retention and complexity). Match the primitive to the requirement.

### No End-To-End Tracing Or Reconciliation

Publishing messages with no correlation id and no reconciliation, so a missing effect is discovered late and cannot be traced. Propagate correlation ids and run reconciliation for high-stakes flows.

## Self-Check

- [ ] The delivery semantic for each flow is explicit (at-most-once, at-least-once, effectively-once), chosen against the cost of loss versus duplication, and the consumer is designed for the real baseline of at-least-once redelivery.
- [ ] Every consumer is idempotent under redelivery: a stable deduplication key or conditional side effect prevents duplicate external effects (charges, emails, inventory changes), and the deduplication scope is correct (semantic operation, not per-instance or over-broad).
- [ ] Ordering requirements are explicit per flow (none, per-key, global), partitioning keys group the events that must be ordered together, and strict-order flows handle retries without skipping ahead or processing out of order.
- [ ] Partition count and key are chosen for both parallelism (enough partitions for peak throughput) and affinity (co-located events per entity, no hot partitions), with awareness that rekeying is costly.
- [ ] Failure handling is designed: bounded retries with backoff, a DLQ for exhausted retries, poison-message detection and quarantine, and a monitored DLQ with a defined resolution process — not an infinite loop or silent loss.
- [ ] Backpressure and flow control are designed properties: consumer-driven pacing or bounded queues with an explicit overflow policy, retention limits known, and consumer lag monitored against retention so the producer cannot run the system into failure.
- [ ] The queue-versus-stream choice matches the requirement (work distribution with deletion vs. durable history, replay, and independent fan-out), and one is not forced to emulate the other.
- [ ] Message schemas evolve compatibly: consumers tolerate unknown fields, changes are additive, a schema registry or versioned format catches incompatibilities, and breaking changes are versioned with coordinated migration.
- [ ] The system is observable and reconcilable: producer/consumer rates and lag, retry counts and DLQ depth/age are monitored, correlation ids trace messages end-to-end, and high-stakes flows have a reconciliation job that repairs gaps.
- [ ] The highest-risk cases were specifically verified — duplicate side effects on redelivery, poison-message loops, ordering under retry, schema-break across deployed consumers, and unbounded lag — not only the publish-consume-ack happy path.
