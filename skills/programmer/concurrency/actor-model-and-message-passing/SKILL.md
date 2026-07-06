---
name: actor_model_and_message_passing.md
description: Use when the agent is designing or evaluating an actor-model or message-passing system, choosing between actors and shared-state concurrency, designing mailbox and message semantics, reasoning about location transparency and distribution, handling failure propagation and supervision, implementing backpressure in message systems, or debugging message-ordering and delivery issues. Covers actor isolation and the no-shared-state principle, mailbox semantics (FIFO, priority, bounded), synchronous vs asynchronous messaging, at-most-once vs at-least-once vs exactly-once delivery, message ordering guarantees, location transparency and its limits, supervision and failure propagation (let-it-crash), backpressure in async message systems, the actor-vs-thread tradeoff, and the discipline of modeling the problem as message flows rather than shared mutable state. Also use when a message system deadlocks or livelocks, when messages are lost or duplicated, when ordering assumptions break under distribution, or when deciding whether the actor model fits a problem.
---

# Actor Model And Message Passing

The actor model is a concurrency model where independent actors, each with private state, communicate exclusively by passing messages, and its central promise is compelling: no shared mutable state means no data races, no locks, and components that fail independently. The promise is real, but it relocates complexity rather than eliminating it. The data races of shared-state concurrency become the message-delivery, ordering, and duplication problems of distributed messaging; the lock deadlocks become message-cycle deadlocks; the shared-state inconsistency becomes the consistency problem across actors that update independently. Teams adopt actors expecting simpler concurrency and discover that message-passing concurrency has its own failure modes — lost messages, duplicated work under redelivery, ordering violations across network boundaries, mailboxes that grow unbounded, supervision strategies that restart an actor and lose its state — and that these are no easier to reason about than the locks they replaced, just different.

Agents tend to adopt the actor model for its appeal (isolation, no locks) without internalizing that message passing over a boundary is a distributed-systems problem even when the actors are local. The defects live in the assumptions about messages: assuming delivery when the model offers at-most-once; assuming ordering that holds only within a single sender-receiver pair and breaks across actors or after a restart; assuming a mailbox is unbounded when it is bounded and silently drops or blocks; assuming a supervisor restart restores consistency when the actor's pre-crash messages may now be lost or duplicated. The judgment problem is treating every message send as a small distributed-systems interaction with explicit delivery, ordering, and failure semantics — and designing the supervision, backpressure, and idempotency that the model requires — rather than treating messages as function calls that always arrive in order.

This skill is about designing actor and message-passing systems correctly. It complements the thread-safety skill (shared-state concurrency) and the async-runtime skill (the event-loop model); here the question is the specific model of isolated actors communicating by messages, and the disciplines that keep it correct.

## Core Rules

### Preserve Actor Isolation; Do Not Leak Shared Mutable State

The actor model's correctness rests on each actor's state being private and accessible only through its message handling. Leaking shared mutable state reintroduces the data races and lock complexity the model was meant to avoid.

- **Keep actor state private.** An actor's internal mutable state is accessed only by the actor itself, in the course of handling messages. Do not hand out references to internal state (which lets another actor mutate it concurrently) or share mutable structures across actors.
- **Pass immutable data or copies in messages.** If a message carried a mutable object and both sender and receiver hold references, they share mutable state — defeating isolation. Pass immutable values, or deep-copy mutable data into the message.
- **Beware "shared nothing" violations through external resources.** Two actors writing to the same file, database row, or global mutable resource are sharing state through the back door. Coordinate access to shared external resources explicitly (e.g., route all access through one actor), or accept that the resource is a shared-state concurrency problem requiring its own synchronization.

### Define Message Delivery Semantics Explicitly

"Send a message" is underspecified. The system must define whether delivery is guaranteed, how many times, and in what order — and these semantics determine what the receiver must handle.

- **At-most-once** (messages may be lost, never duplicated). The default for many local actor systems; cheap, but the receiver must tolerate lost messages or the sender must tolerate no reply. Suitable for ephemeral or best-effort messages; dangerous for anything that must be processed.
- **At-least-once** (messages are never lost, but may be duplicated). Requires acknowledgments and redelivery; the receiver must be idempotent — processing the same message twice must not corrupt state or duplicate side effects. Most production-reliable systems are at-least-once.
- **Exactly-once** (delivered exactly once) is effectively impossible end-to-end without distributed consensus or transactional guarantees; what is called "exactly-once" is usually at-least-once delivery plus at-least-once processing made idempotent so the effect is once. Prefer idempotent at-least-once over chasing true exactly-once.
- **State the semantics and design the receiver for them.** If delivery is at-most-once, the design must tolerate loss; if at-least-once, it must be idempotent. Mismatched assumptions (sender assumes delivery, model drops messages) cause silent failures.

### Understand And State Message Ordering Guarantees

Ordering is the other underspecified property of messaging, and incorrect assumptions cause subtle bugs, especially across network boundaries.

- **Per sender-receiver pair, FIFO is common but not universal.** Many systems guarantee that messages from actor A to actor B arrive in the order A sent them, but this holds only for that pair — messages from C to B are not ordered relative to A's, and messages from A to a different actor D are not ordered relative to A's to B.
- **Ordering often breaks under redelivery and restart.** A redelivered message may arrive out of order; a restarted actor may process messages in a different order than before the crash. Do not assume global or cross-restart ordering unless the system explicitly guarantees it.
- **Ordering across a network is harder.** Local actor systems may offer strong ordering; once messages cross a network (location-transparent actors on different nodes), ordering weakens to per-connection or per-partition, and redelivery can reorder. Design for the weakest ordering the system provides across its actual deployment.
- **Make receivers order-independent where possible.** If the receiver's correctness depends on message order, it is fragile to redelivery and distribution; if it is idempotent and order-independent, it is robust. Prefer designs where order does not matter.

### Design Mailboxes With Bounded Capacity And Backpressure

A mailbox is the queue holding messages awaiting an actor's processing, and an unbounded mailbox under a faster producer than consumer grows until memory exhausts. Mailboxes need backpressure.

- **Use bounded mailboxes.** A bounded mailbox forces the sender to handle a full queue (block, apply backpressure, or fail) rather than buffering infinitely. This propagates slowness from a slow consumer to its producers, preventing unbounded memory growth.
- **Choose a full-mailbox strategy deliberately.** When the mailbox is full: block the sender (backpressure), drop messages (lossy), or fail the send (error). Each has consequences; blocking can deadlock cycles, dropping loses work, failing pushes error handling to the sender.
- **Beware deadlock in bounded mailbox cycles.** If actor A blocks sending to a full B, and B blocks sending to a full A, both deadlock. Detect or avoid cycles; use timeouts on sends; or design the topology to be acyclic.
- **Monitor mailbox depth.** A growing mailbox is the symptom of a slow consumer or a faster producer; it must be visible in metrics so it can be addressed before it exhausts memory.

### Design Supervision And Failure Propagation Deliberately

The actor model's "let it crash" philosophy holds that a failing actor should be restarted by a supervisor rather than patched with defensive code, isolating failures and restoring a known-good state. This is powerful but has specific requirements.

- **Define the supervision strategy.** When a child actor fails, the supervisor decides: restart it (resume processing with a fresh actor), resume it (continue, rare and dangerous if state is corrupt), stop it (give up on the child), or escalate (fail itself, letting its supervisor decide). The strategy must be chosen per child and per failure type.
- **A restart loses the actor's in-memory state.** The restarted actor begins fresh; any state not persisted or recoverable is gone. Design actors to recover state on restart (from persistent storage, from a journal, or by replaying messages) if state must survive, or accept state loss if it is recomputable.
- **A restart may lose or duplicate in-flight messages.** Messages the actor had not yet processed may be lost (at-most-once) or redelivered (at-least-once); the system's delivery semantics determine which, and the actor must handle it (idempotency for redelivery).
- **Beware failure cascades.** If a supervisor restarts a failing child repeatedly, or escalates and fails itself, failures can cascade up the supervision tree and take down large parts of the system. Add restart-rate limits (circuit breakers) so a persistently failing actor does not thrash or cascade.
- **Let-it-crash is for transient faults, not all faults.** Restarting an actor that fails because of a bug or a corrupt input will fail again immediately; supervision handles transient faults (a temporary resource unavailability, a race), not deterministic ones. Do not use restart as a substitute for fixing bugs.

### Treat Location Transparency With Skepticism

Many actor systems offer "location transparency" — the same code whether actors are local or remote — and the appeal is obvious. The danger is that local and remote messaging have fundamentally different failure modes, and code that works locally fails remotely.

- **Local messaging is fast, ordered, and reliable; remote messaging is none of these.** A local message send is a method call; a remote one crosses a network with partial failure, partition, reordering, and redelivery. Code assuming local semantics breaks remotely.
- **Partial failure is the defining difference.** Locally, an actor is either up or the whole process is down; remotely, an actor may be unreachable (partition), slow (load), or crashed (node failure), and the sender cannot always tell which. The system must handle timeouts and uncertainty.
- **Do not assume location transparency makes distribution free.** Designing for distribution requires idempotency, timeout handling, and ordering tolerance regardless of the abstraction. Treat "same code local and remote" as a convenience, not a guarantee that remote works like local.

### Model The Problem As Message Flows, Not As Shared State

The actor model rewards thinking in terms of message flows and responsibilities rather than shared mutable state, and forcing a shared-state design into actors produces awkward, failure-prone systems.

- **Assign each responsibility to one actor.** A piece of state or a resource has one owner actor that mediates all access; other actors request changes by message. This is the no-shared-state principle applied to design.
- **Model interactions as request/response or event flows, not as direct manipulation.** Instead of actor A mutating actor B's state, A sends B a message requesting the change; B applies it and optionally replies. This serializes access and makes the interaction explicit.
- **Prefer coarse-grained messages over fine-grained chatty ones.** Each message has overhead; a design where actors exchange many tiny messages in a tight loop performs poorly and is hard to follow. Batch related work into coarser messages.
- **Recognize when actors are the wrong model.** Some problems are naturally shared-state (a numerical simulation over a large array, a graph algorithm) and are awkward as actors; for these, shared-state concurrency (threads, data parallelism) is simpler. Use actors where the problem decomposes into independent components communicating by messages.

## Common Traps

### Leaking Shared Mutable State Through Messages

Passing a mutable object in a message so sender and receiver share references, defeating actor isolation and reintroducing data races. Pass immutable values or copies.

### Assuming Delivery When The Model Is At-Most-Once

Treating message send as a guaranteed function call when the system may drop messages, causing silent failures. State the delivery semantics; design the receiver to tolerate loss (at-most-once) or duplication (at-least-once with idempotency).

### Assuming Ordering That Breaks Under Distribution Or Restart

Relying on global or cross-restart message ordering that holds locally but breaks remotely or after a crash. Understand the actual ordering guarantee; make receivers order-independent where possible.

### Unbounded Mailbox Under Faster Producer

An unbounded mailbox growing until memory exhausts because the producer is faster than the consumer. Use bounded mailboxes with an explicit full-queue strategy and backpressure.

### Bounded Mailbox Cycle Deadlock

Actors blocking on each other's full mailboxes in a cycle, deadlocking the system. Detect or avoid cycles; use send timeouts; design acyclic topologies.

### Restart Losing State Or In-Flight Messages

A supervisor restart that loses the actor's in-memory state or its unprocessed messages, breaking consistency. Design actors to recover state on restart and to handle redelivery (idempotency).

### Failure Cascade From Repeated Restarts

A persistently failing actor restarted without limit, thrashing or cascading failure up the supervision tree. Add restart-rate limits (circuit breakers); recognize that restart handles transient, not deterministic, faults.

### Assuming Location Transparency Makes Distribution Free and forcing A Shared-State Problem Into Actors

Writing code that works locally and assuming it works the same remotely, when remote messaging has partial failure, partition, reordering, and redelivery. Design for distribution explicitly; treat location transparency as a convenience, not a guarantee.

Modeling a naturally shared-state problem (large-array numerical work, graph algorithms) as actors, producing awkward chatty message flows. Use actors where the problem decomposes into independent message-communicating components; use shared-state concurrency where the problem is naturally shared.

## Self-Check

- [ ] Actor isolation is preserved: state is private, messages carry immutable data or copies, and no mutable references leak across actors; shared external resources are mediated by a single owner actor or explicitly synchronized.
- [ ] Message delivery semantics (at-most-once, at-least-once, or idempotent-effectively-once) are explicitly stated, and the receiver is designed for them — tolerating loss for at-most-once, or idempotent for at-least-once.
- [ ] Message ordering guarantees are understood (per sender-receiver pair, not global; weakened under redelivery, restart, and distribution) and the receiver is order-independent or robust to the weakest ordering the system provides.
- [ ] Mailboxes are bounded with an explicit full-queue strategy (block/drop/fail) that provides backpressure, mailbox depth is monitored, and cycles that could deadlock under bounded mailboxes are detected, avoided, or guarded with timeouts.
- [ ] The supervision strategy is defined per child and failure type, restart-rate limits (circuit breakers) prevent thrashing or cascades, actors recover state on restart where state must survive, and let-it-crash is applied to transient faults (not deterministic bugs).
- [ ] Location transparency is treated with skepticism: code is designed for distribution (idempotency, timeout handling, ordering tolerance, partial-failure uncertainty) regardless of the abstraction, not assumed to work remotely because it works locally.
- [ ] The problem is modeled as message flows with single-owner responsibilities and coarse-grained messages, not as shared mutable state forced into actors; actors are chosen because the problem decomposes into independent message-communicating components.
- [ ] The actor model is chosen over shared-state concurrency for the right reasons (independent components, fault isolation, natural message flow), not as a default; the team understands that message-passing complexity (delivery, ordering, backpressure, supervision) replaces lock complexity rather than eliminating it.
