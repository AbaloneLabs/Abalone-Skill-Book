---
name: kotlin_coroutines_and_structured_concurrency.md
description: Use when the agent is writing or reviewing Kotlin code using coroutines, suspend functions, CoroutineScope, Job and SupervisorJob, coroutine builders like launch and async, Dispatchers, structured concurrency, cancellation and cancellation propagation, integrating blocking calls, bridging callback and reactive APIs, or diagnosing coroutine leaks, lost cancellation, and dispatcher misuse in async Kotlin code.
---

# Coroutines And Structured Concurrency In Kotlin

Kotlin coroutines make asynchronous code look sequential, and that lightness is the source of both their appeal and their danger. A coroutine is not "a thread" or "a background task" in the familiar sense; it is a suspendable computation whose lifetime, dispatcher, and cancellation are governed by a structured-concurrency model built around `CoroutineScope` and `Job`. When you respect that model — launching coroutines in a scope, choosing the right dispatcher, making functions `suspend`, propagating cancellation cooperatively — concurrency becomes composable and leak-free. When you do not — using `GlobalScope`, blocking inside a coroutine, ignoring cancellation, losing the scope — you get coroutines that outlive their intended context, threads that block, work that cannot be cancelled, and leaks that are invisible because coroutines do not crash the way threads do.

Agents frequently treat coroutines as "lighter threads" and reach for `launch`/`async` without thinking about scope, dispatcher, or cancellation. The harm is subtle and severe: a `GlobalScope.launch` lives until the process ends or the work completes, leaking when the screen or request that started it is gone; a blocking call (`Thread.sleep`, a blocking JDBC call, `runBlocking`) on `Dispatchers.Main` or the default dispatcher freezes other work; a `suspend` function that ignores cancellation runs forever after its caller cancelled; an `async` whose exception is never awaited crashes the parent or swallows the error unpredictably. The judgment problem is to design every coroutine around its scope and lifetime, to choose dispatchers by the kind of work, to make cancellation cooperative, and to bridge blocking and callback APIs explicitly rather than pretending they are suspend functions.

## Core Rules

### Never Launch Without A Scope Whose Lifetime Matches The Work

Structured concurrency means every coroutine is launched in a `CoroutineScope`, and that scope's lifetime bounds the coroutines in it: when the scope is cancelled, all its coroutines are cancelled. This is the mechanism that prevents leaks.

- Launch coroutines in a scope tied to the work's natural lifetime: `viewModelScope` for Android view-model work, `lifecycleScope` for view work, a request-scoped scope for server request handling, a scope you create and cancel explicitly for a feature.
- Never use `GlobalScope` for application code. `GlobalScope` has no bound; coroutines launched in it run until they complete or the process dies, leaking when the component that started them is destroyed. `GlobalScope` is appropriate only for process-lifetime daemons, and even there a dedicated scope is clearer.
- A `suspend` function inherits the caller's scope and should not launch "fire and forget" work into another scope without an explicit reason; if it must, document the lifetime.

Strong choice: `class Feature { private val scope = CoroutineScope(SupervisorJob() + Dispatchers.Main); fun start() { scope.launch { ... } }; fun stop() { scope.cancel() } }`. Weak choice: `GlobalScope.launch { ... }` with no cancellation path.

### Make suspend Functions The Default For Asynchronous Work

A `suspend` function is the unit of asynchronous work in Kotlin: it can suspend without blocking, it inherits the caller's dispatcher and cancellation, and it composes with other suspend functions naturally. Prefer `suspend` over callbacks, futures, and launching coroutines, for operations that produce a value or complete.

- A function that does async work and returns a result should be `suspend fun fetch(): Result`, called from within a coroutine or another suspend function. The caller decides the scope and dispatcher.
- Do not launch a coroutine inside a function just to return a result (`fun fetch(): Deferred<T> = scope.async { ... }`) unless you have a reason; a suspend function is simpler and gives the caller control.
- A suspend function should not block (see below); if it must call blocking code, switch dispatchers explicitly with `withContext`.

### Choose Dispatchers By The Kind Of Work, And Switch Explicitly

Dispatchers determine what threads a coroutine runs on. Using the wrong dispatcher blocks other coroutines or violates threading constraints.

- `Dispatchers.Main` — UI thread (Android main, JavaFX Application Thread, Swing EDT). Use for UI work; never block on it.
- `Dispatchers.Default` — CPU-bound work, shared bounded thread pool sized to CPU cores. Use for computation; do not use for blocking I/O (it will starve the pool).
- `Dispatchers.IO` — blocking I/O work, larger elastic pool. Use for file, network, database blocking calls.
- `Dispatchers.Unconfined` — runs on the caller's thread until the first suspension; rarely the right choice, useful only for specific interop.

Switch explicitly with `withContext(Dispatchers.IO) { blockingCall() }` when calling blocking code from a non-blocking context. The explicit switch documents that a blocking call is happening and routes it to the right pool. A suspend function that does both CPU and I/O work should switch dispatchers at each transition.

### Make Cancellation Cooperative

Coroutines are cancelled cooperatively: cancellation sets a flag, and the coroutine must check it and exit. Code that never checks for cancellation runs forever after its job is cancelled.

- Suspending functions in kotlinx.coroutines (`delay`, suspend network calls, `withContext`) check for cancellation and throw `CancellationException`. Long-running CPU work between suspensions must check manually (`if (!isActive) return` or `ensureActive()` which throws).
- Catching `CancellationException` and continuing defeats cancellation. If you catch `Throwable` (e.g., for logging), rethrow `CancellationException` (or use `currentCoroutineContext().ensureActive()`) before continuing, so cancellation propagates.
- `CancellationException` is the mechanism, not an error; do not treat it as a failure to log or report. Resources held by the coroutine should be released in `finally` blocks, which run during cancellation.

### Use Job And SupervisorJob To Structure Failure Propagation

A `Job` has children, and by default a child's failure cancels its parent and siblings (structured failure propagation). A `SupervisorJob` isolates children: a child's failure does not cancel its siblings. The choice determines failure behavior.

- Use a regular `Job` (the default for `launch`/`async` in a scope) when children are part of one unit of work and should all fail together if one fails.
- Use a `SupervisorJob` (or `supervisorScope`) when children are independent and one's failure should not cancel the others (e.g., a dashboard loading independent widgets).
- `async` exceptions are only delivered when the `Deferred` is awaited; an unawaited failed `async` propagates the exception to its parent. Do not start `async` you will not await unless you handle its failure.

### Bridge Blocking And Callback APIs Explicitly

Much real-world code is blocking or callback-based, and coroutines must interoperate. The bridge should be explicit, not implicit.

- Blocking calls: wrap in `withContext(Dispatchers.IO) { ... }` so they run on the I/O pool and the suspend signature is honest. Do not call blocking code directly from `Dispatchers.Default` or `Dispatchers.Main`.
- Callback APIs: convert with `suspendCancellableCoroutine` or `callbackFlow`, so the callback becomes a suspend function that supports cancellation. This is cleaner than wrapping callbacks in futures.
- Reactive APIs (Flow, RxJava): use `asFlow()`/`asPublisher()` interop or collect/emit at the boundary; do not mix paradigms inside a single function.
- `runBlocking` bridges the blocking world to coroutines and should be used only at the top level (`main`, tests), never inside a coroutine (it blocks the thread and can deadlock).

### Use Flow For Streams, Not Coroutines For Single Values

`Flow` is Kotlin's cold asynchronous stream. Use it for sequences of values over time (events, sensor data, paginated results); use `suspend` for single values; use `StateFlow`/`SharedFlow` for hot shared state.

- A function returning multiple async values should return `Flow<T>`, collected by the caller. Do not expose a `Channel` or a callback where a Flow fits.
- Cold flows do nothing until collected; the collector's scope and cancellation govern the flow. This composes naturally with structured concurrency.
- `StateFlow` for observable state (a single up-to-date value, like LiveData), `SharedFlow` for fan-out events. Choose by whether you need state-holding or event-broadcast semantics.

## Common Traps

### GlobalScope.launch Leaking

`GlobalScope.launch { ... }` runs until completion or process exit, with no cancellation tied to the component that started it. When the screen/request/view-model is destroyed, the coroutine keeps running, holding references and doing work no one wants. Use a scoped lifetime (`viewModelScope`, `lifecycleScope`, a custom scope you cancel).

### Blocking On Dispatchers.Main Or Default

A blocking call (`Thread.sleep`, blocking I/O, `runBlocking`, a busy loop) on `Dispatchers.Main` freezes the UI; on `Dispatchers.Default` it starves the CPU pool and stalls other coroutines. Wrap blocking calls in `withContext(Dispatchers.IO)`.

### Ignoring Cancellation In CPU-Bound Loops

A long `for` loop with no suspension runs forever after its job is cancelled, because nothing checks `isActive`. Insert `ensureActive()` or `if (!isActive) return` periodically in long CPU work.

### Catching CancellationException And Continuing

`try { ... } catch (e: Exception) { /* continue */ }` catches `CancellationException` and defeats cancellation, because `CancellationException` extends `Exception` (via `Throwable`/`IllegalStateException` hierarchy — it is an `Exception`). Always rethrow `CancellationException` or check `isActive` before continuing, so cancellation propagates.

### async Without await

`async { throw ... }` whose `Deferred` is never awaited propagates the exception to the parent job, which may cancel siblings unexpectedly or surface the error in a confusing place. Only use `async` when you will `await` it; for fire-and-forget, use `launch` and handle exceptions explicitly.

### Wrong Job Type For Independent Children

A scope with a regular `Job` cancels all children when one fails; if the children are independent (loading separate widgets, separate downloads), use `SupervisorJob`/`supervisorScope` so one failure does not cancel the rest.

### runBlocking Inside A Coroutine

`runBlocking` blocks the current thread until the coroutine completes; calling it from inside a coroutine blocks the thread and can deadlock if the inner coroutine needs to resume on the same thread. Use `runBlocking` only at the top level.

### Switching Dispatchers Too Often

Each `withContext` switch has overhead and, more importantly, obscures the threading story. Switch at the boundary of a meaningful block of work, not around every individual call. Group CPU work together and I/O work together.

## Self-Check

- [ ] Every coroutine is launched in a scope whose lifetime matches the work (`viewModelScope`, `lifecycleScope`, a request-scoped or feature-scoped scope that is explicitly cancelled), and no `GlobalScope` is used for application code.
- [ ] Asynchronous operations are `suspend` functions that inherit the caller's scope and dispatcher; coroutines are launched only at scope owners, not inside suspend functions returning immediate results.
- [ ] Dispatchers are chosen by work kind (Main for UI, Default for CPU, IO for blocking), and blocking calls are wrapped in `withContext(Dispatchers.IO)` with an honest suspend signature.
- [ ] Long CPU-bound work checks `isActive`/`ensureActive()` periodically, and `CancellationException` is never swallowed — it is rethrown or the coroutine exits cooperatively.
- [ ] Job vs SupervisorJob is chosen by whether children should fail together or independently, and `async` is used only where its `Deferred` will be awaited.
- [ ] Blocking and callback APIs are bridged explicitly (`withContext`, `suspendCancellableCoroutine`, `callbackFlow`), and `runBlocking` appears only at top level (main, tests), never inside a coroutine.
- [ ] Streams of async values use `Flow` (cold) or `StateFlow`/`SharedFlow` (hot state/events), collected within a scoped context; single values use `suspend`.
- [ ] Resources held by coroutines are released in `finally` blocks that run during cancellation, and exceptions are handled or propagated deliberately, not swallowed.
- [ ] No blocking call runs on `Dispatchers.Main` or `Dispatchers.Default`, and dispatcher switches are grouped at meaningful boundaries rather than per-call.
- [ ] The concurrency design has been considered under cancellation mid-flight, exception in one child, and component destruction while work is in progress, and remains correct and leak-free.
