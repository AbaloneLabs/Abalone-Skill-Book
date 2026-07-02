---
name: scope_and_complexity_assessment.md
description: Use when the agent is scoping a feature or change, assessing its complexity, deciding what is in versus out of scope, identifying hidden integration costs, evaluating whether a change is really small, spotting scope creep, or sizing the full cost of a request before committing to it.
---

# Scope And Complexity Assessment

The phrase "it's just a small change" is the most expensive sentence in software. It is spoken at the start of work that turns out to touch a dozen modules, requires a migration, breaks an implicit contract three teams depend on, and needs a rollback path no one planned for. The gap between the visible change (the one line, the one field, the one endpoint) and the real change (everything that line connects to, depends on, or constrains) is where schedules blow up and incidents are born. Scoping is the discipline of seeing the real change before committing to it.

Agents tend to scope by looking at the surface: the feature as described, the file to edit, the obvious code path. They miss the integration points, the migration costs, the test debt, the implicit contracts, and the second-order effects that only appear once the change is underway. The result is the chronic under-scoping that makes "small changes" consume sprints and "quick wins" cause outages. The judgment problem is to map the full blast radius of a change — what it touches, what it depends on, what depends on it, and what must change alongside it — before estimating or committing, and to push back on scope that has grown past its justification.

## Core Rules

### Map The Blast Radius Before Estimating

A change's cost is not the cost of writing the new code; it is the cost of everything that must change for the new code to coexist correctly with the existing system. Before estimating, trace the change outward: who calls this, what depends on this behavior, what contracts does this create or break, what data does this touch, what must be migrated, what must be tested, and what must be coordinated with other teams or systems.

Concretely, for any non-trivial change, identify:

- **Callers and dependents** — every place that consumes the code, data, or contract being changed, and what breaks if the change is visible to them.
- **Data and schema** — whether the change requires a migration, a backfill, a dual-write period, or coordination with existing data.
- **Contracts and compatibility** — whether the change alters a public API, an error semantic, an ordering guarantee, or a format that consumers rely on.
- **Tests and verification** — what tests exist for the affected behavior, what tests must be added, and how the change will be verified in a way that catches regressions.
- **Rollout and rollback** — how the change ships (all-at-once, behind a flag, incrementally) and how it reverts if something breaks.
- **Coordination** — which other people, teams, or systems must change in step.

Each item is real work. An estimate that counts only the new code has silently omitted most of the actual cost.

### Treat "Just A Small Change" As A Hypothesis To Test

When a request is described as small, treat that as a claim to verify, not a fact. Small changes are small only when they are isolated — when nothing else depends on the thing being changed, no contract shifts, no migration is needed, and the tests already cover the behavior. The moment any of those conditions fail, the change is no longer small, regardless of how few lines it touches.

Before accepting "small," ask:

- Does anything depend on the current behavior that the change alters?
- Does the change require data, schema, or configuration to move?
- Is there a test that will catch it if the change is wrong?
- Can it be reverted independently if it breaks?

If the answer to any is no, the change is larger than its line count suggests. Surface this before committing, not after. The cost of a wrong "small" estimate is borne during the change, under pressure, when it is too late to resize.

### Identify Hidden Complexity Early

Much complexity is invisible from the feature description and only surfaces when the implementation begins. The scoping job is to surface it before the commitment, by looking for the patterns that reliably hide work:

- **Implicit contracts** — behavior others depend on without it being documented (ordering, caching, error semantics, null handling). Changing these breaks consumers who did not know they were depending on them.
- **Cross-cutting concerns** — auth, logging, observability, rate limiting, i18n. A feature that ignores these is incomplete; one that must thread them through is larger than it looks.
- **Concurrency and ordering** — changes that introduce or alter shared mutable state, retries, or eventual consistency carry correctness risks that require careful design and testing.
- **Backward and forward compatibility** — changes that must coexist with old code during rollout (old clients, old servers, old data) need compatibility layers and migration paths.
- **Test debt** — code with poor test coverage costs more to change safely, because the safety net must be built or the risk accepted.

Name the hidden complexity explicitly in the scope. "This looks like one endpoint, but it requires a schema migration, a backfill, a compatibility layer for old clients, and new tests" is a far more honest scope than "add an endpoint."

### Account For Integration Cost, Not Just Implementation Cost

Building a component in isolation is the cheap part. Connecting it to the rest of the system — wiring it into the existing flows, handling the existing data shapes, satisfying the existing contracts, deploying it alongside the existing infrastructure, and verifying it does not break the existing behavior — is where the effort and risk concentrate. Integration is consistently underestimated because it is where the unknowns live: the other system's actual behavior, its edge cases, its undocumented assumptions.

When scoping, separate implementation from integration and estimate both. A feature that is a week of implementation and three weeks of integration is not a one-week feature; it is a four-week feature, and the integration weeks are where the surprises will arrive. Plan verification and a rollout strategy that assumes integration will surface problems.

### Make Scope Decisions Explicit And Defended

Scope is a set of choices about what is in and what is out. When those choices are implicit, scope creeps: each addition feels small in isolation, and the cumulative growth is invisible until the change is far larger than planned. Make scope explicit — what this change will and will not do — and defend it when additions are proposed.

For each proposed addition during the work, ask:

- Does it serve the original goal, or is it a separate goal that should be a separate change?
- Is the added cost (including integration, testing, and rollout) justified by the value?
- Does it increase the risk or the rollback complexity of the core change?

It is legitimate to expand scope deliberately, with eyes open, when the value justifies it. It is not legitimate to let scope grow by accretion, where each "while we're here" addition bloats the change and entangles its risk without a decision being made. A change that does one thing well is reviewable, testable, and revertable; a change that accreted five things is none of those.

### Separate The Must-Have From The Nice-To-Have

Not every part of a change carries equal necessity. Distinguishing the core (without which the change has no value) from the optional enhancements lets the change ship in a useful state even if time or circumstances force a cut. This is not about lowering quality; it is about sequencing so that the essential value is delivered and verified before the optional work is layered on.

Define the minimum viable version of the change — the smallest thing that delivers the core value safely — and treat the rest as subsequent changes. This protects against the failure mode where a feature is 90% done for months because the last 10% (the polish, the edge cases, the optional integrations) blocks the whole thing from shipping.

## Common Traps

### Estimating The Lines, Not The Blast Radius

Counting the code to be written while ignoring the callers, contracts, data, and tests that must change produces estimates that are structurally low. Always map the blast radius first; the visible code is usually the minority of the work.

### Trusting "Small" Without Verifying Isolation

A change is small only when it is isolated. Accepting "small" without checking dependencies, contracts, migrations, and test coverage leads to the chronic under-scoping that turns quick wins into sprints. Verify isolation before accepting the size.

### Ignoring Integration And Compatibility Work

Building in isolation is cheap; integrating with the existing system is where the cost and risk concentrate. Scope that counts only implementation and omits integration, compatibility layers, and rollout planning is missing its largest component.

### Scope Creep By Accretion

Allowing "while we're here" additions to accumulate without explicit decisions bloats the change, entangles its risk, and makes it unreviewable and unrevertable. Make each scope expansion a deliberate, justified choice.

### Missing The Implicit Contracts

Changing undocumented behavior that others depend on (ordering, error semantics, caching) breaks consumers who did not know they were depending on it. These contracts are invisible until they break; look for them during scoping, not during the incident.

### Bundling Optional Work Into The Critical Path

Tying the essential value to optional enhancements means the whole change is blocked by the least important part. Separate must-have from nice-to-have so the core can ship and be verified before the rest is layered on.

### Accepting Scope Without A Rollback Plan

A change that cannot be reverted independently is riskier than its size suggests, because a failure forces a forward fix under pressure rather than a clean rollback. Scope includes the rollout and rollback strategy; a change without one is incompletely scoped.

## Self-Check

- [ ] The blast radius was mapped before estimating: callers, dependents, data and schema, contracts and compatibility, tests, rollout, rollback, and required coordination are all identified.
- [ ] "Small" was verified by checking isolation (no dependents on changed behavior, no migration, existing tests, independent revert) rather than accepted from the line count.
- [ ] Hidden complexity — implicit contracts, cross-cutting concerns, concurrency, compatibility, test debt — was named explicitly in the scope rather than discovered mid-change.
- [ ] Implementation cost and integration cost were estimated separately, and integration (where the unknowns live) was not assumed to be trivial.
- [ ] Scope is explicit about what is in and out, and each expansion during the work was a deliberate, justified decision rather than accretion.
- [ ] The minimum viable version of the change is defined, so the core value can ship and be verified before optional work is layered on.
- [ ] The rollout and rollback strategy is part of the scope; the change can be reverted independently if it breaks.
- [ ] The estimate reflects the full change (implementation, integration, testing, migration, rollout), not just the visible new code.
