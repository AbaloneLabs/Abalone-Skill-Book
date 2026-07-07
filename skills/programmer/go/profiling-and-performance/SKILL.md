---
name: go_profiling_and_performance.md
description: Use when the agent is writing or reviewing Go code for performance — using pprof (CPU, memory, goroutine, block, mutex profiles), benchmarking and benchstat, escape analysis, inlining and compiler optimization, reducing allocations, choosing algorithms and data structures for throughput, concurrent performance (GOMAXPROCS, contention), or diagnosing slow programs, GC pressure, lock contention, false sharing, or performance regressions after a change.
---

# Go Profiling And Performance

Go has excellent built-in profiling (`net/http/pprof`, `runtime/pprof`) and benchmarking (`testing.B`) tools, but they only help if you measure before optimizing. The judgment problem is that performance intuition is unreliable: the slow part is rarely where you guess, allocations matter more than obvious "fast" code, lock contention and GC pressure dominate under load, and an "optimization" without a benchmark and a profile is a guess that often makes things slower. The discipline is to measure (profile, benchmark), identify the actual bottleneck, optimize that, and re-measure — and to resist optimizing cold paths or speculatively.

Agents tend to optimize by intuition (rewriting a loop, avoiding an interface), to add complexity without measuring, or to chase micro-optimizations that do not move the macro benchmark. The judgment problem is to profile first (CPU, allocation, contention), to benchmark before and after with `benchstat`, to understand the Go-specific performance levers (escape analysis, inlining, GC tuning, GOMAXPROCS), and to optimize the bottleneck rather than the hot-looking code. This skill is about treating performance as a measurement-driven discipline, not a guessing game.

## Core Rules

### Measure Before Optimizing: Profile CPU, Allocations, And Contention

Never optimize without a profile. Go's profilers identify where time is spent (CPU), where memory is allocated (heap), where goroutines block (block profile), and where mutexes contend (mutex profile):

- **CPU profile** (`go test -cpuprofile`, or `net/http/pprof` for a running server): shows where CPU time is spent, by function. The `top` command in `go tool pprof` lists the hottest functions; `list <func>` shows line-level cost; `web` visualizes the call graph.
- **Heap/allocation profile** (`-memprofile`, or `pprof heap`): shows allocation sources. Allocation matters because every heap allocation is GC work; reducing allocations often improves throughput more than reducing CPU.
- **Block profile** (`-blockprofile`): shows where goroutines block (on channels, mutexes, I/O), revealing serialization points.
- **Mutex profile** (`-mutexprofile`): shows lock contention, revealing locks that serialize concurrent work.

Collect profiles from a realistic workload (a production-like load test, not a trivial one). A profile from a trivial run misleads. For long-running servers, use `net/http/pprof` to capture a profile from the live process under load.

### Benchmark Before And After, With benchstat For Significance

A benchmark (`func BenchmarkX(b *testing.B)`) measures a specific operation. Run with `go test -bench=. -benchmem -count=10` for many iterations, and compare before/after with `benchstat` to determine whether a change is a real improvement or noise:

```
go test -bench=. -benchmem -count=10 > old.txt
# make the change
go test -bench=. -benchmem -count=10 > new.txt
benchstat old.txt new.txt
```

`benchstat` reports whether the difference is statistically significant. A single benchmark run is noisy (GC, scheduling, thermal); never conclude from one run. Benchmarks must respect `b.N`, reset timers after setup, and prevent compiler elision (sink results). Report allocations (`-benchmem`) because allocation reduction often matters more than time.

### Reduce Allocations; They Drive GC Pressure

In Go, allocation is the dominant performance lever for many workloads. Every heap allocation is tracked by the GC, and GC pauses and background work scale with allocation rate. Reducing allocations often improves throughput more than reducing CPU:

- Preallocate slices and maps (avoid reallocation churn).
- Use `sync.Pool` for repeatable short-lived objects (buffers, scratch).
- Use `strings.Builder` (not `+=`) for string concatenation.
- Avoid `interface{}`/`any` boxing of value types in hot paths (boxing allocates).
- Pass `[]byte` rather than `string` when converting back and forth (each `string(b)` copies).
- Use escape analysis (`-gcflags="-m"`) to find allocations that could stay on the stack.

Measure with the allocation profile; the biggest allocation sources are the targets. An allocation-free hot path is often dramatically faster.

### Understand Inlining And Escape Analysis

The Go compiler inlines small, simple functions (eliminating call overhead) and decides whether a value stays on the stack (cheap) or escapes to the heap (GC work). Both are visible with `-gcflags="-m"`:

- **Inlining**: a function that is too complex (loops, deferred calls, sometimes `interface` dispatch) is not inlined. Inlining a hot helper eliminates call overhead; if a hot helper is not inlined, simplify it or mark it (the compiler decides; you cannot force inlining, but you can reduce complexity).
- **Escape analysis**: a local that has its address taken and returned, stored in an interface, or captured by a closure escapes to the heap. Restructure to avoid unnecessary escapes (return values not pointers for small types; avoid `interface{}` where concrete works; be careful with closures capturing by reference).

An "optimization" that adds a layer of indirection (a wrapper struct, an interface) can pessimize inlining and escape decisions, making code slower. Measure.

### Tune Concurrency: GOMAXPROCS, Contention, False Sharing

For concurrent performance:

- **GOMAXPROCS**: defaults to the number of CPU cores; the scheduler distributes goroutines. Reducing it (e.g., in a container with CPU limits, or via `GOMAXPROCS` env) can improve latency under contention. Go 1.25+ automatically respects cgroup CPU limits.
- **Lock contention**: a hot mutex serializes goroutines. Profile with the mutex profile; if a lock is contended, shard the data, use lock-free structures, or reduce critical-section length. `sync.RWMutex` helps read-heavy cases.
- **False sharing**: per-goroutine counters or scratch data on the same cache line serialize through cache coherency. Pad per-goroutine data to cache-line boundaries (`[64]byte` padding) for counters updated in tight concurrent loops.
- **Goroutine creation**: goroutines are cheap but not free; for very short tasks, a worker pool reuses goroutines. For most cases, spawn per-task goroutines freely.

### Choose Algorithms And Data Structures For The Workload

Algorithmic complexity dominates micro-optimization for large inputs. Choose the right structure: a map for O(1) lookup, a sorted slice + binary search for small sets with infrequent updates, a slice for sequential access. Avoid O(n²) algorithms hiding in nested loops over large data. For throughput-critical code, the data structure choice (e.g., a flat array vs a map vs a tree) often matters more than allocation tuning.

Measure the macro benchmark (end-to-end throughput/latency), not just micro-benchmarks, because micro-optimizations that do not move the macro number are wasted effort.

## Common Traps

### Optimizing By Intuition Without A Profile

The slow part is rarely where you guess. Profile first; optimize the actual bottleneck.

### Single Benchmark Run, Concluding From Noise

One benchmark run is noisy. Use `-count=10` and `benchstat` to determine significance.

### Chasing CPU When Allocation Is The Bottleneck

Allocation drives GC pressure, often more than CPU. Check the allocation profile; reducing allocations frequently improves throughput more.

### "Optimization" Adding Indirection That Defeats Inlining

A wrapper or interface that prevents inlining or causes escapes can make code slower. Measure the effect.

### Interface Boxing In A Hot Path

`interface{}`/`any` boxing of value types allocates. Use concrete types or generics in hot paths.

### Contended Lock Serializing Concurrent Work

A hot mutex serializes goroutines. Profile with the mutex profile; shard, use RWMutex, or reduce critical sections.

### False Sharing On Per-Goroutine Counters

Per-goroutine data on the same cache line serializes through cache coherency. Pad to cache-line boundaries for tight concurrent counters.

### Optimizing A Cold Path

Time spent on a rarely-run path is wasted. Profile the actual workload and optimize the hot path.

## Self-Check

- [ ] Performance work starts with a profile (CPU, heap/allocation, block, mutex) from a realistic workload, not intuition; the bottleneck is identified before optimizing.
- [ ] Benchmarks measure before and after with `-count=10` and `benchstat`, respecting `b.N`, resetting timers, sinking results, and reporting allocations; conclusions are drawn from significant differences, not single noisy runs.
- [ ] Allocation reduction (preallocation, `sync.Pool`, `strings.Builder`, avoiding `interface{}` boxing and `string`/`[]byte` conversions) is guided by the allocation profile, targeting the biggest sources.
- [ ] Inlining and escape decisions are checked with `-gcflags="-m"`, and "optimizations" that add indirection are measured for their effect on inlining and escapes.
- [ ] Concurrency is tuned via GOMAXPROCS, lock-contention reduction (sharding, RWMutex, shorter critical sections, guided by the mutex profile), and false-sharing avoidance (cache-line padding for per-goroutine counters).
- [ ] Algorithm and data-structure choices are made for the workload's complexity characteristics, and macro benchmarks (end-to-end throughput/latency) confirm that micro-optimizations move the real number.
- [ ] Cold paths are not optimized; effort is focused on the hot path identified by the profile.
- [ ] No change ships without a before/after benchmark demonstrating a significant improvement.
