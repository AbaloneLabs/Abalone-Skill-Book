---
name: design_quality_governance.md
description: Use when the agent is establishing or reviewing design quality governance, including design system stewardship, contribution models, deprecation policy, review authority, quality bars, audit processes, ownership boundaries, and how design standards are enforced and evolved across a product organization.
---

# Design Quality Governance

Quality governance is the system of rules, owners, and decisions that keeps a design organization's output consistent and improving as it scales. Without governance, a design system becomes a museum of past decisions, reviews become opinion battles, and quality bars drift until they mean nothing. With over-governance, contribution stalls, the system becomes a bottleneck, and teams route around it with unofficial patterns.

Use this skill when defining who owns design quality, how the system evolves, how contributions are accepted or rejected, and how standards are enforced without becoming bureaucracy. The goal is to prevent the agent from proposing either a laissez-faire model where anything ships, or a gatekeeping model where nothing moves.

## Core Rules

### Distinguish Governance From Ownership Of Execution

Governance is about who decides the rules and how they change. Execution is about who does the work. Confusing the two produces teams that either over-control day-to-day work or abdicate responsibility for the system.

Clarify separately:

- **System owners**: the people accountable for the design system's direction, tokens, and core components.
- **Feature teams**: the people who consume and occasionally contribute to the system while shipping product work.
- **Review authority**: who can approve changes to the system versus changes within an existing pattern.
- **Escalation path**: where disagreements about quality or fit are resolved.

Governance should set the boundaries; execution should happen inside them.

### Define A Contribution Model That Matches Scale

A design system that no one can contribute to dies. A system that anyone can change without review fragments. The contribution model must balance openness with integrity.

Common models, each with tradeoffs:

- **Closed core, open request**: feature teams request additions; system team builds or approves. High consistency, slower throughput.
- **Federated contribution**: designated contributors from each team can propose and build, with system team review. Faster, requires clear standards.
- **Open contribution with review**: anyone proposes; maintainers review against published criteria. Fastest, highest review load.

Choose the model that matches team size and maturity. A small team does not need a federated model; a large organization cannot survive a closed core.

### Make The Quality Bar Explicit And Calibrated

"High quality" is meaningless without a shared definition. A quality bar must be concrete enough that two reviewers applying it reach similar conclusions.

A useful quality bar specifies:

- the craft dimensions that matter, such as visual alignment, spacing rhythm, typography, motion, and interaction states;
- the required coverage, including empty, loading, error, and edge states;
- the accessibility requirements that are non-negotiable;
- the consistency requirements against tokens and components;
- the evidence required, such as tested with real content or reviewed on device.

Calibrate the bar periodically by reviewing past decisions together. Without calibration, reviewers drift apart and decisions feel arbitrary.

### Govern Deprecation As Actively As Addition

Most governance attention goes to adding new components. But a system's health depends equally on removing or deprecating what no longer fits. Without a deprecation policy, the system accumulates redundant variants, legacy tokens, and patterns that contradict current direction.

A deprecation policy should define:

- the criteria that trigger deprecation, such as low usage, duplication, or conflict with a newer pattern;
- the migration path and who owns it;
- the timeline and communication for removal;
- whether the old pattern is frozen, discouraged, or blocked.

Deprecation without a migration path is abandonment. Deprecation with a clear path is healthy evolution.

### Separate Standards From Preferences

Governance must enforce standards and leave room for preference. Standards are the rules that protect consistency, accessibility, and brand. Preferences are stylistic choices that do not affect system integrity.

Enforce as standards:

- token usage and component adoption;
- accessibility and interaction requirements;
- state coverage and error handling;
- naming and structural conventions.

Leave as preferences:

- minor copy tone within a surface;
- illustrative or photographic direction within brand bounds;
- micro-layout choices that do not affect the system.

Reviewing preferences as standards burns credibility and makes teams resist governance.

### Establish Audit Cadences Without Burdening Delivery

Audits surface drift before it becomes structural. But audits that require stopping delivery become resented and skipped. Design audits into the rhythm of work rather than against it.

Effective audit practices:

- sample surfaces regularly rather than auditing everything at once;
- focus each audit on a specific risk, such as token drift, accessibility, or state coverage;
- produce actionable findings tied to owners, not a generic report;
- feed findings back into the system as new checks or documentation.

An audit that produces a slide deck and no changes is theater.

### Make Decisions And Rationale Visible

Governance loses legitimacy when decisions are opaque. When a contribution is rejected or a pattern is deprecated, the rationale should be recorded so future decisions are consistent and contributors can learn.

Maintain:

- decision records for significant system changes;
- published criteria for acceptance and deprecation;
- a changelog that explains what changed and why;
- examples of accepted and rejected contributions with reasoning.

Visibility turns governance from gatekeeping into a shared standard.

## Common Traps

### Gatekeeping Without Published Criteria

When acceptance criteria live only in maintainers' heads, contributions feel arbitrary and teams stop proposing them. Governance without transparency becomes a bottleneck.

### Letting The System Become A Museum

Preserving every past decision for backward compatibility prevents evolution. Without deprecation, the system grows until no one can hold it in mind.

### Reviewing Preferences As Standards

Enforcing personal taste under the banner of governance erodes trust. Teams learn to avoid review rather than benefit from it.

### Audit As Theater

Producing audit reports that no one acts on consumes effort without changing quality. Findings must have owners and follow-through.

### Governance Owned By Absent Stakeholders

When system ownership is assigned to people who have no time or context, decisions stall or default to the loudest voice. Ownership must come with capacity.

### Confusing Consistency With Uniformity

Governance should protect coherence, not force every surface to look identical. Over-uniformity kills appropriate variation and makes products feel generic.

## Self-Check

- [ ] Governance ownership is distinguished from execution ownership, with clear authority and escalation paths.
- [ ] A contribution model is chosen that matches team scale, balancing openness with system integrity.
- [ ] The quality bar is explicit, concrete, and calibrated across reviewers through periodic review.
- [ ] A deprecation policy exists with criteria, migration paths, timelines, and communication.
- [ ] Standards are enforced while preferences are left to teams, and the boundary is documented.
- [ ] Audits are cadenced, focused on specific risks, and produce actionable findings with owners.
- [ ] Significant decisions and their rationale are recorded and visible to contributors.
- [ ] Governance protects coherence without forcing inappropriate uniformity.
