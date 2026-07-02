---
name: strangler_fig_and_incremental_rewrite.md
description: Use when the agent is replacing a legacy system, choosing between rewrite and incremental migration, applying the strangler fig pattern via routing or facades or wrapping, validating parallel run output, defining switchover and rollback criteria, or deciding when to stop maintaining the old system.
---

# Strangler Fig and Incremental Rewrite

Replacing a legacy system is one of the highest-risk endeavors in software. The classic failure mode is the big-bang rewrite: a team spends a year or more building the new system in parallel, and when they cut over, they discover the new system does not match the old behavior, the migration cannot be reversed, and the business loses months. The strangler fig pattern is the alternative: incrementally build the new system alongside the old, route traffic to it piece by piece, validate that it behaves correctly, and let it gradually replace the old until the old can be removed. The pattern works because every step is small, reversible, and verifiable—but only if applied with discipline.

The judgment problem is deciding whether to rewrite or refactor, choosing the routing/facade mechanism that lets you migrate incrementally, validating the new system against the old without a big-bang cutover, and knowing when to cut over, when to roll back, and when the old system can finally be removed. The agent should not treat "rewrite it" as a strategy; the strategy is the incremental path and its validation.

This skill applies whenever you are replacing a legacy system, migrating to a new architecture, or planning a large-scale rewrite.

## Core Rules

### Decide rewrite vs refactor based on the cost of change

Before choosing a rewrite, assess whether the existing system can be evolved:

- **Refactor incrementally** when the existing code is structurally improvable, the team understands it, and the desired changes are localized. Refactoring preserves working behavior and spreads risk over time.
- **Rewrite (incrementally, via strangler)** when the existing system's architecture fundamentally cannot support the new requirements, the cost of every change is prohibitive, or the technology is unsupported. A rewrite is justified by the cost of continued evolution, not by dislike of the old code.

A big-bang rewrite is almost never the right choice; an incremental rewrite (strangler) captures the benefits of a rewrite while controlling risk.

### Introduce a routing/facade layer as the migration seam

The strangler pattern requires a seam where you can decide, per request or per feature, whether to route to the old or new system:

- **Routing/facade**: place a layer (API gateway, reverse proxy, facade service) in front of both systems. Incoming requests are routed to the old system by default; as each capability is migrated, routing for that capability is shifted to the new system.
- **Wrapping**: wrap the legacy system behind a new interface, then replace the implementation behind the interface incrementally. Useful when you cannot change the entry points.
- **Branching by abstraction**: within a single codebase, abstract the capability behind an interface and switch implementations behind it, migrating callers one at a time.

The seam is what makes the migration incremental and reversible. Without it, you are forced into a big-bang cutover.

### Migrate incrementally, one capability at a time

Do not migrate the whole system at once. Migrate one capability (one endpoint, one domain, one report), validate it thoroughly, then move to the next:

- Start with a low-risk, well-understood capability to prove the migration path and the validation approach.
- Migrate read paths before write paths where possible, because read mismatches are easier to detect and recover from than write mismatches.
- Keep each migration step small enough that it can be validated and, if necessary, reversed independently.

Each migrated capability should be independently routable, so a problem in one does not block the others.

### Validate with parallel run (shadowing) before cutover

Before routing production traffic to the new system for real, validate that it produces the same results:

- **Shadow/parallel run**: send each request to both old and new systems, serve the response from the old, and compare the new system's response asynchronously. Differences reveal behavioral mismatches without affecting users.
- **Reconciliation**: for write paths, reconcile the new system's state against the old (or against an independent source of truth) to detect divergence.
- **Differential testing**: feed the same inputs to both and assert on equivalent outputs, accounting for acceptable differences (ordering, formatting, non-determinism).

A capability is not ready to cut over until parallel run shows acceptable equivalence over sufficient volume and edge cases.

### Define explicit switchover and rollback criteria

Before flipping routing to the new system, decide in advance:

- **Switchover criteria**: what metrics must hold to proceed (error rate, latency, correctness compared to shadow). Define the threshold for "good enough."
- **Rollback criteria**: what metrics trigger an immediate revert to the old system. Define the abort conditions and make rollback fast (a routing flip, not a redeploy).
- **Incremental rollout**: shift a small percentage of traffic first (canary), monitor, then increase. Never cut over 100% on the first step.

Rollback must be a routing change, not a code redeploy, so it is fast. If rollback requires redeploying, the migration is not reversible enough.

### Handle data migration as part of the capability

Migrating code without migrating data leaves the new system dependent on the old system's database, which prevents removing the old system. Plan data migration:

- **Dual-write**: write to both old and new stores during migration, so the new store is populated.
- **Backfill**: migrate historical data in bulk, then dual-write for ongoing changes.
- **Cutover**: once the new store is authoritative, stop writing to the old.

Data migration is often the slowest part and the part that determines when the old system can finally be removed.

### Define when the old system can be removed

The migration is not complete until the old system is gone. Define the end state:

- All traffic routes to the new system.
- All data is migrated and the old store is read-only or decommissioned.
- The old code is deleted, not left running "just in case."

A common failure is leaving the old system running indefinitely because nobody is sure it is safe to remove. Set a target date and a checklist for decommission.

## Common Traps

### Big-bang rewrite

Building the new system entirely in parallel and cutting over in one step maximizes risk: the new system is untested at production scale, the cutover cannot be reversed easily, and behavioral mismatches are discovered too late. Prefer incremental migration.

### Migrating without a routing seam

Without a facade or routing layer that can shift traffic per-capability, you cannot migrate incrementally or roll back per-capability. The seam must come first.

### Cutting over without parallel-run validation

Routing production traffic to the new system without shadow/parallel validation discovers behavioral mismatches in production, affecting users. Validate equivalence first.

### No rollback plan, or rollback that requires redeploy

If the new system fails in production and rollback means redeploying the old system, the outage is long. Rollback should be a fast routing change.

### Migrating write paths before read paths

Write mismatches corrupt data and are hard to recover from; read mismatches are visible and recoverable. Migrate reads first to validate behavior, then writes.

### Leaving data in the old store

If the new system still reads from or writes to the old database, the old system can never be removed. Plan and complete data migration.

### Never decommissioning the old system

Leaving the old system running "just in case" means you maintain two systems forever, doubling cost and complexity. Define and execute decommission.

### Migrating everything at the same granularity

Migrating all capabilities in one batch prevents independent validation and rollback. Migrate one capability at a time.

## Self-Check

- Was the decision to rewrite (vs refactor) justified by the cost of evolving the existing system, rather than by preference?
- Is there a routing/facade seam that allows per-capability routing between old and new systems?
- Is the migration incremental, one capability at a time, starting with low-risk capabilities and read paths before write paths?
- Is each capability validated via parallel run/shadowing or reconciliation before production traffic is routed to the new system?
- Are switchover criteria (what must hold to proceed) and rollback criteria (what triggers immediate revert) defined in advance?
- Is rollback a fast routing change rather than a code redeploy?
- Is traffic shifted incrementally (canary first, then increasing), never 100% on the first step?
- Is data migration planned (dual-write, backfill, cutover) so the new system does not depend on the old store?
- Is there a defined end state with a checklist and date for decommissioning the old system, rather than leaving it running indefinitely?
- Have you confirmed that each migrated capability can be validated and rolled back independently of the others?
