---
name: exception_handling.md
description: Use when the agent is designing a Java exception strategy, choosing checked vs unchecked exceptions, writing try/catch/finally blocks, implementing AutoCloseable and try-with-resources, creating custom exception hierarchies, translating exceptions at API boundaries, using Optional vs checked exceptions, suppressing exceptions, or diagnosing swallowed exceptions, resource leaks from missing finally, lost cause chains, overly broad catch (Exception/Throwable), exception masking in finally, or broken AutoCloseable contracts. Covers the checked-vs-unchecked debate, exception translation, resource management, cause chaining, and the failure modes of bad exception handling.
---

# Exception Handling

Java's exception model is checked-exception-based by design, and this design choice has produced thirty years of debate, workaround patterns, and silent bugs. The checked exception mechanism forces callers to declare or handle `IOException`, `SQLException`, and similar recoverable failures — which is valuable for forcing attention on real failure modes, but which also produces the most common Java anti-pattern: catching a checked exception and swallowing it (or wrapping it in a generic `RuntimeException`) just to make the compiler happy. The result is that failures disappear: the system continues running in a corrupted state, errors are logged at DEBUG and never seen, and the root cause is unrecoverable because it was discarded three layers down.

The judgment problem is not "how do I write try/catch" but "which failures are recoverable and which are fatal, where should each failure be caught and what should happen, how do I preserve the cause chain so debugging is possible, and how do I manage resources so that no `finally` block masks the real exception or leaks a handle." Agents tend to catch too broadly (`catch (Exception e)`), swallow exceptions (empty catch blocks), lose the cause when wrapping (`throw new RuntimeException(e.getMessage())` instead of `throw new RuntimeException(e)`), forget `finally` or `try-with-resources` for cleanup, and design exception hierarchies that are either too flat (everything is `AppException`) or too deep (a class per error code). The cost is systems that fail silently, leak resources, and produce stack traces that point at the wrong line.

## Core Rules

### Decide Checked vs Unchecked Based On Recoverability, Not Convenience

The checked-vs-unchecked decision should reflect whether the caller can realistically recover. A checked exception (`IOException`, `InterruptedException`, `TimeoutException`) says "this failure is expected and the caller must decide what to do" — it forces a decision at compile time. An unchecked exception (`RuntimeException` and subclasses, including `NullPointerException`, `IllegalArgumentException`, `IllegalStateException`) says "this is a programming error or an unrecoverable condition; let it propagate to a boundary handler."

The modern consensus, and the right default for application code, is to prefer unchecked exceptions for most application failures, because the recovery logic that checked exceptions force is usually "log and propagate" anyway, and checked exceptions leak through layers and viral `throws` clauses. Use checked exceptions when: the failure is a legitimate, expected condition that a well-written caller will handle differently (e.g., `TimeoutException` in a retry loop), or you are writing a library whose callers span contexts you cannot predict (the JDK's `IOException` is the canonical example). Do not use checked exceptions for programming errors (`InvalidStateException` from bad input validation should be unchecked) or for conditions the caller cannot recover from.

When in doubt, prefer unchecked and document the failure modes in the method's Javadoc (`@throws`). A documented unchecked exception is better than a checked exception that callers swallow, because documentation at least informs the reader while swallowing actively hides the failure.

### Never Swallow Exceptions; Translate Or Propagate

The worst exception handling is the empty catch block: `catch (Exception e) {}`. This discards the failure entirely — the system continues in an unknown state, the operator never sees the error, and debugging is impossible. The second-worst is logging-and-continuing without consideration: `catch (Exception e) { log.warn("failed", e); }` is acceptable only if continuing in a degraded state is genuinely correct (e.g., a best-effort cache refresh), not as a default.

When you catch an exception, you must do one of three things: recover (retry, fall back to a default, and continue in a known-good state), translate (wrap it in an exception appropriate to your layer and rethrow, preserving the cause), or propagate (let it bubble to a boundary that handles it). Recovery requires that you actually know how to recover — if you do not, do not catch. Translation requires preserving the cause (see below). Propagation is the default; catching should be a deliberate, narrow act, not a reflex.

### Always Preserve The Cause When Wrapping

When you translate an exception, you must pass the original as the cause: `throw new MyException("context", e)`, never `throw new MyException(e.getMessage())` or `throw new MyException("failed")`. The cause chain is the only way to trace a failure from the symptom back to the root; discarding it makes debugging a forensic exercise. Every exception constructor accepts a `Throwable cause`, and `Throwable.initCause` is available for exceptions that lack such a constructor.

The cause should carry enough context to explain what was being attempted: not just "failed" but "failed to connect to database at " + url, or "failed to parse config " + path. The original exception's message and stack trace are preserved automatically; your job is to add the layer-specific context (what operation, what inputs) that the original exception did not know about. A wrapped exception with a good cause and contextual message is debuggable; one without is a dead end.

### Use try-with-resources For All AutoCloseable Resources

Any resource that implements `AutoCloseable` (`InputStream`, `OutputStream`, `Connection`, `PreparedStatement`, `ResultSet`, `Socket`, `Lock` via a wrapper) must be closed, and the correct way to close it is `try-with-resources`, not a manual `finally`. `try-with-resources` guarantees the resource is closed even if the body throws, handles suppressed exceptions correctly (if both the body and `close()` throw, the close exception is attached as suppressed rather than masking the body exception), and is strictly better than hand-written `finally` blocks.

The classic bug that `try-with-resources` eliminates is exception masking in `finally`: if the `try` body throws and then `close()` in `finally` also throws, the second exception replaces the first, and the real cause is lost. `try-with-resources` attaches the close exception as suppressed (`tryException.addSuppressed(closeException)`), preserving both. Never hand-write `finally` for resource cleanup when `try-with-resources` is available — it is more verbose and more bug-prone.

When composing multiple resources, declare them in order in the `try` header; they are closed in reverse order, which is correct (close the `ResultSet` before the `Statement` before the `Connection`). Nesting `try-with-resources` blocks is also valid and sometimes clearer for resources with different lifetimes.

### Catch Specific Types, Never Exception Or Throwable

`catch (Exception e)` and `catch (Throwable e)` are almost always wrong. They catch everything — including `NullPointerException` from a bug in the handler, `OutOfMemoryError` (which you should never catch), and exceptions from completely unrelated code that happened to be in the `try` block. Catch the specific exception types you intend to handle: `catch (IOException e)`, `catch (SQLException e)`, and if you need to handle multiple, use multi-catch (`catch (IOException | SQLException e)`).

The one place a broad catch is legitimate is at a top-level boundary (a request handler, a thread's `run` method, a message consumer loop) where the goal is to prevent the thread or request from dying and to log/report the unexpected failure. Even there, catch `Exception` (not `Throwable` — never catch `Error` subclasses like `OutOfMemoryError` or `StackOverflowError`, which indicate the JVM is in a broken state and recovery is unsafe). In all other places, narrow catches force you to think about what you are actually handling.

### Design Exception Hierarchies Shallow And Meaningful

A custom exception hierarchy should be shallow (two or three levels at most) and organized by recovery category, not by error code. A root `AppException` with subclasses like `ValidationException`, `NotFoundException`, `ConflictException`, and `UnavailableException` lets callers catch by the category they can handle (`catch (NotFoundException e) { return 404; }`). A hierarchy with one class per error code (`UserNotFoundException`, `AccountNotFoundException`, `InvoiceNotFoundException`) forces callers to catch dozens of types and produces brittle code.

Do not create a custom exception unless it carries information or semantics that a standard exception does not. `IllegalArgumentException`, `IllegalStateException`, `UnsupportedOperationException`, and `NoSuchElementException` cover most programming-error cases. Reserve custom exceptions for domain-specific recoverable conditions or for adding structured fields (an error code, a failing field name) that callers will actually use.

## Common Traps

### The Empty Catch Block

`catch (Exception e) {}` swallows the failure silently. Always log, recover, translate-with-cause, or rethrow — never discard.

### Wrapping Without The Cause

`throw new RuntimeException(e.getMessage())` loses the original stack trace. Always pass the original exception: `throw new RuntimeException("context", e)`.

### Catching Exception Or Throwable

Catches bugs in your own handler, masks `OutOfMemoryError`, and conflates unrelated failures. Catch specific types; use multi-catch for several.

### Manual finally For Resource Cleanup

Hand-written `finally` blocks mask the body exception if `close()` also throws. Use `try-with-resources`, which attaches suppressed exceptions correctly.

### Using Checked Exceptions For Programming Errors

`throws ValidationException` (checked) for invalid input forces every caller to declare it, even those that just propagate. Make programming-error exceptions unchecked.

### Closing Resources In The Wrong Order

Closing a `Connection` before its `Statement` before its `ResultSet` can fail or leave the statement hanging. Close in reverse acquisition order; `try-with-resources` does this automatically.

### Logging And Continuing As A Default

`catch (Exception e) { log.warn(e); }` without a recovery plan leaves the system in an unknown state. Log-and-continue is correct only for genuinely best-effort operations.

## Self-Check

- [ ] Every exception is either recovered from (with a known-good state restored), translated (wrapped with the cause preserved and layer-specific context added), or propagated — there are no empty catch blocks and no log-and-continue without a documented recovery rationale.
- [ ] Checked exceptions are used only for expected, recoverable conditions or in library APIs with unpredictable callers; programming errors and unrecoverable conditions use unchecked exceptions documented via `@throws`.
- [ ] All wrapped exceptions preserve the original cause via the two-argument constructor or `initCause`, with a message that adds layer-specific context (operation, inputs) the original did not have.
- [ ] All `AutoCloseable` resources (`InputStream`, `Connection`, `ResultSet`, etc.) are managed with `try-with-resources`, never hand-written `finally`; multiple resources are declared in acquisition order and closed in reverse.
- [ ] Catch blocks target specific exception types (or use multi-catch), never `Exception` or `Throwable` except at documented top-level boundaries where catching `Exception` prevents thread/request death.
- [ ] The custom exception hierarchy is shallow (≤3 levels) and organized by recovery category, not by error code; standard exceptions (`IllegalArgumentException`, `IllegalStateException`) are used where they suffice.
- [ ] No `Error` subclass (`OutOfMemoryError`, `StackOverflowError`) is caught anywhere — these indicate a broken JVM state and recovery is unsafe.
- [ ] Suppressed exceptions are considered: when a body exception and a close exception both occur, `try-with-resources` is used so both are preserved rather than the close exception masking the body exception.
