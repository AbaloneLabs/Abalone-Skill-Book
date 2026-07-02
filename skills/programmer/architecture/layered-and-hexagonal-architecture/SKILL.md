---
name: layered_and_hexagonal_architecture.md
description: Use when the agent is choosing an architectural style for a service or application, deciding between layered (n-tier), hexagonal (ports and adapters), clean, or onion architecture, placing code into presentation, application, domain, or infrastructure layers, designing how data flows across architectural boundaries, deciding whether to introduce a port or adapter, evaluating whether an architecture is over-engineered for the problem, or reviewing why business logic has leaked into controllers or infrastructure. Also covers dependency direction across layers, the active record problem, anemic versus rich domain models at the architectural level, read-side versus write-side separation, and the tradeoff between architectural rigor and delivery speed for simple systems.
---

# Layered And Hexagonal Architecture

An architectural pattern is a bet about which forces will dominate the system's life. Layered architecture bets that concerns separate cleanly by technical responsibility. Hexagonal (ports and adapters) bets that the domain is the center and every external concern is a swappable adapter. Clean and onion architectures bet the same with different vocabulary. None of these is universally correct; each is a structure that pays off when the force it anticipates is real and imposes ceremony when it is not. The judgment problem is not "which architecture is best" but "which forces are actually present in this system, and does the pattern's cost buy protection against them."

Agents tend to apply architectural patterns by shape rather than by force. They draw the hexagon, create folders for every adapter, and wrap every concern in a port, because the pattern says to. The result is a system where a one-table CRUD service carries the ceremony of an enterprise domain — five layers, a dozen interfaces, and a trace through four files to understand a single field update. The reverse failure is also real: skipping all structure because the app "is simple," embedding business rules in controllers and SQL, and then discovering six months later that the rules are now unreachable for testing and tangled with three different transports. This skill exists to match the architecture to the real force, not to impose a pattern as a virtue.

## Core Rules

### Name The Force Before Choosing The Shape

Before adopting a layered or hexagonal style, name the force that makes the structure worth its cost. The legitimate forces are specific:

- **Multiple transports or frontends.** The same business operation must be reachable from HTTP, a CLI, a message consumer, a job runner, or a test harness. A domain core with adapters per transport earns its keep here.
- **Swappable or evolving infrastructure.** The database, the message broker, the payment provider, or the cloud SDK may be replaced, and the cost of replacement must not reach the domain.
- **Rich, shared, long-lived business rules.** The domain has invariants and state machines reused across operations and transports, where scattering rules into handlers would duplicate and drift them.
- **Independent testability of the core.** The domain must be testable without a database, a server, or a network, because those are slow or unavailable in the fast feedback loop.

If none of these forces is real, a simpler structure — handlers that call the database directly, or a thin service layer over an ORM — is usually the cheaper choice. Do not adopt hexagonal architecture to demonstrate rigor; adopt it when the force it addresses is present and named.

### Decide Layer Responsibilities Explicitly And Write Them Down

Whether you choose layered or hexagonal, the value comes from a written contract about what each layer may know and import. Without that contract, "layers" degrade into whatever the last developer felt like, and the architecture becomes advisory.

A durable contract for a domain-centric architecture:

- **Presentation / driving adapters** (controllers, CLI, job handlers, consumers) own transport, serialization, and response shaping. They know nothing of persistence or domain rules. They translate external requests into application calls and map results back.
- **Application / use case** owns the orchestration of one operation: transaction scope, cross-aggregate coordination, sequencing of domain and port calls. It depends on the domain and on ports, never on concrete adapters.
- **Domain** owns the business rules, invariants, and state transitions. It depends on nothing external — no framework, no ORM base class, no cloud type, no transport.
- **Infrastructure / driven adapters** implement the ports: databases, queues, providers, filesystems, clocks. They depend on the domain's abstractions and on the concrete external thing.

Write this contract down where the team can see it, and enforce it with import rules where the toolchain allows. A layer whose allowed dependencies are undefined will accumulate the wrong ones.

### Make The Domain The Center, Not A Folder

In hexagonal and clean architecture, the domain is the center: it defines the ports it needs, and everything else adapts to it. The common failure is to create a `domain` folder but let it depend on infrastructure types — an ORM base class, a framework annotation, a serialization attribute, a cloud SDK. The folder is named "domain" but the dependency direction still points outward, so the "center" is coupled to the edge.

The test of a real domain center is whether the domain package compiles and tests with nothing but the language and its own types. If instantiating a domain entity requires a database session, or a domain method returns a framework response object, the center is not independent. Keep the domain dependent only on language primitives, its own types, and the ports it declares. Mapping between domain objects and storage or transport shapes belongs in adapters, not in the domain.

### Design Data Flow Across Boundaries Deliberately

Data that crosses an architectural boundary is part of the contract, and passing the wrong shape across it is how layers leak into each other. Decide, for each boundary, what shape crosses and who owns it.

- **Commands and results, not entities.** A driving adapter should pass a command object (a plain data structure describing the intent) into the application layer, and receive a result object, not a live domain entity it can mutate or a storage row it can inspect.
- **Domain objects stay inside.** Domain entities with behavior should not escape to the presentation layer, where they would be serialized, partially mutated, or coupled to a transport schema. Project them to DTOs at the boundary.
- **Storage shapes stay in infrastructure.** ORM entities, document records, and table rows belong to the adapter. The application layer should never receive or return a storage object; the adapter maps between storage and domain.

A boundary where an ORM entity is returned to a controller, or a domain entity is serialized directly to JSON, has leaked. The leak is small at first and structural once clients depend on it.

### Separate The Read Path From The Write Path When They Diverge

Many systems have read needs (dashboards, lists, search, reports) that do not fit the write-side domain model. Forcing reads through the same entities, repositories, and ports produces N+1 queries, anemic query methods, and a domain model bloated with projection concerns. This is the motivation behind CQRS-style separation at the architectural level.

When read and write needs diverge:

- Keep the write path through the domain and its invariants; writes are where correctness matters.
- Allow a separate read path: query services that read from a read-optimized view, a projection, or a dedicated read port, returning DTOs directly without loading domain entities.
- Do not let the read path bypass authorization or consistency rules; separation of shape is not separation of safety.

This is not a license to skip the domain on writes. It is a recognition that reads often have different scale, shape, and consistency needs, and forcing them through the write model punishes both.

### Match The Architecture's Weight To The System's Complexity

Architecture is a spectrum, not a binary. A system with one transport, one store, and simple CRUD rules does not need ports, adapters, and a use-case layer; a thin service over the ORM is honest and fast. A system with rich domain rules, multiple transports, and swappable infrastructure earns the full structure. Most real systems are mixed: a few rich cores surrounded by simple CRUD peripheries.

Apply structure where the force is real and leave the rest direct:

- A payment or ordering core with invariants, state machines, and multiple transports earns hexagonal treatment.
- A user-profile CRUD endpoint with one transport and one store does not; a handler calling the database is clearer and cheaper.
- Mixing the two in one system is fine. Forcing the whole system to the heaviest pattern penalizes the simple parts, and forcing it all to the lightest pattern strands the complex parts.

The error is applying one weight uniformly. Match the structure to the local force.

### Watch For The Active Record Trap

A common pattern in rapid-development frameworks is the active record: a domain class that is also its own persistence mechanism (`user.save()`, `user.update()`). This collapses the domain and infrastructure layers into one type. It is fast to start and expensive to evolve: the domain cannot be tested without a database, business rules accrete onto persistence objects, and swapping the store means rewriting the domain.

Active record is acceptable for genuinely simple CRUD where no rich rules exist. It becomes a trap the moment rules appear, because the rules now live on a persistence-coupled object and cannot be reasoned about or tested in isolation. When rules emerge, extract them into a domain service or a rich domain model and let the active record become a storage adapter, not the domain.

## Common Traps

### Architecture By Shape, Not By Force

Drawing the hexagon and creating the folders because the pattern says to, in a system with one transport, one store, and trivial rules. The ceremony is paid every day; the force it addresses never arrives. Adopt the structure when the force is named and present.

### The Domain Folder That Depends Outward

Naming a package `domain` but letting it import an ORM base class, framework annotations, or a cloud SDK. The folder label says "center" but the dependency direction points outward, so the domain is coupled to the edge. The domain must compile and test with only the language and its own types.

### Returning Storage Or Domain Objects Across The Boundary

A controller receiving an ORM entity and serializing it, or an application service returning a live domain entity that the controller mutates. The boundary has leaked, and the transport is now coupled to storage or domain internals. Pass commands in and DTOs out.

### Forcing Reads Through The Write Model

Routing every list, dashboard, and report through domain entities and repositories, producing N+1 queries and a domain model full of projection methods. Separate the read path when read and write needs diverge; do not punish writes by bloating the model with reads.

### Over-Abstracting A Simple CRUD System

Wrapping a one-table, one-transport, no-rules service in ports, adapters, use cases, and domain services. Every change traverses five layers to do a field update. The architecture's cost exceeds the coupling it removes. Use a thin service until a real force appears.

### Collapsing Layers Into A God Service

A service layer that holds validation, business rules, persistence calls, and response formatting, with the domain as a bag of getters. This looks layered but behaves as a single tangled module that cannot be tested or reused without the whole stack. Either commit to a real domain model or admit it is a transaction script and drop the ceremony.

### Treating The Pattern As Permanent

Adopting hexagonal architecture as a one-time decision and then refusing to simplify a part of the system that turned out to be simple, or refusing to add structure to a part that grew complex. Architecture should respond to the force as it evolves; a part that gained rich rules earns more structure, and a part that stayed CRUD can be simplified.

## Self-Check

- [ ] The architectural style was chosen because a named force is present (multiple transports, swappable infrastructure, rich shared rules, or independent core testability), not because the pattern is considered best practice.
- [ ] The responsibilities of each layer are written down, including what each may import and what it must not know, and the contract is enforced by tooling where possible.
- [ ] The domain compiles and tests with only the language and its own types; it imports no framework, ORM, transport, or cloud type, and ports are declared in the domain and implemented by adapters.
- [ ] Data crossing each boundary is a deliberate shape (commands in, DTOs out); no storage object or live domain entity escapes to the presentation layer.
- [ ] Read paths that diverge from the write model use a separate query model or read port rather than bloating the domain with projections, without bypassing authorization or consistency.
- [ ] The architecture's weight matches local complexity: rich cores receive full structure, simple CRUD peripheries stay direct, and one weight is not forced uniformly across the system.
- [ ] No active record with emergent business rules remains as the domain; rules were extracted into a domain service or rich model, with the record becoming a storage adapter.
- [ ] The domain is not anemic-with-a-god-service nor over-ceremonialized; the transaction-script versus domain-model choice is honest per operation.
- [ ] The architecture is treated as responsive to evolving forces, not permanent: parts that gained complexity gained structure, and parts that stayed simple were not over-built.
- [ ] The cost of the chosen structure (layers, interfaces, mapping) was weighed against the coupling or risk it removes, and the structure is not pure ceremony for the problem at hand.
