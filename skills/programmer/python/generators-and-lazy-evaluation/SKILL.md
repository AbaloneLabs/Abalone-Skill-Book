---
name: python_generators_and_lazy_evaluation.md
description: Use when the agent is designing Python generators or iterator pipelines, choosing between a generator function (yield) and a materialized list, building lazy evaluation chains with generator expressions and itertools, reasoning about one-shot consumption of generators, using yield from for delegation, implementing __iter__/__next__, handling resource cleanup in generators (close, GeneratorExit), or debugging "StopIteration", "generator ignored ValueError", or pipelines that consume too much memory. Covers generator vs list tradeoffs, single-consumption semantics, yield from, itertools composition, generator cleanup, and the tradeoff between lazy memory efficiency and reusability/debuggability.
---

# Generators And Lazy Evaluation

Generators let Python produce values lazily, one at a time, without materializing an entire collection in memory. This is powerful for large or infinite sequences and for composing pipelines, but it introduces semantics that differ sharply from lists: a generator is consumed once, errors surface late, and debugging a deep lazy chain is harder than a materialized one. The judgment problem is choosing laziness where it earns its keep, composing pipelines that do not silently break, and handling the one-shot nature of generators deliberately.

Agents tend to default to list comprehensions for everything (wasting memory on large data), or to chain generator expressions without realizing the result is single-use, or to pass a generator to a function that iterates it twice and silently gets nothing the second time. The harm appears as memory blowups on large files, `RuntimeError: generator ignored StopIteration`-style surprises, pipelines that work once in testing and fail on reuse, and errors that surface far from their cause because evaluation is deferred. The real work is using laziness for large or streaming data, materializing when reuse or debugging matters, and treating generator consumption as a first-class design concern.

## Core Rules

### Choose Generators For Large Or Streaming Data, Lists For Reuse

The core tradeoff is memory versus reusability.

- **Use a generator** (`yield`, generator expressions `(x for x in ...)`) when the data is large, streamed, or potentially infinite, and you will consume it once. A generator processing a 10GB file line by line uses constant memory; a list comprehension would load all of it.
- **Use a list** (or `tuple`) when you need to iterate multiple times, index randomly, get the length, or debug by inspecting intermediate values. A list is materialized in memory and reusable.

The mistake is using a list comprehension for a pipeline over a large dataset because it "feels simpler", then discovering memory pressure. The opposite mistake is returning a generator from a function whose callers expect to reuse the result. Match the choice to the data size and the consumption pattern.

### Remember A Generator Is Consumed Once

A generator yields its values and then is exhausted; iterating it again yields nothing. This is the single most important generator semantic.

- If a caller needs to iterate twice, either materialize to a list first (`list(gen)`) or have the function return a fresh generator each call (a generator factory).
- Passing a generator to `len()` fails; passing it to two loops fails silently on the second. Functions that accept "an iterable" should document whether they consume it once or multiple times.
- `list(gen)`, `sum(gen)`, `max(gen)` each fully consume the generator. After any of these, the generator is empty.

When you return a generator from a function, callers may not realize it is single-use. If reuse is expected, return a list, or return a callable that produces a fresh generator.

### Compose Pipelines With Generator Expressions And itertools

Lazy pipelines compose well: each stage pulls from the previous on demand. `itertools` provides the building blocks (`chain`, `islice`, `groupby`, `takewhile`, `dropwhile`, `accumulate`, `starmap`).

- Chain generator expressions: `lines = (line.strip() for line in file); non_empty = (l for l in lines if l)`. Nothing is materialized until a consumer pulls.
- Use `itertools.islice(gen, n)` to take a prefix without consuming the rest; useful for peeking or limiting infinite generators.
- `itertools.chain(*iterables)` flattens without materializing; prefer it over nested list comprehensions for large inputs.

The benefit is constant memory for large pipelines. The cost is that errors and behavior are harder to inspect because nothing exists until consumed. When debugging, insert `list()` at a stage to materialize and inspect it.

### Use `yield from` For Delegation And Flattening

`yield from subgenerator` delegates to another generator, yielding all its values, and (in modern Python with generator-based coroutines historically) propagates `send`, `throw`, and return values. Its common use is flattening nested generators and delegating to sub-iterables without an explicit `for x in sub: yield x` loop.

- `yield from` is clearer and slightly faster than the manual loop for delegation.
- It also forwards `close()` and `throw()` into the subgenerator, which matters for cleanup.

Use it when one generator's job is to yield everything from another iterable.

### Implement `__iter__` (Preferably As A Generator) For Custom Iterables

To make a custom class iterable, implement `__iter__`, which should return an iterator. The cleanest implementation is often a generator method: `def __iter__(self): for item in self._items: yield item`. This is simpler than implementing `__next__` and managing `StopIteration` by hand.

- Prefer `__iter__` (the iterable protocol) over `__next__` (the iterator protocol) for most classes. `__iter__` returning a fresh generator each call makes the object reusable across multiple iterations.
- Reserve `__next__` for when you are building a single iterator object with explicit state.

### Handle Generator Cleanup And `GeneratorExit`

When a generator is garbage-collected before exhaustion, Python runs its `finally` blocks and raises `GeneratorExit` at the `yield` point, allowing resource cleanup. This matters for generators that hold resources (open files, locks).

- Put cleanup in `try/finally` inside the generator so it runs whether the generator is exhausted or abandoned.
- `gen.close()` explicitly triggers cleanup; calling it ensures resources release promptly rather than waiting for GC.
- A generator's `finally` runs on `close()`, on normal exhaustion, and on GC — but relying on GC timing for prompt cleanup is fragile; prefer explicit `with`/`close()` for resources.

### Beware `StopIteration` Leaking Out Of Generators

Historically, a `StopIteration` raised inside a generator (e.g., from calling `next()` on an inner iterator that is empty) would silently end the outer generator. PEP 479 changed this: a `StopIteration` bubbling out of a generator is now a `RuntimeError`. When you call `next(inner)` inside a generator, catch the `StopIteration` explicitly rather than letting it leak.

## Common Traps

### Returning A Generator When Callers Expect Reuse

A function returning a generator looks like it returns a collection, but callers iterating twice get nothing. If reuse is expected, return a list or a factory.

### Passing A Generator To Two Consumers

`gen = (x for x in data); a = list(gen); b = list(gen)` — `b` is empty because `a` consumed `gen`. Materialize once and share the list.

### Memory Blowup From List Comprehension On Large Data

`[line for line in huge_file]` loads the whole file. Use a generator or iterate directly when you consume once.

### Silent Consumption By `len`, `sum`, `max`

These consume the generator entirely. Calling them "to check" leaves the generator empty for later use.

### `StopIteration` Leaking And Becoming `RuntimeError`

Calling `next(inner)` without catching `StopIteration` inside a generator now raises `RuntimeError` under PEP 479. Catch it explicitly.

### Deferred Errors Far From The Cause

Because generator expressions are lazy, an error in a transformation surfaces only when the generator is consumed, far from the expression. Materialize (`list()`) when debugging to localize the failure.

### Infinite Generator Without A Bound

A generator with no termination (`while True: yield ...`) consumed by `list()` hangs forever. Use `islice` or a consuming loop with a break condition.

## Self-Check

- [ ] Generators are used for large, streaming, or infinite data consumed once; lists are used when reuse, random access, length, or debuggability is needed.
- [ ] Functions that return generators document or signal the single-use nature, or return a list/factory when callers expect reuse.
- [ ] No generator is iterated twice without being re-materialized; `list()`/`tuple()` is used where multiple passes are needed.
- [ ] Pipelines compose generator expressions and `itertools` for constant-memory processing of large data.
- [ ] `yield from` is used for delegation and flattening instead of manual nested yield loops.
- [ ] Custom iterables implement `__iter__` (preferably as a generator method) for reusability across iterations.
- [ ] Generators holding resources use `try/finally` for cleanup, and callers use `close()` or `with` for prompt release rather than relying on GC.
- [ ] `next()` calls inside generators catch `StopIteration` explicitly to avoid PEP 479 `RuntimeError`.
- [ ] Infinite generators are only consumed with an explicit bound (`islice`, `break`), never by `list()`.
