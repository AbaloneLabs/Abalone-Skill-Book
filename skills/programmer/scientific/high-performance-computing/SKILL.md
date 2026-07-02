---
name: high_performance_computing.md
description: Use when the agent is running large-scale scientific or engineering computations on HPC clusters or supercomputers, writing MPI or hybrid MPI+OpenMP or MPI+CUDA code for distributed multi-node jobs, requesting scheduler resources (Slurm, PBS), designing checkpoint-restart for long jobs, managing parallel I/O at scale, or debugging jobs that fail, hang, underutilize nodes, or hit wall-clock limits on cluster infrastructure.
---

# High-Performance Computing

High-performance computing is the discipline of running computations that exceed the capacity of a single machine across a cluster or supercomputer — hundreds or thousands of nodes, each with many cores and often GPUs, connected by a specialized interconnect. The governing constraint is not raw compute but the ecosystem around it: jobs run under a scheduler that allocates nodes for a fixed wall-clock time, communication between nodes crosses a network whose latency and bandwidth are finite, parallel I/O to a shared filesystem is contended and slow, and a job that runs for days must survive node failures and wall-clock limits through checkpointing. A kernel that is correct and fast on one node can fail, hang, or waste most of its allocated resources at scale, because the scale exposes problems that single-node development never sees: communication deadlocks, I/O contention that serializes the whole job, load imbalance that leaves most nodes idle, and the sheer fact that a 1000-node job running for a week will experience a hardware failure somewhere and must recover from it.

Agents tend to approach HPC as "more of the same parallelism" — write the MPI or CUDA code, submit the job, collect the result — because the happy path runs on the cluster and the failures only appear at scale, under contention, or over long run times. The judgment problem is deciding how to request and use cluster resources efficiently (not wasting nodes, not hitting wall-clock limits), how to structure communication and I/O so they do not dominate at scale, how to checkpoint long jobs so a failure does not lose a week of computation, and how to diagnose why a large job failed or underperformed when the only evidence is a log file and a killed process. Getting this wrong produces jobs that queue for hours to run for minutes, that hang on communication, that lose days of work to an unhandled failure, or that use a thousand nodes to do the work of ten.

## Core Rules

### Understand The Cluster's Resource Model And Request Deliberately

An HPC cluster is a shared, scheduled resource. Jobs are submitted to a scheduler (Slurm, PBS/PBS Pro, LSF) which allocates nodes, cores, memory, and wall-clock time based on the request and the queue's policies. The request is a set of decisions — how many nodes, how many tasks per node, how much memory, how long — and each decision affects both whether the job runs and how efficiently it uses the resources.

Request resources deliberately against the job's actual needs:

- **Node and task count** should match the parallelism the job can actually exploit. Requesting 100 nodes for a job that scales to 20 wastes 80 nodes (and may violate the queue's fair-use policy), while requesting too few serializes work that could run in parallel. Know the job's scaling curve (from the parallel-scientific-computing skill) before requesting at scale.
- **Wall-clock time** should be estimated with margin, because the scheduler kills the job at the limit. A job that needs 10 hours requested as 8 will be killed at 8; a job requested as 24 "to be safe" may queue longer (queues prioritize shorter jobs) and blocks the allocation longer than needed.
- **Memory per node** must fit the job's per-node memory footprint, or the job is killed for exceeding the limit. Estimate the memory footprint (including communication buffers and temporary arrays) and request with margin.
- **GPUs and accelerators** are often a separate, scarcer resource with their own queues and limits; request them only when the job uses them.

A job request that does not match the job's actual needs either fails to run, runs inefficiently, or is killed. Treat the resource request as a design decision, not a guess.

### Minimize And Overlap Communication, Because The Interconnect Is The Bottleneck At Scale

On a single node, computation and memory access dominate. Across nodes, communication across the interconnect (InfiniBand, Slingshot, Ethernet) becomes the bottleneck, because every message has a latency (microseconds) and the aggregate bandwidth is finite and shared. A job that communicates frequently with small messages spends most of its time in communication latency; a job that communicates large messages saturates the bandwidth. Either way, communication — not computation — governs runtime at scale, and the parallel-scientific-computing skill's rules (aggregate messages, overlap communication with computation, minimize global synchronization) apply with full force.

HPC-specific communication practices:

- **Use collectives (MPI_Allreduce, MPI_Allgather) rather than point-to-point patterns** for global operations, because the MPI library implements them with topology-aware, logarithmic-time algorithms that are far faster than naive send/receive loops.
- **Overlap communication with computation** using non-blocking operations (MPI_Isend, MPI_Irecv) so a process computes on local data while boundary data is in transit. A bulk-synchronous job that communicates then computes then communicates serializes the communication; an overlapped job hides it.
- **Be aware of network topology.** Communication between nodes on the same switch is faster than across the cluster. Some schedulers and MPI implementations support topology-aware process placement that keeps communicating processes close; use it where available.
- **Avoid communication libraries' naive defaults** for large-scale runs; tune the MPI's collective algorithms and communication buffer sizes for the node count and interconnect.

### Use Parallel I/O Correctly, Because The Shared Filesystem Is Contended

HPC filesystems (Lustre, GPFS, BeeGFS) are parallel and shared, and naive I/O patterns that work on a local disk catastrophically contend at scale. A thousand processes each writing a separate small file creates a metadata storm that cripples the filesystem; a thousand processes each reading the same file sequentially serializes on the file server. Parallel I/O must be structured to use the filesystem's parallelism, not fight it.

Practices for scalable I/O:

- **Use parallel I/O libraries** (MPI-IO, HDF5 with parallel mode, NetCDF-4) that coordinate reads and writes across processes to a shared file, rather than each process writing its own file. A single shared file written with collective MPI-IO uses the filesystem's parallel data servers; a thousand separate files overload the metadata server.
- **Aggregate small writes into large ones.** Filesystems are optimized for large sequential transfers; many small writes are slow regardless of parallelism. Buffer output and write in large chunks.
- **Separate metadata-heavy operations from data transfers.** Opening and closing many files, or stat-ing many files, is a metadata operation that does not scale; minimize file counts.
- **Read input once and broadcast** (or use parallel read) rather than having every process read the input independently.

I/O that is not designed for parallelism becomes the bottleneck at scale, and a job that is compute-efficient but I/O-bound spends most of its wall-clock waiting on the filesystem.

### Checkpoint Long Jobs, Because Failures And Wall-Clocks Are Inevitable

A job that runs for hours or days will, with high probability, experience a failure (a node crash, a network error, a scheduler preemption) or hit its wall-clock limit. Without checkpointing, the entire computation is lost and must restart from the beginning. With checkpointing, the job periodically writes its full state to durable storage and can restart from the last checkpoint, losing at most the work since then.

Design checkpointing deliberately:

- **Checkpoint the full state needed to restart** — all process state, not just the primary data, including iteration counters, random number generator states, and communication context. A checkpoint that omits state produces a restart that diverges from the original.
- **Checkpoint at a frequency that balances overhead against restart cost.** Checkpointing has a cost (writing the full state to storage, which can be large and slow); checkpointing too often dominates runtime with I/O, too rarely loses too much work on failure. The optimal frequency balances the checkpoint cost against the expected lost work; a common heuristic is to checkpoint at intervals comparable to the checkpoint write time.
- **Use coordinated (synchronous) or uncoordinated checkpointing deliberately.** Coordinated checkpoints (all processes checkpoint together via a collective) are simpler and produce a consistent restart point but require global synchronization. Uncoordinated checkpoints let each process checkpoint independently but require message logging to produce a consistent restart, adding complexity.
- **Verify restart actually works.** A checkpoint that has never been used to restart may be corrupt or incomplete. Test the restart path before relying on it for a multi-day job.
- **Store checkpoints on the scratch or parallel filesystem**, not a slow network filesystem, and clean up old checkpoints to avoid filling the quota.

### Profile Node Utilization, Because Idle Nodes Are Wasted Allocation

An HPC allocation is expensive (in compute-hours, in queue wait, sometimes in real cost), and a job that leaves nodes idle is wasting that allocation. A job that requests 100 nodes but is load-imbalanced so that 50 finish early and wait at a barrier uses half its allocation. A job whose communication pattern leaves processes idle while others compute wastes the idle processes. Profiling at the node level — not just total runtime — reveals this waste.

Profile to find:

- **Load imbalance** — processes with unequal work, so the slowest governs runtime and the rest wait. Mitigate with better partitioning (the parallel-scientific-computing skill) or dynamic work stealing.
- **Communication idle time** — processes blocked in MPI calls waiting for messages, indicating communication-bound sections. Overlap or aggregate to reduce the blocking.
- **I/O idle time** — processes blocked on filesystem operations while others wait. Use parallel I/O and aggregate transfers.
- **Single-node underutilization** — within a node, cores or GPUs that are idle because the code is not using them (insufficient thread parallelism, serial sections, memory bandwidth limits). The node is allocated but not fully used.

A scaling and utilization profile tells you whether the job is using its allocation efficiently or wasting it, and where the waste is. A job run at scale without a utilization profile is an uncharacterized use of expensive resources.

### Handle The Ecosystem's Failure Modes

HPC jobs fail in ways single-machine jobs do not, and the code must handle them or lose the computation:

- **Node failures** (a node crashes, is preempted, or has a hardware error) kill the processes on that node. MPI can detect this (MPI errors become communicator failures); the job must either checkpoint-restart or tolerate the failure (fault-tolerant MPI, algorithmic resilience).
- **Wall-clock kills** terminate the job at the requested limit. The job must checkpoint before the limit (track elapsed time and checkpoint with margin) or lose the computation.
- **Filesystem quota and space limits** cause writes to fail. The job must handle write failures (a full scratch filesystem is common) and not corrupt its state.
- **Network errors and timeouts** can manifest as hung communication. Set MPI timeouts or heartbeat checks to detect hangs rather than waiting indefinitely.

Designing for these failure modes is not pessimism; it is the recognition that a 1000-node, week-long job will experience a failure somewhere, and the choice is between recovering from a checkpoint and losing the whole run.

## Common Traps

### Requesting Resources That Do Not Match The Job

Requesting too many nodes for a job that does not scale to them (wasting allocation and possibly violating queue policy), too little memory (killed for exceeding the limit), or too little wall-clock (killed at the limit). Match the request to the job's measured scaling, memory, and runtime.

### Communication-Bound At Scale Due To Frequent Small Messages

A job that runs fine on a few nodes but spends most of its time in communication at scale, because messages are small and frequent. Aggregate messages, use collectives, and overlap communication with computation.

### Naive I/O That Contends On The Filesystem

A thousand processes each writing separate small files, crippling the metadata server, or each reading the same file independently, serializing on the file server. Use parallel I/O libraries and aggregate transfers.

### No Checkpointing On A Long Job

Running a multi-day job without checkpoints, then losing the entire computation to a node failure or wall-clock kill. Checkpoint the full state at a sensible frequency and verify the restart path.

### Load Imbalance Leaving Nodes Idle

A partition that equalizes data but not work, so some nodes finish early and wait at a barrier while the slowest governs runtime. Profile utilization; partition by work.

### Assuming Single-Node Behavior Predicts Scale

Developing on one node and submitting at scale without testing the scaling, then discovering communication or I/O dominates or the job deadlocks at the new process count. Test scaling incrementally before the full-scale run.

### Untested Restart From Checkpoint

Relying on checkpoints that have never been used to restart, then discovering at the failure that they are corrupt or incomplete. Test the restart path before the long run.

### Ignoring The Scheduler's Policies And Quotas

Requesting resources or runtimes that violate queue limits, or filling the scratch filesystem with old checkpoints, causing the job to fail or queue indefinitely. Understand the cluster's policies; clean up.

## Self-Check

- [ ] The resource request (nodes, tasks per node, memory, wall-clock, GPUs) matches the job's measured scaling curve, memory footprint, and runtime with margin — not over-requested (wasteful, policy-violating) or under-requested (killed).
- [ ] Communication is minimized and overlapped with computation; collectives are used for global operations; messages are aggregated; and the communication pattern was tested at the target node count, not only on a few nodes.
- [ ] I/O uses parallel libraries (MPI-IO, parallel HDF5/NetCDF) with aggregated large transfers, not thousands of separate files or independent reads of shared files; metadata-heavy operations are minimized.
- [ ] Long jobs checkpoint their full state (including iteration counters and RNG state) at a frequency balancing overhead against restart cost, checkpoints are stored on the parallel filesystem, old checkpoints are cleaned up, and the restart path has been tested.
- [ ] Node utilization was profiled at scale: load imbalance, communication idle time, I/O idle time, and single-node underutilization were measured, and the dominant waste was addressed.
- [ ] The job handles cluster failure modes — node failures (checkpoint-restart or fault tolerance), wall-clock kills (checkpoint before the limit), filesystem quota (handle write failures), and network hangs (timeouts or heartbeats).
- [ ] Scaling was tested incrementally (not jumping from one node to a thousand) before the full-scale run, so communication, I/O, and deadlock issues were found at intermediate scale.
- [ ] The job's use of the cluster's policies, quotas, and scratch space is understood and respected, so it does not fail or queue indefinitely for administrative reasons.
