---
name: distributed_consistency_and_cap.md
description: Use when the agent is designing or reviewing a distributed database, replicated store, microservice data flow, multi-region system, cache-plus-database combination, event-sourced system, or any architecture where data lives on more than one node; choosing a consistency model (strong, linearizable, sequential, causal, eventual, read-your-writes, monotonic reads); reasoning about the CAP theorem and partition behavior; selecting read consistency levels or quorum sizes; deciding between synchronous and asynchronous replication; evaluating distributed transactions (2PC, saga, outbox), consensus (Raft/Paxos), or CQRS; handling split-brain, last-write-wins, conflict resolution, and partition recovery; or deciding how much staleness, unavailability, or inconsistency a given business operation can tolerate. Also covers the cost of strong consistency (latency, availability, throughput), the real meaning of eventual consistency, and the failure mode of assuming a "strongly consistent" database makes the whole system consistent.
---

# Distributed Consistency And CAP

Distributed consistency is the judgment of what guarantees a multi-node system actually makes about the data it returns, and what it gives up to make those guarantees. Every distributed system sits on a tradeoff curve between consistency (every read sees the latest write), availability (every request gets a response), and partition tolerance (the system keeps working when the network splits). You cannot have all three at once under a network partition; you choose which to sacrifice, and that choice is usually made implicitly and wrongly — by picking a database or a replication mode without understanding which side of the tradeoff it lands on.

Agents tend to under-invest here because the single-node mental model works on the happy path: you write, you read, you see the write. The harm appears only under failure — a network partition, a slow replica, a node restart, a cross-region write — and it is severe: double-spending, lost updates, stale reads shown to users as authoritative, broken invariants across services, money or inventory counted twice, and recovery procedures that make things worse. The judgment problem is deciding, for each piece of data and each operation, what consistency the business actually requires, what the chosen system provides, and what happens during the partitions and failures that will eventually occur.

## Core Rules

### Understand What CAP Actually Says — And What It Does Not

CAP is widely misquoted as "choose two of three." The precise statement is narrower and more useful: in the presence of a network partition (which is unavoidable in a real network), the system must choose between consistency (rejecting requests that cannot be answered consistently) and availability (answering requests, possibly with stale or divergent data). Partition tolerance is not optional — partitions will happen — so the real choice is **CP versus AP under partition**.

What this means in practice:

- **CP systems** (many consensus-based stores, e.g., etcd, ZooKeeper, Spanner, strongly-consistent single-leader setups) refuse to serve or accept writes on the minority side of a partition to preserve consistency. During a split, part of the system is unavailable.
- **AP systems** (many Dynamo-style stores, eventually-consistent multi-leader setups) keep accepting reads and writes on both sides of a partition and reconcile afterward. During a split, the system stays up but may return stale or conflicting data.

The crucial refinement: **when there is no partition, you can have both consistency and availability.** CAP governs only the partitioned case. Most modern systems are tunable: they offer consistency levels or modes that let you move along the CP–AP spectrum per operation. Do not classify a system as simply "CP" or "AP" and stop; learn what it does under partition, under partial failure, and under normal operation, because those are three different answers.

### Match The Consistency Guarantee To The Business Invariant

Different data has different tolerance for staleness and inconsistency, and applying one consistency level everywhere is either too expensive (strong everywhere) or too dangerous (eventual everywhere). For each piece of data, name the invariant the business requires:

- **Money, inventory, and account balances.** Usually require strong/linearizable consistency — you cannot allow two concurrent withdrawals to both succeed against a stale balance. The cost (latency, lower availability under partition) is justified by the cost of being wrong.
- **Session state, user preferences, activity feeds.** Often tolerate eventual consistency — a slightly stale "last seen" or a feed entry that appears a second late is harmless, and forcing strong consistency here wastes capacity and hurts availability.
- **Reads after the user's own write.** Often require read-your-writes consistency even when general reads can be stale — a user who updates their profile and immediately reloads must see the update, or they will assume it failed and retry.
- **Cross-aggregate invariants** (e.g., "total debits equal total credits," "no two bookings for the same seat") cannot be enforced by per-item consistency alone; they need transactions, uniqueness constraints, sagas, or explicit coordination.

Write down, for each critical operation, "what must be true immediately, what may be eventually true, and what must never be true." Then choose the consistency level that meets the first and prevents the third.

### Know The Consistency Ladder And Where Your System Sits

Consistency models form a hierarchy from strongest to weakest, and each weaker model permits more anomalies. Know which one your system actually provides, because the name on the box ("ACID," "strong," "tunable") often hides the real guarantee.

- **Linearizability.** Every operation appears to take effect instantaneously at some point between its invocation and response; reads always return the latest write. Strongest, most expensive (requires consensus and waits for quorum or leader).
- **Sequential consistency.** Operations appear in some total order consistent with each client's program order, but not necessarily in real time. Cheaper than linearizable; still strong for many uses.
- **Causal consistency.** Operations that are causally related are seen in order; concurrent operations may be seen differently by different clients. Preserves "happens-before" without global real-time ordering.
- **Read-your-writes.** A client always sees its own writes. Session-level guarantee; often the minimum acceptable for user-facing flows.
- **Monotonic reads.** A client never sees data go backward in time across successive reads. Prevents the jarring "the data reverted" experience.
- **Monotonic writes.** A client's writes are seen in the order it issued them.
- **Eventual consistency.** If no new updates occur, all replicas eventually converge — but with no bound on when, and no guarantee about what is seen in the meantime. The weakest commonly-used model.

"Eventually consistent" does **not** mean "almost as good as strong." It means: reads may return stale data, conflicting concurrent writes may occur, and convergence is eventual and may require conflict resolution. Choose it deliberately where the business tolerates these, not by default.

### Quantify The Cost Of Strong Consistency Before Requiring It

Strong consistency is not free, and requiring it where it is not needed turns a cheap operation into an expensive one. The costs are concrete:

- **Latency.** Linearizable writes require coordination (consensus round-trips, quorum waits, or a leader that may be in another region). A strongly-consistent cross-region write pays the speed-of-light round trip on every write.
- **Availability under partition.** A CP system refuses requests it cannot answer consistently; during a split, the minority side is unavailable. The stronger the consistency, the more failure modes cause unavailability.
- **Throughput.** Coordination serializes work; a single consensus group has a ceiling on writes per second regardless of how many nodes you add. Scaling strong-consistency writes usually requires sharding, which reintroduces cross-shard coordination for multi-key transactions.

Before requiring strong consistency for an operation, ask: is the cost of an occasional stale read or conflicting write genuinely higher than the latency and availability cost of coordination? For financial invariants, yes. For most read-heavy, user-facing content, no. The common error is over-requiring strong consistency because it feels safe, then paying in latency and outages.

### Treat Distributed Transactions As A Last Resort With Known Costs

Transactions that span nodes or services are the most expensive and fragile construct in distributed systems. The options, and their real costs:

- **Two-phase commit (2PC).** Atomic across participants, but blocking: if the coordinator fails mid-commit, participants hold locks and block until recovery. Slow under normal conditions, catastrophic under coordinator failure. Suitable only within a tightly controlled cluster, not across independent services.
- **Distributed consensus per transaction** (Spanner-style). Provides external consistency with TrueTime/clock uncertainty, but pays global-protocol latency on every transaction. Excellent for correctness, expensive in latency and operational complexity.
- **Saga pattern.** A sequence of local transactions with compensating actions on failure. Avoids distributed locking but gives up isolation — intermediate states are visible, and compensation must be designed for every step. Suitable for cross-service workflows where eventual consistency is acceptable and every step has a meaningful undo.
- **Outbox pattern.** Write to the database and publish an event in the same local transaction; a relay publishes the event asynchronously. Solves the dual-write problem (database commit + message publish) without 2PC, at the cost of at-least-once event delivery requiring idempotent consumers.

The general principle: prefer local transactions plus asynchronous coordination (outbox, saga, event-driven reconciliation) over synchronous distributed transactions. Reach for 2PC or global transactions only when the invariant genuinely cannot be maintained any other way and the team can operate the complexity.

### Make Read Consistency Explicit At The Call Site

A common defect is assuming reads are consistent because the database is "consistent," when the read path actually goes through a stale replica, a cache, or a different service's eventually-consistent store. Read consistency is a property of the specific read, not of the database.

For each read, decide and make explicit:

- **What consistency does this read need?** Read-your-writes for post-write confirmation, strong for balance checks before a withdrawal, stale-acceptable for a product catalog.
- **Where does the read go?** A primary/leader read, a quorum read, a local replica, a cache. Each has different freshness guarantees.
- **Is the read after a write consistent with that write?** If the user writes then reads, route both to the same shard/leader, or use a session token / replica-lag-aware routing, or accept the staleness.
- **Is the cache in front of the database consistent?** A cache invalidation race can return stale data after a write; decide whether the cache is authoritative, read-through, write-through, or eventually consistent, and handle invalidation races (see the caching skill).

State the read's consistency requirement in the code or design, and verify the read path actually delivers it. "It usually returns fresh data" is not a consistency guarantee.

### Plan For Partition Recovery, Not Only Partition Survival

Surviving a partition (staying up, or correctly refusing) is half the problem; recovering from it is the other half, and it is where most damage is done. When the partition heals, the system must reconcile divergent state:

- **Last-write-wins (LWW).** Cheap but lossy — concurrent updates to the same key silently drop one. Acceptable for ephemeral data, destructive for anything where updates carry distinct information.
- **Conflict resolution / merge.** Application-aware merging (CRDTs for commutative data types, custom merge functions) preserves both sides but requires the data model to support it.
- **Reconciliation / repair jobs.** Periodic or on-demand scans that detect and repair divergence. Essential as a backstop even when automatic resolution is in place.
- **Split-brain prevention.** A partitioned system that elected two leaders, or accepted writes on both sides, can produce irreconcilable divergence. Fencing tokens, lease-based leadership, and quorum-based elections exist precisely to prevent this; verify they are in place.

Decide, before the partition happens, how divergence will be reconciled and who can intervene. A system with no defined recovery procedure turns every partition into a manual, error-prone incident.

### Distinguish Database Consistency From System Consistency

A strongly-consistent database does not make a distributed system consistent. Once data crosses a service boundary, a cache, an async event, or a client, the end-to-end consistency is only as strong as the weakest link. Common illusions:

- **"We use a strongly-consistent database, so our system is consistent."** Only the data within one database is consistent. Data replicated to a search index, a cache, a read model, or another service is eventually consistent unless explicitly synchronized.
- **"Event sourcing gives us consistency."** Event sourcing gives a consistent log; the read models projected from it are eventually consistent and may lag.
- **"CQRS with a single writer is consistent."** The write side may be consistent; the read side is a separate, eventually-consistent projection.

Map the full data path — write origin through every store, cache, index, projection, and client — and identify the weakest consistency link for each user-visible read. End-to-end consistency is the product of the whole path, not the strongest node.

## Common Traps

### Quoting "Choose Two Of Three" And Stopping

Treating CAP as a fixed label ("we are AP") and assuming that settles the consistency question. Real systems are tunable and context-dependent; the relevant questions are what the system does under partition, under partial failure, and per-operation consistency levels. The slogan obscures the actual tradeoff.

### Assuming Eventual Consistency Is "Almost Strong"

Choosing eventual consistency because it is the default or the cheapest, then being surprised when users see stale data, conflicting updates, or reverted state. Eventual consistency permits real anomalies during convergence; choose it only where the business tolerates those anomalies, and bound the staleness where you can.

### Requiring Strong Consistency Everywhere

Applying linearizable consistency to every read and write because it feels safe, then paying in cross-region latency, reduced throughput, and outages under partition. Match the consistency level to the invariant; most data tolerates weaker consistency, and the savings are large.

### Trusting The Database Label For System-Wide Consistency

Believing a "strongly consistent" or "ACID" database makes the whole system consistent, when data also flows through caches, search indexes, read replicas, other services, and async events. End-to-end consistency is governed by the weakest link in the data path, not the strongest store.

### Distributed Transactions To "Simplify" Cross-Service Updates

Reaching for 2PC or a global transaction across independent services to keep them in sync, then discovering blocking, coordinator-failure outages, and operational fragility. Prefer local transactions plus outbox/saga/eventual coordination; reserve synchronous distributed transactions for cases where no other approach preserves the invariant.

### Last-Write-Wins For Meaningful Updates

Using timestamp-based LWW to resolve conflicts on data where each update carries distinct information, so concurrent edits silently drop one side. LWW is acceptable for ephemeral or overwrite-equivalent data and destructive for anything where lost updates matter.

### Ignoring Read Consistency At The Call Site

Reading from a replica or cache and assuming the value is current, then acting on stale data for a decision that required freshness (e.g., checking a balance before a withdrawal against a lagging replica). Make the read's required consistency explicit and route accordingly.

### No Partition Recovery Plan

Designing the system to survive partitions but never defining how divergent state is reconciled when the partition heals, so recovery becomes a manual, error-prone incident. Define conflict resolution, reconciliation jobs, and split-brain prevention before the partition occurs.

### Treating Clocks As Authoritative For Ordering

Using wall-clock timestamps for ordering or conflict resolution across nodes, ignoring clock skew, drift, leap seconds, and NTP inaccuracy. Clock skew across nodes can reverse the apparent order of events; use logical clocks, vector clocks, version vectors, hybrid clocks, or consensus-based ordering where ordering correctness matters.

### Overgeneralizing From A Single Workload

Assuming the consistency choice that worked for one service or dataset applies to all. A read-heavy catalog tolerates eventual consistency; a write-heavy ledger does not. Decide consistency per invariant, not per system.

## Self-Check

- [ ] For each critical operation, the required invariant is explicit: what must be true immediately, what may be eventually true, and what must never be true — and the chosen consistency model meets the first and prevents the third.
- [ ] The system's behavior under network partition is known (CP: which side refuses; AP: how divergence is reconciled), and the choice was made deliberately per data category, not inherited from a database default.
- [ ] The consistency model the system actually provides is identified (linearizable, sequential, causal, read-your-writes, monotonic, eventual), and the team understands "eventual" permits stale reads, conflicts, and unbounded convergence time.
- [ ] Strong consistency is required only where the business invariant justifies the latency, throughput, and partition-availability cost; weaker models are used where staleness is tolerable.
- [ ] Cross-node/cross-service coordination uses the right tool (local transactions + outbox/saga/eventual by default; 2PC or global transactions only where unavoidable), and the costs of each are understood.
- [ ] Each read states its required consistency and is routed to a path that delivers it (primary/quorum/session-aware for fresh reads; replica/cache only where stale is acceptable); post-write reads are consistent with the write.
- [ ] Partition recovery is designed before it is needed: conflict resolution (LWW only where lossy updates are acceptable, CRDTs or custom merge otherwise), reconciliation/repair jobs, and split-brain prevention (fencing, leases, quorum elections) are in place.
- [ ] End-to-end consistency was mapped across the full data path (database, cache, search index, read models, other services, client), and the weakest link for each user-visible read was identified and accepted or strengthened.
- [ ] Ordering correctness does not rely on wall-clock timestamps alone; logical/vector/hybrid clocks or consensus ordering is used where ordering matters, and clock skew assumptions were challenged.
- [ ] The consistency decisions were made per invariant/workload, not generalized from a single system or database label, and the highest-risk cases (money/inventory balances, read-after-write, cross-service invariants, partition recovery) were specifically verified.
