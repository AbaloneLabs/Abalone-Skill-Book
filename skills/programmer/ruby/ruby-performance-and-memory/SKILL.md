---
name: ruby_performance_and_memory.md
description: Use when the agent is optimizing Ruby code for speed or memory, profiling hot paths, reducing allocations and GC pressure, choosing between mutable and frozen strings, evaluating dynamic dispatch cost, considering JIT tradeoffs, tuning garbage collector settings, or diagnosing memory bloat and slow requests in Ruby and Rails applications.
---

# Ruby Performance and Memory

Ruby optimizes for developer happiness, not raw throughput, and that tradeoff becomes load-bearing the moment a request path gets hot or a process's memory grows without bound. The judgment problem is that Ruby's performance characteristics are dominated by object allocation, string mutation, and garbage collection, and most "optimizations" developers reach for (caching, inlining, metaprogramming) either do not address the actual bottleneck or make memory worse. A developer who adds a cache to avoid a query but stores full ActiveRecord objects has traded a fast, bounded cost (one query) for a slow, unbounded one (memory growth and GC pauses).

The recurring failure mode is optimizing the wrong thing: micro-tuning a method that runs once per request while ignoring the N+1 that runs a hundred times, or "reducing allocations" by mutating strings in a way that defeats frozen-string optimization and intern tables. Real Ruby performance work starts with a profile of the actual hot path under realistic load, identifies whether the cost is CPU (dispatch, allocation churn) or memory (retained objects, GC pressure), and changes the smallest thing that moves the metric.

## Core Rules

### Profile before optimizing, and profile the right thing

Ruby performance intuition is notoriously wrong. Use a profiler (`stackprof`, `memory_profiler`, `rbspy`) on the actual hot path under realistic input, not on a synthetic benchmark. Distinguish:

- **Wall-clock hot path**: where time is spent (CPU-bound work, dispatch, algorithmic cost).
- **Allocation hot path**: where objects are created (drives Gen 1 GC churn and promotion).
- **Retention hot path**: where objects are kept alive (drives memory growth and Gen 2/old generation pressure).

Optimizing allocations in a method that is called once per request does nothing; optimizing a method called in a tight loop does. Measure the call count alongside the per-call cost.

### Allocation is the dominant Ruby performance lever

Every `Object.new`, every string operation that produces a new string, every array/hash literal in a hot loop allocates. Young-generation GC is fast but not free, and objects that survive get promoted and become expensive. Reduction strategies, in order of impact:

- Hoist constants and immutable values out of loops (`REGEXP = /.../` at the top, not inline).
- Use `each_with_object` or mutation over `map` chains that allocate intermediate arrays when you only need a side effect.
- Prefer `<<` on a pre-allocated array over `+` which allocates a new array.
- Freeze strings (`# frozen_string_literal: true`) to avoid per-call string allocation and enable deduplication.

Measure allocations before and after; the goal is fewer surviving objects, not fewer lines of code.

### Strings are the silent allocation tax

Every string literal in a method body allocates a new string on each call unless frozen. In hot paths (parsing, formatting, logging), this dominates. Rules:

- Add `# frozen_string_literal: true` to files with hot string code; literals become frozen and deduplicated.
- Use `String#dup` only when you genuinely need a mutable copy; otherwise operate on frozen strings.
- Beware `gsub`/`split` returning many strings; for high-volume parsing consider `StringScanner` or pre-compiled regexps.
- For repeated string building, `<<` on a single mutable `String` beats `+` chains.

### Dynamic dispatch and metaprogramming have real cost

`send`, `method_missing`, `define_method`, and `respond_to_missing?` bypass the inline cache and slow dispatch. In hot loops this is measurable. Rules:

- Avoid `method_missing` for frequently-called methods; define real methods or use delegation explicitly.
- Hoist `send`/`public_send` out of loops; call the method directly when the name is static.
- `define_method` closures capture variables and prevent some optimizations; prefer `define_method` only when the method name is dynamic.

Metaprogramming is a tool for reducing duplication in cold paths (DSLs, configuration), not for hot-path dispatch.

### Understand the garbage collector before tuning it

Modern Ruby (2.7+, 3.x) uses a generational, incremental, mostly-concurrent GC with tunable parameters (`RUBY_GC_HEAP_*`, `GC.config`, `GC.auto_compact`). Tuning rules:

- Increasing `RUBY_GC_HEAP_INIT_SLOTS` reduces growth events for allocation-heavy boots; tune for long-running processes.
- `GC.auto_compact` reduces fragmentation but adds compaction cost; enable deliberately and measure.
- Forcing `GC.start` in production almost never helps; it promotes objects prematurely and adds pause. Let the GC schedule itself.
- Tune based on measured GC stats (`GC.stat`), not intuition. The relevant metrics are `major_gc_count`, `minor_gc_count`, `heap_live_slots`, and `old_objects`.

### Memory bloat is usually retained objects, not allocation rate

A Ruby process whose RSS grows and never returns is usually retaining objects, not allocating too fast. Common retainers:

- Caches holding full objects (use IDs or TTLs; bounded caches beat unbounded `Hash`).
- Thread/actor-local state that accumulates across requests.
- Closures capturing large objects via `self`.
- Gems that hold references in class-level state.

Profile retained objects (`ObjectSpace`, `memory_profiler` retention mode) to find what survives a request cycle. The fix is removing the reference, not tuning the GC.

### JIT is not a free speedup

Ruby's built-in JIT (MJIT, then YJIT) speeds up some workloads (long-running, CPU-bound, hot loops) and adds memory and warmup cost. It is not appropriate for short-lived processes (CLI, serverless) where warmup dominates. Measure end-to-end throughput and memory with and without JIT for your workload before adopting it. YJIT in Ruby 3.1+ is generally a win for web workloads but still worth measuring.

### Benchmark honestly

Microbenchmarks lie when they ignore allocation, GC, and warmup. Use `benchmark-ips` for iterations-per-second and `memory_profiler` for allocations. Run the benchmark in the same context as production (same Ruby version, same GC settings, representative input). A 2x speedup on a synthetic input that allocates 10x more memory is a regression in production.

## Common Traps

### Caching full ActiveRecord objects

Storing `User.find(id)` in a cache retains the object, its associations, and its mutation history. Cache primitive IDs or serialized snapshots, and re-fetch the live object when needed.

### Mutating frozen strings to "avoid allocation"

Once `# frozen_string_literal` is on, `str << "x"` raises. Developers sometimes `dup` to mutate, which allocates anyway and defeats the optimization. Decide per operation whether you need a mutable buffer or can work on frozen strings.

### Optimizing a method called once per request

A 10x speedup on a method that takes 0.1ms of a 100ms request is invisible. Profile to find the method that is both slow *and* called often.

### Forcing GC to "reduce memory"

`GC.start` in a request path adds a stop-the-world pause and promotes survivors. It almost always hurts latency. The fix for memory growth is removing retained references.

### Trusting `each` over `for`

`for` in Ruby desugars to `each` and is not faster. Real wins come from algorithmic changes (O(n) vs O(n^2)) and allocation reduction, not syntactic micro-choices.

### Metaprogramming in hot paths

`send(method_name)` inside a tight loop, or `method_missing` for a frequently-called accessor, bypasses optimizations. Replace with direct calls or pre-defined methods.

### Ignoring memory in favor of speed

A change that is faster but allocates more surviving objects trades a small CPU win for larger GC and RSS cost. Always measure both wall-clock and retained memory.

## Self-Check

- Did you profile the actual hot path under realistic input before optimizing, distinguishing wall-clock, allocation, and retention costs?
- For each optimization, did you measure allocations and retained memory before and after, not just wall-clock time?
- Are hot-path files using `# frozen_string_literal: true`, and are constants/regexps hoisted out of loops?
- Have you removed `method_missing`, `send`, and `define_method` from hot-path dispatch where a direct or pre-defined method is possible?
- If you tuned GC settings, are the changes based on `GC.stat` measurements under realistic load, and have you avoided forcing `GC.start` in request paths?
- For any memory bloat, have you profiled *retained* objects to find the reference that survives the request cycle, rather than blaming allocation rate?
- If adopting JIT, have you measured end-to-end throughput and memory for your workload, including warmup cost for short-lived processes?
- Are caches storing primitive IDs or bounded TTLs rather than full objects that retain association graphs?
