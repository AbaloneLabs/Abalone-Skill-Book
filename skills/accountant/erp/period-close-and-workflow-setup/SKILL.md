---
name: period_close_and_workflow_setup.md
description: Use when the agent is designing or configuring the period close in an ERP, building close workflows and task dependencies, setting up close checklists and approvals, or configuring period locking, accrual automation, and close status tracking for a monthly or annual financial close.
---

# Period Close And Workflow Setup

The period close is the disciplined process by which a period's transactions are finalized, adjusted, reconciled, and locked so that financial statements can be produced. In an ERP, the close is shaped by workflow configuration: task sequences, dependencies, approvals, accrual automation, period locking, and status tracking. When the close workflow is well designed, the close is predictable, controlled, and auditable. When it is poorly designed, the close becomes a scramble of manual adjustments, missed accruals, unlocked periods, and late-emerging errors that undermine the reliability of the financial statements.

Use this skill before designing or reconfiguring the period close, building close workflows, setting up close checklists and approvals, or configuring period locking and accrual automation. The goal is to prevent the agent from building a close workflow that looks automated but skips critical steps, allows uncontrolled period access, or produces a close that is fast but not reliable.

## Core Rules

### Design The Close As A Controlled Sequence

The close is not a list of independent tasks. It is a sequence in which outputs of earlier tasks become inputs to later ones. Design it as a controlled workflow.

Sequence the close so that:

- subledger close precedes GL reconciliation;
- bank and credit card reconciliation precedes cash finalization;
- accruals and estimates are posted before pre-close trial balance review;
- intercompany matching and elimination precede consolidation;
- review and adjusting entries precede financial statement preparation;
- approval and period lock follow final review.

Tasks with dependencies should not be allowed to run in parallel where the dependency matters. The workflow should enforce the sequence, not rely on memory.

### Build A Complete Close Checklist With Owners

A close checklist captures every task required to close reliably. It should be complete, owned, and tracked.

For each task define:

- the task description and the expected output;
- the owner, by role or named individual;
- the dependency on prior tasks;
- the target day in the close calendar;
- the evidence of completion, a reconciliation, a report, a sign-off;
- the reviewer who confirms completion.

A close that relies on informal memory of what to do will miss tasks, especially when staff change. The checklist is the single source of truth for what must happen.

### Automate Accruals And Recurring Entries Carefully

Many close tasks are recurring accruals, allocations, amortizations, and reversals that can be automated. Automation improves speed and consistency, but it must be controlled.

For automated entries:

- define the calculation basis and the source data;
- configure the posting rule and the reversal logic;
- require review of the automated result before final posting;
- handle exceptions where the source data is missing or unusual;
- document the rule so it can be audited and refreshed.

Automation that posts without review, or that uses stale assumptions, produces confident-looking errors every period. Automate the mechanics, but keep a human review of the result.

### Enforce Period Locking And Access Control

Once a period is closed, it should be locked so that subsequent postings require explicit, approved reopening. Uncontrolled access to closed periods undermines the integrity of every close.

Configure:

- period locking at the appropriate point in the workflow, after final review and approval;
- access rights that restrict who can post to open, closing, and closed periods;
- a controlled reopening process that requires approval and documentation;
- audit logging of any postings to reopened periods;
- coordination with subledger periods so they lock consistently with the GL.

Backdated postings into closed periods, without control, are a common source of restatements and audit findings.

### Build Reconciliation Into The Workflow

Reconciliation is not an afterthought; it is a close task that should be embedded in the workflow. A close that produces financial statements without reconciled accounts is not reliable.

Embed:

- subledger-to-GL reconciliation for each material balance;
- bank and credit card reconciliation;
- intercompany matching before consolidation;
- account analysis for significant expense and accrual accounts;
- review of significant estimates and their support;
- a final trial balance review for unusual balances or movements.

Reconciliations should have a defined owner, a reviewer, and evidence of completion within the close timeline.

### Track Close Status And Exceptions

Close status tracking makes the close visible. It lets the close manager see what is done, what is pending, and what is blocked.

Track:

- task completion against the close calendar;
- dependencies that are blocking downstream tasks;
- exceptions and errors that require resolution;
- sign-offs and approvals at each gate;
- slippage against target close day.

Visibility turns the close from a black box into a managed process. Lack of visibility is why closes slip and errors emerge late.

### Design For Both Speed And Reliability

A fast close is valuable, but only if it is reliable. Do not trade control for speed.

Balance by:

- automating mechanical tasks to save time;
- keeping review and approval gates where judgment or risk is high;
- front-loading tasks that can be done before period end, such as pre-close accruals;
- resolving recurring issues at their root rather than adjusting every period;
- measuring close duration and error rates together, not just duration.

A close that is fast but produces adjustments and restatements is not actually fast. Reliability is part of speed.

### Acknowledge Framework And Professional Limits

The close workflow implements accounting policy and control, but the policy must comply with the applicable reporting framework. Accrual, cutoff, consolidation, and estimate treatments configured into the close must reflect GAAP, IFRS, tax, or regulatory requirements. Significant close configurations, especially those involving revenue, leases, hedging, or intercompany, should be confirmed with qualified accounting professionals. Do not treat workflow configuration as a substitute for accounting policy and framework review.

## Common Traps

### Tasks Run In Parallel Despite Dependencies

Allowing dependent tasks to run together produces reconciliations on incomplete data and late adjustments.

### Incomplete Or Informal Checklist

Relying on memory instead of a complete, owned checklist guarantees missed tasks when staff change.

### Unreviewed Automated Accruals

Automation that posts without review propagates stale assumptions and errors every period.

### Uncontrolled Access To Closed Periods

Backdated postings without approval or audit logging undermine close integrity.

### Close Without Embedded Reconciliation

Producing financial statements without reconciled accounts produces unreliable results.

### No Status Visibility

A close without status tracking slips and surfaces errors late.

### Speed Without Reliability

A fast close that requires constant adjustments and restatements is not actually fast.

### Configuration Without Framework Review

Close automation must implement framework-compliant policy confirmed by qualified professionals.

## Self-Check

- Is the close designed as a controlled sequence where dependencies are enforced, not run in parallel where they matter?
- Is there a complete close checklist with task, owner, dependency, target day, evidence, and reviewer for each item?
- Are automated accruals and recurring entries configured with defined basis, reversal logic, human review, and exception handling?
- Are periods locked after final review, with access control, approved reopening, and audit logging of any backdated postings?
- Is reconciliation embedded in the workflow for subledgers, bank, intercompany, accounts, and estimates?
- Is close status tracked against the calendar, with visibility into completion, blockers, exceptions, and slippage?
- Does the close balance speed with reliability, automating mechanics while keeping review gates and measuring error rates?
- Does the workflow implement framework-compliant accounting policy confirmed with qualified professionals, rather than substituting configuration for policy review?
