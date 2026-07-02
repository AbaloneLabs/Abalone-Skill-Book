---
name: framework_and_platform_migration.md
description: Use when the agent is migrating an application between frameworks, runtimes, or platforms (web framework, cloud provider, message broker, ORM), planning a strangler-fig or parallel-run migration, building a compatibility layer, sequencing a phased cutover, estimating migration cost, or deciding when to retire the old platform.
---

# Framework And Platform Migration

A framework or platform migration is the replacement of a load-bearing part of the system while the system continues to serve its users. Unlike a greenfield build, the old and new must coexist during the transition, the business cannot stop while you migrate, and the migration touches code, data, operations, and team knowledge all at once. The failure mode is well-known: a migration that begins with optimism, discovers the old platform's implicit dependencies halfway through, stalls as the parallel-run cost doubles the maintenance burden, and ends either as an abandoned half-migration or as a panicked big-bang cutover under pressure. Most platform migrations that fail do so not because the new platform is wrong but because the transition was not designed to be survivable.

Agents tend to approach a framework migration as a porting exercise: take the old code, rewrite it on the new platform, switch over. This ignores the properties that make migrations hard — the old platform's hidden coupling, the cost of running two platforms in parallel, the team's unfamiliarity with the new platform, the difficulty of verifying equivalence, and the question of when the old platform can finally be turned off. The judgment problem is to design the migration as a phased, verifiable, reversible transition (most often a strangler pattern), to build compatibility where it reduces risk, to plan the cutover in increments that can each be held or rolled back, and to drive toward a defined end state where the old platform is genuinely retired rather than left running forever as a zombie.

## Core Rules

### Migrate With The Strangler Pattern, Not A Big-Bang Rewrite

The safest way to replace a load-bearing system is incrementally, routing one piece at a time from the old platform to the new while both continue to operate. This is the strangler-fig pattern: the new system grows around the old, taking over function by function, until the old has nothing left to do and can be removed. Each increment is small enough to verify, to hold, and to roll back independently, so a problem in the new platform is contained rather than catastrophic.

The alternative — rebuilding the entire system on the new platform and switching over in one operation — concentrates all risk at the cutover. It can only be verified after the switch, it can only be rolled back by reverting the entire switch, and any defect is a production incident. Big-bang migrations are justified only when incremental migration is truly impossible (a hard platform end-of-life with no coexistence path); in every other case, the strangler's incremental discipline is safer and more likely to complete.

Sequence the increments by risk and value: migrate a low-risk, self-contained piece first to validate the new platform and the migration machinery, then progressively take over higher-value and more coupled pieces, each building confidence. The first increment is a proof that the approach works; the last is the retirement of the old platform's final function.

### Run Old And New In Parallel And Verify Equivalence

During the transition, the old and new platforms coexist, and this coexistence is the opportunity to verify that the new platform behaves equivalently before it carries real consequences. Parallel running — sending the same inputs to both and comparing the outputs, or routing a fraction of traffic to the new while the old continues as the source of truth — lets you detect divergence before it reaches users.

Parallel verification practices:

- **Shadow traffic** — send a copy of real requests to the new platform without affecting users, and compare responses to the old. Divergence surfaces as a difference to investigate, not as a user-facing failure.
- **Dual-write or dual-read** — for stateful systems, write to both stores and reconcile, or read from the new while trusting the old, switching the source of truth only after sustained equivalence.
- **Canary routing** — route a small percentage of real traffic to the new platform, monitor errors and behavior, and increase the percentage as confidence grows, with the ability to route back instantly if something is wrong.

The goal is that no increment of the migration is trusted on faith; each is verified against the old platform's behavior before it carries production consequences. A migration that switches to the new platform without parallel verification is hoping the new platform is correct, and hope is not a verification strategy.

### Build Compatibility Layers Where They Reduce Risk

A compatibility layer — an adapter, a facade, an abstraction that presents the old interface while backed by the new platform — can decouple the migration of the platform from the migration of the code that depends on it. Consumers keep calling the interface they know; the implementation behind it moves to the new platform. This lets the platform change without forcing every consumer to change in lockstep.

Use compatibility layers deliberately:

- **Introduce the abstraction before the migration**, so consumers depend on the interface, not the old platform directly. This is branch-by-abstraction: the seam exists before the swap, making the swap a single, reversible change behind the seam.
- **Keep the layer thin and temporary.** A compatibility layer that grows complex becomes its own maintenance burden and can obscure the new platform's actual behavior. It exists to bridge the transition, then to be removed.
- **Do not let the layer become a permanent crutch.** If consumers never migrate off the old interface, the compatibility layer becomes permanent debt and the migration never truly completes.

The layer is a tool for reducing migration risk, not for avoiding it. Plan its removal as part of the migration's end state.

### Sequence The Cutover In Verifiable, Reversible Increments

The cutover — the point at which the new platform becomes the source of truth — should never be a single all-or-nothing switch. Sequence it so that each increment of traffic, data, or function moves to the new platform independently, is verified, and can be held or reversed.

For each increment:

- **Define success criteria** — error rates, latency, correctness checks — that must hold before proceeding to the next increment.
- **Define the rollback trigger** — which symptoms mean "route this increment back to the old platform" — so the decision is mechanical under pressure.
- **Hold at each stage** long enough to observe behavior under realistic conditions (different traffic patterns, time of day, failure modes) before increasing exposure.

A cutover that proceeds without criteria or holds is a cutover that discovers problems only after they affect all users. Incremental cutover with holds is how you find the problems at 1% rather than at 100%.

### Account For The Team's Learning Curve And Capability

A platform migration is also a migration of team knowledge. The new platform has different idioms, failure modes, operational characteristics, and tooling, and the team must learn these while maintaining the old platform and building the new. Underestimating the learning curve is a common reason migrations stall: the team can maintain the old platform but cannot yet operate the new one confidently, so the cutover is perpetually deferred.

Address the learning curve deliberately:

- **Invest in the team's capability on the new platform before relying on it**, through the early low-risk increments, pairing, and documentation of the new platform's operational behavior.
- **Do not assume the old platform's operational knowledge transfers.** On-call, incident response, capacity planning, and debugging all differ; the team must build new operational muscle before the new platform carries critical load.
- **Plan for the period of doubled operational burden**, when both platforms must be monitored, understood, and staffed. This period is costly and should be minimized by completing the migration rather than letting it drift.

A migration to a platform the team cannot yet operate is a migration that cannot safely cutover. Capability is a prerequisite, not an afterthought.

### Estimate Cost Honestly And Define The End State

Platform migrations are consistently underestimated because the visible work (porting code) is a fraction of the real work (compatibility, parallel running, verification, operational capability, data migration, retiring the old platform). Estimate the full cost — including the doubled maintenance during transition and the often-neglected cost of decommissioning the old platform — before committing, and revisit the estimate as the migration reveals its actual scope.

Define the end state explicitly: what does "done" mean? It should include not only the new platform carrying the load but the old platform being fully retired — its code removed, its infrastructure decommissioned, its costs eliminated. A migration whose end state is "the new platform works" but leaves the old platform running is a migration that has shifted cost, not eliminated it, and the old platform will linger as permanent debt. Drive toward retirement, and treat the old platform's continued existence as an unfinished migration.

## Common Traps

### The Big-Bang Cutover

Switching the entire system to the new platform in one operation concentrates all risk at a single moment, with verification only possible after the switch and rollback only by reverting everything. Use the strangler pattern and incremental cutover so each step is verifiable and reversible.

### Parallel Running That Becomes Permanent

Running old and new indefinitely, because the migration stalls or the old is never retired, doubles the maintenance and operational burden forever. Define the end state as old-platform retirement and drive toward it; parallel running is a transition state, not a destination.

### Skipping Equivalence Verification

Switching to the new platform without parallel-run or shadow comparison trusts the new platform on faith. Verify equivalence (shadow traffic, dual-write, canary) before the new platform carries production consequences.

### A Compatibility Layer That Becomes A Permanent Crutch

An adapter meant to bridge the transition that is never removed becomes permanent debt and prevents the migration from completing. Plan the layer's removal as part of the end state.

### Underestimating The Learning Curve

Migrating to a platform the team cannot yet operate leaves the cutover perpetually deferred because no one is confident running the new system under load. Build operational capability on the new platform before relying on it.

### Estimating Only The Porting Cost

Counting the code port while ignoring compatibility, parallel running, verification, operational burden, and decommissioning produces estimates that are structurally low and migrations that overrun. Estimate the full lifecycle cost, including retiring the old platform.

### Leaving The Old Platform As A Zombie

A migration that gets the new platform working but never decommissions the old leaves doubled cost and permanent debt. The end state includes the old platform's full retirement, not just the new platform's operation.

## Self-Check

- [ ] The migration follows a strangler or incremental pattern, routing one piece at a time from old to new, with each increment small enough to verify, hold, and roll back — not a big-bang rewrite.
- [ ] Old and new run in parallel during transition, with equivalence verified through shadow traffic, dual-write/read reconciliation, or canary routing before the new platform carries production consequences.
- [ ] Compatibility layers were introduced before the migration (branch-by-abstraction), are thin and temporary, and have a planned removal as part of the end state.
- [ ] The cutover is sequenced in verifiable, reversible increments, each with defined success criteria, a rollback trigger, and a hold period to observe behavior under realistic conditions.
- [ ] The team's operational capability on the new platform was built (through early increments, pairing, documentation) before the new platform carried critical load, and the doubled operational burden during transition is planned for and minimized.
- [ ] The cost estimate includes the full lifecycle — porting, compatibility, parallel running, verification, operational capability, data migration, and decommissioning — not only the visible code port.
- [ ] The end state is defined to include full retirement of the old platform (code removed, infrastructure decommissioned, costs eliminated), and the old platform's continued existence is treated as an unfinished migration rather than an acceptable steady state.
