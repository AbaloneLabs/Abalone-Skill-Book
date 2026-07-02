---
name: recovery_and_failover.md
description: Use when the agent is designing or executing recovery and failover procedures during or after an incident, defining RTO/RPO targets and whether they are met, planning failover for stateful services and databases, deciding between partial recovery and full restoration, validating that a recovery actually succeeded, handling data restoration and loss, or reviewing why a failover did not work when invoked. Also covers the failure mode of an untested failover that silently breaks, recovery that restores service but loses or corrupts data, failover to a region that cannot handle the load, and the gap between a documented procedure and one that works under pressure.
---

# Recovery And Failover

Recovery and failover are the actions that restore service after a failure, and they are where the gap between "documented" and "works" is most exposed. A failover procedure that has never been rehearsed will, with high probability, fail when invoked, because the conditions of a real incident (a region down, a corrupted primary, a panicked on-call at 3 a.m.) are nothing like the conditions under which the procedure was written. Recovery that restores service but loses or corrupts data is a partial success that registers to users as a disaster, because the service is back but their work is gone. The judgment problem is to define recovery targets (how fast, how much data loss is acceptable) before the incident, to design failover that has been tested under failure conditions rather than assumed to work, to prioritize recovery actions by user impact when not everything can be restored at once, and to validate that a recovery actually succeeded — service responding is not the same as service correct — before declaring the incident over.

Agents tend to treat recovery as a procedure to follow rather than a capability to verify. The harm appears only during the incident: the failover script fails because the target region's capacity was never sized for failover traffic, the database replica is hours behind because replication lag was never monitored, the "restore from backup" path takes ten times longer than the RTO because backups were never tested for restore speed. The discipline is to define RTO and RPO as design constraints (not aspirations), to rehearse failover under realistic failure conditions, to plan for partial recovery when full recovery is slow, to treat data integrity as part of recovery (not separate from it), and to verify recovery against expected behavior before declaring victory. An untested recovery plan is a wish; a tested one is a capability.

## Core Rules

### Define RTO And RPO As Design Constraints, Not Aspirations

Recovery Time Objective (how fast service must be restored) and Recovery Point Objective (how much data loss is acceptable) are the targets that should shape the architecture. If the RTO is five minutes, the architecture must support five-minute recovery (automated failover, pre-warmed capacity); if the RPO is zero, the architecture must support zero-loss (synchronous replication). Aspirational RTO/RPO that the architecture cannot meet are worse than none, because they create false confidence.

- **Define RTO and RPO per service based on business impact.** A revenue path may need minutes and zero loss; an internal analytics tool may tolerate hours and some loss. The targets must reflect actual need, not a default "everything is critical."
- **Make the architecture actually support the targets.** A five-minute RTO requires automated failover and pre-sized capacity; a zero RPO requires synchronous replication. If the architecture does not support the target, either change the architecture or revise the target honestly.
- **Measure whether targets are met during real incidents and tests.** A target that is never measured is aspirational; track actual recovery time and data loss against the objectives.

### Rehearse Failover Under Realistic Failure Conditions

A failover procedure that has never been run will, with high probability, fail when invoked, because real failures break assumptions the procedure depends on. Rehearsal — game days, chaos engineering, regular failover drills — is what converts a documented procedure into a working capability.

- **Run failover drills regularly, not just at launch.** Capabilities decay: DNS caches change, capacity drifts, runbooks rot. Regular drills catch the decay before the incident does.
- **Test under realistic failure conditions.** A planned failover during business hours with everyone watching proves little; a failover that simulates a region loss, a corrupted primary, or a partial network partition proves the procedure works when it matters.
- **Test the full path, including rollback.** Failover is half the procedure; failing back, or rolling over to a new primary, is the other half and is often neglected.

### Size Failover Capacity For The Failover Scenario

A common failure is failing over to a secondary region or replica that cannot handle the load, so the failover "succeeds" technically but the recovered service immediately collapses under traffic it was never sized for. Failover capacity must be sized for the failover scenario, not for steady-state secondary usage.

- **Size failover targets for production load, plus headroom.** A standby that idles at 10% utilization must be able to take 100% (or more, if users retry during the incident) without collapsing.
- **Account for retry storms and traffic spikes during recovery.** Users and automated clients retry during an outage, so the recovered service may see higher-than-normal load immediately.
- **Verify capacity assumptions in drills.** A failover drill that does not also validate capacity proves the mechanism but not the sufficiency.

### Prioritize Recovery By User Impact When Full Recovery Is Slow

When a failure affects many components, full recovery may be slow, but partial recovery can restore the most important functionality quickly. The judgment is to sequence recovery by user impact: restore the revenue path before the analytics dashboard, the customer-facing service before the internal tool.

- **Define recovery priority by user and business impact.** Know in advance which services or features matter most, so recovery effort goes there first.
- **Accept partial recovery when it restores the most value fastest.** A degraded-but-up revenue path is better than a down-everything waiting for full recovery.
- **Communicate partial recovery honestly.** Users should know which features are back and which are still degraded, not be told "all clear" when parts are still broken.

### Treat Data Integrity As Part Of Recovery, Not Separate From It

Service responding is not the same as service correct. A recovery that brings the service up but with lost, duplicated, or inconsistent data has restored the interface while breaking the contract. Data integrity checks must be part of the recovery validation, not an afterthought.

- **Verify data integrity after recovery.** Check for lost records, duplicated writes, referential inconsistencies, and sequence gaps that indicate the recovery missed or duplicated transactions.
- **Reconcile against independent sources where possible.** Compare recovered data against source-system totals, logs, or replicas to catch loss or corruption the recovery itself would not reveal.
- **Decide explicitly how much data loss is acceptable and communicate it.** If the RPO was not met, users and stakeholders must know what was lost, not discover it later.

### Validate Recovery Before Declaring The Incident Over

The incident is not over when the service responds; it is over when the service is verified correct under real load. Declaring victory prematurely — because the health check passed — leaves users hitting a service that is up but wrong, and the incident re-opens.

- **Validate against expected behavior, not just liveness.** A health check returning 200 is necessary but not sufficient; verify that real transactions succeed, data is consistent, and performance is within bounds.
- **Watch the recovered service for secondary failures.** Recovery often surfaces latent issues (a cache that is cold, a connection pool that is exhausted, a replica that is behind) that appear only under real traffic.
- **Confirm stability before standing down.** A service that flaps between up and down is not recovered; require a stable period before declaring the incident resolved.

## Common Traps

### Untested Failover That Silently Breaks

A failover procedure documented but never rehearsed, which fails when invoked because conditions (DNS, capacity, credentials, runbook steps) have drifted since it was written. Rehearse failover regularly under realistic failure conditions.

### Aspirational RTO/RPO The Architecture Cannot Meet

RTO/RPO targets that sound good but that the architecture does not support, creating false confidence until the incident reveals the gap. Define targets as design constraints and verify the architecture meets them.

### Failover To A Target That Cannot Handle The Load

Failing over to a secondary region or replica sized for standby usage, so the recovered service collapses under production traffic. Size failover targets for the failover scenario, including retry storms.

### Service Restored But Data Lost Or Corrupted

A recovery that brings the service up but with lost, duplicated, or inconsistent data, restoring the interface while breaking the contract. Verify data integrity and reconcile against independent sources as part of recovery validation.

### Declaring Victory On Liveness Alone

Declaring the incident over because the health check returns 200, leaving users hitting a service that is up but wrong or slow. Validate against expected behavior and watch for secondary failures under real load before standing down.

### Recovery Sequence That Ignores User Impact

Restoring components in an arbitrary or convenience-based order, so low-impact internal tools come back while the revenue path stays down. Sequence recovery by user and business impact and accept partial recovery when it restores the most value fastest.

### Replication Lag Discovered During Failover

A database replica assumed to be current that is hours behind, so failover to the replica loses hours of data — discovered only during the incident. Monitor replication lag continuously and ensure it stays within the RPO.

## Self-Check

- [ ] RTO and RPO are defined per service based on business impact, the architecture actually supports them (automated failover for short RTO, synchronous replication for zero RPO), and actual recovery time and data loss are measured against them during incidents and drills.
- [ ] Failover is rehearsed regularly under realistic failure conditions (region loss, corrupted primary, partial network partition), including the failback path, not only at launch — because capabilities decay and drift.
- [ ] Failover targets (secondary regions, replicas, standby capacity) are sized for the failover scenario including retry storms, not for steady-state secondary usage, and capacity assumptions are verified in drills.
- [ ] When full recovery is slow, recovery is sequenced by user and business impact, partial recovery is accepted when it restores the most value fastest, and the partial state is communicated honestly to users.
- [ ] Data integrity is verified after recovery (lost records, duplicated writes, referential inconsistencies, sequence gaps), reconciled against independent sources where possible, and any data loss beyond the RPO is communicated explicitly rather than discovered later.
- [ ] Recovery is validated against expected behavior (real transactions succeed, data consistent, performance within bounds) and watched for secondary failures under real load before the incident is declared over — liveness alone is not sufficient.
- [ ] Replication lag and backup restore speed are monitored/tested continuously, not discovered during the incident to be outside the RPO/RTO.
- [ ] The highest-risk cases were verified — an untested failover rehearsed before it was needed, a recovery that was checked for data loss not just liveness, a failover target validated for capacity not just mechanism, and a partial recovery sequenced by impact — not only the clean full-recovery path.
