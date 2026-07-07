---
name: javascript_closure_and_scope.md
description: Use when the agent is designing JavaScript functions that rely on closures as a design tool — the module/IIFE pattern, currying and partial application, memoization, once/debounce/throttle wrappers, private state encapsulation, iterators and generators, or reasoning about closure memory retention and lifecycle. Covers closure-based design patterns, factory and higher-order function design, encapsulation via closures vs classes vs private fields, and the memory and lifecycle tradeoffs of long-lived closures. Distinct from the rules of scope and this binding, which are covered separately.
---

# Closure And Scope As A Design Tool

Closures are not just a scope rule to avoid bugs — they are the primary mechanism for encapsulation, partial application, and stateful function design in JavaScript. A closure captures the variables of its enclosing scope and keeps them alive for as long as the closure itself is reachable. The judgment problem is using closures deliberately as a *design tool*: when to encapsulate state in a closure versus a class, how to build higher-order functions that configure behavior, and how to manage the memory and lifecycle of closures that outlive their creation context.

Agents tend to reach for classes by default when a closure factory would be simpler, build closures that capture more than they need (retaining large objects), or copy imperative patterns from other languages instead of leveraging JavaScript's first-class functions. The harm appears as over-engineered class hierarchies for stateful helpers, memory leaks from event handlers and caches that close over large scopes, and missed opportunities for clean functional composition. The real work is choosing the right encapsulation mechanism, designing higher-order functions that capture only what they need, and releasing closures when their work is done.

## Core Rules

### Choose Closure, Class, Or Private Field By The Problem

JavaScript offers three ways to encapsulate state, and the choice should match the problem, not habit.

- **Closure factory**: a function that returns an object or function capturing private variables. Best for single-purpose stateful units (a counter, a debounced handler, a memoized function) where the state is private and the API is small. The state is truly private (not even accessible to subclasses) and the surface is minimal.
- **Class**: best when you need multiple instances with shared behavior, inheritance, or a rich API with many methods operating on shared state. Classes make the relationship between instances explicit and support `instanceof` and subclassing.
- **Private class fields** (`#field`): best when you want class structure with true privacy. `#x` is inaccessible outside the class, unlike the convention-based `_x`. Use them when a class is the right shape but you need encapsulation.

Do not default to a class for a single stateful function. A `createCounter()` closure factory is clearer than a `Counter` class when there is one piece of state and one or two operations. Conversely, do not force a closure pattern onto something that genuinely needs many instances and inheritance.

### Use The Module/IIFE Pattern For Encapsulation

Before ES modules, the Immediately-Invoked Function Expression (IIFE) was the standard way to create a private scope. The pattern is still relevant for creating a private namespace within a module or for one-time setup that should not leak names.

- An IIFE `(function() { var private = ...; window.public = ...; })()` creates a scope, runs once, and exposes only what is attached externally. Modern code uses ES modules for file-level privacy, but the IIFE pattern remains useful for grouping related setup or creating a private scope inside a larger module.
- The *revealing module pattern* (return an object of public functions that close over private state) gives a clear public API while keeping internals private. It is a closure factory applied at module scale.
- Prefer ES module `export` for inter-file encapsulation; use IIFEs for intra-file privacy where a helper should not be visible outside its immediately enclosing scope.

### Build Higher-Order Functions With Currying And Partial Application

Closures enable functions that return functions, capturing some arguments now and the rest later. This is the foundation of functional composition.

- **Currying** transforms `f(a, b, c)` into `f(a)(b)(c)`, each step returning a function that captures the previous arguments. Useful for configuration (`const add = a => b => a + b; const add5 = add(5)`) and for composing with APIs that expect single-argument functions.
- **Partial application** fixes some arguments and returns a function awaiting the rest (`const log = (level) => (msg) => console.log(level, msg); const info = log("info")`). Use it to specialize generic functions for specific contexts.
- Capture only what the returned function needs; do not close over an entire configuration object when one field suffices, to keep memory minimal and intent clear.

These patterns shine for building configurable handlers (event handlers bound to specific data, API clients configured with base options) without repeating parameters.

### Encapsulate Stateful Logic: Memoize, Once, Debounce, Throttle

Closures are the natural home for stateful wrappers that track calls, results, or timing.

- **Memoization**: a closure caches results keyed by arguments so repeated calls skip recomputation. Ensure the cache key is correct (serialize objects consistently) and that the function is pure (memoizing side-effectful functions causes wrong behavior).
- **Once**: a closure with a flag ensures a function runs only once, returning the cached result thereafter. Use for initialization that must not repeat.
- **Debounce/throttle**: closures track timers/last-run to coalesce rapid calls. They must clean up timers on teardown (return a cancel function) or they leak.

For all of these, the closure must expose a way to reset or cancel its state, or it accumulates indefinitely. A debounce without a cancel leaks the pending timer if the component unmounts.

### Design Iterators And Generators Around Closure State

Generators (`function*`) and iterators use closure-like state to produce sequences lazily. The state (the current position) is retained between `next()` calls.

- Generators are clearer than hand-rolled closure iterators for sequences: `function* naturals() { let n = 1; while (true) yield n++; }`.
- Closures can build custom iterators (`const iter = () => { let i = 0; return () => ++i; }`) when a generator does not fit.
- Lazy evaluation via closures/generators avoids materializing large sequences; use them for streams, pagination, and infinite sequences consumed on demand.

### Capture Only What You Need; Release What You Do Not

A closure keeps alive every variable it references, for as long as the closure is reachable. This is the core memory tradeoff.

- A long-lived closure (event handler, interval callback, cache) that references a large object keeps that object from garbage collection even after the rest of the program is done with it. Capture the minimum: extract the specific values needed rather than closing over a whole context or DOM tree.
- Release closures when their work is done: remove event listeners (`removeEventListener` requires the *same* function reference), clear intervals/timeouts (`clearInterval`), and null out references to caches.
- Audit what a closure actually closes over; a callback that "only needs the id" but is defined inside a scope holding a large `event` object retains the whole event.

## Common Traps

### Over-Engineering A Class For A Single Stateful Function

A `Counter` class with one field and `inc()` is heavier than a `createCounter()` closure factory when inheritance and multiple rich methods are not needed. Match the mechanism to the problem.

### Capturing More Than Needed In A Long-Lived Closure

An event handler closing over a large component object retains it. Extract the specific values the handler needs; do not close over the whole scope.

### Forgetting To Release Event Listeners And Timers

`addEventListener` without a matching `removeEventListener` (same reference) leaks the handler and everything it closes over. Intervals never cleared run forever. Return and call cancel/teardown functions.

### Memoizing An Impure Function

Caching results assumes the function is pure. Memoizing a function with side effects or one that depends on external mutable state returns stale results and skips necessary effects.

### Debounce/Throttle Without A Cancel

A debounced function with a pending timer leaks if the component unmounts before the timer fires. Always expose a cancel method and call it on teardown.

### Convention-Based Privacy (`_field`) Treated As Real Privacy

`_x` is a convention, not enforcement; any code can read or write it. Use closure-private variables or `#field` for real encapsulation.

### Closing Over A Loop Variable By Reference

A closure created in a loop captures the variable, not its value at creation (with `var`). Use `let` (block-scoped per iteration) so each closure captures its own value.

### Currying That Obscures Intent

Over-currying (`f(a)(b)(c)(d)`) can be hard to read. Use partial application where it clarifies configuration, but do not curry everything reflexively; readability matters more than functional purity.

## Self-Check

- [ ] The encapsulation mechanism (closure factory, class, private `#field`) matches the problem: single stateful unit vs multiple instances with inheritance vs class with privacy.
- [ ] The module/IIFE or revealing-module pattern is used for intra-file privacy where ES module export is not sufficient; public APIs are explicit.
- [ ] Higher-order functions (curry/partial application) capture only the arguments they need and clarify configuration rather than obscure it.
- [ ] Stateful wrappers (memoize/once/debounce/throttle) are applied only to pure functions where caching is safe, and expose reset/cancel methods.
- [ ] Long-lived closures capture the minimum necessary; large objects are not retained after the rest of the program is done with them.
- [ ] Event listeners and timers are released (matched `removeEventListener`, `clearInterval`) on teardown; cancel functions are exposed and called.
- [ ] Privacy is real (closure-private or `#field`), not convention-based (`_field`), where encapsulation actually matters.
- [ ] Loop closures use `let` so each captures its own value, not the final loop value.
- [ ] Generators/iterators are used for lazy sequences instead of materializing large or infinite collections.
- [ ] No closure retains a large scope (whole event/component object) when only a few values are needed.
