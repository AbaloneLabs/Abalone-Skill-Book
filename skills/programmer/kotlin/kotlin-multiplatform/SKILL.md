---
name: kotlin_multiplatform.md
description: Use when the agent is building a Kotlin Multiplatform library or application, sharing code across iOS Android JVM JS, using expect/actual declarations, configuring Gradle targets, managing platform-specific dependencies, handling interop with Swift Objective-C, or diagnosing compatibility, serialization, and build issues in Kotlin Multiplatform projects.
---

# Kotlin Multiplatform

Kotlin Multiplatform (KMP) promises write-once-share-everywhere, and that promise hides a set of tradeoffs that dominate real projects. The shared module looks like ordinary Kotlin, but every line of it must compile and behave correctly across JVM, iOS, JS, and native targets whose standard libraries, concurrency models, and interop constraints differ. The judgment problem is not "how do I add an iOS target" but deciding what belongs in common code (where it must be platform-agnostic and pay the cost of `expect`/`actual` boundaries) versus what belongs in platform code (where it is correct and idiomatic but not shared). Get this boundary wrong and you either ship a shared module full of leaky abstractions that behave differently per platform, or you duplicate so much that the sharing provides no value.

The recurring failure mode is a developer who pushes as much as possible into common code to maximize sharing, then discovers that the common code depends on platform behavior that does not exist everywhere (file I/O, threading, reflection, time zones), or that the iOS interop produces an API no Swift developer would want to call. The opposite failure is sharing almost nothing, leaving the common module as a thin bag of data classes. KMP is a set of decisions about the common/platform boundary, interop ergonomics, and dependency compatibility, and the right answer is rarely "share everything" or "share nothing."

## Core Rules

### Decide the common/platform boundary by behavior, not by syntax

Code belongs in common only if its behavior is genuinely platform-independent *and* its dependencies are available in the commonstdlib. Rules:

- Pure logic, data models, algorithms, and serialization belong in common.
- Anything touching the file system, network (beyond what a multiplatform library abstracts), threads, UI, or platform-specific APIs belongs in platform sources or behind an `expect` declaration.
- When behavior differs by platform, use `expect`/`actual` to declare a common contract and provide platform implementations, rather than cramming platform checks into common code.

The test: if you would need `when (platform)` in common code, that code belongs behind an `expect`.

### Design `expect`/`actual` as a stable contract

An `expect fun`/`expect class` declares an API that each platform `actual` must implement. This is a contract like any API. Rules:

- Keep `expect` signatures minimal and stable; changing them requires updating every platform's `actual`.
- Prefer `expect` functions and properties over `expect class`; an `expect class` is a heavier commitment and harder to evolve.
- Do not use `expect`/`actual` to expose platform-specific concepts that have no common meaning; if the concepts differ, keep them in platform code.

### Account for iOS interop friction

Kotlin/Native compiles to a framework that Objective-C and Swift consume, and the interop layer imposes constraints:

- Kotlin `suspend` functions appear as completion handlers in Swift, not async/await (though newer versions bridge to Swift concurrency; verify your Kotlin version).
- Kotlin `object` and companion functions expose as class methods; Swift naming conventions differ.
- Kotlin `enum` with associated values maps awkwardly to Swift; sealed classes map to Objective-C class hierarchies with caveats.
- Names are mangled to Objective-C conventions; long names and generics are truncated or suffixed.

Design the public API of an iOS-facing module with interop in mind. Expose Swift-friendly names (`@ObjCName`), avoid exotic Kotlin features in the public surface, and consume the framework from Swift during development to catch ergonomic problems early.

### Choose multiplatform dependencies deliberately

A common dependency must itself be multiplatform. Rules:

- Verify each library publishes the targets you need (`kotlinx.coroutines`, `kotlinx.serialization`, `Ktor`, `SQLDelight` are common choices).
- A JVM-only or iOS-only library cannot be used in common code; gate it behind an `expect` or use it only in platform sources.
- Dependency version conflicts across targets cause hard-to-diagnose build errors; align versions and use version catalogs.

### Concurrency model differs across targets

Kotlin/Native historically had a strict memory model (immutable shared state, frozen objects) that the new memory model (default in Kotlin 1.9+) relaxed. Rules:

- On the new memory model, sharing mutable state across threads is allowed but still requires synchronization; coroutines and `kotlinx.coroutines` are the recommended abstraction.
- Do not assume JVM threading semantics in common code; use `Dispatchers` and coroutine abstractions rather than raw threads.
- Background work on iOS must still respect the platform's main-thread rules for UI; bridge via coroutines, not `dispatch_async`.

### Serialization and data formats are the lingua franca

`kotlinx.serialization` is the standard for cross-platform data exchange in KMP. Rules:

- Annotate shared data classes `@Serializable`; they work across targets.
- Be aware that reflection-based serialization is not available on all targets; `kotlinx.serialization` generates code at compile time, which is why it works.
- Define DTOs in common; map to platform-specific models in platform code if the platform needs different shapes.

### Gradle configuration is part of the API surface

The Gradle module defines targets, source sets, and dependencies, and it is where most "why does this not build for iOS" issues live. Rules:

- Configure each target's specifics (min iOS version, JVM target, JS module kind) deliberately; defaults may not match your needs.
- Use a version catalog to keep dependency versions consistent across targets.
- Test on every target in CI; a common module that compiles for JVM but not iOS is not actually shared.

### Test on every target, not just the convenient one

Common code that passes JVM tests can fail on iOS due to different standard library behavior, different numeric types, or different threading. Rules:

- Write `commonTest` tests that run on every target.
- Add target-specific tests for platform behavior.
- CI must build and test all targets; "it works on Android" is not sufficient for a shared module.

## Common Traps

### Pushing platform-specific behavior into common code

File I/O, threading, or reflection in common code compiles only if a multiplatform library abstracts it; otherwise it fails on some target. Gate behind `expect` or move to platform sources.

### iOS interop that produces an unidiomatic Swift API

Kotlin features that are fine in common (complex generics, `suspend` functions, exotic enums) can produce an Objective-C framework that is painful to call from Swift. Design the iOS-facing surface for Swift ergonomics.

### Using a JVM-only library in common code

A library that is not multiplatform cannot be referenced from common code; the build fails for other targets. Gate it behind `expect` or use it only in JVM sources.

### Changing an `expect` signature without updating all `actual`s

An `expect` change requires updating every platform's `actual`; missing one breaks the build for that target. Treat `expect` as a versioned contract.

### Assuming JVM threading semantics in common code

Common code runs on JVM, Native, and JS with different threading models. Use coroutine abstractions; do not assume threads or locks are available.

### Skipping CI builds for some targets

A common module that builds for JVM but not iOS is not shared. CI must build and test every target on every change.

### Over-sharing to maximize common code

Sharing code that behaves differently per platform, or that exists only to share, adds abstraction cost without value. Share what is genuinely common; keep the rest platform-specific.

## Self-Check

- Is the common/platform boundary drawn by genuine behavioral independence, with platform-specific behavior behind `expect`/`actual` rather than `when (platform)` in common code?
- Are `expect` declarations minimal and stable, with every platform's `actual` updated when they change, and are `expect class` usages justified over `expect fun`?
- Is the iOS-facing public API designed for Swift ergonomics, with `suspend`/generic/exotic Kotlin features avoided or bridged where they produce unidiomatic frameworks?
- Are all common dependencies verified multiplatform across the targets you need, with versions aligned via a catalog?
- Does common code avoid assuming JVM threading, using coroutine abstractions and respecting platform main-thread rules?
- Are shared data models `@Serializable` with `kotlinx.serialization`, mapped to platform-specific shapes only where needed?
- Does Gradle configuration specify each target's specifics deliberately, and does CI build and test every target on every change?
- Is the amount of sharing justified by genuine commonality, rather than over-sharing code that behaves differently per platform?
