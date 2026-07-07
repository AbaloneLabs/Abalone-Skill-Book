---
name: soc_report_reliance.md
description: Use when the agent is relying on a SOC 1 (SSAE 18) report from a service organization in a SOX ICFR engagement or financial statement audit, evaluating the scope and coverage of a SOC report, determining whether the service organization's controls are relevant to user entity ICFR, assessing complementary user entity controls and subservice organizations, or deciding whether and how to place reliance on a SOC report in lieu of direct testing.
---

# SOC Report Reliance

Many entities use service organizations to process transactions, host applications, or perform functions relevant to ICFR and the financial statements. In a SOX engagement or financial statement audit, the auditor often relies on a SOC 1 (SSAE 18 / ISAE 3402) report issued by the service auditor rather than testing the service organization's controls directly. The recurring failure is treating a SOC report as a blanket assurance that covers all user-entity risks, without reading the scope, the period of coverage, the qualified opinion, or the complementary user entity controls that the user entity must implement for the service organization's controls to achieve their objectives. Reliance placed on a SOC report that does not cover the relevant processes, the relevant period, or the relevant control objectives is unsupported, and any conclusion resting on it is unreliable. The auditor must read the SOC report critically, confirm that it covers the relevant scope and period, identify the user entity controls that must operate, and determine whether reliance is appropriate and sufficient.

Use this skill when evaluating a SOC report for reliance in a SOX or financial statement audit, when determining scope and period coverage, and when identifying complementary user entity controls and subservice organizations. The goal is reliance that is supported by the report's actual coverage and that accounts for the controls the user entity must itself operate.

## Core Rules

### Confirm The Report Type And Relevance

SOC reports differ in purpose and audience. A SOC 1 report, Type 2, is the report that addresses controls relevant to user entity ICFR and is the primary report for SOX and financial statement audit reliance. A SOC 1 Type 1 addresses control design at a point in time only and does not support conclusions about operating effectiveness. SOC 2 and SOC 3 reports address security, availability, processing integrity, confidentiality, and privacy criteria and are generally not designed for ICFR reliance.

Confirm by:

- verifying the report is a SOC 1 (SSAE 18 / ISAE 3402), not a SOC 2 or SOC 3, for ICFR reliance;
- verifying the report is a Type 2, covering design and operating effectiveness over a period, not a Type 1 design-only report;
- confirming the report is issued by a qualified service auditor and is intended for user auditor reliance;
- rejecting reliance on a report of the wrong type, regardless of how convenient it would be.

Reliance on a SOC 2 for ICFR, or on a Type 1 for operating effectiveness, is unsupported.

### Confirm Scope And Control Objective Coverage

A SOC 1 report covers only the control objectives the service auditor was engaged to test. If the objectives relevant to the user entity's ICFR are not included, the report does not cover them, and reliance cannot be placed for those processes.

Confirm by:

- reading the description of the service organization's system to confirm the in-scope services and processes;
- identifying the control objectives tested and matching them to the user entity's relevant ICFR processes;
- identifying any relevant processes or control objectives that are excluded from the scope;
- where relevant objectives are excluded, planning direct testing or obtaining an expanded report.

A report that does not cover a relevant process provides no assurance for that process, regardless of its overall opinion.

### Confirm The Period Of Coverage

A SOC 1 Type 2 report covers a defined period. The user entity's fiscal year may not align with the service organization's reporting period, leaving gaps during which no assurance is available.

Confirm by:

- reading the period covered by the report and comparing it to the user entity's fiscal year;
- identifying any gap between the report's period-end and the user entity's reporting date;
- where a gap exists, obtaining a gap letter, a bridge letter, or performing procedures to cover the interim period;
- confirming that coverage extends across the entire period relevant to the audit.

Reliance on a report whose period does not cover the user entity's fiscal year, without addressing the gap, is unsupported.

### Read The Opinion And Qualifications

The service auditor's opinion may be unqualified or qualified, and may include exceptions noted in testing. A qualified opinion, or a report with numerous exceptions, may not support reliance on the affected control objectives.

Read by:

- reading the service auditor's opinion for qualifications or modifications;
- reading the testing results for exceptions, deviations rates, and the service auditor's conclusions;
- identifying control objectives affected by exceptions or qualifications;
- assessing whether the exceptions undermine the controls on which the user entity's ICFR relies.

An unqualified opinion does not, by itself, establish that the specific controls relevant to the user entity operated effectively; the testing results must be read.

### Identify And Test Complementary User Entity Controls

SOC 1 reports often assume that the user entity implements certain controls, called complementary user entity controls (CUECs), for the service organization's controls to achieve their objectives. If the user entity does not implement those controls, the service organization's controls cannot be relied upon in isolation.

Identify by:

- reading the report for the CUECs the service organization expects the user entity to implement;
- matching each CUEC to a control at the user entity;
- testing the operating effectiveness of the relevant user entity controls;
- where a CUEC is not implemented or is ineffective, assessing the effect on reliance.

Reliance that ignores CUECs treats the service organization's controls as standalone when they depend on user entity controls to be effective.

### Consider Subservice Organizations

A service organization may itself rely on subservice organizations, using either the inclusive method (subsidiary controls included in the description) or the carve-out method (subsidiary controls excluded, with complementary subservice organization controls identified). The user auditor must understand which method is used and address the subservice organization accordingly.

Consider by:

- identifying whether the report uses the inclusive or carve-out method for subservice organizations;
- for the inclusive method, confirming the subservice organization's controls are included in the scope;
- for the carve-out method, identifying the complementary subservice organization controls and determining how to address them;
- obtaining a SOC report for the subservice organization where the carve-out controls are relevant.

Ignoring subservice organizations leaves a gap in the assurance chain, particularly where the subservice performs a relevant function.

### Determine Whether Reliance Is Appropriate And Sufficient

After confirming type, scope, period, opinion, CUECs, and subservice organizations, the auditor determines whether reliance on the SOC report is appropriate and sufficient, or whether direct testing or additional procedures are needed.

Determine by:

- concluding whether the report covers the relevant processes, objectives, and period;
- concluding whether the opinion and testing results support reliance on the relevant controls;
- concluding whether the CUECs are implemented and effective at the user entity;
- where coverage, opinion, or CUECs are insufficient, planning direct testing of the service organization's controls or the affected processes.

Reliance is a judgment, not a default. Where the report does not support it, direct testing is required.

### Document The Basis For Reliance

The basis for reliance, including the report identification, the scope and period confirmed, the opinion read, the CUECs tested, and the conclusion, must be documented. Documentation supports the reliance conclusion and enables review.

Document by:

- recording the report title, date, period, service auditor, and opinion;
- documenting the scope and control objectives confirmed as relevant;
- documenting the CUECs identified and the testing of the user entity controls;
- documenting the conclusion on reliance and any additional procedures performed.

Undocumented reliance cannot be reviewed or defended, particularly under PCAOB inspection.

## Common Traps

### Treating A SOC Report As Blanket Assurance

A SOC report covers only its stated scope, period, and objectives. Treating it as blanket assurance overstates the coverage.

### Relying On A SOC 2 Or Type 1 For ICFR

SOC 2 addresses security criteria, not ICFR. Type 1 addresses design only, not operating effectiveness. Reliance for ICFR requires a SOC 1 Type 2.

### Ignoring Scope And Control Objective Gaps

A report that does not cover a relevant process or objective provides no assurance for it. Ignoring gaps leaves unsupported conclusions.

### Overlooking Period Gaps

The report period may not cover the user entity's fiscal year. Overlooking the gap leaves a period without assurance.

### Stopping At The Unqualified Opinion

An unqualified opinion does not confirm that specific relevant controls operated effectively. The testing results and exceptions must be read.

### Ignoring Complementary User Entity Controls

Service organization controls often depend on user entity controls. Ignoring CUECs treats dependent controls as standalone.

### Missing Subservice Organizations

Subservice organizations may perform relevant functions. Missing them, or the carve-out controls, leaves a gap.

### Undocumented Reliance

Reliance must be documented, including report identification, scope, period, opinion, CUECs, and conclusion. Undocumented reliance cannot be defended.

## Self-Check

- Is the report confirmed as a SOC 1 Type 2, issued by a qualified service auditor, and intended for user auditor reliance?
- Does the scope cover the relevant services, processes, and control objectives, with any gaps identified and addressed?
- Does the period of coverage align with the user entity's fiscal year, with any gap addressed through a bridge letter or procedures?
- Has the opinion been read for qualifications, and have the testing results been read for exceptions affecting relevant controls?
- Are complementary user entity controls identified, matched to user entity controls, and tested for operating effectiveness?
- Are subservice organizations considered, with the inclusive or carve-out method understood and addressed?
- Is the conclusion on reliance supported by coverage, opinion, and CUECs, with direct testing planned where reliance is insufficient?
- Is the basis for reliance documented, including report identification, scope, period, opinion, CUECs, and conclusion?
- Could an independent reviewer confirm that reliance is supported by the report's actual coverage and accounts for user entity controls?
- Is the application free of treating the report as blanket assurance, relying on the wrong report type, or ignoring CUECs and subservice organizations?
