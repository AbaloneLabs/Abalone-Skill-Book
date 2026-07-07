---
name: data_partitioning_and_sharding.md
description: Use when the agent is partitioning or sharding a database or dataset — splitting data across multiple nodes, databases, or tables by a shard key; choosing a partitioning strategy (range, hash, consistent hashing, geographic); designing the shard key; handling cross-shard queries, joins, and transactions; rebalancing and resharding; or diagnosing hot spots and uneven distribution. Covers partition key selection, distribution evenness, hot-spot avoidance, cross-partition operation limitations, resharding and rebalancing, and the tradeoff between write scalability and query/transaction constraints that partitioning introduces.
---

# Data Partitioning And Sharding

Partitioning (sharding) splits a dataset across multiple nodes to scale beyond what a single node can handle, and its central tension is between write scalability and the constraints it imposes on queries and transactions. A single database can serve a finite amount of data and write throughput; partitioning distributes the load, but it does so by imposing a rule — the shard key — that determines where each record lives, and that rule constrains everything else. A query that filters by the shard key is fast (it routes to one shard); a query that does not must fan out to all shards (slow, expensive) or be answered from a secondary index (which itself must be partitioned or replicated). A transaction that touches records on one shard is straightforward; a transaction that touches records on multiple shards requires a distributed transaction (slow, complex, often unavailable). The shard key is the most consequential and hardest-to-change decision in a partitioned system: it determines distribution evenness, hot-spot risk, and which queries and transactions are efficient, and once data is distributed by a key, changing the key is a massive resharding effort. Choosing it casually is a mistake that constrains the system for its lifetime.

Agents tend to choose a shard key for immediate convenience (auto-increment id, a timestamp) without analyzing distribution or query patterns, to assume partitioning solves scalability without acknowledging its query and transaction costs, and to defer the resharding problem until it is forced. The judgment problem is recognizing that the shard key governs the system's query and transaction capabilities, that distribution evenness and hot-spot avoidance are properties of the key choice, and that resharding is so costly that the key must be chosen with the long-term access pattern in mind. This skill covers the discipline of data partitioning: shard key selection, distribution and hot spots, cross-partition operations, resharding, and the scalability-vs-flexibility tradeoff.

## Core Rules

### Choose The Shard Key Based On Access Patterns And Distribution

The shard key determines where data lives, which queries are fast, and how evenly data is distributed. It is the most important partitioning decision and the hardest to reverse.

- **Choose a key that the dominant queries filter on.** The most common queries (by user id, by tenant id, by order id) should route to a single shard, which requires the shard key to be a prefix of or equal to the query filter. A key that the queries do not use forces fan-out to all shards.
- **Ensure the key distributes data evenly.** A key with low cardinality or skewed distribution (a status field with one dominant value, a geographic key concentrated in one region) creates uneven shards — some overloaded, others idle. Hash partitioning (hashing the key before assignment) evens out skewed keys; range partitioning does not.
- **Avoid monotonically increasing keys (auto-increment, timestamps) for hash partitioning.** A monotonically increasing key hashed to N shards spreads inserts across shards (good for write throughput) but the "hot" range of new data moves over time, complicating range scans. For time-series data, range partitioning by time (with archival of old partitions) is often better than hash.
- **Consider compound keys for access-pattern flexibility.** A compound key (tenant_id, user_id) allows routing by tenant (queries filtered by tenant go to one shard) while distributing across tenants. The order matters: the leading component is the routing key.
- **Make the key immutable or near-immutable.** Changing a record's shard key means moving it between shards (expensive, complex). Choose a key that does not change (a user id does not; a geographic region might if users move).

### Understand What Partitioning Enables And What It Costs

Partitioning scales write throughput and data volume beyond a single node, but it imposes costs on queries, joins, and transactions. These costs must be understood, not assumed away.

- **Scales: write throughput (writes distributed across shards), data volume (each shard holds a fraction), and locality (data accessed together on one shard).** A partitioned system handles more data and more writes than a single node, and queries filtered by the shard key are fast (single-shard routing).
- **Costs: cross-shard queries (fan-out, slow, expensive), cross-shard joins (often unsupported or very expensive), cross-shard transactions (distributed, slow, complex, often unavailable), and uniqueness enforcement across shards (a unique constraint cannot span shards unless the uniqueness key is the shard key).** Any operation not scoped to a single shard pays these costs.
- **Partitioning trades query and transaction flexibility for scalability.** The system scales because operations are scoped to shards; operations that span shards are limited or expensive. Design the access patterns around single-shard operations; accept that cross-shard operations are constrained.
- **Do not partition prematurely.** A single node that handles the load does not need partitioning, and partitioning's costs (query constraints, operational complexity) are not worth paying before they are needed. Partition when a single node is demonstrably insufficient (data volume, write throughput), not speculatively.

### Avoid Hot Spots In The Shard Key Distribution

A hot spot is a shard that receives disproportionate load (writes or reads), becoming a bottleneck while other shards are underutilized. Hot spots defeat partitioning's scalability.

- **Hash the shard key to spread skewed distributions.** A key with skewed values (a few dominant tenants, a concentrated region) creates hot shards if range-partitioned. Hashing the key before assignment spreads the values evenly across shards, eliminating the hot spot (at the cost of losing range locality).
- **Avoid hot keys in the access pattern.** Even with even distribution, a single key that receives disproportionate traffic (a celebrity's account, a global counter) creates a hot spot on its shard. Redesign hot keys (sub-shard the celebrity across sub-shards, use a distributed counter) to spread their load.
- **Beware time-based hot spots in time-series data.** The current time window (today's data) receives all writes, creating a hot shard. Range-partition by time (today's shard is hot but bounded; old shards are cold) and design for the hot shard's capacity.
- **Monitor shard load and rebalance hot shards.** Detect uneven load (one shard's CPU, I/O, or request rate far exceeding others) and rebalance (split the hot shard, migrate data) before it becomes a bottleneck.

### Handle Cross-Shard Queries And Aggregations Deliberately

Queries that do not filter by the shard key must fan out to all shards. This is expensive and must be designed for, not stumbled into.

- **Prefer queries filtered by the shard key (single-shard routing).** Design access patterns so the common queries include the shard key. A query "get user X's orders" routes to one shard if sharded by user id; "get all orders over $100" fans out to all shards.
- **For fan-out queries, use scatter-gather with bounded parallelism.** A cross-shard query sends to all shards (scatter) and combines the results (gather). Bound the parallelism (do not overload with thousands of parallel shard queries), paginate the results, and accept the latency (the query is as slow as the slowest shard).
- **For aggregations (count, sum), consider pre-computed or approximate results.** A precise cross-shard count requires querying all shards and summing — expensive at scale. Pre-compute aggregates (maintained on write) or use approximate counting (HyperLogLog) for dashboards and monitoring where exactness is not required.
- **Use a secondary index or search system for non-shard-key lookups.** A lookup by a non-shard-key field (find user by email) requires either a fan-out or a secondary index mapping the field to the shard key. The index itself must be partitioned or replicated; maintaining it adds write cost.

### Plan For Resharding And Rebalancing

As data grows or access patterns change, the initial partitioning may become insufficient (shards too large, distribution uneven, hot spots). Resharding is the answer, and it is costly — plan for it.

- **Choose a partitioning scheme that supports incremental resharding.** Consistent hashing (used by DynamoDB, Cassandra) allows adding shards with minimal data movement (only the keys in the new shard's range move). Hash partitioning with a fixed shard count requires resharding all data when the count changes (expensive). Range partitioning allows splitting or merging ranges incrementally.
- **Reshard without downtime (online resharding).** A resharding that requires downtime is a major operation. Online resharding (dual-writing to old and new shards during migration, or backfilling from old to new with cutover) allows resharding while the system runs. Design for it; use tooling that supports it.
- **Monitor for the need to reshard.** Track shard size, load, and distribution; alert when shards approach capacity or distribution becomes uneven. Reshard proactively, before a shard is overwhelmed.
- **Treat the shard key choice as near-permanent.** Changing the shard key (not just adding shards) requires redistributing all data by the new key — a massive effort. Choose the key carefully; assume it will not change.

## Common Traps

### Shard Key Chosen Without Analyzing Access Patterns

A key that the dominant queries do not filter on, forcing fan-out to all shards for common operations. Choose a key matching the access pattern.

### Monotonically Increasing Key Causing Hot Spots

An auto-increment id or timestamp as a range-partitioned key, concentrating writes on the "current" shard. Hash the key, or range-partition time-series by time with archival.

### Skewed Distribution Creating Uneven Shards

A key with a few dominant values (one huge tenant) creating overloaded shards. Hash the key, or sub-shard the dominant value.

### Cross-Shard Transactions Assumed Available

A transaction touching multiple shards assumed to work like a single-shard transaction, when distributed transactions are slow, complex, or unavailable. Design transactions to be single-shard.

### Uniqueness Constraint Across Shards

A unique constraint on a non-shard-key field (unique email) that cannot be enforced across shards without a secondary index. Make the uniqueness key the shard key, or maintain a global index.

### Premature Partitioning

Partitioning before a single node is insufficient, paying the query and transaction costs before they are justified. Partition when measured need demands it.

### No Plan For Resharding

A fixed shard count with no path to add shards, or a resharding that requires downtime. Choose a resharding-friendly scheme; plan for online resharding.

### Fan-Out Queries Overloading The System

Cross-shard scatter-gather queries that are too parallel or too frequent, overloading the shards. Bound parallelism, paginate, and pre-compute or approximate aggregates.

## Self-Check

- [ ] The shard key is chosen based on the dominant access patterns (common queries filter by the key, enabling single-shard routing), distributes data evenly (hash partitioning for skewed keys, compound keys for flexibility), avoids monotonically-increasing hot spots, and is immutable or near-immutable.
- [ ] The tradeoff partitioning introduces is understood — it scales write throughput, data volume, and shard-key-filtered locality, at the cost of cross-shard queries (fan-out), cross-shard joins (limited/expensive), cross-shard transactions (slow/complex/unavailable), and cross-shard uniqueness (requires the uniqueness key to be the shard key or a global index).
- [ ] Hot spots are avoided: skewed keys are hashed, hot keys in the access pattern are sub-sharded or use distributed counters, time-series hot windows are range-partitioned by time, and shard load is monitored with rebalancing for detected unevenness.
- [ ] Cross-shard queries are designed for deliberately — common queries route to a single shard via the shard key, fan-out uses bounded parallelism with pagination, aggregations are pre-computed or approximate, and non-shard-key lookups use a partitioned or replicated secondary index.
- [ ] Resharding is planned: the scheme supports incremental resharding (consistent hashing or range splitting), online resharding (dual-write or backfill with cutover) avoids downtime, shard size and load are monitored to trigger proactive resharding, and the shard key is treated as near-permanent.
- [ ] Partitioning was not adopted prematurely — a single node's sufficiency was evaluated first, and partitioning was chosen only when measured data volume or write throughput demanded it, accepting the query and transaction constraints as the cost of scale.
- [ ] The system's limitations (what queries are fast, what operations are constrained, what transactions are possible) are documented, so developers do not inadvertently write cross-shard queries or transactions that perform poorly or fail.
- [ ] Secondary indexes (if used) are themselves partitioned or replicated with their write cost understood, global counters (if used) are distributed (not a single hot shard), and the operational tooling exists to monitor distribution, detect hot spots, and perform resharding when needed.
