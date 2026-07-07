---
name: performance_regression_debugging.md
description: Use when the agent is diagnosing a performance regression — the system got slower, latency increased, throughput dropped, or resource usage rose — without an obvious cause; using profiling, flame graphs, benchmarking, git bisect, or before/after comparison to locate the change that caused the regression; distinguishing a code change from a data change, a dependency upgrade, a config change, or an environmental factor; or investigating slow degradation over time. Covers regression isolation (when did it start, what changed), profiling and bottleneck identification, the bisection method for finding the introducing change, distinguishing code vs data vs environment, and the discipline of comparing against a known-good baseline rather than optimizing without measurement.
---

# Performance Regression Debugging

A performance regression is a deterioration in the system's speed, latency, throughput, or resource usage, and debugging it differs from general performance optimization in a crucial way: a regression has a cause that was introduced at a specific point in time, and finding that cause is a search for the change that introduced it, not an open-ended optimization. This temporal anchor — the system was fast, now it is slow, something changed — is the most powerful clue, and it enables techniques that general optimization cannot use: bisection (finding the exact commit that introduced the regression by testing progressively narrower ranges), before/after comparison (profiling both versions and diffing), and correlation with changes (code, dependency, config, data, environment). The failure mode is treating a regression as an optimization problem ("let's make it faster") rather than a search problem ("what changed"), which leads to speculative optimization that may mask the regression without addressing its cause, or to optimizing the wrong thing because the actual cause was never identified. The disciplined approach treats the regression as a mystery with a specific introduction point, and uses the temporal anchor to isolate the cause before attempting a fix.

Agents tend to jump to speculative optimization (tuning, caching, rewriting) without isolating the cause, to conflate a regression with a pre-existing bottleneck, and to overlook non-code causes (data growth, dependency updates, config changes, environmental factors). The judgment problem is recognizing that a regression is a search for an introduced change, that the baseline (the known-good performance) is essential for comparison, and that the cause may not be code (data, dependencies, environment are all candidates). This skill covers the discipline of performance regression debugging: establishing the baseline, isolating the introduction point, profiling and diffing, distinguishing cause categories, and verifying the fix.

## Core Rules

### Establish The Baseline And Characterize The Regression

Before searching for a cause, establish what the performance was before the regression (the baseline) and characterize the regression precisely.

- **Establish the baseline: what was the performance before the regression?** Use historical metrics (latency, throughput, resource usage over time) to identify the pre-regression performance level. Without a baseline, there is no way to measure the regression or to verify a fix.
- **Characterize the regression precisely: which metric, how much, when did it start, under what conditions.** "It's slower" is insufficient. "P99 latency went from 50ms to 200ms starting around Tuesday, for the checkout endpoint under normal load" is a characterized regression that can be investigated. Identify the specific metric, the magnitude, the onset, and the conditions.
- **Distinguish a regression from a pre-existing limitation.** A regression is a deterioration from a previous state; a pre-existing bottleneck was always there. A regression investigation searches for the change; a pre-existing bottleneck investigation optimizes. Confirm the performance was previously better before treating it as a regression.
- **Distinguish a sudden regression from gradual degradation.** A sudden regression (latency jumps on a specific day) suggests a specific change (a deploy, a config update); a gradual degradation (latency slowly increasing over months) suggests a scaling factor (data growth, memory fragmentation, accumulating state). The pattern guides the search.

### Isolate The Introduction Point

The regression was introduced at a specific point. Narrowing that point is the most efficient path to the cause.

- **Correlate the regression onset with changes: deploys, config updates, dependency upgrades, data changes, infrastructure changes.** Overlay the regression onset with the change log; a change at the same time is the primary suspect. This correlation often identifies the cause immediately.
- **Use bisection (git bisect) to find the introducing commit.** If the regression is in code, binary search the commit history: test a known-good old version and a known-bad new version, then test the midpoint, narrowing until the introducing commit is found. This is far more efficient than reading the diff of a large range.
- **Compare the regressed version against the baseline version directly.** Run both versions under the same conditions and profile both; the difference in profiles reveals what changed (a new hot function, a changed query, a different allocation pattern). This before/after diff is the most direct evidence.
- **Consider non-code changes: data volume or distribution, dependency versions, config flags, infrastructure (instance type, network, disk), load patterns.** A regression is not always a code change; data growth crossing a threshold (a table now too large for an in-memory sort), a dependency update with a performance bug, a config change disabling an optimization, or an infrastructure change (slower disk, smaller instance) can all cause regressions. Investigate these alongside code.

### Profile To Identify The Bottleneck Introduced By The Regression

Once the introduction point is narrowed, profiling identifies the specific bottleneck the regression introduced.

- **Profile the regressed version and compare to the baseline.** A CPU profile (flame graph), an allocation profile, an I/O profile, or a database query log of the regressed version, compared to the baseline, reveals where the new time or resource is spent. The difference is the regression's bottleneck.
- **Look for the new hot spot: a function, query, or operation that is now significantly more expensive.** The regression introduced (or worsened) a specific cost; the profile shows it. A function that was 1% of CPU and is now 30%, a query that was 1ms and is now 50ms, an allocation pattern that now creates 10x more garbage — these are the regression's signature.
- **Distinguish the regression's bottleneck from pre-existing bottlenecks.** The profile shows all costs; the regression's cost is the one that changed. A pre-existing bottleneck (always 40% of CPU) is not the regression, even if it is the largest cost; focus on what changed.
- **Use sampling profilers for production, instrumenting profilers for reproduction.** A sampling profiler (low overhead, statistical) suits production; an instrumenting profiler (precise, high overhead) suits a reproduction in a test environment. See profiling-and-bottleneck-analysis.

### Investigate The Specific Cause Categories

Once the bottleneck is identified, investigate the specific cause within its category.

- **Algorithmic regression: a change that increased time or space complexity.** A loop that was O(n) becoming O(n^2) (a nested loop added), a sort that was bounded becoming unbounded, a cache that was hit becoming missed. These are the most impactful regressions (scaling poorly) and the most important to find.
- **Query regression: a database query that became slower.** A query plan change (the optimizer chose a worse plan due to data distribution change or missing statistics), an N+1 query introduced (a loop that now queries per iteration), a missing index after a schema change, a query that now scans more data. Check query plans and execution stats.
- **Resource regression: increased memory, CPU, I/O, or network usage.** A memory leak (accumulating state), increased allocations (more garbage collection), more I/O (a change that now reads more files or makes more network calls), increased network chatter. Resource regressions often manifest as latency regressions under load (GC pauses, I/O contention).
- **Dependency regression: a library or external service that became slower.** A dependency upgrade with a performance bug, an external API that slowed, a cache layer that became less effective. Isolate the dependency (benchmark it directly) to confirm.
- **Concurrency regression: a change that reduced parallelism or introduced contention.** A lock that is now contended (serializing previously parallel work), a thread pool that is now undersized, an async operation that is now blocking. Concurrency regressions manifest as throughput drops under load.

### Verify The Fix Resolves The Regression

After identifying and fixing the cause, verify the fix restores the baseline performance.

- **Measure the performance after the fix, against the baseline.** The fix should restore the pre-regression performance (latency back to 50ms, throughput back to the baseline). If it does not, the cause was not fully addressed, or there is an additional contributing factor.
- **Test under the conditions that exhibited the regression.** A fix that works under light load but not under the load that exhibited the regression has not resolved it. Test under realistic conditions (load, data volume, concurrency).
- **Guard against re-introduction with a performance test.** Add a benchmark or performance test that would catch the regression if reintroduced. A regression that is found, fixed, and not guarded against will recur. See performance-testing-and-load-testing.
- **Document the regression, its cause, and the fix.** A record of the regression (what happened, why, how it was found, how it was fixed) builds organizational knowledge and speeds future investigations. The pattern (e.g., "N+1 queries tend to be introduced when...") helps prevent similar regressions.

## Common Traps

### Speculative Optimization Without Isolating The Cause

Optimizing (tuning, caching, rewriting) without identifying what changed, masking the regression's cause rather than addressing it. Isolate the cause first; fix the cause, not the symptom.

### Conflating Regression With Pre-Existing Bottleneck

Treating a pre-existing bottleneck (always slow) as the regression, optimizing the wrong thing. Confirm the performance was previously better; focus on what changed.

### Overlooking Non-Code Causes

Searching only code changes when the cause is data growth, a dependency update, a config change, or an environmental factor. Investigate all change categories.

### No Baseline For Comparison

Investigating without a baseline (what the performance was before), so the regression cannot be measured or a fix verified. Establish the baseline first.

### Bisection Not Used When It Would Be Efficient

Reading a large diff manually when bisection would find the introducing commit faster. Use git bisect for code regressions.

### Profile Not Compared To Baseline

Profiling only the regressed version, without comparing to the baseline, so the changed cost is not distinguished from pre-existing costs. Diff the profiles.

### Fix Not Verified Under Realistic Conditions

A fix verified under light load but not under the conditions that exhibited the regression. Test under the regression's conditions.

### No Performance Test To Guard Against Re-Introduction

A regression fixed but not guarded with a performance test, so it recurs when the cause is reintroduced. Add a benchmark that catches the regression.

## Self-Check

- [ ] The baseline (pre-regression performance) is established from historical metrics, the regression is characterized precisely (which metric, how much, when, under what conditions), and it is confirmed to be a regression (previously better) rather than a pre-existing bottleneck, with the pattern (sudden vs gradual) noted.
- [ ] The introduction point is isolated by correlating the onset with changes (deploys, config, dependencies, data, infrastructure), using bisection (git bisect) to find the introducing commit for code regressions, comparing the regressed and baseline versions directly, and investigating non-code causes (data growth, dependency updates, config, environment) alongside code.
- [ ] Profiling (CPU flame graph, allocation, I/O, query log) of the regressed version is compared to the baseline, the new or worsened hot spot (the changed cost) is identified and distinguished from pre-existing bottlenecks, and the specific bottleneck introduced by the regression is located.
- [ ] The specific cause category is investigated — algorithmic (complexity increase), query (plan change, N+1, missing index), resource (leak, allocations, I/O), dependency (slower library or service), concurrency (contention, reduced parallelism) — and the specific change within that category is identified.
- [ ] The fix addresses the cause (not the symptom), is verified to restore the baseline performance under the conditions that exhibited the regression, and is guarded against re-introduction with a performance test or benchmark.
- [ ] The regression, its cause, and the fix are documented, building organizational knowledge and helping prevent similar regressions.
- [ ] Non-code causes were seriously investigated (data volume and distribution checked, dependency versions diffed, config changes reviewed, infrastructure verified) rather than assuming the cause is in the application code.
- [ ] The investigation distinguished correlation from causation — a change at the same time as the regression is a suspect, not a confirmed cause, and is verified (reproducing the regression by applying the change, or fixing it and observing the regression resolve) before being declared the root cause.
