---
name: monolith_vs_microservices.md
description: Use when the agent is deciding whether to split a system into services, designing service boundaries, evaluating a monolith versus microservices tradeoff, avoiding a distributed monolith, choosing where data and ownership live across services, planning inter-service communication, or deciding when and whether to decompose a system. Also covers service granularity, shared databases, synchronous versus asynchronous coupling, eventual consistency boundaries, and decomposition driven by scale, team, or release pressure.
---

# Monolith Versus Microservices

The choice between a monolith and microservices is not a choice between bad and good architecture. It is a choice about which kind of complexity a system is willing to carry. A monolith concentrates complexity in one process and one deployment; microservices distribute complexity across network, data, and operational boundaries. The judgment problem is deciding, for a specific system under specific pressures, which complexity is cheaper to bear — and just as importantly, where to draw the boundaries once the choice is made.

Agents tend to recommend decomposition too readily. "Microservices" sounds modern, and splitting a system feels like progress. But a bad split produces a distributed monolith: all the costs of a network boundary with none of the independence, where every change still crosses services and every failure still cascades. The reverse failure is also real — keeping a system monolithic long after it has outgrown the team, the release cadence, or the scaling pressure that a boundary would relieve. This skill exists to make the decomposition decision deliberate and the boundaries honest.

## Core Rules

### Name The Pressure Before Choosing The Shape

Do not start from "should this be microservices." Start from the pressure that is making the current shape painful. The legitimate pressures are specific:

- **Team scale and ownership** — multiple teams are stepping on each other, deploy conflicts are frequent, and ownership of a coherent area is unclear.
- **Release cadence** — one part of the system needs to ship daily while another must change slowly and stably.
- **Scaling asymmetry** — one workload has very different resource, latency, or availability needs from the rest.
- **Failure isolation** — a failure in one area should not take down unrelated areas.
- **Technology divergence** — one area genuinely needs a different runtime, language, or library stack.
- **Organizational or regulatory boundaries** — data or process ownership must be separated for compliance or business reasons.

If none of these pressures is real, a well-structured modular monolith is usually the cheaper choice. If one or more is real, name which one, because it determines where the boundary should go.

### Draw Boundaries Around Bounded Contexts, Not Around Tables

A service boundary should follow a domain boundary — a coherent capability that can be described in one sentence and that changes for one set of reasons. Boundaries drawn around technical layers (a "data service" that other services call), around single tables, or around nouns without behavior produce services that are coupled by their very purpose.

For each candidate boundary, ask:

- Does this capability have its own vocabulary, rules, and lifecycle?
- Could this team own this capability end to end, including its data?
- Does this capability change for different reasons than its neighbors?
- Is there a stable contract at the edge, or would the boundary force frequent cross-service changes?

If the boundary does not align with a bounded context, the decomposition will not buy independence; it will buy coordination overhead.

### Make Each Service Own Its Data

The single strongest signal of a real service boundary is data ownership: one service writes to its store, and no other service reads or writes that store directly. Shared databases are the most common way to create a distributed monolith, because every schema change becomes a cross-service negotiation and every query can couple services that were meant to be independent.

For each piece of data, decide:

- Which service is the single source of truth?
- Which services need a read view, and how do they get it (API call, replicated read model, event projection)?
- What is the consistency expectation — strong, read-your-writes, or eventual?
- What happens when the source of truth is unavailable?

If two services share a writable database, they are one service with a network boundary between them. Treat that as a problem to fix, not an architecture to keep.

### Decide Synchronous Versus Asynchronous Coupling Deliberately

Every cross-service call is a coupling decision. Synchronous calls (request/response) couple availability and latency: the caller fails or slows when the callee does. Asynchronous calls (events, messages, queues) couple consistency and causality: the caller proceeds without confirmation, and the system must handle out-of-order, delayed, or duplicate effects.

Match the coupling style to the requirement:

- **Synchronous** when the caller cannot proceed without the result (auth check, price lookup at checkout, validation that must be immediate).
- **Asynchronous** when the callee's work can happen later, when the producer should not depend on the consumer's availability, or when one event must reach multiple independent consumers.
- **Saga / compensating workflow** when a multi-service operation must either complete or visibly undo, and there is no distributed transaction.

Avoid synchronous chains that must all succeed for a user-facing operation unless you can afford their combined failure probability and latency. Avoid event-only flows for operations where the user must see an immediate, consistent result.

### Cost The Distributed Tradeoffs Honestly

A network boundary is not free. Before decomposing, account for the costs that a monolith avoids:

- **Network failure modes** — timeouts, retries, partial failures, duplicate delivery, and reordered messages all become possible and must be handled.
- **Operational surface** — more services mean more deployments, more monitoring, more on-call rotations, more tracing and correlation infrastructure.
- **Data consistency** — cross-service transactions are hard; eventual consistency, compensation, and reconciliation become first-class concerns.
- **Latency** — an in-process call becomes a network hop, often multiplied by serialization and auth.
- **Debugging** — a single stack trace becomes a trace across services, requiring distributed tracing to reconstruct.
- **Schema evolution** — contracts between services must be versioned and evolved without forcing simultaneous deploys.

If the team cannot invest in observability, deployment automation, and contract management, decomposition will produce a system that is harder to operate than the monolith it replaced.

### Prefer Modular Monolith Until A Boundary Earns Its Cost

A modular monolith — one deployment with strictly enforced internal module boundaries — captures most of the design benefits of services (clear ownership, separable concerns, testable units) without the operational cost of a network boundary. It also keeps the option open: a well-modularized monolith can be split later at a real seam, while a tangled monolith cannot be split at all.

Decompose when a specific pressure (team, release, scale, failure isolation) makes the module boundary insufficient. Do not decompose because the team read that they should.

### Plan The Decomposition Path, Not Just The End State

Moving from monolith to services is a migration, not a flip. A common, durable sequence is:

1. Enforce module boundaries inside the monolith first; if you cannot do this in-process, you cannot do it across a network.
2. Extract the boundary with the strongest pressure and the cleanest seam.
3. Establish the inter-service contract, observability, and deployment story for that one extraction before extracting the next.
4. Validate that the extracted service is genuinely independent — that it deploys, fails, and scales on its own.
5. Repeat only where the pressure is real.

Each extraction should be justifiable on its own. A big-bang decomposition that splits everything at once usually produces a distributed monolith and an operational crisis.

### Decide What Not To Decompose

Not everything should be a service even in a decomposed system. Cross-cutting concerns (auth, configuration, observability), shared domain kernels (money types, address models, identifier formats), and tightly co-changing features often belong together. Decomposing them creates coordination cost without independence.

For each candidate, ask whether independence is real. If two areas must change together for most features, keeping them together is the cheaper choice, even if they look like separate nouns.

## Common Traps

### The Distributed Monolith

Services that share a database, that must deploy together, that call each other synchronously on every request, or that change in lockstep are a monolith with network calls added. The organization pays the operational cost of microservices and the coupling cost of a monolith. The fix is either to make the boundary real (separate data, asynchronous contracts, independent deploys) or to merge the services back.

### Decomposing For "Scalability" Without Measuring

A common justification is "we need to scale." But most systems scale vertically and within a module long before they need a network boundary. Decomposition for scale only pays off when the scaling pressure is asymmetric — one workload genuinely needs different resources than the rest. Decomposing a uniformly-scaling system adds cost without relief.

### Shared Database As A "Temporary" Shortcut

Letting two services read the same database to avoid building an API is the fastest way to couple them permanently. Once the reads exist, schema changes break both services, and the "temporary" shortcut becomes a constraint that blocks real independence. Either own the data in one place and expose an API, or accept that the services are one service.

### Synchronous Chains That Must All Succeed

A user request that calls service A, which calls B, which calls C, where all three must succeed, has the combined failure probability and latency of all three, plus the network. Such chains often grow accidentally as services are added. Prefer asynchronous composition, caching, or collapsing the chain where the synchronous dependency is not truly required.

### Event-Driven Coupling Disguised As Decoupling

Publishing an event feels decoupled, but if downstream services depend on the event's order, timing, or payload shape to function, they are coupled — just less visibly. Event-driven systems need explicit contracts for event schema, ordering guarantees, idempotency, and failure handling. Without those, "decoupled" becomes "impossible to reason about."

### Decomposing By Noun Instead Of By Capability

Splitting "users," "orders," "products," and "payments" into services sounds clean but often produces services that are thin data wrappers and that must coordinate on every real operation. Capabilities ("checkout," "fulfillment," "identity") that bundle related behavior usually make better boundaries than isolated nouns.

### Keeping Services That Were Never Independent

A service extracted years ago that still deploys only with its parent, shares its data, and is owned by the same team carries the cost of a boundary without the benefit. Merging it back is a legitimate architectural decision, not a failure. Independence is the test; if it is absent, the boundary is debt.

## Self-Check

- [ ] The decomposition is justified by a named, specific pressure (team ownership, release cadence, scaling asymmetry, failure isolation, technology divergence, or regulatory boundary), not by a general preference for microservices.
- [ ] Each service boundary follows a bounded context with its own vocabulary, rules, and lifecycle, not a technical layer or a single noun.
- [ ] Each data store has a single owning service; no two services write to the same database, and shared reads go through a deliberate API or replicated read model.
- [ ] Synchronous versus asynchronous coupling is chosen per interaction based on whether the caller can proceed without the result, and synchronous chains are minimized.
- [ ] The distributed tradeoffs (network failures, operational surface, consistency, latency, debugging, schema evolution) were costed honestly, and the team can invest in observability, deployment, and contract management.
- [ ] A modular monolith was considered and preferred unless a specific pressure makes a network boundary necessary.
- [ ] The decomposition follows a staged path with enforced in-process boundaries first, one extraction at a time, and validation of genuine independence before the next.
- [ ] Cross-cutting concerns, shared kernels, and tightly co-changing features were deliberately kept together rather than decomposed.
- [ ] No service exists that must deploy with another, shares writable data, or changes in lockstep with its neighbor; if such a service exists, it is flagged for re-merging or for making the boundary real.
- [ ] Event-driven interactions have explicit contracts for schema, ordering, idempotency, and failure handling, and are not treated as automatically decoupled.
