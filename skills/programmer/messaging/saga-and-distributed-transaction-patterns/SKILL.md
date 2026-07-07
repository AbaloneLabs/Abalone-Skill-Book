---
name: saga_and_distributed_transaction_patterns.md
description: Use when the agent is designing or reviewing a distributed transaction or multi-service business workflow that cannot use a single database transaction, deciding how to coordinate writes across independent services without two-phase commit, applying the saga pattern with choreography or orchestration, designing compensating transactions that safely undo completed work, implementing the transactional outbox pattern for reliable event publishing, or guarding against the trap of assuming a distributed transaction is atomic. Also covers compensation failure handling, the dual-write problem between a database and a message broker, semantic rollback versus literal undo, pivot points and non-reversible effects, and the recurring mistake of building a saga whose compensations themselves fail and leave the system inconsistent with no recovery path.
---

# Saga And Distributed Transaction Patterns

A single business operation — place an order, book a trip, transfer funds, provision an account — often requires updates across multiple services and databases, and no single ACID transaction can span them all. Two-phase commit (2PC), the classical answer to distributed transactions, is impractical across independent services: it requires a coordinator that all participants trust, holds locks while coordinating, has poor availability under participant failure, and is rarely supported by modern heterogeneous datastores and cloud services. The saga pattern is the practical alternative: the operation is broken into a sequence of local transactions, each atomic within its own service, and each paired with a compensating transaction that semantically undoes its effect if a later step fails. The judgment problem is that this trades the atomicity and isolation of a real transaction for a set of weaker guarantees that must be painstakingly designed. A saga is not atomic — partial results are visible mid-process. It is not isolated — concurrent sagas interleave and conflict. And its correctness rests entirely on compensations that actually undo their steps, including when those compensations themselves fail. Agents who treat a saga as "a distributed transaction" reproduce all the guarantees of neither and ship systems that corrupt state on the first real failure.

Agents tend to under-invest here because the happy path — every step succeeds — works trivially and looks like a simple call chain. The harm appears only under failure, and it is severe and silent. A payment is charged, then shipment fails, and the compensation to refund is missing or buggy, leaving the customer charged for an order that will never ship. A service commits its database write and then crashes before publishing the event that would trigger the next step, so the saga stalls with the write applied and no one notified — the dual-write problem. A compensation fails because the downstream it targets is down, and there is no handling for that case, so the system sits half-done forever. Two concurrent sagas reserve the last item of inventory and both succeed, overselling, because nothing isolated them. The discipline is to design the saga against these specific failure modes: choose coordination deliberately, publish events reliably through an outbox, make compensations correct and safe under their own failure, and never assume atomicity that the pattern does not provide.

This skill is the deep treatment of distributed transactions without 2PC. It complements the saga-and-process-manager skill (which focuses on orchestration versus choreography, isolation, and the process-manager lifecycle) by concentrating on the transactional mechanics that make a saga correct: why 2PC is rejected and what that costs, the transactional outbox that solves the dual-write problem, compensation design that holds even when compensations fail, and the atomicity illusion that causes engineers to under-design failure paths. It also connects to the idempotency-and-duplicate-handling skill (compensations and outbox relays must be idempotent) and the message-queue-design-and-delivery skill (delivery semantics for saga messages).

## Core Rules

### Reject Two-Phase Commit And Accept What Its Absence Costs

The first decision is to accept that a cross-service operation cannot be made atomic by a coordinator, and to design explicitly for the absence of atomicity. Two-phase commit is the textbook mechanism for distributed atomicity, and in modern microservice environments it is almost always the wrong choice: it holds locks across participants during coordination, it blocks if the coordinator or a participant fails during the second phase, it requires every participant to support the protocol (which cloud services and most third-party systems do not), and its availability is the product of its participants' availability. The practical answer is the saga: a sequence of local transactions whose collective effect approximates a distributed transaction, with compensations providing rollback.

Accepting this means accepting three costs that must be designed for, not wished away:

- **Loss of atomicity.** Intermediate states are visible. Other services and queries can observe a saga half-complete (inventory reserved but payment not yet taken). The system and its clients must tolerate this, or the saga must be structured so visible intermediate states are themselves meaningful.
- **Loss of isolation.** Concurrent sagas interleave; there is no serial-equivalent guarantee. Two sagas acting on the same resource can conflict, and isolation must be provided explicitly through semantic locks, reservations, or optimistic concurrency.
- **Rollback is semantic, not literal.** A database transaction rolls back by discarding uncommitted state; a saga cannot, because each step's local transaction has already committed. Compensation is a forward action that semantically undoes a committed effect (a refund, a release, a cancellation) — and that means compensation must be designed, it can fail, and some effects cannot be undone at all.

The rule: never describe a saga as a distributed transaction. It is a coordinated sequence of local transactions with compensating actions, and every guarantee a real transaction would give for free must be provided by design.

### Solve The Dual-Write Problem With The Transactional Outbox

The most common and most subtle failure in a saga is the dual-write: a service commits a state change to its database, then publishes an event or message to trigger the next step, and crashes between the two — leaving the state changed but the saga stalled, or (if the order is reversed) publishing an event for a change that was never committed. The two writes are to different systems (database and broker) and cannot be atomically committed together, so any crash between them produces inconsistency. This is the dual-write problem, and the transactional outbox is its solution.

- **Write the outbound event to an outbox table in the same local transaction as the state change.** The state update and the event record commit or roll back together, so they can never diverge. The event is not sent to the broker within the transaction; it is stored locally as a row to be relayed.
- **A separate relay process reads the outbox and publishes to the broker.** The relay polls or streams the outbox table, publishes each event to the message broker, and marks it as sent. Because the relay operates on committed rows, a crash during relay simply re-reads and re-publishes — safely, because the consumer side is idempotent.
- **The relay must be at-least-once and the consumer idempotent.** The relay may publish an event twice (it crashed after publishing but before marking sent), so the downstream consumer must deduplicate by a stable event id. This couples the outbox directly to the idempotency discipline: an outbox without idempotent consumers reintroduces duplicate effects.
- **Never publish directly from the application after the commit.** The pattern of "commit, then send" is the dual-write bug in its purest form. Any code path that publishes a saga event outside the outbox is a path that can lose or duplicate the event under crash.

The outbox is the mechanism that makes saga event publishing reliable. Without it, every saga step that publishes an event is a dual-write waiting to fail; with it, event publication inherits the local transaction's atomicity and the relay's at-least-once delivery.

### Design Compensations That Hold Even When They Fail

A saga's correctness on failure rests on its compensations, and the hardest part of compensation design is not the happy compensation (refund succeeds) but the failed compensation (the refund service is down). A compensation that can fail without a defined response leaves the system in an inconsistent state that no automatic mechanism will repair. Compensation design must therefore treat compensation failure as a first-class case, not an afterthought.

- **Define a compensation for every side-effecting step.** If a step reserves inventory, its compensation releases it; if it charges payment, its compensation refunds; if it sends a confirmation email, its compensation may send a correction or may be a no-op. Identify which steps are compensable and which are not.
- **Make compensations idempotent and retriable.** A compensation may be invoked more than once after a crash and recovery; it must be safe to re-run. Refund by a stable idempotency key, not by re-charging; release by a reservation id, not by decrementing again. A non-idempotent compensation doubles the rollback on retry.
- **Handle compensation failure explicitly with a defined policy.** When a compensation fails (the target service is unavailable, the refund is rejected), the saga cannot simply give up. Define the response: retry with backoff, escalate to human intervention, mark the saga as requiring manual attention, and crucially keep the saga's state durable so it can be resumed rather than lost. Silent compensation failure is how systems drift into permanent inconsistency.
- **Recognize non-reversible effects and structure around them.** Some steps cannot be undone — a wire transfer that settled, a physical shipment that left the warehouse, a notification already received by a user. Place these at a pivot point: the saga's irreversible action happens only after every compensable step that must precede it has succeeded, so that once the pivot is crossed the saga commits to completion and compensation is no longer needed for the steps before it.

The rule: a compensation is not "the undo function." It is a forward operation with its own failure modes, its own idempotency requirement, and its own escalation path, and it must be designed with the same rigor as the forward step it reverses.

### Choose Choreography Or Orchestration Against The Saga's Complexity

A saga is coordinated either by choreography (each service reacts to events and emits the next) or by orchestration (a central coordinator issues commands and tracks state), and the choice has direct consequences for reliability and failure handling. This decision is treated in depth in the saga-and-process-manager skill; the transactional concern here is how the choice interacts with the outbox and compensation safety.

- **Choreography distributes the coordination across event handlers, and its reliability is the composition of each handler's outbox and idempotency.** Each step's event is published via that service's outbox, and each consumer is idempotent. The saga's flow is implicit and can become hard to trace for complex flows, which makes failure diagnosis harder.
- **Orchestration centralizes the flow in a coordinator whose state is persisted.** The coordinator records each step's completion and drives compensations explicitly, which makes the flow readable and compensation ordering explicit. The coordinator must itself be durable and recoverable — an in-memory orchestrator that crashes loses the saga.
- **Match the choice to complexity.** Simple, short, decoupled flows may choreograph cleanly; complex flows with branching compensation are far more reliable and maintainable under orchestration, because the compensation logic is explicit and testable rather than emergent across event handlers.

The transactional implication: whichever is chosen, every step's event publication must go through an outbox and every consumer (including the orchestrator) must be idempotent, or the saga will lose or duplicate steps under failure.

### Make Saga State Durable And Recoverable, And Detect Stuck Sagas

A saga spans multiple services and multiple messages, and any of them can fail mid-process. The saga must be recoverable to a consistent position, and the cases it cannot recover from automatically must be detectable.

- **Persist saga state at each transition, before performing the action.** Record "about to charge payment" before charging, so a crash between recording and charging resumes by retrying the action rather than skipping it. Recording after the action risks skipping it on crash.
- **Make every action and compensation idempotent so recovery retry is safe.** On recovery, the saga may re-invoke an action that partially completed; without idempotency, recovery duplicates the effect.
- **Detect and escalate stuck sagas.** A saga that can neither complete nor compensate — a compensation failed permanently, a dependency is unavailable for longer than any retry budget — must not sit invisibly forever. Monitor for long-running and stuck sagas, alert on them, and route them to human intervention with enough persisted context to be resolved manually.

## Common Traps

### Assuming A Saga Is Atomic Like A Database Transaction

Describing or designing the saga as if it provided the atomicity and isolation of an ACID transaction, so intermediate states are not designed to be visible, concurrent sagas are not isolated, and rollback is assumed to be automatic. The trap is the word "transaction." A saga is a sequence of local transactions with compensations; atomicity, isolation, and rollback must each be designed explicitly.

### The Dual-Write Between Database And Broker

Committing a state change to the database and then publishing an event to the broker in separate steps, so a crash between them leaves the state changed but the saga stalled (or publishes an event for an uncommitted change). The trap is that the two operations look coupled in code. Use the transactional outbox so the event is committed with the state and relayed separately.

### A Compensation That Itself Fails With No Handling

A compensation that calls a downstream service which is down, with no retry, escalation, or durable state, so the saga is left half-rolled-back with no signal and no recovery. The trap is treating compensation as the undo that always works. Design compensation failure as a first-class case with a defined policy and escalation.

### Non-Idempotent Compensations Under Retry

A compensation that is not safe to re-run, so a crash and recovery that retries it doubles the rollback (a double refund, a double release). The trap is assuming compensation runs once. Make compensations idempotent by a stable key, exactly as the forward actions must be.

### Crossing A Pivot Point With Compensable Steps Still Ahead

Performing an irreversible action (a settled transfer, a shipped package) before the compensable steps that must precede it have all succeeded, so a later failure cannot roll back because the irreversible step has already happened. The trap is sequencing for the happy path. Structure the saga so irreversible actions are pivot points crossed only after all required compensable steps succeed.

### An In-Memory Orchestrator That Loses The Saga

An orchestrator whose state lives in memory, so a crash loses the saga's progress and recovery re-runs or skips steps blindly. The trap is treating the coordinator as stateless. Persist the orchestrator's state at each transition and recover to a consistent position on restart.

### Publishing Saga Events Outside The Outbox

A code path that publishes a saga event directly to the broker after the commit (a quick fix, a notification, an ad-hoc integration), reintroducing the dual-write problem for that path. The trap is that it works in testing. Every saga event publication must go through the outbox.

### A Stuck Saga With No Detection

A saga that cannot complete and cannot compensate, left forever in limbo because nothing monitors for it, so the inconsistency is discovered only when a customer reports it. The trap is assuming sagas always resolve. Monitor for long-running and stuck sagas and escalate them.

## Self-Check

- [ ] The design explicitly rejects two-phase commit and accepts its costs: intermediate states are designed to be visible (or the saga is structured so they are meaningful), concurrent sagas are isolated through semantic locks or optimistic concurrency, and rollback is understood as semantic compensation rather than literal undo.
- [ ] Every saga step that publishes an event uses the transactional outbox — the event is written to an outbox table in the same local transaction as the state change and relayed by a separate at-least-once process — and no code path publishes saga events directly after a commit, so the dual-write problem is eliminated.
- [ ] The outbox relay is at-least-once and every saga consumer (including the orchestrator) is idempotent by a stable event id, so a relay crash that re-publishes does not duplicate effects — the outbox and idempotency are designed together, not separately.
- [ ] Every side-effecting step has a defined, idempotent compensation that semantically undoes it, compensation failure is handled with an explicit policy (retry with backoff, escalate, durable state for resume), and non-reversible effects are placed at pivot points crossed only after all required compensable steps succeed.
- [ ] Saga state is persisted at each transition before the action is performed, every action and compensation is idempotent so recovery retry is safe, and an orchestrator (if used) persists its state rather than holding it in memory so a crash does not lose the saga.
- [ ] Stuck sagas — those that can neither complete nor compensate — are detected by monitoring for long-running sagas, alerted on, and routed to human intervention with enough persisted context to be resolved manually, rather than left in limbo.
- [ ] The coordination choice (choreography or orchestration) matches the saga's complexity, with orchestration preferred for complex flows with branching compensation so that compensation ordering is explicit and testable.
- [ ] The highest-risk cases were specifically verified — a crash between database commit and event publish, a compensation that failed, a non-idempotent compensation retried, an irreversible action sequenced before a compensable step failed, and a stuck saga left undetected — rather than only the all-steps-succeed happy path.
