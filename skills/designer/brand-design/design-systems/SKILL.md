---
name: design_systems.md
description: Use when the agent is building or scaling a design system, defining component architecture, establishing contribution and governance processes, deciding what belongs in the system versus product-specific, managing versioning and releases, or ensuring design and engineering share a single source of truth for UI patterns.
---

# Design Systems

A design system is the shared language that lets a product be built consistently at scale. It looks like a component library, but it is really a social and technical contract between design and engineering about what UI patterns exist, how they are named, how they compose, and who maintains them. Agents tend to treat a design system as a folder of components, build it without governance, or let it grow until it duplicates itself. The harm is invisible at first and expensive later: teams rebuild the same patterns differently, the system fragments into incompatible versions, and updates that should propagate do not.

Use this skill before finalizing system architecture, contribution rules, or component APIs. The goal is to prevent the agent from building a system no one owns, from making components too rigid to adopt or too loose to be consistent, or from creating a library that solves yesterday's problems and cannot evolve.

## Core Rules

### Treat The System As A Product, Not A Project

A design system is never finished. It has users (the product teams who consume it), stakeholders, a roadmap, and maintenance burden. Treating it as a one-time project produces a system that launches, decays, and is eventually abandoned or forked. Sustainable systems are run as ongoing products with dedicated ownership.

Run the system as a product:

- assign explicit owners responsible for the system's health, not just its creation;
- maintain a roadmap driven by consumer needs, not internal preferences;
- budget for ongoing maintenance, bug fixes, and deprecation;
- measure adoption and treat low adoption as a signal to improve, not a reason to mandate.

A system with no owner becomes an orphan that teams fork rather than contribute to. Ownership is the single most important predictor of system survival.

### Define Clear Boundaries Between System And Product

Not everything belongs in the design system. A common failure is pushing product-specific patterns into the shared system, bloating it with one-off components that other teams never use. Equally harmful is keeping generalizable patterns in product code, where they are rebuilt everywhere. The boundary must be deliberate.

Establish what belongs in the system:

- include patterns that are used across multiple teams or products;
- keep product-specific layouts and flows in the product, not the system;
- define the criteria for promotion: a pattern earns system inclusion when it is reused, not when it is merely reusable;
- define the criteria for demotion: patterns no longer used should be deprecated, not left to rot.

A system that contains everything is unusable; a system that contains too little is bypassed. The boundary is a continuous judgment, not a one-time decision.

### Design Components For Composition, Not Just Appearance

A component that looks right in isolation often fails when combined. Real interfaces compose components: a button inside a card inside a grid. If components are designed without considering how they nest, spacing doubles, alignment drifts, and the assembled result looks broken even though each piece is fine.

Build for composition:

- design components to sit inside containers without imposing their own outer spacing;
- let layout spacing be controlled by the parent, not baked into the child;
- test components in realistic assemblies, not just in isolation;
- define how components behave at different widths, densities, and content lengths.

The unit of design is not the component but the composition. A system whose components only work alone is a collection, not a system.

### Establish A Single Source Of Truth Shared By Design And Engineering

The most damaging split is when design and engineering maintain separate definitions of the same patterns. Designers reference one set of tokens and components in their tool; engineers implement another in code. The two drift, and what ships does not match what was designed. A true system unifies these.

Unify the source of truth:

- ensure design tokens (color, spacing, type) are defined once and consumed by both design tools and code;
- version the system so design and engineering reference the same release;
- make the component library in code the implementation of the components in design, with clear mapping;
- avoid shadow systems where teams maintain local overrides that diverge.

When design and code disagree, trust erodes and teams stop relying on the system. A single source of truth is the foundation that makes the system credible.

### Define Contribution And Governance Processes

A system that no one can contribute to becomes a bottleneck, and teams bypass it. A system that anyone can change without review becomes inconsistent. The governance model determines whether the system grows healthily or chaotically.

Establish governance:

- define how teams propose new components or changes, with a clear intake process;
- require review by system owners before inclusion, checking for reusability, consistency, and quality;
- document the criteria for acceptance so proposals are evaluated consistently;
- recognize and reward contribution so teams are motivated to give back rather than fork.

Governance that is too strict stifles adoption; governance that is too loose fragments the system. The right balance invites contribution while protecting coherence.

### Version And Release Deliberately

A design system consumed by many teams cannot change silently. Unversioned changes break consumers without warning, destroying trust. A mature system versions its releases, communicates changes, and gives consumers control over when to upgrade.

Manage versioning:

- use semantic versioning so consumers understand the impact of an update;
- maintain a changelog that documents what changed, why, and how to migrate;
- support a deprecation path: old patterns are marked deprecated before removal, with migration guidance;
- communicate releases through channels consumers actually read.

Breaking changes without warning or migration support cause teams to pin old versions indefinitely, fragmenting the ecosystem. Predictable releases build the trust that keeps teams current.

### Document Usage, Not Just Anatomy

A component documented only by its properties is half-documented. Teams need to know when to use it, when not to, and how it relates to alternatives. Documentation that omits usage guidance leads to the wrong component being chosen for the job, producing inconsistent experiences even when the system is adopted.

Document completely:

- show the component's anatomy and available properties;
- state when to use it and when to use an alternative;
- provide examples of correct and incorrect usage;
- document accessibility requirements and behavior;
- link related components so teams understand the options.

Usage guidance is what turns a component library into a design system. Without it, teams have the parts but not the judgment to assemble them.

### Measure Adoption And Health

A design system's value is realized only when consumed. Without measurement, the team cannot tell whether the system is helping or has been abandoned. Adoption metrics reveal where the system is used, where it is bypassed, and where investment should go.

Track system health:

- measure how many teams and surfaces consume the system;
- track contribution volume and response times to proposals;
- monitor bug reports and issues to find fragile components;
- survey consumers about satisfaction and unmet needs.

Measurement prevents the team from optimizing for the system's internal elegance rather than its actual usefulness.

## Common Traps

### Treating The System As A One-Time Project

A system with no ongoing ownership and roadmap decays and is abandoned; run it as a product with dedicated owners.

### Pushing Product-Specific Patterns Into The System

One-off components bloat the system; include only patterns reused across teams, with clear promotion criteria.

### Components That Only Work In Isolation

Components designed without composition produce broken assemblies; design for nesting and let parents control spacing.

### Separate Design And Code Sources Of Truth

When design tools and code maintain different definitions, what ships does not match what was designed; unify tokens and components.

### No Contribution Process

A system teams cannot contribute to becomes a bottleneck they bypass; establish reviewed intake with clear acceptance criteria.

### Unversioned Changes

Silent breaking changes destroy trust; use semantic versioning, changelogs, and deprecation paths with migration guidance.

### Documenting Anatomy Without Usage

Properties without when-to-use guidance leads to wrong component choices; document usage, alternatives, and accessibility.

### Optimizing For Elegance Over Adoption

A beautiful system no one consumes has no value; measure adoption and invest where the system is bypassed.

## Self-Check

- [ ] The system has dedicated owners, a roadmap, and a maintenance budget, run as an ongoing product.
- [ ] Clear criteria define what belongs in the system (reused across teams) versus what stays in products.
- [ ] Components are designed for composition, tested in realistic assemblies, with spacing controlled by parents.
- [ ] Design and engineering share a single source of truth for tokens and components, with version alignment.
- [ ] A contribution process with reviewed intake and documented acceptance criteria invites healthy growth.
- [ ] Releases are semantically versioned, with changelogs, deprecation paths, and migration guidance.
- [ ] Documentation covers anatomy, when to use, when not to, examples, accessibility, and related components.
- [ ] Adoption, contribution, issues, and consumer satisfaction are measured to guide investment.
