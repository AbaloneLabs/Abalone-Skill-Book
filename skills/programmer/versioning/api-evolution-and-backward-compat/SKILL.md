---
name: api_evolution_and_backward_compat.md
description: Use when the agent is evolving a public or internal API, adding, removing, or changing fields and endpoints, maintaining backward compatibility, using expand-contract or parallel change patterns, versioning endpoints, planning migration periods for clients, or monitoring for breakage as an API changes over time.
---

# API Evolution and Backward Compatibility

An API is a contract with callers you do not control, and every change to it is a negotiation with every client that depends on it. The failure mode is treating the API as your own code that you can refactor freely: rename a field, change a type, remove an endpoint, and discover that dozens of clients break in ways you cannot see, because the clients are not in your codebase and their breakage arrives as support tickets, failed integrations, and eroded trust. Agents who have not maintained a widely-used API underestimate how conservatively it must evolve, because the cost of a breaking change is paid by others, invisibly, and is often much larger than the cost of designing the change to be compatible.

The judgment problem is that APIs must evolve (new requirements, corrections, improvements) but must not break existing clients, and these goals are in tension. The skill is evolving in ways that are additive and backward-compatible by default, using patterns like expand-contract (add the new alongside the old, migrate clients, then remove the old) to make breaking changes safely over time, versioning endpoints when a genuinely incompatible change is unavoidable, and giving clients a migration window measured in their reality, not your convenience. The agent must distinguish a compatible change (additive, optional, defaulted) from a breaking one (removal, type change, semantic change, tightened validation), and must monitor actual usage so that a removal does not surprise a client you assumed was gone.

## Core Rules

### Default to additive, backward-compatible changes

Most needed changes can be made compatibly. Adding an optional field with a default, adding a new endpoint, adding a new response field, or loosening a constraint are all backward-compatible: existing clients continue to work unchanged. Make these the default. The changes that break clients are removals, type changes, semantic changes (same shape, different meaning), tightened validation, and changes to error behavior; treat each of these as a significant event requiring a migration plan, not as a routine edit. When you can achieve the goal with an additive change, do that, even if it leaves some redundancy; the cost of redundancy is far less than the cost of breaking clients.

### Use expand-contract (parallel change) for changes that must eventually break

When a change cannot be made purely additive forever (a field must be removed, a type must change, a semantic must shift), use the expand-contract pattern. Expand: add the new field/endpoint/behavior alongside the old, so both work. Migrate: update clients to use the new path, over a migration window long enough for all clients to update. Contract: once usage of the old path is zero (verified, not assumed), remove the old. This converts a breaking change into a sequence of safe ones. The pattern fails when the contract step is rushed (removing the old before clients have migrated) or when the migrate step has no end condition (the old path lives forever because no one tracks usage). Define the migration window and the usage-based removal criterion up front.

### Version endpoints when a genuinely incompatible change is unavoidable

Some changes cannot be made incrementally (a fundamentally different resource model, a security model change, a response format overhaul). For these, version the endpoint (/v1/, /v2/) so old and new coexist, giving clients a migration path rather than a break. Versioning is expensive (you maintain multiple versions, and old versions linger), so reserve it for changes that truly require it; do not version for changes that expand-contract can handle. Define the version's support lifetime and end-of-life process at introduction, so v1 does not linger forever by inertia. Communicate the deprecation timeline clearly and repeatedly.

### Never make a silent semantic change

The most dangerous API change is one where the shape is identical but the meaning changes: a field that used to be in seconds now returns milliseconds, an error code that used to mean one thing now means another, a status that used to be terminal is now intermediate. Existing clients continue to "work" (no syntax break) but produce wrong behavior, and the bug is invisible until it causes real harm. Treat a semantic change as a breaking change even when the type is unchanged. When semantics must change, change the field name or version the endpoint so clients cannot silently misinterpret the new meaning.

### Give migration windows measured in client reality, not your convenience

A deprecation that gives clients two weeks to migrate assumes clients update on your schedule, which they do not. Clients have their own release cycles, priorities, and constraints, and a client that updates quarterly cannot migrate in two weeks. Size migration windows to the slowest realistic client (often months, sometimes years for widely-used public APIs), communicate the timeline repeatedly through multiple channels, and verify migration through usage telemetry before removing the old. A removal justified by "we announced it" that breaks clients who did not see the announcement is still your break.

### Monitor usage before removing anything

Never remove an API path based on the assumption that no one uses it. Measure actual usage (request counts per endpoint/field, across client identifications) and remove only what usage telemetry confirms is zero (or acceptably low, with outreach to the remaining users). Long-tail clients (internal tools, scheduled jobs, forgotten integrations) often use endpoints that "everyone migrated away from" years ago. Usage telemetry is the evidence that makes a removal safe; without it, the removal is a guess.

### Document compatibility expectations explicitly

State what clients can rely on and what they cannot. Without explicit compatibility promises, clients will depend on incidental details (field ordering, exact error messages, undocumented behaviors) that you did not intend to guarantee, and any change then "breaks" them. Document the contract (what is guaranteed stable) versus the implementation (what may change). For public APIs, publish a compatibility policy. This protects both sides: clients know what they can rely on, and you know what you are free to change.

## Common Traps

### Treating the API as your own refactorable code

Renaming, retyping, or removing freely breaks clients you cannot see. Evolve conservatively; the cost of a break is paid by others.

### Rushing the contract step of expand-contract

Removing the old before clients have migrated breaks them. Verify zero usage via telemetry before removing.

### A silent semantic change with unchanged shape

Same field, different meaning (seconds to milliseconds) breaks clients invisibly. Treat semantic changes as breaking; change the name or version.

### Migration windows sized to your convenience

Clients update on their own schedules. Size windows to the slowest realistic client and verify migration via telemetry.

### Removing based on assumed non-use

Long-tail clients use "abandoned" endpoints for years. Measure usage and remove only what telemetry confirms is zero.

### Versioning when expand-contract would suffice

Versioning is expensive to maintain. Reserve it for genuinely incompatible changes; use expand-contract for the rest.

### No explicit compatibility policy

Without one, clients depend on incidental details and any change "breaks" them. Document what is guaranteed stable versus what may change.

## Self-Check

- Is the default change style additive and backward-compatible (optional fields with defaults, new endpoints, loosened constraints), with removals and type changes treated as significant events?
- For changes that must eventually break, is the expand-contract pattern used with a defined migration window and a usage-based removal criterion (verified by telemetry, not assumed)?
- Is endpoint versioning reserved for genuinely incompatible changes, with a defined support lifetime and end-of-life process communicated at introduction?
- Are semantic changes (same shape, different meaning) treated as breaking, with the field renamed or the endpoint versioned to prevent silent misinterpretation?
- Are migration windows sized to the slowest realistic client and communicated repeatedly, with removal gated on verified usage telemetry?
- Before removing any endpoint or field, has actual usage been measured and confirmed zero (or remaining users contacted), rather than assumed unused?
- Is there an explicit compatibility policy documenting what clients can rely on (guaranteed stable) versus what may change, so clients do not depend on incidental details?
- Have you distinguished compatible changes (additive, optional) from breaking ones (removal, type change, semantic change, tightened validation) for every change under consideration?
