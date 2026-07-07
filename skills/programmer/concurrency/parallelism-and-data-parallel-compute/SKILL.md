---
name: parallelism_and_data_parallel_compute.md
description: Use when the agent is parallelizing computation across multiple cores, threads, or workers — CPU-bound workloads, batch processing, map-reduce style fan-out, parallel data processing, SIMD/vectorization, GPU compute, or deciding how to split work into parallel tasks; choosing between task parallelism and data parallelism; reasoning about Amdahl's and Gustafson's laws, speedup limits, scheduling and work-stealing, granularity and overhead, or diagnosing why a parallelized workload is not faster than sequential. Covers parallel speedup limits, work distribution and load balancing, granularity tradeoffs, synchronization overhead, and the difference between concurrency (structure) and parallelism (execution).
---

# Parallelism And Data Parallel Compute

Parallelism is the use of multiple execution units to do work faster, and its central law — Amdahl's law — is unforgiving: the speedup from parallelism is limited by the fraction of the work that remains sequential. A workload that is 90% parallelizable has a maximum speedup of 10x regardless of how many cores you throw at it; the remaining 10% serial portion caps the gain. This is the first and most often-missed truth of parallelism: throwing cores at a problem yields diminishing returns governed by the serial fraction, and a workload that is not actually parallel (one with dependencies, shared state, or a serial bottleneck) will not speed up, no matter how many workers are assigned. The second truth is overhead: parallelism introduces coordination cost — task scheduling, work distribution, synchronization, and result merging — and if the per-task work is too small, the overhead exceeds the parallel benefit and the parallel version is slower than the sequential one.

Agents tend to conflate concurrency (structuring a program as multiple independent activities) with parallelism (executing them simultaneously for speed), and to parallelize work without analyzing the serial fraction, the task granularity, or the overhead. The judgment problem is recognizing that parallel speedup is governed by laws that do not yield to enthusiasm, that the work must be partitionable into independent chunks of the right granularity, and that load imbalance, synchronization, and overhead are the practical limits that determine whether parallelism helps. This skill covers the discipline of parallelizing compute effectively: analyzing the speedup ceiling, partitioning and balancing work, choosing granularity, and avoiding the overhead and imbalance that make parallel code slower than serial.

## Core Rules

### Analyze The Speedup Ceiling Before Parallelizing

Before parallelizing, determine the maximum speedup achievable, governed by the serial fraction. This tells you whether parallelism is worth the effort at all.

- **Identify the serial fraction.** What portion of the work cannot be done in parallel — setup, a dependency chain, a merge step, a shared resource? The serial fraction caps the speedup: with serial fraction s, the maximum speedup on N cores is 1/(s + (1-s)/N), approaching 1/s as N grows.
- **Use Amdahl's law to set expectations.** A workload with a 10% serial fraction tops out at 10x speedup, however many cores you add. If the requirement is 100x speedup, parallelism alone will not deliver it; the serial fraction must be reduced (algorithmic change, restructuring the dependencies) before parallelism can help further.
- **Consider Gustafson's law for workloads that scale with resources.** For workloads where the problem size grows with available compute (more data processed when more cores are available), the effective serial fraction shrinks and speedup can be more favorable. Distinguish "fixed problem, faster" (Amdahl) from "more resources, bigger problem" (Gustafson).
- **Do not parallelize a workload whose serial fraction is high.** If 50% of the work is serial, the maximum speedup is 2x, rarely worth the complexity. Reduce the serial fraction first, or accept the sequential performance.

### Partition Work Into Independent Chunks Of The Right Granularity

Parallelism requires that the work be divisible into chunks that can be executed independently. The chunk size — granularity — must balance parallelism against overhead.

- **Ensure chunks are independent (no dependencies, minimal shared state).** Chunks that depend on each other's results cannot run in parallel; chunks that share mutable state require synchronization that serializes them. The ideal parallel chunk reads its input, computes its output, and writes to a distinct location, with no cross-chunk coordination.
- **Choose granularity large enough that per-task overhead is negligible.** Each parallel task has scheduling and synchronization overhead (creating the task, assigning it to a worker, merging the result). If the task's work is smaller than its overhead, parallelism is slower than serial. Chunk the work so each task does substantially more work than its overhead — often milliseconds of computation, not microseconds.
- **Choose granularity small enough that work is evenly distributed.** If chunks are too large, there are too few to distribute across cores, and some cores idle while others finish their large chunk. There must be more chunks than cores, and ideally many more, so load balancing can keep all cores busy.
- **Balance granularity between the two extremes.** Too fine: overhead dominates. Too coarse: load imbalance dominates. The right granularity is typically found by experimenting with chunk sizes and measuring, not by calculation.

### Balance The Load Across Workers

Parallel speedup is limited by the slowest worker: if one chunk takes longer than the others, all workers finish and wait for it, and the speedup is governed by the longest chunk, not the average. Load imbalance is a primary limiter of real-world parallel speedup.

- **Use dynamic scheduling (work-stealing, work queue) for variable chunk sizes.** When chunks vary in duration (uneven input sizes, early-exit conditions), static assignment (round-robin) leaves fast workers idle waiting for slow ones. A shared work queue or work-stealing (idle workers take work from busy ones) balances the load dynamically.
- **Use static partitioning for uniform chunk sizes.** When chunks are uniform, static assignment (each worker gets a contiguous range) avoids the overhead of dynamic scheduling. Match the scheduling strategy to the chunk uniformity.
- **Avoid the "long pole" — a single chunk much larger than the rest.** One chunk that takes 10x as long as the others limits speedup to the time of that chunk. Split large chunks, or restructure so chunk sizes are uniform.
- **Account for stragglers in distributed parallelism.** In distributed map-reduce or cluster compute, a slow or failed worker (a straggler) delays the whole job. Speculative execution (running a duplicate of the slow task on another worker) and backup tasks mitigate stragglers.

### Minimize Synchronization And Shared State

Synchronization — locks, barriers, atomic operations, result merging — is where parallel execution becomes serial, and it is a primary limiter of speedup. Every synchronization point is a place where workers wait rather than compute.

- **Prefer embarrassingly parallel structures: each worker reads input, computes, writes distinct output, with no coordination.** The ideal parallel workload has no shared mutable state and no synchronization; results are merged once at the end. Structure the computation to maximize independence.
- **Avoid locks on the hot path of a parallel computation.** A contended lock serializes the workers contending for it, collapsing speedup. Use lock-free structures, per-worker accumulators merged at the end, or partition the state so workers do not contend.
- **Minimize barrier synchronization.** A barrier (all workers wait until all reach it) serializes at the barrier; the slowest worker determines the barrier time. Reduce the number of barriers; prefer asynchronous merging over barrier-based synchronization where possible.
- **Merge results efficiently.** Combining per-worker results into a final result is itself work; a naive merge (one thread combining all results serially) can be a serial bottleneck. Use a tree-based or associative merge (workers merge pairwise, then pairs merge, etc.) to parallelize the merge.

### Distinguish Task Parallelism From Data Parallelism, And Use SIMD/GPU Where Appropriate

Parallelism comes in forms suited to different workloads, and the hardware offers specialized parallelism (SIMD, GPU) for data-parallel work that general-purpose threading cannot match.

- **Task parallelism: different workers do different tasks.** Suited to workloads with diverse, independent operations (a pipeline of stages, a set of unrelated jobs). Coordinate via task graphs or schedulers.
- **Data parallelism: workers do the same operation on different data.** Suited to uniform computation over large datasets (matrix operations, image processing, bulk transforms). The natural fit for SIMD and GPU.
- **Use SIMD/vectorization for data-parallel computation on a single core.** Modern CPUs operate on vectors (multiple data per instruction); auto-vectorization and explicit intrinsics can speed up uniform numeric loops by 4-8x. Ensure loops are vectorizable (no dependencies between iterations, contiguous data, predictable control flow).
- **Use GPU compute for large-scale data parallelism.** GPUs offer massive data parallelism (thousands of cores) for workloads that fit the model (uniform computation, high arithmetic intensity, transferable data). The overhead of transferring data to and from the GPU means GPU compute pays off only for sufficiently large or compute-intensive work.

## Common Traps

### Parallelizing Without Checking The Serial Fraction

Throwing cores at a workload with a high serial fraction, achieving little speedup despite the effort. Analyze the speedup ceiling first; reduce the serial fraction if the ceiling is too low.

### Tasks Too Fine-Grained (Overhead Dominates)

Parallelizing into tiny tasks whose scheduling and synchronization overhead exceeds their work, making the parallel version slower than serial. Increase granularity so per-task work dominates overhead.

### Tasks Too Coarse-Grained (Load Imbalance)

Too few large tasks to distribute across cores, leaving workers idle while one finishes a long chunk. Use more, smaller chunks and dynamic scheduling.

### Contended Lock On The Hot Path

A lock that all workers contend for, serializing the parallel computation and collapsing speedup. Eliminate shared state, use per-worker accumulators, or partition the state.

### Long Pole Limiting Speedup

One chunk much larger than the rest, so all workers finish and wait for it. Split large chunks; aim for uniform chunk sizes.

### Barrier-Heavy Synchronization

Frequent barriers where the slowest worker determines the wait, serializing at each barrier. Reduce barriers; use asynchronous merging.

### Serial Result Merge

Combining per-worker results in a single serial merge that becomes the bottleneck. Use tree-based or associative parallel merge.

### Ignoring SIMD/GPU For Data-Parallel Work

Using general-purpose threads for uniform numeric computation that SIMD vectorization or GPU compute would accelerate far more. Match the parallelism form to the workload.

## Self-Check

- [ ] The serial fraction of the workload is identified, Amdahl's law is applied to set the speedup ceiling, and parallelism is pursued only where the ceiling justifies the effort (the serial fraction is reduced first if the ceiling is too low).
- [ ] Work is partitioned into independent chunks (no cross-chunk dependencies, minimal shared state) of a granularity that balances overhead (chunks large enough that per-task overhead is negligible) against load balance (enough chunks to keep all workers busy).
- [ ] Load is balanced across workers — dynamic scheduling (work queue, work-stealing) for variable chunk sizes, static partitioning for uniform chunks, large/long-pole chunks split, and stragglers handled (speculative execution) in distributed settings.
- [ ] Synchronization and shared state are minimized: embarrassingly parallel structure preferred, no contended locks on the hot path (per-worker accumulators, partitioned state, lock-free structures), minimal barrier synchronization, and efficient (tree-based/associative) result merging.
- [ ] The form of parallelism (task vs data) matches the workload, and SIMD vectorization and GPU compute are considered for large-scale uniform data-parallel computation where they offer speedups general threading cannot.
- [ ] The parallel implementation is benchmarked against the sequential baseline (not assumed faster), with measurement of actual speedup, scaling with core count, and identification of the limiter (serial fraction, overhead, imbalance, or synchronization) when speedup falls short of the ceiling.
- [ ] Granularity was tuned by experiment (varying chunk size and measuring), not by guess, to find the point where overhead and load imbalance are both acceptable.
- [ ] The workload's data layout supports the parallelism (contiguous, aligned data for SIMD; coalesced access for GPU; cache-friendly access patterns) rather than defeating it with poor locality.
