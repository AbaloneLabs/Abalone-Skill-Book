---
name: external_and_cross_team_dependency_management.md
description: Use when the agent is managing dependencies on external parties vendors other teams or platforms, coordinating handoffs across organizational boundaries, escalating dependency risk outside direct control, or structuring agreements that make external commitments visible and enforceable.
---

# External And Cross-Team Dependency Management

The hardest dependencies to manage are the ones you cannot command. When your project depends on another team, a vendor, a platform owner, a customer, or a regulator, you lack the authority to assign, prioritize, or reassign the work. You can only request, coordinate, contract, and escalate. Yet these external and cross-team dependencies are often where projects actually stall, because the external party has its own priorities, its own capacity constraints, and its own definition of urgency that has nothing to do with your timeline. Agents tend to treat an external dependency like an internal one, list it on the plan with a date, and assume delivery, then discover at the last moment that the other party never owned the commitment.

The judgment problem is how to make external dependencies visible and owned rather than assumed, how to structure handoffs and agreements so commitments are explicit, how to monitor what you cannot control, and how and when to escalate. External dependency management is fundamentally a coordination and influence discipline, not a scheduling one.

## Core Rules

### Make Every External Dependency Explicit With A Named Owner On Both Sides

An external dependency listed without a named owner on the external side is a hope, not a commitment. For each external dependency, identify the specific person or role on the other party who owns delivering it, and the person on your side who owns tracking and coordinating it. Two named owners, an interface point on each side, transform a vague dependency into a managed relationship. Dependencies owned by "the vendor" or "the platform team" in the abstract drift, because no individual feels the obligation. Name the humans.

### Convert Dependencies Into Explicit, Documented Commitments

A verbal "yes, we'll get that to you" is not a commitment you can plan against. Convert each significant external dependency into a documented agreement: a statement of work, a service-level expectation, an interface agreement, an email confirmation, or a ticket in the other party's system. The form depends on the relationship, but the commitment must be explicit about what is being delivered, when, in what form, and to what quality. Documented commitments create accountability that verbal assurances cannot, and they give you something to reference and escalate against when delivery slips.

### Understand The External Party's Priorities And Constraints

Your urgent dependency is one of many demands on the external party, and it may rank low in their priorities. Understand their context: what else are they committed to, what is their capacity, what is their planning cadence, and what motivates them. A vendor driven by contract penalties behaves differently from an internal team driven by relationship and roadmap alignment. Mapping the external party's incentives lets you predict whether they will prioritize your work and lets you shape the request accordingly. Treating the other party as a black box that should simply deliver sets up disappointment.

### Define Clean Handoff Interfaces And Acceptance Criteria

Cross-boundary handoffs fail most often at the interface, where what one side delivered does not match what the other side expected. Define the handoff precisely: the format, the data schema, the API contract, the documentation, the acceptance tests, and the definition of done from the receiver's perspective. Ambiguous interfaces produce rework, finger-pointing, and delay that neither side anticipated. Invest in the interface definition before the handoff, and get both sides to agree to it, because the interface is where most cross-team friction actually lives.

### Monitor External Dependencies More Closely Than Internal Ones

Because you cannot control external delivery, you must monitor it more, not less. Establish a cadence for checking status with the external owner, not waiting for the due date. Ask for evidence of progress, not just assurances: is the work started, are there intermediate artifacts, are there risks on their side. Early signals that an external dependency is slipping are precious, because your recovery options narrow as the date approaches. Build external dependency check-ins into your regular rhythm rather than treating them as one-time requests.

### Escalate Early And Along The Right Path

When an external dependency is at risk and the direct owner cannot or will not prioritize it, escalate. But escalate effectively: identify whose authority can move the other party, whether that is your sponsor talking to their sponsor, a contract mechanism, a steering committee, or executive escalation. Escalate early enough to matter, not after the slip is locked in, and escalate with specifics: what is needed, by when, the impact of non-delivery, and what you have already tried. Escalation that comes too late or that is vague rarely moves the needle.

### Build In Buffer And Contingency For External Dependencies

External dependencies deserve more schedule buffer than internal ones because their variance is higher and your control is lower. Where an external dependency sits on or near the critical path, build in explicit contingency and define a fallback: an alternative supplier, a degraded-scope option, a mock or stub that lets your work proceed, or a decision to descope. A plan that treats external delivery as deterministic is a plan that will break. Buffer and contingency are not pessimism; they are honest acknowledgment of where the uncertainty concentrates.

### Distinguish Cooperative From Contractual External Relationships

The management approach differs by relationship type. A contractual vendor relationship is governed by the contract, with penalties, service levels, and formal change control as the levers. A cooperative internal-team relationship is governed by influence, shared goals, relationship capital, and escalation through management chains. Using contract tactics on a cooperative partner damages the relationship; using only soft influence on a vendor who responds to contractual levers under-manages the risk. Match the management style to the relationship type.

## Common Traps

### Assuming External Delivery Without A Named Owner

Listing a dependency on "the platform team" with no individual owner lets it drift. The trap is that no one on the other side feels the obligation until it is too late. Name owners on both sides.

### Relying On Verbal Commitments

A verbal yes cannot be planned against or escalated. The trap is that the commitment evaporates under the other party's competing priorities. Document the agreement.

### Treating The External Party As A Black Box

Ignoring the other party's priorities and capacity sets up predictable disappointment. The trap is assuming urgency transfers across organizational boundaries. Understand their incentives.

### Ambiguous Handoff Interfaces

Undefined formats, schemas, or acceptance criteria produce rework and finger-pointing at the handoff. The trap is that the delay appears only at integration. Define interfaces precisely up front.

### Waiting For The Due Date To Check Status

Monitoring external dependencies only at delivery removes every recovery option. The trap is that the slip is discovered too late to act. Check progress on a cadence.

### Late Or Vague Escalation

Escalating after the slip is locked in, or without specifics, rarely moves the other party. The trap is that the escalation fails and the dependency still slips. Escalate early with concrete asks.

### No Buffer Or Fallback For Critical External Dependencies

Treating external delivery as deterministic breaks the plan when variance hits. The trap is a critical-path slip with no contingency. Build buffer and define fallbacks.

### Mismatched Management Style To Relationship Type

Using contract tactics on a cooperative partner, or soft influence on a contractual vendor, mismanages the relationship. The trap is damaged trust or under-managed risk. Match style to relationship.

## Self-Check

- [ ] Does each external dependency have a named owner on both the external party's side and your side?
- [ ] Have significant external dependencies been converted into documented commitments specifying what, when, form, and quality?
- [ ] Have you mapped the external party's priorities, capacity, and incentives rather than treating them as a black box?
- [ ] Are handoff interfaces defined precisely, with formats, schemas, contracts, and acceptance criteria agreed by both sides?
- [ ] Is external dependency status monitored on a regular cadence with evidence of progress, not just assurances?
- [ ] Is there a defined escalation path, and are escalations made early with specific asks and impact statements?
- [ ] Do critical external dependencies carry schedule buffer and a defined fallback or descope option?
- [ ] Is the management style matched to the relationship type, contractual versus cooperative?
- [ ] Can you identify, for each external dependency, the specific risk to your timeline if it slips?
- [ ] Has the project avoided treating external delivery as deterministic where variance is genuinely high?
