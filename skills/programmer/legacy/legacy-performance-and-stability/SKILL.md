---
name: legacy_performance_and_stability.md
description: Use when the agent is diagnosing performance or stability problems in legacy code it does not fully understand, profiling production legacy systems without risky changes, stabilizing a flaky or crash-prone legacy system before attempting optimization, reasoning about capacity headroom and saturation, deciding whether to optimize, rewrite, or leave alone a slow legacy path, or judging which of many possible changes will actually move the metric. Also covers the failure mode of optimizing legacy code and introducing regressions, optimizing the wrong thing based on intuition rather than measurement, chasing micro-optimizations while the real bottleneck is ignored, destabilizing a fragile system by tuning it, and mistaking a stability problem for a performance problem. Distinct from general performance engineering; this skill covers the specific judgment of improving performance in code whose behavior is not fully understood and where changes carry regression risk.
---

# Legacy Performance And Stability

Improving the performance of legacy code is a different problem from optimizing code you wrote last week. In code you understand, you know where the hot paths are, what the invariants are, and what a change can safely touch. In legacy code you often know none of this: the slow path may be slow for a reason that once mattered, the "obvious" optimization may break an implicit invariant a caller depends on, and the system may be fragile in ways that only surface under load. Worse, a legacy system that is both slow and flaky usually needs to be *stabilized before it is optimized* — tuning a system that crashes intermittently just makes the crashes faster. The judgment problem is to measure before changing, to stabilize before optimizing, to find the few changes that actually move the metric rather than the many that feel productive, and to treat every change to fragile code as a regression risk that must be guarded.

Agents tend to optimize by intuition — the loop that "looks" slow, the data structure that "should" be faster, the query that "obviously" needs an index — and in legacy code intuition is unreliable because the code's actual behavior diverges from its appearance. The harm appears as optimizations that change behavior (a cache that returns stale data a caller did not expect, a batch that reorders side effects), as effort spent on code that was not the bottleneck while the real bottleneck is ignored, and as a fragile system made more fragile by tuning that introduced a new crash or deadlock under load. The discipline is to profile the real system under real conditions before changing anything, to fix stability first so that performance work is even measurable, to target the few high-impact changes the profile reveals, to guard every change with characterization tests and incremental rollout, and to recognize when the right answer is to leave the code alone or replace it rather than to optimize it.

## Core Rules

### Stabilize Before You Optimize

A flaky, crash-prone, or deadlock-prone legacy system cannot be meaningfully optimized, because its performance is noise: a crash masks the real bottleneck, a flaky timeout makes before/after comparison unreliable, and tuning a system that fails intermittently just produces faster failures. Stability is the precondition for trustworthy performance measurement.

- **Fix the crashes, deadlocks, and flakiness first.** A system that falls over under load cannot have its latency optimized until it stops falling over.
- **Make performance measurable before measuring it.** If the metric is noisy because the system is unstable, the instability must be resolved before any before/after comparison can be trusted.
- **Distinguish a stability problem from a performance problem.** A system that is slow because it is retrying after crashes has a stability problem, not a speed problem; optimizing the retry loop treats the symptom.

### Profile The Real System Under Real Conditions Before Changing Anything

Intuition about legacy performance is unreliable: the slow path is rarely where it appears, the hot loop is often not the bottleneck once you include I/O and lock contention, and the code's actual runtime behavior diverges from a static read. Measure the real system under realistic load before deciding what to change.

- **Profile in production or a production-like environment, not on a laptop with toy data.** A profile of synthetic input often misses the real bottleneck (cache misses at scale, lock contention under concurrency, GC pressure under real allocation patterns).
- **Measure end-to-end latency and resource use, not just CPU.** The bottleneck is often I/O, lock contention, memory allocation, network, or database — not CPU. A CPU profile alone hides these.
- **Attribute cost to the actual code paths, with sampling flame graphs or equivalent.** Know which functions dominate wall time before deciding what to change; do not optimize by reading code and guessing.
- **Capture a baseline before any change.** Without a measured baseline you cannot tell whether a change helped, hurt, or did nothing — and "it feels faster" is not a measurement.

### Target The Few High-Impact Changes, Not The Many Convenient Ones

Performance improvement in legacy code follows a steep curve: a small number of changes (the actual bottleneck, an N+1 query, a missing index, a redundant serialization, a lock held too long) produce most of the gain, and the rest is micro-optimization that barely moves the metric while carrying regression risk. Find the few, ignore the many.

- **Optimize the bottleneck the profile identifies, not the code that looks inefficient.** A clean-looking hot loop that is actually fast is not worth touching; an ugly-looking cold path is not worth touching either.
- **Prefer algorithmic and structural wins over micro-optimizations.** Replacing an O(n²) scan with a hash lookup, eliminating an N+1 query, or adding a needed index moves the metric; rewriting a loop to avoid one allocation usually does not.
- **Stop when the remaining gains are not worth the risk.** Once the bottleneck is addressed and the metric meets the target, further optimization in fragile legacy code has diminishing return and rising regression risk; stop.

### Guard Every Change Against Regression In Code You Do Not Fully Understand

Every performance change to legacy code is also a behavior change, and in code whose behavior you do not fully understand, the risk is that the optimization silently alters behavior — a cache that changes what callers receive, a batch that reorders side effects, a lock change that introduces a deadlock under a load path you did not exercise. Guard each change.

- **Characterize the current behavior before optimizing.** Write characterization tests for the path you will change (including its quirks), so a behavior shift is detected — the testing-strategy skills cover how.
- **Make changes small, isolated, and reversible.** One optimization per change, behind a flag where possible, rolled out incrementally so a problem affects a slice of traffic and is revertible in minutes.
- **Measure before and after on the real workload.** Confirm the change actually improves the metric and does not regress correctness or other metrics (memory, error rate, tail latency).
- **Watch for second-order effects under load.** A change that is correct at low concurrency may deadlock, thrash, or exhaust a pool under production load; load-test the change, not just unit-test it.

### Decide Between Optimize, Replace, Or Leave Alone — Deliberately

Not every slow legacy path should be optimized. Sometimes the right answer is to replace the component (incremental modernization), sometimes to leave it alone (the cost and risk exceed the benefit), and sometimes to optimize in place. Make this decision explicitly rather than defaulting to optimization.

- **Optimize in place when the bottleneck is localized, the change is low-risk, and the gain is worth it.** A single query or hot loop is often worth optimizing; a tangled subsystem usually is not.
- **Replace (via strangler/incremental modernization) when the code is both slow and unmaintainable, and optimization would just polish a component that needs to go.** Do not invest deeply in optimizing code you intend to retire.
- **Leave it alone when the performance is acceptable for the business need, the risk of change exceeds the benefit, or the code is too fragile to touch safely.** "Good enough and stable" beats "faster but broken."

## Common Traps

### Optimizing By Intuition Instead Of Measurement

Reading the code, deciding a loop or data structure "must" be the bottleneck, and optimizing it — only to find the real cost was elsewhere (I/O, a lock, a query) and the change moved nothing. Profile the real system first; never optimize by reading alone.

### Optimizing Before Stabilizing

Tuning a system that crashes or deadlocks under load, so the performance work is unmeasurable and the instability is made worse. Stabilize first; performance work on an unstable system is wasted or harmful.

### The Optimization That Changes Behavior

A cache, batch, reorder, or lock change that improves speed but silently alters the behavior callers depend on (stale data, reordered side effects, a new deadlock under load). Characterize behavior before changing; guard each change; load-test, not just unit-test.

### Chasing Micro-Optimizations While The Bottleneck Is Ignored

Spending effort on allocations, loop tweaks, and local cleanups while the actual bottleneck (an N+1 query, a missing index, a contended lock, a synchronous I/O in a hot path) goes unaddressed. Target the few high-impact changes the profile reveals; stop when gains diminish.

### Mistaking A Stability Problem For A Performance Problem

Treating a system that is slow because it is retrying after crashes, or timing out under contention, as a speed problem and optimizing the symptom. Diagnose whether the root cause is instability; fix that first.

### Optimizing Code That Should Be Replaced

Investing deeply in tuning a component that is both slow and unmaintainable, when the right answer is incremental replacement. Decide optimize-vs-replace-vs-leave-alone deliberately; do not default to optimization.

### No Baseline, So No Way To Know If It Worked

Making performance changes without a measured baseline, so "it feels faster" or a single before/after run is the only evidence. Capture a baseline under realistic load and measure before and after; anecdotal improvement is not verification.

## Self-Check

- [ ] Stability was addressed before optimization: crashes, deadlocks, and flakiness were resolved (or explicitly scoped out) so that performance is measurable, and stability problems were not mistaken for performance problems.
- [ ] The real system was profiled under realistic (production or production-like) conditions before any change, measuring end-to-end latency and resources (I/O, locks, memory, network, DB — not CPU alone), with cost attributed to actual code paths via sampling flame graphs or equivalent.
- [ ] A measured baseline was captured before any change, and before/after comparison is done on the real workload — not on toy data, not by intuition, and not by "it feels faster."
- [ ] The few high-impact changes identified by the profile were targeted (the actual bottleneck, algorithmic/structural wins like N+1 elimination or index addition), and micro-optimizations were avoided or stopped once gains diminished.
- [ ] Every performance change was guarded: current behavior characterized before changing, changes made small/isolated/reversible behind a flag where possible, rolled out incrementally, and load-tested for second-order effects (deadlocks, pool exhaustion, tail latency) — not only unit-tested.
- [ ] The optimize-vs-replace-vs-leave-alone decision was made deliberately: optimize in place for localized low-risk gains, replace (incremental modernization) for code that is slow and unmaintainable, leave alone where performance is acceptable or the code is too fragile to touch.
- [ ] The highest-risk cases were verified — an optimization that changed behavior was caught by characterization, a change that deadlocked under load was caught by load testing, an intuition-based optimization that moved nothing was rejected after profiling, and a stability problem was correctly diagnosed before performance work — not only the clean localized-bottleneck case.
- [ ] The scope stayed on performance/stability of existing code and did not collapse into whole-system replacement planning (incremental-modernization) or into adding the first tests to untested code (legacy-testing-strategy), though those skills apply where they apply.
