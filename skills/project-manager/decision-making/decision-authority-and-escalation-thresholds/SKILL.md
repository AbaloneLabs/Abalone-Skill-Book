---
name: decision_authority_and_escalation_thresholds.md
description: Use when the agent is deciding who has authority for a project decision, setting escalation thresholds, defining decision rights through a RACI or RAPID model, routing a decision to the correct level, or designing the governance routing that prevents bottlenecks and unapproved commitments.
---

# Decision Authority and Escalation Thresholds

Most project dysfunction around decisions is not about making the wrong choice. It is about making the choice in the wrong place. Decisions get made too low, by someone without the authority or information to bear the consequences, or too high, where every choice queues behind an executive and the project stalls. Sometimes the same decision gets made three times by three different people because no one knew who actually owned it. Decision authority is the discipline of deciding who decides, and it is usually left implicit until the conflict or the delay forces it into the open.

The judgment problem is not "draw a RACI chart." It is how to match decision type to the right authority level, set thresholds that route decisions without manual negotiation, and avoid the two opposite failures: bottlenecks where nothing moves because everything escalates, and drift where commitments get made by people who lacked the standing to make them. Agents tend to treat authority as a political question rather than a design problem, and to set thresholds that are either so vague they require constant interpretation or so rigid they break on the first edge case.

## Core Rules

### Classify Decisions by Type, Reversibility, and Consequence

Authority routing starts with decision classification. A decision about a one-week internal task assignment is not the same kind of decision as one that commits the project to a multi-year vendor contract or a regulatory commitment. Classify each recurring decision type by its reversibility (can it be undone cheaply?), its blast radius (how much of the project does it affect?), and its strategic weight (does it shape future options?). The classification, not the topic, determines where authority should sit.

### Match Authority to Consequence, Not to Seniority or Availability

The person who should decide is the one best placed to bear the consequences and act on the information, which is often not the most senior person. Routine, reversible, well-bounded decisions belong close to the work. Irreversible, high-consequence, or strategically constraining decisions belong with whoever owns the outcome and can absorb the risk. Routing everything upward wastes executive attention; routing everything downward creates unaccountable commitments. Match the level to the consequence.

### Make Decision Rights Explicit Before the Decision Is Needed

The worst time to decide who decides is during the disagreement. Define decision rights during planning, when stakes are abstract and no one is defending a position. For each decision type, name the accountable owner, who must be consulted, who must be informed, and who has veto. Use a model like RACI for responsibility mapping or RAPID for decision roles, but treat the model as a communication device, not the deliverable. The deliverable is clarity about who can commit the project.

### Set Quantitative Escalation Thresholds, Not Vague Judgments

"Escalate if it is significant" is not a threshold; it guarantees inconsistency and negotiation. Define thresholds in measurable units: cost variance above a dollar amount, schedule slip beyond a number of days, scope changes crossing a baseline boundary, risk exposure above a scored level, or any decision touching a regulatory or contractual commitment. Quantitative thresholds route decisions automatically and remove the social friction of deciding whether something is "big enough" to escalate.

### Define the Escalation Path and the Decision SLA

A threshold without a path is half a rule. For each level, define who receives the escalation, what information they need to decide, and how long they have. An escalation that lands in an inbox with no context and no deadline becomes a bottleneck. Include the decision inputs required (impact analysis, options, recommendation) and a service-level expectation so escalations do not silently age. Track escalation cycle time as a governance health metric.

### Distinguish Approve, Consult, and Inform Deliberately

The most common authority confusion is between consultation and approval. Consulting someone means their input is sought but does not bind the decision; approval means the decision cannot proceed without their sign-off; informing means they learn after the fact. When these are conflated, people who expected a veto feel overruled, and people who expected to be consulted feel ignored. State the role explicitly for each stakeholder on each decision type.

### Preserve a Fast-Path for Urgent, Reversible Decisions

Not every decision can wait for the full routing. Build a defined fast-path for urgent decisions that are also reversible: a single named owner can decide immediately, with notification and a mandatory after-the-fact review. The fast-path exists so that urgency does not become an excuse to bypass governance entirely. The danger to watch is mission creep, where more and more decisions claim urgency to skip the normal path.

### Review and Re-Tune Authority Periodically

Authority structures decay. Thresholds set at project start become wrong as the project scales, sponsors change, or risk tolerance shifts. Bottlenecks migrate as people join or leave. Review decision routing periodically using real signals: where are decisions stalling, where are they being remade, where is someone repeatedly overruled. Re-tune thresholds and owners against evidence, not against the original org chart.

## Common Traps

### Routing Everything Upward to Be Safe

Escalating every decision to the sponsor or steering committee feels safe because no individual takes the risk, but it creates a bottleneck at the top and disempowers the team. The trap is confusing escalation with diligence. The result is slow decisions, executive fatigue, and a team that stops deciding anything on its own. Push reversible, bounded decisions down and reserve escalation for genuine consequence.

### Letting the Loudest or Most Available Person Decide by Default

When authority is unclear, the person who speaks first, loudest, or has the most calendar availability tends to decide, regardless of whether they own the outcome. The trap is that this produces decisions with no real backing, which unravel when the actual owner surfaces. Make authority explicit so default dynamics do not substitute for design.

### Thresholds So Vague They Require Constant Negotiation

"Material," "significant," and "as appropriate" are not thresholds; they are invitations to argue every time. The trap is that vague thresholds feel collaborative but actually concentrate power in whoever interprets them. Replace judgment words with numbers wherever possible, and reserve qualitative thresholds only for genuinely unquantifiable decisions like reputational or safety risk.

### Treating Consultation as a Veto

Stakeholders who are marked "consulted" come to believe their input must be followed, which silently converts consultation into approval and multiplies the effective number of approvers. The trap is that nothing gets decided because everyone consulted feels overruled. Clarify the distinction at the moment of consultation, and if a stakeholder genuinely needs veto power, grant approval authority explicitly rather than smuggling it in.

### Bypassing Governance Through the Urgency Exception

Once a fast-path exists for urgent decisions, more and more decisions get labeled urgent to avoid the normal process. The trap is that urgency becomes a permanent workaround. Track how often the fast-path is used and audit whether the decisions were genuinely urgent and reversible. If the fast-path is the dominant path, the normal path is too slow and needs redesign.

### Stale Authority That No Longer Matches the Project

The authority structure was right at kickoff, but the project changed scale, people rotated, and risk appetite shifted, yet the routing was never updated. The trap is governance by inertia. Decisions get routed to people who no longer have the context or the standing, and the real decisions happen informally around the official structure. Re-tune authority against current reality, not the original charter.

### Single Point of Failure in the Decision Owner

All authority for a class of decisions sits with one person who then becomes a bottleneck when unavailable, a risk when they leave, and a political problem when they disagree with the team. The trap is concentration for simplicity. Define a deputy or a backup owner for every critical decision type, and ensure the structure survives a single absence.

### Escalations That Arrive Without a Recommendation

A decision gets escalated as an open question with no analysis, forcing the higher level to do the work the lower level should have done. The trap is using escalation as a way to shed the hard part of the decision. Require that every escalation arrive with options, impact, and a recommendation, so the higher level decides rather than analyzes.

## Self-Check

- [ ] Are recurring decision types classified by reversibility, blast radius, and strategic weight?
- [ ] Does authority sit with whoever bears the consequence and holds the information, rather than defaulting to seniority?
- [ ] Were decision rights defined during planning, before any disagreement, using a clear responsibility model?
- [ ] Are escalation thresholds expressed in measurable units rather than vague judgment words?
- [ ] Is there a defined escalation path with required inputs and a decision turnaround expectation?
- [ ] Is each stakeholder's role on each decision type explicitly labeled as approve, consult, or inform?
- [ ] Is there a defined fast-path for urgent reversible decisions, with usage monitored for mission creep?
- [ ] Has the authority structure been reviewed and re-tuned against recent decision-flow evidence, not the original org chart?
- [ ] Does every critical decision type have a backup owner to avoid a single point of failure?
- [ ] Do escalations arrive with options, impact analysis, and a recommendation rather than as open questions?
