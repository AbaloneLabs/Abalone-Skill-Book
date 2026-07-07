---
name: transaction_rollback_and_compensating_actions.md
description: Use when the agent is handling errors that occur mid-multi-step-operation where some steps have already committed — designing transaction rollback, compensating actions, the Saga pattern, distributed transaction coordination, idempotent undo operations, or cleanup-on-failure for workflows spanning multiple services or resources; deciding between ACID transactions, eventual consistency with compensation, or 2PC; or handling partial-failure states where a rollback is impossible or complex. Covers transaction boundaries, compensating action design (idempotent, reversible, order-aware), the Saga pattern and its coordination, the limits of compensation (non-reversible operations like sending email or payments), and the discipline of designing multi-step operations for failure from the start rather than assuming success.
---

# Transaction Rollback And Compensating Actions

A transaction is a unit of work that should be all-or-nothing: either all its steps commit, or none do, so the system is never left in a partial state. Within a single database, ACID transactions provide this guarantee — if any step fails, the transaction rolls back, undoing all prior steps as if they never happened. But the moment an operation spans multiple resources (two databases, a database and a message queue, a database and an external API), a single ACID transaction cannot encompass it, and the all-or-nothing guarantee must be achieved differently. The two approaches are distributed transactions (2PC, which coordinates commit across resources but is slow, blocking, and fragile) and compensating actions (the Saga pattern, where each step has a corresponding undo, and a failure triggers the undos for the completed steps). The compensating-action approach accepts that steps commit independently and achieves semantic atomicity by undoing on failure — but this only works if every step is reversible, and some operations are not. An email cannot be un-sent; a payment cannot be un-charged (only refunded, which is a different operation); a message published to a queue cannot be un-published (only offset, which consumers may have already processed). The discipline is recognizing which operations are reversible and designing their compensations, which are not and must be handled differently (deferred to the end, idempotent with out-of-band reconciliation), and structuring multi-step operations so that failure at any point has a defined recovery path.

Agents tend to assume multi-step operations will succeed (not designing for mid-failure), to attempt rollback for non-reversible operations (sending an "undo" email that confuses users), and to write non-idempotent compensating actions that break under redelivery. The judgment problem is recognizing that distributed all-or-nothing requires explicit design (compensation or 2PC), that compensation's correctness depends on the reversibility of each step and the idempotency of each compensation, and that non-reversible steps must be handled by reordering or out-of-band reconciliation. This skill covers the discipline of transaction rollback and compensating actions: transaction boundaries, compensating action design, the Saga pattern, non-reversible operations, and failure-first design.

## Core Rules

### Identify The Transaction Boundary And Whether A Single ACID Transaction Suffices

The first question is whether the operation fits within a single resource's transaction. If so, use ACID; if not, design compensation.

- **Keep operations within a single database transaction where possible.** A multi-step operation against one database should be a single ACID transaction; the database handles rollback on failure. Do not split what a transaction can cover (separate updates to the same database that should be atomic).
- **Recognize when an operation spans resources and cannot be a single transaction.** An operation touching two databases, a database and a queue, or a database and an external API cannot be a single ACID transaction (without 2PC). This is where compensation or distributed transactions are needed.
- **Design the operation to minimize the span where possible.** Sometimes restructuring reduces the span: collecting all data first, then doing a single database write, rather than writing to the database and then calling an external API. The less the operation spans, the less compensation is needed.
- **Choose between 2PC (distributed transactions) and Saga (compensation) by the requirements.** 2PC provides true atomicity but is slow, blocking, and fragile (coordinator failure stalls participants). Saga provides eventual consistency via compensation, is more available, but allows intermediate states to be visible. Match the choice to the need for atomicity vs availability.

### Design Compensating Actions For Each Reversible Step

In a Saga, each step has a compensating action that semantically undoes it. Design these carefully.

- **For each step, define its compensating action (the semantic undo).** A step "create order" has compensation "cancel order"; a step "charge credit card" has compensation "refund credit card"; a step "reserve inventory" has compensation "release inventory." The compensation reverses the step's effect.
- **Make compensating actions idempotent.** A compensation may be called multiple times (if the retry is not acknowledged, or if the failure recovery re-triggers it). The compensation must be safe to call multiple times: refunding the same charge twice is a bug; marking an order cancelled twice is fine (if idempotent). See idempotency-and-race-safety.
- **Make compensating actions order-aware.** Compensations execute in reverse order of the original steps (undo the last step first, then the second-to-last, etc.). Design them to handle being called in this order, and to handle dependencies between compensations (a refund may require the order to be cancelled first).
- **Ensure compensating actions can fail safely.** A compensation that itself fails (the refund API is down) must be retried (with backoff) and, if persistently failing, escalated (human intervention, a reconciliation process). A failed compensation leaves the system in an inconsistent state that must be resolved, not ignored.

### Handle Non-Reversible Operations Specially

Some operations cannot be undone by a compensation. Identify these and handle them differently.

- **Identify non-reversible operations: sending emails, publishing messages, triggering side effects, payments (refundable but not un-done), physical actions.** These cannot be compensated in the sense of "undo as if it never happened." A compensation for a non-reversible operation is a different operation (a refund for a charge, a correction email for a wrong email) that mitigates but does not erase.
- **Defer non-reversible operations to the end of the transaction.** If an operation cannot be undone, do it last, after all reversible steps have committed, so a failure in earlier steps does not require undoing it. Send the confirmation email only after the order, payment, and inventory are all committed.
- **For non-reversible operations that must happen mid-transaction, use idempotency and out-of-band reconciliation.** If a non-reversible operation must happen mid-transaction (a payment that gates the rest), make it idempotent (so redelivery is safe) and plan for out-of-band reconciliation (a process that detects and resolves inconsistencies, such as a payment charged but the order not completed).
- **Do not pretend a non-reversible operation is reversible.** A "compensation" that sends an "undo" email or publishes a "rollback" message does not undo the original; it adds a new side effect that consumers must handle. Be honest about what is reversible and design accordingly.

### Use The Saga Pattern For Multi-Step Distributed Operations

The Saga pattern coordinates a multi-step distributed operation with compensations. Understand its forms.

- **Choreography saga: each service emits an event on success, triggering the next step; on failure, emits a compensation event.** Decentralized (no central coordinator); each service reacts to events. Simple to start, but the flow is implicit (hard to follow) and cyclic event chains are a risk.
- **Orchestration saga: a central orchestrator coordinates the steps, calling each service and triggering compensations on failure.** Centralized (the orchestrator owns the flow); the flow is explicit and visible. More complex (an orchestrator to build and operate) but clearer and easier to manage for complex flows.
- **Choose the form by the flow's complexity.** Simple flows (a few steps, few services) suit choreography; complex flows (many steps, conditional logic, many services) suit orchestration. Do not over-engineer a simple flow with an orchestrator, or under-manage a complex flow with choreography.
- **Persist the saga state for recovery.** A saga that crashes mid-execution must be recoverable: its state (which steps completed) must be persisted, so on restart it can resume or compensate. An in-memory saga state is lost on crash, leaving the operation in an ambiguous state.

### Design For Failure From The Start, Not As An Afterthought

Multi-step operations fail, and the failure handling must be designed alongside the happy path, not added later.

- **Design the compensation as you design each step.** When you add a step to an operation, immediately define its compensation (or mark it non-reversible and handle accordingly). A step without a defined compensation is a step whose failure leaves an inconsistent state.
- **Test the failure paths, not just the happy path.** Inject failures at each step (step 1 fails, step 2 fails, ..., step N fails) and verify the compensations execute correctly, in order, and leave the system consistent. Untested failure paths fail when first encountered in production.
- **Monitor for stuck or incomplete sagas.** A saga that stalls (a compensation persistently failing, a step that never completes) leaves the system inconsistent. Monitor saga state; alert on sagas that are stuck or have been running too long.
- **Provide reconciliation for cases compensation cannot resolve.** Some failures cannot be automatically compensated (a non-reversible operation failed mid-way, a compensation exhausted retries). Provide a reconciliation process (automated or manual) that detects and resolves these cases, rather than leaving them inconsistent indefinitely.

## Common Traps

### Assuming Success (No Failure Design)

A multi-step operation designed only for the happy path, with no compensation or rollback for mid-failure, leaving inconsistent states on failure. Design failure paths alongside the happy path.

### Compensating A Non-Reversible Operation

Treating a non-reversible operation (email, payment) as reversible with a naive "undo," adding side effects rather than undoing. Identify non-reversible operations; defer or reconcile.

### Non-Idempotent Compensating Action

A compensation that double-applies on redelivery (double refund, double cancellation), because it is not idempotent. Make compensations idempotent.

### Compensation Order Not Handled

Compensations executed out of order, violating dependencies (refunding before cancelling the order), causing failures. Execute compensations in reverse order; handle dependencies.

### Failed Compensation Ignored

A compensation that fails (API down) and is not retried or escalated, leaving the system inconsistent. Retry with backoff; escalate persistently-failing compensations.

### Saga State Not Persisted

An in-memory saga state lost on crash, leaving the operation in an ambiguous state (completed steps with no record, no compensation triggered). Persist saga state for recovery.

### 2PC Where Saga Suffices

Using 2PC (slow, blocking, fragile) for an operation that tolerates eventual consistency and could use a Saga. Match the transaction model to the requirement.

### Non-Reversible Operation In The Middle Of The Transaction

A non-reversible operation (sending the email) executed mid-transaction, requiring undo on later failure. Defer non-reversible operations to the end.

## Self-Check

- [ ] The transaction boundary is identified — operations fitting within a single resource use ACID transactions (with rollback handled by the database), operations spanning resources use compensation (Saga) or 2PC chosen by the atomicity-vs-availability requirement, and the operation is structured to minimize span where possible.
- [ ] Each reversible step has a defined compensating action (the semantic undo) that is idempotent (safe under redelivery), order-aware (executes in reverse order, handles dependencies), and fails safely (retried with backoff, escalated if persistent).
- [ ] Non-reversible operations (emails, published messages, side effects, payments) are identified and handled specially — deferred to the end of the transaction (after reversible steps commit), or made idempotent with out-of-band reconciliation if they must occur mid-transaction — not naively "compensated."
- [ ] The Saga pattern (choreography or orchestration, chosen by flow complexity) coordinates multi-step distributed operations, with saga state persisted for crash recovery (so a crashed saga resumes or compensates rather than leaving an ambiguous state).
- [ ] Failure paths are designed alongside the happy path (each step's compensation defined when the step is added), tested by injecting failures at each step (verifying compensations execute correctly and leave the system consistent), and monitored for stuck or incomplete sagas.
- [ ] A reconciliation process (automated or manual) exists for cases compensation cannot resolve (non-reversible operation failures, persistently-failing compensations), so the system is not left inconsistent indefinitely.
- [ ] Compensating actions are tested for idempotency (calling twice produces the same effect as once), order (executing in reverse order with dependencies handled), and failure (retry and escalation behavior), not just for the happy path.
- [ ] The operation's intermediate states (visible between steps in a Saga) are acceptable to the business logic, or the operation is restructured (single transaction, or steps reordered) to avoid unacceptable intermediate states — because a Saga does not hide intermediate states the way an ACID transaction does.
