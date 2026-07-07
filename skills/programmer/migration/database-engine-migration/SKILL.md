---
name: database_engine_migration.md
description: Use when the agent is migrating a database from one engine to another (Postgres to MySQL, MongoDB to Postgres, self-hosted to managed/cloud like RDS/Aurora/Cloud Spanner), translating SQL dialects, porting stored procedures and triggers, setting up replication during cutover, running dual-write or dual-read verification, comparing query planners and indexing behavior, or assessing whether two databases with "the same SQL" actually behave the same under load.
---

# Database Engine Migration

A database engine migration is the replacement of the system that stores and serves your data, not merely a change to the data itself. Moving from one engine to another — Postgres to MySQL, MongoDB to a relational store, a self-hosted cluster to a managed cloud database — is a migration of the query language, the execution model, the operational contract, and the failure characteristics all at once. The database is the part of the system where assumptions are most deeply buried and most expensive to discover wrong: an index that worked on one engine silently degrades on another, a transaction isolation level that meant one thing means something subtly different, a query plan that was stable becomes unstable under a different cardinality estimator. Getting the data copied is the easy part; getting behavior to survive the change is the hard part.

Agents tend to treat a database engine migration as "export, transform, import," optimizing for moving the bytes rather than for the behavioral differences that break production. The migration runs against a sample, the queries return the right rows, and then it fails on production because the new engine's query planner chose a catastrophic plan for a large table, because a stored procedure's semantics differ, because replication lag during dual-write produced read-after-write inconsistency, or because "SQL is SQL" concealed a difference in null handling, type coercion, or transaction visibility that only surfaced under concurrency. The judgment problem is to treat the engine as a behavioral contract to be replicated and verified at scale, to design a cutover that keeps both engines consistent during transition, and to discover the behavioral differences before users do.

This is distinct from data migration safety, which concerns transforming data correctly and reversibly within or between stores. Here the concern is the engine itself: its dialect, its query planner, its operational characteristics, and the cutover of which database serves the system.

## Core Rules

### Never Assume "SQL Is SQL" — Audit Dialect And Behavioral Differences First

Two databases that both speak SQL are not the same database. Each engine has its own dialect, its own type system, its own null and coercion semantics, its own transaction isolation guarantees, its own defaults for case sensitivity, collation, rounding, and timestamp handling. A migration that treats the engines as interchangeable because the queries "look the same" will produce data that is subtly wrong or queries that fail under conditions the sample did not exercise.

Before migrating, audit the behavioral surface systematically:

- **Dialect and syntax** — every query, view, and migration script must be translated, not assumed portable. Window functions, CTEs, JSON operators, upserts, and pagination syntax differ across engines.
- **Type semantics** — null handling, boolean representation, numeric precision, datetime/timezone storage, string collation, and implicit coercion differ. A query that relied on the old engine's coercion may return different rows or fail on the new.
- **Transaction and isolation behavior** — the meaning of a given isolation level (read committed, repeatable read, serializable) is not identical across engines. Snapshot visibility, locking, and deadlock behavior differ, and concurrency bugs surface only under load.
- **Stored procedures, triggers, and functions** — these are engine-specific code that must be rewritten, not copied. Their semantics (error handling, transaction scope, recursion limits) rarely translate directly.

The audit is the work that prevents the migration from shipping a system that looks correct but behaves differently. Treat every query and every procedural object as a thing to verify, not a thing to assume.

### Verify Behavioral Parity At Scale, Not Just Row Counts

A migration that confirms the new engine returns the same rows as the old for a sample query has verified almost nothing. Behavioral parity means the new engine produces the same results, at the same performance, under the same concurrency, for the full range of queries and data shapes the system actually executes. Differences hide at the edges: large tables, skewed data, concurrent writes, complex joins, and queries whose performance depends on a specific plan the new engine may not choose.

Verify parity across the dimensions that matter:

- **Correctness parity** — run the full production query set against both engines on production-shaped data and compare results row-by-row, including ordering, null handling, and type representation. A count match is not a value match.
- **Performance parity** — measure query latency and resource use on production-scale data, because the new engine's query planner may choose a plan that is correct but catastrophically slow for a large table or a skewed distribution. An index that worked on one engine may not be used, or may not exist in equivalent form, on the other.
- **Concurrency parity** — exercise the real transaction and locking patterns under realistic concurrency, because isolation and locking differences surface only when multiple writers contend.

The goal is to find the divergence before cutover, when it is a finding to fix, rather than after, when it is an incident. Parity verified only on sample data and single-threaded queries is parity that has not been verified.

### Keep Both Engines Consistent During Cutover With Dual-Write Or Dual-Read

During the transition, the system must serve correct data from one engine while the other is being populated and validated, and the two must stay consistent so that the switch is a change of source, not a change of truth. The mechanism is dual-write or dual-read with reconciliation, designed so that the new engine reaches and maintains parity with the old before it carries production consequences.

Cutover consistency patterns:

- **Dual-write** — the application writes to both engines for every mutation, so the new engine stays current as the old continues to serve reads. The risk is write divergence when one write fails; design for detection and reconciliation, and decide which engine is authoritative on conflict.
- **Replication** — set up replication from the old engine to the new (or via a transformation layer), so the new is continuously populated without application-level dual-write. Verify replication lag is bounded and that transformations preserve semantics.
- **Dual-read with shadow comparison** — read from the new engine while still trusting the old, compare results, and alert on divergence. This detects correctness drift in real traffic without exposing users to it.
- **Reconciliation jobs** — periodically compare the two engines (counts, checksums, sampled row comparison) to catch drift that real-time comparison misses, especially for data touched by edge-case paths.

The cutover is safe to complete only when the new engine has demonstrated sustained parity with the old under real load. Switching the source of truth before parity is sustained is switching on faith.

### Account For Operational And Performance Differences Of The New Engine

A new engine is a new operational system: its capacity profile, its scaling model, its failure modes, its observability, and its maintenance procedures are all different, and the team must be prepared to operate it before it carries critical load. A migration that replicates the old engine's provisioning, monitoring, and runbooks onto the new engine will discover that the assumptions do not transfer.

Address the operational transition:

- **Provisioning and scaling** — the new engine's CPU, memory, IOPS, and storage characteristics differ; size it based on the new engine's behavior under production load, not the old engine's numbers. Managed/cloud engines have different scaling and burst behavior than self-hosted.
- **Query planner and indexing** — re-examine the indexing strategy for the new engine, because the optimal indexes and the planner's choices differ. An index that was load-bearing on the old engine may be unused or harmful on the new.
- **Observability** — rebuild monitoring, slow-query analysis, and alerting for the new engine's metrics and tooling, because the old engine's dashboards measure the old engine's reality.
- **Operational runbooks** — backup, restore, failover, vacuum/reclaim, connection pooling, and incident response all differ. The team must be able to operate the new engine under pressure before relying on it.

An engine the team cannot operate is an engine that cannot safely be the source of truth. Operational capability is a prerequisite for cutover, not a task for after it.

## Common Traps

### "SQL Is SQL" — Treating Engines As Interchangeable

Two SQL databases differ in dialect, types, null handling, isolation, and coercion. A migration that assumes portability ships queries that look right and behave differently. Audit and translate the full behavioral surface; verify, do not assume.

### Verifying Parity Only On Sample Data And Single Queries

Row counts and sample-query correctness do not establish parity. Differences hide in large tables, skewed data, concurrency, and query plans. Verify correctness, performance, and concurrency parity on production-scale data and the full query set.

### A Query Plan That Is Correct But Catastrophically Slow

The new engine may produce the same results with a plan that scans where the old indexed, degrading from milliseconds to minutes under production scale. Measure performance parity on production-scale data and re-examine indexing for the new engine's planner.

### Dual-Write Without Reconciliation Hiding Silent Divergence

Dual-writing to both engines without comparing them lets the engines drift undetected, so the cutover switches to an engine that has been wrong for weeks. Pair dual-write with reconciliation and shadow comparison, and treat unbounded divergence as a cutover blocker.

### Replication Lag Causing Read-After-Write Inconsistency

If reads move to the new engine before replication has caught up to a just-completed write, users see stale data. Bound and monitor replication lag, and keep reads on the authoritative engine until lag is proven negligible.

## Self-Check

- [ ] The full behavioral surface was audited — dialect, type semantics, null/coercion handling, transaction isolation, collation, and stored procedures — and every query and procedural object was translated and verified, not assumed portable because both engines speak SQL.
- [ ] Behavioral parity was verified at production scale across correctness (row-by-row result comparison including ordering and nulls), performance (latency and plan quality on production-scale data), and concurrency (real transaction and locking patterns), not merely sample row counts.
- [ ] Indexing strategy was re-examined for the new engine's query planner, and no query silently degrades from indexed to scanning under production scale.
- [ ] During cutover, both engines are kept consistent via dual-write, replication, or dual-read with reconciliation, and the new engine demonstrated sustained parity under real load before becoming the source of truth.
- [ ] Replication or dual-write lag is bounded and monitored, and reads do not move to the new engine in a way that produces read-after-write inconsistency.
- [ ] The new engine is provisioned, monitored, and instrumented based on its own characteristics, not the old engine's numbers, with observability rebuilt for its metrics and tooling.
- [ ] Operational runbooks (backup, restore, failover, reclaim/vacuum, connection pooling, incident response) exist for the new engine, and the team can operate it under pressure before it carries critical load.
- [ ] No behavioral difference (isolation level, null handling, coercion, procedure semantics) was discovered only after cutover affected users.
