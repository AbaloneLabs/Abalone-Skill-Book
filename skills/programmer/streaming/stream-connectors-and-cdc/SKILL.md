---
name: stream_connectors_and_cdc.md
description: Use when the agent is designing stream ingestion from databases (change data capture / CDC), choosing between log-based and query-based CDC, integrating external systems as stream sources or sinks (connectors), handling schema evolution in the stream, reasoning about exactly-once across a connector boundary, or diagnosing connector failures (offset loss, duplicate capture, missed changes, schema drift breaking consumers). Also covers the failure mode of query-based CDC missing deletes or updates that happen between polls, log-based CDC coupling consumers to the database's WAL/transaction log, connectors that cannot guarantee exactly-once across the boundary, and the recurring mistake of treating a CDC stream as a clean event stream when it carries the database's internal semantics (transactions, schema, ordering) that consumers must respect.
---

# Stream Connectors And CDC

Change data capture (CDC) turns a database's changes into a stream, letting downstream systems react to inserts, updates, and deletes in near-real-time without batch extraction. Connectors bridge stream processing systems to external sources and sinks (databases, message logs, object stores). The judgment problem is that CDC and connectors sit at the boundary between two systems with different semantics, and the seam is where correctness breaks. A query-based CDC polls the database and misses changes that happen between polls (or cannot capture deletes); a log-based CDC reads the transaction log and couples consumers to the database's internal mechanics (transaction boundaries, WAL retention, schema history). A connector may not preserve exactly-once across the boundary, producing duplicates or loss. The CDC stream carries the database's internal semantics — transactions that commit or roll back, schemas that evolve, ordering within and across tables — and consumers that treat it as a clean event stream break those semantics. The discipline is to choose CDC mode for completeness and coupling, to design connectors for delivery guarantees and offset management, to handle schema evolution in the stream, and to make consumers respect the database semantics the CDC carries.

Agents tend to wire a connector and see events flow, assuming the stream is correct and complete. The harm appears as silently missed deletes (query-based CDC), as duplicate events after a connector restart (offset not checkpointed with the consumer's state), as broken consumers after a schema change (the stream's schema evolved but consumers were not updated), and as misordered or partial events (a transaction's changes split across the stream, or cross-table ordering violated). The judgment is to treat the connector boundary as a contract that must preserve completeness, ordering, and delivery semantics; to choose CDC mode deliberately; to manage offsets and schema as first-class concerns; and to verify capture completeness and delivery guarantees under failure. A CDC stream that misses deletes or duplicates after recovery is worse than a batch pipeline, because it appears live and correct while being neither.

This skill covers CDC mode selection, connector delivery guarantees and offset management, schema evolution in the stream, and consumer-side database semantics. It complements the exactly-once-and-state skill (delivery semantics within the processor), the ordering-and-partitioning skill (message ordering), and the data-pipeline ETL skill (batch extraction). Here the focus is the source/sink connector boundary and CDC specifically.

## Core Rules

### Choose CDC Mode For Completeness And Coupling

CDC can be query-based (polling the database for changes) or log-based (reading the transaction log), and the choice governs what is captured and how tightly consumers couple to the database:

- **Log-based CDC captures all changes including deletes.** Reading the transaction log (WAL, binlog, oplog) captures every insert, update, and delete, at low database impact, and is the only reliable way to capture deletes and transient state. It couples consumers to the database's log format and retention.
- **Query-based CDC misses changes between polls and cannot reliably capture deletes.** Polling a "updated_at" column or a changes table misses changes that happen between polls (overwritten before the next poll) and cannot capture hard deletes (the row is gone). Use query-based CDC only when log-based is impossible and the missed-change risk is acceptable.
- **Assess the coupling log-based CDC introduces.** Log-based CDC depends on the database's transaction log format, retention period (the log is retained for a limited time; if the consumer falls behind past retention, changes are lost), and schema history. Plan for log retention, consumer lag monitoring, and schema history management.
- **Match CDC mode to the requirement.** If deletes and completeness matter (most CDC use cases: cache invalidation, search indexing, downstream replicas), log-based is required; if approximate recency suffices, query-based may be acceptable.

### Manage Offsets And Delivery Guarantees At The Connector Boundary

A connector reads from a source and writes to a sink (or the stream), and the boundary is where delivery guarantees must be designed, not assumed:

- **Checkpoint the source offset with the consumer's state.** If the offset advances independently of the consumer's committed state, a failure between them loses events (offset advanced, state not committed) or duplicates (state committed, offset not advanced). Use a framework that checkpoints both atomically, or commit offsets only after the downstream state is durably committed.
- **Make sinks idempotent or transactional across the boundary.** Exactly-once across a connector boundary requires the sink to be idempotent (keyed writes that replay identically) or transactional (commit output and offset atomically). A non-idempotent sink turns at-least-once delivery into duplicate outputs.
- **Handle connector restart and failover.** A connector that restarts must resume from the last committed offset, not re-read from the start (duplicate) or skip ahead (loss). Verify restart behavior and offset durability.
- **Do not assume the connector preserves exactly-once.** Many connectors provide at-least-once by default; verify the delivery guarantee under failure injection (kill the connector mid-stream and check for duplicates or loss).

### Handle Schema Evolution In The Stream

A CDC stream's schema is the source table's schema, and source schemas evolve. The stream must carry schema information and consumers must tolerate evolution:

- **Carry schema in the stream.** Each event (or the stream's metadata) should identify its schema version, so consumers can parse and adapt rather than assuming a fixed shape. A schemaless stream that breaks on every source change is fragile.
- **Use a schema registry for evolving schemas.** A schema registry (Avro, Protobuf with a registry) lets producers register new versions and consumers fetch the schema, enabling backward/forward compatibility checks before a change reaches production.
- **Plan for backward and forward compatibility.** Source schema changes (adding a column, renaming, changing a type) must be compatible with deployed consumers; incompatible changes require a coordinated consumer update or a migration. Define and enforce compatibility rules (backward-compatible adds, no breaking removals without notice).
- **Handle the schema-history problem for log-based CDC.** Log-based CDC needs the schema at the time of each log entry to parse it; if the schema history is lost or inconsistent with the log, the connector cannot parse old changes. Manage the schema history durably and consistently with the log.

### Make Consumers Respect The Database Semantics The CDC Carries

A CDC stream is not a clean event stream; it carries the source database's internal semantics, and consumers must respect them:

- **Respect transaction boundaries.** A database transaction's changes may appear as multiple events; a consumer that processes them independently sees a partial (uncommitted) state. Consumers that need transactional consistency must group events by transaction and apply them atomically, or tolerate intermediate states.
- **Respect ordering within and across tables.** Changes within a table have a defined order (the log order); changes across tables or partitions may not. Consumers that depend on cross-table ordering must not assume it from independent partitions.
- **Handle the before and after state.** Updates carry both the before and after image (in log-based CDC); consumers that need the old value (for diffing, for computing the change) must use the before image, not recompute it. Query-based CDC often lacks the before image.
- **Treat the stream as eventually consistent with the source.** The CDC stream lags the source; a consumer that reads the source directly and the stream will see inconsistency. Consumers that need consistency must read from one or the other, or use the stream to invalidate and re-fetch from the source.

## Common Traps

### Query-Based CDC Missing Deletes Or Between-Poll Changes

Using query-based CDC (polling updated_at) when deletes or fast changes matter, missing hard deletes and overwritten changes. Use log-based CDC when completeness is required.

### Log-Based CDC Losing Changes Past Log Retention

The consumer falling behind past the database's log retention, so old changes are gone before capture. Monitor consumer lag against log retention and size retention for the worst-case lag.

### Offset And State Not Checkpointed Together

The source offset advancing independently of the consumer's committed state, losing events (offset ahead of state) or duplicating (state ahead of offset) after a failure. Checkpoint both atomically.

### Non-Idempotent Sink Duplicating Across The Boundary

A sink that is not idempotent or transactional, turning at-least-once connector delivery into duplicate outputs after a restart or recovery. Make sinks idempotent (keyed writes) or transactional.

### Schema Drift Breaking Consumers

The source schema evolving without the stream carrying schema info or consumers tolerating evolution, breaking parsing on the next change. Carry schema versions and use a registry with compatibility rules.

### Treating The CDC Stream As A Clean Event Stream

Consumers ignoring transaction boundaries, cross-table ordering, or before/after images, producing partial or inconsistent state. Respect the database semantics the CDC carries.

### Assuming The Connector Preserves Exactly-Once

Assuming a connector's default is exactly-once when it is at-least-once, producing duplicates after failure. Verify the delivery guarantee under failure injection.

## Self-Check

- [ ] CDC mode is chosen for completeness and coupling: log-based when deletes and completeness matter (with log retention sized for worst-case consumer lag and schema history managed), query-based only when missed-change risk is acceptable.
- [ ] Offsets are checkpointed with the consumer's state (atomically, or offset committed only after downstream state is durably committed), the sink is idempotent or transactional across the boundary, and connector restart/failover resumes from the committed offset without duplication or loss.
- [ ] The stream carries schema information (schema version per event or in metadata), a schema registry with compatibility rules is used for evolving schemas, and log-based CDC's schema-history dependency is managed durably and consistently with the log.
- [ ] Consumers respect the database semantics the CDC carries: transaction boundaries (grouping events by transaction for atomic apply), ordering within and across tables (no cross-partition ordering assumption), before/after images (using the before image for diffs), and eventual consistency with the source (not mixing direct source reads with the stream).
- [ ] The connector's delivery guarantee was verified under failure injection (kill mid-stream, check for duplicates and loss), not assumed from documentation.
- [ ] The highest-risk cases were verified — query-based CDC missing a delete, log-based CDC losing changes past retention, offset/state divergence after failure, a schema change breaking consumers, a transaction's changes consumed as partial state — not only the clean "events are flowing" demo.
