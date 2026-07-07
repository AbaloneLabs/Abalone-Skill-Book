---
name: javascript_web_workers_and_off_main_thread.md
description: Use when the agent is moving expensive computation, parsing, image processing, or blocking work off the main thread using Web Workers, Service Workers, SharedArrayBuffer, transferable objects, structured clone, message ports, BroadcastChannel, or comlink-style RPC, or is diagnosing "the UI freezes during processing", worker startup cost, data-copy overhead, postMessage serialization stalls, SharedArrayBuffer cross-origin isolation requirements, or "the worker never receives the message". Covers the decision of what to move off-thread, the message-passing boundary, transferables and zero-copy, and the security and lifecycle pitfalls of workers.
---

# Web Workers And Off-Main-Thread Work In JavaScript

The browser main thread runs the DOM, event handling, layout, paint, and all JavaScript by default. Any script that runs longer than a frame (~16ms) blocks all of that, producing a frozen UI, unresponsive inputs, and dropped interactions. The remedy is to move expensive work — parsing, encoding, image and signal processing, large sorts, crypto, simulation — to a Web Worker, which runs on a separate thread with its own event loop. The judgment problem is not "can I put this in a worker" but "what is worth the boundary cost, how do I move data across it efficiently, and how do I keep the worker's lifecycle and the message-passing contract correct." Workers are powerful and underused, but they are not free: each worker is a separate global scope with startup cost, every message crosses a serialization boundary, shared memory requires cross-origin isolation, and a worker that holds references or runs forever is a leak.

Agents either avoid workers entirely (freezing the UI) or use them naively (copying huge data on every message, blocking on synchronous `postMessage` expectations, or leaving workers running after their work is done). The remedy is to decide per task whether the work justifies a worker, to choose the right data-transfer mechanism (structured clone, transferables, SharedArrayBuffer) by data size and ownership, to design the message contract explicitly (request/response correlation, error propagation, cancellation), and to manage worker lifecycle (creation, pooling, termination) deliberately.

## Core Rules

### Move Only Work Whose Per-Call Cost Exceeds The Boundary Cost

A worker has a startup cost (spawning the thread, loading the script, parsing) and a per-message cost (serializing the input, copying or transferring, serializing the response). For tiny or one-off work, the boundary cost exceeds the benefit and the main thread is fine. Move work that is both CPU-heavy and repeated or long-running: decoding images, parsing large files, running simulations, hashing large data, complex search. For one-shot heavy work, consider reusing a long-lived worker rather than spawning per task.

- Measure: if the main-thread version drops frames or blocks input for >50–100ms, it is a worker candidate.
- Reuse workers across calls (a pool, or a single long-lived worker) to amortize startup; do not spawn a worker per event unless the work is genuinely independent and long.
- Keep the worker's module small and focused so startup is fast.

### Choose The Data-Transfer Mechanism By Size And Ownership

`postMessage` serializes the message with the structured clone algorithm and, by default, copies it. For large data this copy dominates. Three mechanisms change the cost:

- **Structured clone (default):** a deep copy; safe, but O(data size) per message. Fine for small messages, prohibitive for large buffers sent frequently.
- **Transferable objects:** `ArrayBuffer`, `MessagePort`, `ImageBitmap`, `OffscreenCanvas`, and a few others can be *transferred* (`postMessage(msg, [transferList])`), moving ownership zero-copy and detaching the original. Use when the sender no longer needs the buffer. This is the standard high-throughput path.
- **SharedArrayBuffer:** shared memory accessible to both threads without copying, enabling streaming and concurrent computation. Requires the document to be cross-origin isolated (`COOP: same-origin`, `COEP: require-corp` headers), which is a deployment constraint that affects the whole site. Use only when you need concurrent access and can meet the isolation requirement; otherwise prefer transferables.

Match the mechanism to the workload: small messages clone; large one-way buffers transfer; continuous bidirectional streaming with cross-origin isolation uses SharedArrayBuffer plus `Atomics` for synchronization.

### Design The Message Contract Explicitly

A worker is a separate global with no shared memory (unless SharedArrayBuffer) and no shared call stack. Communication is asynchronous message passing, and the contract must be designed like an API: request/response correlation, error propagation, cancellation, and backpressure.

- Correlate requests with an id: each request carries a unique id, and the worker returns it with the response, so a single long-lived worker can multiplex concurrent requests.
- Propagate errors as messages, not just thrown exceptions; an uncaught error in a worker fires `error` on the worker object but does not reject a specific request unless you model it.
- Support cancellation: a long task should check a cancellation flag (a message, or an `Atomics`-shared flag) periodically, and the caller should be able to abandon a request without leaking the worker.
- Consider an RPC layer (comlink) that hides `postMessage` behind async function proxies, reducing contract boilerplate — but understand it serializes arguments the same way.

### Do Not Block, And Do Not Assume Synchronous Responses

`postMessage` is asynchronous; the response arrives in a later task. Never write code that "waits" for a worker response synchronously — there is no such mechanism on the main thread, and `Atomics.wait` cannot be used there. Structure the caller around promises/callbacks. Inside the worker, the same event-loop rules apply: long synchronous work in the worker blocks that worker's message processing, so chunk it and yield if it must remain responsive to new messages (including cancellation).

### Manage Worker Lifecycle: Creation, Pooling, Termination

A worker runs until terminated or until its owner is gone. A worker created per event and never terminated leaks threads and memory. Decide the lifetime:

- Long-lived worker for the application's lifetime (e.g., a background indexer), terminated on page hide/unload.
- A pool of workers sized to the hardware concurrency (`navigator.hardwareConcurrency`), reused across tasks, for parallel batch work.
- Per-task workers for genuinely independent heavy one-shots, terminated on completion (`worker.terminate()`).

Terminate workers when their owning component is destroyed; a worker retains the resources it loaded and the messages it holds.

### Use Service Workers For Caching And Network Control, Not Computation

A Service Worker is a different kind of worker: an event-driven proxy between the page and the network, used for offline caching, push notifications, and request interception. It is not a general compute worker: it can be killed by the browser at any time when idle, so it must not hold stateful long-running computations or rely on staying alive. Use Service Workers for caching and network control with idempotent, restartable handlers; use Web Workers for computation.

## Common Traps

### Spawning A Worker Per Event

`new Worker(...)` on every keystroke or every file spawns threads faster than they finish, exhausting resources. Reuse a long-lived worker or a pool.

### Copying Huge Buffers On Every Message

`postMessage({ data: hugeUint8 })` clones the buffer each call; for a video frame pipeline this dominates. Transfer the buffer (`postMessage({ data }, [data.buffer])`) or use SharedArrayBuffer.

### Expecting A Synchronous Response

Code written as `const result = worker.doWork()` assumes a blocking call that does not exist. Workers are async; use promises and message correlation.

### Uncaught Worker Error Losing The Request

A throw inside the worker fires `worker.onerror` but does not tell the caller which request failed. Wrap worker logic in try/catch and post an error message with the request id.

### SharedArrayBuffer Without Cross-Origin Isolation

`new SharedArrayBuffer(n)` throws if the page is not cross-origin isolated. Enabling isolation (`COOP`/`COEP`) can break third-party iframes and resources that lack `CORP` headers; budget for that migration, do not assume it is free.

### Worker Never Terminated

A worker created in a component that is destroyed but never `terminate()`d keeps its thread and memory. Terminate on teardown, or scope to page lifetime.

### Service Worker Treated As A Long-Running Compute Process

A Service Worker can be killed mid-computation when idle; storing state or running long tasks in it produces intermittent failures. Keep SW handlers idempotent and stateless across restarts.

### Modifying A Shared Buffer Without Synchronization

Two threads writing a `SharedArrayBuffer` without `Atomics` or another synchronization mechanism races and corrupts. Use `Atomics` for any shared-memory coordination.

## Self-Check

- [ ] Work was moved off-thread only when its per-call cost exceeds the worker startup and message boundary cost, and workers are reused (long-lived or pooled) rather than spawned per event.
- [ ] The data-transfer mechanism matches the workload: small messages clone, large one-way buffers transfer (and the sender accepts detachment), and SharedArrayBuffer is used only with cross-origin isolation and `Atomics` synchronization.
- [ ] The message contract is explicit: requests carry ids for correlation, errors are propagated as messages with the request id, and long tasks support cancellation via a flag or message.
- [ ] No code waits synchronously for a worker response; the caller is structured around promises/callbacks, and the worker yields periodically if it must stay responsive to new messages.
- [ ] Worker lifecycle is managed: long-lived workers are terminated on page/component teardown, pools are sized to `navigator.hardwareConcurrency`, and no worker leaks past its owner.
- [ ] Service Workers are used only for caching/network control with idempotent, restartable handlers, not for stateful long-running computation.
- [ ] Worker errors are handled at the owner (`onerror`) and per-request (error messages), so no failure is silent, and resources held by a failed task are released.
- [ ] The design has been considered under rapid task creation, cancellation mid-flight, worker termination, and large/continuous data transfer, and remains correct and bounded.
