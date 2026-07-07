---
name: data_warehouse_modeling.md
description: Use when the agent is designing or reviewing the dimensional model of a data warehouse or analytics layer, choosing between star and snowflake schemas, defining fact and dimension tables, modeling slowly changing dimensions (SCD type 1, 2, or 3), designing conformed dimensions and surrogate keys, deciding how much to denormalize for query performance versus storage, or reviewing whether a warehouse is being modeled like an OLTP database. Also covers the failure modes of over-normalizing an analytical warehouse, choosing the wrong grain for a fact table, mishandling historical attribute changes so history is lost or duplicated, and joining on natural keys that corrupt aggregates when source keys change.
---

# Data Warehouse Modeling

A data warehouse is an analytical system optimized for reading, aggregating, and joining large volumes of history across many subject areas. Its model is not the same shape as the transactional database it was sourced from, and treating it as if it were is the single most common modeling mistake. An OLTP database is normalized for fast, conflict-free writes: many narrow tables, heavy normalization, minimal redundancy, designed so a single row update touches one place. A warehouse is the opposite: it is optimized for fast, understandable reads across millions or billions of rows, where analysts write ad-hoc SQL joining facts to dimensions and slicing by many attributes. Copying the source's normalized third-normal-form schema into the warehouse produces a model that is correct for storage and painful for every query — dozens of joins to assemble a single report, slow queries, and a model so intricate that only its author can use it. The judgment problem is choosing a model that serves analytical workloads, where denormalization, dimensional structure, and query performance dominate the design, not write-path normalization.

Agents tend to under-invest in warehouse modeling because the source schema is right there — it looks reasonable, and loading it as-is "works." The harm appears in use: queries that join eight tables to count orders by region; a fact table at the wrong grain that double-counts when joined to a dimension; a dimension that overwrites history so last quarter's report by the old sales-region structure can no longer be reproduced; a natural key reused by the source so a historical join silently maps to the wrong entity. These are modeling failures, not pipeline failures, and they are expensive to fix after the warehouse is populated and consumers depend on it. The discipline is to model deliberately for the analytical workload: choose the fact grain, design dimensions that carry the slicing attributes, decide how history changes, and denormalize where query performance and usability demand it.

This skill covers dimensional modeling: star versus snowflake schemas, fact and dimension design, slowly changing dimensions, conformed dimensions, surrogate keys, and the denormalization tradeoff. It complements the ETL and pipeline skills (which cover moving and quality-checking the data) and the lakehouse skill (which covers storage formats and table architecture). Here the focus is the logical model of the analytical layer.

## Core Rules

### Model For Analytical Reads, Not OLTP Writes

The foundational decision is to abandon source-system normalization as the default and adopt a dimensional model built for analytical queries. In a dimensional model, the warehouse is organized into fact tables (the measurable events — sales, clicks, shipments) and dimension tables (the descriptive context — product, customer, date, geography). A fact row holds numeric measures and foreign keys to dimensions; a dimension row holds the attributes analysts slice by. This star shape — a central fact table surrounded by dimensions — lets a query join fact to a few dimensions directly, with no long chains of normalized sub-tables.

The reason is query performance and usability. A normalized warehouse requires many joins per query, each join scanning and shuffling data, which is slow and expensive at scale and which analysts struggle to write correctly. A star schema denormalizes the descriptive attributes into wide dimension tables, so a single join to the product dimension brings name, category, brand, and region at once. Resist the instinct to normalize the warehouse the way the source was normalized; the workloads are opposites, and the correct models differ accordingly.

### Choose The Fact Table Grain Deliberately

The grain of a fact table is what one row represents, and getting it wrong is a subtle but devastating error. The grain must be stated explicitly before any design: "one row per order line item," "one row per daily product-store sales aggregate," "one row per click event." Every measure and every dimension foreign key must be valid at that grain. A fact table whose grain is ambiguous or mixed produces double-counts when joined: if some rows are line items and others are order totals, an aggregation over the table counts revenue twice.

Choose the lowest grain that the business needs to analyze, because you can always aggregate up but cannot disaggregate down. A grain that is too coarse (daily totals when analysts need hourly) forces a redesign; a grain that is too fine (every event when only daily aggregates are queried) wastes storage and slows queries. State the grain, ensure every measure is consistent with it, and never mix grains in one fact table — use separate fact tables for different grains.

### Decide How Dimensions Change Over Time (SCD)

Source attributes change: a customer's region is reassigned, a product's category is reclassified, a salesperson moves territories. How the warehouse records that change determines whether history can be reproduced. The slowly changing dimension (SCD) types are the standard answers, and the choice must be deliberate per attribute:

- **SCD Type 1 (overwrite).** The new value replaces the old; no history is kept. Fits attributes where only the current value matters (a corrected typo, a transient flag). Historical reports re-run today will reflect the new value, which may be wrong if the attribute drove past results.
- **SCD Type 2 (new row with validity window).** A new dimension row is created for the changed attribute, with effective-from and effective-to dates and a current flag, so a fact can be joined to the dimension version that was correct at the fact's event time. This preserves full history — last quarter's report by the old region structure is reproducible — at the cost of a larger dimension and the need to join on the time window. This is the right default for attributes that drive analysis and that change meaningfully.
- **SCD Type 3 (add a column).** A second column holds the prior value (current and previous region). Fits attributes where you need only the immediate prior state, not full history.

The trap is choosing Type 1 by default (simplest) and discovering, when an attribute changes, that historical reports can no longer be reproduced. For analytically significant attributes, default to Type 2; reserve Type 1 for attributes whose history is irrelevant.

### Use Surrogate Keys, Not Source Natural Keys, For Dimension Joins

Join facts to dimensions on warehouse-generated surrogate keys, not on the source's natural keys. A surrogate key is a stable, warehouse-owned identifier (often a sequence) assigned to each dimension row, independent of the source's business key. This matters for three reasons: it decouples the warehouse from source key changes (a source that reuses or reassigns a business key would otherwise corrupt historical joins), it supports SCD Type 2 (multiple dimension versions for one business entity each get their own surrogate key, and the fact records which version applied at event time), and it makes joins faster and unambiguous.

Joining on natural keys is a latent corruption risk: when a source reuses an id, or changes its key scheme, historical facts silently join to the wrong dimension version. Assign surrogate keys at load, carry the natural key as an attribute for traceability, and join on the surrogate.

### Prefer Star Over Snowflake Unless Normalization Earns Its Place

A star schema keeps dimensions fully denormalized (wide, flat dimension tables); a snowflake schema normalizes dimensions further (a product dimension snowflakes into separate brand, category, and subcategory tables). Star is the default for analytical warehouses because it minimizes joins — one hop from fact to dimension brings all attributes — which is faster and easier for analysts. Snowflake saves storage by removing redundancy, but at the cost of extra joins on every query, and storage is cheap relative to compute and analyst time.

Choose snowflake only when normalization earns its place: a very large dimension with a heavily redundant attribute that is updated frequently (so denormalizing it creates massive rewrite cost), or a dimension attribute shared across many dimensions where a single source of truth reduces inconsistency. These cases exist but are the exception. Default to star; justify each snowflake explicitly.

### Design Conformed Dimensions For Cross-Process Consistency

A conformed dimension is one shared identically across multiple fact tables — the same date dimension, the same customer dimension, the same product dimension — so that revenue facts and shipment facts and return facts can be sliced by the same attributes and compared on the same axes. Without conformed dimensions, each fact table carries its own version of "customer" or "product," and cross-process analysis (revenue per customer versus returns per customer) produces inconsistent or unmatched results because the dimensions disagree.

Design conformed dimensions when more than one fact table will share an entity, and enforce that they are truly the same (same keys, same attributes, same semantics). This is what makes a warehouse a coherent analytical system rather than a collection of isolated data marts that cannot be combined.

## Common Traps

### Over-Normalizing The Warehouse Like An OLTP Database

Copying the source's normalized schema into the warehouse, producing a model correct for writes but requiring many joins per analytical query, slow at scale, and hard for analysts to use. Model dimensionally for reads; denormalize where it serves the analytical workload.

### The Wrong Or Mixed Fact Grain

A fact table whose grain is unstated or mixed (some rows line items, some order totals), causing double-counts and inconsistent joins. State the grain explicitly, keep one grain per fact table, and choose the lowest grain the business needs.

### SCD Type 1 Where History Matters

Overwriting a dimension attribute that drives analysis, so when it changes, historical reports can no longer be reproduced by the old structure. Default analytically significant attributes to SCD Type 2 with validity windows.

### Joining On Natural Keys

Using the source business key for fact-to-dimension joins, so a source key change or reuse silently corrupts historical joins. Use warehouse-owned surrogate keys; carry the natural key as an attribute.

### Snowflaking Without Justification

Normalizing dimensions into chains of sub-tables to save storage, adding joins to every query for a saving that is usually negligible. Default to star; justify each snowflake by a concrete rewrite-cost or shared-consistency benefit.

### Non-Conformed Dimensions Across Fact Tables

Each fact table carrying its own divergent version of a shared dimension, so cross-process analysis is inconsistent or impossible. Design and enforce conformed dimensions for entities shared across facts.

### Ignoring The Date Dimension

Using raw timestamps and computing time attributes (week, quarter, fiscal period) in every query, instead of a dedicated date dimension that carries all calendar and fiscal attributes. Build a date dimension; it is the most universally conformed dimension.

## Self-Check

- [ ] The warehouse is modeled dimensionally for analytical reads (fact and dimension tables, star schema), not copied from the source's normalized OLTP schema — denormalization is applied where it serves query performance and usability.
- [ ] Each fact table has an explicitly stated grain (what one row represents), every measure is valid at that grain, no grains are mixed in one table, and the grain is the lowest the business needs.
- [ ] Slowly changing dimensions are chosen deliberately per attribute — Type 2 (validity-windowed rows) as the default for analytically significant attributes so history is reproducible, Type 1 only where history is irrelevant, Type 3 where only the prior value matters.
- [ ] Facts join to dimensions on warehouse-owned surrogate keys, not source natural keys, so source key changes or reuse cannot corrupt historical joins, and the natural key is carried as an attribute for traceability.
- [ ] Star schema is the default and each snowflake is justified by a concrete benefit (rewrite cost on a large frequently-updated attribute, or a shared single-source-of-truth dimension), not adopted for storage savings alone.
- [ ] Conformed dimensions are designed and enforced for entities shared across multiple fact tables, so cross-process analysis is consistent on the same keys and attributes.
- [ ] A dedicated date dimension carries calendar and fiscal attributes, rather than recomputing them from raw timestamps in every query.
- [ ] The highest-risk cases were verified — a historical report reproducible after an attribute change (SCD), an aggregation that does not double-count (correct grain), a join that survives a source key change (surrogate keys), and cross-fact analysis on conformed dimensions — not only the current-snapshot happy path.
