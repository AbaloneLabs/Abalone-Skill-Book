---
name: milestone_design_and_decision_gates.md
description: Use when the agent is designing milestones as meaningful decision gates, defining entry and exit criteria, deciding what constitutes a meaningful milestone versus an arbitrary date marker, or structuring gate reviews that can actually stop or redirect a project.
---

# Milestone Design And Decision Gates

Most milestones in most project plans are not milestones at all; they are dates with labels. A real milestone is a decision gate: a point where the project must prove a meaningful state change has occurred and where a genuine go, adjust, or stop decision is made. The craft of milestone design is deciding which state changes deserve gate authority, what evidence proves they happened, and what decision the gate empowers. Agents tend to populate plans with calendar markers, "end of phase," "sprint 4 close," or "month two review," because those fill a timeline neatly. But a date that does not enable a decision is decoration, and a gate that always approves is not a gate.

The judgment problem is how to choose milestones that correspond to real transitions, how to write exit criteria that are objectively verifiable rather than negotiable, how to scope the decision authority of each gate, and how to avoid both milestone inflation (too many trivial gates) and milestone starvation (so few gates that problems accumulate invisibly). Good milestone design is a design discipline, not a scheduling afterthought.

## Core Rules

### Design Milestones Around State Changes, Not Calendar Rhythm

A milestone should mark a transition that changes what is true about the project: scope approved, feasibility proven, design locked, contract signed, integration complete, acceptance passed, launch readiness confirmed. Each milestone must answer the question, what is true when this is achieved that was not true before? If the only answer is "we reached a date," it is a deadline, not a milestone. Design from the state changes the project genuinely needs, then place them on the calendar; do not start from weekly or monthly cadence and bolt on labels.

### Write Exit Criteria That Are Objectively Verifiable

A gate is only as strong as its exit criteria. Criteria must be checkable by someone who did not do the work, using evidence rather than opinion. "Design is complete" is not a criterion; "design document reviewed and signed by architecture, security, and product, with all blocking comments resolved" is. Specify the artifacts, the approvers, the quality bars, and the acceptance tests. Vague criteria turn gate completion into a political negotiation, where the milestone is declared done because the date arrived. The discipline of writing objective criteria is what makes a gate real.

### Define Entry Criteria So Gates Are Not Reached Prematurely

Entry criteria state what must be true before work toward the milestone can even be considered for completion. A user acceptance gate might require feature complete, test environment ready, and test data loaded as entry criteria. Without entry criteria, teams arrive at gates with prerequisites missing, and the gate either rubber-stamps incomplete work or becomes a surprise blocker. Entry criteria make readiness visible in advance, so the gate reviews actual completion rather than discovering unreadiness on the day.

### Give Each Gate A Real Decision, Including The Option To Stop

A decision gate must empower a genuine choice: proceed, proceed with conditions, redirect, pause, or stop. If the only permitted outcome is "proceed," the gate is theater and wastes everyone's time. The hardest but most valuable gate authority is the option to stop or fundamentally redirect, because that is what protects the organization from sunk-cost continuation of a project whose business case has eroded. Make the decision options explicit before the gate, and ensure the gate owner has the authority to exercise them. A gate that cannot say no cannot protect the project.

### Assign A Single Accountable Gate Owner With Decision Authority

Each gate needs one accountable owner who holds the decision authority and is responsible for confirming exit criteria. This is not the person doing the work; it is the person accountable for the outcome and empowered to decide. Diffuse or committee ownership produces compromise approvals where no one owns the decision. If a steering committee is the gate owner, name the single executive who breaks ties and signs the decision. Accountability must be singular at the moment of decision.

### Calibrate Milestone Density To Risk And Uncertainty

Too many gates create review fatigue and slow the project without adding control; too few gates let problems compound. Calibrate density to where the risk and uncertainty are. Early, high-uncertainty phases deserve tighter gates because assumptions are being tested. Mature, execution-heavy phases can run with fewer, more spaced gates. Put gates where decisions are most valuable: after feasibility, after design lock, before expensive commitments, before irreversible steps like launch or migration. Density should follow risk, not a uniform template.

### Make The Consequence Of Missing A Gate Explicit In Advance

Before a gate, define what happens if exit criteria are not met: conditional proceed with a remediation plan, delay to next gate, scope reduction, or escalation. If the consequence is unstated, the default becomes a negotiated compromise that erodes the gate's authority. Stating consequences in advance depoliticizes the decision when criteria are missed, because the response was agreed before the pressure of the moment.

### Distinguish Decision Gates From Progress Checkpoints

Not every tracking point needs to be a decision gate. It is healthy to have lightweight progress checkpoints that inform without empowering stop decisions, alongside fewer authoritative gates. Conflating the two either trivializes gates (everything becomes a checkpoint) or over-weights checkpoints (every status update becomes a go/no-go). Be explicit about which milestones carry decision authority and which are informational.

## Common Traps

### Calendar Markers Labeled As Milestones

"End of Q2" or "week six review" with no state change fills a plan but tracks nothing. The trap is that the schedule looks structured while no real transitions are being governed. Design from state changes.

### Vague Exit Criteria That Become Negotiations

"Design complete" or "testing done" without artifacts, approvers, and quality bars turns completion into a political call. The trap is that the gate approves on schedule regardless of reality. Write verifiable criteria.

### Gates That Cannot Say No

A gate whose only real outcome is "proceed" provides no control. The trap is that the review consumes effort while protecting nothing, and sunk-cost continuation goes unchallenged. Empower the stop option.

### Premature Gate Arrival Without Entry Criteria

Teams reach a gate with prerequisites missing and the gate either rubber-stamps or surprises everyone. The trap is that unreadiness is discovered too late to act. Define entry criteria up front.

### Diffuse Or Committee Ownership

When a committee owns the gate with no tiebreaker, decisions drift to compromise approvals. The trap is that no one is accountable for the decision, so bad projects continue. Name a single decision owner.

### Milestone Inflation

Making every status point a gate creates review fatigue and slows delivery without adding control. The trap is that gates lose gravity and teams stop taking them seriously. Calibrate density to risk.

### Unstated Consequences For A Missed Gate

When criteria are missed and the response is improvised under pressure, the decision becomes political. The trap is erosion of gate authority over time. Agree consequences in advance.

### Conflating Checkpoints With Decision Gates

Treating every progress point as a go/no-go over-weights routine updates. The trap is decision fatigue and diluted authority for the gates that actually matter. Separate informational from authoritative.

## Self-Check

- [ ] Does each milestone represent a verifiable state change, answering what becomes true when it is achieved, rather than merely marking a date?
- [ ] Are exit criteria written as objectively checkable artifacts, approvers, and quality bars, not as vague completion statements?
- [ ] Are entry criteria defined so teams arrive at gates with prerequisites already met?
- [ ] Does each decision gate empower a real choice including the option to stop, pause, or redirect, not only to proceed?
- [ ] Is each gate owned by a single accountable decision-maker with the authority to exercise the choice, including a named tiebreaker on committees?
- [ ] Is milestone density calibrated to risk and uncertainty, denser in high-uncertainty phases and at irreversible commitments, rather than uniform?
- [ ] Are the consequences of missing each gate's exit criteria defined in advance, so responses are not improvised under pressure?
- [ ] Are authoritative decision gates distinguished from informational progress checkpoints, so gate gravity is preserved?
- [ ] Could an outside reviewer verify gate completion using only the documented criteria and evidence, without relying on opinion?
- [ ] Has the project avoided both milestone inflation (trivial gates everywhere) and milestone starvation (too few gates to catch compounding problems)?
