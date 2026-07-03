---
name: requirements_elicitation_and_traceability.md
description: Use when the agent is gathering project requirements, interviewing stakeholders for needs, writing requirements documents, tracing requirements to deliverables and tests, or checking whether stated requirements actually capture what the project must deliver.
---

# Requirements Elicitation And Traceability

Requirements are the bridge between a business need and a deliverable. When they are gathered superficially, the project builds something plausible that still fails the real need. When they are not traced, changes and verification become guesswork. The project manager must distinguish needs from wants, surface unstated expectations, resolve conflicts, and maintain a thread from each requirement through design, build, and acceptance. Requirements work is not a one-time collection exercise; it is a living discipline that protects scope and quality.

Use this skill before interviewing stakeholders, drafting a requirements document, building a traceability matrix, defining acceptance criteria, or reviewing whether the project will actually satisfy its intended users. The goal is to prevent the agent from producing a tidy requirements list that omits the constraints, conflicts, and implicit expectations that determine real success.

## Core Rules

### Elicit Needs, Not Just Stated Wants

Stakeholders often describe solutions rather than needs. A request for a specific feature may hide a deeper goal that could be met in a better way. The project manager must probe beneath the surface request to the underlying problem.

Use techniques suited to context:

- interviews and workshops for direct stakeholder needs;
- observation and shadowing for current process pain;
- document analysis for regulatory and policy obligations;
- prototyping for ambiguous or novel capabilities;
- surveys for broad but shallow input;
- analysis of existing data and metrics for actual behavior.

Distinguish the stated requirement from the real need and record both. The real need is what protects scope from churn later.

### Separate Functional, Non-Functional, And Constraint Requirements

Requirements come in different shapes and conflating them causes gaps. Maintain explicit categories.

- Functional requirements describe what the system or deliverable must do.
- Non-functional requirements describe qualities such as performance, reliability, security, usability, accessibility, maintainability, and scalability.
- Constraints are fixed boundaries such as deadline, budget, technology stack, regulatory limits, or organizational standards.

Non-functional requirements are the most frequently omitted and the most common cause of late failure. A deliverable can meet every functional requirement and still be unusable, insecure, or too slow.

### Make Requirements Testable And Unambiguous

A requirement that cannot be tested cannot be verified or accepted. Vague words such as fast, user-friendly, robust, or scalable invite different interpretations.

Rewrite requirements so that completion can be observed. Replace fast with a measurable target such as a response time under a stated load. Replace user-friendly with a specific usability criterion or test scenario. Where a requirement cannot be made fully measurable, define the acceptance evidence that will be used.

Each requirement should be unique, clear, and free of internal contradiction.

### Resolve Conflicts Explicitly

Stakeholders will disagree. One group wants speed, another wants thoroughness. One wants flexibility, another wants simplicity. Conflicts that are ignored do not disappear; they surface late as rework or rejection.

When requirements conflict, surface the conflict, identify the tradeoff, involve the right decision maker, and record the resolution and its rationale. Documenting the resolution prevents the conflict from reopening later.

### Build And Maintain Traceability

Traceability connects each requirement to its source, to the design and deliverables that satisfy it, and to the test or acceptance that verifies it. Without it, scope changes become uncontrolled and verification becomes incomplete.

A traceability matrix typically links:

- requirement to originating stakeholder or source;
- requirement to design element or work package;
- requirement to test case or acceptance criterion;
- requirement to risk where relevant.

Traceability does not need to be heavy, but it must be sufficient to answer what is affected when a requirement changes.

### Manage Requirements Scope Deliberately

Requirements grow. New needs emerge, stakeholders refine their thinking, and the environment shifts. Uncontrolled growth is scope creep. Controlled evolution is normal project life.

Establish how requirements are added, changed, prioritized, and baselined. Use a change process that evaluates impact on scope, schedule, cost, quality, and risk before acceptance. Prioritize requirements so that the most valuable work is delivered first and optional work can be deferred.

### Validate Requirements With Stakeholders

A requirements document that is never validated is an assumption. Review the consolidated requirements with stakeholders to confirm they collectively represent the real need, that nothing important is missing, and that expectations are aligned.

Validation is not a signature ceremony. It is a check that the requirements, taken together, would actually satisfy the business goal and that stakeholders recognize their own needs in the document.

## Common Traps

### Capturing Solutions Instead Of Needs

Recording the requested feature without the underlying problem locks the project into one solution and hides the real goal.

### Omitting Non-Functional Requirements

Performance, security, usability, accessibility, and reliability are easy to forget and expensive to add late.

### Ambiguous Adjectives

Words like fast, easy, secure, and scalable mean different things to different people and cannot be verified.

### Ignoring Conflicting Requirements

Letting contradictory requirements coexist in the document guarantees rework and stakeholder friction later.

### No Traceability

Without links from requirement to design and test, changes and acceptance become guesswork and gaps hide silently.

### Treating The Requirements Document As Done

Requirements evolve. Freezing them at signoff and never revisiting them produces drift between the document and the actual project.

### One-Sided Elicitation

Gathering requirements only from the sponsor or only from one team misses the perspectives of users, operations, support, compliance, and downstream owners.

### Over-Specifying Early

Excessive detail on uncertain scope creates false precision and resists necessary change. Decompose and detail progressively as understanding grows.

## Self-Check

- [ ] Requirements capture underlying needs, not only stated solutions or feature requests.
- [ ] Functional, non-functional, and constraint requirements are separated, and non-functional categories are explicitly covered.
- [ ] Each requirement is testable, with measurable acceptance evidence rather than vague adjectives.
- [ ] Conflicting requirements are surfaced, resolved with the right decision maker, and documented with rationale.
- [ ] A traceability matrix links requirements to source, design or work package, and test or acceptance.
- [ ] Requirements changes go through a controlled process that assesses impact before acceptance.
- [ ] Requirements are prioritized so the most valuable work is delivered first.
- [ ] The consolidated requirements were validated with stakeholders as a coherent whole.
- [ ] Requirements are revisited and refined progressively rather than frozen at signoff.
- [ ] Elicitation included users, operations, support, compliance, and downstream owners, not only the sponsor.
