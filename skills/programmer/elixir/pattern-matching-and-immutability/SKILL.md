---
name: pattern_matching_and_immutability.md
description: Use when the agent is writing Elixir code that uses pattern matching in function heads, case and with expressions, the pipe operator, immutable data structures, working with maps and keyword lists, refactoring nested data access, or diagnosing match errors, non-exhaustive clauses, performance from copying, and readability issues in Elixir applications.
---

# Pattern Matching and Immutability

Elixir's pattern matching and immutability are the foundation of its readability and concurrency model, and they are also where developers from mutable, object-oriented languages most often write code that fights the runtime. Pattern matching is not just a switch statement; it is a binding mechanism that decomposes data while asserting shape, and using it as a control-flow afterthought (deeply nested `case`, `if` chains, defensive `Map.get`) produces code that is both verbose and fragile. Immutability is not a limitation to work around with `Agent` or `Process.put`; it is the reason Elixir code is safe to run across processes, and reaching for mutable state to "avoid copying" usually introduces contention that the copy would not have had. The judgment problem is not "how do I write a `case`" but when to pattern match in the function head versus the body, how to structure data so the happy path matches cleanly, and when immutability's copying cost is real versus imagined.

The recurring failure mode is a developer who writes imperative code in Elixir: nested `case` expressions that bury the happy path, defensive `Map.get` calls that signal the data shape was never modeled, or an `Agent` used to hold a counter that could be a value passed through a recursion. The result is code that compiles and works but is hard to read, hard to test, and slower than the idiomatic alternative because it allocates intermediate structures and locks on shared state. Real Elixir fluency is about letting pattern matching and the pipe operator express the data transformation directly, and treating immutability as an enabler rather than an obstacle.

## Core Rules

### Pattern match in function heads to dispatch by shape

Elixir allows multiple function clauses, each with its own pattern, and the runtime selects the first matching clause. This is the primary dispatch mechanism. Rules:

- Express distinct cases as separate function clauses with patterns, rather than one clause with an internal `case`. This makes each case testable and composable.
- Put the most specific patterns first; clauses are tried in order.
- Use guard clauses (`when ...`) to refine matches on values the pattern cannot express (ranges, comparisons), but keep guards pure and side-effect free.
- If a function has many clauses that share setup, consider whether the dispatch belongs in the head or whether a helper is cleaner.

### Use `case` for matching a single value, `cond` for disjoint conditions, `with` for chained happy paths

- **`case`**: match one expression against several patterns. Appropriate when the value being matched is computed once and branched on.
- **`cond`**: a sequence of boolean conditions when none of them naturally pattern-match (e.g., comparing numeric thresholds). Prefer `case` with guards when the data is structured.
- **`with`**: chain expressions that each may return an error tuple, binding the happy-path results and short-circuiting on the first `{:error, _}`. Use for linear "all must succeed" flows; do not use for general branching.

Deeply nested `case` is usually a sign the data shape should be re-modeled or the flow should be a `with`.

### Model data so the happy path matches cleanly

The quality of pattern-matching code depends on the data shape. Rules:

- Prefer tagged tuples (`{:ok, value}`, `{:error, reason}`) for results that may fail; this makes `with` and `case` clean.
- Prefer atoms for known finite sets of states (`:active`, `:paused`); pattern matching on atoms is clear and exhaustive.
- Prefer structs over bare maps for domain entities; structs give compile-time key checks and document shape.
- Avoid unstructured maps with optional keys that require `Map.get` with defaults; model the variants as distinct structs or tagged tuples instead.

If you reach for `Map.get(struct, :maybe_missing, default)`, the data model is probably underspecified.

### Use the pipe operator for linear data transformation

`|>` passes the result of one expression as the first argument of the next. It is ideal for a linear sequence of transformations on a value. Rules:

- Pipe when the flow is "take this, transform it, transform again, format it"; each step is a function call with the previous result.
- Do not pipe when the flow branches or when intermediate values are needed in multiple later steps; use explicit bindings (`let`-style) instead.
- Each piped function should take the data as its first argument; design your functions accordingly (data first, options later).

Piping into functions that do not take the data first, or mixing branches into a pipe, produces unreadable code.

### Treat immutability as the default; reach for mutable state deliberately

Elixir data is immutable; "mutation" produces a new value sharing structure with the old. Rules:

- Express state changes as functions that take old state and return new state; recursion and `Enum.reduce` are the idiomatic accumulators.
- Use `Agent`/`GenServer`/`ETS` for state only when it must be shared across processes or persisted between calls; do not use them to "avoid copying" a local value.
- The copying cost of immutability is usually negligible because BEAM shares structure; do not pre-optimize by introducing mutable state, which adds contention and complexity.

### Understand persistent data structure sharing

BEAM's lists and maps are persistent: `Map.put(map, key, value)` shares most of the map's structure with the original, so "copying" is cheap. Rules:

- Updating a small field in a large map is O(log n), not O(n); do not avoid it out of fear of copying.
- Prepending to a list (`[head | tail]`) is O(1); appending is O(n), so build lists by prepending and reverse at the end if order matters.
- For large ordered data, consider `:array` or specialized structures rather than lists.

### Make clause sets exhaustive or document the intentional fallthrough

A `FunctionClauseError` at runtime means a pattern did not match. Rules:

- For functions that should handle all cases, add a final catch-all clause or ensure the patterns cover all constructors.
- For functions that intentionally reject some inputs, let them raise (no match) or return an explicit error tuple; do not silently swallow.
- Dialyzer and `@spec` annotations help verify exhaustiveness; use them for public APIs.

## Common Traps

### Nested `case` burying the happy path

Three levels of `case` make the success path unreadable. Use `with` for linear happy paths, or restructure into multiple function clauses.

### `Agent` for a counter that could be a passed value

An `Agent` holding a counter introduces serialization and contention for what could be a value threaded through a recursion. Use mutable state only for genuinely shared, cross-process state.

### Defensive `Map.get` with defaults

`Map.get(map, :key, default)` everywhere signals the map's shape is undefined. Model the variants as structs or tagged tuples so the shape is explicit.

### Piping into branching logic

A pipe that contains a `case` or conditional at each step is unreadable. Break the pipe into explicit bindings when branches are involved.

### Appending to lists in a loop

`list ++ [item]` is O(n) per append, O(n^2) for a build. Prepend (`[item | list]`) and reverse at the end.

### Non-exhaustive clauses causing runtime `FunctionClauseError`

A missing clause for a valid input crashes at runtime. Cover all cases or document intentional rejection, and use `@spec`/dialyzer to verify.

### Overusing guards for complex logic

Guards must be pure and limited to a set of allowed expressions; pushing business logic into guards makes them unreadable and fragile. Move complex logic into the body.

## Self-Check

- Are distinct cases expressed as separate function clauses with patterns in the head, ordered most-specific first, rather than one clause with an internal `case`?
- Is `case` used for single-value matching, `cond` for disjoint boolean conditions, and `with` for linear happy-path chains (not general branching)?
- Is data modeled as tagged tuples, atoms, and structs so the happy path matches cleanly, rather than bare maps requiring defensive `Map.get`?
- Is the pipe operator used for linear transformations with data-first functions, and are branches broken into explicit bindings rather than piped?
- Is mutable state (`Agent`/`GenServer`/`ETS`) reserved for genuinely shared cross-process state, with local state threaded through recursion or `reduce`?
- Is the persistent-data-structure sharing understood (small map updates are cheap, list prepend is O(1), append is O(n)) so that immutability is not avoided out of imagined copying cost?
- Are clause sets exhaustive or intentionally rejecting with documentation, with `@spec`/dialyzer verifying public APIs?
- Are guards kept pure and simple, with complex logic moved to the function body rather than crammed into guard expressions?
