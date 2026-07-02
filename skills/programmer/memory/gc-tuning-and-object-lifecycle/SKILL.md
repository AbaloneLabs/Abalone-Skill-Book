---
name: gc_tuning_and_object_lifecycle.md
description: Use when the agent is tuning a garbage collector, reasoning about object lifetimes and allocation rate in a managed runtime, diagnosing GC pauses or high GC CPU, sizing the heap, analyzing GC logs, reducing allocation on a hot path, or evaluating escape analysis and object allocation eligibility. Covers generational collection and the weak generational hypothesis, the effect of allocation rate and object tenure on GC frequency and pause time, heap sizing and its tradeoffs, pause-time versus throughput collectors, object escape analysis and scalar replacement, reading GC logs to diagnose the cause, the interaction between allocation patterns and GC behavior, and the discipline of reducing allocation before tuning the collector. Also use when GC pauses cause latency spikes, when GC CPU is high, when the heap grows without bound, or when a latency-sensitive path must avoid GC.
---

# GC Tuning And Object Lifecycle

Garbage collection trades manual memory management for automatic reclamation, and its performance is governed less by the collector's settings than by the allocation behavior of the program. The weak generational hypothesis — that most objects die young — is the foundation of generational collectors, and it means a program that allocates many short-lived objects and lets them die young is cheap to collect, while one that promotes short-lived objects into the old generation (where collection is rare and expensive) is expensive. The recurring failure is tuning the collector (heap sizes, pause targets, generation ratios) to compensate for an allocation pattern that fights the collector's assumptions, when the higher-leverage change is to the allocation pattern itself. Teams chase GC flags when the real problem is a hot loop boxing integers, a stream API creating millions of intermediate objects, or a cache tenuring ephemeral data into old generation — all of which no collector flag fixes.

Agents tend to treat GC as a black box whose knobs are the lever, because the knobs are visible and the allocation pattern is not. The defects live in the lifecycle: short-lived objects that escape into long-lived structures (a list that accumulates per-request objects, a closure capturing a large object), inflating old generation and triggering expensive full collections; an allocation rate so high that the young generation fills and collects constantly, burning CPU; a heap sized too small (constant collection) or too large (long pauses when collection finally runs); a "throughput" collector on a latency-sensitive service causing multi-second stops-the-world pauses. The judgment problem is treating GC behavior as a downstream consequence of the program's object lifecycle and allocation rate — to be addressed first at the source (reduce allocation, fix tenure) and only then at the collector (tune for the residual) — and reading GC logs to diagnose which it is.

This skill is about tuning GC and managing object lifecycles in managed runtimes. It complements the memory-leaks skill (unintentional retention) and the allocation-patterns skill (allocation strategy in general); here the question is how allocation behavior and collector tuning interact in a GC'd runtime, and how to address GC problems at the right level.

## Core Rules

### Address Allocation At The Source Before Tuning The Collector

The highest-leverage GC optimization is almost always reducing allocation or fixing object tenure, not adjusting collector flags. A collector tuned for a pathological allocation pattern is a patch; fixing the pattern is the cure.

- **Reduce allocation on hot paths.** A loop that allocates per iteration (boxing, intermediate collections, string concatenation, lambdas capturing state) generates garbage proportional to throughput, and the collector burns CPU reclaiming it. Identify hot-path allocation with a profiler and eliminate it (reuse objects, use primitive paths, avoid intermediate allocations).
- **Fix premature promotion (tenure).** Short-lived objects promoted to old generation (because they survived enough minor collections, often by being referenced from a long-lived structure) turn cheap young-gen collection into expensive old-gen collection. Find structures that retain ephemeral objects longer than intended.
- **Re-measure GC after allocation changes.** Reducing allocation often resolves the GC problem without any collector tuning; confirm before reaching for flags.

### Understand Generational Collection And The Generational Hypothesis

Most production collectors are generational, and understanding why is the foundation of tuning. The weak generational hypothesis observes that most allocated objects die young, so the collector separates objects by age: a small young generation collected frequently and cheaply, and a larger old generation collected rarely and expensively.

- **Young-gen collection is cheap and frequent.** Most objects die in young gen and are reclaimed quickly; this is the collector working as intended. High young-gen collection frequency is often fine if the objects truly die there.
- **Old-gen (full) collection is expensive.** Objects that survive into old gen are collected rarely, but when they are, the pause is long and the CPU cost high. The goal of tuning is usually to keep ephemeral objects out of old gen and to make old-gen collections infrequent.
- **Promotion is the enemy of cheap GC.** Every object that reaches the tenure threshold and promotes to old gen adds to the expensive-to-collect set. Understanding why objects promote (they survived minor collections, usually because something referenced them) is the key to reducing full collections.

### Manage Allocation Rate And Object Tenure

Allocation rate and tenure are the two program-side variables that dominate GC behavior, and both are controllable.

- **Allocation rate drives young-gen collection frequency.** The faster the program allocates, the faster young gen fills and collects. High allocation rate is not inherently wrong (if objects die in young gen), but it burns CPU on collection proportional to allocation. Reducing allocation rate reduces GC CPU directly.
- **Tenure drives old-gen collection frequency and cost.** Objects that promote inflate old gen; when old gen fills, an expensive full collection runs. Reducing promotion (by not retaining ephemeral objects in long-lived structures) keeps old gen small and full collections rare.
- **Watch for "mid-life" objects.** Objects that live long enough to promote but die soon after are the worst case: they survive into old gen (expensive) and then die there (requiring a full collection to reclaim). These often come from caches, connection/session objects, or request-scoped objects retained too long.
- **Pre-size collections to avoid growth-driven promotion.** A collection that grows incrementally may keep its contents alive across minor collections, promoting them; pre-sizing or using structures with the right lifetime can avoid this.

### Size The Heap Deliberately, Understanding The Tradeoff

Heap size is the most impactful single GC setting, and it involves a fundamental tradeoff: a larger heap collects less frequently but pauses longer when it does; a smaller heap pauses shorter but collects more often.

- **A too-small heap collects constantly.** If the heap is near the live set, the collector runs continuously, burning CPU and causing frequent pauses. Increase the heap until collection frequency is reasonable.
- **A too-large heap pauses too long.** A very large heap collects rarely but, when old gen fills, the full collection scans a large heap and pauses for seconds. For latency-sensitive services, an oversized heap is a latency hazard.
- **Match the heap to the pause target and live set.** The right heap is large enough that the live set is a comfortable fraction of it (leaving room for allocation between collections) but small enough that pauses meet the target. This is found by measurement, not by formula.
- **Consider the collector's pause behavior.** Throughput collectors stop the world for the full collection; concurrent/partial collectors (G1, ZGC, Shenandoah) reduce pause time at the cost of throughput and overhead. Choose the collector class for the workload's pause sensitivity before fine-tuning size.

### Choose The Collector For The Workload's Pause Sensitivity

Different collectors optimize for different goals, and the choice depends on whether the workload is throughput-bound or latency-bound.

- **Throughput collectors** maximize application CPU at the cost of long stop-the-world pauses. Suitable for batch jobs where pauses do not matter; dangerous for latency-sensitive services.
- **Concurrent and region-based collectors (G1, ZGC, Shenandoah)** reduce pause time (often to milliseconds) by collecting concurrently with the application or in small regions, at the cost of some throughput and overhead. Suitable for interactive services where pause time matters.
- **Match the collector to the requirement.** A latency-sensitive service on a throughput collector will have multi-second pauses no amount of tuning fixes; switching collectors is the answer. A batch job on a low-pause collector pays overhead for pauses it does not care about.

### Read GC Logs To Diagnose, Not To Guess

GC behavior is observable in logs and metrics, and diagnosis should be data-driven. Guessing at flags without reading the logs is a waste.

- **Identify the symptom.** Is the problem high GC CPU (allocation rate too high, or collecting too often), long pauses (old-gen collections, or a collector with long pauses), or frequent full collections (old gen filling, promotion too high)? Each points to a different cause.
- **Read the GC log to find the cause.** The log shows collection frequency, duration, generation sizes before and after, and promotion. A pattern of frequent young-gen collections with low promotion is an allocation-rate problem; frequent full collections with growing old gen is a promotion/leak problem; long pauses on full collections is a collector-or-heap-size problem.
- **Distinguish a leak from a high live set.** If old gen grows without bound and full collections reclaim less and less, it may be a memory leak (objects never released — see the memory-leaks skill). If old gen is stable but large, it is a high live set; the fix is reducing retention or sizing the heap, not fixing a leak.

### Use Escape Analysis To Reduce Allocation Where The Runtime Supports It

Some runtimes perform escape analysis: an object that does not "escape" the method (no reference leaves) may be allocated on the stack or have its fields scalar-replaced, avoiding heap allocation entirely. This is automatic, but it depends on the code not defeating the analysis.

- **Do not defeat escape analysis unnecessarily.** Returning an object, storing it in a field, passing it to a non-inlined call, or capturing it in a closure causes it to escape and forces heap allocation. For hot paths, structure code so short-lived objects do not escape.
- **Escape analysis is not a substitute for not allocating.** It helps where it applies, but it is best-effort and runtime-dependent; do not rely on it for hot-path allocation that you could eliminate directly.
- **Verify with a profiler.** Whether escape analysis eliminated an allocation is observable (allocation profiling shows no allocation); verify rather than assume.

## Common Traps

### Tuning Collector Flags Before Fixing Allocation

Adjusting heap and generation ratios to compensate for a hot loop that allocates millions of objects, when reducing the allocation would resolve the GC problem directly. Address allocation at the source first; tune the collector for the residual.

### Premature Promotion Of Short-Lived Objects

Ephemeral objects retained in long-lived structures (a cache, a list, a session) surviving into old generation, turning cheap young-gen collection into expensive full collections. Find and fix structures that retain ephemeral objects longer than intended.

### Heap Too Small (Constant Collection) Or Too Large (Long Pauses)

A heap near the live set collecting constantly and burning CPU, or an oversized heap whose full collections pause for seconds. Size the heap by measurement to balance frequency and pause against the live set and pause target.

### Throughput Collector On A Latency-Sensitive Service

Using a stop-the-world throughput collector for an interactive service, causing multi-second pauses no flag tuning resolves. Switch to a low-pause collector (G1, ZGC, Shenandoah) for latency-sensitive workloads.

### Guessing At Flags Without Reading GC Logs

Adjusting collector settings by trial and error without reading the logs to identify whether the problem is allocation rate, promotion, or collector/heap choice. Diagnose from logs; tune to the diagnosed cause.

### "Mid-Life" Objects Promoting Then Dying

Objects that live just long enough to promote to old gen and then die there, the worst case for GC cost. Identify mid-life objects (caches, sessions, request-scoped objects retained too long) and shorten their lifetime or keep them in young gen.

### Defeating Escape Analysis Unnecessarily

Code that causes short-lived objects to escape (returning them, capturing in closures, passing to non-inlined calls) forcing heap allocation the runtime could have eliminated. Structure hot-path code so short-lived objects do not escape, and verify with a profiler.

### Confusing A High Live Set With A Leak

Treating a large-but-stable old generation as a memory leak, when it is a legitimately high live set whose fix is reducing retention or sizing the heap, not leak hunting. Distinguish by whether old gen grows without bound (leak) or is stable (live set).

## Self-Check

- [ ] Allocation on hot paths was profiled and reduced (boxing, intermediate collections, string concatenation, capturing lambdas) before adjusting collector flags, since the highest-leverage GC optimization is usually at the allocation source.
- [ ] The generational model is understood: young-gen collection is cheap and frequent, old-gen (full) collection is expensive, and the tuning goal is to keep ephemeral objects dying in young gen rather than promoting to old gen.
- [ ] Allocation rate and object tenure are managed: high allocation rate is reduced where it burns GC CPU, promotion is reduced by not retaining ephemeral objects in long-lived structures, and "mid-life" objects (promote then die) are identified and eliminated.
- [ ] The heap is sized by measurement to balance collection frequency and pause time against the live set and pause target — neither so small that it collects constantly nor so large that full-collection pauses are excessive.
- [ ] The collector class matches the workload's pause sensitivity: a low-pause collector (G1, ZGC, Shenandoah) for latency-sensitive services, a throughput collector only for batch work where pauses do not matter.
- [ ] GC logs are read to diagnose the cause (allocation rate, promotion, collector/heap size, or a genuine leak) before tuning, and a high live set is distinguished from a memory leak by whether old gen grows without bound.
- [ ] Escape analysis is leveraged where the runtime supports it: hot-path code is structured so short-lived objects do not escape (not returned, not captured, not passed to non-inlined calls), and elimination is verified with a profiler rather than assumed.
- [ ] After any change (allocation reduction, heap resize, collector switch), GC behavior is re-measured to confirm the problem is resolved, since GC interactions are non-linear and changes can have unexpected effects.
