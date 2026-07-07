---
name: incident_response_and_postmortem.md
description: Use when the agent is responding to a production incident (outage, degradation, data loss), acting as incident commander or on-call, performing triage and mitigation, running a blameless postmortem, writing an incident timeline and root-cause analysis, defining action items, or designing an incident response process. Covers incident severity and roles, mitigation-over-investigation priority, communication during incidents, blameless postmortem culture, root-cause analysis (5-whys, causal chains), and the action-item discipline that prevents recurrence.
---

# Incident Response And Postmortem

A production incident is a pressure test of an organization's engineering culture, not just its technical systems. Under time pressure, with users failing and executives watching, the temptation is to investigate the root cause immediately — to understand why the outage happened before fixing it. This is the first and most common incident-response error: investigation before mitigation prolongs the outage while the root cause is sought, when reverting the triggering change or failing over would have restored service in minutes. The defining principle of incident response is mitigate first, investigate later: restore service by whatever safe means is fastest (revert, failover, scale, disable a feature), then understand why it happened. The second defining principle is that the postmortem is blameless: incidents are caused by systemic vulnerabilities (a deploy process that allowed a bad change, a missing safeguard, an undocumented dependency), not by individual carelessness, and a culture that punishes individuals hides the systemic causes and guarantees recurrence.

Agents tend to investigate during the incident (delaying mitigation), to fix symptoms without finding the systemic cause, or to write postmortems that stop at the first plausible cause without tracing the causal chain to its systemic root. The judgment problem is recognizing that incident response is a disciplined process with defined roles and priorities, that mitigation speed depends on preparation (runbooks, rollback capability, failover plans) made before the incident, and that the postmortem's value is in the systemic causes and action items that prevent recurrence, not in assigning blame. This skill covers the discipline of incident response and postmortem: severity and roles, mitigation-first priority, communication, blameless root-cause analysis, and the action-item follow-through that turns an incident into improvement.

## Core Rules

### Mitigate First, Investigate Later

The overriding priority during an incident is restoring service, not understanding it. Investigation during the incident prolongs the outage.

- **Restore service by the fastest safe means before investigating root cause.** Revert the triggering deploy, fail over to a standby, scale up capacity, disable the feature causing the problem, or route traffic away from the failing component. The goal is users back to working, not understanding why they failed.
- **Have rollback and failover ready before you need them.** Mitigation speed depends on capabilities built before the incident: one-command rollback, tested failover, feature flags to disable problematic features, and runbooks for common incidents. An incident is the worst time to discover rollback does not work or failover is untested.
- **Do not make changes that could worsen the situation without consideration.** A hasty "fix" during an incident (deleting data, changing config, restarting a dependent service) can cause a second incident. Evaluate each mitigation for blast radius before applying it.
- **Once mitigated, preserve evidence for the postmortem.** Capture logs, metrics, thread dumps, and the state of the system before they rotate or are overwritten. The postmortem needs this evidence; an incident that is mitigated but whose evidence is lost cannot be fully understood.

### Define Severity, Roles, And Communication

An incident needs a clear severity (how bad is it), clear roles (who does what), and clear communication (who needs to know). Without these, response is chaotic.

- **Define severity levels that drive the response.** A sev-1 (total outage, user-data impact) triggers immediate paging, an incident commander, and executive notification; a sev-2 (degraded service) triggers urgent response without the full escalation. Severity determines who is involved and how fast.
- **Assign an incident commander (IC) to coordinate.** The IC is not debugging; the IC is managing the response — tracking what has been tried, deciding next steps, coordinating communication, and keeping the response organized. Without an IC, multiple people work at cross-purposes and steps are missed.
- **Separate the roles: IC coordinates, responders debug, a communicator updates stakeholders.** One person cannot coordinate, debug, and communicate simultaneously. Split the roles so each is done well; the communicator keeps stakeholders (internal teams, customers, executives) informed on a regular cadence.
- **Communicate on a regular cadence.** Stakeholders need updates at a defined interval (every 15-30 minutes for a sev-1), even if the update is "still investigating." Silence breeds speculation and escalation; regular updates contain the communication.

### Run A Blameless Postmortem

The postmortem is where the incident becomes learning, and its culture determines whether the organization improves or repeats its mistakes. A blameless postmortem focuses on systemic causes, not individual fault.

- **Make the postmortem blameless.** Individuals do not cause incidents; systems do. A deploy that broke production was not "the engineer's fault" — it was a deploy process that allowed an unverified change, a missing test that should have caught it, a safeguard that was absent. Punishing individuals hides these systemic causes and makes future incidents more likely (people conceal mistakes and near-misses).
- **Focus on the systemic causes and the conditions that allowed the incident.** Why did the bad change pass review? Why did tests not catch it? Why did the safeguard fail? Why was the dependency undocumented? These are the questions that find the systemic vulnerability.
- **Trace the full causal chain, not just the trigger.** The trigger (the specific bad change) is the last link; the causal chain includes the deploy process, the test coverage, the safeguard design, the architectural dependency. Fixing only the trigger prevents this exact incident; fixing the chain prevents the class of incidents.
- **Include a detailed timeline.** The postmortem should reconstruct the incident timeline: when it started, when it was detected, when mitigation began, when service was restored. The timeline reveals detection delay, mitigation delay, and communication gaps that are themselves findings.

### Perform Rigorous Root-Cause Analysis

Root-cause analysis (RCA) is the process of tracing the incident to its systemic origin. It must go beyond the first plausible cause to the underlying conditions.

- **Use 5-whys or causal-chain analysis to go deeper.** For each cause identified, ask why it occurred, and repeat until the cause is systemic (a process, a design, a culture) rather than individual. Stopping at the first cause ("the deploy broke it") misses the systemic root ("the deploy process lacks canary verification").
- **Identify all contributing factors, not just one root cause.** Incidents usually result from multiple contributing factors (a bad change, a missing test, a delayed alert, an undocumented dependency). Each is a finding; each has its own action item. A single "root cause" oversimplifies.
- **Distinguish the trigger from the vulnerability.** The trigger (the bad change) is what set it off; the vulnerability (the missing safeguard) is what allowed it to cause an incident. The vulnerability is the durable finding; the trigger is specific to this incident.
- **Question whether the "root cause" is real.** A superficial RCA finds a plausible cause and stops; a rigorous RCA tests the cause (does it explain the full timeline? does it explain why the incident happened now and not before?) and continues if the explanation is incomplete.

### Define And Track Action Items To Completion

A postmortem without follow-through is a document, not improvement. Action items must be specific, owned, and tracked to completion.

- **Define specific, actionable action items tied to each finding.** "Improve testing" is not actionable; "add integration test X covering scenario Y, owned by Z, due in two weeks" is. Each action item has a clear owner, a due date, and a verifiable completion criterion.
- **Prioritize action items by impact and effort.** Not all action items are equal; some prevent a class of incidents (high impact), others address a specific edge case (lower impact). Prioritize the high-impact items; do not let the list become a backlog that is never completed.
- **Track action items to completion and verify they work.** An action item is not done when it is written; it is done when the safeguard is implemented, tested, and verified to prevent the incident class. Track status; review stale items; verify effectiveness.
- **Share learnings across the organization.** An incident in one team often reveals a vulnerability others share. Share the postmortem and action items broadly so other teams can apply the learnings to their own systems.

## Common Traps

### Investigating Before Mitigating

Seeking the root cause during the incident, prolonging the outage while the cause is found. Mitigate first (revert, failover, disable); investigate in the postmortem.

### No Incident Commander Or Role Separation

Multiple people working at cross-purposes, with no one tracking what has been tried or coordinating communication. Assign an IC; separate coordination, debugging, and communication.

### Infrequent Or Absent Stakeholder Communication

Silence during an incident, breeding speculation and escalation. Communicate on a regular cadence, even if the update is "still investigating."

### Blameful Postmortem Culture

Assigning individual fault, which hides systemic causes and makes future incidents more likely as people conceal mistakes. Make postmortems blameless; focus on systemic vulnerabilities.

### Stopping At The Trigger Instead Of The Systemic Cause

Fixing the specific bad change without addressing the process, test, or safeguard that allowed it. Trace the full causal chain; fix the class of incident, not just this instance.

### Single "Root Cause" Oversimplification

Attributing the incident to one cause when multiple factors contributed, missing the other findings and action items. Identify all contributing factors.

### Action Items Not Tracked To Completion

A postmortem with action items that are never implemented or verified, so the vulnerability persists. Assign owners, due dates, and verification; track to completion.

### Lost Evidence Before The Postmortem

Logs, metrics, and state rotating or being overwritten before the postmortem can use them, preventing full understanding. Preserve evidence during mitigation.

## Self-Check

- [ ] During the incident, mitigation (restoring service by the fastest safe means — revert, failover, scale, disable) took priority over investigation, rollback/failover/feature-flag capabilities were ready before the incident, changes were evaluated for blast radius before applying, and evidence (logs, metrics, state) was preserved for the postmortem.
- [ ] Severity levels drove the response (who is involved, how fast), an incident commander coordinated (tracking attempts, deciding next steps), roles were separated (IC, responders, communicator), and stakeholders received updates on a regular cadence.
- [ ] The postmortem was blameless — focusing on systemic causes (processes, designs, safeguards) rather than individual fault — and traced the full causal chain from trigger to systemic vulnerability, not stopping at the first plausible cause.
- [ ] Root-cause analysis used 5-whys or causal-chain tracing to go beyond the trigger, identified all contributing factors (not a single oversimplified root cause), distinguished the trigger from the durable vulnerability, and questioned whether the identified cause fully explains the incident timeline.
- [ ] The postmortem includes a detailed timeline (detection time, mitigation start, restoration time) that reveals detection delay, mitigation delay, and communication gaps as findings.
- [ ] Action items are specific and actionable (clear owner, due date, verifiable completion criterion), prioritized by impact, tracked to completion with effectiveness verified, and shared across the organization so other teams can apply the learnings.
- [ ] The incident's class (not just this specific instance) is addressed — the systemic vulnerability (deploy process, test coverage, safeguard design, architectural dependency) is fixed so similar incidents are prevented, not just this exact trigger.
- [ ] The response process itself is reviewed: was detection fast enough (alerting — see alerting-design), was mitigation fast enough (rollback/failover readiness — see deployment skills), was communication effective, and are there process improvements alongside technical action items.
