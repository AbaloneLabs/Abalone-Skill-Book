---
name: hybrid_delivery_design.md
description: Use when the agent is designing a hybrid methodology that blends predictive and adaptive approaches, deciding which phases or workstreams use which approach, managing the seams between waterfall and agile, or governing a project that runs defined phases alongside iterative delivery.
---

# Hybrid Delivery Design

Hybrid delivery is often presented as a best-of-both-worlds compromise, but in practice it is the hardest lifecycle to run well because it requires two different delivery disciplines to coexist and to hand off cleanly. A pure predictive project has one set of muscles: define, sequence, gate, build. A pure adaptive project has another: prioritize, iterate, inspect, adapt. A hybrid demands both, plus a deliberate design for the seams where defined artifacts feed iterative teams and where iterative output is folded back into defined milestones. When the seams are not designed, the result is not a blend but a collision: predictive teams block on adaptive teams that cannot commit to dates, adaptive teams chafe under gates that assume complete definition, and governance cannot reconcile the two logics.

The judgment problem is to decide which parts of the work should be predictive and which adaptive, how the interfaces between them are specified and synchronized, and how a single governance model can respect both a fixed baseline and an evolving backlog. Agents tend to call a project hybrid without designing the integration, to let the two approaches drift into conflict, and to assume that combining them is simpler than it is. A well-designed hybrid is a deliberate architecture; a poorly designed one is two failing projects stapled together.

## Core Rules

### Decide the Split by Workstream Characteristics, Not by Compromise

The boundary between predictive and adaptive should follow the actual characteristics of the work. Work that is well-understood, stable, and foundational, such as infrastructure, data migration, regulatory interfaces, or platform architecture, suits a predictive phase with upfront definition. Work that is exploratory, user-facing, or dependent on feedback, such as product features, user experience, or analytics, suits adaptive delivery. Do not split the project down the middle to appease both camps; split it where the nature of the work changes. Document which workstream uses which approach and why.

### Define the Seams as Explicit Contracts

Where predictive work feeds adaptive work, or vice versa, the interface must be an explicit contract: what artifact is handed over, in what form, with what acceptance criteria, by when. A predictive requirements phase that hands a vague brief to an agile team has designed a bad seam. An agile team that hands an evolving API to a predictive integration team without a stable contract has designed a bad seam. For each interface, name the artifact, its stability level (fixed, versioned, evolving), and the synchronization point. Seams that are not specified become the places where the project stalls.

### Align Cadence and Synchronization Points

Predictive and adaptive work run on different clocks: phases and gates versus iterations and releases. A hybrid must define how these clocks meet. Common patterns include releasing adaptive increments on a regular cadence that aligns with predictive milestones, or running a predictive integration phase after each adaptive release. Decide explicitly how often the two streams synchronize, what is integrated at each synchronization, and what happens when they are out of phase. Without alignment, the adaptive stream produces work the predictive stream cannot consume, or the predictive stream demands integration the adaptive stream cannot meet.

### Reconcile Scope Logic Across the Boundary

Predictive work is governed by a fixed baseline scope; adaptive work is governed by a prioritized, evolving backlog. At the seam, these two logics must be reconciled. Decide which scope is truly fixed (the predictive baseline) and which is flexible (the adaptive backlog), and make the relationship explicit: does the adaptive backlog live within the predictive baseline, or can it reshape it? A common failure is the predictive side treating the baseline as immutable while the adaptive side treats it as a suggestion. Resolve this ambiguity in the design, not in the middle of delivery.

### Design a Single Governance Model That Respects Both Logics

Governance that measures the whole project by predictive variance will declare the adaptive stream a failure because its scope evolves. Governance that measures by adaptive flow will declare the predictive stream a failure because it cannot demonstrate working increments early. Design a governance model with two lenses: predictive milestones and variance for the defined work, adaptive flow and value delivery for the iterative work, and a roll-up that presents both honestly rather than forcing one metric onto the whole. Stakeholders must be taught to read both.

### Manage Dependencies Across the Lifecycle Boundary

Dependencies that cross the predictive-adaptive boundary are the highest-risk elements of a hybrid. A predictive workstream waiting on an adaptive deliverable faces date uncertainty the predictive logic cannot tolerate; an adaptive workstream blocked by a predictive deliverable faces rigidity the adaptive logic cannot absorb. Map cross-boundary dependencies explicitly, assign owners, and build buffers or decoupling mechanisms (mocks, stable interfaces, parallel tracks) so that neither stream blocks the other. Dependencies left implicit become the chronic blockers of hybrid projects.

### Decide How Requirements Flow Between the Two Halves

In a hybrid, requirements often originate predictively (a defined requirements phase) and then flow into adaptive delivery, or originate adaptively (discovered through iteration) and then get formalized for predictive integration. Decide the direction of flow and the transformation required. Predictive requirements may need decomposition into backlog items; adaptive discoveries may need consolidation into baseline updates. Without a defined flow, requirements get lost, duplicated, or contradicted across the boundary.

### Plan for the Organizational Reality of Two Cultures

Predictive and adaptive work attract different mindsets and often different people. A hybrid project asks these cultures to collaborate, which requires explicit role clarity, shared goals, and often mediation of the natural tension between definition-minded and discovery-minded practitioners. Do not assume the two halves will cooperate because they are on the same project. Design the collaboration, define shared objectives, and address cultural friction directly when it appears.

## Common Traps

### Calling It Hybrid Without Designing the Integration

The project is labeled hybrid but no one designs how the predictive and adaptive parts connect, so they collide at every handoff. The trap is that the label substitutes for the architecture. Design the seams explicitly.

### Splitting Down the Middle to Appease Both Camps

The split is made politically rather than by work characteristics, so each half contains work that fits the other approach. The trap is compromise masquerading as design. Split where the nature of the work changes.

### Undefined Seams Between the Two Approaches

Predictive work hands off to adaptive work with no specified artifact, form, or acceptance criteria, and the handoff stalls. The trap is assuming the interface will work itself out. Specify each seam as a contract.

### Governance That Forces One Logic Onto the Whole

A single metric, predictive variance or agile velocity, is applied to both halves, misrepresenting the half it does not fit. The trap is measurement uniformity. Use a dual-lens governance model.

### Cross-Boundary Dependencies Left Implicit

Dependencies between the predictive and adaptive streams are not mapped, so they surface as chronic blockers during delivery. The trap is that the risk is invisible until it bites. Map and decouple cross-boundary dependencies up front.

### Immutable Baseline Clashing With Evolving Backlog

The predictive side treats scope as fixed while the adaptive side evolves it, producing disputes at every integration. The trap is unresolved scope logic. Decide and document the relationship between baseline and backlog.

### Assuming Two Cultures Will Cooperate Automatically

Definition-minded and discovery-minded practitioners are expected to collaborate without role clarity or shared goals, and friction is treated as a personality issue. The trap is ignoring the cultural dimension. Design the collaboration explicitly.

## Self-Check

- [ ] Is the split between predictive and adaptive work based on workstream characteristics, documented with rationale, rather than a political compromise?
- [ ] Is each seam between the two approaches specified as an explicit contract (artifact, form, acceptance criteria, timing, stability level)?
- [ ] Are the cadences of the two streams aligned at defined synchronization points, with rules for what is integrated when?
- [ ] Has the relationship between the fixed predictive baseline and the evolving adaptive backlog been decided and documented?
- [ ] Does governance use a dual-lens model (predictive milestones and variance plus adaptive flow and value) rather than forcing one metric onto the whole project?
- [ ] Are cross-boundary dependencies mapped, owned, and decoupled with buffers or stable interfaces so neither stream blocks the other?
- [ ] Is the direction of requirements flow between the halves defined, with a transformation process for crossing the boundary?
- [ ] Has the cultural and role collaboration between predictive and adaptive practitioners been designed, not assumed?
- [ ] Can stakeholders read both governance lenses and understand the status of each half honestly?
- [ ] If the hybrid is struggling, has the seam design (not just the individual halves) been examined as the likely source?
