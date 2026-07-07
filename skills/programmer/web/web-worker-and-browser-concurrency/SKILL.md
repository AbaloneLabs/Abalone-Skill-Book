---
name: web_worker_and_browser_concurrency.md
description: Use when the agent is using Web Workers (dedicated, shared, service) or other browser concurrency primitives to offload heavy computation from the main thread, designing the worker boundary and message-passing interface, reasoning about transferable objects and structured cloning cost, deciding what to parallelize and how many workers to use, or diagnosing worker issues (main-thread jank despite a worker, data-copy overhead dominating, a worker blocking on a shared resource, deadlocks in worker coordination). Also covers the failure mode of offloading work to a worker but paying more in data-copy overhead than the computation saves, workers that are too coarse or too fine-grained, the main thread still janking because the worker's results are processed on the main thread, and the recurring mistake of treating workers as "free parallelism" when their value depends entirely on whether the computation-to-communication ratio justifies the worker boundary cost.
---

# Web Worker And Browser Concurrency

JavaScript on the web runs on a single main thread, and heavy computation on it blocks rendering and interaction (jank, frozen UI). Web Workers run scripts on background threads, enabling true parallelism for CPU-heavy work. The judgment problem is that a worker is only valuable when the computation saved on the main thread exceeds the cost of the worker boundary — the message-passing and data-copying overhead — and that tradeoff is easy to get wrong. Offloading work to a worker but copying large data to and from it can cost more than the computation saves, leaving the main thread janking during the copy. A worker that is too coarse (one worker doing everything serially) or too fine-grained (many workers with coordination overhead) is inefficient. The main thread can still jank if the worker's results are processed or rendered on the main thread. And worker coordination (shared state, locks) can deadlock or contend. The discipline is to assess the computation-to-communication ratio, to design the worker boundary and message interface for low overhead (transferables, shared memory where appropriate), to size and partition work correctly, and to ensure the main thread is actually freed.

Agents tend to reach for a worker whenever they see heavy computation, because "offload to a worker" is the standard advice. The harm appears as no improvement (the data-copy overhead negates the worker's benefit), as new jank (copying large data to the worker blocks the main thread), as coordination overhead (many workers contending or deadlocking), and as complexity without payoff (a worker boundary that adds maintenance cost for negligible gain). The judgment is to measure whether a worker helps, to minimize boundary cost, to partition and size work well, and to verify the main thread is actually freed. A worker is not free parallelism; it is a tradeoff that pays off only when the computation is heavy relative to the communication.

This skill covers the worker value tradeoff, the boundary and message interface, work partitioning and worker count, and main-thread verification. It complements the pwa-and-service-worker-architecture skill (service workers specifically), the animation-state-and-orchestration skill (main-thread animation work), and the browser-apis-and-capabilities skill (browser capabilities broadly). Here the focus is concurrency via workers and the computation/communication tradeoff.

## Core Rules

### Assess The Computation-To-Communication Ratio Before Using A Worker

A worker pays off only when the computation offloaded exceeds the cost of the worker boundary, and this ratio must be assessed, not assumed:

- **Measure whether the computation is heavy enough to justify a worker.** A worker has fixed costs (spawning, message passing, data copying); for light computation, the boundary cost exceeds the benefit. Profile the computation on the main thread first; if it is not actually causing jank, a worker is premature.
- **Account for data-copy cost.** Data sent to a worker is copied (structured clone) unless transferable; copying large data (big arrays, images) can block the main thread and negate the worker's benefit. Estimate the copy cost against the computation cost.
- **Use transferable objects to avoid copying.** ArrayBuffers and similar objects can be transferred (ownership moved, not copied) to and from a worker, eliminating copy cost for large binary data; design the interface to use transferables where the data is large.
- **Use SharedArrayBuffer for shared memory (where available).** SharedArrayBuffer lets the worker and main thread share memory without copying, but it requires a cross-origin-isolated context (COOP/COEP headers) and introduces synchronization complexity (Atomics); use it when the data is large and the copy is prohibitive, and handle synchronization carefully.

### Design The Worker Boundary And Message Interface For Low Overhead

The boundary between the main thread and the worker is where overhead lives, and its design governs efficiency:

- **Minimize the frequency and size of messages.** Each message has overhead; batch small messages into fewer larger ones, and avoid sending large data frequently. Design the interface for coarse, infrequent communication.
- **Choose the right data representation.** Sending a large object graph is expensive to clone; sending a compact binary representation (an ArrayBuffer) that the worker decodes is cheaper. Match the representation to the boundary.
- **Design for request/response and streaming patterns.** A worker that takes a request and returns a result is simple; a worker that streams results (incremental computation) can keep the main thread responsive during long work. Choose the pattern for the work's nature.
- **Handle errors and worker termination.** A worker can throw or be terminated; the main thread must handle worker errors gracefully (re-spawn, fall back) and clean up workers when no longer needed (terminate to free resources).

### Partition And Size Work Correctly

How work is divided among workers, and how many workers to use, determines efficiency:

- **Partition work to minimize coordination.** Workers that share state must coordinate (locking, messaging), which adds overhead and contention; partition work so each worker is independent (embarrassingly parallel) where possible.
- **Size work units to amortize overhead.** Too-small work units make the boundary overhead dominate; too-large units underutilize parallelism. Size work units so each is large enough to amortize messaging but small enough to parallelize and keep the main thread responsive.
- **Use an appropriate number of workers.** More workers than CPU cores adds context-switching overhead without throughput gain; use roughly one worker per core for CPU-bound work, and fewer if the work is I/O-bound or memory-bound. A worker pool avoids per-task spawn cost.
- **Beware memory and resource limits.** Each worker has its own memory and resource footprint; many workers multiply memory use and can hit browser limits. Size the worker count for memory as well as CPU.

### Ensure The Main Thread Is Actually Freed

The point of a worker is to free the main thread, and this must be verified, not assumed:

- **The main thread must not jank during the worker's work.** If the main thread copies large data to the worker, processes the results, or renders heavily, it janks despite the worker. Profile the main thread during the worker's execution.
- **Offload result processing where possible.** If the worker's results require heavy processing before rendering, do that processing in the worker too (return ready-to-render data), not on the main thread.
- **Use requestIdleCallback or scheduling for non-critical main-thread work.** Main-thread work that remains (small result processing, rendering) can be scheduled to avoid blocking interaction; use the scheduler for responsiveness.
- **Measure main-thread responsiveness, not just total time.** A worker can make total time longer (boundary overhead) while making the main thread more responsive; the goal is responsiveness, so measure frame rate and interaction latency, not only completion time.

## Common Traps

### Offloading Work That Doesn't Justify The Boundary Cost

Using a worker for computation too light to exceed the messaging and copying overhead, adding complexity for no gain. Measure whether the computation is heavy enough first.

### Data-Copy Overhead Dominating

Copying large data to/from the worker blocking the main thread and negating the worker's benefit. Use transferable objects or SharedArrayBuffer for large binary data.

### Main Thread Still Janking

The worker runs off the main thread, but copying data to it, processing results, or rendering on the main thread still janks. Profile the main thread; offload result processing to the worker.

### Too-Fine Or Too-Coarse Partitioning

Work units too small (boundary overhead dominates) or too large (underutilized parallelism, unresponsive main thread). Size units to amortize overhead while parallelizing.

### Coordination Overhead Or Deadlocks

Workers sharing state and contending or deadlocking on locks. Partition work to be independent; use SharedArrayBuffer with careful Atomics synchronization only when needed.

### Too Many Workers Multiplying Memory

Spawning many workers that each have a memory footprint, hitting browser limits or degrading the system. Size the worker count for memory and CPU; use a pool.

## Self-Check

- [ ] The computation-to-communication ratio was assessed: the computation is heavy enough to cause main-thread jank (profiled, not assumed), and the data-copy cost is accounted for (transferables or SharedArrayBuffer used for large data).
- [ ] The worker boundary and message interface are designed for low overhead: messages are coarse and infrequent (batched), the data representation is compact (binary where large), the pattern (request/response vs streaming) matches the work, and errors/termination are handled.
- [ ] Work is partitioned to minimize coordination (independent where possible), work units are sized to amortize overhead while parallelizing, the worker count is appropriate (roughly per-core for CPU-bound, fewer for I/O/memory-bound, using a pool), and memory limits are respected.
- [ ] The main thread is verified as actually freed: no jank during the worker's work (data copy, result processing, rendering offloaded or scheduled), result processing is done in the worker where possible, and responsiveness (frame rate, interaction latency) is measured, not only total time.
- [ ] SharedArrayBuffer (if used) requires a cross-origin-isolated context (COOP/COEP) and its synchronization (Atomics) is handled carefully to avoid races.
- [ ] The highest-risk cases were verified — a worker that doesn't help due to copy overhead, main-thread jank from result processing, coordination deadlock, memory blowup from too many workers — not only the "it runs in parallel" demo.
