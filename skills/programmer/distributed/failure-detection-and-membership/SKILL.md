---
name: failure_detection_and_membership.md
description: Use when the agent is designing or reviewing how a distributed system detects failed nodes, decides a peer is alive or dead, or tracks cluster membership; implementing heartbeats, ping/pong, gossip protocols, or phi-accrual failure detectors; tuning failure detection timeouts and thresholds; handling false positives from GC pauses, stop-the-world, CPU scheduling, or network jitter; preventing split-brain and premature failover under pause or partition; managing membership and cluster reconfiguration (adding, removing, evacuating nodes); or designing quorum and leader-failover behavior that avoids cascading failures from overly aggressive detection. Covers the failure-detection and membership judgment problem, distinct from the consensus algorithm internals (Raft/Paxos leader election) and from consistency-model theory.
---

# Failure Detection And Membership

Failure detection is the problem of deciding, in a distributed system where you cannot directly observe another node's state, whether a peer is alive, dead, or merely slow — and membership is the problem of tracking which nodes are part of the cluster and propagating joins, leaves, and failures consistently. These are the foundation that every higher-level mechanism (failover, replication, leader election, load balancing, quorum) depends on, and they are uniquely hard because of a fundamental ambiguity: in an asynchronous network, you cannot distinguish a crashed node from a slow or paused one. A node that stops responding may be dead, or it may be in a long garbage-collection pause, swapped to disk, delayed by a network blip, or starved by a noisy neighbor — and the detector cannot tell the difference. The judgment problem is how to detect failures fast enough to fail over promptly without producing so many false positives that the system tears itself apart.

Agents tend to under-invest here because the naive rule — "if I don't get a heartbeat within a timeout, the node is dead" — feels sufficient and works on the happy path. The harm appears only under the conditions that real systems hit: a GC pause longer than the heartbeat timeout marks healthy nodes dead and triggers cascading failover; an aggressive detector under load evacuates half the cluster and overwhelms the survivors; a partition is misread as a wave of failures; a membership change races with a failure and leaves the cluster in an inconsistent view. For the consensus algorithm that builds on failure detection (Raft leader election, log replication) see consensus-and-raft; for consistency-model theory see distributed-consistency-and-cap. This skill is the failure-detection and membership judgment problem itself: how to detect, how to avoid false positives, and how to reconfigure membership safely.

## Core Rules

### Treat "No Heartbeat" As "Suspect," Never As "Dead"

The foundational rule of failure detection is that the absence of a heartbeat proves only the absence of a heartbeat — it does not prove the node has crashed. In an asynchronous network, a missed heartbeat can be caused by a process pause (garbage collection, JIT compilation, scheduling delay), a transient network issue (packet loss, retransmission, routing flap), resource starvation (CPU contention, disk I/O wait, memory pressure triggering swapping), or clock drift — none of which mean the node is permanently gone. Treating "no heartbeat within T" as "node is dead" is a binary decision imposed on a continuous reality, and it produces false positives that cascade.

The strong design separates **suspicion** from **conviction**. A missed heartbeat marks a node suspect; only sustained evidence (a threshold of consecutive misses, or an accrual detector that integrates the history of inter-arrival times) escalates to a declared failure, and even then the action taken is reversible where possible (stop sending traffic, but don't destroy the node's state). Tune the detection threshold to the operational reality of the deployment: a system running on preemptible VMs with frequent GC needs a more forgiving threshold than one on dedicated hardware. The cost of a false positive (unnecessary failover, cascading load, split-brain risk) is usually higher than the cost of a slightly slower true-positive detection, so err toward fewer false positives. Measure the actual pause and latency distribution of your runtime before setting timeouts, and revisit them when the workload or platform changes.

### Use Accrual (Phi-Accrual) Detection To Handle Adaptive Latency

A fixed timeout ("declare dead after 3 seconds") is brittle because real heartbeat inter-arrival times vary with load, network conditions, and runtime behavior, and a single threshold cannot fit both a quiet cluster and one under heavy GC. The robust approach is **accrual failure detection** (the phi-accrual detector used in Cassandra, Akka, and others), which models the historical distribution of heartbeat inter-arrival times and expresses the current suspicion level as a continuous value (phi) that grows as the current arrival is more statistically anomalous. Instead of a binary timeout, the detector outputs "how suspicious is this node right now," and the consumer decides what phi threshold triggers what action.

The advantage is adaptivity: the detector learns the normal heartbeat pattern and adjusts to changing conditions, so a cluster with naturally jittery heartbeats does not produce constant false positives, and a sudden genuine failure still escalates quickly relative to the learned baseline. The judgment is in setting the phi threshold: a low threshold detects fast but false-positives more; a high threshold is conservative but slower. Understand that accrual detection does not eliminate false positives — a pause longer than the historical tail still looks anomalous — but it makes the tradeoff tunable and data-driven rather than a guessed constant. A weak design uses a single hardcoded timeout across a heterogeneous fleet; a strong design uses accrual or at least dynamically adjusted thresholds, and monitors the false-positive and detection-latency distributions.

### Prevent Split-Brain And Premature Failover With Quorum And Fencing

Failure detection that triggers failover (promoting a replica, electing a new leader) must not create split-brain: two nodes believing they are the primary for the same resource. The classic scenario: a partition isolates the primary from the rest; the detectors on the majority side declare the primary dead and promote a new one; the old primary is still alive on the minority side and may still accept writes — now two primaries serve conflicting state. The defense is that failover must require a **quorum**: only the side that can contact a majority of nodes may promote a replacement, and the isolated minority must stand down. This is the core property consensus provides (see consensus-and-raft), but the failure-detection layer must feed it accurate information and must not promote without quorum confirmation.

Even with quorum, the **stale-primary problem** remains: a primary that is paused (long GC) rather than partitioned may wake up after a new primary has been promoted and briefly act as if it is still in charge. The robust fix is **fencing tokens**: every primary promotion carries a monotonically increasing token, and protected resources (storage, downstream services) reject operations carrying a stale token. This requires the protected resource to check the token, which works when you control the resource and fails when you do not. A weak failover promotes on a simple heartbeat miss with no quorum and no fencing; a strong failover requires quorum, uses leases with bounded expiry, and fences the old primary at every external boundary. Never let failure detection alone grant authority — detection identifies a candidate, quorum agrees, and fencing enforces.

### Design Membership Reconfiguration To Be Safe Under Concurrent Failure

Membership — the set of nodes believed to be in the cluster — changes as nodes join, leave, fail, and are replaced, and these changes must propagate consistently or the cluster fragments into disagreeing views. The hazard is a membership change that races with a failure: a node is being added at the moment another fails, or a node is removed while it still holds unreplicated data, or two coordinators issue conflicting membership updates. The strong design treats membership as a replicated, agreed-upon state (via consensus or a gossip protocol with convergence guarantees) and sequences changes so that no transition leaves the cluster without a coherent majority.

For joins: a new node must catch up the log or data before it is counted in the quorum or receives traffic, or it can reduce effective fault tolerance or serve stale data. For leaves and evacuations: drain a node (redirect traffic, wait for in-flight operations) before removing it from membership, and ensure its data is fully replicated elsewhere before decommission. For failures: distinguish temporary suspicion from permanent removal — a flapping node should not be repeatedly added and removed, and a permanently dead node's replacement should be added deliberately with the catch-up and quorum rules above. Use a gossip protocol (SWIM, HyParView) for large clusters where centralized membership is a bottleneck, but understand that gossip provides eventual consistency of the membership view — there is a window during which different nodes disagree about who is in the cluster, and the system must tolerate that. A weak design lets any node edit membership and discovers inconsistent views under concurrent failure; a strong design makes membership a consensus or gossip-convergent state with sequenced, verified transitions.

### Avoid Cascading Failures From Aggressive Detection Under Load

The most dangerous systemic failure mode is a positive feedback loop: load rises, nodes pause or slow their heartbeats, the detector marks them suspect, the system fails over and redistributes their load to the survivors, the survivors now pause under the increased load, more are marked suspect, and the cluster collapses by evacuating itself. This is how aggressive failure detection turns a transient overload into a total outage. The judgment problem is sizing detection to fail over when genuinely needed without amplifying load spikes.

Defenses: set detection thresholds conservatively enough that transient pauses under expected load do not trigger failover; bound the rate of failover (do not evacuate many nodes at once); shed load (backpressure, request shedding) before failing over, so the system degrades rather than redistributes into collapse; and monitor the correlation between load and failure-detection events — if false positives spike under load, the threshold is too aggressive for the runtime. Also recognize that failover itself has a cost (state transfer, cache warming, connection storms) that can worsen the overload it is reacting to. A weak design treats every missed heartbeat as a reason to move load and amplifies the spike; a strong design pairs detection with backpressure, rate-limited failover, and conservative thresholds tuned to the observed pause distribution.

## Common Traps

### Treating A Missed Heartbeat As Proof Of Death

Declaring a node dead on a single missed or timed-out heartbeat, then failing over — only to have the node wake from a GC pause and find a replacement already promoted. Treat absence as suspicion, use accrual or threshold-based escalation, and make the action reversible where possible.

### Using A Fixed Timeout Tuned To The Happy Path

Setting the heartbeat timeout to a constant that works when the cluster is idle, then suffering constant false positives under load when pauses and jitter exceed it. Measure the actual pause and latency distribution and use accrual or dynamically adjusted thresholds.

### Failing Over Without Quorum And Creating Split-Brain

Promoting a replacement primary on a simple heartbeat miss with no quorum check, so a partitioned or paused old primary and the new one both serve writes. Require quorum for any promotion and fence the old primary at external boundaries.

### Aggressive Detection That Cascades Under Load

Tuning detection to be fast, then watching load spikes cause pauses that mark nodes suspect, failover redistributes load, survivors pause, and the cluster evacuates itself. Pair detection with backpressure, rate-limited failover, and conservative thresholds tuned to the runtime.

### Editing Membership Without Sequencing Or Catch-Up Verification

Adding a node to membership before it has caught up the data (reducing fault tolerance or serving stale reads), removing a node before its data is replicated elsewhere, or letting concurrent uncoordinated edits fragment the cluster view. Treat membership as agreed-upon state with sequenced, verified transitions.

## Self-Check

- [ ] Failure detection treats a missed heartbeat as suspicion, not proof of death — escalation uses sustained evidence (threshold of misses or accrual/phi-accrual integration), and the action taken is reversible where possible.
- [ ] Detection thresholds are tuned to the measured pause and latency distribution of the actual runtime and platform, not to a guessed constant or the happy-path default, and are revisited when workload or platform changes.
- [ ] Accrual or dynamically-adjusted detection is used where the heartbeat distribution is jittery, and the phi/threshold tradeoff (fast detection vs false-positive rate) was chosen deliberately and is monitored.
- [ ] Failover requires a quorum — no promotion happens on the minority side of a partition — and the stale-primary problem is handled with fencing tokens or bounded leases enforced at every external boundary.
- [ ] Membership is treated as replicated, agreed-upon state (consensus or convergent gossip), joins are sequenced with catch-up before quorum inclusion, leaves are drained before removal, and failures are distinguished from flapping.
- [ ] The system tolerates the eventual-consistency window of gossip-based membership — different nodes may disagree about who is in the cluster transiently, and the design does not break under that disagreement.
- [ ] Detection is paired with backpressure, load shedding, and rate-limited failover so that a load spike cannot cascade into mass evacuation and cluster collapse; the correlation between load and false-positive events is monitored.
- [ ] The highest-risk scenarios were traced: a GC pause longer than the timeout, a partition misread as mass failure, a membership change racing with a failure, and an aggressive detector under overload — and the design degrades rather than tears itself apart in each.
