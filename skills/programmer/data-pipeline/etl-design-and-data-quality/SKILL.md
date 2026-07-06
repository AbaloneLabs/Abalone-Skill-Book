---
name: etl_design_and_data_quality.md
description: Use when the agent is designing or reviewing a data pipeline (ETL or ELT) that moves data from source systems into a warehouse, lakehouse, analytics store, or feature store; choosing between ETL and ELT; defining data quality checks (completeness, accuracy, consistency, timeliness, uniqueness, validity); handling schema evolution and backward/forward compatibility; establishing data lineage and traceability; designing idempotent, re-runnable pipelines; planning failure, retry, and partial-failure handling; implementing data validation and data contracts between producers and consumers; dealing with late-arriving, duplicate, or out-of-order records; or setting up pipeline monitoring, alerting, and reconciliation. Also covers the failure modes of silent data corruption, unbounded retries that duplicate downstream effects, schema drift breaking consumers, and the myth that a pipeline that "finishes successfully" produced correct data.
---

# ETL Design And Data Quality

A data pipeline is a chain of trust: each stage asserts that the data it hands downstream is complete, correct, and meaningful, and each downstream consumer builds on that assertion. The judgment problem is not "move the data from A to B" — that is the easy part. It is ensuring that what arrives at B is actually what was at A, transformed correctly, with losses and corruptions detected rather than silently propagated, and that re-running the pipeline produces the same result rather than compounding it. A pipeline that finishes "successfully" can still have dropped half the records, duplicated the other half, applied the wrong transformation, or delivered yesterday's schema to today's consumer. The hardest bugs in data engineering are the ones the pipeline does not report, because the data looks plausible and the job status is green.

Agents tend to under-invest here because the happy path is trivial — extract, transform, load, done — and the job succeeds as long as it runs to completion. The harm appears later and diffusely: a dashboard whose numbers are quietly wrong for weeks because a source field changed type; a machine learning model trained on duplicated rows; a financial report built on a partial load that the pipeline marked complete; a schema change that silently nulls a column for every downstream consumer; a backfill that doubles the data because the pipeline is not idempotent. The judgment problem is deciding, for each pipeline, what "correct" means, how to detect when the data is not correct, how to fail and recover without corrupting downstream state, and how to make the pipeline's guarantees explicit to the consumers who depend on it.

This skill covers ETL/ELT design, data quality, schema evolution, idempotency, failure handling, and observability. It complements the message-queue and stream-processing skills (which cover continuous data movement) and the reliability skill (which covers retries and idempotency at the operation level). Here the question is the architecture of trustworthy batch and micro-batch data movement.

## Core Rules

### Choose ETL Vs ELT By Where The Transformation Belongs

ETL (Extract-Transform-Load) transforms data before loading it into the target; ELT (Extract-Load-Transform) loads raw data first and transforms it inside the target using its compute. The choice changes where logic lives, who owns it, and how reworkable it is:

- **ELT fits when the target is a powerful analytical engine** (a cloud warehouse or lakehouse with abundant compute). Loading raw data preserves fidelity, transformations run close to the data at scale, logic is versioned in SQL/dbt that analysts can own and re-run, and reprocessing a transformation does not require re-extracting from the source. This is the modern default for analytics.
- **ETL fits when the target is weak or the transformation must occur before load** — loading into a key-value store, a search index, a feature store with strict shape requirements, or when source data must be filtered/aggregated to reduce volume before crossing a network or storage boundary. It also fits when the transform requires enrichment the target cannot perform.
- **The deciding factors:** compute location and cost, who owns the transformation (data engineers vs analysts), whether raw data should be preserved for re-derivation, and whether the target can express the needed logic. Forcing ETL onto a warehouse workload throws away the reworkability of ELT; forcing ELT onto a weak target pushes logic it cannot execute.

Prefer ELT for analytics on capable warehouses (load raw, transform with versioned SQL, re-run freely); prefer ETL when the target shape or constraints demand pre-load transformation. In both cases, preserve raw data when feasible so transformations can be re-derived without re-extraction.

### Define Data Quality As Explicit Dimensions, Not A Vague "Looks Right"

"Data quality" is not one property. It is a set of dimensions, each of which must be defined, measured, and alerted on. The standard dimensions:

- **Completeness** — are all expected records and fields present? (row count vs source, null rates on required fields, no missing partitions.)
- **Accuracy** — do the values match reality? (a total that reconciles to the source system, a code that exists in the reference table.)
- **Consistency** — do related values agree across systems and within the dataset? (the sum of line items equals the invoice total; the same entity has one address in two tables.)
- **Timeliness** — is the data current enough for its purpose? (a "today" report built on data that is two days stale is wrong for its purpose even if internally consistent.)
- **Uniqueness** — are entities represented once? (no duplicate rows from a re-run, no double-counted keys.)
- **Validity** — do values conform to expected types, ranges, and formats? (dates in range, enums in the allowed set, numerics within bounds.)

For each pipeline, define concrete checks per dimension with thresholds (not aspirations): expected row-count variance, reconciliation totals, null-rate ceilings, duplicate-key counts, referential integrity checks. A pipeline with no defined quality checks reports success on corrupt data; the checks are what make "success" meaningful.

### Make Pipelines Idempotent And Re-Runnable

Pipelines will be re-run — for backfills, after failures, to apply a fix, to reload historical data. A pipeline that is not idempotent corrupts downstream state on re-run: it appends duplicate rows, double-counts aggregates, or layers stale data over fresh. Idempotency — running the pipeline N times produces the same result as running it once — is a core requirement, not an optimization.

- **Overwrite or upsert by key, never blind append.** Load into a partition or table keyed by the data's natural or temporal key, and write with upsert/merge semantics so re-running the same partition replaces rather than adds. Blind `INSERT` on every run is the most common idempotency bug.
- **Partition by load time or business key.** A partition per day (or per run) lets a re-run replace exactly that partition without touching others. Re-running an unpartitioned whole-table append duplicates everything.
- **Make transformations deterministic.** A transform that depends on "now," on random values, or on mutable external state produces different output on re-run, so backfilled history drifts from the original. Capture the relevant time (event time, batch time) as data and derive deterministically.
- **Stage intermediate data.** Persist raw and intermediate layers so a transformation fix can be re-applied without re-extracting from the source (which may be expensive, rate-limited, or historically unavailable).

The test: can you re-run yesterday's partition, or backfill the last 90 days, and end up with exactly the correct state? If re-running risks duplication or drift, the pipeline is not production-ready.

### Plan For Failure: Partial Failure, Retries, And Recovery

Pipelines fail — source systems are unavailable, network calls time out, a transformation hits bad data, the warehouse is overloaded. The design must define what happens at each failure point, because the default (fail the whole job, lose all progress, retry from the top) is often the worst behavior:

- **Fail fast at extraction, retry at load.** If extraction from the source fails, retry the extraction; do not half-load. If load fails partway, the target must be in a state that a retry can safely resume or replace — which is why partition-based overwrite/upsert matters.
- **Bound retries and isolate poison batches.** A batch that always fails (malformed source data, a transformation bug) will loop forever under unbounded retries. Bound retries, route unrecoverable batches to a quarantine/dead-letter for inspection, and keep the rest of the pipeline moving.
- **Define partial-failure semantics.** If the pipeline loads 10 source tables and table 7 fails, does the whole run fail (and downstream sees nothing), or do tables 1–6 load and 7 is flagged? Decide explicitly: all-or-nothing is safer for tightly coupled datasets; independent loads with per-table status is more resilient for loosely coupled ones. The worst design is ambiguous partial state with no clear contract.
- **Atomic/transactional loads where possible.** Load into a staging table and swap, or use transactional DML, so consumers never see a half-loaded dataset. A pipeline that exposes a half-written table to downstream queries is a source of wrong analytics.

State, for each failure mode, what the target state is after the failure and how recovery resumes. A pipeline with no failure model becomes a manual, error-prone incident every time something breaks.

### Treat Schemas As Contracts And Evolve Them Safely

A pipeline's schema is a contract between the source, the pipeline, and every downstream consumer. Schema changes that are not handled deliberately break consumers silently: a renamed column becomes null downstream, a type change truncates or misparses data, a dropped column removes a field a dashboard depends on.

- **Detect schema drift.** The pipeline should detect when the source schema changes (new column, type change, dropped field) and alert, not silently adapt. Silent adaptation propagates a change consumers were not prepared for.
- **Make additive changes.** Adding an optional column with a default is backward-compatible; renaming, removing, or retyping a column is breaking. Prefer additive evolution and coordinate breaking changes as versioned migrations.
- **Define and enforce data contracts.** A data contract is an explicit, machine-checkable agreement on schema, types, constraints, and semantics between a producer and consumer. Enforce it at the boundary (schema registry, contract tests) so a producer change that violates the contract fails before it reaches consumers.
- **Version and document breaking changes.** When a breaking change is unavoidable, version the dataset, run old and new in parallel during migration, and notify consumers with lead time. A surprise breaking change is a multi-consumer incident.

Treat the schema the way you treat a public API: changes are deliberate, communicated, and compatibility-checked. A schema is not an internal detail once more than one consumer depends on it.

### Handle Late-Arriving, Duplicate, And Out-Of-Order Records Explicitly

Real source data is messy: records arrive late (an event generated yesterday ingested today), duplicated (a source retry sends the same batch twice), and out of order. The pipeline must define handling for each, because the defaults (process by arrival time, append everything) produce wrong results:

- **Late-arriving data.** Decide the latency window: how late can a record be and still update its correct period? Records within the window update (upsert into) the relevant partition; records beyond it go to a late-arrivals store or are dropped per policy. Without a defined window, late data either lands in the wrong period (arrival-time bucketing) or is lost.
- **Duplicates.** Deduplicate by a stable key (source event id, natural key) before or during load, using upsert/merge so a re-sent batch does not double-count. Assume sources will resend; design for it.
- **Out-of-order within a window.** For aggregates, define whether the window is finalized on a schedule (and late updates re-open it) or whether late records update a prior result. State the semantics so consumers know whether a number is final or provisional.

These are not edge cases; they are the normal behavior of real source systems. A pipeline that assumes clean, on-time, unique data works in testing and fails in production.

### Establish Data Lineage And Make The Pipeline Observable

When a downstream number is wrong, the first question is "where did this come from, and which stage could have broken it?" Without lineage, that question is unanswerable and debugging becomes archaeology. Lineage and observability are not optional polish; they are how pipelines stay trustworthy at scale:

- **Lineage traces each dataset to its source and transformations.** For each output field, know: which source tables and fields it derives from, which transformations applied, which pipeline run produced it, and which consumers depend on it. Tooling (dbt, OpenLineage, warehouse-native lineage) helps, but the principle is that the path is recorded, not reconstructed manually each time.
- **Monitor pipeline health, not just success/failure.** Track run duration, row counts, data-quality check results, freshness (time since last successful load), and failure rates over time. A pipeline that "succeeds" but loads 10% of the usual rows is failing; row-count and quality-dimension monitoring catches what job-status does not.
- **Alert on data, not only on jobs.** A job-status alert fires when the job fails; a data-quality alert fires when the data is wrong. Both are needed: a pipeline can succeed and produce wrong data, and a data-quality check can pass while the job is stuck. Alert on freshness (no new data in N hours), on quality-dimension threshold breaches, and on volume anomalies.
- **Make reconciliation routine.** For high-stakes datasets, a periodic reconciliation compares pipeline output against the source of truth (source-system totals, an independent count) and flags divergence. Reconciliation is the safety net beneath the pipeline's own checks, because every layer occasionally misses something.

A pipeline with no lineage and no data-level monitoring is a black box that quietly degrades; the team learns of problems from downstream consumers, weeks late.

## Common Traps

### A "Successful" Job With Silently Corrupt Data

A pipeline that runs to completion and reports success but dropped, duplicated, or mis-transformed records, because success only means "no exception thrown." Define data-quality checks (counts, reconciliation, null rates, uniqueness) so success means the data is correct, not merely present.

### Non-Idempotent Loads That Duplicate On Re-Run

Blind `INSERT` or append on every run, so a backfill or retry doubles the data. Load with partition-based overwrite or upsert/merge by key so re-running a partition replaces rather than adds.

### Silent Schema Drift Breaking Consumers

A source adds, renames, or retypes a column and the pipeline silently adapts or nulls it, so downstream consumers get nulls or wrong types with no warning. Detect schema drift, alert on it, and evolve via additive changes and data contracts.

### Unbounded Retries On A Poison Batch

A batch that always fails (malformed source data, a transformation bug) retried forever, blocking the pipeline and burning resources. Bound retries and route unrecoverable batches to quarantine for inspection while the rest proceeds.

### Ambiguous Partial Failure With No Contract

A pipeline loading multiple datasets fails partway, leaving the target in a half-loaded state that downstream queries read as authoritative. Define partial-failure semantics (all-or-nothing vs independent loads with per-dataset status) and use atomic/staged loads so consumers never see half-written data.

### Arrival-Time Bucketing For Time-Sensitive Data

Grouping records by when they were ingested rather than when they occurred, so late-arriving data lands in the wrong period and aggregates are wrong. Define a late-arrival window and update records into their correct period by event time.

### No Freshness Monitoring, So Stale Data Looks Current and transformations That Depend On "Now"

A pipeline that has not run (or has failed silently) leaves downstream dashboards showing old data with no indication it is stale. Monitor freshness and alert when data exceeds its expected age, separately from job-status alerts.

A transform using the current wall-clock time so that backfilling history produces different output than the original run, causing historical drift. Capture relevant times (event time, batch time) as data and derive deterministically; never let "now" drive a historical re-derivation.

### No Lineage, So Debugging Is Archaeology and assuming The Source Is Clean And Unique

An output number is wrong and no one can trace which source field, transformation, or run produced it, so debugging starts from scratch each time. Record lineage (source → transform → output → consumers) so the path is queryable, not reconstructed.

Designing the pipeline for idealized source data and being surprised by duplicates, nulls, late arrivals, and schema changes in production. Assume sources are messy; deduplicate, validate, handle lateness, and detect drift as core pipeline behavior.

## Self-Check

- [ ] ETL vs ELT was chosen deliberately based on target compute capability, transformation ownership, raw-data preservation, and target constraints — not familiarity — and raw data is preserved where re-derivation may be needed.
- [ ] Data quality is defined as explicit, measurable dimensions (completeness, accuracy, consistency, timeliness, uniqueness, validity) with concrete checks and thresholds per pipeline, so "success" means the data is correct, not merely loaded.
- [ ] The pipeline is idempotent: loads use partition-based overwrite or upsert/merge by key (never blind append), transformations are deterministic, and re-running a partition or backfilling history produces exactly the correct state without duplication or drift.
- [ ] Failure handling is designed: bounded retries, poison-batch quarantine, explicit partial-failure semantics (all-or-nothing vs independent per-dataset loads), and atomic/staged loads so consumers never see half-written data.
- [ ] Schema is treated as a contract: drift is detected and alerted, changes are additive and compatibility-checked, breaking changes are versioned and coordinated, and data contracts are enforced at the boundary.
- [ ] Late-arriving, duplicate, and out-of-order records have explicit handling: a defined late-arrival window with records upserted into their correct period by event time, deduplication by stable key, and stated final-vs-provisional window semantics.
- [ ] Lineage is recorded (source → transform → output → consumers) so the origin of any field or dataset is queryable, not reconstructed during incidents.
- [ ] Monitoring covers data, not only jobs: freshness (time since last successful load), row-count and volume anomalies, and data-quality-dimension thresholds each have alerts, independent of job success/failure.
- [ ] Reconciliation against a source of truth is in place for high-stakes datasets, so divergence between pipeline output and reality is detected rather than discovered by downstream consumers.
- [ ] The highest-risk cases were verified — a re-run/backfill that must not duplicate, a schema change against deployed consumers, a partial failure exposing half-loaded data, stale-but-"successful" data, and late/duplicate source records — not only the clean, on-time, single-run happy path.
