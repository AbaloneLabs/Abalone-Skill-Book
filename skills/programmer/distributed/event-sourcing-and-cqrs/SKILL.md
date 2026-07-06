---
name: event_sourcing_and_cqrs.md
description: Use when the agent is designing or reviewing an event-sourced system or a CQRS architecture, modeling domain state as an append-only event stream, rebuilding aggregate state by replaying events, adding snapshots to bound replay cost, building read models or projections from an event log, separating command and query sides with different schemas, versioning and evolving immutable event schemas with upcasters, replaying and re-projecting events idempotently, or handling read-after-write expectations across an eventually consistent read side. Covers the ES and CQRS pattern mechanics and their operational complexity tax, distinct from consistency-model theory and from general messaging.
---

# Event Sourcing And CQRS

Event sourcing and CQRS are two distinct but frequently paired patterns. Event sourcing makes the immutable sequence of domain events the source of truth and reconstructs current state by replaying them; CQRS separates the model used to validate and apply writes (commands) from the models used to answer reads (queries). Both are powerful, and both carry a steep operational and conceptual complexity tax that is routinely underestimated. The judgment problem is not "how do I implement event sourcing" but "is the auditability, temporal query power, or write/read asymmetry genuinely worth the cost of immutable history, schema evolution, projection management, and eventual consistency between write and read — and if so, how do I avoid the failure modes that make these systems expensive to run."

Agents tend to under-invest here because the patterns read cleanly in a tutorial: append events, replay them, project to a read model, done. The harm appears in the parts the tutorial omits — old events whose schema you can no longer change, projections that drift from the event log and must be rebuilt, read models that lag the write side and confuse users who expect read-after-write, unbounded replay times as streams grow, and the operational burden of running an event store and a fleet of projectors as first-class infrastructure. For consistency-model theory see distributed-consistency-and-cap; for the messaging plumbing see event-driven-architecture. This skill is the ES and CQRS pattern itself.

## Core Rules

### Decide Whether Event Sourcing Earns Its Complexity

Event sourcing is not a default architecture; it is a deliberate choice justified by a specific need. Adopt it when one or more of these is genuinely true: you have an audit or regulatory requirement to retain every state change as an immutable record; you need temporal queries ("what was the balance at this date," "reconstruct state as of this event"); your domain has complex state transitions where having the full history materially simplifies the model; or you need to feed multiple derived views from a single authoritative log. A strong justification names the specific capability that event sourcing unlocks and acknowledges the cost.

The weak choice is adopting event sourcing because it is "modern" or because someone read a DDD book. For simple CRUD — create, read, update, delete over a record with no audit or temporal need — event sourcing is needless complexity: you gain an append-only log, schema evolution headaches, and projection lag in exchange for nothing the business asked for. Before committing, ask what concrete question the business needs to answer that only an event log can answer, and whether a conventional store with an audit table would satisfy 80% of the need at 20% of the cost. If the honest answer is "nothing specific," do not adopt event sourcing.

### Treat The Event Store As Specialized Infrastructure With Hard Requirements

The event store is not a generic database used in a particular way; it has requirements a normal store does not meet well. It must be strictly append-only — events are immutable history and are never edited or deleted (legal/privacy exceptions aside, which need explicit handling). It must provide strong ordering within a stream so that replay produces deterministic state, and it must offer optimistic concurrency on a per-stream basis so concurrent appends to the same aggregate are detected and serialized. It must be durable and highly available, because it is the single source of truth — losing it loses the business state irrecoverably.

A weak design uses a relational table as an event store without enforcing append-only semantics or stream-level concurrency, then discovers that updates or out-of-order appends corrupt replay. A strong design either uses a purpose-built event store or carefully enforces append-only and optimistic concurrency (a stream version column, compare-and-set on append) on top of a general database. Name the stream naming scheme explicitly — typically one stream per aggregate instance, e.g. `order-12345` — because stream identity governs concurrency boundaries and replay scope, and getting it wrong causes contention or split state.

### Bound Replay Cost With Snapshots, And Design Rebuild As A First-Class Operation

Rebuilding aggregate state by replaying every event in a stream works for short streams and breaks for long-lived ones. A bank account or a long-running process can accumulate thousands of events, and loading current state by replaying all of them on every command becomes slow. The standard remedy is snapshots: periodically persist a materialized state plus the event version it was computed from, and on load, take the latest snapshot and replay only the events after it. Snapshots are an optimization, not a correctness mechanism — the event log remains the source of truth, and a corrupt snapshot must be rebuildable from the log.

Treat projection and aggregate rebuild as first-class operations that must be repeatable and fast, because you will rebuild them: when a projection drifts, when you add a new read model, when you fix a projector bug, when you migrate. A weak design assumes rebuild is a rare disaster and has no tooling for it; a strong design has tested rebuild pipelines, can rebuild a projection from the log within an acceptable time window, and can do so without downtime. If a full replay is too slow to ever run, you have an operational time bomb — design for bounded replay via snapshots and partitioned replay from the start.

### Design Projections And Read Models For The Read, Not The Write

A projection is a function from the event log to a read-optimized view — a denormalized table, a search index, a materialized view tuned for the queries it serves. The whole point is that the read model's shape is driven by query needs, unconstrained by the write model's aggregate boundaries. Different read models can join across aggregates, precompute summaries, flatten hierarchies, and shape data exactly as the UI or API consumes it.

The key property to design for is that the read side is **eventually consistent with the write side**. A command appends an event; the projector reads the event asynchronously and updates the read model; the read model lags by some amount. This lag is inherent, not a bug, and you must decide how to handle it. A weak design exposes the lag raw and confuses users ("I saved it but the list doesn't show it"). A strong design either accepts bounded staleness where the business tolerates it, routes read-after-write through the write side or a session-aware path, or uses a read model version token so the client can wait until its write is reflected. Never assume the read model is current the instant a command succeeds.

### Separate CQRS Deliberately, And Know When The Asymmetry Pays Off

CQRS uses one model to handle commands (validation, state transition, event emission) and one or more different models to handle queries. The models can have different schemas, different storage, even different databases. The asymmetry pays off when reads and writes genuinely differ — very high read volume with low write volume (or vice versa), reads that span many aggregates, or read shapes that are expensive to serve from a normalized write model. It also composes naturally with event sourcing, where the write side is the event store and the read sides are projections.

The cost is real and often hidden: two models to keep coherent, a projection pipeline to operate, eventual consistency between them, and a more complex mental model for the team. A weak CQRS adoption splits a simple CRUD app into command and query sides for no reason and doubles the code and operational surface. A strong adoption identifies the specific asymmetry (read-heavy workload, complex read shapes, scaling read independently of write) that justifies the split. If your read and write loads are similar and your queries are simple, CQRS is overhead without benefit — use a single model.

### Solve The Immutable-History Schema Evolution Problem Before You Have History

Events are immutable, but your understanding of the domain evolves. An event schema written today will be wrong in a year, and you cannot edit the millions of events already in the log — that would violate immutability and break audit. This is the defining long-term cost of event sourcing, and you must have a strategy from day one.

The standard approach is **upcasters**: versioned events (e.g. `OrderPlaced_v1`, `OrderPlaced_v2`, or a schema-version field) plus functions that transform old events to the current shape as they are read from the store, before replay or projection. Upcasters are pure functions applied at read time, so old events stay immutable in storage but are presented in the current schema to the application. A weak design has no versioning and reaches a point where a domain change requires either editing history (forbidden) or a painful migration. A strong design versions every event type from the first one, keeps upcasters small and composable (chain v1→v2→v3), and tests replay through the full upcaster chain. Also avoid storing data in events that you cannot reinterpret later — prefer stable, business-meaningful fields over implementation details that will churn.

### Make Projectors Idempotent And Replayable From A Checkpoint

Projectors read events from the log and update read models, and they will fail, restart, and reprocess events. A projector must therefore be **idempotent**: reprocessing the same event must not corrupt the read model (no double-counting, no duplicate rows). Achieve this by keying read-model updates on the event's stable identifier and position, so a replayed event is recognized and skipped or applied harmlessly. Projectors must also track their position (checkpoint) durably, so a restart resumes from the right place rather than reprocessing the whole log or skipping events.

A weak projector reprocesses from the beginning on every restart or double-applies events on redelivery, corrupting aggregates like counters and balances. A strong projector checkpoints per stream or globally, handles out-of-order and redelivered events safely, and can be reset and replayed from any point to rebuild a read model. Design the projector's updates to be commutative where possible, and never let a projector assume exactly-once delivery — the messaging layer is at-least-once, and idempotency is the projector's responsibility.

### Handle Read-After-Write Expectations Explicitly

Because the read model lags the write side, the classic user-facing defect is "I submitted the form and the next page doesn't show my change." Decide per flow how to handle this. Options include routing the immediately-following read to the write side, using a session token or correlation id to wait until the projector has caught up to the client's last write, accepting bounded staleness with a clear UI signal, or projecting the optimistic result client-side until confirmed. The wrong choice is to ignore the lag and let users conclude their action failed and retry it, creating duplicates.

### Choose Event Granularity Deliberately — It Shapes Everything Downstream and budget For The Operational Complexity Tax Honestly

Event granularity — how much business meaning a single event carries — is an early, hard-to-reverse decision that shapes replay cost, projection simplicity, and audit usefulness. Two failure modes bracket the spectrum. **Too fine-grained** events (property-level deltas: `OrderShippingAddressChanged`, `OrderBillingAddressChanged`) multiply event counts, bloat the log, make replay slow, and force every consumer to reassemble business meaning from fragments. **Too coarse-grained** events (a single `OrderModified` carrying the entire aggregate state) lose the temporal and audit value that justified event sourcing in the first place — you can no longer answer "what specifically changed and why" from the log.

The strong choice models events at the level of a meaningful business occurrence — a fact that domain experts recognize (`OrderPlaced`, `OrderShipped`, `PaymentCaptured`). Each event should represent one thing that happened, carry the data needed to understand and apply it (including the intent behind the change where audit matters), and be named in the past tense as an immutable fact. Avoid "crud-events" that mirror database column updates, and avoid god-events that bundle unrelated changes. Granularity also affects idempotency and ordering: a coarse event is easier to make idempotent but harder to evolve; a fine event is more flexible but produces more volume. Decide the grain by what the business needs to query and audit, not by what is easiest to emit.

ES and CQRS are powerful but expensive to run. The costs include operating an event store as critical infrastructure, running and monitoring a fleet of projectors, handling projection drift and rebuilds, managing event schema evolution and upcasters over years, debugging issues that require correlating across the log and multiple read models, and onboarding team members to a more demanding mental model. Before adopting, name these costs and confirm the team can absorb them. A system that delivers audit and temporal power but is too complex to operate reliably is a net loss.

## Common Traps

### Adopting Event Sourcing For Simple CRUD

Using an event log where a conventional table with an audit column would do, gaining schema evolution and projection complexity for no business benefit. Adopt event sourcing only when audit, temporal query, or complex state transitions genuinely justify it.

### Editing Or Deleting Old Events To "Fix" A Schema

Mutating immutable history when a domain model changes, breaking audit guarantees and replay determinism. Use versioned events and upcasters applied at read time; never edit stored events.

### Unbounded Replay Without Snapshots

Letting streams grow without snapshots until loading an aggregate by full replay takes seconds, making every command slow and rebuilds impractical. Add snapshots early and treat rebuild performance as a continuous design constraint.

### Assuming The Read Model Is Current Right After A Command

Returning a read from a lagging projection immediately after a write, so users see stale state and assume their action failed. Handle read-after-write explicitly via session-aware routing, version tokens, or accepted-and-signaled staleness.

### Non-Idempotent Projectors That Double-Apply On Redelivery and splitting A Simple App Into CQRS For No Reason

Projectors that increment counters or insert rows without deduplication, so a restarted or redelivered event corrupts the read model. Key updates on event identity and position, and design projectors for at-least-once delivery.

Introducing separate command and query models on a workload with similar read/write loads and simple queries, doubling code and operational surface for no gain. Use CQRS only where read/write asymmetry or complex reads justify it.

### Losing The Event Store Because It Was Treated As Ordinary Data and no Replay Or Rebuild Tooling Until A Projection Drifts

Failing to treat the event store as the irreplaceable source of truth — inadequate backups, no HA, or a schema that allows destructive updates. The event store is the business state; protect and operate it accordingly.

Discovering at 3am that a projector bug corrupted a read model and there is no tested way to rebuild it from the log. Build and exercise rebuild pipelines before you need them.

### Letting Projections Drift Silently With No Monitoring and choosing Event Granularity By What Is Easiest To Emit

Running projectors without lag monitoring, so a stalled projector makes the whole system stale with no alert. Monitor projection lag and event-store append-to-projection delay as first-class metrics.

Modeling events as property-level deltas (mirroring database column updates) or as god-events bundling the whole aggregate, both of which undermine the audit and temporal value that justified event sourcing. Model events at the level of a business-meaningful occurrence that domain experts recognize.

### Storing Implementation Details In Events That Will Churn and using A Generic Database As An Event Store Without Append-Only Enforcement

Persisting internal identifiers, enum ordinals, or framework-specific structure inside events, so that a refactor or library change makes old events unreadable without a migration. Persist stable, business-meaningful fields and reinterpret them with upcasters if the representation must change.

Storing events in a relational table without enforcing append-only semantics or per-stream optimistic concurrency, then discovering that updates or out-of-order appends corrupt replay. The event store has specialized requirements — append-only, stream-level ordering, optimistic concurrency — that a naive table does not meet.

## Self-Check

- [ ] Event sourcing was adopted for a concrete, named capability (audit, temporal query, complex state transitions, multi-view derivation), not as a default — and a conventional store with an audit table was considered as the lower-complexity alternative.
- [ ] The event store enforces append-only semantics, per-stream strong ordering, and optimistic concurrency on append; stream naming is explicit and matches aggregate boundaries.
- [ ] Replay cost is bounded by snapshots, and aggregate/projection rebuild is a tested, repeatable, fast operation that can run without downtime — not an untested disaster procedure.
- [ ] Each read model is shaped for its queries (denormalized, joined, flattened as needed), and the eventual consistency between write and read sides is acknowledged and handled per flow.
- [ ] Read-after-write expectations are handled explicitly (session-aware routing, version tokens, accepted-and-signaled staleness) — no flow silently returns stale data right after a write.
- [ ] Every event type is versioned from day one, upcasters transform old events to the current schema at read time as pure composable functions, and replay through the full upcaster chain is tested — old events are never edited in storage.
- [ ] All projectors are idempotent (keyed on event identity/position, safe under redelivery), checkpoint their position durably, and can be reset and replayed from any point to rebuild a read model.
- [ ] CQRS was adopted only where read/write asymmetry or complex read shapes justify it, not applied to a simple symmetric workload that a single model would serve.
- [ ] The operational complexity tax is budgeted honestly — event store HA/backups, projector fleet monitoring with lag alerts, schema evolution over years, rebuild tooling, and team onboarding are all accounted for.
- [ ] Event granularity is chosen at the level of a business-meaningful occurrence, not as property-level deltas or god-events, and events carry stable business-meaningful fields rather than implementation details that will churn; [ ] Projection lag and event-store-to-read-model delay are monitored as first-class metrics with alerts, so a stalled projector cannot make the system silently stale
