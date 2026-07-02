---
name: gpu_compute_and_compute_shaders.md
description: Use when the agent is writing compute shaders, dispatching GPU compute work, designing workgroup grids and shared-memory reductions, using barriers or atomics across threads, moving data between CPU and GPU for GPGPU work, implementing prefix sums or histograms or sorting on the GPU, or debugging compute kernels that produce wrong results, race conditions, bank conflicts, or poor occupancy.
---

# GPU Compute And Compute Shaders

Compute shaders run the same hardware as the graphics pipeline, but without the rasterizer's scaffolding. There is no vertex fetch, no fixed-function interpolation, no render target handed to you — you dispatch a grid of workgroups, each containing threads that share memory and synchronize, and you are responsible for how those threads cooperate, where their data lives, and how the result gets back to the CPU or into the next pass. This freedom is the point: compute lets you express algorithms the rasterizer fights (reductions, prefix sums, histogramming, sorting, simulation, spatial hashing). But the same freedom is where the bugs hide, because the correctness model is now yours to manage. A compute kernel that races on shared memory, deadlocks on a barrier, or reads a buffer before the producer wrote it produces wrong answers silently — the GPU does not throw, it just computes garbage at high speed.

Agents tend to write compute kernels as if they were CPU loops with more parallelism, because the happy path produces correct output and the failures only appear on specific workgroup sizes, specific data distributions, or when the kernel is reused in a different pipeline. The judgment problem is deciding how to map the problem onto a workgroup grid so threads cooperate efficiently, how to use shared memory and barriers without racing or deadlocking, how to move data across the CPU-GPU boundary without stalling the pipeline, and how to verify a compute result that has no visual output to inspect. Getting this wrong produces kernels that are correct on one GPU and wrong on another, that race intermittently, or that run slower than the CPU version because the dispatch was never the bottleneck.

## Core Rules

### Map The Problem Onto A Workgroup Grid That Fits The Hardware

A compute dispatch launches a grid of workgroups, each containing a fixed number of threads (the local size). The hardware executes each workgroup's threads together on a single compute unit, where they share fast on-chip memory and can synchronize. The mapping of your problem onto this grid governs both correctness and performance, and it is a structural decision.

Choose the workgroup size against the hardware. GPUs execute threads in lockstep groups (warps of 32 on NVIDIA, wavefronts of 32/64 on AMD), so workgroup sizes that are multiples of the warp/wavefront size avoid partial warps with idle threads. The total thread count per workgroup is bounded by hardware (typically 1024), and the optimal size depends on how much shared memory and how many registers each thread uses — more resources per thread means fewer workgroups can be resident simultaneously, reducing the hardware's ability to hide latency (occupancy). A workgroup size of 64 or 256 is a common starting point; benchmark to find what maximizes occupancy for your kernel's resource usage.

Map the problem so each workgroup's work is coherent and local. For an image filter, a 2D workgroup processes a tile so neighboring threads share a cache line. For a particle simulation, a workgroup processes a spatial cluster so neighbor lookups hit shared memory. A naive mapping that scatters a workgroup's threads across unrelated data thrashes the cache and defeats the purpose of the workgroup.

### Use Shared Memory For Intra-Workgroup Cooperation, But Avoid Bank Conflicts

Shared memory (also called local memory or thread-group shared memory) is fast on-chip storage shared by all threads in a workgroup. It is the primary mechanism for cooperation: threads load data from global memory into shared memory once, then each thread reads what its neighbors loaded, avoiding repeated global fetches. A tiled matrix multiply or a stencil computation that loads a tile into shared memory and reuses it can be an order of magnitude faster than the naive version that fetches global memory per access.

Shared memory is divided into banks (typically 32), and accesses are serviced in parallel as long as different threads access different banks. When two threads in the same warp access the same bank, the accesses are serialized — a bank conflict — and the kernel stalls. The classic trap is a stride-32 access pattern (each thread reading the next column of a row-major matrix) where every thread hits the same bank and the access serializes to 32x the expected time. Detect bank conflicts with a profiler and eliminate them by padding arrays (adding a dummy column so the stride is not a multiple of the bank count) or restructuring the access pattern.

### Synchronize With Barriers Correctly, And Never Race On Shared State

Within a workgroup, threads execute in warps that may run at different times. When threads cooperate through shared memory, you must insert barriers so that a thread does not read shared memory before the thread that writes it has finished. The memory barrier (`barrier`, `groupMemoryBarrier`, `workgroupBarrier`) guarantees two things: all threads in the workgroup reach the barrier before any proceeds (execution synchronization), and memory writes before the barrier are visible to threads after it (visibility synchronization). Omitting the barrier produces a data race — a thread reads stale or partially-written data — and the result is nondeterministic.

The rules that make barriers correct:

- **Every thread in the workgroup must hit the barrier unconditionally.** A barrier inside a conditional branch where only some threads take it deadlocks — the barrier waits for all threads, and some never arrive. This is the most common compute bug, and it hangs the GPU.
- **Pair writes with barriers.** If thread A writes shared memory and thread B reads it, a barrier between the write and the read is mandatory. No amount of "it works on my GPU" substitutes for the barrier; the race is real and will fail on different hardware or driver versions.
- **Use the finest barrier that suffices.** A full workgroup barrier is heavier than a memory-only barrier. Use memory barriers for visibility without execution synchronization when the threads do not need to wait for each other, only for the writes to be visible.

For cross-workgroup cooperation, there is no barrier — workgroups run independently and may execute in any order. Communication between workgroups requires global memory with atomics or multiple dispatches separated by a pipeline barrier. Do not assume workgroup A finishes before workgroup B; design the algorithm so the order does not matter, or split it into separate dispatches with an explicit barrier between them.

### Use Atomics For Contended Writes, But Know Their Cost

When multiple threads write to the same global or shared memory location (a counter, a histogram bin, a reduction accumulator), the writes race and the result is wrong. Atomic operations (`atomicAdd`, `atomicMin`, `atomicExchange`) serialize the conflicting accesses so they are correct, but serialization has a cost: many threads atomically updating the same location form a bottleneck, and the atomic throughput on a single address is far below the GPU's parallel capacity.

Mitigate atomic contention by reducing the number of threads that contend on the same address. For a histogram, give each workgroup (or each warp) a private histogram in shared memory, accumulate locally with no cross-thread contention, then atomically merge the private histograms into the global one once per workgroup. For a reduction, use a tree reduction within the workgroup (each thread reduces a pair, then half the threads reduce again) rather than having every thread atomically add to a single global sum. The pattern is always: reduce contention locally in shared memory, then merge globally with minimal atomics.

### Manage The CPU-GPU Data Transfer, Because It Is Often The Bottleneck

Compute work usually begins with data on the CPU and ends with a result the CPU needs. Each transfer crosses the PCIe bus (or unified memory boundary), and the transfer time often dominates the compute time for kernels that do little work per byte. A kernel that computes for 0.1 ms but requires a 5 ms upload and a 5 ms readback is 99% transfer-bound, and no optimization of the kernel itself will help.

Strategies to manage transfer cost:

- **Keep data on the GPU across kernels.** If a pipeline runs several compute passes, keep intermediate data in GPU memory and pass buffers between passes, rather than reading back to the CPU between each. The GPU-to-GPU path is fast; the GPU-to-CPU round trip is not.
- **Use pinned (page-locked) host memory for transfers** so the driver can use DMA and avoid an intermediate copy. Unpinned memory requires the driver to copy to a pinned staging buffer first, doubling the transfer cost.
- **Overlap transfer and compute** with double-buffering: while the GPU computes on buffer A, the CPU uploads data into buffer B, then swap. This hides transfer latency behind compute, but requires careful synchronization so the GPU never reads a buffer the CPU is still writing.
- **Avoid readback unless necessary.** Many compute results can be consumed by the next GPU pass (a render pass that reads the computed data) without ever going back to the CPU. Readback forces a full pipeline stall (the CPU waits for the GPU to finish), which destroys parallelism.

### Insert Pipeline Barriers Between Dependent Passes

When one GPU pass (compute or graphics) produces data that another consumes, a pipeline barrier (or memory barrier in the API) is required to ensure the producer's writes are complete and visible before the consumer reads. Without the barrier, the consumer may read stale or partially-written data, producing corruption. With too many barriers, the pipeline stalls waiting for each barrier to complete, serializing passes that could have overlapped.

Insert barriers only where there is a genuine data dependency, and use the most precise barrier available (a barrier scoped to the specific buffer or image and the specific access types) rather than a global pipeline stall. Batch independent passes before a barrier so the barrier is hit once for several consumers. A frame that inserts a barrier after every compute pass, when one barrier before the consuming render pass would suffice, pays the synchronization cost many times over.

## Common Traps

### Branching Into A Barrier So Not All Threads Arrive

Placing a `barrier()` inside an `if` where only some threads take the branch, so the barrier waits for threads that never arrive and the workgroup hangs. Every thread in the workgroup must reach the barrier unconditionally; move the barrier outside the branch or restructure so the condition is uniform.

### Omitting The Barrier Between Shared-Memory Write And Read

Thread A writes shared memory, thread B reads it, and no barrier separates them — a data race that produces nondeterministic results and fails on different hardware. Pair every shared-memory handoff with a barrier; the race is real even if it has not bitten yet.

### Stride-32 Access Causing 32-Way Bank Conflicts

Each thread reading consecutive columns of a row-major shared-memory array (stride 32), so every thread hits the same bank and the access serializes to 32x. Pad the array or restructure the access so the stride is not a bank-conflict multiple.

### Every Thread Atomically Updating One Global Counter

Thousands of threads all calling `atomicAdd` on a single global sum, serializing on that address and making the reduction slower than a CPU loop. Reduce locally in shared memory with a tree reduction, then merge globally with one atomic per workgroup.

### Reading Back To The CPU After Every Compute Pass

Forcing a GPU-to-CPU readback between each kernel, stalling the pipeline and making the transfer dominate. Keep data on the GPU across passes; read back only the final result, or consume it in a subsequent GPU pass.

### Assuming Workgroup Execution Order

Writing a kernel that assumes workgroup (0,0) runs before workgroup (1,0), so a global-memory handoff between them races. Workgroups run in any order; design for order-independence or split into separate dispatches with an explicit barrier.

### Unpinned Host Memory For Large Transfers

Using pageable host memory for uploads, forcing the driver to copy to a pinned staging buffer and doubling the transfer time. Use pinned memory for transfers that matter.

### Treating Compute As Free Because It Is "Not The Graphics Pipeline"

Adding compute passes without accounting for their dispatch cost, their barrier cost against subsequent passes, and their transfer cost, then wondering why the frame regressed. Compute shares the GPU's bandwidth and time budget with graphics; account for all of it.

## Self-Check

- [ ] The workgroup size is chosen against the hardware (a multiple of the warp/wavefront size, sized for occupancy given the kernel's shared-memory and register usage), and the problem is mapped so each workgroup's threads access coherent, local data.
- [ ] Shared memory is used for intra-workgroup reuse of global data, and the access pattern was checked for bank conflicts (no stride-32 patterns); padding or restructuring was applied where conflicts were found.
- [ ] Every shared-memory write-to-read handoff is protected by a barrier, and no barrier sits inside a conditional branch where threads might not arrive — the workgroup-hang bug was ruled out.
- [ ] Cross-workgroup communication does not assume execution order; algorithms are order-independent or split into separate dispatches with explicit pipeline barriers between them.
- [ ] Atomics are used for contended writes but contention is minimized — local reductions in shared memory feed a small number of global atomics, rather than every thread contending on one address.
- [ ] CPU-GPU transfers are minimized (data stays on the GPU across passes where possible), use pinned memory where they matter, and are overlapped with compute via double-buffering; readback is avoided unless the CPU genuinely needs the result.
- [ ] Pipeline barriers are inserted only where a genuine data dependency exists, scoped precisely to the resource and access type, and batched so independent passes share a single barrier rather than each triggering a stall.
- [ ] The compute result was verified independently of the graphics pipeline (a checksum, a CPU reference comparison, a debug readback), because a wrong compute kernel produces no visual artifact to catch it.
