---
name: reflection_and_annotations.md
description: Use when the agent is using Java reflection (Class, Method, Field, Constructor, getDeclaredMethods, setAccessible, Method.invoke, Field.get/set, dynamic proxies), processing or defining annotations (retention, target, @Repeatable, annotation processing), building frameworks that discover types at runtime (classpath scanning, service loaders), serializing/deserializing via reflection (Jackson, Gson, JAXB), implementing InvocationHandler, or diagnosing performance overhead of reflection, IllegalAccessException/setAccessible breaking under modules, annotation retention mismatches (CLASS vs RUNTIME), reflective creation failures, or fragile classpath scanning. Covers reflection cost, module-system access, annotation retention, and the tradeoffs of runtime introspection.
---

# Reflection And Annotations

Reflection is Java's mechanism for runtime introspection and invocation: inspecting classes, methods, and fields at runtime, invoking methods dynamically, and accessing fields that the compiler would normally forbid. It is the foundation of every Java framework — serialization (Jackson, Gson), dependency injection (Spring, CDI), ORM (Hibernate, JPA), testing (JUnit), and mocking (Mockito) all use reflection to discover and manipulate types the framework did not write. Annotations are the metadata layer that makes reflection useful: a framework scans for `@Inject`, `@Entity`, `@Test`, or `@RestController` and acts on the annotated elements. Together they enable the declarative, convention-over-configuration style that defines modern Java.

The judgment problem is not "how do I call `getDeclaredMethods()`" but "is reflection the right tool here, what does it cost in performance and safety, how do I handle the module system's access restrictions that now block `setAccessible`, and have I matched annotation retention to when the annotation is actually read." Agents tend to overuse reflection (reaching for it where a plain interface or generics would be clearer and faster), ignore its performance cost (reflection is 10-100x slower than direct invocation), break under JPMS modules (where `setAccessible` on a non-exported package throws `InaccessibleObjectException`), and use the wrong annotation retention (`SOURCE` or `CLASS` when the framework reads at runtime, so the annotation is invisible). The cost is slow startup, fragile frameworks that break on JDK upgrades, and annotations that silently do nothing.

## Core Rules

### Treat Reflection As A Framework Tool, Not An Application Tool

Reflection trades compile-time safety for runtime flexibility. That trade is worth it for a framework that must handle types it did not write (Jackson serializing any POJO, Spring injecting any bean) — the framework cannot know the types at compile time, so reflection is the only option. It is almost never worth it in application code, where you control the types and can use interfaces, generics, or plain method calls that the compiler can verify.

The rule: if you can solve the problem with an interface, a lambda, a generic method, or a factory, do that instead of reflection. Reflection should be a last resort in application code, used only when the type is genuinely unknown until runtime (loading a plugin, reading a config-specified class). The costs of unnecessary reflection: lost compile-time checking (a typo in a method name is a runtime `NoSuchMethodException` instead of a compile error), worse performance (reflective invocation is far slower than direct), worse IDE support (refactoring does not update reflective string names), and worse debugging (the call site is `Method.invoke`, not the actual method).

When you do use reflection, isolate it behind an interface. The framework internals can be reflective; the public API should be typed. This confines the fragility to a small surface and lets the rest of the code compile-check normally.

### Understand And Minimize The Performance Cost

Reflective operations are significantly slower than their direct equivalents: `Method.invoke` is typically 10-50x slower than a direct call (the JVM must check access on each invocation, box primitive arguments, and handle the invocation generically), and `Field.get`/`Field.set` carry similar overhead. For a single call this is negligible; in a hot loop or a framework that reflects over thousands of fields per object (a serializer), the cost dominates.

Mitigations, in order of preference: (1) cache the `Method`/`Field` objects — looking them up (`getDeclaredMethod`) is expensive, but once cached, `invoke` is cheaper (though still slower than direct). (2) Use `MethodHandle` (Java 7+) — after warming up, `MethodHandle.invokeExact` can be as fast as a direct call because the JVM can inline it. (3) Use `VarHandle` (Java 9+) for field access — similar performance benefits to `MethodHandle`. (4) Generate bytecode at runtime (ASM, ByteBuddy) — the fastest option, used by high-performance serializers, but the most complex. (5) For annotation processing, use annotation processors at compile time (see below) to generate typed code instead of reflecting at runtime.

Always measure before optimizing reflection. The overhead matters in hot paths and frameworks; it is irrelevant for one-time startup reflection. But do not put reflective calls in inner loops without awareness of the cost.

### Handle The Module System's Access Restrictions (JPMS)

Since Java 9, the module system (`module-info.java`) restricts reflective access: a module's non-exported packages are not reflectively accessible to other modules, even with `setAccessible(true)`. Calling `setAccessible(true)` on a `Field` or `Method` in a non-exported package of another module throws `InaccessibleObjectException`. This breaks a lot of legacy reflection-heavy code (frameworks that reflect into JDK internals, libraries that access private fields of third-party classes).

The long-term fix is to respect modules: frameworks should only reflect into exported packages or their own internals, and libraries should export the packages frameworks need. The short-term workaround is `--add-opens` JVM flags, which force-open a package to reflection — but this is a migration aid, not a permanent solution, and it is increasingly refused by libraries that have migrated to module-safe alternatives.

The practical guidance: if you are writing a framework that reflects into user classes, require those classes to be in packages you can access (the user's own modules, or the unnamed module for classpath code), and document the access requirement. If you are reflecting into JDK internals (`sun.*`, `java.lang.invoke.*`), stop — find a supported API or accept that your code will break on future JDKs. The era of "reflect into anything with `setAccessible`" is over; design for module-safe access.

### Match Annotation Retention To When The Annotation Is Read

Annotations have three retention policies: `SOURCE` (discarded by the compiler, visible only to annotation processors — `@Override`, `@SuppressWarnings`), `CLASS` (written to the class file but not loaded into the runtime — the default, rarely useful), and `RUNTIME` (available via reflection at runtime — what most frameworks need). The retention is set via `@Retention(RetentionPolicy.RUNTIME)` on the annotation's definition.

The critical bug: if a framework reads an annotation via reflection at runtime but the annotation's retention is `SOURCE` or `CLASS`, the annotation is invisible — `getAnnotation(MyAnnotation.class)` returns `null`, and the framework silently does nothing. This is a common, hard-to-spot bug because the code compiles, the annotation is present in source, and the only symptom is that the framework ignores the annotated element.

The rule: if any runtime code reads the annotation via reflection, the annotation must be `@Retention(RetentionPolicy.RUNTIME)`. If only an annotation processor reads it at compile time, `SOURCE` is correct and more efficient. The default (`CLASS`) is almost never what you want — be explicit. Also set `@Target` to constrain where the annotation can be used (a `@FieldType` annotation that can be applied to a method is a latent bug).

### Prefer Annotation Processors Over Runtime Reflection Where Possible

Annotation processors (JSR 269) run at compile time and can generate source code based on annotations — this is how Dagger (DI), MapStruct (mappers), Immutables (value types), and the Spring Boot configuration metadata work. The generated code is typed, compile-checked, and fast (no runtime reflection), which avoids both the performance cost and the fragility of runtime reflection. An annotation processor that generates a `switch` over known types is faster and safer than runtime reflection that discovers those types.

The tradeoff: annotation processors require a build step (they run during compilation), they generate code that must be understood (and sometimes debugged), and they cannot handle types that are not present at compile time (dynamically loaded plugins). For frameworks that know their types at compile time, prefer annotation processing; for frameworks that must handle unknown runtime types (a plugin loader, a generic serializer), runtime reflection is necessary.

The modern trend is toward compile-time generation (Dagger over Spring for Android, MapStruct over hand-written mappers, records over reflection-based DTOs) because it is faster, safer, and works better with ahead-of-time compilation (GraalVM native images, which cannot use runtime reflection without configuration).

### Use Dynamic Proxies And InvocationHandler For Interface-Based Interception

`java.lang.reflect.Proxy` creates an implementation of one or more interfaces at runtime, delegating each call to an `InvocationHandler`. This is the mechanism behind Spring's AOP proxies, Hibernate's lazy-loading proxies, and Mockito's mocks. It is the right tool when you need to intercept calls to an interface (logging, transactions, caching, authorization) without modifying the implementation.

The limitations: proxies only work for interfaces, not classes (use ByteBuddy/CGLIB for class proxying, which has its own gotchas around final classes and constructors). The `InvocationHandler.invoke` must handle `hashCode`, `equals`, and `toString` explicitly (the default `Object` methods are routed through the handler) or the proxy behaves incorrectly in collections. And each call goes through the handler, adding overhead — fine for coarse-grained interception (a service method), bad for fine-grained (every field access).

## Common Traps

### Using Reflection Where An Interface Or Generic Would Do

Reflection loses compile-time safety, IDE support, and performance. If the type is known at compile time, use an interface, lambda, or generic method instead.

### Forgetting @Retention(RUNTIME) On A Framework-Read Annotation

A `SOURCE` or `CLASS` annotation is invisible to runtime reflection; `getAnnotation` returns null and the framework silently ignores the element. Always set `RUNTIME` for runtime-read annotations.

### setAccessible Failing Under JPMS Modules

`InaccessibleObjectException` when reflecting into a non-exported package of another module. Design for module-safe access; use `--add-opens` only as a documented migration aid.

### Calling getDeclaredMethod In A Hot Loop

Method/Field lookup is expensive; cache the `Method`/`Field` objects and reuse them, or switch to `MethodHandle`/generated bytecode for hot paths.

### Not Handling hashCode/equals/toString In A Dynamic Proxy

`Proxy` routes `Object` methods through `InvocationHandler`; if the handler does not special-case them, the proxy breaks in collections and equality checks.

### Reflecting Into JDK Internals (sun.*)

Unsupported and blocked by default in modern JDKs. Find a supported API; code that reflects into `sun.misc.Unsafe` or `java.lang.invoke.*` internals will break on future JDK releases.

### Annotation Without @Target Applied Anywhere

An annotation without `@Target` can be placed on any element, including nonsensical ones (`@Field` on a method). Always constrain with `@Target(ElementType.FIELD)` etc.

## Self-Check

- [ ] Reflection is confined to framework/plugin code where types are genuinely unknown at runtime; application code uses interfaces, generics, lambdas, or factories instead of reflection wherever the type is known at compile time.
- [ ] Reflective `Method`/`Field`/`Constructor` objects are cached after lookup and reused, not re-looked-up in hot paths; for performance-critical paths, `MethodHandle`/`VarHandle` or generated bytecode is considered and measured.
- [ ] `setAccessible(true)` is used only on accessible targets (own classes, exported packages, unnamed-module classpath code); no code reflects into JDK internals (`sun.*`) or non-exported packages of other modules without a documented `--add-opens` migration note.
- [ ] Every annotation read via runtime reflection has `@Retention(RetentionPolicy.RUNTIME)`; annotations read only at compile time have `@Retention(SOURCE)`; no annotation relies on the default `CLASS` retention without a reason.
- [ ] Every custom annotation has an explicit `@Target` constraining where it can be applied, preventing misuse (a field annotation applied to a method).
- [ ] Where types are known at compile time, annotation processors (Dagger, MapStruct, Immutables) are preferred over runtime reflection for the performance and safety benefits.
- [ ] Dynamic proxies (`java.lang.reflect.Proxy`) handle `hashCode`, `equals`, and `toString` explicitly in the `InvocationHandler`, and class proxying (ByteBuddy/CGLIB) is used only where interface proxying is insufficient, with awareness of final-class and constructor limitations.
- [ ] Reflection-heavy code is isolated behind typed interfaces so the fragility is confined; the public API does not expose `Method`/`Field`/`Class` objects to callers.
