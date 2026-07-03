---
name: resource_allocation_and_capacity.md
description: Use when the agent is allocating limited team capacity across competing product initiatives, modeling throughput, sequencing work across multiple teams, deciding portfolio-level investment splits, or explaining why one team cannot take on more work.
---

# Resource Allocation And Capacity

Allocating capacity is not the same as ranking opportunities. A perfectly ordered priority list is useless if the teams who must do the work are already saturated, if the work lands on the wrong team, or if the critical path runs through a single constrained skill. Capacity allocation is where strategy meets the physical reality of how many people, with which skills, can produce how much, against how much change the system can absorb. Product managers fail here by treating capacity as infinite, by reading one team's velocity as a general truth, or by promising a portfolio that no realistic staffing can deliver.

Use this skill before answering broad questions such as "how should we split the team across these initiatives", "how do we model how much we can deliver", "how do we sequence work across dependent teams", "what should the portfolio investment mix be", or "why can't we just add this to the roadmap". The goal is to prevent the agent from producing an allocation that ignores the real constraint, ignores cross-team dependencies, or hides overcommitment behind optimistic throughput numbers.

## Core Rules

### Treat Capacity As The Real Constraint, Not Value

Value ranks opportunities; capacity decides what can actually move. The most valuable initiative is irrelevant if the team that must build it has no room, or if the scarce skill it needs is shared across four other efforts.

Identify the true constraint before allocating:

- Which teams or roles are involved: engineering, design, data, research, legal, security, ops, localization, content?
- Which specific skill is scarce: ML, mobile, payments, accessibility, platform, on-call experience?
- Is the constraint headcount, throughput, knowledge, or availability windows?
- Does the work require a specialist whose calendar is already fragmented?

A common error is to allocate against average team size while ignoring that one irreplaceable person sits on the critical path of half the portfolio. Name the bottleneck role explicitly, because that is where sequencing and deferral decisions actually bite.

### Model Throughput As A Range, Never A Point

Throughput is uncertain. Velocity fluctuates with on-call load, hiring ramp, vacation, incident recovery, rework, and the hidden tax of supporting shipped software. A single number becomes a false promise.

Model capacity with ranges and assumptions:

- What is the realistic sustained throughput, not the best sprint ever?
- How much capacity is already consumed by maintenance, support, on-call, bugs, and live-system stewardship?
- What fraction remains for new committed work after overhead?
- How variable is the team historically: tight or wide?
- What events will change the range: a launch, a migration, a hiring freeze, a reorg?

State the assumptions behind the number. "Forty story points per sprint assuming no major incident and one designer shared across two teams" is far more honest than "we can do forty points". When throughput is uncertain, the allocation should preserve slack rather than fill every slot.

### Map Cross-Team Dependencies And The Critical Path

Portfolio work rarely lives inside one team. Initiatives chain through platform teams, shared services, design systems, data pipelines, and release coordination. The critical path is the longest chain of dependent work, and it determines when value can actually ship, not when any single team finishes its slice.

Before committing a sequence, map:

- Which teams must touch this work and in what order?
- Where are the handoff points and integration points?
- Which dependencies are hard (API contract, data migration) versus soft (review, advice)?
- Which team is the serialized bottleneck that everything waits on?
- Can work be restructured to remove a dependency rather than schedule around it?

Sequencing decisions should be made against the critical path, not per team in isolation. Two teams each finishing early can still be blocked by a third team that is the single serialized constraint. The allocation must protect and de-risk that path first.

### Balance The Portfolio Across Investment Types

Capacity is not only for new customer-facing features. A healthy portfolio allocates across distinct investment types, and an allocation that spends everything on new value quietly starves the foundations the product runs on.

Balance across:

- new value: net-new capabilities and outcomes;
- retention and growth: improving what exists for current users;
- maintenance and reliability: bugs, stability, performance, observability;
- platform and tech debt: foundations that enable future work;
- compliance, security, and risk: obligations that cannot be deferred indefinitely;
- discovery and learning: reducing uncertainty before larger bets.

There is no universal split. A mature product under reliability pressure needs more maintenance; a new product needs more discovery. The mistake is allocating implicitly by letting the loudest features consume all capacity. Make the split explicit and revisit it when the product context shifts.

### Limit Work In Progress To Protect Throughput

Starting more work does not finish more work. High work-in-progress increases context-switching, handoff delay, integration risk, and rework, and it makes throughput estimates less reliable. Capacity allocation is partly a decision about how many things run at once.

Apply WIP thinking:

- How many initiatives is each team genuinely progressing in parallel?
- Does starting this now force two other efforts to stall?
- Would finishing and closing work before opening new work raise real throughput?
- Where does context-switching tax fall heaviest: deep technical work, design, research?
- Is parallelism real, or are people nominally on five things but actually blocked on all five?

Prefer fewer things done to completion over many things started. When leadership asks to add an initiative, the honest answer is often that something else must stop or finish first, not that the team will absorb it.

### Sequence Discovery And Delivery Deliberately

Discovery and delivery compete for the same scarce people but run on different rhythms. Starting all discovery in parallel starves delivery; starting all delivery in parallel starves the learning that prevents building the wrong thing.

Decide deliberately:

- Which bets need discovery before they deserve delivery capacity?
- Can discovery run ahead of delivery to feed a healthy pipeline without blocking it?
- How much capacity should stay reserved for exploratory work versus committed delivery?
- When does a discovery result convert into a committed delivery commitment?

A balanced portfolio keeps a discovery pipeline feeding delivery rather than treating discovery as a one-off activity. This also smooths throughput: teams always have a next validated thing to pick up.

### Make Hire, Build, And Defer Tradeoffs Explicit

When capacity is the constraint, the real levers are hiring, building with current staffing, deferring, or descaling. Allocation is incomplete if it only shuffles existing work.

For each constrained theme, weigh:

- Hire or contract: how long until net capacity gain, and at what onboarding cost?
- Build now: what is the opportunity cost and the risk of overload?
- Defer: what is the cost of delay, and does value decay?
- Descale: can a smaller version capture most value with less capacity?
- Partner or buy: does an external path remove an internal bottleneck?

These tradeoffs belong in the allocation conversation, not after the roadmap is already overcommitted. A roadmap that assumes no hiring, no deferral, and no descaling is usually fictional.

### Communicate Capacity Limits Honestly

Overcommitment is the most common capacity failure, and it usually starts with optimistic communication. The product manager must be able to say, credibly, that a team cannot take more work without dropping something.

Honest capacity communication includes:

- what the team can sustainably commit to, with stated assumptions;
- what is explicitly not happening because capacity is finite;
- which dependencies threaten the commitment;
- what would need to change to take on more: staffing, scope, or sequencing;
- where slack is intentionally held for uncertainty and support load.

Credibility comes from being right about limits over time. A team that consistently delivers what it commits, and that visibly defers when overloaded, earns the trust to make hard allocation calls.

## Common Traps

### Allocating Against Average Capacity

Using a smoothed average hides the weeks lost to incidents, on-call, and support. Allocations built on averages overcommit in the weeks that matter most, because real work arrives in bursts and capacity is uneven.

### Treating One Team's Velocity As Universal

Velocity is local and contextual. A platform team, a growth team, and a new-product team have different rhythms, risk profiles, and overhead. Copying one number across the portfolio produces allocations that some teams cannot meet and others find trivial.

### Ignoring The Serialized Bottleneck

The portfolio ships at the speed of its slowest shared dependency. Allocating generously to every team except the one everyone waits on changes nothing about real delivery, because all paths still converge on that constraint.

### Filling Every Slot And Calling It A Plan

An allocation with zero slack is not a plan; it is a guarantee of missed commitments. Unplanned work always arrives, and a portfolio with no buffer redistributes every surprise into delay.

### Counting Headcount Instead Of Throughput

Two more engineers do not equal two more teams' worth of delivery next quarter. Hiring ramp, onboarding, pairing, and knowledge transfer all delay the point at which new headcount becomes net capacity.

### Letting Features Consume All Capacity

When new features absorb the entire portfolio, maintenance, reliability, and platform work silently accumulate as debt until a reliability crisis forces a painful reallocation. The trap is that the cost is invisible until it is urgent.

### Starting Parallel Work To Show Progress

Opening many initiatives makes a roadmap look active but slows actual completion. Progress measured by starts rather than finishes hides the context-switching tax and the integration queue building downstream.

### Promising Dates That Ignore Dependencies

A date that assumes every dependent team is ready on time is fragile. The trap is committing to a portfolio date built from independent per-team estimates rather than from the real cross-team critical path.

## Self-Check

- [ ] The true constraint (team, role, or scarce skill) is named, not just the value ranking.
- [ ] Throughput is modeled as a range with stated assumptions, including maintenance and support overhead.
- [ ] Cross-team dependencies and the critical path are mapped, and the serialized bottleneck is identified.
- [ ] The portfolio is balanced across new value, retention, maintenance, platform, compliance, and discovery.
- [ ] Work-in-progress is limited deliberately, and adding work is paired with stopping or finishing something.
- [ ] Discovery and delivery are sequenced to maintain a learning pipeline without starving delivery.
- [ ] Hire, build, defer, descale, and buy tradeoffs are explicit for constrained themes.
- [ ] Capacity limits are communicated honestly, including what is explicitly not happening.
- [ ] Slack is preserved for unplanned work rather than filling every capacity slot.
- [ ] No commitment relies on a date that ignores cross-team dependencies or assumes perfect handoffs.
