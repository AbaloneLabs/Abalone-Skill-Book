---
name: manual_intervention_points.md
description: Use when the agent is identifying manual intervention points in a process, assessing risks where humans override or re-enter data, evaluating manual workarounds and spreadsheet controls, or determining where manual steps create opportunity for error fraud or override in an otherwise automated flow.
---

# Manual Intervention Points

Automation creates a false sense of security. A process that is "mostly automated" still has points where humans enter, override, adjust, or re-enter data, and those points are where errors, fraud, and override actually occur. Agents often assess the automated controls carefully and treat the manual interventions as minor exceptions, when in fact the manual points are the highest-risk parts of the flow. A manual journal entry, a spreadsheet adjustment, or an override of a system block can undo the work of every automated control upstream. Identifying and assessing manual intervention points is therefore not a side task; it is where the real control risk in a modern process lives.

Use this skill when analyzing a process for manual points, when assessing the risk of manual overrides and workarounds, and when evaluating spreadsheet-based or offline controls. The goal is to locate every point where a human can alter the flow and to ensure each is controlled.

## Core Rules

### Identify Every Point Where A Human Can Alter The Flow

Begin by mapping every point in the process where a human can enter, change, override, or re-enter data. These are the manual intervention points, and each is a control risk.

Look for:

- manual journal entries, especially adjusting and top-side entries;
- manual overrides of system blocks, validations, or approvals;
- manual re-entry of data between systems or from spreadsheets;
- manual adjustments to estimates, reserves, or allocations;
- manual corrections, reversals, or write-offs;
- manual creation of records that bypass the normal initiation flow;
- manual approval steps where the approver can be bypassed or pressured;
- spreadsheet-based calculations or reconciliations that feed the ledger.

Document each manual point, who can perform it, and what triggers it. A process map that does not flag manual points hides the highest-risk locations.

### Assess The Risk At Each Manual Point

Each manual point carries a different risk profile depending on what it can affect, who can perform it, and whether it is controlled. Assess each deliberately.

Assess risk by considering:

- what the manual action can affect, such as revenue, expenses, estimates, or balances;
- whether the performer can both initiate and conceal the action;
- whether the action is logged and reviewable;
- whether the action requires approval or can be performed unilaterally;
- whether the action occurs near period end, when manipulation is most likely;
- whether the action affects a significant risk or a sensitive account.

Rank manual points by risk. A manual adjusting entry to revenue near period end by a senior person with no secondary review is a far higher risk than a manual data re-entry with a matching control.

### Evaluate Manual Overrides Of Automated Controls

Automated controls are reliable until they are overridden. The override capability is itself a control risk that must be assessed, often more serious than the control it bypasses.

For each override capability, assess:

- who has the authority to override;
- whether the override is logged and the log is reviewed;
- whether the override requires justification and approval;
- whether overrides are frequent, suggesting the control is mis-designed;
- whether overrides can be performed without detection;
- whether management can override silently, which is an override-risk pathway.

Treat override capability as a control in itself, requiring its own design and operating effectiveness assessment. An automated control with an uncontrolled override is not really a control.

### Scrutinize Spreadsheet And Offline Controls

Spreadsheets and offline tools are manual intervention points that are often poorly controlled. They can be powerful, but they are also where data is most easily altered.

For spreadsheet and offline controls, assess:

- whether the spreadsheet is the system of record or a supporting tool;
- who can edit the formulas, inputs, and structure;
- whether changes to the spreadsheet are version-controlled or logged;
- whether inputs are linked to source systems or manually entered;
- whether the spreadsheet output feeds the ledger or a disclosure;
- whether there are integrity controls, such as lock cells, access restrictions, or independent review.

A spreadsheet that feeds a material balance, with editable formulas and no change control, is a significant manual intervention point and often a significant risk.

### Assess Manual Workarounds For System Limitations

Workarounds arise when systems cannot handle a case and staff develop an informal process. These are manual intervention points that documentation often omits.

Identify workarounds by asking:

- what staff do when the system rejects or cannot process an item;
- whether items are processed offline and entered later in batches;
- whether manual records are kept outside the system and reconciled periodically;
- whether the workaround is known to management or is informal;
- whether the workaround has its own controls or operates uncontrolled.

Workarounds are often necessary, but uncontrolled workarounds are where errors and manipulation accumulate. Document each and assess whether it is controlled.

### Determine Whether Manual Points Have Compensating Controls

A manual intervention point does not always need to be eliminated, but it must be controlled. Assess whether a compensating control exists and operates.

Look for compensating controls such as:

- review and approval of manual entries by an independent person;
- reconciliation of manual entries to source documents or system records;
- exception reporting that surfaces manual or unusual entries;
- access restrictions that limit who can perform the manual action;
- monitoring of override logs by an independent function;
- periodic independent review of spreadsheet logic and outputs.

Where no compensating control exists, the manual point is an uncontrolled risk and must be addressed through procedure design, redesign, or a substantive response.

### Tie Manual Points To Fraud And Override Risk

Manual intervention points are the practical pathway for both fraud and management override. Connect the manual point analysis to the fraud and override assessments.

Connect by:

- identifying which manual points management could use to manipulate results;
- ensuring journal entry testing covers the manual points identified;
- ensuring estimate bias review covers manual adjustments to estimates;
- ensuring unusual transaction review covers manually created or adjusted items;
- assessing whether the manual points align with the fraud-risk areas identified in brainstorming.

A fraud and override assessment that does not incorporate the manual point analysis misses the practical pathways through which fraud and override occur.

## Common Traps

### Treating A Mostly Automated Process As Fully Automated

Even highly automated processes have manual points. Assuming automation eliminates manual risk leaves the highest-risk locations unexamined.

### Assessing Automated Controls But Ignoring Overrides

An automated control with an uncontrolled override is not really a control. The override capability is itself the control risk.

### Overlooking Spreadsheet And Offline Controls

Spreadsheets that feed material balances, with editable formulas and no change control, are significant manual intervention points and significant risks.

### Missing Informal Workarounds

Workarounds that staff develop to handle system limitations are manual points that documentation omits. Ask what happens when the system cannot process an item.

### Failing To Rank Manual Points By Risk

Not all manual points are equal. A manual adjusting entry to revenue near period end is a far higher risk than a routine data re-entry with a matching control.

### Leaving Manual Points Without Compensating Controls

A manual point does not need to be eliminated, but it must be controlled. An uncontrolled manual point is an uncontrolled risk.

### Disconnecting Manual Points From Fraud And Override Testing

Manual points are the practical pathway for fraud and override. If journal entry and estimate testing do not cover the identified manual points, the override response has a hole.

## Self-Check

- Is every point where a human can enter, override, adjust, or re-enter data identified and documented, including journal entries, overrides, spreadsheets, and workarounds?
- Is each manual point assessed for risk, considering what it affects, who can perform it, logging, approval, timing, and significance?
- Are override capabilities for automated controls assessed as control risks in themselves, with logging, approval, and frequency examined?
- Are spreadsheet and offline controls scrutinized for editable formulas, change control, input sourcing, and integrity controls?
- Are informal workarounds for system limitations identified, documented, and assessed for whether they are controlled?
- Does each material manual point have a compensating control, and where none exists, is the risk addressed through procedures or redesign?
- Are manual points tied to the fraud and override assessments, ensuring journal entry, estimate, and unusual transaction testing covers them?
- Could a reviewer locate, from the documentation, every manual point in the process and the control, if any, that governs it?
