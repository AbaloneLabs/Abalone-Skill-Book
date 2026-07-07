---
name: scala_actor_systems_and_akka.md
description: Use when the agent is designing or debugging an actor-based or Akka/Pekko system (actors, actor refs, message protocols, supervision strategy, location transparency, dispatchers, mailboxes, sharding, Akka Streams, Akka Typed vs Classic), reasoning about message delivery semantics (at-most-once), backpressure with streams, actor lifecycle (preStart/postStop), distributing actors with sharding, or is diagnosing "messages lost / out of order", "actor blocks the dispatcher", "dead letters", "supervision restart loses state", "backpressure overflow", or sharding distribution problems. Covers the actor model and message passing, supervision and lifecycle, dispatchers and blocking, mailboxes and ordering, Akka Streams backpressure, Akka Typed message protocols, sharding, and the pitfalls of shared mutable state, blocking, and lost messages.
---

# Actor Systems And Akka In Scala

The actor model isolates state behind message passing: each actor processes one message at a time, so its internal state is concurrency-safe without locks. Akka (and its successor Pekko) implements this with `ActorRef`s (location-transparent references), mailboxes (queued messages), dispatchers (thread pools), and supervision (parent restarts failing children). This model is powerful for distributed, fault-tolerant systems but has sharp edges. Agents block inside an actor (a blocking DB call starves the dispatcher's threads), lose messages (at-most-once delivery; a crash before processing drops in-flight messages), misunderstand supervision (a `Restart` resets the actor's state but the mailbox survives, while `Stop` drops pending messages), misuse Akka Streams backpressure (a buffer overflow strategy of `Fail` vs `DropHead`), or build untyped message protocols that the compiler cannot check. The judgment problem is to keep actors non-blocking, to design supervision and lifecycle deliberately, to use the right dispatcher for blocking work, to apply backpressure in streams, and (with Akka Typed) to encode message protocols in the type system.

Agents block in actors, assume reliable delivery, restart-and-lose-state, or ignore backpressure. The remedy is non-blocking actors, explicit supervision, dedicated dispatchers for blocking, and streams with backpressure.

## Core Rules

### Keep Actors Non-Blocking; Use A Dedicated Dispatcher For Blocking Work

An actor processes messages one at a time on a dispatcher thread; if it blocks (a synchronous JDBC call, `Await.result`, a long computation), that thread is held and the dispatcher's throughput collapses (a default dispatcher with few threads stalls). Rules:

- Never block inside an actor. For blocking I/O, use a dedicated `dispatchers` with a `thread-pool-executor` of appropriate size (e.g., `my-blocking-dispatcher { type = Dispatcher; executor = "thread-pool-executor"; fixed-pool-size = 50 }`), or wrap the blocking call in `Future` on such a dispatcher, or use a non-blocking client (async DB/HTTP).
- For CPU-bound actors, a small `fork-join`/default dispatcher suffices; tune `throughput` for fairness.
- `Await.result`/`Await.ready` inside an actor is almost always wrong — it blocks the dispatcher.

- No blocking inside actors; offload blocking I/O to a dedicated thread-pool dispatcher or use async clients.
- `Await` inside an actor starves the dispatcher; avoid.

### Understand Message Delivery Semantics (At-Most-Once, No Guaranteed Order Across Actors)

Akka provides at-most-once delivery: a message is delivered zero or one times, never duplicated; if the sending or receiving actor crashes, in-flight messages are lost. There is no guaranteed delivery — confirmations/acknowledgments and the Akka Persistence event-sourcing model exist for that. Within a single sender→receiver pair, messages are delivered in order (per the mailbox FIFO); across actors or after a restart, ordering is not guaranteed. Do not assume a message arrived; design for idempotent handlers and, where needed, use `Ask` (request-response with timeout) or persistence/journaling for durability. "Dead letters" in the log indicate messages sent to an actor that does not exist (stopped, never started, or wrong path) — investigate them.

- At-most-once delivery; messages can be lost on crash. Design idempotent handlers; use persistence for durability.
- In-order only within a single sender→receiver pair; no global ordering across actors.
- "Dead letters" = messages to nonexistent/stopped actors; investigate.

### Design Supervision And Lifecycle Deliberately

A failing actor is handled by its parent's supervision strategy: `Resume` (keep going, keep state and mailbox), `Restart` (recreate the actor, reset state via `preRestart`/`postStop`, keep mailbox), or `Stop` (terminate, drop pending messages). Choose deliberately:

- `Restart` for transient errors (reset to a clean state); the mailbox survives so pending messages are processed by the new instance.
- `Stop` for unrecoverable errors, but pending messages are lost.
- `Resume` rarely (state may be corrupt).
- One-for-one vs all-for-one: one-for-one restarts only the failing child; all-for-one restarts siblings too (for tightly-coupled children).
- Lifecycle hooks (`preStart`, `postStop`, `preRestart`, `postRestart`) for resource setup/teardown; do heavy init in `preStart`, release in `postStop`.

- Supervision: `Restart` (reset state, keep mailbox), `Stop` (drop pending), `Resume` (rare). Choose by failure mode.
- One-for-one vs all-for-one by child coupling; use lifecycle hooks for resource management.

### Use Akka Streams For Backpressure On Pipelines

When data flows through stages (source → flow → sink), use Akka Streams rather than raw actor messaging, because streams provide backpressure: a slow downstream signals upstream to slow down, preventing unbounded buffering/OOM. Characterize each stage's buffer and overflow strategy (`buffer(n, OverflowStrategy.fail/dropHead/dropTail/backpressure)`); `backpressure` is the default and safest. Avoid `Source.tick`/`Source.unfold` that emit faster than the downstream can consume without backpressure. For integration with non-streams APIs, use `Source.queue`, `Sink.queue`, or the reactive-streams interop. Streams also give graph composition, fusion optimization, and materialization semantics that raw actors lack.

- Use Akka Streams (not raw actors) for pipelines; backpressure prevents OOM.
- Choose buffer/overflow strategy deliberately (`backpressure` default; `fail`/`dropHead` for bounded).
- Use reactive-streams/queue interop for non-streams boundaries.

### Encode Message Protocols With Akka Typed

Akka (Typed) makes each actor's accepted messages a type (`Behavior[Command]`), so the compiler checks that you send only valid messages to an actor — eliminating the "wrong message shape" runtime errors of Classic untyped actors. Define a sealed protocol (`sealed trait Command; case class DoX(replyTo: ActorRef[Result]) extends Command`) and route in a `receive`/`Behaviors.setup`. Use the `ask` pattern with typed `ActorRef[Reply]` for request-response. Prefer Typed for new code; Classic remains for legacy and some libraries. The typed model also enforces supervision via `Behaviors.supervise` and encourages a clear protocol boundary.

- Use Akka Typed for new code; the `Behavior[Command]` type checks message protocols.
- Define sealed protocols with reply channels (`replyTo: ActorRef[Reply]`); use typed `ask`.
- Typed supervision via `Behaviors.supervise`; clear protocol boundary.

### Use Sharding For Distributed Actors

For horizontally-scaled actors (millions of entities across nodes), use Akka Cluster Sharding: entities are addressed by a stable `EntityId` and automatically placed/relocated across the cluster, with passivation to reclaim memory. Design entities to be addressable by id (a user id, an order id), keep per-entity state in memory (persisted via Akka Persistence if needed), and route messages via `ShardRegion`. Beware: sharding requires a cluster (seed nodes, split-brain resolver), and an entity's state is on one node at a time (no replication). Plan for cluster membership, downing, and shard rebalancing deliberately.

- Cluster Sharding for millions of entities across nodes, addressed by `EntityId`.
- Per-entity state in memory (+ persistence for durability); one node per entity at a time.
- Plan cluster membership, downing (split-brain resolver), and rebalancing.

## Common Traps

### Blocking Inside An Actor

A synchronous DB/HTTP call or `Await` starves the dispatcher. Use a dedicated thread-pool dispatcher or async clients.

### Assuming Reliable Delivery

At-most-once: messages can be lost on crash. Use persistence/acks for durability; idempotent handlers.

### Restart Losing State Unexpectedly

`Restart` resets state (via `preRestart`) but keeps the mailbox; `Stop` drops pending messages. Choose deliberately.

### No Backpressure On A Pipeline

Raw actor messaging can overflow mailboxes/OOM. Use Akka Streams with backpressure.

### Dead Letters Ignored

Messages to stopped/nonexistent actors. Investigate the path and lifecycle.

### Untyped Protocols With Runtime Message Errors

Classic actors accept `Any`; wrong message shapes fail at runtime. Use Akka Typed for compile-time checks.

### Shared Mutable State Across Actors

Actors must not share mutable state (that defeats the model). Communicate only via messages.

### Wrong Dispatcher For The Workload

CPU-bound actors on a huge thread pool, or blocking I/O on a small fork-join pool. Match dispatcher to workload.

## Self-Check

- [ ] Actors contain no blocking calls (no synchronous I/O, no `Await`); blocking work uses a dedicated thread-pool dispatcher or async clients.
- [ ] Message handlers are designed for at-most-once delivery (idempotent, with persistence/acks where durability matters); dead letters are investigated, not ignored.
- [ ] Supervision strategy (`Resume`/`Restart`/`Stop`, one-for-one/all-for-one) is chosen deliberately by failure mode, with lifecycle hooks managing resources.
- [ ] Data pipelines use Akka Streams with backpressure (deliberate buffer/overflow strategy), not raw actor messaging that can overflow.
- [ ] New actors use Akka Typed with sealed protocols and typed `ActorRef[Reply]` (compile-time message checks); Classic is reserved for legacy/library interop.
- [ ] Distributed actors use Cluster Sharding by `EntityId` with persistence for durability, and cluster membership/downing/rebalancing are planned.
- [ ] No shared mutable state across actors (communication is via messages only); dispatchers match the workload (CPU-bound vs blocking I/O).
- [ ] The system has been considered under crash/restart, message loss, backpressure, and cluster-split scenarios, and remains correct.
