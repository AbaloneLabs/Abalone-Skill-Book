---
name: schema_and_migration.md
description: Use when the agent is designing or changing a database schema, adding migrations, choosing constraints, modeling relationships, changing indexes, handling deletion, or moving persistent data between application versions.
---

# Schema And Migration

Database schema work is product design, operational design, and failure design at the same time. A schema defines what states are possible, which assumptions the application can trust, how data survives version changes, and what future migrations will cost. A programmer should not treat schema changes as incidental storage details.

Use this skill before adding tables, columns, indexes, foreign keys, enum values, uniqueness rules, soft delete behavior, audit fields, backfills, data migrations, or persistence changes that must run against existing data.

## Core Rules

### Model The Invariant, Not Only The Current Form

Start with the rule the system must preserve. Then choose columns, constraints, and relationships that make invalid states hard or impossible.

Ask:

- What entities exist independently?
- What relationships are mandatory, optional, one-to-one, one-to-many, or many-to-many?
- Which states are impossible and should be prevented?
- Which states are valid today but may change later?
- What data is historical fact versus current configuration?
- Which fields are user input, derived data, external provider data, or internal state?

Do not add columns only because the current UI has fields. UIs change. The schema should capture durable domain meaning.

### Use Database Constraints For Critical Truth

Application validation is necessary but insufficient. Any invariant that must survive concurrency, multiple services, scripts, imports, workers, retries, or future code should be backed by the database when practical.

Common constraints:

- `NOT NULL` for required facts;
- `UNIQUE` for identity and de-duplication;
- foreign keys for ownership and relationship integrity;
- check constraints for ranges, state values, and mutually exclusive fields;
- partial unique indexes for active-only uniqueness;
- cascading or restricted deletes when ownership implies lifecycle.

Avoid relying on "we always check this in code" for security, billing, identity, or consistency-critical data.

### Design IDs And Uniqueness Deliberately

Choose identifiers based on exposure, merge behavior, operational needs, and external integration. Internal primary keys, public ids, slugs, provider ids, idempotency keys, and natural keys serve different purposes.

For uniqueness, define the full scope. An email may be unique globally, per tenant, per auth provider, or only among active accounts. A project slug may be unique inside an organization, not globally. A scheduled job key may be unique per period and target.

If soft deletes exist, decide whether deleted rows keep claiming uniqueness. If not, use a partial index or an explicit lifecycle rule.

### Treat Null As A State

`NULL` is not just "unknown". It can mean not applicable, not collected, deleted, hidden, pending, inherited, failed, or not yet migrated. If the meaning differs by field, document it in naming, constraints, code comments, or type modeling.

Avoid overloaded nulls. If a field needs several states, use an explicit status, separate timestamp, or related table. If null is only temporary during a migration, remove it or tighten it after the backfill.

### Plan Migrations For Existing Data

A migration must work on real production data, not only on an empty database. For each schema change, think through:

- old rows that violate the new rule;
- large tables and lock duration;
- backfill order and batching;
- deploy order when old and new application versions overlap;
- rollback or roll-forward strategy;
- default values and generated values;
- validation before enforcing constraints.

For risky changes, split into phases: add nullable column, write both old and new fields, backfill, verify, read new field, enforce constraint, remove old field. Do not combine every step into one irreversible migration unless the data set is small and the blast radius is clear.

### Index For Actual Access Patterns

Indexes should follow queries, filters, joins, ordering, uniqueness, and tenant boundaries. Guessing indexes from column names creates write overhead without solving performance.

Check:

- most common filters and sort orders;
- multi-tenant prefix columns;
- pagination strategy;
- foreign key join paths;
- uniqueness constraints;
- partial indexes for active records;
- covering indexes for frequent reads;
- write volume and update cost.

An index for `created_at` alone may not help a query that filters by `organization_id`, `status`, and then orders by `created_at`. Composite index order matters.

### Decide Deletion And Retention Explicitly

Deletion has product, legal, operational, and analytics consequences. For each entity, decide whether deletion means hard delete, soft delete, anonymization, archival, revocation, or tombstone.

Consider:

- user expectations and legal deletion rights;
- audit requirements;
- billing and tax retention;
- referential integrity;
- restore behavior;
- search index and cache cleanup;
- backups and delayed erasure;
- uniqueness after deletion;
- whether child records should cascade, restrict, or remain.

Do not add `deleted_at` everywhere without lifecycle rules. Soft delete can make every query and uniqueness rule more complex.

### Keep External Provider Data Separate From Internal Truth

External ids, webhook payloads, sync cursors, billing states, and imported records may be stale, duplicated, delayed, or corrected. Store enough provenance to reconcile them, but do not let provider-specific shape dominate the core domain model unless the application is truly a thin wrapper over that provider.

Keep raw payloads only when necessary, protect them as sensitive, and avoid using unvalidated external data as trusted authorization or billing truth.

## Common Traps

### Adding A Column Instead Of Modeling A Relationship

A quick `foo_id`, `is_primary`, or `extra_json` field may hide a missing relationship. If the data can have multiple values, history, ordering, ownership, or permissions, it may deserve its own table.

### Using JSON As An Escape Hatch

JSON fields are useful for flexible provider payloads, feature configuration, and low-value metadata. They are dangerous when they replace queryable, constrained, security-sensitive, or lifecycle-managed data. If the application depends on a JSON key for authorization, billing, or workflow state, consider promoting it to a real column.

### Making Enums Too Rigid Or Too Loose

Database enums and check constraints protect state, but can make deployments harder when values change often. String states without constraints drift over time. Pick based on change frequency and operational control. If state drives critical behavior, constrain it somewhere reliable.

### Forgetting Backward Compatibility During Deploys

Rolling deploys mean old code and new code can run together. A migration that removes a column before all code stops reading it, or enforces a new non-null field before all writers populate it, can break production.

### Ignoring Time And History

Many values need time context: price, role, status, membership, consent, address, plan, assignment, or policy. Overwriting current state may destroy facts needed for audit, reconciliation, or analysis.

### Creating Unbounded Tables Without Lifecycle

Events, logs, notifications, sessions, jobs, webhooks, metrics, and audit rows can grow forever. Define retention, partitioning, archival, and cleanup before the table becomes operational debt.

## Self-Check

- [ ] The schema models durable domain invariants, not only the current UI fields.
- [ ] Critical correctness, identity, ownership, and consistency rules are backed by database constraints where practical.
- [ ] Identifier, public id, slug, provider id, and uniqueness scopes are deliberate.
- [ ] Null values have clear meaning and are not hiding multiple unrelated states.
- [ ] Migration order handles existing data, rolling deploys, backfills, validation, and roll-forward or rollback.
- [ ] Indexes match real filters, joins, sort order, tenant boundaries, pagination, and uniqueness needs.
- [ ] Delete, soft delete, retention, archival, restore, and cascade behavior are explicit.
- [ ] External provider data is separated from internal source of truth and has provenance.
- [ ] Large tables, locks, write amplification, and backfill cost were considered before migration.
- [ ] Tests or migration checks cover old data, invalid states, uniqueness conflicts, and expected query paths.
