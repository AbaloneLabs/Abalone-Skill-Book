---
name: csharp_reflection_and_source_generators.md
description: Use when the agent is writing or reviewing C# code that uses reflection (Type, MethodInfo, attributes, Activator), expression trees, dynamic, System.Text.Json or serialization reflection, or Roslyn source generators and incremental generators, or when deciding between runtime reflection and compile-time source generation, diagnosing trim/AOT warnings (IL2050, IL2026), slow reflection in hot paths, or generator design errors. Covers when reflection is justified vs when source generation or expression compilation is the better choice, and NativeAOT compatibility.
---

# C# Reflection And Source Generators

Reflection lets C# inspect types and members at runtime and invoke them dynamically, which is powerful for serialization, ORMs, plugin systems, and DI containers. But reflection is slow (each lookup involves metadata traversal), allocates, and — critically for modern .NET — is incompatible with trimming and NativeAOT, because the trimmer cannot see runtime-driven type usage and may remove the very members reflection tries to use. Source generators (C# 9, Roslyn) are the compile-time answer: they inspect code at build time and emit additional C# source, giving you the metaprogramming capability of reflection without its runtime cost or AOT incompatibility. The judgment problem is to recognize when reflection is the wrong tool (slow, AOT-incompatible) and when a source generator, compiled expression tree, or code generation is the better choice, and to design reflection code that is trim-safe when reflection is genuinely required.

Agents tend to reach for reflection because it is familiar and dynamic, then ship code that is slow in hot paths, that breaks under trimming/NativeAOT, or that scatters `DynamicallyAccessedMembers` annotations incorrectly. The judgment problem is to prefer source generation for known-at-compile-time metaprogramming (JSON serialization, DI registration, mapping), to cache reflection results and compile to delegates where reflection is unavoidable, and to annotate correctly for trim/AOT safety. This skill is about choosing the right metaprogramming mechanism — reflection, expression trees, or source generation — for the scenario.

## Core Rules

### Prefer Source Generators Over Runtime Reflection For Compile-Time-Known Metaprogramming

When the types and members involved are known at compile time (which is most metaprogramming — serializing a known DTO, registering known DI services, mapping between known types), a source generator is strictly better than runtime reflection: it runs at build time, emits C# source that the compiler type-checks, has zero runtime reflection cost, and is fully trim/AOT-safe. Modern .NET libraries use source generators extensively (`System.Text.Json` source generation, minimal APIs, `Microsoft.Extensions.Logging` compile-time loggers, DI `AddSingleton<T>` registration generators).

Use source generators when: the set of types is bounded by compile-time attributes or patterns; you want zero runtime cost; you need AOT/trim compatibility. Write an incremental generator (not the older `ISourceGenerator`) for performance, so regeneration is scoped to changed files. Reserve runtime reflection for genuinely dynamic scenarios (plugins loaded at runtime, unknown types).

### Cache Reflection Results And Compile To Delegates When Reflection Is Unavoidable

When you must use runtime reflection (a plugin system, an unknown-type scenario), the two performance killers are repeated metadata lookup and reflective invocation. Both can be mitigated:

- **Cache lookup results**: a `MethodInfo`/`PropertyInfo`/`FieldInfo` looked up once should be cached (in a static dictionary, keyed by type) rather than re-looked-up per call. The lookup is the expensive part.
- **Compile to delegates**: `MethodInfo.CreateDelegate<T>()` or `Expression.Compile` produces a typed delegate that invokes as fast as a direct call, eliminating the per-call reflective invocation overhead. For a hot path that reflectively invokes a method, compile it to a delegate once and cache the delegate.

Never call `methodInfo.Invoke(...)` in a hot path without caching and compiling; it is orders of magnitude slower than a direct or delegated call.

### Annotate For Trim And NativeAOT Compatibility

.NET trimming removes unused code to reduce app size, and NativeAOT compiles ahead of time. Both break reflection that the trimmer/AOT compiler cannot statically see: if you reflectively access a member that the trimmer thinks is unused, it is removed, and the runtime reflection fails. The `[DynamicallyAccessedMembers(DynamicallyAccessedMemberTypes.PublicMethods)]` annotation on a `Type` parameter tells the trimmer "this Type's public methods are accessed reflectively, keep them," restoring trim safety.

- Annotate `Type` parameters and `string` type-name parameters that flow to reflection with the appropriate `DynamicallyAccessedMembers`.
- Ensure types whose members are reflectively accessed are rooted (referenced statically, or annotated) so they survive trimming.
- Test with `dotnet publish -p:PublishTrimmed=true` (and `-p:PublishAot=true` for NativeAOT) to surface trim/AOT warnings (IL2050, IL2026, etc.) and fix them by annotating or restructuring.

A library that uses unannotated reflection will produce trim/AOT warnings (or runtime failures) for consumers who trim. Treat trim/AOT compatibility as a first-class concern for any library.

### Use Attributes To Drive Source Generation Or Selective Reflection

Attributes (`[MySerializable]`, `[GenerateMapper]`) on types or members are the standard way to signal to a source generator (or to runtime reflection) which types to process. The generator scans for the attribute at compile time and emits code; runtime reflection scans for it at startup. This makes the metaprogramming explicit and opt-in, rather than implicit and surprising.

Prefer attribute-driven source generation over convention-based reflection ("all types in this namespace") because the generator sees exactly the intended types, the generated code is type-checked, and there is no runtime scan. When using runtime reflection with attributes, cache the scan results at startup.

### Use Expression Trees For Dynamic Code That Must Be Fast And Verifiable

`System.Linq.Expressions` lets you build code as a data structure at runtime and compile it to a delegate with `Expression.Compile`. It is faster than reflective invocation (the compiled delegate is near-direct-call speed) and verifiable (the expression tree is type-checked at compile time). Use it for scenarios where you build a dynamic pipeline (a mapper, a serializer, a rules engine) from runtime-known structure: build the expression tree once, compile to a delegate, cache it, and invoke the delegate in the hot path.

Expression trees are more verbose than reflection but far faster in hot paths. For one-off dynamic invocation, reflection is fine; for a hot path, compile to an expression.

### Avoid dynamic And Reflection Where Static Typing Suffices

`dynamic` and reflection both defer type checking to runtime, losing compile-time safety and IDE support. Use them only where the types are genuinely not known at compile time (COM interop, dynamic JSON, plugins). For everything else, static typing, generics, and source generation give you compile-time checking, better performance, and better tooling. Reaching for `dynamic` to avoid defining a type is usually a mistake.

## Common Traps

### Reflection In A Hot Path Without Caching

`type.GetMethod("Foo").Invoke(...)` per call is orders of magnitude slower than a direct call. Cache the `MethodInfo` and compile to a delegate, or use a source generator.

### Unannotated Reflection Breaking Under Trimming/NativeAOT

A library that reflectively accesses members without `[DynamicallyAccessedMembers]` annotations fails when the consumer trims or AOT-compiles. Annotate and test with `PublishTrimmed`/`PublishAot`.

### Using Reflection Where A Source Generator Would Do

Serializing a known DTO with runtime reflection is slower and AOT-incompatible; `System.Text.Json` source generation produces trim-safe, fast serialization. Prefer source generation for compile-time-known metaprogramming.

### Non-Incremental Source Generator Regenerating Everything

An `ISourceGenerator` (older) regenerates on every change; an `IIncrementalGenerator` (newer) scopes regeneration to changed inputs. Use incremental generators for build performance.

### MethodInfo.Invoke For A Repeatedly-Called Method

Compiling to a delegate (`MethodInfo.CreateDelegate<T>()`) is near-direct-call speed; reflective `Invoke` is slow. Compile and cache.

### Convention-Based Runtime Reflection Scan At Startup

Scanning all assemblies for types with an attribute is slow and trim-unfriendly. Use a source generator driven by the attribute, or scope the scan and cache it.

### dynamic To Avoid Defining A Type

`dynamic` loses compile-time checking; defining a record or using generics preserves it. Use `dynamic` only for genuinely dynamic scenarios (COM, dynamic JSON).

### Expression.Compile In A Hot Path (Recompiling)

Building and compiling an expression per call is slow; build once, compile once, cache the delegate, and invoke the cached delegate in the hot path.

## Self-Check

- [ ] Compile-time-known metaprogramming (serialization, DI, mapping) uses source generators (incremental, attribute-driven) rather than runtime reflection, giving zero runtime cost and AOT/trim compatibility.
- [ ] Runtime reflection, where unavoidable (plugins, unknown types), caches lookup results and compiles to delegates (`MethodInfo.CreateDelegate` / `Expression.Compile`) so the hot path invokes a cached delegate, not reflective `Invoke`.
- [ ] Reflection code is annotated with `[DynamicallyAccessedMembers]` and tested with `PublishTrimmed`/`PublishAot` so consumers can trim/AOT-compile without runtime failures.
- [ ] Attributes drive source generation or selective reflection so the processed types are explicit and opt-in, not convention-scanned at runtime.
- [ ] Expression trees are used for dynamic code that must be fast and verifiable, built and compiled once and cached, not recompiled per call.
- [ ] `dynamic` and reflection are reserved for genuinely runtime-known types (COM, dynamic JSON, plugins); static typing, generics, and source generation are used where types are known at compile time.
- [ ] Source generators are incremental (`IIncrementalGenerator`) and scoped to changed inputs for build performance, and generated code is type-checked by the compiler.
- [ ] No library ships unannotated reflection that produces trim/AOT warnings for consumers; trim/AOT compatibility is tested in CI.
