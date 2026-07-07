---
name: data_replication_and_consistency.md
description: Use when the agent is designing or operating a replicated data system — primary/replica, multi-primary, quorum-based, or consensus-based replication; choosing a consistency model (strong, eventual, causal, read-your-writes); handling replication lag and stale reads; deciding between synchronous and asynchronous replication; managing failover and split-brain; or diagnosing consistency anomalies (reading stale data, divergent replicas, lost updates). Covers replication topologies, consistency models and the CAP/PACELC tradeoff, replication lag and its consequences, synchronous vs asynchronous tradeoffs, quorum and consensus, and the design discipline of matching the consistency model to the application's requirements.
---

# Data Replication And Consistency

Replication copies data across multiple nodes for availability, durability, and read scalability, and its central tension is consistency: when data is copied, the copies can diverge, and the question of what a reader sees after a write — and when — is governed by the consistency model. The spectrum runs from strong consistency (a read always returns the latest write, at the cost of coordination that limits availability during partitions) to eventual consistency (reads may return stale data for a while, in exchange for availability and performance). This is the CAP tradeoff: during a network partition, a replicated system must choose consistency (refuse operations that cannot be synchronized) or availability (serve from the partitioned nodes, risking divergence). Most systems land somewhere in between, with consistency models (read-your-writes, causal, bounded staleness) that offer specific guarantees. The choice is not "strong is better"; it is a match between the consistency the application requires and the cost (availability, latency, complexity) the consistency level imposes. A system that needs stronger consistency than it has produces anomalies (lost updates, stale reads, violated invariants); a system with stronger consistency than it needs pays unnecessary availability and latency costs.

Agents tend to assume replication provides strong consistency (it often does not), to ignore replication lag (leading to stale-read bugs), and to choose a consistency level without understanding its availability or latency implications. The judgment problem is recognizing that consistency is a spectrum with specific models and tradeoffs, that replication lag is a real phenomenon with application-visible consequences, and that the consistency model must be matched to what the application's correctness requires. This skill covers the discipline of data replication and consistency: topologies, consistency models and tradeoffs, replication lag, synchronous vs asynchronous, quorum and consensus, and failover.

## Core Rules

### Match The Consistency Model To The Application's Requirements

Different operations need different consistency. Matching the model to the requirement avoids both anomalies (too weak) and unnecessary cost (too strong).

- **Identify what consistency each operation requires.** A financial transaction needs strong consistency (no lost updates, no double-spending); a social feed can tolerate eventual consistency (a slight delay before a post appears is acceptable). Not all data needs the strongest consistency; not all data can tolerate the weakest.
- **Strong consistency (linearizability): a read returns the latest write; operations appear in a single global order.** Required for data where reading stale or divergent values causes incorrectness (balances, inventory, identity). Achieved via synchronous replication, quorum, or consensus; costs availability during partitions (CAP) and latency (coordination).
- **Eventual consistency: replicas converge eventually, but reads may return stale data in the meantime.** Acceptable for data where temporary inconsistency is tolerable (a like count, a feed, a cache). Offers availability and performance; risks stale reads, divergent intermediate states, and conflict.
- **Intermediate models: read-your-writes (a user sees their own writes), causal (preserves causality, no reversed effects), bounded staleness (stale by at most a time delta).** These offer specific guarantees between strong and eventual, often matching application needs (a user should see their own post immediately) without full strong consistency's cost.
- **Choose the weakest consistency that satisfies correctness.** Stronger consistency than needed wastes availability and latency; weaker consistency than needed causes anomalies. Identify the requirement per operation, not globally.

### Understand The Replication Topology And Its Tradeoffs

How data is replicated — primary/replica, multi-primary, quorum, consensus — determines the consistency, availability, and conflict characteristics.

- **Primary/replica (single-primary, leader-follower): writes go to the primary, replicated to replicas.** Strong consistency for writes (the primary is the single source of truth); reads can be strong (from the primary) or eventual (from replicas, possibly stale). Failover promotes a replica; split-brain risk if two nodes believe they are primary. The common pattern for relational databases.
- **Multi-primary (multi-leader): writes accepted at multiple nodes, replicated between them.** Higher write availability (no single primary) and geographic locality (write locally); requires conflict resolution (concurrent writes to the same data diverge) and has weaker consistency. Suited to offline-capable or geographically distributed systems; complex to get right.
- **Quorum-based (NWR): writes and reads contact a quorum (majority) of nodes; consistency tunable via the quorum sizes.** With N replicas, W write quorum, R read quorum: if W + R > N, reads and writes overlap, giving strong consistency; if W + R <= N, eventual consistency with higher availability/performance. Dynamo-style systems (Cassandra, Riak, DynamoDB) use this.
- **Consensus-based (Raft, Paxos): a consensus protocol ensures all nodes agree on the order of operations.** Strong consistency with availability (tolerating minority failures); the foundation of systems like etcd, Consul, Spanner. Higher latency (the consensus round-trip) and complexity.

### Account For Replication Lag And Its Consequences

Asynchronous replication has lag — the replica is behind the primary by some amount. This lag has application-visible consequences.

- **Replication lag means reads from replicas may return stale data.** A user writes, then reads from a replica that has not received the write, and sees stale data. This is the read-your-writes violation, a common consistency bug.
- **Bound the lag's impact for read-your-writes consistency.** To ensure a user sees their own writes, route their subsequent reads to the primary (strong, but loads the primary), or to a replica confirmed to have caught up (via a replication timestamp or session token). Do not blindly read from any replica after a write.
- **Monitor replication lag.** Lag that grows unbounded indicates the replica cannot keep up (insufficient capacity, network issues). Alert on lag exceeding a threshold; a lagging replica serves increasingly stale data.
- **Design for lag during failover.** When a primary fails and a replica is promoted, the replica's lag represents potential data loss (writes on the primary not yet replicated). This is the RPO for replication-based availability; understand and bound it.

### Choose Synchronous Or Asynchronous Replication Deliberately

Whether replication is synchronous (the write waits for replicas to confirm before acknowledging) or asynchronous (the write acknowledges immediately, replication happens after) determines consistency vs latency/availability.

- **Synchronous replication: strong consistency, higher latency, availability cost during partitions.** The write is durable on replicas before the client is told it succeeded, so reads (from primary or caught-up replicas) are fresh. But the write waits for replicas (latency), and if a replica is unreachable (partition), the write may block or fail (availability cost). Used for data requiring strong durability and consistency.
- **Asynchronous replication: eventual consistency, lower latency, available during partitions.** The write acknowledges immediately, replication happens after, so the replica may be stale (lag). The write does not wait for replicas (low latency) and succeeds even if a replica is partitioned (available). Used for performance and geographic replication where eventual consistency suffices.
- **Mix: synchronous to a local quorum, asynchronous to remote regions.** A common pattern: synchronous replication within a region (strong consistency locally, tolerating local failures), asynchronous replication to remote regions (eventual consistency globally, tolerating network issues). Balances consistency and geographic availability.
- **Do not assume asynchronous replication provides durability.** A write acknowledged under asynchronous replication may be lost if the primary fails before replicating. If durability is required, use synchronous replication or a quorum.

### Handle Failover And Prevent Split-Brain

When a primary fails, a replica must be promoted (failover). The risk is split-brain: two nodes believing they are primary, accepting divergent writes.

- **Use fencing or consensus to ensure a single primary.** Failover must guarantee only one primary. Quorum-based failover (a majority of nodes agree on the new primary) or consensus (Raft leader election) prevents split-brain. Avoid failover mechanisms that can elect two primaries under partition.
- **Account for data loss on failover with asynchronous replication.** If replication was asynchronous, the promoted replica may be behind the failed primary, losing recent writes. Quantify this (the RPO); decide if it is acceptable or if synchronous replication is needed.
- **Plan the failover process: detection, promotion, traffic shift.** Detect the primary failure (health checks, quorum), promote a replica (with the consistency guarantees above), shift traffic (DNS, load balancer — see disaster-recovery-and-backup). Practice the failover; an untested failover fails.
- **Handle the failed primary's recovery (divergence reconciliation).** If the old primary accepted writes not yet replicated, those writes must be reconciled when it recovers (replay, discard, or manual repair). Plan for this; a naive "old primary rejoins as replica" can corrupt data if its unreplicated writes conflict with the new primary's state.

## Common Traps

### Assuming Replication Provides Strong Consistency

Treating a replica as fresh when replication is asynchronous, causing stale-read bugs. Understand the consistency model; route read-your-writes reads appropriately.

### Ignoring Replication Lag

Reading from a lagging replica without bounding the staleness, returning data far behind the primary. Monitor lag; bound it for read-your-writes consistency.

### Consistency Stronger Than Needed

Using strong consistency (synchronous replication, quorum) for data that tolerates eventual consistency, paying unnecessary latency and availability costs. Match the model to the requirement.

### Consistency Weaker Than Needed

Using eventual consistency for data that requires strong (financial balances, inventory), causing lost updates or invariant violations. Identify the requirement; choose accordingly.

### Split-Brain During Failover

A failover that elects two primaries under partition, causing divergent writes. Use fencing, quorum, or consensus to ensure a single primary.

### Unreplicated Writes Lost On Failover

Asynchronous replication with a failed primary, losing recent writes (RPO exceeded). Quantify the RPO; use synchronous replication if the loss is unacceptable.

### Conflict Resolution Done Wrong In Multi-Primary

Concurrent writes to the same data in a multi-primary system, resolved naively (last-write-wins losing data) rather than correctly (application-aware merge, CRDTs). Design the conflict resolution; do not assume LWW suffices.

### Failover Never Tested

A failover plan that has not been exercised, failing when invoked (promotion broken, traffic shift broken, divergence unhandled). Test failover regularly.

## Self-Check

- [ ] The consistency model (strong, eventual, read-your-writes, causal, bounded staleness) is matched to each operation's correctness requirement — the weakest consistency that satisfies correctness, identified per operation rather than globally — and the application does not assume stronger consistency than the system provides.
- [ ] The replication topology (primary/replica, multi-primary, quorum, consensus) is understood, with its consistency, availability, and conflict characteristics, and chosen to match the system's needs (strong consistency via primary/replica or consensus; geographic availability via multi-primary; tunable consistency via quorum).
- [ ] Replication lag is accounted for: read-your-writes reads are routed to the primary or a confirmed-caught-up replica, lag is monitored and alerted on, and the lag's impact on failover (potential data loss, RPO) is quantified and bounded.
- [ ] Synchronous vs asynchronous replication is chosen deliberately — synchronous for strong durability/consistency (accepting latency and partition cost), asynchronous for performance/availability (accepting eventual consistency and potential loss), or a mix (local quorum synchronous, remote asynchronous) — matching the data's requirements.
- [ ] Failover is designed to prevent split-brain (quorum or consensus ensuring a single primary), accounts for data loss under asynchronous replication (RPO quantified), includes a practiced process (detection, promotion, traffic shift), and handles the failed primary's recovery (divergence reconciliation).
- [ ] Conflict resolution in multi-primary systems is designed correctly (application-aware merge, CRDTs, or version vectors), not naively (last-write-wins losing data), and the application handles the conflicts the consistency model can produce.
- [ ] The CAP/PACELC tradeoff is understood: during a partition (P), the system chooses consistency (C) or availability (A); else (E), it chooses lower latency (L) or consistency (C) — and the choice is documented and matched to the application's priorities.
- [ ] The system has been tested for consistency anomalies (reading stale data after a write, divergent replicas after a partition, lost updates under concurrent writes) to verify the consistency model holds in practice, not just in theory.
