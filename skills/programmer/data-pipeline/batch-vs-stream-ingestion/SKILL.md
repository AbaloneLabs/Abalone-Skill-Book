---
name: batch_vs_stream_ingestion.md
description: Use when the agent is choosing how to ingest data into a pipeline or warehouse (batch, micro-batch, change-data-capture, or continuous streaming), deciding between scheduled bulk loads and event-driven ingestion, reasoning about latency versus throughput tradeoffs, setting up watermarks and checkpointing for ingestion, planning backfill of historical data, handling schema evolution during ingestion, choosing an ingestion framework (Kafka Connect, Flink CDC, Airbyte, Debezium, scheduled extract jobs), or diagnosing ingestion lag, data loss, or duplicate loads. Also covers the failure mode of choosing streaming for a workload that only needs daily data, choosing batch for a workload whose value decays in minutes, and treating ingestion as a simple copy rather than a contract with latency, ordering, and completeness guarantees.
---

# Batch Vs Stream Ingestion

Ingestion is the act of moving data from where it is produced (a transactional database, an application event stream, a third-party API, a log) into where it will be processed or stored (a warehouse, lakehouse, stream, or feature store). The first and most consequential decision is the ingestion pattern: batch (scheduled, bulk extracts of data that already exists), micro-batch (frequent small batches narrowing the gap toward real-time), change-data-capture (streaming row-level changes from a database log), or continuous streaming (event-by-event as the source emits). Each pattern carries a different latency, throughput, cost, and operational profile, and the wrong choice either wastes enormous effort over-engineering a workload that needed daily data, or fails a workload whose value decayed before the batch ran. The judgment problem is not "how do we move the data" — it is "what latency, completeness, and ordering does the downstream consumer actually need, and which ingestion pattern delivers that at acceptable cost."

Agents tend to under-invest in this decision because the default (scheduled batch extracts) is simple and works on the happy path, and because "real-time" sounds more modern than "batch." Both directions fail. Choosing batch for a fraud-detection or monitoring workload means the data is stale by the time it is useful; choosing streaming for a daily report means paying for checkpointing, state management, watermark handling, and operational complexity that the business never needed. The judgment is to name the latency requirement concretely, match the ingestion pattern to it, and understand that each pattern is a different contract with its own failure modes — batch fails by being stale and by re-run duplication; streaming fails by lag, out-of-order data, and exactly-once complexity.

## Core Rules

### Name The Latency Requirement Concretely, Then Match The Pattern

"Real-time" is not a requirement; it is a vibe. The first step is to state, in concrete units, how fresh the data must be for the downstream consumer to act on it:

- **Daily or hourly freshness → batch.** If the consumer is a daily report, a weekly aggregate, or a dashboard refreshed on a schedule, batch ingestion is correct and streaming is over-engineering. The value of the data does not decay within the day, so paying for continuous ingestion buys nothing.
- **Minutes freshness → micro-batch.** Frequent small batches (every few minutes) narrow the gap toward real-time without the full complexity of streaming. Suitable for near-real-time dashboards and operational metrics where seconds-level freshness is not required.
- **Seconds freshness → streaming / CDC.** If the consumer must react within seconds of the source event (fraud detection, real-time alerting, live personalization), continuous ingestion is justified. The value of the data decays fast, so the latency is worth the cost.
- **Sub-second or event-driven → streaming.** When the downstream logic is itself event-driven (trigger an action the moment an event occurs), streaming ingestion is the only pattern that fits.

Name the requirement first. A workload that is "real-time" in aspiration but consumed daily in practice should be batch; a workload whose value vanishes in five minutes must be streaming. The pattern follows the requirement, not the fashion.

### Understand What Each Pattern Actually Guarantees And Costs

Each ingestion pattern is a different contract. Choosing without understanding the contract leads to surprise failures:

- **Batch (scheduled bulk extract).** High throughput, simple, cheap, operationally low-complexity. Guarantees: a complete snapshot at extract time, processed on a schedule. Costs/failures: data is as stale as the batch interval; re-runs duplicate unless idempotent; a failed batch must be re-run cleanly; large extracts strain the source during the extract window.
- **Micro-batch (frequent small extracts).** Narrows batch latency to minutes. Same contract as batch with a shorter interval; the cost is more frequent source load and more pipeline runs to manage. A good middle ground when minutes suffice but seconds do not.
- **Change-data-capture (CDC, streaming row changes from a DB log).** Near-real-time row-level changes without source queries. Guarantees: each change event in order, low latency. Costs/failures: the log is the source of truth (schema changes affect it), exactly-once requires care (a redelivered change must not double-apply), ordering must be preserved per row, and the CDC connector must handle source schema changes and log retention.
- **Continuous streaming (event-by-event from a producer).** Lowest latency, highest complexity. Guarantees: events as they are produced (with the ordering and delivery semantics of the broker). Costs/failures: backpressure, out-of-order events, late data, checkpointing, state management, and the full stream-processing operational burden.

Match the pattern to the guarantee the consumer needs. Do not choose streaming for its low latency and then ignore its ordering and exactly-once requirements; do not choose batch for its simplicity and then discover the data is too stale to be useful.

### Make Ingestion Idempotent And Re-Runnable

Ingestion will be re-run — after a failure, to backfill, to recover from corruption. A non-idempotent ingestion corrupts downstream state on re-run: it appends duplicate rows, double-counts, or layers stale data over fresh. Idempotency — running the ingestion N times produces the same result as running it once — is a core requirement regardless of pattern.

- **Batch: overwrite or upsert by key into a partition, never blind append.** A partition per batch interval lets a re-run replace exactly that partition. Blind `INSERT` on every run is the most common idempotency bug.
- **CDC and streaming: upsert by primary key, with deduplication by a stable event identifier.** A redelivered change event must update rather than insert a duplicate. The sink must support merge/upsert semantics keyed on the entity.
- **Make transformations deterministic.** A transform that depends on "now" or on mutable external state produces different output on re-run, so backfilled history drifts. Capture the relevant time as data and derive deterministically.

The test: can you re-run yesterday's ingestion, or backfill the last 90 days, and end up with exactly the correct state? If re-running risks duplication or drift, the ingestion is not production-ready.

### Plan The Backfill Before You Need It

Backfill — re-ingesting historical data, usually after a pipeline change, a new transformation, or to populate a new dataset — is inevitable. A pipeline designed only for forward-going data breaks the moment a backfill is needed: the transformation depends on "now," the partitioning cannot accommodate old dates, the source no longer holds the historical data, or the re-run doubles everything.

- **Preserve raw ingested data** in a staging/raw layer so transformations can be re-derived without re-extracting from the source (which may be rate-limited, expensive, or historically unavailable).
- **Make partitioning and keying backfill-friendly.** Partition by the data's natural or temporal key so a backfill can target and replace specific partitions.
- **Ensure determinism.** A backfill of historical data must produce the same output the original run did; any dependence on wall-clock time or mutable state breaks this.
- **Plan the source's historical availability.** Can the source provide 90 days of past data? At what cost and rate? A backfill that discovers the source purges data after 30 days is a forced scope reduction.

Design for the backfill from the start. It is not an edge case; it is a certainty.

### Handle Schema Evolution At The Ingestion Boundary

The source schema will change — columns added, renamed, retyped, dropped — and the ingestion boundary is where that change meets the pipeline. Without deliberate handling, a source change silently corrupts downstream data: a renamed column becomes null, a type change truncates or misparses, a dropped column removes a field consumers depend on. (Schema evolution strategy in depth is covered by the dedicated skill; here the concern is the ingestion-specific dimension.)

- **Detect schema drift at ingestion.** The ingestion layer should detect when the source schema changes and alert, not silently adapt. Silent adaptation propagates a change consumers were not prepared for.
- **Prefer additive evolution.** Adding an optional column with a default is backward-compatible; renaming, removing, or retyping is breaking. Coordinate breaking changes as versioned migrations with consumer notification.
- **For CDC, the log schema is the contract.** A source DDL change affects the CDC stream immediately; the connector and downstream must handle it (often via schema registry integration) rather than breaking.
- **Enforce data contracts at the boundary** so a producer change that violates the contract fails before it reaches consumers.

### Choose The Ingestion Tool Against The Pattern And The Source

The tool follows the pattern and the source characteristics, not the reverse:

- **Scheduled extract jobs** (Airflow, Dagster, Prefect orchestrating extract queries or API calls) fit batch and micro-batch from queryable sources or APIs.
- **CDC connectors** (Debezium, Flink CDC, cloud-native CDC) fit streaming row-level changes from transactional databases, reading the transaction log without querying the source.
- **Kafka Connect / managed connectors** fit streaming from sources that emit to or can be polled into a log.
- **Event-producer SDKs** fit when the application emits events directly to a stream.

Consider source load: batch extracts query the source during the extract window (can strain it); CDC reads the log (lower source impact but requires log access and retention). Consider rate limits and cost for third-party API sources. The tool must fit both the pattern and what the source can support.

## Common Traps

### Choosing Streaming Because It Sounds Modern

Building a complex streaming ingestion pipeline for a workload consumed daily or hourly, paying for checkpointing, state, and operational burden for latency the business does not need. Match the pattern to the concrete latency requirement; batch or micro-batch is often simpler and more robust.

### Choosing Batch For A Workload Whose Value Decays In Minutes

Using a daily batch for fraud detection, real-time alerting, or live personalization, where the data is stale by the time it is useful. If the value decays within the freshness window, the pattern is wrong.

### Non-Idempotent Ingestion That Duplicates On Re-Run

Blind `INSERT` or append on every run, so a backfill or retry doubles the data. Load with partition-based overwrite or upsert/merge by key so re-running replaces rather than adds.

### No Backfill Plan, Discovered Too Late

A pipeline designed only for forward-going data that breaks when a backfill is needed: the transformation depends on "now," the source no longer holds the history, or the re-run duplicates. Preserve raw data, partition by temporal key, ensure determinism, and verify source historical availability before you need the backfill.

### Silent Schema Drift At The Ingestion Boundary

A source adds, renames, or retypes a column and the ingestion silently adapts or nulls it, so downstream consumers get nulls or wrong types with no warning. Detect schema drift, alert on it, and evolve via additive changes and enforced data contracts.

### CDC Treated As "Just Streaming"

Choosing CDC for its low latency and ignoring that the log is the source of truth, that ordering must be preserved per row, and that exactly-once requires idempotent sinks. CDC is a different contract than batch; handle its ordering, schema, and redelivery requirements deliberately.

### Transforms That Depend On "Now"

An ingestion transform using the current wall-clock time so that backfilling history produces different output than the original run, causing historical drift. Capture relevant times (event time, batch time) as data and derive deterministically.

### Source Strain During Batch Extracts

A batch extract that runs heavy queries against a transactional source during business hours, degrading the source system. Schedule extracts off-peak, use CDC to avoid source queries, or replicate the source for extraction.

### Ignoring Late-Arriving Data In Batch

Batch ingestion that buckets data by extract time rather than event time, so late-arriving records land in the wrong period. Define the late-arrival window and upsert records into their correct period by event time.

## Self-Check

- [ ] The latency requirement is stated concretely (e.g., "data fresh within N minutes/hours"), and the ingestion pattern (batch, micro-batch, CDC, streaming) was chosen to match it — not defaulted to streaming for modernity or to batch for simplicity.
- [ ] The chosen pattern's contract (latency, throughput, ordering, completeness, cost) is understood and accepted, including its specific failure modes (batch staleness and re-run duplication, streaming lag and exactly-once complexity, CDC log-schema and ordering requirements).
- [ ] Ingestion is idempotent: batch loads use partition-based overwrite or upsert/merge by key (never blind append), CDC/streaming sinks deduplicate by stable event id and upsert by primary key, and re-running a partition or backfilling produces exactly the correct state.
- [ ] A backfill plan exists: raw data is preserved for re-derivation, partitioning accommodates historical dates, transformations are deterministic (no "now" dependence), and source historical availability and cost were verified before the backfill was needed.
- [ ] Schema evolution is handled at the ingestion boundary: drift is detected and alerted (not silently adapted), changes are additive and compatibility-checked, CDC log-schema changes are handled, and data contracts are enforced.
- [ ] The ingestion tool was chosen against the pattern and the source characteristics (source load, rate limits, log access for CDC, historical availability), not by familiarity alone.
- [ ] Late-arriving data has a defined window and is upserted into its correct period by event time, not bucketed by extract/arrival time.
- [ ] Source strain was considered: batch extracts are scheduled off-peak or against a replica, CDC reads the log rather than querying, and third-party API rate limits and cost are accounted for.
- [ ] The highest-risk cases were verified — a re-run/backfill that must not duplicate, a schema change against deployed consumers, late-arriving data landing in the correct period, and source unavailability during extract — not only the clean, on-time, single-run happy path.
