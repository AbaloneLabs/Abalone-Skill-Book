---
name: disaster_recovery_and_backup.md
description: Use when the agent is designing disaster recovery (DR), backup and restore procedures, multi-region failover, RTO/RPO targets, or business continuity; planning for regional outages, data center loss, ransomware, or catastrophic failure; designing backup strategy (full/incremental/snapshot, frequency, retention, off-site/off-region storage); testing restore and failover; or defining recovery objectives. Covers RTO/RPO definition, backup strategy and the 3-2-1 principle, multi-region and hot/cold standby architectures, failover and DNS/traffic shifting, restore testing, and the cultural discipline that an untested recovery plan is not a recovery plan.
---

# Disaster Recovery And Backup

Disaster recovery is the discipline of surviving catastrophic failures — the loss of a region, a data center, a primary database, or a cloud account — and the central truth of DR is that a recovery plan that has not been tested is not a recovery plan. Organizations routinely maintain backup and failover procedures that look correct on paper, have never been exercised, and fail catastrophically when first invoked in a real disaster, because the restore tooling is broken, the credentials are expired, the runbook steps are stale, the data is corrupt, or the failover target is misconfigured. The gap between "we have backups" and "we can recover" is vast, and it is closed only by regularly exercising the recovery path end to end, in conditions as close to a real disaster as feasible. A backup that has never been restored from is an article of faith; a failover that has never been executed is a hypothesis. The discipline of DR is converting faith and hypotheses into demonstrated capability.

Agents tend to treat backups as a checkbox ("we take daily backups") without defining recovery objectives, to design failover architectures without testing them, and to store backups in ways that are themselves vulnerable to the disaster (same region, same account, no offline copy). The judgment problem is recognizing that DR is defined by measurable objectives (RTO, RPO), that backup strategy must account for the full range of failure modes including malicious deletion and ransomware, that multi-region failover introduces data-consistency challenges, and that the entire plan's value depends on regular testing. This skill covers the discipline of disaster recovery and backup: defining objectives, backup strategy, multi-region architectures, failover, and the testing that makes recovery real.

## Core Rules

### Define Recovery Objectives (RTO And RPO) From Business Requirements

Recovery objectives translate business needs into engineering targets. Without them, DR design has no anchor.

- **RPO (Recovery Point Objective): how much data loss is acceptable, measured in time.** An RPO of 1 hour means the system can lose up to 1 hour of recent data in a disaster. RPO drives backup frequency and replication strategy (continuous replication for low RPO; periodic snapshots for higher RPO).
- **RTO (Recovery Time Objective): how fast the system must be back, measured in time.** An RTO of 4 hours means the system must be restored and serving within 4 hours of a disaster. RTO drives the failover architecture (hot standby for low RTO; cold restore for higher RTO).
- **Derive RTO/RPO from business impact, not from what is technically convenient.** The acceptable downtime and data loss are business decisions (revenue impact, contractual obligations, user trust), not engineering preferences. Define them with stakeholders; design the system to meet them.
- **Different components may have different objectives.** A billing database may need near-zero RPO (no financial data loss) while a reporting cache tolerates hours. Tier the DR strategy by component criticality.

### Design Backup Strategy For Coverage, Retention, And Survivability

Backups must cover the full range of failure modes, retain data long enough, and survive the disaster itself.

- **Follow the 3-2-1 principle: 3 copies, 2 different media, 1 off-site.** Three copies of the data (primary plus two backups), on two different media/types (guarding against media-specific failure), with at least one copy off-site/off-region (surviving a regional disaster). This is the baseline for survivability.
- **Cover all failure modes, including malicious deletion and ransomware.** A backup in the same account as the primary is deleted when the primary account is compromised (ransomware, malicious insider). Store at least one backup in a separate account or offline, with immutable/wORM (write-once-read-many) storage where feasible, so it cannot be deleted by the same compromise.
- **Choose backup types by recovery need: full, incremental, snapshot.** Full backups are simple to restore but expensive; incremental backups are cheap but require a chain to restore; snapshots (storage-level) are fast and consistent. Match the type to the RTO (fast restore needs snapshots or full backups, not long incremental chains) and the storage budget.
- **Define retention by recovery and compliance needs.** Retain backups long enough to recover from detected-late failures (a corruption discovered weeks later needs a backup old enough to predate it) and to meet compliance (regulatory retention periods). Balance retention against storage cost and the risk of retaining sensitive data longer than needed.
- **Verify backup integrity, not just existence.** A backup that was taken but is corrupt or incomplete is no backup. Verify checksums, test restorability, and monitor backup success — a silently-failing backup process is a common DR gap.

### Choose A Multi-Region Architecture Matching The RTO/RPO

Multi-region deployment provides regional disaster recovery, with architectures ranging from cold backup to active-active, chosen by the RTO/RPO and cost tolerance.

- **Cold/backup-only (highest RTO, lowest cost): primary in one region, backups in another, manual restore on disaster.** RTO is hours to days (restore from backup to a new region); RPO is the backup interval. Simple and cheap; suitable for systems tolerating long downtime.
- **Warm standby (moderate RTO, moderate cost): a scaled-down copy of the system in a second region, continuously receiving data, promoted on disaster.** RTO is minutes to hours (scale up the standby, shift traffic); RPO is the replication lag (seconds to minutes). A common middle ground.
- **Hot standby / active-passive (low RTO, higher cost): a full replica in a second region, ready to serve, switched to on disaster.** RTO is minutes (traffic shift); RPO is near-zero (continuous replication). More expensive; suitable for critical systems.
- **Active-active (near-zero RTO, highest cost): serving from multiple regions simultaneously.** RTO is near-zero (traffic already distributed); RPO depends on the multi-region data consistency model. The most complex (conflict resolution, consistency); reserved for the most critical global systems.

### Handle Failover: Traffic Shifting, Data Consistency, And Split-Brain

Failover is the act of promoting a standby to primary, and it involves traffic shifting, data consistency, and the risk of split-brain (two primaries diverging).

- **Plan the traffic shift: DNS, load balancer, or routing.** Failover shifts traffic from the failed region to the healthy one via DNS (slow propagation, cached), global load balancer (faster), or routing at the edge. Understand the propagation delay; DNS-based failover can take minutes to hours due to caching.
- **Address data consistency during and after failover.** If replication was asynchronous, the standby may be behind the primary at failover, losing recent writes (RPO). After failover, if the old primary recovers, its unreplicated writes must be reconciled (or discarded) to avoid divergence. Plan for this reconciliation.
- **Prevent split-brain.** If both the old primary and the new primary believe they are primary (network partition masking the failure), they both accept writes and diverge. Use fencing (quorum, consensus, fencing tokens) to ensure only one primary accepts writes at a time. See idempotency-and-race-safety and distributed-consensus.
- **Automate failover where safe, manual where risky.** Automated failover is fast but can trigger on false positives (a network blip causing an unnecessary failover). Manual failover is safer but slower. Match the automation to the failure-detection confidence and the cost of an unnecessary failover.

### Test The Recovery Path Regularly, End To End

The defining discipline of DR is testing. An untested recovery plan is a hypothesis, not a capability.

- **Exercise restore from backup regularly.** Restore backups to a test environment on a schedule (weekly or monthly), verifying the data is complete, the tooling works, and the restore meets the RTO. A backup that has never been restored from is unverified.
- **Exercise regional failover regularly.** Run failover drills (Game Days, chaos engineering) that simulate a regional outage and promote the standby, verifying traffic shifts, data is consistent, and the system serves from the new region. An untested failover is a hypothesis.
- **Test in conditions as close to real as feasible.** A test that uses pre-staged data and a known-good runbook does not catch the failures a real disaster exposes (broken tooling, expired credentials, stale runbooks, corrupt data). Inject realism: restore from an actual backup, fail over without warning, have someone who did not write the runbook execute it.
- **Document and fix the failures each test reveals.** Each DR test reveals gaps (a missing step, a broken tool, a credential issue). Fix them; re-test to confirm. The recovery plan improves through the cycle of test, find-gap, fix, re-test.
- **Measure RTO and RPO in tests, not just in design.** The designed RTO/RPO are targets; the tested RTO/RPO (how long restore actually took, how much data was actually lost) are the real capability. If tested RTO/RPO exceed targets, the design or process must improve.

## Common Traps

### Backups Without Tested Restore

Daily backups that have never been restored from, so corrupt, incomplete, or tooling-broken backups are discovered only during a disaster. Test restore regularly.

### Backups In The Same Region/Account As Primary

Backups deleted alongside the primary when the account is compromised (ransomware, malicious deletion). Follow 3-2-1; store off-region/off-account/immutable.

### RTO/RPO Defined By Convenience, Not Business Need

Recovery targets set by what is easy to build rather than what the business requires, leaving the system unable to meet business needs in a disaster. Derive RTO/RPO from business impact.

### Untested Failover

A multi-region failover architecture that has never been exercised, failing when first invoked (traffic shift broken, data inconsistent, credentials expired). Run failover drills.

### Split-Brain After Failover

Two primaries accepting writes during/after failover, causing divergence. Use fencing (quorum, consensus, fencing tokens) to ensure single-primary writes.

### Replication Lag Exceeding RPO

Asynchronous replication with lag greater than the RPO, losing more data than the target on failover. Monitor replication lag; ensure it stays within RPO.

### No Verification Of Backup Integrity

Backups assumed complete because the backup job reported success, without checksum or restore verification. Verify integrity; monitor for silent failures.

### Recovery Runbook Known Only To Its Author

A runbook that only its author can execute, failing when the author is unavailable or the steps are stale. Have someone else execute the runbook in tests; document for executability by others.

## Self-Check

- [ ] Recovery objectives (RTO — time to restore; RPO — acceptable data loss) are defined from business requirements (not engineering convenience), tiered by component criticality, and drive the backup frequency, replication strategy, and failover architecture.
- [ ] Backup strategy follows the 3-2-1 principle (3 copies, 2 media, 1 off-site/off-region), covers malicious deletion and ransomware (separate account or immutable/offline copy), uses appropriate backup types (full/incremental/snapshot) for the RTO, and retains data per recovery and compliance needs.
- [ ] Backup integrity is verified (checksums, test restoration), backup success is monitored for silent failures, and corrupt or incomplete backups are detected before a disaster reveals them.
- [ ] The multi-region architecture (cold/warm/hot/active-active) matches the RTO/RPO and cost tolerance, with the failover mechanism (DNS, load balancer, edge routing) understood including its propagation delay.
- [ ] Failover addresses data consistency: replication lag is monitored and kept within RPO, asynchronous replication's potential data loss is planned for, post-failover reconciliation of the old primary's unreplicated writes is designed, and split-brain is prevented via fencing (quorum, consensus, fencing tokens).
- [ ] The recovery path is tested regularly and end to end — restore from backup is exercised on a schedule, regional failover is drilled (Game Days), tests use realistic conditions (actual backups, no-warning failover, runbook executed by someone other than its author), and each test's revealed gaps are fixed and re-tested.
- [ ] Tested RTO and RPO (actual restore time, actual data loss in drills) are measured against targets, and the design or process is improved when tested capability falls short of the targets.
- [ ] The DR plan accounts for the full disaster scope (regional outage, account compromise, primary database loss, ransomware), documents the recovery path executably, and is reviewed and updated as the system evolves so the plan does not drift from reality.
