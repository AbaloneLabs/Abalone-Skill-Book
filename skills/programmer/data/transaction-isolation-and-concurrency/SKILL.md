---
name: transaction_isolation_and_concurrency.md
description: Use when the agent is choosing transaction isolation levels, reasoning about dirty reads and phantom reads and repeatable reads, understanding MVCC, selecting lock strategies, preventing deadlocks, or choosing optimistic versus pessimistic concurrency control.
---

# Transaction Isolation and Concurrency

Concurrency bugs are the most insidious class of database defect because they rarely reproduce in development and almost never surface in unit tests. Two requests that each work perfectly in isolation can corrupt data when they overlap in production, and the symptoms (a duplicated charge, an overdrawn balance, a lost update) appear long after the code shipped. The default isolation level of most databases is weaker than developers assume, and writing correct concurrent code requires understanding exactly what guarantees the database does and does not provide.

The judgment problem is choosing an isolation level that matches the correctness requirement without paying for more locking than necessary, understanding the specific anomalies each level permits, picking a concurrency-control strategy (optimistic vs pessimistic) that fits the contention profile, and structuring transactions to minimize deadlock risk. The agent should not treat "wrap it in a transaction" as sufficient; a transaction with the wrong isolation level can still produce wrong results.

This skill applies whenever you are writing code that reads and writes shared state concurrently, designing workflows that must be atomic, or diagnosing data anomalies that appear only under load.

## Core Rules

### Know what each isolation level actually guarantees

Isolation levels are defined by which anomalies they prevent, and the names are misleading:

- **Read Uncommitted**: permits dirty reads (reading uncommitted data from another transaction). Almost never appropriate.
- **Read Committed** (the default on most databases): prevents dirty reads, but permits non-repeatable reads (a row re-read in the same transaction may have changed) and phantom reads (a range re-query may return new rows inserted by another transaction).
- **Repeatable Read**: prevents non-repeatable reads; phantom behavior is database-dependent (the SQL standard permits phantoms; some databases like PostgreSQL/MySQL with MVCC prevent them, but still permit write skew under snapshot isolation).
- **Serializable**: prevents all anomalies, including write skew, typically at the cost of more locking or restarts.

The critical realization is that Read Committed (the common default) does not guarantee that two reads of the same data within a transaction return the same value, and does not prevent two concurrent transactions from both reading the same initial state and then both writing conflicting updates (the lost update anomaly). If your logic depends on a stable view of the data, the default is insufficient.

### Match the isolation level to the correctness requirement

Choose the weakest level that provides the guarantees your logic needs, because stronger levels cost more (locking, restarts, reduced throughput):

- For a simple read-modify-write of a single row where you need to avoid lost updates, use `SELECT ... FOR UPDATE` (pessimistic row lock) at Read Committed, or compare-and-update with a version column, rather than escalating to Serializable.
- For logic that reads multiple rows and makes a decision based on their aggregate state (e.g., "allow withdrawal if total balance across accounts > 0"), you need protection against write skew, which requires Serializable or explicit locking/select-for-update on the read set.
- For reporting queries that need a consistent snapshot, use a single transaction with Repeatable Read (or a defined snapshot), accepting that the snapshot may be slightly stale.

Weak choice: defaulting to Read Committed everywhere and assuming the transaction provides consistency. Strong choice: explicitly choosing the level per transaction based on the anomaly it must prevent.

### Understand MVCC and snapshot isolation

Most modern databases use Multi-Version Concurrency Control: readers see a consistent snapshot of data as of the transaction start, and writers create new versions rather than overwriting in place. This means readers do not block writers and vice versa, which is great for concurrency. The subtlety is that snapshot isolation (the basis of Repeatable Read in many databases) prevents many anomalies but still permits write skew: two transactions each read a snapshot, each verifies a constraint against that snapshot, and each writes a change that together violates the constraint (e.g., two on-call doctors both going off-duty when the rule requires one to remain). Serializable isolation closes this gap, usually via predicate locking or serialization-rate monitoring with retries.

### Choose optimistic vs pessimistic concurrency control by contention profile

- **Pessimistic** (`SELECT FOR UPDATE`, explicit locks): the transaction assumes conflict and locks data up front. Best when contention is high and conflicts are likely, because retrying is expensive. The risk is deadlocks and reduced concurrency.
- **Optimistic** (version columns, compare-and-set, or serializable-with-retry): the transaction proceeds without locking and validates at commit time, retrying on conflict. Best when contention is low, because it avoids locking overhead and deadlocks. The risk is that under high contention, retry storms waste work and can livelock.

The trap is choosing by familiarity rather than by the actual contention profile. A low-contention system with pessimistic locking pays unnecessary lock overhead; a high-contention system with optimistic control spends most of its CPU on retries.

### Prevent deadlocks by acquiring locks in a consistent order

Deadlocks occur when two transactions each hold a lock the other needs. Most databases detect and abort one transaction (a serialization/deadlock error), which the application must handle by retrying. To minimize deadlocks:

- Acquire locks (via `SELECT FOR UPDATE` or row updates) in a consistent, deterministic order across all transactions (e.g., always order multi-row updates by primary key).
- Keep transactions short to shrink the window during which locks are held.
- Avoid user interaction or external calls (HTTP, queues) inside a transaction, which hold locks open for unbounded time.

Always handle the deadlock/serialization-failure error code by retrying the transaction (with a bounded retry count), because the database has already determined the transaction cannot commit.

### Bound transaction scope and resource holding

Long-running transactions hold locks, prevent vacuuming/compaction, and increase the chance of conflicts. Keep transactions as short as possible: do the work, commit, release. Never hold a transaction open across a user think-time, an external API call, or a long computation. If you need atomicity across an external system, use an outbox pattern or a saga rather than holding a database transaction open.

### Decide explicit locking vs application-level coordination

Some concurrency problems are better solved outside the database: distributed locks (for cross-process coordination), idempotency keys (for exactly-once request semantics), or application-level versioning. Do not force every coordination problem into database transactions; conversely, do not reinvent transactional guarantees in application code when the database already provides them.

## Common Traps

### Assuming the default isolation level is sufficient

Read Committed is the default and is weaker than most developers assume. Code that reads a value, decides based on it, and writes can produce wrong results under concurrent access. The trap is never checking the level and assuming "it's in a transaction" means "it's safe."

### Lost updates at Read Committed

Two transactions both read balance=100, both compute 100-50=50, both write 50; one withdrawal is lost. This is the classic lost-update anomaly and is permitted at Read Committed unless you use `FOR UPDATE`, a version check, or a higher isolation level. It is extremely common in hand-written read-modify-write code.

### Write skew under snapshot isolation

Two transactions each read a snapshot, verify a constraint, and each write a change that is individually valid but jointly violates the constraint. This is invisible at Repeatable Read in MVCC databases and requires Serializable or explicit locking. The trap is assuming Repeatable Read prevents all anomalies.

### Holding transactions open across external calls

A transaction that awaits an HTTP response holds locks and bloats the transaction log. Under load, this causes lock contention, connection exhaustion, and vacuum bloat. External calls belong outside the transaction, coordinated via an outbox or saga.

### Inconsistent lock ordering causing deadlocks

If transaction A locks rows 1 then 2, and transaction B locks rows 2 then 1, they can deadlock. The trap is writing multi-row updates without a deterministic order. Order by primary key everywhere to make lock acquisition deterministic.

### Not retrying on serialization/deadlock errors

When the database aborts a transaction due to a deadlock or serialization failure, it expects the application to retry. Treating the error as fatal (returning a 500 to the user) is incorrect; the transaction was valid, just unlucky. Always retry with a bounded count and backoff.

### Confusing application-level uniqueness with database constraints

Checking "does this row exist?" then inserting is not safe under concurrency; two requests can both see no row and both insert. Use a unique constraint and handle the duplicate-key error, rather than relying on a check-then-insert race.

## Self-Check

- Is the isolation level chosen explicitly per transaction based on the anomaly it must prevent, rather than relying on the default?
- For every read-modify-write, is a lost update prevented via `FOR UPDATE`, a version column, or a higher isolation level?
- For logic that decides based on aggregate state across rows, is write skew prevented via Serializable isolation or explicit locking of the read set?
- Are locks acquired in a consistent, deterministic order (e.g., by primary key) across all transactions that touch the same data?
- Are transactions kept short, with no external calls or user think-time held inside them?
- Are serialization-failure and deadlock errors handled by retrying the transaction with a bounded count and backoff?
- Have you chosen optimistic vs pessimistic control based on the actual contention profile, not familiarity?
- Are uniqueness guarantees enforced by database constraints rather than check-then-insert application logic?
- For cross-system atomicity, are you using an outbox or saga rather than holding a database transaction open?
- Have you tested the concurrent paths (not just sequential execution) to confirm the chosen isolation level prevents the target anomalies?
