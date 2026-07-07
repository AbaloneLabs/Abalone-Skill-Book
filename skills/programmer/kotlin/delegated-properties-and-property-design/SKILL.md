---
name: kotlin_delegated_properties_and_property_design.md
description: Use when the agent is designing Kotlin properties with delegated properties (by lazy, by observable, by Delegates, custom property delegates), lateinit vs nullable vs lazy initialization, const val vs val, backing fields and custom getters/setters, @JvmField, computed properties vs methods, observable/vetoable properties, or is diagnosing "lateinit property has not been initialized", delegate not working across instances, lazy thread-safety mode surprises, custom getter infinite recursion, or property that should be a function. Covers property initialization strategies, delegated property semantics and lifecycle, lazy threading modes, custom delegates, and the pitfalls of lateinit, computed properties, and field-vs-method design.
---

# Delegated Properties And Property Design In Kotlin

Kotlin elevates properties to a first-class design surface: `val`/`var` with backing fields, custom getters/setters, `lateinit`, `by lazy`, `by Delegates.observable`, and fully custom property delegates. This power is easy to misuse because the distinctions between the options are semantic, not syntactic, and the wrong choice produces initialization crashes, thread-safety surprises, infinite recursion, or properties that are really methods in disguise. `lateinit var` defers initialization but crashes with a clear error if read before set, and works only for non-primitive `var`s; `by lazy` initializes on first read with a chosen thread-safety mode; a custom getter with no backing field computes on every access; a `const val` must be a compile-time constant. The judgment problem is to choose the initialization strategy by the lifecycle and thread-safety needs, to know when a property should be a function, to write custom delegates that manage their own per-property state correctly, and to avoid the recursion and lifecycle traps.

Agents reach for `lateinit` to avoid nullability, then read it before initialization and crash; or they use `by lazy` with the default `SYNCHRONIZED` mode in a single-threaded context (unnecessary overhead) or `NONE` in a multithreaded one (race); or they write a custom getter `val x get() = x` that recurses infinitely. The remedy is to match the strategy to the need, to prefer nullable or `lazy` over `lateinit` where the lifecycle is uncertain, and to verify delegate state isolation and getter/setter correctness.

## Core Rules

### Choose The Initialization Strategy By Lifecycle And Thread-Safety

The options for a property whose value is not known at construction: `lateinit var` (deferred set, non-null, crashes if read before set, `var` only, non-primitive, no custom getter); `by lazy` (computed on first read, cached, `val` only, thread-safety mode selectable); nullable `var` with null until set (explicit null-handling at each read); or initialize at construction with a parameter/default. Choose by when the value becomes known and whether the property is `val` or `var`:

- `lateinit` for a `var` set later in a known lifecycle (a view-model injected after creation, a field set in `onCreate`/`@Setup`) — check `::prop.isInitialized` before reading if uncertain.
- `by lazy` for a `val` computed once on first use (an expensive derived value, a resource opened on demand).
- Nullable `var` where the value may genuinely be absent or set/unset repeatedly.
- Avoid `lateinit` for primitives (use `Lazy` or a nullable boxed value) and for `val` (impossible).

### Understand by lazy Thread-Safety Modes

`by lazy` defaults to `LazyThreadSafetyMode.SYNCHRONIZED` (double-checked locking, safe for concurrent first-access). `PUBLICATION` allows multiple threads to compute the value, with the first result winning (safe but may run the initializer more than once). `NONE` has no synchronization (fast, but unsafe if first-access can be concurrent). Choose `SYNCHRONIZED` when the initializer must run once and may be accessed concurrently (the safe default); `NONE` when access is provably single-threaded (a UI-bound property) to remove the lock overhead; `PUBLICATION` rarely. The default is right for most cases; override deliberately with a reason.

- `SYNCHRONIZED` (default): safe, initializer runs once. Keep it unless you have measured the overhead.
- `NONE`: only when concurrent first-access is impossible; removes the double-checked lock.
- `PUBLICATION`: when you accept multiple initializations and take the first result.

### Use lateinit Deliberately, Check isInitialized Where Uncertain

`lateinit var x: Service` defers initialization; reading before setting throws `UninitializedPropertyAccessException`. It is appropriate when the property is always set before first read in the normal lifecycle (a Spring/JUnit-injected field, an Android field set in `onCreate`). If the lifecycle does not guarantee set-before-read, prefer nullable (and handle null) or `lazy`. To check safely, use `::x.isInitialized` (reflection-light, compile-checked). `lateinit` is `var`-only, non-primitive, and cannot have a custom getter; do not fight these constraints.

- `lateinit` for guaranteed-set-before-read fields (DI, lifecycle setup).
- `if (::x.isInitialized) x else fallback` where the lifecycle is uncertain.
- Nullable or `lazy` where the value may be absent or computed.

### Know When A Property Should Be A Function

A property with a custom getter that does non-trivial work (I/O, allocation, computation with side effects, O(n) work) is often better as a function. The convention: a property should be cheap, deterministic, and side-effect-free; anything expensive, non-deterministic, or mutating belongs in a function (so callers know it does work). `val items: List<T> get() = repo.load()` reads like a cheap field access but does I/O — make it `fun loadItems(): List<T>`. A computed property that derives from other properties cheaply (`val fullName get() = "$first $last"`) is fine as a property.

- Property: cheap, pure, no side effects (`fullName`, `isEmpty`).
- Function: expensive, I/O, mutating, or non-deterministic (`load()`, `save()`, `compute()`).
- A getter that allocates or does I/O on every access is a hidden cost; callers assume properties are cheap.

### Write Custom Delegates With Per-Property State

A custom property delegate (`by myDelegate()`) implements `getValue`/`setValue` (for `var`) operator functions and manages the value. The delegate instance is shared per property-delegation site, so a delegate that holds state must isolate it per property (typically by keying on the `thisRef` and property metadata, or by being instantiated per use). `Delegates.observable(initial) { _, old, new -> ... }` and `vetoable` are built-in delegates for change observation/validation. When writing a custom delegate, ensure thread-safety if accessed concurrently, and avoid capturing per-instance state in a way that leaks across instances.

- `observable`/`vetoable` for change tracking/validation on a `var`.
- A custom delegate's state must be isolated per property; a single shared instance with mutable state across properties is a bug.
- Implement `getValue` (and `setValue` for `var`) with the right `thisRef` and `KProperty` types.

### Avoid Getter/Setters Recursion And Backing-Field Confusion

A custom getter `val x: Int get() = x` recurses infinitely (it calls itself, not a backing field). To reference the backing field from a getter/setter, use `field` (available only in the getter/setter of a property with a backing field). A property with a custom getter and no `field` has no backing field (computed). A `var` setter `set(value) { x = value }` recurses; use `field = value`. These recursions are easy to write and crash at runtime; use `field` explicitly.

- Use `field` inside a getter/setter to refer to the backing field.
- A custom getter without `field` is computed (no backing storage).
- `@JvmField` exposes the field directly (no getter/setter) for Java interop/performance; use deliberately.

## Common Traps

### lateinit Read Before Initialization

`lateinit var x; fun use() { x.foo() }` crashes if `x` was not set. Check `isInitialized` or use nullable/lazy.

### lazy NONE In A Concurrent Context

`by lazy(LazyThreadSafetyMode.NONE) { ... }` races if first-access is concurrent. Use the default `SYNCHRONIZED`.

### Custom Getter Recursion

`val x get() = x` loops forever. Use `field` to refer to the backing field.

### Expensive Work In A Property Getter

`val data get() = loadFromNetwork()` looks cheap but does I/O per access. Make it a function.

### lateinit On A Primitive

`lateinit var count: Int` does not compile (primitives only). Use `Lazy` or a nullable boxed value.

### Shared Mutable State In A Custom Delegate

A delegate instance reused across properties with mutable state leaks between them. Isolate state per property.

### const val Used For A Non-Constant

`const val x = compute()` does not compile (`const` requires a compile-time constant). Use `val` or a top-level computed value.

### @JvmField Breaking A Custom Getter

`@JvmField` exposes the raw field, bypassing a custom getter/setter, so Java interop skips your logic. Use deliberately, not by habit.

## Self-Check

- [ ] The initialization strategy matches the lifecycle: `lateinit` for guaranteed-set-before-read `var`s (with `isInitialized` checks where uncertain), `by lazy` for once-computed `val`s, nullable for genuinely-absent values, and construction-time otherwise.
- [ ] `by lazy` thread-safety mode is chosen deliberately (`SYNCHRONIZED` default for concurrent safety, `NONE` only where single-threaded is provable), and `lateinit` is not used for primitives or `val`s.
- [ ] Properties are cheap, pure, and side-effect-free; expensive, I/O, mutating, or non-deterministic operations are functions, not property getters.
- [ ] Custom getters/setters use `field` to refer to the backing field (no infinite recursion), and `@JvmField` is applied deliberately where raw-field access is intended.
- [ ] Custom delegates isolate per-property state, implement the right operator functions, and are thread-safe where accessed concurrently; `observable`/`vetoable` are used for change tracking.
- [ ] `const val` is used only for compile-time constants; `val` for computed top-level/instance values.
- [ ] No `lateinit` is read before its guaranteed set, and no `lazy(NONE)` is accessed concurrently.
- [ ] The property design has been considered under initialization order, concurrency, Java interop, and readability, and remains correct and clear.
