---
name: segregation_of_duties_and_access_control_testing.md
description: Use when the agent is testing segregation of duties, conducting user access reviews, evaluating privileged access, performing access recertification, or assessing whether users have permissions that create conflicts or enable override of compliance controls.
---

# Segregation Of Duties And Access Control Testing

Segregation of duties (SoD) and access control testing address a fundamental compliance and control risk: the ability of a single person to initiate, approve, record, and conceal an improper transaction. Most fraud, override, and data misuse paths run through excessive or conflicting access. Yet access reviews are frequently treated as a rubber-stamp recertification where managers approve whatever they are shown. Effective SoD and access testing requires understanding the business process, the conflicting permission combinations, the privileged accounts, and the difference between having access and using it.

Use this skill before designing an access review, testing SoD conflicts, evaluating privileged access, scoping a recertification campaign, or responding to an access-related finding. The goal is to make the agent treat access as a live risk surface tied to specific duties, not as a list of usernames to be approved.

## Core Rules

### Define Conflicting Duties From The Business Process, Not From A Generic Matrix

SoD conflicts are meaningful only when tied to how work actually flows. A generic matrix of conflicting permissions may miss the conflicts that matter in a specific process and flag conflicts that are irrelevant.

For each in-scope process:

- map the duties from initiation through authorization, recording, reconciliation, and custody of assets or data;
- identify which duties, if held by one person, would allow them to both commit and conceal an error or improper act;
- define the specific conflicting permission combinations (for example, create vendor and approve payment, enter journal and reconcile, modify master data and post transactions, grant access and use the access);
- account for both system permissions and manual capabilities such as physical access or signature authority;
- consider surrogation, where one person performs duties on behalf of another.

Conflicts should be expressed in terms of the risk they enable, not only the technical permission names. A conflict that allows a user to set up a payee and release payment to that payee is a vendor-fraud risk; name it accordingly.

### Test Both SoD Conflicts And Excess Access

SoD addresses conflicting combinations; excess access addresses permissions beyond what a role requires. Both matter.

Test:

- SoD conflicts across the user population, using role and permission data;
- excess or inappropriate access, where users hold permissions unrelated to their duties;
- toxic combinations that cross systems, such as system access plus physical access plus approval authority;
- access to high-risk transactions such as payments, master data changes, journal entries, and data exports;
- access held by terminated, transferred, or long-inactive users;
- shared, generic, or service accounts and who can use them.

Excess access is often the larger exposure because it is broader and less scrutinized than named conflicts.

### Give Privileged And Administrative Access Heightened Scrutiny

Privileged accounts can bypass controls, alter logs, change configurations, and access data outside normal application controls. They deserve separate, deeper testing.

For privileged access evaluate:

- who holds privileged or administrative rights, by system;
- whether privileged access is granted by default to roles that do not need it;
- whether privileged sessions are logged, monitored, and reviewed;
- whether privileged access is time-limited, brokered, or require dual control;
- whether break-glass procedures exist and how their use is reviewed;
- whether administrators can modify the logs that would reveal their own actions;
- whether privileged users are subject to SoD, such as not also being able to initiate the transactions they administer;
- whether vendor or third-party privileged access is controlled and monitored.

Privileged access that is unaudited or self-audited is a serious design weakness. Confirm that monitoring of privileged users is independent and tamper-resistant.

### Make Recertification Meaningful, Not Ceremonial

Access recertification frequently fails because reviewers approve lists without scrutiny, because the information presented is unintelligible, or because reviewers do not understand the risk. A recertification that produces no removals across years is a red flag, not a success.

To make recertification meaningful:

- present access in business terms, mapping technical permissions to duties and risks, not raw role names;
- show the reviewer only the access relevant to their direct reports, with last-login and risk indicators;
- flag SoD conflicts, excess access, dormant accounts, and high-risk permissions for explicit attention;
- require reviewers to justify retention of flagged access, not merely sign;
- set a deadline and track completion and exceptions;
- act on the results: remove access, document accepted risk, or remediate conflicts;
- audit a sample of recertifications to confirm reviewers engaged meaningfully.

If reviewers consistently retain flagged high-risk access, investigate whether the access is genuinely required or whether the review is ceremonial.

### Reconcile Access To The Source Of Truth, Not Only To The Application

Access testing often examines what a user can do within an application but misses the broader access picture. A user may have limited application rights yet broad database, infrastructure, or data-warehouse access that undermines the application controls.

Reconcile:

- application roles and permissions;
- underlying database or schema permissions;
- infrastructure, cloud, and network access;
- data warehouse, reporting, and analytics access;
- single sign-on and identity provider group memberships;
- access granted through nested groups or inherited roles;
- third-party or vendor access to the same systems.

Nested groups are a frequent blind spot. A user may inherit high-risk access through a group they were added to for an unrelated reason.

### Test The Access Lifecycle, Not Only A Point In Time

A clean access review at year-end does not prove access was appropriate all year. Joiners, movers, and leavers create continuous risk.

Test the lifecycle:

- new hire provisioning timeliness and appropriateness;
- transfer or role-change access adjustments, including removal of access no longer needed;
- termination timeliness across application, infrastructure, and physical access;
- temporary access granted for projects and whether it was removed;
- access requests with documented approval and business justification;
- emergency or break-glass access and post-use review.

Many access failures occur in the mover phase, where a user gains new access but retains the access from their prior role, accumulating excess permissions over time.

### Connect Access Findings To Compensating Controls And Residual Risk

Not every SoD conflict can be eliminated. Some are accepted because of small team size, specialized skills, or cost. The question is whether compensating controls reduce the residual risk to acceptable levels.

For each accepted conflict:

- identify the compensating control, such as supervisory review, reconciliation, monitoring, or rotation;
- confirm the compensating control actually operates and generates evidence;
- assess whether it is independent of the person whose access creates the conflict;
- document the risk acceptance with owner, rationale, and review date;
- re-evaluate periodically, because team changes can eliminate the justification for the acceptance.

A risk acceptance without a functioning compensating control is an unmanaged exposure, not a mitigation.

### Treat Access Data Quality As A Testing Prerequisite

Access testing depends on accurate identity, role, and permission data. If the data is stale, incomplete, or inconsistent across systems, conclusions will be wrong.

Validate:

- the identity source and whether it reflects current employment status;
- whether role definitions are current and documented;
- whether permission-to-duty mappings are maintained;
- whether orphaned accounts, duplicate identities, or stale roles exist;
- whether access data can be reconciled across connected systems.

State limitations where data quality is poor, and avoid broad conclusions that the data cannot support.

## Common Traps

### Approving Access Lists Without Scrutiny

Recertification that produces no changes for years signals ceremonial review, not strong control. Flag and investigate.

### Relying On A Generic SoD Matrix Untied To The Process

Generic conflicts miss process-specific risks and create noise. Derive conflicts from the actual business flow.

### Ignoring Nested Groups And Inherited Access

Inherited permissions through groups are a frequent source of unintended excess access. Expand nested memberships before testing.

### Overlooking Privileged And Administrative Access

Privileged access can bypass application controls and logs. Test it separately and more deeply than standard access.

### Testing Application Access But Missing Database Or Infrastructure Access

Broad backend access can undermine tight application controls. Reconcile across the stack.

### Accepting SoD Conflicts Without A Functioning Compensating Control

A documented risk acceptance is not a mitigation unless the compensating control operates and is independent. Verify it.

### Treating Year-End Cleanliness As Proof Of Year-Long Control

Joiners, movers, and leavers create continuous risk. Test the lifecycle, not only a snapshot.

## Self-Check

- Are SoD conflicts defined from the actual business process, including initiation, authorization, recording, reconciliation, custody, and surrogation?
- Does testing cover both SoD conflicts and excess access, including high-risk transactions, dormant accounts, and shared or generic accounts?
- Is privileged and administrative access tested separately, with logging, monitoring, break-glass, and independence of self-audit evaluated?
- Is recertification presented in business terms with risk flags, last-login indicators, and required justification for retaining flagged access?
- Is access reconciled across application, database, infrastructure, data warehouse, identity provider, and nested group sources?
- Does testing cover the joiner, mover, and leaver lifecycle, including timeliness and removal of unneeded access after transfers?
- Are accepted SoD conflicts supported by functioning, independent compensating controls with documented risk acceptance and review dates?
- Is access data quality validated for identity currency, role definitions, permission mappings, orphans, and cross-system reconciliation before conclusions are drawn?
- Are findings acted upon through access removal, remediation, or documented risk acceptance rather than only reported?
- Is a sample of recertifications audited to confirm reviewers engaged meaningfully rather than rubber-stamping?
