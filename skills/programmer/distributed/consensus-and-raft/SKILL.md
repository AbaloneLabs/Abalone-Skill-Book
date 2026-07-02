---
name: consensus_and_raft.md
description: Use when the agent is building or operating a consensus-based system (Raft, Paxos, Multi-Paxos), designing a replicated state machine, reasoning about leader election and log replication, choosing quorum and cluster sizes for fault tolerance, handling split-brain and stale-leader problems with fencing or leader leases, performing membership or configuration changes on a live cluster, recovering from quorum loss, or deciding whether to use a managed consensus service (etcd, Consul, ZooKeeper) versus building one. Covers the algorithm mechanics and operational reality, distinct from the consistency-model overview.
---

# Consensus And Raft

Consensus is the problem of getting independent nodes to agree on a single value, a log entry order, or a leader, despite failures, message loss, reordering, and partitions. A replicated state machine built on consensus applies the same ordered sequence of commands to deterministic replicas so they converge on identical state — this is the foundation of systems like etcd, Consul, ZooKeeper, and the replication layer of many databases. The judgment problem is not "which consensus algorithm is best" but "what does consensus actually guarantee, what does it cost in latency and availability, what are the operational hazards of running a quorum-based cluster, and — critically — whether you should be running your own consensus at all instead of a managed service."

Agents tend to under-invest here because the abstraction is clean: a leader replicates a log, a majority commits, the system is consistent. The harm appears in the operational reality the abstraction hides — a partitioned minority that must refuse all writes, a stale leader that briefly believes it is still in charge, a cluster that has permanently lost quorum and cannot recover without manual and dangerous intervention, a botched membership change that splits the cluster into two disjoint majorities and silently loses data, and the temptation to "fix" an unavailable consensus cluster with a step that violates the safety invariant. For the consistency-model theory and when consensus is the right tool, see distributed-consistency-and-cap; this skill is the algorithm mechanics and the operational reality of running a consensus cluster.

## Core Rules

### Understand What Consensus Solves And What It Costs

Consensus solves agreement under failure: a set of nodes agree on a single outcome (a value, a log order, a leader) that is consistent despite crashes, reboots, message loss, and network partitions, provided a majority of nodes can communicate. The guarantees are strong — once a value is decided it is never changed, and a committed log entry is never lost as long as a majority survives — but the costs are concrete and must be accepted deliberately. Every decision requires a majority round trip, so write latency is bounded below by the network round-trip time to a quorum. Throughput of a single consensus group is capped regardless of node count, because a single leader serializes the log. And availability is quorum-dependent: if a majority cannot communicate, the system stops accepting writes to preserve consistency.

A weak understanding treats consensus as "replication that always works." A strong understanding knows that consensus buys linearizable consistency and split-brain-free operation at the price of quorum latency, single-leader throughput ceilings, and mandatory unavailability when a quorum is lost. Decide whether the business invariant genuinely needs these guarantees before paying for them; many workloads are better served by cheaper eventually-consistent replication. See distributed-consistency-and-cap for the consistency-model decision; here, assume the decision to use consensus has been made and focus on doing it correctly.

### Know Raft Mechanics: Terms, Leader Election, And Randomized Timeouts

Raft is the most commonly implemented consensus algorithm because it is designed for understandability, and understanding its mechanics is essential to operating it. Time is divided into **terms** (monotonically increasing integers); each term has at most one leader. Nodes are followers, candidates, or leaders. A follower that does not hear from a leader within a randomized **election timeout** becomes a candidate, increments the term, votes for itself, and requests votes from other nodes. If it wins a majority of votes for that term, it becomes leader. The randomized election timeouts are critical: they make it overwhelmingly likely that one node times out first and wins before others start competing, avoiding repeated split votes. Election timeout range must be set well above expected network round-trip times, or the cluster will churn through leaders.

A weak deployment uses default timeouts without measuring the network, then suffers constant leader changes under latency jitter. A strong deployment measures round-trip times across the deployment (especially cross-region) and sets election timeouts conservatively above them, accepting that a slower cluster is more stable than one that flips leaders constantly. Understand that leader election itself is a consensus decision — once a node wins a term, no other node can lead that term, and this is what prevents split brain at the leadership level.

### Know Raft Log Replication, Commit, And The Safety Properties

Once a leader exists, it replicates a log of commands: the leader appends the command to its log, sends `AppendEntries` (with preceding entries for consistency) to followers, and once a majority have persisted the entry, the leader **commits** it (marks it and all prior entries committed) and applies it to its state machine. The leader then notifies followers of the commit index in subsequent messages so they can apply it too. The core safety property is that if an entry is committed, it is present in the log of any future leader — this is enforced by the majority-quorum rule and the restriction that a leader never overwrites or deletes its own log entries.

The subtle correctness points an operator must know: a leader only commits entries from its current term directly; entries from older terms are committed implicitly when a later-term entry on the same leader is committed. This avoids a subtle bug where a leader could commit an old entry that a future leader lacks. Followers that are behind catch up by the leader sending earlier log entries, and conflicting entries on a follower are overwritten by the leader's entries — the leader's log is authoritative. If you observe a cluster where followers' logs diverge in ways the leader does not reconcile, something is wrong with the replication path. Also understand the difference between **persisted** (on disk, durable) and **committed** (majority-replicated and safe to apply) — applying an uncommitted entry, or crashing before persisting, are both real failure modes the implementation must handle.

### Prevent Split Brain With Fencing And Leader Leases

Raft guarantees a single leader per term and that a committed entry survives, but it does not by itself prevent a **stale leader** from serving clients briefly after it has been deposed. Consider: a leader is partitioned from the majority; the majority elects a new leader; the old leader does not yet know it is deposed and may still accept requests from clients that can reach it. Within Raft this is bounded — the old leader cannot commit new entries because it lacks a quorum — but for client-facing operations and especially for external side effects, a stale leader that returns "success" or acts on outdated belief of leadership is a real hazard.

The robust fix is **fencing tokens** (monotonic tokens, often the term number or a lease epoch) attached to every leader action, so that protected resources and external systems reject operations carrying a stale token. This requires the protected resource to check the token, which works when you control the resource (a storage system with fencing support) and is impossible when you do not (an external API). An alternative is **leader leases**: a leader holds a lease renewed by a quorum and only acts while the lease is valid, bounding the stale-leader window to the lease duration — but leases depend on clocks and must be sized conservatively. A weak design assumes Raft's single-leader-per-term guarantee is enough and lets a stale leader serve external side effects. A strong design adds fencing or leases for any action where a stale leader could cause harm, and never relies on the leader's own belief that it is in charge.

### Change Membership And Quorum With Extreme Care

Adding or removing nodes from a consensus cluster changes who constitutes a majority, and doing it wrong is one of the few ways to silently lose data in an otherwise-correct system. The danger is **two disjoint majorities**: if you switch directly from an old configuration to a new one without coordination, it is possible for the old-config majority and the new-config majority to disagree and elect different leaders, producing split brain. Raft solves this with **joint consensus** (also called single-membership changes done one node at a time): the cluster transitions through an intermediate state where decisions require a majority of both the old and new configurations, so no two disjoint majorities can form.

The operational rules that follow from this: change membership one node at a time, waiting for each change to be committed and stable before the next; never bulk-edit the cluster configuration; ensure the new node has caught up the log before it is counted in the quorum (otherwise adding it can reduce effective fault tolerance or stall the cluster); and have a documented rollback if a change goes wrong. A weak operation edits the membership of multiple nodes simultaneously or restarts nodes mid-change and ends up with a cluster that cannot agree. A strong operation treats membership changes as a careful, sequenced procedure with verification at each step. For removing the leader specifically, know that some implementations require special handling — plan it rather than discovering the behavior during an incident.

### Expect The Minority Side Of A Partition To Serve Nothing

Under a network partition, a Raft cluster splits into a majority side and a minority side. The majority side elects (or keeps) a leader and continues to serve. The minority side cannot form a quorum, cannot elect a leader, and must refuse all writes and (for linearizable reads) all reads — serving anything from the minority would risk returning stale or divergent data, violating the consistency guarantee the system exists to provide. This is correct behavior, not a bug, but it surprises operators who expected "high availability" and find part of the cluster dark.

Design around this deliberately. A 3-node cluster tolerates 1 failure and stays available; a 5-node cluster tolerates 2; a 7-node cluster tolerates 3. The odd number matters because it avoids even-split ambiguity. If your availability requirement is "survive a single node failure," 3 nodes suffice; if it is "survive a rack or zone failure plus a node," you need 5. Do not run a 2-node consensus cluster for fault tolerance — it tolerates zero failures for availability (either node down loses quorum) and adds the consensus overhead for no benefit. Also plan for the case where the partition is not symmetric: a node may be alive but isolated, and it must not serve; verify your reads are quorum-checked or leader-leased, not served from any node that happens to be up.

### Almost Always Use A Managed Consensus System

The single most important operational rule in this area: unless you have a compelling, well-justified reason, do not build your own consensus implementation. Consensus algorithms are notoriously easy to implement incorrectly in subtle ways that pass tests and corrupt data in production — the literature is full of implementations with fencing bugs, log-recovery bugs, and membership-change bugs. Managed systems (etcd, Consul, ZooKeeper, and the consensus layers of managed databases like Spanner, DynamoDB strong-consistency, or cloud-native key-value stores) are battle-tested, operated by experts, and handle the operational hazards for you.

A weak choice is "we will implement Raft over the weekend" for a system whose correctness matters. A strong choice uses a proven library or managed service and reserves custom consensus implementation for cases where you genuinely need an embedded consensus library (e.g., building a database) and have the expertise to operate it. Even using a well-known open-source consensus system means you must operate the cluster — sizing, backups, quorum-loss recovery, upgrades — so prefer the fully managed option where it exists. The decision to run your own consensus cluster is a decision to take on a serious operational burden; make it deliberately.

### Size The Cluster For The Fault Tolerance You Need, And Plan For Quorum Loss

Cluster sizing is a fault-tolerance decision, not a capacity decision. The standard sizes and what they tolerate:

- **3 nodes**: tolerates 1 failure, stays available. The minimum for consensus. Common for non-critical or single-zone systems.
- **5 nodes**: tolerates 2 failures, stays available. The standard for production systems that must survive a failure during a maintenance window or a zone outage.
- **7 nodes**: tolerates 3 failures. Used for very high availability or when you need to tolerate multiple simultaneous failures. Beyond 7, the quorum latency and operational cost usually outweigh the benefit — scale by sharding consensus groups, not by growing one cluster.

Plan for **permanent quorum loss**, because it happens: in a 5-node cluster, losing 3 nodes means no majority and the cluster cannot accept writes. Recovery from permanent quorum loss is dangerous — the documented procedure (force a new cluster from surviving replicas) can lose already-committed data if the surviving nodes are behind, and must be done with full awareness of the risk. Have a runbook, know that it exists, and treat invoking it as a last resort. Backups and the ability to restore to a managed service are your real safety net; consensus protects against node failures, not against catastrophic quorum loss or operator error.

### Handle Snapshotting And Log Compaction Operationally

A consensus log grows without bound as commands are replicated, and replaying it on restart or on a new joiner would be infeasible. Consensus systems address this with **snapshotting and log compaction**: periodically the applied state is snapshot, and the log entries the snapshot subsumes are discarded. A follower that is far behind, or a new node joining, receives the snapshot plus the log tail rather than the full history.

The operational points: snapshotting is expensive (it serializes state and can pause the state machine), so the snapshot threshold must be tuned to balance log size against snapshot cost. A node that falls behind the leader's oldest retained log entry must receive a snapshot to catch up — if snapshots are misconfigured or storage is constrained, a lagging follower may be unable to catch up at all. Monitor log size, snapshot frequency, and follower lag. A weak operation ignores compaction until the log fills the disk or a rejoin fails. A strong operation tunes snapshot thresholds, monitors log and snapshot growth, and verifies that new nodes can join and catch up within an acceptable window.

### Decide Read Consistency Explicitly — Reads Are Not Free Of The Stale-Leader Hazard

Consensus clusters are often read from as well as written to, and read consistency is a decision that is easy to get wrong because "it's just a read." The options, from strongest to weakest: **linearizable reads** require the read to go through the leader (or a quorum-confirmed read-index / lease-based read) so the result reflects all committed writes and never returns stale data; **stale or local reads** served from any follower are fast and scalable but can return data behind the committed state, including from a node that is partitioned or lagging. The hazard is symmetric with the stale-leader write problem: a follower serving a local read may be arbitrarily behind, and if clients read from followers to reduce leader load, they can observe values older than what they just wrote or observe time moving backwards across consecutive reads hitting different nodes.

The strong choice matches read consistency to the operation's requirement. Reads that drive correctness-critical decisions (does this account have funds, is this lock held, is this id taken) must be linearizable — route them to the leader or use a quorum-confirmed read. Reads that tolerate staleness (dashboards, listing, cached lookups) can use local or stale reads to gain throughput and reduce leader pressure, but the staleness bound should be known and the consumer should tolerate it. The weak choices are assuming any read from the cluster is correct (followers lag), or making every read linearizable when the workload does not need it and paying the leader-serialization cost that caps the cluster's read throughput. Also know your implementation's default: some systems serve follower reads as stale by default, and an operator who assumes "reads are consistent" will ship subtle bugs.

## Common Traps

### Implementing Raft From Scratch For A Correctness-Critical System

Writing a custom consensus implementation and trusting it because the algorithm "reads simply," then hitting a fencing, log-recovery, or membership-change bug in production that corrupts data. Use a proven library or managed service unless you have deep expertise and a genuine need.

### Running A 2-Node Or Even-Size Cluster For Fault Tolerance

Deploying 2 or 4 nodes and believing it adds availability, when an even-size cluster tolerates one fewer failure than expected or splits ambiguously. Use odd sizes (3, 5, 7) matched to the failure tolerance you need.

### Letting A Stale Leader Serve External Side Effects Without Fencing

Assuming Raft's single-leader-per-term rule prevents a deposed leader from acting, when a partitioned old leader can briefly serve clients. Add fencing tokens or leader leases for any action where staleness causes harm.

### Bulk-Editing Cluster Membership And Creating Two Disjoint Majorities

Adding or removing multiple nodes at once, or restarting nodes mid-change, ending with old-config and new-config majorities that disagree and split brain. Change membership one node at a time with verification between steps.

### Expecting The Minority Side Of A Partition To Stay Available

Assuming "high availability" means every node serves under partition, then being surprised that the minority refuses all operations. This refusal is the correctness guarantee; size the cluster and set expectations so the majority side covers your availability need.

### Using Default Election Timeouts Without Measuring The Network

Leaving election timeouts at defaults on a cross-region or high-latency link, then suffering constant leader churn under latency jitter. Measure round-trip times and set timeouts conservatively above them.

### Treating Quorum-Loss Recovery As Routine

Invoking the force-new-cluster procedure casually after losing quorum, without recognizing it can discard committed data if the surviving nodes are behind. Treat it as a last resort with a runbook, and rely on backups as the real safety net.

### Ignoring Log Compaction Until It Fills The Disk

Never tuning snapshot thresholds or monitoring log size, so the log grows unbounded and a lagging follower cannot catch up. Tune compaction and monitor log size, snapshot frequency, and follower lag.

### Applying Uncommitted Entries Or Crashing Before Persisting

In a custom implementation, applying entries before they are committed, or acknowledging before durably persisting, introducing data loss on crash. Persist before acknowledging, apply only committed entries, and verify the implementation handles the persisted-vs-committed distinction.

### Assuming Every Read From The Cluster Is Consistent

Reading from any follower to reduce leader load and assuming the result reflects committed state, when followers lag and can return stale data or data from a partitioned node. Make read consistency an explicit choice per operation — linearizable for correctness-critical reads, stale/local only where the business tolerates the lag.

## Self-Check

- [ ] The need for consensus (linearizable consistency, split-brain-free replication) was justified against cheaper eventually-consistent alternatives, and the quorum-latency, single-leader-throughput, and quorum-availability costs are accepted deliberately.
- [ ] Election timeouts are set conservatively above measured network round-trip times (especially cross-region), not left at defaults that cause leader churn under jitter.
- [ ] The log replication, commit, and safety properties are understood — a leader commits its own term's entries directly, old-term entries commit implicitly, the leader's log is authoritative over divergent followers, and persisted-vs-committed is handled correctly.
- [ ] Stale-leader risk is handled with fencing tokens or leader leases for any client-facing or external-side-effect action; the system does not rely on a leader's own belief that it is in charge.
- [ ] Membership and quorum changes are done one node at a time with verification between steps (joint consensus / single-membership-change semantics), never bulk-edited, with new nodes caught up before counting in the quorum and a documented rollback.
- [ ] The cluster is sized with an odd number of nodes (3/5/7) matched to the required failure tolerance, and the team understands the minority side of a partition must refuse all operations — this is the correctness guarantee, not a bug.
- [ ] A managed consensus system (etcd, Consul, ZooKeeper, managed database) is used unless there is a compelling, documented reason to run a custom implementation, and even self-operated clusters have a runbook for sizing, backups, upgrades, and quorum-loss recovery.
- [ ] Quorum-loss recovery is treated as a dangerous last resort with a runbook that acknowledges the risk of losing committed data, and backups/restores are the real safety net.
- [ ] Snapshotting and log compaction are tuned and monitored — log size, snapshot frequency, and follower lag are tracked, and new nodes can join and catch up within an acceptable window.
- [ ] Read consistency is an explicit per-operation choice — correctness-critical reads are linearizable (leader or quorum-confirmed), stale/local reads are used only where the business tolerates the lag and the implementation's default is known.
- [ ] No operational step (force-new-cluster, bulk membership edit, stale-leader tolerance, even-size cluster) silently violates the consensus safety invariant, and the highest-risk procedures were traced through their failure modes before an incident.
