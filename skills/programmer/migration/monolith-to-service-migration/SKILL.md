---
name: monolith_to_service_migration.md
description: Use when the agent is decomposing a monolith into services or microservices, extracting a bounded context or service boundary, splitting a shared database into a database-per-service, applying the strangler pattern to extract services, transferring data ownership between services, deciding whether to split a monolith at all, sizing service boundaries, or diagnosing a distributed monolith where services are coupled through a shared database or synchronous call chains.
---

# Monolith To Service Migration

Decomposing a monolith into services is not primarily a code-splitting problem; it is a problem of finding the right boundaries and then paying the cost of those boundaries honestly. A monolith's great advantage is that calls are in-process, transactions are local, and refactoring across modules is cheap. The moment you split a boundary into a service, every cross-boundary call becomes a network call, every shared transaction becomes a distributed-transaction problem, and every schema change that used to be a join becomes data that must be synchronized between independent stores. Services deliver their benefits — independent deployment, independent scaling, team ownership, technology diversity — only when the boundaries align with real seams in the domain. When they do not, the result is the well-known failure mode of the distributed monolith: you have paid all the costs of distribution (latency, failure, operational complexity) while retaining all the coupling of the monolith, because the services cannot move independently of each other.

Agents tend to approach a monolith split as an extraction exercise: pick a module, pull it into its own service, repeat. This ignores the judgment that determines whether the split helps or hurts — whether the boundary is a real domain seam or an arbitrary one, whether the data can actually be separated or is deeply joined, whether the team structure will own the resulting services, and whether the monolith perhaps should not be split at all. The judgment problem is to identify boundaries that are load-bearing seams (not just convenient module edges), to extract them with the strangler pattern so the split is incremental and reversible, to transfer data ownership deliberately rather than leaving services coupled through a shared database, and to recognize when the coupling that remains makes the split a net cost rather than a benefit.

This is distinct from framework-and-platform-migration, which concerns replacing a load-bearing technology while preserving the system's shape. Here the concern is reshaping the system's decomposition itself: where the boundaries go, how data ownership transfers, and how to avoid recreating the monolith's coupling across a network.

## Core Rules

### Find Boundaries At Real Domain Seams, Not Module Edges

The success of a service decomposition is determined almost entirely by where the boundaries fall, and the most common failure is placing them at the wrong seams. A boundary is good when the interactions across it are sparse, coarse-grained, and conceptually meaningful — when the domain genuinely separates into contexts that change and are owned independently. A boundary is bad when it cuts through a dense, fine-grained, constantly-changing interaction, because that interaction does not disappear in the split; it becomes a high-volume network conversation with all the latency, failure, and consistency problems distribution introduces.

Identify boundaries by domain characteristics:

- **Bounded contexts** — group together what changes together and is understood together in the domain. A boundary that aligns with a ubiquitous-language seam (orders, billing, inventory, identity) tends to be stable; one drawn across such a seam tends to churn.
- **Data cohesion** — the strongest signal of a good boundary is whether the data on each side is loosely coupled. If two concerns share tables that are constantly joined, splitting them forces you to synchronize that joined data across a network, which is where distributed monoliths are born.
- **Interaction density and granularity** — prefer boundaries where cross-boundary calls are rare and coarse. A boundary that requires many fine-grained synchronous calls per operation is a boundary that will be slow and fragile.

When in doubt, the cheaper experiment is often to refactor toward cleaner module boundaries inside the monolith first, and observe whether the domain actually separates cleanly, before paying the cost of extracting it as a service.

### Extract Incrementally With The Strangler Pattern And Transfer Data Ownership Deliberately

A service extraction is a migration of code and of data ownership, and both should happen incrementally so that each step is verifiable and reversible. The strangler pattern applies here as in any migration: route one capability at a time from the monolith to the new service, verify equivalence, and expand, rather than splitting the system in a big-bang rewrite that can only be judged after the fact.

Extraction discipline:

- **Extract one bounded context at a time**, starting with one that is genuinely separable (loose data coupling, sparse interactions), to validate the extraction machinery and the boundary choice before tackling harder ones.
- **Transfer data ownership, do not just copy it.** The defining property of a real service is that it owns its data and no other service reads or writes that store directly. A "service" that shares the monolith's database is not a service; it is a module with a network address. Plan the data ownership transfer: move the tables to the service's store, replace direct access with an API, and sever the shared-database coupling.
- **Replace joins with deliberate integration.** Where the monolith joined tables that now live in different stores, decide explicitly how the relationship is maintained — an API call, a replicated read model, an event-driven projection — and accept the consistency and latency tradeoffs that choice implies, rather than assuming the join can be preserved transparently.
- **Keep the monolith and service consistent during transition**, with the same dual-write, event, or reconciliation patterns used in any data migration, until the service is the sole owner and the monolith no longer touches that data.

Data ownership transfer is the part most often skipped, and skipping it is what produces services that are coupled to the monolith through the database indefinitely.

### Avoid The Distributed Monolith — Sever Coupling, Do Not Just Move It

The distributed monolith is the failure state where services have been extracted but remain coupled — through a shared database, through synchronous call chains, or through deployment dependencies — such that they cannot change or deploy independently. It is strictly worse than the original monolith, because it has the coupling of a monolith and the operational cost of a distributed system, with none of the benefits of either. The purpose of decomposition is independent movement; if the services cannot move independently, the decomposition has not succeeded.

Recognize and sever the coupling modes:

- **Shared database** — the most common coupling. If multiple services read or write the same tables, a schema change in one breaks the others, and there is no ownership. Each service must own its data exclusively; cross-service data needs go through an API or an event, not a shared table.
- **Synchronous call chains** — if service A synchronously calls B which calls C to complete a single user request, then A, B, and C must all be available and versioned together, and a failure or slowness in C breaks A. Prefer asynchronous integration (events, queues) for coupling that does not need to be synchronous, and keep synchronous dependencies shallow and few.
- **Deployment and release coupling** — if services must be deployed in a fixed order or version-locked to work, they are not independent. Genuine services can be deployed and rolled back without coordinating with every other service.
- **Shared libraries that dictate contract** — a shared client library that all services must upgrade together reintroduces lockstep coupling through code. Version contracts through APIs and tolerate multiple client versions.

A decomposition is succeeding when the services can change, deploy, and fail independently. If they cannot, the remaining coupling is the work that is unfinished.

### Align Boundaries With Team Ownership (Conway's Law)

Service boundaries that do not match team boundaries do not survive. Conway's Law is not a curiosity; it is the observation that the communication structure of the organization determines the architecture the system converges to. A service boundary drawn between two parts of the same team will be eroded over time, because the team has every incentive to optimize across it (shared code, shared schema, direct calls) and no organizational friction to stop them. A boundary that matches a team boundary is reinforced, because the team owns the service end to end and has both the authority and the incentive to maintain it.

Align decomposition with organization:

- **A service should be owned by one team**, and a team should be able to own the service's code, data, deployment, and on-call without crossing organizational lines for routine work.
- **If the desired service boundary crosses team boundaries, either move the boundary or move the team.** Forcing a boundary the organization does not support produces a service no one fully owns.
- **Size services to team capacity.** A service finer-grained than a team can own is a service that will be neglected; the team boundary is a real constraint on viable service size.

Ignoring the organizational dimension produces architectures that look right on a diagram and erode in practice, as the team structure pulls the system back toward its own shape.

### Recognize When The Monolith Should Not Be Split

Not every monolith needs to become services, and a decomposition undertaken for fashion rather than need is a decomposition that incurs cost without benefit. The monolith is a legitimate, often superior, architecture for systems that are small, owned by one team, deployed as a unit, and not straining the limits that services solve. Splitting such a system introduces distributed-systems complexity (network failure, consistency, operational overhead) to solve problems the system does not have.

Consider whether the drivers for decomposition are real:

- **Scale or deployment pressure** — if independent scaling or deployment of specific capabilities is a real, felt need, decomposition has a clear driver. If everything scales and deploys fine together, the driver is absent.
- **Team boundaries** — if multiple teams need to own and move parts of the system independently, services align ownership. If one team owns it all, the monolith's lower coordination cost wins.
- **Technology or resilience isolation** — if a capability needs a different technology stack or must be isolated for failure, extraction is justified. Absent that, the monolith's in-process simplicity is an advantage.

A decomposition without a real driver produces a distributed monolith by default. The honest assessment of whether to split is itself part of the judgment, and "the monolith is fine" is often the correct answer.

## Common Traps

### Splitting At Module Edges Instead Of Domain Seams

A boundary drawn across a dense, fine-grained, constantly-changing interaction turns that interaction into a high-volume, fragile network conversation. Find boundaries at real domain seams with sparse, coarse interactions; refactor toward cleaner internal boundaries first if the seam is unclear.

### Extracting Services That Share The Database

A service that reads or writes the monolith's tables is not a service; it is a coupled module with a network address. Transfer data ownership so each service owns its store exclusively, and replace shared-table access with APIs or events.

### The Distributed Monolith: All Cost, No Independence

Services coupled through a shared database, synchronous chains, or deployment lockstep have the coupling of a monolith and the cost of distribution. A decomposition is succeeding only when services can change, deploy, and fail independently; sever the coupling rather than relocating it.

### Boundaries That Ignore Team Ownership

A service boundary that does not match a team boundary erodes over time, because the organization has no friction to maintain it. Align boundaries with team ownership, or move the team to match the boundary.

### Splitting A Monolith That Does Not Need Splitting

A decomposition undertaken without a real driver (scale, deployment, team, or isolation pressure) incurs distributed-systems cost to solve problems the system does not have. Recognize when the monolith is the correct architecture and do not split for fashion.

## Self-Check

- [ ] Service boundaries align with real domain seams (bounded contexts, data cohesion, sparse and coarse cross-boundary interactions), not arbitrary module edges, and ambiguous seams were validated by refactoring internal boundaries before extraction.
- [ ] Each extracted service owns its data exclusively — no other service reads or writes its tables directly — and data ownership was transferred deliberately, with shared-table access replaced by APIs or events rather than left in place.
- [ ] Cross-service relationships that were joins in the monolith were converted to a deliberate integration mechanism (API, replicated read model, event projection) with consciously accepted consistency and latency tradeoffs, not transparently assumed.
- [ ] The decomposition avoids the distributed monolith: no shared database, no deep synchronous call chains, no deployment or version lockstep, and services can change, deploy, and fail independently.
- [ ] Boundaries align with team ownership — each service is owned by one team end to end, and no desired boundary crosses organizational lines without the team structure being adjusted to support it.
- [ ] The decision to decompose was driven by a real need (scale, deployment pressure, team ownership, technology or failure isolation), and the alternative of keeping the monolith was honestly considered rather than assumed away.
- [ ] Extraction proceeds incrementally via the strangler pattern, one bounded context at a time, with each extraction verified for equivalence before the next — not a big-bang rewrite.
- [ ] No extracted service is practically still coupled to the monolith (shared schema, synchronous dependency, shared deployment) in a way that means it cannot move independently.
