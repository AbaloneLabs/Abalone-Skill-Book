---
name: schema_evolution_in_pipelines.md
description: Use when the agent is evolving a data pipeline or streaming schema (adding, renaming, retyping, or removing fields), setting up a schema registry, choosing backward, forward, or full compatibility modes, planning a migration that does not break deployed consumers, handling deserialization failures from schema mismatch, dealing with slow or lagged consumers during a producer schema change, versioning Avro, Protobuf, or JSON schemas across a fleet of services, or diagnosing silent data corruption from a field that became null or was misparsed after a source change. Also covers the failure mode of treating a schema as an internal detail once multiple consumers depend on it, the cost of breaking changes, and the discipline of additive, contract-governed evolution.
---

# Schema Evolution In Pipelines

A pipeline schema is a contract between the data's producer, the pipeline, and every downstream consumer. Once more than one consumer depends on a dataset, the schema stops being an internal detail of the producer and becomes a public interface — and like any public interface, changing it carelessly breaks the consumers who rely on it. The recurring failures are predictable: a producer renames a column and every consumer that selected it by name gets null; a type change truncates or misparses historical data; a dropped field removes a feature a dashboard depends on; a new required field breaks old consumers that cannot populate it. None of these fail at the producer; they all fail silently downstream, where the data looks plausible but is wrong, and the wrongness propagates into dashboards, models, and decisions before anyone notices.

Agents tend to under-invest in schema evolution because on the happy path a change "just works" — the producer ships the new schema, the pipeline adapts, and the immediate consumer is updated. The harm appears in the consumers who were not updated: the team that built a dashboard on the old field name six months ago, the model trained on the old type, the downstream service that deserializes the old format and crashes or silently drops records on the new one. The judgment problem is treating the schema as a versioned contract, evolving it additively and compatibly by default, coordinating breaking changes with lead time and parallel run, and enforcing the contract at the boundary so a producer change that would break consumers fails before it reaches them. This is the data equivalent of API versioning discipline, applied to schemas.

## Core Rules

### Treat The Schema As A Versioned Contract Once It Has Consumers

The moment a dataset has more than one consumer, its schema is a contract. Changing it is changing a public interface, and the discipline of public interfaces applies: changes are deliberate, communicated, compatibility-checked, and versioned. A producer who "owns" the schema and changes it freely is a producer who breaks consumers they do not know about.

- **Inventory the consumers before changing.** Who reads this dataset, which fields, with what assumptions? A change that is safe for the producer may break a consumer the producer is unaware of.
- **Version the schema explicitly.** Use a schema registry or versioned schema definitions so the contract is recorded, not implicit. A schema that exists only in the producer's code is an undocumented contract.
- **Communicate changes with lead time.** A breaking change shipped without warning is a multi-consumer incident. Notify consumers, run old and new in parallel during migration, and retire the old only when consumers have migrated.

### Choose The Compatibility Mode Deliberately

Schema registries and evolution frameworks define compatibility modes, and the choice determines what changes are safe:

- **Backward compatibility (new schema can read old data).** A consumer updated to the new schema can still read data produced with the old schema. Achieved by adding fields with defaults, removing fields, or widening types. This protects consumers when they upgrade after the producer.
- **Forward compatibility (old schema can read new data).** A consumer still on the old schema can read data produced with the new schema (ignoring new fields, using defaults for removed ones). This protects consumers who have not yet upgraded when the producer changes.
- **Full compatibility (both backward and forward).** The strongest guarantee: any consumer, old or new, can read any data, old or new. Required when consumers upgrade at different times and the producer cannot coordinate. This is usually the right default for shared datasets.

Match the mode to the consumer-upgrade reality. If all consumers upgrade in lockstep with the producer, backward may suffice; if consumers upgrade independently and asynchronously (the common case for shared datasets), require full compatibility.

### Make Additive Changes The Default

The safest schema changes are additive: add an optional field with a default, add a new enum value (with consumer tolerance for unknown values), widen a numeric type. These are backward and often forward compatible, and they do not break any consumer that does not use the new field. Default to additive evolution:

- **Adding an optional field with a default** is safe in all compatibility modes; old consumers ignore it, new consumers populate it.
- **Adding a new enum value** is safe if consumers handle unknown values gracefully (treat unknown as a default, do not crash); otherwise it requires coordination.
- **Widening a type** (int to long, float to double) is often safe; narrowing (long to int) is breaking.

Breaking changes — renaming, removing, retyping, or adding a required field without a default — must be coordinated, not slipped in.

### Coordinate Breaking Changes As Versioned Migrations

When a breaking change is unavoidable, treat it as a migration project, not a one-shot edit:

- **Do not rename in place.** A rename breaks every consumer selecting by the old name. Instead, add the new field alongside the old, populate both during a transition period, migrate consumers to the new field, and only then remove the old.
- **Do not retype in place.** A type change misparses or truncates historical data and breaks consumers expecting the old type. Add a new field with the new type, backfill it, migrate consumers, then remove the old.
- **Do not remove a field that has consumers.** Confirm no consumer depends on it before removing; if any does, coordinate the removal with them.
- **Run old and new in parallel** during the migration so consumers can switch at their own pace, then retire the old schema version when migration is complete.
- **Notify with lead time.** A breaking change deployed the same day it is announced leaves consumers no time to adapt.

A breaking change shipped without coordination is the most common cause of multi-consumer data incidents.

### Handle Deserialization Failures Explicitly

When a consumer receives data it cannot deserialize — a schema mismatch, an unknown required field, a type it cannot parse — the failure must be handled explicitly, not allowed to crash the consumer or silently drop the data:

- **Route undecodable records to a dead-letter or quarantine** rather than crashing the pipeline or dropping them silently. A record that cannot be parsed is a signal of a schema mismatch that needs investigation.
- **Distinguish "unknown field" (often safe to ignore) from "missing required field" or "type mismatch" (breaking).** Forward-compatible consumers should ignore unknown fields; they should not silently accept a type they cannot parse.
- **Alert on deserialization failure rates.** A spike in failures after a producer change is the signal that a breaking change reached consumers; without monitoring, the failure is silent.

### Account For Slow Or Lagged Consumers During A Producer Change

In a streaming or log-based system, consumers read at their own pace, and a lagged consumer may be reading old data long after the producer has moved to a new schema. A producer that drops the old schema version before all consumers have caught up breaks those consumers:

- **Do not remove old schema versions until all consumers have migrated.** Track consumer offsets and schema versions; retire an old version only when no consumer is reading it.
- **For log-based systems, the log retains old-format records.** A consumer reading from the head of the log may encounter a mix of old and new format records during the transition; it must handle both (which is what forward/backward compatibility provides).
- **Slow consumers are a real operational constraint.** A consumer that lags by hours or days cannot be forced to upgrade instantly; the producer must keep the old schema available until they catch up.

### Enforce The Contract At The Boundary

A data contract — a machine-checkable agreement on schema, types, constraints, and semantics — enforced at the boundary prevents a producer change from reaching consumers if it violates the contract:

- **Register schemas and check compatibility on publish.** A producer that tries to publish an incompatible schema version should fail at the registry, before the data reaches consumers.
- **Define and test contracts between producers and consumers.** The contract is the source of truth; both sides test against it. A producer change that breaks the contract is caught in CI, not in production.
- **Include semantic constraints, not just structure.** A field may be the right type but semantically wrong (a timestamp in seconds vs milliseconds, a currency code vs a symbol). Contracts that capture semantics catch more than structural checks.

## Common Traps

### Renaming A Field In Place

Renaming a column or field directly, breaking every consumer that selects or deserializes by the old name, who then get null or a parse failure. Add the new field alongside the old, migrate consumers, then remove the old.

### Retyping A Field In Place

Changing a field's type (int to string, timestamp units) so historical data is misparsed or truncated and consumers expecting the old type break. Add a new field with the new type, backfill, migrate, then remove.

### Dropping A Field With Unknown Consumers

Removing a field because the producer no longer needs it, breaking a downstream consumer (a dashboard, a model, a service) that depended on it. Inventory consumers before removing; coordinate the removal.

### Adding A Required Field Without A Default

Adding a new required field with no default, which is a breaking change: old consumers cannot produce it, old data lacks it, and the schema is no longer backward compatible. Add new fields as optional with defaults.

### No Compatibility Check, So Breaking Changes Ship Silently

A producer publishing a new schema version without a compatibility check, so a breaking change reaches consumers undetected until they fail. Register schemas and enforce compatibility at the boundary.

### Removing An Old Schema Version Before Consumers Migrate

In a streaming system, dropping the old schema version while a lagged consumer is still reading old-format records, breaking that consumer. Track consumer offsets and retire old versions only when no consumer reads them.

### Silent Deserialization Failures

A consumer that cannot parse new data crashing or silently dropping records, with no alert, so the schema mismatch goes unnoticed while data is lost. Route failures to a dead-letter, alert on failure rates.

### Treating The Schema As The Producer's Internal Detail

A producer changing "its" schema freely because it owns the data, breaking consumers it does not know about. Once a dataset has consumers, the schema is a contract; evolve it with the discipline of a public interface.

### New Enum Value That Crashes Old Consumers

Adding an enum value that old consumers do not handle (they crash or misbehave on the unknown value), when the consumers were not written to tolerate unknown enum values. Coordinate enum additions or ensure consumers handle unknowns gracefully.

## Self-Check

- [ ] The schema is treated as a versioned contract: consumers are inventoried before changes, the schema is registered/versioned explicitly, and changes are communicated with lead time rather than shipped unilaterally.
- [ ] The compatibility mode (backward, forward, full) is chosen deliberately against the consumer-upgrade reality — full compatibility for shared datasets where consumers upgrade asynchronously — and enforced via the schema registry.
- [ ] Additive changes are the default (optional fields with defaults, new enum values with tolerant consumers, type widening); breaking changes (rename, retype, remove, required-without-default) are coordinated migrations, not in-place edits.
- [ ] Breaking changes follow the migration pattern: add new alongside old, populate both during transition, migrate consumers at their own pace, run old and new in parallel, retire the old only when migration is complete.
- [ ] Deserialization failures are handled explicitly: undecodable records route to a dead-letter/quarantine, unknown fields are distinguished from missing-required/type-mismatch, and failure rates are alerted so a schema mismatch is detected rather than silent.
- [ ] In streaming/log systems, old schema versions are retained until all consumers (including lagged ones) have migrated; consumer offsets and schema versions are tracked before retiring an old version.
- [ ] Data contracts are enforced at the boundary: schemas are registered and compatibility-checked on publish, contracts are tested by both producer and consumer in CI, and semantic constraints (units, codes) are captured where structure alone is insufficient.
- [ ] The highest-risk cases were verified — a rename/retype/remove against deployed consumers, a new required field, a lagged consumer reading old-format records after the producer changed, and a deserialization failure that would have silently dropped data — not only the producer-and-immediate-consumer happy path.
