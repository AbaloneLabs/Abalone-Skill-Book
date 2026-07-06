---
name: severity_and_escalation.md
description: Use when the agent is defining or applying incident severity levels, deciding when to escalate and to whom, setting up on-call rotation and paging policy, drafting an incident communication protocol for stakeholders and customers, deciding who has authority to roll back or fail over during an incident, or reviewing whether an incident was under- or over-escalated. Also covers the failure mode of a sev that does not reflect user impact, escalation paths that route to the wrong person or fire too late, noisy pages that burn out responders, and the ambiguity of severity at the moment a symptom first appears.
---

# Severity And Escalation

Severity is the judgment you make in the first minutes of an incident about how bad this is and how urgently it must be addressed, and escalation is the set of paths that move the incident to the people and level of authority it needs. Both are decisions made under uncertainty and time pressure, and both are easy to get wrong in characteristic ways. Severity tends to drift toward the responder's comfort — an engineer who knows the system calls a partial outage "minor" because they can fix it fast, while a customer losing revenue on the same system experiences it as critical. Escalation tends to lag because responders want to solve it themselves before "bothering" someone, so a fast-moving incident burns the first responder's attention while the people who could help sit idle. The judgment problem is to classify severity by user and business impact (not by the responder's familiarity), to define escalation triggers that fire on objective conditions rather than on a human's reluctance to ask for help, and to make the authority to act (roll back, fail over, spend money, take the service down) explicit so that no one is stuck waiting for permission during the worst minutes.

Agents tend to treat severity as a label and escalation as a contact list, but both are load-bearing decisions. A severity that understates impact means the incident gets too few people too late; one that overstates impact means responders burn out from false alarms and the next real incident gets less attention. An escalation path that routes to a named individual (who is on vacation) rather than a role, or that requires three hops to reach someone who can authorize a rollback, means the incident stalls at the moment speed matters most. The discipline is to anchor severity on measurable user and business impact, to make escalation automatic and role-based, to rehearse the paths before the incident, and to separate the decision-making authority (who decides what to do) from the communication responsibility (who tells stakeholders) so neither blocks the other.

## Core Rules

### Classify Severity By User And Business Impact, Not By Responder Familiarity

Severity exists to align the response to the harm. The correct anchor is the impact on users and the business: how many users are affected, how severely (degraded vs. down, recoverable vs. data loss), how much revenue or trust is at stake, and whether the impact is growing. The wrong anchor is the responder's confidence — "I can fix this in five minutes" is not a severity, because if you are wrong, the incident ran at the wrong priority for the whole five minutes.

- **Anchor on user and business impact.** Full outage of a revenue path is top severity regardless of how fixable it looks; a cosmetic bug in an admin panel is low severity regardless of how mysterious it looks.
- **Use objective, pre-agreed criteria.** Define each level by measurable conditions (e.g., "X% of requests failing," "a paid feature unavailable," "data loss confirmed," "security incident with active exfiltration") so two people looking at the same symptom assign the same severity.
- **Re-classify as the picture changes.** Severity is set in the first minutes on partial information and must be revised as impact grows or shrinks. A "minor" that turns into a full outage must be re-classified immediately, not left at the original level because that is what was paged.
- **When uncertain between two levels, lean to the higher.** The cost of over-classifying once is a little wasted attention; the cost of under-classifying a real incident is a slow response during the worst minutes.

### Define Escalation Triggers On Objective Conditions, Not On Reluctance

Escalation should fire when objective conditions are met, not when a responder decides they have struggled enough. The failure mode is a responder who wants to solve it themselves and waits too long to pull in help or authority, so the incident runs on one person's bandwidth while others who could help sit idle.

- **Time-based triggers.** "If not resolved or contained in N minutes, escalate" forces help to arrive on a schedule rather than on a human's judgment about their own struggle.
- **Impact-based triggers.** A severity jump (minor → major), a scope expansion (one service → many), or a confirmed data-loss/security condition triggers escalation automatically.
- **Role-based routing, not name-based.** Escalate to a role (on-call SRE, service owner, incident commander, duty manager) backed by a rotation, not to "Alice," who may be asleep or on vacation.
- **Make the "pull in more help" path low-friction.** Responders should not have to overcome social friction to add people; the default during a real incident is more hands, not fewer.

### Make Decision Authority Explicit Before The Incident

During the worst minutes, no one should be stuck waiting for permission to act. Decide in advance who has the authority to roll back, fail over, take the service down, spend money on a fix, or make a customer-visible decision, and make that authority known to responders.

- **Pre-authorize reversible high-value actions.** Rollback and failover are usually pre-authorized for the on-call responder because they are reversible and time-critical; waiting for a manager to approve a rollback is a common cause of needless outage minutes.
- **Identify who can authorize irreversible or costly actions** (taking the service down, a destructive data operation, a public statement) and make that person reachable through the escalation path.
- **Separate the incident commander from the hands-on fixer.** One person holds the big picture, coordinates, and decides; others execute. When the same person is both debugging and coordinating, both suffer.
- **Separate decision authority from communication responsibility.** The person deciding the technical response and the person communicating to stakeholders and customers should be different, so neither blocks the other.

### Design The Communication Protocol For Stakeholders And Customers

Communication during an incident is a separate workstream from the fix, and it has its own severity-dependent cadence. Stakeholders and customers need timely, accurate updates even when — especially when — there is nothing new to say, because silence breeds speculation and erodes trust.

- **Define a cadence per severity.** A top-severity incident may warrant updates every 15–30 minutes; a minor one may warrant a single note at resolution. Define the cadence in advance so the responder is not improvising under pressure.
- **Designate a communicator.** One person owns stakeholder and customer communication so engineers can focus on the fix and the message stays consistent.
- **Say what is known, what is unknown, and what is being tried.** Honest, bounded updates ("we have identified the likely cause and are testing a rollback; next update in 15 minutes") build more trust than overconfident claims or silence.
- **Pre-draft templates.** Having a skeleton for the first response, the ongoing update, and the resolution notice speeds communication and prevents omitting important details under stress.

### Design On-Call To Be Sustainable

On-call is the human substrate of escalation. If it burns people out, the escalation path collapses when the next person is too exhausted to respond effectively. Sustainable on-call is a correctness requirement, not a perk.

- **Rotate fairly and respect rest.** No one should be primary on-call continuously; follow-the-sun rotations, reasonable shift lengths, and protected recovery time prevent the burnout that makes the whole system fragile.
- **Page for actionable symptoms only.** Every non-actionable page (the "FYI" alert, the symptom that resolves itself) trains responders to ignore pages and erodes the signal needed for real incidents.
- **Make handoffs explicit.** A shift change during an active incident must transfer the current picture, the actions tried, and the next steps, not just the pager.

### Document the Basis and the Reasoning

Every conclusion should be traceable to its evidence, assumptions, and alternatives considered. Record not only the outcome but the reasoning path: what was checked, what was ruled out, what uncertainty remains, and what would change the conclusion. Documentation that captures the basis allows another practitioner to review, reproduce, or challenge the work, and it prevents confident conclusions from becoming unrepeatable assertions. A decision made without a recorded basis cannot be audited, improved, or safely handed off.

## Common Traps

### Severity Anchored On Responder Familiarity

Classifying an incident as minor because the responder knows how to fix it fast, when the user impact is large — if the fix estimate is wrong, the incident ran at the wrong priority the whole time. Anchor severity on user and business impact, not on the responder's confidence.

### Escalation That Lags Because Responders Want To Solve It Themselves

A responder who waits too long to pull in help or authority, running a fast-moving incident on one person's bandwidth while others sit idle. Use objective time- and impact-based triggers so escalation fires on a schedule, not on a human's reluctance.

### Name-Based Escalation Routing

Escalation paths that route to a named individual who is asleep, on vacation, or no longer on the team, so the page lands nowhere. Route to roles backed by a rotation.

### Authority Bottlenecks During The Worst Minutes

No one authorized to roll back or fail over, or authority buried three management levels up, so the incident stalls waiting for permission. Pre-authorize reversible high-value actions and make the path to irreversible-action authority explicit and reachable.

### Severity Never Re-Classified

A "minor" that grows into a full outage but stays at the original severity because no one revisited it, so the response never scales up. Re-classify as impact changes.

### Over-Escalation That Burns Out Responders

Escalating or paging too aggressively for low-impact events, training responders to ignore pages and eroding the signal needed for the next real incident. Calibrate escalation to severity and page only for actionable conditions.

### Silent Stakeholder Communication

Engineers focus on the fix and leave stakeholders and customers in the dark, so speculation fills the vacuum and trust erodes. Define a severity-dependent cadence and designate a communicator.

## Self-Check

- [ ] Severity is classified by user and business impact (users affected, severity of impact, revenue/trust at stake, whether impact is growing), not by responder familiarity, using objective pre-agreed criteria for each level.
- [ ] Severity is re-classified as the picture changes; a minor that grows into a full outage is escalated immediately rather than left at the original level, and when uncertain between two levels the response leans higher.
- [ ] Escalation triggers are objective (time-based "if not contained in N minutes" and impact-based severity jumps, scope expansion, confirmed data loss/security) and fire on a schedule, not on a responder's reluctance to ask for help.
- [ ] Escalation routes to roles backed by a rotation (on-call SRE, service owner, incident commander), not to named individuals who may be unavailable.
- [ ] Decision authority is explicit and pre-agreed: reversible high-value actions (rollback, failover) are pre-authorized for the on-call responder, the person who can authorize irreversible/costly actions is reachable through the path, and the incident commander role is separated from the hands-on fixer.
- [ ] Decision authority is separated from communication responsibility, with a designated communicator owning stakeholder/customer updates so neither role blocks the other.
- [ ] A communication protocol exists per severity with a defined cadence, designated communicator, and pre-drafted templates covering what is known, what is unknown, and what is being tried.
- [ ] On-call is sustainable: fair rotation with protected rest, pages limited to actionable symptoms, and explicit incident handoffs at shift changes.
- [ ] The highest-risk cases were verified — a symptom whose initial severity understated impact, an escalation that needed to fire on a time trigger rather than human judgment, an action that needed pre-authorized authority during the worst minutes, and a stakeholder update cadence that held during a long incident — not only the clean single-responder path.
