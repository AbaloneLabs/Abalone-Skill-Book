---
name: haskell_concurrency_and_stm.md
description: Use when the agent is writing concurrent or parallel Haskell (forkIO, async, MVar, TVar and STM transactions, Chan/TQueue/TChan, Software Transactional Memory atomically/retry/orElse), choosing between MVar and STM/TVar, designing concurrent data structures, handling exceptions in threads (async exceptions, bracket, mask), using Software Transactional Memory for composable atomicity, parallelism with par/pseq/Strategies/Eval, or is diagnosing "thread leaked / never terminates", "MVar deadlock", "STM retry livelock", "exception kills a thread silently", or concurrency-design issues. Covers forkIO/async, MVar vs STM/TVar, STM composition (retry/orElse), exception handling in threads (async exceptions, bracket/mask), parallel vs concurrent, and the traps of leaks, deadlocks, silent exceptions, and retry storms.
---

# Concurrency And STM In Haskell

Haskell's GHC runtime has a lightweight-thread (green-thread) model: `forkIO` spawns a Haskell thread (cheap, millions possible) scheduled across OS threads (capabilities). The synchronization primitives are `MVar` (a single-slot channel/lock) and STM (Software Transactional Memory, via `TVar`/`atomically`/`retry`/`orElse`). Agents leak threads (a `forkIO`'d thread that never terminates, holding resources), deadlock on `MVar`s (taking them in inconsistent orders), misuse STM `retry` (a transaction that always retries livelocks), let exceptions kill threads silently (an unhandled exception in a `forkIO`'d thread kills it with no signal to the parent), or conflate concurrency (threads, nondeterministic interleaving) with parallelism (multi-core speedup via `par`/`Strategies`). The judgment problem is to manage thread lifetimes (with `async`/`withAsync`), to choose `MVar` vs STM by composition needs, to handle exceptions across threads, and to use STM for composable atomicity.

Agents leak threads, deadlock `MVar`s, livelock STM, or silently lose threads to exceptions. The remedy is `withAsync` for lifetime management, STM for composability, explicit exception handling, and the right primitive for the job.

## Core Rules

### Manage Thread Lifetimes With async / withAsync, Not Bare forkIO

A bare `forkIO` spawns a thread with no link to the parent — it can run forever, leak resources, and die silently. Use the `async` library: `withAsync action $ \a -> ...` spawns a thread tied to a scope (the thread is killed when the scope exits, no leak), `wait a`/`waitCatch a` retrieves the result (or exception), `race a b`/`concurrently a b` compose threads (first-to-finish / both), and `cancel a` cancels. `withAsync` (the scoped form) is preferred over `async` (which needs explicit `cancel`/`wait`) to guarantee cleanup. For long-running background workers, structure them under a supervisor (`async` + `waitCatch` + restart) rather than fire-and-forget `forkIO`.

- Use `async`/`withAsync` (scoped, no leak) over bare `forkIO` (leaks, dies silently).
- `wait`/`waitCatch` to retrieve result/exception; `race`/`concurrently` to compose.
- `withAsync` ties the thread to a scope (auto-killed on exit); supervise long-running workers.

### Choose MVar vs STM/TVar By Composition Needs

- `MVar a`: a single-slot mutable cell (empty/full), usable as a lock, a one-place channel, or a container. Simple, fast for single-resource coordination, but composing multiple `MVar`s atomically is impossible (no multi-`MVar` transaction) — you risk deadlock taking them in inconsistent orders.
- STM (`TVar`/`TChan`/`TQueue` + `atomically`): composable atomic transactions. Multiple `TVar`s read/written in one `atomically` block commit all-or-nothing, with `retry` to block until a condition holds and `orElse` to compose alternatives. No deadlock (the runtime manages it), composable, and the right choice when coordinating multiple shared variables or needing atomicity across them.

Use `MVar` for simple single-resource coordination (a lock, a one-shot result); use STM for anything requiring atomicity across multiple variables or composability. Default to STM for shared concurrent state; `MVar` for simple cases.

- `MVar`: simple, single-resource (lock/one-place channel); no atomic multi-`MVar` composition (deadlock risk).
- STM (`TVar`/`atomically`): composable atomic transactions, `retry`/`orElse`, no deadlock. Default for shared state.
- Use `MVar` for simple cases; STM for atomicity/composition across multiple variables.

### Use STM's retry And orElse Deliberately

`atomically $ do { v <- readTVar t; if v < 0 then retry else ... }` — `retry` blocks the transaction until a `TVar` it read changes, then re-runs it (no busy-wait, the runtime wakes it on change). `orElse` composes alternatives (`atomically (left `orElse` right)` tries `left`, and if it `retry`s, tries `right`). Traps: a transaction that unconditionally `retry`s (the condition never becomes true) livelocks/blocks forever — ensure the `retry` condition is reachable. A transaction with side effects (a `TVar` write that should be observed) commits only on success; do not put `IO` inside `atomically` (STM is pure — use `TVar`/`TQueue`, not `IO`). Keep transactions short (long transactions retry more, reducing throughput).

- `retry` blocks until a read `TVar` changes (no busy-wait); ensure the condition is reachable (no livelock).
- `orElse` composes alternatives; transactions are all-or-nothing.
- No `IO` inside `atomically` (STM is pure); keep transactions short.

### Handle Exceptions Across Threads (async Exceptions, bracket, mask)

Exceptions in a `forkIO`'d thread kill that thread silently (the parent gets no signal) unless handled. Use `async`/`waitCatch` to surface the exception to the parent, or `bracket`/`finally` within the thread for cleanup. Haskell distinguishes *synchronous* exceptions (thrown by the thread's own actions, e.g., `throwIO`) from *asynchronous* exceptions (thrown *to* a thread by another, e.g., `killThread`/`cancel`/timeout). `bracket`/`mask`/`uninterruptibleMask` protect critical regions from async exceptions during cleanup. Use `timeout` (which sends an async exception) to bound long operations. In STM, exceptions propagate from `atomically`. Always clean up resources (`bracket`) so a killed thread does not leak.

- Exceptions in `forkIO` kill the thread silently; use `async`/`waitCatch` or handle within.
- Synchronous (own action) vs asynchronous (sent to the thread); `bracket`/`mask` protect cleanup.
- `timeout`/`cancel` send async exceptions; clean up resources with `bracket` to avoid leaks.

### Distinguish Concurrency (Threads) From Parallelism (Multi-Core Speedup)

Concurrency is about structuring a program as independent threads (for responsiveness, I/O overlap, modularity) — `forkIO`/`async`/STM, scheduled nondeterministically. Parallelism is about using multiple cores to speed up a computation — `par`/`pseq`/`Eval`/`Strategies` (`rpar`, `rseq`, `rparWith`), or `Control.Concurrent.ParallelIO`. A concurrent program need not be parallel (single-core, interleaved); a parallel program need not be concurrent (pure `par` sparks). Choose by goal: threads for I/O overlap/structure; `par`/`Strategies` for CPU speedup. Measure parallel speedup with `+RTS -N` (cores) and the eventlog/ThreadScope; sparks that fizzle (never evaluated) indicate wasted parallelism.

- Concurrency (threads/STM) for structure/I/O overlap; Parallelism (`par`/`Strategies`) for multi-core speedup.
- A concurrent program need not be parallel; a parallel program need not be concurrent.
- Measure parallel speedup (`+RTS -N`, ThreadScope); fizzled sparks = wasted parallelism.

## Common Traps

### Thread Leak From Bare forkIO

A `forkIO`'d thread never terminates, holding resources. Use `withAsync`/supervision.

### MVar Deadlock

Taking multiple `MVar`s in inconsistent orders deadlocks. Use STM for atomic multi-variable coordination.

### STM retry Livelock

A `retry` whose condition is never true blocks forever. Ensure reachability.

### Silent Thread Death From Exceptions

An unhandled exception kills a `forkIO` thread with no signal. Use `async`/`waitCatch`/`bracket`.

### IO Inside atomically

STM is pure; `IO` in `atomically` is a type error (or smuggled unsafely). Use `TVar`/`TQueue`.

### Conflating Concurrency And Parallelism

Threads for structure, `par` for speedup. Match the tool to the goal.

### Long STM Transactions

Long transactions retry more, reducing throughput. Keep transactions short.

### Wasted Parallel Sparks

`par` sparks that fizzle (never used) waste effort. Measure with ThreadScope; tune granularity.

## Self-Check

- [ ] Threads are managed with `async`/`withAsync` (scoped, no leak, exception-surfaced), not bare `forkIO`; long-running workers are supervised.
- [ ] `MVar` is used for simple single-resource coordination; STM (`TVar`/`atomically`) is the default for shared state requiring atomicity/composition across variables.
- [ ] STM `retry` conditions are reachable (no livelock); `orElse` composes alternatives; no `IO` inside `atomically`; transactions are short.
- [ ] Exceptions across threads are handled (`async`/`waitCatch`/`bracket`); synchronous vs asynchronous exceptions are understood; `bracket`/`mask` protect cleanup; resources are cleaned up on thread death.
- [ ] Concurrency (threads/STM for structure/I/O overlap) is distinguished from parallelism (`par`/`Strategies` for multi-core speedup); the tool matches the goal.
- [ ] Parallelism is measured (`+RTS -N`, ThreadScope); spark granularity is tuned to avoid fizzled sparks.
- [ ] `MVar` multi-lock deadlocks are avoided (STM for multi-variable atomicity); `timeout`/`cancel` are used to bound operations.
- [ ] The concurrent/parallel design has been considered under leaks, deadlocks, livelocks, silent exceptions, and parallel efficiency, and remains correct and performant.
