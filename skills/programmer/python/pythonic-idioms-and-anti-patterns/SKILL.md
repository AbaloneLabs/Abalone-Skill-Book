---
name: pythonic_idioms_and_anti_patterns.md
description: Use when the agent is writing or refactoring Python and choosing between idiomatic and naive patterns, using iterators/generators/comprehensions/context managers/decorators, avoiding common Python anti-patterns (mutable default arguments, bare except, global state, late-binding closures, == on identities), or reviewing code for readability, correctness, and performance. Covers EAFP vs LBYL, context manager design, decorator pitfalls, dataclass usage, and performance footguns like quadratic string concatenation and repeated dict lookups.
---

# Pythonic Idioms And Anti-Patterns

"Pythonic" is not aesthetic preference; it is a set of conventions that align code with how the language and its community expect things to behave. Idiomatic code is shorter, faster, and — more importantly — matches the mental model experienced readers bring, so bugs hide less easily. Anti-patterns are the reverse: they look reasonable to a reader from another language or to a hurried author, but they break Python's assumptions about scoping, equality, mutability, and iteration.

Agents tend to translate directly from another language (loop indices instead of iteration, manual resource cleanup instead of context managers, getter/setter methods instead of properties) or to reach for clever one-liners that obscure intent. The harm is subtle: a mutable default argument shared across calls corrupts state, a late-binding closure captures the wrong value, a bare `except` swallows `KeyboardInterrupt`, a `==` where identity was meant hides a real bug. The real work is choosing the idiom that makes the code's intent and its invariants obvious.

## Core Rules

### Iterate, Do Not Index

Python iteration is the default and is both clearer and usually faster than index-based loops. Prefer `for item in collection` over `for i in range(len(collection))`. When you need both index and value, use `enumerate`. When you need parallel iteration, use `zip` (and `zip(..., strict=True)` in 3.10+ to catch length mismatches).

Strong: `for user in users:`. Weak: `for i in range(len(users)): user = users[i]`. The indexed form adds noise, invites off-by-one errors, and signals the author did not trust the language. Reserve indexing for cases where the position itself is the data (random access, windowing).

### Prefer Comprehensions And Generators Over Accumulation Loops

List/set/dict comprehensions and generator expressions express "transform and filter" more directly than a loop that appends.

- `names = [u.name for u in users if u.active]` beats a four-line append loop.
- Use a generator expression (`(transform(x) for x in xs)`) when you only need to iterate once, to avoid building an intermediate list.
- Do not nest comprehensions deeply; if a comprehension needs two `for` clauses plus a condition plus a function call, a named helper or a plain loop is clearer.

The goal is readability, not maximal concision. A comprehension that must be read three times has lost its purpose.

### Use Context Managers For Resource Lifecycle

Resources that must be closed, released, rolled back, or restored (files, locks, sockets, database transactions, temporary `sys.path` changes) belong in a `with` block. This guarantees cleanup runs even on exception, which manual `try/finally` easily forgets.

- Built-ins: `open`, `lock`, `contextlib.suppress`, `contextlib.redirect_stdout`.
- Custom: implement `__enter__`/`__exit__`, or use `@contextlib.contextmanager` on a generator that `yield`s the resource and cleans up after.
- Compose multiple managers with `ExitStack`/`contextlib.chdir` rather than nesting deeply.

The invariant: setup and teardown are paired and exception-safe. If you find yourself writing `acquire(); ...; release()` without a `with` or `finally`, that is a latent leak.

### Follow EAFP Over LBYL Where Practical

Python culture favors "Easier to Ask Forgiveness than Permission" — try the operation and handle the failure — over "Look Before You Leap" — check a condition then act. EAFP avoids race conditions (the check can be stale by the time you act) and is often faster when the failure case is rare.

- `try: value = d[key] except KeyError:` rather than `if key in d: value = d[key]`.
- But use LBYL when the check is cheap and the failure is common, or when an exception would be expensive or misleading. The choice is about cost and clarity, not dogma.

### Make Decorators Honest And Composable

Decorators are powerful but easy to abuse. A good decorator is transparent: it preserves the wrapped function's name, signature, and docstring, and its behavior is obvious from its name.

- Always use `functools.wraps` so introspection (`__name__`, `__doc__`, `__wrapped__`) works.
- Be careful with decorators that change the signature or return type; they make the call site lie about what the function does. Prefer decorators that add behavior (logging, caching, auth) without altering the contract.
- Stacking order matters: `@a @b def f` applies `b` first, then `a`. Document the intended order if it affects behavior.
- Avoid decorators with hidden global side effects (registering into a module-level dict) unless that registration is the explicit purpose.

### Use dataclasses And Properties Instead Of Boilerplate

For data containers, `@dataclass` removes `__init__`/`__repr__`/`__eq__` boilerplate and makes the fields explicit. Use `frozen=True` for immutable value types, `slots=True` (3.10+) for memory efficiency, and `field(default_factory=...)` for mutable defaults.

For computed or validated attributes, use `@property` instead of explicit getter/setter methods. Properties let an attribute start simple and gain logic later without changing the call site. Reserve them for cheap computations; a property that does I/O or heavy work violates the "attribute access is fast" expectation.

### Keep State Local And Explicit

Global and module-level mutable state is the fastest way to make code untestable and order-dependent. Prefer passing state explicitly (function arguments, a class instance, a dependency-injected object) over reading and writing module globals.

When shared state is unavoidable (a cache, a registry), isolate it behind a small, well-named API, document the lifecycle, and make concurrency explicit. A `global` statement inside a function is almost always a sign the design needs rethinking.

## Common Traps

### Mutable Default Arguments

`def f(items=[]):` shares one list across all calls because default values are evaluated once at function definition. Mutating it persists between calls. Use `None` as a sentinel and create the collection inside: `def f(items=None): items = items or []` (or `if items is None: items = []`). This trap also applies to default dicts, sets, and dataclass fields without `default_factory`.

### Bare `except` And `except Exception:`

`except:` catches everything including `KeyboardInterrupt`, `SystemExit`, and `GeneratorExit`, making the program un-killable and hiding real errors. `except Exception:` is safer but still broad. Catch the specific exceptions you can actually handle; let the rest propagate.

### Late Binding In Closures

`funcs = [lambda: i for i in range(3)]` produces three functions that all return `2`, because they close over the variable `i`, not its value at creation. Bind the current value with a default argument: `lambda i=i: i`, or use `functools.partial`.

### `==` Where Identity Was Meant

`==` compares values; `is` compares identity. Using `==` to check for `None` works but is slower and non-idiomatic; use `is None` / `is not None`. Conversely, using `is` to compare integers or strings can fail due to interning subtleties — reserve `is` for `None`, singletons, and explicit identity checks.

### Mutable Shared State Across Default Args Or Class Attributes

Class-level mutable attributes (`class C: items = []`) are shared across all instances, just like default arguments. Mutating `self.items` then modifies the class attribute. Define mutable per-instance state in `__init__`.

### Quadratic String Concatenation

`result = ""; for s in parts: result += s` is O(n^2) for many parts because strings are immutable and each concat copies the whole accumulator. Use `"".join(parts)`. Similarly, prefer comprehensions and `join` over repeated concatenation in loops.

### Repeated Dict/List Lookups In A Loop

`for k in d: process(d[k]); other(d[k])` looks up `d[k]` twice. Bind it once: `for k, v in d.items(): process(v); other(v)`. Micro, but it signals care and avoids re-evaluation when the lookup has side effects.

### Using `type(x) ==` Instead Of `isinstance`

`type(x) == int` rejects subclasses; `isinstance(x, int)` accepts them, which is almost always what you want. Reserve exact-type checks for cases where subclassing must be excluded.

### `__del__` For Cleanup

Relying on `__del__` for resource cleanup is unreliable: the interpreter does not guarantee when (or whether) it runs, especially at interpreter shutdown or in the presence of reference cycles. Use a context manager or an explicit `close()` method instead.

## Self-Check

- [ ] Loops iterate directly (`for x in xs`, `enumerate`, `zip(..., strict=True)`); index-based loops exist only where position is the data.
- [ ] Transform/filter logic uses comprehensions or generator expressions; deeply nested ones were replaced with named helpers or plain loops.
- [ ] All resources (files, locks, connections, transactions) are managed with `with` or `try/finally`; no bare `acquire()/release()` pairs.
- [ ] No mutable default arguments or mutable class-level attributes; `None` sentinels or `default_factory` are used instead.
- [ ] `except` clauses catch specific exceptions; no bare `except` or broad `except Exception` swallows `KeyboardInterrupt`/`SystemExit`.
- [ ] Closures capture loop variables by value (default arg or `partial`), not by late binding.
- [ ] `is` is used only for `None`/singletons/identity; `==` is used for value comparison; no `is` on integers/strings.
- [ ] Decorators use `functools.wraps` and do not silently change the wrapped function's contract.
- [ ] String building uses `"".join`, not repeated `+=` in loops; hot-path lookups are bound to locals.
- [ ] State is passed explicitly, not read from module globals; unavoidable shared state is isolated and its lifecycle documented.
