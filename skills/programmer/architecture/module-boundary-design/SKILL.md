---
name: module_boundary_design.md
description: Use when the agent is organizing code into modules, packages, or layers, deciding what is public versus internal, fixing circular dependencies, choosing a coupling axis, reviewing why one change touches many files, or planning how to split, merge, or extract a component. Also covers package layout, dependency direction, encapsulation boundaries, interface segregation, and detecting boundary violations during refactor or review.
---

# Module Boundary Design

A module boundary is a decision about what changes together and what changes independently. It is not a folder layout exercise. Every boundary declares a theory: "these things know about each other, and those things do not." When the theory is wrong, the cost shows up later as a change that ripples across the codebase, a test that needs the whole system to run, a refactor that cannot be finished, or a feature that must be built through a back door.

Agents tend to under-invest in boundary decisions because the harm is delayed. Organizing files by type, exposing helpers because they are convenient, or letting one package reach into another's internals produces working code immediately and debt silently. The judgment problem is deciding where to draw lines, what to publish, what to hide, and which axis a boundary should follow.

## Core Rules

### Choose A Boundary Axis Deliberately

Every boundary follows an axis. The common axes are capability, ownership, deployment, data, lifecycle, and rate of change. Pick the axis that matches the real pressure on the system, not the one that looks cleanest.

Ask which of these pressures is real:

- **Capability** — "payments knows about charging; billing does not." Good when domains have distinct rules and vocabulary.
- **Ownership** — "team A owns this, team B owns that." Good in multi-team organizations; bad when teams are fictional.
- **Deployment** — "this must ship independently." Good for plugin systems, mobile shells, or independently released services.
- **Data** — "this owns its own store and no one else writes to it." Good for encapsulating persistence and migrations.
- **Lifecycle** — "this changes fast, that changes slowly." Good for isolating volatile code from stable contracts.
- **Rate of change** — "experimental code must not destabilize core." Good for feature-flagged or rapidly iterating areas.

A boundary that follows the wrong axis rots. A capability boundary in a single-team startup adds ceremony without ownership pressure. A deployment boundary inside one process adds network hops without release independence. Name the axis you chose and the pressure it answers.

### Make Dependency Direction Explicit And Acyclic

Dependencies must point in one direction: from the thing that depends on the decision toward the thing that owns the decision. In layered systems this usually means presentation depends on domain, domain depends on abstractions, and infrastructure implements those abstractions. The goal is not "layers" as a shape but "no rule that says A needs B and B needs A."

For each cross-module reference, ask:

- Does the caller depend on a stable contract or on a concrete implementation?
- Could the called module be replaced without changing the caller?
- Is there any path by which the dependency reverses (events, callbacks, shared mutable state, global registries)?
- Does the dependency cross the chosen axis, and if so, is that intentional?

Enforce direction with tools when possible: package visibility, access modifiers, dependency-cop lint rules, import allowlists, or module manifests. A convention that is not enforced becomes a boundary violation the moment a deadline arrives.

### Separate Public Contract From Internal Implementation

A module has two surfaces: what it promises to the outside and how it fulfills that promise internally. Only the public contract is a stable dependency target. Everything else may change freely.

Make the distinction visible in code:

- Public types and functions live in an explicitly exported API (a `public` module, an interface package, an `exports` list).
- Internal helpers, data structures, configuration wiring, and persistence details are private.
- DTOs and request/response shapes that cross the boundary are public; their backing entities are not.
- Tests exercise the public surface. Tests that import internals are a signal that the public surface is incomplete.

When a consumer reaches past the public API, treat it as a boundary bug. Either the API is missing a needed capability, or the consumer is coupled to implementation it should not know about. Resist the shortcut of "just make it public for now."

### Favor Cohesion Over Apparent Reuse

A common mistake is splitting code because it looks reusable. Two modules that share a function are not necessarily cohesive; forcing them through a shared utility can couple their change cycles. Cohesion means the elements of a module change together for the same reason. Reuse without cohesion creates fragile shared dependencies where a change for one consumer breaks another.

Before extracting shared code, ask:

- Do the consumers change for the same reason or different reasons?
- If I change this helper for consumer A, must consumer B also change or retest?
- Is the shared abstraction real, or is it a coincidence of current code?

It is often better to duplicate a small piece of code than to invent a premature abstraction that entangles two unrelated modules. Duplication is local debt; a wrong shared abstraction is structural debt.

### Size A Boundary To Its Responsibility, Not To A Number

There is no correct module size. A boundary is right when it can be described in one sentence, has a clear public surface, hides a meaningful implementation, and can be reasoned about without reading its internals. A boundary is wrong when its description is "various utilities" or "stuff that other modules need."

Signs a boundary is too large:

- Its public surface has several unrelated responsibilities.
- Changes for unrelated reasons keep landing in the same module.
- It is impossible to test one feature without loading the whole module.

Signs a boundary is too small:

- The public surface is one function used by one caller.
- Every change requires editing the boundary and its single caller together.
- The boundary exists only to satisfy a layering convention.

Resize by responsibility, not by line count.

### Treat Cross-Boundary Data As A Contract

Data that crosses a boundary is part of the public contract even if it is "just an object." Internal entity shapes, enum values, error types, and nullability rules all leak coupling when they cross a boundary unchanged.

For each cross-boundary payload, decide:

- Is the shape owned by the producer, the consumer, or a neutral contract module?
- Are internal enum values exposed, or is a stable public representation used?
- Are nullability and error semantics documented and stable?
- Does the payload carry internal identifiers, flags, or state that callers should not see?

Prefer defining boundary types in the module that owns the contract, not in the module that happens to produce or consume them. This keeps the contract independent of either side's implementation.

### Plan For Evolution Before You Need It

Boundaries that are easy to evolve share a property: the public contract is narrow and additive. Boundaries that are hard to evolve have broad contracts where consumers depend on internal behavior.

When designing a boundary, assume it will change. Prefer:

- adding capabilities over changing existing ones;
- optional parameters over new required ones;
- versioned or feature-flagged contracts over silent shape changes;
- deprecation cycles over removals;
- documenting which parts of the public surface are stable versus experimental.

A boundary that cannot change without coordinating many consumers was not really a boundary; it was a shared implementation with a public label.

## Common Traps

### Organizing By Type Instead Of By Responsibility

Putting all controllers in one folder, all services in another, and all repositories in a third produces boundaries that follow a technical axis. A change to "user signup" then touches three packages, each of which also contains unrelated features. This is convenient to scaffold and painful to evolve. Prefer feature or capability boundaries, with technical layers inside each.

### Exposing Internals "For Testing"

Making a method public, weakening an access modifier, or adding a test-only constructor so a test can reach private state is a boundary erosion. It signals the public surface is incomplete. Either add a real public capability the test can use, or test through the public surface and accept that internals are covered indirectly.

### The Shared Utility Module That Everything Depends On

A `common`, `utils`, or `helpers` package with no cohesive responsibility becomes a magnet for unrelated code. Once many modules depend on it, any change to it risks the whole system, and it grows without review. Treat a catch-all utility module as a symptom: each piece usually belongs in the module that uses it, or in a focused library with its own contract.

### Circular Dependencies Disguised As Events Or Callbacks

Two modules that do not import each other can still be circularly coupled through a shared event bus, a global registry, inversion-of-control wiring, or callback registration. The import graph looks clean while the runtime dependency is cyclic. Check the runtime dependency, not only the import graph, when reasoning about boundaries.

### Boundary By Framework Convention Without Intent

Many frameworks prescribe a folder layout: controllers here, models there, services elsewhere. Following the layout without asking what each boundary is for produces a system where the framework owns the architecture. The boundary should express your system's pressures; the framework's conventions are a starting point, not the answer.

### Treating "Internal" As A Naming Convention

Prefixing a package or class with `_` or naming it `internal` does not enforce a boundary if the language or tooling allows access. If access is not enforced, the boundary is advisory and will be crossed under deadline pressure. Prefer real access control, lint rules, or separate packages with explicit export lists.

### Merging Modules To "Simplify" A Tangled Dependency

When two modules are circularly coupled, the tempting fix is to merge them. This removes the cycle but also removes the boundary, and the merged module now carries both responsibilities and both change cycles. First try to find the shared responsibility and extract it, or invert one dependency through an abstraction. Merge only when the two were never truly separate.

## Self-Check

- [ ] Each boundary can be described in one sentence naming the axis it follows (capability, ownership, deployment, data, lifecycle, or rate of change).
- [ ] Dependency direction is explicit and acyclic at runtime, not only in the import graph; events, callbacks, registries, and shared state were checked for hidden cycles.
- [ ] Each module has a visible public surface, and consumers reach only that surface rather than internal helpers or entities.
- [ ] Shared code was extracted only where consumers change for the same reason; duplication was preferred over premature abstraction where reasons differ.
- [ ] Cross-boundary payloads are defined as contract types, and internal enums, identifiers, flags, and nullability rules are not leaked unchanged.
- [ ] No boundary exists only as "utils," "common," or "helpers" without a cohesive responsibility.
- [ ] Access restrictions are enforced by language, tooling, or package boundaries, not only by naming convention.
- [ ] The public surface is narrow and additive, so the boundary can evolve without coordinating many consumers.
- [ ] Tests exercise the public surface; test-only access widening was not used to reach private state.
- [ ] Folder layout follows responsibility and the chosen axis, not only framework convention or technical type.
