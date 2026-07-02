---
name: concurrency_and_structured.md
description: Use when the agent is writing Swift async/await code, using Task, TaskGroup, actor, Sendable, AsyncSequence, handling cancellation, timeouts, data races, structured concurrency, or diagnosing deadlocks, task leaks, actor reentrancy, and Sendable conformance errors in Swift applications.
---

# Concurrency and Structured

Swift's modern concurrency model (async/await, `Task`, `actor`, `Sendable`) was designed to make data-race-free code the default, and it largely succeeds at compile time. The bugs it does not prevent are the ones that come from treating structured concurrency as a drop-in replacement for GCD. A `Task` that is never awaited or cancelled leaks. An actor method that reenters itself via a callback can observe inconsistent state. A `Sendable` conformance asserted incorrectly lets a data race through the compiler's safety net. An unstructured `Task` started in a function that returns before the task finishes silently drops work or outlives its intended scope. The judgment problem is not "how do I write async func" but how to structure tasks so their lifetime, cancellation, and error propagation match the work's actual semantics.

The recurring failure mode is a developer who sprinkles `Task { }` to "make it async" without thinking about who owns the task, when it cancels, and what happens to errors and results. Unstructured tasks behave like detached threads: they outlive the scope that created them, they do not propagate cancellation, and their errors are swallowed unless explicitly handled. Structured concurrency (`TaskGroup`, child tasks) fixes this by tying child lifetimes to parents, but it requires you to actually use the structured primitives and to design functions around the boundary between synchronous and asynchronous code.

## Core Rules

### Distinguish structured from unstructured tasks

- **Structured** tasks (`async let`, `TaskGroup` child tasks) are tied to their parent's lifetime: they cancel when the parent cancels, and the parent cannot complete until they do. Use these when the work belongs to the current operation.
- **Unstructured** tasks (`Task { }`, `Task.detached { }`) outlive the current scope. Use these for fire-and-forget work whose lifetime is independent (e.g., starting a top-level operation from a sync context), and handle their errors and cancellation explicitly.

Default to structured. Reach for unstructured only when the work genuinely outlives the current operation, and always retain the `Task` handle if you need to cancel or await it.

### Design cancellation into every async API

Cancellation in Swift is cooperative: a cancelled task is signaled, not killed. Every `async` function should check cancellation at natural points (`try Task.checkCancellation()`, or by checking `Task.isCancelled` and aborting early) and should propagate cancellation to its sub-tasks. Rules:

- Long-running loops should check cancellation periodically.
- An async function that ignores cancellation will keep running after the caller has moved on, wasting resources.
- When wrapping callback-based APIs, bridge cancellation by cancelling the underlying work in `withTaskCancellationHandler`.

If you write an async function that does not respond to cancellation, callers cannot stop it; that is a contract violation.

### Actors serialize access, but do not make reentrancy safe

An `actor` ensures mutual exclusion for its state, but an `await` inside an actor method suspends and releases the isolation, allowing other calls to interleave. This means an actor method that awaits and then reads its own state can observe state mutated by a reentrant call. Rules:

- Treat each synchronous segment between `await`s as a critical section; do not assume state is unchanged across an `await`.
- Move state reads and writes into synchronous segments that do not span an `await`.
- For complex state machines, consider performing the whole operation synchronously on the actor or using a model where the state is computed before any suspension.

Actor reentrancy is the new data race: the compiler allows it, and it produces inconsistent state.

### `Sendable` is a compile-time data-race safety contract

`Sendable` marks a type as safe to pass across concurrency domains. The compiler enforces it at boundaries (task closures, actor parameters). Rules:

- Value types (`struct`, `enum`) with `Sendable` fields are `Sendable`; mark them conforming.
- Reference types are `Sendable` only if they are internally synchronized (e.g., `actor`, or a class with a lock and no mutable exposed state) or immutable.
- Do not assert `Sendable` conformance with `@unchecked Sendable` unless you have verified thread safety; it silences the compiler but not the data race.
- Closures passed across domains must be `@Sendable`; capturing mutable state in them is an error.

Treat `Sendable` errors as real data-race risks, not as ceremony to suppress.

### Avoid blocking calls in async contexts

A synchronous blocking call (a lock, `sleep`, a synchronous network API) on the cooperative thread pool blocks a thread that the runtime expects to be available for other tasks, causing thread starvation and priority inversion. Rules:

- Use `async` versions of I/O and locking (`await` an actor instead of `lock.synchronized`).
- If you must call a blocking API, offload it with `Task.detached` or an unstructured task on a dedicated executor, and document why.
- `DispatchQueue` and semaphores inside `async` code are a smell; prefer structured concurrency and actors.

### `TaskGroup` is the tool for dynamic fan-out with structured lifetime

When you need to run a dynamic number of child tasks and collect results, `TaskGroup` provides structured lifetime, cancellation propagation, and result aggregation. Rules:

- Use `withTaskGroup`/`withThrowingTaskGroup` for parallel fan-out whose lifetime is the current operation.
- Cancel the group to cancel all children; children must still check cancellation cooperatively.
- Collect results via `for await result in group { }`; ordering is not guaranteed unless you tag results.

### Timeouts and races need explicit boundaries

Swift has no built-in `withTimeout`, so timeouts require racing the work against a delay task in a group and cancelling on first result. Rules:

- Implement timeouts as a two-task race in a `TaskGroup`, cancelling the group when either completes.
- For "first of N," use a group and take the first result, cancelling the rest.
- Document the cancellation behavior: after a timeout, the underlying work should be cancelled, not left running.

## Common Traps

### `Task { }` that is never awaited or cancelled

An unstructured task started and discarded leaks: it runs to completion, its errors are swallowed, and it outlives the operation that needed it. Retain the handle and cancel/await it, or use structured concurrency.

### Ignoring cancellation in long async functions

A function that never checks `Task.isCancelled` keeps running after the caller cancels, wasting CPU and battery. Check cancellation at loop boundaries and between major steps.

### Actor reentrancy across `await`

Reading actor state, awaiting, then writing produces inconsistent state because another call can interleave. Keep critical sections synchronous within the actor.

### `@unchecked Sendable` to silence the compiler

Asserting `Sendable` without verification lets a data race through the type system. Only use it with a documented synchronization guarantee.

### Blocking the cooperative pool

`semaphore.wait()`, `NSLock`, or synchronous I/O in an `async` function starves the shared pool. Use async primitives or offload to a dedicated executor.

### Fire-and-forget tasks that swallow errors

`Task { try await work() }` discards a throwing result. Handle errors inside the task (log, recover) or propagate via the task handle.

### `Task.detached` when inheritance was intended

`Task.detached` does not inherit the actor, priority, or task local values of the caller. Use `Task` (which inherits context) unless you specifically want detachment.

## Self-Check

- Is each concurrent unit structured (tied to a parent's lifetime) where possible, with unstructured tasks retained for cancellation and error handling?
- Does every long-running `async` function check cancellation at natural points and propagate it to sub-tasks and wrapped callback APIs?
- For each `actor`, are critical sections synchronous between `await`s, with no assumption that state is unchanged across a suspension?
- Are `Sendable` conformances earned (value types or internally synchronized), with `@unchecked Sendable` used only with a documented guarantee?
- Are blocking calls (locks, semaphores, synchronous I/O) kept out of `async` contexts, offloaded where unavoidable?
- Are dynamic fan-out operations using `TaskGroup` with structured lifetime and cooperative cancellation?
- Are timeouts and "first of N" patterns implemented as explicit races that cancel the losing branches?
- Are fire-and-forget tasks handling their errors internally rather than swallowing them, and is `Task` vs `Task.detached` chosen based on whether context should be inherited?
