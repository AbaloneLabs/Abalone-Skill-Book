---
name: record_to_report_process_design.md
description: Use when the agent is designing or improving the record-to-report process, mapping the flow from transaction recording through general ledger, reconciliation, close, and financial reporting, or building a controlled and efficient R2R process for a finance function.
---

# Record To Report Process Design

Record-to-report (R2R) is the end-to-end flow from recording a transaction to producing the financial statements and management reports. It encompasses journal entries, subledger posting, reconciliation, close, consolidation, and reporting. When R2R is well designed, the flow is controlled, efficient, and auditable, and the financial statements emerge reliably each period. When it is poorly designed, the close becomes a manual reconstruction, reconciliations are late or absent, and reporting is produced through heroic effort that cannot be sustained or trusted. Designing R2R is not about documenting what people already do; it is about engineering a process that produces reliable financial information with appropriate control and reasonable effort.

Use this skill before designing or reengineering the record-to-report process, mapping the flow from recording to reporting, building close and reconciliation procedures, or assessing whether the R2R process is controlled and efficient. The goal is to prevent the agent from designing an R2R process that looks complete on paper but lacks reconciliation, control, or sustainability.

## Core Rules

### Map The Process End To End, From Source To Statement

R2R design begins with understanding the full flow. Map it from the origin of each transaction type to its appearance in the financial statements.

Map for each major transaction type:

- the source event and the system where it originates;
- the recording step and the controls at that point;
- the posting path to the subledger and then the GL;
- the reconciliation point where the subledger ties to the GL;
- the close steps that finalize the period;
- the reporting step where the transaction appears in statements.

A map that stops at the GL misses reconciliation and reporting. A map that starts at the journal entry misses the source controls. Map the whole flow.

### Build Reconciliation Into The Process, Not Around It

Reconciliation is not an add-on performed under time pressure at close. It is a control that should be embedded in the process with defined owners and timing.

Embed reconciliation by:

- assigning each material account a reconciliation owner;
- defining the frequency, monthly for most, more often for high-risk accounts;
- specifying the expected evidence and review;
- building the reconciliation into the close calendar with adequate time;
- escalating aged or unresolved items.

A process where reconciliation is optional, late, or unowned produces accounts that drift and a close that produces adjustments rather than confirmed balances.

### Design The Close As A Controlled Sequence

The close is the heart of R2R. It should be designed as a sequence of dependent tasks with gates, not a scramble of parallel activity.

Design the close so that:

- subledgers close before GL reconciliation;
- bank and intercompany reconcile before consolidation;
- accruals and estimates post before pre-close review;
- review and adjusting entries precede statement preparation;
- approval and period lock follow final review.

Sequence tasks that have dependencies. Allow parallelism only where tasks are genuinely independent. Enforce the sequence through the close calendar and workflow.

### Separate Duties And Build Approval Gates

R2R must include segregation of duties and approval gates that prevent a single person from initiating, recording, and approving a transaction without independent check.

Build in:

- separation between transaction initiation, recording, and approval;
- approval authority limits for entries and payments;
- independent review of estimates, accruals, and non-routine entries;
- review of reconciliations by someone other than the preparer;
- escalation for exceptions and overrides.

A process without segregation concentrates the ability to misstate or misappropriate within one role. Segregation is a primary control, not a formality.

### Define Roles, Owners, And Handoffs

R2R spans multiple people and often multiple teams. Ambiguity about who does what produces gaps and duplication.

Define for each task:

- the role responsible, not just a named individual;
- the handoff to the next role, with the expected input and output;
- the backup when the primary owner is absent;
- the reviewer who confirms completion;
- the escalation path when a task is blocked.

A process that depends on specific individuals, without documented roles and backups, fails when those individuals are unavailable.

### Design For Efficiency Without Sacrificing Control

A well-designed R2R process is both controlled and efficient. Efficiency comes from eliminating rework, automating mechanics, and resolving root causes, not from cutting controls.

Improve efficiency by:

- automating routine accruals, allocations, and recurring entries;
- front-loading tasks that can be done before period end;
- eliminating manual reclassifications by fixing root classification issues;
- standardizing the close calendar and checklist;
- resolving recurring reconciling items at their source.

Do not achieve speed by skipping reconciliation, review, or approval. A fast process that produces unreliable statements is not efficient; it is risky.

### Build In Monitoring And Continuous Improvement

R2R is not static. Processes degrade, volumes change, and systems evolve. Build in monitoring and improvement.

Monitor and improve:

- close duration and slippage against the calendar;
- the volume and aging of reconciling items;
- the number and nature of adjusting entries;
- recurring exceptions and their root causes;
- feedback from the team on bottlenecks and workarounds.

Use these signals to drive improvement. A process that is never reviewed and improved accumulates inefficiency and control drift.

### Produce Reporting That Serves Decisions

The output of R2R is not just the financial statements; it is the management information that drives decisions. Design reporting as part of the process.

Design reporting to deliver:

- financial statements compliant with the applicable framework;
- management reports by segment, product, geography, or project;
- variance analysis against budget and prior periods;
- key metrics and dashboards for decision-makers;
- timely delivery to support decisions, not just compliance.

Reporting that arrives too late to inform decisions, or that lacks the dimensions managers need, reduces the value of the entire R2R process.

### Acknowledge Framework And Professional Limits

R2R design implements accounting policy and control, but the policy must comply with the applicable reporting framework, tax, and regulatory requirements. Close, consolidation, revenue, lease, hedge, and intercompany processes often involve framework-specific requirements. Confirm significant process design decisions with qualified accounting professionals, and validate that the designed process produces framework-compliant, auditable results. Do not treat process design as a purely operational exercise; it is an accounting integrity decision.

## Common Traps

### Mapping That Stops At The GL

A map that ends at posting misses reconciliation and reporting, where most R2R failures appear.

### Reconciliation As An Afterthought

Optional, late, or unowned reconciliation produces drifting accounts and adjustment-heavy closes.

### Unsequenced Close

Running dependent tasks in parallel produces reconciliations on incomplete data and late adjustments.

### No Segregation

A process without separation of duties concentrates misstatement and misappropriation risk in one role.

### Role Ambiguity

Undefined roles and handoffs produce gaps, duplication, and failure when individuals are absent.

### Speed Without Control

Cutting reconciliation, review, or approval to close faster produces unreliable statements.

### No Monitoring Or Improvement

A process that is never reviewed accumulates inefficiency and control drift.

### Reporting That Misses Decisions and process Design Without Framework Review

Statements and reports that arrive too late or lack needed dimensions reduce R2R's value.

R2R must implement framework-compliant policy confirmed by qualified professionals.

## Self-Check

- Is the R2R process mapped end to end from source event through recording, posting, reconciliation, close, and reporting for each major transaction type?
- Is reconciliation embedded with owners, frequency, evidence, review, and escalation rather than treated as an afterthought?
- Is the close designed as a controlled sequence with dependencies enforced and parallelism only where genuinely independent?
- Are segregation of duties and approval gates built in to prevent concentration of initiation, recording, and approval in one role?
- Are roles, owners, handoffs, backups, reviewers, and escalation paths defined for each task?
- Is efficiency improved through automation, front-loading, and root-cause resolution without sacrificing reconciliation, review, or approval?
- Is the process monitored for close duration, reconciling items, adjusting entries, and exceptions, with continuous improvement?
- Does reporting deliver framework-compliant statements plus management information and metrics timely enough to inform decisions?
- Does the design reflect framework-compliant accounting policy confirmed with qualified professionals?
