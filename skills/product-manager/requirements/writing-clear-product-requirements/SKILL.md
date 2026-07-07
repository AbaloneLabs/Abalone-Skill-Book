---
name: writing_clear_product_requirements.md
description: Use when the agent is writing product requirements, drafting a PRD, specifying features for engineering, defining functional and non-functional requirements, deciding how much detail to include, or reviewing requirements for completeness and ambiguity.
---

# Writing Clear Product Requirements

A product requirement is a communication artifact that must survive translation across roles, time, and ambiguity. It is read by engineers who will build from it, designers who will shape it, QA who will test it, and stakeholders who will judge it, often months after it was written and without the author available to clarify. The judgment problem is that requirements fail in two opposite ways: they can be too vague, leaving gaps that engineering fills with guesses, or too prescriptive, dictating implementation and blocking better solutions. Good requirements occupy the narrow ground of being specific enough to build and test against, yet framed around the problem and outcome rather than the implementation, so that the team can solve it well rather than merely execute instructions.

Use this skill before writing a PRD or requirement spec, before handing a feature to engineering, before reviewing a requirement for completeness, or before deciding how much detail a given audience needs. The goal is to prevent the agent from producing requirements that are either so abstract they invite reinterpretation or so detailed they become a implementation manual that strips the team of ownership.

## Core Rules

### Frame Requirements Around Problems And Outcomes, Not Solutions

A requirement that dictates the solution, "add a dropdown here with these five options", removes the team's ability to find a better answer to the underlying need. A requirement that states the problem and outcome, "users need to select from a large option set without scrolling, here is the success criteria", invites the best solution.

Frame by stating:

- the user problem or job to be done;
- the target segment and context;
- the desired outcome and how it will be measured;
- the constraints that bound the solution;
- the non-goals that are explicitly out of scope.

Implementation detail belongs in design and engineering docs, referenced from the requirement, not embedded as mandate.

### Make Each Requirement Unambiguous And Testable

A requirement that two readers interpret differently is a defect in the requirement. Each requirement should be specific enough that a tester can determine whether it is met.

Make each requirement:

- singular, one requirement per statement;
- testable, with observable, verifiable success conditions;
- unambiguous, with defined terms rather than vague qualifiers;
- bounded, with clear scope and explicit exclusions;
- prioritized, must-have versus should-have versus nice-to-have.

Words like "fast", "intuitive", "seamless", and "robust" are not requirements; they are aspirations. Replace them with measurable criteria.

### Specify Functional And Non-Functional Requirements Together

Functional requirements describe what the system does; non-functional requirements describe how well it must do it. Omitting non-functionals leaves engineering to guess at performance, reliability, and security expectations, and the result is usually wrong.

Include non-functional requirements for:

- performance: latency, throughput, response time targets;
- reliability: uptime, error rates, failure behavior;
- scalability: expected load, growth headroom;
- security and privacy: authentication, authorization, data handling;
- accessibility: standards compliance, assistive technology support;
- compatibility: browsers, devices, integrations, locales;
- observability: logging, metrics, alerting needs.

A feature that works functionally but fails its non-functional bar is not done.

### Define Edge Cases, Error States, And Empty States

Requirements that describe only the happy path leave the team to improvise every other state, and those improvisations are where bugs and bad experiences cluster. Anticipate the states the system will actually encounter.

Specify:

- empty state: no data, first use, nothing to show;
- error state: input invalid, network failed, permission denied;
- loading and partial states: async, slow, interrupted;
- boundary cases: maximum values, extreme inputs, concurrent actions;
- permission and role variations: what each user type sees;
- offline, degraded, and migration states.

The measure of a complete requirement is that the unhappy paths are as well specified as the happy one.

### Separate Must-Have From Deferrable Explicitly

A requirement document that lists everything as essential prevents smart scoping and forces all-or-nothing delivery. Distinguish what truly blocks value from what enhances it.

Prioritize by:

- must-have: blocks the outcome, no ship without it;
- should-have: important but workable without for initial release;
- nice-to-have: enhancement, defer without harm;
- explicit non-goals: deliberately excluded to prevent scope creep.

This structure enables incremental delivery and honest tradeoff conversations when time or complexity pressures arise.

### Include The Why, The Evidence, And The Context

A requirement without context becomes untethered from its purpose. The team needs to know why this matters to make good micro-decisions during build.

Include:

- the problem evidence, research, data, customer voice;
- the strategic context and related roadmap items;
- the success metrics and how they will be measured;
- the dependencies and assumptions;
- the open questions and decisions still pending.

Context lets the team solve the problem rather than merely follow instructions.

### Make Assumptions And Dependencies Visible

Hidden assumptions become silent failures. State what the requirement assumes and what it depends on, so that changes in those assumptions trigger re-evaluation.

State:

- assumptions about user behavior, data, or environment;
- dependencies on other teams, systems, or releases;
- technical assumptions that, if wrong, change the approach;
- business assumptions about pricing, packaging, or policy;
- the impact if an assumption proves false.

### Choose The Right Level Of Detail For The Audience

Different audiences need different depth. Engineering needs buildable specificity; leadership needs outcome and scope; design needs constraints and user context. One document rarely serves all.

Adapt by:

- leading with outcome and scope for leadership;
- providing detailed functional and non-functional specs for engineering;
- referencing design and research rather than duplicating them;
- keeping a living document rather than a frozen artifact;
- defining what changes require re-review versus autonomous decision.

## Common Traps

### Dictating Implementation

Specifying the how so tightly that engineering cannot find a better solution or own the approach.

### Happy-Path Only

Describing only the success flow and leaving error, empty, and edge states to improvisation.

### Vague Qualifiers

Words like "fast" and "intuitive" that cannot be tested or verified.

### Missing Non-Functionals

Omitting performance, reliability, security, and accessibility, leaving them to guesswork.

### Everything Is Must-Have

Marking all requirements as essential prevents scoping and forces brittle all-or-nothing delivery.

### Context-Free Requirements

Specs without the why, evidence, or success criteria, leaving the team unable to make good judgment calls.

### Hidden Assumptions

Unstated assumptions that become silent failures when reality differs.

### Frozen Artifacts

Treating the requirement doc as final when it must evolve as understanding deepens.

## Self-Check

- [ ] Requirements are framed around problems and outcomes, with implementation left to design and engineering.
- [ ] Each requirement is singular, testable, unambiguous, bounded, and prioritized.
- [ ] Functional and non-functional requirements, performance, reliability, security, accessibility, are both specified.
- [ ] Edge cases, error states, empty states, loading states, and permission variations are specified, not only the happy path.
- [ ] Must-have, should-have, nice-to-have, and explicit non-goals are distinguished.
- [ ] The why, evidence, strategic context, success metrics, and open questions are included.
- [ ] Assumptions and dependencies are stated with their impact if false.
- [ ] The detail level matches the audience, with a living document rather than a frozen artifact.
- [ ] Vague qualifiers are replaced with measurable criteria.
- [ ] The requirement enables the team to solve the problem rather than merely execute instructions.
