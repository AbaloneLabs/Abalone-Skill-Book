---
name: pagination_filtering_and_sorting.md
description: Use when the agent is designing list endpoints, choosing cursor versus offset pagination, handling deep pagination, building compound filters, ensuring sort stability, or reasoning about cardinality and query performance for collection resources.
---

# Pagination, Filtering, and Sorting

List endpoints look simple and are one of the most common sources of correctness and performance bugs in an API. The surface decision ("return an array and a page number") hides a cascade of harder questions: what happens when a user pages past millions of rows, how filters compose with each other and with sorting, whether sort order is deterministic when keys collide, and how to keep a paginated view consistent while data changes underneath it. A naive offset-based design works in a demo and degrades catastrophically in production.

The judgment problem is choosing a pagination model that matches the access patterns and the data volume, defining filter and sort semantics that are predictable and composable, and avoiding the failure modes that only appear at scale or under concurrent modification. The agent should not default to `?page=3&size=20` without considering whether the dataset and the clients actually fit that model.

This skill applies whenever you are designing, reviewing, or refactoring collection endpoints, search endpoints, export pipelines, or any interface that returns a bounded slice of a larger set.

## Core Rules

### Match the pagination model to the access pattern

There are three common models, and they have very different cost profiles:

- **Offset/limit pagination** (`?page=3&size=20`): simple, supports jumping to arbitrary pages, but expensive on large datasets because the database must skip over all preceding rows. Deep pages get linearly slower. It is also unstable under concurrent writes: if a row is inserted before the current page, the next page shifts and a row is skipped or duplicated.
- **Cursor/keyset pagination** (`?after=eyJpZCI6MTIzfQ`): the client passes an opaque cursor encoding the last seen row's sort key. The next page is fetched with a `WHERE key > cursor` predicate, which is constant-cost regardless of depth when backed by an index. It is stable under inserts before the cursor. The cost is that you cannot jump to arbitrary pages, and cursors are tied to a specific sort order.
- **Seek with a compound key**: a refinement of cursor pagination for non-unique sort fields, where the cursor must encode the full sort tuple (including a unique tiebreaker like `id`) to remain deterministic.

Choose offset pagination only when the dataset is small and bounded, or when random page access is a hard product requirement. Choose cursor pagination for unbounded or large collections, infinite-scroll UIs, and export pipelines. Weak choice: offset pagination on a table expected to grow to millions of rows. Strong choice: keyset pagination with an indexed compound cursor.

### Guarantee sort stability with a unique tiebreaker

Sorting by a non-unique field (e.g., `created_at`) produces undefined order among rows with equal keys. Without a deterministic tiebreaker, pagination can skip or duplicate rows between pages, because the database is free to return equal-keyed rows in any order, and that order can change between queries. Always append a unique column (typically the primary key) as the final sort key so the total order is deterministic. This is mandatory for cursor pagination and prevents flaky results in offset pagination too.

### Define filter semantics explicitly

Filters are a contract. Decide and document:

- **Combinatorics**: do multiple filters AND together, or do some support OR? Most APIs default to AND, but search endpoints often need OR within a field (`status=open OR status=pending`). Make this explicit.
- **Operators**: equality, range (`gt`, `gte`, `lt`, `lte`), membership (`in`), substring/prefix, and null checks. Choose a consistent naming convention rather than inventing per-field syntax.
- **Field eligibility**: not every field is filterable. Filtering on an unindexed high-cardinality field causes full scans. Maintain an allowlist of filterable fields and document which are indexed.
- **Unknown-field behavior**: should an unknown filter be ignored (lenient) or rejected with `400` (strict)? Strict is safer because it catches client typos that would silently return unfiltered results.

Weak choice: silently ignoring unrecognized filter parameters, so a typo in `statu=open` returns all rows. Strong choice: rejecting unknown filters with a clear error.

### Keep cursors opaque and self-describing

A cursor should be an opaque, signed/encoded token (typically base64 of the sort tuple plus optional context like the filter hash). Encoding the filter signature in the cursor lets the server detect when a client changes filters between pages and reject or reset, preventing inconsistent results. Do not expose raw IDs or internal row pointers in cursors; they leak structure and invite tampering.

### Reason about deep pagination explicitly

Deep pagination (page 50,000 of 20-row pages) is where offset pagination collapses. Decide up front how the endpoint behaves at depth:

- For offset pagination on large tables, consider a hard cap (e.g., refuse pages beyond 10,000 rows) and guide clients to cursor pagination or a search/export API instead.
- For export or analytics access patterns, prefer a dedicated streaming or cursor-based endpoint rather than forcing clients to page through millions of rows interactively.
- Document the cost model so clients understand why deep offset pages are slow or refused.

### Make ordering and filtering compose safely

When a client filters and sorts simultaneously, the filter must be applied before pagination, and the sort must be over the filtered set. The trap is an index that supports the sort but not the filter (or vice versa), forcing the database to choose between using an index for one or the other. Design filterable+sortable field combinations with a covering index in mind, and document which combinations are efficient.

### Handle concurrent modification honestly

Paginated views are a snapshot in intent but often not in implementation. With offset pagination, inserts/deletes between page fetches shift results. With cursor pagination on a stable sort key, new rows may or may not appear depending on whether they sort before or after the cursor. Decide and document the consistency guarantee. For UIs that must not show duplicates, consider cursor pagination with a unique tiebreaker; for strict snapshot consistency, consider a snapshot/cursor that pins a read timestamp (at the cost of holding resources).

## Common Traps

### Defaulting to offset pagination without a size analysis

Offset pagination is the path of least resistance and works fine until it does not. The trap is that "works in testing" hides the linear cost of deep pages, which only surfaces in production with real data volume.

### Sorting by a non-unique key without a tiebreaker

`ORDER BY created_at DESC` with no secondary key means rows sharing a timestamp can appear in any order, and pagination across them is non-deterministic. This produces intermittent duplicate/skip bugs that are hard to reproduce.

### Exposing cursors as raw IDs

`?after_id=12345` is not really a cursor; it reveals row structure, invites clients to construct or skip cursors, and breaks when the sort order changes. Opaque encoded cursors are safer and more flexible.

### Silently ignoring unknown filters

Returning all rows because a filter parameter was misspelled is a data-leak and correctness hazard. Lenient parsing feels friendly but masks bugs. Strict parsing catches them early.

### Allowing filters on unindexed high-cardinality fields

A filter like `?email_contains=foo` on an unindexed text column forces a full table scan. The endpoint works in dev and times out in production. Either index the field, restrict it, or document it as unsupported.

### Assuming pagination is consistent across writes

If the dataset changes between page fetches, offset pagination shifts and cursor pagination may surface or hide new rows. Treating the result as a consistent snapshot when it is not leads to duplicate or missing records in the client.

### Letting clients request unbounded page sizes

Accepting `?size=1000000` lets a single request exhaust memory. Always cap the maximum page size and return an error (or clamp with a warning) when exceeded.

## Self-Check

- Is the pagination model (offset vs cursor) chosen based on dataset size and access pattern, not just familiarity?
- Does every sort include a unique tiebreaker column so ordering is deterministic across pages?
- Are cursors opaque, encoded, and (ideally) signed with the filter context so changed filters are detected?
- Is there an explicit, documented policy for filter combinatorics (AND/OR), operators, and unknown-field handling?
- Are filterable fields restricted to an indexed allowlist, with unsupported fields rejected rather than silently ignored?
- Is deep pagination behavior defined: hard caps for offset, or a dedicated cursor/export path for large collections?
- Is the maximum page size enforced server-side?
- Are filter+sort combinations designed with a covering index in mind, with inefficient combinations documented or disallowed?
- Is the consistency guarantee under concurrent writes documented, so clients know whether to expect duplicates or gaps?
- Have you confirmed the chosen conventions are consistent across all list endpoints in the API?
