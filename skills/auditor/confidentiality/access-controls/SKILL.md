---
name: access_controls.md
description: Use when the agent is evaluating or relying on access controls over financial systems and data, testing user access provisioning and review, segregation of duties, privileged access management, or deciding whether access control weaknesses constitute findings, recognizing that access controls are the foundation on which most other application controls depend.
---

# Access Controls

Access controls determine who can do what within financial systems and data, and they are the foundation on which most other controls rest. If the wrong people can enter, alter, or approve transactions, then controls over authorization, accuracy, and completeness are undermined at the root. Audits frequently identify access control weaknesses, but they also frequently underweight them, treating a list of excessive access rights as a minor housekeeping finding rather than as the precondition for override, fraud, and unreliable reporting. Evaluating access controls well requires understanding that access is not only about login credentials but about provisioning, review, segregation of duties, privileged access, and the joiner-mover-leaver lifecycle. A control environment with strong preventive controls but weak access management is a house with good locks on doors that are left open.

Use this skill when evaluating access controls, testing provisioning and review, assessing segregation of duties, reviewing privileged access, or determining the significance of access control findings. The goal is to assess whether access controls genuinely restrict inappropriate activity and to recognize when weaknesses undermine the reliability of the entire control environment.

## Core Rules

### Evaluate Access Across The Full Lifecycle

Access is not a static state but a lifecycle: users join, their roles change, and they leave. Weaknesses at any stage create exposure. A robust access control environment manages each transition promptly and correctly.

Evaluate the lifecycle by testing:

- joiner processes: how are new users provisioned, with what authorization, and are rights appropriate to role?
- mover processes: when users change roles, are old rights removed and new rights granted promptly?
- leaver processes: when users leave, is access disabled promptly, and are shared or generic accounts addressed?
- periodic review: are managers reviewing their staff's access and removing unnecessary rights?
- recertification: is access periodically re-authorized, or does it persist indefinitely once granted?

Access that is correct at provisioning but never reviewed accumulates excessive rights over time, as users move roles and old access is never removed. The lifecycle view catches what point-in-time testing misses.

### Test Segregation Of Duties As A System Property

Segregation of duties (SoD) prevents a single person from both performing and concealing an inappropriate action, such as initiating a payment and approving it, or recording a transaction and reconciling it. SoD is a system property, not an individual one, and conflicts must be assessed across the combination of rights a person holds, not role by role.

Test SoD by:

- obtaining the full set of rights for each user, across all relevant systems;
- mapping rights to sensitive functions and identifying conflicts where one user can perform incompatible actions;
- assessing whether conflicts are mitigated by compensating controls, such as supervisory review;
- reviewing the population for toxic combinations, such as the ability to create a vendor and approve its payment;
- recognizing that SoD conflicts in privileged or administrative accounts are especially dangerous.

A role-by-role review that misses the combinations a single user holds will declare segregation adequate while conflicts exist. Assess at the user level, across systems.

### Scrutinize Privileged And Administrative Access

Privileged and administrative access, the ability to bypass controls, change configurations, or access data without restriction, is the highest-risk category of access. Those with privileged access can, in effect, override the control environment, and weaknesses here are more significant than equivalent weaknesses in ordinary user access.

Scrutinize privileged access by:

- identifying all accounts with privileged or administrative rights;
- confirming each is individually attributable, not shared or generic, so actions can be traced to a person;
- verifying privileged access is limited to those who genuinely require it;
- testing whether privileged activity is logged and reviewed by someone independent of the privileged users;
- assessing whether emergency or firecall access is controlled, time-limited, and reviewed after use.

Privileged access that is shared, unlogged, or unreviewed is an open door to override and fraud. Treat weaknesses here as significant regardless of the number of accounts involved.

### Assess Provisioning Against Documented Authorization

Access should be granted based on documented authorization, with rights matched to role requirements. Access granted informally, or in excess of what was authorized, creates exposure that the formal process is meant to prevent.

Test provisioning by:

- selecting a sample of new or changed access rights and tracing each to documented authorization;
- confirming the rights granted match what was authorized, with no scope creep;
- verifying the authorizer had appropriate authority and was independent where SoD requires;
- testing for access granted without any visible authorization, which indicates a breakdown in control;
- assessing whether access requests are justified by business need or granted by default.

Access that exists without a visible authorization trail, or that exceeds what was authorized, indicates the provisioning control is not operating as designed.

### Evaluate Periodic Access Review For Effectiveness

Many environments rely on periodic manager review of staff access as a detective control over accumulated excessive rights. The effectiveness of this control depends on whether reviews are actually performed, whether reviewers have the information to identify unnecessary rights, and whether identified issues are corrected.

Evaluate review effectiveness by:

- confirming reviews occur at the required frequency and are documented;
- assessing whether reviewers receive meaningful information, not just a list to sign;
- testing whether identified unnecessary rights were actually removed;
- checking whether review covers all relevant systems, not only the primary application;
- looking for rubber-stamp reviews where everything is confirmed with no changes ever made.

A review process that produces no changes across cycles is either perfectly provisioned or, more likely, not genuinely performed. Test for evidence of actual review activity.

### Determine The Significance Of Access Findings

Access control weaknesses are often reported as long lists of excessive rights, which can obscure their significance. The auditor must assess which weaknesses matter, by connecting them to the risks they enable: override, fraud, error, unauthorized changes, or unreliable reporting.

Assess significance by considering:

- could the excessive access enable override of a significant control or misstatement?
- is the weakness in privileged access, which is higher-risk than ordinary user access?
- does the weakness reflect a systemic provisioning or review failure, or isolated lapses?
- are there compensating controls that mitigate the exposure?
- has the weakness persisted across multiple review periods without correction?

A long list of minor user-access issues may be less significant than a single unreviewed privileged account. Significance follows from the risk enabled, not from the count of exceptions.

### Connect Access Weaknesses To Broader Control Reliance

Access control weaknesses do not exist in isolation; they affect the reliability of every control that depends on access being appropriately restricted. When access controls are weak, the auditor must consider whether controls that appear to operate effectively can be relied upon, given that the wrong people may have been able to influence the transactions or records involved.

Consider the effect on broader reliance by:

- reducing reliance on controls where access weaknesses could enable override of those controls;
- increasing substantive testing where access weaknesses undermine control reliance;
- assessing whether access weaknesses affect the risk of material misstatement or fraud;
- evaluating whether prior-year reliance remains appropriate if weaknesses are long-standing.

Access is foundational. Weaknesses here propagate through the control assessment and must be reflected in the overall audit approach, not reported and set aside.

## Common Traps

### Point-In-Time Testing Missing The Lifecycle

Testing access at a single date misses joiner, mover, and leaver failures. Evaluate across the lifecycle.

### Role-Level SoD Missing User-Level Conflicts

Assessing roles individually misses the combinations a single user holds across systems. Test SoD at the user level.

### Underweighting Privileged Access

Treating privileged access weaknesses as equivalent to ordinary user access misses their override potential. Scrutinize and weight them heavily.

### Long Lists Obscuring Significance

Reporting every excessive right equally hides which weaknesses matter. Assess significance by risk enabled.

### Rubber-Stamp Review Accepted As Effective

A review process that never changes anything is likely not genuine. Test for evidence of actual review.

### Access Findings Disconnected From Control Reliance

Reporting access weaknesses without considering their effect on broader control reliance leaves the audit approach unchanged where it should adjust. Connect the two.

### Provisioning Tested Without Authorization Trace

Confirming access exists without tracing to authorization misses the control's core. Trace rights to documented authorization.

## Self-Check

- Is access evaluated across the full joiner-mover-leaver lifecycle, including provisioning, role changes, deprovisioning, and periodic review?
- Is segregation of duties tested at the user level across systems, identifying toxic combinations and assessing compensating controls?
- Is privileged and administrative access individually attributable, limited to genuine need, logged, and independently reviewed?
- Is provisioning traced to documented authorization, with rights matching what was authorized and no scope creep?
- Is periodic access review tested for effectiveness, including whether reviewers have meaningful information and whether identified issues are corrected?
- Are access findings assessed for significance based on the risk they enable, with privileged access and override potential weighted heavily?
- Are systemic provisioning or review failures distinguished from isolated lapses in assessing significance?
- Are access control weaknesses connected to their effect on broader control reliance and on the risk of misstatement or fraud?
- Is the audit approach adjusted where access weaknesses undermine reliance on controls that depend on appropriate access restriction?
- Could an independent reviewer confirm that access weaknesses were evaluated for their real significance rather than reported as routine housekeeping?
