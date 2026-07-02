---
name: dependency_inversion_and_coupling.md
description: Use when the agent is fixing a circular dependency, deciding which way a dependency should point, introducing an interface or abstraction to break a coupling, evaluating whether a dependency is stable or volatile, choosing where an abstraction should live, deciding whether to invert a dependency, reviewing whether indirection is justified or excessive, or diagnosing why a small change ripples across many modules. Also covers the dependency inversion principle, stable dependencies principle, stable abstractions principle, acyclic dependencies principle, the cost of unnecessary indirection layers, shared kernel coupling, and the difference between afferent and efferent coupling.
---

# Dependency Inversion And Coupling

Coupling is not a style preference; it is a prediction about how change and failure will propagate. Every dependency a module takes is a statement that "when the thing I depend on changes or breaks, I may have to change or break too." A well-managed dependency graph concentrates volatility in a few places and shields the stable core from it. A poorly managed graph spreads volatility everywhere, so that any change becomes a multi-module event and any failure cascades. The judgment problem is not "minimize dependencies" — a system with no dependencies does nothing — but "point dependencies in the right direction, toward things that change less, and shield things that matter from things that do not."

Agents tend to treat coupling as a local property: "does this module compile against that one?" The harm is structural and delayed. A dependency added for convenience today becomes a constraint that blocks a refactor tomorrow, a cycle that cannot be untangled, or an abstraction layer so thick that every change must traverse four hops to do real work. The reverse failure is also real: refusing to invert a dependency because "it's just one interface" leaves the core coupled to a volatile detail that will force rework every time the detail changes. This skill exists to make each dependency a deliberate decision about direction, stability, and cost.

## Core Rules

### Point Dependencies Toward Stability

The foundational rule of coupling is that dependencies should point toward things that change less often and that more things depend on. This is the stable dependencies principle in concrete terms. A module that many others rely on should itself rely only on things at least as stable; otherwise a change in a volatile dependency ripples through everything that depends on the dependent.

For each dependency, ask:

- How often does the target actually change? A domain concept (money, an order state machine) changes rarely; a framework, a third-party client, or an infrastructure detail changes whenever the vendor or ops team decides.
- How many modules depend on the target? A module depended on by fifty others is stable by pressure — changing it coordinates fifty consumers — so it must not depend on something volatile.
- Does this dependency make the dependent more or less stable? Depending on a volatile library drags the dependent's stability down to that library's.

When a module that should be stable depends on something volatile, invert the dependency: define the abstraction in the stable module, let the volatile thing implement it. The stable core no longer depends on the detail; the detail depends on the core's abstraction. This is the whole point of dependency inversion, and it exists to protect stability, not to add ceremony.

### Put The Abstraction Where The Decision Belongs

Dependency inversion requires an abstraction (interface, trait, protocol). The question that determines whether the inversion is correct is *where that abstraction lives*. The rule: the abstraction belongs to the module that consumes the capability and defines its need, not to the module that provides the implementation.

- A domain module that needs to "record a payment" should define a `PaymentRecorder` port in its own package. The payment provider adapter implements that port and lives in infrastructure. The domain depends on its own abstraction; the adapter depends on the domain's abstraction.
- Putting the interface in the provider's package ("here is the StripePaymentClient interface, the domain uses it") does not invert anything; it makes the domain depend on a provider-shaped abstraction. The direction is unchanged; only the syntax moved.

When you review an inversion, check where the interface is declared. If it mirrors the provider's API or lives in the provider's namespace, the inversion is cosmetic. A real inversion means the consumer owns the contract and the provider conforms to it.

### Keep The Dependency Graph Acyclic At Runtime

A cycle means two or more modules cannot be understood, tested, built, or replaced independently, because each requires the other to exist. Cycles are the most expensive form of coupling. The acyclic dependencies principle is not about import graphs alone; a cycle can hide in runtime wiring.

Check for cycles in every direction:

- **Import / build graph.** Does A import B and B import A? Most compilers catch this, but some languages and some module systems tolerate it.
- **Event and callback cycles.** A publishes an event B listens to; B, in handling it, calls back into A. The import graph is clean; the runtime cycle is real.
- **Shared mutable state and registries.** A registers a handler with a global that B owns; B invokes handlers that reach into A. The registry hides the cycle behind a string key.
- **Configuration and inversion-of-control wiring.** A and B are wired together by a container; neither imports the other, but neither works without the other.

When you find a cycle, do not merge the modules to make it compile. Find the shared responsibility and extract it, or invert one direction through an abstraction owned by the stable side. Merging removes the cycle but also removes the boundary and concentrates two change cycles in one module.

### Measure The Cost Of Indirection Against The Coupling It Removes

Dependency inversion and abstraction are not free. Each interface, adapter, and layer of indirection adds types to maintain, hops to trace, and cognitive load to read a single behavior. The cost is justified when it removes real coupling; it is pure overhead when it removes coupling that did not exist or would never have mattered.

Before adding an abstraction layer, verify the coupling it addresses is real:

- Is there more than one implementation, or a realistic second one (a test double counts only if the double preserves real behavior)?
- Does the thing being abstracted actually change independently of its consumer?
- Would the consumer be materially harder to test, replace, or reason about without the abstraction?

If the answer is no, the indirection is speculative. A single-implementation interface "for flexibility," a repository wrapping one ORM with no second store planned, or a port whose shape is identical to the provider's API are all cases where the indirection costs more than the coupling it removes. Prefer direct, readable code until a real second variant or a real volatility justifies the seam. You can always add the abstraction when the need appears; removing a layer of speculative indirection is harder because consumers accrete around it.

### Distinguish Stable Coupling From Volatile Coupling

Not all coupling is equal. Coupling to a stable, well-understood abstraction (a language primitive, a standard library collection, a domain value object) is cheap: it rarely forces change. Coupling to a volatile concrete thing (a specific framework version, a raw third-party client, a particular database feature) is expensive: it forces change whenever the thing changes.

When evaluating a dependency, classify it:

- **Stable coupling** — language types, standard library, your own stable domain kernel. Tolerate freely; do not abstract it away.
- **Volatile coupling** — frameworks, ORMs, cloud SDKs, third-party clients, specific infrastructure. Invert or isolate it behind a seam so the volatile thing lives at the edge, not the core.
- **Shared kernel coupling** — a small common module (money types, identifiers, base exceptions) that many modules depend on. Keep it tiny, stable, and additive-only, because every change to it is a change for every consumer.

A system that abstracts stable coupling and leaves volatile coupling embedded in its core has inverted the work: it pays indirection cost where there is no benefit and remains exposed where the risk is real.

### Make Coupling Visible And Enforced

A dependency that exists only by convention will be violated under deadline pressure. Make the coupling graph visible and, where possible, enforced by tooling.

- Use package visibility, access modifiers, and module systems to prevent reaching past the public surface.
- Use dependency lint rules or import allowlists to forbid dependencies that cross a forbidden direction (infrastructure importing domain, one feature importing another's internals).
- Periodically generate and review the dependency graph. A graph that grows denser and more cyclic over time is accumulating structural debt even if every individual change felt justified.

A convention that is not enforced is a boundary that will be crossed. Prefer an enforced rule over a documented one, and a documented one over an assumed one.

## Common Traps

### Inverting Every Dependency Out Of Habit

Adding an interface in front of every class or module "for testability and flexibility" produces a system where tracing one behavior requires hopping through several abstractions, most of which have a single implementation. The indirection is real cost; the flexibility is hypothetical. Invert dependencies where the volatility or the second implementation is real, not as a default.

### The Abstraction That Mirrors The Provider

Defining a `PaymentGateway` interface whose methods are `createCharge`, `captureCharge`, `refundCharge` — a direct mirror of the provider's REST API — and calling it dependency inversion. The domain now depends on a provider-shaped contract. A real inversion expresses the domain's need ("record that a payout was initiated") in domain terms, and the adapter translates.

### Breaking A Cycle By Merging

When A and B are circularly coupled, merging them into one module makes the cycle disappear but removes the boundary and concentrates both change cycles in one place. The merged module is now harder to reason about than either was alone. First try extracting the shared responsibility or inverting one direction; merge only if the two were never truly separate.

### Treating The Import Graph As The Whole Dependency Story

Concluding "there is no cycle" because the import graph is acyclic, while runtime wiring, events, callbacks, or shared registries create a cycle the compiler cannot see. Always check the runtime dependency, especially in event-driven or IoC-container systems.

### Abstracting Stable Coupling, Embedding Volatile Coupling

Wrapping language primitives or stable domain types in interfaces "for symmetry," while the domain imports a concrete ORM and a cloud SDK directly. The indirection is spent where there is no risk, and the real risk is left unprotected. Direct effort at the volatile boundaries, not the stable core.

### The Shared Kernel That Grew

A `common` or `shared` kernel that starts as money types and identifiers and accumulates utilities, base classes, and cross-cutting concerns until every module depends on it and every change to it is a system-wide event. A shared kernel must stay tiny, stable, and guarded; anything that is not universally needed does not belong there.

### Depending On A Concrete Volatile Thing Because "It Works For Now"

Importing a specific framework or SDK directly into the domain because the abstraction "can be added later." Later, consumers accrete around the concrete dependency, the volatile thing changes, and the now-needed inversion requires touching every consumer. Inverting early at a genuine seam is cheaper than retrofitting one across a codebase that has coupled to the detail.

## Self-Check

- [ ] Dependencies point toward stability: modules depended on by many are themselves dependent only on things at least as stable, and volatile details are inverted behind abstractions owned by the stable side.
- [ ] Each inversion places the abstraction in the module that defines the need (the consumer), not in the provider's package; the abstraction is expressed in the consumer's vocabulary, not a mirror of the provider's API.
- [ ] The dependency graph is acyclic at runtime, including events, callbacks, shared registries, and IoC wiring — not only in the import graph.
- [ ] Every abstraction layer is justified by a real second implementation, real volatility, or a real testability/isolation need; no single-implementation interface exists purely for speculative flexibility.
- [ ] Coupling is classified: stable coupling (language, standard library, domain kernel) is tolerated; volatile coupling (frameworks, SDKs, infrastructure) is isolated at the edge; the shared kernel is tiny and additive-only.
- [ ] The system does not abstract stable coupling while leaving volatile coupling embedded in the core; indirection effort is directed at the real risk.
- [ ] Dependency direction is enforced by tooling (visibility, access modifiers, import rules) where possible, not only by convention.
- [ ] The dependency graph was reviewed recently and is not growing denser or more cyclic; any cycle found was resolved by extraction or inversion, not by merging.
- [ ] No dependency on a volatile concrete thing was deferred as "works for now" in a place where consumers will later accrete around it and make inversion expensive.
- [ ] The shared kernel, if one exists, contains only universally needed, stable types, and changes to it are additive and rare.
