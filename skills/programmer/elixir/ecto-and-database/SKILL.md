---
name: elixir_ecto_and_database.md
description: Use when the agent is working with Ecto (schemas, changesets, validations, queries via Ecto.Query, associations, multi/transactions, migrations, Repo configuration, connection pooling), designing database interactions, writing efficient queries (avoiding N+1, preload vs join, select for projections), handling transactions and `Ecto.Multi`, managing migrations safely (locking, backfills, null constraints), or is diagnosing "N+1 query", "changeset validations vs database constraints", "preload vs assoc/has_many query explosion", "migration locks the table", "transaction rolled back but side effects happened", or pooling/timeout issues. Covers Ecto schemas and changesets, query composition, N+1 and preload/join, Multi transactions, migration safety, constraint-vs-validation, and pooling/timeouts.
---

# Ecto And Database Work In Elixir

Ecto is Elixir's data-mapping and query library, and its design (schemas separate from queries, changesets for validation/casting, explicit preloads) is powerful but easy to misuse. Agents trigger N+1 queries (iterating records and querying associations per-record instead of preloading), confuse changeset validations with database constraints (a `unique_constraint` requires both a DB constraint and the changeset annotation, and the annotation must match the constraint name), preload explosions (preloading deep nested associations when a flat query would do), unsafe migrations (adding a NOT NULL column or an index `CONCURRENTLY`-missing on a large table locks it), and half-committed transactions (a side effect outside the `Repo.transaction` is not rolled back). The judgment problem is to compose queries efficiently (avoid N+1, choose preload vs join), to align changeset validations with DB constraints, to use `Ecto.Multi` for multi-step transactions with per-step error handling, and to write migrations that do not lock the table.

Agents write N+1 queries, mismatch constraint names, lock tables in migrations, or leak side effects past a rollback. The remedy is explicit preloads, aligned constraints, `Ecto.Multi`, and online-safe migrations.

## Core Rules

### Compose Queries Efficiently; Avoid N+1 With Explicit Preloads

An N+1 query happens when you iterate a list of records and query each one's association inside the loop (N records → N+1 queries). Ecto makes this explicit: you must `preload` associations when you fetch the parent (`Repo.all(from u in User, preload: [:posts])`), or the association is `nil`/empty and accessing it later triggers per-record queries. Choose `preload:` (a separate query per association, efficient for large sets) vs `join:`/`assoc:` (a single joined query, good for filtering by association or small sets). For projections, use `select:` to fetch only needed fields (not whole schemas). Use `Repo.aggregate` for counts/sums rather than loading records. Profile slow queries with `Ecto.DevLogger`/logs and the DB's `EXPLAIN`.

- `preload` associations when fetching the parent; otherwise accessing them triggers N+1.
- `preload:` (separate query) vs `join:`/`assoc:` (single query) by use case; `select:` for projections.
- `Repo.aggregate` for counts; profile with logs/`EXPLAIN`.

### Align Changeset Validations With Database Constraints

Ecto changesets have two kinds of checks: *validations* (`validate_required`, `validate_format`, `validate_length`) run in Elixir before the DB call, and *constraints* (`unique_constraint`, `foreign_key_constraint`, `check_constraint`) translate a DB error into a changeset error *after* the insert/update. Critical rules:

- A `unique_constraint` requires an actual DB unique constraint/index, and the constraint *name* passed to `unique_constraint` must match the DB constraint name (or Ecto won't recognize the error and will raise). Check the migration's `unique_index` name.
- Validations that can race (uniqueness, check constraints) must be backed by DB constraints — a `validate_required` is safe in Elixir, but "unique email" must have a DB unique index (two concurrent requests can pass the Elixir check and both insert).
- Use constraints for anything the DB enforces; validations for format/presence Elixir can decide alone.

- `unique_constraint`/`foreign_key_constraint` need matching DB constraints with matching names.
- Race-prone checks (uniqueness) must be DB-enforced, not Elixir-only.
- Validations for format/presence; constraints for DB-enforced integrity.

### Use Ecto.Multi For Multi-Step Transactions With Per-Step Errors

`Ecto.Multi` composes multiple Repo operations (inserts/updates/deletes/`Ecto.Multi.run` for arbitrary effects) into a single transaction that either all succeeds or all rolls back, with errors attributed to the failing step. This is far better than nested `case`/`with` for multi-step DB work: `Multi.new() |> Multi.insert(:user, user_changeset) |> Multi.insert(:profile, fn %{user: u} -> profile_changeset(u) end) |> Repo.transaction()`, then pattern-match on `{:ok, results}` / `{:error, failed_op, failed_value, changes_so_far}`. The `changes_so_far` lets you reason about partial state. Note: side effects *outside* the Multi (sending an email, publishing to a queue) are NOT rolled back if the transaction fails — perform them after a successful commit, or use a pattern (transactional outbox) for atomicity.

- `Ecto.Multi` for multi-step transactions; per-step error attribution and `changes_so_far`.
- Side effects outside the Multi (emails, queue publishes) are not rolled back — do them post-commit.
- Pattern-match `{:ok, results}` / `{:error, failed_op, failed_value, changes_so_far}`.

### Write Migrations That Do Not Lock The Table

Migrations on large tables can lock them (blocking reads/writes) for the duration. Safe patterns:

- Adding a column: default to nullable, or add then backfill in batches (not a single `UPDATE` on millions of rows).
- Adding a NOT NULL column: add nullable, backfill, then set NOT NULL in a later migration (or use `DEFAULT` carefully — some DBs rewrite the table).
- Adding an index: use `create index(..., concurrently: true)` (Postgres `CONCURRENTLY`, no write lock) — but note `CONCURRENTLY` cannot run inside a transaction (Ecto's `@disable_ddl_transaction`).
- Adding a foreign key: add it `NOT VALID` (Postgres) then validate later, to avoid a full table scan lock.
- Large backfills: batch them (`Repo.update_all` in chunks) to avoid long transactions.

Plan destructive migrations (column drops, type changes) carefully; they can be hard to reverse.

- Nullable columns + batched backfill; NOT NULL in a follow-up migration.
- `CONCURRENTLY` for indexes (`@disable_ddl_transaction`); `NOT VALID` + validate for FKs.
- Batch large backfills; plan destructive changes for reversibility.

### Configure Pooling And Timeouts For The Workload

The DB pool (`pool_size`) must match the concurrency: too small and processes queue for connections (timeouts under load); too large and the DB is overwhelmed. For a web app, pool size ~ (concurrency / expected query time) tuned to the DB's `max_connections`. Set `timeout` (per-query) and `pool_timeout` (waiting for a connection) appropriately; a long query hitting the per-query `timeout` raises, while a saturated pool hitting `pool_timeout` raises `DBConnection.ConnectionError`. For migrations, set a long `migration_timeout`. In tests, the sandbox pool (`pool_size: 10`, `sandbox: true`) handles ownership. Monitor pool wait time and queue length as scaling signals.

- `pool_size` tuned to concurrency and DB `max_connections`; not too small (queue/timeouts) or large (DB overload).
- Per-query `timeout` vs `pool_timeout` (waiting for a connection); long migrations need `migration_timeout`.
- Monitor pool wait/queue as scaling signals; sandbox pool for tests.

## Common Traps

### N+1 Queries From Accessing Un-preloaded Associations

Iterating records and touching associations triggers per-record queries. `preload` when fetching.

### unique_constraint Name Mismatch

The constraint name in `unique_constraint(:email)` must match the DB index name, or Ecto raises instead of returning an error.

### Elixir-Only Uniqueness Check

Two concurrent inserts both pass `validate_required`-style uniqueness. Back uniqueness with a DB index.

### Index Creation Locking The Table

A plain `create index` on a large table blocks writes. Use `concurrently: true` (`@disable_ddl_transaction`).

### NOT NULL On A Large Table

Adding NOT NULL with a default rewrites/locks the table. Add nullable, backfill, then NOT NULL.

### Side Effects Not Rolled Back

An email sent inside a `Repo.transaction` is not undone on rollback. Send post-commit.

### Pool Exhaustion Under Load

Too-small pool queues and times out. Tune `pool_size` to concurrency; monitor wait times.

### Loading Whole Schemas For Projections

Fetching full schemas when only IDs are needed wastes memory. Use `select:` projections.

## Self-Check

- [ ] Associations are `preload`ed when fetched (no N+1); `preload:` vs `join:`/`assoc:` chosen by use case; `select:` projections used where full schemas are unnecessary.
- [ ] Changeset constraints (`unique_constraint`/`foreign_key_constraint`/`check_constraint`) match actual DB constraints by name; race-prone checks (uniqueness) are DB-enforced, not Elixir-only.
- [ ] Multi-step DB work uses `Ecto.Multi` with per-step error attribution; side effects (emails, publishes) are performed post-commit, not inside the transaction.
- [ ] Migrations are online-safe: nullable columns + batched backfill, NOT NULL in follow-up, `CONCURRENTLY` indexes (`@disable_ddl_transaction`), `NOT VALID` + validate FKs.
- [ ] Pool size and timeouts (`timeout`, `pool_timeout`, `migration_timeout`) are tuned to the workload and DB `max_connections`; pool wait/queue is monitored.
- [ ] Large backfills are batched; destructive migrations (drops, type changes) are planned for reversibility.
- [ ] Queries are profiled (logs/`EXPLAIN`) for N+1, missing indexes, and over-fetching.
- [ ] The schema/migration/query design has been considered under concurrency, large tables, partial failure, and rollback, and remains correct and performant.
