---
name: lazy_evaluation_pitfalls.md
description: Use when the agent is writing Haskell code that suffers from space leaks, needs strictness annotations like bang patterns and strict data fields, uses foldl and accumulators, deals with bottom and non-termination, evaluates infinite or large structures, or diagnoses memory blowups, stack overflows, and unexpected laziness in Haskell applications.
---

# Lazy Evaluation Pitfalls

Haskell is non-strict by default, and laziness is the source of both its expressive power (infinite structures, compositional abstractions) and its most common runtime failures. A space leak in Haskell is almost always a thunk that the program kept alive longer than necessary: a fold that builds a chain of deferred additions instead of accumulating a value, a data structure whose fields are thunks that hold references to large computations, an `iterate` whose head retains the entire chain. The judgment problem is not "how do I write a lazy function" but understanding when laziness helps (composability, short-circuiting, streaming) and when it hurts (retained thunks, unpredictable memory, bottoms hiding in deferred computations), and knowing the tools (strictness annotations, strict folds, `seq`, `force`) to make evaluation happen when the lazy default is wrong.

The recurring failure mode is a developer who writes idiomatic-looking Haskell that works on small inputs and blows up memory on large ones, because an accumulator built a chain of thunks proportional to input size, or a record field held a reference to a computation that retained a large structure. The opposite failure is making everything strict out of fear, defeating the compositional benefits of laziness and sometimes making performance worse (by evaluating work that would have been short-circuited). Real Haskell performance work requires understanding what the program is retaining, why, and applying strictness precisely where the retention happens, not globally.

## Core Rules

### Understand thunk creation and retention

A thunk is a deferred computation. Laziness means expressions are evaluated only when their values are needed (to weak head normal form, WHNF). Rules:

- Binding `let x = expensive` creates a thunk; `x` is not computed until forced. If `x` is retained (stored in a data structure, captured in a closure), the thunk and everything it references stay alive.
- Forcing to WHNF evaluates only the outermost constructor; fields remain thunks. `seq x y` forces `x` to WHNF, not fully.
- Deep evaluation (normal form) requires `deepseq`/`force`, which forces all fields.

The question to ask constantly: what thunks is this code retaining, and do they hold references to large structures?

### Diagnose space leaks before fixing them

A space leak shows as memory growth disproportionate to the true working set. Rules:

- Profile with `+RTS -h` (heap profile) to see what type of data is retained and where.
- `+RTS -s` summarizes allocations and maximum residency; high residency relative to the problem size indicates retention.
- A fold that should run in constant space but grows linearly is the classic symptom; the accumulator is building thunks.

Do not add strictness blindly; identify the retaining structure first.

### Use the right fold and strict accumulator

- `foldl` is lazy in the accumulator: `foldl (+) 0 [1..n]` builds a thunk `(((0+1)+2)+...)` of size n, causing a space leak. Avoid `foldl` for numeric folds.
- `foldl'` (from `Data.List`) is strict in the accumulator: it forces the accumulator at each step, running in constant space. Use `foldl'` for strict left folds.
- `foldr` is right-associative and lazy; it can short-circuit (good for early exit) but builds thunks for operations that must traverse the whole list.

Match the fold to the operation: `foldl'` for strict accumulation, `foldr` for lazy/short-circuiting.

### Annotate strictness where retention occurs

Haskell provides several strictness tools, each for a specific site:

- **Bang patterns** (`!x` in a pattern or `!` before a `let` binding): force the binding to WHNF when the function is called. Use for accumulators and loop variables that must not retain thunks.
- **Strict data fields** (`data T = T !Int !String`): force fields to WHNF when the constructor is applied. Use for records whose fields should not retain thunks.
- **`Strict` and `StrictData` language extensions**: make bindings (Strict) or data fields (StrictData) strict by default at the module level. Useful for modules where laziness is the exception.
- **`seq` and `deepseq`/`force`**: explicit forcing; `seq` to WHNF, `deepseq` to normal form. Use sparingly; prefer bang patterns and strict folds for clarity.

Apply strictness at the point of retention, not globally.

### Keep laziness where it earns its keep

Laziness is valuable for:

- **Short-circuiting**: `and`, `or`, `any`, `all`, `take`, `head` stop early; strict evaluation would do unnecessary work.
- **Infinite structures**: `iterate`, `repeat`, `cycle`, lazy `Data.List` functions; only possible with laziness.
- **Compositional pipelines**: `map f . map g` fuses under laziness and can avoid intermediate structures.
- **Control structures**: `if`/`case` branches are not evaluated unless taken.

Do not make these strict; you lose the benefit. The goal is precise strictness where retention hurts, not blanket strictness.

### Beware bottoms hiding in deferred computations

Because evaluation is deferred, a bottom (`undefined`, an infinite loop, a pattern-match failure) may not surface until the value is forced, which can be far from its definition. Rules:

- A field that is never forced can hold a bottom silently; this is sometimes intended (a placeholder) and sometimes a latent bug.
- Deep `seq`/`force` can surface bottoms early, useful in testing.
- Pattern matches that are not exhaustive produce bottoms when the unmatched case is hit; `-Wall` and `-Wincomplete-patterns` flag these at compile time.

### `evaluate` and `force` for IO-bound forcing

In `IO`, `evaluate :: a -> IO a` forces a value to WHNF in the IO monad, sequencing the evaluation. Use it to control when (and in what order) evaluation happens relative to side effects. `force` (from `Control.DeepSeq`) forces to normal form; use when you need full evaluation for memory predictability.

### Test with large inputs and memory limits

Space leaks often appear only at scale. Rules:

- Test with inputs large enough to expose linear retention.
- Run with a restricted heap (`+RTS -M<size>`) to catch unbounded growth as a failure rather than a slowdown.
- Benchmark maximum residency, not just wall-clock time.

## Common Traps

### `foldl` for numeric accumulation

`foldl (+) 0` builds a thunk chain and leaks space. Use `foldl'`.

### Strict data fields applied blindly

Making all fields strict can evaluate work that would have been short-circuited or never needed, increasing allocation. Apply strictness where retention is measured, not everywhere.

### Forcing to WHNF expecting deep evaluation

`seq x y` forces `x` only to WHNF (the outer constructor); a list's spine or a record's fields remain thunks. Use `deepseq`/`force` for full evaluation.

### Retaining the head of an infinite structure

`let xs = iterate f x0 in something` retains the head, and if `something` holds `xs`, the entire computed chain stays alive. Consume with `take` or drop the reference.

### `sum` and similar Prelude functions on large lists

Some Prelude functions are lazier than expected; on large numeric inputs they can retain thunks. Check the specific function's strictness or use strict variants from `Data.List`.

### Non-exhaustive patterns producing bottoms

A missing case yields `*** Exception: Non-exhaustive patterns` only when hit, which may be rare in tests. Enable `-Wincomplete-patterns` to catch at compile time.

### Assuming laziness always fuses

`map f . map g` can fuse, but not all compositions do, and fusion depends on rewrite rules that may not fire. Profile allocation rather than assuming fusion.

## Self-Check

- Have you profiled memory (`+RTS -h`, `-s`) to identify what thunks are retained and where, rather than adding strictness blindly?
- Are numeric accumulators using `foldl'` (not `foldl`), and are folds chosen by operation (`foldl'` for strict accumulation, `foldr` for short-circuiting)?
- Is strictness applied precisely at the retention site (bang patterns for accumulators, strict data fields for records), rather than globally via blanket `Strict`/`StrictData` without reason?
- Are short-circuiting, infinite, and compositional abstractions kept lazy so their benefits are preserved?
- Are bottoms (non-exhaustive patterns, `undefined`) flagged at compile time with `-Wall`/`-Wincomplete-patterns`, and are deep `force`/`evaluate` used in tests to surface latent bottoms?
- In `IO`, is `evaluate`/`force` used to sequence evaluation relative to side effects where predictability matters?
- Have you tested with large inputs and a restricted heap (`+RTS -M`) to confirm constant-space behavior where it is required?
- Have you verified maximum residency under realistic input, not just wall-clock time, before claiming a space leak is fixed?
