---
name: csharp_linq_and_functional_patterns.md
description: Use when the agent is writing or reviewing C# code using LINQ queries and methods, choosing between deferred and immediate execution, dealing with multiple enumeration of IEnumerable, selecting IQueryable vs IEnumerable for database vs in-memory queries, composing chains of Select/Where/SelectMany, building functional pipelines, deciding when LINQ hurts readability, or reasoning about closure allocations and delegate capture in hot paths.
---

# LINQ And Functional Patterns In C#

LINQ lets you express data transformations declaratively, and that expressiveness is also its risk. A LINQ query is a description of a computation, not necessarily the computation itself: many operators defer execution until enumeration, each enumeration may re-run the whole pipeline, and the same expression that is a clean in-memory pipeline becomes a translated SQL query (with very different performance and semantics) when the source is an `IQueryable`. Agents who treat LINQ as "the functional way to write loops" miss that deferred execution, multiple enumeration, provider translation, and closure allocation are all live concerns that determine whether the code is fast, correct, or subtly broken.

The judgment problem is to know, for each query, when it executes, how many times it executes, what executes it (your code or a database engine), and what it allocates. LINQ that reads beautifully can enumerate a source three times, translate an unsupported method into client-side evaluation that pulls a whole table, capture a variable into a closure that allocates on every call, or hide a nested loop behind a `SelectMany` that is O(n*m). The skill is not "use LINQ" or "avoid LINQ"; it is to use LINQ where its semantics are clear and its costs are known, and to recognize when a plain loop is more honest.

## Core Rules

### Know Whether Each Operator Defers Or Executes Immediately

LINQ operators split into two groups, and confusing them causes both correctness and performance bugs.

- **Deferred operators** (`Select`, `Where`, `OrderBy`, `SelectMany`, `Take`, `Skip`, `GroupBy`, `Join`, `Distinct`, `Reverse`, `Cast`, `OfType`, `Concat`, `Zip`, etc.) do no work when called; they build a query object that executes when enumerated. Calling `Where(...)` does not filter anything until you enumerate the result.
- **Immediate operators** (`ToList`, `ToArray`, `ToDictionary`, `ToLookup`, `Count`, `Any`, `All`, `First`, `Single`, `Max`, `Min`, `Sum`, `Average`, `Aggregate`, `SequenceEqual`, `Contains`) execute when called, enumerating the source once and producing a value or materialized collection.

The consequence: a query built from deferred operators does nothing until you enumerate it, and each enumeration re-runs it. If you need the result more than once, materialize it once with `ToList`/`ToArray` and reuse the list. If you need to know the count, `Count()` on a deferred query re-runs the whole pipeline each time.

### Avoid Multiple Enumeration Of A Deferred Source

Enumerating an `IEnumerable<T>` that represents a deferred query more than once re-executes the query each time. For an in-memory source this is wasted CPU; for a database-backed `IQueryable` it is a fresh round trip each time; for a source with side effects (reading a file, consuming a stream) it can throw or produce different results on the second pass.

- If you enumerate the same `IEnumerable` twice (e.g., `if (items.Any()) { foreach (var x in items) ... }`), materialize first: `var list = items.ToList();` then check and iterate the list.
- Be suspicious of methods that take `IEnumerable<T>` and enumerate it multiple times internally; prefer taking `IReadOnlyList<T>` or `IReadOnlyCollection<T>` when the method needs to enumerate more than once or needs the count, to make the contract explicit at the type level.
- The ReSharper/Roslyn "possible multiple enumeration" warning exists for a reason; do not silence it without deciding whether to materialize.

Strong choice: `int Count<T>(this IReadOnlyCollection<T> source) => source.Count;` with the type carrying the "already materialized" guarantee. Weak choice: a method taking `IEnumerable<T>`, calling `.Count()` and then `foreach`, re-running the source twice.

### Distinguish IQueryable From IEnumerable By Provider

`IEnumerable<T>` executes in process, operator by operator, via delegates. `IQueryable<T>` builds an expression tree that a provider (Entity Framework, LINQ to SQL) translates into a target language (SQL) and executes remotely. The same method name means very different things.

- On `IQueryable`, the expression is translated; methods the provider cannot translate either throw at runtime or trigger client-side evaluation (pulling rows into memory and evaluating there), which can be catastrophically slow and can change semantics (e.g., a `Where` that runs client-side pulls the whole table first).
- C# method syntax does not tell you which world you are in; the source type does. A query over `DbContext.Set<T>()` (IQueryable) is translated; the same operators over `new List<T>()` (IEnumerable) run in memory. Mixing them (e.g., calling an in-memory helper inside an IQueryable query) forces client-side evaluation.
- Keep query construction in `IQueryable` until you genuinely need in-memory results, then materialize (`ToList`, `AsEnumerable`) and continue with `IEnumerable`. Do not call arbitrary .NET methods inside an IQueryable `Where` and expect translation.

### Mind Closure Captures And Delegate Allocation

Each LINQ lambda that captures a local variable allocates a closure object and a delegate. In a hot path (called millions of times), this allocation adds up to GC pressure.

- A lambda that captures nothing can be cached as a static delegate; a lambda that captures locals allocates a closure and a delegate instance per invocation. In hot paths, hoist the lambda to a static method or use a cached delegate.
- `foreach` loop variable capture in older C# versions had the famous "all lambdas see the last value" bug; modern C# fixed this by making the loop variable per-iteration, but be aware when reading older code.
- For performance-critical code, a plain `for` loop over a list with indexed access avoids both the enumerator allocation and the delegate allocation, and is often clearer about intent than a convoluted LINQ expression.

This is not a reason to avoid LINQ in general; it is a reason to use a loop in the small number of places where allocation matters measurably.

### Choose Readable Queries, Not Maximally Clever Ones

LINQ enables dense functional pipelines, but density is not clarity. A chain of seven operators with nested lambdas and a `SelectMany` may be correct and unreadable; the next maintainer will not know whether it is correct or rewrite it to find out.

- Prefer method syntax for short, linear pipelines (`items.Where(...).Select(...).ToList()`); prefer query syntax only when it is materially clearer (multiple `from`/`SelectMany`, `group ... by ... into`, range variables that improve readability).
- Split long pipelines into named steps or intermediate variables with descriptive names. A pipeline named in stages is easier to debug and to test than a single expression.
- When a query expresses a nested loop or a complex join, ask whether a plain `foreach` with an accumulator would be clearer. LINQ is a tool, not a mandate; a loop that reads as a loop is better than a LINQ expression that reads as a puzzle.

### Understand The Performance Characteristics Of Each Operator

Operators have different costs, and chaining them without awareness produces accidental quadratic behavior.

- `OrderBy` followed by `ThenBy` is fine (single sort); multiple independent `OrderBy` calls each re-sort.
- `Distinct`, `GroupBy`, `Join`, `GroupJoin` build internal hash-based structures and allocate; on large inputs they are O(n) time but non-trivial memory.
- `First`/`Any` short-circuit (stop at the first match); `Count` and `Sum` enumerate everything. Use `Any()` not `Count() > 0` to check for presence.
- `ElementAt` on an `IEnumerable` (not `IList`) enumerates up to the index; in a loop calling `ElementAt(i)`, this is O(n^2). Use indexed access on a list.
- `Contains` on a list is O(n); on a `HashSet` it is O(1). If you check membership repeatedly, build a `HashSet` once.

## Common Traps

### Calling Count() When You Mean Any()

`if (items.Count() > 0)` enumerates the entire source to count it; `if (items.Any())` stops at the first element. For a presence check, always use `Any()`. This is both a performance and (for side-effecting sources) a correctness issue.

### Multiple Enumeration From Passing IEnumerable Around

A method that takes `IEnumerable<T>`, checks `.Any()`, then `foreach`-es, then maybe `.Count()`-es, runs the source up to three times. For a database query that is three round trips; for a file read it can throw on the second pass. Materialize once at the boundary and pass `IReadOnlyList<T>`.

### Client-Side Evaluation Of An IQueryable

Calling an unsupported .NET method (a custom function, `DateTime.Parse`, a property that is not mapped) inside an EF `Where` either throws or evaluates client-side, pulling the entire table into memory before filtering. Check the generated SQL or use methods the provider documents as translatable.

### Accidental Quadratic Behavior With ElementAt Or IndexOf In A Loop

`for (int i = 0; i < seq.Count(); i++) { var x = seq.ElementAt(i); ... }` is O(n^2) because each `ElementAt` re-enumerates from the start. If you need indexed access, materialize to a list and use the indexer.

### Capturing Loop Variables Or Allocating Closures In Hot Paths

A LINQ query inside a tight loop that captures locals allocates a closure per iteration; in a hot path this is significant GC pressure. Hoist the lambda or rewrite as a loop where allocation matters.

### Overusing LINQ For Imperative Logic With Side Effects

LINQ is designed for pure transformations. Using `Select` or `ForEach` (via `ToList()`) to mutate external state or perform I/O hides side effects in a functional-looking pipeline and is hard to reason about. Use a plain `foreach` when the body has side effects; reserve LINQ for computing values.

### Assuming Order Is Preserved When It Is Not

`Where` and `Select` preserve order for sequences; `GroupBy` does not guarantee group order (in some providers); `Distinct` historically did not guarantee order before .NET Core fixes. Do not depend on ordering unless the operator documents it or you add an explicit `OrderBy`.

### Treating LINQ As Always Faster Or Always Slower

Neither extreme is true. LINQ adds enumerator and delegate overhead versus a hand-written loop, but it also enables deferred composition and provider translation that a loop cannot. Measure in the cases that matter; do not optimize or pessimize by superstition.

## Self-Check

- [ ] Each query's execution timing is known (deferred vs immediate), and sources that will be enumerated more than once are materialized with `ToList`/`ToArray` at the boundary.
- [ ] Methods that need multiple enumeration or a count take `IReadOnlyList<T>`/`IReadOnlyCollection<T>` rather than `IEnumerable<T>`, making the contract explicit.
- [ ] Queries over `IQueryable` stay in expression form until materialization, with no untranslatable .NET methods that would force client-side evaluation or pull whole tables.
- [ ] Presence checks use `Any()`, not `Count() > 0`; membership checks on repeated lookups use a `HashSet`, not `Contains` on a list.
- [ ] No accidental quadratic patterns (`ElementAt`/`Count` in loops, repeated `OrderBy`) exist in the code.
- [ ] Closure captures and delegate allocations in hot paths have been identified and either hoisted or replaced with plain loops where allocation matters.
- [ ] Long or dense LINQ pipelines are split into named, readable stages, and a plain `foreach` is used where it would be clearer or where the body has side effects.
- [ ] Ordering is not assumed where the operator does not guarantee it; explicit `OrderBy` is used where order matters.
- [ ] The choice between LINQ and a hand-written loop reflects measured performance needs and readability, not a stylistic default.
- [ ] For database-backed queries, the generated SQL has been inspected to confirm the intended translation and to rule out client-side evaluation.
