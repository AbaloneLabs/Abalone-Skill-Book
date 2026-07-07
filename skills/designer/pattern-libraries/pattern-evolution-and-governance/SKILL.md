---
name: pattern_evolution_and_governance.md
description: Use when the agent is governing a design system or pattern library over time, deciding how patterns evolve, version, deprecate, and migrate, managing contribution and review, handling breaking changes, or preventing drift between documented patterns and real product usage.
---

# Pattern Evolution And Governance

A pattern library is not finished when it is published; it begins to decay the moment it is. Components change underneath patterns, products adopt patterns in ways the library never sanctioned, new needs appear that the current set cannot serve, and documented examples drift out of sync with live code. Without governance, a library that started as a source of truth becomes a museum of how things used to be done, while the real, de facto patterns live in scattered product code that no one can find or trust. Governance is the work that keeps a library alive, coherent, and actually used. It is also the work most often skipped, because publishing feels like completion.

Use this skill before changing, versioning, deprecating, or migrating patterns, before setting up contribution and review processes, and before auditing why a library has stopped being trusted. The goal is to prevent the agent from making untracked changes to shared patterns, from breaking consumers without a migration path, or from letting the library drift from real usage until it is irrelevant.

## Core Rules

### Treat The Library As A Product With Owners, Not A Shared Drive

A library without owners is a library no one is responsible for. Changes land without review, defects go unfixed, and contradictions accumulate. Governance starts with clear ownership.

Establish:

- a named owner or team accountable for the library's coherence;
- maintainers responsible for specific categories or patterns;
- a defined process for proposing, reviewing, and accepting changes;
- a cadence for review, cleanup, and retirement;
- a way for consumers to report issues and request patterns.

Ownership does not mean gatekeeping every pixel; it means someone is accountable for consistency, quality, and evolution.

### Version Patterns And Communicate Change

Patterns and components change, and consumers depend on them. Without versioning and communication, a change that is invisible to maintainers becomes a breaking change for every consumer. Versioning makes change legible.

Versioning practices:

- version components and patterns with a clear scheme;
- distinguish breaking from non-breaking changes explicitly;
- maintain a changelog that consumers can read;
- communicate changes through release notes, not only internal commits;
- provide migration guidance for breaking changes, not just a notice.

A change without a version and a migration path is a change that will hurt consumers, and the library will be blamed for the resulting breakage.

### Deprecate Deliberately, Never Silently

Removing or changing a pattern without a deprecation path strands every consumer that depends on it. Deprecation is how a library evolves without abandoning its users.

A healthy deprecation process:

- marks the old pattern as deprecated with a clear replacement;
- keeps the old pattern working during a stated migration window;
- emits warnings or signals so consumers discover the deprecation;
- documents why the change is happening and how to migrate;
- communicates the eventual removal date in advance;
- removes the old pattern only after consumers have migrated.

Silent removals and behavior changes are the fastest way to destroy trust in a library.

### Manage Breaking Changes Through Migration, Not Surprise

Breaking changes are sometimes necessary, but they must be managed as migrations, not delivered as surprises. The cost of a breaking change is paid by every consumer, and that cost must be planned for.

For breaking changes:

- assess the blast radius before committing, by surveying usage;
- provide a codemod, migration guide, or compatibility layer where feasible;
- stage the change: announce, deprecate, provide migration tooling, then break;
- offer a bridge period where both old and new work;
- support major consumers through the migration.

A breaking change shipped without migration support forces every consumer to absorb the cost at once, often by forking the library.

### Govern Contributions To Prevent Drift And Bloat

Contribution is how a library stays relevant to real needs, but ungoverned contribution is how it bloats and fragments. Every accepted contribution becomes a maintenance burden and a precedent.

Govern contributions by:

- defining what qualifies for inclusion (recurrence, clarity, sustainability);
- requiring documentation, examples, and usage boundaries with each contribution;
- reviewing for consistency with existing patterns, tokens, and conventions;
- declining one-off or team-specific solutions that do not generalize;
- assigning ownership for accepted contributions so they are maintained.

A library that accepts every contribution becomes unsearchable; one that accepts none becomes irrelevant. The balance is governed contribution.

### Audit Real Usage To Keep The Library Honest

A library that is not audited against real usage drifts into fiction: documented patterns no one uses, real patterns no one documented, and components that have evolved away from their examples. Regular audits keep the library honest.

Audit for:

- adoption of each pattern and component, to retire the unused;
- undocumented patterns that have emerged in product code, to decide whether to adopt or replace them;
- drift between documented examples and live components, to resync;
- duplicated or competing patterns that should be consolidated;
- accessibility, responsive, and state gaps that surfaced in real use.

The library should describe how the product actually works, not how it was intended to work years ago.

### Reconcile Drift Between Library And Product

Drift is inevitable; the question is whether it is reconciled. When products diverge from the library, the divergence should be a signal, not a silent fact.

Reconcile by:

- investigating why a product diverged (missing capability, wrong pattern, local preference);
- deciding whether to absorb the divergence into the library or to correct the product;
- documenting the decision so the same divergence does not recur;
- updating the library when the divergence reveals a real gap.

Unreconciled drift accumulates until the library and the product describe two different systems.

### Plan For Long-Term Sustainability

A library is a long-lived artifact, and its governance must be sustainable, not heroic. Processes that depend on a single overworked maintainer or on constant manual sync will fail.

Sustainable practices:

- automate what can be automated (example syncing, linting, changelog generation);
- distribute ownership across maintainers and categories;
- keep the contribution and review process lightweight enough to be used;
- budget maintenance time explicitly, rather than treating it as leftover work;
- retire aggressively so the maintained set stays manageable.

## Common Traps

### No Ownership Or Review

A library without owners accumulates unreviewed changes, defects, and contradictions until no one trusts it.

### Unversioned Or Silent Changes

Changing patterns without versioning or communication turns routine maintenance into breaking changes for consumers.

### Removal Without Deprecation

Deleting or changing a pattern without a migration window strands consumers and destroys trust in the library.

### Breaking Changes Without Migration Support

Shipping breaking changes without codemods, compatibility layers, or bridge periods forces every consumer to absorb the cost at once.

### Ungoverned Contribution

Accepting every requested pattern bloats the library with overlapping, one-off, or unsustainable entries that bury the useful ones.

### Library That Drifts From Real Usage

Without audits, the library documents patterns no one uses while real patterns live undocumented in product code.

### Unreconciled Divergence

When products diverge from the library and the divergence is ignored, the library and the product gradually describe two different systems.

### Unsustainable Governance

Processes that depend on a single maintainer or constant manual effort collapse under the library's long-term maintenance load.

## Self-Check

- [ ] The library has named owners and maintainers, a defined change process, a review cadence, and a channel for consumer issues and requests.
- [ ] Patterns and components are versioned, with breaking changes distinguished from non-breaking ones, a readable changelog, and release communication.
- [ ] Deprecations mark replacements, keep the old pattern working during a stated window, signal the deprecation, document the migration, and communicate removal in advance.
- [ ] Breaking changes are staged with migration guidance, compatibility layers, bridge periods, and support for major consumers.
- [ ] Contributions are governed by inclusion criteria, required documentation, consistency review, and ownership assignment, and one-off solutions are declined.
- [ ] Real usage is audited regularly to retire unused patterns, absorb undocumented ones, resync examples, consolidate duplicates, and close accessibility and state gaps.
- [ ] Divergence between library and product is investigated, reconciled by absorbing or correcting, and documented to prevent recurrence.
- [ ] Governance is sustainable: automation, distributed ownership, lightweight process, explicit maintenance budget, and aggressive retirement.
- [ ] The library was reviewed for whether it still describes how the product actually works, not only how it was intended to work.
