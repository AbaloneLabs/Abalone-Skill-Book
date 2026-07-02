---
name: ghc_and_optimization.md
description: Use when the agent is optimizing Haskell performance with GHC, choosing compiler flags and optimization levels, using INLINE and SPECIALIZE pragmas, relying on list fusion and rewrite rules, interpreting strictness analysis and CORE output, running time and space profiling, or diagnosing slow allocation, retained memory, and unexpected complexity in Haskell applications.
---

# GHC and Optimization

GHC is an aggressively optimizing compiler, and that aggressiveness is both its strength and the source of most performance confusion. GHC applies optimizations (inlining, fusion, strictness analysis, worker/wrapper transformation) that can dramatically change the runtime characteristics of code that looks identical at the source level, and it does so based on heuristics that do not always fire. The judgment problem is not "how do I add `-O2`" but understanding what GHC does to your code (what gets inlined, what fuses, what is strict or lazy after analysis), reading the evidence (CORE, profiling, allocation profiles) before changing source, and knowing which knobs (pragmas, flags, strictness annotations) move the metric you measured. Treating GHC as a black box and "trying things" produces inconsistent results because the same source change can help or hurt depending on whether a rewrite rule fires.

The recurring failure mode is a developer who adds `-O2`, sprinkles `INLINE` pragmas, or rewrites a function to be "more efficient" without profiling, then observes no improvement or a regression because the actual bottleneck was elsewhere (allocation in a hot loop, a missing `SPECIALIZE` on a polymorphic function, a rewrite rule that did not fire). The opposite failure is over-relying on GHC's optimizations to fix structurally inefficient code: a quadratic algorithm does not become linear because of fusion. Real GHC optimization requires measuring allocation and time first, understanding which optimization the hot path depends on, and making the smallest change that lets GHC do its job or that removes the structural inefficiency.

## Core Rules

### Profile before optimizing, and profile allocation as well as time

Haskell performance is dominated by allocation (garbage collection cost) and retention (space leaks), often more than CPU. Rules:

- Compile with profiling enabled (`-prof -fprof-auto`) and run with `+RTS -p` for a time profile and `+RTS -h` for a heap profile.
- `+RTS -s` gives a summary including total allocation and maximum residency; allocation proportional to input size is expected, but high constant factors or super-linear growth signal inefficiency.
- Identify the hot function by both time and allocation; optimizing a function that allocates little and runs rarely does nothing.

The first question is always: where does the allocation and time go?

### Understand optimization levels and what they enable

- `-O0`: no optimization; fast compile, slow runtime. Use for development cycles.
- `-O1` (default for `-O`): standard optimizations; the baseline for production.
- `-O2`: more aggressive optimizations (more inlining, more rewrite-rule firings, some that increase code size). Use for production after measuring, but do not assume it is always better; it can regress via code-size blowup or lost sharing.

Do not default to `-O2` without measuring; sometimes `-O1` is faster due to smaller code and better cache behavior.

### Inlining is the lever, but it must be guided

GHC inlines functions to enable further optimization (fusion, specialization), and inlining decisions are heuristic-based. Rules:

- GHC inlines functions marked `INLINE` eagerly (at call sites) and uses its own heuristics for others based on size and phase.
- `INLINE` on a small, hot function can enable fusion and specialization; use it where you depend on rewrite rules firing.
- `NOINLINE` prevents inlining; use it to prevent a function from being inlined into a context where it would hurt (e.g., a large function, or to control sharing), or to preserve a stable optimization boundary.
- `INLINABLE` exposes an unfolding for cross-module inlining/specialization without forcing eager inlining; use for library functions you expect callers to specialize.

Inlining too much can blow code size and hurt instruction cache; inlining too little can prevent fusion. Measure.

### Rewrite rules and fusion depend on structure and phase

GHC rewrite rules (library-supplied and user-supplied) transform expressions to more efficient forms; list/stream fusion is the canonical example (`map f . map g` fusing into a single pass). Rules:

- Fusion depends on the functions involved being marked for fusion and on the consumer being "good" (e.g., a fold that consumes the structure); producing a structure that is retained (e.g., `length (map f xs)`) may or may not fuse.
- Rewrite rules fire in optimization phases; `INLINE`/`NOINLINE` phase control affects whether a rule sees the right form. `[~n]` and `[n]` annotations control phase ordering.
- User-defined `RULES` are powerful and fragile; they can silently fail to fire (no warning) or fire in unintended contexts. Test that rules fire by inspecting CORE or benchmarking.

If you depend on fusion, verify it fires; do not assume.

### Specialize polymorphic functions

A polymorphic function compiled generically uses dictionary passing (for type classes) or higher-order abstraction that defeats optimization. `SPECIALIZE` (or `SPECIALIZE inline`) generates a monomorphic version for a specific type, enabling GHC to optimize it fully. Rules:

- Use `SPECIALIZE` for hot polymorphic functions at the concrete types they are called with.
- `SPECIALIZE inline` goes further and inlines the specialized version.
- Type-class-heavy numeric code (e.g., over `Num a`) often benefits dramatically from specialization.

Polymorphism has a runtime cost; specialization removes it for known types.

### Read CORE to understand what GHC produced

GHC's CORE (a small functional IR) is the ground truth for what optimizations applied. Rules:

- `-ddump-simpl` (often with `-dsuppress-all` for readability) shows the simplified CORE; this reveals whether a function was inlined, whether fusion fired, and what the final form is.
- Reading CORE is a skill; start by comparing before/after a change to see its effect.
- `-ddump-rule-rewrites` shows which rewrite rules fired.

You do not need to read CORE for every function, but for a hot path that is not behaving as expected, CORE tells you why.

### Strictness analysis and worker/wrapper

GHC's strictness analyzer detects when a function is strict in its arguments and transforms it (worker/wrapper) to evaluate them eagerly, avoiding thunk retention. Rules:

- The analyzer is good but conservative; it may not detect strictness through complex control flow.
- Bang patterns and strict data fields make strictness explicit, helping the analyzer and guaranteeing evaluation.
- Worker/wrapper can split a function into a strict worker and a lazy wrapper; understanding this helps interpret CORE.

### Benchmark honestly with criterion

Use `criterion` for microbenchmarks; it handles warmup, statistical analysis, and GC noise. Rules:

- Benchmark representative inputs, including large ones that expose allocation behavior.
- Compare before/after with criterion's analysis, not wall-clock intuition.
- Beware microbenchmarks that fuse away or that the optimizer special-cases; sometimes the benchmark measures the optimizer, not your code.

## Common Traps

### Adding `-O2` without measuring

`-O2` is not universally better; it can regress via code-size blowup. Measure before adopting.

### `INLINE` on everything

Blanket `INLINE` blows code size and can hurt instruction cache. Inline selectively where fusion/specialization depends on it.

### Assuming fusion fires

Rewrite rules silently fail to fire in many contexts. Verify via CORE or benchmark that the fusion you expect actually happens.

### Polymorphic hot path without specialization

A hot function over `Num a` pays dictionary-passing cost. `SPECIALIZE` to the concrete type.

### Optimizing without profiling allocation

Time-only profiling misses allocation-driven GC cost, which is often the dominant factor. Profile allocation (`+RTS -s`, `-h`) too.

### Trusting microbenchmarks that fuse away

A benchmark of `sum [1..n]` may measure the optimizer's fusion, not your code. Use inputs and structure that exercise the real path.

### Rewrite rules that fire in production but not tests

Rules depend on optimization level and phase; a rule that fires under `-O2` may not fire under `-O0`/tests. Test under the production optimization level.

### Structural inefficiency hidden by optimization

Fusion and inlining hide quadratic algorithms behind fast-looking microbenchmarks until input grows. Profile at scale; fix the algorithm, not the inlining.

## Self-Check

- Did you profile both time and allocation (`+RTS -p`, `-h`, `-s`) before optimizing, identifying the hot function by both metrics?
- Is the optimization level (`-O1` vs `-O2`) chosen by measurement, not by default, with awareness that `-O2` can regress via code size?
- Are `INLINE`/`INLINABLE`/`NOINLINE` pragmas applied selectively where inlining enables fusion/specialization, rather than blanket-applied?
- Have you verified (via CORE `-ddump-simpl` or benchmarking) that the fusion and rewrite rules you depend on actually fire?
- Are hot polymorphic functions `SPECIALIZE`d to their concrete call-site types, especially type-class-heavy numeric code?
- Have you read CORE for the hot path to confirm what GHC actually produced (inlined? fused? strict?), rather than assuming?
- Are bang patterns and strict data fields used to make strictness explicit where the analyzer is conservative?
- Are benchmarks run with `criterion` on representative (including large) inputs, under the production optimization level, and checked against inputs that expose allocation growth?
