---
name: design-system-governance.md
description: Use when the agent is governing a design system, deciding how components are proposed and accepted, managing contribution and versioning, handling deprecation and breaking changes, defining ownership and roles, or scaling a design system across many teams without fragmentation.
---

# Design System Governance

A design system is not just a library of components; it is a product with users (the teams who consume it), a roadmap, contributors, and a lifecycle. Governance is the set of decisions and processes that determine whether the system scales coherently or fragments into noise. Agents frequently treat a design system as a static artifact: build the components, publish them, done. In reality, without governance, the system drifts — teams fork components, duplicate patterns proliferate, contributions pile up unreviewed, deprecated patterns never die, and the system that was meant to create consistency becomes a source of inconsistency.

The judgment problem is balancing control against velocity, standardization against legitimate variation, and stability against evolution. Too little governance and the system fragments; too much and teams bypass it because contributing or adopting is too slow. This skill helps the agent establish governance that keeps the system trustworthy, adopted, and alive.

## Core Rules

### Treat the design system as a product with users and a roadmap

A design system serves consuming teams, and like any product it needs a clear value proposition, an understanding of its users' needs, a roadmap, and a feedback loop. Governance that ignores this treats the system as a one-time deliverable and lets it stagnate. Define who the system serves, what problems it solves for them, how success is measured, and how the roadmap is set with input from consumers. A system without a product mindset decays.

### Define clear ownership, roles, and decision rights

Governance fails when no one owns decisions or when everyone does. Establish who maintains the core system, who reviews contributions, who decides what is accepted or deprecated, and how disputes are resolved. Distinguish core maintainers from contributors from consumers, and make decision rights explicit. Ambiguous ownership produces either paralysis (no one decides) or fragmentation (everyone decides differently).

### Establish a contribution model with a real path to acceptance

If consuming teams cannot influence the system, they fork it. A healthy contribution model lets teams propose new components, variants, or fixes, with a transparent process for review, acceptance, and release. Define what makes a pattern ready for the system (generality, demand, quality), how proposals are evaluated, and the timeline for decisions. A contribution process that is opaque or too slow drives teams to build local alternatives, defeating the system's purpose.

### Govern adoption, not just creation

A system only creates consistency if teams actually use it. Governance includes adoption: tracking which teams use which version, identifying barriers to adoption, providing migration support, and making the system easier to consume than building local. Adoption metrics, office hours, documentation quality, and migration tooling are governance concerns, not afterthoughts. A beautifully built system that no one adopts creates no value.

### Manage versioning, deprecation, and breaking changes deliberately

Components evolve, and evolution without discipline breaks consumers. Establish a versioning policy that signals what changes are safe versus breaking, a deprecation process that gives consumers time to migrate, and clear communication of changes. Never remove or break a pattern without a documented migration path and notice. Abrupt breaking changes destroy trust and cause teams to pin old versions or avoid the system.

### Balance standardization with legitimate variation

A design system enforces consistency, but not every context is identical. Brand variations, regional differences, dense data applications, and marketing surfaces may legitimately diverge from the core. Governance must define what is standardized, what is configurable, and what is out of scope, so that variation is intentional and governed rather than accidental. Blanket rigidity drives teams outside the system; unbounded flexibility produces fragmentation.

### Maintain quality and consistency through review

Acceptance into the system should require meeting quality bars: accessibility, visual consistency, code quality, documentation, and testing. A review process that lets in low-quality components pollutes the system and erodes trust. Define the quality criteria, who reviews, and how feedback is incorporated. The system's reputation depends on the reliability of what it contains.

### Keep documentation and communication as first-class

A component without documentation is half-built. Governance must ensure every pattern has usage guidance, do and do-not examples, accessibility notes, and code references, and that changes are communicated to consumers through release notes, changelogs, and announcements. Undocumented or silently changed components cause misuse and breakage.

## Common Traps

### Building the system once and assuming it is done

The most common failure is treating the design system as a deliverable rather than a living product. Without ongoing maintenance, contribution review, and roadmap, the system decays and consumers drift away.

### No clear ownership leading to paralysis or fragmentation

When decision rights are ambiguous, either nothing gets decided (paralysis) or everyone decides differently (fragmentation). Explicit ownership and roles are non-negotiable for a system of any scale.

### Contribution process too slow or opaque

If teams cannot get patterns accepted in reasonable time, they build local versions, and duplication proliferates. A contribution model that does not respect consumers' timelines defeats adoption.

### Abrupt breaking changes without migration paths

Removing or changing components without notice and migration support breaks consumers and destroys trust. Teams respond by pinning versions or abandoning the system.

### Governing creation but not adoption

Measuring what is built but not what is consumed misses the point. A system with low adoption creates no consistency. Track and actively drive adoption.

### Rigidity that drives teams outside the system

Enforcing identical patterns across contexts that legitimately differ pushes teams to bypass the system. Govern variation intentionally rather than forbidding all divergence.

### Letting low-quality contributions in

Accepting components without quality review pollutes the system. One unreliable component damages trust in the whole library.

### Silent changes and missing documentation

Changing or releasing components without documentation and communication causes misuse and breakage. Documentation and changelogs are governance obligations, not extras.

## Self-Check

- Is the design system treated as a product with defined users, value proposition, roadmap, and success metrics?
- Are ownership, roles, and decision rights explicit, with a clear path for resolving disputes?
- Is there a transparent contribution model with defined readiness criteria, review process, and timelines?
- Are you tracking and actively driving adoption, not only measuring what was built?
- Is there a versioning and deprecation policy that protects consumers with notice and migration paths?
- Have you defined what is standardized, configurable, and out of scope to govern variation intentionally?
- Does acceptance require meeting quality bars for accessibility, consistency, code, documentation, and testing?
- Does every component have documentation, do and do-not examples, and communicated changes?
