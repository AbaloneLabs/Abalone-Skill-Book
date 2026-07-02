---
name: query_optimization_and_n_plus_one.md
description: Use when the agent is writing or reviewing database queries, ORM access code, data-loading loops, list or detail endpoints, pagination, batch reads, or reporting queries; diagnosing slow endpoints, high database load, or latency that grows with data or user count; deciding between eager and lazy loading, joins versus separate queries, or batch loading versus per-row access; reading or interpreting query execution plans; adding or evaluating indexes; or choosing how much work to do in the database versus the application. Also covers N+1 query patterns, the hidden cost of ORM convenience methods, Cartesian explosions from nested relations, keyset versus offset pagination, and the tradeoff between query count and query complexity.
---

# Query Optimization And N+1

Most data-access performance problems are not algorithmic. They are structural: the code issues the right kind of query the wrong number of times, or asks the database to do work it was never indexed to do efficiently. The signature failure is the N+1 — one query to load a list of N parent rows, then N more queries to load each parent's children — which looks fine in development with ten rows and collapses in production with ten thousand. The harm is invisible until scale exposes it: an endpoint that was fast in QA becomes slow under real data, database connection pools exhaust, and cost spikes on managed databases trace back to a loop that ran one extra query per row.

Agents tend to miss these problems because ORMs hide the queries. A line like `order.user.profile` reads as a single field access but may issue a fresh query each time it is evaluated. The local test passes because the data set is tiny and the extra queries return in milliseconds. The judgment problem is learning to see data access as a set of queries with a shape and a cost, not as a chain of object property accesses — and then choosing loading strategies, indexes, and pagination that scale with data rather than with a development sample.

This skill is about query performance. It complements the schema-and-migration skill, which covers how to design tables, constraints, and indexes for the invariants. Here the schema is assumed and the question is how the application reads from it efficiently.

## Core Rules

### Count Queries Per Request, Not Per Function

The first diagnostic habit is to count how many queries a single request or operation issues, and how that count grows with input size. A healthy read path issues a small, constant number of queries regardless of how many rows it returns. A path whose query count scales with the result set is an N+1 waiting to happen.

Enable query logging in development and tests. For any list or detail endpoint, inspect the actual query log, not the code's apparent structure. Patterns to flag:

- **N+1.** One query loads N parents; a loop then loads each parent's relation individually → 1 + N queries.
- **N+1+M (Cartesian or nested).** Each parent loads its children, and each child loads its own relation → query count explodes with nesting depth.
- **Per-row lookup in a loop.** Loading a list, then calling `find(id)` for each element inside a loop instead of one `whereIn(ids)`.
- **Repeated identical queries.** The same row fetched multiple times within one request because no caching or batching deduplicates it.

The target is a constant or near-constant query count per request. If the count is `1 + N` or worse, fix the loading strategy before optimizing anything else.

### Choose Eager Versus Lazy Loading By Access Pattern, Not By Default

ORMs offer eager loading (fetch relations upfront in the same or a batched query) and lazy loading (fetch each relation on first access). Neither is universally correct; the right choice depends on whether the relation is always needed, sometimes needed, or rarely needed.

- **Always needed (the response always renders the relation).** Eager-load it. Lazy loading here guarantees an N+1.
- **Sometimes needed (depends on caller or view).** Make loading explicit at the call site that needs it, rather than defaulting to lazy and hoping. A serializer that sometimes needs `comments` should receive objects with comments already loaded when it needs them.
- **Rarely needed (one edge case).** Lazy loading is acceptable, but isolate it to that edge case so the common path stays query-efficient.

Beware ORMs that lazy-load by default. The convenience of `order.user` hides a query per access. Prefer explicit eager-loading calls (`with('user')`, `include(:user)`, `select_related`, `fetch join`) at the point where you know the relation is needed, so the query shape is visible in the code.

### Batch Reads Instead Of Looping Over Single Fetches

Whenever code loads many things, load them in one batched query, not in a loop of single fetches. The canonical fix for N+1 is "load all parents, collect their ids, load all children in one `WHERE id IN (...)` query, then group children by parent in memory."

This turns 1 + N queries into exactly 2 queries regardless of N. The same principle applies to:

- loading related entities by a set of foreign keys;
- resolving references stored as JSON or arrays;
- prefetching data for a list view before serializing.

Batching trades query count for query complexity (a larger `IN` list or a join). That trade is almost always worth it: one query with 1000 ids is cheaper than 1000 queries with one id each, because per-query overhead (parse, plan, round trip, connection checkout) dominates at small row counts.

### Read The Execution Plan Before Optimizing

Before changing a query or adding an index, look at what the database actually does. `EXPLAIN` (PostgreSQL/MySQL) or the equivalent shows the access path: sequential scan, index scan, join strategy, estimated rows, and cost.

Learn to read the signals:

- **Seq scan on a large filtered table** — missing or unused index; the database reads every row to find the few that match.
- **Index scan returning few rows** — good; the index is selective.
- **Nested loop join with many outer rows** — can be fine for small sets and catastrophic for large ones; the inner side is re-scanned per outer row.
- **Huge estimated row counts that collapse to few actual rows** — stale statistics or a predicate the planner cannot estimate; the plan may be wrong.
- **Filesort or temporary table on a large input** — the query cannot use an index for ordering or grouping and spills to disk.

Optimize against the plan, not against intuition. A query that "looks slow" may already use the right index; a query that "looks fine" may be doing a sequential scan over millions of rows. Re-examine the plan after any change to confirm it improved.

### Index For The Real Predicates, In The Right Order

Indexes help only when they match the predicates, joins, and ordering the queries actually use. An index on `created_at` does nothing for a query filtering by `organization_id` and `status`. A composite index helps only when its column order matches the query's access pattern: the columns tested by equality come first, then range or sort columns.

Check each hot query against its indexes:

- Which columns are filtered by equality? They should lead the composite index.
- Which column is used for range or sorting? It should follow the equality columns.
- Is the query multi-tenant? The tenant column usually belongs first, because every query scopes by it.
- Does the query join on a foreign key? That FK needs an index on the child side; many databases do not index FKs automatically.
- Is the sort order served by the index, or does the database filesort after fetching?

An index is not free: it costs writes and storage. Add indexes for queries that are frequent or slow, and remove indexes that no query uses. But the common failure is too few or mis-ordered indexes on hot paths, not too many.

### Decide Pagination Strategy By Depth And Stability

Pagination is a query-optimization decision, not just a UX one. The two main strategies have very different cost profiles:

- **Offset pagination (`LIMIT 20 OFFSET 10000`).** Simple and stateless, but the database must scan and discard the first 10000 rows to return page 501. Cost grows linearly with depth; deep pages become slow and expensive. Fine for shallow browsing, bad for "export everything" or infinite scroll.
- **Keyset / cursor pagination (`WHERE (created_at, id) < (?, ?) ORDER BY created_at DESC, id DESC LIMIT 20`).** Cost is constant regardless of depth, because it seeks directly to the cursor using an index. Required for deep or unbounded traversal. It needs a stable, unique, indexed ordering column and breaks the "jump to page N" UX.

Match the strategy to the access pattern. Shallow UI pagination can use offset. Anything that may go deep — exports, infinite feeds, background scans — must use keyset, or it will degrade as data grows. Also decide explicitly how to handle newly inserted rows during pagination: keyset is stable under inserts; offset shifts.

### Balance Query Count Against Query Complexity

There is a tradeoff between many simple queries and one complex query. Reducing query count by joining everything can produce a query so large, or a result set so denormalized, that it is slower and harder to maintain than a few focused queries.

Guidelines:

- Prefer few queries over many, but not one giant query over a few medium ones. A handful of targeted queries is often the sweet spot.
- Avoid joins that multiply rows (Cartesian products) when you only need existence or counts; use subqueries or separate existence checks instead.
- Do aggregation in the database (`SUM`, `COUNT`, `GROUP BY`) when the grouped set is large; do it in the application only when the set is already small and loaded.
- Resist loading entire tables into memory to "process in code." If the database can filter and aggregate, let it.

The goal is not minimum query count nor maximum database work; it is the right amount of work in the right place, verified against the execution plan and real data volumes.

### Measure Against Realistic Data Volumes

A query plan and a loading strategy that look correct can still fail at production scale. Test data access against data volumes and distributions similar to production: not ten rows, but the row counts and skew (a few huge tenants, many small ones; long history; hot and cold partitions) you actually have.

Specifically validate:

- Does the endpoint stay flat in query count as the list grows?
- Does the index remain selective when a tenant has many rows?
- Does pagination stay fast at depth?
- Does a join explode in row count when relations are populated?

Synthetic, uniform, tiny test data hides the problems that scale reveals. Seed test databases with realistic volume and skew before declaring a query path healthy.

## Common Traps

### Trusting The ORM Property Access As A Single Query

Writing `for order in orders: print(order.user.name)` and assuming the ORM cached or batched the user. By default it did not; each access issues a query. The fix is to eager-load `user` when the list is fetched, not to hope the ORM is smart.

### Eager-Loading Everything To "Be Safe"

Calling `include`/`with` on every possible relation on every query, loading large nested graphs that the current endpoint never uses. This trades N+1 for over-fetching: more memory, larger payloads, and slower queries that join tables the response does not need. Eager-load what the response actually renders.

### Fixing N+1 With A Join That Cartesian-Explodes

Replacing N+1 with a single join, but the join multiplies rows (one parent with many children returns the parent many times), and the result is deduplicated in application memory. For nested relations this can produce row counts that dwarf the original problem. Prefer batched separate queries (`WHERE parent_id IN (...)`) over mega-joins for multi-level relations.

### Adding An Index On The Wrong Column Or In The Wrong Order

Indexing `created_at` for a query that filters by `tenant_id` and `status`, or building a composite index `(status, tenant_id)` when every query filters tenant first. The index exists but is never used, while the query still seq-scans. Match index column order to real predicate order.

### Assuming Offset Pagination Scales

Using `OFFSET N` for an infinite feed or export and discovering it slows to a crawl at depth, because each page re-scans all prior rows. Deep or unbounded traversal needs keyset pagination; offset is only safe for shallow browsing.

### Optimizing Query Count While Ignoring Per-Query Cost

Reducing 50 small queries to 1 query that seq-scans a million-row table and sorts in memory. Fewer queries, but slower. Always re-check the execution plan after reducing query count.

### Testing Only With Tiny, Uniform Data

Validating a query path against ten evenly-distributed rows and shipping it, then discovering at production scale that the index is not selective for the largest tenant, or that pagination depth destroys latency. Test against realistic volume and skew.

### Doing In Code What The Database Should Do

Loading all rows into the application to filter, sort, count, or group them. This moves work out of the indexed, optimized database and into unindexed application memory, and it often forces loading far more data than the result requires. Push filtering, aggregation, and ordering to the database; keep in the application only what must be computed there.

## Self-Check

- [ ] The query count per request is constant (or near-constant) and does not scale with the number of rows returned; no N+1 or nested N+1+M patterns remain, verified against a query log.
- [ ] Eager versus lazy loading was chosen by access pattern: relations the response always renders are eager-loaded at the fetch site, not lazy-loaded by default.
- [ ] Bulk reads use batched queries (`WHERE id IN (...)`) rather than per-row fetches in a loop; relations are prefetched in batches and grouped in memory.
- [ ] The execution plan for each hot query was read and confirms index usage, selective scans, and no large filesorts or temp tables; the plan was re-checked after any change.
- [ ] Indexes match real predicates with correct column order (equality columns first, then range/sort), cover tenant/foreign-key join paths, and serve the actual sort order.
- [ ] Pagination strategy matches depth: offset only for shallow browsing, keyset/cursor for deep or unbounded traversal, with explicit handling of newly inserted rows.
- [ ] Query count was reduced without introducing over-fetching, Cartesian row multiplication, or moving aggregation/filtering into unindexed application code.
- [ ] The query path was validated against realistic data volumes and distribution (large tenants, long history, skew), not only tiny uniform test data.
- [ ] Work is placed in the right layer: filtering, joining, and aggregation in the database; only genuinely application-side computation in memory.
- [ ] No query path silently degrades as data grows — endpoint latency, query count, and index selectivity were checked at scale, not only at development sample size.
