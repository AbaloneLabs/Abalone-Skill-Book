---
name: csharp_ef_core_and_data_access.md
description: Use when the agent is writing or reviewing C# data access code with Entity Framework Core (DbContext, entities, migrations, change tracking, lazy/eager/explicit loading, LINQ-to-Entities queries, raw SQL, transactions, concurrency tokens), Dapper, or ADO.NET, or when designing the data access layer, choosing between EF Core and a micro-ORM, diagnosing N+1 queries, change-tracking overhead, migration conflicts, detached-entity problems, connection lifetime issues, or async-over-sync data access.
---

# C# EF Core And Data Access

Data access in .NET spans a spectrum from a full ORM (Entity Framework Core) to a micro-ORM (Dapper) to raw ADO.NET. EF Core tracks entity state, translates LINQ to SQL, manages migrations, and handles relationships — at the cost of overhead and a significant abstraction whose behavior you must understand to avoid performance cliffs and correctness bugs. The judgment problem is that EF Core is not "LINQ over objects": a LINQ-to-Entities query is translated to SQL and executed on the server, and constructs that work in LINQ-to-Objects (a client-side `DateTime.Parse`, an arbitrary method call) either throw at translation time or silently pull data client-side and process it there (the N+1 and over-fetching problems). Choosing the wrong tool (EF Core for a read-heavy reporting query, raw ADO.NET for CRUD) and misunderstanding EF Core's change tracker and query translation produce slow, incorrect, or surprising data access.

Agents tend to write LINQ-to-Entities as if it were LINQ-to-Objects, to enable lazy loading without realizing the N+1 cost, to misuse DbContext lifetime (singleton DbContext is a bug), or to ignore concurrency and transaction boundaries. The judgment problem is to choose the data access tool by workload, to write queries that translate to efficient SQL, to manage DbContext and connection lifetime correctly, and to handle transactions and concurrency deliberately. This skill is about treating the data access layer as a design problem with performance and correctness consequences, not a CRUD convenience.

## Core Rules

### Choose The Data Access Tool By Workload

The EF Core vs Dapper vs raw ADO.NET choice depends on the workload:

- **EF Core**: best for CRUD-heavy applications where the object-relational mapping, change tracking, migrations, and relationship management pay off. The overhead is acceptable when you need the abstraction. Use for typical line-of-business applications.
- **Dapper (micro-ORM)**: best for read-heavy or performance-sensitive queries where you write the SQL and map results to objects with minimal overhead. Use for reporting, complex queries, or hot paths where EF Core's translation or tracking overhead matters.
- **Raw ADO.NET (`DbDataReader`)**: best for maximum control or for bulk operations where even Dapper's mapping is overhead. Rarely needed, but correct for bulk inserts or specialized database features.

Mixing is fine and common: EF Core for most CRUD, Dapper or raw SQL for specific hot queries. Do not force EF Core onto a query it translates poorly, and do not write raw SQL for simple CRUD that EF Core handles well.

### Write LINQ-To-Entities Queries That Translate To Efficient SQL

A LINQ-to-Entities query is translated to SQL and executed on the database server. The key skill is writing queries that translate fully and efficiently:

- **Filter on the server**: `Where` clauses that use translatable operators (comparison, string methods that EF translates) run on the server. A `Where` that calls a client-side method (`Where(x => MyValidator.IsValid(x.Name))`) cannot translate and either throws or pulls the whole table client-side.
- **Project to what you need** (`Select(x => new Dto { Name = x.Name })`): selecting only needed columns avoids fetching entire entities. Fetching full entities when you need two fields is wasteful.
- **Avoid client-side evaluation**: methods like `DateTime.Parse`, arbitrary C# methods, and some string operations do not translate. Use `EF.Functions.Like`, `EF.Functions.DateDiffDay`, or computed columns instead. EF Core 3.0+ throws on client-side evaluation rather than silently doing it, which surfaces the problem.
- **Be careful with `Contains` on large lists**: an `IN` with thousands of items is slow; batch or use a join with a temp table.

Read the SQL EF generates (logging) for any non-trivial query. If it is surprising or slow, rewrite the LINQ or drop to raw SQL.

### Avoid N+1 Queries: Load Related Data Deliberately

The N+1 problem: fetching a list of entities, then separately fetching a related entity for each, produces 1 query for the list plus N for the relations. EF Core's default is no loading of relations (they are null unless loaded), which avoids accidental N+1 but requires you to load relations deliberately:

- **Eager loading**: `.Include(x => x.Children)` loads the relation in the original query (via a JOIN). Use when you know you need the relation.
- **Explicit loading**: `dbContext.Entry(entity).Reference(x => x.Parent).Load()` loads on demand. Use sparingly.
- **Lazy loading** (if enabled): accessing a navigation property triggers a query. This is convenient but is the source of N+1 — each access in a loop is a separate query. Avoid lazy loading in performance-sensitive code; prefer eager loading with `Include` or split queries.

Use `.AsNoTracking()` for read-only queries to skip change tracking (faster, less memory). Use `.AsSplitQuery()` when an `Include` would produce a Cartesian explosion (one query per collection).

### Manage DbContext Lifetime Correctly

A `DbContext` is not thread-safe and is meant to be short-lived and scoped (one per unit of work / request). The default DI registration is `Scoped`, which gives one instance per HTTP request — correct for web apps. The bugs:

- **Singleton DbContext**: a single instance shared across requests is not thread-safe and its change tracker grows unbounded (every entity ever queried is tracked forever, a memory leak and stale-data bug). Never register DbContext as singleton.
- **Long-lived DbContext**: even if not singleton, a DbContext held for a long time accumulates tracked entities and stale data. Keep it scoped to a unit of work.
- **Shared DbContext across threads**: a DbContext used from multiple threads concurrently corrupts. One per thread/request.

For background services, create a scope (`IServiceScopeFactory.CreateScope`) per unit of work and resolve the DbContext from it, rather than injecting a singleton or the request-scoped instance.

### Handle Transactions And Concurrency Deliberately

By default, EF Core wraps a single `SaveChanges` in a transaction, so a set of changes all commit or all roll back. For multi-`SaveChanges` atomicity, wrap in an explicit transaction (`dbContext.Database.BeginTransaction()` or `TransactionScope`). Be aware of isolation levels and lock duration: a long-running transaction holds locks and can cause contention.

For concurrent updates, use **optimistic concurrency**: a `[Timestamp]`/`RowVersion` column (a byte array the database updates on each write) that EF checks on update — if the row was modified since it was read, the update affects zero rows and EF throws `DbUpdateConcurrencyException`, which the app resolves (refresh, merge, or error). This is the correct model for web apps where two users might edit the same row; pessimistic locking (holding a lock) does not scale across web requests.

### Use Migrations With Discipline

EF Core migrations are versioned, named changes to the schema, generated from the model. The discipline: generate a migration per model change, review the generated `Up`/`Down`, test it against a copy of production data (a migration that drops/recreates a column loses data), and apply migrations deliberately (at deploy time, not at app startup in production). Common problems:

- **Migration conflicts**: two developers generate migrations off different models; resolve by regenerating or merging.
- **Data migrations**: a schema change that needs data transformation (splitting a column) requires custom migration code, not just `AddColumn`.
- **Production drift**: the production schema diverging from the migration history. Always back up before applying.

Do not let the app auto-migrate on startup in production (a failed migration mid-startup leaves the app down); apply migrations as a controlled deploy step.

### Prefer Async Data Access; Never Block On Async

Database calls are I/O-bound; use `ToListAsync()`, `FirstOrDefaultAsync()`, `SaveChangesAsync()` rather than their synchronous counterparts, and never block on async (`.Result`, `.Wait()`) which can deadlock in a synchronization-context environment and always wastes a thread. Propagate cancellation via `CancellationToken`. Async data access scales far better under load because it does not hold thread-pool threads during I/O.

## Common Traps

### LINQ-To-Entities Query That Evaluates Client-Side

A `Where` or `Select` calling a non-translatable method pulls data client-side or throws. Use translatable operators or `EF.Functions.*`.

### N+1 From Lazy Loading In A Loop

Accessing a navigation property per item in a loop triggers N queries. Eager-load with `Include` or project to a DTO.

### Singleton Or Shared DbContext

A DbContext shared across requests/threads is not thread-safe and its change tracker grows unbounded. Use scoped, one per unit of work.

### Fetching Entire Entities When A Projection Would Do

`Select(x => x)` (or implicit) fetches all columns; `Select(x => new Dto { Name = x.Name })` fetches only needed columns. Project read-only queries.

### Forgetting AsNoTracking On Read-Only Queries

Read-only queries that do not need change tracking should use `.AsNoTracking()` to skip tracking overhead and reduce memory.

### Blocking On Async Data Access

`.Result` / `.Wait()` on an async DB call can deadlock and wastes threads. Use `await` and propagate `CancellationToken`.

### Missing Concurrency Token (Last-Write-Wins Data Loss)

Without a `[Timestamp]`/`RowVersion`, two concurrent updates silently overwrite each other (last write wins). Add a concurrency token and handle `DbUpdateConcurrencyException`.

### Auto-Migrating On Startup In Production

A failed migration at startup leaves the app down. Apply migrations as a controlled deploy step, with a backup.

## Self-Check

- [ ] The data access tool matches the workload: EF Core for CRUD with mapping/tracking, Dapper/raw SQL for read-heavy or hot queries, mixed where appropriate.
- [ ] LINQ-to-Entities queries translate fully to server-side SQL (filtering, projection, translatable operators); client-side evaluation is avoided or surfaced; generated SQL is reviewed for non-trivial queries.
- [ ] Related data is loaded deliberately (eager `Include`, split queries for collections, or explicit) and lazy loading is avoided in hot paths to prevent N+1.
- [ ] DbContext is scoped per unit of work (per request), never singleton or shared across threads; background services create a scope per unit of work.
- [ ] Multi-change atomicity uses explicit transactions with deliberate isolation levels, and concurrent updates use optimistic concurrency (`[Timestamp]`/`RowVersion`) with `DbUpdateConcurrencyException` handling.
- [ ] Migrations are generated per model change, reviewed, tested against production-like data, and applied as a controlled deploy step, not auto-applied at startup in production.
- [ ] Data access is async (`ToListAsync`, `SaveChangesAsync`) with `CancellationToken` propagated, and never blocks on async with `.Result`/`.Wait()`.
- [ ] Read-only queries use `.AsNoTracking()` and project to DTOs to avoid fetching/tracking unnecessary data.
