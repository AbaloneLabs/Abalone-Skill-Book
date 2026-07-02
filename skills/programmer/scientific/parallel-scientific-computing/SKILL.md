---
name: parallel_scientific_computing.md
description: Use when the agent is parallelizing scientific or numerical code with MPI, OpenMP, CUDA, OpenCL, or distributed arrays; designing domain decompositions or data partitions; reducing communication overhead in distributed solvers; reasoning about Amdahl's law, strong vs weak scaling, or reproducibility of parallel reductions; debugging race conditions, deadlocks, load imbalance, or results that differ between process counts; or deciding between shared-memory and distributed-memory parallelism for a computational kernel. Covers data parallelism, communication cost, load balancing, and the reproducibility limits of floating-point reduction.
---

# Parallel Scientific Computing

Parallel scientific computing is governed by two facts that fight each other: computation scales with processor count, but communication between processors does not scale away — and for most real scientific kernels, communication (not computation) is the bottleneck at scale. Amdahl's law then bites hard: the serial fraction (communication, synchronization, the unparallelizable part) caps the achievable speedup, and a kernel that is 95% parallel tops out at 20x speedup no matter how many processors you add. Worse, floating-point reduction is non-associative, so the same parallel computation gives different results depending on the process count and the reduction tree, which destroys bitwise reproducibility — a serious problem for verification, regression testing, and any workflow that must reproduce a published result. The difficulty is that the failure modes are emergent at scale: a kernel that parallelizes cleanly on 4 processes deadlocks or load-balances terribly on 1024, and the bug only appears in the run you cannot easily debug.

Agents tend to parallelize by sprinkling `#pragma omp parallel for` or wrapping a loop in MPI calls, treating parallelism as a performance knob rather than a restructuring of the algorithm around data ownership and communication. The judgment problem is deciding what parallelism model fits the problem (shared memory, distributed memory, GPU, and which combination), how to partition data so that computation is local and communication is minimized, how to balance load across processors with unequal work, and how to reason about reproducibility when the reduction order varies. Getting this wrong produces code that scales to 8 cores and degrades on 64, deadlocks intermittently, gives different answers on different run sizes, or spends 90% of its time in communication and never approaches the theoretical speedup.

## Core Rules

### Match The Parallelism Model To The Hardware And The Problem

The choice between shared-memory (OpenMP, threads, OpenACC on one node), distributed-memory (MPI across nodes), GPU (CUDA, OpenCL, HIP, SYCL), and combinations is a structural decision driven by the data size, the hardware available, and the communication pattern.

- **Shared-memory** parallelism is simplest and lowest-latency within a node, but limited to the cores and memory on one machine. Best for kernels that fit in a node's memory and parallelize across cores.
- **Distributed-memory** (MPI) is required when the data exceeds one node's memory or when you need many nodes. It introduces explicit communication (message passing), which is the dominant cost at scale, and requires the algorithm to be restructured around data ownership.
- **GPU** parallelism is data-parallel and high-throughput but requires the kernel to map to thousands of threads with coherent access patterns; not all algorithms fit (irregular, branchy, or scatter-heavy kernels often do not).
- **Hybrid** (MPI + OpenMP, or MPI + CUDA) is common in modern HPC: MPI between nodes, shared memory or GPU within a node. The hybrid adds complexity but reduces communication (fewer, larger messages).

Pick the model against the data size, the hardware, and the kernel's structure. The wrong model caps performance for the life of the code; restructuring from shared-memory to distributed-memory late is expensive.

### Partition Data To Minimize Communication, Because Communication Dominates At Scale

In distributed-memory parallelism, each process owns a portion of the data (a domain decomposition or block of a distributed array), and computation that needs another process's data requires communication. Communication latency is high (microseconds per message) and bandwidth is finite, so the dominant cost at scale is almost always the volume and frequency of communication, not the computation.

Minimize communication by: choosing a decomposition that keeps each process's needed data local (a 1D slab decomposition for stencil codes minimizes surface area; a 2D or 3D decomposition reduces it further for large problems), aggregating small messages into large ones (latency-bound small messages are far worse than bandwidth-bound large ones), overlapping communication with computation (non-blocking sends/receives, compute on interior points while boundary data is in flight), and reducing the frequency of global synchronizations (which force every process to wait for the slowest). The halo/ghost-cell exchange in stencil codes is the canonical example: minimize the halo width, pack it into one message, and overlap it with interior computation. A kernel that communicates per-iteration per-point will not scale; one that communicates per-iteration per-boundary will.

### Apply Amdahl's Law Honestly To Predict And Diagnose Scaling

Amdahl's law: if a fraction `p` of the work is parallelizable and `1-p` is serial, the maximum speedup on `N` processors is `1 / ((1-p) + p/N)`. A kernel that is 95% parallel (`p = 0.95`) tops out at 20x speedup; 99% parallel tops out at 100x. The serial fraction — communication, synchronization, serial setup, I/O — caps scaling, and the cap is brutal.

Use Amdahl's law to set expectations and to diagnose. If a kernel scales well to 16 processes and poorly to 64, the serial fraction is the limit; profile to find it (usually communication, a global reduction, or I/O). Distinguish strong scaling (fixed problem size, more processors) from weak scaling (problem size grows with processors). Most scientific kernels weak-scale better than they strong-scale, because per-process work stays constant; strong scaling hits Amdahl's limit faster because the serial fraction is fixed while the parallel work shrinks. Report both when evaluating a parallel kernel.

### Balance Load Across Processes, Because The Slowest Process Governs Runtime

In a bulk-synchronous parallel program (most scientific codes), every process synchronizes at communication points, so the runtime is governed by the slowest process. If one process has twice the work of the others, the whole program waits for it, and you get half the speedup you expected. Load imbalance is one of the most common and most fixable scaling limits.

Sources of imbalance: non-uniform physics (a region with reactions or phase change does more work per cell than a uniform region), adaptive mesh refinement (refined regions have more work), irregular geometry (some processes own empty space). Mitigations: partition to equalize work (not just cell count — use a weighted partitioner that accounts for per-cell cost), dynamic load balancing (re-partition periodically as the work distribution changes), and oversubscription (give each process many small pieces so imbalance averages out, at the cost of more communication). A partition that equalizes cell count but not work will not scale; measure work per process, not just data per process.

### Reason About Reproducibility, Because Parallel Reduction Is Non-Associative

Floating-point addition is not associative, so the result of a parallel reduction (sum, dot product, norm) depends on the reduction tree, which depends on the number of processes and the data distribution. The same computation on 4 processes and 8 processes gives different results in the last digits. For an iterative solver, those last digits can amplify over iterations and produce visibly different trajectories. This is not a bug; it is a property of floating-point parallel reduction, and it has real consequences for verification and regression testing.

Decide explicitly what reproducibility the workflow requires. For bitwise reproducibility (required by some verification and publication workflows), fix the reduction order (same process count, same tree, deterministic algorithms) or use a reproducible reduction (compensated sum with a fixed tree, or fixed-point accumulation). For statistical reproducibility (most cases), accept the last-digit variation and test with tolerances scaled to the expected rounding. Do not assume parallel results match a serial reference exactly; they will not, and chasing the difference is chasing a property of floating-point arithmetic.

### Avoid Deadlocks And Races By Structuring Communication Symmetrically

MPI programs deadlock when processes wait for messages that arrive in a different order than they are sent — process A blocks on a receive from B while B blocks on a receive from C, and no message flows. The defense is symmetric communication: every process issues its sends and receives in the same order, use non-blocking communication (`Isend`/`Irecv`/`Waitall`) to avoid ordering dependencies, or use collective operations (`Allreduce`, `Allgather`) that the library guarantees do not deadlock.

Race conditions in shared-memory or GPU code produce nondeterministic results: two threads writing the same location without synchronization, or a reduction whose order varies. Use atomics or critical sections for contested writes, and use deterministic reductions where reproducibility matters. A parallel program that gives different results on repeated identical runs has a race; find it before relying on the results.

### Measure With The Right Metrics: Time, Scaling, Efficiency, Roofline

Parallel performance is reported as wall-clock time, speedup (relative to serial or to a baseline process count), parallel efficiency (speedup divided by processor count — 100% is perfect scaling), and the roofline (achieved FLOPs/s against the hardware's peak, plotted against arithmetic intensity). A single metric misleads: a kernel can show good speedup but poor efficiency (it uses many processors wastefully), or good efficiency but poor absolute performance (it scales but is slow).

Report scaling curves (speedup vs process count) for both strong and weak scaling, profile to identify the bottleneck (computation, communication, load imbalance, memory bandwidth), and compare achieved performance to a roofline model to know whether the limit is computation, memory, or communication. A parallel kernel without a scaling curve and a profile is not characterized; you do not know whether it will scale to the target process count.

## Common Traps

### Sprinkling Parallel Pragmas Without Restructuring Around Data Ownership

Adding `#pragma omp parallel for` or MPI calls to a serial kernel and expecting speedup, then getting poor scaling because the data access pattern is not local or the communication dominates. Parallelism requires restructuring around data ownership and communication, not annotating a serial loop.

### Communicating Per-Point Instead Of Per-Boundary

Sending a message per grid point or per iteration per neighbor, paying the per-message latency thousands of times, instead of aggregating into one boundary message per iteration. Aggregate communication; latency-bound small messages are far worse than bandwidth-bound large ones.

### Hitting Amdahl's Limit And Calling It A Bug

Watching speedup flatten at 16 processes and assuming the code is broken, when the serial fraction (a global reduction, I/O, serial setup) caps the speedup per Amdahl's law. Profile to find the serial fraction; if it is intrinsic, the cap is expected.

### Equalizing Cell Count Instead Of Work

Partitioning so each process has the same number of cells, then watching one process with heavier physics run twice as long and govern the runtime. Partition to equalize work, not data; weight the partition by per-cell cost.

### Expecting Bitwise Reproducibility Across Process Counts

Running the same solver on 4 and 8 processes and treating the last-digit difference as a bug. Parallel floating-point reduction is non-associative; decide what reproducibility is required and test with appropriate tolerances.

### Deadlocking On Ordered Blocking Communication

Having each process block on a receive before sending, so processes wait in a cycle and the program hangs. Use non-blocking communication, symmetric send/receive ordering, or collectives that the library guarantees deadlock-free.

### Reporting Speedup Without Efficiency Or A Profile

Claiming "16x on 64 processors" as a success, when 25% efficiency means 75% of the processors are mostly idle and a profile would show communication or load imbalance as the limit. Report scaling curves and profiles; a single number misleads.

### Assuming A GPU Will Speed Up Any Kernel

Moving an irregular, branchy, or scatter-heavy kernel to the GPU and getting worse performance than on the CPU, because the kernel does not map to coherent data-parallel threads. Match the kernel's structure to the hardware; GPUs reward regular, coherent, data-parallel work.

## Self-Check

- [ ] The parallelism model (shared-memory, distributed-memory, GPU, hybrid) was chosen against the data size, hardware, and kernel structure — not by familiarity — and the choice is documented as structural, not incidental.
- [ ] The data partition minimizes communication: each process's needed data is local, messages are aggregated into large boundary exchanges, communication is overlapped with computation where possible, and global synchronizations are minimized.
- [ ] Amdahl's law was applied to predict scaling and to diagnose the serial fraction when scaling plateaus; both strong and weak scaling were measured and reported.
- [ ] Load is balanced by work (not cell count), using a weighted partitioner or dynamic rebalancing; work per process was measured and found roughly equal.
- [ ] The required level of reproducibility (bitwise or statistical) was decided explicitly; parallel reductions use deterministic order or fixed-point/compensated accumulation where bitwise reproducibility is required, and tests use tolerances scaled to expected rounding otherwise.
- [ ] Communication is structured to avoid deadlocks (non-blocking, symmetric ordering, or collectives), and shared-memory/GPU writes are synchronized (atomics, critical sections) to avoid races; repeated identical runs produce identical or tolerance-consistent results.
- [ ] Performance is reported as scaling curves (speedup and efficiency vs process count) with a profile identifying the bottleneck (computation, communication, imbalance, memory), and compared to a roofline model.
- [ ] A scaling run at the target process count (not just 4 or 8 processes) confirmed the kernel meets its performance target and does not degrade, deadlock, or imbalance at scale.
