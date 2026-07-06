---
name: stream_processing_and_backpressure.md
description: Use when the agent is building or reviewing a real-time or near-real-time data processing system; deciding whether a workload needs stream processing versus batch; choosing a stream processing framework or engine (Kafka Streams, Flink, Spark Structured Streaming, ksqlDB, Pulsar Functions, cloud managed streaming); reasoning about event time versus processing time, watermarks, and late-arriving data; designing windowing (tumbling, sliding, session); implementing backpressure and flow control between producers and consumers; managing stateful processing, checkpoints, and savepoints; aiming for exactly-once or effectively-once semantics; handling event ordering and out-of-order data; or diagnosing lag, state blowup, checkpoint failures, duplicate output, or windows that emit wrong results. Also covers the myth of true exactly-once, the cost of strong ordering and stateful processing, and the failure mode of treating a stream like a batch that happens to run continuously.
---

# Stream Processing And Backpressure

Stream processing is the judgment of computing over data that arrives continuously, unbounded, and often out of order, where the answer must be produced while the world is still moving. It is not "a batch job that runs more often." Batch operates on a known, closed dataset: you can sort it, wait for it, and compute an exact answer. A stream never closes; at any moment you have seen only part of the data, events arrive late and out of order, and the system must produce a provisional answer and revise it as more arrives. Every stream processing decision — how to define "when" an event happened, when a window closes, how to handle data that arrives after the close, how to keep state consistent across failures, how to avoid being overwhelmed — is a judgment about trading correctness, latency, and cost against a dataset that is never complete.

Agents tend to under-invest here because the batch mental model works on the happy path: events arrive roughly in order, windows close roughly on time, and the output looks right. The harm appears only under the conditions that define real streams: a burst of late events that belong to a window already emitted (wrong aggregates), a producer that outruns the consumer (lag, state blowup, dropped data), a failure mid-checkpoint (duplicated or lost output), a key whose events arrive out of order (corrupted stateful computation), a "real-time" requirement that is actually fine with minutes of delay (over-engineered, fragile). The judgment problem is deciding, for each data flow, whether it genuinely needs streaming, and if so, how to define time, handle lateness and backpressure, manage state, and choose delivery semantics that match what the business can tolerate.

This skill covers stream processing design: time semantics, windowing, backpressure, state and checkpointing, delivery semantics, and ordering. It complements the message-queue skill (which covers the broker, partitioning, and delivery mechanics) and the reliability skill (which covers retries and timeouts at the call site). Here the question is the architecture of continuous computation over unbounded data.

## Core Rules

### Decide Whether The Workload Actually Needs Streaming — And At What Latency

Streaming is harder and more expensive than batch, and the first question is whether the business needs it. Streaming is justified when the value of the result decays with time — fraud detection, monitoring, real-time pricing, live recommendations, anomaly detection — where an answer minutes or hours late is worth much less. It is not justified when the data is consumed on a schedule anyway (a daily report, a weekly aggregate) or when near-real-time adds cost without changing decisions.

- **Latency requirement drives the choice.** If the business needs results in seconds to low minutes, streaming is appropriate. If minutes-to-hours is acceptable, micro-batch (Spark Structured Streaming) or frequent batch may be simpler and more robust. If daily is fine, batch is correct and streaming is over-engineering.
- **Streaming adds operational cost.** Stateful stream processors require checkpointing, state management, monitoring of lag and watermarks, and handling of late data — all absent from a batch job. Do not pay this cost unless the latency requirement demands it.
- **Lambda vs pure streaming.** Some workloads combine a fast (approximate) streaming path with a slow (accurate) batch path that corrects it. This is legitimate but doubles the logic and the reconciliation burden; prefer a single well-designed streaming path with late-data handling where feasible.

Name the latency requirement first. "Real-time" is not a requirement; "results within 10 seconds of the event, with events up to 1 hour late incorporated" is.

### Separate Event Time From Processing Time — And Decide Which Governs Correctness

This is the foundational concept that most stream bugs get wrong. Every event has two timestamps: **event time** (when the event actually occurred in the world) and **processing time** (when the processing system received it). They diverge because of producer buffering, network delay, retries, and outages — an event generated at 10:00 may arrive at 10:05, or at 11:00 after a downstream outage recovers.

- **Correctness over real-world data almost always requires event time.** A per-minute aggregate of clicks is correct only when events are grouped by when they happened, not by when the stream processor happened to receive them. Grouping by processing time produces wrong answers whenever arrival is delayed or bursty.
- **Processing time is simpler but wrong under delay.** It is tempting because it needs no notion of lateness or watermarks — you process whatever arrived in the wall-clock window. But it silently misattributes delayed events to the wrong period, and the error grows with any arrival skew.
- **The gap is the core complexity.** Event-time processing requires the system to hold state for windows that have not yet closed, wait for late events, and decide when to stop waiting (the watermark). This is the price of correctness over a never-complete dataset.

Choose event time whenever the result must reflect when events actually occurred. Accept processing time only for workloads where arrival-time grouping is semantically correct (e.g., "events processed in this hour" as an operational metric) or where approximation is explicitly acceptable.

### Use Watermarks To Define Progress, And Decide Deliberately How Late Is Too Late

Because a stream never ends, the system needs a way to decide "no more events for this window will arrive, so I can emit the final result." The **watermark** is that mechanism: an assertion that the system does not expect to see events with event time earlier than the watermark. When the watermark passes a window's close, the window finalizes.

- **The watermark is a heuristic, not a guarantee.** It is usually derived from observed event-time progress (e.g., the max event time seen minus a configured allowed lateness). It trades off latency against completeness: an aggressive watermark (small allowed lateness) emits fast but drops genuinely late events; a conservative watermark (large allowed lateness) captures more events but delays output and holds more state.
- **Late data must have a defined handling.** An event that arrives after the watermark has passed its window is "late." Options: drop it (fast, lossy), update the already-emitted window (requires the downstream to accept retractions/corrections), or route it to a side output for separate handling. The choice depends on whether the late event carries enough value to justify the complexity of corrections.
- **Watermarks assume event-time progress.** If a partition stalls (no new events), the watermark stalls and downstream windows never close, blocking output. This is a real failure mode; systems offer idle-timeout or punctuation mechanisms to advance stalled watermarks.

Set the allowed lateness against the real arrival distribution, not a guess. Measure how late events actually arrive in production, set the watermark to capture the bulk of them, and define explicit handling for the long tail beyond it.

### Choose The Window By The Question Being Asked

Windowing slices an unbounded stream into finite chunks to aggregate. The window type must match the question, and the wrong type produces confidently wrong aggregates:

- **Tumbling windows** — fixed-size, non-overlapping (e.g., each 1-minute period, disjoint). Use for "how many events in each minute/hour." Simple and disjoint; each event belongs to exactly one window.
- **Sliding windows** — fixed-size, overlapping (e.g., a 5-minute window emitted every 1 minute). Use for moving averages and rolling rates. An event can belong to multiple windows; the overlap smooths short-term variation.
- **Session windows** — variable-size, grouped by gaps of inactivity (e.g., a user session ends after 30 minutes of no activity). Use for per-user or per-entity activity bursts where the boundaries are behavioral, not clock-based. Requires a gap timeout and a key.
- **Global window** — no natural segmentation; requires a custom trigger to emit. Almost always a sign the design should use one of the above or a different mechanism; an unbounded global window accumulates state forever.

Match the window to the semantics of the question. "Active users per minute" is tumbling; "average request rate over the last 5 minutes" is sliding; "sessions per user" is session. A tumbling window used where a session is needed merges distinct user activities and splits a single session across boundaries.

### Make Backpressure A Designed Property — Producers Must Not Outrun Consumers

In a continuous pipeline, if the producer emits faster than the consumer can process, the unconsumed data accumulates somewhere — in a queue, in memory, on disk — until that buffer fills and the system fails, often catastrophically (out of memory, broker disk full, oldest data dropped under retention). Backpressure is the mechanism that propagates "slow down" from the consumer to the producer so the system runs at the sustainable rate. It must be designed, not hoped for.

- **Reactive/stream-native backpressure.** Frameworks with native backpressure (Flink, reactive streams, Akka streams) propagate demand: the consumer signals how much it can accept, and the producer emits only that much. This bounds in-flight data naturally and is the most robust model for streaming.
- **Buffer-based with explicit overflow policy.** Where native backpressure is unavailable, a bounded buffer with a deliberate policy: block the producer (preserves all data, adds latency), shed load (drop low-priority data), or sample. An unbounded buffer with no policy is a latent outage.
- **Downstream is the usual bottleneck.** A fast producer into a slow database, API, or sink will back up regardless of the processor's own speed. Identify the slowest stage and ensure backpressure reaches all the way to the producer; a backpressure that stops at an intermediate buffer does not protect the system.
- **Retention vs backpressure.** In log-based systems (Kafka), the consumer falling behind does not immediately fail — events accumulate in the log up to retention. This masks the problem until retention is exceeded and data is silently dropped. Monitor consumer lag against retention; lag is the leading indicator of an unsustainable rate.

The default question: what happens if the producer runs at full speed and the consumer is down or slow? If the answer is "data accumulates until something breaks," backpressure is not designed.

### Manage State Explicitly — It Is Where Correctness And Cost Live

Stateful stream processing (aggregates, joins, per-key accumulators) holds state that must survive failures and be consistent. State is both the source of correctness and the dominant operational cost, and it must be designed deliberately:

- **State must be checkpointed for fault tolerance.** On failure, the processor restarts from a checkpoint rather than from scratch. Checkpointing periodically snapshots state and the input position so recovery resumes consistently. The checkpoint interval trades recovery cost (more data to reprocess on failure) against checkpoint overhead (frequent snapshots slow processing).
- **Exactly-once requires state and output to checkpoint together.** To avoid emitting duplicates or losing output on recovery, the output commit must be tied to the checkpoint — either transactionally (two-phase commit to a transactional sink) or via idempotent sinks with deterministic offsets. Without this, recovery produces duplicates or gaps.
- **State size must be bounded.** State that grows with the key space or with time (e.g., per-user windows held forever, unbounded joins) eventually exhausts memory or disk. Use state TTL/cleanup, window eviction, and keyed state that shards across the cluster. Unbounded state is a slow leak that becomes an outage.
- **State backend choice matters.** In-memory state is fast but limited; RocksDB-style on-disk state handles large state at the cost of I/O. Match the backend to the state size and latency requirement.

Treat state as a first-class artifact: where it lives, how big it gets, how it is checkpointed, and how it is recovered. A stateful processor with no answer to "what happens to state on failure and over time" is not production-ready.

### Understand What "Exactly-Once" Really Means — And Aim For Effectively-Once

True end-to-end exactly-once processing — where every input event affects the output exactly once, despite failures, retries, and redelivery — is not achievable by the stream processor alone; it requires cooperation from sources and sinks. What frameworks offer is **effectively-once**: under specific conditions (checkpointing, transactional or idempotent sinks, deterministic processing), the observable output is as if each event was processed once.

- **Effectively-once requires idempotent or transactional sinks.** If the sink deduplicates (idempotent writes by key) or commits atomically with the checkpoint (transactional sinks), then redelivery during recovery produces no duplicate observable effect. Without this, recovery duplicates output.
- **Nondeterminism breaks exactly-once.** If processing is nondeterministic (uses processing time, wall-clock, random values, non-deterministic external calls), reprocessing the same events after recovery produces different results, and the "once" guarantee collapses. Deterministic processing is a prerequisite.
- **The cost is real.** Checkpointing, transactional sinks, and coordination add overhead and complexity. Many workloads do not need effectively-once and are better served by at-least-once with idempotent consumers, accepting that recovery may reprocess but not double-apply effects.

Decide per flow: does the business require that an event's effect appear exactly once in the output (financial aggregates, billing), or is at-least-once with idempotent effects acceptable (most analytics, most monitoring)? Require effectively-once only where the cost of duplicate or missing effects justifies the complexity, and verify the source, processor, and sink all participate.

### Treat Ordering As Per-Key And Explicit

Global ordering across an unbounded, parallel stream is impractical and usually unnecessary. Ordering requirements must be stated per key and matched to the partitioning:

- **Per-key ordering is the common requirement.** Events for the same entity (same user, same order, same device) must often be processed in event-time order for stateful logic to be correct (e.g., a balance update before a withdrawal). Achieve this by partitioning on the key so all its events route to one parallel instance that processes them in order.
- **Cross-key ordering is usually unnecessary and expensive.** Requiring global order forces a single serial processing path, destroying parallelism. Question any requirement for global ordering; it usually indicates the design should be rethought.
- **Out-of-order within a key still happens.** Even per-key, events can arrive out of event-time order due to retries or source skew. Stateful operators must tolerate this (e.g., update state for a late event, or hold state until the watermark allows emission), or they produce wrong results.

State the ordering requirement per key, partition so each key's events are co-located, and confirm the operator handles intra-key reordering. "Events are processed in order" is not true by default in a parallel stream.

## Common Traps

### Treating A Stream Like A Batch That Runs Continuously

Designing the pipeline as a batch job with a short trigger, ignoring event time, watermarks, and late data. Results are grouped by processing time, so delayed or bursty arrival misattributes events to the wrong period. Use event time and explicit lateness handling whenever correctness reflects when events occurred.

### Processing-Time Windows For Real-World Aggregates

Aggregating "events per minute" by the wall-clock minute they were processed in, so an event delayed from 10:00 to 10:05 lands in the 10:05 bucket. The aggregate is operationally meaningful (what we processed) but factually wrong (what happened). Use event-time windows with watermarks for real-world aggregates.

### No Backpressure, So Lag Grows Until Data Is Lost

A producer outrunning a consumer with no flow control, accumulating data in a buffer or log until memory, disk, or retention is exhausted and the oldest data is silently dropped. Design backpressure (native demand propagation or bounded buffers with a policy) and monitor consumer lag against retention.

### Watermark Set Too Aggressively, Dropping Late Events

An allowed-lateness too small for the real arrival distribution, so events that genuinely arrive late are dropped or routed away, silently undercounting aggregates. Measure actual arrival lateness in production and set the watermark to capture the bulk of it, with explicit handling for the tail.

### Watermark Set Too Conservatively, Stalling Output

An allowed-lateness so large that windows wait a long time to close, delaying output and holding excessive state. Or a stalled partition whose watermark never advances, blocking downstream windows. Balance lateness against latency, and handle idle partitions with timeouts.

### Unbounded State That Grows Until Failure

Per-key windows, joins, or accumulators held indefinitely, so state grows with the key space or time until memory or disk is exhausted. Bound state with TTL, window eviction, and keyed sharding; monitor state size.

### Assuming The Framework Provides Exactly-Once End-To-End and nondeterministic Processing Breaking Recovery

Reading "exactly-once" in the framework's feature list and using a non-idempotent, non-transactional sink, so recovery after a failure produces duplicate output. Effectively-once requires the source, processor, and sink to participate; verify the sink deduplicates or commits transactionally with the checkpoint.

Using wall-clock time, randomness, or non-deterministic external calls inside the processing logic, so reprocessing the same events after recovery yields different results and the exactly-once guarantee collapses. Make stateful processing deterministic for recoverable pipelines.

### Global Ordering Requirement That Kills Parallelism and choosing Streaming When Batch Would Do

Demanding all events across the stream be processed in order, forcing a single serial path that cannot scale. Identify the real per-key ordering requirement, partition on the key, and process keys in parallel.

Building a complex stateful streaming pipeline for a workload that is consumed daily or hourly, paying checkpointing, state, and operational cost for latency the business does not need. Match the architecture to the actual latency requirement; batch or micro-batch is often simpler and more robust.

## Self-Check

- [ ] The latency requirement is stated concretely (e.g., "results within N seconds, events up to M late incorporated"), and streaming was chosen because the value of the result decays with time — not because "real-time" sounded better than batch.
- [ ] Event time governs correctness wherever the result must reflect when events occurred; processing time is used only where arrival-time grouping is semantically correct or approximation is explicitly acceptable, and the two are not conflated.
- [ ] The watermark and allowed lateness are set against the measured arrival distribution in production (not a guess), with explicit handling for late data beyond the watermark (drop, retract/correct, or side output) and for stalled/idle partitions.
- [ ] The window type (tumbling, sliding, session, global) matches the question being asked, and the choice is justified per aggregate rather than defaulted.
- [ ] Backpressure is a designed property: native demand propagation or bounded buffers with an explicit overflow policy (block, shed, sample), the slowest stage is identified, and consumer lag is monitored against retention so the producer cannot run the system into silent data loss.
- [ ] State is managed explicitly: checkpointed for fault tolerance, tied to output commit for effectively-once (transactional or idempotent sink), bounded by TTL/eviction/sharding, and on a backend matched to its size — with no unbounded growth path.
- [ ] Delivery semantics are chosen per flow: effectively-once (with deterministic processing and a participating sink) only where duplicate or missing effects are unacceptable; at-least-once with idempotent effects elsewhere — and the source, processor, and sink all participate in the chosen semantic.
- [ ] Ordering is specified per key, partitioning co-locates each key's events, and stateful operators tolerate intra-key reordering; no global-ordering requirement is forcing a serial bottleneck.
- [ ] The framework/engine choice (Kafka Streams, Flink, Spark Structured Streaming, etc.) matches the workload's state, latency, and exactly-once needs, and its checkpointing, state backend, and late-data mechanisms are configured rather than defaulted.
- [ ] The highest-risk cases were verified — late events into already-emitted windows, producer outrunning consumer, checkpoint failure and recovery, out-of-order per-key events, stalled watermarks, and state growth — not only the in-order, steady-rate happy path.
