---
name: pipeline_orchestration_and_scheduling.md
description: Use when the agent is designing or reviewing a data pipeline orchestration system (Airflow, Dagster, Prefect, dbt jobs, Argo Workflows), defining task dependencies and DAGs, scheduling recurring runs, handling cross-pipeline dependencies, managing SLAs and freshness contracts, planning backfills and catch-up runs, choosing between scheduled batches and event-triggered runs, or deciding retry, concurrency, and queueing policies for orchestrated tasks. Also covers the failure modes of tightly coupled DAGs that cascade failures, blind catch-up runs that reprocess the world, missed upstream dependencies producing silent staleness, SLA violations discovered too late, and the recurring mistake of treating orchestration as a cron replacement rather than a dependency-aware, observable control plane.
---

# Pipeline Orchestration And Scheduling

Orchestration is the control plane of data movement: it decides which tasks run, in what order, when they run, what happens when an upstream task is late or fails, and how a missed run is caught up. The mechanics — define a DAG, schedule it daily — look trivial, and a two-task pipeline on a cron schedule works fine. The judgment problem appears at scale, where dozens of pipelines share infrastructure, depend on each other and on external systems, must meet freshness SLAs for downstream consumers, and must recover from failures without reprocessing the world or leaving gaps. An orchestration system that is treated as "cron with retries" produces pipelines that fail in cascades (one late upstream task breaks ten downstream DAGs), miss SLAs silently (a run is hours late and no one knows until a dashboard is stale), or reprocess destructively on catch-up (a week of backfills floods the warehouse and duplicates data). The discipline is to treat orchestration as a dependency-aware, observable, SLA-governed control plane, not a scheduler.

Agents tend to under-invest here because the happy path is trivial — a DAG with two tasks on a daily schedule succeeds, and the orchestrator handles retries. The harm appears when pipelines multiply and interdepend. A downstream DAG runs on a fixed schedule regardless of whether its upstream completed, so it processes stale or partial data. A tightly coupled DAG fails entirely when one leaf task fails, blocking tasks that did not depend on it. A catch-up run after a two-day outage fires 48 hourly runs in a burst, overwhelming the warehouse and hitting rate limits. An SLA is defined but never monitored, so a run that is three hours late is discovered by a frustrated consumer, not by an alert. The judgment problem is to design the orchestration topology, scheduling, dependency model, and operational policies so that pipelines run in the right order, recover gracefully, meet their freshness contracts, and fail loudly and locally rather than silently and globally.

This skill covers DAG and dependency design, scheduling and triggering, SLAs and freshness, backfill and catch-up, concurrency and resource management, and orchestration observability. It complements the etl-design skill (data movement and quality within a task), the schema-evolution skill (contract evolution), and the data-lineage skill (tracing). Here the focus is the control plane that governs when and how tasks execute and depend on each other.

## Core Rules

### Model Dependencies Explicitly, Do Not Rely On Schedule Timing

The most common orchestration mistake is expressing a dependency implicitly through schedule timing — "pipeline B runs at 2am because pipeline A finishes by 1am." This is a latent failure: when A is late (a slow source, a retry), B runs on stale or partial data with no error. Dependencies must be explicit and enforced:

- **Express data dependencies, not just task order.** If pipeline B consumes pipeline A's output, B should depend on A's completion (a sensor, an event, a dataset trigger), not on a clock. Schedule-timed dependencies break silently when upstream is late.
- **Use sensors or dataset/event triggers for external dependencies.** If a pipeline depends on an external system (a source file landing, an upstream team's table being ready), wait on a signal that it is ready, rather than assuming it will be done by a fixed time. A sensor that polls for the file's arrival and times out loudly is safer than a blind scheduled run.
- **Distinguish task dependencies from cross-pipeline dependencies.** Within a DAG, model task dependencies as a DAG (A before B). Across pipelines, model dependencies on datasets or events (B triggers when A's output partition is ready), not on A's schedule.
- **Make missing dependencies visible, not silent.** A pipeline that runs on a schedule without its upstream ready should either wait (sensor) or fail loudly — never silently process partial data and report success.

### Design DAG Topology For Localized Failure And Parallelism

A DAG's topology determines what fails together and what can run in parallel. A poorly structured DAG cascades failures and serializes work that could be parallel:

- **Keep tasks decoupled so a leaf failure does not block unrelated tasks.** A monolithic task that does everything fails entirely if any step fails. Split into independent tasks with explicit dependencies, so a failure in one branch does not block tasks in another branch that does not depend on it.
- **Parallelize independent tasks, serialize dependent ones.** Tasks with no dependency between them should run concurrently (subject to resource limits); only genuinely dependent tasks must be serial. A fully serial DAG where tasks could be parallel wastes time; a fully parallel DAG where tasks have hidden dependencies produces race conditions.
- **Avoid overly deep or wide DAGs that strain the orchestrator.** A DAG with 10,000 tasks (one per partition, one per table) strains the scheduler's metadata store and UI. Use dynamic task mapping or sub-DAGs judiciously; a giant fan-out that worked in dev may destabilize the orchestrator in production.
- **Model idempotency at the task level.** Each task should be safely re-runnable (see the etl-design skill), so the orchestrator can retry a failed task without manual cleanup. A task that is not idempotent turns every retry into a corruption risk.

### Choose Scheduling And Triggering By The Data's Nature

The decision between scheduled batches, event triggers, and continuous streaming is a judgment about the data's arrival pattern and the freshness required:

- **Scheduled batches fit periodic, batch-oriented sources.** A daily warehouse load from a source that updates nightly fits a scheduled daily run. Schedule-based orchestration is simple and predictable for periodic data.
- **Event or dataset triggers fit continuous or unpredictable sources.** A pipeline that should run when a file lands, when an upstream partition is ready, or when a source signals completion fits an event/dataset trigger — it runs at the right time regardless of schedule, with no idle gaps and no polling waste.
- **Match trigger frequency to freshness requirements, not to habit.** If consumers need data within an hour, an hourly schedule (or event trigger) is right; if they need it daily, a daily batch is cheaper and simpler. Over-scheduling (hourly when daily suffices) wastes resources; under-scheduling (daily when hourly is needed) misses the SLA.
- **Beware trigger storms.** An event trigger that fires per-record can launch millions of runs; batch the trigger (fire per file, per micro-batch, or on a debounce) so the orchestrator is not overwhelmed by run metadata.

### Define And Monitor SLAs And Freshness Contracts

A pipeline exists to deliver data to consumers, and consumers need it by a certain time. An SLA (the maximum acceptable delay between data availability at the source and availability to consumers) makes that contract explicit and monitorable:

- **Define a freshness SLA per pipeline.** State, for each consumer-facing dataset, how fresh it must be (e.g., "today's data available by 7am") and derive the pipeline's required completion time. Without an SLA, "late" is undefined and unmonitorable.
- **Monitor SLA attainment and alert on breach.** Track whether each run completes within its SLA and alert when a run is at risk of breaching (running long) or has breached. An SLA discovered by a consumer complaint is a failed SLA.
- **Account for the full dependency chain in the SLA.** A pipeline's SLA depends on its upstream pipelines' SLAs. If A must finish by 1am for B to finish by 3am, A's SLA is part of B's. Model the critical path, not just each pipeline in isolation.
- **Distinguish generation SLA from delivery SLA.** Data may be generated on time but delivered late (a slow load, a consumer lag). Monitor both the pipeline's completion and the consumer's actual access to the data.

### Plan Backfills And Catch-Up Deliberately

After an outage, a bug fix, or a schema change, pipelines must reprocess historical data. Catch-up and backfill are where orchestration most often goes wrong destructively:

- **Decide catch-up policy per pipeline: skip, run-once, or replay.** When a pipeline misses runs during an outage, decide whether to skip the missed runs (acceptable for idempotent aggregations that re-derive from source), run once to the latest (acceptable when the source still has the full history), or replay each missed run (needed when each run's logic is period-specific). The wrong choice either leaves gaps or floods the system.
- **Throttle catch-up to avoid resource storms.** Replaying 48 missed hourly runs all at once overwhelms the warehouse, hits rate limits, and can take down shared infrastructure. Throttle catch-up concurrency, run backfills in a separate queue, and pace them to avoid impacting production runs.
- **Ensure idempotency before backfilling.** A backfill re-runs historical partitions; if the pipeline is not idempotent (see the etl-design skill), the backfill duplicates or corrupts data. Verify idempotency (partition overwrite/upsert) before any backfill.
- **Backfill from staged raw data when possible.** If raw and intermediate layers are preserved, a transformation fix can be backfilled by re-running the transform over staged data, without re-extracting from the source (which may be rate-limited or historically unavailable).

### Manage Concurrency, Queues, And Shared Resources

Orchestrated tasks share infrastructure (warehouse compute, API rate limits, source system capacity). Unmanaged concurrency causes contention, throttling, and cascading failures:

- **Set per-pipeline and global concurrency limits.** Limit how many tasks a pipeline runs concurrently and how many runs of the same pipeline can exist, to prevent a single pipeline from monopolizing shared resources.
- **Use priority queues for production vs backfill.** A backfill flooding the queue should not block time-sensitive production runs. Separate queues or priority weighting keeps production SLAs safe during catch-up.
- **Respect external rate limits.** A pipeline that calls a rate-limited API or a source with connection limits must throttle its concurrency to stay within limits; unmanaged parallelism triggers throttling and backoff storms.
- **Pool and reuse expensive resources.** Database connections, authenticated clients, and compute clusters should be pooled and reused across tasks, not created per task, to avoid connection exhaustion and startup overhead.

### Make Orchestration Observable And Operable

An orchestrator running many pipelines is a production system that must be observable and operable, not a black box:

- **Monitor orchestrator health, not just pipeline status.** Track scheduler lag (runs launching late), queue depth, worker capacity, and metadata store health. A scheduler that is behind launches runs late, breaching SLAs even when pipelines themselves are healthy.
- **Track run-level and task-level metrics.** Duration, success/failure rates, retry counts, and SLA attainment per pipeline and per task reveal degradation before it becomes an outage.
- **Make failures actionable.** A failed task should report what failed, where (which task, which run, which partition), and why (the error), with logs and a clear retry/resume path. A failure that requires archaeology to diagnose lengthens every incident.
- **Support safe operational actions.** The orchestrator should support clearing a task state, marking a task success, re-running a partition, and pausing a pipeline — operations needed during incidents. An orchestrator that cannot be safely operated during an outage forces risky manual interventions.

## Common Traps

### Schedule-Timed Dependencies That Break When Upstream Is Late

Expressing "B runs at 2am because A finishes by 1am" as a clock dependency, so when A is late, B runs on stale data and reports success. Express dependencies on datasets/events/sensors, not on schedule timing.

### A Monolithic Task That Cascades On Any Failure

One giant task that does extraction, transformation, and load, so any failure fails the whole thing and blocks unrelated work. Split into decoupled tasks with explicit dependencies so failures are localized.

### Blind Catch-Up That Reprocesses The World

An outage of two days triggering 48 hourly runs at once, overwhelming the warehouse and hitting rate limits. Decide catch-up policy deliberately and throttle backfill concurrency.

### An SLA That Is Defined But Never Monitored

A freshness SLA stated in a doc but never alerted on, so a late run is discovered by a consumer complaint. Define the SLA, monitor attainment, and alert on at-risk and breached runs.

### Over-Scheduling Or Trigger Storms

Running hourly when daily suffices (wasted resources), or an event trigger firing per-record and launching millions of runs (orchestrator overload). Match trigger frequency to freshness needs and batch triggers.

### Non-Idempotent Tasks Under Retry And Backfill

A task that is not safely re-runnable, so every orchestrator retry or backfill risks duplicating or corrupting data. Make each task idempotent (partition overwrite/upsert) before relying on retries.

### Unmanaged Concurrency Contending On Shared Resources

A pipeline fanning out hundreds of parallel tasks against a rate-limited API or a shared warehouse, triggering throttling and backoff storms. Set concurrency limits, use priority queues, and respect external rate limits.

### Treating Orchestration As Cron With Retries

Using the orchestrator as a dumb scheduler with no dependency awareness, SLA monitoring, or observability, and being surprised by silent staleness and cascading failures. Treat orchestration as a dependency-aware, observable, SLA-governed control plane.

## Self-Check

- [ ] Dependencies are modeled explicitly (data/dataset/event/sensor triggers, not schedule timing), so a late upstream is waited on or fails loudly rather than producing stale-data success.
- [ ] DAG topology localizes failure (decoupled tasks, no monolithic task that fails wholesale) and parallelizes independent work while serializing only genuinely dependent tasks, without straining the orchestrator with extreme fan-out.
- [ ] Scheduling/triggering matches the data's arrival pattern and the freshness requirement (scheduled batches for periodic data, event/dataset triggers for continuous or unpredictable sources), with trigger storms avoided via batching/debouncing.
- [ ] A freshness SLA is defined per consumer-facing pipeline, monitored for attainment, and alerted on at-risk and breach, accounting for the full dependency chain (upstream SLAs) and distinguishing generation from delivery freshness.
- [ ] Catch-up and backfill policy is decided deliberately (skip / run-once / replay), throttled to avoid resource storms, run in a separate/priority queue, and gated on verified idempotency and staged raw data.
- [ ] Concurrency is managed: per-pipeline and global limits, priority queues separating production from backfill, external rate limits respected, and expensive resources pooled and reused.
- [ ] Orchestration is observable and operable: scheduler health (lag, queue depth, worker capacity), run/task metrics (duration, failure rates, SLA attainment), actionable failures (clear what/where/why and resume path), and safe operational actions (clear state, mark success, re-run partition, pause).
- [ ] The highest-risk cases were verified — an upstream running late, a catch-up storm, an SLA breach, a shared-resource contention, and a scheduler falling behind — not only the clean scheduled daily run.
