---
name: search_index_design.md
description: Use when the agent is designing or reviewing a search index (Elasticsearch, OpenSearch, Solr, Meilisearch, Typesense, Postgres full-text, or a vector store), choosing which fields to index and which to store, deciding analyzer/tokenizer chains, mapping field types (text vs keyword vs numeric vs dense_vector), structuring the inverted index, sizing indexes against query latency, or planning reindexing and index lifecycle. Also covers the tradeoffs between index size and search speed, the cost of over-indexing, the dangers of the dynamic mapping default, multifield mappings, exact-match versus full-text fields, denormalization for search, and the recurring mistake of treating a search engine like a primary datastore.
---

# Search Index Design

A search index is a derived, denormalized projection of your data built for one job: fast retrieval by relevance. It is not a normalized source of truth, not a relational store, and not a place to enforce invariants. The judgment problem is deciding, for each field, whether it exists for matching, for sorting, for aggregating, for highlighting, or merely for display — because those four purposes demand different storage and analysis, and conflating them is the source of most search performance and correctness problems. An index that indexes everything looks safe but balloons in size and slows every query; an index that indexes too little cannot serve the filters and sorts the UI needs.

Agents tend to miss these problems because the defaults are forgiving: a search engine with dynamic mapping happily ingests any JSON, and a query against ten documents returns instantly. The harm appears only at scale and only in specific ways. The `keyword` field that was never declared means the UI's category filter runs a full-text query and matches substrings. The field indexed with the standard analyzer means a search for `smith` matches `blacksmith`. The numeric field indexed as text means sorts return lexicographic order. The index that stored the full document body is four times larger than needed and reindexing takes a day. The judgment problem is to treat index design as a deliberate contract — what each field is for, how it is analyzed, and what the query patterns actually require — rather than letting the engine infer a schema that happens to work on sample data.

This skill covers field mapping decisions, analyzer chains, the index-size-versus-query-speed tradeoff, denormalization, and index lifecycle. It complements the relevance-ranking skill (which covers scoring once results are retrieved) and the multilingual-search skill (which covers language-specific analysis).

## Core Rules

### Map Each Field By Its Query Purpose, Not By Its Type

The single most important decision is what each field is *for*, because that determines its type and analysis. The same underlying value often needs to be mapped several ways to serve different queries:

- **Full-text matching (relevance-ranked search).** Map as `text` with an analyzer that tokenizes and normalizes. Used for the main search box where partial and fuzzy matches should rank by relevance.
- **Exact / structured matching (filters, facets, joins).** Map as `keyword` (not analyzed). Used for `category`, `status`, `tags`, foreign-key-like ids, and any `term` filter or `terms` aggregation. A field used for faceting or filtering that is only `text` will silently do full-text matching and return wrong results.
- **Sorting and range queries.** Map as a numeric, date, or `keyword` type with `doc_values` enabled. Sorting on an analyzed `text` field is either disallowed or meaningless.
- **Aggregations (facets, metrics).** Require aggregatable types (`keyword`, numeric, date); `text` fields are not aggregatable by default.
- **Display only.** Store or fetch from the source, but do not index. A long description rendered in the result card needs to be retrievable, not searchable or sortable.

The strong pattern is the **multifield**: declare one physical field as both `text` (for search) and `keyword` (for filtering/sorting/aggregation), e.g. `title` (`text`) and `title.keyword` (`keyword`). The weak pattern is relying on the engine's default, which for many engines makes a string both analyzed and keyword-indexed — convenient, but it hides which subfield a given query should use, and analysts reach for the wrong one.

### Choose The Analyzer Chain To Match The Matching Semantics You Want

Analysis is what turns a document into the tokens the inverted index stores, and what turns a query into the tokens it looks up. Mismatched analysis between index time and query time is the most common cause of "the search doesn't find what it obviously should":

- **Standard analyzer.** Splits on word boundaries, lowercases, removes punctuation. The sane default for most Latin-script full text.
- **Keyword / no analysis.** The whole value is one token. Required for ids, codes, and exact filters; using analysis here breaks equality.
- **Language-specific analyzers.** Apply stemming, stopword removal, and decompounding tuned to a language. A field searched in English should use the English analyzer; searching German text with the standard analyzer loses stemming and compounds.
- **N-gram / edge n-gram.** For prefix or substring search-as-you-type. Index-time n-grams trade a much larger index for fast partial matching; use `search_analyzer` set to `standard` so the query is not re-n-grammed.
- **Custom chains.** Combine char filters (e.g., HTML stripping), tokenizers, and token filters (synonyms, lowercase, asciifolding, stop) in the order the engine applies them. Synonyms usually belong at query time (or as a `search_analyzer`), not index time, so they can be changed without reindexing.

Write down, for each searchable field: what tokens does indexing produce, and what tokens does the query produce? If they diverge (e.g., index stems but query does not), matches will be lost. Test the analyzer on real values with the engine's analyze API, not on intuition.

### Separate Search Fields From Filter Fields Explicitly

A recurring bug is using a `text` field where a `keyword` field is required, or vice versa. Make the distinction structural:

- **Filter fields are `keyword`/numeric/date and queried with `term`/`terms`/`range`.** These are boolean predicates; they do not contribute to relevance score (or contribute a constant). Examples: `tenant_id`, `is_published`, `category_id`, `price`, `created_at`.
- **Search fields are `text` and queried with `match`/`multi_match`.** These contribute to relevance and tolerate tokenization differences.
- **Never run a `term` query against an analyzed `text` field.** A `term` query for `category: "Books"` on an analyzed field stores the token `books`, so the query for `Books` finds nothing. Either query the `.keyword` subfield or re-map.

Declare the filter fields as `keyword` up front. The weak choice is discovering, after launch, that the category filter "sometimes returns too many results" because it was full-text matching substrings.

### Manage Index Size Deliberately — Every Enabled Feature Costs

Search index size is not free. It costs disk, memory (the OS cache that makes queries fast), reindexing time, and snapshot/restore time. Every feature you enable grows the index, and the growth is multiplicative across fields:

- **`index: true` (inverted index)** is required for search but unnecessary for fields only used for retrieval or sorting. Disable it for display-only fields.
- **`doc_values`** enables sorting, aggregations, and scripting. It is on by default for most types; disable it for fields you will never sort or aggregate to save space — but be sure, because re-enabling requires reindexing.
- **`norms`** store length normalization used in relevance scoring. Disable them for fields whose score should not depend on length (e.g., log-style fields, fields with uniform length) to save space.
- **`store: true` vs `_source`.** Storing individual fields duplicates data already in `_source`; prefer `_source` fetching and disable per-field store unless you need to fetch a single large field cheaply.
- **N-grams and synonyms at index time** can multiply index size several-fold. Prefer query-time synonyms; measure index size before committing to index-time n-grams.

The discipline: for each field, ask "do I search it, sort it, aggregate it, or only display it?" and enable only the structures that purpose requires. An index where every field has every feature enabled is two to four times larger than it needs to be, and slower because less of it fits in the OS cache.

### Denormalize For Search, But Own The Synchronization

Search engines are not relational; joins are limited or absent. To filter and sort by attributes that live in related entities (the product's category name, the author's reputation), you must denormalize — copy the needed attributes into the indexed document at index time. This is correct and expected, but it creates a synchronization obligation that agents often overlook:

- **Denormalize query-critical related fields into the document.** If users filter products by category name, store `category_name` on the product document, do not try to join at query time.
- **Define how related changes propagate.** When a category is renamed, every product in it must be reindexed, or search will show stale category names. Decide the propagation mechanism: synchronous reindex on change, a change-data-capture pipeline, or periodic full reindex. The wrong default is "nothing," and the index drifts silently from the source.
- **Avoid deep denormalization of volatile data.** If a related attribute changes every minute (a live stock count, a real-time price), denormalizing it into the search index makes the index constantly stale. Keep volatile filters in the source system and apply them as a post-filter, or accept a defined staleness window.

State explicitly, for each denormalized field: where does its value come from, and what event updates it in the index? A denormalized field with no update path is a future bug.

### Plan Index Lifecycle: Aliases, Reindexing, And Versioning

An index mapping is not immutable, but most mapping changes (changing a field type, switching an analyzer) cannot be applied in place — they require creating a new index and reindexing. Design for this from the start:

- **Query through aliases, never the concrete index name.** An alias (`products`) points at the current index (`products_v3`); to reindex, build `products_v4`, reindex into it, then flip the alias. Clients are unaware of the swap.
- **Version indices.** Name them with a version or date suffix so reindexing and rollback are first-class operations, not emergencies.
- **Make reindexing routine and tested.** A reindex that has only ever been run once will fail when you need it. Reindex into a shadow index periodically, measure duration, and verify document counts and sample queries match.
- **Size reindexing against real volume and the source load.** Reindexing reads the entire source; on a hot database this can cause load incidents. Throttle, batch, and run outside peak where it matters.

The weak pattern is a single unversioned index with clients pointing at it directly, so the first mapping change becomes a risky, unplanned migration.

### Treat The Search Engine As A Derived Store, Not The Source Of Truth

A search index can be lost and rebuilt from the primary store. This is not a weakness; it is the correct architecture, and it should shape every decision:

- **The primary database owns invariants, transactions, and authority.** The search index owns fast retrieval. Do not write business-critical state only to the search engine.
- **Make the index rebuildable.** If the index were deleted, could you reconstruct it from the source in a known time? If not, you have leaked authority into the index. Keep the source complete enough to rebuild.
- **Do not use the search engine for transactional writes or strong consistency.** Indexing is eventually consistent with the source; a write visible in the database may not be searchable for milliseconds to seconds. Design the read path around this (accept brief staleness, or read-through for freshly written records).

## Common Traps

### Trusting Dynamic Mapping On Production Data

Letting the engine infer types from the first document it sees, so a field that happened to be a number in the first record is mapped numeric and rejects later string values, or a field mapped as `text` is later needed as `keyword`. Define an explicit mapping before loading real data.

### Indexing Every Field "To Be Safe"

Enabling search, sort, and aggregation on every field, producing an index two to four times larger than necessary, slower queries (less fits in cache), and painful reindexes. Index, sort, and aggregate only the fields each query pattern actually needs.

### Using A `text` Field For A Filter Or Facet

Running a `term` filter or `terms` aggregation on an analyzed `text` field, so a category filter matches substrings or returns nothing. Declare `keyword` subfields and use them for filters and facets.

### Mismatched Index-Time And Query-Time Analysis

Indexing with stemming or synonyms but querying with the standard analyzer (or vice versa), so tokens never match and results are silently missing. Verify with the analyze API that index and query tokens align.

### Index-Time Synonyms That Force Reindexing For Every Change

Loading synonyms at index time, so every synonym edit requires a full reindex. Apply synonyms at query time (via `search_analyzer`) so they can be tuned without reindexing.

### Sorting Or Aggregating An Analyzed Field

Trying to sort by a `text` field and getting an error or lexicographic-on-tokens nonsense. Sort and aggregate on `keyword`, numeric, or date fields with `doc_values`.

### Denormalizing Without An Update Path and n-Gramming Everything For "Fuzzy" Search

Copying a related attribute into the document for filtering, but never reindexing when the related entity changes, so the index shows stale category names or prices forever. Define the propagation event for every denormalized field.

Applying n-gram analyzers to all text fields to get substring matching, multiplying index size and degrading relevance (every short token matches too much). Use n-grams only on specific search-as-you-type fields with a non-n-gram `search_analyzer`.

### Pointing Clients At The Concrete Index Name and treating The Search Index As Authoritative State

Hardcoding the concrete index name, so a mapping change forces a coordinated migration with downtime instead of an alias swap. Always query through an alias.

Writing records only to the search engine and losing them when the index is rebuilt, or relying on strong consistency from an eventually-consistent index. Keep the primary store authoritative and the index rebuildable.

## Self-Check

- [ ] Each field is mapped by its query purpose: `text` for relevance search, `keyword`/numeric/date for filters/sorts/aggregations, and unindexed for display-only; multifields declare both `text` and `.keyword` where both search and filter/sort are needed.
- [ ] No filter, facet, or sort runs against an analyzed `text` field; filter fields are explicitly `keyword` and queried with `term`/`terms`/`range`.
- [ ] The analyzer chain was tested with the engine's analyze API on real values, and index-time and query-time analysis produce matching tokens (including stemming, synonyms, and normalization).
- [ ] Synonyms are applied at query time (or with a separate `search_analyzer`) so they can be changed without reindexing, unless index-time synonyms are a deliberate, measured choice.
- [ ] Index size was considered: `index`, `doc_values`, `norms`, and `store` are enabled only where the query pattern requires them, and n-gram fields are scoped to specific search-as-you-type use cases.
- [ ] Every denormalized related field has a defined update path (the event that reindexes it when the source changes); volatile attributes are handled as post-filters or with an explicit staleness window rather than constantly reindexed.
- [ ] Clients query through an alias, indices are versioned, and reindexing is a tested, routine operation sized against real volume and source load — not an unplanned emergency.
- [ ] The search index is treated as a derived, rebuildable projection: the primary store is authoritative, the index can be reconstructed from source, and the read path accounts for indexing latency rather than assuming strong consistency.
- [ ] The highest-risk cases were verified — dynamic mapping on production data, a `term` query against an analyzed field, mismatched analysis, an unversioned index, and a denormalized field with no update path — not only the clean sample-data happy path.
