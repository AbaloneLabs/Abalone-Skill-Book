---
name: assumption_identification_and_logging.md
description: Use when the agent is surfacing project assumptions, building an assumptions register, distinguishing assumptions from facts and risks, reviewing a plan for hidden premises, or logging the unproven beliefs a schedule depends on before they quietly become delivery failures.
---

# Assumption Identification and Logging

Every project plan rests on beliefs that have not been proven yet. These beliefs are assumptions: statements treated as true for the purpose of planning, even though evidence is incomplete or absent. Assumptions are not failures of planning; they are unavoidable, because no project can wait until every fact is confirmed. The danger is that assumptions are usually invisible. A schedule, a budget, or a resource plan is presented as solid, while underneath it sits a layer of premises that, if false, collapse the whole structure. The judgment problem is not whether to assume, but which assumptions are load-bearing, who owns them, and what happens if they are wrong.

Agents tend to miss assumptions because plans are written in declarative language. "The migration will complete in the maintenance window" reads like a fact, but it hides assumptions about data volume, tooling performance, rollback feasibility, and team availability. The skill here is to read every plan as a stack of unproven claims, surface the ones that matter, and log them in a form that supports later validation rather than disappearing into narrative.

## Core Rules

### Read Plans as Stacks of Unproven Claims

Treat each estimate, dependency, and commitment as a hypothesis. When reviewing a schedule, ask what must be true for each milestone to hold: availability of people, performance of systems, cooperation of third parties, stability of requirements, accuracy of volume estimates, and timeliness of decisions. The goal is to convert implicit premises into explicit, testable statements. A plan that cannot list its assumptions has not been understood; it has only been transcribed.

### Distinguish Assumptions From Facts, Risks, and Dependencies

Use precise categories so the log stays actionable. A fact is verified and evidenced. An assumption is believed true but unproven. A risk is an uncertain event that may cause impact. A dependency is something the project needs from an external party or system. The same statement can move between categories over its life: "the vendor supports the API" is an assumption until tested, then becomes a fact or a risk. Mislabeling blurs ownership and weakens response. Force an explicit category on every entry and reclassify as evidence accumulates.

### Identify Load-Bearing Versus Convenience Assumptions

Not every assumption matters equally. A load-bearing assumption is one whose falsity would invalidate a milestone, estimate, or decision; a convenience assumption simplifies work but has cheap alternatives. Concentrate logging effort on load-bearing assumptions, because those are the ones whose failure reshapes the plan. For each entry, ask: if this is false, what breaks, and how expensive is the break? Rank by consequence, not by likelihood alone.

### Write Assumptions as Specific, Falsifiable Statements

Vague assumptions are untestable and therefore useless. "Stakeholders will cooperate" cannot be validated or invalidated. Rewrite it as "the legal team will approve the data-sharing terms within ten business days of receipt." A good assumption statement names the actor, the action, the measurable condition, and the timeframe. Specificity is what later allows confirmation tracking to produce a clear true, false, or still-pending verdict.

### Capture Source, Owner, and Confidence

Every logged assumption should record who made it, who is accountable for confirming it, and a confidence level. The source matters because assumptions inherited from a prior project or a vendor proposal carry different credibility than ones derived from direct investigation. The owner is the person who will either confirm or invalidate the assumption; an assumption without an owner is orphaned and will never be closed. Confidence, even a rough high/medium/low, helps prioritize validation effort.

### Log Assumptions at the Point They Enter the Plan

Assumptions are easiest to capture when they are first made, during estimation, scoping, or scheduling conversations. Waiting until a later review forces reconstruction from memory and loses context. Build assumption capture into the artifacts where premises originate: the project charter, the schedule notes, the estimate worksheet, and the risk workshop output. The register is a living aggregation of these captured entries, not a document written once at the start.

### Connect Each Assumption to the Plan Element It Supports

An assumption is only meaningful in relation to what it underpins. Record which deliverable, milestone, estimate, or decision depends on it. This linkage is what makes the register useful during replanning: when an assumption fails, you can immediately see which parts of the plan are affected instead of guessing. Without traceability, the register becomes a list of concerns with no operational value.

### Maintain the Register as a Living Artifact

A assumptions register written at initiation and never revisited is documentation theater. Assumptions are confirmed, invalidated, refined, and newly created throughout the project. Schedule regular reviews to close confirmed assumptions, convert invalidated ones into risks or issues, and add assumptions that emerged from recent decisions. The register's value comes from currency, not from completeness at a single point in time.

## Common Traps

### Treating Optimistic Estimates as Facts

When a team says "we can do this in three weeks," the estimate is often recorded as a commitment without logging the assumptions behind it, such as uninterrupted focus, no competing priorities, and familiar tooling. The trap is that optimistic estimates feel like facts in the moment, so no one thinks to question them. Always extract and log the premises behind any estimate that drives a critical date or budget.

### Logging Assumptions So Vaguely They Cannot Be Tested

Assumptions like "the platform is stable" or "users will adopt the tool" survive in registers for months because no one can prove them wrong. The trap is that vague statements feel responsible while being operationally empty. The cost surfaces later when the team cannot tell whether the assumption held or failed. Force every entry to be specific enough that a third party could determine its truth.

### Confusing Assumptions With Risks

Writing "risk: vendor may be late" when the underlying issue is actually an assumption, "vendor will deliver by the agreed date," conflates two disciplines with different responses. Risks call for mitigation and contingency; assumptions call for confirmation and validation. The trap is that mixing them produces a register where nothing is actionable because the right response was never attached. Keep the categories distinct and let items migrate as their nature clarifies.

### Capturing Only the Comfortable Assumptions

Teams readily log assumptions about things they control and quietly omit premises about politically sensitive topics, such as executive sponsorship, funding continuity, or a key person's availability. The trap is that the uncomfortable assumptions are usually the most load-bearing, and omitting them leaves the largest blind spot. Deliberately probe for assumptions people are reluctant to state aloud, because silence often marks the highest-stakes premises.

### Letting the Register Become a Static Document

A register created to satisfy a process requirement and then filed away provides false assurance. The trap is that the artifact exists, so stakeholders believe assumptions are managed, when in fact nothing is being confirmed or updated. The damage is delayed until an unvalidated assumption fails late. Treat the register as a working tool reviewed in every status cycle, not a one-time deliverable.

### Assigning Ownership to a Team Instead of a Person

Logging an assumption as owned by "the engineering team" or "operations" ensures no individual feels accountable for confirming it. The trap is that collective ownership feels collaborative but produces no action. Every load-bearing assumption needs a single named owner who is responsible for driving it to a confirmed or invalidated state.

### Failing to Trace Assumptions to Plan Elements

An assumption logged in isolation, with no link to the milestone or estimate it supports, cannot inform replanning when it fails. The trap is that the register looks complete but is disconnected from the plan it is meant to protect. Without traceability, a failed assumption triggers a scramble to figure out what is affected instead of a targeted response.

## Self-Check

- [ ] Has the plan been read as a stack of unproven claims, with the load-bearing premises surfaced explicitly?
- [ ] Is each logged entry categorized as fact, assumption, risk, or dependency, rather than blurred together?
- [ ] Are assumptions prioritized by consequence (what breaks if false), not just by likelihood?
- [ ] Is every assumption written as a specific, falsifiable statement naming actor, action, condition, and timeframe?
- [ ] Does each entry record source, a single named owner, and a confidence level?
- [ ] Were assumptions captured at the point they entered the plan, not reconstructed later from memory?
- [ ] Is each assumption traced to the specific deliverable, milestone, estimate, or decision it supports?
- [ ] Has the register been reviewed recently, with confirmed assumptions closed and invalidated ones converted to risks or issues?
- [ ] Were politically uncomfortable assumptions (sponsorship, funding, key-person reliance) deliberately probed rather than omitted?
- [ ] Does the register avoid vague entries that could never be proven true or false?; [ ] Are the premises behind optimistic estimates and critical dates logged rather than accepted as facts?
