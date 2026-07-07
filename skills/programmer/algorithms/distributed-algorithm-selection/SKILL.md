---
name: distributed_algorithm_selection.md
description: Use when the agent is choosing or implementing algorithms for distributed systems — leader election, consensus (Raft, Paxos), distributed locking, distributed queues, service discovery, gossip protocols, distributed transactions (2PC, 3PC, Saga), or clock/synchronization (logical clocks, vector clocks, TrueTime); deciding between strong and eventual consistency for a distributed computation; or reasoning about the FLP impossibility, partial failure, and network asynchrony. Covers consensus and the CAP/FLP constraints, leader election, distributed locking and its pitfalls, gossip and membership, distributed transactions and their costs, logical and physical clock synchronization, and the judgment of when to use a vetted library versus building a distributed protocol.
---

# Distributed Algorithm Selection

Distributed algorithms operate across multiple nodes that communicate over a network, and their defining challenge is partial failure: in a distributed system, some components fail while others continue, the network delays or partitions messages, and there is no global clock to order events. An algorithm correct on a single node is often incorrect distributed, because it assumes synchronous operation, reliable communication, and consistent state — none of which hold. The fundamental results are unforgiving: the FLP impossibility proves that no asynchronous algorithm can guarantee both safety and liveness under crash failures (consensus cannot be solved in a purely asynchronous network with even one faulty node); the CAP theorem proves that during a partition, a system cannot guarantee both consistency and availability. These are not limitations to engineer around; they are constraints that shape every distributed algorithm, requiring the designer to choose which guarantees to provide and which to relax, and to understand that the choice has consequences for correctness, availability, and performance.

Agents tend to apply single-node intuitions to distributed problems (assuming operations are atomic across nodes, that a response means success, that clocks are synchronized), to build distributed protocols (locking, consensus, transactions) rather than using vetted implementations, and to choose consistency models without understanding their availability or latency costs. The judgment problem is recognizing that distributed algorithms are governed by impossibility results that do not yield to cleverness, that partial failure and asynchrony are the norm, and that the vast majority of needs are met by proven, vetted systems (etcd, ZooKeeper, Spanner, Kafka) rather than hand-rolled protocols. This skill covers the discipline of distributed algorithm selection: consensus and its constraints, leader election, distributed locking, gossip, distributed transactions, clock synchronization, and the judgment of build versus use.

## Core Rules

### Respect The Impossibility Results (FLP, CAP)

Distributed algorithms operate within constraints proven by FLP and CAP. Understanding these constraints prevents attempting the impossible.

- **FLP impossibility: consensus cannot be guaranteed in an asynchronous network with even one crash failure.** In a purely asynchronous network (no bound on message delay), no algorithm can guarantee both that consensus terminates (liveness) and that it never agrees on a wrong value (safety), if even one node may crash. Practical consensus algorithms (Raft, Paxos) work around this by adding timing assumptions (eventual synchrony, leader election with timeouts) that hold in practice but are not guaranteed.
- **CAP theorem: during a partition, choose consistency or availability.** A replicated data system cannot simultaneously guarantee consistency (all nodes see the same data), availability (every request gets a response), and partition tolerance (the system operates despite network partitions). Since partitions happen, the real choice is CP (consistency, refusing operations during partition) or AP (availability, allowing divergence during partition). Match the choice to the data's requirements.
- **PACELC extends CAP to the no-partition case.** Even without a partition (E), the system chooses lower latency (L) or consistency (C): a system can be fast (EL) or consistent (EC) when there is no partition. This captures the latency-vs-consistency tradeoff that CAP omits.
- **Do not attempt to circumvent these results with cleverness.** A "novel" consensus algorithm that claims to solve FLP is almost certainly wrong. Use proven algorithms; understand their assumptions.

### Choose Consensus And Replication Based On Requirements

Consensus (getting nodes to agree) and replication (copying state) are the foundations of consistent distributed systems. Choose the approach based on the consistency and availability requirements.

- **Strong consistency requires consensus (Raft, Paxos, Zab).** To guarantee that all nodes agree on a single order of operations (linearizability), use a consensus protocol. Raft (understandable, used by etcd, Consul) and Paxos (foundational, used by Spanner, Chubby) are the standard. These provide strong consistency with availability tolerating minority failures.
- **Quorum-based systems (NWR) offer tunable consistency.** Dynamo-style systems (Cassandra, Riak, DynamoDB) use read and write quorums: with N replicas, requiring W writers and R readers where W+R>N ensures overlap (strong consistency); relaxing W or R allows higher availability/performance at the cost of eventual consistency. Tune the quorum to the consistency need.
- **Eventual consistency avoids consensus for availability.** Systems that only guarantee convergence (eventual consistency) can be available during partitions and faster (no consensus round-trip), at the cost of intermediate inconsistency. Use for data tolerating it (feeds, caches, counters); avoid for data requiring strong consistency.
- **Choose the consistency level per operation, not globally.** A system may offer strong consistency for some operations (via consensus or quorum) and eventual for others. Match each operation's consistency to its requirement.

### Handle Leader Election And Membership Carefully

Many distributed systems use a leader (a single coordinator) to simplify operation, and must elect and track leaders and members correctly.

- **Leader election must handle split-brain and split votes.** Two nodes believing they are leader (split-brain) causes divergent state; a split vote (no majority) stalls the election. Use a consensus-based election (Raft, which uses terms and majority votes) or a vetted leader-election library; avoid ad-hoc election that can elect two leaders.
- **Account for the leader as a bottleneck and single point of failure.** A leader processes all writes (in strong-consistency systems), concentrating load and becoming a failure point the system must recover from (re-election). Design for the leader's capacity and the re-election time.
- **Track membership (which nodes are in the system) consistently.** A distributed system must know its membership (for quorums, replication, routing). Gossip protocols (eventual membership) and consensus-based membership (strong) are the approaches; choose based on the consistency need.
- **Use fencing tokens to prevent stale leaders.** A leader that is partitioned and recovers may believe it is still leader, issuing commands that conflict with the new leader. Fencing tokens (a monotonically increasing token per leader) ensure the system rejects the stale leader's commands.

### Be Cautious With Distributed Locking

Distributed locks are used for mutual exclusion across nodes, and they are subtler than local locks, with failure modes that can cause incorrectness.

- **Distributed locks do not provide the same guarantees as local locks.** A local lock guarantees mutual exclusion within a process; a distributed lock, held across a network, can be violated by clock skew, process pauses (GC), or lease expiry — a holder may believe it holds the lock after the lease expired, causing two holders.
- **Use leases with fencing tokens, not indefinite locks.** A distributed lock should be a lease (expires after a timeout, so a crashed holder's lock is released) with a fencing token (so a stale holder's writes are rejected). See deadlock-prevention-and-detection.
- **Do not use distributed locks for correctness that requires consensus.** If correctness depends on mutual exclusion (only one node at a time), and a lock violation causes incorrectness, the lock is insufficient — use consensus or a transaction. Distributed locks are for efficiency (avoiding duplicate work), not correctness, unless carefully designed with fencing.
- **Prefer vetted locking systems (etcd, ZooKeeper, Redis Redlock with caveats).** Building a distributed lock is error-prone; use a vetted implementation. Understand its guarantees (etcd/ZK provide fencing via leases; Redlock has known limitations around clock assumptions).

### Choose Distributed Transactions By Their Cost

Distributed transactions (spanning multiple nodes or shards) provide atomicity across nodes, but at high cost. Choose based on whether the atomicity is worth the cost.

- **2PC (two-phase commit): atomic but blocking and fragile.** 2PC ensures all participants commit or all abort, but the coordinator is a single point of failure (if it crashes mid-commit, participants block), and it is slow (two round trips). Use for cases requiring atomicity across nodes where the blocking risk is acceptable.
- **3PC: non-blocking but more rounds and assumes bounded delays.** 3PC avoids 2PC's blocking by adding a phase, but assumes bounded message delays (which do not hold in asynchronous networks), so it is not a complete solution. Rarely used in practice.
- **Saga pattern: eventual consistency via compensating transactions.** A saga breaks a distributed transaction into a sequence of local transactions, each with a compensating action (undo) on failure. It provides eventual consistency (not atomicity) without 2PC's blocking. Use for long-running or cross-service workflows where atomicity is not required and compensation is feasible.
- **Do not assume distributed transactions are cheap.** 2PC's latency and blocking, or a saga's complexity, are significant costs. Prefer designing the system so transactions are local (single shard — see data-partitioning-and-sharding) and avoid distributed transactions where possible.

### Synchronize Clocks Appropriately (Or Avoid Depending On Them)

Distributed systems need to order events, and physical clocks are unreliable for this. Understand clock synchronization and its limitations.

- **Physical clocks are not perfectly synchronized.** Clocks on different nodes drift; NTP keeps them close but not identical; a timestamp from one node is not directly comparable to another's. Do not use physical timestamps to order events across nodes without accounting for skew.
- **Logical clocks (Lamport) and vector clocks order events by causality.** Lamport timestamps assign a logical time that respects causality (if A causes B, A's time < B's time); vector clocks detect concurrency (whether two events are causally ordered or concurrent). Use these for ordering where physical clocks are insufficient.
- **TrueTime (Spanner) uses bounded clock uncertainty for external consistency.** Spanner's TrueTime API returns a time interval (the clock is within [earliest, latest]) and waits out the uncertainty to guarantee external consistency. This requires specialized hardware (atomic clocks, GPS); not available in general systems.
- **Prefer designing without dependence on synchronized clocks.** Use consensus to order events (the log of operations defines the order) rather than timestamps. Where timestamps are needed (last-write-wins, TTLs), account for skew with margins.

### Prefer Vetted Systems Over Hand-Rolled Protocols

Distributed algorithms are exceptionally hard to get right, with subtle bugs that survive years of review. The strong default is to use vetted systems.

- **Use proven consensus systems (etcd, ZooKeeper, Spanner, FoundationDB).** These implement consensus, replication, and locking correctly, with the fencing, leader election, and failure handling that hand-rolled versions miss. Build on them rather than reimplementing.
- **Use proven messaging systems (Kafka, RabbitMQ, NATS) for distributed queues and logs.** A distributed log (Kafka) provides ordering and durability; use it rather than building a custom protocol.
- **Build a custom distributed protocol only when vetted systems do not fit, and with extreme rigor.** A custom consensus or locking protocol is a research-grade undertaking; subject it to formal verification, extensive testing (including network partitions, clock skew, process pauses), and review by experts.
- **Do not treat "distributed" as a modifier that makes single-node algorithms work.** A single-node algorithm applied distributed, without addressing partial failure and asynchrony, is almost certainly wrong.

## Common Traps

### Single-Node Intuitions Applied Distributed

Assuming operations are atomic across nodes, responses mean success, or clocks are synchronized — none of which hold distributed. Design for partial failure and asynchrony.

### Attempting To Circumvent FLP Or CAP

A novel algorithm claiming to solve consensus in asynchronous networks or provide consistency and availability during partitions. These are proven impossible; use proven algorithms with their assumptions.

### Hand-Rolled Distributed Lock Or Consensus

A custom distributed lock or consensus protocol missing the fencing, leader election, or failure handling that makes it correct. Use vetted systems.

### Distributed Lock Used For Correctness

Relying on a distributed lock for correctness that requires consensus, when the lock can be violated by clock skew or lease expiry. Use locks for efficiency, consensus for correctness.

### 2PC Used Where Saga Or Local Transactions Suffice

2PC's blocking and latency used for a workflow that tolerates eventual consistency or could be local transactions. Match the transaction model to the requirement.

### Depending On Synchronized Physical Clocks

Using physical timestamps to order events across nodes, ignoring clock skew. Use logical clocks or consensus ordering.

### Ignoring Partial Failure And Network Partitions

An algorithm that assumes all nodes are up and the network is reliable, failing when nodes crash or the network partitions. Test under failure; design for recovery.

### Eventual Consistency For Data Requiring Strong

Using eventual consistency for data that requires strong (financial, identity), causing lost updates or invariant violations. Match the consistency model to the requirement.

## Self-Check

- [ ] The impossibility results (FLP — consensus cannot be guaranteed in asynchronous networks with crash failures; CAP — consistency or availability during partitions; PACELC — latency or consistency when no partition) are understood, and the system does not attempt to circumvent them — it uses proven algorithms with their assumptions, and chooses consistency vs availability per the data's requirements.
- [ ] Consensus and replication are chosen based on requirements — strong consistency via consensus (Raft, Paxos) or quorum (NWR with W+R>N), eventual consistency for availability, with the consistency level matched per operation.
- [ ] Leader election handles split-brain and split votes (consensus-based, with terms and majority), the leader's bottleneck and failure-recovery are designed for, membership is tracked consistently (gossip or consensus), and fencing tokens prevent stale leaders.
- [ ] Distributed locking uses leases with fencing tokens (not indefinite locks), is not relied upon for correctness that requires consensus, and prefers vetted systems (etcd, ZooKeeper) over hand-rolled implementations.
- [ ] Distributed transactions are chosen by cost — 2PC for atomicity where blocking is acceptable, Saga for eventual consistency with compensation, local transactions preferred by partitioning to avoid distributed transactions where possible.
- [ ] Clock synchronization is handled appropriately — logical clocks (Lamport, vector) for causal ordering, TrueTime where available, physical timestamps only with skew margins, and consensus ordering preferred over timestamp dependence.
- [ ] Vetted systems (etcd, ZooKeeper, Spanner, Kafka) are used for consensus, replication, locking, and messaging, with custom distributed protocols built only when vetted systems do not fit and with extreme rigor (formal verification, failure testing, expert review).
- [ ] The system has been tested under the distributed failure modes — network partitions, node crashes, clock skew, process pauses (GC), and message delay/reordering — to verify the algorithm's correctness holds under partial failure and asynchrony, not just in the happy path.
