---
name: otp_supervision_tree.md
description: Use when the agent is designing an Elixir or Erlang/OTP supervision tree, choosing supervisor strategies (one_for_one, one_for_all, rest_for_one), writing child specs, configuring restart intensity and max_restarts, building DynamicSupervisor or Registry, implementing GenServer or Supervisor behaviour, or debugging supervisor restart loops, cascading crashes, let-it-crash failures, or systems that degrade catastrophically instead of gracefully. Covers restart semantics, child ordering, fault containment, and the limits of the let-it-crash philosophy.
---

# OTP Supervision Tree

The supervision tree is the load-bearing structure of an OTP application. It decides which processes restart when others crash, in what order, how often, and what happens when the restart budget is exhausted. Done well, it is the reason BEAM systems survive faults that would crash other runtimes: a worker dies, the supervisor restarts it, the system keeps serving. Done poorly, it is the reason BEAM systems fail catastrophically and silently — a single worker crash cascades through a `one_for_all` supervisor and takes down unrelated workers, a flapping process exhausts its restart budget and the whole application terminates, or a supervisor restarts a process that can never succeed and the system thrashes forever. "Let it crash" is a philosophy with hard limits, and the supervision tree is where those limits are enforced (or violated).

Agents tend to treat the supervisor as boilerplate — copy the default child spec, pick `one_for_one` because it is the least surprising, set `restart: :permanent` everywhere — and miss that every choice is a fault-propagation decision. The judgment problem is deciding, for each child and each sibling group, what should restart when what else crashes, whether a crash is recoverable by restart at all, and how to contain faults so that a problem in one subsystem degrades that subsystem rather than the whole application. The harm of getting this wrong is a system that passes functional tests and fails operationally: cascading restarts, supervisor death, and total application termination under conditions the functional tests never exercised.

## Core Rules

### Choose The Strategy Based On Sibling Independence, Not By Default

The supervisor strategy determines which siblings restart when one child crashes, and it is the most consequential supervision decision.

- **`one_for_one`** restarts only the crashed child. Correct when siblings are independent — a crash in the cache should not restart the database connection pool. The default and the most common choice, but only correct when independence is real.
- **`one_for_all`** restarts all children when any one crashes. Correct when siblings share state or invariants that the crash invalidates — a worker that holds a reference to a sibling's state, a group that must start together to be consistent. Dangerous when misapplied, because a crash in a non-critical child takes down critical siblings.
- **`rest_for_one`** restarts the crashed child and all children started after it. Correct when children have a dependency order — a later child depends on an earlier one, so the earlier one's crash invalidates the later ones, but not vice versa. Models startup-ordering dependencies as crash-propagation rules.

Name the dependency relationship between siblings before choosing. Strong choice: `rest_for_one` for a listener that depends on a connection pool that depends on a config server, so a config crash restarts everything that read the old config. Weak choice: `one_for_all` "to be safe," so a metrics-shipper crash takes down the request handler. The strategy is a fault-propagation contract; make it match the real dependencies.

### Classify Each Child's Restart Type Against Whether Restart Can Succeed

`restart: :permanent`, `:transient`, and `:temporary` decide whether the supervisor restarts a child after termination, and the choice should reflect whether a restart can plausibly produce a working process.

- **`:permanent`** — always restart, for any termination. Correct for long-lived infrastructure that should never stop (a database connection pool, a telemetry shipper). The assumption is that the crash was transient and the environment will support a restart.
- **`:transient`** — restart only if the process terminated abnormally (not `:normal` or `:shutdown`). Correct for processes that may legitimately finish (`Task`-like workers) but should restart on crash.
- **`:temporary`** — never restart. Correct for processes whose crash means restart would not help (a process that crashed because its configuration is invalid, or a worker that should not be auto-resurrected).

The trap is `:permanent` everywhere. A process that crashes because of a permanent condition (bad config, missing dependency, corrupt state on disk) will be restarted, crash again, restart, crash again — burning the restart budget and then taking the supervisor down. If a crash is not recoverable by restart, mark it `:temporary` or fix the root cause; do not let the supervisor thrash.

### Bound Restart Intensity To Prevent Thrashing And Supervisor Death

Every supervisor has a restart intensity (`max_restarts` within `max_seconds`). If a child (or its siblings, depending on strategy) restarts more than `max_restarts` times in `max_seconds`, the supervisor itself terminates — and if that supervisor has a parent, the parent must decide what to do, potentially cascading up to the application root. This is the mechanism that prevents infinite restart loops, and it is also the mechanism by which a flapping child can kill an entire application.

Set the intensity deliberately against the child's expected stability. A stable infrastructure process might warrant `3` restarts in `5` seconds (tight — flapping kills the supervisor fast, protecting the rest of the system). A process that legitimately restarts under load (a worker that crashes on bad input and recovers) might warrant a looser budget. The default (`3` in `5` seconds) is reasonable for many cases but should be a conscious choice, not an accident. Crucially, understand that exceeding the budget terminates the supervisor, which terminates its siblings (per strategy) and propagates to its parent — a small flap can take down a large subtree if the tree is shaped wrong.

### Order Children So Startup Dependencies Are Satisfied

Children start in declaration order and (for `:permanent`/`:transient`) are restarted in that order after a `rest_for_one` or `one_for_all` event. If child B depends on child A being up, A must be declared before B. A supervisor that starts a worker before its config server, connection pool, or registry will produce a worker that crashes on startup because its dependency is not ready — and then restarts, and crashes again, burning the budget.

Model the dependency order in the child list, and verify it by reading the supervisor top-to-bottom as a startup sequence. If a child cannot function without a later sibling, either reorder or restructure (move the dependency into a nested supervisor whose ordering expresses the relationship).

### Build A Tree, Not A Flat List, To Contain Faults

A flat supervisor with many children means a single restart-intensity breach takes down everything. A nested tree — supervisors supervising supervisors — contains faults to subtrees. A flapping worker in a leaf supervisor breaches that supervisor's budget and takes down only its subtree; the rest of the application survives. This is the central structural insight of OTP: the tree shape is the fault-containment boundary.

Group children by fault domain. A connection-pool supervisor, a request-handler supervisor, a background-job supervisor — each can fail independently without taking down the others. Strong choice: a per-subsystem supervisor under the application supervisor, so a subsystem's restart storm is contained. Weak choice: one supervisor with twenty children, so any child's flap can breach the shared budget and kill everything.

### Recognize The Limits Of Let-It-Crash

"Let it crash" means: do not defend against every error with `try/rescue`; let the process die and let the supervisor restart it, so the process starts from a known-clean state. It does not mean: every error is recoverable by restart. Crashes caused by permanent conditions (bad config, missing files, corrupt persisted state, programming bugs) will recur on every restart. Let-it-crash is appropriate for transient faults (a network blip, a malformed message from a client, a temporary resource shortage) and inappropriate for permanent ones.

For permanent-fault conditions, the correct response is not to crash-and-restart but to fail fast and loud (terminate the application with a clear error), to degrade gracefully (serve a reduced feature set), or to fix the root cause. A supervision tree that restarts a process doomed to crash again is not resilient; it is a thrashing system that will exhaust its budget and die. Distinguish transient from permanent faults, and design the restart policy accordingly.

### Make State Recovery Explicit, Because Restart Loses In-Memory State

When a process restarts, its in-memory state is gone. If the process held state that must survive restarts (a session, a counter, a connection mapping), that state must be recovered from durable storage (ETS with `heir`, disk, a database, a sibling process) on `init`. A GenServer that "works" because it accumulated state over time will lose that state on every restart, producing a system that appears functional in steady state and broken under faults.

For each stateful process, ask: what happens to its state on restart? If the answer is "it starts empty," verify that empty is acceptable. If not, persist the state or delegate it to a process that survives. Restart is a state-reset event; design for it explicitly.

### Use DynamicSupervisor For Bounded, Runtime-Created Children

`Supervisor` is for a fixed set of children known at startup. `DynamicSupervisor` is for children created at runtime (a process per connection, per session, per job). The discipline for `DynamicSupervisor` is bounding the count: an unbounded dynamic supervisor leaks processes under load exactly as an unbounded mailbox leaks messages. Pair it with a registry, a pool, or a concurrency limit so the child count has a ceiling.

## Common Traps

### Defaulting To `one_for_one` When Siblings Share State

Using `one_for_one` for siblings that share invariants, so a crash in one leaves the others in an inconsistent state. If siblings depend on each other, use `rest_for_one` (ordered dependency) or `one_for_all` (shared invariant); `one_for_one` assumes independence that may not exist.

### Using `one_for_all` "To Be Safe"

Choosing `one_for_all` so that "everything restarts together," then watching a non-critical child's crash take down critical siblings. `one_for_all` propagates faults broadly; use it only when siblings truly share fate, not as a conservative default.

### Marking Everything `:permanent` Including Unrecoverable Processes

Setting `restart: :permanent` on a process that crashes for a permanent reason (bad config, corrupt state), so the supervisor restarts it forever, exhausts the budget, and dies. If restart cannot succeed, use `:temporary` or fix the root cause.

### Ignoring Restart Intensity Until The Application Dies

Leaving the default intensity and discovering under load that a flapping child breached the budget and took down the supervisor (and, via the parent, the whole application). Set intensity deliberately per child's expected stability, and understand that a breach terminates the supervisor and propagates.

### Starting Children In The Wrong Order

Declaring a worker before its config server or connection pool, so the worker crashes on startup because its dependency is not ready, then restarts and crashes again. Order children by dependency; read the supervisor top-to-bottom as a startup sequence.

### A Flat Supervisor Where A Subtree Should Be

Putting many unrelated children under one supervisor, so any child's restart-budget breach kills the whole group. Group children by fault domain under nested supervisors, so a flap is contained to its subtree.

### Assuming Restart Recovers State

Letting a GenServer accumulate state in memory, then watching every restart reset it to empty and break the system under faults. Persist state that must survive restarts (ETS heir, disk, sibling process) and recover it in `init`.

### Let-It-Crash On A Permanent Fault and unbounded DynamicSupervisor

Crashing and restarting a process whose crash cause is permanent (missing file, invalid config, programming bug), producing an infinite thrash that exhausts the budget. Let-it-crash is for transient faults; permanent faults need fail-fast, degradation, or a fix.

Starting a process per request or connection under a `DynamicSupervisor` with no ceiling, leaking processes under load. Pair dynamic supervision with a registry, pool, or concurrency limit so the child count is bounded.

## Self-Check

- [ ] Each supervisor's strategy (`one_for_one`, `one_for_all`, `rest_for_one`) was chosen against the real dependency relationship between siblings, not by default; the choice is documented as a fault-propagation contract.
- [ ] Each child's restart type (`:permanent`, `:transient`, `:temporary`) reflects whether a restart can plausibly succeed; unrecoverable processes are not `:permanent`.
- [ ] Restart intensity (`max_restarts`/`max_seconds`) was set deliberately per child's expected stability, and the team understands that a breach terminates the supervisor and propagates to its parent.
- [ ] Children are ordered so startup dependencies are satisfied; reading the supervisor top-to-bottom is a valid startup sequence with no forward references.
- [ ] The tree is shaped by fault domain — nested supervisors per subsystem — so a flapping subtree is contained rather than taking down the whole application.
- [ ] Each stateful process has a defined state-recovery strategy on restart (persisted, delegated, or acceptably empty); no process silently loses state it needs.
- [ ] Let-it-crash is applied only to transient faults; permanent-fault conditions fail fast, degrade gracefully, or are fixed at the root rather than thrashed by restart.
- [ ] Any `DynamicSupervisor` is paired with a registry, pool, or concurrency limit so the child count has a ceiling under load.
- [ ] A fault-injection test (kill each child, kill each supervisor, breach a restart budget) confirmed that faults propagate as intended and do not cascade beyond their intended fault domain.
