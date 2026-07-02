---
name: python_asyncio_and_concurrency.md
description: Use when the agent is writing or reviewing Python async/await code, choosing between asyncio tasks/threads/multiprocessing, debugging event loop blocking, handling task cancellation and cleanup with try/finally and AsyncExitStack, reasoning about the GIL, mixing blocking and async libraries, designing async context managers, or building concurrent pipelines. Covers I/O-bound vs CPU-bound selection, executor offloading, cancellation propagation, timeouts, backpressure, and the boundary between cooperative concurrency and true parallelism.
---

# Asyncio And Concurrency

Python's `asyncio` gives cooperative concurrency on a single thread, not parallelism. The event loop runs one coroutine at a time and switches only at `await` points, so any synchronous stretch of code blocks every other task. The judgment problem is choosing the right concurrency model for the workload, keeping the loop unblocked, and making cancellation and cleanup correct under failure — areas where superficially working code hides deadlocks, leaked resources, and unbounded queues.

Agents tend to reach for `async def` because it looks modern, then call blocking libraries inside it, spawn tasks without storing references, swallow `CancelledError`, or assume `asyncio` will speed up CPU-heavy work. The harm ranges from latency collapse (one slow call stalls the whole service) to silent resource leaks (tasks garbage-collected mid-flight, locks never released, connections never closed). The real work is matching the concurrency model to the workload and treating cancellation, timeouts, and shutdown as first-class design problems.

## Core Rules

### Choose The Concurrency Model By Workload Character

The three models solve different problems; picking the wrong one wastes effort or makes things slower.

- **asyncio**: best for many I/O-bound tasks (HTTP clients, databases, web servers). One thread, many concurrent waits, low per-task overhead. Useless for CPU-bound work because the GIL and single-threaded loop prevent parallel compute.
- **threading** (`concurrent.futures.ThreadPoolExecutor`): good for I/O-bound code that uses blocking libraries you cannot or will not rewrite as async, and for C extensions that release the GIL (NumPy, compression, some crypto). The GIL still serializes pure-Python CPU work, so threads rarely speed up CPU-bound Python.
- **multiprocessing** (`ProcessPoolExecutor`, separate processes): the only model that gives true parallel CPU execution for pure Python, because each process has its own GIL. Pays for it with serialization/pickling overhead and higher startup cost.

Ask first: is the bottleneck I/O or CPU? For I/O, prefer asyncio if the libraries support it, else threads. For CPU, use multiprocessing or offload to a C/native library that releases the GIL. Never assume asyncio parallelizes computation.

### Never Block The Event Loop

The single most important async invariant is that the loop must keep running. A blocking call — `time.sleep`, `requests.get`, `file.read` on a slow disk, a long pure-Python computation, `time.sleep` inside a sync function called from async — freezes every coroutine, timer, heartbeat, and health check until it returns.

Mitigations, in order of preference:

1. Use the async-native equivalent (`asyncio.sleep`, `httpx`/`aiohttp`, `anyio`), or an async driver for the database.
2. If only a blocking API exists, run it in an executor: `await loop.run_in_executor(None, blocking_fn, arg)`. Keep the offloaded unit coarse (one blocking call, not a tight loop) to avoid executor thrash.
3. For long CPU work, move it to a separate process so the loop thread is fully free.

Profile suspected blockers with `asyncio` debug mode (`PYTHONASYNCIODEBUG=1`) or tools that flag long sync stretches. A loop that blocks for tens of milliseconds is already a problem under load.

### Store Task References And Manage Their Lifecycle

`asyncio.create_task(coro)` schedules the task, but if you discard the returned `Task` object, the garbage collector may collect and cancel it at any time — Python warns about this explicitly. Always keep a reference until the task completes (a set, a field, an `AsyncExitStack`).

Beyond references, decide explicitly:

- **Fire-and-forget vs awaited**: if you need the result or must know about failure, await it. If truly background, still track it for cancellation on shutdown.
- **Structured concurrency**: prefer `asyncio.TaskGroup` (3.11+) or `anyio.create_task_group` so that a failure in one task cancels siblings and exceptions propagate. Bare `gather()` with `return_exceptions=False` is similar but less explicit about scoping.
- **Fan-out limits**: unbounded `create_task` in a loop overwhelms downstream resources. Use a `Semaphore`, a bounded queue, or chunked processing to apply backpressure.

### Handle Cancellation And Cleanup Correctly

Cancellation is how asyncio stops work, and it propagates by raising `asyncio.CancelledError` at the current `await`. Mishandling it is the most common async bug.

Rules:

- `CancelledError` is a `BaseException` in 3.8+. Never swallow it with `except Exception`. If you catch it (to clean up), re-raise it after cleanup.
- Put cleanup in `finally` blocks or async context managers (`async with`) so it runs whether the coroutine returns, raises, or is cancelled. This is where connections, locks, files, and temp resources must be released.
- `AsyncExitStack` composes multiple async context managers and guarantees ordered cleanup even when setup fails partway.
- When you cancel a task yourself (`task.cancel()`), `await` it so the `CancelledError` (or a different exception) is observed; an un-awaited cancelled task can surface errors later or hide them.
- Distinguish cancellation from real errors in logs and metrics; a flood of cancellations during shutdown is expected, a flood during normal operation signals a design problem.

### Apply Timeouts And Backpressure Deliberately

Unbounded waits are how async systems stall silently. Every external call that can hang should have a timeout, and every producer faster than its consumer needs backpressure.

- Use `asyncio.wait_for(coro, timeout)` or `async with asyncio.timeout(t)` (3.11+) for deadlines. Choose realistic values per operation, not a single global timeout.
- For streams and queues, prefer bounded `asyncio.Queue(maxsize=N)` so a slow consumer slows the producer instead of growing memory unbounded.
- Propagate deadlines through your abstraction rather than letting each layer pick its own; a request-scoped deadline that shortens as it nears zero is usually the right model.

### Reason About The GIL Honestly

The Global Interpreter Lock means only one thread executes Python bytecode at a time. This shapes every concurrency decision:

- Threads do not speed up pure-Python CPU work; they only help when waiting (I/O) or when calling GIL-releasing native code.
- asyncio runs on one thread, so it gets no CPU parallelism at all — its win is concurrency on I/O waits.
- Free-threaded CPython (PEP 703, no-GIL builds) changes this, but as of writing it is experimental; do not assume it is available or that existing C extensions are thread-safe under it.

When someone asks "will making this async make it faster," the honest answer is: only if it is I/O-bound and currently waiting synchronously. For CPU-bound work, async makes it slower (more overhead, same single thread).

## Common Traps

### Calling Blocking Code Inside `async def`

`requests.get`, `time.sleep`, synchronous database drivers, and heavy CPU loops inside a coroutine freeze the loop. The code "works" in tests and collapses under load. Audit every `await`-less call inside async functions.

### Losing The Task Reference

`asyncio.create_task(poll())` with the result discarded lets the GC cancel the task unpredictably. Keep references in a collection and clear them in callbacks (`task.add_done_callback(tasks.discard)`).

### Swallowing `CancelledError`

`except Exception:` does not catch `CancelledError` (it is a `BaseException`), but `except BaseException:` or bare `except:` does, and silently breaks cancellation. Always re-raise `CancelledError` after cleanup.

### `asyncio.gather` Hiding Errors

`gather(*tasks)` with default `return_exceptions=False` cancels remaining tasks on the first exception and raises it — usually what you want. But `return_exceptions=True` returns exceptions as values, which callers often forget to inspect, silently dropping failures. Decide explicitly and check results when using `return_exceptions=True`.

### Assuming Async Libraries Are Faster

Async gives concurrency, not speed per request. A single async request has more overhead than a sync one. The win is throughput when many requests wait on I/O simultaneously. For low-concurrency or CPU-bound work, async is pure overhead.

### Forgetting `loop.run_in_executor` For Blocking Calls

Wrapping a blocking call in `async def` does not make it async; it still blocks the loop. You must offload it to an executor. Conversely, offloading tiny calls adds overhead — batch them.

### Mixing Async And Sync Code Across A Boundary

Calling async functions from sync code (or vice versa) without a bridge (`asyncio.run`, `run_in_executor`, `loop.run_until_complete`) leads to "coroutine was never awaited" or nested-loop errors. Decide the boundary explicitly, usually at the top of the program (entry point runs the loop; internals are async) or at a clear adapter layer.

### Deadlocks From Awaiting In Locked Sections

Holding an `asyncio.Lock` across an `await` that needs the same lock (reentrancy) or that waits on a resource held by a task waiting on the lock causes deadlocks. Keep critical sections short and free of awaits on contended resources, or redesign the locking.

## Self-Check

- [ ] The concurrency model (asyncio / threads / multiprocessing) matches the workload: I/O-bound vs CPU-bound, and async libraries vs blocking ones.
- [ ] No blocking calls (`time.sleep`, sync HTTP/DB, long CPU loops) run inside `async def` without an executor offload or async-native replacement.
- [ ] Every `create_task` result is retained until completion; fire-and-forget tasks are tracked for shutdown cancellation.
- [ ] `CancelledError` is never swallowed; cleanup runs in `finally` or async context managers and re-raises cancellation.
- [ ] External calls that can hang have explicit, realistic timeouts; bounded queues or semaphores apply backpressure where producers can outrun consumers.
- [ ] Structured concurrency (`TaskGroup`/`anyio`) or explicit `gather` error handling is used so failures propagate and siblings are cancelled.
- [ ] The GIL's effect on the chosen design is acknowledged; CPU-bound work uses multiprocessing or GIL-releasing native code, not threads or asyncio alone.
- [ ] Async/sync boundaries are explicit and bridged correctly (`asyncio.run`, executors), with no "coroutine never awaited" warnings.
- [ ] Locks are not held across awaits that can deadlock, and critical sections are short.
- [ ] Shutdown cancels outstanding tasks, releases resources, and logs cancellation distinctly from real errors.
