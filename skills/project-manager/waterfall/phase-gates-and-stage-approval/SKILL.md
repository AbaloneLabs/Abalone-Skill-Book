---
name: phase_gates_and_stage_approval.md
description: Use when the agent is designing or running phase gate reviews, defining stage approval criteria, deciding go or no-go kill points, governing transitions between waterfall phases, or structuring decision gates in a predictive lifecycle.
---

# Phase Gates and Stage Approval

Phase gates are the decision points that govern a predictive project, and their power is widely misunderstood. A real gate is a kill point: a moment where the project is honestly evaluated against criteria and where continuation is not guaranteed. When gates function this way, they concentrate attention, force unresolved issues to the surface, and prevent a weak project from consuming more investment on momentum alone. When gates are reduced to status updates that always approve continuation, they become expensive ceremonies that add delay without discipline, and the project loses its only structural mechanism for stopping bad direction early. The difference between a gate that governs and a gate that rubber-stamps is the difference between a controlled project and one that runs on hope.

The judgment problem is to design gates whose criteria genuinely predict downstream success, to run them with the authority to say no, and to resist the political gravity that turns every gate into a yes. Agents tend to set criteria that are easy to satisfy rather than predictive, to treat gates as reporting milestones rather than decision points, and to allow schedule pressure to override the signal a gate is meant to produce. The discipline of gating is the willingness to act on bad news, which most organizations find harder than the mechanics of the review itself.

## Core Rules

### Define Gate Criteria That Predict Downstream Success

Gate criteria should be the conditions that, if unmet, predict that the next phase will fail or waste money. A design gate that checks only whether documents exist is worthless; a design gate that checks whether the architecture is feasible, whether key risks are retired, and whether estimates are credible is useful. For each gate, ask: what would have to be false at this point for the next phase to fail? Those are your criteria. Avoid criteria that measure activity (documents produced, meetings held) rather than outcome (risks retired, feasibility confirmed, quality achieved).

### Make Gates Genuine Kill Points, Not Reporting Milestones

A gate that cannot say no is not a gate; it is a status meeting. For a gate to function, the authority to delay, rework, or kill the project must be real and must be exercised when criteria are unmet. If every gate approves regardless of findings, the team learns that gates are theater and stops preparing for them seriously. Establish who has kill authority, make it clear that a no-go is a possible and acceptable outcome, and act on it when warranted. A gate that has never said no is probably not doing its job.

### Separate Go, No-Go, and Conditional Approval

Gate decisions are not binary. A project can be approved to proceed (go), stopped (no-go), or approved with conditions (conditional go), such as proceeding while retiring specified risks or completing specified rework. Conditional approval is often the most useful outcome because it lets the project advance while forcing resolution of real issues. Define the decision options explicitly, and use conditional approval rather than forcing a false binary or defaulting to unconditional go.

### Time Gates to Maximize Option Value

Gates are most valuable when they occur before large, hard-to-reverse investments. A gate before build begins, when most of the spend is still ahead, has high option value because killing or redirecting the project there saves the most. A gate after build, when most money is spent, has low option value because the decision is largely foreclosed. Place gates where the information to decide meets the investment still at risk. Too few gates concentrate risk; too many gates add overhead without decision value.

### Require Evidence, Not Assertions

Gate criteria should be evaluated against evidence: test results, prototype outcomes, risk assessments, estimate ranges with bases. Assertions that the work is on track, without supporting evidence, are not a basis for a decision. Require that gate submissions include the evidence behind each criterion, and challenge submissions where evidence is thin. A gate that accepts assertions will approve weak projects because the weakness is invisible. Make evidence the currency of the gate.

### Include the Right Decision-Makers and Stakeholders

A gate's authority depends on who attends and decides. The decision-makers must have the authority to commit resources, kill the project, or redirect it, and the key stakeholders whose buy-in is needed must be present or formally represented. A gate run without the real decision-makers becomes a recommendation that must be re-litigated elsewhere, adding delay. Define the gate forum, its membership, and its decision authority, and ensure the people who can actually decide are in the room.

### Capture and Track Gate Conditions and Actions

When a gate grants conditional approval, the conditions become commitments that must be tracked to closure. When a gate raises actions or risks, they must be assigned owners and due dates and reviewed at the next gate. Gates that produce conditions and actions but do not track them to closure allow issues to recur at the next gate. Maintain a gate log of decisions, conditions, actions, and their status, and review prior gate items before each new gate.

### Resist Schedule Pressure Override

The most common gate failure is approving continuation because the schedule demands it, even when criteria are unmet. This converts the gate into a schedule-driven formality and guarantees that problems surface later, more expensively. When schedule pressure pushes toward approval despite unmet criteria, the honest response is conditional approval with mandatory remediation, not unconditional go. Protect the gate's integrity against the very pressure it exists to counter.

## Common Traps

### Gates as Status Reports That Always Approve

The gate reviews status and approves continuation regardless of findings, so it adds delay without discipline. The trap is that the gate loses its governing function. Make gates genuine kill points with real authority.

### Criteria That Measure Activity, Not Outcome

Gate criteria check whether documents were produced or meetings held, not whether risks are retired or feasibility confirmed. The trap is criteria that are easy to satisfy but predict nothing. Define criteria that predict downstream success.

### Rubber-Stamping Under Schedule Pressure

The gate approves because the schedule demands progress, even when criteria are unmet, deferring problems to a more expensive moment. The trap is letting schedule override signal. Use conditional approval to address real issues.

### Gates Placed Where Option Value Is Low

Gates cluster after most investment is spent, so killing the project there saves little. The trap is gates that decide too late. Place gates before large, hard-to-reverse commitments.

### Accepting Assertions Without Evidence

Gate submissions assert that work is on track without supporting evidence, so weakness is invisible. The trap is decisions on hope rather than fact. Require evidence for every criterion.

### Missing the Real Decision-Makers

The gate is run without the people who can actually commit, kill, or redirect, so decisions are re-litigated later. The trap is a gate that produces recommendations, not decisions. Ensure real authority is in the room.

### Conditions and Actions Not Tracked to Closure

Gate conditions and actions are recorded but not followed up, so the same issues recur at the next gate. The trap is accountability without closure. Maintain and review a gate log.

### Binary Go or No-Go Only

The gate forces a choice between unconditional approval and killing the project, so weak projects get unconditional approval because killing feels too drastic. The trap is the absence of conditional approval. Use conditional go to advance while forcing remediation.

## Self-Check

- [ ] Do gate criteria measure outcomes that predict downstream success (risks retired, feasibility confirmed, quality achieved) rather than activity (documents produced)?
- [ ] Is each gate a genuine kill point with real authority to delay, rework, or stop the project, and has that authority ever been exercised?
- [ ] Are decision options explicit, including conditional approval, and is conditional go used rather than forcing a binary or defaulting to unconditional approval?
- [ ] Are gates placed where the information to decide meets the investment still at risk, maximizing option value?
- [ ] Do gate submissions require evidence (test results, prototypes, estimate bases) rather than assertions for each criterion?
- [ ] Are the real decision-makers and necessary stakeholders present in the gate forum, with defined decision authority?
- [ ] Is a gate log maintained capturing decisions, conditions, actions, and their closure status, reviewed before each subsequent gate?
- [ ] Does the gate resist approving continuation solely because of schedule pressure when criteria are unmet?
- [ ] Has the gate ever produced a no-go or conditional approval, or does it always approve unconditionally (a sign it may be theater)?
- [ ] Are gate findings acted upon, or do the same issues recur at successive gates indicating non-closure?
