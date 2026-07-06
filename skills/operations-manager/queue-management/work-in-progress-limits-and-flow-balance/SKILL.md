---
name: work-in-progress-limits-and-flow-balance.md
description: Use when the agent is setting WIP limits, balancing queue flow, reducing multitasking, managing bottlenecks, reviewing blocked work, deciding pull versus push allocation, controlling expedited lanes, or improving operational throughput without hiding quality, service, or workload risk.
---

# Work In Progress Limits And Flow Balance

Work in progress is the amount of work currently started but not finished. Too much WIP makes operations look busy while slowing completion, increasing context switching, hiding blockers, and delaying feedback. Too little WIP can starve downstream teams or waste capacity. Agents often recommend "process more" or "prioritize harder" when the real problem is uncontrolled started work. This skill helps the agent use WIP limits and flow balance to improve completion, not just activity.

## Core Rules

### Define What Counts As WIP

Decide when work becomes in progress and when it exits. In some operations, WIP begins when an item is assigned; in others, when a worker opens it, requests evidence, starts review, picks inventory, contacts a vendor, or commits a customer timeline. Define the status boundaries precisely.

Include hidden WIP: items waiting for customer response, vendor confirmation, supervisor approval, quality review, system retry, missing evidence, or worker follow-up. If "waiting" items are excluded, the queue may look healthy while commitments accumulate.

### Set Limits Where Congestion Happens

WIP limits should protect the constraint, not decorate every status. Identify bottlenecks by cycle time, queue age, rework, blocked items, specialist capacity, supervisor approvals, system limits, or downstream handoffs. Set limits where too much started work creates delay or quality risk.

Limits can apply to individuals, teams, work types, priority classes, vendors, sites, review stages, or escalation lanes. The right limit depends on the flow problem being controlled.

### Prefer Finishing Over Starting

When WIP is high, the operating discipline should favor completing, unblocking, or closing work before starting new items. This may require pulling from near-finished states, assigning help to blockers, pausing intake, batching approvals, or reallocating specialist time.

Starting more work can feel productive because more items show activity. It often increases cycle time and creates more status management. The agent should ask what needs to finish, not only what needs to begin.

### Make Blockers Visible

Blocked work needs a reason, owner, next action, and review date. Common blockers include missing evidence, customer nonresponse, vendor delay, system issue, policy ambiguity, approval wait, dependency on another team, quality defect, or capacity shortage. A blocked label without accountability is only a parking place.

Blocked items should not count the same as active work for every purpose, but they should remain visible in WIP and risk review. If too much work is blocked, the system needs upstream fixes, clearer intake, escalation, or revised service promises.

### Control Expedite Lanes

Expedite work can protect urgent outcomes, but it disrupts flow. Define criteria for expedited handling, who approves it, what work it can interrupt, how long it remains expedited, and how the impact is reviewed. Expedite lanes should be small and visible.

If the expedite lane is always full, it is no longer an exception. It indicates demand, capacity, priority policy, or service promise failure. Expanding expedite without fixing the cause will harm routine work and staff focus.

### Balance Specialization And Flexibility

Specialists can process complex work well, but narrow specialization can create bottlenecks. Generalists provide flexibility, but routing complex work to them can create defects. Use WIP data to decide where cross-training, tiered review, pair work, standard work, or automation would improve flow.

Do not assume equal WIP limits for every role. A complex compliance review and a simple address correction do not carry the same cognitive load or risk.

### Align Push And Pull Rules

Push systems assign work to people, while pull systems let workers take the next eligible item. Push may help with accountability and specialized routing. Pull may reduce idle time and improve flow when work is standardized. Many operations need a hybrid.

Define who can pull what, when assignment is required, and how priority is preserved. An uncontrolled pull system can become cherry-picking. An overly rigid push system can overload assigned workers while others wait.

### Use Flow Metrics Carefully

Measure cycle time, lead time, throughput, WIP age, blocked age, rework, SLA attainment, queue abandonment, and handoff delay. Segment by work type and priority so simple work does not hide complex work. Pair metrics with quality and customer or requester impact.

Do not celebrate lower WIP if it was achieved by rejecting work, hiding waiting states, closing items prematurely, or pushing burden downstream. Flow improvement should mean better completion, not cleaner dashboards.

### Change Limits With Operational Review

WIP limits are not permanent. Review them after demand shifts, staffing changes, tooling changes, product launches, policy updates, vendor changes, and peak seasons. Document why a limit changes and what signal will show whether it worked.

When changing WIP limits, watch for second-order effects: upstream backlog, downstream starvation, longer wait for low-priority work, more escalations, or quality drift.

## Common Traps

- Defining WIP only as actively touched items and excluding waiting, blocked, assigned, or committed work.
- Setting WIP limits everywhere without identifying the actual bottleneck or risk being controlled.
- Starting more items to show progress while near-finished or blocked items remain unresolved.
- Using a blocked status as a storage area without owner, next action, reason, or review date.
- Letting expedite lanes grow until routine work has no stable flow.
- Applying the same WIP limit to work with very different complexity, risk, or cognitive load.
- Allowing pull systems to become cherry-picking of easy work; using push assignment so rigidly that some staff are overloaded while others are idle
- Improving a flow metric by hiding work, closing prematurely, or moving burden to another team; failing to revisit WIP limits after demand, staffing, tool, or service-promise changes

## Self-Check

- Is the definition of WIP clear, including start, exit, waiting, blocked, assigned, and committed states?
- Are WIP limits placed where congestion, delay, quality risk, or bottleneck capacity actually occurs?
- Does the operating rule prioritize finishing and unblocking over starting more work when WIP is high?
- Does every blocked item have a reason, owner, next action, and review date?
- Are expedited items controlled by criteria, approval, duration, and impact review?
- Do WIP rules account for differences in complexity, risk, skill, and cognitive load?
- Are push and pull assignment rules clear enough to prevent overload and cherry-picking?
- Are flow metrics segmented and paired with quality, rework, SLA, and stakeholder impact?
- Has the plan avoided improving dashboards by hiding or prematurely closing work?
- Are WIP limits reviewed when demand, staffing, tools, vendors, or service promises change?
