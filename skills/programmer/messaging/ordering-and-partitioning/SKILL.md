---
name: ordering_and_partitioning.md
description: Use when the agent is designing or reviewing the partitioning and ordering strategy of a streaming or partitioned messaging system (Kafka, Kinesis, Pulsar, NATS JetStream, partitioned queues), choosing partition keys and their cardinality, deciding the scope of ordering guarantees (global, per-partition, per-key, none), reasoning about the throughput-versus-ordering tradeoff, planning for consumer group rebalancing and its disruption, mitigating hot partitions via key salting or sub-partitioning, or evaluating how retries and consumer concurrency within a partition break ordering. Also covers composite keys, sticky assignment to reduce rebalance churn, the cost of rekeying, and the interplay between ordering and exactly-once processing.
---

# Ordering And Partitioning

Partitioning is the mechanism that lets a messaging system scale horizontally: data is split across partitions so that many consumers can process in parallel, and within a partition the system can offer ordering. These two properties — parallelism and ordering — are in direct tension, and the partition key is the lever that decides how the tension resolves for each entity in the system. Getting this wrong is expensive because the key and partition count are extraordinarily costly to change later: rekeying redistributes every message, breaks ordering during migration, and often requires a full replay. The judgment problem is choosing, once and durably, how to group, order, and parallelize the system's data.

Agents tend to under-invest here because the abstraction looks clean — "pick a key, the broker handles the rest" — and the documentation's promise of "ordering per partition" sounds sufficient. The harm appears under load and under failure. A high-cardinality key chosen for "even distribution" scatters one entity's events across partitions and silently breaks the per-entity ordering the business required. A low-cardinality key creates a hot partition whose consumer is overloaded while the rest sit idle, capping throughput far below the cluster's capacity. A retry of a failed message, processed after later messages have already succeeded, violates the ordering the partition was supposed to guarantee. A consumer group rebalance pauses processing and causes a burst of duplication. None of these show up in the happy path; all of them are the actual operating reality.

This skill is the deep design treatment of the ordering-partitioning-throughput tradeoff and the key-selection decision. It goes beyond the overview in the message-queue-design-and-delivery skill (which establishes that ordering is per-partition and that keys group related messages) and focuses on the hard decisions: what to key on, what scope of ordering to promise, what breaks ordering in practice, and how to handle hot partitions and rebalancing without sacrificing the guarantees the system depends on.

## Core Rules

### Choose The Partition Key Against The Ordering Requirement, Not For Distribution

The partition key determines two things at once — which events are co-located (and therefore ordered together) and how evenly load spreads across partitions — and these two goals can conflict. The key must be chosen first and foremost to satisfy the ordering requirement, because ordering is the property the business usually depends on and the property the key uniquely provides. Even distribution is secondary and can be recovered by other means; broken ordering usually cannot.

- **Key on the entity whose events must be ordered together.** If an order's events must be processed in sequence, the key is the order id; if an account's events must be ordered, the key is the account id. All events sharing a key land on the same partition and are delivered to the same consumer in order (modulo the failure modes below).
- **Match key granularity to the ordering scope.** Keying too coarsely (e.g., on a region when per-customer ordering is needed) fails to provide the required ordering; keying too finely (e.g., on a unique event id) provides no meaningful co-location and effectively disables per-entity ordering. The key must be exactly the entity whose event sequence matters.
- **Do not key on a field chosen only for spread.** A high-cardinality field (a UUID per event) distributes load evenly but scatters one entity's events across all partitions, destroying per-entity ordering. This is the most common key-selection mistake. If you need spread and you need per-entity ordering, the entity id is the key and spread must come from having enough entities and enough partitions, not from a high-cardinality key.
- **Composite keys for sub-entity ordering.** Where ordering is needed within a sub-entity (events for one item within an order), a composite key (`orderId.itemId` or `orderId` plus a sub-key) can preserve the needed scope. Use composites deliberately, since they change which events co-locate.

The decision sequence: identify the entity whose event order matters, key on that entity's id, and only then consider whether distribution is acceptable. If distribution is unacceptable with the correct ordering key, the resolution is more partitions or hot-partition mitigation (below), not a different key that breaks ordering.

### Make The Scope Of Ordering Explicit And Match It To The Requirement

Ordering is not a single property; it is a spectrum, and the system can provide different scopes at very different costs. The scope must be chosen deliberately per topic, because over-promising ordering destroys throughput and under-providing it breaks correctness.

- **No ordering.** The system delivers messages with no order guarantees. Maximum throughput and parallelism; consumers scale freely. Suitable for independent events (clicks, individual metric points, idempotent updates) where order does not matter.
- **Per-key (per-partition) ordering.** Events sharing a key are delivered in order; events across keys are unordered. This is the practical default for most business event streams — it provides the per-entity ordering the business needs while allowing cross-entity parallelism. Achieved by keying on the entity.
- **Global ordering.** All events across the entire topic are delivered in order to all consumers. This requires a single partition, which caps throughput at what one consumer can process and removes all parallelism. It is almost never the right choice at scale; if a design seems to require it, the design usually needs to be rethought (e.g., move the global-ordering requirement to a per-key requirement by identifying the real entity).

The judgment call is identifying the minimal scope that satisfies correctness. Most "we need ordering" requirements are actually per-key requirements in disguise — the mistake is escalating to global ordering because the entity boundary was not identified. Document the scope as a contract, because consumers will come to depend on it.

### Understand What Breaks Ordering In Practice

Even when the key and scope are correct, ordering can be violated by operational realities. The system's ordering guarantee is per-partition delivery order, not per-partition processing-completion order, and several mechanisms reorder processing despite in-order delivery.

- **Retries processed out of band.** A message fails, is retried, and the retry is processed after later messages in the partition have already succeeded — so the consumer sees message 1, 2, 3, then a late retry of 1. If the consumer processes retries on a separate path or with delay, ordering is violated. Where strict ordering matters, the consumer must not skip ahead past a failed message: block on the failure (with bounded retries and DLQ) rather than processing ahead and retrying behind.
- **Consumer concurrency within a partition.** If multiple threads process messages from one partition in parallel, they can complete out of order even though delivery was in order. Strict per-key ordering requires single-threaded processing per key (or per partition), which limits the throughput of that partition. Where ordering matters less, limited concurrency with key-affinity (a thread owns all messages for a key) can preserve per-key order while allowing cross-key parallelism within a partition.
- **Partition rebalancing and reassignment.** When consumers join or leave the group, partitions are reassigned, and during the handoff messages may be processed twice, skipped, or seen out of order by the new owner. Rebalancing is an ordering hazard by nature.
- **Producer retries and idempotent producers.** A producer that retries a send after a network blip may write a duplicate or deliver messages out of order if the broker does not provide an idempotent producer with sequence numbers. Enable the broker's idempotent producer where available to preserve order across producer retries.

The implication: ordering is a property of the whole path (producer, broker, consumer, retry logic), not just the broker. Each link must preserve it, or the guarantee is fictional.

### Reason About The Throughput-Versus-Ordering Tradeoff Explicitly

Partition count is the primary lever for throughput, and it trades directly against the strength of ordering. The tradeoff must be made explicit, because the temptation is to maximize one and discover the other is insufficient under load.

- **More partitions means more parallelism.** The consumer group can scale to one consumer per partition, so partition count is the ceiling on parallel consumption. Underprovisioning partitions caps throughput below peak demand, and the bottleneck cannot be fixed by adding consumers.
- **More partitions means ordering is narrower.** With many partitions, ordering exists only within each partition (per-key), never across them. The system can never order events that the key scattered across partitions.
- **Global ordering caps throughput at one consumer.** Choosing global ordering (single partition) means the entire topic's throughput is limited to what one consumer can process — usually unacceptable at any real scale.
- **Partition count is hard to change.** Adding partitions does not rehash existing data, so old messages stay on their old partitions while new messages use the new count; a key's events can split across old and new partitions, breaking ordering during and after the change. Choose the partition count for growth up front, with headroom for peak load over the system's expected lifetime.

The decision: identify the required ordering scope, key to provide it, and provision enough partitions that peak throughput fits with headroom — accepting that ordering will be per-key, not global. If global ordering is truly required, accept the single-consumer throughput ceiling or redesign.

### Plan For Rebalancing And Minimize Its Disruption

Consumer group rebalancing — the reassignment of partitions across consumers when consumers join, leave, crash, or scale — is a necessary part of horizontal scaling, and it is also a recurring source of disruption. The strategy must anticipate it.

- **Rebalancing pauses processing.** During a rebalance, consumers stop consuming while partitions are reassigned, which manifests as a latency spike or throughput dip. Frequent rebalances (from flapping consumers or aggressive autoscaling) degrade the system continuously.
- **Rebalancing can cause double-processing and reordering.** Depending on the coordination protocol, a message in flight during a handoff may be processed by both the old and new owner (duplicate) or by neither (loss, in non-exactly-once setups), and the new owner may start from a committed offset that skips or repeats recent messages. Consumers must be idempotent (see the message-queue-design-and-delivery skill) to survive rebalancing.
- **Sticky assignment reduces churn.** Sticky (or cooperative) assignment protocols keep partitions with their current owner when possible during a rebalance, minimizing movement and the disruption it causes. Prefer sticky assignment where the broker supports it.
- **Stabilize consumer membership.** Avoid flapping consumers (crash-looping pods, aggressive health-check failures) that trigger constant rebalances. Tune heartbeat and session timeouts so that a brief network blip does not cause a rebalance, but not so long that a genuinely dead consumer holds its partitions.
- **Minimize rebalances during steady state.** Scale consumers deliberately rather than frequently, and prefer fewer larger consumers over many small ones where the protocol's per-consumer overhead is significant.

### Mitigate Hot Partitions Without Sacrificing Ordering Unnecessarily

A hot partition arises when one key dominates the traffic — a single large customer, a viral entity, a "default" bucket — so its partition's consumer is overloaded while others are idle. The naive fix (rekey to spread the dominant key) breaks ordering for that key, so the mitigation must be chosen against whether ordering matters for the hot entity.

- **Key salting spreads a dominant key.** Appending a random suffix (or a bounded number of buckets) to the hot key distributes its events across several partitions, relieving the hot partition. The cost is that the key's events are no longer co-located, so per-key ordering is broken. Acceptable when ordering does not matter for that entity (e.g., independent events); unacceptable when it does.
- **Sub-partitioning preserves a weaker ordering.** Splitting a hot key into N sub-keys (`customerId#0` through `customerId#N`) spreads load while keeping each sub-key ordered internally, at the cost of ordering only within a sub-key, not across them. Useful when partial ordering is acceptable.
- **Do not sacrifice ordering silently.** The decision to salt or sub-partition is a decision to weaken a guarantee the system may have depended on. Make it explicit, document it, and ensure no consumer assumes the old ordering for the affected entity.
- **Address the root cause where possible.** A hot key is sometimes a modeling problem (a "default" or "catch-all" bucket that should be its own entity) rather than a partitioning problem. Fixing the model can remove the hotness without weakening ordering.

The judgment call is whether the hot entity's ordering matters. If it does, the only safe mitigations are more partitions (to spread the overall load, though the hot key still lands on one) or moving that entity to a dedicated stream; if it does not, salting is acceptable.

### Understand The Interplay Between Ordering And Exactly-Once

Exactly-once processing (more precisely, effectively-once — see the message-queue-design-and-delivery skill) and ordering interact in ways that constrain the design. The combination is achievable but only with specific mechanisms, and assuming both without providing for both produces silent violations.

- **Ordered processing plus idempotent consumers is the practical effectively-once.** Per-key ordering ensures events are applied in the right sequence; idempotent consumers ensure redelivery does not duplicate effects. Together they approximate exactly-once for the side effects that matter.
- **Transactions and ordering.** Broker-level transactions (Kafka transactions, transactional sessions) can tie consumption to production atomically, but they do not remove the need for the consumer's side effects to be idempotent, and they interact with ordering in subtle ways (aborted transactions leave gaps that later reads skip). Understand the specific broker's transactional ordering semantics before relying on them.
- **Ordering across retry and redelivery is the weak point.** Even with idempotent consumers, a redelivered message processed out of its original order can produce a wrong final state if the side effect is order-sensitive (e.g., a balance update applied after a later withdrawal). Where side effects are order-sensitive, the consumer must apply them in order, which means blocking on failures rather than processing ahead.

## Common Traps

### Keying On A High-Cardinality Field For Distribution

Choosing a UUID or unique event id as the partition key because it "spreads load evenly," and thereby scattering one entity's events across all partitions and silently disabling per-entity ordering. The trap is that distribution looks good in metrics while correctness breaks invisibly. Key on the entity whose order matters; accept the distribution that results.

### Keying On A Low-Cardinality Field And Creating A Hot Partition

Keying on a region, type, or status with few distinct values, so one key dominates and its partition's consumer is overloaded while others idle — capping throughput far below the cluster's capacity. The trap is that the key feels meaningful. Check the key's cardinality and skew against real traffic before committing.

### Escalating To Global Ordering Unnecessarily

Declaring global ordering because the per-key entity boundary was not identified, and accepting a single-partition throughput ceiling that the system cannot live with. The trap is treating "we need ordering" as monolithic. Identify the entity, key on it, and provide per-key ordering instead.

### Assuming Per-Partition Delivery Order Equals Processing Order

Relying on the broker's per-partition delivery guarantee while the consumer processes messages concurrently or retries failures out of band, so processing completes out of order despite in-order delivery. The trap is that the broker's guarantee is real but narrow. Ensure the consumer's processing path preserves order where ordering matters.

### Processing Ahead Past A Failed Message

Letting the consumer skip a failed message and continue with later messages, then retrying the failure later — so a retry lands after the messages it should have preceded. The trap is maximizing throughput at the cost of ordering. Where ordering matters, block on failures with bounded retries and DLQ rather than skipping ahead.

### Underprovisioning Partitions And Hitting A Throughput Ceiling

Choosing a small partition count that fits current load, then discovering at peak that the consumer group cannot scale beyond the partition count and the bottleneck cannot be fixed without rekeying. The trap is sizing for today. Provision partitions for peak load over the system's expected lifetime, since the count is hard to change.

### Rekeying Or Adding Partitions And Breaking Ordering During Migration

Changing the key or partition count to fix a problem, and breaking per-key ordering because old messages remain on old partitions while new messages hash to new ones. The trap is treating partitioning as configurable. Treat the key and count as near-permanent; if change is unavoidable, plan a migration that preserves ordering (e.g., a full replay onto the new scheme).

### Frequent Rebalances Degrading The System and salting A Hot Key And Silently Breaking Ordering

Flapping consumers or aggressive autoscaling triggering constant rebalances, so the system spends its time coordinating rather than processing, with latency spikes and duplication throughout. The trap is treating rebalancing as free. Stabilize consumer membership and use sticky assignment.

Applying key salting to relieve a hot partition without recognizing that the affected entity's events are no longer ordered, so consumers that depended on that ordering produce wrong results. The trap is a load fix that is silently a correctness regression. Make the ordering tradeoff explicit before salting.

## Self-Check

- [ ] The partition key is chosen first against the ordering requirement (the entity whose event sequence matters), not for distribution; key granularity matches the ordering scope exactly; and no high-cardinality field is used as a key where per-entity ordering is required.
- [ ] The scope of ordering is explicit per topic (none, per-key, global), chosen as the minimal scope that satisfies correctness, documented as a contract, and global ordering is used only where genuinely unavoidable with its single-consumer throughput ceiling acknowledged.
- [ ] The ordering guarantee accounts for the whole path, not just the broker: retries do not process out of band where ordering matters (the consumer blocks on failures with bounded retries and DLQ rather than skipping ahead), consumer concurrency within a partition does not break per-key order, and the broker's idempotent producer is enabled to preserve order across producer retries.
- [ ] The throughput-versus-ordering tradeoff is explicit: partition count is provisioned for peak load over the system's expected lifetime with headroom (since the count is hard to change), and the system accepts per-key rather than global ordering as the practical scope.
- [ ] Consumer group rebalancing is planned for: consumers are idempotent to survive handoff duplication, sticky or cooperative assignment is used to reduce churn, consumer membership is stabilized against flapping, and rebalance frequency is minimized during steady state.
- [ ] Hot partitions are mitigated deliberately: key salting or sub-partitioning is used only where the affected entity's ordering does not matter (or where weaker ordering is acceptable), the ordering tradeoff is made explicit rather than silently weakened, and root-cause modeling problems are considered before rekeying.
- [ ] Where effectively-once processing is required, the design combines per-key ordering with idempotent consumers, the specific broker's transactional ordering semantics are understood before relying on them, and order-sensitive side effects are applied in order to avoid wrong final states under redelivery.
- [ ] The highest-risk cases were specifically verified — a high-cardinality key silently breaking per-entity ordering, a hot partition capping throughput, a retry processed after later messages violating order, and a rekey or partition-count change breaking ordering during migration — rather than only the in-order-delivery happy path.
