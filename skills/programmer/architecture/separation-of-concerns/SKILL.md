---
name: separation_of_concerns.md
description: Use when the agent is deciding which layer holds a responsibility, separating business logic from infrastructure or presentation, applying dependency inversion, choosing between transaction script and domain model, preventing layer bypass, or reviewing where validation, orchestration, persistence, and policy decisions should live. Also covers ports and adapters, hexagonal and clean architecture, anemic models, fat controllers, and leaky abstractions across layers.
---

# Separation Of Concerns

Separation of concerns is the practice of putting each responsibility in the layer that owns it, so that changing one concern does not force changes in unrelated ones. The hard part is not knowing that layers exist; it is deciding, for each concrete decision, which layer a concern belongs to. Validation, error mapping, transaction boundaries, policy checks, formatting, caching, and orchestration all look like they could live in several places, and the wrong placement rots quietly.

Agents tend to put logic wherever the current framework makes it easiest. A web framework invites business rules into controllers. An ORM invites queries into handlers. A service locator invites infrastructure calls from domain code. The result is a system where every layer knows about every other layer, where tests require a database and an HTTP server, and where a change to a notification channel requires editing the billing module. This skill exists to make those placement decisions deliberate.

## Core Rules

### Name The Layers And What Each One Owns

Before placing a responsibility, agree on the layers and what each is allowed to know. A common, durable shape is:

- **Presentation** — receives external input, returns external output. Owns transport (HTTP, CLI, gRPC), serialization, status codes, and response shaping. Knows nothing about persistence or domain rules.
- **Application / use case** — orchestrates a single user-facing operation. Owns transaction boundaries, cross-cutting coordination, and the sequence of domain and infrastructure calls. Knows the domain contract and the ports it needs, not the concrete infrastructure.
- **Domain** — owns the business rules and invariants that survive regardless of transport or storage. Knows nothing about HTTP, databases, queues, or frameworks.
- **Infrastructure** — implements technical concerns: databases, queues, email providers, external APIs, clocks, filesystems. Depends on abstractions defined by the layer that needs them.

This is one shape, not the only one. The point is to write down, for your system, what each layer is allowed to import, what it must not know, and which concerns it owns. Without that written contract, "separation" becomes whatever the last developer felt like.

### Decide Per Responsibility, Not Per File

Most concerns do not announce which layer they belong to. Decide each one explicitly against the layer contract:

- **Input validation** — format and shape checks belong near the boundary (presentation or application). Semantic and invariant checks belong in the domain. A check like "email is well-formed" is presentation; "an account may have at most three active subscriptions" is domain.
- **Authorization** — transport-level gating ("is this a logged-in user") is presentation or application; policy ("may this user approve a refund over $1000") is domain or a dedicated policy module. Do not split the same authorization decision across layers inconsistently.
- **Transaction boundaries** — the application layer typically owns the unit of work, because it knows the operation's scope. Domain code should not start transactions; infrastructure should not decide commit scope.
- **Error mapping** — domain raises domain errors; the boundary maps them to transport-specific responses. Do not let HTTP status codes leak into the domain, and do not let raw database exceptions reach the client.
- **Caching** — caching is a read-path optimization and usually belongs in infrastructure or a dedicated port, not inside domain rules.
- **Formatting and localization** — presentation. The domain should produce values, not localized strings.

When a concern could live in two layers, prefer the layer that lets the other layer stay free of that concern's dependencies.

### Invert Dependencies At The Seam

The reason infrastructure does not contaminate the domain is dependency inversion: the domain defines a port (interface, trait, abstract type), and infrastructure provides an adapter. The application and domain layers depend on the port, never on the concrete database, queue, or provider.

For each external concern the domain or application needs, ask:

- Is there a port defined in the consuming layer?
- Does the consuming layer import a concrete adapter, or only the port?
- Could the adapter be replaced (test double, different provider, in-memory) without touching the consuming layer?
- Does the port's shape reflect what the domain needs, or does it mirror the provider's API?

A port that mirrors the provider's API is a leaky abstraction. The port should describe the operation in domain terms ("record that a payout was initiated"), not in provider terms ("call the `POST /v1/payouts` endpoint").

### Choose Transaction Script Or Domain Model By Rule Density

Two valid styles coexist, and the choice depends on how much business logic each operation contains.

- **Transaction script** — a procedural function per operation that validates, computes, and persists. Best when each operation is simple, the rules are few, and the value is in correct orchestration rather than rich behavior. Most CRUD and workflow-style services are honestly transaction scripts.
- **Domain model** — persistent objects that encapsulate rules and invariants, with operations expressed as methods on the model. Best when entities have many rules, states, and transitions, and when the same rules apply across multiple operations.

Do not force a domain model onto simple operations; it produces anemic models where objects hold data and services hold all the logic, which is transaction script with extra ceremony. Do not force transaction script onto rich domains; it scatters the same rule across every operation and lets invariants drift.

The two styles can coexist in one system. Use the richer style where rules are dense and shared, and the simpler style where they are not.

### Keep The Domain Free Of Framework And Infrastructure Types

A domain layer that imports a web framework, an ORM base class, a serialization annotation, or a cloud SDK type is no longer separable. The framework now owns part of the domain. Symptoms include: domain entities that cannot be instantiated without a database session, domain methods that return framework response objects, and domain tests that require a running server.

Keep the domain dependent only on language primitives, its own types, and the ports it defines. Annotations and persistence mapping belong in infrastructure or in mapping layers that translate between domain objects and storage rows.

### Make Layer Bypass Visible And Expensive

Layer bypass is when a higher layer reaches past the layer below it: a controller calling the database directly, a domain object importing a queue client, an application service returning an ORM entity that the controller then serializes. Each bypass makes the layering advisory rather than enforced.

Detect bypass by asking, for each cross-layer call:

- Does the caller import a type from a layer it should not know about?
- Does the call skip the layer that owns the concern (e.g., controller calling repository, skipping the use case that owns the transaction)?
- Is the return type a storage object rather than a domain or contract type?

Enforce layering with tooling where possible: import linters, dependency rules, separate packages with restricted visibility. Where tooling is not available, treat every bypass in review as a decision to be justified, not a convenience to be accepted.

### Separate Read Paths From Write Paths When They Diverge

Many systems have read needs (list views, dashboards, reports, search) that do not fit the write-side domain model. Forcing reads through the same entities and repositories produces N+1 queries, anemic query methods, and domain models bloated with projection concerns.

When read and write needs diverge, consider a separate read path: query services that read directly from a read-optimized view, a separate query model, or a dedicated read port. The write path still goes through the domain and its invariants. This is not a license to skip the domain on writes; it is a recognition that reads often have different shape, scale, and consistency needs.

### Treat Cross-Cutting Concerns As Explicit, Not Invisible

Logging, tracing, metrics, authorization, caching, retries, and idempotency are cross-cutting. They are easy to scatter, and scattered they become inconsistent. Decide for each cross-cutting concern where it is applied:

- at the transport boundary (middleware, interceptors),
- through decorators or aspect-oriented composition,
- inside a port adapter,
- or explicitly in the application layer.

Document the choice. A system where logging is "wherever someone added it" will have gaps in observability exactly where failures happen.

## Common Traps

### The Fat Controller

Putting validation, business rules, persistence calls, and response formatting in one handler is the fastest path to working code and the slowest path to change. The handler becomes untestable without the whole stack, the rules cannot be reused by another transport, and the persistence shape leaks into the response. Controllers should be thin: parse, delegate, map the result.

### The Anemic Domain Model With A God Service

When entities are bags of getters and setters and one service holds all logic, the layering looks object-oriented but behaves procedurally. Every new operation duplicates rules that should have lived on the model, and invariants are only as good as the last developer who remembered to check them. Either commit to a real domain model or admit it is a transaction script and stop paying the ceremony cost.

### Domain Code Importing Infrastructure "Just For Logging"

A domain class that imports a logging framework, a configuration client, or a clock "just for one thing" has opened a seam through which every other infrastructure concern will eventually arrive. Inject a port (a logger interface, a clock abstraction) and let infrastructure provide it. The cost is one interface; the benefit is a domain you can reason about and test in isolation.

### Application Layer That Only forwards Calls

If the application layer does nothing but pass arguments from controller to repository, it is not earning its place. Either it owns real orchestration (transaction scope, cross-aggregate coordination, policy checks, event publishing) or it should be removed and the controller should call the domain directly. A pass-through layer adds indirection without adding a boundary.

### Mixing Transport Concerns Into Domain Errors

Throwing an exception that carries an HTTP status, or returning a domain result shaped like a REST response, couples the domain to one transport. When a second transport arrives (CLI, job, message consumer), the domain either gets re-wrapped or starts branching on transport. Domain errors should be domain-meaningful; the boundary maps them to transport.

### One Repository Method Per Screen

Designing repositories or services around UI screens ("getDashboardData", "getUserProfileCard") bakes presentation into the data layer. When the screen changes, the query changes; when a second screen needs overlapping data, a near-duplicate query appears. Design around domain operations and projections, and let the presentation layer compose what it needs.

### Caching Inside Domain Logic

Embedding cache reads and writes inside domain methods makes correctness depend on cache state, makes tests nondeterministic, and makes cache invalidation a domain problem. Keep caching in infrastructure or behind a port so the domain reasons about values, not about whether a value was cached.

## Self-Check

- [ ] The layers and what each is allowed to import are written down, and every placed responsibility was checked against that contract.
- [ ] Business rules and invariants live in the domain; transport, persistence, and provider details do not appear in domain code.
- [ ] Infrastructure is reached only through ports defined by the consuming layer; concrete adapters are not imported by domain or application code.
- [ ] Each operation honestly follows transaction script or domain model style, and the choice matches the rule density of that operation.
- [ ] Transaction boundaries, error mapping, authorization, caching, and cross-cutting concerns each have an explicit owner layer, not a scattered default.
- [ ] No layer bypass exists where a higher layer reaches past the layer that owns a concern (controller-to-repository, domain-to-provider, etc.).
- [ ] Domain entities do not import framework base classes, ORM types, serialization annotations, or cloud SDK types.
- [ ] Ports describe operations in domain terms, not in provider API terms; the abstraction is not a mirror of the vendor.
- [ ] Read paths that diverge from the write model use a deliberate query model or read port rather than bloating the domain with projections.
- [ ] Tests for domain and application logic run without a database, HTTP server, or external provider; only adapter tests need those.
