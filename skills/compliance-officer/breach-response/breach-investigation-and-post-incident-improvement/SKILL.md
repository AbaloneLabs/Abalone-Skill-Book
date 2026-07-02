---
name: breach-investigation-and-post-incident-improvement.md
description: Use when the agent is conducting a post-breach forensic investigation and root cause analysis, coordinating with forensic experts and law enforcement, managing privilege over investigation findings, designing remediation to prevent recurrence, or conducting a post-incident review to improve the breach response capability.
---

# Breach Investigation and Post-Incident Improvement

Containing and notifying a breach is not the end of the response; it is the midpoint. The investigation that follows determines the root cause, the full scope, the accountability, and — most importantly — what must change so the breach does not recur. Organisations that contain and notify but never investigate root cause or improve their controls experience repeat breaches from the same unaddressed weaknesses. The investigation also carries its own legal complexity: findings may be privileged or discoverable, may expose regulatory or contractual liability, and may be scrutinised by authorities. This skill addresses the judgment involved in investigating a breach thoroughly, managing the legal dimensions of the investigation, and translating findings into durable improvement.

## Core Rules

### Conduct a structured root cause analysis, not just a narrative

A breach investigation must identify not only what happened but why it was possible — the root cause. Use a structured method (for example, the five whys, fault tree, or causal factor analysis) to trace from the incident back to the enabling conditions:

- What was the immediate technical cause (the exploit, the misconfiguration, the error)?
- What control failed or was absent that allowed it?
- What process, policy, or governance gap permitted the control failure?
- What systemic condition (resourcing, culture, prioritisation) underlay the gap?

A narrative that stops at "a phishing email led to credential compromise" misses that multi-factor authentication was not enforced, that the email filter was misconfigured, and that security training had lapsed. Trace to the systemic root.

### Preserve and analyse evidence with forensic rigour

The investigation relies on evidence: logs, images, network captures, system configurations, access records, and communications. Ensure:

- Evidence is preserved before systems are rebuilt (captured in the containment phase);
- A chain of custody is maintained for evidence that may support legal or enforcement action;
- Forensic analysis is conducted by qualified personnel (internal or external experts);
- Analysis determines the scope: what data was accessed, exfiltrated, altered, or destroyed, and which data subjects were affected.

Forensic conclusions that are not based on rigorously preserved and analysed evidence are speculative and indefensible.

### Manage legal privilege and discoverability deliberately

Investigation findings may be privileged (where conducted under legal direction for the purpose of legal advice or anticipated litigation) or discoverable (where conducted for ordinary business purposes). The privilege status depends on how the investigation is structured:

- Engage counsel to direct the investigation;
- Use forensic experts engaged through counsel where privilege is desired;
- Label work product appropriately;
- Understand that mandatory notifications and regulatory disclosures may waive privilege over shared content.

Do not assume investigation findings are privileged by default; structure the investigation to achieve the desired status, and consult counsel on what must be disclosed versus what may remain privileged.

### Determine the full scope and affected parties

The initial breach assessment is often incomplete. The investigation must refine:

- The exact data categories and volumes affected;
- The data subjects affected (customers, employees, third parties);
- Whether the breach affected other controllers' or processors' data;
- Whether the breach is ongoing or fully contained;
- The temporal scope (when did it start, when was it contained).

Scope refinement may require supplemental notifications if the initial notification underestimated the impact. Plan for phased notification as scope clarifies.

### Engage external experts and law enforcement where appropriate

For significant breaches, engage:

- **External forensic and incident response firms**: for expertise, capacity, and independence;
- **Law enforcement**: for cybercrime, extortion, or where criminal investigation is warranted;
- **Breach counsel**: for regulatory strategy and privilege management;
- **Cyber insurer**: under policy notification and cooperation conditions;
- **Public relations**: for external communications strategy.

Engage external experts early; their value diminishes if engaged after internal actions have compromised evidence or privilege.

### Design remediation that addresses root cause, not just symptoms

Remediation must address the root causes identified in the investigation, not only the immediate symptoms. For each root cause, define:

- The specific corrective action (for example, enforce MFA, patch the vulnerability, rewrite the procedure, retrain the team);
- The owner and deadline;
- The verification that the corrective action was effective.

Symptomatic fixes (blocking one IP, resetting one set of credentials) without systemic fixes leave the root cause in place. Track remediation to completion and verify effectiveness.

### Conduct a post-incident review of the response itself

Separate from the technical root cause, review how the response performed:

- Was detection timely, or was the breach undetected for too long?
- Was triage and escalation effective?
- Was the 72-hour notification met, and was the notification accurate?
- Were roles clear and coordination effective?
- What failed in the response that should be improved?

Document the lessons learned and update the incident response plan, runbooks, and training accordingly. A response that is never reviewed cannot improve.

### Feed findings into the broader control environment

Breaches reveal weaknesses that often extend beyond the specific incident. Use the findings to:

- Update the risk assessment and control framework;
- Inform DPIAs for affected processing;
- Revise vendor management and security requirements;
- Brief senior management and the board on systemic issues.

A breach is costly intelligence; failing to use it to improve the broader environment wastes the learning.

## Common Traps

### Stopping the investigation at the immediate technical cause

The investigation identifies the exploit but not the control failure or governance gap that enabled it. The same weakness causes the next breach. Trace to root cause.

### Investigation without privilege management

Findings are generated in the ordinary course and are fully discoverable, exposing the organisation to litigation and enforcement. Structure the investigation under counsel where privilege matters.

### Remediation that addresses symptoms, not root cause

Patching the specific vulnerability or blocking the specific actor without addressing why the vulnerability existed or why the control was absent. Track systemic fixes.

### No post-incident review of the response

The breach is closed without examining how the response performed. The same response gaps (slow detection, unclear roles, missed notification) recur in the next incident.

### Engaging external experts too late

Experts are brought in after internal actions have overwritten logs or compromised evidence. Their analysis is limited by what was preserved. Engage early.

### Failing to supplement notifications as scope clarifies

The initial notification underestimated the scope, and the refined scope is not communicated to the authority or data subjects. Supplement as the picture clarifies.

## Self-Check

- Does the investigation use a structured root cause method to trace from the immediate cause to the systemic enabling condition?
- Is evidence preserved with chain of custody and analysed by qualified forensic personnel to determine the full scope and affected parties?
- Is the legal privilege status of the investigation deliberately managed (counsel direction, expert engagement, labelling, disclosure awareness)?
- Is the scope refined as the investigation progresses, with supplemental notifications where the initial assessment underestimated impact?
- Are external experts, law enforcement, counsel, insurer, and PR engaged early where the breach warrants?
- Does remediation address root causes with specific actions, owners, deadlines, and effectiveness verification, not just symptomatic fixes?
- Is a post-incident review conducted of the response itself, with lessons learned fed into the IR plan, runbooks, and training?
- Are findings used to update the broader risk assessment, control framework, DPIAs, and vendor requirements, and briefed to senior management?
