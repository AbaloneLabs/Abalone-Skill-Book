---
name: business-continuity-plan-design.md
description: Use when the agent is designing a business continuity plan, defining continuity strategies, deciding degraded service modes, planning manual fallbacks, assigning continuity roles, preparing disruption procedures, or determining how operations should continue during site, staffing, vendor, system, weather, safety, or large-scale disruption.
---

# Business Continuity Plan Design

A business continuity plan is a set of operating choices for disruption. It decides what continues, what pauses, who decides, how work is routed, how customers and staff are informed, and how risk is accepted while normal conditions are unavailable. Agents often write broad continuity checklists that do not tell a team what to do during a real disruption. This skill helps the agent design continuity plans that are actionable, prioritized, and testable.

## Core Rules

### Start From Critical Processes

Use the critical process review as the foundation. For each process, name the continuity objective, maximum tolerable downtime, minimum viable operation, dependencies, and owner. Do not create a generic plan that treats every activity equally.

The plan should make triage explicit. During disruption, preserving essential service may require pausing lower-priority work.

### Define Disruption Scenarios

Plan for credible scenarios: site outage, system outage, vendor failure, workforce shortage, severe weather, safety event, data access loss, network failure, power loss, transport disruption, civil emergency, or regional event. Scenarios should test different dependencies.

Avoid planning only for the last incident. The plan should be flexible enough to handle different causes while preserving critical outcomes.

### Choose Continuity Strategies

Options include manual fallback, alternate site, remote work, alternate vendor, cross-trained backup team, queue deferral, service tier reduction, emergency staffing, offline records, alternate communication channel, or temporary policy exception. Match strategy to recovery time and risk.

Each strategy should state when it is activated, who approves it, what it can handle, and what risk it creates.

Compare strategies against likely duration. A workaround that is safe for two hours may create unacceptable reconciliation, privacy, or fatigue risk if used for three days.

### Define Roles And Decision Rights

Assign continuity lead, process owners, communications owner, staffing coordinator, vendor contact, risk or compliance reviewer, technology contact, facilities contact, and recovery owner. Smaller operations can combine roles, but responsibilities must remain clear.

Decision rights should cover activating the plan, pausing work, changing service levels, approving manual work, spending emergency funds, and accepting residual risk.

### Specify Manual Fallbacks Carefully

Manual fallback must include forms, required fields, approval rules, data capture, reconciliation, privacy handling, customer communication, and later system entry. A manual process that lacks evidence or reconciliation can create more risk than the outage.

Define volume limits. A manual fallback may work for ten cases but fail for a thousand.

### Plan Communication Channels

Define how staff, leaders, vendors, customers, and internal stakeholders receive updates if normal channels fail. Include contact lists, templates, status cadence, escalation paths, and backup channels.

Messages should say what service is available, what is paused, what workaround exists, what not to promise, and when the next update will occur.

### Protect Records And Compliance

Continuity mode can tempt teams to bypass controls. Define which controls remain mandatory, which are modified, who approves modifications, and what evidence is required. Include privacy, security, financial, safety, employment, and regulatory obligations where relevant.

If a control is temporarily relaxed, record residual risk and review after normalization.

Plan how evidence is stored when normal systems are unavailable. Paper notes, offline spreadsheets, screenshots, and shared drives need ownership, access limits, retention rules, and later upload or destruction instructions.

### Define Recovery And Reconciliation

The plan should not stop at operating during disruption. Define how manual records are entered, backlogs are cleared, duplicate work is prevented, customer promises are reconciled, vendors are followed up, and temporary permissions are removed.

Recovery work needs owners and capacity. Otherwise continuity mode leaves hidden debt.

### Make The Plan Usable Under Stress

Use clear triggers, role cards, quick-reference actions, contact paths, and decision tables. Long policy documents are not enough during an outage. The plan should help a tired manager act quickly without inventing the process.

Keep current versions accessible when systems are down.

## Common Traps

- Designing one broad continuity plan without critical process priorities.
- Planning only for the last disruption scenario.
- Saying "manual process" without fields, controls, reconciliation, and volume limits.
- Assigning roles but not decision rights.
- Forgetting how communication works if normal tools are unavailable.
- Reducing controls during disruption without approval, evidence, or residual risk ownership.
- Maintaining service promises that continuity capacity cannot meet; ignoring recovery and reconciliation until after the disruption
- Writing a plan that is too long or inaccessible to use during stress; treating the plan as complete without testing whether people can execute it

## Self-Check

- Is the plan based on prioritized critical processes and minimum viable operation?
- Are disruption scenarios broad enough to test different dependencies?
- Are continuity strategies tied to activation triggers, approval, capacity, and risk?
- Are continuity roles and decision rights explicit?
- Are manual fallbacks defined with data, approval, privacy, reconciliation, and volume limits?
- Are backup communication channels, templates, cadence, and escalation paths available?
- Are mandatory controls and approved temporary control changes clear?
- Does the plan include recovery, reconciliation, backlog, duplicate prevention, and access rollback?
- Is the plan usable under stress and available during system outage?
- Is plan execution scheduled for testing rather than assumed?
