---
name: parallel_change_and_deprecation.md
description: Use when the agent is evolving a legacy API, library, schema, or interface that has consumers the agent does not fully control, applying expand-contract (parallel change) refactoring, running a deprecation cycle, using branch-by-abstraction to swap an implementation behind a stable interface, deciding whether to keep old and new APIs coexisting or to force a breaking change, setting deprecation timelines and communication, or judging when parallel change is worth its cost versus a clean break. Also covers the failure mode of breaking consumers you do not control (internal teams, external clients, generated code, language-level callers), the deprecation that never completes because the old path is never removed, removing an API before its consumers migrated, and the cost of maintaining two paths indefinitely. Distinct from incremental-modernization (system-level replacement via strangler) and data-migration-safety; this skill covers the API/code-level parallel-change judgment problem of evolving an interface without breaking its consumers.
---

# Parallel Change And Deprecation

Evolving an interface that already has consumers is a different problem from designing a new one. The new interface can be whatever you judge best; the evolving interface is constrained by every caller that already depends on it — internal teams, external clients, generated code, language-level consumers, code you cannot even see. Parallel change (the expand-contract pattern: add the new path, migrate consumers, then remove the old) and deprecation cycles exist to let you improve an interface without breaking the consumers that depend on the current one. The judgment problem is that a "clean" breaking change is clean only from the author's seat; from each consumer's seat it is an unannounced break, and the consumers you do not control — external clients, other repos, generated callers — cannot be fixed by editing them. Parallel change trades the author's short-term simplicity for the consumers' continued operation, and the discipline is to decide deliberately when that trade is worth it and to run the deprecation to completion rather than leaving two paths alive forever.

Agents tend to optimize for the clean break because it is simpler to author and looks better in the diff. The harm appears as downstream builds that break on upgrade with no migration path, as external clients pinned to an old version because the new one removed what they use, as a deprecation announced but never enforced so the old API lingers for years, and as the reverse — an API removed before its consumers migrated because the timeline was optimistic. The discipline is to expand before you contract (add the new path alongside the old, never replace), to migrate consumers behind a stable abstraction (branch by abstraction), to communicate and time the deprecation to the slowest consumer you intend to keep, and to actually remove the old path once migration is complete — because a parallel change that never contracts is just two interfaces to maintain forever. This skill is distinct from incremental-modernization, which replaces whole systems via the strangler pattern; here the unit of change is an interface, and the central risk is the consumers.

## Core Rules

### Expand Before You Contract

The expand-contract (parallel change) pattern is: add the new capability alongside the old, migrate callers to the new, then remove the old. The contract step — removal — comes only after migration. Replacing in place (contract without expand) breaks every consumer at once.

- **Add, don't replace.** The new method/field/endpoint/route ships alongside the old; the old continues to work unchanged. Consumers can migrate at their own pace.
- **Migrate callers before removing anything.** No removal happens until the consumers you intend to keep have moved; removal ahead of migration is the breaking change parallel change exists to prevent.
- **Contract only when migration is complete.** The old path is removed once its callers are gone — not before, and not "never."

### Decide Deliberately Whether Parallel Change Is Worth The Cost

Parallel change is not free: you maintain two paths, keep them behaviorally consistent, and carry the migration to completion. For some changes the cost exceeds the benefit, and a clean break (or no change at all) is correct. Make this decision explicitly rather than defaulting.

- **Parallel change is worth it when consumers you do not control depend on the current shape** (external clients, other teams' code, generated callers, published APIs). The cost of breaking them exceeds the cost of maintaining two paths through the deprecation window.
- **A clean break is acceptable when you control all consumers and can migrate them in the same change**, or when the interface is internal and trivial. Do not pay the parallel-change cost where there is no consumer risk.
- **No change is acceptable when the only motivation is aesthetics.** Renaming or restructuring an interface that works, for style alone, imposes migration cost for no consumer benefit; leave it unless there is a functional reason.

### Use Branch By Abstraction To Swap Implementation Behind A Stable Interface

When the change is to an *implementation* rather than the interface itself, branch by abstraction lets you swap the implementation behind a stable abstraction while consumers keep calling the same interface. You introduce an abstraction (interface, facade, strategy), route consumers through it, build the new implementation behind it, and switch over — all without changing the consumer-facing interface.

- **Introduce the abstraction first, with the current implementation behind it.** Consumers now depend on the abstraction, not the concrete implementation.
- **Build the new implementation behind the same abstraction.** Both implementations coexist; you can switch between them (feature flag, config, routing) without touching consumers.
- **Switch over incrementally and reversibly.** Move traffic/calls to the new implementation behind a flag, observe, and keep the rollback path until the new implementation is proven.

### Communicate And Time The Deprecation To The Consumers You Must Keep

A deprecation only works if the consumers who must migrate learn of it, understand the migration path, and have time to do it. A deprecation that is announced in a changelog no one reads, with no migration guide and an optimistic timeline, breaks consumers as surely as no deprecation at all.

- **Announce with a migration path, not just a warning.** `@Deprecated` or a deprecation header is necessary but not sufficient; pair it with documentation of the replacement and how to move to it.
- **Set the timeline to the slowest consumer you intend to keep, not the average.** If one critical external client needs two quarters, the removal date is two quarters, unless you are willing to break that client.
- **Make deprecation visible at the point of use.** Compiler/runtime warnings, logs, and headers reach consumers who never read the changelog; a note buried in release notes does not.
- **For consumers you do not control, version the interface.** A new major version signals the break and lets consumers upgrade on their schedule; an in-place removal with a version bump is honest where a silent in-place removal is not.

### Run The Deprecation To Completion — Remove The Old Path

A parallel change that never contracts is a failure: two interfaces maintained forever, the deprecation warning ignored because it never escalates, and the migration stalled because there is no forcing function. The discipline is to set and enforce the removal.

- **Set a removal date and hold it (or revise it deliberately).** An open-ended deprecation never completes; a dated one does.
- **Escalate the signal as the date approaches.** Move from silent deprecation to warnings to errors so consumers who have not migrated are forced to act before removal.
- **Remove the old path once migration is complete.** Carrying the old path indefinitely is the trap parallel change was meant to avoid; the change is finished when the old path is gone.

## Common Traps

### Breaking Consumers You Do Not Control

Treating an interface with external or cross-team consumers like an internal one, replacing it in place, and breaking builds/clients you cannot fix. Use parallel change when consumers are outside your control; the clean break is acceptable only when you control every consumer.

### Contracting Before Expanding (In-Place Replacement)

Replacing the old path with the new in a single change so every consumer breaks at once, defeating the entire purpose of parallel change. Expand first (add alongside), migrate, then contract (remove).

### The Deprecation That Never Completes

Announcing a deprecation but never removing the old path, so two interfaces are maintained forever, the warning is ignored, and the migration stalls. Set a removal date, escalate the signal, and remove the old path when migration is done.

### Removing Before Migration Is Complete

Removing the old API ahead of the published timeline because "everyone should have migrated by now," breaking consumers who were following the stated schedule. Hold the timeline you communicated, or revise it deliberately and re-communicate.

### Optimistic Timelines Set To The Average Consumer

Setting the removal date based on the average migration speed and breaking the slowest critical consumer. Set the timeline to the slowest consumer you intend to keep.

### Deprecation Visible Only In The Changelog

Announcing the deprecation only in release notes that consumers may not read, while the code itself gives no signal. Make deprecation visible at the point of use (warnings, headers, compiler diagnostics) with a migration path.

### Parallel Change For A Change That Did Not Need It

Paying the full expand-contract cost for an internal interface whose consumers you fully control and could migrate in one change, or for a purely cosmetic rename. Match the mechanism to the consumer risk; a clean break or no change is sometimes correct.

## Self-Check

- [ ] Expand happened before contract: the new path was added alongside the old (old unchanged), consumers were migrated, and the old path is removed only after migration — never replaced in place.
- [ ] The decision to use parallel change versus a clean break versus no change was made deliberately based on consumer risk: parallel change where consumers outside the agent's control depend on the current shape; clean break only where all consumers are controlled and migrated together; no change where the motive is purely aesthetic.
- [ ] Where the change was to an implementation rather than the interface, branch by abstraction was used: abstraction introduced with the current implementation behind it, new implementation built behind the same abstraction, cutover incremental and reversible behind a flag.
- [ ] The deprecation was communicated with a migration path (not just a marker), made visible at the point of use (warnings/headers/compiler diagnostics, not only a changelog), and timed to the slowest consumer the team intends to keep rather than the average.
- [ ] For consumers outside the agent's control, the interface was versioned (new major version) so the break is explicit and consumers upgrade on their schedule, rather than a silent in-place removal.
- [ ] The deprecation is run to completion: a removal date is set and held (or deliberately revised and re-communicated), the signal escalates as the date approaches, and the old path is actually removed once migration is complete rather than carried indefinitely.
- [ ] The highest-risk cases were verified — a consumer outside the agent's control was not broken, an in-place replacement was avoided in favor of expand-first, a deprecation that had stalled was driven to removal, and a removal was not done ahead of the communicated timeline — not only the clean internal-API case.
- [ ] The scope stayed at the interface/API level and did not drift into whole-system strangler replacement (incremental-modernization) or data migration concerns, which are separate judgment problems.
