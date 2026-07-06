---
name: segregation_of_duties_and_access_controls.md
description: Use when the agent is designing segregation of duties in a finance or ERP system, defining role-based access for accounting functions, evaluating access control risks, or remediating segregation-of-duties conflicts in the general ledger and subledgers.
---

# Segregation Of Duties And Access Controls

Segregation of duties and access controls are the foundation of internal control over financial reporting. The principle is simple to state and difficult to implement: no single person should control all stages of a transaction from initiation through recording through custody of assets. When duties are not segregated, one person can initiate, approve, record, and conceal a fraudulent transaction, and the control system has no independent check. In modern ERP and accounting systems, segregation of duties is largely a matter of role and access configuration, which means that access design is control design. A system that gives one role the ability to create a vendor, enter an invoice, approve payment, and post the entry has built the fraud risk into its configuration.

Use this skill before designing segregation of duties, defining role-based access, evaluating access risks, or remediating conflicts. The goal is to prevent the agent from designing access that concentrates incompatible duties, from treating access as an IT concern rather than a control concern, or from accepting residual conflicts without compensating controls.

## Core Rules

### Identify Incompatible Duties Across The Transaction Lifecycle

Segregation begins with identifying which duties must not reside in the same hands. Map the transaction lifecycle and separate incompatible functions.

Core incompatibilities to separate include:

- initiation and approval, such as creating a purchase order and approving it;
- recording and custody, such as posting a cash entry and having access to the bank account;
- authorization and reconciliation, such as approving payment and reconciling the bank;
- master data maintenance and transaction processing, such as creating a vendor and entering its invoice;
- system configuration and transaction posting, such as changing a validation rule and posting entries that rely on it.

The objective is that no single role can both execute and conceal a transaction. Where one role spans an incompatibility, a compensating control is required.

### Design Roles Around The Principle Of Least Privilege

Access should grant the minimum needed to perform a function. Broad access is the most common cause of segregation conflicts.

Design roles so that:

- each role maps to a defined job function, not to a person;
- access is granted by role, not by individual user assignment;
- roles do not bundle incompatible permissions for convenience;
- superuser or administrator access is tightly restricted and monitored;
- emergency or temporary access is granted through a controlled, logged, and time-limited mechanism.

Convenience-driven roles, such as a single role that can do everything an accountant needs, embed conflicts by design. Design for the function, then combine roles only where compatible.

### Map Roles To Specific System Transactions And Permissions

Segregation analysis is only as good as the permission mapping. Vague role descriptions hide real conflicts.

Map each role to:

- the specific transactions it can perform, such as FB60 for invoice entry or FBZ2 for payment run;
- the authorizations and authorization objects those transactions require;
- the organizational levels, such as company code or plant, the role spans;
- the master data the role can create, change, or display;
- the configuration and customization access, if any.

A role described as "AP clerk" without a transaction-level mapping cannot be assessed for conflicts. The mapping is the evidence.

### Detect And Resolve Conflicts Through A Conflict Matrix

Build a matrix that cross-references roles against incompatible duties to detect conflicts systematically, not by anecdote.

Use the matrix to:

- identify roles that contain incompatible permissions;
- identify users who, through multiple roles, accumulate incompatible access;
- detect sensitive access, such as the ability to post to suspense or to change posted entries;
- prioritize conflicts by risk, focusing on cash, procurement, and master data first.

Resolution options, in order of preference, are to remove the incompatible permission, split the role, or, where removal is not feasible, apply a documented compensating control such as enhanced review or monitoring.

### Apply Compensating Controls Where Segregation Is Not Possible

Some conflicts cannot be eliminated, particularly in small organizations with few staff. In these cases, a compensating control must reduce the residual risk to acceptable levels.

Valid compensating controls include:

- detailed review of transactions by someone independent of the conflicting role;
- monitoring reports that flag suspicious patterns, such as payments to new vendors;
- mandatory vacation or job rotation that requires another person to perform the function;
- reconciliations performed by someone without transaction initiation access;
- owner attestation and periodic access review.

A compensating control must be actually performed, documented, and tested. An unperformed compensating control provides no assurance and is worse than an acknowledged gap because it creates false confidence.

### Review Access Periodically And Remove Terminated Or Changed Access

Access that was appropriate when granted becomes a risk when roles change. Periodic review removes stale and excessive access.

Review access by:

- certifying user access at least annually, with managers confirming each user's roles are appropriate;
- removing access promptly upon termination or role change, including system and bank access;
- reviewing high-risk access, such as administrator and superuser, more frequently;
- reconciling active users against active employees to detect orphan accounts;
- reviewing shared or generic accounts and eliminating them where possible.

Stale access is a common audit finding and a real fraud vector. A terminated employee who retains system access is a segregation failure.

### Monitor And Log Access To Sensitive Transactions

Access design is preventive; monitoring is detective. Both are needed.

Monitor by:

- logging access to sensitive transactions, such as master data changes and period-open adjustments;
- reviewing logs for after-hours activity, excessive volumes, or access to unrelated modules;
- generating exception reports for sensitive access use and reviewing them regularly;
- ensuring logs themselves are protected from alteration by those whose activity they record.

A control that is never monitored will be tested by someone. Logging without review provides no assurance.

### Acknowledge Framework And Professional Limits

Segregation of duties and access controls support reliable financial reporting under the applicable framework, but control design must also address regulatory requirements such as SOX, tax, and industry regulation, which can impose specific control expectations. Confirm significant control design decisions with qualified accounting and audit professionals, and validate that the control environment supports framework-compliant and regulation-compliant reporting. Do not treat access configuration as a purely technical exercise; it is the implementation of the control environment.

## Common Traps

### Convenience Roles That Bundle Incompatible Duties

A role designed for a person's convenience embeds conflicts and makes segregation impossible.

### Vague Role Descriptions Without Permission Mapping

Roles described only by job title cannot be assessed for real conflicts.

### Conflicts Accepted Without Compensating Controls

A known conflict with no compensating control is an unmanaged fraud risk.

### Compensating Controls That Are Never Performed

A documented but unperformed compensating control creates false confidence and provides no assurance.

### Stale Access After Termination Or Role Change

Retained access for terminated or reassigned staff is a common and serious control failure.

### Unmonitored Sensitive Access

Logging without review, or no logging at all, means sensitive access is never tested.

### Shared Or Generic Accounts

Shared accounts destroy individual accountability and make segregation analysis impossible.

### Treating Access As An IT Concern and ignoring Regulatory Control Expectations

Access configuration is control design and must be owned by finance and control owners, not only IT.

SOX, tax, and industry regulation may impose specific control expectations beyond general segregation principles; confirm with professionals.

## Self-Check

- Are incompatible duties across the transaction lifecycle identified and separated, with no single role able to execute and conceal a transaction?
- Are roles designed on least privilege, mapped to functions rather than individuals, with restricted superuser and emergency access?
- Is each role mapped to specific transactions, authorizations, organizational levels, master data, and configuration access?
- Is a conflict matrix used to detect role-level and user-level conflicts and sensitive access, prioritized by risk?
- Where conflicts cannot be eliminated, are compensating controls documented, performed, and tested?
- Is user access reviewed periodically, with prompt removal of terminated or changed access and reconciliation against active employees?
- Is access to sensitive transactions logged, reviewed, and protected from alteration?
- Are shared and generic accounts eliminated or tightly controlled to preserve individual accountability?
- Does the control design reflect framework and regulatory expectations confirmed with qualified professionals?
