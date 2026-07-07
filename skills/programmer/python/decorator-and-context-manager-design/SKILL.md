---
name: python_decorator_and_context_manager_design.md
description: Use when the agent is writing or reviewing Python decorators (function decorators, class decorators, decorator factories, stacked decorators, functools.wraps), building context managers (with-statement, __enter__/__exit__, contextlib.contextmanager, ExitStack, async context managers), designing resource lifecycle abstractions, implementing retry/caching/logging/timing wrappers, or debugging decorators that lose metadata or mismanage exceptions. Covers decorator mechanics and ordering, preserving signatures with wraps, context manager exception handling and suppression, ExitStack composition, generator-based context managers, and the pitfalls of decorators that mutate behavior invisibly or context managers that swallow exceptions.
---

# Decorator And Context Manager Design

Decorators and context managers are Python's two main tools for wrapping behavior: decorators transform or augment functions and classes, while context managers scope resource acquisition and cleanup. Both look like conveniences, but each carries hidden mechanics that determine whether the abstraction helps or silently corrupts the program. The judgment problem is designing wrappers that preserve the wrapped thing's contract, manage resources and exceptions correctly, and stay legible to the next reader.

Agents tend to write decorators that drop metadata and signatures, stack decorators without understanding the bottom-up application order, build context managers that swallow exceptions they should propagate, or use `contextlib.contextmanager` generators without handling the early-return and exception paths. The harm appears as broken introspection (help, debuggers, type checkers see the wrapper not the function), resources that leak on exception, errors that vanish inside a `with` block, and decorators so clever that no one can trace what code actually runs. The real work is respecting the wrapped contract, handling the full exception lifecycle, and choosing the simplest mechanism that expresses the intent.

## Core Rules

### Preserve The Wrapped Function's Identity And Signature

A decorator returns a new function that replaces the original. By default that new function has its own name, docstring, and signature, which breaks introspection (`help()`, tracebacks, type checkers, and tooling that relies on `__name__`/`__wrapped__`). Always use `functools.wraps` on the wrapper to copy the original's metadata.

- Apply `@functools.wraps(func)` to every wrapper function so `__name__`, `__doc__`, `__module__`, `__qualname__`, and `__dict__` are preserved and `__wrapped__` points to the original.
- `wraps` preserves metadata but not the runtime signature. Code that inspects `inspect.signature` may still see the wrapper's parameters. For decorators that must preserve the exact signature (e.g., for runtime validation or RPC frameworks), use `functools.wraps` plus a library like `wrapt` that does transparent signature-preserving wrapping, or accept the limitation explicitly.
- Document what the decorator changes about behavior. A decorator named `@retry` that also logs, caches, or mutates the return value is a debugging trap if the side effects are invisible.

A decorator that silently breaks `help()` or signature inspection erodes trust in the codebase. Preserve metadata as a default, not an afterthought.

### Understand Decorator Application Order

Decorators apply bottom-up: the decorator closest to the function is applied first, and the result is passed up. For:

```python
@dec_a
@dec_b
def f(): ...
```

the effect is `f = dec_a(dec_b(f))`. `dec_b` wraps the original first; `dec_a` wraps the result. This means `dec_a`'s wrapper runs outermost (first on entry, last on exit) and `dec_b` runs innermost.

- Reason about ordering when decorators interact: a `@retry` above a `@cache` retries the cached result; the reverse order caches the retried result. The order changes semantics.
- When a decorator must run before or after another (e.g., auth check before business logic), place it accordingly and document the dependency.
- Stacking many decorators obscures what runs; if a function has five decorators, consider whether the composition belongs in a named helper.

Misordering is a common, subtle bug because each decorator looks independent but the composition is order-sensitive.

### Distinguish Decorators, Decorator Factories, And Class Decorators

Three patterns that look similar but serve different needs:

- **Plain decorator** (`@dec`): `dec` takes the function and returns a (possibly wrapped) function. Use when the decorator needs no arguments.
- **Decorator factory** (`@dec(arg)`): `dec(arg)` returns the actual decorator. The call with arguments happens at definition time; the returned decorator wraps the function. Use when the decorator is parameterized (retry count, cache size, log level). Recognize the two-level structure: outer function takes args, inner takes the function.
- **Class decorator** (`@dec` on a class): `dec` takes the class and returns a (possibly modified) class. Use to register classes, add methods, or enforce invariants (e.g., ensuring all subclasses implement a method). Class decorators that rewrite the class heavily are hard to debug; prefer light augmentation.

Choose the simplest form. A plain decorator wrapped in a factory "for flexibility" adds a call level for no benefit if there are no arguments.

### Design Context Managers Around The Full Exception Lifecycle

A context manager's `__exit__` method receives the exception (if any) that occurred in the `with` block. The return value of `__exit__` controls whether the exception propagates: returning a true value suppresses it; returning `None`/false lets it propagate. This is the most error-prone part of context manager design.

- **Default to propagating exceptions.** A context manager that suppresses exceptions by returning `True` from `__exit__` hides bugs. Only suppress when the manager's contract is explicitly to swallow a specific exception (e.g., `suppress(FileNotFoundError)`), and document it.
- **Always run cleanup in `__exit__`.** The cleanup (closing a file, releasing a lock, rolling back a transaction) must happen whether the block succeeded or raised. `__exit__` is called in both cases; put cleanup there, not in a success-only path.
- **Do not re-raise naively.** If `__exit__` raises its own exception, it replaces the original exception from the block, which can hide the real failure. Let the original propagate by default; if you must raise, chain it (`raise NewError from exc`).

The `with` statement guarantees `__exit__` runs even on exception and on return — that is its value. Honor that guarantee by making cleanup exception-safe.

### Use contextlib.contextmanager For Simple Cases, Classes For Stateful Ones

`@contextlib.contextmanager` turns a generator into a context manager: code before `yield` is `__enter__`, the `yield` value is the bound value, and code after `yield` is `__exit__`. This is concise for simple managers.

- The generator form is ideal for managers with linear setup/teardown and no complex state.
- Handle the exception path: if the `with` block raises, the exception is thrown into the generator at the `yield`. Wrap the `yield` in `try/finally` so cleanup runs; if you do not catch, the exception propagates and cleanup still runs via `finally`.
- For managers with reusable state, multiple entry points, or complex logic, write a class with `__enter__`/`__exit__` instead. Generators are single-use (one yield); classes can be re-entered if designed for it.

`ExitStack` composes multiple context managers dynamically and guarantees cleanup in reverse order even when setup fails partway. Use it when the set of resources is not known at write time (e.g., opening a variable list of files).

### Make Decorators And Context Managers Observable

Invisible wrappers are debugging hazards. A decorator that retries, caches, or logs should be discoverable.

- Preserve metadata with `wraps` so the wrapper is traceable to the original.
- Log or expose what the wrapper did (retry attempts, cache hits) at a debug level so operators can see the augmented behavior.
- Name wrappers clearly (`_retry_wrapper`, not `wrapper`) so tracebacks point to something meaningful.
- Avoid decorators that change the return type or arguments silently; if `@cached` returns a cached value, that should be obvious from the name and documented.

## Common Traps

### Missing functools.wraps

A decorator without `@wraps(func)` replaces the function's `__name__` and `__doc__` with the wrapper's, breaking `help()`, loggers that use `__name__`, and introspection. Always wrap.

### Swallowing Exceptions In __exit__

Returning `True` from `__exit__` (or letting `contextmanager` swallow) hides the exception from the caller. Default to propagation; suppress only with explicit intent and documentation.

### Misordered Stacked Decorators

`@retry` over `@cache` differs from `@cache` over `@retry`. Reason bottom-up; document when order matters.

### contextmanager Generator That Does Not Handle The Exception Path

A generator context manager where cleanup is after a bare `yield` (not in `try/finally`) can skip cleanup if the block raises in some patterns, or replace the original exception. Use `try/finally` around the `yield`.

### Decorator Factory Applied As A Plain Decorator

`@dec` when `dec` is a factory (expects to be called) returns the decorator function instead of wrapping — the function is never wrapped and the bug is silent. Match the application to the factory shape.

### __exit__ Raising And Masking The Original

If `__exit__` raises during cleanup, it masks the exception from the `with` block. Make cleanup exception-safe; chain if you must raise.

### Context Manager Used Only For Side Effects Without A Clear Resource

A `with` block that manages no resource (just runs setup/teardown code) is often clearer as explicit calls. Reserve context managers for acquire/release lifecycles.

### Class Decorator That Rewrites The Class Opaquely

A class decorator that deletes methods or swaps the class entirely is hard to debug. Prefer light augmentation; if you rewrite, document it loudly.

## Self-Check

- [ ] Every decorator wrapper uses `@functools.wraps(func)` so `__name__`, `__doc__`, and `__wrapped__` are preserved; introspection and tooling see the original.
- [ ] Decorator application order is understood (bottom-up) and documented where decorators interact; stacked decorators do not silently change semantics.
- [ ] The decorator form matches the need: plain decorator for no args, factory for parameterized, class decorator for light class augmentation — not over-engineered.
- [ ] Context managers default to propagating exceptions; `__exit__` returns falsy unless suppression is the explicit, documented contract.
- [ ] Cleanup runs in `__exit__` (or `finally` in a generator manager) on both success and exception paths; the manager is exception-safe.
- [ ] `__exit__` does not raise in a way that masks the original exception; cleanup failures are chained or swallowed deliberately.
- [ ] `@contextlib.contextmanager` is used for simple linear managers; classes with `__enter__`/`__exit__` are used for stateful or reusable managers; `ExitStack` composes dynamic sets.
- [ ] Generator context managers wrap the `yield` in `try/finally` so cleanup runs regardless of whether the block raised.
- [ ] Wrappers are observable: clear names, debug-level logging of augmented behavior, and no silent changes to return type or arguments.
- [ ] No context manager swallows exceptions it should propagate; suppression (like `suppress(FileNotFoundError)`) is explicit and narrow.
