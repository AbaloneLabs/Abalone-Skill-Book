---
name: multi_region_and_disaster_recovery.md
description: Use when the agent is designing multi-region resilience or disaster recovery, defining RTO and RPO objectives from business requirements, choosing between active-active and active-passive failover, designing cross-region replication and backup strategy, planning failover and failback, handling data consistency across regions during failover, or scheduling DR testing such as game days and failover drills. Also covers regional versus zonal failure, the shared-responsibility boundary for resilience, conflict resolution and last-write-wins risks, async versus sync replication lag, promoted-replica versus full multi-region patterns, and the tradeoff between the cost of over-engineering DR and the cost of an outage. Use when provisioning a multi-region database, designing a DR runbook, reviewing an architecture for regional failure, or deciding how much resilience a workload actually needs.
---

# Multi-Region And Disaster Recovery

Multi-region and disaster recovery design is where engineers most reliably build the wrong amount of resilience. The two failure modes are symmetrical and equally damaging: teams either over-engineer an elaborate active-active multi-region system for a workload whose business value cannot justify the complexity, or they declare "we have backups and a replica" and discover at the moment of failure that the replica cannot be promoted, the backups have never been restored, the failover has never been rehearsed, and the system is down for far longer than anyone promised. Both errors come from the same root cause: treating resilience as a feature to add rather than as a set of explicit, measurable objectives derived from the business, then verified by practice.

The judgment problem is that DR is judged by an event that has not happened yet, on objectives that were never written down, using mechanisms that are only proven by exercising them. Every backup is a hope until it is restored; every replica is a theory until it is promoted; every failover runbook is a draft until it is run against a real regional failure. Agents tend to copy resilience patterns from a reference architecture, add a cross-region replica and a nightly backup, and consider the work done — without defining what recovery actually requires, without checking whether the replica is usable, without resolving the data-consistency problems that failover creates, and without ever testing the whole path. The harm is concentrated and severe: at the moment of a real regional outage, an untested DR plan fails in ways that were invisible in steady state, and the outage extends from hours into days. This skill covers the practical architecture of resilience — RTO/RPO, backups, replication, failover, and DR testing. It does not cover cost (a separate skill) nor distributed-consistency theory (a separate concern); here the focus is making recovery objectives real and verified.

## Core Rules

### Derive RTO And RPO From Business Requirements Before Choosing Any Mechanism

Recovery Time Objective (how long the system can be down) and Recovery Point Objective (how much data loss is tolerable) are the two numbers that govern every other DR decision. They are not technical defaults to inherit; they are business commitments derived from the cost of downtime and the cost of data loss for this specific workload. Choosing mechanisms before defining RTO/RPO is the most common DR mistake, because it optimizes for an objective nobody stated.

- **RTO comes from the cost of unavailability.** How much revenue, reputation, safety, or contractual penalty accrues per hour of downtime? A workload whose downtime costs thousands per hour needs a short RTO and an automated or near-automated failover; a workload whose downtime is inconvenient but cheap can tolerate a longer RTO and a manual recovery. Different workloads in the same system often have different RTOs — do not impose one DR posture on everything.
- **RPO comes from the cost of data loss.** How much recent data can the business afford to lose? An order-processing system may tolerate near-zero loss (RPO of seconds); an analytics or logging workload may tolerate hours or a day. RPO drives the replication and backup design: synchronous or low-lag async replication for near-zero RPO, periodic snapshots or batch backup for tolerant RPOs.
- **State the objectives explicitly and get them agreed.** Write RTO and RPO down per workload, get them accepted by the business owner, and let them drive the architecture. Implicit objectives ("as little downtime as possible") produce either over-engineering or under-engineering, because "as possible" is unbounded in both directions.
- **Re-examine the objectives as the business changes.** A workload that was low-criticality at launch may become mission-critical later; a DR posture designed for the old RTO/RPO will silently under-protect the new reality.

### Treat Backups As Untested Hopes Until A Restore Has Been Proven

A backup that has never been restored is not a backup; it is a file you hope contains recoverable data. The single highest-leverage DR practice is regularly restoring backups into a fresh environment and confirming the data is complete, correct, and usable. Agents frequently configure backups, check the box, and never restore — then lose everything when the backup turns out to be corrupt, incomplete, encrypted with a lost key, or in a format the restore tool can no longer read.

- **Match backup type to RPO and recovery needs.** Full backups are simple and slow to restore; incremental backups are fast to take but require a chain that must be reconstructed on restore (and a broken chain breaks recovery); snapshots are near-instant and volume-consistent but tied to the storage layer; continuous backup (WAL/CDC/log shipping) gives near-zero RPO and point-in-time recovery but is more complex. Choose the type whose restore characteristics, not just whose backup characteristics, meet the RTO/RPO.
- **Test restores on a cadence, against a clean environment, with verification.** Restore the backup into fresh infrastructure, open it, and confirm row counts, recent records, and that applications can actually read it. An unverified restore is still a hope. Schedule this as a recurring drill, not a one-time project — backup tooling, formats, and encryption keys drift, and a backup that restored fine a year ago may not restore today.
- **Store backups off the primary failure domain.** A backup in the same region, account, or storage system as the primary dies with the primary. Cross-region, cross-account, or offline/immutable backup (resistant to ransomware and to accidental or malicious deletion) is what survives a regional or account-level event. The backup location must outlive the failure you are protecting against.
- **Encrypt backups and manage the keys independently.** A backup encrypted with a key stored alongside it, or with a key nobody can find during an incident, is unrecoverable in practice. Key custody is part of the backup design.

### Choose Failover Topology Against RTO, RPO, And The Cost Of Complexity

Failover topology — active-active versus active-passive, and the flavor of replication that supports it — is the architectural decision that most directly determines realized RTO and RPO. It is also where over-engineering is most common, because active-active sounds more robust and is dramatically harder and more expensive to operate correctly.

- **Active-passive (standby in a second region).** One region serves traffic; the second stands ready to take over. RTO is bounded by detection time plus promotion plus DNS/traffic shift. RPO is bounded by replication lag. This is the right default for most workloads: it provides regional resilience at bounded complexity. The "promoted replica" pattern (a read replica or standby that is promoted to primary on failover) is the common implementation for databases.
- **Active-active (both regions serve live traffic).** Both regions handle writes (global write capability) or at least serve reads and can absorb the other's traffic. This gives the shortest RTO (traffic already flows to the surviving region) and can improve latency for geographically distributed users, but it requires solving multi-writer data consistency, conflict resolution, and cross-region write coordination — which is genuinely hard and a frequent source of correctness bugs. Active-active is justified when the business genuinely cannot tolerate a failover gap, or when geo-latency demands it; it is over-engineering for a workload that could tolerate a few minutes of failover.
- **Match the database capability to the topology.** Some managed databases offer native multi-region active-active or global tables; others offer only async cross-region read replicas that must be promoted manually. The failover design is constrained by what the data store actually supports — a promoted-replica design has an RPO equal to the replication lag at the moment of failure, and a global-active design has consistency and conflict-resolution characteristics you must understand.
- **Plan failback, not just failover.** Failing over to the standby is half the operation; returning to the primary region (failback) without data loss or split-brain is the other half and is often harder. A DR plan that covers only failover leaves the system running in the DR region indefinitely after an event, with the original region unrecoverable as the steady state. Rehearse failback too.

### Understand Cross-Region Replication Lag And The Consistency It Actually Delivers

Replication is what makes RPO real, and the gap between "we have replication" and "we have the RPO we think we do" is replication lag — the delay between a write in the primary and its arrival in the replica. Agents frequently assume replication is instantaneous and design an RPO of zero, when the actual lag under load or network stress is seconds to minutes.

- **Sync versus async replication trades latency for durability.** Synchronous replication confirms a write only after it is durable in both regions, giving RPO zero at the cost of per-write latency (a cross-region round trip on every commit) and a failure mode where a slow or unreachable second region stalls writes in the primary. Asynchronous replication confirms the write locally and ships it later, giving low primary latency but an RPO equal to the lag. Most cross-region designs are async because sync across regions is too slow for interactive workloads; understand which you have and what RPO it implies.
- **Measure replication lag under realistic load, including spikes.** Lag that is milliseconds in steady state can balloon to minutes during a write burst, a network degradation, or a large transaction. The RPO you can promise is the lag at the worst moment before a failure, not the average. Monitor lag and alert when it threatens the committed RPO.
- **During failover, lag becomes potential data loss.** If the primary fails with N seconds of un-replicated writes, those writes are lost when the replica is promoted. The failover procedure must acknowledge this: decide whether to promote (accepting the loss) or wait (extending RTO), and never assume the replica is a perfect copy of the moment of failure.

### Resolve Data Consistency Across Regions Deliberately, Never By Default

Multi-region systems that allow writes or reads in more than one region create consistency problems that single-region systems do not have, and "last-write-wins" is the default resolution strategy that silently loses data. Agents often enable multi-region features without understanding the conflict model, then discover duplicate or lost records after an event that exposed a conflict.

- **Last-write-wins is dangerous when clocks disagree or writes are concurrent.** Wall-clock timestamps from different regions can skew, and two concurrent updates to the same record resolved by timestamp will silently keep one and discard the other — data loss dressed as convergence. LWW is acceptable for ephemeral or last-update-wins-by-nature data; it is a bug for data where every update matters.
- **Choose a conflict resolution strategy that matches the data's semantics.** Application-level merge, CRDTs for commutative data, version vectors to detect concurrent writes, or single-writer-per-partition designs that avoid conflicts entirely — the right strategy depends on whether updates commute, whether the data has a natural merge, and whether the business can tolerate occasional manual reconciliation. Do not accept the database's default resolution without understanding what it does to your data on conflict.
- **Beware split-brain and fencing during failover.** If the primary is not actually down but is partitioned from the failover coordinator, both regions may believe they are primary and accept writes — producing divergent histories that cannot be cleanly merged. Fencing (ensuring the old primary cannot continue to accept writes) is essential to safe failover; a failover without fencing risks two primaries and data divergence.

### Test DR By Rehearsing Failure, Because Untested DR Fails When Needed

The defining practice of real resilience is exercising the failover against a simulated (or real) failure, on a cadence, with the people who would do it for real. A DR plan that exists only as a document will fail when executed under the stress of an actual incident, because the steps that look correct on paper break against the realities of permissions, DNS propagation, stale runbooks, missing tooling, and human error.

- **Run game days and failover drills regularly.** Simulate a regional failure (or a zone failure for zonal resilience) and execute the failover end to end: detect, decide, promote, shift traffic, verify. Measure the realized RTO and RPO against the objectives. The first drill almost always takes far longer than expected and exposes gaps; subsequent drills tighten the time and harden the runbook.
- **Inject chaos at bounded risk.** Deliberately kill instances, sever network paths, or fail over a database in a controlled window to confirm the system recovers as designed. Chaos testing finds the resilience assumptions that are wrong (the "non-critical" dependency that actually takes the system down, the retry storm that amplifies a failure) before a real event does.
- **Keep the runbook alive.** A runbook written once and never re-run rots: commands change, endpoints move, tooling is replaced, and the people who wrote it leave. Each drill updates the runbook against reality, so that the document an on-call engineer reaches for at 3 a.m. during a real outage actually works.
- **Distinguish regional from zonal failure and test both.** Zonal failures (one availability zone) are common and should be handled transparently by multi-AZ design with no customer impact; regional failures are rarer and catastrophic and require the full multi-region failover. Test the zonal path (frequent, low-cost) and the regional path (less frequent, higher-cost) on different cadences, because they exercise different mechanisms.

### Respect The Regional And Zonal Failure Boundary And Shared Responsibility

Resilience is split between the provider and you, and the boundary is not where agents assume. The provider guarantees the resilience of the service within its defined scope (a managed database's multi-AZ, a region's independent zones); you are responsible for everything above and around that scope, including the architecture that combines services, the failover design, the data placement, and the application's behavior during partial failure.

- **Know whether your failure scenario is zonal or regional.** A zonal failure takes one AZ; a multi-AZ managed service absorbs it. A regional failure takes the whole region; only a multi-region design survives it. Many teams build multi-AZ and believe they have regional DR — they do not. Match the design to the failure you actually need to survive.
- **The provider's resilience does not extend to your configuration.** A multi-AZ database behind a single-AZ application tier, or a multi-region service fronted by a single-region DNS, has a single point of failure you introduced. Audit the whole path, not just the data store, for the failure domain it actually spans.
- **Application behavior during partial failure is your responsibility.** Retries that hammer a failing dependency, timeouts that are too long, caches that serve stale data indefinitely, and clients that do not fail over — these are application-level resilience concerns the provider cannot fix. Design the application to degrade gracefully under the partial failures your topology will produce.

### Balance The Cost Of DR Against The Cost Of The Outage It Prevents

DR has a standing cost — second-region infrastructure, replication traffic, the operational tax of running and testing a more complex system — and it is easy to spend that cost on resilience a workload does not need, or to skip it for a workload that does. The discipline is to size the DR investment to the actual cost of the outage being prevented.

- **Match DR investment to workload criticality.** A tier-1 revenue or safety system justifies active-active or robust active-passive with frequent drills; a tier-3 internal tool may justify only backups and a manual recovery with a longer RTO. Imposing maximal DR on every workload wastes money and operational attention; imposing minimal DR on a critical workload borrows a catastrophe.
- **Beware the cost of complexity as a resilience risk itself.** An elaborate multi-region active-active system is harder to operate correctly, and the operational errors it invites (misconfigured failover, a botched promotion, a conflict-resolution bug) can cause outages that a simpler active-passive design would not. Complexity is itself a source of downtime; the simplest design that meets the RTO/RPO is usually the right one.

## Common Traps

### Defining Mechanisms Before RTO And RPO

Adding a cross-region replica and nightly backups because "we need DR," without stating how much downtime and data loss the business tolerates, producing a design that is either far more or far less than the workload requires. Derive RTO and RPO from the business cost of downtime and data loss first, then choose mechanisms.

### Backups That Have Never Been Restored

Configuring backups and treating the configuration as the resilience, then losing data when the backup proves corrupt, incomplete, encrypted with an unreachable key, or in a format the tooling can no longer read. A backup is only proven by restoring it into a fresh environment and verifying the data, on a recurring cadence.

### Assuming Replication Lag Is Zero

Designing an RPO of zero on asynchronous cross-region replication, when real lag under load is seconds to minutes and becomes data loss at failover. Measure lag under realistic load, set the RPO to the worst-case lag, and monitor it against the committed objective.

### Active-Active For A Workload That Tolerates Failover

Building a global multi-writer active-active system — with all its consistency, conflict-resolution, and operational complexity — for a workload that could tolerate a few minutes of failover, incurring complexity that itself causes outages. Use active-active only when the business genuinely cannot tolerate a failover gap or when geo-latency demands it; otherwise active-passive is the right default.

### Last-Write-Wins On Data Where Every Update Matters

Enabling a multi-region or global feature with default last-write-wins conflict resolution on data where concurrent updates must not be silently discarded, then losing records on the first conflict. Choose conflict resolution to match the data's semantics; do not accept the database default without understanding it.

### Failover Without Fencing (Split-Brain Risk)

Promoting a replica without ensuring the old primary cannot keep accepting writes, so a partitioned-but-alive primary diverges from the new primary and produces unreconcilable histories. Fencing is essential to safe failover; a failover procedure without it risks two primaries.

### A DR Runbook That Has Never Been Run and multi-AZ Mistaken For Multi-Region DR

Writing a failover runbook and trusting it, then discovering during a real incident that the commands, endpoints, permissions, or tooling have drifted and the steps do not work. Rehearse the failover on a cadence and update the runbook against reality each time.

Believing a multi-AZ deployment provides disaster recovery against a regional failure, when a regional outage takes the whole system down. Multi-AZ handles zonal failure; only a multi-region design handles regional failure. Match the design to the failure you need to survive.

### Skipping Failback In The Plan and dR Investment Mismatched To Criticality

Designing and rehearsing failover to the standby but never failback to the primary, so after a real event the system is stranded in the DR region with no clean path back. Plan and rehearse failback too; it is often the harder half of the operation.

Spending on maximal resilience for low-criticality workloads while leaving a tier-1 system with only nightly backups, or skipping DR entirely for a workload whose outage would be catastrophic. Size DR investment to the actual cost of the outage prevented, per workload.

## Self-Check

- [ ] RTO and RPO are stated explicitly per workload, derived from the business cost of downtime and data loss, agreed with the business owner, and re-examined as criticality changes — not inherited as technical defaults or left as "as little as possible."
- [ ] Backups match the RPO and recovery needs by type (full, incremental, snapshot, continuous), are stored off the primary failure domain (cross-region, cross-account, or immutable), are encrypted with independently managed keys, and — critically — have been restored into a fresh environment and verified on a recurring cadence, not just configured.
- [ ] The failover topology (active-passive with promoted replica, or active-active) was chosen against RTO/RPO and the cost of complexity; active-active is used only where the business cannot tolerate a failover gap or geo-latency demands it, and failback is planned and rehearsed, not just failover.
- [ ] Cross-region replication is understood as sync or async with its latency and durability tradeoff, replication lag is measured under realistic load including spikes, the committed RPO reflects worst-case lag, and the failover procedure explicitly decides between accepting lag-induced loss and extending RTO.
- [ ] Data consistency across regions is resolved by a strategy chosen for the data's semantics (application merge, CRDTs, version vectors, or single-writer-per-partition), last-write-wins is used only where data loss on conflict is acceptable, and failover includes fencing to prevent split-brain and divergent primaries.
- [ ] DR is tested by rehearsing failure on a cadence — game days, failover drills, and bounded chaos injection — measuring realized RTO/RPO against objectives, and the runbook is updated against reality each drill rather than trusted as a static document.
- [ ] The regional versus zonal failure boundary is understood and matched to the design: multi-AZ handles zonal failure, multi-region handles regional failure, the whole request path (not just the data store) is audited for its failure domain, and application behavior during partial failure (retries, timeouts, caching, client failover) is designed for graceful degradation.
- [ ] DR investment is sized to the actual cost of the outage prevented, per workload — maximal resilience for tier-1 systems, lighter recovery for low-criticality ones — and the complexity introduced by resilience is itself weighed as a source of downtime, favoring the simplest design that meets the RTO/RPO.
