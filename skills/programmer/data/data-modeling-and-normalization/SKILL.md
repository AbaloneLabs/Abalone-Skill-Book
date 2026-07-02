---
name: data_modeling_and_normalization.md
description: Use when the agent is designing database schemas, deciding between normalization and denormalization, modeling entity relationships, designing for read patterns, handling time-series or hierarchical data, or planning schema evolution over time.
---

# Data Modeling and Normalization

Schema design is the part of a system that is most expensive to change later. Code can be refactored and redeployed, but a data model that does not match the domain or the access patterns haunts every query, every migration, and every report for the life of the system. The textbook answer ("normalize to 3NF") is correct for write integrity but is often wrong for read-heavy systems, and the real skill is knowing when to normalize, when to denormalize, and how to keep the model evolvable as the domain and access patterns shift.

The judgment problem is modeling entities and relationships to preserve integrity while serving the actual read and write patterns efficiently, choosing the right structure for special data shapes (time-series, hierarchical, polymorphic), and designing schemas that can evolve without painful migrations. The agent should not normalize reflexively nor denormalize eagerly; each decision is a tradeoff with concrete costs.

This skill applies whenever you are designing a new schema, reviewing a proposed schema, or planning how a schema must change to support new requirements.

## Core Rules

### Model the domain first, then adapt to access patterns

Start by representing the entities, attributes, and relationships of the domain faithfully (the normalized model). This protects write integrity: each fact is stored once, so updates cannot leave inconsistent duplicates. Only then adapt the physical model to access patterns by denormalizing where reads demand it. Inverting this order—designing for reads first—produces schemas that are fast for one query and inconsistent under writes.

### Treat normalization vs denormalization as an explicit tradeoff

Normalization and denormalization optimize different things:

- **Normalization** (one fact in one place): optimizes write integrity and storage efficiency. Updates touch one row. The cost is that reads must join, which is expensive at scale and under concurrency.
- **Denormalization** (duplicating data for read speed): optimizes read performance by precomputing joins or embedding related data. The cost is update complexity: every change to the source fact must propagate to all copies, or the data drifts inconsistent.

Make the choice explicit per relationship. A customer's name denormalized into an order row speeds order display but must be updated whenever the customer renames. Decide whether the read benefit justifies the update burden, and document how consistency is maintained (triggers, application logic, eventual consistency, or accepting staleness).

Weak choice: denormalizing eagerly because "joins are slow," without measuring or considering update cost. Strong choice: normalizing by default, denormalizing only measured hot read paths, with a documented consistency mechanism.

### Model relationships to match cardinality and access direction

The structure of a relationship (one-to-one, one-to-many, many-to-many) determines the schema, but so does the access direction:

- **One-to-many**: foreign key on the "many" side. Simple and correct for most cases.
- **Many-to-many**: a join table, possibly with attributes on the relationship itself (e.g., enrollment date on a student-course link).
- **One-to-one**: shared primary key or unique foreign key. Use when the entity is conceptually split for performance or access control (e.g., user vs user_profile).
- **Access direction matters**: if you almost always query "orders for a customer," the foreign key on orders is right. If you equally need "customer for an order," ensure an index supports the reverse lookup.

### Design for read patterns, not just write integrity

After the normalized model is correct, examine the hot read paths. Common read-driven adaptations:

- **Precomputed aggregates**: store running totals or counts to avoid recomputing them on every read, updating them on write.
- **Materialized views**: pre-join or pre-aggregate for reporting queries that cannot tolerate join cost.
- **Embedded/nested structures** (especially in document stores): embed sub-entities that are always read together and rarely queried independently.
- **Separation of OLTP and OLAP**: transactional and analytical workloads have opposed access patterns; a normalized OLTP schema with a separate denormalized warehouse or read replica often serves both better than a single compromise schema.

### Handle special data shapes deliberately

Certain data shapes have well-known modeling patterns that beat naive approaches:

- **Time-series**: append-mostly, high write volume, queried by time range. Model with time-partitioning, narrow rows, and specialized indexes; consider columnar storage or purpose-built TSDBs for very high volume. Avoid updating old points; treat them as immutable.
- **Hierarchical/tree data**: adjacency list (parent_id), nested sets, materialized path, or closure table. Each has different read/write tradeoffs: adjacency list is cheap to write but recursive to query subtrees; closure table is fast for subtree queries but expensive to maintain on writes. Choose by whether you read or write the hierarchy more.
- **Polymorphic associations**: when one entity can relate to several target types. Avoid a generic "target_type + target_id" column that defeats referential integrity; prefer separate foreign keys per type, a join table per type, or class-table inheritance.
- **Event/audit logs**: append-only, immutable, ordered. Model for append speed and replayability; derive current state via projection rather than mutating the log.

### Plan schema evolution from the start

Schemas change. Design for it:

- Prefer additive changes (new nullable columns, new tables) over destructive ones.
- Avoid `NOT NULL` without a default on existing tables until backfill is complete; add nullable, backfill, then constrain.
- Version rows or documents when shape can change, so readers can handle multiple versions during rollout.
- Keep migrations expand-then-contract (expand the schema, dual-write/read, then contract the old shape) to enable zero-downtime deployments.
- Avoid overloading column semantics (a `status` column whose meaning changes over time) because it corrupts historical data.

### Choose keys deliberately

Key choice has long-term consequences:

- **Surrogate keys** (e.g., auto-increment or UUID): decouple the key from domain data, so business changes do not require key changes. UUIDs avoid sequential allocation hotspots in distributed systems but are larger and less index-friendly than sequential integers.
- **Natural keys** (e.g., a real-world identifier): meaningful but can change or be reused, breaking references. Use only when truly immutable.
- **Avoid mutable business data as primary keys**: if the key can change, every foreign key reference must cascade, which is expensive and error-prone.

## Common Traps

### Denormalizing before measuring

"Joins are slow" is often true at scale but is frequently assumed rather than measured. Premature denormalization adds update complexity and inconsistency risk for read speedups that may be unnecessary. Normalize first, denormalize measured hot paths.

### Embedding everything in document stores

Document databases make embedding feel free, but embedding sub-entities that grow unbounded (e.g., all comments in a post document) causes document bloat, slow partial updates, and the 16MB-style limits. Embed what is bounded and always read together; reference what grows or is queried independently.

### Generic polymorphic columns

`target_type, target_id` columns defeat foreign keys and referential integrity, making orphaned references easy and joins impossible. The trap is that it feels flexible; the cost is that the database can no longer protect consistency.

### Mutable natural keys as primary keys

Using an email or username as a primary key means a change cascades to every referencing row, or forces surrogate identifiers anyway. Prefer surrogate keys for stability.

### Overloading column semantics

A `status` enum whose values are repurposed over time ("pending" meaning two different things in different eras) makes historical data ambiguous. Add new values rather than redefining old ones.

### Ignoring access direction in relationship modeling

Putting the foreign key on the side you rarely query from makes the common query require a reverse scan or an extra index. Model relationships to favor the dominant access direction.

### Treating schema as write-once

Schemas designed for the initial requirements without evolution in mind require painful big-bang migrations later. The expand-contract pattern and additive-by-default changes prevent this.

### Mixing OLTP and OLAP in one schema

A schema that serves both transactional writes and analytical scans compromises both: writes contend with heavy reads, and analytics suffer join overhead. Separating them (read replicas, warehouse, or CDC) serves both better.

## Self-Check

- Does the schema model the domain faithfully first, with denormalization applied only as a measured, documented adaptation to hot read paths?
- For every denormalized field, is there a documented mechanism keeping it consistent with its source (trigger, application logic, eventual consistency, or accepted staleness)?
- Are relationships modeled with the correct cardinality and with foreign keys/indexes favoring the dominant access direction?
- Are special data shapes (time-series, hierarchical, polymorphic, event logs) using the appropriate pattern rather than a naive flat table?
- Are keys chosen for stability (surrogate keys preferred) rather than mutable business data?
- Is schema evolution planned with additive-by-default changes and expand-then-contract migrations?
- Are column semantics stable over time, with new values added rather than old ones repurposed?
- Are OLTP and OLAP workloads separated when their access patterns conflict?
- For document/embedded schemas, are unbounded sub-collections referenced rather than embedded?
- Have you confirmed the normalized model preserves write integrity (each fact stored once) before adding read optimizations?
