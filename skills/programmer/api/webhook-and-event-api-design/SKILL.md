---
name: webhook_and_event_api_design.md
description: Use when the agent is designing a webhook or event-driven API — sending notifications to consumers on state changes, defining event payloads and schemas, handling webhook delivery and retries, signing and verifying webhooks, managing event ordering and delivery semantics, designing an event stream or pub/sub API, or consumers building reliable webhook receivers. Covers event payload design, delivery guarantees (at-least-once and idempotent consumers), webhook signing and security, ordering and deduplication, retry and dead-letter handling, versioning event schemas, and the asymmetry where the producer cannot assume the consumer's reliability.
---

# Webhook And Event API Design

A webhook (or event API) inverts the normal request-response relationship: instead of the consumer calling the producer, the producer calls the consumer when something happens. This inversion creates a reliability asymmetry that defines the design. In a request-response API, the caller controls when calls happen and how failures are handled; in a webhook, the producer must deliver events to a consumer whose availability, performance, and correctness the producer does not control. The consumer may be down, slow, return errors, or mishandle the event, and the producer must handle all of these without losing events (which represent real state changes) or blocking on unresponsive consumers (which would back up the producer's own system). The delivery guarantee is almost always at-least-once — the producer retries until the consumer acknowledges, which means the consumer may receive the same event more than once, and must be idempotent. And because the producer calls out to arbitrary endpoints, security (proving the event came from the producer, not an attacker) and protection (the producer must not be exploited via consumer endpoints) are first-order concerns.

Agents tend to design webhooks as "send an HTTP POST when something happens" without addressing delivery retries, idempotency, signing, ordering, or consumer failure modes, producing systems that lose events when consumers are briefly unavailable, deliver duplicates that corrupt consumer state, or are vulnerable to forged events. The judgment problem is recognizing that a webhook is an asynchronous, at-least-once, producer-to-arbitrary-consumer channel whose reliability rests on retries, consumer idempotency, cryptographic signing, and bounded backoff, and that the producer must protect itself from consumer failures while guaranteeing delivery to healthy consumers. This skill covers the discipline of webhook and event API design: event payloads, delivery semantics, signing, ordering, retries, versioning, and consumer-side reliability.

## Core Rules

### Design Event Payloads That Are Self-Contained And Versioned

An event travels without its producer's context, so it must carry everything the consumer needs, and its schema must evolve without breaking consumers.

- **Make events self-contained: include the event type, timestamp, producer identity, the entity affected, and the relevant data.** A consumer receiving the event may have no other context; the event must identify what happened, when, to what, and the new state (or a pointer to it). Include an event ID for deduplication.
- **Include a version in the event schema.** Event schemas evolve (fields added, semantics changed); a version allows consumers to handle multiple schema versions and producers to evolve without breaking existing consumers. See api-versioning-and-deprecation.
- **Decide between carrying the full new state (event-carried state transfer) vs a notification (the consumer fetches the current state).** Full state makes the consumer independent (no callback to the producer) but larger events and potential data duplication; notification is smaller but couples the consumer to the producer's read API. Match the choice to the consumer's needs.
- **Use a stable, documented schema (ideally a registry).** Events consumed by multiple parties need a documented, versioned schema, ideally in a schema registry (Avro, Protobuf, JSON Schema) that producers and consumers reference. Undocumented event schemas drift and break consumers.

### Guarantee At-Least-Once Delivery With Idempotent Consumers

Webhook delivery is at-least-once: the producer retries until acknowledged, so the consumer may receive duplicates. The system's correctness rests on consumer idempotency.

- **Deliver at-least-once: retry until the consumer acknowledges (2xx).** An event represents a real state change; losing it is incorrect. Retry (with backoff — see retries-timeouts-and-circuit-breakers) until the consumer acknowledges, so a briefly-unavailable consumer eventually receives the event.
- **Make consumers idempotent by event ID.** The consumer tracks processed event IDs and ignores duplicates; processing the same event twice produces the same effect as once. Without idempotency, a redelivered event causes duplicate effects (double charges, duplicate records).
- **Do not assume exactly-once delivery.** Networks are unreliable, consumers crash mid-processing, and acknowledgements can be lost; exactly-once delivery is not achievable without prohibitive coordination. Design for at-least-once with idempotent consumers.
- **Handle the "acknowledged but not processed" case.** A consumer that acknowledges before processing (to avoid redelivery) and then crashes loses the event; a consumer that processes before acknowledging and crashes gets a redelivery (handled by idempotency). Prefer process-then-acknowledge with idempotency for at-least-once processing.

### Sign Webhooks For Authenticity And Integrity

A consumer receiving a webhook must verify it came from the claimed producer and was not tampered with. Signing provides this.

- **Sign each webhook with a cryptographic signature (HMAC or asymmetric).** The producer signs the payload (and metadata: timestamp, event ID) with a secret or private key; the consumer verifies the signature with the shared secret or public key. A forged webhook (from an attacker) fails verification.
- **Include a timestamp in the signature and reject replayed events.** Without a timestamp, a captured valid webhook can be replayed later (a replay attack). Include a timestamp in the signed data; the consumer rejects webhooks with timestamps outside a window (e.g., 5 minutes).
- **Use a constant-time signature comparison.** A naive comparison (==) is vulnerable to timing attacks that reveal the signature byte by byte. Use a constant-time comparison function.
- **Rotate signing keys and support multiple active keys.** Keys must be rotatable (compromised or aged); support multiple active keys during rotation so consumers verifying against the old key are not broken before the new key is distributed.

### Handle Ordering And Concurrency Deliberately

Events have a natural order (the order they occurred), but delivery may not preserve it (retries, parallel delivery). Define the ordering guarantee and help consumers handle disorder.

- **Decide whether ordering matters and guarantee it where it does.** For state mutations (a balance updated twice), the order matters; the consumer must apply them in order. For independent events (separate user signups), order does not matter. Guarantee ordering only where it matters, because it constrains delivery (sequential, no parallelism).
- **For ordered delivery, use a partition key that routes related events to the same delivery stream.** Events for the same entity (same user, same order) go to the same partition, delivered sequentially; events for different entities are delivered in parallel. This balances ordering (within an entity) with throughput (across entities).
- **Include sequence or version numbers for consumers to detect gaps and disorder.** A monotonically increasing sequence per entity lets the consumer detect out-of-order delivery (sequence went backwards) or gaps (a missing sequence). The consumer can then buffer, fetch the missing event, or alert.
- **Do not assume global ordering across unrelated events.** A global order across all events requires serial delivery (no parallelism), which limits throughput. Order within an entity is usually sufficient.

### Manage Retries, Dead-Lettering, And Consumer Failure

The producer must deliver to consumers that may be down, slow, or persistently failing, without backing up its own system or losing events.

- **Retry with exponential backoff and jitter.** A consumer that is briefly down should receive the event on retry; backoff (increasing delay between retries) avoids overwhelming a recovering consumer, and jitter spreads retries. See retries-timeouts-and-circuit-breakers.
- **Set a maximum retry count and dead-letter persistently-failing events.** A consumer that never acknowledges (misconfigured endpoint, persistent failure) should not be retried forever. After N retries, move the event to a dead-letter queue for investigation (and later reprocessing). See queue-and-background-job-reliability.
- **Set delivery timeouts and concurrency limits per consumer.** A slow consumer should not hold delivery connections indefinitely; set a timeout (e.g., 30 seconds). Limit concurrent deliveries to a single consumer (to avoid overwhelming it) and across all consumers (to bound the producer's resource use).
- **Circuit-break persistently-failing consumers.** A consumer that consistently fails (returns 5xx, times out) should be circuit-broken (delivery paused) to avoid wasting resources, with periodic probes to detect recovery. See retries-timeouts-and-circuit-breakers.
- **Provide delivery status visibility.** Consumers (and the producer's operators) should be able to see delivery status (sent, acknowledged, failed, dead-lettered) and replay or manually trigger redelivery for specific events.

### Version And Evolve The Event Schema Without Breaking Consumers

Event schemas evolve, and evolution must not break existing consumers, who may not update in sync with the producer.

- **Make additive changes (new optional fields) backward-compatible.** Adding a new optional field does not break consumers that ignore unknown fields. Prefer additive evolution; avoid removing or renaming fields (breaking) or changing semantics (subtly breaking).
- **Use versioning for breaking changes, with a migration path.** When a breaking change is necessary, version the event (a new event type, a version field) and deliver both versions during a transition period, deprecating the old after consumers migrate. See api-versioning-and-deprecation.
- **Communicate schema changes to consumers in advance.** Consumers relying on the event need notice of changes (additive or breaking) to update their handling. Use a changelog, schema registry notifications, or direct communication.
- **Consider a schema registry for producer-consumer contract management.** A registry (Avro/Protobuf schema registry) enforces compatibility rules (backward, forward, full) on schema evolution, preventing breaking changes mechanically.

## Common Traps

### Fire-And-Forget Delivery (Events Lost)

Sending a webhook without retrying on failure, losing events when the consumer is briefly unavailable. Deliver at-least-once with retries.

### Non-Idempotent Consumer Under At-Least-Once Delivery

A consumer that applies duplicate effects when an event is redelivered, because it does not deduplicate by event ID. Make consumers idempotent.

### Unsigned Or Weakly-Signed Webhook

A webhook without a cryptographic signature, allowing an attacker to forge events the consumer accepts. Sign with HMAC or asymmetric; verify with constant-time comparison; include a timestamp.

### No Timestamp / Replay Protection

A valid webhook that can be captured and replayed later because there is no timestamp in the signature. Include a timestamp; reject outside a window.

### Assuming Exactly-Once Delivery

Designing as if the consumer receives each event exactly once, when at-least-once delivery means duplicates are possible. Design for at-least-once with idempotency.

### Retrying Forever On A Persistently-Failing Consumer

Retrying a consumer that never acknowledges indefinitely, backing up the producer's delivery. Set a max retry count; dead-letter; circuit-break.

### No Ordering Guarantee Where Ordering Matters

Delivering state-mutating events out of order (parallel delivery to the same entity), corrupting consumer state. Use partition keys for per-entity ordering.

### Breaking Schema Change Without Versioning

Changing the event schema (removing/renaming a field) without versioning or migration, breaking consumers silently. Use additive changes; version breaking changes; communicate in advance.

## Self-Check

- [ ] Event payloads are self-contained (event type, timestamp, producer identity, entity, data, event ID), versioned (schema version for evolution), and follow a documented schema (ideally in a registry), with a deliberate choice between event-carried state transfer and notification.
- [ ] Delivery is at-least-once (retried until the consumer acknowledges, with exponential backoff and jitter), consumers are idempotent by event ID (duplicates produce no extra effect), exactly-once delivery is not assumed, and processing is process-then-acknowledge so crashes produce redelivery (handled by idempotency) rather than loss.
- [ ] Webhooks are cryptographically signed (HMAC or asymmetric) for authenticity and integrity, the signature includes a timestamp and is verified with constant-time comparison to prevent replay and timing attacks, and signing keys are rotatable with multiple active keys supported during rotation.
- [ ] Ordering is handled deliberately — guaranteed where it matters (state mutations) via partition keys routing related events to sequential delivery, sequence/version numbers included for consumers to detect gaps and disorder, and global ordering across unrelated events is not assumed (it would limit throughput).
- [ ] Retries are bounded (max count, exponential backoff with jitter), persistently-failing events are dead-lettered, delivery timeouts and per-consumer concurrency limits prevent slow consumers from backing up the producer, persistently-failing consumers are circuit-broken, and delivery status is visible for replay and debugging.
- [ ] The event schema evolves without breaking consumers — changes are additive (new optional fields), breaking changes are versioned with a migration path and transition period, changes are communicated in advance, and a schema registry enforces compatibility where used.
- [ ] The producer protects itself from consumer failures: delivery is asynchronous (does not block the producing operation), timeouts and concurrency limits bound resource use, circuit-breaking prevents wasting resources on dead consumers, and the delivery infrastructure (queue, workers) is itself reliable (see queue-and-background-job-reliability).
- [ ] Consumer-side guidance is provided: consumers should verify signatures, handle idempotency, respond quickly (acknowledge before heavy processing, process asynchronously), handle out-of-order events, and have their own retry/redelivery capability — because the producer cannot assume the consumer's reliability.
