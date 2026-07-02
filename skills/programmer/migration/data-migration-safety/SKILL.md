---
name: data_migration_safety.md
description: Use when the agent is migrating data between schemas, databases, formats, or storage systems, running a backfill, changing a column type or constraint, planning a migration's rollback and downtime, sizing migration batches, handling foreign keys during migration, or testing a migration before running it against production data.
---

# Data Migration Safety

A data migration is a change to data that already exists and that the business already depends on, and that combination is what makes it uniquely dangerous. Code can be rolled back to a previous version; a feature can be turned off. But data, once transformed, often cannot be un-transformed. A migration that corrupts, duplicates, or loses records does so to the only copy of the truth, and the damage may be invisible — silent missing rows, subtly wrong values, broken foreign keys — discovered only when a report is wrong or a customer complains, by which point the backup window may have closed and the clean state is gone forever.

Agents tend to treat a data migration as a script to write and run, optimizing for getting the data moved rather than for surviving the failure modes that actually occur. The migration runs on a representative sample, looks fine, and then fails or corrupts on production because the production data has edge cases the sample lacked, because the batch was too large and locked the table, because a constraint fired mid-migration, or because the rollback that was assumed to exist does not actually reverse the transformation. The judgment problem is to design a migration that is verifiable before it runs, incremental and reversible while it runs, and validated after it runs — so that a failure at any point is contained, detectable, and recoverable rather than catastrophic.

## Core Rules

### Back Up And Make The Migration Reversible Before You Start

Before any transformation touches production data, ensure two things exist: a backup you can restore from, and a rollback path that reverses the migration's effect. These are not optional ceremony; they are the only things standing between a mistake and permanent data loss. A migration without a rollback is a one-way operation whose failure mode is manual data repair under pressure.

- **Take a verified backup** immediately before the migration, and confirm it can be restored (an untested backup is a hope, not a backup). Know the backup's point-in-time and how to restore to it.
- **Design the migration to be reversible**, meaning there is a concrete, tested operation that returns the data to its pre-migration state. Some transformations are inherently irreversible (a one-way hash, a destructive merge, a column drop); for these, reversibility means "restore from backup," which must be feasible within your downtime window.
- **Test the rollback**, not just the forward migration. A rollback that has never been run will fail when you need it, usually because of a detail (a constraint, a dependent object) that only surfaces under execution.

If the data is valuable enough that losing it would harm the business, the migration is not safe to run without these in place.

### Migrate Incrementally, Never In One Big-Bang Step

A migration that transforms all records in a single operation concentrates all risk at one moment: if it fails at 80%, you have a partially migrated dataset with no clean state, and resuming or rolling back is a bespoke problem. An incremental migration transforms the data in small, verifiable batches, each of which can succeed, fail, and be retried or rolled back independently.

Design for incremental execution:

- **Process in batches** small enough that a single batch's failure is contained and its retry is cheap. The batch size is constrained by locking behavior, transaction size limits, and the need to avoid degrading the live system — a batch that locks a large table for minutes is a batch that takes down production.
- **Make each batch idempotent**, so that re-running it after a failure does not double-apply the transformation. Idempotency lets you retry safely without manual cleanup of partial state.
- **Track progress persistently**, so a resumed migration knows where it stopped rather than re-processing or skipping records. A migration whose progress lives only in memory is a migration that loses its place on any interruption.
- **Verify between batches**, checking counts, checksums, or sample values so a drift is caught early rather than after the whole dataset is transformed.

Incremental migration turns a single high-risk operation into a sequence of low-risk ones, each of which is independently safe.

### Handle Constraints, Foreign Keys, And Dependencies Deliberately

Data does not exist in isolation; it is connected by constraints, foreign keys, triggers, and application-level invariants that the migration must respect or temporarily relax. A migration that ignores these fails mid-execution (a constraint violation halts the batch) or, worse, succeeds at the database level while breaking an application invariant that corrupts logic downstream.

For each migration, map the dependencies:

- **Foreign keys** — does the migration change a key that other tables reference? Changing a referenced key requires either cascading the change, dropping and recreating the constraint, or a multi-step migration that keeps both keys valid during transition.
- **Constraints** (unique, check, not-null) — will the transformed data satisfy existing constraints? A migration that produces data violating a constraint will fail when the constraint is enforced; decide whether to validate before enforcing, or to relax and re-tighten in steps.
- **Triggers and application invariants** — does the migration bypass logic that normally maintains consistency? A backfill that writes directly to a table may skip a trigger that maintains a derived field, leaving the data internally inconsistent.
- **Schema coupling** — is the migration tied to a schema change, and if so, must the schema change and the data change happen together or can they be decoupled for safety?

A common safe pattern for constraint changes: add the new column or constraint in a non-enforcing state, backfill the data, validate that all rows satisfy the constraint, then enforce it — each step separately deployable and reversible. Forcing the schema and data change in one step is where constraint violations take down migrations.

### Validate The Migration Against Production-Like Data Before Running

A migration that works on a small, clean sample often fails on production data, because production has scale, edge cases, nulls, duplicates, and historical anomalies the sample lacked. The migration must be validated against data that resembles production in size and messiness before it is trusted on production.

Validation practices:

- **Run against a production-sized copy**, not a toy sample, so performance and locking behavior are realistic. A migration that takes a second on a thousand rows may lock a table for an hour on a billion.
- **Test the edge cases explicitly** — nulls, empty strings, duplicates, out-of-range values, unicode, very large rows, and any historical anomalies you know the data contains. These are where transformations break.
- **Verify the transformation's correctness** with checksums, counts, and sampled row-by-row comparison between source and target, not just "it ran without error." A migration that runs cleanly but produces wrong values is worse than one that fails loudly.
- **Test the rollback** on the same data, confirming it restores the pre-migration state. A rollback that works on clean data but not on the post-migration state is a rollback that will fail when needed.

### Minimize And Plan For Downtime Explicitly

Every migration has a downtime profile: does it require the system to be unavailable, and if so, for how long? The migration design should make the downtime explicit and minimize it, because unplanned downtime during a migration is how maintenance windows become incidents.

Strategies to reduce or eliminate downtime:

- **Expand-and-contract** — add the new schema alongside the old, migrate the data while both are live, switch reads and writes to the new, then remove the old. This decouples schema change, data migration, and cutover into independently safe steps, often with zero downtime.
- **Dual-write during transition** — write to both old and new stores while migrating, so the new store is kept current and can be verified against the old before the switch.
- **Backfill in the background** — migrate existing records in batches while the system serves traffic, with the application handling the mix of migrated and unmigrated records until the migration completes.

Where downtime is unavoidable, plan it explicitly: schedule it, communicate it, define the success criteria and the abort criteria, and have the rollback ready to execute if the migration does not meet criteria within the window. A migration that drifts past its window with no abort decision is a migration becoming an incident.

### Test The Migration End-To-End, Including The Failure Paths

A migration is not tested by running it forward on clean data; it is tested by exercising the failure paths that will actually occur. Before running on production, rehearse:

- **The forward migration** completing successfully and producing verified-correct data.
- **A mid-migration failure** (kill the process, simulate a constraint error) and confirm the migration can resume or roll back without corruption.
- **The rollback** returning to the pre-migration state, including any schema or constraint state.
- **The cutover** (if applicable) — the application reading from the new data and behaving correctly.

Each of these is a thing that will happen in production if the migration runs enough times. Rehearsing them in advance turns a crisis into a procedure.

## Common Traps

### The Big-Bang Migration With No Resume Or Rollback

Transforming all data in one operation concentrates risk and leaves no clean state on failure. Migrate incrementally in idempotent batches with tracked progress, so failure is contained and recovery is a retry or a partial rollback.

### An Untested Backup Or Untested Rollback

A backup that has never been restored, or a rollback that has never been run, will fail when needed. Verify the backup can be restored and test the rollback on production-like data before relying on either.

### Batches That Lock Or Degrade Production

A batch large enough to lock a hot table takes down the live system. Size batches to the locking and performance characteristics of the production schema, and migrate in a way that does not block serving traffic.

### Ignoring Constraints And Foreign Keys

A migration that violates a constraint fails mid-run, and one that bypasses triggers or invariants leaves the data internally inconsistent. Map and handle constraints, foreign keys, and application invariants deliberately, often with expand-and-contract sequences.

### Validating Only On Clean Sample Data

A migration that works on a small, clean sample fails on production's scale, nulls, duplicates, and anomalies. Validate against production-sized, messy data, and verify correctness with counts and sampled comparison, not just absence of errors.

### Assuming Success From "It Ran Without Error"

A clean run does not mean correct data. A migration can complete while producing wrong values, duplicates, or missing rows. Verify the result with checksums, counts, and row-level sampling against the source.

### Coupling Schema, Data, And Cutover In One Step

Combining the schema change, the data transformation, and the application cutover into one operation maximizes the blast radius of any failure. Decouple them with expand-and-contract so each step is independently safe and reversible.

## Self-Check

- [ ] A verified, restorable backup exists immediately before the migration, and the rollback path is concrete and has been tested (not just assumed to reverse the transformation).
- [ ] The migration runs incrementally in small, idempotent batches with persistently tracked progress, so a failure is contained, resumable, and independently rollback-able.
- [ ] Batch sizes respect production locking and performance characteristics and do not degrade or block the live system.
- [ ] Foreign keys, constraints, triggers, and application invariants are mapped and handled deliberately, often via expand-and-contract (add non-enforcing, backfill, validate, enforce) rather than forced in one step.
- [ ] The migration was validated against production-sized, messy data with explicit edge cases (nulls, duplicates, anomalies), and correctness was verified with counts, checksums, and sampled row comparison — not just absence of errors.
- [ ] Downtime is explicit and minimized (expand-and-contract, dual-write, background backfill where possible), and where unavoidable, the window, success criteria, abort criteria, and rollback are planned in advance.
- [ ] Failure paths were rehearsed — mid-migration failure and resume, rollback to pre-migration state, and application cutover — not only the happy-path forward run.
- [ ] No step of the migration is a one-way, irreversible operation without a backup-and-restore fallback that is feasible within the downtime window.
