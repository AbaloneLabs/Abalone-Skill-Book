---
name: schema_registry_and_contract_evolution.md
description: Use when the agent is designing or reviewing message schema management and contract evolution for an event-driven or messaging system, choosing or configuring a schema registry, defining backward, forward, or full compatibility policies, evolving Avro, Protobuf, or JSON Schema message definitions, managing breaking changes across independently deployed producer and consumer teams, adopting consumer-driven contracts, or preventing a producer schema change from silently breaking deployed consumers. Also covers schema versioning and migration strategy, the compatibility rules per serialization format, coordinated deployment ordering across teams, handling required-versus-optional field transitions, and the recurring trap of treating a message schema as a private type rather than a public cross-team API.
---

# Schema Registry And Contract Evolution

A message schema is a contract between a producer and every consumer, and unlike an in-process function signature, the parties to that contract are deployed independently, owned by different teams, and running versions that may be years apart. A schema change that looks trivial to the producer — renaming a field, changing a type, removing an unused field — can silently break consumers that were deployed against the old shape, causing them to drop messages, misparse them, or fail at runtime. The judgment problem is that message schemas evolve in a context where there is no compile-time check across the producer-consumer boundary, no coordinated deploy, and often no immediate signal that a change has broken a downstream service. The harm is a production incident spread across every consumer, discovered only when messages start failing.

Agents tend to under-invest here because the schema looks like a local data type and the first change usually works. The harm appears later and diffusely. A field renamed in a "refactor" breaks a consumer team that did not know about the change. A field type widened from int to long breaks consumers that deserialize strictly. A field removed because "nothing uses it" breaks a consumer that did use it, silently, because the producer had no visibility into that consumer's code. A new required field added without a default breaks every consumer still running the old version, because old producers cannot fill it. None of these surface in the producer's tests, because the producer's tests do not include the consumers. The discipline is to treat the message schema as a public, versioned API; to govern its evolution through a registry and a compatibility policy; and to coordinate breaking changes across the teams that depend on it.

This skill is the deep treatment of schema management and contract evolution in messaging systems. It goes beyond the schema-evolution overview in the message-queue-design-and-delivery skill (which establishes additive-only changes and registry use) and focuses on the hard decisions: which compatibility policy to enforce per topic, how the choice of serialization format constrains evolution, how to manage genuinely breaking changes across independent teams, and how consumer-driven contracts make implicit dependencies visible before they break.

## Core Rules

### Treat The Message Schema As A Public Versioned API, Not A Private Type

The foundational decision is to treat every message schema as a public API with the same discipline applied to an HTTP or RPC interface. A producer does not own its message schema in the sense of being free to change it; it owns it in the sense of being responsible for not breaking its consumers. This means the schema is versioned, its changes are governed by a policy, its consumers are known (or at least assumed to exist), and breaking changes are coordinated rather than pushed.

- **The schema outlives any single deploy.** Consumers may run versions that lag the producer by days, weeks, or longer, especially in environments with independent release cadences. A change is only safe if it is safe against every deployed consumer version, not just the latest.
- **The producer has incomplete visibility into consumers.** A producer team often cannot enumerate every consumer, especially in a large organization or an open event stream. Assume unknown consumers exist and design changes to be safe against them.
- **Govern changes through a schema registry.** A registry (Confluent, Apicurio, AWS Glue, Pulsar's built-in) stores schemas, versions them, and enforces a compatibility policy at publish time, so an incompatible change is rejected before it reaches production rather than discovered at runtime. Without a registry, compatibility is enforced only by convention and discipline, which fails under pressure.
- **Document the schema as a contract.** Field semantics, nullability, units, and lifecycle (stable, experimental, deprecated) must be documented, because consumers will build against assumptions that the raw schema does not encode.

The mindset shift: a message schema is an interface published to the world, and changing it carries the same weight and risk as changing a public API.

### Choose The Compatibility Policy Deliberately Per Topic

Compatibility policy is the rule the registry enforces when a new schema version is registered, and it determines which deployment orderings are safe. The choice must be explicit per topic, because the right policy depends on which direction of deployment lag the system must tolerate.

- **Backward compatibility** means new consumers can read data produced by the old schema. This is the safe default when consumers are upgraded before producers, or when a stream retains old data that new consumers will read. Achieved by making additive changes only: new fields are optional with defaults, no fields are removed or renamed, no types are changed incompatibly.
- **Forward compatibility** means old consumers can read data produced by the new schema. Required when producers are upgraded before consumers, or when a producer change must not break currently-deployed consumers. Achieved by making new fields optional and ensuring old consumers tolerate unknown fields (which they must be designed to do).
- **Full compatibility** means both — the change is safe regardless of deployment order. This is the strictest and safest policy, and the right choice for streams with many independent consumers, retained history, and uncoordinated deploys. It permits only additive changes with defaults and forbids removals, renames, and incompatible type changes.
- **Match the policy to the stream's reality.** A transient command queue with a single consumer and coordinated deploys may tolerate a looser policy; a retained event stream with many consumers across teams demands full compatibility. Choosing a looser policy than the stream requires is how silent breakage happens.

The judgment call: identify which deployment orderings must be safe (can consumers lag producers? can producers lag consumers? is history retained and re-read?), and choose the policy that guarantees those orderings. When in doubt, choose full compatibility, because the cost of a stricter policy is minor and the cost of breakage is large.

### Understand How The Serialization Format Constrains Evolution

The choice of Avro, Protobuf, or JSON Schema is not just a performance or ergonomics decision; it determines which evolutions are possible and how compatibility is achieved, because each format has different rules for defaults, field identification, and unknown-field handling.

- **Avro** resolves schemas by field name and requires defaults for new fields to maintain backward compatibility. Removing a field is backward-compatible (old readers ignore it) but forward-incompatible if old readers expect it; renaming a field is a breaking change because resolution is by name. Avro's reliance on writer and reader schemas makes it powerful but sensitive to field naming stability.
- **Protobuf** resolves by field number, not name, so renaming a field is safe but reusing a field number is catastrophic. Adding a field is backward and forward compatible (unknown fields are preserved by default); removing a field requires reserving its number so it is never reused. The cardinal rule in Protobuf is never reuse a field number, because that silently corrupts deserialization across versions.
- **JSON Schema** is the most flexible and the least enforced: consumers typically tolerate unknown fields by default, but there is no native field-numbering or default mechanism, and compatibility depends heavily on how strictly consumers validate. JSON Schema is easy to evolve loosely and easy to break silently, because the format does not enforce a resolution strategy the way Avro and Protobuf do.

The implication: the evolution strategy must be designed against the specific format's rules. A change that is safe in Protobuf (rename) is breaking in Avro; a change that is safe in Avro (if defaults are set) may be unsafe in JSON Schema if consumers validate strictly. Do not carry evolution assumptions across formats.

### Make Additive Changes The Default And Sequence Breaking Changes Deliberately

Most evolution should be additive, because additive changes are compatible across policies and formats with the least risk. When a genuinely breaking change is required — a field must be removed, a type must change, a semantic must shift — it must be sequenced as a multi-step migration, not pushed as a single change.

- **Additive first.** New fields are optional with defaults; new enum values are tolerated by consumers designed for forward compatibility; new message types are introduced alongside the old. Additive changes can be deployed without coordination in most cases.
- **Breaking changes require a multi-step migration.** To remove a field: first stop the producer from populating it and wait until all consumers no longer read it, then remove it. To change a type: introduce a new field with the new type, populate both, migrate consumers to the new field, then deprecate and eventually remove the old. To rename: introduce the new name, dual-populate, migrate consumers, then remove the old.
- **Version the message type for irreconcilable changes.** Where a change cannot be made compatibly (a structural reshape, a semantic redefinition), publish a new versioned topic or event type (`OrderCreatedV2`) and run old and new in parallel during migration, retiring the old when all consumers have moved.
- **Coordinate deployment order across teams.** A breaking change is only safe when the deployment order is controlled: consumers upgraded before producers for backward-incompatible changes, producers before consumers for forward-incompatible changes. This requires communication and a release plan across the teams that depend on the schema, not a unilateral push.

The rule: additive changes are the steady state; breaking changes are projects, with a migration plan, a timeline, and coordination across every affected team.

### Make Dependencies Visible Through Consumer-Driven Contracts And Testing

The deepest risk in schema evolution is invisible dependencies: a consumer relies on a field, a semantic, or an invariant that the producer does not know about and does not test for. A producer's own tests cannot catch a breakage it does not know exists. Consumer-driven contracts make those dependencies visible by having consumers publish the aspects of the schema they depend on, and by testing producer changes against those contracts before release.

- **Publish consumer contracts or expectations.** Each consumer documents (ideally as code) what it requires of the schema: which fields it reads, which invariants it assumes, which enum values it handles. Aggregated, these form the effective contract the producer must not break.
- **Test producer schemas against consumer contracts before release.** A schema registry with compatibility checking catches structural breakage; consumer contract tests catch semantic breakage (a field still exists but its meaning changed). Run these in the producer's CI so a breaking change fails the build.
- **Communicate changes to consumer teams proactively.** Even with tooling, a planned breaking change must be communicated to the teams that depend on the schema, with a timeline, so they can prepare. A registry catches accidents; communication handles intent.
- **Treat deprecation as a process, not a flag.** Marking a field deprecated is the start of a migration, not the end. Track which consumers still read the deprecated field, and only remove it when the migration is complete.

The goal is to convert invisible cross-team dependencies into explicit, tested contracts, so that a schema change fails fast and loudly rather than silently breaking a consumer in production.

## Common Traps

### Treating The Schema As A Private Type And Changing It Freely

A producer team treating its message schema as an internal data type and refactoring it freely — renaming fields, changing types, removing "unused" fields — so deployed consumers silently break because the producer had no visibility into what those consumers depended on. The trap is that the change passes the producer's own tests. Treat the schema as a public API and govern it through a registry.

### Choosing A Looser Compatibility Policy Than The Stream Requires

Selecting backward-only compatibility for a retained event stream with many consumers that lag the producer, so a producer-first deploy breaks old consumers reading new data. The trap is that backward compatibility sounds sufficient. Choose the policy against which deployment orderings and history re-reads must be safe; default to full compatibility for multi-consumer retained streams.

### Renaming A Field In Avro Or Reusing A Field Number In Protobuf

Applying an evolution rule from one format to another — renaming a field (safe in Protobuf, breaking in Avro) or reusing a retired field number (catastrophic in Protobuf) — because the format-specific rules were not understood. The trap is that the change looks innocuous by general intuition. Learn the specific format's resolution and compatibility rules before evolving.

### Adding A Required Field Without A Default

Introducing a new required field with no default, so old producers (or retained old data) cannot satisfy it and new consumers fail on every message that lacks it. The trap is that the field feels necessary. New fields must be optional with defaults to remain compatible; make them required only through a coordinated migration.

### Pushing A Breaking Change As A Single Deploy

Making a breaking change (field removal, type change) in a single deploy without a multi-step migration, so currently-deployed consumers break immediately and simultaneously. The trap is the convenience of one change. Sequence breaking changes as additive-then-migrate-then-remove projects with coordinated deployment order.

### Assuming The Producer Knows All Consumers

A producer team believing it can enumerate every consumer and therefore coordinate directly, missing a consumer (an analytics pipeline, a newly-onboarded team, an open stream reader) that breaks silently. The trap is overconfidence in visibility. Assume unknown consumers exist, enforce compatibility through the registry, and prefer full compatibility for streams with broad readership.

### Removing A Field That A Consumer Still Reads

Removing a field because the producer stopped populating it, without verifying that no consumer still reads it, so a consumer that depended on the field breaks. The trap is conflating "the producer no longer writes it" with "no one needs it." Track consumer dependencies through contracts and remove only when the migration is complete.

### Ignoring Semantic Changes That Preserve Structure

Changing a field's meaning without changing its structure — a `status` enum gains a new value old consumers do not handle, a `timestamp` changes unit, a `currency` field silently starts holding a different code — so consumers that deserialized correctly now behave incorrectly. The trap is that structural compatibility checking passes. Treat semantic changes as breaking and coordinate them, even when the schema validates.

## Self-Check

- [ ] Every message schema is treated as a public versioned API: it is registered in a schema registry, governed by an explicit compatibility policy, documented with field semantics and lifecycle, and changes are governed rather than pushed freely by the producing team.
- [ ] The compatibility policy per topic (backward, forward, full) is chosen deliberately against which deployment orderings and history re-reads must be safe, defaulting to full compatibility for retained multi-consumer streams, and is not looser than the stream's reality requires.
- [ ] The evolution strategy is designed against the specific serialization format's rules — field-name resolution and defaults in Avro, field-number reservation and never-reuse in Protobuf, validation strictness in JSON Schema — and no evolution assumption is carried across formats.
- [ ] Steady-state changes are additive (new optional fields with defaults, tolerated new enum values, new types alongside old), and genuinely breaking changes are sequenced as multi-step migrations (introduce, dual-populate, migrate consumers, then remove) with coordinated deployment order across affected teams.
- [ ] Cross-team dependencies are made visible through consumer-driven contracts or expectations that are tested against producer schema changes in CI, so a structural or semantic breakage fails the build before reaching production, and deprecation is tracked as a migration process rather than a flag.
- [ ] Irreconcilable breaking changes are handled by versioning the message type or topic (`V2`) and running old and new in parallel until all consumers migrate, rather than forcing an incompatible change through the existing schema.
- [ ] The highest-risk cases were specifically verified — a rename in Avro, a reused field number in Protobuf, a required field without a default, a semantic change that passes structural validation, and a breaking change pushed without coordination to an unknown consumer — rather than only the additive-change happy path.
