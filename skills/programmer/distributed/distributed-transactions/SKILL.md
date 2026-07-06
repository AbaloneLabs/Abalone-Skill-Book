---
name: distributed_transactions.md
description: Use when the agent is implementing or reviewing a cross-service or cross-node transaction, choosing between 2PC and 3PC, designing a saga (choreography or orchestration) and its compensating actions, adopting the TCC (Try-Confirm-Cancel) pattern, building a transactional outbox for reliable event publishing, introducing a distributed lock with Redis or etcd or ZooKeeper, reasoning about coordinator failure and recovery, or deciding whether a distributed transaction is even warranted versus local transactions plus eventual coordination. Covers the deep implementation craft, not the consistency-model overview.
---

# Distributed Transactions

A distributed transaction is any operation that must keep state consistent across more than one node, database, or service even though no single local transaction can span them all. The judgment problem is not "how do I make this atomic" — it is "what guarantee do I actually need, what partial-failure and concurrency anomalies am I willing to tolerate, and which pattern delivers that guarantee at a cost I can operate." Every pattern here is a different point on the tradeoff curve between atomicity, isolation, latency, availability, and operational complexity, and the common failure is to pick the pattern that sounds strongest rather than the one that matches the real requirement.

Agents tend to under-invest in this because the happy path is easy: both writes succeed, both commits happen, the message is delivered. The harm appears only under partial failure — a coordinator crash, a network blip during commit, a message redelivery, a lock that expired while a client still held it — and it is severe: double-spending, orphaned reservations, permanently stuck resources, duplicate side effects like charging a customer twice or sending an order twice, and outages caused by a blocking coordinator holding locks across the whole system. For the consistency-model theory and when each pattern is appropriate at a high level, see distributed-consistency-and-cap; this skill is the implementation craft of each pattern.

## Core Rules

### Decide Whether A Distributed Transaction Is Warranted At All

Before reaching for any cross-node transaction, ask whether the invariant can be preserved with a single local transaction plus asynchronous coordination. The strongest rule in this area is: prefer local transactions plus eventual coordination (outbox, saga, reconciliation) over synchronous distributed transactions, and reserve true atomicity across nodes for cases where no other approach preserves the invariant.

A weak choice is "we have two services that must agree, so we need a distributed transaction." A strong choice first asks: can one service own the write and notify the other asynchronously through an outbox? Can the second operation be made idempotent and retried? Can the cross-service invariant be relaxed to eventual consistency with a reconciliation job? Only when the answer to all of these is genuinely no — typically money movement, inventory decrement, or seat booking where the invariant must hold before the user proceeds — should a synchronous distributed transaction or a tightly-coupled saga be used. Document the invariant that forced the choice, because the operational cost is justified only by that invariant.

### Understand 2PC Mechanics And The Blocking Coordinator Problem

Two-phase commit gives atomicity across participants via a coordinator. Phase one (prepare): the coordinator asks each participant to prepare to commit; each participant writes the work to a durable log and locks the resources, then votes yes or no. Phase two (commit or abort): if all voted yes, the coordinator writes a commit decision and tells participants to commit; if any voted no, it tells all to abort. Participants act on the decision and acknowledge.

The defining weakness is the **blocking coordinator failure**: if the coordinator crashes after some participants have voted yes but before the decision is recorded, those participants are holding locks and cannot proceed unilaterally — they do not know whether the decision was commit or abort, and acting either way could violate atomicity. They block until the coordinator recovers, which can be indefinitely. This is why 2PC is suitable only within a tightly controlled cluster with a highly available coordinator, not across independent services with separate failure domains. If you adopt 2PC, you must plan for coordinator recovery, participant timeout behavior, and the locked-resource blast radius during a coordinator outage. Treat 2PC as a pattern that is correct on paper but operationally fragile, and prefer it only when the participants share an administrative boundary and the coordinator can be made highly available.

### Understand 3PC, Its Goal, And Why Its Assumptions Often Fail

Three-phase commit (CanCommit, PreCommit, DoCommit) attempts to remove the blocking failure by adding a pre-commit phase so that a participant can make forward progress after a coordinator failure using bounded timeouts. The intent is that no participant is left in an indeterminate state where it must block forever.

The catch is that 3PC's correctness relies on assumptions that real networks violate — specifically synchronous bounded-delay networking and reliable failure detection. Under a network partition where a participant cannot tell whether the coordinator is crashed or merely unreachable, 3PC can still produce inconsistent outcomes (some participants commit while others abort). In practice, 3PC is rarely used in production because the failure modes it cannot handle are exactly the ones that occur (partitions, pause, clock issues), and the extra round trip adds latency. Know that it exists and what it tries to solve, but do not assume it is a safe upgrade from 2PC; the real fix for coordinator blocking is consensus-based coordination or avoiding synchronous distributed transactions entirely.

### Design Sagas Deliberately, Choreography Versus Orchestration

A saga is a sequence of local transactions where each step has a compensating action executed if a later step fails. It gives up ACID isolation in exchange for avoiding distributed locks: intermediate states are visible, and the overall outcome is eventual consistency achieved through compensation rather than atomic commit. Two coordination styles exist, and the choice is a real design decision.

**Choreography**: each service emits an event when it completes its step, and the next service reacts. No central coordinator. This works for short, stable sagas with few participants, but degrades quickly as the number of steps grows — the flow becomes implicit, the failure paths are spread across services, and adding a step requires understanding the whole event chain. A weak choreography design has services silently compensating on their own with no visibility into the overall saga state.

**Orchestration**: a dedicated orchestrator service drives the saga, calls each step, and decides compensations on failure. The flow is explicit and observable, retry and compensation logic is centralized, and the saga state is queryable. The cost is that the orchestrator is now a critical component that must itself be reliable and stateful, and it can become a coupling point. Prefer orchestration for sagas with more than two or three steps, with complex branching, or where operational observability of the workflow matters. A strong design makes the orchestrator's state machine durable and recoverable so a crash mid-saga resumes correctly rather than restarting or abandoning.

### Design Compensating Actions As First-Class Logic, Not Undo

The most common and dangerous saga error is treating compensation as a simple reverse of the forward step. A good compensating action is semantically meaningful: it brings the business back to a consistent state, which is often not the same as the original state. Cancelling a reservation releases the held seat; refunding a charge returns money but does not erase the audit record; cancelling a shipment stops fulfillment but the order history must show the attempt. Compensation is almost always not the literal inverse, and designing it as if it were loses information and breaks audit trails.

For each step, before implementing the saga, answer: is this step compensable at all? Some steps are effectively non-compensable once completed — an email sent, an SMS delivered, a payment pushed to an external rail, a physical item shipped. For non-compensable steps, the saga design must either place them last (so they only execute once everything compensable has succeeded), make them idempotent and retry-safe, or accept that the saga has a point of no return after which failure requires manual intervention. A saga that puts an irreversible side effect in the middle and then fails afterward cannot be automatically compensated, and pretending otherwise is the classic saga defect. Also design compensation to be idempotent and retriable, because compensation messages can be redelivered just like any other message.

### Reason Explicitly About Saga Isolation Anomalies

Because sagas do not hold locks across steps, concurrent sagas can observe each other's intermediate (uncommitted) states. This produces the classic isolation anomalies, and you must decide for each how to handle them:

- **Dirty reads**: a saga reads a value written by another saga that has not finished and may be compensated away. Mitigate with semantic locks (mark records as in-progress so other transactions treat them specially), pessimistic version checks, or by accepting the read and re-validating before committing.
- **Lost updates**: two sagas read the same value and both write based on the stale read, dropping one update. Mitigate with optimistic concurrency control (version numbers, compare-and-set) at each step.
- **Phantom reads**: a saga's decision depends on the existence or count of rows that another saga adds or removes mid-flight. Mitigate with commutative updates (adjust a counter rather than recompute a total), re-validation at commit, or business-level reservation of the resource.

A strong saga design names, for each step, which anomalies are possible and how they are handled. A weak design assumes "eventual consistency handles it" and discovers double-bookings or negative balances in production.

### Use TCC Where You Need Stronger Per-Resource Semantics Than A Saga Gives

Try-Confirm-Cancel reserves a resource in the Try phase, finalizes it in Confirm, and releases it in Cancel. Unlike a saga, TCC holds an explicit reservation between Try and Confirm, giving stronger isolation for the reserved resource — the resource is not freely available to other transactions during the window. This makes TCC a good fit for inventory, seat, or balance operations where you need to tentatively hold a resource across a multi-step flow without a real lock.

TCC's costs are real: every participating resource must implement all three interfaces (Try, Confirm, Cancel) with idempotency, the framework must guarantee Confirm or Cancel is eventually called for every Try (or resources leak forever), and Confirm/Cancel must be idempotent and safe to retry. A weak TCC design leaks reservations when Confirm or Cancel is lost. A strong design treats the Try as a durable reservation record, drives Confirm/Cancel through a reliable coordinator or outbox, and has a reconciliation sweep that finds dangling Try records past their timeout and cancels them. Do not adopt TCC lightly; it is more invasive than a saga because it requires the resource to support reservation semantics natively.

### Use The Transactional Outbox To Publish Events Reliably Without 2PC

The dual-write problem is fundamental: you want to commit a database change and publish a message, but the message broker is not a participant in the database transaction. If you commit then publish, a crash after commit loses the message; if you publish then commit, a crash after publish delivers a message for a change that never happened. The transactional outbox solves this by writing the event into an outbox table in the same local transaction as the business change, so they commit atomically, and a separate relay (polling publisher or change-data-capture stream) reads the outbox and publishes to the broker.

The key correctness points: the write to the outbox must be in the same database transaction as the business write, or you have reintroduced the dual-write problem. The relay must be at-least-once, which means consumers must be idempotent — the same event can be published and consumed more than once after a relay restart or crash. Each outbox row needs a stable unique identifier so consumers can deduplicate, and the relay must track its publishing position durably so it does not skip or replay large ranges on restart. A strong design also bounds outbox growth (archive or delete published rows) and monitors outbox lag, because a stalled relay silently makes the whole system eventually consistent with unbounded delay. Do not assume publishing from a transaction's after-commit hook is equivalent to an outbox — the hook fires after commit and can still be lost on crash.

### Treat Distributed Locks As A Correctness-Hazardous Primitive and plan For Coordinator And Participant Recovery Explicitly

A distributed lock coordinates exclusive access across nodes, but it is one of the most frequently misimplemented primitives because the intuitive mental model (a lock guarantees mutual exclusion) breaks under clock skew, lease expiry, and process pauses. Before using one, ask whether you actually need mutual exclusion or whether a simpler idempotent operation, a database constraint, or an optimistic compare-and-set would achieve the same goal more robustly. Locks add a failure mode (the lock service itself) and a correctness surface (expiry, fencing) that is easy to get wrong.

If you do use a distributed lock, you must handle the **stale-lock-holder problem**: a client acquires a lock with a lease, then gets paused (garbage collection, scheduling, slow downstream call) longer than the lease. The lease expires, another client acquires the lock, and now two clients believe they hold it. The fix is **fencing tokens**: every lock acquisition returns a monotonically increasing token, and the protected resource rejects operations that arrive with a stale token. This requires the protected resource to check the token, which is why fencing works cleanly with a resource you control (a database row with a version column) and poorly with a resource you do not (an external API with no token concept). Without fencing, a lock with lease expiry is a probability, not a guarantee, of mutual exclusion.

Know the semantics of the specific lock implementation. Redis-based locks (Redlock and single-node variants) rely on wall-clock expiry and have known correctness arguments against them under pause and clock drift; they are acceptable for low-stakes mutual exclusion (avoiding redundant work) and unsafe for correctness-critical exclusion. etcd and ZooKeeper leases are built on consensus and are far more robust, but still require fencing for the stale-holder case because lease expiry and client action are not atomic. Choose the lock to match the stakes: consensus-backed lease plus fencing for correctness, Redis for efficiency, and never rely on expiry timing alone for a correctness invariant.

Every coordinator-based pattern (2PC, orchestrated saga, TCC coordinator) has a recovery story, and "it will just work" is not one. For each, decide before deployment: where is the coordinator's durable state stored, what happens if the coordinator crashes between phases, how are in-flight transactions discovered on restart, and what is the manual recovery procedure if recovery automation itself fails. Participants must handle duplicate commit/abort/confirm/cancel messages idempotently because the coordinator will retry on timeout.

A strong design logs every transition durably, has a replay-on-restart path, exposes in-flight transactions for inspection, and documents the runbook for a stuck transaction. A weak design holds coordinator state in memory and loses in-flight transactions on restart, or relies on timeouts that resolve to an unsafe default. The recovery design is where most production distributed-transaction incidents are born, so treat it as a first-class deliverable, not an afterthought.

## Common Traps

### Treating Compensation As The Literal Inverse Of The Forward Step

Designing a refund that deletes the original charge record, or a cancellation that erases the shipment attempt. Compensation must preserve audit history and return the business to a consistent — not identical — state. Designing compensation as undo loses information and breaks regulatory and audit requirements.

### Placing A Non-Compensable Side Effect In The Middle Of A Saga

Sending the confirmation email or pushing the external payment before the saga's later steps have succeeded, then failing a later step and having no way to undo the irreversible action. Move non-compensable steps to the end, or define an explicit point of no return with manual recovery.

### Publishing An Event From An After-Commit Hook Instead Of An Outbox

Firing the message publish in a post-commit callback and assuming the transaction guarantees delivery. A crash between commit and publish (or a broker outage at that instant) silently drops the event. Use a transactional outbox written in the same transaction, or accept that events can be lost.

### Assuming At-Least-Once Delivery Means At-Most-Once Effect

Building consumers that are not idempotent because "the relay delivers once." Relays, retries, and crash recovery all redeliver. Every outbox consumer, saga step, and compensation handler must be idempotent on a stable identifier, or you will double-charge, double-ship, or double-reserve.

### Using A Redis Lock With Expiry For A Correctness Invariant And No Fencing

Relying on `SETNX` with a TTL to prevent double-spend, with no fencing token checked at the protected resource. A GC pause longer than the TTL lets two clients hold the lock simultaneously and corrupt the invariant. Use consensus-backed leases with fencing, or restructure to avoid the lock.

### Picking 2PC Across Independent Services Because It Sounds Atomic

Applying two-phase commit across services owned by different teams with independent deployment and failure, then suffering coordinator outages that lock resources across the whole organization. Reserve 2PC for a single administrative boundary; use saga or outbox across service boundaries.

### Letting Saga Intermediate States Leak Without Isolation Handling

Allowing concurrent sagas to read uncommitted intermediate values and act on them, producing double-bookings or negative balances. Name the isolation anomalies (dirty read, lost update, phantom) for each step and handle them with semantic locks, optimistic checks, or commutative updates.

### Trusting The Orchestrator To Be Stateless and assuming TCC Confirm Or Cancel Will Always Be Called

Building a saga orchestrator that keeps workflow state in memory and loses it on restart, abandoning in-flight sagas or restarting them from the beginning and re-executing side effects. Make orchestrator state durable and resumable, and make every step idempotent so resume is safe.

Leaking reservations indefinitely when the coordinator crashes between Try and Confirm/Cancel, because there is no reconciliation sweep. Add a timeout-based sweep that cancels dangling Try records, and make Confirm/Cancel idempotent.

## Self-Check

- [ ] The need for a distributed transaction was justified against the alternative of local transactions plus eventual coordination (outbox, saga, reconciliation), and the invariant that forced the choice is documented.
- [ ] If 2PC is used, the blocking-coordinator-failure behavior is understood, the coordinator is highly available within one administrative boundary, and participant timeout/lock behavior is designed — not used across independent services.
- [ ] The saga coordination style (choreography vs orchestration) was chosen deliberately based on step count and observability needs, and an orchestrator's state is durable and resumable on crash.
- [ ] Every saga step's compensating action is semantically meaningful (not literal undo), idempotent, retriable, and preserves audit history; non-compensable steps are placed last or marked as a point of no return with a manual recovery runbook.
- [ ] Saga isolation anomalies (dirty reads, lost updates, phantom reads) are named per step and handled with semantic locks, optimistic concurrency, or commutative updates — not assumed away by eventual consistency.
- [ ] If TCC is used, every resource implements Try/Confirm/Cancel idempotently, Confirm/Cancel are guaranteed eventually callable via a reliable coordinator or outbox, and a reconciliation sweep cancels dangling Try records.
- [ ] Event publishing uses a transactional outbox written in the same database transaction as the business change (not an after-commit hook), the relay is at-least-once with a durable position, and every consumer is idempotent on a stable identifier.
- [ ] Any distributed lock is matched to the stakes (consensus-backed lease for correctness, Redis for efficiency), uses fencing tokens checked at the protected resource, and does not rely on expiry timing alone for a correctness invariant.
- [ ] Coordinator and participant recovery is designed as a first-class deliverable: durable state transitions, replay-on-restart, idempotent duplicate handling, in-flight transaction inspection, and a documented manual recovery runbook.
- [ ] No step silently drops or duplicates a real-world side effect (charge, shipment, reservation, notification) under partial failure, message redelivery, or coordinator crash — the failure paths were traced end to end, not just the happy path.
