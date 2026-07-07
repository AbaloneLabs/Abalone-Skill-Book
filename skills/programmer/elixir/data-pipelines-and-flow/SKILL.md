---
name: elixir_data_pipelines_and_flow.md
description: Use when the agent is building data-processing pipelines in Elixir (Flow/GenStage/Broadway for backpressure-aware streaming, processing large files or message streams, consuming Kafka/SQS/Amazon SQS via Broadway, batching, rate limiting, backpressure), choosing between Enum (eager) and Stream (lazy), handling concurrency in pipelines (producer/consumer stages), or is diagnosing "memory blowup processing a large file", "no backpressure / upstream overwhelms downstream", "Flow slower than plain Enum for small data", "Broadway messages lost or duplicated", or pipeline-stall/throughput issues. Covers Enum vs Stream eagerness, GenStage/Flow/Broadway for backpressure streaming, producer/consumer demand, batching and rate limiting, exactly-once vs at-least-once semantics, and the traps of unbounded buffering, no backpressure, and over-engineering small data.
---

# Data Pipelines And Flow In Elixir

Elixir offers a spectrum for data processing: `Enum` (eager, materializes intermediate lists), `Stream` (lazy, computes on demand), and `GenStage`/`Flow`/`Broadway` (concurrent, backpressure-aware streaming for large/continuous data). Agents materialize a multi-million-row file into a list and OOM, build a pipeline with no backpressure so a fast producer overwhelms a slow consumer (memory blowup), reach for `Flow`/`GenStage` on small data where `Enum` is faster (the concurrency overhead exceeds the benefit), or consume a message stream (Kafka/SQS) without understanding the delivery semantics (at-least-once, so handlers must be idempotent). The judgment problem is to choose the right tool by data size and concurrency need, to enforce backpressure (demand-driven) so producers slow for slow consumers, to batch and rate-limit deliberately, and to handle message-stream semantics correctly.

Agents OOM large files, build backpressure-less pipelines, over-engineer small data, or assume exactly-once delivery. The remedy is `Stream` for laziness, `GenStage`/`Flow`/`Broadway` for concurrent backpressure streaming on large data, and idempotent handlers for at-least-once streams.

## Core Rules

### Choose Enum (Eager) vs Stream (Lazy) vs Flow/Broadway (Concurrent) By Data Size

- `Enum` (eager): materializes each intermediate list. Fast and simple for in-memory/small data. Use by default.
- `Stream` (lazy): computes per-element on traversal, no intermediate lists. Use for large files/sequences to avoid OOM, or for pipelines you may short-circuit (`Stream.take`).
- `GenStage`/`Flow`/`Broadway` (concurrent, backpressure-aware): multi-stage parallel processing for large/continuous data (a 10GB file, a Kafka topic, an SQS queue). Use when data is large, continuous, or benefits from parallelism; the concurrency overhead makes them *slower* than `Enum` for small data.

Match the tool to the workload: `Enum` for small/in-memory, `Stream` for large-but-sequential, `Flow`/`Broadway` for large/continuous/concurrent. Do not reach for `Flow` for a 1000-row list.

- `Enum` (eager) for small/in-memory data — the default.
- `Stream` (lazy) for large files/sequences (no OOM) or short-circuit pipelines.
- `Flow`/`Broadway` (concurrent, backpressure) for large/continuous/concurrent data; slower than `Enum` for small data.

### Enforce Backpressure (Demand-Driven) So Producers Slow For Slow Consumers

The core value of `GenStage`/`Flow`/`Broadway` is *backpressure*: a consumer signals *demand* to the producer, which only produces what's demanded, so a slow consumer is not overwhelmed. Without backpressure (a raw `GenServer` pushing to a faster producer, an unbounded queue), memory grows unbounded. In `GenStage`, producers are `:demand`-driven (the consumer's `min_demand`/`max_demand`); tune these to balance latency and throughput. In `Flow`, `Flow.from_enumerable`/`Flow.from_stage` with stages and windowing provides backpressure. In `Broadway`, the producer (a Kafka/SQS connector) is demand-driven by the pipeline. Verify backpressure exists: under load, the producer's queue should be bounded, not growing without limit.

- Backpressure = demand-driven production; a slow consumer throttles the producer.
- `GenStage` `min_demand`/`max_demand`; `Flow` stages/windowing; `Broadway` connectors are demand-driven.
- Without backpressure (unbounded queue/push), memory grows unbounded under load.

### Batch And Rate-Limit Deliberately

For high-throughput pipelines, batch processing (handle N items per stage invocation) amortizes per-item overhead (DB round-trips, API calls). `Broadway` has `broadway`'s `batchers` (group messages by key, process in batches) and `max_demand`; `Flow` has windowing. Rate limiting (a token-bucket/leaky-bucket on outbound calls) prevents overwhelming a downstream API/DB — implement with a `GenServer`/`Hammer` rate limiter, or the client's built-in throttling. Choose batch size by the downstream's economics (a DB insert batch of 100–1000; an API call batch by its limits). Too-large batches increase latency and memory; too-small lose the amortization.

- Batch to amortize per-item overhead (DB/API calls); `Broadway` batchers, `Flow` windowing.
- Rate-limit outbound calls (token bucket / `Hammer` / client throttling) to protect downstream.
- Batch size by downstream economics; balance latency/memory vs amortization.

### Handle Message-Stream Delivery Semantics (At-Least-Once, Idempotent Handlers)

Message-stream consumers (Broadway with Kafka/SQS, GenStage with a queue) deliver *at-least-once*: a message may be redelivered after a crash/rebalance, so handlers must be **idempotent** (processing the same message twice yields the same result) — track processed IDs, use upserts, or design operations to be naturally idempotent. Exactly-once is rare and expensive (requires transactional producer+consumer); do not assume it. Acknowledge (ack) only after the work is durably committed (DB write done), or a crash before ack redelivers (correct) but a crash after work-before-ack redelivers and re-does the work (must be idempotent). For ordering, partition by key (messages with the same key go to the same stage, preserving order within the key).

- At-least-once delivery is the norm; handlers must be idempotent (track IDs, upsert, naturally idempotent).
- Ack only after durable commit; do not assume exactly-once.
- Partition by key for per-key ordering; cross-key ordering is not guaranteed.

### Structure Stages For The Pipeline's Shape

A pipeline's stage count and parallelism should match the workload: a CPU-bound transform benefits from multiple parallel mapper stages (`Flow.map`, several stages); an I/O-bound stage (DB/API) benefits from concurrency (many stages) but bounded by the downstream's capacity. Use `GenStage`/`Flow`'s `stages` option to set parallelism per stage. Avoid a single bottleneck stage (its throughput caps the pipeline); parallelize the bottleneck. For windowing (aggregating over time/count), `Flow.windowed` groups events; choose static or dynamic windows by the aggregation need. Profile (Telemetry on stage events) to find the bottleneck.

- Stage parallelism matches the workload (CPU-bound → many mapper stages; I/O-bound → concurrency bounded by downstream).
- Parallelize bottleneck stages; a single slow stage caps throughput.
- Windowing (`Flow.windowed`) for aggregations; profile (Telemetry) to find the bottleneck.

## Common Traps

### Materializing A Large File Into A List (OOM)

`File.read!`/`Enum` on a multi-GB file blows memory. Use `File.stream!`/`Stream`.

### No Backpressure (Producer Overwhelms Consumer)

A push-based pipeline grows unbounded. Use demand-driven `GenStage`/`Flow`/`Broadway`.

### Flow/GenStage For Small Data

Concurrency overhead exceeds benefit; `Enum` is faster. Use `Flow` only for large/concurrent data.

### Assuming Exactly-Once Delivery

Message streams are at-least-once; non-idempotent handlers double-process. Make handlers idempotent.

### Acking Before The Work Is Committed

A crash after ack loses the message. Ack after durable commit.

### Single Bottleneck Stage

One slow stage caps the pipeline. Parallelize the bottleneck.

### Unbounded Outbound Calls (No Rate Limiting)

Overwhelming a downstream API/DB. Rate-limit/batch outbound calls.

### Over-Windowing / Too-Large Batches

High latency/memory. Tune batch/window size to the downstream economics.

## Self-Check

- [ ] The tool matches the workload: `Enum` for small/in-memory, `Stream` for large-but-sequential (no OOM), `Flow`/`GenStage`/`Broadway` for large/continuous/concurrent data (not used for small data where `Enum` is faster).
- [ ] The pipeline enforces backpressure (demand-driven production); under load, the producer's buffer is bounded, not growing without limit.
- [ ] Batching and rate limiting protect the downstream (batch size by economics, outbound rate-limited); batch/window sizes balance latency/memory vs amortization.
- [ ] Message-stream handlers are idempotent (at-least-once delivery assumed); acks happen after durable commit; partitioning by key preserves per-key ordering.
- [ ] Stage parallelism matches the workload (CPU-bound parallelized; I/O-bound bounded by downstream); bottleneck stages are parallelized, not left as a single cap.
- [ ] Large files/sequences use `File.stream!`/`Stream` (no materialization into a list); short-circuit pipelines use lazy streams.
- [ ] The pipeline is profiled (Telemetry on stage events) to find bottlenecks, and throughput/latency are measured, not assumed.
- [ ] The design has been considered under OOM, backpressure, delivery semantics, downstream protection, and bottleneck stages, and remains correct and efficient.
