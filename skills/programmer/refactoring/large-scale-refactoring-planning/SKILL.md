---
name: large_scale_refactoring_planning.md
description: Use when the agent is planning or executing a large-scale refactoring — migrating a whole codebase to a new pattern, replacing a core library or framework, restructuring an architecture, splitting or merging services, or any refactoring too large to do in one commit; using the strangler fig pattern, branch-by-abstraction, parallel change, feature flags for refactoring, or staged migration; or managing the risk, coordination, and rollback of a refactoring that spans many files, teams, or services. Covers refactoring patterns for large changes (strangler fig, branch-by-abstraction, parallel change/expansion-contraction), staging and sequencing, coexistence of old and new, rollback planning, team coordination, and the discipline of keeping the system working at every step rather than big-bang rewrites.
---

# Large Scale Refactoring Planning

A large-scale refactoring — replacing a core library across a codebase, migrating from one framework to another, splitting a monolith into services, or restructuring an architecture — differs from an everyday refactoring in a way that determines its success or failure: it cannot be done in one commit. A change spanning hundreds of files, multiple teams, or several services, done as a big-bang rewrite (stop the world, rewrite everything, switch over), fails at a high rate: it takes longer than estimated (surprise at the scope), the old and new drift (the old system evolves during the rewrite, and the new is stale at completion), it cannot be rolled back (the cutover is a point of no return), and the organization loses confidence (the team disappears into the rewrite while the product stalls). The disciplined alternative is incremental migration: the new system is built alongside the old, traffic or usage is migrated gradually (the strangler fig pattern), the old and new coexist during the transition (allowing comparison and rollback), and the system is working and shippable at every step. The refactoring is a campaign, not a battle, planned and sequenced to minimize risk and preserve the ability to ship and roll back throughout.

Agents tend to approach large refactorings as big-bang rewrites (underestimating the scope and the drift risk), to lack a rollback plan (the cutover is irreversible), and to not sequence the migration (trying to do everything at once). The judgment problem is recognizing that large refactorings must be incremental to be safe, that specific patterns (strangler fig, branch-by-abstraction, parallel change) enable incremental migration of even fundamental changes, and that keeping the system working and shippable at every step is the key risk management. This skill covers the discipline of large-scale refactoring planning: the incremental patterns, staging and sequencing, coexistence, rollback, coordination, and the campaign mindset.

## Core Rules

### Use Incremental Migration Patterns, Not Big-Bang Rewrites

The foundational principle is that large refactorings are done incrementally, with the new system growing alongside the old, using patterns that enable gradual migration.

- **Strangler fig pattern: build the new system alongside the old, route traffic gradually, decommission the old when migration is complete.** Named for the strangler fig vine that grows around a host tree and eventually replaces it, this pattern builds the new system incrementally, routes individual features or endpoints to it (via a router or facade), and decommissions the old system piece by piece. At every point, the system works (some features on the new, some on the old), and the migration can pause or roll back.
- **Branch-by-abstraction: introduce an abstraction layer, migrate callers to it incrementally, swap the implementation behind it.** When replacing a component used in many places, introduce an abstraction (interface) that both the old and new implementations satisfy, migrate callers to use the abstraction (one at a time), then swap the implementation behind the abstraction. The abstraction allows incremental migration without a big-bang cutover.
- **Parallel change (expand and contract): add the new alongside the old (expand), migrate callers to the new (migrate), remove the old (contract).** An API or component change that would break callers is done in three phases: add the new (callers unaffected), migrate callers one by one to the new (both work during migration), then remove the old (now-unused). No breaking change happens until all callers are migrated.
- **Never big-bang rewrite unless there is no alternative.** A big-bang rewrite (stop the world, rebuild, switch over) is the highest-risk approach: long-running, drift-prone, irreversible. Use it only when incremental migration is impossible (the old system is fundamentally unmaintainable, or the new is fundamentally incompatible), and even then, scope the rewrite as small as feasible.

### Keep The Old And New Coexisting During Migration

During an incremental migration, the old and new systems coexist. Design for coexistence: data synchronization, traffic routing, and behavioral parity.

- **Route traffic or usage between old and new via a router, facade, or feature flag.** A router (for requests), a facade (for library calls), or a feature flag (for code paths) directs each unit of work to the old or new system. This allows gradual migration (route a few units to the new, verify, route more) and rollback (route back to the old).
- **Synchronize data if old and new have separate stores.** If the migration involves a new database or data format, the old and new stores must be synchronized during the transition (dual-writes, or a migration with cutover). Data synchronization is a common source of migration bugs; design and test it carefully. See data-replication-and-consistency.
- **Verify behavioral parity between old and new.** The new system should produce the same behavior as the old (for the migrated parts); discrepancies are migration bugs. Compare outputs (shadow traffic: run both, compare results), run the same tests against both, and monitor for divergences during migration.
- **Plan the coexistence period's duration.** Coexistence has a cost (maintaining two systems, synchronization complexity); it should not be indefinite. Plan the migration timeline; complete the migration and decommission the old within a reasonable period.

### Sequence And Stage The Migration To Manage Risk

A large refactoring is sequenced into stages, each delivering value and reducing risk, rather than attempted all at once.

- **Break the refactoring into independent, shippable stages.** Each stage (migrate one service, replace one library, route one feature to the new) is independently shippable and verifiable. Stages build on each other but each leaves the system working.
- **Sequence stages by risk and dependency.** Migrate the lowest-risk, highest-learning stages first (to validate the approach), and the highest-risk stages later (when the approach is proven). Respect dependencies (a stage that depends on another's completion comes after it).
- **Ship and verify each stage before proceeding.** Each stage is shipped to production, monitored, and verified before the next begins. A stage that reveals problems (the new system is slower, data sync is buggy) is addressed before more migration is built on a flawed foundation.
- **Make each stage reversible.** A stage that can be rolled back (route traffic back to the old) is safe to attempt; a stage that cannot (a data migration that cannot be undone) is high-risk. Prefer reversible stages; design irreversible stages with extreme care and fallback plans.

### Plan Rollback At Every Stage

A large refactoring must be rollable back at every stage, not just at the end. An irreversible migration is a point of no return that risks the system.

- **Design each stage for rollback.** For each stage, define how to roll it back (route traffic back, restore the old code, restore the old data). If a stage cannot be rolled back, it is a high-risk commitment that must be explicitly acknowledged and planned.
- **Test the rollback before you need it.** A rollback plan that has not been tested fails when invoked (the old code does not work, the data cannot be restored). Test the rollback (in staging, or via a canary rollback) before relying on it.
- **Set rollback criteria: when to roll back.** Define the conditions that trigger a rollback (error rate above X, latency above Y, a specific failure). Having criteria prevents debate under pressure (roll back when the criteria are met, do not deliberate during an incident).
- **For data migrations, plan the data rollback specifically.** Data migrated to a new format may not be trivially reversible (the new format may have fields the old lacked). Plan the data rollback (keep the old data until the migration is confirmed, or design the migration to be reversible) or accept the irreversibility deliberately.

### Coordinate Across Teams And Communicate The Plan

A refactoring spanning multiple teams or services requires coordination and communication that an individual refactoring does not.

- **Define the target architecture and the migration path clearly.** Everyone involved must understand where the refactoring is going (the target state) and how it gets there (the migration stages). A shared design document communicates this.
- **Coordinate dependencies across teams.** A stage that depends on another team's work (a service they must update, an interface they must adopt) requires coordination: agree on the interface, the timeline, and the dependency order. Uncoordinated dependencies stall the migration.
- **Communicate the plan, progress, and changes.** Regular communication (a migration tracking doc, status updates, announcements of traffic shifts) keeps everyone informed. A migration that is invisible to the rest of the organization surprises people (a team's service is suddenly routed to the new system without notice).
- **Assign clear ownership of the refactoring.** A large refactoring needs an owner (a person or team) responsible for the plan, the coordination, and the progress. A refactoring owned by everyone is owned by no one and stalls.

### Measure And Verify The Refactoring's Success

A large refactoring must be verified to have achieved its goals (performance, maintainability, correctness) without regressions.

- **Define success criteria upfront.** What does success look like (the new system is faster by X, the codebase is simpler by Y, the new framework is adopted by all services)? Define measurable criteria; verify them at completion.
- **Monitor for regressions during and after migration.** A refactoring can introduce subtle regressions (a performance change, a behavioral difference, a new failure mode). Monitor metrics (latency, error rate, resource usage) during migration and after completion; compare to the baseline.
- **Verify behavioral parity where required.** For migrations where the new must match the old (a library replacement, a service split), verify parity (shadow comparison, test suites, monitoring). A migration that changes behavior silently is a bug.
- **Decommission the old system to complete the refactoring.** A refactoring is not complete until the old system is removed; an old system that lingers (the "we'll get to it" decommission) retains the maintenance cost and the confusion of two systems. Plan and execute the decommission as a final stage.

## Common Traps

### Big-Bang Rewrite

Stopping the world to rewrite everything, risking drift, delay, and irreversibility. Use incremental patterns (strangler fig, branch-by-abstraction, parallel change).

### No Rollback Plan

A migration with no way to roll back, making the cutover a point of no return. Design each stage for rollback; test the rollback.

### Old And New Not Coexisting

A migration that requires the old to be removed before the new is complete, preventing gradual migration and comparison. Design for coexistence (routing, data sync, parity verification).

### Stages Not Independent Or Shippable

A refactoring staged such that no stage is shippable until the whole is complete, defeating incremental migration. Break into independent, shippable stages.

### Uncoordinated Cross-Team Dependencies

A stage depending on another team's work without coordination, stalling the migration. Coordinate dependencies; agree on interfaces and timelines.

### Coexistence Dragging On Indefinitely

The old and new coexisting forever (the old never decommissioned), retaining double maintenance and confusion. Plan the coexistence duration; decommission the old as a final stage.

### No Success Criteria Or Regression Monitoring

A refactoring completed without verifying it achieved its goals or monitoring for regressions. Define success criteria; monitor metrics; verify parity.

### Data Migration Not Reversible

A data migration that cannot be undone, making the cutover irreversible. Plan the data rollback or accept irreversibility deliberately with fallback plans.

## Self-Check

- [ ] The refactoring uses incremental migration patterns (strangler fig for system replacement, branch-by-abstraction for component replacement, parallel change/expand-migrate-contract for API changes) rather than a big-bang rewrite, keeping the system working and shippable at every step.
- [ ] The old and new systems coexist during migration: traffic or usage is routed between them (via router, facade, or feature flag), data is synchronized if stores are separate, behavioral parity is verified (shadow comparison, test suites, monitoring), and the coexistence period has a planned duration.
- [ ] The refactoring is sequenced into independent, shippable stages, ordered by risk and dependency (low-risk learning stages first, high-risk stages later, dependencies respected), with each stage shipped, verified, and reversible before the next proceeds.
- [ ] Rollback is planned at every stage: each stage has a defined rollback (route back, restore old code/data), the rollback is tested before it is needed, rollback criteria are defined (when to roll back), and data migrations specifically plan for reversibility or accept irreversibility deliberately.
- [ ] Cross-team coordination is in place: the target architecture and migration path are documented and shared, dependencies across teams are coordinated (interfaces, timelines, order), progress is communicated regularly, and the refactoring has a clear owner.
- [ ] Success criteria are defined upfront (measurable goals: performance, maintainability, adoption), regressions are monitored during and after migration (metrics compared to baseline), behavioral parity is verified where required, and the old system is decommissioned as a final stage to complete the refactoring.
- [ ] The refactoring has been scoped to the minimum necessary (the smallest change that achieves the goal), rather than expanding scope to "while we're at it" changes that inflate risk and duration.
- [ ] The organization's confidence is maintained throughout: the system ships continuously during the refactoring (no long freeze), progress is visible (stages shipped and announced), and the refactoring does not become a multi-quarter disappearance that erodes trust — because a large refactoring is a campaign managed for sustained trust, not a gamble.
