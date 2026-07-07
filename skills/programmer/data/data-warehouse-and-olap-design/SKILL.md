---
name: data_warehouse_and_olap_design.md
description: Use when the agent is designing an analytical data warehouse or OLAP layer, choosing star versus snowflake schemas, designing fact and dimension tables, selecting grain and measures, handling slowly changing dimensions, deciding between columnar and row storage or aggregate and materialized-view strategies, modeling for BI tool consumption, or migrating from a normalized OLTP schema to an analytical model. Also covers the failure mode where an analytical model is slow, ambiguous, or misleading because the grain was wrong, a dimension was not conforming, history was handled inconsistently, or the schema was optimized for writes instead of analytical queries.
---

# Data Warehouse And OLAP Design

Analytical data design is governed by different laws than transactional design, and the most common failure is importing OLTP instincts into an analytical context. A normalized schema that serves an operational system well — many narrow tables, optimized for single-row writes, preserving update-in-place semantics — is hostile to analytical queries that aggregate millions of rows across many dimensions. The discipline of dimensional modeling exists precisely because analytical workloads have opposed access patterns: users filter and group by business dimensions (date, product, region, customer segment) and aggregate measures (revenue, units, count of events), and they need a schema whose shape makes those queries fast, unambiguous, and consistent across every BI tool that consumes it.

Agents tend to under-design the analytical layer because the source data already exists and the temptation is to expose it nearly as-is. The harm appears as queries that scan too much and run for minutes, as inconsistent numbers across dashboards (because two reports used different definitions of "revenue"), as dimensions that cannot be compared across fact tables, as history that is silently overwritten when a dimension changes, and as a warehouse that is technically correct and operationally unusable. The judgment is to model deliberately for analytical access: choose the grain before anything else, build conforming dimensions, handle history as a first-class decision, choose storage and aggregation for analytical scan patterns, and treat consistency of business definitions across the warehouse as a hard requirement, not a nicety.

## Core Rules

### Model For Analytical Access Patterns, Not Transactional Ones

Analytical workloads filter and aggregate across large row ranges by business dimension. The schema must make those scans efficient and the semantics unambiguous. Do not expose a normalized source schema directly as the analytical layer.

- **Separate the analytical model from the operational source.** The warehouse is a purpose-built model optimized for queries, not a replica of the OLTP schema. ETL/ELT transforms source data into the dimensional model; the two are allowed (and expected) to differ.
- **Optimize for the query, not the write.** Warehouse loads are bulk and batched; query performance dominates. Denormalization, pre-aggregation, and wide dimension tables are appropriate here precisely because they are wrong in OLTP.
- **Prefer star schemas for general-purpose analytical access.** A fact table surrounded by denormalized dimension tables gives BI tools a predictable join shape, minimizes join depth, and performs well on columnar engines. Reserve snowflaking (normalized dimensions) for cases where a dimension is genuinely large or reused in ways that justify the extra joins.

### Choose The Grain Before Anything Else

The grain is the precise definition of what one row in a fact table represents, and every other decision depends on it. Getting the grain wrong makes measures ambiguous and the fact table difficult or impossible to aggregate correctly.

- **Declare the grain as an atomic business event or level of detail.** "One row per order line shipped" or "one row per daily balance per account" — not "one row per order" if an order contains multiple lines with different products, because that hides the line-level detail.
- **Decide grain before choosing measures or dimensions.** Measures and dimensions are only meaningful relative to a grain; a "quantity" measure means different things at order-line grain versus daily-aggregate grain.
- **Avoid mixed grains in one fact table.** A table that mixes line-level and order-level rows cannot be summed consistently. Use separate fact tables for different grains.

### Build Conforming Dimensions For Cross-Fact Consistency

The single most valuable property of a well-designed warehouse is that the same dimension means the same thing everywhere. A "date" dimension, a "product" dimension, and a "customer" dimension shared across fact tables is what lets a user compare revenue and returns by the same product hierarchy without reconciliation work.

- **Make shared dimensions conforming across fact tables.** The same dimension key and attributes must be identical wherever the dimension appears, so joins across facts produce consistent results.
- **Standardize business definitions in the dimension.** Product category, sales region, and customer segment hierarchies must be defined once and used everywhere; divergent definitions are the root cause of "the two dashboards disagree."
- **Use surrogate keys for dimensions, not natural source keys.** Surrogate keys decouple the warehouse from source-system key changes and are essential for handling dimension history (a natural key that changes breaks historical rows).

### Handle Slowly Changing Dimensions Deliberately

Dimension attributes change over time: a customer moves regions, a product is reclassified, a salesperson changes territory. How you record that change determines whether historical analysis is correct, and it must be a deliberate, documented choice, not an accident of how the ETL happens to behave.

- **Choose a type per attribute, by analytic need.** Type 1 (overwrite) loses history but keeps the dimension small — appropriate for corrections or truly irrelevant history. Type 2 (add a new row with new surrogate key, preserving the old row) preserves full history — necessary when past analysis must reflect the dimension as it was at the time. Type 3 (keep prior and current in separate columns) tracks a single prior state.
- **Default to Type 2 for attributes that drive analysis and change over time.** If a customer's region affects revenue analysis and customers move regions, overwriting loses the ability to attribute past revenue correctly. Type 2 with effective-from/to dates and a current-row flag is the workhorse.
- **Record validity windows and keep them queryable.** Every Type 2 row needs effective dates so a fact at time T can be joined to the dimension state that was correct at T. This is what makes historical analysis accurate.
- **Handle late-arriving facts and dimensions correctly.** A fact that arrives after its dimension has changed must still join to the dimension state that was correct at the fact's event time, not the current state. Effective-date ranges make this possible.

### Choose Measures And Handle Additivity Correctly

Measures are the numeric facts, and their additivity — whether they can be meaningfully summed across dimensions — determines how they can be aggregated. Misunderstanding additivity produces wrong totals.

- **Distinguish additive, semi-additive, and non-additive measures.** Additive measures (quantity, revenue) sum across all dimensions. Semi-additive measures (account balance, inventory level) sum across some dimensions but not time — you cannot sum a balance across days. Non-additive measures (ratios, averages, distinct counts) cannot be summed and must be recomputed from components at each aggregation level.
- **Store measures at the most atomic grain that preserves additivity.** Store the numerator and denominator separately rather than a precomputed ratio, so the ratio can be correctly recomputed at any aggregation level.
- **Document the business definition of every measure.** "Revenue" must mean the same thing in every fact table; two measures with the same name and different definitions will produce reconcilable-looking numbers that never agree.

### Choose Storage And Aggregation For Scan Patterns

Warehouse engines are optimized for scanning and aggregating large column ranges. Storage and physical design choices should serve those scan patterns, and the right choices depend heavily on the engine.

- **Prefer columnar storage for analytical workloads.** Columnar formats read only the columns a query touches, compress repeated dimension values highly, and accelerate the filter-and-aggregate pattern that defines analytical queries. Row storage is appropriate for OLTP, not OLAP.
- **Sort, partition, and cluster by the most-filtered columns.** Date is almost always the primary filter; partition or sort by date so queries scoped to a time range scan only the relevant data. Choose secondary sort/cluster keys by actual query patterns.
- **Use materialized views and aggregates for hot query patterns, with discipline.** Pre-aggregating common rollups (daily sales by product by region) can transform minute-long queries into sub-second ones, but each aggregate is a maintenance and consistency burden. Build them for proven, repeated query patterns, not speculatively.
- **Do not over-index as in OLTP.** Many analytical engines do not use traditional indexes the way row stores do; rely on sort order, partitioning, and zone maps instead, and follow the engine's own guidance rather than OLTP habits.

### Treat Consistency And Governance As A First-Class Requirement

A warehouse is only as valuable as the trust users place in its numbers. Inconsistent definitions, undocumented transformations, and untraceable lineage erode that trust and drive users back to exporting source data into spreadsheets.

- **Maintain a business glossary mapping each measure and dimension to its definition.** Every measure and dimension attribute should have a single authoritative definition that ETL implements and that reports reference.
- **Preserve lineage from source to warehouse.** A user must be able to trace a number in a dashboard back through the transformation logic to the source field. Lineage is what makes a number trustworthy and debuggable.
- **Validate data quality on load.** Row counts, key integrity, measure ranges, and null checks on each load catch silent ETL failures before they corrupt dashboards.

## Common Traps

- **Exposing a normalized source schema as the analytical layer.** Serving the OLTP schema directly, producing slow multi-join queries and confusing semantics for BI users. Build a purpose-built dimensional model.
- **Ambiguous or mixed grain.** A fact table whose rows represent different levels of detail, making measures un-summaggregable and queries ambiguous. Declare one grain per fact table.
- **Non-conforming dimensions across fact tables.** A "product" dimension in one fact and a slightly different "product" dimension in another, so cross-fact comparisons never reconcile. Make shared dimensions conforming.
- **Overwriting dimension history (Type 1 where Type 2 is needed).** Losing the ability to attribute past facts to the dimension state that was correct at the time, because a changed attribute was overwritten. Use Type 2 for analytically-relevant, time-varying attributes.
- **Joining late-arriving facts to current dimension state.** A fact arriving after a dimension change joined to the current (wrong) state instead of the state effective at the fact's event time, distorting historical analysis. Join by effective-date range.
- **Summing semi-additive or non-additive measures across all dimensions.** Summing a daily balance across days, or summing a ratio, producing totals that are wrong. Track additivity per measure and recompute non-additives from components.
- **Precomputing and storing a ratio instead of its components.** Storing a margin ratio rather than revenue and cost, so the ratio cannot be correctly recomputed at aggregated levels. Store components.
- **Using natural source keys as dimension keys.** A source key that changes or is reused breaks historical rows and complicates Type 2 history. Use surrogate keys.
- **Over-indexing using OLTP habits.** Adding many b-tree indexes as in a row store, which analytical engines may ignore or that may slow loads without helping queries. Follow the engine's analytical guidance: sort, partition, cluster.
- **Speculative aggregate tables.** Building many pre-aggregated tables in anticipation of queries that never materialize, adding maintenance and consistency burden for no benefit. Build aggregates for proven, repeated patterns.
- **Undocumented measure definitions.** Two reports named "revenue" using different definitions, so numbers never reconcile and users lose trust. Maintain a business glossary and single authoritative definitions.

## Self-Check

- Is the analytical model purpose-built and separated from the OLTP source schema, optimized for filter-and-aggregate query patterns rather than single-row writes?
- Does every fact table have an explicitly declared, single grain (the atomic business event or detail level), with no mixed grains in one table?
- Are shared dimensions conforming across fact tables — same keys, same attributes, same hierarchies — so cross-fact comparisons reconcile without manual work?
- Is dimension history handled deliberately, with Type 2 (effective-dated rows) for analytically-relevant time-varying attributes, and are late-arriving facts joined to the dimension state effective at their event time rather than the current state?
- Is each measure's additivity understood (additive, semi-additive, non-additive), with components stored separately so ratios and averages can be recomputed correctly at any aggregation level?
- Are dimensions keyed by surrogates rather than natural source keys, decoupling the warehouse from source key changes and enabling history?
- Is storage chosen for analytical scan patterns — columnar format, sort/partition/cluster by the most-filtered columns (date first), and aggregates built only for proven hot query patterns?
- Is there a business glossary with a single authoritative definition per measure and dimension, full source-to-dashboard lineage, and data-quality validation on every load?
- Did I verify the highest-risk cases — a mixed-grain fact table caught, a non-conforming dimension reconciled, a Type 2 history that correctly attributes past facts, a semi-additive measure not summed across time — rather than only the clean single-fact-table path?
