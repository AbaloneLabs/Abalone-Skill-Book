---
name: launch_rollback_and_incident_response.md
description: Use when the agent is preparing launch rollback plans and incident response, defining on-call and escalation for launch, planning communication during a launch incident, or ensuring a launch can be reversed safely if it fails.
---

# Launch Rollback And Incident Response

A launch is a moment of elevated risk, and the thing that makes a launch safe to attempt is the ability to survive its failure. Rollback planning and incident response preparation are how a team ensures that if the launch breaks, the break is contained, reversible, and communicated, rather than a public emergency discovered by customers at 2am. This is the work of being prepared to fail safely, done while the team is calm, because it cannot be improvised under pressure.

Agents miss this because launch preparation focuses on success. They confirm the feature works, the checklist is complete, and the announcement is scheduled, then treat failure response as something that will be handled if it happens. The harm is that when something does break, the team cannot roll back cleanly because the path was never tested, the incident has no defined owner or escalation, and the communication is improvised, turning a technical problem into a trust problem on top of it. The opposite failure is over-planning, building elaborate incident runbooks for a launch whose risk does not warrant them and burying the team in process.

Use this skill before answering broad questions such as "what is our rollback plan", "who is on call for the launch", "how do we communicate if something goes wrong", "what is our incident response", or "how do we make sure we can reverse this". The goal is to prevent the agent from treating rollback and incident response as afterthoughts and to make reversibility, ownership, and communication explicit before exposure.

## Core Rules

### Make The Launch Reversible Or Explicitly Accept That It Is Not

Every launch should be reversible in a defined way, or the team should have explicitly accepted that it is not, documented why, and adjusted the rollout conservatism accordingly. Reversibility is the foundation of safe launch, because it is what allows the team to attempt the launch without betting everything on success.

Confirm how the feature is disabled, whether by flag, config, or deploy, and whether rollback preserves data integrity. For changes that touch data, schema, or external integrations, rollback may be partial or impossible, and that constraint must drive a more cautious rollout. A launch whose rollback is uncertain is a launch that cannot afford to fail, which means it cannot afford to be aggressive.

### Test The Rollback Path Before Relying On It

Rollback that has never been exercised is a hope, not a plan. Many teams assume they can revert, then discover under pressure that a migration, a flag dependency, or a data state makes rollback painful or impossible. The time to find that out is before the launch, not during an incident.

Run the rollback in a staging or pre-production environment, and confirm it works end to end, covers all entry points, and leaves the system in a consistent state. For high-stakes launches, rehearse rollback as part of the dry run. A rollback path you have watched succeed is a tool; one you have only assumed works is a risk.

### Define On-Call And Escalation For The Launch Window

A launch window has elevated risk and needs elevated coverage. The team must know who is on call during and after the launch, who has authority to roll back, and how to escalate if the on-call person cannot resolve the issue. Without defined coverage, an overnight failure waits until morning, and a small problem becomes a large one.

Name the on-call owner for the launch window, confirm they have the access and authority to act, and define the escalation path if they need help. For major launches, consider a dedicated launch on-call distinct from the regular rotation, so the launch gets focused attention and the regular rotation is not disrupted. Coverage that depends on someone happening to be available is not coverage.

### Set Severity Thresholds That Trigger Action

The team must decide in advance what level of problem triggers investigation, what triggers rollback, and what is acceptable to monitor. Without defined thresholds, the response to an incident is improvised: the team debates whether it is bad enough while the problem grows.

Define severity levels and the response each demands. A clear, well-understood failure signature may trigger automatic rollback. An ambiguous regression may trigger a pause and human review. A minor cosmetic issue may be monitored and fixed forward. Write the thresholds down before the launch, so the decision under pressure is execution, not debate.

### Prepare Incident Communication In Advance

Communication during a launch incident cannot be improvised without making the incident worse. Customers, support, sales, and leadership all need information, and ad hoc messaging produces contradictions, overclaims, and a sense that the team has lost control. Pre-drafted communication templates let the team respond fast and consistently.

Prepare templates for the likely scenarios: a partial rollback, a full rollback, a known issue being investigated, and a resolution. Each template should name the audience, the channel, the tone, and the information that must be confirmed before sending. Decide who is authorized to communicate externally and internally, so the message is consistent rather than contradictory.

### Run The Incident Response Before The Incident

For major launches, a rehearsal of the incident response is as valuable as a rehearsal of the happy path. It surfaces the gaps that only appear under failure: the runbook that assumes access nobody has, the escalation that routes to someone who is offline, the dashboard that does not show the failing metric, the communication template that does not fit the actual scenario.

Rehearse a plausible failure: detect it, decide to roll back, execute the rollback, and communicate. The gaps found in rehearsal are cheap to fix; the gaps found during a real incident are expensive and public. A team that has practiced failing together fails gracefully; one that has not fails chaotically.

### Plan For The Post-Launch Window, Not Just Launch Day

Risk does not return to normal the moment the launch goes live. The hours and days after exposure are when delayed effects surface, when usage patterns reveal defects the canary missed, and when support volume reveals confusion the team did not anticipate. Treating launch day as the end of the risk window is a common mistake.

Sustain elevated monitoring, on-call awareness, and communication readiness through the post-launch window, defined by how long delayed effects take to appear. For some launches that is hours; for others, days or weeks, if the change affects a weekly or monthly cycle. Stand down deliberately when the risk has actually subsided, not when the launch feels done.

## Common Traps

### Rollback Assumed But Never Tested

A rollback path that exists in theory but has never been exercised fails under pressure, revealing dependencies or data states that make it slow, partial, or impossible exactly when speed matters most.

### No Defined On-Call For The Launch

Relying on the regular rotation or on someone happening to be available leaves a launch uncovered at the moment of highest risk, and an overnight failure waits for morning.

### Improvised Incident Communication

Messaging invented during an incident contradicts itself across audiences, overclaims, and signals that the team has lost control, turning a technical problem into a trust problem.

### No Severity Thresholds

Without pre-defined triggers, the team debates whether a problem is bad enough to act on while the problem grows, and the decision becomes social rather than operational.

### Rehearsing Only The Happy Path

Dry runs that cover only the successful launch miss the gaps that appear only under failure, which is precisely when the team cannot afford to discover them.

### Standing Down Too Early

Treating launch day as the end of the risk window ignores the delayed effects, weekly cycles, and support patterns that surface in the hours and days after exposure.

### Irreversible Change Treated Like A Reversible One

A migration or schema change rolled out with the same aggression as a flagged feature, because the team assumed rollback was available when it was not.

## Self-Check

- [ ] The launch is reversible in a defined way, or the team has explicitly accepted it is not and adjusted the rollout conservatism accordingly.
- [ ] The rollback path has been tested end to end, including data integrity and coverage of all entry points, not just assumed to work.
- [ ] A named on-call owner with access and authority is assigned for the launch window, with a defined escalation path.
- [ ] Severity thresholds are defined in advance, specifying what triggers investigation, rollback, or continued monitoring.
- [ ] Incident communication templates are prepared for likely scenarios, naming audience, channel, tone, and required confirmations.
- [ ] Who is authorized to communicate internally and externally is decided before the launch, so messaging stays consistent.
- [ ] For major launches, the incident response has been rehearsed, not just the happy path.
- [ ] Elevated monitoring, on-call awareness, and communication readiness are sustained through the post-launch window until delayed effects have had time to surface.
- [ ] Irreversible changes such as migrations are treated with more conservatism than reversible flagged features.
- [ ] The team stands down from launch posture deliberately when risk has actually subsided, not when launch day ends.
