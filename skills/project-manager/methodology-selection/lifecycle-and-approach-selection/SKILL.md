---
name: lifecycle_and_approach_selection.md
description: Use when the agent is deciding whether a project should use a predictive waterfall, adaptive agile, or hybrid lifecycle, matching the delivery approach to requirements stability, technology uncertainty, stakeholder involvement, regulatory constraints, or contract type before committing to a delivery model.
---

# Lifecycle and Approach Selection

Selecting a project lifecycle is framed as a one-time, early decision, but it is really a structural choice that determines how uncertainty is absorbed, how scope is controlled, how stakeholders participate, and how progress is measured for the entire engagement. The lifecycle is the contract between the team and the world about what will be defined when, and what will be allowed to change. A predictive lifecycle promises definition before execution; an adaptive lifecycle promises definition through execution; a hybrid makes a deliberate split between the two. Each promise carries a cost, and the wrong promise creates either rigid delivery of the wrong thing or unbounded exploration that never converges.

The judgment problem is to read the actual characteristics of the work, requirements clarity, technology maturity, stakeholder availability, regulatory exposure, and match the lifecycle to them rather than to organizational habit or personal preference. Agents tend to default to whatever lifecycle is familiar, to treat the choice as ideological rather than situational, and to ignore the signals (ambiguous requirements, fixed-price contracts, compliance obligations) that should drive the decision. The most damaging error is choosing a lifecycle for political reasons and then blaming the team when its assumptions do not hold.

## Core Rules

### Read the Requirement Clarity and Stability Before Choosing

The strongest single signal for lifecycle choice is how well requirements are understood and how stable they are expected to be. When requirements are clear, stable, and unlikely to change, a predictive lifecycle lets you define scope, estimate cost, and sequence work with confidence. When requirements are ambiguous, evolving, or discoverable only through delivery, an adaptive lifecycle lets the product emerge through feedback. Ask explicitly: could a competent analyst write a complete specification today that would still be correct at delivery? If yes, predictive; if no, adaptive. Do not choose based on what you wish were true.

### Assess Technology and Solution Uncertainty

A second axis is how well-understood the solution and technology are. Proven technology and a familiar solution domain favor predictive planning because estimates are reliable and risk is low. Novel technology, integration unknowns, or a solution being invented as you go favor adaptive delivery, because the work itself is a learning process and upfront estimates will be wrong. When the team has never done this kind of work before, a predictive plan is a fiction; when the work is routine and well-trodden, an adaptive approach adds ceremony without insight.

### Map Stakeholder Availability and Feedback Cadence

Adaptive lifecycles depend on stakeholders who can engage continuously, review increments, and steer the product. If those stakeholders are not available, or can only engage at defined milestones, adaptive delivery loses its feedback engine and degrades into iterative waterfall. Predictive lifecycles front-load stakeholder engagement into requirements and design phases and then expect approval at gates. Match the lifecycle to the realistic availability of the people who must steer it: continuous availability supports adaptive; milestone availability supports predictive. Do not assume availability that the organization will not actually provide.

### Account for Contract, Regulatory, and Compliance Constraints

Fixed-price, fixed-scope contracts push strongly toward predictive definition because both sides need a baseline to measure against. Time-and-materials or capacity-based contracts are compatible with adaptive scope evolution. Regulatory and compliance regimes that require documented requirements, traceability, and approved designs before implementation push toward predictive. Audit and safety-critical domains often mandate defined phases and artifacts. Identify these hard constraints first: they can override pure problem-fit considerations and force a lifecycle the team must then make work.

### Choose the Lifecycle That Fits Each Major Workstream

A single project can contain workstreams with different characteristics. Infrastructure setup, integration with legacy systems, or compliance work may be predictable; product features, user experience, or exploratory analytics may be adaptive. The mature choice is often not one lifecycle for the whole project but the right lifecycle for each workstream, coordinated at the seams. Identify the dominant characteristics of each major chunk of work and assign the lifecycle that fits, rather than imposing uniformity for administrative convenience.

### Make the Decision Explicit and Document the Rationale

Because the lifecycle choice is consequential and often invisible, make it an explicit decision with a documented rationale: which characteristics drove it, what assumptions it rests on, and what would trigger revisiting it. An undocumented choice becomes an unchallengeable default. When the assumptions change, requirements stabilize or destabilize, stakeholders appear or disappear, the documented rationale lets you recognize that the lifecycle may need to change too. Treat the lifecycle as a decision under uncertainty, not as a permanent identity.

### Re-evaluate at Natural Inflection Points

Lifecycles chosen at project initiation may become wrong as the project learns. A project that starts adaptive may, once the core is validated, shift to predictive for the predictable build-out. A predictive project that hits deep uncertainty may need an adaptive probe to reduce risk before resuming. Build in explicit checkpoints where the lifecycle fit is reassessed, rather than assuming the initial choice holds forever. The cost of switching is real, but the cost of persisting in a misfit lifecycle is usually higher.

### Align Governance and Measurement With the Chosen Lifecycle

A predictive lifecycle is governed by milestone completion, variance from baseline, and phase gate approval. An adaptive lifecycle is governed by working increments, flow metrics, and stakeholder feedback. Mismatched governance, measuring an agile project against a predictive baseline or running predictive gates over adaptive work, breaks both. Once the lifecycle is chosen, align the reporting, the success criteria, and the steering mechanisms to that lifecycle's native signals.

## Common Traps

### Choosing the Lifecycle You Are Comfortable With

The team defaults to the lifecycle it knows, regardless of whether the work fits, and then forces the work into that shape. The trap is that familiarity feels like fit. Read the problem characteristics first and derive the lifecycle from them.

### Ideological Commitment to Agile or Waterfall

The choice becomes a statement of identity rather than a situational judgment, and evidence that contradicts the ideology is ignored. The trap is that the lifecycle serves the team's beliefs instead of the project's needs. Treat the choice as instrumental, not as a value judgment.

### Assuming Stakeholder Availability That Will Not Materialize

An adaptive lifecycle is chosen on the assumption that stakeholders will engage continuously, but they cannot or will not, so feedback never arrives and the team builds blind. The trap is optimistic assumption about human availability. Verify realistic engagement capacity before committing to adaptive.

### Ignoring Contract and Regulatory Hard Constraints

A lifecycle is chosen on pure problem-fit, but a fixed-price contract or a compliance regime makes it unworkable, and the conflict surfaces late as disputes or audit findings. The trap is treating constraints as secondary. Identify hard constraints first and let them bound the choice.

### Forcing One Lifecycle Across Heterogeneous Work

A uniform lifecycle is imposed for administrative simplicity, but the workstreams have fundamentally different characteristics, so some are over-managed and others under-managed. The trap is mistaking uniformity for coherence. Match the lifecycle to each major workstream.

### Persisting in a Misfit Lifecycle Too Long

The initial choice becomes wrong as the project learns, but no one revisits it because the decision was never framed as revisable. The trap is treating the lifecycle as permanent rather than as a hypothesis. Build in re-evaluation checkpoints.

### Governing With the Wrong Lifecycle's Signals

An agile project is measured against a predictive baseline, or a predictive project is run with adaptive gates, producing governance that fights the delivery model. The trap is governance inherited from a different paradigm. Align measurement to the chosen lifecycle.

## Self-Check

- [ ] Was the lifecycle chosen by reading actual requirement clarity and stability, rather than by default or preference?
- [ ] Was technology and solution uncertainty assessed as a second axis, with novel work steering toward adaptive and routine work toward predictive?
- [ ] Has realistic stakeholder availability for continuous feedback been verified before committing to an adaptive lifecycle?
- [ ] Have contract type (fixed-price vs time-and-materials) and regulatory or compliance constraints been identified as hard bounds on the choice?
- [ ] Where the project has heterogeneous workstreams, has each been assigned the lifecycle that fits its characteristics rather than a single uniform one?
- [ ] Is the lifecycle decision documented with its driving characteristics, assumptions, and re-evaluation triggers?
- [ ] Are there explicit checkpoints where the lifecycle fit is reassessed as the project learns?
- [ ] Are governance, reporting, and success criteria aligned to the chosen lifecycle's native signals (milestones and variance for predictive; increments and flow for adaptive)?
- [ ] Has the choice been treated as an instrumental, situational decision rather than an ideological commitment?
- [ ] If a hybrid is under consideration, is the split between predictive and adaptive workstreams deliberate and documented, not accidental?
