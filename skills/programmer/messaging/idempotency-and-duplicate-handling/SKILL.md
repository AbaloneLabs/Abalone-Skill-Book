---
name: idempotency_and_duplicate_handling.md
description: Use when the agent is designing or reviewing idempotency and duplicate handling for an asynchronous or distributed messaging system, implementing deduplication at the consumer boundary, choosing idempotency keys and deduplication store strategy, deciding between natural and synthetic idempotency, reasoning about at-least-once delivery and the duplicates it produces, guarding against the trap of assuming exactly-once delivery, managing deduplication store TTL and growth, or making side effects safe under redelivery, replay, and crash recovery. Also covers idempotency key derivation, conditional state transitions, external idempotency keys for third-party APIs, dedup store scoping pitfalls, and the interaction between idempotency and ordering, partitions, retries, and outbox-based publishing.
---

# Idempotency And Duplicate Handling

At-least-once delivery is the real-world baseline of almost every production messaging system, and it means a message can be delivered more than once. The duplicate is not a bug in the broker; it is the unavoidable consequence of combining reliable delivery with crash recovery. A consumer that crashes after performing a side effect but before acknowledging will have the message redelivered, and if the consumer is not idempotent, the side effect happens twice — a second charge, a second email, a second inventory decrement, a second order. Idempotency is the contract the consumer owes the system: the guarantee that processing the same logical message more than once produces the same effect as processing it once. The judgment problem is not whether to be idempotent, but how to derive a correct key, where to enforce deduplication, what to store, how long to keep it, and how to reason about the cases where true idempotency is impossible.

Agents tend to under-invest here because the happy path never produces a duplicate and the broker's documentation often markets "exactly-once" features that sound like they remove the obligation. The harm appears only under failure, and it is disproportionately large and disproportionately hard to detect. A duplicate charge is a customer-facing incident; a duplicate inventory decrement is a silent accounting error discovered weeks later; a duplicate external API call with no idempotency key is a duplicate real-world action that cannot be undone. Worse, the duplicates cluster during exactly the moments the system is already stressed — crashes, rebalances, redeployments, replays — so a design that survived normal load fails precisely when resilience matters most. The discipline is to treat every consumer of an at-least-once channel as a potential duplicate target and to design deduplication that is correct, scoped, durable, and observable.

This skill is the deep treatment of idempotency and deduplication in distributed messaging. It goes beyond the consumer-idempotency overview in the message-queue-design-and-delivery skill (which establishes the need) and the retry-and-replay concerns in the dead-letter-and-retry-strategy skill (which establish that retries and replays deliver duplicates). Here the focus is the idempotency mechanism itself: key derivation, deduplication store design, natural versus synthetic idempotency, the boundary at which deduplication is enforced, and the recurring trap of assuming the broker or a transaction provides exactly-once.

## Core Rules

### Assume At-Least-Once And Never Rely On Exactly-Once From The Broker

The first decision is to accept that duplicates will occur and design for them, rather than trust a broker feature to prevent them. Exactly-once delivery, in the strict end-to-end sense across an arbitrary consumer with external side effects, is not achievable by the broker alone. What brokers offer is effectively-once under specific, narrow conditions — Kafka transactions tie produce to consume, SQS deduplication suppresses duplicates within a short window, transactional sessions acknowledge atomically — and these reduce the redelivery window but do not eliminate it. Crashes outside the transaction, redeployments, replays from a dead-letter queue, and operational interventions all occur outside those windows and all produce duplicates.

The judgment rule: assume at-least-once as the baseline for every consumer, regardless of what the broker markets, and make every side-effecting operation idempotent. Treat broker exactly-once features as an optimization that reduces duplicate frequency, not as a substitute for consumer idempotency. A consumer that skips idempotency because "the broker is transactional" will produce duplicate effects the first time it is replayed, redeployed, or crashed outside the transaction. The cost of idempotency is low and bounded; the cost of a duplicate side effect is unbounded and customer-visible.

### Derive The Idempotency Key From The Logical Operation, Not The Delivery

A correct idempotency key identifies the logical operation, not the physical delivery. The same logical event redelivered must produce the same key; two distinct logical events must produce distinct keys. Getting this wrong in either direction is catastrophic: a key that changes across redelivery defeats deduplication (the duplicate is treated as new); a key that collides across distinct events suppresses legitimate work (a real second operation is silently dropped).

- **Prefer a producer-supplied stable identifier.** A message id, event id, or a composite of source plus source-event-id, set once by the producer and preserved across redelivery and replay, is the strongest key. The producer knows the logical identity of the event; the consumer should not re-derive it heuristically.
- **Never key on delivery-level metadata.** Broker-assigned offsets, partition, delivery timestamp, or a consumer-generated UUID change on every redelivery and therefore never match. These identify the delivery, not the operation.
- **Composite keys for multi-tenant or multi-source systems.** Where the same numeric id can originate from independent sources, scope the key with a source or tenant prefix so that `sourceA:order:123` and `sourceB:order:123` do not collide. The scope must match the semantic operation: same producer, same logical event.
- **Replay must preserve the original key.** When a failed message is replayed from a dead-letter queue, it must carry the same id it originally carried. A replay that generates a fresh id is, from the consumer's perspective, a new operation, and deduplication fails. This is the single most common way replay tooling reintroduces duplicates.

The test of a correct key: if the message is delivered twice, the two deliveries yield the same key; if two genuinely different operations occur, they yield different keys. If you cannot state with confidence which logical identity the key captures, the key is not yet correct.

### Choose Natural Idempotency Where The Operation Allows It, Synthetic Otherwise

Idempotency comes in two forms, and the choice determines whether a deduplication store is needed at all.

- **Natural idempotency** is when the operation itself is safe to repeat by its semantics, with no store required. A conditional database update (`UPDATE ... SET status='paid' WHERE id=? AND status='pending'`) applied twice leaves the row in the same state; an increment rewritten as an absolute assignment (`SET balance=100` rather than `SET balance=balance+50`) is repeatable; a `PUT` to a keyed resource is idempotent by HTTP contract. Natural idempotency is preferable where available because it has no storage cost, no TTL problem, and no race window.
- **Synthetic idempotency** is when the operation is not naturally repeatable (a charge, an email, an append) and deduplication must be enforced externally by recording the idempotency key in a store and rejecting or no-oping subsequent deliveries with the same key. This requires a deduplication store and the discipline to manage it.

The judgment call is per operation. Do not reach for a deduplication store when a conditional state transition already makes the operation safe; do not assume a conditional transition suffices when the side effect is external and append-only (sending an email twice is wrong even if the local row is already `notified`). Map each side-effecting operation to its form, and use synthetic deduplication only where natural idempotency is genuinely unavailable.

### Enforce Idempotency At The Consumer Boundary, Coupled To The Side Effect

The point at which deduplication is enforced must be coupled to the side effect, not separated from it. The recurring failure mode is a check-then-act sequence — check the dedup store, then perform the effect — with a crash between them, which either performs the effect without recording it (duplicate on retry) or records it without performing it (lost effect). The deduplication check and the side effect must be atomic with respect to crash recovery.

- **Record the key in the same transaction as the side effect.** Where the side effect is a database write, insert the idempotency key into a dedup table within the same local transaction, so the effect and the record commit or roll back together. A duplicate delivery then finds the key present and no-ops.
- **Use the outbox pattern for side effects published as further messages.** When consuming a message causes the consumer to publish another, write the outbound message to an outbox table in the same transaction as the side effect and the dedup record, and have a separate relay publish it. This couples effect, dedup, and publication, preventing both duplicate effects and duplicate publications under crash.
- **External side effects need an external idempotency key.** For calls to third-party systems (a payment provider, an email service), send the provider's idempotency key so the provider deduplicates on its side. The local dedup store prevents the call from being made twice; the external key protects against the call being made, failing ambiguously, and being retried.
- **Acknowledge the message only after the effect is durably recorded.** Committing the side effect and the dedup record, then acknowledging the message, ensures a crash before ack produces a redelivery that the dedup record safely no-ops. Acknowledging before the effect is recorded is the classic source of lost effects; recording the effect without coupling the ack is the classic source of duplicates.

The rule: the dedup check, the side effect, and the acknowledgement form one atomic unit with respect to crash. Any gap between them is a window for duplicate or lost effects.

### Manage The Deduplication Store Deliberately

A synthetic deduplication store is stateful infrastructure, and its lifecycle must be designed, not defaulted. The recurring failures are unbounded growth (the store fills because keys are never expired), premature expiry (keys expire before the maximum redelivery window, so a late duplicate is treated as new), and scope errors (keys scoped per instance miss cross-instance duplicates; keys scoped globally may falsely suppress distinct operations).

- **Set the TTL against the maximum redelivery window, not arbitrarily.** A key must live at least as long as a redelivery can plausibly arrive — accounting for retry backoff, broker retention, replay latency, and operational interventions. A TTL shorter than the worst-case redelivery window is equivalent to no deduplication for late duplicates. Typical values range from hours to days depending on the system's retry and replay behavior.
- **Bound the store's growth.** A dedup table that never expires will grow without limit. Combine a TTL with partitioning or archival so the working set stays bounded. For high-throughput systems, a time-partitioned or rolling-window store keeps lookups fast.
- **Scope the key to the semantic operation.** A key scoped per consumer instance misses duplicates when a different instance processes the redelivery (common during rebalances). A key scoped globally across unrelated producers may suppress a legitimately distinct operation. The scope is: same producer, same logical event, across all consumer instances.
- **Make the store highly available and consistent.** A dedup store that is unavailable blocks processing (if the consumer waits for it) or admits duplicates (if the consumer proceeds without it). A dedup store that is eventually consistent may allow two concurrent deliveries to both pass the check. Use a store with the consistency and availability the operation requires; for high-stakes effects, a strongly consistent store is necessary.

### Recognize When True Idempotency Is Impossible And Design Around It

Some operations cannot be made idempotent by any mechanism, and pretending otherwise is dangerous. A side effect whose effect depends on the timing or sequence of application — a balance update that must apply in order, a notification that must not fire after a cancellation, an operation that interacts with concurrent state — may produce a wrong final state even if each individual application is deduplicated. For these, idempotency alone is insufficient, and the design must additionally enforce ordering (see the ordering-and-partitioning skill) or use a state machine that rejects out-of-sequence application.

The judgment rule: idempotency guarantees that repeating the same operation is safe; it does not guarantee that applying operations out of order is safe. Where order matters, combine idempotency with per-key ordering and a consumer that applies events in sequence. Where the operation is genuinely non-repeatable and non-orderable (an irreversible real-world action with no provider idempotency key), the design must gate that action on durable confirmation that it has not already been performed, and accept that some failure modes require human intervention rather than automatic retry.

## Common Traps

### Trusting Broker Exactly-Once And Skipping Consumer Idempotency

Reading "exactly-once" in the broker's feature list and designing the consumer without deduplication, so the first crash, redeployment, or replay outside the transaction window produces a duplicate side effect. The trap is that the feature sounds comprehensive but covers only narrow conditions. Assume at-least-once and make consumers idempotent regardless of broker features.

### Keying On Delivery Metadata That Changes On Redelivery

Using the broker offset, partition, delivery timestamp, or a consumer-generated UUID as the idempotency key, so a redelivered message produces a different key and is treated as a new operation. The trap is that these values feel unique and stable within a single delivery. Derive the key from the producer-supplied logical identity, preserved across redelivery and replay.

### Replay That Generates Fresh Ids

Building dead-letter replay tooling that re-publishes messages with new ids, defeating consumer deduplication and causing duplicate effects on replay. The trap is treating replay as a fresh publish rather than a re-delivery. Replay must preserve the original message identity so the consumer's deduplication holds.

### Check-Then-Act With A Crash Window

Checking the dedup store, then performing the side effect in a separate step, so a crash between them either performs the effect without recording it (duplicate on retry) or records it without performing it (lost effect). The trap is that the two steps look correct in isolation. Couple the dedup record and the side effect in one transaction, and acknowledge only after both are durable.

### Dedup Store TTL Shorter Than The Redelivery Window

Expiring idempotency keys after an arbitrary short window that is shorter than the worst-case redelivery latency, so a late duplicate (after a long retry backoff or a delayed replay) arrives after the key expired and is treated as new. The trap is that the TTL looks generous against normal latency. Set the TTL against the maximum plausible redelivery window including retries and operational replays.

### External API Call Without An Idempotency Key

Calling a third-party system (payment, email, SMS) without sending the provider's idempotency key, so an ambiguous failure (timeout after the request was sent) forces a retry that performs the effect twice on the provider's side, beyond the reach of local deduplication. The trap is that local dedup prevents the second call but cannot undo a call that already reached the provider. Always send an external idempotency key for non-repeatable external effects.

### Assuming Idempotency Implies Order Safety

Treating an idempotent consumer as sufficient for correctness when the side effect is order-sensitive, so a redelivered or out-of-order event applied after later events produces a wrong final state even though no effect was duplicated. The trap is conflating "safe to repeat" with "safe to apply out of order." Where order matters, combine idempotency with per-key ordering and in-sequence application.

### Dedup Store Scoped Per Instance

Scoping the deduplication store to a single consumer instance, so a redelivery processed by a different instance (common during rebalancing or scaling) misses the prior record and duplicates the effect. The trap is that per-instance storage is easy to reach for. Scope the store across all instances of the consumer, keyed by the semantic operation.

## Self-Check

- [ ] Every consumer assumes at-least-once delivery as the baseline and is designed to be safe under redelivery, regardless of any broker-provided exactly-once or transactional feature, with the understanding that crashes, redeployments, and replays occur outside those features' windows.
- [ ] The idempotency key is derived from the producer-supplied logical identity (message id, event id, or source plus source-event-id), preserved across redelivery and replay, never from delivery-level metadata (offset, partition, timestamp, consumer UUID), and scoped with source or tenant prefixes where the same id can originate from independent producers.
- [ ] Each side-effecting operation is classified as naturally idempotent (conditional state transition, absolute assignment) or requiring synthetic deduplication, and a deduplication store is used only where natural idempotency is genuinely unavailable — not as a default wrapper around operations that are already safe to repeat.
- [ ] The deduplication check, the side effect, and the message acknowledgement are coupled as one atomic unit with respect to crash recovery (same local transaction, or outbox pattern for downstream publications), so a crash between them cannot produce a duplicate effect or a lost effect.
- [ ] External side effects to third-party systems carry a provider idempotency key so that an ambiguous failure and retry does not perform the effect twice on the provider's side, beyond the reach of local deduplication.
- [ ] The deduplication store's TTL is set against the maximum plausible redelivery window (including retry backoff, broker retention, and operational replay latency), its growth is bounded by TTL plus partitioning or archival, and its scope spans all consumer instances keyed by the semantic operation — not per-instance and not so global that it suppresses distinct operations.
- [ ] Where side effects are order-sensitive, idempotency is combined with per-key ordering and in-sequence application, and the design does not assume that deduplication alone prevents wrong final states under out-of-order or redelivered events.
- [ ] The highest-risk cases were specifically verified — a crash between effect and ack, a redelivery processed by a different instance, a replay with a fresh id, a late duplicate arriving after TTL expiry, and an ambiguous external call retried without a provider key — rather than only the single-delivery happy path.
