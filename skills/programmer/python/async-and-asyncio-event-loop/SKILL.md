---
name: python_async_and_asyncio_event_loop.md
description: Use when the agent is writing or reviewing Python asyncio code, creating and running an event loop, scheduling tasks and gather/wait, awaiting coroutines, mixing sync and async code, bridging blocking calls with run_in_executor, debugging "coroutine was never awaited" or "event loop is closed" errors, choosing asyncio.run vs manual loop management, or reasoning about where the event loop runs and how coroutines are driven. Covers the mechanics of the event loop, coroutine vs task vs future, await point semantics, loop lifecycle, thread-async bridging, and the pitfalls of blocking the loop, nesting loops, or discarding task references.
---

# Async And The Asyncio Event Loop

Python's `asyncio` runs coroutines on an event loop that suspends and resumes them at `await` points. The mechanics — what a coroutine is versus a task versus a future, where the loop runs, how blocking calls interact with it — are what separate async code that works from async code that freezes, leaks, or crashes with cryptic errors. The judgment problem is understanding the loop's lifecycle, driving coroutines correctly, and keeping the loop unblocked so cooperative concurrency actually cooperates.

Agents tend to call async functions without awaiting, run blocking code inside coroutines, manage the event loop manually when `asyncio.run` would do, nest or reuse loops across threads, or discard task references and watch them vanish. The harm appears as "coroutine was never awaited" warnings, loops that stall because one task blocked, tasks garbage-collected mid-flight, and `RuntimeError: event loop is closed` or "already running" surprises. The real work is using `asyncio.run` as the entry point, converting coroutines to tasks deliberately, offloading blocking work, and treating the loop as a single-threaded scheduler whose invariants you must respect.

## Core Rules

### Use asyncio.run As The Entry Point, Avoid Manual Loop Management

For applications and scripts, `asyncio.run(main())` is the correct way to start: it creates a fresh event loop, runs the coroutine to completion, cancels remaining tasks, closes the loop, and handles cleanup. It is the single sanctioned top-level entry point.

- Prefer `asyncio.run` over manually calling `loop = asyncio.new_event_loop(); loop.run_until_complete(...)`. Manual management is error-prone (forgetting to close, leaving tasks running) and rarely needed.
- `asyncio.run` cannot be called from within a running loop. If you are already in async context, you are inside a loop — do not start another. Nesting loops is an error.
- In long-running servers (web frameworks), the framework owns the loop and calls your async entry point; you do not call `asyncio.run` yourself. Know who owns the loop in your context.
- In tests, use `pytest-asyncio` or `anyio` runners that manage the loop per-test; do not call `asyncio.run` inside an async test.

Manual loop manipulation is a smell outside of library internals and test harnesses. Let `asyncio.run` or the framework manage the lifecycle.

### Distinguish Coroutine, Task, And Future

These three are often conflated, and confusing them causes most "it didn't run" bugs.

- **Coroutine**: the result of calling an `async def` function. It does nothing until it is awaited or scheduled. `async def f(): ...; f()` returns a coroutine object that sits idle — you get a "coroutine was never awaited" warning. You must `await f()` or schedule it as a task.
- **Task**: a coroutine wrapped for concurrent execution. `asyncio.create_task(coro)` schedules the coroutine to run on the loop as soon as possible, concurrently with the current coroutine. The task begins executing at the next await point; it does not need to be awaited to run, but you should await it to get its result or observe its exception.
- **Future**: a low-level placeholder for a result that will be available later. Application code rarely creates futures directly; tasks are built on them. Use futures only when bridging callbacks to async.

The critical distinction: a coroutine is lazy (runs only when awaited/scheduled); a task is eager (runs once scheduled). If you `create_task` and discard the reference, the task may be garbage-collected and cancelled — keep references.

### Never Block The Loop; Offload Blocking Calls

The event loop runs one coroutine at a time, switching only at `await`. Any synchronous stretch blocks every other coroutine, timer, and callback. This is the single most important invariant.

- Use async-native libraries (`httpx`/`aiohttp`, async DB drivers, `asyncio.sleep`) instead of blocking ones (`requests`, sync DB drivers, `time.sleep`).
- When only a blocking API exists, offload it: `await loop.run_in_executor(None, blocking_fn, arg)`. The executor runs the blocking call in a thread, freeing the loop.
- For long CPU-bound work, move it to a separate process; an executor thread is still subject to the GIL for pure Python.
- Audit every line inside an `async def` that is not an `await` — long sync stretches (parsing huge JSON, heavy loops) block the loop even though the function is async.

A loop that blocks for tens of milliseconds collapses under load. Profile with `PYTHONASYNCIODEBUG=1` to find sync stretches.

### Keep Task References And Observe Their Results

`asyncio.create_task(coro)` schedules the task, but if you discard the returned `Task`, the garbage collector may collect it mid-execution (Python warns about this). Always retain the reference until the task completes.

- Store task references in a set/field; clear them in a done callback (`task.add_done_callback(tasks.discard)`).
- Await tasks to observe their results and exceptions. A task whose exception is never retrieved logs "Task exception was never retrieved" and can hide failures.
- For fire-and-forget work, still track tasks so you can cancel them on shutdown.
- Prefer structured concurrency: `asyncio.TaskGroup` (3.11+) or `anyio.create_task_group` scope tasks so a failure cancels siblings and exceptions propagate. Bare `gather()` is less explicit about scoping.

### Bridge Async And Sync Deliberately

Mixing async and sync code across a boundary without a bridge produces errors. Decide the boundary explicitly.

- **Calling async from sync**: use `asyncio.run(coro)` at a sync entry point (not inside a running loop).
- **Calling sync (blocking) from async**: use `loop.run_in_executor` to run it in a thread.
- **Calling async from a different thread**: use `asyncio.run_coroutine_threadsafe(coro, loop)` to schedule a coroutine onto a loop owned by another thread, then await the returned future.
- Do not call `loop.run_until_complete` from within an async context — you are already in a loop.

The clean design is to pick one model per layer: the entry point runs the loop, internals are async, and blocking calls are isolated behind executor offloads at clear adapter boundaries.

### Handle Cancellation And Cleanup Correctly

Cancellation propagates by raising `asyncio.CancelledError` (a `BaseException`) at the current `await`. Mishandling it is a common bug.

- Never swallow `CancelledError` with `except Exception` (it is a `BaseException`, so `except Exception` does not catch it, but `except BaseException` or bare `except` does, breaking cancellation).
- If you catch `CancelledError` to clean up, re-raise it after cleanup.
- Put cleanup in `finally` or async context managers so it runs on return, raise, and cancel.
- When you cancel a task (`task.cancel()`), await it so the `CancelledError` is observed.

## Common Traps

### Calling An Async Function Without Awaiting

`f()` where `f` is `async def` returns a coroutine that never runs; you get "coroutine was never awaited." Always `await f()` or schedule it as a task.

### Blocking The Loop With Sync Calls

`requests.get`, `time.sleep`, sync DB calls, or a long CPU loop inside a coroutine freezes every other task. Use async libraries or `run_in_executor`.

### Discarding The Task Reference

`asyncio.create_task(coro)` with the result discarded lets the GC cancel the task unpredictably. Keep references until completion.

### asyncio.run Inside A Running Loop

Calling `asyncio.run` (or `run_until_complete`) from within async code raises "already running event loop." You are already in a loop; await instead.

### Swallowing CancelledError

`except BaseException` or bare `except` catches `CancelledError` and breaks cancellation propagation. Re-raise it after cleanup.

### Forgetting To Await A Task To Observe Its Exception

A task that raises an unobserved exception logs "Task exception was never retrieved" and hides the failure. Await tasks or attach done callbacks.

### Manual Loop Management In Application Code

Creating and closing the loop by hand instead of `asyncio.run` leads to leaked tasks, unclosed loops, and "event loop is closed" errors. Use `asyncio.run`.

### Assuming create_task Runs Immediately

`create_task` schedules the task; it runs at the next loop iteration, not inline. Code after `create_task` in the current coroutine runs first until the current coroutine awaits.

## Self-Check

- [ ] The event loop entry point uses `asyncio.run` (or a framework that owns the loop); manual loop creation/closing is absent from application code.
- [ ] Coroutines, tasks, and futures are distinguished: async functions are awaited or scheduled as tasks; no coroutine is created and discarded un-awaited.
- [ ] No blocking calls (`requests`, `time.sleep`, sync DB, long CPU loops) run inside `async def` without an executor offload or async-native replacement.
- [ ] Every `create_task` result is retained until completion; fire-and-forget tasks are tracked for shutdown cancellation.
- [ ] Tasks are awaited (or have done callbacks) so exceptions are observed; no "Task exception was never retrieved" warnings.
- [ ] Cancellation is handled correctly: `CancelledError` is re-raised after cleanup, never swallowed by broad exception handlers.
- [ ] Async/sync boundaries are explicit and bridged (`asyncio.run`, `run_in_executor`, `run_coroutine_threadsafe`); no nested or reused loops.
- [ ] Structured concurrency (`TaskGroup`/`anyio`) or explicit `gather` error handling is used so failures propagate and siblings are cancelled.
- [ ] The loop is profiled for blocking stretches (`PYTHONASYNCIODEBUG=1`); no sync stretch blocks for tens of milliseconds.
- [ ] Cleanup runs in `finally` or async context managers on all paths (return, raise, cancel).
