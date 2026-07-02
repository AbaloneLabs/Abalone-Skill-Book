---
name: idempotency_and_race_safety.md
description: Use when the agent is implementing retries, background jobs, concurrent requests, transactions, queue consumers, payment or order operations, shared state updates, or any workflow where duplicate or overlapping execution could corrupt state.
---

# Idempotency And Race Safety

Many programs work correctly in a single happy-path request and fail when two requests, retries, jobs, tabs, workers, or deployments act at the same time. Reliability work requires identifying what can run twice, what can overlap, what must be serialized, and what state transition must remain true under failure.

Use this skill before implementing create/update flows, payment handling, order placement, inventory updates, email sending, file processing, imports, scheduled jobs, webhooks, queue consumers, locks, caches, counters, or any operation where "the same thing happens twice" would be harmful.

## Core Rules

### Identify The Unit Of Correctness

Before choosing locks or transactions, define what must be true. The unit may be an account balance, order id, idempotency key, user action, inventory item, file, tenant, invoice period, or external provider event.

For each operation, answer:

- What is the business action?
- What state transition should happen once?
- What inputs identify the same action repeated?
- Which side effects must not duplicate?
- Which side effects can safely repeat?
- What is the final state after retry, timeout, or partial failure?

Do not start with "add a mutex" or "wrap it in a transaction". Start with the invariant.

### Make Retries Safe

Clients, load balancers, job runners, webhooks, and humans retry. Timeouts do not prove failure. A request may complete on the server after the client gives up. A worker may crash after performing an external side effect but before marking the job done.

Use idempotency keys for operations that create durable effects. Store the key with enough scope and result data to answer repeated requests consistently. A key should usually be scoped by actor, tenant, endpoint, and semantic action, not just by raw payload.

For background jobs, design retry behavior explicitly. A job should be able to re-run without duplicating irreversible side effects. If a step cannot be repeated, record a durable marker before or after it in the correct order and reconcile uncertain states.

### Use Database Constraints As Concurrency Controls

Application checks can race. If two requests both run "check if row exists, then insert", both may pass the check. Prefer unique constraints, conditional updates, version columns, row locks, or transactional state transitions.

Examples:

- unique idempotency key prevents duplicate order creation;
- partial unique index prevents two active subscriptions for the same account;
- conditional update changes status only from `pending` to `confirmed`;
- optimistic version check prevents lost updates;
- row lock serializes balance changes for one account;
- foreign key prevents orphaned child rows.

Let the database reject impossible states, then map conflict errors to deliberate application behavior.

### Keep Transactions Small But Complete

A transaction should include all database changes that must commit together. It should not include slow network calls unless the system deliberately accepts the lock duration and failure modes.

When external side effects are involved, consider patterns such as:

- write intent, commit, then process side effect asynchronously;
- outbox table for messages and events;
- provider idempotency keys;
- reconciliation job for uncertain provider state;
- compensation action when true rollback is impossible.

Avoid claiming atomicity across your database and an external API unless you have a real distributed transaction, which most applications do not.

### Choose Lock Scope Deliberately

Locks are tools, not proof of correctness. A global lock may be simple but can destroy throughput and create deadlocks or availability problems. A narrow lock may miss related state.

Define:

- lock key and scope;
- timeout and wait behavior;
- what happens if lock acquisition fails;
- whether lock survives process crash;
- whether multiple app instances share it;
- how deadlocks are detected or avoided.

In-process mutexes protect only one process. They do not protect multiple containers, workers, cron jobs, scripts, or database clients. Use database locks, advisory locks, distributed locks, or constraints when the race crosses process boundaries.

### Preserve Ordering Where It Matters

Some workflows are order-sensitive: status transitions, event streams, message delivery, ledger entries, and sync cursors. Out-of-order processing can revert state, send stale notifications, or process old provider events after new ones.

Use monotonic versions, sequence numbers, timestamps with tie-breakers, transition guards, or event ordering rules. Do not rely only on wall-clock precision if events can share timestamps or come from different systems.

### Make Side Effects Observable And Reconciliable

Email, payment, shipment, notification, file upload, cache invalidation, and webhook delivery can fail independently of database commits. Record enough state to detect and repair partial completion.

For each side effect, define:

- when it is scheduled;
- how many attempts are allowed;
- whether duplicate delivery is acceptable;
- which provider response id is stored;
- how operators can inspect status;
- what reconciliation can do if state is uncertain.

### Test The Race, Not Only The Function

Single-thread tests do not prove concurrency safety. Add tests that run duplicate requests, concurrent creates, stale updates, repeated webhooks, job retries, and transaction conflicts. Use unique constraints and conditional updates in tests so the expected conflict path is exercised.

If deterministic concurrency tests are hard, at least isolate the state transition function and verify conflict handling. Add integration tests for the most damaging duplicate paths.

## Common Traps

### Check Then Insert

The pattern "if no row exists, insert one" is unsafe without a unique constraint or transaction isolation that prevents the race. Two workers can both observe absence.

### Assuming A Queue Delivers Exactly Once

Most queues are at-least-once. Workers must assume a message can be delivered more than once, after a timeout, or after partial work. Deduplicate by durable keys and make handlers idempotent.

### Using Timestamps As Ordering Alone

Timestamps can tie, drift, truncate, or arrive late. Use sequence numbers, provider event ids, versions, or deterministic tie-breakers where ordering is critical.

### Holding Locks During External Calls

Calling a payment provider, email service, or file API while holding a database lock can block unrelated work and create deadlocks or long transactions. Prefer intent records, outbox patterns, and reconciliation.

### Treating HTTP Method Names As Idempotency

`PUT` is not automatically safe, and `POST` is not automatically unsafe. The implementation determines behavior. A `PUT` can still double-send an email. A `POST` can be idempotent with the right key.

### Forgetting Human Double Actions

Users double-click, open two tabs, refresh, go back, and submit again. Product flows need duplicate-safe server behavior, not only disabled buttons.

## Self-Check

- [ ] The operation's unit of correctness and "happens once" invariant are explicit.
- [ ] Duplicate client requests, retries, worker crashes, webhook redelivery, and human double actions are safe.
- [ ] Idempotency keys or durable deduplication records are scoped correctly and return consistent results.
- [ ] Critical uniqueness and state transition rules are enforced with database constraints, conditional updates, row locks, or version checks.
- [ ] Transactions include all required database changes but avoid unnecessary external network calls.
- [ ] Lock scope, timeout, failure behavior, process boundary, and crash behavior are deliberate.
- [ ] External side effects have provider ids, status records, retry policy, and reconciliation behavior.
- [ ] Ordering-sensitive workflows use versions, sequences, transition guards, or deterministic tie-breakers.
- [ ] Caches, queues, scheduled jobs, and multiple service instances were included in the race analysis.
- [ ] Tests cover duplicate requests, concurrent execution, stale updates, conflict handling, and retry paths.
