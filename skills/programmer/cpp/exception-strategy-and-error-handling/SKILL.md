---
name: cpp_exception_strategy_and_error_handling.md
description: Use when the agent is designing or reviewing C++ error handling strategy — exceptions vs error codes vs std::expected, noexcept specifications, exception safety guarantees (basic/strong/nothrow), exception hierarchies, what to throw and catch, error handling in constructors and destructors, noexcept move, exceptions across module or FFI boundaries, embedded/no-RTTI/no-exceptions builds, or diagnosing terminate calls, lost exceptions, partial mutation, or code that cannot decide between exceptions and error codes.
---

# C++ Exception Strategy And Error Handling

C++ supports exceptions, but whether to use them, how to use them, and how they interact with the rest of the language (constructors, destructors, move semantics, noexcept, FFI boundaries) is a strategy decision, not a default. Unlike languages where exceptions are the only error mechanism, C++ lets you choose exceptions, error codes, `std::expected` (C++23), or a mix — and each choice has correctness, performance, and portability consequences. A destructor that throws terminates the program; a move that throws forces containers to copy; an exception that crosses into C or a no-exceptions build is undefined. The judgment problem is to pick an error strategy deliberately, to honor the exception safety guarantees, and to handle the boundaries where exceptions are not allowed.

Agents tend to throw exceptions for everything (including expected conditions like "file not found"), to forget noexcept on move, or to mix exceptions and error codes inconsistently so callers do not know what to expect. The judgment problem is to reserve exceptions for truly exceptional conditions, to provide at least the basic exception guarantee everywhere, to mark operations noexcept where the language or performance demands it, and to decide the strategy per boundary (a library API, an FFI edge, an embedded build). This skill is about treating error strategy as an architectural decision, not a per-function afterthought.

## Core Rules

### Choose The Error Strategy Per Boundary, And Apply It Consistently

The error strategy is a boundary-level decision, not a per-function one. Within a boundary (a library, a module, a team's code), pick one primary mechanism and apply it consistently:

- **Exceptions**: best for truly exceptional failures that propagate up several call levels and where the caller usually cannot recover locally (allocation failure, invariant violation, unexpected IO error). The advantage is that error propagation is automatic and cannot be forgotten; the cost is the implicit control flow and the requirement that every function be exception-safe.
- **Error codes / `std::expected` (C++23)**: best for expected, recoverable failures that the caller should handle locally (parse error, not-found, validation failure). `std::expected<T, E>` carries either a value or an error and forces the caller to check, combining the explicitness of error codes with value semantics.
- **Mixed**: a common robust strategy is exceptions for unexpected failures and `expected`/error codes for expected ones, with a documented boundary between them.

Do not mix within a boundary: a library that throws for some failures and returns codes for others forces callers to handle both, and they will forget one. Document the strategy in the API.

### Reserve Exceptions For Truly Exceptional Conditions

Throwing an exception for an expected, recoverable condition (a missing config key, a parse error, a "not found" result) is a misuse: exceptions are expensive, they conflate expected flow with errors, and they force every caller to be exception-safe for a condition that should be a return value. Reserve exceptions for conditions where continuing is not meaningful: invariant violations, resource exhaustion, unrecoverable IO failures.

For expected failures, return `std::expected<T, Error>` (C++23), `std::optional<T>`, or an error code. The caller checks and handles locally. This is more explicit and cheaper than exceptions, and it cannot be silently ignored the way a thrown exception propagating up can bypass intermediate handlers.

### Provide At Least The Basic Guarantee Everywhere; Strong Where Affordable

Every function must offer at least the **basic exception guarantee**: if it throws, the program remains in a valid state, no resources leak, and no invariants are broken. This is the floor, not an aspiration. The **strong guarantee** (commit-or-rollback: if it throws, the state is unchanged) is desirable for operations that can afford it.

- Achieve the basic guarantee with RAII: members that clean up in their destructors guarantee no leaks even on a throw.
- Achieve the strong guarantee with copy-and-swap or prepare-then-commit: do all may-throw work in temporaries, then commit with non-throwing operations (pointer swaps, noexcept moves).
- The **nothrow guarantee** is required for destructors and for operations the language requires not to throw (deallocation, noexcept functions).

Code that does not meet the basic guarantee (leaks or corrupts state on throw) is incorrect, even if the throw is rare.

### Mark Destructors And Deallocation noexcept; Mark Move noexcept Where Possible

Destructors must not throw — a throw during stack unwinding calls `std::terminate`. Mark destructors `noexcept` (they are by default in C++11+) and ensure their bodies cannot throw (catch and log internally). Move constructors and move assignment should be `noexcept` when they cannot throw (the common case for pointer/resource transfers), because containers check `is_nothrow_move_constructible` and will only use move during reallocation if it is noexcept — otherwise they copy to preserve the strong guarantee, which defeats the point of move.

`swap` should also be `noexcept` (it is the foundation of copy-and-swap and of the strong guarantee). Mark functions `noexcept` when you can guarantee they will not throw; this enables optimizations and documents the contract. Do not lie with `noexcept` (a function marked noexcept that throws calls `std::terminate`).

### Throw By Value, Catch By Reference

Throw exceptions by value (throw `MyException("msg")`, not `throw new MyException(...)`) to avoid leaks and to let the exception object live on the exception stack. Catch by reference (`catch (const std::exception& e)`) to avoid slicing (catching by value copies the base part and drops the derived type) and to enable rethrow with `throw;`.

Use `throw;` (not `throw e;`) to rethrow the current exception, preserving its original type and the throw site. `throw e;` slices and resets the type.

### Design A Exception Hierarchy Around Catch Needs, Not Categories

If you use exceptions, design the hierarchy around how callers catch, not around a taxonomy of categories. A common base (`std::runtime_error`, or a project `AppException`) lets callers catch broadly; specific derived types let them catch narrowly. Do not over-engineer a deep hierarchy — most callers catch the base or one or two specific types. Provide `what()` messages with actionable context.

Do not catch and swallow exceptions silently (`catch (...) {}`) except at a deliberate boundary (a thread's top level, an FFI edge) where you must not let the exception propagate. Log and rethrow, or translate to the boundary's error mechanism.

### Handle The Boundaries Where Exceptions Are Forbidden

Exceptions cannot cross certain boundaries, and the code must translate them:

- **Destructors and noexcept functions**: a throw calls `std::terminate`. Catch internally.
- **C / FFI boundaries**: an exception propagating into C (or a no-exceptions build) is undefined behavior. Catch at the boundary and translate to an error code.
- **Thread entry points**: an uncaught exception in a thread calls `std::terminate`. Catch at the thread's top level and translate (store in a promise, log, or report).
- **Embedded / no-exceptions builds** (`-fno-exceptions`): `throw` and `try`/`catch` are not available; the code must use error codes/`expected` exclusively, and the standard library's exception-throwing operations abort. Many embedded and game builds disable exceptions; design for this if it is a target.

### In C++23, Consider std::expected For Expected Failures

`std::expected<T, E>` carries either a value of type T or an error of type E, forcing the caller to check before accessing the value. It is the modern C++ answer to error codes with value semantics, and it is the right tool for expected, recoverable failures where exceptions would be overkill. Use it where you previously returned a pair or an error code plus out-parameter. Combine with exceptions for unexpected failures, with a documented boundary.

## Common Traps

### Throwing For An Expected Condition

`throw NotFound` for a lookup miss forces every caller to be exception-safe for a normal case. Return `std::optional` or `std::expected` for expected failures.

### Destructor That Throws

A destructor that calls a may-throw operation without a try/catch calls `std::terminate` during unwinding. Wrap destructor bodies in try/catch, or move throwing cleanup to an explicit method.

### Move Not Marked noexcept

A move constructor without `noexcept` forces `std::vector` to copy during reallocation, defeating the point of move. Mark move `noexcept` when it cannot throw.

### Catching By Value (Slicing)

`catch (MyException e)` slices the exception to the static type, losing derived info. Catch by `const` reference.

### Mixing Exceptions And Error Codes Inconsistently

A library that throws for some failures and returns codes for others forces callers to handle both. Pick one primary strategy per boundary and document it.

### catch(...) That Swallows Silently

`catch (...) {}` at a non-boundary location hides errors. Log and rethrow, or translate to the boundary's mechanism.

### Exception Crossing Into C / A no-exceptions Build

An exception propagating into a C function (via FFI) or a `-fno-exceptions` build is undefined behavior. Catch at the boundary and translate to an error code.

### noexcept That Lies

Marking a function `noexcept` that can throw calls `std::terminate` if it throws. Only mark noexcept if you can guarantee no throw, including from called functions.

## Self-Check

- [ ] The error strategy is chosen per boundary (exceptions for unexpected failures, `std::expected`/error codes for expected ones) and applied consistently within each boundary, with the strategy documented.
- [ ] Exceptions are reserved for truly exceptional conditions; expected, recoverable failures return `std::optional` or `std::expected` and force the caller to check.
- [ ] Every function provides at least the basic exception guarantee (no leaks, valid state on throw) via RAII; operations that can afford it provide the strong guarantee via copy-and-swap or prepare-then-commit.
- [ ] Destructors and deallocation are `noexcept` and their bodies cannot throw; move constructors/assignment and swap are `noexcept` where possible so containers can move.
- [ ] Exceptions are thrown by value and caught by `const` reference; rethrow uses `throw;` to preserve type.
- [ ] The exception hierarchy is designed around catch needs (a broad base plus specific derived types) and `what()` messages carry actionable context; `catch(...)` is used only at deliberate boundaries and translates the error.
- [ ] Exceptions are caught and translated at every boundary they cannot cross: destructors, noexcept functions, C/FFI edges, thread entry points, and no-exceptions builds.
- [ ] `noexcept` is only applied where no throw is guaranteed, and `std::expected` is used for expected failures in C++23 codebases.
