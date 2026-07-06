---
name: process_and_message_passing.md
description: Use when the agent is writing Elixir or Erlang/BEAM code that spawns processes, uses send/receive, links or monitors processes, stores state in a GenServer or Agent, uses the process dictionary, builds registries or process pools, or debugs message box overflow, selective receive stalls, memory leaks from unprocessed messages, zombie processes, or crashes that propagate unexpectedly. Covers BEAM process isolation, message delivery semantics, links vs monitors, and the hazards of unbounded mailboxes and hidden shared state.
---

# Process And Message Passing

BEAM processes are the central abstraction of Elixir and Erlang: lightweight, isolated, garbage-collected independently, and communicating only by message passing. This model is the source of the platform's legendary fault tolerance and concurrency, and it is also the source of its most characteristic bugs — message boxes that grow until the VM runs out of memory, `receive` blocks that never match a message that has been sitting in the box for minutes, processes that crash and take their linked neighbors down because nobody expected the link to propagate, and state that quietly rots in a long-lived GenServer because no one audited it. The difficulty is that the model looks trivial (`spawn`, `send`, `receive`) while the failure modes are all about what happens at the boundaries: when messages arrive faster than they are processed, when a process dies between send and receive, when state lives forever, and when isolation is violated by the process dictionary or shared ETS tables.

Agents trained on shared-memory concurrency tend to treat BEAM processes as "threads with a fancier API" and miss that the isolation is the whole point. The judgment problem is deciding when something should be a process (it has its own failure, lifecycle, or state), how to keep message flow bounded so a slow consumer cannot exhaust a producer's memory, how to observe process death without coupling lifecycles accidentally, and how to avoid smuggling shared mutable state back in through the process dictionary or ETS. Getting this wrong produces systems that pass load tests and degrade catastrophically the day a consumer stalls or a process leaks.

## Core Rules

### Make A Process Only When There Is A Reason To Be A Process

A BEAM process is the right unit when one of these holds: the thing has independent failure semantics (it can crash without taking the rest down), it owns state that should be serialized through a single mailbox, it has a distinct lifecycle (start, run, stop), or it should be scheduled independently. It is the wrong unit for "I want to run some code" (use a Task), "I want to share a variable" (use a GenServer or pass it explicitly), or "I want it to be concurrent" (concurrency is the default; processes are about isolation and ownership).

Over-spawning is a real failure mode. Every process has a memory footprint (initial heap, mailbox, stack), a scheduling cost, and a supervision expectation. A system that spawns a process per request without a bound leaks memory and overwhelms supervisors under load. Strong choice: one GenServer per long-lived entity with serialized state (a connection, a session, a device). Weak choice: spawning a bare process for every operation "for concurrency" and never linking or monitoring it.

### Bound The Mailbox, Because An Unprocessed Message Lives Forever

Every BEAM process has a mailbox, and a message that is sent but not matched stays in the mailbox indefinitely — there is no automatic expiry, and the mailbox grows without bound until the process matches or the VM runs out of memory. This is the single most important and most-violated property of the model. A GenServer that handles `:cast` messages faster than it can process them, a consumer that is slower than its producer, or a process blocked on a synchronous call while messages pile up all produce the same outcome: silent memory growth and eventual OOM.

Defend against this at the design level. Make producers aware of consumer pressure (back-pressure via a bounded queue, a `GenStage`/`Flow` demand model, or explicit flow control), prefer `:call` over `:cast` when the sender cares about the result (a `:call` blocks the sender, providing natural back-pressure), and instrument mailbox sizes in production (`Process.info(pid, :message_queue_len)`) so growth is visible before it is fatal. A process whose mailbox can grow without bound under any input is a defect; identify the bound or the back-pressure mechanism explicitly.

### Choose Links For Shared Fate And Monitors For Observation

Links and monitors both let one process react to another's death, but they propagate differently and serve different purposes. A link (`Process.link/1`) is bidirectional: if either process dies, the other dies too (unless it traps exits). Links express "these processes share fate" — a worker and its supervisor, two halves of a request. A monitor (`Process.monitor/1`) is unidirectional and observable: the monitoring process receives a `:DOWN` message when the monitored process dies, but the monitored process is unaffected. Monitors express "I want to know if that dies, without dying myself."

The trap is linking when you meant to monitor. Linking a client to a worker so the client "knows when the work is done" means a worker crash also crashes the client — often the opposite of what was intended. Use monitors for observation (a request waiting on a worker, a registry tracking members), and reserve links for genuine shared fate (a supervisor and its children, components that are meaningless alone). When a process must both observe and survive, trap exits (`Process.flag(:trap_exit, true)`) deliberately and handle `{:EXIT, pid, reason}` messages — but know that trapping exits changes the process's crash semantics, so do it for a reason.

### Handle The Race Between Send And Process Death

A `send` is asynchronous and fire-and-forget; the sender gets no confirmation that the message was received or processed, and if the destination process dies between the send and the receive, the message is silently lost. Messages to a dead PID vanish; messages to a registered name that no longer exists vanish. This is by design (it is what makes the model fault-tolerant), but it means any protocol that assumes delivery must build its own acknowledgment on top.

For correctness-critical delivery, use a synchronous `:call` (which raises or returns `{:error, ...}` if the callee dies), a monitor to detect death and retry, or an at-least-once pattern with idempotent handlers. Do not assume `send` delivered; assume it did not, and verify. A common bug is a fire-and-forget `:cast` to update state that is lost when the GenServer restarts, leaving the system inconsistent.

### Keep State Bounded And Audited In Long-Lived Processes

A GenServer or Agent that lives for the lifetime of the application accumulates state. If that state is a list, map, or ETS table that only grows (cached entries, connected clients, observed events), the process leaks memory slowly and indefinitely. Unlike a short-lived request process, a long-lived server never gets its memory reclaimed by dying.

Audit every long-lived process's state for growth. Ask, for each field: what adds to it, what removes it, and what is the steady-state size? If the answer is "it grows forever," there is a leak. Use TTLs, bounded caches (LRU), explicit cleanup on disconnect, and periodic compaction. A GenServer whose state map grows with every event is a memory leak dressed up as a server.

### Avoid The Process Dictionary; It Reintroduces Hidden Global State

The process dictionary (`Process.put/2`, `Process.get/1`) is a per-process key-value store that looks like a convenient way to stash context (request IDs, user identity) without threading it through every function. It is also a way to reintroduce hidden mutable global state into an otherwise functional codebase: a function's behavior now depends on `Process.get(:current_user)`, which is invisible in its signature, untestable without setup, and a source of subtle bugs when the dictionary is not populated.

Use the process dictionary only for genuinely per-process diagnostic context (request IDs for logging, telemetry) that does not affect correctness. For anything that affects behavior, pass it explicitly as a function argument or store it in the GenServer state. The convenience is not worth the loss of referential transparency and testability.

### Treat ETS As Shared State With The Same Discipline As Any Shared State

ETS (Erlang Term Storage) tables are shared mutable in-memory tables accessible from any process. They are essential for high-performance shared lookups (registries, caches), and they break the isolation that makes BEAM processes safe. Two processes writing to the same ETS table have a data race unless the operations are atomic, and a bug in one process can corrupt state visible to all others.

Own each ETS table in a single process (the table is destroyed when its owner dies, unless `heir` is set), use atomic operations (`:ets.insert_new`, `:ets.update_counter`) for concurrent writes, and treat the table as you would any shared mutable state in another language — with explicit concurrency control and a clear ownership contract. ETS is not magic; it is a shared database, with all the consistency concerns that implies.

## Common Traps

### Spawning Processes Without Linking Or Monitoring Them

`spawn`-ing a process and losing track of its PID, so when it crashes nobody knows and the work it was doing silently stops. Always spawn under a supervisor (for long-lived work) or a Task with supervision (for bounded work); never leave a process unobserved.

### Letting A Mailbox Grow Unbounded Under Load

A GenServer receiving `:cast` messages faster than it can process them, so the mailbox grows until the VM OOMs. `:cast` provides no back-pressure; prefer `:call` for correctness-critical messages, and use a demand-driven flow (`GenStage`, `Flow`) or a bounded queue when a producer can outrun a consumer.

### Linking When You Meant To Monitor

`Process.link/1`-ing a client to a worker so the client "knows when it finishes," then watching a worker crash take down the client too. Use `Process.monitor/1` for observation; reserve links for genuine shared fate. The two APIs look similar and behave oppositely on crash.

### Assuming `send` Delivered

Treating `send` as reliable delivery, then losing state updates when the destination process restarts between send and receive. `send` is fire-and-forget; build acknowledgment (`:call`, monitor-and-retry, idempotent handlers) when delivery matters.

### Growing State Forever In A Long-Lived GenServer

A GenServer whose state map accumulates an entry per event with no eviction, leaking memory for the lifetime of the application. Audit every state field for what adds, what removes, and the steady-state size; add TTLs, LRU bounds, or explicit cleanup.

### Using The Process Dictionary For Behavior-Affecting Context

Stashing `current_user` in the process dictionary so functions can read it implicitly, then producing bugs where a code path runs without the dictionary populated and behaves differently. Pass behavior-affecting context explicitly; reserve the dictionary for diagnostic-only context.

### Blocking A GenServer With A Synchronous Call To Itself Or Another Slow Server

A GenServer making a synchronous `:call` to another server while holding its own state, deadlocking (calling itself directly) or serializing badly (holding state while waiting on a slow downstream). GenServers should not block; offload slow work to a Task and reply asynchronously, and never `:call` yourself.

### Treating ETS As Magic Race-Free Storage and assuming `receive` Matches In Arrival Order Universally

Writing to an ETS table from multiple processes with non-atomic read-modify-write sequences, then seeing lost updates. Use atomic ETS operations (`:ets.update_counter`, `:ets.insert_new`) or single-writer ownership for any contested key.

Believing `receive` always processes the oldest message first, when in fact selective `receive` skips non-matching messages and can leave old messages stranded in the mailbox indefinitely. A selective receive that never matches a certain message type lets those messages accumulate forever.

## Self-Check

- [ ] Every process exists for a reason (independent failure, serialized state, distinct lifecycle, independent scheduling); processes are not spawned for "concurrency" without a bound or supervision.
- [ ] Every mailbox has a defined bound or back-pressure mechanism (demand-driven flow, bounded queue, `:call` instead of `:cast`), and mailbox length is instrumented in production so growth is visible before OOM.
- [ ] Links are used only for genuine shared fate (supervisor/child, meaningless-alone components); observation uses monitors, and any `trap_exit` is deliberate and its `{:EXIT, _, _}` messages are handled.
- [ ] Correctness-critical message delivery uses acknowledgment (`:call`, monitor-and-retry, idempotent handlers); no code assumes a `send` was received or processed.
- [ ] Every state field in a long-lived GenServer has a defined add path, remove path, and steady-state size; nothing grows forever without a TTL, LRU bound, or explicit cleanup.
- [ ] The process dictionary is used only for diagnostic context (request IDs, telemetry); behavior-affecting context is passed explicitly or stored in GenServer state.
- [ ] ETS tables have a single owner process, concurrent writes use atomic operations, and the table is treated as shared mutable state with explicit concurrency control.
- [ ] No GenServer blocks on a synchronous call (especially to itself); slow work is offloaded to a Task with asynchronous reply.
- [ ] A load test confirmed that a stalled or slow consumer does not exhaust producer memory, and that process crashes are observed (via monitor or supervisor) rather than silently lost.
