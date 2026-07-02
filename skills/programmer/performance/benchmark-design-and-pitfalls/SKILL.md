---
name: benchmark_design_and_pitfalls.md
description: Use when the agent is designing or running benchmarks, microbenchmarks, or performance comparisons, interpreting benchmark results, claiming a speedup, choosing a baseline, or reviewing whether a performance claim is trustworthy. Covers microbenchmark traps (dead code elimination, warmup, JIT and caching effects), measurement overhead and resolution, statistical significance and noise, choosing and justifying a baseline, environment variation and confounders, wall-clock vs CPU time, the difference between benchmarking a function and a system, and the discipline of predefining what is measured before running. Also use when a benchmark shows an implausible result, when results vary run to run, or when deciding whether a measured difference is real or noise.
---

# Benchmark Design And Pitfalls

A benchmark is a measurement, and like any measurement it can be wrong in ways that look right. The defining failure of benchmarking is producing a number with confidence and acting on it, when the number reflects the measurement apparatus rather than the system under test. A microbenchmark that reports a function taking two nanoseconds is almost certainly measuring nothing — the compiler elided the work, the input was constant-folded, the timer resolution cannot resolve the operation, or the result was never used and dead-code elimination removed it. A benchmark that shows a 3x speedup on one run and a 0.9x on the next is measuring noise, not signal. A comparison against a "baseline" that was itself unoptimized or misconfigured proves nothing about the change you made. These failures do not announce themselves; the benchmark runs, prints a number, and invites a confident wrong conclusion.

Agents tend to trust benchmark output because it is numeric and reproducible-looking. The defects live in the setup the output does not reveal: insufficient warmup so the first runs measure cold caches and JIT compilation; single runs reported as if they were central tendencies; comparisons without a statistical sense of whether the difference exceeds noise; baselines chosen for convenience rather than relevance; environments that drift between runs (thermal throttling, background load, container neighbor noise). The judgment problem is treating a benchmark as an experiment whose validity must be established — warmup, repetition, statistical treatment, controlled environment, relevant baseline — before its result is trusted, rather than as a black box that emits truth.

This skill is about designing benchmarks whose results mean what they appear to mean. It complements the profiling skill (finding where time goes) and the prioritization skill (deciding what to optimize); here the question is how to measure a change or a component so that the measurement is trustworthy.

## Core Rules

### Establish Validity Before Trusting A Number

A benchmark result is trustworthy only if the measurement actually reflects the work intended. Several conditions must hold, and each is a common failure point:

- **The work must not be optimized away.** Compilers aggressively eliminate computations whose results are unused, constant-fold inputs known at compile time, and unroll or vectorize loops in ways that change what is measured. Feed inputs the compiler cannot predict (e.g., from a random source or a black-box function), and consume the output (write it to a volatile sink, hash it, return it) so it cannot be proven dead. A microbenchmark reporting implausibly fast times often measures nothing because the work was elided.
- **The timer must resolve the operation.** If an operation takes 10ns and the timer resolution is 100ns, you are measuring timer noise. Either run the operation many times in a loop and divide, or use a higher-resolution timer. Beware that looping changes what you measure (loop overhead, cache effects from repeated identical input).
- **Warm up before measuring.** The first iterations of any benchmark measure cold state: cold instruction and data caches, un-JIT-compiled code (in runtimes with a JIT), uninitialized branch predictors, lazy-loaded resources. Run enough iterations to reach steady state, discard the warmup, then measure. Reporting the first run's time as "the result" measures startup, not steady-state performance.
- **Measurement must not perturb the result.** The act of measuring adds overhead: timer calls, logging, statistics collection. If the measurement apparatus is a significant fraction of the measured time, the number reflects the apparatus. Use lightweight measurement, and verify that removing the measurement does not change the result materially.

### Use Repetition And Statistics, Not Single Runs

A single benchmark run is one sample from a noisy distribution. Reporting it as "the result" ignores variance that may exceed the effect being measured.

- **Run many iterations and report a distribution, not a point.** Mean alone is misleading because outliers (GC pauses, page faults, scheduler preemption) skew it. Report median and tail percentiles (p95, p99) to characterize both the typical case and the worst cases.
- **Assess whether the difference exceeds the noise.** A 5% improvement is meaningless if run-to-run variance is 10%. Use enough samples and a statistical test (or at minimum, non-overlapping confidence intervals) to determine whether the observed difference is real. If two implementations' distributions overlap substantially, you have not shown a difference.
- **Watch for outliers and explain them.** A run 10x slower than the rest is not "noise to discard"; it is information about a tail behavior (GC, paging, lock contention) that may matter more than the median. Investigate before averaging it away.
- **Report the conditions.** Number of runs, warmup iterations, hardware, load, software versions, and the statistical treatment (median? mean? which percentiles?) must accompany the number, or it cannot be interpreted or reproduced.

### Choose And Justify The Baseline Carefully

A speedup is a ratio, and the denominator is the baseline. A misleading baseline makes any speedup meaningless.

- **The baseline must be the realistic alternative, fairly configured.** Comparing your optimized path against an unoptimized strawman, or against the old path with optimizations disabled, inflates the speedup. Compare against the best realistic non-optimized version, configured as it would actually run.
- **The baseline must be measured under the same conditions.** If the optimized version was measured warm on dedicated hardware and the baseline cold on a loaded machine, the comparison is invalid. Measure both in the same run, interleaved if possible, to control for environment drift.
- **Beware the "compared to nothing" baseline.** A claim of "10x faster" is meaningless without saying faster than what, and whether that what was realistic. Always name the baseline and why it is the right comparison.
- **Re-establish the baseline when the environment changes.** A baseline measured on one machine or one software version does not transfer to another. Re-measure when the platform changes.

### Control The Environment, Or Acknowledge Its Variation

Benchmark results are sensitive to the environment in ways that are invisible unless controlled for.

- **Hardware matters.** CPU model, core count, cache sizes, memory speed, disk type, and NUMA topology all affect results. Report the hardware; do not compare results across different hardware as if they were comparable.
- **Load and isolation matter.** A benchmark run while other processes compete for CPU, memory bandwidth, or disk yields different results than one run in isolation. Containerized environments add neighbor noise. Run in as isolated an environment as feasible, and report what else was running.
- **Thermal and power state matter.** CPUs throttle under sustained load (thermal) and vary frequency by power state; a benchmark's early iterations may run faster than later ones as the chip heats and throttles. Allow the system to reach a stable thermal/power state, or report the variation.
- **Software environment matters.** OS version, kernel configuration, runtime version, compiler flags, and feature flags change results. Pin and report them. A "speedup" that appears after a runtime upgrade may be the runtime, not your change.

### Distinguish Microbenchmark From System Benchmark

A microbenchmark measures a component in isolation; a system benchmark measures the whole system doing real work. They answer different questions and have different pitfalls.

- **A microbenchmark proves the component got faster, not that the system got faster.** Because of Amdahl's law, a component that is 10x faster but is 1% of the system yields no user-visible improvement. Always connect a microbenchmark result to the system-level impact before claiming benefit.
- **A microbenchmark can mislead by removing context.** Isolating a function removes the cache state, contention, and data patterns of real use. A function that benchmarks fast in isolation may be slow in the system because of cache misses on real data layout, or contention with other threads.
- **A system benchmark captures real behavior but is harder to attribute.** When the system gets faster or slower, a system benchmark shows the change but not which component caused it. Use it to confirm user-visible impact; use profiling (not microbenchmarks) to attribute the cause.
- **Match the benchmark to the question.** "Did this function get faster?" is a microbenchmark question. "Did the user-visible latency improve?" is a system benchmark question. Answering the wrong one produces a number that does not address the decision.

### Predefine What You Measure Before You Run

The discipline that prevents most benchmarking failures is deciding, in advance, what metric defines success and what threshold must be met. Without it, benchmarking becomes a search for a favorable result.

- **Name the metric and the success threshold before running.** "Median latency of X must decrease by at least 15% with p99 not regressing." This prevents moving the goalposts to whatever metric happened to improve.
- **Run the comparison blind or interleaved if bias is a risk.** If you know which run is "yours," you may unconsciously favor it. Interleaving or labeling hides the identity until the numbers are in.
- **Report negative results.** If the benchmark shows no improvement or a regression, report it. Selectively reporting only favorable benchmarks distorts understanding of the system.

## Common Traps

### Dead Code Elimination Producing Implausibly Fast Times

A microbenchmark reporting nanosecond-scale times for nontrivial work, because the compiler proved the result unused and removed the work. Consume the result through a black-box sink; feed inputs the compiler cannot constant-fold.

### Measuring Cold State Without Warmup

Reporting the first run's time, which includes JIT compilation, cold caches, and lazy initialization, as the steady-state result. Warm up until times stabilize, discard warmup, then measure.

### Single Run Reported As The Result

One benchmark run treated as definitive, when run-to-run variance exceeds the effect size. Run many iterations, report median and tail percentiles, and check whether the difference exceeds the noise.

### Mean Of Skewed Data Hiding The Tail

Reporting the mean when outliers (GC, paging, contention) dominate it, hiding that the tail is bad. Report median and percentiles; investigate outliers rather than averaging them away.

### Strawman Baseline Inflating The Speedup

Comparing against an unoptimized or misconfigured baseline to produce a large speedup. Compare against the best realistic alternative, fairly configured and measured under identical conditions.

### Comparing Across Different Environments

Benchmarking the optimized version on a fast isolated machine and the baseline on a slow loaded one, then claiming a speedup. Measure both in the same environment, interleaved; report and pin the environment.

### Treating A Microbenchmark Win As A System Win

Celebrating a component speedup without confirming the user-visible metric moved, when the component is a small fraction of the total. Connect microbenchmark results to system-level impact via Amdahl's law.

### Benchmarking Until A Favorable Result Appears

Running many configurations and reporting only the one that shows improvement. Predefine the metric and threshold; report negative results honestly.

## Self-Check

- [ ] The benchmarked work cannot be optimized away by the compiler: inputs are not constant-foldable (sourced from a black box or random), and outputs are consumed through a volatile/black-box sink so they cannot be proven dead.
- [ ] Timer resolution is adequate for the operation (the operation is looped or a high-resolution timer is used), and warmup iterations are run and discarded so the measurement reflects steady state, not cold caches/JIT/initialization.
- [ ] Results are based on many iterations reporting a distribution (median and tail percentiles such as p95/p99), not a single run or a bare mean; outliers are investigated rather than averaged away.
- [ ] The difference between compared implementations was assessed against run-to-run noise (enough samples, non-overlapping confidence intervals or a statistical test) before being declared real.
- [ ] The baseline is the realistic alternative, fairly configured and measured under identical conditions (ideally interleaved in the same run), and is named and justified rather than being a strawman or "compared to nothing."
- [ ] The environment is controlled and reported: hardware, isolation/load, thermal/power state, and software versions are pinned and documented; results are not compared across differing environments as if equivalent.
- [ ] The benchmark type matches the question: a microbenchmark is used for "did the component get faster" and connected to system impact via Amdahl's law, while a system benchmark confirms user-visible effect — the two are not conflated.
- [ ] The metric and success threshold were predefined before running, the comparison was run without favoring the preferred result (interleaved or blind where bias is a risk), and negative results are reported rather than only favorable ones.
