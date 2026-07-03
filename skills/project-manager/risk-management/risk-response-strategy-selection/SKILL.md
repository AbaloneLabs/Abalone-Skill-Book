---
name: risk_response_strategy_selection.md
description: Use when the agent is choosing risk response strategies, deciding whether to avoid transfer mitigate or accept a risk, designing mitigation and contingency plans, or reviewing whether risk responses are proportional and owned.
---

# Risk Response Strategy Selection

Identifying a risk is only the beginning. The decision that protects the project is which response strategy to apply and how to make it real. Each strategy, avoid, transfer, mitigate, accept, and exploit, carries different costs, side effects, and limits. A risk listed with the response monitor and manage is not a response at all. The project manager must choose strategies deliberately, make them actionable, assign owners, and confirm the response is proportional to the risk rather than a ritual entry in a register.

Use this skill before selecting risk responses, designing mitigation or contingency plans, deciding whether to transfer or accept a risk, or reviewing a risk register for substance. The goal is to prevent the agent from filling a register with strategy labels that do not translate into action.

## Core Rules

### Match The Strategy To The Risk

The five core response strategies for threats are not interchangeable. Each fits a different situation.

- Avoid changes the plan to eliminate the risk, such as removing a risky component or choosing a proven vendor. Use when the risk is severe and the avoidance cost is acceptable.
- Transfer shifts the impact to a third party, such as insurance, a warranty, or a fixed-price contract. Use when another party can manage the risk better, and remember that transfer moves impact, not necessarily accountability for project success.
- Mitigate reduces probability or impact before the risk occurs, such as adding testing, building redundancy, or starting early. Use when reduction is cheaper than the expected impact.
- Accept acknowledges the risk and chooses not to act proactively, either passively or with a defined contingency. Use when the risk is small, when response cost exceeds benefit, or when no effective response exists.
- Exploit, share, and enhance are the positive counterparts for opportunities, ensuring that beneficial uncertainty is realized rather than left to chance.

Choosing a strategy is a cost-benefit decision, not a label.

### Make Every Response Actionable

A strategy without action is decoration. For each material risk, define the specific actions that implement the strategy, who will take them, by when, and what evidence shows they happened.

"Mitigate by close monitoring" is not actionable. "Mitigate by conducting a vendor dry-run integration test by week six, owned by the tech lead, with a pass-or-fail report" is actionable. If you cannot describe the action concretely, you have not chosen a response.

### Distinguish Mitigation From Contingency

Mitigation reduces the probability or impact of a risk before it happens. Contingency is the planned response if the risk happens. Both may be needed, but they are different.

A risk may have a mitigation that lowers the chance of late vendor delivery and a contingency that defines the fallback scope if delivery is still late after a trigger date. Confusing the two leaves the project with monitoring but no fallback, or with a fallback but no effort to prevent the problem.

### Define Triggers That Convert Uncertainty To Action

A risk under monitoring needs a trigger: a measurable condition or date that forces a decision or activates the contingency. Without a trigger, teams keep hoping and the window for action closes.

Triggers can be threshold-based, such as a vendor missing an interim milestone, date-based, such as a contingency decision date, or signal-based, such as a quality metric crossing a limit. Each trigger should have an owner watching it and a defined action when it fires.

### Assign A Single Owner Per Response

Every material risk and its response need one owner. Ownership means watching the signals, driving the mitigation, and escalating when the trigger fires. It does not always mean personally doing the work.

Risks with no owner become meeting topics. Risks with many owners become no one's responsibility.

### Consider Secondary Risks

Every response can create new risk. Avoiding a risky vendor may introduce a dependency on a less capable one. Transferring risk through a fixed-price contract may create cost risk if the vendor cuts quality to protect margin. Adding redundancy may increase complexity and integration risk.

When selecting a response, ask what new risk it introduces and whether the net effect is still favorable. Track significant secondary risks in the register rather than letting them hide.

### Keep Responses Proportional

The cost and effort of a response should be justified by the risk it addresses. Over-responding to low risks wastes resources and adds complexity. Under-responding to high risks leaves the project exposed.

Use expected value, probability times impact, as a sanity check on proportionality, but adjust for severity and for risks where the impact, while unlikely, would be catastrophic. Some high-impact low-probability risks deserve disproportionate response because survival matters.

### Review And Retire Responses

Risks change. A mitigation that was appropriate may become unnecessary as conditions change, or a risk may materialize into an issue that needs a different response. Review responses at a regular cadence.

Close risks that no longer apply, convert realized risks into issues, update responses as the situation evolves, and confirm that owners are still actively managing their risks.

## Common Traps

### Monitor And Manage As A Response

Listing a strategy without action is not a response. If there is no concrete action, the risk is effectively unmanaged.

### Strategy Mismatch

Transferring a risk the project still owns, or accepting a risk too severe to accept, produces false comfort.

### No Trigger Conditions

Monitoring without a trigger lets uncertainty drift past the point where action was possible.

### Diffuse Or Absent Ownership

Risks without a single owner are discussed but not driven.

### Ignoring Secondary Risks

A response that creates a new unmanaged risk can leave the project worse off.

### Over-Responding To Low Risks

Spending disproportionate effort on minor risks wastes resources and buries the risks that matter.

### Confusing Mitigation With Contingency

Mixing the two leaves the project without prevention or without a fallback.

### Static Register

Risks and responses frozen at kickoff stop reflecting the live project and stop protecting it.

## Self-Check

- [ ] Each material risk has a deliberately chosen strategy matched to its severity and to cost-benefit.
- [ ] Every response translates into a concrete action with an owner, a due date, and completion evidence.
- [ ] Mitigation and contingency are distinguished, and both are defined where needed.
- [ ] Each monitored risk has a measurable trigger that forces a decision or activates contingency.
- [ ] Each material risk has a single accountable owner.
- [ ] Secondary risks introduced by responses are identified and tracked.
- [ ] Responses are proportional to expected value and severity, including high-impact low-probability risks.
- [ ] The risk register is reviewed at a regular cadence and responses are retired or updated as conditions change.
- [ ] No risk carries a strategy label without an accompanying action.
- [ ] Realized risks are converted into issues with appropriate response rather than left in the risk register.
