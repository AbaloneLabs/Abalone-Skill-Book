---
name: indexing_strategy.md
description: Use when the agent is choosing database indexes, ordering composite index columns, designing covering or partial indexes, weighing write-cost tradeoffs, diagnosing unused or misused indexes, or tuning query plans for relational and document stores.
---

# Indexing Strategy

Indexes are the single most powerful performance lever in a database, and also one of the easiest to misuse. The instinct to "add an index on every column I query" feels safe but silently taxes every write, bloats storage, and can still leave queries slow if the index does not match the actual access pattern. Conversely, the instinct to index nothing keeps writes fast and makes reads catastrophically slow as data grows. Good indexing is not about adding indexes; it is about matching indexes to real query shapes and understanding the full cost.

The judgment problem is deciding which queries deserve an index, how to order columns within a composite index so it serves the widest set of queries, when a covering or partial index earns its cost, and how to detect indexes that exist but are never used or that the planner refuses to use. The agent should not treat indexing as a checklist applied after the fact; it should be designed alongside the query patterns.

This skill applies whenever you are designing schema, reviewing slow queries, planning migrations that add or remove indexes, or diagnosing queries that should be fast but are not.

## Core Rules

### Index for actual query shapes, not for columns

An index is only useful if a query's predicates match its leading columns. Before adding an index, enumerate the real queries: which columns appear in `WHERE`, which in `JOIN`, which in `ORDER BY`, and which are projected in `SELECT`. An index on `(last_name)` does not help a query that filters on `email`; an index on `(status, created_at)` helps queries that filter on `status` and optionally sort on `created_at`, but not queries that filter on `created_at` alone.

Weak choice: indexing every column individually because each is sometimes queried. Strong choice: a small set of composite indexes whose leading columns match the most common filter combinations.

### Order composite index columns by selectivity, equality, then range

The order of columns in a composite index determines which queries it can serve. The widely useful heuristic is:

- **Equality columns first**: columns used with `=` predicates go leftmost, because the index can seek directly to those entries.
- **Range columns last**: columns used with range predicates (`<`, `>`, `BETWEEN`) go after equality columns, because once a range is applied, subsequent columns cannot be used for index seeks (only for covering).
- **Sort columns**: if the query sorts, matching the index order to the sort lets the database avoid a separate sort step.
- **Tiebreaker**: append the primary key or another unique column as a final tiebreaker so the order is deterministic, which also helps pagination.

Selectivity matters but is secondary to the equality-before-range rule. A low-selectivity equality column (e.g., `status`) still belongs before a high-selectivity range column (e.g., `created_at`) when the query filters on both.

### Consider covering indexes to avoid table lookups

When an index contains all the columns a query needs (filter columns plus selected columns), the database can satisfy the query from the index alone without fetching the row (a "covering" index, using `INCLUDE` clauses in some databases). This dramatically reduces I/O for hot read paths. The tradeoff is index size and write cost: every included column increases the index footprint and must be updated on every write to that column. Use covering indexes for high-frequency read paths where the extra write cost is justified, not for every query.

### Use partial indexes to shrink and focus indexes

If a large fraction of queries only ever target a subset of rows (e.g., `WHERE active = true`, or `WHERE deleted_at IS NULL`), a partial/index-condition index covers only those rows. It is smaller, faster to scan, and cheaper to maintain on writes to other rows. Partial indexes are especially valuable for "soft delete" or status-flag patterns where most queries skip inactive rows.

### Account for write cost, not just read benefit

Every index adds cost to every `INSERT`, `UPDATE` (on indexed columns), and `DELETE`. The cost is proportional to the number of indexes and their fanout. A table with ten indexes may see write throughput collapse under load even though reads are fast. When proposing an index, ask: how many writes per second hit this table, and how many indexes does this add to the write path? Indexes that benefit rare queries may not earn their write cost. Measure, do not assume.

### Verify the planner actually uses the index

An index that exists but is never used is pure overhead. After adding an index, confirm with `EXPLAIN`/`EXPLAIN ANALYZE` that the planner selects it for the target queries. Planners sometimes reject indexes due to stale statistics, type mismatches (e.g., comparing a string column to a numeric literal prevents index use), function-wrapped predicates (`WHERE LOWER(email) = ...` needs a function-based/expression index), or because a sequential scan is genuinely cheaper for small tables. Build expression indexes for queries that wrap columns in functions.

### Plan index maintenance

Indexes are not set-and-forget. Over time, indexes accumulate bloat (especially B-trees after heavy deletes), statistics drift, and some indexes become unused as query patterns shift. Establish a review cadence: identify unused indexes (via database monitoring of index reads), identify duplicate or prefix-redundant indexes (one index whose leading columns are a prefix of another), and rebuild/reorganize bloated indexes during maintenance windows.

## Common Traps

### Indexing every column individually

Single-column indexes on every queried field rarely match real query shapes, which usually combine multiple predicates. They multiply write cost without helping the composite queries that matter. The trap is that each looks justified in isolation.

### Wrong column order in a composite index

An index on `(created_at, status)` does not help a query filtering on `status` alone, because `status` is not a leading column. The order must match how queries filter. Reversing the order silently makes the index useless for the intended query.

### Function-wrapped predicates that defeat the index

`WHERE DATE(created_at) = '2024-01-01'` or `WHERE LOWER(email) = 'x'` cannot use a plain index on the column, because the function transforms the key. The trap is writing the predicate the intuitive way and getting a full scan. Use range predicates (`created_at >= ... AND created_at < ...`) or expression indexes.

### Ignoring write cost until production degrades

Reads look great in dev because the dataset is small and writes are infrequent. In production, the write amplification from many indexes surfaces as latency spikes and reduced throughput. Always size the write cost against expected write volume.

### Assuming a sequential scan is always bad

For small tables, or for queries returning a large fraction of rows, a sequential scan is faster than using an index (which requires random I/O to fetch each row). Forcing index use can be slower. Trust the planner when the table is small, and focus indexing effort on large tables and selective queries.

### Keeping unused or redundant indexes "just in case"

Unused indexes cost writes forever. Prefix-redundant indexes (where one index's columns are a leading prefix of another) are almost always removable. The trap is fear of removing something that might be used; monitoring tells you the truth.

### Type mismatches silently disabling index use

Comparing an indexed `VARCHAR` column to a numeric value, or a column with one collation to a literal with another, can prevent index use. The query works (returns correct rows) but scans. This is invisible without checking the plan.

## Self-Check

- Does each index correspond to a real, observed query shape, with leading columns matching equality predicates and trailing columns matching ranges and sorts?
- Are composite index columns ordered equality-first, then range, then sort, with a unique tiebreaker for determinism?
- Have you considered covering indexes (with `INCLUDE`) for high-frequency read paths, and justified their write cost?
- Are partial indexes used where a large fraction of queries target a stable row subset (soft deletes, status flags)?
- Have you accounted for write cost per index against the table's expected write volume, not just read benefit?
- Did you confirm with `EXPLAIN ANALYZE` that the planner actually uses each new index for its target queries?
- Are function-wrapped predicates handled with expression indexes or rewritten as range predicates?
- Is there a process to identify and remove unused, duplicate, or prefix-redundant indexes?
- Are index statistics kept up to date so the planner makes good decisions?
- For queries that scan, have you confirmed the scan is justified (small table or large result set) rather than a missed index?
