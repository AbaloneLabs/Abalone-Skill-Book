---
name: assembly_and_versioning.md
description: Use when the agent is building or refactoring .NET libraries, managing assembly references, strong naming, signing, the GAC, binding redirects, side-by-side loading, NuGet package versioning, API compatibility, or diagnosing type identity, MissingMethodException, and load-context conflicts in C# and .NET applications.
---

# Assembly and Versioning

The .NET assembly is the unit of deployment, type identity, and version policy, and most teams treat versioning as a build detail until something breaks in production. The classic failure modes are all version problems in disguise: `MissingMethodException` at runtime because a method was renamed between the version compiled against and the version loaded, `FileLoadException` because two dependencies demand incompatible versions of a shared library, type identity errors because the "same" type loaded from two different assemblies is treated as different by the runtime. These are not exotic; they are the predictable result of treating public surface area as mutable and version numbers as cosmetic.

The judgment problem is that a library's public API, its assembly name, its version number, and its strong-name token together form a contract that the loader uses to decide what is compatible. Change any of them carelessly and you either break callers at compile time, at load time, or at runtime in ways that are extremely hard to debug. The goal is to make versioning decisions deliberately and to understand which changes are source-compatible, binary-compatible, or runtime-compatible, and which are breaking.

## Core Rules

### Distinguish the four kinds of compatibility

Every change to a published library falls into one of these categories, and you must know which:

- **Source compatible**: existing source recompiles and works. Adding a method is usually source-compatible.
- **Binary compatible**: existing compiled binaries link and run without recompilation. Adding a method is binary-compatible; changing a method signature is not.
- **Runtime compatible**: the version actually loaded at runtime behaves like the version expected. Even binary-compatible changes can fail if binding policy loads a different version.
- **Behavioral compatible**: the observable behavior is unchanged. A "fix" that changes timing or return values can break callers that depended on the old behavior.

State which of these each change preserves, and assume callers depend on all of them.

### Treat the public API as immutable once published

Once a library is consumed by code you do not control, its public surface (public classes, methods, properties, parameters, exceptions, return values) is a contract. Safe additions include new public members, new classes, new overloads that do not create ambiguity. Unsafe changes include removing or renaming members, changing signatures, changing namespaces, adding mandatory parameters, widening exception contracts, and changing return-value semantics. When you must break, bump the major version and provide migration guidance, and consider shipping a parallel API for a release cycle.

### Understand assembly version vs file version vs package version

These three numbers are independent and commonly confused:

- **Assembly version** (`AssemblyVersion`) is what the loader uses for binding and is the only one that affects type identity and binding policy. This is the one that must follow semantic rules carefully.
- **File version** (`FileVersion`) tracks the build and is informational.
- **Package version** (NuGet) is what consumers see and restore.

A common, safe policy: keep `AssemblyVersion` stable across compatible patch and minor releases (so binding redirects are unnecessary) and advance it only on breaking changes, while letting the package version follow full SemVer. Decide your policy explicitly and document it.

### Decide strong naming and signing deliberately

Strong naming gives an assembly a public-key token and a fully qualified name, which the loader uses for identity and (historically) for the GAC. It is not a security measure in modern .NET. Rules:

- Libraries intended for broad reuse should be strong-named so consumers that require it can reference them.
- Strong-naming pins identity; once signed, changing the key or the name is a breaking change.
- In .NET Core/.NET 5+, strong naming is mostly ceremonial for identity, but many enterprise and framework hosts still require it.

Do not sign ad hoc; the key, its storage, and the policy around rotation are operational concerns.

### Binding redirects exist because of version policy, not bugs

When application A references Library B 1.0 and Library C references Library B 1.1, the runtime unifies to one version via binding redirect. This works only if B 1.1 is binary-compatible with B 1.0. If it is not, you get `MissingMethodException` or behavioral bugs. Binding redirects are a symptom of multiple consumers demanding different versions; they do not make incompatible versions compatible. Audit redirect chains in large dependency graphs and prefer packages whose authors keep assembly versions stable.

### Side-by-side loading has real costs

The runtime can load multiple versions of the same assembly into one process, but each version is a distinct type identity: `MyLib.Type, V1` and `MyLib.Type, V2` are different types and cannot be assigned to each other. This causes cast failures, "type T cannot be assigned to T" errors, and duplicate singletons. In .NET Framework, the GAC enabled side-by-side; in modern .NET, prefer a single unified version via binding redirects or package downgrade. Avoid designs that require loading multiple versions of the same library into one process.

### Manage load context and dependency resolution

The runtime resolves assemblies in different load contexts (default, load-from, neither). Loading the "same" assembly via different paths or contexts creates duplicate type identity. Prefer `AssemblyLoadContext` in modern .NET for plugin isolation, and understand that plugin assemblies that share a dependency must agree on which version loads or you get duplicate types. For plugins, isolate per-context and serialize across the boundary rather than sharing live objects.

### Version your NuGet packages with SemVer, and earn it

Package version should reflect the *consumer-visible* change: major for breaking, minor for new compatible features, patch for compatible fixes. The trap is shipping a "minor" or "patch" that is actually breaking because of a behavioral change or a transitive dependency bump. Run a compatibility check (API diff tools, behavioral tests) before publishing and treat any break as a major bump regardless of how small the diff looks.

## Common Traps

### Treating assembly version as the build number

Auto-incrementing `AssemblyVersion` on every build forces binding redirects everywhere and makes type identity unstable. Keep assembly version stable across compatible releases; use file/build version for the incrementing number.

### Removing or renaming a public member in a "minor" release

Callers compiled against the old member throw `MissingMethodException` at runtime. This is a breaking change regardless of how trivial the rename seems. Either keep the old member as deprecated, or bump major.

### Adding an overload that changes overload resolution

A new overload can change which method an existing call binds to, especially with optional parameters or generics. Test that existing call sites still resolve identically after adding overloads.

### Changing the default value or behavior of an optional parameter

Changing a default is a source-compatible but binary-incompatible change: callers compiled against the old default bake the old value in, new callers get the new value, and behavior diverges. Treat default changes as breaking.

### Assuming the GAC or global store solves conflicts

Putting an assembly in the GAC (or a shared runtime store) does not make incompatible versions compatible; it just centralizes where the conflict surfaces. Shared stores require stricter version discipline, not less.

### Shipping a transitive dependency bump as a patch

Bumping a transitive dependency in a patch release can force a binding redirect or a breaking change on consumers who did not opt in. Pin transitive dependencies conservatively and bump them deliberately.

### Using `InternalsVisibleTo` as a public API extension

`InternalsVisibleTo` creates a compile-time coupling to internal members that can change freely. If a consumer depends on internals via this attribute, your freedom to refactor internals is gone. Reserve it for test assemblies and tightly co-versioned partners.

## Self-Check

- For every change to a published library, have you classified it as source/binary/runtime/behaviorally compatible, and is the version bump (major/minor/patch) consistent with that classification?
- Is the assembly version kept stable across compatible releases, with only the file/package version incrementing for non-breaking changes?
- Are public members treated as immutable? If any were removed, renamed, or signature-changed, is the major version bumped and migration guidance provided?
- Have you verified that new overloads or default-parameter changes do not alter existing overload resolution or baked-in defaults?
- If the library is strong-named, is the key, its storage, and the rotation policy documented and consistent?
- Have you audited binding redirect chains in the dependency graph, and are transitive dependency bumps deliberate rather than incidental?
- Does the design avoid requiring multiple versions of the same assembly in one process, or if it requires it (plugins), is type identity isolated per load context?
- Are `InternalsVisibleTo` grants limited to test assemblies and co-versioned partners, not exposed as a stable API?
