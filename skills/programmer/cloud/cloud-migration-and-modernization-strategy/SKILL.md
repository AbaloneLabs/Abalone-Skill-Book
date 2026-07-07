---
name: cloud_migration_and_modernization_strategy.md
description: Use when the agent is migrating workloads to the cloud, choosing between lift-and-shift (rehost), replatform, and refactor (rearchitect) strategies, sequencing migration waves, planning a big-bang versus phased cutover, designing data migration and dual-running, or validating parity between source and target systems. Also covers the trap of migrating without re-architecting for cloud cost and operations, dependency mapping and the "six Rs" dispositioning, rollback and cutover risk, strangler-fig incremental migration, data consistency during dual-run, and validating functional and non-functional parity before decommissioning the source. Use when planning a cloud migration, reviewing a migration runbook, deciding migration strategy per workload, or executing a cutover.
---

# Cloud Migration And Modernization Strategy

Cloud migration is the project class that most reliably delivers a system that is technically moved but operationally and economically worse, because the work is judged on "did it get to the cloud" rather than on "does it run well and cost-effectively in the cloud." A lift-and-shift that reproduces the on-premises architecture verbatim arrives paying cloud prices for an on-premises shape — no elasticity, no managed-service reliability, and all the egress and idle costs the original never had.

Migration forces two decisions that pull against each other: how much to change the workload as you move it, and how to sequence and cut over without breaking the business. Change too little and you carry forward every on-premises anti-pattern, now billed hourly. Change too much and the migration never completes. Agents tend to pick one strategy for all workloads, ignore dependency ordering, and treat cutover as a single event, then discover at cutover that a dependency was missed, the data does not reconcile, or the new system cannot handle the real load.

## Core Rules

### Disposition Each Workload Individually, And Tie The Choice To Cloud Value — Not Just Speed

The first and most consequential decision is how to treat each workload, and it must be made per workload, not once for the whole estate. The "six Rs" (rehost, replatform, refactor/re-architect, repurchase, retain, retire) are not a maturity ladder; they are a menu where the right choice depends on the workload's business value, shape, dependencies, and the risk each path implies.

- **Rehost (lift-and-shift) for speed, accepting deferred modernization.** Move largely as-is to exit a data center or reduce near-term risk. Fast and low-effort, but carries forward the existing architecture's costs and operational model, and almost never realizes cloud benefits beyond hosting relocation. Use it only with an explicit plan to replatform or refactor later, otherwise "temporary" becomes permanent.
- **Replatform for cloud-aware improvements at bounded risk.** Move with targeted changes that adapt to the cloud without re-architecting: managed databases, managed load balancing, autoscaling, snapshots instead of custom backups. Captures meaningful benefit at far less risk than a full refactor — often the highest-value default.
- **Refactor / re-architect only when the business value justifies the cost and risk.** Restructure to exploit cloud-native patterns (serverless, microservices, event-driven) when the workload's future depends on capabilities the current shape cannot reach. High effort and high risk; justified only when the strategic payoff is real, not because refactor is "more cloud-native."
- **Repurchase, retain, or retire are legitimate dispositions.** Replacing custom code with SaaS is often better than migrating bespoke code. Retaining on-premises is correct when migration cost exceeds value or compliance forbids it. Retiring a workload no longer needed is frequently the best ROI decision in the estate.

### Map Dependencies And Sequence Migration Waves Before Moving Anything

Migration order is determined by the dependency graph, and moving a workload before its dependencies creates integration gaps, forced dual-running of the wrong things, and cutover deadlocks. Map upstream and downstream dependencies for every workload, including databases, queues, APIs, shared services, and identity stores. Sequence waves by dependency, risk, and learning value: move foundational shared services and low-risk workloads first to build migration muscle; then move workloads in dependency-coherent groups so each wave is internally complete; reserve the highest-risk, most coupled, or most business-critical workloads for later. Stand up shared services and foundational dependencies (identity, networking, DNS, observability, shared data stores) before the workloads that need them.

### Choose Cutover Strategy Against Risk And Reversibility, Not Just Speed

Cutover is the moment of maximum migration risk: the business depends on the new system, the old is about to be shut down, and any undiscovered gap becomes a production incident. Big-bang cutover — switching the whole workload at once — is fast and avoids dual-running cost, but has no fallback if the new system fails under real load; reserve it for small, isolated, or stateless workloads. Phased cutover (canary by tenant, region, or feature) bounds risk and preserves rollback: move a slice of traffic, validate, expand, repeat, so a problem affects only the migrated slice. Plan rollback explicitly, including data rollback: a cutover without a tested rollback is a one-way door. Rehearse it, especially data rollback, since once the new system has accepted writes, reverting loses them unless reconciled back.

### Design Data Migration And Dual-Running For Consistency, Not Just Copy

Data is the hardest part of migration: it is large, changes continuously, and must be consistent between source and target while both systems exist. A one-time copy produces stale target data the moment the source changes. Migrate in stages: bulk-copy the historical data first, then keep source and target in sync via continuous replication (CDC, log shipping, or dual-writes) until cutover, so the target is near-current at switch. Dual-running is the bridge, with a cost and consistency burden: running both in parallel doubles some costs and opens a window where the two can diverge. Handle schema and transformation deliberately, since source and target schemas rarely match exactly, so define and test the transformation, and reconcile row counts, checksums, and business-meaningful queries before declaring the data migrated.

### Validate Functional And Non-Functional Parity Before Decommissioning The Source

"Parity" is the gate that permits shutting down the old system, and it is the gate most often waved through prematurely. Functional parity (the new system does what the old did) is necessary but not sufficient; non-functional parity (performance, reliability, observability, security) determines whether the migration actually succeeded in production. For functional parity, validate the new system produces the same outputs for the same inputs across the full surface area, including rarely-used paths, batch jobs, integrations, and reporting. For non-functional parity, load-test against the real traffic profile, confirm latency and throughput, verify failover and recovery, and confirm observability covers the new system; a migration that matches features but degrades latency or loses observability is a regression. Do not decommission the source until parity is proven under real load.

## Common Traps

### One Strategy Applied To The Whole Estate

Choosing lift-and-shift (or refactor) for every workload, ignoring that the right disposition depends on each workload's value, shape, dependencies, and risk. Disposition per workload; retire and retain are legitimate, and replatform is often the best default.

### Lift-And-Shift Declared Done Without Cloud Adaptation

Moving the architecture verbatim and calling the migration complete, then paying cloud prices for oversized always-on instances, self-managed middleware, and manual operations. Plan replatform/refactor as a real follow-on, or accept the cost and toil knowingly.

### Migrating Without A Dependency Map

Moving workloads in an order that ignores the dependency graph, stranding workloads that depend on unmigrated shared services and forcing costly cross-environment integration. Map dependencies first and sequence waves so each is internally complete.

### Big-Bang Cutover For A Workload That Could Have Been Phased

Switching a large or critical workload all at once to save dual-running cost, then having no fallback when the new system fails under real load. Prefer phased cutover with incremental traffic shift wherever the workload supports it.

### Cutover Without A Tested Rollback, Especially For Data

Declaring a cutover plan with no rehearsed rollback, or a rollback that ignores data written to the new system during the failed window, turning a recoverable problem into a one-way door. Rehearse rollback including data reconciliation.

### One-Time Data Copy Without Ongoing Replication

Bulk-copying data and cutting over, so the target is stale relative to source changes made during and after the copy. Use bulk load plus continuous replication (CDC, log shipping, dual-writes) and reconcile before cutover.

### Parity Waived Through On Features Alone

Declaring parity because the new system passes functional tests, without load-testing performance, verifying failover, or confirming observability. Validate non-functional parity under real load before decommissioning.

### Decommissioning The Source Too Early (Or Too Late)

Shutting down the old system immediately at cutover, removing the fallback before late defects surface; or leaving it running indefinitely, extending dual-running cost and ambiguity. Define an explicit validation window and decommission only after parity is proven under real load.

## Self-Check

- [ ] Each workload was dispositioned individually (rehost, replatform, refactor, repurchase, retain, retire) based on its business value, shape, dependencies, risk, and effort — not one strategy for the whole estate — and retain/retire were considered where migration cost exceeds value.
- [ ] A dependency map (upstream and downstream, including shared services, identity, networking, data stores) was built before sequencing; foundational and shared services move first, each wave is dependency-coherent, and high-risk workloads are sequenced later.
- [ ] The cutover strategy matches risk and reversibility: phased (by tenant, region, or feature) is the default for workloads of meaningful size or criticality, big-bang is reserved for workloads that cannot dual-run, and rollback is defined and rehearsed including handling of data written to the new system during a failed window.
- [ ] Data migration is staged (initial bulk load plus ongoing replication via CDC, log shipping, or dual-writes) so the target is near-current at cutover, dual-running has a defined window and budget, and schema transformation is reconciled (row counts, checksums, business queries) before declaration.
- [ ] Parity is validated on both functional (behavior across the full surface including edge cases, batch jobs, integrations) and non-functional (load-tested performance, capacity, failover, observability) dimensions under real traffic, and the source is retained through an explicit validation window before decommissioning.
- [ ] Cloud cost and operations adaptation is part of the plan: the workload is right-sized to cloud instance families, adopts managed services for heavy-operational components, and uses cloud-native backup/DR/observability — or the decision to defer is explicit, budgeted, and scheduled.
- [ ] Pure lift-and-shift, where chosen, has an explicit replatform/refactor follow-on that is planned and resourced, not a "later" that becomes never; the migration does not merely relocate hosting while leaving the cost and toil of the on-premises posture intact.
