---
name: dependency_decoupling_and_layering.md
description: Use when the agent is decoupling modules, introducing abstraction layers, breaking circular dependencies, refactoring toward a layered or hexagonal architecture, applying dependency inversion, extracting interfaces, reducing coupling between modules, or diagnosing a codebase that is hard to change because everything depends on everything. Covers dependency direction (dependencies point inward toward stable abstractions), the dependency inversion principle, breaking cycles, layering and module boundaries, abstraction cost vs benefit, the "shotgun surgery" smell of excessive coupling, and the discipline of decoupling to enable independent change rather than decoupling for its own sake.
---

# Dependency Decoupling And Layering

Coupling — the degree to which modules depend on each other — is the primary determinant of a codebase's changeability. In a tightly coupled codebase, a change to one module ripples to many others (shotgun surgery: one logical change requires touching dozens of files), testing is hard (a module cannot be tested without its dependencies, and their dependencies, transitively), and understanding is hard (no module can be read in isolation). The discipline of decoupling and layering is structuring dependencies so that modules can change independently: dependencies point in a consistent direction (toward stable abstractions, not concrete implementations), cycles are eliminated (a cycle means the modules cannot change or be understood independently), and module boundaries align with the system's domains (so a change within a domain does not cross boundaries). The goal is not zero coupling (modules must interact to form a system) but directed, bounded coupling: each module depends on stable abstractions, changes in one module do not force changes in unrelated modules, and the dependency graph is acyclic and aligned with the architecture.

Agents tend to decouple reactively (extracting interfaces everywhere, creating abstraction layers speculatively) without analyzing where coupling actually causes pain, to introduce abstractions that add indirection without reducing coupling, and to break cycles by moving code rather than by introducing the right abstraction. The judgment problem is recognizing that decoupling serves changeability (decouple where independent change is needed, not everywhere), that the dependency direction and cycle-freedom are the structural invariants, and that abstraction has a cost (indirection, complexity) that must be justified by the coupling reduction. This skill covers the discipline of dependency decoupling and layering: dependency direction, inversion, cycle-breaking, layering, abstraction cost, and decoupling where it matters.

## Core Rules

### Direct Dependencies Toward Stable Abstractions

The direction of dependencies determines the codebase's flexibility. Dependencies should point toward stable abstractions (interfaces, core domain), not toward volatile concrete implementations.

- **Apply the dependency inversion principle: high-level policy should not depend on low-level detail; both should depend on abstractions.** A business logic module should not depend directly on a specific database or external API; it should depend on an interface (a repository, a gateway), and the concrete implementation depends on the same interface. This allows the business logic to be tested without the database, and the database to change without the business logic.
- **Point dependencies inward, toward the domain core (hexagonal/onion architecture).** In a layered or hexagonal architecture, the domain core (business logic) has no dependencies on infrastructure (databases, frameworks, external services); infrastructure depends on the core (implementing the core's interfaces). This makes the core stable, testable, and independent of infrastructure choices.
- **Depend on interfaces, not concrete classes, at module boundaries.** A module that depends on another module's interface (not its concrete class) can be tested with a stub, and the depended-on module can be replaced without the depending module changing. The interface is the stable contract; the concrete class is the volatile implementation.
- **Do not invert dependencies that do not need inversion.** A module using a standard library or a stable utility does not need an interface; the dependency is stable, and inverting it adds indirection without benefit. Invert where the dependency is volatile, where testing requires isolation, or where multiple implementations are plausible.

### Eliminate Circular Dependencies

A circular dependency (module A depends on B, B depends on A) means the modules cannot be changed, tested, or understood independently — they form a monolith despite appearing separate. Cycles are a structural defect to eliminate.

- **Detect cycles (module dependency analysis, compile errors in languages that forbid cycles).** A cycle manifests as compile errors (in languages that detect them), as test setup that requires the whole cycle (cannot test A without B without A), or as a dependency graph analysis showing the cycle. Identify cycles explicitly.
- **Break cycles by extracting the shared dependency into a lower layer.** If A and B depend on each other because they share a concept, extract that concept into a module C that both A and B depend on (downward), eliminating the A-B cycle. The shared concept moves to a stable lower layer.
- **Break cycles by inverting one dependency (introduce an interface).** If A depends on B and B depends on A, and the B-to-A dependency is the one to break, define an interface in a lower layer (or in B) that A implements; B depends on the interface, not on A directly. The cycle is broken by the abstraction.
- **Break cycles by introducing an event or callback (decouple via indirection).** If A and B are coupled because A calls B and B needs to notify A, use an event or callback: B emits an event that A subscribes to, rather than B calling A directly. The runtime coupling remains, but the source-level dependency cycle is broken.

### Align Module Boundaries With Domain Boundaries

Coupling is lowest when module boundaries align with domain boundaries: a change within a domain stays within the module, not crossing to unrelated modules.

- **Group by domain (business capability), not by technical layer.** A "user management" module (domain) that contains its own data, logic, and interface is more cohesive than a "data access" module (technical) that mixes user, order, and billing data. Domain-aligned modules change together (a user-management change is in one module); technical-aligned modules force cross-cutting changes.
- **Define clear module interfaces (what the module exposes vs what is internal).** A module should have a small, stable public interface and a large private implementation. Changes to the private implementation do not affect dependents; changes to the public interface are deliberate and reviewed. Without this distinction, every internal change risks breaking dependents.
- **Minimize the interface surface (expose less, not more).** A module that exposes everything is coupled to its dependents on everything; a module that exposes a small interface is coupled only on that interface. Expose the minimum that dependents need; keep the rest internal.
- **Avoid "leaky abstractions" where the implementation details bleed through the interface.** An interface that exposes implementation details (a database-specific exception, a framework-specific type) couples dependents to the implementation, defeating the abstraction. The interface should be implementation-agnostic.

### Decouple Where Independent Change Is Needed, Not Everywhere

Decoupling has a cost (indirection, complexity, more code). Apply it where it serves independent change, testability, or multiple implementations — not speculatively.

- **Decouple modules that change for different reasons or at different rates.** If module A changes frequently (business logic) and module B is stable (a standard utility), decoupling them (A depends on an interface B implements) allows A to change without touching B. Decoupling two modules that always change together adds cost without benefit.
- **Decouple for testability: isolate the module under test from its dependencies.** A module that cannot be tested without its real dependencies (a database, an external API) is hard to test reliably and fast. Decoupling (depending on interfaces, injecting test doubles) enables fast, deterministic unit tests.
- **Decouple for multiple implementations: where more than one implementation is plausible or needed.** If a module might have multiple implementations (a payment processor with Stripe and PayPal, a repository with SQL and in-memory), an interface with multiple implementations is justified. If only one implementation will ever exist, the interface is speculative.
- **Do not decouple speculatively (YAGNI).** An interface extracted "in case we need another implementation" that never materializes adds indirection without benefit. Decouple when the need is present or clearly imminent, not speculatively.

### Weigh The Cost Of Abstraction Against The Benefit

Every abstraction (interface, layer, indirection) has a cost: more code, more indirection to follow, potential for abstraction-implementation mismatch. Weigh the cost against the coupling reduction.

- **An abstraction is justified when it reduces coupling that causes pain.** If the coupling makes changes hard, tests slow, or understanding difficult, the abstraction's benefit (reducing that pain) justifies its cost. If the coupling causes no pain (the modules change together, testing is fine), the abstraction is unnecessary cost.
- **Avoid "abstraction for abstraction's sake" (premature abstraction).** Extracting interfaces, creating layers, or generalizing before the need is clear produces abstractions that may not fit the eventual requirement, that add indirection, and that are hard to remove once added. Prefer concrete code until the abstraction's shape is clear.
- **Prefer simple decoupling (module boundaries, clear interfaces) over complex decoupling (elaborate frameworks, deep layer hierarchies).** A clear module boundary with a small interface achieves most of the decoupling benefit without the complexity of a framework. Reach for simple structures first; add complexity only when simple structures are insufficient.
- **Review abstractions for fit over time.** An abstraction that fit when created may not fit as requirements evolve (the abstraction's assumptions no longer hold). Refactor or remove abstractions that no longer fit, rather than working around them.

### Refactor Coupling Incrementally, With Tests

Decoupling a tightly coupled codebase is a large refactoring; do it incrementally, with tests guarding the behavior.

- **Decouple incrementally, one boundary at a time, not all at once.** A big-bang decoupling (rewriting the architecture) is risky and hard to verify. Decouple one boundary (extract one interface, break one cycle), verify the behavior is unchanged (tests pass), and proceed to the next.
- **Have characterization tests in place before decoupling.** Decoupling changes structure without changing behavior; tests verify the behavior is preserved. If tests are lacking, write characterization tests (capturing current behavior) before the refactoring, so changes are caught. See test-refactoring-and-test-smells.
- **Use the refactoring patterns (extract interface, move class, introduce parameter object) mechanically.** These are well-defined transformations with known steps; apply them carefully, one step at a time, with tests after each step. See large-scale-refactoring-planning.
- **Commit frequently, in small, reviewable chunks.** A decoupling refactoring committed in small chunks (one boundary per commit) is reviewable and revertible; a large commit is not.

## Common Traps

### Speculative Decoupling (Abstraction Without Need)

Extracting interfaces or layers "in case we need them," adding indirection without reducing real coupling pain. Decouple where independent change is needed; apply YAGNI.

### Dependency Cycle Left In Place

A circular dependency tolerated (worked around) rather than broken, keeping the modules coupled. Detect and break cycles by extraction or inversion.

### Leaky Abstraction

An interface that exposes implementation details (database exceptions, framework types), coupling dependents to the implementation. Keep interfaces implementation-agnostic.

### Decoupling Modules That Change Together

Decoupling modules that always change together (adding an interface between them), paying the abstraction cost without the coupling-reduction benefit. Decouple where change is independent.

### Over-Abstraction (Deep Layer Hierarchies)

Elaborate layer hierarchies or frameworks where simple module boundaries would suffice, adding complexity. Prefer simple structures; add complexity when needed.

### Big-Bang Decoupling Without Tests

Rewriting the architecture in one large change without tests to verify behavior, risking subtle behavior changes. Decouple incrementally with characterization tests.

### Exposing Too Much (No Public/Internal Distinction)

A module that exposes everything, coupled to dependents on all of it. Expose a small stable interface; keep the rest internal.

### Technical Layering Over Domain Layering

Modules grouped by technical layer (all data access together) rather than domain, forcing cross-cutting changes for domain changes. Group by domain.

## Self-Check

- [ ] Dependencies point toward stable abstractions (interfaces, domain core) via dependency inversion — high-level policy depends on abstractions, infrastructure depends on the core, and module boundaries depend on interfaces not concrete classes — where the dependency is volatile, testing requires isolation, or multiple implementations are plausible.
- [ ] Circular dependencies are detected and eliminated (by extracting shared concepts to a lower layer, inverting one dependency via an interface, or decoupling via events/callbacks), so modules can change, test, and be understood independently.
- [ ] Module boundaries align with domain boundaries (grouped by business capability, not technical layer), each module has a clear, small public interface (with the rest internal), the interface surface is minimized, and abstractions do not leak implementation details.
- [ ] Decoupling is applied where it serves independent change, testability, or multiple implementations — not speculatively (YAGNI), not where modules change together, and the cost of abstraction (indirection, complexity) is weighed against the coupling-reduction benefit.
- [ ] Simple decoupling (clear module boundaries, small interfaces) is preferred over complex decoupling (elaborate frameworks, deep hierarchies), abstractions are reviewed for fit over time and refactored or removed when they no longer fit.
- [ ] Decoupling refactorings are done incrementally (one boundary at a time), with characterization tests in place before the refactoring, using well-defined refactoring patterns mechanically, with frequent small commits.
- [ ] The dependency graph is acyclic and aligned with the architecture (dependencies point inward toward the domain core, infrastructure on the outside depending inward), and this structure is verified (dependency analysis tooling, architecture tests) rather than assumed.
- [ ] The decoupling has achieved its goal: a change within a domain stays within the module (no shotgun surgery), modules can be tested in isolation (fast, deterministic unit tests), and modules can be understood independently (no need to read the whole codebase to understand one module) — the measures of successful decoupling.
