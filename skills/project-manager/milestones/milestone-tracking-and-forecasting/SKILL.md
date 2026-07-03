---
name: milestone_tracking_and_forecasting.md
description: Use when the agent is tracking milestone progress, forecasting milestone achievement dates, managing milestone slippage, deciding when to commit versus forecast a milestone, or determining whether a milestone is genuinely at risk before its date arrives.
---

# Milestone Tracking And Forecasting

A milestone set at kickoff and never re-examined is a wish frozen in time. Real projects move: prerequisites slip, dependencies shift, scope grows, and the conditions for achieving a milestone change week to week. Milestone tracking is the discipline of continuously assessing whether each upcoming milestone is on track, forecasting its likely achievement date with honest confidence, and surfacing slippage early enough to act. Agents tend to treat the original milestone date as the truth and report status against it, marking things green until the date arrives and the milestone is missed. That pattern converts milestones from early-warning tools into failure announcements.

The judgment problem is how to forecast milestone achievement from the real state of prerequisites and dependencies rather than from the original plan, how to distinguish a committed milestone from a forecast or at-risk one, how to manage slippage once it appears, and how to decide whether to recover, replan, or recommit. Tracking is not bookkeeping; it is continuous prediction.

## Core Rules

### Forecast From Prerequisites And Dependencies, Not From The Original Date

A milestone forecast should be built from the bottom up: are the prerequisite deliverables complete or on track, are the dependencies resolved or resolving, is the remaining critical path work achievable in the time left, and are the open decisions likely to close in time? A forecast that simply repeats the original date ignores whether the conditions for achievement are actually forming. Re-derive each upcoming milestone's likely date from the state of its inputs at every reporting cycle. The original date is a plan, not a prediction.

### Distinguish Committed, Forecast, And At-Risk Milestones

Not all milestone dates carry the same weight, and conflating them destroys trust. A committed milestone is one the project has obligated itself to deliver, often with external consequences if missed. A forecast milestone is the current best prediction of achievement, subject to change. An at-risk milestone is one whose prerequisites or dependencies make the committed date unlikely. Label each milestone with its status, and update the label as conditions change. When everything is reported as committed, slips feel like failures; when statuses are honest, the project manages expectations proactively.

### Surface Slippage Early Enough To Act

The value of milestone tracking is early warning. A milestone that will slip should be flagged as at-risk weeks before its date, while there is still time to recover, replan, negotiate scope, or reset expectations. Holding bad news until the milestone date converts a manageable problem into an announced failure and removes every recovery option. Establish a rule: the moment the critical path to a milestone can no longer fit in the remaining time at realistic velocity, the milestone is at-risk and must be escalated.

### Base Confidence On Evidence, Not Optimism

A milestone confidence level should reflect the state of prerequisites, not hope. High confidence requires completed prerequisites, resolved dependencies, no blocking decisions, and a clear path through remaining critical path work. Medium confidence means most prerequisites are on track but one or two carry risk. Low confidence means a prerequisite or dependency is unresolved or the remaining work cannot realistically fit. Resist the optimism that labels everything high confidence; inflated confidence is the precursor to every surprise slip.

### Manage Slippage As A Decision, Not A Confession

When a milestone slips, the response is a decision, not merely an announcement. The options are recover (compress the remaining work to hit the original date), replan (move the milestone and understand the cascade), reduce scope (hit the date with less), or escalate (raise the tradeoff to sponsors). Choose deliberately based on the cause of slippage and the consequence of the slip. Document why the milestone moved, what tradeoff was chosen, and the new forecast. A slip handled as a managed decision preserves trust; a slip handled as a confession erodes it.

### Track The Cascade When A Milestone Moves

Milestones are rarely independent. A slip in one often pushes downstream milestones, dependencies, external commitments, and resource plans. When a milestone moves, immediately assess the cascade: which subsequent milestones are affected, which external parties need to be notified, which contracts or commitments are impacted, and which resource allocations must change. Tracking a milestone in isolation hides the real cost of the slip. The cascade analysis is where the true impact becomes visible.

### Review And Update Milestone Status At A Regular Cadence

Milestone status is not set once at kickoff. Review the milestone register at a fixed cadence, update each forecast based on current prerequisite and dependency state, and record movement with reasons. A register that is created and never updated stops reflecting reality and stops being trusted. The cadence should be frequent enough that slippage is caught early but not so frequent that updating becomes busywork; for most projects, a weekly or biweekly cycle fits.

### Separate Milestone Tracking From Activity Percent Complete

Tracking percent complete on tasks tells you about effort consumed, not about milestone achievement. A milestone is achieved when its exit criteria are met, regardless of how many hours have been logged. Track milestones by the state of their prerequisites and exit criteria, not by aggregate percent complete across their constituent tasks. Percent complete can hide blocked or incomplete work behind a reassuring number; prerequisite and criteria status cannot.

## Common Traps

### Reporting Against The Original Date As Truth

Treating the plan date as the prediction ignores whether achievement conditions are forming. The trap is that status stays green until the date, then the milestone is missed with no warning. Forecast from current state.

### Conflating Committed And Forecast Milestones

Labeling everything as committed makes every slip feel like a failure and trains stakeholders to distrust the dates. The trap is brittle expectations and eroded credibility. Distinguish the statuses explicitly.

### Late Disclosure Of Slippage

Holding bad news until the milestone date removes every recovery option. The trap is that a manageable problem becomes an announced failure. Surface at-risk milestones early.

### Inflated Confidence Based On Hope

Labeling milestones high confidence when prerequisites are unresolved sets up surprise slips. The trap is that the optimism feels reasonable in the moment yet systematically misleads. Base confidence on evidence.

### Treating A Slip As A Confession Rather Than A Decision

Announcing a slip without presenting recovery or replan options surrenders control of the narrative. The trap is eroded trust and reactive stakeholder management. Frame slips as managed decisions.

### Ignoring The Cascade Of A Moved Milestone

Updating one milestone without assessing downstream impact hides the real cost. The trap is that subsequent slips appear later as separate surprises. Run the cascade analysis immediately.

### Stale Milestone Register

A register built at kickoff and never updated stops reflecting reality. The trap is that it becomes decoration no one trusts. Update at a regular cadence.

### Percent Complete Masking Blocked Work

Reporting aggregate percent complete hides blocked prerequisites behind a reassuring number. The trap is false comfort until the milestone is missed. Track prerequisite and exit-criteria state instead.

## Self-Check

- [ ] Is each upcoming milestone forecast from the current state of prerequisites, dependencies, and critical path, rather than restating the original plan date?
- [ ] Are milestones labeled as committed, forecast, or at-risk, with the labels updated as conditions change?
- [ ] Is slippage surfaced as at-risk weeks before the milestone date, while recovery options still exist?
- [ ] Is each milestone's confidence level based on completed prerequisites and resolved dependencies, not on optimism?
- [ ] When a milestone slips, is the response framed as a deliberate decision among recover, replan, reduce scope, or escalate?
- [ ] Is the cascade of a moved milestone assessed immediately, including downstream milestones, external commitments, and resource plans?
- [ ] Is the milestone register reviewed and updated at a regular cadence with reasons recorded for any movement?
- [ ] Is milestone status tracked by prerequisite and exit-criteria state rather than by aggregate task percent complete?
- [ ] Are external parties notified promptly when a milestone slip affects commitments to them?
- [ ] Could a reviewer tell, from the register alone, which milestones are genuinely on track versus merely not yet missed?
