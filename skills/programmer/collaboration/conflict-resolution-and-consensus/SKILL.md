---
name: conflict_resolution_and_consensus.md
description: Use when the agent is resolving a technical disagreement between engineers, mediating an architecture debate, choosing between competing proposals, writing an RFC or ADR to drive a decision, persuading stakeholders, giving or receiving critical feedback, or distinguishing consensus from unanimity in a team decision.
---

# Conflict Resolution and Consensus

Technical conflict is normal and healthy in engineering; the problem is not that engineers disagree but that disagreements are often resolved badly. The two failure modes are opposites: either the loudest or most senior person wins by attrition (decisions made by stamina, not merit), or the team avoids conflict so thoroughly that no decision is made and the default wins by inertia (decisions made by whoever coded first). Both produce worse outcomes than a structured disagreement, and both erode trust over time. Agents often treat conflict as a social problem to be smoothed over rather than a decision problem to be worked, and in doing so miss that the goal is a good decision the team can commit to, not a compromise everyone resents.

The judgment problem is that consensus does not mean unanimity. A team that waits until everyone agrees ships nothing. The skill is distinguishing a decision that needs true consensus (irreversible, high-stakes, affects many people) from one that needs consent (reversible, bounded, can proceed unless there is a reasoned objection), running a fair process that surfaces the best arguments on all sides, documenting the decision so it is not relitigated, and separating the technical merit of an option from the social dynamics of who proposed it. The agent must also recognize when a disagreement is actually about values, risk tolerance, or priorities rather than facts, because those cannot be resolved by more evidence.

## Core Rules

### Diagnose what the disagreement is actually about

Most technical disputes are misframed. What looks like "microservices vs. monolith" may actually be about deployment autonomy, team boundaries, or risk tolerance. What looks like "Postgres vs. MongoDB" may be about consistency requirements or operational capacity. Before arguing options, establish the shared facts (what are we deciding, what constraints exist, what are the success criteria) and surface the underlying values or priorities driving each position. A debate about surface options without shared criteria becomes a stalemate because each side is optimizing a different goal. Make the real disagreement explicit before trying to resolve it.

### Match the decision process to the stakes and reversibility

Not every disagreement deserves the same process. Use the reversibility test:

- **Reversible, bounded decisions:** Use consent. Proceed unless there is a reasoned, specific objection. Do not require unanimity. The cost of waiting exceeds the cost of a wrong reversible choice.
- **Irreversible or high-stakes decisions (architecture, data model, public API, security):** Use consensus-seeking with a heavier process (RFC, ADR, multiple reviewers, explicit dissent). Invest in getting it right because undoing it is expensive.

A common failure is applying heavyweight consensus to reversible decisions (paralysis) or applying lightweight consent to irreversible ones (rushing a one-way door). Calibrate the process to the stakes.

### Use written proposals (RFC/ADR) to separate the argument from the arguer

Verbal debates favor the loudest, fastest-thinking, or most senior person in the room. Written proposals level the field: the argument must stand on its merits, introverts and remote participants can engage equally, and the reasoning is captured for the future. Use an RFC (Request for Comments) to propose and gather input, and an ADR (Architecture Decision Record) to record the final decision and its rationale, including alternatives considered and why they were rejected. The discipline of writing forces clarity that speaking does not, and the artifact prevents the decision from being relitigated later or forgotten.

### Seek consensus, but do not require unanimity

Consensus means the group converges on a decision everyone can live with and commit to, not that everyone's first choice wins. Distinguish:

- **Consent:** "I do not object" (no reasoned disagreement blocking progress).
- **Consensus:** "I can support this" (active agreement to commit).
- **Unanimity:** "This is my first choice" (rarely achievable, rarely necessary).

Require consensus or consent, not unanimity. A team that waits for unanimity cannot decide. When someone disagrees after a fair process, capture their dissent in the ADR and ask for commitment to the group's decision; they do not have to agree, but they should not block or undermine. "Disagree and commit" is the healthy norm.

### Separate the person from the position, and the position from the outcome

Attack arguments, not people. Seniority, volume, and persistence are not evidence. Make it safe to propose, to dissent, and to change one's mind; a culture where changing your mind is a loss means people defend bad positions to save face. Conversely, make it costly to block without reason; a veto without a specific, reasoned objection is not dissent, it is obstruction. The agent facilitating should explicitly invite dissenting views (silence is not consent), restate opposing positions charitably before responding, and watch for who is being talked over.

### Recognize value and risk-tolerance disagreements that evidence cannot resolve

Some disagreements are not about facts but about values or risk appetite: "move fast and accept breakage" vs. "stability first," "buy vs. build," "vendor lock-in is acceptable" vs. "must remain portable." More benchmarks will not resolve these because the parties weight outcomes differently. Name these as value disagreements explicitly, surface the tradeoff, and decide based on stated team or product priorities rather than pretending one more experiment will settle it. Escalate to the decision owner (tech lead, product owner) when values conflict and the team cannot converge.

### Close the loop and prevent relitigation

A decision that is not recorded and communicated will be re-argued endlessly. Every significant decision should produce an ADR that states the decision, the context, alternatives considered, and the consequences, and should be linked from the relevant code or docs. When the decision is revisited later, point to the ADR; reopening it requires new information or changed circumstances, not just a new objection. This is what lets a team make forward progress instead of circling.

## Common Traps

### The loudest or most senior person wins by attrition

In verbal debates, stamina and authority substitute for merit. Use written proposals and explicit decision criteria so the argument, not the arguer, carries weight.

### Conflict avoidance lets the default win by inertia

Teams that equate harmony with health avoid disagreeing, and the decision goes to whoever coded first or whatever the system defaulted to. Surface disagreements deliberately and decide explicitly.

### Treating consensus as unanimity and stalling

Waiting for everyone's first choice produces paralysis on reversible decisions. Require consent or consensus (can-live-with-and-commit), not unanimity.

### Relitigating decided issues because there is no record

Without an ADR, the same argument recurs every few months as people forget the rationale or new members re-raise it. Record decisions and require new information to reopen them.

### Misframing a values disagreement as a factual one

When parties weight risk or priorities differently, more evidence will not converge them. Recognize value conflicts, surface the tradeoff, and decide on priorities rather than endless analysis.

### Blocking without a reasoned, specific objection

A bare "I don't like it" or repeated veto without a concrete alternative is obstruction, not dissent. Require objectors to articulate a specific harm and a path forward.

### Smoothing over a conflict without resolving the underlying decision

"Let's just compromise" or splitting the difference often produces a worse option than either original. Resolve the decision on merit; compromise only when it genuinely serves the goal.

## Self-Check

- Before arguing options, did you establish shared facts and surface the underlying values or priorities driving each position, rather than debating surface options against different criteria?
- Did you calibrate the decision process to the stakes, using consent for reversible decisions and consensus-seeking with written artifacts for irreversible, high-stakes ones?
- Did you use a written proposal (RFC) to gather input and an ADR to record the decision, alternatives, and rationale, so the argument stands on merit and the decision is preserved?
- Did you seek consensus (can-live-with-and-commit) rather than unanimity, and did you capture reasoned dissent in the record rather than suppressing it?
- Did you separate people from positions, invite dissenting views, restate opposing arguments charitably, and make it safe to change one's mind?
- If the disagreement was about values or risk tolerance rather than facts, did you name it as such and decide on stated priorities instead of collecting more evidence?
- Is the decision recorded in an ADR linked from relevant code or docs, with a rule that reopening it requires new information or changed circumstances?
- Did you distinguish reasoned dissent (captured, then commitment requested) from obstruction (veto without a specific objection or alternative)?
