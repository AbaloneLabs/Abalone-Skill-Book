---
name: dependency_management_and_blocking.md
description: Use when the agent is managing dependencies across teams, tracking and unblocking cross-team work, sequencing dependent deliverables, or escalating blockers without damaging relationships.
---

# Dependency Management And Blocking

Dependencies are the hidden load-bearing structure of any cross-team roadmap. A feature that looks on track within one team can stall for weeks waiting on an API from a partner team whose roadmap never accounted for it. The failure mode is treating dependencies as line items in a plan rather than as commitments held by other people with their own priorities, and then discovering the gap late, when the cost of re-sequencing is highest. Dependency management is not making a list of things you need; it is building the relationships, contracts, and visibility that make those needs actually get met on time.

Use this skill before committing to a date that depends on other teams, when sequencing work that has upstream or downstream coupling, and when a blocker threatens a committed deliverable. The goal is to prevent the agent from treating dependencies as someone else's problem, from escalating too late or too aggressively, and from confusing a documented dependency with a secured one. A dependency written in a spreadsheet is a hope until the owning team has agreed to it in their own plan.

The core judgment is deciding how much buffer, ceremony, and escalation rigor each dependency deserves, and recognizing that a dependency unblocked a week early is cheap while a dependency discovered a week late is expensive.

## Core Rules

### Distinguish A Real Dependency From A Preference

Not everything you want from another team is a dependency. A true dependency means your work cannot proceed, or cannot ship, without it; a preference means the work would be better with it but can move without it. Conflating the two erodes credibility, because partner teams stop treating your dependency flags as urgent when most of them turn out to be optional. For each item, ask whether you can ship the user outcome without it, whether there is a workaround, and whether the cost of proceeding without it is acceptable. Reserve the word "dependency" for what genuinely blocks progress.

### Secure The Commitment In The Owning Team's Plan

A dependency is not secured until the team that owns the work has accepted it into their own roadmap and assigned it capacity. Your Jira ticket pointing at their backlog is not a commitment; it is a request. The confirmation that matters is the partner team's lead acknowledging the date, the scope, and the priority in their own planning forum. Get that acknowledgment in writing, reference their plan, and confirm the owner who is accountable on their side. Without this, the dependency is a hope resting on someone else's unawareness.

### Sequence Work To Expose Dependencies Early

The most expensive dependency is the one discovered late. Sequence the work so that the riskiest, least certain cross-team dependencies surface early, often through a spike, a proof of integration, or a contract negotiation before the full build. If your feature depends on a partner API, get a stub or a contract in place before your team builds six weeks of work against an assumption. Front-load the integration points and the unknowns so that a missed dependency reorders the plan rather than killing the launch. Sequencing is risk management: do the parts that could fail first.

### Track Dependencies With Explicit Owners And Dates

A dependency without a named owner and a date on both sides is an open loop. For each dependency, record who needs it, who provides it, what exactly is being delivered, the date it is needed by, and the date it is promised for. Track the gap between needed-by and promised-for explicitly, because that gap is your schedule risk. Review the dependency list at a regular cadence and treat any slip on the providing side as a schedule event for your side, not their problem. Visibility is what turns a hidden risk into a managed one.

### Unblock Before You Escalate

Most blockers can be resolved at the working level if the product manager invests in the relationship and the context. Before escalating, attempt to unblock directly: clarify the requirement, reduce the scope of the ask, offer to share capacity, or remove a constraint on the partner's side. Escalation should be the path you take after genuine effort, not the first call, because escalating without trying first damages the relationship and signals that you offload hard problems upward. That said, do not let the desire to be collaborative become an excuse to delay escalation past the point where leadership could still help.

### Escalate With Precision And Respect

When a blocker genuinely requires leadership intervention, escalate as a clear decision request, not a complaint. State what is blocked, what you have already tried, what you need from leadership, by when, and the consequence of inaction. Name the partner team factually without blame, and frame the ask as a sequencing or priority decision that only a shared leader can make. A precise, blame-free escalation gets resolved; a venting escalation gets ignored and burns the relationship with the partner team. The goal of escalation is to move the work, not to assign fault.

### Build Reciprocity For Future Dependencies

Every dependency you call in draws on a balance of reciprocity with the partner team. When you ask for an exception, a rush, or a reprioritization, you are spending credit that you build by helping them when they need it. Invest in the relationship outside of moments of need, make your asks visible to leadership so the cost is recognized, and look for opportunities to absorb cost on your side before pushing it onto partners. A team known for fair, reciprocal dependency handling gets its urgent requests met; a team known for dumping dependencies and hoarding credit gets slow responses when it matters.

## Common Traps

### Documenting A Dependency And Calling It Secured

Writing a dependency in a plan and linking to another team's backlog feels like progress but commits nothing. Until the owning team has accepted it into their roadmap with capacity, it is an unconfirmed request that can slip without notice. Secure the commitment in their plan, not just yours.

### Discovering The Dependency Late

Building six weeks of work against an assumed integration, only to find the partner team never prioritized it, turns a manageable risk into a launch-killing surprise. Front-load the riskiest dependencies through contracts, stubs, and spikes so they fail early and cheaply.

### Escalating Too Late

Waiting until the dependency has already slipped the date to escalate removes the option for leadership to help, because the time to re-sequence has passed. Escalate when the risk becomes probable, not when the miss is certain, while there is still room to act.

### Escalating Too Aggressively Or With Blame

Jumping to leadership with a complaint about a partner team before attempting to unblock directly damages the relationship and trains partners to route your work through formal channels. Escalate as a precise, blame-free decision request only after genuine effort to resolve at the working level.

### Treating Every Ask As A Dependency

Flagging preferences and nice-to-haves as blocking dependencies trains partner teams to deprioritize your dependency flags, because most turn out to be optional. Reserve the dependency label for what genuinely blocks progress, and your real dependencies will be taken seriously.

### Ignoring The Needed-By Versus Promised-For Gap

Tracking only the date a dependency is promised, without comparing it to the date you actually need it, hides schedule risk until the slip lands. Make the gap explicit and treat any widening gap as an active schedule event requiring mitigation.

### Hoarding Credit And Dumping Costs

Asking partners for exceptions while never absorbing cost on your side, and never making your asks visible to leadership, depletes the reciprocity balance. Teams that operate this way find their urgent requests met with slow, formal responses when they most need speed.

## Self-Check

- [ ] Each flagged dependency is a genuine blocker, not a preference or nice-to-have conflated with a hard dependency.
- [ ] The dependency is secured in the owning team's own roadmap with acknowledged capacity, not merely listed in your plan.
- [ ] Risky, uncertain dependencies were sequenced early through contracts, stubs, or spikes to surface failure before full build.
- [ ] Each dependency has a named owner and date on both the needing and providing sides, with the needed-by versus promised-for gap tracked.
- [ ] Direct unblocking was attempted, with scope reduction or shared capacity, before escalating to leadership.
- [ ] Any escalation was framed as a precise, blame-free decision request with the consequence of inaction stated, not a complaint.
- [ ] The relationship with the partner team was respected in escalation, naming facts without assigning fault.
- [ ] Reciprocity was maintained by investing in the relationship outside moments of need and making asks visible to leadership.
- [ ] The dependency list is reviewed at a regular cadence, and slips on the providing side are treated as schedule events on your side.
- [ ] Buffer and ceremony were calibrated to the stakes, with heavier tracking reserved for high-stakes, hard-to-reverse dependencies.
