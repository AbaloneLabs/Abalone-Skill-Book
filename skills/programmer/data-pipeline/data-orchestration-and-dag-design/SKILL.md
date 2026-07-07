---
name: data_orchestration_and_dag_design.md
description: Use when the agent is designing the structure of a data pipeline DAG (Airflow, Dagster, Prefect, Argo), deciding task granularity and how to split a pipeline into tasks, modeling fan-out/fan-in and dynamic task mapping, making individual tasks idempotent and safely retriable, planning a backfill as a structural concern over partitions, choosing between scheduled runs and event-triggered runs, or deciding how to separate orchestration control logic from transformation logic. Also covers the failure modes of monolithic pipelines that fail atomically and lose all progress, tasks that are not safely re-runnable so retries duplicate data, fan-out that overwhelms the scheduler, and transformation logic embedded in the orchestrator that cannot be tested or reused independently.
---

# Data Orchestration And DAG Design

A data pipeline DAG is a computational graph: it expresses which tasks run, in what order, what can run in parallel, what must wait, and how a re-run or backfill maps onto historical partitions. The mechanics look trivial — define tasks, wire dependencies, schedule daily — and a two-task DAG works fine. The judgment problem is structural, and it appears as the pipeline grows: a monolithic task that does extraction, transformation, and load in one block fails atomically and loses all progress on any error; a task that is not idempotent turns every retry into a duplication event; a fan-out that spawns thousands of tasks destabilizes the scheduler; transformation logic embedded in operator callbacks cannot be unit-tested, versioned, or reused outside the orchestrator. The recurring harm is that the DAG is treated as a script with a schedule rather than as a dependency-aware, re-runnable, partitioned computation whose structure determines how it fails and how it recovers.

Agents tend to under-invest in DAG structure because the happy path needs none of it: the pipeline runs once, succeeds, and produces correct output. The cost appears only under failure, re-run, and backfill — exactly when the structure matters most. A retry that duplicates data, a backfill that cannot target historical partitions, a partial failure that forces a full restart from extraction, a giant task whose failure is opaque — each is a consequence of structure decided too casually. The discipline is to choose task boundaries deliberately, make each task independently re-runnable, separate the control plane (the orchestrator) from the data plane (the transformation logic), and design the DAG so that failure is localized, recovery is cheap, and history is re-derivable.

This skill covers DAG structure and task granularity, dependency modeling, fan-out and fan-in, idempotent task design, backfill as a structural concern, scheduling versus triggering, and the separation of orchestration from transformation. It complements the pipeline-orchestration-and-scheduling skill, which covers scheduling, SLAs, freshness contracts, concurrency limits, and observability of the control plane. Here the focus is the shape of the computation graph itself.

## Core Rules

### Choose Task Boundaries By What Can Fail And Be Re-Run Independently

The fundamental question of DAG design is where to draw task boundaries, and the answer is not "one task per pipeline" nor "one task per line." A task boundary is a recovery point: the unit the orchestrator can retry without redoing earlier work, and the unit whose failure can be isolated from other work. Draw boundaries so that each task is cohesive (does one logical thing), independently re-runnable (re-executing it is safe and produces the correct state), and a sensible unit of failure (if it fails, retrying just it is cheaper than restarting the whole pipeline).

A monolithic task — extract, transform, and load in a single block — is the worst structure: it fails atomically, so any error late in the load discards the expensive extraction, and a retry redoes everything from scratch. Splitting into extract, transform, and load tasks lets a failed load retry without re-extracting, and a failed transform retry against already-staged data. But over-splitting into hundreds of micro-tasks has its own cost: scheduler overhead, metadata-store pressure, and a DAG that is hard to read. The right granularity is coarse enough to be cohesive and fine enough that failures and retries are cheap. Stage intermediate data at task boundaries so a downstream retry does not force an upstream redo.

### Model Dependencies Explicitly, Never By Schedule Timing

A dependency that is expressed as "task B runs at 2am because task A finishes by 1am" is a latent failure: when A is late, B runs on stale or partial data and reports success. Dependencies must be explicit edges in the graph, not implicit assumptions about timing. If B consumes A's output, B must have a dependency edge on A (or on the dataset/partition A produces), so the orchestrator waits for A or fails loudly — never silently processes partial data.

Distinguish task dependencies (within a DAG: A before B) from data dependencies (across pipelines: B waits on A's output partition). Within a DAG, model edges directly. Across pipelines, depend on the dataset or an event signal rather than on another pipeline's schedule, so cross-team dependencies are enforced rather than hoped. A sensor that waits for a file or partition and times out loudly is safer than a blind scheduled run that assumes upstream is ready.

### Use Fan-Out And Fan-In Deliberately, And Bound Them

Fan-out (one task spawning many parallel tasks) and fan-in (many parallel tasks feeding a final aggregation) are powerful but have structural risks. Fan-out is how you parallelize — one task per source table, per partition, per region — and fan-in is how you reconcile or aggregate the results. Used well, they turn a slow serial pipeline into a fast parallel one. Used carelessly, they destabilize the orchestrator.

Static fan-out (a fixed set of parallel branches) is predictable and easy to reason about. Dynamic fan-out (task count determined at runtime, e.g., one task per file that arrived today, one task per partition in a backfill range) is more powerful but must be bounded: a fan-out that expands to thousands of tasks strains the scheduler's metadata store, floods the UI, and can exhaust worker slots. Use dynamic task mapping with a sane ceiling, batch very large fan-outs, and ensure the fan-in task tolerates partial failure (does one failed branch fail the whole aggregation, or are branches independent?). Decide explicitly whether fan-in is strict (all branches must succeed) or tolerant (aggregate whatever succeeded), because the default is often wrong for the workload.

### Make Every Task Idempotent And Safely Retriable

The orchestrator will retry tasks — after failure, after a redeploy, during a backfill. A task that is not idempotent corrupts data on every retry: it appends duplicate rows, double-counts, or layers stale data over fresh. Idempotency — running a task N times produces the same result as running it once — is a structural requirement of every task, not an optimization.

Make tasks idempotent by writing outputs to a location keyed by the run's logical partition (date, batch id) with overwrite or upsert/merge semantics, never blind append. Make transformations deterministic: a task that depends on the current wall-clock time produces different output when backfilled, so historical results drift from the original — capture the relevant time (event time, batch time) as data and derive from it. The test: can you re-run yesterday's task, or backfill the last 90 days, and end up with exactly the correct state? If re-running risks duplication or drift, the task is not production-ready, and no amount of retry logic will save it.

### Plan The Backfill As A Structural Concern, Not An Afterthought

Backfill — reprocessing historical data after a fix, a new transformation, or to populate a new dataset — is inevitable, and whether it is easy or catastrophic is determined by DAG structure, not by the backfill command. A pipeline whose tasks are partitioned by date and idempotent can backfill a range by running each partition once, in parallel, throttled. A pipeline whose tasks depend on "now," or whose loads blind-append, or whose intermediate data was never staged, cannot backfill cleanly — it duplicates, drifts, or cannot run at all.

Design for backfill from the start: partition tasks by the data's temporal key so a backfill maps cleanly onto partitions; stage raw and intermediate data so a transformation fix can be re-applied without re-extracting from the source; ensure determinism so backfilled history matches the original; and verify the source's historical availability and cost before you need it. Throttle backfill concurrency and run it in a separate queue so it does not starve production runs.

### Separate Orchestration Logic From Transformation Logic

The orchestrator is the control plane: it decides what runs, when, and in what order. The transformation is the data plane: the actual extract, transform, and load logic. Mixing them — embedding SQL transforms inside operator callbacks, putting business logic in scheduling code — creates code that cannot be unit-tested without the orchestrator, cannot be reused outside it, and couples the data logic's lifecycle to the orchestrator's. Keep transformation logic in versioned, testable modules (functions, dbt models, SQL files) that the DAG merely invokes. The DAG should read as a description of what runs and in what order, not as the implementation of the transform. This separation lets you test the transform in isolation, reuse it in a backfill or ad-hoc run, and change orchestrators without rewriting the data logic.

## Common Traps

### The Monolithic Task That Fails Atomically

One giant task doing extract-transform-load, so any error late in the process discards all prior work and forces a full restart from extraction. Split into cohesive tasks with staged intermediate data so a downstream retry does not redo upstream work.

### Tasks That Are Not Idempotent Under Retry

A task that blind-appends or depends on "now," so every orchestrator retry or backfill duplicates data or produces historical drift. Make every task write to a keyed partition with overwrite/upsert and derive time from data, not from the wall clock.

### Dependencies Expressed By Schedule Timing

Wiring "B runs after A" via clock alignment rather than an explicit edge, so a late upstream produces silent stale-data success. Express dependencies as graph edges or dataset/event triggers.

### Unbounded Dynamic Fan-Out

A runtime fan-out that expands to thousands of tasks, overwhelming the scheduler, flooding the UI, and exhausting workers. Bound dynamic task mapping, batch large fan-outs, and verify the orchestrator can handle the task count.

### Transformation Logic Embedded In The Orchestrator

Business logic buried in operator callbacks, untestable without the orchestrator and unreusable in backfills. Keep transforms in versioned, testable modules the DAG invokes.

### A Backfill That Cannot Target Historical Partitions

A pipeline structured only for forward-going data, so a backfill cannot run a historical range, duplicates on re-run, or drifts because of "now" dependence. Partition by temporal key, stage intermediates, and ensure determinism from the start.

### Fan-In With The Wrong Partial-Failure Policy

A fan-in aggregation that fails entirely if one branch fails (when branches are independent and should be tolerated), or that silently aggregates partial results (when branches are interdependent and must all succeed). Make the partial-failure policy explicit per fan-in.

## Self-Check

- [ ] Task boundaries are chosen so each task is cohesive, independently re-runnable, and a sensible unit of failure — no monolithic extract-transform-load task that fails atomically and loses all progress — with intermediate data staged at boundaries so retries are cheap.
- [ ] Dependencies are explicit graph edges or dataset/event triggers, never schedule timing, so a late upstream is waited on or fails loudly rather than producing silent stale-data success.
- [ ] Fan-out/fan-in is deliberate: static fan-out where predictable, dynamic fan-out bounded and batched, and each fan-in's partial-failure policy (strict vs tolerant) decided explicitly rather than left to the default.
- [ ] Every task is idempotent — outputs written to a keyed partition with overwrite/upsert (never blind append), transformations deterministic with no "now" dependence — so re-running a task or backfilling a partition produces exactly the correct state without duplication or drift.
- [ ] Backfill is a structural concern handled from the start: tasks partitioned by temporal key, raw and intermediate data staged, transformations deterministic, source historical availability verified, and backfill concurrency throttled in a separate queue.
- [ ] Orchestration logic is separated from transformation logic: transforms live in versioned, testable modules the DAG invokes, not embedded in operator callbacks, so they can be unit-tested and reused in backfills or a different orchestrator.
- [ ] The highest-risk cases were verified — a mid-pipeline failure that retries without redoing upstream work, a retry/backfill that must not duplicate, a dynamic fan-out at production scale, and a backfill over a historical range — not only the clean single-run happy path.
