---
name: risk_register_design_and_maintenance.md
description: Use when the agent is creating or maintaining a risk register, deciding how to structure risk entries, establishing risk review cadences, ensuring risks are tracked to closure, or diagnosing why a risk register became stale and failed to catch real issues.
---

# Risk Register Design And Maintenance

A risk register is the central tool of risk management, the single source of truth for what could go wrong, how bad it would be, and what is being done about it. A well-designed and well-maintained register makes risks visible, assigns ownership, and tracks responses to closure. A poorly designed or neglected register becomes a compliance artifact, created once and ignored, that fails to catch the risks that actually materialize. The judgment problem is that a register's value comes from active maintenance, not initial creation, and maintenance is the part most often skipped. The most common failure is a register that is built at project start, grows stale, and bears no relationship to the risks the project actually faces as it evolves. The skill is designing a register that is usable and maintaining it as a living tool throughout the project.

Use this skill before creating a risk register, before establishing risk review processes, before diagnosing why risk management is ineffective, or when a risk materialized that was not in the register. The goal is to prevent the agent from creating a register that decays into a stale artifact, from tracking risks without ownership or closure, or from treating the register as documentation rather than an active management tool.

## Core Rules

### [ ] Design The Register For Usability And Action

A risk register that is too complex to maintain or too vague to act on will be abandoned. Each entry must contain the information needed to understand and manage the risk: description, category, probability, impact, owner, response, and status. The structure should make action obvious, not bury it in fields.

- [ ] Include for each risk: clear description, probability, impact, owner, response, status, due date.
- [ ] Keep the structure simple enough to maintain regularly.
- [ ] Make ownership and next action visible at a glance.
- [ ] Avoid fields that are collected but never used.

### [ ] Write Risk Descriptions That Are Specific And Actionable

Vague risks like "schedule slippage" or "resource issues" cannot be managed because they do not point to a cause or a response. A good risk description specifies the cause, the event, and the consequence: "Because vendor X has a history of late delivery, they may deliver component Y late, which would delay integration testing by two weeks." Specificity enables action.

- [ ] Describe the cause, the risk event, and the consequence.
- [ ] Make the description specific enough to assign an owner and response.
- [ ] Avoid generic risks that could apply to any project.
- [ ] Separate distinct risks rather than combining them.

### [ ] Assign A Single Accountable Owner Per Risk

Risks without an owner are unmanaged. Each risk must have one accountable owner responsible for monitoring and responding, even if multiple people contribute. Diffuse ownership means no one acts.

- [ ] Assign one accountable owner to each risk.
- [ ] Ensure the owner has the authority and capability to respond.
- [ ] Distinguish accountability (the owner) from contribution (others who help).
- [ ] Reassign ownership promptly when it changes.

### [ ] Establish A Regular Review Cadence

A register is only current if it is reviewed regularly. The cadence should match the project's pace: weekly for active projects, more often in crisis. Review confirms that risks are still relevant, responses are progressing, and new risks are captured. Without cadence, the register decays.

- [ ] Schedule regular risk reviews at a cadence matching project pace.
- [ ] Review each risk's status, probability, and impact for changes.
- [ ] Capture newly identified risks in each review.
- [ ] Close risks that have passed or been resolved.

### [ ] Differentiate Risks From Issues

Risks are uncertain future events; issues are problems that have already occurred. Confusing them pollutes the register and confuses response. Risks get preventive or contingency responses; issues get corrective action. Keeping them separate clarifies management.

- [ ] Maintain separate tracking for risks (future, uncertain) and issues (current, real).
- [ ] Use the appropriate response type for each.
- [ ] Convert risks to issues when they materialize.
- [ ] Avoid letting the register become an issue list or vice versa.

### [ ] Prioritize Risks To Focus Management Attention

Not all risks deserve equal attention. Prioritization by probability and impact, often through a risk score or matrix, focuses management on the risks that matter most. Without prioritization, attention scatters across all entries and the critical risks get lost.

- [ ] Score each risk by probability and impact.
- [ ] Use a risk matrix or ranking to prioritize.
- [ ] Focus active management on high-priority risks.
- [ ] Re-prioritize as probabilities and impacts change.

### [ ] Track Responses To Closure

A risk with a response plan but no follow-through is unmanaged. Each response must have an owner, a due date, and tracked status until complete or the risk closes. Tracking to closure ensures responses actually happen rather than remaining intentions.

- [ ] Assign an owner and due date to each response action.
- [ ] Track response status in each review.
- [ ] Confirm completion of response actions.
- [ ] Escalate responses that are stalled.

### [ ] Capture Risks From All Sources And Stakeholders

Risks identified only by the project manager reflect only one perspective. The richest risk identification draws from the whole team, stakeholders, and historical data. Broadening sources catches risks a single perspective misses.

- [ ] Solicit risks from team members, stakeholders, and subject matter experts.
- [ ] Use techniques like brainstorming, checklists, and assumption analysis.
- [ ] Review lessons learned from similar past projects.
- [ ] Create a culture where raising risks is valued, not punished.

### [ ] Keep The Register Aligned With The Evolving Project

As the project evolves, risks emerge, change, and pass. A register that reflects the project at kickoff but not at month three is useless. The register must evolve with the project, with new risks added, old ones updated or closed, and priorities shifted.

- [ ] Add new risks as they are identified throughout the project.
- [ ] Update probability and impact as conditions change.
- [ ] Close risks that have passed or been mitigated.
- [ ] Ensure the register reflects current, not historical, reality.

### [ ] Use The Register To Drive Decisions And Communication

The register's purpose is to inform decisions and communication, not to exist as documentation. It should surface in status reporting, steering meetings, and go/no-go decisions. A register that does not drive action has failed its purpose.

- [ ] Use the register to brief stakeholders on top risks and responses.
- [ ] Inform decisions with the current risk picture.
- [ ] Escalate risks that exceed the project's authority.
- [ ] Treat the register as a decision tool, not a filing requirement.

## Common Traps

### [ ] Create-And-Abandon

Building the register at kickoff and never maintaining it.

### [ ] Vague Risk Descriptions

Generic risks that cannot be owned, measured, or responded to.

### [ ] No Single Owner

Risks with diffuse or no ownership that no one manages.

### [ ] No Review Cadence

A register that decays because it is never reviewed and updated.

### [ ] Risks And Issues Confused

Mixing future uncertainties with current problems, confusing response.

### [ ] Equal Attention To All

Spreading management across all risks without prioritization.

### [ ] Responses Without Follow-Through

Response plans that are never executed or tracked to closure.

### [ ] Single-Perspective Identification

Missing risks because only the project manager identifies them.

## Self-Check

- [ ] Is the register designed for usability with the fields needed for action?
- [ ] Are risk descriptions specific, with cause, event, and consequence?
- [ ] Does each risk have a single accountable owner with authority to respond?
- [ ] Is there a regular review cadence that keeps the register current?
- [ ] Are risks and issues tracked separately with appropriate responses?
- [ ] Are risks prioritized to focus management attention where it matters?
- [ ] Are response actions tracked to closure with owners and due dates?
- [ ] Are risks captured from all sources, including the team and stakeholders?
- [ ] Does the register evolve with the project, adding, updating, and closing risks?
- [ ] Does the register drive decisions and communication rather than serve as documentation?
