---
name: remediation_tracking_and_evidence.md
description: Use when the agent is managing a remediation tracker, defining evidence requirements for closed actions, responding to regulator or auditor requests for remediation status, deciding whether an action is genuinely complete, or designing a remediation governance and reporting process.
---

# Remediation Tracking And Evidence

A remediation program lives or dies on the quality of its tracking and evidence. Regulators, auditors, and boards do not assess remediation by intention; they assess it by the record. A tracker that shows actions closing on time but cannot produce evidence of completion is worse than no tracker, because it creates a false assurance that drives away scrutiny until a failure exposes the gap. Conversely, a tracker burdened with excessive process can consume the compliance team's capacity while producing documentation that no one uses to make decisions. The compliance officer's job is to build a tracking and evidence regime that is rigorous enough to withstand external examination and lean enough to be maintained honestly under operational pressure.

Use this skill before designing a remediation tracker, defining evidence standards, preparing remediation reporting for a board or regulator, or deciding whether to accept an action as complete. The goal is to make the agent define what completion means before actions close, require evidence that matches the claim, and maintain a record that an independent party could reconstruct and trust. Remediation commitments made to regulators carry legal weight; coordinate closure decisions with counsel and do not represent actions as complete without verifiable support.

## Core Rules

### Define Completion Evidence Before The Action Closes

The most damaging remediation failures occur when an action is declared complete and only afterward does someone realize no evidence was generated. The claim cannot be substantiated, the closure is reversed, and credibility with the regulator or auditor collapses. Completion evidence must be defined at action creation, not at closure.

For each action, specify the evidence artifact that will demonstrate completion. Evidence types include system configuration screenshots with timestamps, revised policy documents with approval and effective dates, training completion reports with attendee lists and assessment scores, control test results over a defined population, exception reports showing zero or explained exceptions, and sign-offs from independent reviewers. The evidence must correspond to the completion criteria. If the criterion is that the control operates effectively, a single screenshot is insufficient; a test over a population with documented results is required.

### Require Evidence That Matches The Claim

The evidence must prove what is claimed, not something adjacent. A common weakness is evidence that documents activity without proving outcome. A training attendance roster proves people attended; it does not prove they learned or that behavior changed. A revised policy proves a document was written; it does not prove it was communicated, adopted, or followed. Match the evidence tier to the strength of the claim.

Tier evidence by rigor. Level one evidence shows the action was initiated, such as a project plan or kickoff memo. Level two shows the action was executed, such as a deployed configuration or delivered training. Level three shows the action is operating and effective, such as test results over time or monitoring data. Regulators and auditors generally require level two or three for closure of a material finding; level one is insufficient.

### Maintain An Immutable Audit Trail

Every change to a remediation action, its status, its owner, its due date, and its evidence must be recorded with a timestamp and the identity of the person who made the change. A tracker that can be silently edited cannot be trusted as a record. Status changes from open to closed, extensions of due dates, and reassignment of ownership are the events that regulators scrutinize most closely when assessing whether remediation was managed in good faith.

Use a system that enforces audit logging, whether a governance risk and compliance platform, a controlled spreadsheet with change history, or a ticketing system with immutable history. Avoid email-based tracking where the record is scattered across inboxes and cannot be reconstructed. If a regulator requests the history of a specific action, the response should be producible from a single authoritative source.

### Define Governance For Closure Approval

Closure should not be a unilateral decision by the action owner. The person who performed the remediation has an inherent interest in marking it done. Closure approval must come from an independent party, typically the second line of defense or a designated remediation governance forum. For material findings, internal audit or a committee may need to validate closure before the action is reported as complete externally.

Define closure authority by materiality. Low-risk actions may close with second-line review. Material actions require committee or internal audit validation. Actions committed to a regulator should not be reported as closed until the evidence has been independently verified, because misrepresenting closure to a regulator can itself constitute a violation.

### Manage Due Date Changes Transparently

Due dates change. Scope expands, dependencies slip, and resources prove insufficient. The problem is not change itself but undocumented or unjustified change. Every due date extension must record the reason, the approving authority, and the impact on residual risk during the extension period. A pattern of repeated extensions without justification signals that the remediation was never feasible or that ownership is not driving it.

Require that any extension of a regulator-committed date be assessed for whether it requires notification to the regulator. Silently missing a committed date and later explaining it is far more damaging than proactively disclosing the need for an extension with a credible revised plan.

### Report By Risk And Aging, Not Just Status

A status report that says twelve of fifteen actions are complete is misleading if the three open actions are the highest-risk ones and they are ninety days overdue. Remediation reporting must surface risk and aging alongside completion counts. Report open actions by risk rating, by days overdue, and by root cause category so that leadership can see where exposure remains concentrated.

Include trend information. Are closures keeping pace with new findings? Is the open backlog growing? Are the same root causes appearing in new findings, suggesting prior remediation was ineffective? A static snapshot hides these dynamics; trend reporting exposes whether the program is improving or merely treading water.

### Reconcile The Tracker Against Source Systems

A remediation tracker that is maintained separately from the systems where the work actually happens drifts out of sync. An action marked complete on the tracker may not reflect that the system change was rolled back, the training population was incomplete, or the control stopped operating. Periodic reconciliation between the tracker and source systems, control monitoring data, and training records catches divergence before an external party does.

## Common Traps

### Closing On Declaration Rather Than Evidence

Accepting an owner's statement that an action is complete without requiring the predefined evidence artifact. The closure cannot be substantiated and collapses under examination.

### Evidence That Documents Effort Rather Than Outcome

Submitting meeting minutes, draft documents, or activity logs as completion evidence when the claim is that a control is operating effectively. Effort evidence does not prove effectiveness.

### Silent Due Date Changes

Extending dates without recording the reason or approval, or without assessing whether the regulator must be notified. The tracker looks clean while commitments are being missed.

### Tracker Drift From Reality

Maintaining the tracker as a standalone artifact that is never reconciled against the systems, training records, or control data where the remediation actually lives.

### Reporting Green While Risk Concentrates

Reporting high completion percentages while the few open actions are the most material and the most overdue. Aggregate metrics mask concentrated residual risk.

### Owner Self-Closure

Allowing the person who performed the remediation to mark it complete without independent verification. The conflict of interest undermines the reliability of every closure.

### Over-Documenting Low-Risk Actions

Spending the same evidence rigor on a minor documentation fix as on a material control failure. This burns capacity and trivializes the process, leading to fatigue and corner-cutting on the actions that matter.

## Self-Check

- Is completion evidence defined for each action at creation, specifying the artifact that will prove the claim rather than being determined after the fact?
- Does the evidence tier match the claim, with material findings requiring level two or three evidence showing execution and effectiveness rather than only initiation?
- Is every tracker change, including status, owner, due date, and evidence, recorded in an immutable audit trail with timestamp and user identity?
- Is closure approved by an independent party scaled to materiality, with regulator-committed actions verified before external reporting?
- Are due date extensions documented with reason, approval, residual risk assessment, and a determination of whether regulator notification is required?
- Does remediation reporting surface risk rating, aging, and root cause concentration rather than only completion percentages?
- Are trend metrics included showing closure pace versus new findings and recurrence of prior root causes?
- Is the tracker periodically reconciled against source systems, control monitoring data, and training records to catch divergence?
- Are low-risk actions evidence-right-sized so that rigor is concentrated on material findings rather than spread uniformly?
- Could an independent reviewer or regulator reconstruct the full history and evidence basis of any closed action from the tracking system alone?
