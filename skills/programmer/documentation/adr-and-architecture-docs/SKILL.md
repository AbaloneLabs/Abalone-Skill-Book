---
name: adr_and_architecture_docs.md
description: Use when the agent is recording an architecture or design decision, writing an ADR, capturing why an alternative was rejected, preserving context for a future maintainer, deciding whether to document a design at all, or auditing whether existing architecture docs are still trustworthy.
---

# ADR And Architecture Docs

Architecture decisions are made under pressure with partial information, and the reasoning that produced them is the first thing lost. Six months later, a maintainer stares at a design choice that looks odd, cannot tell whether it was deliberate or accidental, and either reveres it as load-bearing (freezing a decision that should evolve) or rips it out (re-breaking a constraint that was solved on purpose). An Architecture Decision Record exists to carry that reasoning forward so the decision can be understood, challenged, and changed with full knowledge of why it was made.

Agents tend to fail at architecture documentation in two opposite directions. The first is writing nothing — the decision lives in a chat thread or a meeting that evaporates, and the codebase is left with conclusions and no context. The second is over-documenting — a sprawling design doc that narrates every option at length, goes stale the moment the code changes, and becomes a document everyone cites and no one trusts. The judgment problem is to record the decisions that will be hard to reverse or re-derive, at the right level of detail, in a form that stays useful as the system evolves.

## Core Rules

### Record Decisions That Will Be Hard To Re-Derive

Not every choice deserves an ADR. A decision merits a record when future maintainers will struggle to reconstruct the reasoning from the code alone — typically because the constraint that drove it is no longer visible, the alternative looked more obvious, or the cost of reversing it is high. A naming convention does not need an ADR; a choice to reject the standard library's concurrency primitive in favor of a custom scheduler does.

Useful triggers for an ADR:

- A decision that constrains many parts of the system (a data store choice, an auth model, a consistency level).
- A decision that rejected an option which will look obviously better to a future reader (we chose eventual consistency despite strong consistency seeming safer).
- A decision driven by a constraint that may disappear (a latency budget, a compliance requirement, a team structure).
- A decision that is expensive or risky to reverse (a data format, a public API shape, a deployment topology).

If a decision is cheap to reverse and obvious from the code, a record adds overhead without value. Reserve ADRs for the decisions where lost context would cause real harm.

### Capture Context, Decision, And Consequences — Not Just The Outcome

The least useful ADR states only the conclusion: "we use X." A future reader already knows you use X; they can see it in the code. The value is in everything that led to the conclusion and everything it implies. A durable ADR structure covers:

- **Context** — the problem, the constraints, the forces in play at the time. What were we trying to achieve, what were we constrained by, what was the state of the system and the team?
- **Decision** — what we chose, stated precisely enough to act on.
- **Alternatives considered** — the options we rejected, and crucially *why* each was rejected. This is the most valuable and most often omitted section.
- **Consequences** — what this choice gives us, what it costs us, what it rules out, and what we will need to revisit.

The alternatives section deserves emphasis. A future reader who sees only the chosen option will rediscover the rejected alternatives and may re-litigate a settled decision, or worse, re-make a mistake that was already understood and avoided. Recording the rejected path — and the specific reason it failed — closes that loop permanently.

### Record The Constraints And Assumptions, Including The Ones That May Change

Many architectural decisions are driven by constraints that are temporary or situational: a team size, a budget, a compliance regime, a technology's maturity at the time, a deadline. When the constraint is invisible in the ADR, the decision looks arbitrary or outdated. When the constraint is recorded, a future reader can tell whether the decision still applies.

Make assumptions explicit and dated. "We chose a monolith because we have three engineers and one deployable is simpler to operate" is a decision that should be revisited when the team grows. "We chose this queue because it was the only one with feature X" is a decision that should be revisited when alternatives gain feature X. An ADR that names its sell-by conditions lets the system evolve; one that does not freezes decisions past their useful life.

### Keep ADRs Immutable And Append-Only

An ADR is a historical record of a decision made at a point in time. Editing an old ADR to reflect a new reality destroys the very context it exists to preserve. If a decision is superseded, do not rewrite the original — mark it superseded and write a new ADR that references it, explains what changed, and records the new decision.

This append-only discipline has two benefits. It preserves the full history of reasoning, so a future reader can trace how the architecture evolved. And it lowers the threshold for writing an ADR, because the author knows the document will not need to be maintained — it captures a moment, and later moments get their own records. Link superseded ADRs to their successors so a reader landing on an old one can find the current state.

### Keep Architecture Docs Current Enough To Trust

An ADR is immutable, but living architecture docs (overviews, diagrams, current-state descriptions) must track the system or they become actively misleading. A diagram that shows three services when there are now seven sends readers to the wrong places; a doc that describes a data flow that no longer exists causes new contributors to build on a fiction.

Treat living docs differently from ADRs:

- Living docs describe the *current* state and must be updated when the state changes; couple their update to the change, or mark them with a freshness signal and an owner.
- ADRs describe *past* decisions and are never rewritten, only superseded.
- Where a living doc and an ADR would say the same thing, prefer the ADR and let the living doc link to it, so there is one source of truth.

A living doc that no one owns is guaranteed to rot. Every living architecture doc should have an owner accountable for its accuracy, or it should be generated from the system (e.g., service graphs from discovery) so it cannot drift.

### Right-Size The Documentation To The Decision's Half-Life

Documentation effort should scale with how long the decision will matter and how many people it will affect. A decision that shapes the system for years and is read by every new contributor deserves a thorough ADR. A decision that will be moot in a quarter and affects one team needs a paragraph, not a treatise.

Over-documenting is as real a failure as under-documenting. A sprawling design doc for a trivial choice buries the signal, takes time to write and read, and rots faster because no one maintains it. A terse ADR for a load-bearing decision leaves the reasoning fragile. Match the depth to the stakes, and resist the instinct to document everything to the same level of detail.

## Common Traps

### The Conclusion-Only ADR

An ADR that states "we chose PostgreSQL" with no context, alternatives, or consequences preserves nothing the code does not already show. The reader learns the what and remains ignorant of the why, which is the entire point of the record. Always include context and rejected alternatives.

### Editing History Instead Of Superseding

Rewriting an old ADR to match a new decision erases the reasoning that made the original decision make sense, and hides the fact that a change occurred. A reader cannot tell whether the system was always this way or was migrated. Mark superseded and append a new record instead.

### The Living Doc That Quietly Rots

An architecture overview or diagram that no one updates drifts until it describes a system that no longer exists, then actively misleads the readers who trust it. Either couple its update to code changes, generate it from the system, or mark it stale with an owner. A trusted-but-wrong doc is more dangerous than no doc.

### Over-Documenting To Feel Thorough

Writing a long design doc for every minor decision dilutes the value of the records that matter and creates maintenance burden that guarantees rot. Reserve ADRs for decisions where lost context has a real cost, and keep them as short as the decision warrants.

### Omitting The Rejected Alternatives

The chosen option is the least interesting part of an ADR, because it is visible in the code. The rejected options and their failure reasons are what prevent a future reader from re-litigating or re-making a mistake. An ADR without an alternatives section fails its core purpose.

### Assuming Constraints Are Eternal

Recording a decision without the constraint that drove it makes the decision look permanent when it may be situational. A team-size constraint, a budget constraint, or a technology-maturity constraint can disappear, and the decision should be revisited. Name the constraint and the conditions under which the decision should be re-examined.

### Documentation That No One Owns

A living architecture doc without an owner rots by default, because no one is accountable when the system changes. Assign ownership or generate from source; an unowned doc is an abandoned doc.

## Self-Check

- [ ] The decision recorded in the ADR is one whose reasoning would be hard to re-derive from the code alone — cheap-to-reverse or obvious decisions were not over-documented.
- [ ] The ADR captures context, the decision, the alternatives considered and *why each was rejected*, and the consequences — not just the conclusion.
- [ ] Constraints and assumptions are stated explicitly and, where situational, include the conditions under which the decision should be revisited.
- [ ] Existing ADRs are immutable and append-only; superseded decisions are marked and linked to their successors rather than rewritten.
- [ ] Living architecture docs (overviews, diagrams) are coupled to code changes, generated from the system, or marked with an owner and freshness signal so drift is visible.
- [ ] Documentation depth matches the decision's half-life and reach — load-bearing decisions are thorough, trivial ones are terse or absent.
- [ ] A reader landing on any ADR can tell whether it is current or superseded and can trace the chain to the current decision.
- [ ] No living doc is trusted without an owner or generation mechanism that keeps it accurate.
