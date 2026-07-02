---
name: kotlin_null_safety_and_interop.md
description: Use when the agent is writing or reviewing Kotlin code involving nullable and non-null types, platform types from Java interop, the !! operator, safe call and elvis operator patterns, let-based null handling, Java and Kotlin mixed codebases, JSR-305 nullability annotations, migrating Java to Kotlin, or any boundary where Kotlin's null safety is weakened or defeated by interop, inference, or forceful unwrapping.
---

# Null Safety And Interoperability In Kotlin

Kotlin's headline feature is null safety: the type system distinguishes `T` (never null) from `T?` (maybe null), and the compiler refuses to dereference a `T?` without a null check. At compile time, within pure Kotlin, this eliminates the null-pointer exceptions that dominate Java crash logs. The catch — and it is a large catch — is that most real Kotlin code is not pure Kotlin. It calls Java libraries, receives values from Java frameworks, and stores data in systems whose nullability Kotlin cannot see. At those boundaries, Kotlin introduces platform types, whose nullability is unknown, and the compiler's guarantees silently degrade. The judgment problem is not "use null safety" but "preserve null safety across every Java boundary, and never let a platform type or a `!!` smuggle a null into code that believes it cannot be null."

Agents frequently treat Kotlin null safety as absolute, then undermine it with `!!` to satisfy the compiler, or accept Java return values as non-null without checking, or write `let`-chains that obscure intent. The harm is that the NPE returns, but now it is deferred to runtime and dressed in Kotlin syntax. A `!!` on a platform type that is actually null throws `NullPointerException` at runtime, in code the type system said was safe. A Java method returning null, assigned to a Kotlin non-null variable via platform-type inference, throws at first use. The skill is to treat platform types as untrusted, to eliminate `!!` except where an invariant is genuinely proven, to use `let`/`?:`/explicit checks to express intent, and to harden the Java/Kotlin boundary with annotations and wrappers rather than hoping.

## Core Rules

### Treat Platform Types As Untrusted At Every Java Boundary

When Kotlin calls a Java method, the return type is a platform type (`T!`), meaning Kotlin does not know whether it is nullable. The compiler will let you assign it to either `T` or `T?`, and if you assign it to `T` and the value is actually null, you get an NPE — at the point of use, not at assignment, which makes it hard to trace.

- Assign Java return values to explicitly nullable types (`val name: String? = javaObject.getName()`) when there is any doubt about nullability, and handle the null. This is the safe default.
- Only assign a platform type to a non-null type when the Java source is annotated (`@NotNull`, `@NonNull`, JSR-305 `@ParametersAreNonnullByDefault`) or you have read the Java source and confirmed it cannot return null. Document the assumption.
- Be especially careful with Java collections, maps, and generic types: their element nullability is invisible to Kotlin, and a `Map<String, String>` from Java may have null keys or values even though Kotlin's `Map<String, String>` forbids them.

The discipline: every value crossing from Java into Kotlin is suspect until its nullability is established. Default to nullable, narrow to non-null with evidence.

### Eliminate !! Except For Proven Invariants

The `!!` operator throws `NullPointerException` if the value is null. It is the escape hatch that defeats null safety, and like all such hatches it should be rare and justified.

- Acceptable `!!`: a value initialized lazily after a known point and accessed only after that point, where the lifecycle guarantees non-null (and even then, `lateinit` or a nullable with initialization is often clearer); a value from a source you have proven non-null but the compiler cannot see.
- Unacceptable `!!`: silencing the compiler when you are not sure; unwrapping the result of a lookup, parse, or interop call that can genuinely be null; force-unwrapping a value passed from code you do not control.
- Every `!!` should be defensible with a one-sentence proof of why the value is non-null at that point. If you cannot state the proof, replace `!!` with a null check and handle the case.

Strong choice: `val parsed = parse(input) ?: return Result.error(...)`. Weak choice: `val parsed = parse(input)!!` "because the input is always valid" (it is not, and the NPE is in production).

### Express Null Handling With let, ?:, And Explicit Checks By Intent

Kotlin offers several idioms for handling nulls; choose the one that expresses the intent most clearly rather than reaching for `!!` or `let` reflexively.

- **`?:` (elvis)** provides a default or early return: `val name = input.name ?: "unknown"` or `val value = map[key] ?: return`. Use for "use this default if null" or "stop if null."
- **`?.let { }`** runs a block only if non-null: `user?.let { sendEmail(it) }`. Use for "do something with the value if present"; do not use it as a general substitute for `if null return`, where `?: return` is clearer.
- **`if (x != null)` / `guard-style`** narrows `x` to non-null within the branch, with full readability for complex logic. Prefer this over deeply nested `?.let` chains.
- **`requireNotNull(x)` / `checkNotNull(x)`** throw `IllegalArgumentException`/`IllegalStateException` with a message — appropriate for argument validation and invariant checks where a null indicates a programming error, not normal control flow.

Match the idiom to the meaning: elvis for defaults/early-exit, let for "if present do this," explicit checks for complex branches, require/check for invariant violations.

### Annotate Java Code For Nullability To Preserve Safety Across The Boundary

If you own the Java code that Kotlin calls, annotate its nullability so Kotlin sees real types instead of platform types. This is the highest-leverage way to keep null safety across a mixed codebase.

- Use JSR-305 annotations (`@ParametersAreNonnullByDefault` on a package, `@Nonnull`/`@Nullable` on individual declarations) or the JetBrains annotations (`@NotNull`/`@Nullable` from `org.jetbrains.annotations`) so Kotlin treats the Java API as properly typed.
- Apply a package-level default (`@ParametersAreNonnullByDefault`) and annotate the exceptions, rather than annotating every parameter; this scales better.
- For third-party Java libraries without annotations, write thin Kotlin wrapper functions that accept nullable types, validate, and return non-null Kotlin types — a "null-safety membrane" at the boundary.

### Use lateinit And lazy Deliberately For Initialization

Some values cannot be non-null at construction but are non-null after initialization. Kotlin offers `lateinit` and `lazy` for these cases; each has tradeoffs.

- **`lateinit var`** is a non-null property initialized after construction (e.g., in a lifecycle callback, injected by a framework). Accessing it before initialization throws `UninitializedPropertyAccessException`. Use for framework-injected or lifecycle-initialized non-null mutable properties; check `::prop.isInitialized` if access before init is possible.
- **`by lazy { }`** initializes the property on first access, thread-safely by default. Use for non-null values whose initialization is expensive or only needed sometimes; the property is `val`, initialized once.
- Do not use `lateinit` for properties that may legitimately stay unset (use a nullable and handle null) or for `val` (use `lazy`); do not use `lazy` for properties that must be initialized at a specific point.

### Handle Nullable Collections And Generics Carefully

Kotlin's nullable generics (`List<String?>` vs `List<String>?`) distinguish "a non-null list of nullable strings" from "a nullable list of non-null strings." Java collections erase this distinction.

- A `java.util.List<String>` from Java may contain null elements; in Kotlin it is a platform type that can be assigned to `List<String>` (forbidding nulls) without complaint, then throw when a null element is accessed as non-null. Treat Java collections as `List<String?>` at the boundary.
- Nested nullability (`Map<String, List<String?>?>`) is expressive but can become unreadable; if a type signature needs three levels of `?`, consider a wrapper type or a data class that encapsulates the null-handling.
- When converting Java collections to Kotlin, filter or map nulls explicitly (`filterNotNull()`) before treating the result as non-null-element.

## Common Traps

### Assigning A Platform Type To Non-Null And Crashing Later

`val name: String = javaObject.getName()` compiles, and throws `NullPointerException` at first use of `name` if `getName()` returned null. The crash is far from the cause. Assign platform types to nullable and handle the null, or annotate the Java source.

### !! As The Default Response To A Compiler Error

`val x = maybeX!!` makes the compiler happy and throws at runtime. The compiler error exists because the value can be null; `!!` does not make it non-null, it converts a compile-time guarantee into a runtime crash. Use `?:`, `let`, or an explicit check.

### Java Collection Nullability Erased

`val items: List<Item> = javaService.getItems()` where the Java list contains nulls throws when iterating as non-null. Treat Java collections as `List<Item?>` and `filterNotNull()` at the boundary.

### lateinit Accessed Before Initialization

`lateinit var view: View` accessed before `onViewCreated` (or wherever it is initialized) throws `UninitializedPropertyAccessException`. Use `::view.isInitialized` to guard, or a nullable with explicit initialization tracking, when access before init is possible.

### Overusing ?.let For Simple Early Returns

`user?.let { ... long block ... } ?: return` is less readable than `val user = user ?: return; ... ` for the common "stop if null" case. Use `let` for "do something with the value," not as a substitute for a guard.

### Nullable Booleans In Conditions

`if (flag)` where `flag: Boolean?` does not compile; the temptation is `if (flag == true)`, which treats null as false silently. Decide whether null should be false, true, or a distinct case, and handle it explicitly rather than defaulting via `== true`.

### Trusting Java Annotations That Are Wrong

A Java method annotated `@Nonnull` that can actually return null (a bug in the annotation or the code) defeats Kotlin's check and throws at use. Annotated Java code should still be treated with mild suspicion at boundaries where the cost of an NPE is high; wrappers and tests harden the membrane.

### Mixed Codebase With Inconsistent Null Defaults

A codebase where some Java packages are `@ParametersAreNonnullByDefault` and others are not, or where Kotlin wrappers exist for some Java APIs but not others, has inconsistent null-safety guarantees. Standardize the convention (package defaults, wrapper patterns) so the boundary behavior is predictable.

## Self-Check

- [ ] Every value crossing from Java into Kotlin is treated as a platform type and assigned to an explicitly nullable type unless its non-nullability is annotated or proven, and the assumption is documented.
- [ ] No `!!` exists without a one-sentence proof that the value is non-null at that point, and no `!!` unwraps a lookup, parse, interop result, or uncontrolled input.
- [ ] Null handling uses the idiom that matches intent (`?:` for defaults/early-exit, `let` for "if present do this," explicit checks for complex logic, `requireNotNull`/`checkNotNull` for invariant violations).
- [ ] Java code owned by the project is annotated for nullability (JSR-305 package defaults plus per-declaration exceptions), and third-party Java APIs are wrapped in a null-safety membrane at the boundary.
- [ ] `lateinit` is used only for framework/lifecycle-initialized non-null mutable properties with `isInitialized` guards where early access is possible, and `lazy` for non-null values with deferred one-time initialization.
- [ ] Java collections are treated as having nullable elements at the boundary (`List<T?>`) and `filterNotNull()` is applied before non-null-element use.
- [ ] No nullable-boolean or nullable-comparison silently treats null as a default without a deliberate decision documented.
- [ ] The null-safety convention is consistent across the mixed codebase (package defaults, wrapper patterns, annotation usage), so boundary behavior is predictable.
- [ ] No `?.let` chain substitutes for a clear guard where `?: return` or an explicit `if` would be more readable.
- [ ] The code has been considered for the case where a Java-annotated-non-null value is actually null (annotation bug), and high-cost boundaries are hardened with wrappers or tests.
