---
name: python_exceptions_and-error-handling.md
description: Use when the agent is designing Python exception hierarchies, choosing what to raise and catch, deciding between broad except (Exception) and specific exception types, using try/except/else/finally correctly, suppressing exceptions with contextlib.suppress, raising from chained exceptions, writing custom exception classes, handling cleanup and resource release, or reviewing code that swallows errors silently, catches BaseException, or loses the original traceback. Covers exception hierarchy design, the catch-specific-not-broad principle, exception chaining, cleanup with finally and context managers, and the boundary between recoverable exceptions and unrecoverable bugs.
---

# Exceptions And Error Handling

Python uses exceptions for both error signaling and control flow, and the language's permissive `except` makes it easy to catch too much, too little, or the wrong thing. The judgment problem is designing an exception strategy that distinguishes recoverable conditions from programmer bugs, preserves diagnostic information through propagation, cleans up resources reliably, and does not silently swallow failures that should surface.

Agents tend to write bare `except:` or `except Exception:` that catches everything including bugs, swallow exceptions with `pass`, lose the original traceback by raising a new exception without chaining, or catch exceptions they cannot actually handle. The harm appears as bugs masked as handled errors, cleanup that never runs, tracebacks that point to the handler instead of the cause, and `KeyboardInterrupt`/`SystemExit` accidentally swallowed. The real work is catching only what you can handle, chaining to preserve cause, ensuring cleanup, and reserving exceptions for conditions the caller can react to.

## Core Rules

### Catch Specific Exceptions, Never Bare `except` Or `BaseException`

The breadth of an `except` clause is a contract about what you can handle. Catching broadly masks bugs and swallows conditions you did not anticipate.

- **Never use bare `except:`** — it catches `BaseException`, including `KeyboardInterrupt`, `SystemExit`, and `GeneratorExit`, breaking Ctrl-C and shutdown.
- **Avoid `except Exception:` unless you genuinely handle all errors** (e.g., at a top-level boundary that logs and continues). It catches programmer bugs (`AttributeError`, `TypeError`, `KeyError` from logic errors) alongside real conditions, hiding bugs.
- **Catch the specific exception types you can actually react to**: `except FileNotFoundError`, `except (TimeoutError, ConnectionError)`, `except json.JSONDecodeError`. This lets unexpected bugs propagate instead of being masked.

The rule of thumb: if you cannot name the specific recovery action for an exception type, do not catch it. Let it propagate to a layer that can.

### Design A Meaningful Exception Hierarchy

When you define custom exceptions, design a hierarchy that lets callers catch at the right granularity.

- Derive custom exceptions from `Exception` (never `BaseException`), usually under a project-specific base (e.g., `class AppError(Exception)`).
- Provide a hierarchy: a broad base (`AppError`) plus specific subclasses (`ConfigError`, `ValidationError`, `NetworkError`) so callers can catch the base for "any app error" or a subclass for a specific case.
- Include actionable information in the exception: the relevant value, the operation, the context. A `ValidationError("invalid")` is far less useful than `ValidationError(field="email", value="...", reason="missing @")`.

A flat list of unrelated exception classes forces callers to catch each by name; a hierarchy lets them choose granularity.

### Use `raise ... from` To Preserve The Cause

When you catch an exception and raise a different one, chain it with `raise NewError(...) from original`. This preserves the original traceback so debuggers and logs can see the full chain.

- `raise AppError("config load failed") from exc` keeps `exc` as the `__cause__`, shown in tracebacks as "The above exception was the direct cause of...".
- Without `from`, Python may set `__context__` implicitly (during exception handling), producing "During handling of the above exception, another exception occurred", which is noisier and less clear about intent.
- Use `from None` deliberately when you want to suppress the context (e.g., translating a low-level error into a clean public API error), and document why.

Never catch an exception, construct a new message, and discard the original — the traceback and type are diagnostic gold.

### Use `try/except/else/finally` Correctly

The four clauses have distinct roles; using them wrong causes subtle bugs.

- **`try`**: the code that may raise.
- **`except`**: handles a specific raised exception. Code here should not itself be likely to raise the same kind of error.
- **`else`**: runs only if the `try` block did not raise. Put code that should run on success but should not be caught by the `except` here (e.g., further processing that might raise a different error you want to propagate).
- **`finally`**: always runs, including on exception, return, or break. Use it for cleanup that must execute (closing a resource), but keep it minimal — an exception in `finally` masks the original.

A common mistake is putting success-path code in the `try` block, where an error from that code is caught by the same `except` meant for the original operation. Use `else` to separate them.

### Prefer Context Managers Over Manual `finally` For Cleanup

Resource cleanup (files, locks, connections) is more reliable via `with` (context managers) than manual `try/finally`. The context manager guarantees `__exit__` runs even on exception, without you writing the boilerplate.

- `with open(path) as f:` guarantees the file closes.
- Custom resources should implement the context manager protocol (`__enter__`/`__exit__`) or use `contextlib.contextmanager`.
- `contextlib.suppress(SpecificError)` is the explicit, narrow way to ignore an expected exception, far clearer than `except: pass`.

Manual `finally` is appropriate when the cleanup is not naturally a context manager, but reach for `with` first.

### Do Not Swallow Exceptions Silently

`except SomeError: pass` discards an error without trace. If the error is truly expected and ignorable, say so explicitly with `contextlib.suppress(SomeError)` and a comment explaining why. Otherwise, log it, re-raise it, or translate it. Silent swallowing creates debugging black holes: a failure happens, nobody knows, and the system continues in a corrupt state.

### Reserve Exceptions For Recoverable Conditions; Let Bugs Propagate

A `KeyError` from a typo in your own code is a bug, not a condition to catch. A `FileNotFoundError` from a user-supplied path is a recoverable condition. Distinguish:

- **Recoverable conditions** (environmental, user input, transient network): catch at the layer that can react (retry, fall back, report to user).
- **Programmer bugs** (`TypeError`, `AttributeError`, `AssertionError`, logic errors): let them propagate and crash loudly during development; do not mask them with broad `except`.

Catching bugs "to be safe" turns hard failures into silent corruption.

## Common Traps

### Bare `except:` Swallowing `KeyboardInterrupt`

`except:` catches everything including `KeyboardInterrupt` and `SystemExit`, breaking Ctrl-C and clean shutdown. Always catch `Exception` at narrowest, never bare.

### `except Exception: pass` Hiding Bugs

Catching all exceptions and doing nothing masks programmer bugs as handled errors. Catch specific types and handle or log them.

### Losing The Original Traceback

`except E as e: raise NewError(str(e))` discards the original traceback and type. Use `raise NewError(...) from e` to preserve the cause.

### Catching An Exception You Cannot Handle

Catching an error just to log it and re-raise is sometimes fine, but catching it and continuing when you cannot actually recover leaves the system in a bad state. Only catch what you can meaningfully handle.

### Success-Path Code Inside `try`

Putting follow-up logic inside the `try` means its errors are caught by the same `except`. Use `else` for success-path code.

### `finally` That Raises And Masks The Original

A `finally` block that performs a fallible operation can raise and replace the original exception. Keep `finally` minimal and side-effect-free.

### Over-Broad Custom Exception Base

Catching a broad custom base like `AppError` everywhere can mask specific bugs the way `except Exception` does. Provide specific subclasses and catch at the granularity you handle.

## Self-Check

- [ ] No bare `except:` and no `except BaseException`; catches are as specific as the recovery action allows.
- [ ] Custom exceptions derive from `Exception` under a project base, with a hierarchy that lets callers catch at the right granularity.
- [ ] Exceptions carry actionable context (field, value, operation, reason), not just a generic message.
- [ ] Re-raised exceptions use `raise ... from original` to preserve the cause and traceback; `from None` is used deliberately and documented.
- [ ] `try/except/else/finally` is used with each clause in its correct role; success-path code is in `else`, not `try`.
- [ ] Resource cleanup uses context managers (`with`) rather than manual `try/finally` wherever possible.
- [ ] No exception is swallowed silently; expected-and-ignorable errors use `contextlib.suppress` with a comment.
- [ ] Recoverable conditions are caught and handled at the layer that can react; programmer bugs are allowed to propagate loudly.
- [ ] `finally` blocks are minimal and do not perform fallible operations that could mask the original exception.
