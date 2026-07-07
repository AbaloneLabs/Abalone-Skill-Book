---
name: event_sourcing_and_cqrs.md
description: Use when the agent is designing or evaluating an event-sourced system or a CQRS (command-query responsibility segregation) architecture, deciding to store events as the source of truth and derive state from them, separating write models (commands) from read models (projections), handling event schema evolution and versioning, rebuilding state from an event log, managing eventual consistency between write and read sides, or snapshotting long event streams. Also covers the failure modes of unversioned events that break replay, projections that drift from the event log, eventual consistency gaps that serve stale reads, treating events as messages rather than immutable facts, and the recurring mistake of adopting event sourcing for its elegance without accounting for the complexity of versioning, projection rebuilding, and operational burden it introduces.
---

# Event Sourcing And CQRS

Event sourcing stores the domain's events (the things that happened) as the source of truth, and derives current state by replaying them — instead of storing only the current state and overwriting it on each change. CQRS separates the model that handles commands (writes) from the models that serve queries (reads), often because the read patterns differ so much from the write patterns that one model cannot serve both well. The judgment problem is that these patterns are powerful and elegant in the small, and demanding and complex in the large. An event-sourced system never forgets (the full history is retained), enables temporal queries ("what was the state at time T"), and decouples writes from reads — but it also requires evolving immutable events whose consumers may number in the dozens, rebuilding projections when logic changes, reconciling read models that drift from the log, and accepting that reads are eventually consistent with writes. Adopting these patterns without accounting for their operational complexity produces a system whose elegance in design becomes a burden in maintenance.

Agents tend to under-invest here because the patterns are intellectually appealing (a clean event log, decoupled read models) and the demo works — a few events, a single projection, instant consistency. The harm appears as the system ages. An event schema changes and dozens of projections and replays break, because events are immutable facts that cannot be edited in place. A projection's logic is fixed and the projection drifts from the log, serving wrong reads with no detection. A read model lags the write model and a user sees stale data at the worst moment (after their own write). An event stream grows to millions of events and rebuilding state from a full replay becomes impractical, with no snapshotting strategy. The judgment problem is to decide whether event sourcing and CQRS are warranted, to version events for a long life, to keep projections consistent and rebuildable, to manage eventual consistency honestly, and to handle the operational burden (snapshots, replays, drift detection) that the patterns impose.

This skill covers the decision to adopt these patterns, event design and versioning, projection management, eventual consistency, and operational concerns (snapshots, replays, drift). It complements the event-driven-architecture skill (events as communication), the saga-and-process-manager skill (multi-step processes often built on events), and the message-queue-design skill (delivery). Here the focus is event sourcing as the system of record and CQRS as the read/write separation.

## Core Rules

### Decide Whether Event Sourcing And CQRS Are Actually Warranted

These patterns add significant complexity. They should be adopted for the benefits they provide that simpler architectures cannot, not for elegance:

- **Event sourcing is warranted when the history itself is the asset.** Audit requirements ("every state change must be reconstructable"), temporal queries ("what was the balance on date X"), complex domain logic expressed cleanly as a sequence of domain events, and the ability to rebuild read models in new shapes from the log are genuine justifications. If none of these apply, storing current state directly is simpler and sufficient.
- **CQRS is warranted when read and write patterns diverge.** If the writes are command-driven domain operations and the reads are varied aggregations, search, or denormalized views that one model cannot serve efficiently, separating them helps. If reads and writes are similar (simple CRUD over the same shape), CQRS adds two models to maintain for no gain.
- **Account for the cost upfront.** Event sourcing imposes event versioning, projection management, snapshotting, and replay infrastructure; CQRS imposes eventual consistency between sides and the operational burden of multiple models. Adopt only where the benefit clearly exceeds this cost.
- **Do not adopt for elegance or fashion.** A clean event log is appealing in design; the versioning and operational burden is felt for years. Choose based on the domain's actual requirements.

### Design Events As Immutable, Versioned Domain Facts

Events in an event-sourced system are immutable once written, and they will be consumed by projections and replays for the system's life. Their design must anticipate long-lived evolution:

- **Events represent things that happened, in the past tense, and are immutable.** Name them as facts ("OrderPlaced," "PaymentCaptured"), not commands ("PlaceOrder"). Once written, an event is never edited or deleted; corrections are new events ("OrderCancelled"), not modifications.
- **Design events to be self-contained and forward-compatible.** Include the data needed to process the event without joining other state, and prefer additive evolution (new optional fields) over breaking changes. An event that depends on external state at replay time is fragile.
- **Version events and plan for evolution.** Event schemas will change; version them, and provide upcasting (transforming old events to new schemas on read) so old events remain processable. Unversioned events that change break every projection and replay.
- **Avoid leaking implementation details into events.** Events are part of a long-lived contract; internal field names, machine-specific data, or framework artifacts in events become technical debt. Model events in domain terms.

### Manage Projections: Build, Keep Consistent, Rebuild

Projections (read models) are derived from the event log, and their correctness depends on staying consistent with the log and being rebuildable when logic changes:

- **Projections must be deterministically rebuildable from the log.** The defining property of a projection is that it can be rebuilt from scratch by replaying the event log. If a projection holds state not derivable from the log, it is no longer a projection — it is a second source of truth, and consistency is lost.
- **Keep projection logic versioned and replayable.** When a projection's logic changes (a new field, a different aggregation), the projection must be rebuilt from the log with the new logic. Design for rebuild: idempotent projectors, a way to reprocess events, and a strategy to rebuild without downtime.
- **Detect and correct projection drift.** A projection can drift from the log (a missed event, a bug, a crash mid-projection). Periodically reconcile projections against the log, or rebuild them, to catch drift before it serves wrong reads silently.
- **Decouple projection update pace from the write side.** Different projections can update at different paces (a real-time dashboard vs a nightly report); design each projection's update strategy to match its freshness requirement, and make the lag observable.

### Handle Eventual Consistency Between Writes And Reads Honestly

CQRS separates the write model from read models, and they are eventually consistent — a read immediately after a write may not reflect it. This must be designed for, not hidden:

- **Accept and communicate eventual consistency.** Reads lag writes by the projection update latency. For most read-heavy workloads this is fine; for a user reading immediately after their own write ("read-your-writes" consistency), it is not, and the system must provide it (route the user's read to a projection caught up past their write, or serve the write's result directly).
- **Make the consistency lag observable and bounded.** Track how far behind each projection is (the last event processed vs the latest written) and alert if it exceeds the bound. Unbounded, undetected lag serves arbitrarily stale reads.
- **Decide per-read freshness requirements.** Not every read needs the latest data; match the read's freshness requirement to the projection's update pace. Over-requiring freshness (every read must be current) negates the decoupling benefit of CQRS.
- **Handle the "stale read after write" UX.** If a user acts on a stale read, the experience is broken; design the UX (optimistic updates, read-your-writes routing, explicit "processing" states) so the eventual consistency is not felt as a bug.

### Address Operational Concerns: Snapshots, Replays, Retention

An event-sourced system accumulates a long event log, and operating it requires strategies for size, rebuild, and retention:

- **Snapshot long event streams.** Rebuilding state for an entity with millions of events by full replay is impractical; snapshot periodic state ("at event N, the state was X") and replay forward from the snapshot. Design snapshotting and its invalidation (a logic change invalidates old snapshots) deliberately.
- **Plan for projection rebuilds at scale.** Rebuilding a projection over years of events is expensive; design rebuilds to be parallelizable, resumable, and runnable without taking the system down. A projection that takes days to rebuild and requires downtime is an operational liability.
- **Define event retention and archiving.** Infinite event retention is costly and may not be required; define retention (how long events are kept hot vs archived vs deleted) consistent with audit, legal, and rebuild requirements. Deleting events that projections depend on breaks rebuilds.
- **Monitor the event store's health.** The event store is the system of record; its availability, write latency, and storage growth are critical. Treat it with the operational rigor of the most critical database.

## Common Traps

### Adopting For Elegance Without Accounting For Complexity

Choosing event sourcing or CQRS for their intellectual appeal, then bearing years of versioning, projection, and operational complexity the domain did not require. Adopt only where history, temporal queries, or read/write divergence genuinely justify it.

### Unversioned Events That Break Replay And Projections

Changing event schemas without versioning or upcasting, so old events cannot be processed and every projection and replay breaks. Version events, evolve additively, and provide upcasting.

### Projections That Drift From The Log

A projection that misses events or holds state not derivable from the log, serving wrong reads with no detection. Make projections deterministically rebuildable, reconcile against the log, and detect drift.

### Eventual Consistency Felt As A Bug

A user reading stale data immediately after their own write, because read-your-writes consistency was not provided. Route or serve the write's result for the writer, and design the UX around the lag.

### No Snapshotting For Long Streams

Rebuilding state for an entity with millions of events by full replay, which is impractical. Snapshot periodic state and replay forward, with snapshot invalidation on logic change.

### Events Modeled As Commands Or Messages

Naming events as commands ("PlaceOrder") or treating them as transient messages, rather than as immutable past-tense domain facts. Name events as things that happened and treat them as permanent records.

### A Projection That Becomes A Second Source Of Truth

A projection holding state not derivable from the log, so consistency between the log and the projection is lost. Ensure every projection is deterministically rebuildable from the event log.

## Self-Check

- [ ] Event sourcing and CQRS were adopted because the domain genuinely requires history/temporal queries/audit or has read/write divergence — not for elegance — and the added complexity (versioning, projections, eventual consistency, snapshots) was accounted for upfront.
- [ ] Events are immutable past-tense domain facts, self-contained, forward-compatible (additive evolution), versioned with upcasting for old events, and free of leaked implementation details.
- [ ] Projections are deterministically rebuildable from the log, their logic is versioned and replayable, drift is detected via reconciliation, and each projection's update pace matches its freshness requirement with observable lag.
- [ ] Eventual consistency is accepted and communicated: read lag is observable and bounded, per-read freshness requirements are matched to projection pace, and read-your-writes is provided where a user reads immediately after their own write (with UX designed around the lag).
- [ ] Operational concerns are addressed: long streams are snapshotted (with invalidation on logic change), projection rebuilds are parallelizable/resumable/no-downtime, event retention is defined consistent with audit and rebuild needs, and the event store's health is monitored as a critical database.
- [ ] The highest-risk cases were verified — an event schema change against deployed projections, a drifted projection serving wrong reads, a stale read after write, a multi-million-event rebuild, and a snapshot invalidated by a logic change — not only the clean short-stream demo.
