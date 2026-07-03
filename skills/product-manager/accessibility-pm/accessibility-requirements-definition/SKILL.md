---
name: accessibility_requirements_definition.md
description: Use when the agent is defining accessibility requirements for a product or feature, setting conformance levels, writing acceptance criteria for accessibility, deciding which standards apply, or determining how accessibility obligations enter the requirements process.
---

# Accessibility Requirements Definition

Accessibility is too often treated as a final polish step or a compliance afterthought, added at the end of a project when it is expensive and incomplete. By the time a team tries to make a finished feature accessible, the structural decisions, the component choices, the interaction patterns, and the content have already locked in barriers that retrofitting cannot fully remove. The product manager's job is to make accessibility a requirement from the start, with the same clarity and enforceability as security, performance, or functional requirements, so that it shapes decisions rather than chasing them.

Defining accessibility requirements is not the same as listing WCAG success criteria. It is deciding which standards apply, at which conformance level, for which surfaces, with which exceptions, and how conformance will be verified before release. Vague commitments like "the product should be accessible" produce nothing, because they create no obligation and no test. Precise requirements create accountability and give engineering, design, and content teams a target they can actually meet.

Use this skill before writing requirements for a new product or feature, before defining accessibility acceptance criteria, before deciding which conformance level to target, or before establishing release gates for accessibility. Ask: which accessibility standard and conformance level applies, and is that decision justified or merely inherited? Are the requirements specific enough to be tested and verified? Have the exceptions been named explicitly rather than left implicit? Is accessibility treated as a release-blocking requirement or as a nice-to-have?

## Core Rules

### Make Accessibility A Requirement From The Start

Accessibility introduced late is accessibility done poorly. Structural choices made without accessibility in mind, such as custom components without semantics, color systems without sufficient contrast, or interactions that depend on a mouse, are expensive to reverse. The earlier accessibility enters the requirements, the cheaper and more complete it is.

Treat accessibility as a first-class requirement alongside functional, security, and performance requirements. It appears in the product spec, in the acceptance criteria, in the definition of done, and in the release checklist. It is not a separate workstream that happens after the feature works; it is part of what makes the feature work. A feature that is inaccessible has not met its requirements, regardless of how well the functional behavior performs.

### Specify The Standard, Version, And Conformance Level

"Accessible" is not a requirement; it is an aspiration. A requirement names the specific standard, the version, and the conformance level. For most web and app products, this means specifying WCAG 2.1 or 2.2 at Level AA, which is the level referenced by most legal frameworks and procurement standards. Some contexts require Level A as a floor, or Level AAA for specific high-importance flows, or additional standards for specific regions or domains.

State the standard explicitly and justify the choice. If you are targeting Level AA because it matches legal obligations and industry expectation, say so. If you are targeting only Level A because of a constraint, name the constraint and the risk. Do not leave the conformance level ambiguous, because ambiguity defaults to whatever is easiest, which is the lowest effort, which is usually non-compliant. The requirement should be specific enough that an auditor could test against it.

### Define Requirements Per Surface, Not Globally

A product is not uniformly accessible or inaccessible. Different surfaces have different obligations. A marketing site, a logged-in application, a settings panel, an email template, a mobile app, and a third-party embed may each have different standards, different feasibility constraints, and different user impact. A single global accessibility statement papers over these differences and leaves the hardest surfaces unaddressed.

Break requirements down by surface. For each major surface, define the applicable standard, the conformance level, the known constraints (such as third-party content that cannot be fully controlled), and the priority. This surfaces where accessibility is hardest and where exceptions may be needed, rather than hiding those cases inside a vague global commitment. It also lets the team sequence work by risk and impact.

### Write Testable Acceptance Criteria

A requirement that cannot be tested cannot be enforced. "The form must be accessible" is untestable; "all form fields have programmatically associated labels, error messages are announced to assistive technology, the form is fully operable by keyboard, and color is not the only means of conveying validation state" is testable. Each criterion describes an observable condition that can be verified.

Write accessibility acceptance criteria the way you write functional ones: specific, observable, and pass-fail. Reference the relevant WCAG success criteria by number where applicable, so that the team and any auditor share a precise target. Vague criteria let teams declare victory after a superficial check; precise criteria expose gaps and force them to be addressed before release.

### Name Exceptions Explicitly

Some accessibility gaps cannot be fully closed, usually because of third-party content, legacy systems, or technical constraints outside the team's control. Pretending these do not exist, or leaving them unmentioned, creates two problems: the gaps go unaddressed because no one owns them, and the team loses credibility when an audit finds them. Explicit exceptions are more honest and more manageable than implicit ones.

For each exception, document what is excepted, why, what the impact on users is, what mitigation exists (such as an accessible alternative or a roadmap commitment), and when it will be revisited. An exception is not a permanent waiver; it is a documented decision with a review date. This converts hidden gaps into tracked items with owners and plans.

### Establish Verification Before Release

Requirements without verification are wishes. Define how accessibility conformance will be checked before a feature ships. This typically combines automated scanning (which catches a subset of issues cheaply), manual testing with assistive technology (which catches what automation misses), and review by people who actually rely on accessibility features (which catches what neither automation nor manual testing by sighted testers reveals).

Build verification into the release process, not as an optional final check. Define what must pass for release: which automated checks must be clean, which manual tests must succeed, which assistive technologies must be tested. A feature that has not passed accessibility verification has not met its release criteria, on the same footing as a feature that has not passed security review or functional QA.

### Distinguish Legal Obligation From Ethical Obligation

Legal accessibility requirements vary by region and domain, and meeting the legal minimum is not the same as serving all users. Some products have weak or no legal accessibility obligations but serve users who depend on accessibility nonetheless. Conversely, some legally mandated requirements capture only a subset of real user needs.

Be clear about which requirements are legally driven and which are driven by user need and brand commitment. A product that targets only the legal minimum may be compliant but still exclude users; a product that targets genuine usability for people with disabilities goes further and builds trust. State the rationale so the team understands whether they are meeting a floor or pursuing a higher standard, and so future decisions about scope are made deliberately rather than by default.

## Common Traps

### Accessibility As Final Polish

Treating accessibility as something to add at the end locks in barriers that retrofitting cannot remove. The trap is believing a finished feature can be made accessible cheaply, when the structural decisions are already made.

### Aspirational Instead Of Testable Requirements

Vague commitments create no obligation and no verification. The trap is feeling that accessibility has been addressed because a requirement mentions it, while the requirement cannot be enforced.

### Global Statements Hiding Surface Differences

A single accessibility claim papers over the hardest surfaces. The trap is average compliance that leaves the highest-risk areas non-compliant.

### Implicit Exceptions

Unaddressed gaps that no one owns erode credibility and exclude users. The trap is assuming silence about exceptions means they do not exist.

### Verification As Optional

Requirements without a release gate get skipped under deadline pressure. The trap is accessibility criteria that exist on paper but are never actually checked before ship.

### Legal Minimum As The Ceiling

Meeting only the legal obligation excludes users the law does not protect. The trap is confusing compliance with genuine inclusivity.

## Self-Check

- [ ] Accessibility is a first-class requirement from the start of the spec, not a late addition or a polish step.
- [ ] The standard, version, and conformance level are specified explicitly and justified.
- [ ] Requirements are broken down per surface, with the hardest surfaces identified rather than hidden in a global statement.
- [ ] Acceptance criteria are testable, observable, and reference specific WCAG success criteria where applicable.
- [ ] Exceptions are documented explicitly with reason, impact, mitigation, and review date, not left implicit.
- [ ] Verification, combining automated, manual, and user-based testing, is built into the release process as a gate.
- [ ] The rationale distinguishes legal obligation from ethical and brand commitment, and the team knows which floor they are meeting.
- [ ] Accessibility release criteria are enforced on the same footing as security and functional QA, not waived under pressure.
- [ ] Third-party and legacy constraints are identified and have mitigation plans rather than being ignored.
- [ ] The requirements are specific enough that an external auditor could test conformance against them.
