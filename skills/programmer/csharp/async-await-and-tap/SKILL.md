---
name: csharp_async_await_and_tap.md
description: Use when the agent is writing or reviewing C# code using async and await, the Task-based Asynchronous Pattern (TAP), Task and ValueTask, ConfigureAwait, async void methods, SynchronizationContext, synchronization context capture, blocking on async code with GetAwaiter().GetResult() or .Result, thread pool starvation, cancellation tokens, or any asynchronous API design on .NET where deadlock, starvation, or unobserved exceptions are risks.
---

# Async/Await And The TAP Pattern In C#

C# async/await makes asynchronous code read like synchronous code, and that readability is exactly what makes it dangerous. The compiler rewrites an async method into a state machine that captures locals, schedules continuations, and resumes on a context — and most of the bugs live in the gap between what the code looks like and what the machine actually does. Deadlocks from blocking on async code, thread pool starvation from synchronous-over-async wrapping, swallowed exceptions from `async void`, context capture overhead, and `ValueTask` misuse are all consequences of treating async as a keyword rather than a concurrency model with rules.

Agents frequently sprinkle `async` and `await` to "make it asynchronous" without understanding that await captures a synchronization context, that `async void` cannot be awaited and swallows exceptions, that blocking on an async method in a context-capturing environment deadlocks, or that `ValueTask` is not a free optimization. The judgment problem is to know when async is appropriate, how to avoid the blocking-on-async deadlock pattern, how to choose between `Task` and `ValueTask`, how to handle cancellation and exceptions correctly, and how to keep the thread pool healthy. Getting this wrong produces code that passes unit tests and deadlocks or starves under real load.

## Core Rules

### Never Block On Async Code

The single most important rule: do not synchronously block on an asynchronous operation, especially in environments that have a synchronization context (UI frameworks, older ASP.NET). The patterns `.Result`, `.Wait()`, and `.GetAwaiter().GetResult()` on an async method can deadlock.

- The classic deadlock: an async method captures the current synchronization context (e.g., the UI thread context), awaits an operation, and the continuation needs to resume on that context. If the calling code blocks the same thread waiting for the result (`.Result`), the continuation can never run, and the call hangs forever.
- `.GetAwaiter().GetResult()` is slightly better than `.Result` (it unwraps exceptions instead of wrapping them in `AggregateException`) but it still blocks and still deadlocks in a context-capturing environment. It is not a safe escape hatch.
- The correct fix is to be async all the way down: make the caller async and await. If you absolutely must bridge sync and async (e.g., implementing a synchronous interface over an async implementation, or `Main` before top-level statements), use `Task.Run(() => AsyncMethod()).GetAwaiter().GetResult()` to push the async work off the capturing context — but recognize this is a last resort with its own overhead.

Strong choice: `public async Task<User> GetUserAsync(int id)` and callers `await` it. Weak choice: `public User GetUser(int id) => GetUserAsync(id).GetAwaiter().GetResult();` "to keep the API synchronous."

### Use ConfigureAwait Deliberately

By default, `await` captures the current synchronization context and resumes the continuation on it. In library code and most server code, you do not want this; it adds overhead, can cause deadlocks when callers block, and provides no benefit. `ConfigureAwait(false)` tells the await to resume on whatever the task scheduler picks, skipping the context capture.

- In library code (code intended to be called from any host), append `.ConfigureAwait(false)` to every await. This is the conventional, correct default for libraries.
- In application code that owns its context (UI event handlers, ASP.NET Core which has no context), `ConfigureAwait(false)` is less critical but harmless; ASP.NET Core has no synchronization context by default, so the capture is a no-op there.
- The deadlock-avoidance benefit matters most when library code might be called from a context-capturing host. A library that does not use `ConfigureAwait(false)` can deadlock when a UI app blocks on it.

Be consistent: mixing configured and non-configured awaits in the same method is confusing and usually indicates the author did not understand the decision.

### Never Write async void

`async void` methods cannot be awaited by their callers, and exceptions thrown inside them propagate to the synchronization context (typically crashing the process or being swallowed), not to the caller. They are essentially unobservable fire-and-forget methods with broken exception handling.

- The only legitimate use of `async void` is for event handlers, because event handler signatures are dictated by the framework and must return void. Even then, wrap the body in try/catch and handle exceptions explicitly.
- For "fire and forget" work that is not an event handler, return `async Task` and let the caller decide whether to await (or explicitly discard with discard assignment `_ = DoStuffAsync();`), so exceptions are at least observable on the returned task.
- `async void` methods also break composition: they cannot be awaited in a `Task.WhenAll`, cannot have their completion observed, and complicate testing.

### Choose Task vs ValueTask Based On Synchronous Completion And Allocation

`Task` is a reference type and allocates. `ValueTask<T>` is a value type that can avoid allocation when the operation completes synchronously. The choice is about allocation pressure, not a general "newer is better."

- Use `Task`/`Task<T>` as the default for public async APIs. They are well-understood, awaitable multiple times, and support `WhenAll`/`WhenAny` cleanly.
- Use `ValueTask<T>` when the operation frequently completes synchronously (e.g., a cached value, a fast-path check) and the allocation avoidance is measurable. `ValueTask` is more constrained: it should not be awaited multiple times, and its semantics around completion are stricter.
- Do not switch everything to `ValueTask` for "performance." `ValueTask` adds complexity (multiple-await restrictions, interop concerns) and only helps when synchronous completion is common. Measure before optimizing.

### Propagate Cancellation Tokens Everywhere

Async methods should accept a `CancellationToken` and pass it down to every awaitable operation, so cancellation is prompt and cooperative. Forgetting cancellation leaves operations running after the caller has given up, wastes resources, and can leave the system in an inconsistent state.

- Include a `CancellationToken cancellationToken = default` parameter in async method signatures, and pass it to every downstream async call and to loops that should be cancellable (`cancellationToken.ThrowIfCancellationRequested()` in long-running loops).
- Honor cancellation promptly: check the token at natural boundaries, and pass it to I/O and delay operations so they cancel rather than completing.
- Never swallow `OperationCanceledException` unless you are genuinely converting cancellation into a different semantic; let it propagate to the caller that issued the cancellation.

### Observe Every Task's Exceptions

An unobserved task exception (a task that faults and is never awaited or has its exception examined) can, depending on runtime settings, crash the process on GC. Even when it does not crash, it represents a silently swallowed error.

- Always await tasks you start, or attach a continuation that observes exceptions. The discard `_ = DoStuffAsync();` does not observe exceptions; if you use it, attach `.ContinueWith(t => Log(t.Exception), TaskContinuationOptions.OnlyOnFaulted)` or wrap the body in try/catch.
- In `Task.WhenAll`, exceptions from all tasks are aggregated into an `AggregateException`; awaiting the WhenAll rethrows the first, but examining `.Exception` gives all of them. Decide which you need.
- Do not leave fire-and-forget tasks unobserved "because they are background work"; a faulted background task with no observer hides a real bug.

### Avoid Sync-Over-Async And Async-Over-Sync Wrapping

Two anti-patterns starve thread pools and obscure the real execution model.

- **Sync-over-async:** calling an async method and blocking on it (`.Result`, `.Wait()`). Besides the deadlock risk, this ties up a thread pool thread waiting for I/O, which under load starves the pool and cascades into timeouts and failures. If the operation is genuinely asynchronous, the caller should be async too.
- **Async-over-sync:** wrapping a genuinely synchronous CPU-bound operation in `Task.Run` to expose an async API. This adds thread-pool hop overhead and hides that the work is CPU-bound. Expose synchronous CPU-bound work as a synchronous method, and let the caller decide to offload it with `Task.Run` if needed.

The rule: I/O-bound work should be async end-to-end; CPU-bound work should be synchronous and offloaded explicitly by the caller when appropriate.

## Common Traps

### .Result / .Wait() Deadlocking In A Context-Capturing Environment

Calling `.Result` on an async method from the UI thread or from legacy ASP.NET deadlocks because the continuation needs the captured context that the blocking call is holding. The fix is to await, or to push the async work to the thread pool with `Task.Run` before blocking (last resort).

### async void Swallowing Or Crashing On Exceptions

An `async void` method that throws propagates the exception to the synchronization context, which typically terminates the process (UI) or silently drops it. Never use `async void` except for event handlers, and always wrap the body in try/catch.

### Forgetting ConfigureAwait(false) In A Library

A library that omits `ConfigureAwait(false)` can deadlock when a UI application blocks on it, and adds context-capture overhead for every caller. Libraries should configure false on every await as a matter of course.

### Thread Pool Starvation From Sync-Over-Async Under Load

Blocking thread pool threads on `.Result` across many concurrent requests exhausts the pool: new work queues, latency spikes, and timeouts cascade. The symptom is "the app works in test but dies under load." Profile for blocked threads and convert to async end-to-end.

### Switching Everything To ValueTask For "Performance"

`ValueTask` is not a free upgrade. It cannot be safely awaited multiple times, has stricter semantics, and only helps when synchronous completion is common. Indiscriminate use adds bugs without measurable benefit. Measure first.

### Discarding A Task Without Observing Exceptions

`_ = FireAndForgetAsync();` starts a task whose exceptions are never observed. If the task faults, the exception is swallowed (or crashes on GC, depending on settings). Wrap the body in try/catch, or attach a faulting continuation that logs.

### Mixing Configured And Non-Configured Awaits

Inconsistent `ConfigureAwait` use within a method signals confusion about context and can reintroduce the deadlocks the configuration was meant to avoid. Pick a policy for the codebase (libraries configure false; apps follow host convention) and apply it uniformly.

### Not Propagating The CancellationToken

An async method that accepts a token but does not pass it to downstream calls (or forgets `ThrowIfCancellationRequested` in a loop) makes cancellation ineffective: the caller cancels, but the work keeps running. Pass the token at every await boundary.

## Self-Check

- [ ] No code blocks on async operations with `.Result`, `.Wait()`, or `.GetAwaiter().GetResult()` in a context-capturing environment; callers are async all the way down, or sync-async bridging uses `Task.Run` as a documented last resort.
- [ ] Library code applies `ConfigureAwait(false)` to every await, and application code follows a consistent context policy matching its host.
- [ ] No `async void` methods exist except framework-required event handlers, and those wrap their bodies in try/catch.
- [ ] `ValueTask` is used only where synchronous completion is common and the allocation benefit is justified, and its multiple-await restriction is respected.
- [ ] Every async method accepts and propagates a `CancellationToken`, passing it to downstream awaits and checking it in long-running loops.
- [ ] Every started task is observed for exceptions, either by awaiting it or by a faulting continuation; no fire-and-forget task silently swallows faults.
- [ ] No sync-over-async blocking under load, and no async-over-sync wrapping that hides CPU-bound work behind a `Task.Run` facade.
- [ ] Exceptions in `Task.WhenAll` are examined appropriately (first exception via await, all via `.Exception`) for the calling context.
- [ ] The async surface is coherent: an I/O-bound operation is async end-to-end, a CPU-bound operation is synchronous and explicitly offloaded when needed.
- [ ] The code has been considered under real concurrency (multiple concurrent calls, cancellation mid-flight, exceptions during await) and remains correct.
