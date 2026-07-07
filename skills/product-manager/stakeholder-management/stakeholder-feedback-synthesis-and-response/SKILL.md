---
name: stakeholder_feedback_synthesis_and_response.md
description: Use when the agent is collecting feedback from multiple stakeholders, synthesizing conflicting input, responding to review feedback, managing feedback loops on specs and designs, or deciding which feedback to incorporate and which to decline.
---

# Stakeholder Feedback Synthesis And Response

Stakeholder feedback is essential and dangerous in equal measure. Essential, because product managers who build in isolation ship things that fail in the real world of sales, support, legal, and customer reality. Dangerous, because feedback is rarely unified, frequently conflicting, and often phrased as solutions rather than concerns, and a product manager who tries to incorporate every piece of it produces a bloated, contradictory product that satisfies no one. The judgment problem is that feedback is not instruction; it is signal that must be interpreted, weighted by the credibility and perspective of its source, reconciled where it conflicts, traced back to the underlying concern, and responded to in a way that keeps the contributor engaged even when their specific suggestion is declined. Treating feedback as a to-do list produces a worse product; treating it as noise produces a disconnected one.

Use this skill before collecting stakeholder feedback on a spec, design, or roadmap, before responding to review comments, before reconciling conflicting input, or before deciding what to incorporate. The goal is to prevent the agent from either incorporating feedback uncritically until the product becomes incoherent, or dismissing feedback defensively until stakeholders stop contributing.

## Core Rules

### Treat Feedback As Signal, Not Instruction

Feedback is data about a concern, perspective, or risk, not a directive to implement a specific change. The product manager's job is to decode the signal, understand what the contributor is really worried about, and decide the right response, which may or may not be the proposed change.

For each piece of feedback, decode:

- the underlying concern or risk being expressed;
- the perspective and credibility of the source;
- whether the proposed solution addresses the concern well;
- whether the concern is real for the target user or only for this stakeholder;
- the cost and tradeoff of addressing it.

A stakeholder's solution is a hypothesis about their concern; evaluate the concern, not just the solution.

### Collect Feedback Structurally, Not Ad Hoc

Feedback gathered through hallway conversations and scattered emails is impossible to synthesize and easy to lose. A structured collection process ensures feedback is complete, attributable, and reviewable.

Structure collection by:

- defining who needs to review and on what cadence;
- using a shared artifact where comments are captured with attribution;
- setting a clear review window and decision date;
- distinguishing blocking concerns from suggestions;
- capturing the concern and the proposed solution separately.

Structured collection also makes it possible to show stakeholders that their input was received and considered, which sustains engagement.

### Weight Feedback By Source Credibility And Perspective

Not all feedback carries equal weight, and pretending it does either paralyzes decisions or dilutes the product. Weight feedback by how relevant the source's perspective is to the decision.

Weight by:

- proximity to the affected user or customer;
- expertise in the relevant domain, legal, security, UX, technical;
- accountability for the outcome the decision affects;
- representativeness, does this person speak for many or only themselves;
- track record of judgment in this area.

A support leader's feedback on support burden deserves more weight on that topic than on visual design. Match weight to relevance, not to seniority.

### Reconcile Conflicting Feedback Through Underlying Concerns

Stakeholders frequently contradict each other because they represent different users, segments, or priorities. Resolving conflict by picking a side alienates the other; resolving it by tracing both back to underlying concerns often reveals a solution that addresses both.

Reconcile by:

- stating each side's underlying concern explicitly;
- checking whether the concerns are truly in conflict or only appear so;
- looking for a solution that addresses both concerns at different levels;
- escalating to the shared objective both stakeholders serve;
- making the tradeoff explicit and the decision defensible when true conflict remains.

Two stakeholders arguing about a feature often share a deeper goal; surface it.

### Respond To Every Contributor, Even When Declining

Silence after feedback destroys engagement. Contributors who never hear back assume their input was ignored and stop contributing, and the product loses its most valuable feedback sources over time. A timely, explained response sustains the loop.

Respond by:

- acknowledging receipt and thanking the contributor;
- stating the decision and the one-sentence reason;
- connecting the decision back to their underlying concern;
- explaining what would change the decision;
- recording the feedback and outcome for future reference.

A declined suggestion with a clear reason builds more trust than a silent acceptance.

### Distinguish Blocking Concerns From Preferences

Some feedback identifies risks that must be addressed: legal exposure, security vulnerability, accessibility failure, data integrity. Other feedback expresses preference: a different layout, a different wording, a different priority. Conflating them either over-invests in preferences or under-invests in real risks.

Classify feedback:

- blocking: must be addressed before ship, real risk or requirement;
- important: should be addressed, affects quality or adoption;
- preference: optional, address if low cost and aligned;
- out of scope: acknowledge and defer explicitly.

Right-size the response to the classification.

### Close The Loop On Incorporated Feedback

When feedback changes the product, close the loop by telling the contributor how their input shaped the result. This converts feedback from a one-way dump into a collaborative relationship and increases the quality of future input.

Close the loop by:

- crediting the source where appropriate;
- explaining what changed and why;
- showing the contributor the revised artifact;
- inviting further review of the change;
- reinforcing that their input mattered.

### Prevent Feedback From Expanding Scope Silently

Feedback is a major source of scope creep, because each incorporated suggestion adds work and each addition is individually small. Without discipline, the cumulative effect bloats the product and slips the timeline.

Control scope by:

- requiring each addition to displace or defer something;
- routing non-trivial additions through prioritization, not silent acceptance;
- batching feedback into a structured revision rather than continuous accretion;
- separating must-address from defer-to-later explicitly;
- reviewing the cumulative scope impact before committing.

## Common Traps

### Feedback As To-Do List

Incorporating every suggestion uncritically until the product becomes bloated and contradictory.

### Defensive Dismissal

Rejecting feedback that challenges the team's preferences, causing contributors to disengage.

### Weighting By Seniority

Giving the loudest or most senior voice outsized influence regardless of relevance to the decision.

### Surface Solution Focus

Implementing the proposed change without understanding the underlying concern it expresses.

### Silent Loops

Never responding to contributors, destroying the engagement that makes feedback valuable.

### Conflating Preferences With Blockers

Treating optional preferences as requirements, or real risks as mere preferences.

### Scope Creep Through Feedback

Letting accumulated suggestions silently expand scope and slip the timeline.

### Picking Sides In Conflict

Resolving conflicting feedback by choosing a winner rather than tracing to shared underlying concerns.

## Self-Check

- [ ] Feedback is decoded into underlying concerns rather than implemented as literal instructions.
- [ ] Feedback is collected through a structured, attributable process with a clear review window and decision date.
- [ ] Each piece of feedback is weighted by source credibility and relevance to the decision, not by seniority or volume.
- [ ] Conflicting feedback is reconciled by tracing both sides to underlying concerns and the shared objective.
- [ ] Every contributor receives a timely response, including when their suggestion is declined, with the reason.
- [ ] Feedback is classified as blocking, important, preference, or out of scope, with response right-sized accordingly.
- [ ] Incorporated feedback is closed-loop, with contributors told how their input shaped the result.
- [ ] Non-trivial additions are routed through prioritization and made to displace existing work to prevent scope creep.
- [ ] The cumulative scope impact of feedback is reviewed before commitment.
- [ ] No contributor is left without a response, and no feedback is silently accepted or silently ignored.
