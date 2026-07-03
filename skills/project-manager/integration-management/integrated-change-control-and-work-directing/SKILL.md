---
name: integrated_change_control_and_work_directing.md
description: Use when the agent is directing and managing project work, applying integrated change control across knowledge areas, coordinating cross-functional work execution, managing deliverable handoffs, or ensuring approved changes propagate consistently through scope schedule cost quality and risk baselines.
---

# Integrated Change Control and Work Directing

Directing project work and controlling change across an integrated baseline are two halves of the same responsibility. Work directing is the act of translating the plan into coordinated action; integrated change control is the act of ensuring that when the plan changes, every affected component changes consistently. The failure mode is fragmentation: work proceeds against a stale plan in one area while another area has moved on, and changes are applied to the schedule but not the budget, or to the scope but not the quality criteria.

The judgment problem is how to direct execution so that cross-functional work stays coordinated, and how to apply change control so that a change in one knowledge area propagates to all dependent areas without gaps. Agents tend to direct work task-by-task without integration awareness, and to treat change control as a scope-only activity that touches only the schedule.

## Core Rules

### Direct Work Against the Integrated Baseline, Not Task Lists

When directing execution, the reference is the integrated baseline that combines scope, schedule, cost, and resource commitments. Assigning work from isolated task lists produces locally efficient but globally uncoordinated effort. Each work assignment should be traceable to the baseline, so that progress can be rolled up consistently and variances are meaningful.

### Coordinate Cross-Functional Handoffs Explicitly

Most integration failures occur at handoffs: requirements to design, design to build, build to test, one team to another. Define each handoff: what is delivered, the acceptance criteria, the receiving party, and the date. Unmanaged handoffs produce gaps where work falls between teams and defects are discovered late. Make handoffs first-class managed events.

### Apply Integrated Change Control Across All Affected Knowledge Areas

When a change is approved, it must propagate to every affected baseline component: scope, schedule, cost, resources, quality, risk, and contracts. A change that updates the schedule but not the budget creates an unfunded commitment; one that updates scope but not quality criteria produces untested deliverables. Maintain a propagation checklist keyed to the change type, and verify each affected component is updated before the change is closed.

### Maintain a Single Change Authority for Integrated Decisions

Changes that cross knowledge areas require a single integrated decision point, not separate approvals from each area owner. Fragmented approval produces inconsistent decisions: scope approves, budget rejects, and the project is left in limbo. Route cross-area changes to an authority that can decide on the integrated impact, and record the decision against all affected baselines.

### Reconcile Work Execution With the Plan Continuously

Execution drifts from the plan through small daily decisions. Continuous reconciliation, comparing actual work and spend against the baseline, detects drift early. Reconciliation is not just variance reporting; it is the act of deciding whether drift is acceptable, requires a change, or signals a planning defect. Reconcile at a cadence matched to project velocity.

### Manage Approved Changes Into Execution Promptly

A change approved but not reflected in how work is directed creates a gap between the official baseline and actual execution. The team works the old plan while the new plan exists only on paper. Update work assignments, schedules, and budgets promptly after approval, and confirm the team is working to the updated baseline.

### Preserve Configuration Integrity During Execution

As work is directed and changes applied, the configuration, the integrated set of requirements, design, code, documentation, and tests, must stay consistent. A change to one configuration item must propagate to dependent items. Without configuration integrity, the project delivers components that do not fit together. Tie configuration updates to the change control process.

### Use Phase-Gate Reviews as Integration Checkpoints

At each phase gate, verify that the integrated baseline is consistent, that deliverables meet the phase exit criteria, and that the next phase's plan is built on the current integrated state. Phase gates are not just progress checkpoints; they are integration verification points. A phase gate that does not verify integration lets inconsistencies propagate into the next phase.

## Common Traps

### Directing From Isolated Task Lists

Work is assigned team-by-team without reference to the integrated baseline, producing effort that does not roll up coherently. The trap is local efficiency at the cost of global coordination. Direct against the integrated plan.

### Unmanaged Handoffs

Work moves from one team to another with no defined handoff, and defects or gaps appear at the seam. The trap is assuming handoffs happen automatically. Define and manage them explicitly.

### Change Propagating to Only One Baseline Component

A change updates the schedule but the budget, quality criteria, and risk register are untouched. The trap is that the most visible artifact gets updated and the rest is forgotten. Use a propagation checklist.

### Fragmented Approval Across Areas

Scope, budget, and quality owners each approve their piece independently, producing contradictory decisions. The trap is that no one owns the integrated decision. Route cross-area changes to a single integrated authority.

### Execution Drifting Without Reconciliation

Small daily decisions accumulate into significant drift from the baseline, unnoticed because no one reconciles. The trap is treating variance reporting as paperwork rather than as a decision tool. Reconcile continuously and act on drift.

### Approved Changes Not Reflected in Directed Work

The change is approved and documented but the team keeps working the old plan. The trap is the gap between official baseline and actual execution. Update work assignments immediately after approval.

### Configuration Items Drifting Out of Sync

A change updates the code but not the requirements or tests, so the configuration becomes internally inconsistent. The trap is treating configuration as each team's private concern. Tie configuration updates to change control.

## Self-Check

- [ ] Is project work directed against the integrated baseline rather than against isolated task lists?
- [ ] Are cross-functional handoffs defined with deliverables, acceptance criteria, receiving party, and date?
- [ ] Does every approved change propagate to all affected baseline components via a propagation checklist?
- [ ] Are cross-knowledge-area changes routed to a single integrated change authority rather than fragmented approvals?
- [ ] Is execution reconciled against the baseline at a cadence matched to project velocity, with drift acted upon?
- [ ] Are approved changes reflected promptly in directed work assignments, schedules, and budgets?
- [ ] Does configuration integrity, across requirements, design, code, documentation, and tests, stay consistent through changes?
- [ ] Do phase-gate reviews verify integrated baseline consistency and phase exit criteria, not just progress?
- [ ] Can you trace any directed work assignment back to the integrated baseline and forward to a project objective?
- [ ] Would a reviewer find the scope, schedule, cost, quality, and risk baselines mutually consistent after the latest changes?
