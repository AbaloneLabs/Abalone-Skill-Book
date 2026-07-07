---
name: goroutine_and_channel_patterns.md
description: Use when the agent is writing or reviewing Go code that launches goroutines, uses channels (unbuffered, buffered, select, closing), sync primitives (Mutex, WaitGroup, Once, atomic), context for cancellation, worker pools, fan-out/fan-in, pipelines, or any background or concurrent work. Also use when diagnosing goroutine leaks, deadlocks, data races reported by the race detector, channel send/receive blocks, panics on closed channels, or context propagation gaps. Covers goroutine lifecycle, ownership, channel direction, back-pressure, and the discipline of "do not communicate by sharing memory; share memory by communicating."
---

# Goroutine And Channel Patterns

Go's concurrency primitives look small — `go f()`, `make(chan T)`, `select`, `sync.Mutex` — and that apparent simplicity is where most Go concurrency bugs are born. A goroutine is nearly free to start and almost impossible to reliably stop once it has escaped its caller. A channel is a typed queue with blocking semantics, and whether a send or receive blocks depends on buffer size, the presence of a receiver, and whether someone closes the channel — facts that are invisible at the call site. The race detector will catch many data races, but it cannot catch a goroutine that never exits, a channel that is closed twice, a `select` that starves one case, or a pipeline whose final stage blocks forever because nobody drained it.

The judgment problem is not "how do I write a channel" but "who owns this goroutine's lifecycle, who is allowed to close this channel, what guarantees that every send has a matching receive, and what cancels work when the caller gives up." Agents trained on threads-and-locks often reach for `sync.Mutex` to protect shared maps when an owner-goroutine plus a single channel would express the same invariant far more safely, while agents trained on async/await often treat goroutines like fire-and-forget tasks and leak them by the thousand. Both mistakes compile, pass unit tests, and surface only under load or shutdown.

## Core Rules

### Know The Lifecycle Of Every Goroutine You Start

Every `go` statement creates a goroutine that the runtime will keep alive until the function returns. There is no parent/child relationship, no automatic cancellation when the caller returns, and no GC of a blocked goroutine. Before writing `go something()`, answer three questions: what causes `something` to return, who waits for that, and what happens if the caller that started it has already gone away. If you cannot answer all three, the goroutine is a leak waiting to happen.

The dominant pattern is to tie every goroutine to a `context.Context`. The goroutine selects on `ctx.Done()` and returns when the context is cancelled, and the function that started the goroutine either waits for it (via a `sync.WaitGroup`, a done-channel, or an `errgroup.Group`) or documents that it is fire-and-forget with a bounded lifetime. Long-lived goroutines (servers, workers, watchers) should have an explicit Stop/Shutdown method that cancels the context and waits for the goroutine to actually exit before the process tears down. A goroutine that blocks on a channel receive with no `ctx.Done()` case is a goroutine that can hang forever.

### Make Goroutine Lifetimes Bounded And Observable

Unbounded goroutine creation is a resource leak dressed up as concurrency. Spawning a goroutine per request, per incoming message, or per row is fine only if the work is short and the arrival rate is bounded; otherwise you need a worker pool with a fixed number of goroutines reading from a buffered queue, plus back-pressure so the queue cannot grow without limit. `errgroup` with `SetLimit` is a good default for bounded fan-out.

Observability matters because a leaked goroutine is silent. In production, expose goroutine counts (`runtime.NumGoroutine`), and during tests use `goleak` or a shutdown wait to assert that goroutines return. A process whose goroutine count climbs monotonically with load and never recovers has a leak, even if every individual goroutine looks correct in isolation.

### Assign Every Channel A Single Owner Who Is Allowed To Close It

The most dangerous channel operation is `close`. Closing an already-closed channel panics; sending on a closed channel panics; these panics are not recoverable in any useful sense. The safe discipline is single-ownership: exactly one goroutine, the producer, is allowed to close a channel, and it closes it exactly once when there is no more data. Consumers never close. If multiple producers feed one channel, neither producer may close it directly — instead route through an owner goroutine (or a `sync.Once`) that closes only after all producers signal completion.

Directional channel types enforce this at compile time: declare function parameters as `chan<- T` (send-only) for producers and `<-chan T` (receive-only) for consumers. A consumer that only has a receive-only channel cannot close it by construction. This is the single most effective Go idiom for eliminating "send on closed channel" panics, and it costs nothing.

### Match Channel Buffering To The Back-Pressure You Actually Want

An unbuffered channel is a synchronization point: send blocks until receive, receive blocks until send. A buffered channel decouples them up to the buffer size, then re-imposes blocking. The buffer size is a back-pressure decision, not a performance knob. A buffer of zero means "the consumer sets the pace"; a buffer of N means "the producer may get N ahead"; an unbounded buffer (or a large buffer hiding a slow consumer) means "memory grows until the process dies."

Choose the buffer deliberately and name the reason. For pipelines, a small buffer (1 to a few) is usually right and avoids the latency penalty of strict handoff while keeping memory bounded. Never use a large buffer to "fix" a producer that outruns its consumer — that only delays the OOM. If the consumer is genuinely slower, throttle the producer (a semaphore, `errgroup.SetLimit`, a demand signal) rather than buffering the difference.

### Use `select` Deliberately, And Always Consider Cancellation

`select` lets a goroutine wait on multiple channel operations and is the backbone of cancellation-aware code. Two rules dominate. First, every blocking `select` in a long-lived goroutine should include a `case <-ctx.Done()` so the goroutine can be cancelled; a select with no cancellation case can block forever and is a leak. Second, understand the semantics: a `select` with multiple ready cases picks one pseudo-randomly, which is fair but means you cannot rely on order; a `select` with a `default` case is non-blocking and turns the select into a "try" operation, useful for dropping work under load but easy to misuse (silently dropping correctness-critical messages).

Be careful with `nil` channels in a select: a `nil` channel is never ready, so a select case on a nil channel is effectively disabled. This is occasionally useful (dynamically enabling/disabling cases) but is a common source of "why does my select never fire" bugs when a channel was never initialized.

### Prefer "Share Memory By Communicating" For State That Has An Owner

Go's slogan is "do not communicate by sharing memory; share memory by communicating." The practical reading: when some state has a natural single owner and a stream of operations against it, model the owner as a goroutine that holds the state as a local variable and receives operations on a channel, replying on a response channel. This serializes access by construction — there is no lock because only one goroutine ever touches the state — and the type system can express the request/response shape.

This is not a universal rule. For simple shared counters, flags, or caches, `sync/atomic` or `sync.RWMutex` is clearer and faster than a goroutine-plus-channel. For read-heavy state with rare writes, `sync.RWMutex` or `sync.Map` is appropriate. The trap is reaching for a mutex around a large, complex, operation-bearing struct when an owner goroutine would express the same invariant without a lock — and equally, building an elaborate channel protocol around a single integer that an `atomic.Int64` would handle.

### Use `sync` Primitives Correctly, And Copy-Check Every Struct That Contains One

`sync.Mutex`, `sync.WaitGroup`, and `sync.Once` must not be copied after first use — copying them duplicates their internal state and silently breaks mutual exclusion or completion tracking. This means any struct that embeds a `sync` primitive must be passed by pointer, never by value. `go vet` catches the most obvious cases; do not rely on it catching all of them. If you find yourself wanting to copy such a struct, the design is wrong — store it behind a pointer.

Match the primitive to the job: `sync.Once` for one-time init (never hand-roll double-checked locking), `sync.WaitGroup` for waiting on a known set of goroutines (always pair `Add` before the goroutine starts with `Done` inside it), `sync.Mutex`/`RWMutex` for short critical sections (never hold a lock across a channel op, an HTTP call, or any blocking work), and `sync/atomic` for single-word state where a lock would be overkill. Holding a `Mutex` across a blocking call serializes every waiter behind that block and is a leading cause of latency collapse.

### Propagate Context Everywhere, And Respect Its Cancellation

`context.Context` is the standard cancellation and deadline propagation mechanism. The rule is mechanical but widely violated: the first parameter of every function that does I/O, blocking work, or starts goroutines should be `ctx context.Context`, and the function should select on `ctx.Done()` at every blocking point. A function that takes a context and ignores it is a lie — callers will assume cancellation works and it will not.

Do not store contexts inside structs or pass them as fields; contexts are meant to flow along the call stack, not live as object state. Do not create a fresh `context.Background()` in the middle of a request to "escape" cancellation — that defeats the entire mechanism and turns a cancelled request into a runaway goroutine. If you genuinely need work to outlive the request, do it explicitly: spawn a detached task with its own bounded context and lifecycle, and document why.

## Common Traps

### Starting A Goroutine And Forgetting To Wait For Or Cancel It

`go process(item)` inside a loop, with no WaitGroup, no context, and no completion signal. Under load this leaks goroutines; on shutdown they are killed mid-work. Always pair a started goroutine with a mechanism that waits for or cancels it.

### Sending On A Channel That Might Be Closed

A producer that calls `close(ch)` and a consumer (or another producer) that later sends on `ch` panics at runtime. Close is single-owner, single-call. If you are unsure whether a channel is closed, you do not own it and must not close it; restructure so ownership is explicit.

### Closing A Channel From The Consumer Side

Consumers closing a channel to signal "stop" is a classic panic source because the producer may still be sending. Signal "stop" with a separate done-channel or context cancellation, never by closing the data channel from the receiver.

### Loop Variable Capture (Pre-Go-1.22 Semantics)

In Go versions before 1.22, `for _, v := range items { go func() { use(v) }() }` shares a single `v` across all goroutines. Pass `v` as a parameter (`go func(v T) { ... }(v)`) or rely on 1.22+ per-iteration semantics — but verify the module's Go version before assuming the fix.

### Assuming `range` Over A Channel Exits Cleanly

`for v := range ch` exits only when the channel is closed and drained. If nobody closes the channel, the range blocks forever and the goroutine leaks. Pair every channel range with a documented closer.

### Using A Large Buffer To Mask A Slow Consumer

A buffered channel of 10000 "to be safe" hides a producer/consumer rate mismatch and converts a latency problem into an OOM. Size buffers deliberately; if the consumer is slow, throttle the producer.

### Holding A Mutex Across A Blocking Call

Locking a `sync.Mutex`, then doing an HTTP call, a channel send, or any I/O while holding it, serializes every other goroutine behind that call. Keep critical sections to pure memory operations; release before blocking.

### Treating `sync.Map` As A Drop-In For `map`

`sync.Map` is optimized for a specific pattern (append-only keys, stable values, many readers). For general-purpose concurrent maps it is slower and clumsier than a regular map guarded by an `RWMutex`. Do not reach for it by default.

### Ignoring The Race Detector

The race detector (`-race`) catches most data races at the cost of some runtime overhead. A test suite that has never been run with `-race` has not been tested for concurrency correctness. Run it in CI; treat every report as a real bug, even if it "never reproduces."

## Self-Check

- [ ] Every `go` statement has a documented answer for what makes the goroutine return, who waits for or cancels it, and what happens if the caller has already returned.
- [ ] Long-lived goroutines select on `ctx.Done()` (or an equivalent done-channel) at every blocking point, and shutdown cancels the context and waits for the goroutine to actually exit.
- [ ] Goroutine creation is bounded (worker pool, `errgroup.SetLimit`, semaphore) when the arrival rate is unbounded; goroutine counts are observable in production and asserted (e.g. `goleak`) in tests.
- [ ] Every channel has a single owner responsible for closing it exactly once; producers and consumers are typed with `chan<- T` / `<-chan T` to make direction and close-ownership compile-time enforced.
- [ ] Channel buffer sizes are chosen deliberately as back-pressure decisions, not as "large to be safe," and no buffer hides a producer that outruns its consumer.
- [ ] Every blocking `select` in a long-lived goroutine includes a cancellation case (`ctx.Done()`), and `nil` channels in selects are intentional and documented.
- [ ] Shared state with a natural owner is modeled as an owner goroutine plus channels; simple counters/flags use `sync/atomic`; read-heavy maps use `RWMutex` or a documented reason for `sync.Map`.
- [ ] Structs embedding `sync` primitives are always passed by pointer (never copied), critical sections contain only memory operations, and `sync.Once`/`WaitGroup` are used for their intended one-time and wait-for-set roles.
- [ ] `context.Context` is the first parameter of every function doing I/O or blocking work, is respected at every blocking point, is never stored in a struct as long-lived state, and is never replaced with `context.Background()` to escape cancellation without an explicit documented reason.
- [ ] The code has been run under `-race` and every reported race is fixed, not suppressed; loop-variable capture and closed-channel send paths have been reviewed for the module's Go version.
