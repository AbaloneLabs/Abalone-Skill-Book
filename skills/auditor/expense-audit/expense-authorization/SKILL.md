---
name: expense_authorization.md
description: Use when the agent is testing expense approval controls, delegation of authority compliance, purchase card controls, travel and entertainment authorization, segregation between requestor approver and payor, or evaluating override of approval controls and unauthorized transactions.
---

# Expense Authorization

Authorization testing asks whether each expense was approved by the right person, at the right level, before it was incurred or paid, and it is the control most often overridden when someone wants to move money improperly. Agents frequently confirm that an approval stamp or signature exists and conclude the control operated, while the real risks lie in whether the approver had authority for that amount and type, whether the same person requested and approved, whether the approval was obtained after the fact, and whether an override bypassed the workflow entirely. An approval that exists on paper can still be unauthorized if the approver lacked authority, was the beneficiary, or approved their own request.

Use this skill when testing expense approval controls, delegation of authority compliance, purchase card and T&E authorization, segregation of duties, or suspected override. The goal is to test whether authorization was genuine, within authority, timely, and independent, not merely that an approval mark exists.

## Core Rules

### Obtain And Understand The Delegation Of Authority

Authorization testing is impossible without the actual delegation of authority (DOA) framework: who can approve what amount, what type, and under what conditions. Agents often test approvals against an assumed or partial understanding of authority, which produces meaningless conclusions. The DOA is the criterion, and it must be obtained in full before testing begins.

Obtain and confirm:

- the current DOA matrix and its effective dates;
- approval limits by role, amount, and transaction type;
- any temporary or elevated authority granted;
- authority for capital versus operating expenditure;
- authority for contracts, commitments, and one-off payments;
- requirements for dual or committee approval;
- authority for related-party or conflict-of-interest transactions;
- the policy on splitting transactions to evade limits;
- who can override the workflow and under what controls;
- the alignment of system roles with the DOA.

Document the DOA version used and reconcile it to system configuration.

### Test That The Approver Had Actual Authority

An approval is valid only if the approver was authorized for that specific transaction. A signature from someone over their limit, outside their scope, or in the wrong role is an authorization failure even though a mark exists. Test authority against the actual transaction attributes.

For each approved expense, confirm:

- the approver's role and limits at the approval date;
- the transaction amount against the approver's limit;
- the transaction type against the approver's scope;
- the cost center or entity the approver controls;
- whether authority was delegated validly;
- whether the approver was still in role at the time;
- whether dual approval was required and obtained;
- whether the approval reflects the actual commitment, not just the invoice;
- whether the approver had any conflict of interest;
- consistency between system roles and the DOA.

### Verify Segregation Between Requestor, Approver, And Payor

Authorization is undermined when one person can request, approve, and pay the same transaction. Segregation of duties prevents an individual from creating and concealing an improper payment. Test the segregation, not just the presence of an approval.

Test segregation for:

- the requestor differs from the approver;
- the approver differs from the person entering or paying;
- the payor differs from the vendor-set-up function;
- no single user can create a vendor and pay it;
- no single user can raise and approve a manual payment;
- system roles enforce the segregation;
- compensating controls where segregation is impractical;
- the independence of approvers for executive expenses;
- segregation in emergency or after-hours processes;
- segregation across intercompany and related-party payments.

Where segregation cannot be achieved, confirm compensating monitoring controls operate.

### Confirm Approvals Were Timely And Before The Commitment

An approval obtained after the expense was incurred or the goods received is not effective authorization; it is ratification. Late approvals are common and often accepted without question, but they indicate the control did not operate as designed. Test the timing of approval relative to the transaction.

Test timing by:

- comparing approval date to purchase order or commitment date;
- comparing approval date to receiving or service date;
- comparing approval date to invoice and payment date;
- identifying approvals dated the same day as payment;
- identifying batch or backdated approvals;
- testing emergency purchases for after-the-fact approval;
- reviewing approvals clustered at period end;
- testing whether the workflow enforces pre-approval;
- the policy threshold for after-the-fact approval;
- the frequency and cause of late approvals.

A pattern of late approvals indicates a control that exists on paper but not in practice.

### Examine Purchase Card Controls Specifically

Purchase cards bypass the normal purchase-to-pay workflow, which makes their controls critical and frequently weak. P-card transactions are small, numerous, and often approved by the cardholder or a peer, creating both opportunity and weak oversight. Test p-card as a distinct population.

Test p-card authorization by:

- reviewing cardholder limits and merchant restrictions;
- testing that transactions are within limits and policy;
- confirming approver independence from the cardholder;
- reviewing split transactions to evade limits;
- testing personal-use detection and recovery;
- reviewing merchant category codes for policy violations;
- testing recurring and subscription charges for authorization;
- reviewing transactions near the card limit;
- testing reconciliation of statements to supporting receipts;
- the escalation process for policy violations.

### Test Travel And Entertainment Authorization

T&E is high-risk because it is employee-initiated, often self-approved at higher levels, and judged on business purpose rather than amount alone. The authorization must address both the spending limit and the legitimacy of the expense. Test T&E against policy and independence.

Test T&E authorization for:

- approval by someone other than the claimant;
- self-approval by executives and its control;
- policy limits for meals, travel, and entertainment;
- attendee lists and business purpose documentation;
- expenses above policy requiring elevated approval;
- duplicate or overlapping claims;
- expenses near reimbursement thresholds;
- charges in personal or non-business locations;
- consistency with the employee's role and travel;
- exceptions and their approval trail.

### Detect Override Of Approval Controls

Override is the deliberate circumvention of the approval workflow, and it is the most dangerous authorization failure because it is intentional. Override occurs through manual journal entries, superuser access, emergency processes, or splitting transactions below thresholds. Test for override specifically, not just for routine operation.

Look for override indicators:

- manual journal entries bypassing the approval workflow;
- payments processed outside the normal system;
- superuser or administrator transactions;
- emergency or expedited payment processes;
- transactions split to fall below approval limits;
- approvals by users with broad override authority;
- disabled or bypassed workflow steps;
- changes to the DOA or system roles near period end;
- approvals by the beneficiary of the expense;
- transactions reclassified after approval to change limits.

### Reconcile System Roles To The Delegation Of Authority

The DOA is only effective if the system roles that enforce it match the documented authority. Mismatches between policy and system configuration are a common control gap, often invisible because agents test transactions but not the underlying role setup. Reconcile the configuration to the policy.

Reconcile by:

- mapping system approval roles to the DOA matrix;
- testing that limits in the system match policy limits;
- identifying users with roles exceeding their authority;
- reviewing role changes and their approval;
- testing termination and role-removal timeliness;
- identifying shared or generic accounts with approval rights;
- testing emergency access and its revocation;
- reviewing segregation enforced by system configuration;
- the recertification of access rights;
- the change-management controls over role configuration.

### Evaluate Exceptions For Control Implication

Each authorization exception should be evaluated for whether it is isolated, systemic, or indicative of override. Agents often clear exceptions as training issues without assessing the underlying control failure. The pattern and cause matter more than any single instance.

For each exception, determine:

- whether approval was missing, late, or outside authority;
- whether the exception is isolated or systemic;
- the monetary impact and the accounts affected;
- whether segregation was breached;
- whether the exception indicates override;
- whether the same approver or requestor recurs;
- whether management's explanation is supported;
- whether the control design or operation failed;
- whether fraud risk is indicated;
- the root cause and recommended remediation.

## Common Traps

### Accepting An Approval Mark Without Testing Authority

A signature or stamp proves an approval was recorded, not that the approver had authority. Test the approver's limits against the transaction.

### Treating Late Approval As Effective

Approval after the commitment is ratification, not authorization. Test the timing of approval relative to the transaction.

### Ignoring Segregation In Practice

The workflow may require segregation that users circumvent by sharing logins or roles. Test that requestor, approver, and payor are genuinely different people.

### Sampling Away From Override

Random samples miss the few overridden transactions. Target manual entries, superuser activity, and split transactions explicitly.

### Treating P-Card And T&E As Low Risk

Small, numerous, employee-initiated transactions are high-risk for misuse. Test authorization and business purpose, not just receipts.

### Missing Split Transactions Below Limits

Splitting a purchase to evade an approval limit is override. Run analytics for clustered or sequential sub-threshold transactions.

### Trusting System Roles Without Reconciliation

The system may grant authority the policy never intended. Reconcile system roles and limits to the DOA.

### Clearing Exceptions As Training Issues

A pattern of authorization exceptions indicates a control failure or override. Evaluate cause and implication, not just the individual item.

## Self-Check

- Is the current delegation of authority obtained, version-documented, and reconciled to system configuration before testing?
- For each approved expense, is the approver confirmed to have had authority for the amount, type, and scope at the approval date?
- Is segregation between requestor, approver, and payor tested, with compensating controls confirmed where segregation is impractical?
- Are approvals tested for timing relative to commitment, receipt, and payment, with late approvals identified?
- Are purchase card transactions tested as a distinct population for limits, merchant restrictions, approver independence, and personal use?
- Is travel and entertainment tested for approver independence, policy limits, business purpose, and self-approval at executive levels?
- Are override indicators such as manual entries, superuser activity, emergency processes, and split transactions specifically targeted?
- Are system roles and limits reconciled to the delegation of authority, with excess access and termination timeliness tested?
- Is each authorization exception evaluated for whether it is isolated, systemic, or indicative of override and fraud risk?
- Does the conclusion address whether authorization was genuine, within authority, timely, and independent, not merely that an approval mark exists?
