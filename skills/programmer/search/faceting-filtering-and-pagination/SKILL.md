---
name: faceting_filtering_and_pagination.md
description: Use when the agent is designing a search or listing UI's facets (category, brand, price range, tags), filters, and pagination; deciding between query and filter context; building aggregation-based facets with correct counts; handling range and hierarchical facets; choosing offset versus cursor/keyset pagination for search results; solving deep pagination performance problems; or designing filter combinations that stay correct under conjunction and disjunction. Also covers the trap of facet counts that ignore active filters, the cost of computing facets over large result sets, the deep-pagination latency cliff, and the difference between a filter that narrows results and a query that contributes to scoring.
---

# Faceting, Filtering, And Pagination

Facets, filters, and pagination are the navigation layer on top of search: they let users narrow a result set, see what options exist, and move through it. Each looks mechanical but carries a correctness and performance trap. Facet counts that do not reflect the active filters mislead users into dead ends. A filter placed in query context silently contributes to the relevance score and changes ranking. Offset pagination that works on page 2 collapses at page 500. The judgment problem is that these features interact — a facet's count depends on which filters are active, a filter's correctness depends on whether it is a hard predicate or a scored term, and pagination's cost depends on how deep the user goes — and getting any one of them wrong produces a UI that feels broken or an endpoint that falls over under real usage.

Agents tend to miss these problems because the happy path is easy: a handful of filters, a few facets, shallow pages, and everything returns instantly and looks right. The harm appears at the edges. A user applies three filters and the brand facet still shows counts for brands that no remaining product has, because the facet was not scoped to the active filters. A "load more" feed uses offset pagination and the 200th page takes seconds because the engine scans and discards twenty thousand prior hits. A numeric filter accidentally placed in the `must` (query) clause boosts documents instead of filtering them, so results that should be excluded appear with a high score. The judgment problem is to treat faceting, filtering, and pagination as a coordinated system with explicit semantics, not three independent UI widgets.

This skill covers filter vs query context, facet design and scoping, and pagination strategy. It complements the search-index-design skill (which covers making fields aggregatable) and the query-optimization skill (which covers keyset pagination for databases).

## Core Rules

### Distinguish Filter Context From Query Context Explicitly

Most search engines separate two ways a clause affects results: **query context** (contributes to the relevance score; a match makes a document rank higher but does not exclude non-matches) and **filter context** (a hard yes/no predicate that includes or excludes documents without affecting score). Confusing them is the most common filtering bug:

- **Filters belong in filter context.** `category`, `is_published`, `tenant_id`, `price range`, `date range`, and `in_stock` are boolean predicates. Put them in the `filter` clause (or `post_filter` where appropriate) so they exclude non-matches and do not pollute the relevance score.
- **Search terms belong in query context.** The user's free-text query is the one thing that should score. Putting it in filter context would make every matching document score equally, destroying ranking.
- **The symptom of confusion.** A filter that "sometimes lets non-matching results through with a high score" is almost always a filter placed in `must` (query) context, where it boosted instead of excluded. A search where all results have identical scores is almost always a query placed in filter context.

Decide for each clause: does this narrow the result set (filter context) or rank within it (query context)? Almost every structured field is a filter; almost every free-text field is a query.

### Scope Facet Counts To All *Other* Active Filters

A facet shows the user "if you add this filter, how many results remain?" For that count to be honest, it must be computed against the result set that *excludes the facet's own active value but includes every other active filter*. This is the standard faceting contract, and it is easy to get subtly wrong:

- **Each facet is computed against the full filtered set minus its own contribution.** If the user has filtered `category=Shoes` and `color=Red`, the `brand` facet counts brands among red shoes; the `color` facet counts colors among shoes (ignoring the red selection so the user can see alternatives); the `category` facet counts categories among red items (ignoring the shoes selection).
- **Implement with the active filter excluded per-facet, not globally.** Most engines support this via per-aggregation filter clauses or by computing each facet against a filtered query that omits that facet's active value. Naively aggregating against the fully-filtered set makes every selected facet show only its own count and every other facet show only the filtered subset, hiding alternatives.
- **Range and hierarchical facets follow the same rule.** A price-range facet's buckets are counts among the other-filtered set; a hierarchical category facet shows sibling counts at the relevant level.

The weak pattern: one global filter applied to all facets, so selecting `color=Red` makes the `color` facet show only `Red (142)` and hide every other color, trapping the user in their own selection.

### Bound Facet Cardinality And Computation Cost

Facets are aggregations, and aggregations over large result sets are expensive. Unbounded facets are a common source of slow queries and out-of-memory errors:

- **Limit facet cardinality.** A `terms` aggregation on a high-cardinality field (user id, product sku) returns a huge bucket list and burns memory. Cap with `size` on the aggregation and prefer low-cardinality fields (category, brand) for facets.
- **Avoid computing facets over unbounded result sets.** A facet over "all documents" with no filter scales with the index. Always pair facets with at least the tenant/scope filter, and consider whether facets on a multi-million-doc query are needed at all.
- **Use doc_values, not fielddata.** Aggregations require the field to be aggregatable (`keyword`, numeric, date with `doc_values`). Running an aggregation on a `text` field loads fielddata into the heap and is both slow and a known out-of-memory risk.
- **Consider cached vs uncached filters.** Frequently-used filters (tenant, category) are often cached by the engine; one-off filters are not. Profile real facet queries, not synthetic ones.

Decide, for each facet: what is its cardinality, what result set is it aggregating over, and is that set bounded? A facet with no bound on cardinality or result-set size is a future incident.

### Choose Pagination Strategy By Depth, Not By Default

Pagination cost is not uniform. The two strategies have very different depth profiles, and the wrong choice surfaces only when a user goes deep:

- **Offset pagination (`from`/`size` or `LIMIT`/`OFFSET`).** Simple, stateless, supports "jump to page N." But to return page 500, the engine must collect and discard the first 500×size hits; cost grows linearly with depth, and many engines cap `from + size` (often at 10,000) precisely because deep offset is catastrophically expensive. Fine for shallow browsing; broken for infinite scroll, exports, or background traversal.
- **Cursor / search-after pagination.** Return the sort key of the last result; the next page queries `WHERE (sort_key) > (last_key)`. Cost is constant regardless of depth because it seeks rather than scans-and-discards. Required for any unbounded traversal. It needs a unique, stable sort key (typically `(score, id)` or `(created_at, id)`) and does not support "jump to page N."
- **Scroll API (not the same as search-after).** A server-side cursor snapshot meant for bulk export, not real-time pagination; it holds a snapshot and is expensive to maintain. Use only for one-off exports, never for user-facing pagination.

Match the strategy to the access pattern. A product listing browsed a few pages deep can use offset. An infinite feed, an export, or a background scan must use search-after/cursor, or it will hit the depth cap or degrade catastrophically.

### Decide Sort Stability And Insert Handling Explicitly

Pagination correctness depends on a stable, unique ordering. If two results tie on the sort key, pages can skip or duplicate items as the user paginates:

- **Always append a unique tiebreaker.** Sort by the primary key plus a unique field (usually `id`) so no two results are truly tied. Sorting by `created_at` alone means equal timestamps cause items to shift between pages.
- **Handle inserts during pagination.** With offset pagination, a new insert at the top shifts every subsequent page, so the user sees a duplicate or skips an item. With cursor pagination, inserts do not affect already-emitted pages but the new item will not appear (acceptable for most feeds). State the behavior explicitly; for offset, consider a `stable` snapshot or accept the shift; for cursor, document that recent inserts appear only on a fresh query.

### Treat Post-Filter And Pre-Filter As A Deliberate Performance Choice

When filters are expensive (geo distance, script-based, nested), applying them before faceting forces every facet to be computed over the filtered set, which can be slow. Engines offer two placements:

- **Pre-filter (in the `filter` clause).** The filter narrows the result set before facets and pagination are computed. Facet counts reflect the filter; this is correct but can be slow if the filter is expensive.
- **Post-filter (`post_filter`).** The filter is applied only to the returned hits, after facets are computed. Faster, but facet counts do *not* reflect the post-filter, which can mislead users (a facet shows a count for items the post-filter excludes).

Use post-filter only when the filter is too expensive to pre-filter and the misleading facet counts are acceptable (or when the filter is a UI-only concern like "hide out-of-stock"). The default should be pre-filter for correctness; reach for post-filter as a measured optimization, not a habit.

## Common Traps

### A Filter In Query Context Boosting Instead Of Excluding

Placing `price < 100` in the `must` (query) clause, so documents over $100 still appear but rank lower, instead of being excluded. Put structured predicates in the `filter` clause.

### Facet Counts That Ignore Active Filters

Computing facets against the unfiltered index, so a user who filtered `category=Shoes` sees a brand facet with counts for brands that have no shoes. Scope each facet to all other active filters.

### A Facet Trapped By Its Own Selection

Applying the global filter (including the facet's own value) to that facet's aggregation, so selecting `color=Red` makes the color facet show only `Red` and hide alternatives. Exclude each facet's own active value from its own aggregation.

### High-Cardinality Facet Burning Memory

Running a `terms` aggregation on a user-id or sku field with no `size` cap, returning huge bucket lists and risking out-of-memory. Cap facet cardinality and prefer low-cardinality fields.

### Offset Pagination Into The Depth Cap

Using `from`/`size` for an infinite feed or export and hitting the engine's 10,000-hit depth cap or degrading to multi-second latency at depth. Use search-after/cursor for unbounded traversal.

### Unstable Sort Causing Skipped Or Duplicated Items

Sorting by a non-unique key (`created_at` alone) so ties shift items between pages as the user paginates. Always append a unique `id` tiebreaker.

### Aggregating A `text` Field Via Fielddata

Running a facet on an analyzed `text` field, loading fielddata into the heap and risking out-of-memory. Facet on `keyword`/numeric/date fields with `doc_values`.

### Post-Filter Hiding Mismatched Facet Counts and new Inserts Shifting Offset Pages

Using `post_filter` for a user-facing filter so facet counts include items the filter excludes, misleading the user. Prefer pre-filter for correctness; reserve post-filter for measured optimizations.

Offset pagination where a new insert at the top shifts every page, causing duplicates or skips, with no documented behavior. Use cursor pagination for feeds, or document the offset behavior.

## Self-Check

- [ ] Structured predicates (category, status, tenant, price/date range, in_stock) are in filter context (excluded if not matched, no score contribution), and only free-text search is in query context; no filter is accidentally boosting non-matches.
- [ ] Each facet's counts are scoped to all *other* active filters (each facet excludes its own active value), so users see honest counts for alternatives and are not trapped by their own selection.
- [ ] Facet cardinality is bounded (`size` cap), facets run on aggregatable fields (`keyword`/numeric/date with `doc_values`, never `text` via fielddata), and no facet aggregates over an unbounded result set.
- [ ] Pagination strategy matches depth: offset only for shallow browsing with awareness of the depth cap, and search-after/cursor for infinite feeds, exports, or background traversal.
- [ ] The sort key has a unique tiebreaker (primary key plus `id`) so pagination never skips or duplicates items on ties, and insert-during-pagination behavior is documented.
- [ ] Post-filter is used only as a measured optimization where misleading facet counts are acceptable; the default is pre-filter for correctness.
- [ ] Facet and pagination queries were profiled against realistic result-set sizes and cardinalities, not only the small-sample happy path.
- [ ] The highest-risk cases were verified — a filter in query context, a facet ignoring active filters, a facet trapped by its own selection, offset pagination hitting the depth cap, an unstable sort, and a high-cardinality aggregation — not only the shallow, few-filter demo path.
