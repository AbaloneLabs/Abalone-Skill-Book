---
name: lakehouse_and_data_lake_architecture.md
description: Use when the agent is designing or reviewing a data lake or lakehouse architecture, choosing open file formats (Parquet, ORC, Avro) and table formats (Iceberg, Delta Lake, Hudi) on object storage, planning a partitioning strategy, designing a medallion architecture (bronze, silver, gold layers), deciding when a lakehouse versus a warehouse versus a raw data lake is the right platform, or reviewing a data lake for governance, ACID guarantees, and the risk of becoming a data swamp. Also covers the failure modes of dumping unstructured files into a lake with no schema or catalog, choosing a partitioning key that causes small-file or skew problems, expecting ACID from a raw object-store lake, and picking a lakehouse when a warehouse's query performance is actually required.
---

# Lakehouse And Data Lake Architecture

A data lake is a storage architecture that holds large volumes of data in open file formats on cheap, durable object storage (S3, GCS, Azure Blob), decoupling storage from compute. A lakehouse extends the lake with a table format that gives it warehouse-like properties — ACID transactions, schema enforcement, time travel, and efficient upserts — directly on the object store, so analytics and machine learning can run over the same governed data without copying it into a separate warehouse. The judgment problem is that "dump everything into a lake" is trivially easy and almost always wrong: without a table format, a catalog, a schema discipline, and a layering strategy, a lake becomes a data swamp — an unstructured pile of files no one can find, trust, query, or govern. The architecture decisions that determine whether a lake is usable — format, table format, partitioning, layering, governance — are the ones agents skip because the happy path (write a Parquet file, read it back) needs none of them.

Agents tend to under-invest here because object storage is cheap and writing files is easy, so the lake fills up before anyone designs how it will be queried. The harm appears later and diffusely: thousands of tiny files that make every query slow; a partitioning scheme that shards data so queries scan everything anyway; no schema enforcement so a column means one thing in January and another in February; no ACID so a concurrent writer and reader see a half-written table; no catalog so no one knows which dataset is authoritative; no governance so the lake holds sensitive data no one tracked. The discipline is to choose the format and table format deliberately, partition for the actual query patterns, layer the data (medallion) so raw chaos is refined into trustworthy datasets, and put governance and a catalog in place from the start. A lake without these is not an architecture; it is a liability.

This skill covers data lake and lakehouse architecture: file and table formats, ACID on object storage, partitioning strategy, the medallion architecture, platform selection (lakehouse versus warehouse versus lake), and governance. It complements the warehouse-modeling skill (the logical dimensional model) and the ETL and pipeline skills (moving and quality-checking data). Here the focus is the storage and table architecture of the lake.

## Core Rules

### Choose The Table Format To Get Warehouse Properties On The Lake

A raw data lake of Parquet files on object storage gives you cheap storage and columnar reads, but no transactional guarantees: concurrent writers race, readers see partial writes, there is no schema enforcement, upserts require rewriting files, and there is no time travel. A table format — Iceberg, Delta Lake, or Hudi — layers a transactional metadata layer over the files to provide ACID transactions, schema enforcement and evolution, time travel (querying older snapshots), and efficient merge/upsert. This is what turns a file dump into a lakehouse.

The decision is whether your workload needs these properties. If multiple writers or readers contend, if you need atomic upserts or deletes, if schema must be enforced, or if you need reproducible historical queries (time travel), use a table format. If the workload is append-only, single-writer, batch-read, and schema is enforced upstream, raw Parquet may suffice and avoids the metadata overhead. The common error is assuming a raw lake has ACID — it does not, and a concurrent writer and reader will see inconsistent or partial data. Choose the table format deliberately; do not inherit its absence by default.

### Pick A Partitioning Strategy For The Actual Query Patterns

Partitioning divides the data into physical directories by a column value (commonly a date), so a query filtering on that column reads only the relevant partitions rather than scanning the whole table. Good partitioning is the single biggest performance lever on a lake, and bad partitioning is the single biggest performance problem. The strategy must be chosen against the actual query patterns, not against a default:

- **Partition by the most-filtered column.** If nearly every query filters by date, partition by date (year/month/day). If queries filter by region or tenant, consider those. The partition key should be one that queries prune on, so most data is skipped.
- **Avoid over-partitioning (the small-file problem).** Partitioning too finely (by hour, or by a high-cardinality key like user id) creates many tiny files. Object stores and query engines are slow against millions of small files — the metadata and open/close cost dominates. Aim for files in the 100MB–1GB range; partition at a granularity that produces large files.
- **Avoid under-partitioning and skew.** Partitioning by a low-cardinality or skewed key (one region holds 90% of data) means queries still scan huge partitions and compute is unbalanced. Choose a key with enough cardinality to split the data but not so much that files shrink.

State the primary query filter, partition to match it, and verify file sizes and counts are healthy. A partitioning scheme chosen without reference to query patterns is a guess, and the wrong guess cripples performance.

### Use The Medallion Architecture To Layer Trust

The medallion architecture (bronze, silver, gold) layers data from raw ingestion to refined, trustworthy datasets, so that chaos is contained at the bottom and trust accumulates as data moves up:

- **Bronze (raw).** Data landed as-is from the source, preserving fidelity, with minimal transformation — append-only, schema-on-read tolerated, the historical record of what the source sent. This is the backfill source: if a transformation changes, you re-derive from bronze without re-extracting.
- **Silver (cleansed, conformed).** Bronze data cleaned, deduplicated, typed, conformed to a schema, and joined to reference data. Here data quality is enforced and the data becomes trustworthy for exploration.
- **Gold (curated, business-ready).** Aggregated, dimensional, business-level datasets — the star schemas, the metrics tables, the consumption layer that dashboards and models read directly.

The value of layering is that each stage has a clear contract and quality bar, raw chaos is quarantined in bronze, and downstream consumers read gold rather than each rebuilding from raw. Without layering, every consumer re-derives from an unstructured lake, duplicating logic and producing inconsistent numbers. Define what each layer guarantees and enforce the quality bar between them.

### Decide Lakehouse Versus Warehouse Versus Lake By The Workload

These are different platforms for different workloads, and choosing by fashion ("lakehouse is modern") rather than by workload produces a costly mismatch:

- **Data warehouse** (Snowflake, BigQuery, Redshift) when you need the best query performance and concurrency for BI and SQL analytics, strong governance, and a managed experience, and the data fits a structured, modeled layer. Warehouses excel at fast SQL over modeled data; they cost more per byte stored and couple storage to their compute.
- **Data lake** (object storage with open files) when you need cheap storage for vast volumes of raw, semi-structured, or unstructured data (logs, images, ML training sets), accessed by diverse engines, and query performance is not the top priority. Lakes are cheap and flexible but lack ACID, governance, and fast SQL unless augmented.
- **Lakehouse** (lake + table format + catalog) when you want lake economics and openness (open formats, decoupled compute, multiple engines) plus warehouse properties (ACID, schema enforcement, time travel, performant SQL) over the same data, especially for combined analytics and machine learning workloads. The lakehouse is the modern convergence point but carries the operational burden of the table format, catalog, and optimization (compaction).

Match the platform to the workload's priorities: performance and managed SQL (warehouse), cheap flexible storage (lake), or open economics plus transactional analytics (lakehouse). Do not pick a lakehouse and then discover BI query latency is unacceptable, nor pick a warehouse and then chafe at being locked to one engine and storage.

### Put Governance And A Catalog In Place From The Start

A lake without governance and a catalog is a data swamp: files accumulate with no record of what they are, who owns them, what they mean, or what sensitivity they carry. The catalog (a metastore — Glue, Unity, Hive Metastore, a table-format native catalog) is what makes datasets discoverable and queryable as tables rather than as anonymous files. Governance covers data classification, access control, lineage, and retention — knowing which datasets contain PII, who can access them, where they came from, and when they must be deleted.

Establish the catalog and governance when the lake is small, because retrofitting them onto a sprawling, undocumented lake is far harder and riskier. Without them, sensitive data leaks undetected, datasets are unknowable, and the lake becomes a compliance and trust liability that no one can vouch for.

## Common Traps

### The Data Swamp: Files Without Schema, Catalog, Or Governance

Dumping raw files into object storage with no table format, no catalog entry, no schema, and no ownership, so the data is unfindable, untrusted, and ungoverned. Use a table format, register datasets in a catalog, enforce schemas, and record ownership and sensitivity from the start.

### Expecting ACID From A Raw Object-Store Lake

Assuming a lake of Parquet files gives transactional guarantees, so concurrent writers and readers race and readers see partial writes. Use a table format (Iceberg, Delta, Hudi) when you need ACID, upserts, or time travel.

### Over-Partitioning Into Millions Of Tiny Files

Partitioning by a high-cardinality or fine-grained key, producing many small files that make every query slow due to metadata and open/close overhead. Partition to match query filters while keeping files in the 100MB–1GB range; compact small files.

### Partitioning That Does Not Match Query Patterns

Choosing a partition key by default (or by source convenience) rather than by how queries filter, so queries still scan most of the table. Partition by the most-filtered column and verify partition pruning works.

### No Layering, So Every Consumer Rebuilds From Raw

Skipping the medallion layers so each consumer re-derives clean data from an unstructured lake, duplicating logic and producing inconsistent numbers. Define bronze, silver, and gold layers with clear contracts and quality bars.

### Picking A Lakehouse When Warehouse Performance Is Required

Choosing a lakehouse for fashion and then finding BI query latency or concurrency unacceptable for the analytics workload. Match the platform to the workload's performance and managed-SQL needs.

### No Schema Enforcement Across Writes

Allowing writers to evolve or break the schema freely, so a column's meaning drifts over time and historical queries become ambiguous. Enforce schema at the table-format boundary and evolve it deliberately.

## Self-Check

- [ ] A table format (Iceberg, Delta Lake, or Hudi) is used where the workload needs ACID, upserts, schema enforcement, or time travel; raw Parquet is used only for append-only, single-writer, schema-enforced-upstream workloads, and ACID is never assumed from a raw object-store lake.
- [ ] Partitioning is chosen against the actual query filter patterns (typically date), balances pruning against file size (files in the 100MB–1GB range, not millions of tiny files), and avoids skew; partition pruning was verified, not assumed.
- [ ] The medallion architecture (bronze raw, silver cleansed/conformed, gold curated/business-ready) is in place with a clear contract and quality bar at each layer boundary, so raw chaos is quarantined and consumers read gold rather than rebuilding from raw.
- [ ] The platform choice (warehouse, lake, or lakehouse) matches the workload's priorities — performance and managed SQL, cheap flexible storage, or open economics plus transactional analytics — rather than fashion, and the tradeoffs (latency, lock-in, operational burden) are accepted.
- [ ] A catalog/metastore registers datasets as discoverable tables (not anonymous files), and governance (classification, access control, lineage, retention) is established while the lake is small, with PII and sensitive data tracked.
- [ ] Schema is enforced at the table-format boundary, evolution is deliberate and compatibility-aware, and a column's meaning does not drift silently across writes.
- [ ] File health is monitored (file count, file size, compaction needs) so small-file accumulation and metadata bloat are corrected before they cripple query performance.
- [ ] The highest-risk cases were verified — a concurrent writer and reader not seeing partial data (ACID), partition pruning actually skipping the expected data, a backfill re-derived from bronze, sensitive data identified and access-controlled, and BI query latency acceptable for the analytics workload — not only the single-file read/write happy path.
