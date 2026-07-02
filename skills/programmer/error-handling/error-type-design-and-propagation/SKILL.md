---
name: error_type_design_and_propagation.md
description: Use when the agent is designing error types or enums, deciding between exceptions and Result-style returns, modeling an error hierarchy, classifying recoverable versus unrecoverable failures, deciding where to catch or propagate an error, mapping internal errors to API or user-facing responses, adding context to errors that cross boundaries, or reviewing whether a failure path is handled correctly. Also covers error taxonomies, typed errors versus string errors, sentinel errors, error wrapping, retry classification (transient versus permanent), panic versus error boundaries, and the separation of internal failure detail from external error messages.
---

# Error Type Design And Propagation

Error handling is a design decision about what a failure means, not a syntax decision about how to throw it. Every failure path answers four questions: how is this failure represented, who is allowed to know about it, who is responsible for reacting to it, and where does the reaction happen. When those answers are made implicitly, the result is errors that lose meaning as they cross layers, callers that cannot tell a retryable transient failure from a permanent bug, error messages that leak internals to users, and recovery logic scattered so widely that no one path is actually correct.

Agents tend to under-invest here because the happy path works and "just propagate the error" compiles. The harm is delayed and structural: a retry layer that retries a permanent validation error forever, a user-facing message that echoes a raw database error, a caller that swallows an error it cannot actually handle, or an error type so broad that every handler must guess what went wrong. The judgment problem is deciding how to classify a failure, how to represent it so its meaning survives the journey to the layer that should act on it, and where the boundary between "propagate" and "handle" actually sits.

This skill is language-agnostic. It is about the design of failure as a first-class part of an interface. Language-specific mechanics (Rust's `?`, Go's multiple returns, exceptions, `Result` types) are the implementation of these decisions, not the decisions themselves.

## Core Rules

### Classify Failures Before Choosing A Representation

Before writing an error type, classify the failure along the axes that determine what should happen to it. The two axes that matter most are recoverability and source.

- **Recoverable versus unrecoverable.** Can a caller or operator plausibly do something useful about this failure? A network timeout is recoverable (retry, degrade, queue). A programming bug — null dereference, invariant violation, index out of range — is not recoverable in the sense that no caller can safely continue; it should surface as a hard failure, not a returned error code the caller might ignore.
- **Transient versus permanent.** Will the same call succeed if retried unchanged? Transient failures (rate limit, leader election in progress, brief network blip) may warrant retry; permanent failures (malformed input, permission denied, not found) do not, and retrying them wastes resources and can look like an attack.
- **Caller's fault versus environment's fault versus system's fault.** Did the caller send bad input (400-class), lack permission (403-class), ask for something absent (404-class), or hit an infrastructure problem (500-class)? This classification drives both the response and whether the team should be alerted.

State the classification explicitly in the error type. An error that carries a `kind` or variant (`Transient`, `Permanent`, `InvalidInput`, `Unauthorized`, `NotFound`, `Internal`) lets every layer make the right decision without re-deriving it from a message string.

### Choose The Representation To Match The Language's Failure Model

Different languages make different failure mechanisms cheap and safe. Fighting the language's model produces code that is awkward to read and easy to get wrong.

- **Result types / sum types (Rust, Haskell, Swift, modern TS/JS with tagged unions).** Errors are values in the type signature. The compiler forces the caller to acknowledge failure. Prefer this for libraries, for code where forgetting to handle an error is dangerous, and where you want failure to be explicit in the function contract.
- **Exceptions (Python, Java, C#, Ruby, JS).** Failure is control flow. Good when failures are genuinely exceptional and the call stack is the natural propagation path; dangerous when used for ordinary control flow, because every caller implicitly opts into a non-local jump. Use checked exceptions sparingly; they are often worked around rather than honored.
- **Multiple return values / error values (Go).** Error is a returned value paired with the result. Cheap and explicit, but the caller can silently ignore it (`_ = err`), so discipline and linting matter.
- **Panics / aborts.** Reserved for unrecoverable bugs where continuing would be worse than crashing. A panic is a statement that no caller can usefully recover, so it should be reserved for invariant violations, not for expected operational failures.

The representation is not a free choice: it determines whether forgetting to handle a failure is a compile error, a runtime surprise, or a silent no-op. Pick the one whose failure mode matches the cost of an unhandled error in that code.

### Make Error Types Specific Enough To Act On, Broad Enough To Evolve

An error type must carry enough structure that a handler can make a real decision, but not so much detail that every new failure forces a type change or leaks implementation.

Strong design:

- A small set of variants that map to distinct caller behaviors (`Retryable`, `Permanent`, `NotFound`, `Forbidden`, `Conflict`).
- Each variant carries the structured data needed to act: which resource, which constraint, which retry-after hint, which transient cause.
- A stable, documented set of variants at public boundaries, with internal sub-causes wrapped and hidden.

Weak design:

- A single `Error(String)` or `Exception("something failed")` that forces every handler to parse text or guess.
- A giant flat enum with fifty variants where most are only meaningful inside one module and leak implementation detail outward.
- Variants that encode the *layer* that failed (`DatabaseError`, `HttpError`) rather than the *meaning* of the failure — callers rarely care which layer failed; they care whether to retry, show the user, or give up.

Aim for variants that describe the decision a handler must make, not the location where the failure originated.

### Preserve Meaning Across Boundaries By Wrapping, Not Flattening

An error that crosses a module, network, process, or trust boundary must be translated into the vocabulary of the receiving side, while retaining enough context to diagnose the problem. The two failure modes are opposite, and both are common:

- **Flattening** — converting a rich typed error into a string or a generic `InternalError` at the boundary. This loses the structure the caller needs to react.
- **Leaking** — passing the raw internal error straight through. This exposes implementation detail (stack traces, SQL, hostnames, internal codes) to callers who should not see it, and couples the public contract to an internal type.

The correct pattern is wrapping: the boundary layer catches the internal error, records full detail in a log with a correlation id, and emits a typed public error that carries only the meaning the caller needs (`ServiceUnavailable`, retryable, retry-after 30s). The internal cause is preserved for operators; the external surface is stable and safe.

Design the wrapping deliberately at each boundary: where does a low-level `io::Error` become a domain `StorageUnavailable`? Where does that become an HTTP `503`? Each translation is a decision, not an accident.

### Decide Where To Handle, Not Just Whether To Handle

"Handle the error where it occurs" is wrong as a blanket rule. The right rule is: handle the error at the layer that has the context and authority to choose a correct response.

- A low-level socket function cannot decide whether to retry, because it does not know if the operation is idempotent, how many times it has been tried, or whether the user is waiting. It should report; a higher layer decides.
- A request handler can decide to retry a transient storage failure, to fall back to a cached value, or to return a degraded response — because it knows the operation's semantics and the user's expectations.
- A top-level boundary (HTTP middleware, CLI main, job runner) is the last line: it must ensure no error escapes unstructured, map it to a safe response or exit code, and record enough to diagnose.

For each error path, ask: which layer knows whether this is recoverable, which layer owns the retry budget, and which layer is responsible for the user-visible outcome? Handle there. Propagate everywhere else. A handler that catches an error it cannot meaningfully act on is worse than one that propagates it — it hides a problem and prevents the layer that could act from ever seeing it.

### Separate Internal Failure Detail From User-Facing Messages

The message an operator reads in a log and the message a user reads on screen are different artifacts with different audiences and different constraints. Conflating them produces either leaked internals or unhelpful logs.

- **Internal messages** should be precise, technical, and rich: include the failing component, the input that triggered it (minus secrets), the underlying cause, a correlation id, and enough stack or trace to localize the fault. These live in logs and error tracking.
- **User-facing messages** should be actionable, non-judgmental, and free of internal vocabulary. "We couldn't process that right now. Please try again in a moment." is better than "DatabaseError: deadlock detected on relation users." The latter leaks architecture and scares or misleads the user.

Never build a user-facing message by formatting an internal error directly. Map internal errors to a curated set of user-facing strings through the classification, and keep the technical detail in the log keyed by the same correlation id.

### Make Retry Decisions From Classification, Not From Hope

Retrying is a recovery strategy that must be bounded and deliberate. Retrying an unclassified error "a few times just in case" is a common source of cascading failures, wasted capacity, and masked bugs.

A retry layer should:

- Retry only failures classified as transient, and only for operations whose semantics permit repetition (idempotent or safely repeatable — see the idempotency skill for that judgment).
- Bound the number of attempts, the total time budget, and the backoff, including jitter to avoid synchronized retry storms.
- Distinguish "retry later" from "give up permanently" and surface the permanent failure cleanly rather than looping forever.
- Avoid retrying permanent failures (validation, auth, not-found); these will fail identically and the retry only delays the user and loads the system.

If you cannot classify a failure as transient, treat it as permanent. Optimistic retry of unknown failures is how systems amplify partial outages into full ones.

### Treat Missing Error Handling As A Design Smell

Silently ignored errors, empty catch blocks, `err != nil { }`, and `unwrap()` in production paths are not shortcuts; they are deferred decisions. Each one is a place where a failure will occur and produce an unexplained, unobservable symptom.

When you encounter or write one, ask which of these is true:

- The error genuinely cannot occur given the precondition — then encode the precondition in the type or assert with a clear `expect` message that explains why it is impossible.
- The error can occur but the correct response is to propagate — then propagate it instead of swallowing it.
- The error can occur and the correct response is to continue — then log it deliberately with enough context to notice, and document why continuing is safe.

A swallowed error is a future debugging session with no information. Make the choice explicit.

### Plan For Partial Failure And Cleanup

Many real failures happen mid-operation, after some side effects have already occurred. Error design must account for cleanup, not just for the error path's return value.

- Prefer operations that are atomic by construction (a single transaction, an idempotent write) so that a failure leaves no partial state.
- When partial state is possible, define the cleanup contract: does the caller retry, compensate, or reconcile? Make this explicit in the error or the interface documentation.
- Be careful with cleanup that can itself fail: a rollback that throws can mask the original error. Capture and chain the original cause so the diagnosis is not lost.

An error type that says "the operation failed" but not "the system is now in state X" forces every caller to guess the residual state. Where partial failure is possible, the error should carry enough to know what, if anything, needs repair.

## Common Traps

### The Catch-All That Swallows Everything

`catch (Exception e) { log(e); }` or `except Exception: pass` at a boundary turns every failure — including programming bugs, out-of-memory, and invariant violations — into a silent or generic problem. The system keeps running in a corrupted state, the bug is never surfaced as a crash, and the operator sees only vague logs. Catch only the specific failure types you can meaningfully handle; let unexpected failures propagate to a boundary that treats them as real incidents.

### Converting Every Error To A String Early

Calling `.to_string()` or formatting an error at the point of failure destroys the type information every downstream layer needs. A retry layer can no longer tell transient from permanent; a handler can no longer map to the right status code; tests can no longer assert on the variant. Preserve the typed error until the boundary where text is genuinely required, and convert there, deliberately.

### One Giant Error Enum For The Whole System

A single `AppError` enum used everywhere becomes a dumping ground. Every module adds variants, handlers must match against dozens of irrelevant cases, and the public surface leaks every internal failure mode. Keep error types scoped: precise internal types per module, a curated stable type at each public boundary, and translation between them.

### Retrying Based On HTTP Status Alone

Treating `5xx` as retryable and `4xx` as not is a heuristic that fails on the edges: a `429` (rate limit) is retryable, a `501` (not implemented) is not, and a `409` (conflict) depends entirely on the operation. Classify by the meaning the status carries and the operation's semantics, not by the numeric range. The same applies to retrying on any exception type whose name contains "Timeout" — read the actual cause.

### Leaking Internal Errors To Users

Returning the raw exception message, stack trace, SQL fragment, or internal error code in an API response leaks architecture, helps attackers probe the system, and confuses users. Worse, once clients depend on the leaked text, it becomes a de facto contract. Map to curated public errors at the boundary and keep detail in logs.

### Using Exceptions For Expected Control Flow

Throwing to signal "item not found" or "validation failed" in a hot path turns an ordinary branch into an expensive non-local jump and makes the control flow invisible to readers and static analysis. Reserve exceptions for genuinely exceptional conditions; use returned values for expected, frequent, or type-checkable outcomes.

### Assuming The Caller Will Figure It Out

Designing an error type with no guidance on which variants are retryable, which are permanent, and what each means forces every caller to re-derive the policy, usually wrongly. Document the classification as part of the type. If a variant's correct handling is non-obvious, the type is under-specified.

### Catching Only To Re-Throw

`catch (e) { throw e; }` (or re-wrapping without adding context) adds nothing and often strips the original stack or type. Either handle the error meaningfully at that layer, add real context and re-throw, or remove the catch and let it propagate. Catching just to re-throw is noise that hides the real handler.

### Treating Panic As A General Error Mechanism

Using panic/abort for expected operational failures (a missing config file, a network error, a user input problem) crashes the process for conditions a caller could have handled. Reserve panic for invariant violations where continuing would be unsafe; return errors for anything a caller might reasonably recover from.

## Self-Check

- [ ] Each error type classifies failures by recoverability and source (transient versus permanent, caller fault versus environment fault), and the classification is carried as structure (variants or a `kind`), not inferred from message text.
- [ ] The chosen representation (Result, exception, error value, panic) matches the language's failure model and the cost of an unhandled error in that code path; panics are reserved for genuinely unrecoverable invariant violations.
- [ ] Error variants are specific enough that a handler can choose a real action (retry, degrade, map to a status code) but do not leak internal layer names or implementation detail outward.
- [ ] At every boundary (module, network, process, trust), internal errors are wrapped into a stable public type with full detail retained in logs and a correlation id; raw internal errors are not passed through unchanged.
- [ ] Each error is handled at the layer that owns the decision (retry budget, user-visible outcome, authority to degrade), and handlers that cannot act meaningfully propagate rather than swallow.
- [ ] User-facing messages are curated, actionable, and free of internal vocabulary; technical detail lives in logs keyed by correlation id, not in the response body.
- [ ] Retry logic retries only transient failures for operations whose semantics permit repetition, with bounded attempts, total time budget, backoff, and jitter; permanent failures are not retried.
- [ ] No error path silently swallows a failure (`{}`, `pass`, ignored return) without an explicit decision documented or a deliberate log explaining why continuing is safe.
- [ ] Partial-failure cleanup is defined: operations are atomic where possible, and where partial state can remain, the error or interface documents whether the caller should retry, compensate, or reconcile.
- [ ] The error design was reviewed for the worst case — a transient failure during a partial operation, a permanent failure misclassified as transient, and an internal error reaching a user — and each has a defined correct behavior.
