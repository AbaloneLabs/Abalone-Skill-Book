---
name: preventive-vs-detective-design.md
description: Use when the agent is evaluating whether a control is preventive or detective, designing control mixes, assessing whether preventive controls actually block errors before they hit the ledger, deciding if detective controls provide timely enough correction, or reviewing the balance between prevention and detection across a process.
---

# Preventive vs Detective Control Design

Control classification is not a labelling exercise. Whether a control is preventive (stops an error before it is recorded) or detective (finds an error after it is recorded) changes how much residual risk the process carries between the failure point and discovery, how much rework or correction is required, and how the auditor must time its testing. A process dominated by detective controls can look well-controlled on paper while quietly accumulating days or weeks of uncorrected error.

## Core Rules

### Distinguish the classification by timing of intervention, not by label

A control is preventive only if it physically or systematically blocks the error from entering the authoritative record in the first place. A control is detective if the error has already been recorded and the control identifies it for correction. Mislabelling is common: a manager who "reviews and approves" a journal entry before posting is preventive only if the entry cannot post without the approval; if the entry posts automatically and the manager reviews a report the next day, the control is detective regardless of what the procedure document calls it.

Confirm, for each control, the actual sequence:

1. At what exact point does the error originate?
2. Can the transaction complete and be recorded without this control firing?
3. Does the control act on the transaction before it is committed, or on a record of the transaction afterward?

If the answer to (2) is "yes," the control is detective even if it feels like approval.

### Weight preventive controls higher for high-value, irreversible, or time-sensitive transactions

Prevention matters most where correction is costly, slow, or impossible:

- payments that leave the bank and cannot be recalled;
- regulatory filings that are published or submitted;
- entries that feed downstream systems or investor reports in real time;
- payroll disbursements;
- transactions near period close where reversal is awkward.

For these flows, a detective control that finds the error a week later may be too late to matter. Prefer preventive design (system-enforced approval gates, three-way match hard-stops, validation rules that reject bad input) and treat detective controls as backup, not primary.

### Weight detective controls appropriately for high-volume, low-value, or exploratory processes

Detection is reasonable and often superior where:

- transaction volume is very high and per-transaction value is low (pre-approval of every item is impractical);
- the population is homogeneous and errors are easily corrected (pricing updates, small expense reimbursements);
- the risk is rare but material in aggregate (period-end reconciliation, trend monitoring).

Do not force preventive controls onto processes where the cost of prevention exceeds the cost of detection plus correction.

### Evaluate the correction lag, not just the detection

A detective control's value depends on how quickly it finds the error *and* how quickly the error is corrected. A monthly reconciliation that is completed but whose differences sit unresolved for 25 days is much weaker than its label suggests. For every detective control, establish:

- detection frequency (real-time, daily, weekly, monthly);
- investigation timeliness (who looks at exceptions, and when);
- correction timeliness (how long from discovery to corrected record);
- escalation path if correction is disputed or delayed.

A detective control with a slow correction loop provides weak assurance for assertions tied to period-end balances.

### Design for layered defence, not single-point reliance

Strong processes combine both types:

- preventive controls to stop the common, predictable errors;
- detective controls to catch what slips through, including fraud and unusual errors;
- corrective controls to ensure identified items are fixed and root-caused.

When a process relies on a single preventive control with no detective backup, a single control failure (the approver is absent, the validation rule is disabled) leaves the process entirely unprotected. Probe for the second line.

### Match the classification to the assertion being supported

Different financial statement assertions call for different mixes:

- **Existence / occurrence**: prevention of fictitious or duplicate transactions matters; preventive controls (approval gates, matching) are strong evidence.
- **Completeness**: detective controls (reconciliations between subsystems and GL, sequence checks) are often the primary tool because omission is hard to prevent.
- **Accuracy**: both preventive (system-calculated amounts, validation rules) and detective (review of exceptions) contribute.
- **Cutoff**: preventive date controls plus detective period-end review.
- **Classification**: detective review against a chart of accounts is common.

Do not assume one control type serves all assertions equally.

### Consider the human-behaviour dimension

Preventive controls that depend on a human "not clicking through" are weak. A dialog box that asks "Are you sure?" and can be dismissed is preventive in form but detective in effect, because users habitually approve. Truly preventive controls make the undesired action impossible or arduous (the system will not let the field be blank, the cheque cannot print without a second signature captured in the system). When assessing a preventive control, ask whether a determined but hurried user could bypass it in normal operation.

## Common Traps

- **Calling a next-day review "preventive approval."** If the transaction recorded before review, it is detective. This is the single most common misclassification and it materially understates residual risk.
- **Treating a reconciliation as strong just because it is performed.** A reconciliation that is done but whose differences are never investigated or corrected provides little assurance; its detective value is near zero.
- **Over-relying on a single preventive control.** "The system requires approval" is only as good as the approval control's design — segregation, override handling, and exception monitoring all matter.
- **Assuming automated means preventive.** A report that runs automatically and is emailed out is still detective. Automation changes cost and consistency, not classification.
- **Ignoring correction lag.** A control that detects in one day but corrects in thirty is weak for period-end assertions.
- **Forcing preventive controls onto high-volume low-value processes** where they create bottlenecks and get routinely overridden, converting a designed preventive control into an ineffective detective one.
- **Forgetting that fraud is designed to evade prevention.** Collusive or management-override frauds often defeat preventive controls deliberately; detective monitoring and entity-level controls are the backstop.
- **Classifying by the procedure document rather than the actual system behaviour.** Documents often describe intent; the system configuration is what determines whether a gate is truly preventive.

## Self-Check

- For each control I classified as preventive, can I state the exact mechanism that prevents the error from being recorded? If I cannot, reclassify it.
- For each detective control, have I documented the detection frequency, the investigation owner, the typical correction lag, and the escalation path?
- Have I confirmed whether transactions can complete and record without the control firing, by tracing an actual transaction rather than reading the procedure?
- Does the control mix match the assertion? (Preventive-heavy for existence/occurrence, detective present for completeness, etc.)
- For high-value or irreversible flows, is there a genuinely preventive control, or only a detective control dressed up as approval?
- Have I identified the second line of defence for each key preventive control — what catches errors if the preventive control fails or is overridden?
- Did I distinguish automation from classification, so that no automated detective control is being treated as preventive?
- For period-end assertions, have I checked that detective controls' correction lag does not extend past the balance sheet date in a way that leaves errors uncorrected in the reported balance?
