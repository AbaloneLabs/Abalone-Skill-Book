---
name: operational-risk-register-and-prioritization.md
description: Use when the agent is building or reviewing an operational risk register, identifying process risks, prioritizing risks by likelihood and impact, assigning risk owners, reviewing early warning indicators, or deciding which operational risks need controls, monitoring, mitigation, or formal acceptance.
---

# Operational Risk Register And Prioritization

An operational risk register is useful only if it changes attention and action. A list of risks without owners, evidence, indicators, controls, and review cadence becomes a static artifact. Agents often write broad risks such as "process failure" or "vendor issue" without specifying the event, impact, cause, and response. This skill helps the agent turn operational risk identification into a practical management tool.

## Core Rules

### Define The Risk Event Clearly

Write each risk as an event that could happen, not as a vague topic. "Refund approvals bypass review" is better than "refund risk." "Single certified reviewer unavailable during month-end peak" is better than "staffing risk."

Include cause, event, and impact where possible. This makes it easier to design controls and decide priority.

### Tie Risk To Operational Impact

Assess impact across customers, safety, compliance, privacy, financial loss, service level, quality, reputation, employee burden, vendor dependency, and business continuity. A risk with low volume may still be high impact if harm is severe or irreversible.

Avoid ranking only by frequency. Rare events may deserve strong controls when consequences are high.

### Estimate Likelihood With Evidence

Use incident history, near misses, audit findings, queue aging, defect rates, turnover, vendor performance, system logs, process complexity, manual touchpoints, and expert judgment. Label confidence. If evidence is weak, say so and decide whether monitoring is needed.

Do not present risk scoring as objective precision when it is a structured judgment.

### Assign A Real Owner

Every risk needs an accountable owner who can monitor, mitigate, escalate, or accept it. The owner may need support from other functions, but accountability cannot sit with "the team" or "operations" in general.

If no current owner has authority to reduce the risk, escalate the ownership gap. A risk register that names powerless owners is misleading.

### Identify Existing Controls And Gaps

For each material risk, record current preventive controls, detective controls, corrective actions, monitoring, and fallback plans. Note whether controls are documented, operating, tested, manual, automated, or dependent on one person.

Do not assume a control exists because a policy says it should. Check whether it operates in real work.

### Define Early Warning Indicators

Use indicators that signal risk before harm occurs: rising backlog age, more exceptions, increased manual overrides, access anomalies, missed reconciliations, vendor delays, staffing gaps, quality drift, near misses, or customer complaints. Early warning should trigger action, not just reporting.

For slow-moving risks, review trend. For fast-moving risks, define immediate escalation thresholds.

Include indicator owners and response playbooks for the highest risks. A warning is only useful if someone knows what to check, what action is allowed, and when the issue becomes an incident or formal escalation.

### Prioritize Mitigation By Risk And Feasibility

Compare severity, likelihood, control weakness, regulatory or safety obligation, reversibility, cost, implementation effort, and time to effect. High-risk, weakly controlled, quickly spreading issues should rise first.

Do not spend all effort on risks that are easy to fix while high-harm risks remain accepted by default.

### Keep The Register Current

Update risks after incidents, process changes, staffing changes, vendor changes, product launches, automation, audit findings, or service promise changes. Retire risks that no longer apply and add new risks created by mitigation.

A stale register can create false comfort. Review cadence should match the risk environment.

### Link Register To Decisions

The register should feed operating reviews, investment decisions, control testing, training, vendor management, business continuity, and escalation. If it never affects decisions, simplify it or redesign the governance.

Each high-priority risk should have a current action, monitoring plan, or explicit acceptance.

Review dependencies between risks. A staffing risk may weaken controls, a vendor risk may increase service-level risk, and a data-quality risk may make every dashboard less reliable. Linked risks may need coordinated mitigation rather than separate local actions.

## Common Traps

- Listing broad topics instead of clear risk events with impact.
- Ranking only by how often something happens and missing rare high-harm risks.
- Treating risk scores as precise math rather than judgment with confidence limits.
- Assigning owners who do not have authority to reduce or escalate the risk.
- Recording controls from policy documents without checking whether they operate.
- Failing to define early warning indicators or thresholds.
- Prioritizing easy fixes while leaving severe risks unmanaged; letting the register go stale after process, vendor, staffing, or system changes
- Keeping risks open forever with no action, monitoring, or acceptance; maintaining a register that does not feed any operating decision

## Self-Check

- Is each risk written as a specific operational event with plausible cause and impact?
- Are impact dimensions broader than volume and cost?
- Is likelihood based on evidence or clearly labeled judgment?
- Does each risk have an accountable owner with enough authority or escalation path?
- Are existing controls described by how they actually operate?
- Are control gaps and single-person dependencies visible?
- Are early warning indicators and thresholds defined?
- Is mitigation prioritized by risk, feasibility, obligation, and time to effect?
- Is there a cadence to update the register after operating changes and incidents?
- Does every high-priority risk have mitigation, monitoring, or explicit acceptance?
