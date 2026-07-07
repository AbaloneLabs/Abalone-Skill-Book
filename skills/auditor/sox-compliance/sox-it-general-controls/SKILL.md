---
name: sox_it_general_controls.md
description: Use when the agent is testing IT general controls (ITGCs) in a SOX ICFR engagement, evaluating access management change management and computer operations controls, determining which ITGCs are relevant to ICFR, linking ITGCs to the application controls and reports they support, or assessing how ITGC deficiencies affect the reliability of automated controls and the ICFR assessment.
---

# SOX IT General Controls

In a SOX integrated audit, IT general controls (ITGCs) are the foundational controls that sustain the reliability of the financial systems, automated controls, and reports on which ICFR relies. ITGCs cover access management, change management, computer operations, and program development. The recurring failure is treating ITGC testing as a standalone IT exercise, disconnected from the application controls and reports it supports, or testing ITGCs generically without linking them to the specific controls relevant to ICFR. When ITGCs are weak, the reliability of every automated control and system-generated report that depends on them is undermined, which can ripple through the ICFR assessment. The auditor must test the ITGCs that are relevant to ICFR, link them to the application controls and reports they support, and assess the effect of any ITGC deficiencies on the broader ICFR conclusion.

Use this skill when testing ITGCs in a SOX engagement, when linking ITGCs to application controls and reports, and when assessing the effect of ITGC deficiencies on ICFR. The goal is ITGC testing that supports the reliability of the controls and reports on which the ICFR opinion relies.

## Core Rules

### Identify The ITGCs Relevant To ICFR

Not all ITGCs are equally relevant to ICFR. The auditor must identify the ITGCs that affect the financial systems, automated controls, and reports on which ICFR relies, focusing testing on those that matter.

Identify by:

- mapping the financial systems and applications that are relevant to ICFR;
- identifying the ITGCs, in access, change, operations, and development, that affect those systems;
- focusing testing on the ITGCs that sustain the specific automated controls and reports in scope;
- excluding ITGCs over systems that do not affect ICFR from the scope.

Testing ITGCs generically, without linking them to ICFR-relevant systems, wastes effort and misses the controls that matter.

### Test Access Management Controls

Access management controls ensure that only authorized personnel can access financial systems and data, and that access is appropriate to the user's role. Weak access management can allow unauthorized changes, data manipulation, or fraud.

Test by:

- reviewing user access to financial systems, including new, changed, and terminated users;
- testing privileged and administrative access, which carries the highest risk;
- evaluating segregation of duties within system access, such as developers with production access;
- testing the periodic review of access rights by system owners;
- identifying access that is excessive, inappropriate, or unreviewed.

Excessive or unreviewed access, particularly privileged access, is a common ITGC deficiency with broad ICFR implications.

### Test Change Management Controls

Change management controls ensure that changes to financial systems are authorized, tested, and approved before deployment. Weak change management can allow unauthorized or untested changes that alter system behavior.

Test by:

- selecting a sample of changes and tracing each through the change lifecycle;
- verifying authorization, testing, and approval for each change;
- testing segregation between development and production environments;
- identifying emergency changes and their retrospective review;
- identifying direct developer access to production.

Weak change management, particularly developer production access or unreviewed emergency changes, undermines the integrity of the financial systems.

### Test Computer Operations Controls

Computer operations controls ensure that financial systems operate as intended, including job scheduling, processing integrity, backup and recovery, and incident management. Weak operations controls can lead to processing failures, data loss, or undetected errors.

Test by:

- reviewing job scheduling and processing controls for financial batch jobs and interfaces;
- testing controls over failed jobs, exceptions, and reprocessing;
- evaluating backup and recovery capabilities, including test restorations;
- testing incident management controls for financial system incidents;
- identifying operations failures that could affect processing integrity.

Operations failures, particularly uncontrolled reprocessing or untested recovery, can affect the completeness and accuracy of financial data.

### Link ITGCs To Application Controls And Reports

ITGCs do not operate in isolation; they sustain the application controls and system-generated reports on which ICFR relies. The auditor must link each ITGC to the specific application controls and reports it supports.

Link by:

- identifying the application controls and reports that depend on each ITGC;
- documenting the link between ITGCs and the controls and reports they sustain;
- recognizing that an ITGC deficiency affects every application control and report that depends on it;
- using the linkage to assess the scope of effect of any ITGC deficiency.

Without the linkage, ITGC deficiencies cannot be assessed for their effect on ICFR, and reliance cannot be supported.

### Assess The Effect Of ITGC Deficiencies On ICFR

Where ITGC deficiencies are identified, the auditor must assess their effect on the reliability of the dependent application controls and reports, and on the broader ICFR conclusion. An ITGC deficiency can ripple through multiple application controls, potentially aggregating to a significant deficiency or material weakness.

Assess by:

- identifying the application controls and reports affected by the ITGC deficiency;
- determining whether the affected controls and reports can still be relied upon, considering compensating controls;
- assessing whether the deficiency, alone or aggregated with others, reaches significant deficiency or material weakness level;
- where reliance is undermined, expanding substantive testing or adjusting the ICFR assessment.

Treating ITGC deficiencies as isolated IT issues, without assessing their ripple effect, understates their ICFR impact.

### Coordinate With The Financial Statement Audit

In a SOX integrated audit, ITGC testing supports both the ICFR opinion and the financial statement audit. The work should be coordinated to leverage evidence across both, avoiding duplication.

Coordinate by:

- using ITGC testing to support reliance on automated controls and reports for both ICFR and the financial statement audit;
- coordinating the timing and scope of ITGC testing across both audits;
- ensuring ITGC findings are considered in both the ICFR assessment and the financial statement risk assessment;
- documenting the use of ITGC work across both audits.

Treating ITGC testing as ICFR-only, without leveraging it for the financial statement audit, misses efficiency and risks inconsistency.

## Common Traps

### Treating ITGC Testing As A Standalone IT Exercise

ITGCs sustain the controls and reports on which ICFR relies. Treating the testing as standalone disconnects it from its purpose.

### Testing ITGCs Generically Without Linking To ICFR

ITGCs must be linked to the specific systems, controls, and reports in scope. Generic testing wastes effort and misses relevance.

### Overlooking Privileged And Developer Access

Privileged access and developer production access carry the highest risk. Overlooking them leaves severe deficiencies undetected.

### Missing Emergency Changes And Retrospective Review

Emergency changes are high-risk. Missing them, or their retrospective review, leaves a critical gap.

### Ignoring The Link To Application Controls And Reports

ITGC deficiencies affect dependent controls and reports. Without the link, the effect cannot be assessed.

### Treating ITGC Deficiencies As Isolated IT Issues

ITGC deficiencies ripple through ICFR. Treating them as isolated understates their impact.

### Failing To Coordinate With The Financial Statement Audit

ITGC testing supports both audits. Failing to coordinate misses efficiency and risks inconsistency.

## Self-Check

- Are the ITGCs relevant to ICFR identified, with testing focused on those sustaining in-scope systems, controls, and reports?
- Are access management controls tested, including user access, privileged access, segregation, and periodic review?
- Are change management controls tested, including the lifecycle, segregation, emergency changes, and developer production access?
- Are computer operations controls tested, including job scheduling, exception handling, backup and recovery, and incident management?
- Are ITGCs linked to the application controls and reports they sustain, with the linkage documented?
- Where ITGC deficiencies are identified, is their effect on dependent controls and reports and on the broader ICFR conclusion assessed?
- Is ITGC testing coordinated with the financial statement audit, leveraging evidence across both?
- Could an independent reviewer confirm that ITGC testing supports the reliability of the controls and reports on which ICFR relies?
- Is the application free of treating ITGC testing as standalone, testing generically, or overlooking privileged and developer access?
- Are ITGC deficiencies assessed for their ripple effect on ICFR, and is the work coordinated across both audits?
