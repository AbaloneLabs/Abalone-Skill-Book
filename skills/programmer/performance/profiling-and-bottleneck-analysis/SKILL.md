---
name: profiling_and_bottleneck_analysis.md
description: Use when the agent is investigating why a system, endpoint, job, or function is slow; deciding what to optimize; reading a CPU, memory, allocation, or I/O profile or flame graph; choosing a profiling or benchmarking tool; interpreting latency percentiles or throughput metrics; diagnosing high CPU, memory pressure, GC pauses, lock contention, or disk/network bottlenecks; deciding whether a hotspot is worth optimizing; writing or reviewing a microbenchmark; or planning performance work before and after a change. Also covers Amdahl's law, the distinction between a hot line and a real bottleneck, measurement before optimization, production versus development profiling, benchmark validity, and the cost of over-optimization.
---

# Profiling And Bottleneck Analysis

Performance work is a search for the real bottleneck, not a hunt for any code that looks expensive. A profiler points at where time or memory is spent, but "where the profiler is loudest" and "where a change will move the needle" are different questions. Most performance failures are not failures of optimization skill; they are failures of diagnosis: optimizing the wrong thing, optimizing before measuring, or trusting a benchmark that does not resemble production. The result is effort spent, complexity added, and the latency number unchanged.

Agents tend to miss this because optimization feels productive the moment it starts. Rewriting a loop, inlining a function, or swapping a data structure produces a visible diff and a confident story, while the unglamorous work — measuring the current system, forming a hypothesis about the bottleneck, and re-measuring after the change — feels like delay. The harm is twofold: the change often does not help, and it leaves behind harder-to-read code that must now be maintained for no benefit. The judgment problem is deciding where the system actually spends its time, how much of that spend a given change can remove, and whether that removal is worth the cost — before writing any "fast" code.

This skill is about finding and prioritizing bottlenecks. It complements the complexity-analysis skill, which covers reasoning about algorithmic cost, and the query-optimization skill, which covers database access patterns. Here the question is the measurement and prioritization process itself: profile, identify, prioritize, change, re-measure.

## Core Rules

### Never Optimize Without A Measurement

The first rule is procedural, not technical: do not change code for performance until you have evidence that the code is the bottleneck and a measurement of how slow it currently is. Intuition about performance is reliably wrong in systems of any size, because the cost is dominated by layers the developer is not thinking about — memory allocation, cache misses, lock contention, I/O syscalls, serialization, garbage collection. A change made from intuition may be correct in principle and irrelevant in practice.

Before optimizing, establish:

- **A baseline number.** Latency (p50, p95, p99), throughput, CPU%, memory, allocation rate, or whatever metric defines "slow" for this system. A vague "it feels slow" is not a baseline.
- **Where the time or resources go.** A profile — CPU, wall-clock, allocation, I/O, lock — that attributes the cost to specific code.
- **A target.** What number would count as "fixed"? Without a target, optimization has no stopping condition and tends to run until the code is unreadable.

Optimizing without a baseline is the most common performance mistake. The change cannot be evaluated because there is nothing to compare against, and the work is driven by guesswork.

### Distinguish A Hotspot From A Real Bottleneck

A profiler shows where time is spent, but not all time spent is removable, and not all removable time matters. The real bottleneck is the code where (a) a large share of resources is spent and (b) a realistic change can remove a large share of that spend. A hotspot that satisfies only the first condition is a trap.

Apply these filters to every profile entry:

- **Is the cost on the critical path?** Code that runs in the background, off the request path, or outside the user's latency budget is rarely worth optimizing even if it burns CPU. Optimize what the user or the system's capacity constraint actually waits on.
- **Is the cost removable, or inherent?** A function that hashes a large buffer is "slow" because hashing is work; you cannot make it faster without hashing less. A function that is slow because it recomputes the same hash in a loop is removable. The profile looks similar; the opportunity is opposite.
- **How much of the total can this change remove?** A hotspot consuming 40% of runtime where you can halve it removes 20% of total time. A hotspot consuming 5% where you can eliminate it entirely removes 5%. Prioritize by total impact, not by local speedup.

This is Amdahl's law in practice: the speedup of the whole system is bounded by the fraction of time spent in the part you optimize. Spending a week doubling the speed of code that is 5% of runtime yields at most a 2.5% system improvement. Spend that effort on the 40% hotspot instead.

### Profile The Right Resource For The Symptom

"Slow" is ambiguous. Different symptoms point at different resources, and profiling the wrong one produces a profile that explains nothing. Match the profiler to the reported symptom:

- **High latency on a CPU-bound path** → CPU / wall-clock profile. Look for hot functions, expensive loops, repeated computation.
- **High latency with low CPU** → the process is waiting. Profile I/O, network, lock contention, or blocking syscalls (wall-clock or off-CPU profiling, not CPU profiling). A CPU profile of a blocked process shows nothing, which is itself a signal.
- **Memory growth or crashes** → allocation profile and heap dump. Look for retained object graphs, unbounded caches, leaked listeners, growing collections.
- **Latency spikes or stalls** → GC log / pause analysis, lock contention profile, or allocation rate. Spikes are usually caused by stop-the-world events (GC, compaction, lock acquisition storms), not by steady-state CPU cost.
- **Throughput ceiling** → identify the saturated resource (CPU, I/O, connections, thread pool, database connections, rate limit). Throughput stops growing when one resource is exhausted; profiling must find which.

Profiling CPU to explain an I/O-bound slowness, or profiling allocations to explain a CPU-bound slowness, yields a confident-looking answer to the wrong question. Name the symptom, then choose the profiler.

### Use The Right Granularity: Sampling, Instrumenting, And Tracing

Profilers differ in overhead and detail, and the choice changes what you can see:

- **Sampling profilers** (perf, async-profiler, py-spy, pprof) take snapshots at intervals with low overhead. Good for production and for finding hotspots in CPU or wall-clock time. They miss events that happen between samples (rare but expensive calls) and under-report very fast but very frequent functions.
- **Instrumenting profilers / tracing** (distributed tracing, OpenTelemetry, custom spans) add explicit measurement to named operations. Good for attributing latency across services and for measuring specific operations precisely. They impose overhead proportional to instrumentation density and can themselves slow the system if overused.
- **Allocation and event profilers** record every allocation or syscall. High detail, high overhead; use in development or short production windows, not continuously.

For cross-service latency, distributed tracing is usually required: a CPU profile of one service cannot explain why a request took 800ms when that service spent 50ms of CPU. Match the tool to the scope of the question — a single function, a single process, or a whole request path across services.

### Profile In Production, Or As Close As You Can Get

A profile taken in development routinely misleads, because development differs from production in exactly the ways that change performance:

- **Data volume and distribution.** Ten rows in a dev database hide the sequential scan that production's ten million rows trigger. A dev profile shows fast queries; production shows the real cost.
- **Concurrency.** A single local request has no lock contention, no connection-pool exhaustion, no cache eviction pressure. Production runs many requests at once, and the bottleneck is often the contention, not the computation.
- **Hardware and environment.** Different CPU, different memory, different disk (local SSD vs network-attached), different network latency. A microbenchmark on a laptop does not predict production throughput.
- **Realistic code paths.** Dev often exercises the happy path with warm caches; production exercises cold paths, error paths, and rare-but-expensive branches.

Prefer continuous profiling in production (low-overhead sampling profilers run continuously) where available. When that is not possible, reproduce the production condition as closely as possible: production-like data volume and skew, realistic concurrency, and the actual hardware class. A profile of a non-representative workload optimizes the wrong system.

### Read Latency In Percentiles, Not Averages

Average latency hides the experiences that matter most. A system with a 50ms average can have a 5-second tail, and the average will not move while real users time out. Performance work that targets the average optimizes the common case and leaves the tail untouched — but the tail is usually where the complaints come from.

Track and optimize:

- **Percentiles (p50, p95, p99, p99.9).** These show the spread. A flat p50 with a rising p99 means the typical request is fine and a minority is suffering; the fix is different from a uniformly slow system.
- **The worst cases, not just the typical.** Tail latency is driven by different causes than the median (GC pauses, lock contention, cold caches, retry storms, noisy neighbors). Optimizing the median often does nothing for the tail.
- **Throughput under the latency constraint.** "Requests per second" is only meaningful with a latency bound ("1000 rps at p99 < 200ms"). Throughput without a latency target rewards buffering and batching that destroy individual-request latency.

Beware the single-number summary. A dashboard showing only average latency will declare the system healthy during an outage that affects 1% of requests severely.

### Validate Every Benchmark Before Trusting It

A benchmark is a measurement of a tiny, artificial workload, and it lies easily. Before acting on a benchmark result, confirm it measures what it claims:

- **Warmup.** JIT compilation, class loading, cache filling, and branch-prediction warming make the first iterations unrepresentative. Discard warmup runs.
- **Stability.** Run enough iterations that the variance is small. A result that swings 30% between runs measures noise, not performance.
- **No dead-code elimination.** Compilers aggressively remove code whose result is unused. A benchmark that computes a value and discards it may be measuring nothing. Consume the result (print, hash, store in a volatile/global sink).
- **Realistic input and size.** Benchmarking with an empty array, a single element, or a constant the compiler can fold measures the benchmark harness, not the code. Use inputs at the realistic size and distribution.
- **Same environment as the comparison.** Comparing two approaches on different hardware, different load, or different compiler settings produces a meaningless delta.
- **What is actually being measured.** Microbenchmarks isolate a function from its real cost (allocation amortized elsewhere, cache state from prior work, I/O overlapped with computation). A function that benchmarks fast in isolation can be slow in context, and vice versa.

A microbenchmark is a hypothesis, not a verdict. It tells you what is possible under ideal conditions; production tells you what actually happens. Never ship an optimization based on a microbenchmark alone — re-measure the real system after the change.

### Re-Measure After Every Change, And Stop When The Target Is Met

Optimization is iterative, and each change must be verified against the baseline. A change that "should" be faster can be slower (cache effects, allocation shifts, branch prediction changes), neutral, or faster but not enough to matter. Confirm with a measurement, not with reasoning.

Define a stopping condition up front:

- **The target.** "p99 under 200ms." When the measurement meets it, stop.
- **The budget.** "Spend up to two days; if the bottleneck is not found or the target is not reachable, reassess." Open-ended optimization tends to consume whatever time is available.

Without a stopping condition, optimization drifts into diminishing returns: each successive change removes a smaller fraction of the remaining time, and the code becomes progressively harder to maintain for progressively smaller gains. The last 5% is not worth the complexity it costs. When the target is met, stop — even if more optimization is theoretically possible.

### Weigh The Optimization Against Its Cost

Performance is one property of code, traded against readability, maintainability, correctness, and time. An optimization that makes the code harder to read or change carries an ongoing cost that must be justified by the performance gain. Before committing an optimization:

- **Is the gain large enough to justify the complexity?** A 2x speedup on a hot path usually is; a 5% speedup on a cold path almost never is.
- **Is the optimized code still correct and testable?** Some optimizations (bit tricks, manual inlining, lock-free structures) are easy to get subtly wrong. The performance gain must be worth the added risk.
- **Will the next maintainer understand it?** An optimization that is incomprehensible will be "simplified" back to the slow version by someone who does not realize it was deliberate. Document the measurement that justified it.

Prefer the simplest change that meets the target. A cache, an index, or avoiding redundant work often beats a clever algorithm — and survives maintenance better.

## Common Traps

### Optimizing From Intuition Before Profiling

Reading the code, deciding "this loop looks slow," and rewriting it — without a profile showing the loop is on the critical path or consumes meaningful time. The loop is usually not the bottleneck; the change adds complexity and the latency number does not move. Always profile first.

### Treating The Loudest Profile Entry As The Bottleneck

The function at the top of the profile is where the most time is spent, but it may be inherent cost (a hash you must compute) or off the critical path (background work). Optimizing it yields a local speedup with no system impact. Filter profile entries by removability and total impact, not by raw size.

### Microbenchmarking In Isolation And Calling It Done

Benchmarking a function in a harness, seeing a 3x improvement, and shipping — then discovering production latency is unchanged because the function was 2% of the request, or because the real cost was I/O the benchmark did not include. A microbenchmark is a starting point; the real system must be re-measured.

### Profiling The Wrong Resource For The Symptom

Running a CPU profile to explain a request that is slow because it waits on the database. The CPU profile shows low usage (correctly — the CPU is idle while blocked) and offers no actionable signal. Match the profiler to the symptom: blocked latency needs off-CPU / I/O / tracing, not CPU sampling.

### Trusting A Development Profile For A Production Problem

Profiling locally with tiny data, no concurrency, and warm caches, concluding the system is fast, and being surprised when production is slow. Development hides exactly the conditions — volume, contention, cold paths — that create production bottlenecks. Profile production or a production-like environment.

### Averaging Away The Tail

Reporting and optimizing average latency while p99 is multiple seconds. The average looks fine, the dashboard is green, and real users time out. Always look at percentiles, and treat a wide spread between p50 and p99 as a distinct problem from a uniformly slow system.

### Benchmark That Measures Nothing (Dead-Code Elimination)

A benchmark whose result is unused, so the compiler removes the work being measured, and the benchmark reports the speed of an empty loop. The number looks great and proves nothing. Ensure the benchmark consumes its results in a way the compiler cannot optimize away.

### Ignoring Amdahl's Law

Spending days optimizing code that is 5% of runtime to run twice as fast, for a 2.5% system improvement, while a 40% hotspot goes untouched. Prioritize by the share of total time a change can remove, not by how clever the optimization is or how slow the local code looks.

### Optimizing Past The Target

Continuing to optimize after the latency target is met, because "it could be faster." Each further change adds complexity for shrinking gains. Define the target and stop when it is reached; preserve the remaining simplicity.

### Confusing Throughput With Good Latency

Reporting a high requests-per-second number achieved by batching and buffering, while individual requests queue and their latency balloons. Throughput without a latency constraint is not a performance win; it is a trade of latency for capacity that may not serve users.

## Self-Check

- [ ] A baseline measurement (specific metric and value) was recorded before any optimization, and a target value defines what "fixed" means.
- [ ] The bottleneck was identified from a profile of the relevant resource (CPU for CPU-bound, off-CPU/I/O/tracing for blocked latency, allocations for memory growth, GC/locks for spikes) — not from reading the code or intuition.
- [ ] Each candidate hotspot was filtered by removability and total impact (Amdahl's law), and the optimization prioritizes the change that removes the largest share of total time, not the locally slowest code.
- [ ] The profile was taken in production or a production-like environment (realistic data volume, concurrency, hardware), not from a tiny development workload.
- [ ] Latency is tracked in percentiles (p50/p95/p99), and the tail is investigated as a separate problem from the median; no decision relies on average latency alone.
- [ ] Any benchmark used to justify a change was validated: warmup discarded, variance low, results consumed to defeat dead-code elimination, realistic input size, same environment as the comparison.
- [ ] After each change, the real system was re-measured against the baseline to confirm the improvement actually materialized and was large enough to justify the added complexity.
- [ ] A stopping condition (target met or time budget exhausted) was honored; optimization did not continue past diminishing returns into unreadable code.
- [ ] The optimized code remains correct, tested, and documented with the measurement that justified it, so a future maintainer understands why it is not the simple version.
- [ ] The chosen optimization is the simplest one that meets the target (cache, index, avoiding redundant work) rather than a clever algorithm, unless the simpler option provably cannot meet the constraint.
