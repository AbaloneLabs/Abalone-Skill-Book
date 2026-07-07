---
name: database_connection_and_orm.md
description: Use when the agent is configuring or using a database layer in a backend service — connection pools and pool sizing, ORMs (Hibernate/JPA, SQLAlchemy, Django ORM, Prisma, GORM, Entity Framework) vs raw SQL vs query builders, managing sessions/transactions and the unit of work, lazy loading and N+1 queries, mapping entities to tables, migrations, or diagnosing connection leaks, pool exhaustion, "LazyInitializationException" / "detached entity", N+1 query storms, slow generated SQL, lost updates, or transactions held across HTTP calls. Covers connection pool lifecycle, ORM pitfalls (lazy loading, identity maps, cascade), the impedance mismatch, and when to drop down to raw SQL.
---

# Database Connection And ORM

A database layer in a backend service is two coupled systems: a pool of network connections to the database (a scarce, stateful, failure-prone resource), and an Object-Relational Mapper (ORM) or query layer that translates between the object graph the application reasons about and the relational tables the database stores. Both are easy to use superficially and both are where services fail under load: a connection pool sized too small starves requests under concurrency; a connection leaked (never returned to the pool) exhausts the pool and hangs the service; a lazy-loading ORM fires a query per item in a list (the N+1) and turns one endpoint into a thousand database round-trips; a transaction held open across an HTTP call holds locks and bloats the connection count. The database is the slowest, most contended resource in most services, and the data-access layer is where that contention is won or lost.

The judgment problem is not "how do I query the database" but "how many connections does this service need and how are they returned, what does the ORM actually generate and fetch, where are the transaction boundaries, and where is the ORM fighting the relational model." Agents tend to treat the ORM as a magic persistence layer (ignoring the SQL it generates), to leave connection management to defaults (which are often wrong for production), to write transactions that span blocking work (HTTP, locks held), and to reach for ORM features (lazy loading, cascades, identity maps) without understanding their cost. Each produces a service that passes local tests and collapses under real load or real data shapes.

## Core Rules

### Size And Lifecycle The Connection Pool Deliberately; Never Leak Connections

A database connection is a scarce resource: each one holds a TCP connection, a server-side process or thread (on the database), memory, and possibly a transaction. A connection pool reuses connections across requests to amortize their creation cost, and the pool size is a capacity and correctness decision. Too few connections and concurrent requests queue behind them (latency spikes, timeouts); too many and the database is overwhelmed (it spends more time context-switching between connections than doing work), server-side memory explodes, and you hit database or OS limits. There is no universally correct number — it depends on the database, the query latency, and the concurrency — but the common error is leaving it at a framework default (often 10) that is far too small for a real service, or unbounded in a way that lets a burst exhaust the database.

The deeper rule is that every acquired connection must be returned, no matter what path the code takes (success, error, exception, timeout, cancellation). A connection acquired and not returned is a leak; over time leaks exhaust the pool and the service hangs waiting for a connection that never comes. Use the language's automatic resource management (try-with-resources in Java, `defer rows.Close()` in Go, context managers in Python, `using` in C#) so the connection/statement/result-set is closed on every path. Never acquire a connection and then call code that might throw without the close being guaranteed. Pool exhaustion that appears gradually under load and recovers on restart is almost always a connection leak; profile the pool's active/idle counts and set a leak-detection timeout.

### Understand What The ORM Generates: Watch For N+1, Cartesian Products, And Unbounded Fetches

An ORM hides SQL, and the SQL it generates is the actual performance of your service. The dominant ORM pathology is the N+1 query: iterating a list of entities and accessing a relation (an order's items, a user's profile) triggers one query per entity to load the relation lazily, turning a list of 100 orders into 101 queries (1 for the list, 100 for the items). N+1s pass every unit test (the data is correct) and destroy performance under real data. The fixes are eager loading (`JOIN FETCH`, `selectinload`/`joinedload` in SQLAlchemy, Prisma's `include`, GORM's `Preload`) to fetch the relation in one query, or batching (`WHERE id IN (...)`) to load relations for many parents at once.

Other generation pathologies: fetching an unbounded result set into memory (`SELECT *` with no LIMIT, or an ORM `findAll()` on a large table, which loads every row as an object and OOMs); a Cartesian product from joining multiple one-to-many relations (which multiplies rows); a "select everything" default that fetches all columns including large text/blob fields when only the id and name are needed; and an ORM that issues an UPDATE per changed field or a DELETE+INSERT for a collection change, generating far more SQL than the operation needs. The discipline is to log and read the generated SQL during development (Hibernate's `show_sql`, SQLAlchemy's `echo=True`, Prisma's query logging), to use pagination on every list query, and to select only the columns needed. If you do not know what SQL the ORM is running, you do not know your service's performance.

### Bound Transactions Tightly; Never Hold Them Across Blocking Work

A transaction is a unit of work that holds locks (row locks, table locks) and a connection for its duration. The rule is to make transactions as short as possible and to keep them free of blocking work. A transaction that spans a remote call (an HTTP request to another service, a message publish that waits for an ack), a long computation, or a user think-time holds its locks and its connection for all of that time, serializing other work behind it and inflating the connection count. The classic failure is "open transaction at the start of the request handler, do everything including downstream calls, commit at the end" — under load this deadlocks, exhausts connections, and produces lost updates from long-held locks.

The discipline: open the transaction as late as possible and close it as soon as the database work is done; do the non-database work (calling other services, sending emails, publishing events) outside the transaction, often after it commits. If you need atomicity between the database write and a side effect (publish an event only if the commit succeeds), use the transactional-outbox pattern (write the event to an outbox table in the same transaction, a separate process publishes it) rather than holding the transaction open across the publish. Match the isolation level to the requirement (read-committed is the common default; repeatable-read or serializable for stricter correctness, at a concurrency cost) and understand that higher isolation holds more locks and is slower. The transaction-isolation details are covered in the data-area skills; the data-access rule is that transactions are short, scoped to database work, and never span blocking calls.

### Treat Lazy Loading As A Loaded Gun; Prefer Explicit Fetching

Lazy loading (loading a relation only when it is first accessed) is convenient and dangerous. It makes the ORM fire queries at access time, which means: (a) the N+1 problem above, (b) queries fired after the session/context that loaded the parent has closed, producing "LazyInitializationException" (Hibernate), "detached instance" errors, or a `DetachedInstanceError` (SQLAlchemy) — the relation cannot be loaded because the connection/session is gone, and (c) queries fired in surprising places (in a view/template, in a serialized JSON response, in a log statement) that are hard to attribute and optimize. Lazy loading hides data access from the code's structure, which is its appeal and its hazard.

The discipline is to prefer explicit eager fetching at the query site: when you load an order you will display with its items, fetch the items in the query (`JOIN FETCH`, `selectinload`), so the data access is visible and batched. Treat lazy loading as an optimization for relations that are usually not needed, not as the default. Disable lazy loading in serialization boundaries (a `@JsonIgnore` or a DTO projection that does not trigger relations) so serializing an entity does not fire a cascade of queries. And never let an entity with lazy relations escape the session that loaded it (do not return a Hibernate entity from a service whose transaction has closed) — return a DTO or a fully-loaded entity. If you see LazyInitializationException in production, the fix is to load the data eagerly in the query, not to keep the session open longer.

### Choose ORM vs Query Builder vs Raw SQL By The Operation, And Drop Down When The ORM Fights You

An ORM is a productivity win for the common case (simple CRUD on a single entity, straightforward relations) and a liability for the cases it models poorly (complex reporting queries, bulk updates, window functions, recursive CTEs, database-specific features). The mistake is to force every operation through the ORM: a five-table analytic query expressed in ORM criteria API is harder to write, harder to read, and often generates worse SQL than the equivalent raw query. The discipline is to use the ORM where it helps (entity persistence, simple relations, the 80% of operations that are CRUD) and to drop down to a query builder or raw SQL where the ORM is fighting the relational model (the 20% that are complex).

Drop down when: the ORM generates inefficient SQL you cannot tune through it; the query uses features the ORM does not expose (window functions, recursive queries, database-specific functions); you need a bulk operation (`UPDATE ... WHERE`, `INSERT ... ON CONFLICT`) that the ORM would do row-by-row; or the result shape is not an entity (a projection, an aggregate, a report). Use the ORM's escape hatches (Hibernate's native SQL, SQLAlchemy Core or `text()`, Prisma's `$queryRaw`, Django's `raw()`), keep the SQL in one place (a repository/mapper layer), and map the results to DTOs rather than fake entities. The goal is not ORM purity but correct, efficient, maintainable data access — and that sometimes means SQL.

### Manage The Object-Relational Impedance Mismatch Deliberately

The ORM maps between objects (with identity, references, inheritance, collections) and relations (with primary keys, foreign keys, joins, no inheritance), and the two models do not match cleanly. This "impedance mismatch" surfaces as: inheritance (an object hierarchy has no direct relational equivalent — ORM uses single-table-per-hierarchy, joined, or table-per-concrete-class strategies, each with tradeoffs); collections (an object's `List<LineItem>` maps to a separate table with a foreign key, and updating the list means diffing and issuing individual inserts/deletes); identity (an object has Java/object identity and database identity, and the ORM's identity map tries to ensure one object per row within a session, which breaks if bypassed); and navigation (objects navigate by reference, relations by join).

The discipline is to recognize where the mismatch creates cost and to design around it. Prefer composition over inheritance in entities (inheritance mapping is complex and slow). Keep entity collections simple and bounded (an order's items, not a user's entire activity history, which would be huge). Be aware that the ORM's identity map means two loads of the same row in one session return the same object (so changes are visible across references) but that this does not hold across sessions. And consider whether an entity needs to be a full ORM-mapped object at all — many reads are better served by a query that maps directly to a DTO, skipping the entity entirely. The mismatch is unavoidable; managing it deliberately (rather than pretending the ORM makes objects and tables "the same thing") is what keeps the data layer maintainable.

### Keep Migrations And Schema Changes Safe And Versioned

The schema is part of the codebase and changes to it must be managed like code: versioned, reviewed, and applied deliberately through migrations (Flyway, Liquibase, Alembic, Django migrations, Knex, golang-migrate), not by hand or by the ORM auto-generating DDL in production. Migrations must be safe to apply on a live database without downtime: add columns as nullable (or with defaults) before backfilling; avoid locking tables for long periods (a `ALTER TABLE` that rewrites the table on a large table blocks writes); deploy the code change that uses a new column after the column exists, not before. The schema-migration discipline is covered in the data-area skills; the data-access rule here is that the ORM's "auto-create/update schema" feature is for development, never for production — production schema changes are migrations, reviewed and staged.

## Common Traps

### Connection Pool Sized At A Wrong Default

A framework default of 10 connections starving a service under concurrency, or an unbounded pool overwhelming the database. Size the pool to measured concurrency and query latency; never leave it at an unexamined default.

### Leaked Connections Exhausting The Pool

A connection/statement/result-set acquired and not closed on an error path, gradually exhausting the pool until the service hangs. Use automatic resource management; set a leak-detection timeout.

### N+1 Queries From Lazy Loading In A Loop

Iterating entities and accessing a relation fires one query per entity. Eager-fetch (`JOIN FETCH`, `selectinload`, `Preload`) or batch-load relations at the query site.

### Fetching Unbounded Result Sets Into Memory

`findAll()` or `SELECT *` with no LIMIT loading every row as an object, OOM-ing on a large table. Paginate every list query; select only needed columns.

### Transaction Held Across An HTTP Call Or Long Computation

A request-scoped transaction that holds locks and a connection across downstream calls, serializing work and exhausting connections. Keep transactions short and scoped to database work; use the transactional outbox for side effects.

### LazyInitializationException / Detached Entity

Accessing a lazy relation after the session closed, because a detached entity escaped the service layer. Load eagerly in the query, or return a DTO; do not extend the session lifetime as a fix.

### Forcing Complex Queries Through The ORM

A reporting or bulk query written in convoluted ORM criteria that generates worse SQL than raw. Drop down to a query builder or raw SQL where the ORM fights the relational model.

### ORM Auto-DDL In Production

`hibernate.hbm2ddl.auto=update` or `syncdb` running against the production database, applying unreviewed schema changes. Use versioned migrations; disable auto-DDL outside development.

### Assuming ORM Identity Map Semantics Across Sessions

Two loads of the same row returning the same object within a session but not across sessions, surprising code that assumed global identity. Understand the identity map's scope; key caches by id, not by object identity.

## Self-Check

- [ ] The connection pool is sized deliberately for the service's concurrency and query latency (not a framework default), connections/statements/result-sets are closed on every path via automatic resource management, and a leak-detection timeout is configured.
- [ ] The generated SQL is logged and reviewed during development; N+1 queries are eliminated (relations eager-fetched or batch-loaded at the query site), every list query is paginated, and only needed columns are selected.
- [ ] Transactions are short and scoped to database work — opened late, closed after the DB write, with no remote calls, long computations, or user think-time inside them; side effects that need atomicity with the DB write use the transactional-outbox pattern.
- [ ] Lazy loading is used deliberately (not as a default), relations needed for a task are eager-fetched at the query site, entities with lazy relations do not escape the session that loaded them (DTOs or fully-loaded entities are returned), and lazy loading is disabled at serialization boundaries.
- [ ] ORM vs query-builder vs raw SQL is chosen per operation — ORM for CRUD and simple relations, raw SQL or query builder for complex/bulk/database-specific queries — and raw results map to DTOs in a repository layer rather than fake entities.
- [ ] The object-relational mismatch is managed deliberately: composition over inheritance, bounded collections, awareness of the identity map's session scope, and read-only projections that skip entity mapping where a DTO suffices.
- [ ] Schema changes are versioned migrations (not ORM auto-DDL), safe to apply without downtime (nullable/defaulted columns, no long table rewrites), and deployed in the right order relative to code changes.
- [ ] The data layer was load-tested with realistic data volume and concurrency: connection count, query count per request, and p99 latency were measured, and no N+1, leak, or held-transaction pathology appears under load.
