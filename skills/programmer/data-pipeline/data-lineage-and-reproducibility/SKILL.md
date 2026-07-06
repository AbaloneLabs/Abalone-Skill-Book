---
name: data_lineage_and_reproducibility.md
description: Use when the agent is building or reviewing a data pipeline's traceability (lineage from source to output to consumers), making pipelines reproducible and re-runnable, ensuring deterministic transformations for backfill, setting up data quality gates that block bad data from flowing downstream, versioning pipeline code and configuration alongside data, planning a backfill that reproduces historical results, or diagnosing a wrong downstream number whose origin cannot be traced. Also covers the failure mode of a pipeline whose output cannot be explained or regenerated, the cost of non-determinism during backfill, and the discipline of treating data artifacts like versioned, reproducible build outputs.
---

# Data Lineage And Reproducibility

A pipeline output is only trustworthy if you can answer two questions about it: where did this come from, and could you produce it again? Lineage is the answer to the first — the recorded path from source data, through each transformation and pipeline run, to the output field a consumer is looking at. Reproducibility is the answer to the second — whether re-running the pipeline on the same inputs produces the same output, which is what makes backfills, audits, and recovery possible. A pipeline that lacks either is a black box: when a downstream number is wrong, no one can trace which source field, transformation, or run produced it, so debugging is archaeology; when a transformation needs to be re-derived, the re-run produces different results because the logic depended on "now" or on mutable state, so the backfilled history drifts from the original. The judgment problem is treating data outputs the way we treat build artifacts — as versioned, traceable, reproducible products of a known process — rather than as values that emerge inscrutably from a running job.

Agents tend to under-invest in lineage and reproducibility because the happy path does not require them: the pipeline runs, the output looks right, and no one asks where it came from. The harm appears only when something is wrong or when history must be re-derived, and by then the absence is expensive: a dashboard number that no one can explain, a model trained on data whose derivation is lost, a backfill that produces subtly different results and corrupts historical comparisons, a regulatory audit that asks for the provenance of a figure and gets silence. The judgment is to record lineage as the pipeline runs (not reconstruct it during incidents), to make transformations deterministic so re-runs reproduce results, to version pipeline code and configuration alongside the data they produce, and to gate data quality so bad data does not flow downstream and become someone else's wrong number.

## Core Rules

### Record Lineage As The Pipeline Runs, Not During Incidents

Lineage is the recorded path from source to output: which source tables and fields an output derives from, which transformations applied, which pipeline run (version, timestamp, parameters) produced it, and which consumers depend on it. The discipline is to record this automatically as the pipeline runs, because reconstructing it manually during an incident is slow, error-prone, and often impossible (the code has changed, the run is gone, the source state has moved).

- **Capture source → transform → output → consumer links.** For each output field or dataset, know its upstream sources, the transformations applied, the run that produced it, and the downstream consumers. Tooling (dbt, OpenLineage, warehouse-native lineage, orchestrator metadata) helps, but the principle is that the path is recorded, not reconstructed.
- **Record the pipeline version and parameters with each run.** An output produced by pipeline code version X with config Y on data from time T is a specific artifact; without recording X, Y, and T, you cannot explain or reproduce it.
- **Make lineage queryable.** When a downstream number is wrong, the first question is "where did this come from, and which stage could have broken it?" Lineage should answer that in minutes, not days of archaeology.

Without lineage, every wrong number is a mystery investigation. With it, the investigation starts at the recorded path.

### Make Transformations Deterministic So Re-Runs Reproduce Results

Reproducibility requires that running the same transformation on the same input produces the same output, every time. Non-determinism — dependence on wall-clock time, random values, mutable external state, or nondeterministic ordering — breaks this, so a backfill of historical data drifts from the original run and historical comparisons become meaningless.

- **Never let "now" drive a historical derivation.** A transform that uses the current timestamp to bucket data, compute "days since," or filter "active" records produces different output when backfilled, because "now" is different. Capture the relevant time (event time, batch time, the run's reference time) as data and derive deterministically from it.
- **Avoid nondeterministic ordering and randomness in transformations** unless the randomness is seeded and recorded. An aggregation that depends on encounter order, or a sampling step with an unseeded random, produces different output on re-run.
- **Pin external state.** If a transform joins against a reference table or an external API, the reference state at the time of the original run matters. Snapshot reference data with the run, or accept that a re-run against current reference data may differ.
- **Test for determinism.** Re-run a transformation on the same input and confirm the output is identical; drift indicates hidden non-determinism.

Determinism is what makes a backfill trustworthy. A non-deterministic backfill is worse than no backfill, because it produces plausible-but-wrong history.

### Version Pipeline Code And Configuration Alongside The Data

A data output is a product of code, configuration, and input data. To reproduce or explain an output, you must know all three. Version the code and config with the same discipline as the data:

- **Version-control pipeline code and configuration.** The transformation logic, the SQL, the config files, and the parameters are part of the artifact. A pipeline whose code is not version-controlled cannot be reproduced or audited.
- **Record the code version and config with each run.** An output produced by commit A with config B is a specific, reproducible artifact; without recording A and B, the output's derivation is lost when the code moves on.
- **Tie data artifacts to the code that produced them.** When a consumer asks "why did this number change," the answer often lies in a code or config change between runs; the linkage lets you find it.

### Make Pipelines Re-Runnable And Idempotent For Backfill

Backfill — re-running the pipeline over historical data, after a code change, a fix, or to populate a new dataset — is inevitable, and it requires that the pipeline be re-runnable and idempotent. Re-runnable means it can target a historical time range; idempotent means running it N times produces the same result as once.

- **Partition by the data's temporal or natural key** so a backfill can target and replace specific partitions without touching others.
- **Overwrite or upsert by key, never blind append**, so re-running a partition replaces rather than duplicates.
- **Ensure the pipeline can accept a historical time range as input**, not just "process new data since last run." A pipeline that only runs forward cannot backfill.

The test: can you re-run the last 90 days and end up with exactly the correct state, identical to (or deliberately corrected from) the original? If not, the pipeline is not backfill-ready.

### Gate Data Quality So Bad Data Does Not Flow Downstream

A quality gate is a check that blocks bad data from propagating to downstream consumers. Without gates, a corrupt or incomplete batch flows through every stage and becomes every consumer's wrong number, and the corruption is discovered weeks late by a consumer rather than caught at ingestion. Quality gates make "the pipeline succeeded" mean "the data is correct," not merely "no exception was thrown."

- **Define quality checks per dimension** (completeness, accuracy, consistency, timeliness, uniqueness, validity) with concrete thresholds: expected row-count variance, null-rate ceilings, reconciliation totals, duplicate-key counts, referential integrity.
- **Gate at stage boundaries.** Before data moves from raw to staged, or staged to curated, or curated to consumer-facing, run the quality checks and block propagation on failure. A failure routes to quarantine for inspection rather than flowing onward.
- **Make success meaningful.** A pipeline run that passes its quality gates has earned the claim that its output is correct; one with no gates has only proven it did not crash.

### Make Reconciliation Routine For High-Stakes Data

Reconciliation is the periodic comparison of pipeline output against an independent source of truth (source-system totals, an independent count, an external figure). It is the safety net beneath the pipeline's own checks, because every layer occasionally misses something, and drift between the pipeline and reality accumulates silently.

- **Reconcile financial, regulatory, and decision-critical data regularly.** Compare pipeline totals to source-system totals; flag divergence beyond a threshold.
- **Investigate divergence, not explain it away.** A reconciliation mismatch is a signal that something in the pipeline is wrong; resolving it means finding the bug, not adjusting the threshold until it passes.

## Common Traps

### No Lineage, So Debugging Is Archaeology

An output number is wrong and no one can trace which source field, transformation, or run produced it, so debugging starts from scratch each time. Record lineage automatically as the pipeline runs so the path is queryable, not reconstructed during incidents.

### Transformations That Depend On "Now"

A transform using the current wall-clock time so that backfilling history produces different output than the original run, causing historical drift. Capture relevant times as data and derive deterministically; never let "now" drive a historical re-derivation.

### Non-Deterministic Ordering Or Unseeded Randomness

An aggregation that depends on encounter order, or a sampling step with an unseeded random, producing different output on re-run. Make transformations deterministic; seed and record any randomness.

### Unversioned Pipeline Code Or Config

Pipeline code or configuration that is not version-controlled or not recorded with each run, so an output's derivation is lost when the code moves on and the output cannot be reproduced or audited. Version-control code and config and record the version with each run.

### Non-Idempotent Pipeline That Duplicates On Backfill

A pipeline that blind-appends or cannot target a historical range, so a backfill duplicates data or cannot run at all. Partition by temporal key, overwrite/upsert by key, and ensure the pipeline accepts a historical time range.

### No Quality Gates, So Success Means Nothing

A pipeline that reports success when it merely did not crash, allowing corrupt or incomplete data to flow downstream and become consumers' wrong numbers. Gate data quality at stage boundaries with concrete checks and thresholds.

### Stale Data Looking Current

A pipeline that has not run (or failed silently) leaves downstream dashboards showing old data with no indication it is stale. Monitor freshness and alert when data exceeds its expected age.

### No Reconciliation, So Drift Accumulates and reference Data Not Snapshotted

High-stakes data that is never compared to an independent source of truth, so silent drift between the pipeline and reality accumulates until a consumer notices a wrong number. Reconcile financial and decision-critical data regularly and investigate divergence.

A transform that joins against a reference table or external API that changes over time, so a backfill against current reference data differs from the original run. Snapshot reference data with the run or accept and document the difference.

## Self-Check

- [ ] Lineage is recorded automatically as the pipeline runs — source tables/fields, transformations, the producing run (version, timestamp, parameters), and downstream consumers — and is queryable so the origin of any field or dataset is answerable in minutes, not reconstructed during incidents.
- [ ] Transformations are deterministic: no dependence on wall-clock "now" (relevant times captured as data), no unseeded randomness, no nondeterministic ordering, and reference data is snapshotted or its variability documented — verified by re-running on the same input and confirming identical output.
- [ ] Pipeline code and configuration are version-controlled, and the code version and config are recorded with each run, so every output is tied to the specific code and config that produced it and can be reproduced or audited.
- [ ] The pipeline is re-runnable and idempotent for backfill: partitioned by temporal/natural key, overwrite/upsert by key (never blind append), accepts a historical time range, and re-running a partition or backfilling 90 days produces exactly the correct state.
- [ ] Data quality is gated at stage boundaries with concrete checks per dimension (completeness, accuracy, consistency, timeliness, uniqueness, validity) and thresholds, so a run that passes its gates has earned the claim its output is correct, and failures route to quarantine rather than flowing downstream.
- [ ] Freshness is monitored and alerted, so stale data (a pipeline that has not run or failed silently) is detected rather than presented as current to downstream consumers.
- [ ] High-stakes (financial, regulatory, decision-critical) data is reconciled periodically against an independent source of truth, and divergence is investigated (not threshold-adjusted away).
- [ ] The highest-risk cases were verified — tracing a wrong downstream number via lineage, backfilling history without drift, reproducing an output from its recorded code version and config, a quality gate catching corrupt data before it flows downstream, and a reconciliation catching silent drift — not only the clean single-run happy path.
