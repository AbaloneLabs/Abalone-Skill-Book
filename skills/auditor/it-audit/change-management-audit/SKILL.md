---
name: change_management_audit.md
description: Use when the agent is auditing change management controls over financial systems, evaluating the process for requesting approving testing and deploying changes to applications and infrastructure, testing controls over emergency changes and segregation between development and production, assessing the risk that unauthorized or untested changes affect financial systems, or determining whether change management controls support reliance on automated controls and system-generated data.
---

# Change Management Audit

Change management is the set of controls that govern how changes, to applications, infrastructure, configurations, and data, are requested, approved, tested, and deployed in production environments. It is a foundational general IT control: every automated control, every configured rule, and every system-generated report depends on the integrity of the underlying system, and that integrity depends on change management. The recurring failure is treating change management as an IT operational matter, reviewing the policy without testing whether actual changes followed it, or overlooking the highest-risk changes, such as emergency changes and direct updates to production. When change management is weak, unauthorized, untested, or malicious changes can alter the system's behavior in ways that affect every transaction and balance it produces. The auditor must test change management as a control that sustains reliance on the entire financial system.

Use this skill when auditing change management controls, when testing the change process, and when evaluating whether change management supports reliance on automated controls and system-generated data. The goal is change management testing that confirms the integrity of the financial systems on which the audit relies.

## Core Rules

### Understand Why Change Management Is A Foundational Control

Change management is not an IT operational nicety. It is the control that ensures financial systems remain in their intended, tested state. Every automated control, configured rule, and report depends on the system not being altered inappropriately.

Understand that:

- changes to financial systems can alter the behavior of automated controls, calculations, and reports;
- untested or unauthorized changes can introduce errors, vulnerabilities, or fraud into the system;
- change management is the control that prevents such alterations and supports reliance on the system;
- the auditor's reliance on application controls and system-generated data depends, in part, on change management.

Understanding the foundational role prevents treating change management as an IT matter outside the audit's scope.

### Test The Full Change Lifecycle

Change management is a lifecycle: a change is requested, evaluated, approved, tested, deployed, and verified. Testing must cover the full lifecycle, not only one stage, such as approval.

Test by:

- selecting a sample of changes and tracing each through the full lifecycle, from request to deployment;
- verifying that each change had a documented request, an appropriate evaluation, and an approval by authorized personnel;
- confirming that testing was performed before deployment, with evidence of test results;
- verifying that deployment followed controlled procedures and that the change was verified in production;
- identifying changes that bypassed one or more stages of the lifecycle.

Testing only approval, or only deployment, misses weaknesses in the other stages.

### Evaluate The Authorization And Approval Process

Authorization and approval ensure that changes are appropriate and that the right people are accountable. The process must be appropriate, with approvals by personnel independent of the change's development where segregation matters.

Evaluate by:

- confirming that changes are approved by authorized personnel with the appropriate authority;
- ensuring that developers do not approve their own changes for production deployment;
- verifying that approvals consider the change's effect on the financial system and its controls;
- identifying changes approved by personnel with conflicts of interest or insufficient authority.

Self-approval, or approval by personnel with conflicts, undermines the control's effectiveness.

### Test Controls Over Emergency Changes

Emergency changes bypass the normal lifecycle to address urgent issues. They are the highest-risk changes because they are expedited and may bypass testing or approval. Specific controls must govern emergency changes.

Test by:

- identifying emergency changes during the period and the basis for their emergency designation;
- verifying that emergency changes had appropriate, even if expedited, authorization;
- confirming that emergency changes were tested and documented after deployment, with retrospective review;
- assessing whether the volume of emergency changes indicates misuse of the emergency pathway;
- identifying emergency changes that were not subsequently reviewed or documented.

A high volume of emergency changes, or emergency changes without retrospective review, indicates that the emergency pathway is being misused to bypass normal controls.

### Test Segregation Between Development And Production

Segregation between development and production environments prevents untested or unauthorized code from reaching production. Developers should not have the ability to move changes into production or to alter production data directly.

Test by:

- reviewing access rights to determine whether developers have production access;
- confirming that migration to production is performed by personnel independent of development;
- testing for direct updates to production data or configuration by developers;
- identifying any mechanism by which developers can bypass the controlled migration process.

Direct developer access to production is a severe segregation failure that can allow unauthorized changes to the financial system.

### Evaluate Controls Over Configuration And Data Changes

Change management is not limited to application code. Configuration changes, such as changes to workflow rules, validation parameters, and automated controls, and data changes, such as direct updates to master data or transaction records, must also be governed.

Evaluate by:

- identifying the configuration parameters that affect financial controls and reporting;
- testing whether changes to those parameters follow the change management process;
- examining controls over direct data updates, such as master data changes and manual journal entries through the system;
- identifying configuration or data changes that bypass the controlled process.

Overlooking configuration and data changes leaves a significant category of system alteration untested.

### Assess The Effect Of Identified Weaknesses On Reliance

Where change management weaknesses are identified, the auditor must assess their effect on reliance. Weak change management undermines the reliability of automated controls and system-generated data, potentially requiring expanded substantive procedures.

Assess by:

- evaluating the nature and extent of identified weaknesses;
- determining whether the weaknesses could have allowed changes that affect financial controls or reporting;
- where weaknesses are significant, reducing reliance on application controls and system-generated data;
- expanding substantive procedures to compensate for the reduced reliance.

Treating change management weaknesses as isolated IT issues, without assessing their effect on reliance, leaves the audit exposed.

## Common Traps

### Treating Change Management As An IT Operational Matter

Change management sustains reliance on financial systems. Treating it as an IT matter outside the audit's scope is a fundamental error.

### Reviewing The Policy Without Testing Actual Changes

A policy is only as good as its execution. Actual changes must be traced through the lifecycle.

### Testing Only One Stage Of The Lifecycle

The lifecycle includes request, evaluation, approval, testing, deployment, and verification. Testing only one stage misses weaknesses in others.

### Overlooking Emergency Changes

Emergency changes are the highest-risk. Overlooking them, or their volume, leaves a critical gap.

### Allowing Developer Access To Production

Direct developer access to production is a severe segregation failure. It must be tested and identified.

### Ignoring Configuration And Data Changes

Changes to configuration and data affect financial controls. Ignoring them leaves a category untested.

### Treating Weaknesses As Isolated IT Issues

Change management weaknesses affect reliance on the entire financial system. Their effect on reliance must be assessed.

## Self-Check

- Is change management understood as a foundational control that sustains reliance on financial systems, not merely an IT operational matter?
- Are actual changes traced through the full lifecycle, from request to deployment and verification, not only the policy reviewed?
- Is the authorization and approval process evaluated for appropriateness, independence, and authority?
- Are emergency changes identified, with their basis, authorization, and retrospective review tested, and the volume assessed for misuse?
- Is segregation between development and production tested, with developer access to production identified as a severe failure?
- Are configuration and data changes included in the scope, recognizing their effect on financial controls?
- Where weaknesses are identified, is their effect on reliance assessed, with application control reliance reduced and substantive procedures expanded as needed?
- Could an independent reviewer confirm that change management testing supports the integrity of the financial systems on which the audit relies?
- Is the application free of treating change management as an IT matter, reviewing only the policy, or testing only one lifecycle stage?
- Are emergency changes, developer production access, and configuration and data changes addressed rather than overlooked?
