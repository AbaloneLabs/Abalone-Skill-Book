---
name: event_driven_architecture.md
description: Use when the agent is designing or reviewing an event-driven system at the architecture level, distinguishing events from commands and messages, defining event contracts and schemas, choosing pub/sub topology and routing patterns, deciding between a dumb pipe and smart broker, planning event versioning and evolution, reasoning about loose coupling between autonomous services, or reviewing how producers and consumers should relate through an event as the integration contract. Also covers event naming with past-tense verbs, fat versus thin events with a fetch, the producer-owned event boundary, additive schema evolution, and the architectural implications of ordering and idempotency across an event-driven system.
---

# Event-Driven Architecture

Event-driven architecture is a style of integration in which services communicate by emitting and reacting to facts about things that have already happened, rather than by calling each other directly. The attraction is real — producers and consumers become decoupled, new consumers can be added without touching producers, and the system gains an auditable flow of business activity. But the same decoupling that makes the style attractive is what makes it easy to get wrong. A mislabeled event that is really a command, a fat event that leaks producer internals, a breaking schema change deployed while consumers are still on the old version, or a topology that assumes a smart broker routes by business rule — each of these quietly undermines the decoupling the architecture was supposed to provide.

Agents tend to under-invest here because the surface looks like "publish an event, subscribe to it," which sounds trivial. The harm appears later and diffusely: a consumer that breaks every time the producer renames an internal field, because the event was a serialization of private state; a system where every change to a business process requires coordinated releases across five services, because events carried commands that assumed specific receivers; an audit story that does not actually reconstruct what happened, because events were thin to the point of being meaningless without the producer's live database. The judgment problem is deciding, for each integration, what kind of message travels, who owns its meaning, how it evolves, and how the topology preserves or destroys autonomy.

This skill is about the design philosophy and contracts of event-driven systems. It deliberately does not cover broker mechanics, delivery semantics, partitioning, retries, or dead-letter queues — those are covered in the message-queue-design-and-delivery, dead-letter-and-retry-strategy, and ordering-and-partitioning skills. Here the question is the event itself as an architectural artifact: its type, its contract, its evolution, and the coupling it creates.

## Core Rules

### Distinguish Events, Commands, And Messages — They Are Not Interchangeable

The single most consequential decision in event-driven design is the type of message traveling between services, because it determines ownership, routing, and coupling. Conflating them is the root of most architectural decay.

- **An event is a past-tense fact.** It states that something has already happened (`OrderPlaced`, `PaymentCaptured`, `UserRegistered`). The producer emits it after the fact and does not care who, if anyone, consumes it. The producer owns the event's schema and the business truth it represents; consumers react autonomously. Events are the foundation of loose coupling because the producer has no knowledge of consumers.
- **A command is a request to do something.** It is addressed to a specific receiver and expresses intent (`PlaceOrder`, `ReserveInventory`, `SendEmail`). The sender expects the receiver to act and cares about the outcome. Commands create coupling: the sender must know the receiver exists, and the receiver is obligated. Commands routed through a broker are still commands — the broker is just a transport, not a license to pretend the dependency is gone.
- **A message is the generic envelope.** Both events and commands are kinds of messages. Calling everything a "message" hides the distinction and lets teams accidentally build command-shaped integrations on event-shaped infrastructure.

The practical test: if the sender would be harmed by the receiver not acting, it is a command; if the sender has already committed the fact and merely announces it, it is an event. Mixing them — emitting an event whose name is actually an imperative, or relying on a consumer to make the producer's own process succeed — produces systems where a missing consumer silently breaks the business flow. Name accordingly, and never disguise a command as an event to avoid owning the dependency.

### Design Events As Contracts, Not As Serializations Of Internal State

An event is an integration contract between the producer and an unknown set of consumers, present and future. Treating it as a dump of the producer's internal model is the most common and most damaging mistake, because every internal refactor becomes a breaking change for every consumer.

A well-designed event answers: what business fact occurred, what identity does it concern, and what data does a consumer reasonably need to react without calling back? Strong choices:

- **Name events as past-tense business facts.** `OrderPlaced`, `InvoiceIssued`, `ShippingLabelGenerated`. The name describes the occurrence, not the implementation.
- **Include identity and the minimum payload a consumer needs.** At minimum, the id of the entity the event concerns and the fields a typical consumer acts on. A consumer that must immediately call back to the producer to learn anything useful is a sign the event is too thin.
- **Exclude volatile producer internals.** Internal status codes, database primary keys that have no business meaning, denormalized cache fields, and implementation flags should not travel. If a consumer would come to depend on them, they become a frozen public API.

The decision between a **fat event** (carries the full relevant payload) and a **thin event** (carries only an id and a reference, requiring a fetch) is a real tradeoff, not a style preference:

- **Fat events** decouple consumers from the producer's read API and let consumers process offline or during outages. They cost more bandwidth and make the producer's schema a larger contract to maintain. Prefer fat events when consumers need the data to act and the producer may be unavailable or rate-limited.
- **Thin events** keep the contract small and let the producer stay the single source of current truth, but they reintroduce a synchronous dependency (the fetch) and fail if the producer is down or the record is later mutated. Prefer thin events when the payload is large, when consumers need the freshest state rather than the state-at-event-time, or when the event is purely a notification trigger.

The trap to avoid is the inconsistent middle: events that are too thin to act on but too coupled to evolve. Decide deliberately per event type, and document the choice in the contract.

### Make The Broker A Dumb Pipe, And Put Intelligence At The Edges

A recurring architectural decision is whether the broker routes events by business rule (smart routing) or simply delivers everything on a topic to whoever subscribes (dumb pipe). The dumb-pipe model is almost always the better long-term choice, because it preserves the property that the producer does not know its consumers.

- **Dumb pipe (topic-based pub/sub).** The producer publishes to a topic; anyone who subscribes receives it. Adding a consumer requires no change to the producer or the broker configuration. Routing decisions live in the consumer ("do I care about this event?"), which is where autonomy is preserved.
- **Smart routing / content-based broker rules.** The broker inspects payloads and routes to specific queues based on business logic. This centralizes routing intelligence in infrastructure that is hard to test, hard to version, and invisible to producers. It tends to accrete into a brittle coupling layer where changing a routing rule risks breaking consumers that depend on it.

Strong default: topics reflect coarse event categories, consumers subscribe and filter their own interest, and the broker does not understand the business meaning of payloads. Reserve smart routing for genuine infrastructure concerns (dead-letter routing, priority lanes), never for business decisions that belong in services. When you find yourself wanting the broker to route `if order.total > 1000 then route-to-fraud-queue`, that logic belongs in a service consuming the topic, not in the broker.

### Evolve Events Additively, And Treat The Schema As A Public API

Events outlive the code that produced them. Consumers may be deployed days or weeks behind producers, owned by other teams, and impossible to coordinate in lockstep. The schema is therefore a public API, and changing it carelessly is a production incident distributed across every consumer.

Evolution rules, in order of safety:

- **Consumers must tolerate unknown fields.** Forward and backward compatibility both require consumers to ignore fields they do not recognize rather than fail. This is the single most important property, because it makes additive change safe.
- **Make only additive changes.** Add new optional fields with sensible defaults. Do not rename fields, do not remove fields, do not change a field's type incompatibly, do not change a field's meaning while keeping its name. Additive change is invisible to tolerant consumers.
- **Use a schema registry and a versioned format** (Avro, Protobuf, JSON Schema) so incompatible changes are caught at publish time, not discovered when a consumer crashes in production. A registry that enforces backward compatibility is a guardrail against accidental breaks.
- **When a breaking change is unavoidable, version the event type.** Emit `OrderPlaced` and `OrderPlacedV2` in parallel, migrate consumers one at a time, and only retire the old type after confirming no consumer remains. Never swap a schema in place and assume deployment ordering will save you.

The judgment call is when to declare a breaking change at all. Most "we have to break it" moments are actually "we did not design the original contract well" — the right move is often to add a new optional field and deprecate the old one, rather than break. Reserve true version bumps for genuine semantic change.

### Preserve Loose Coupling By Keeping Consumers Ignorant Of Producers

The defining benefit of event-driven architecture is that consumers can be added, removed, and changed without the producer's knowledge, and producers can change their internals without breaking consumers. Every design choice should be tested against this property, because it is easy to throw away accidentally.

Coupling enters through several doors, and each must be watched:

- **Temporal coupling** appears when a producer waits for a consumer to act (which means it was really a command) or when a consumer cannot function during a producer outage (which means the event was too thin). Loosely coupled consumers process events whenever they arrive and do not depend on the producer being live.
- **Schema coupling** appears when an event leaks internal state (see the contract rule). The fix is a stable, business-meaningful contract.
- **Semantic coupling** appears when a consumer depends on the producer's undocumented behavior — for example, assuming events arrive in a particular order, or assuming the producer will never emit two events for the same fact. Document the guarantees you intend to provide, and treat undocumented behavior as non-contractual.
- **Routing coupling** appears when a consumer can only receive events because of bespoke broker configuration tied to the producer. Prefer subscription by topic over point-to-point wiring.

The test for loose coupling: can you add a new consumer tomorrow, owned by a team you have never spoken to, with no change to the producer and no change to the broker? If not, identify which door the coupling came through and close it.

### Reason About Ordering And Idempotency At The Architecture Level

Ordering and idempotency are covered in depth in the message-queue-design-and-delivery and ordering-and-partitioning skills, but at the architecture level they have implications that shape the event contract itself.

- **Do not promise ordering you cannot keep.** If consumers depend on `OrderPlaced` arriving before `OrderShipped`, that ordering must be a documented, designed property (typically per-key ordering via partitioning), not an assumption. Architecture-level ordering is per-entity, achieved by keying, and it is never global. Document what ordering, if any, the system provides, and design consumers to tolerate reordering otherwise.
- **Design events so that idempotent consumption is possible.** Because events may be delivered more than once (the real-world baseline is at-least-once), every event should carry a stable identity (event id, or source plus source-event-id) that consumers can use to deduplicate. An event without a stable id forces consumers to guess, and guessing fails under redelivery.
- **Make the event self-describing about its own occurrence.** Include the timestamp of the business fact (not just the publish time) and a monotonic version or sequence where relevant, so consumers can reason about staleness and ordering without calling back.

These are architectural decisions because they are encoded in the contract; they cannot be retrofitted by the consumer alone.

### Define The Boundary Of The Event And The Saga Around It

Events often participate in long-running business processes (a saga) that span multiple services — placing an order reserves inventory, captures payment, schedules shipping, each as its own event. The architecture must decide where each event's responsibility begins and ends, because the absence of a coordinator is itself a design choice with failure modes.

- **Each event represents one committed business fact, not a whole process.** Do not bundle "order placed, inventory reserved, payment captured" into one event; emit them separately as each fact becomes true. Composite events couple consumers to a process they may only care about part of.
- **Compensation must be modeled as events too.** If a step fails, the system emits compensating events (`PaymentFailed`, `InventoryReleased`) rather than rolling back a transaction that spans services. Consumers must be designed to handle both the forward and compensating events idempotently.
- **Decide explicitly whether a process has a coordinator.** An orchestrator service that drives the saga is easier to reason about but is a coupling point; pure choreography (each service reacts to the previous event) is more decoupled but harder to trace and debug. Neither is universally right; the choice should be deliberate and documented.

## Common Traps

### Disguising A Command As An Event

Naming a message `ProcessOrder` or `ChargeCustomer` and publishing it to a topic, then relying on a specific consumer to act for the business flow to succeed. This looks event-driven but is actually a remote procedure call with worse observability, because the producer has implicitly depended on a consumer it claims not to know. The trap is that it feels decoupled until the consumer is missing and the flow silently stalls. If the producer needs the effect, use a command and own the dependency; if it does not, make the event a genuine past-tense fact and ensure no business correctness depends on a particular consumer.

### Serializing Internal State Into The Event

Publishing the producer's full domain object, including internal status flags, database ids, and denormalized fields, because it was convenient. Every internal refactor then breaks consumers, and consumers come to depend on fields that were never meant to be public. The trap is the short-term convenience of "just send everything." The fix is to design the event payload as a deliberate contract containing only business-meaningful data.

### Events Too Thin To Act On

Emitting `{ "orderId": 123 }` with no other data, forcing every consumer to call back to the producer to learn what happened. This reintroduces a synchronous dependency, fails when the producer is down or the record has been mutated, and defeats the offline-processing benefit of events. The trap is mistaking "small contract" for "good contract." A thin event is only appropriate when consumers genuinely need to fetch fresh state; otherwise include enough payload to act.

### Breaking The Schema In Place And Hoping Deployment Order Saves You

Renaming a field or changing a type, then deploying producer and consumers in a specific order and assuming nothing will go wrong. In a distributed system with multiple consumers and async delivery, some message is always in flight to an old consumer, and the breakage is discovered in production. The trap is treating a schema change like a code refactor. Use additive change, a schema registry, and versioned event types for genuine breaks.

### Assuming A Smart Broker Is Harmless

Putting business routing rules in the broker because it was easy at the time, then discovering the rules have become an untested, unversioned coupling layer that no one dares to change. The trap is that broker-level routing feels like configuration but behaves like distributed code with no tests. Keep the broker a dumb pipe and put business logic in services.

### Promising Global Ordering The System Cannot Deliver

Telling consumers events will arrive in order across the whole system, when the broker only orders within a partition and retries can still reorder even that. The trap is overpromising an ordering guarantee that the infrastructure does not provide. Document per-key ordering as the realistic maximum, and design consumers to tolerate reordering where ordering is not a documented property.

### Events Without A Stable Identity

Omitting an event id or source correlation, so consumers cannot deduplicate redelivered events and process the same fact twice. The trap is assuming the broker delivers exactly once. Encode a stable id in every event so consumers can be idempotent (see the message-queue-design-and-delivery skill).

### Composite Events That Couple Consumers To A Whole Process

Bundling an entire saga's state into one event so that a consumer that only cares about one step must parse and ignore the rest, and breaks when the process changes. The trap is modeling the message around the producer's process rather than around discrete business facts. Emit one event per committed fact.

## Self-Check

- [ ] Every message is correctly classified as an event (past-tense fact, producer does not depend on consumers) or a command (addressed request, sender needs the effect), and no command is disguised as an event to avoid owning a dependency.
- [ ] Event names are past-tense business facts (`OrderPlaced`, not `ProcessOrder`), and the payload contains business-meaningful identity and data while excluding volatile producer internals such as internal status codes or meaningless database ids.
- [ ] The fat-versus-thin decision is made deliberately per event type: fat events carry enough for consumers to act without the producer being live, and thin events are used only where fetching fresh state is genuinely required; no event sits in the inconsistent middle of too thin to act but too coupled to evolve.
- [ ] The broker is a dumb pipe for business routing — topics are coarse, consumers filter their own interest, and no business decision is encoded in broker-side routing rules that would be hard to test or version.
- [ ] Schema evolution is safe by construction: consumers tolerate unknown fields, changes are additive, a schema registry or versioned format catches incompatibilities at publish time, and genuine breaking changes use versioned event types with parallel emission during migration rather than in-place swaps.
- [ ] Loose coupling is verified by the "new consumer tomorrow" test — a consumer owned by an unfamiliar team can subscribe with no change to the producer or broker — and any failure of that test is traced to a specific coupling door (temporal, schema, semantic, or routing) and closed.
- [ ] Ordering is documented as a designed per-key property where required, never promised globally, and consumers tolerate reordering where ordering is not a contractual guarantee; events carry a stable identity (event id or source correlation) and business timestamp so consumers can deduplicate and reason about staleness.
- [ ] Long-running processes are modeled as discrete committed-fact events plus compensating events, with an explicit and documented choice between orchestration and choreography, and no event bundles an entire saga into one message.
- [ ] The highest-risk cases were specifically checked — a command masquerading as an event, a schema break reaching an old consumer in flight, an event too thin to act on during a producer outage, and a consumer that silently no-ops or double-processes under redelivery — rather than only the publish-subscribe happy path.
