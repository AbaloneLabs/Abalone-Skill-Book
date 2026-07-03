---
name: procedure_reexecution.md
description: Use when the agent is reperforming a control or process step during an audit, re-executing a procedure the way it should operate, observing whether the control catches or prevents an error, re-running system reports or reconciliations, or deciding whether reperformance provides evidence of operating effectiveness versus design effectiveness.
---

# Procedure Reexecution

Reperformance is the audit procedure of executing a control or process step independently, exactly as it should operate, to confirm that it functions as described. It is distinct from inquiry, which only asks about the control, and from inspection, which only looks at the evidence the control left behind. Reperformance tests the control in action. But its strength is also the source of a common misunderstanding: reperforming a control once proves that the control can work under the conditions the auditor created, not that it did work throughout the period or that it would catch every error. An auditor who re-runs a reconciliation and gets the right answer may conclude the control is effective, when in fact they have only shown it is well-designed. The discipline of procedure reexecution is to understand precisely what the reperformance proves, to execute it faithfully to the real process, and to combine it with other evidence to reach a conclusion about operating effectiveness.

Use this skill when reperforming controls, reconciliations, system reports, exception follow-ups, or process steps, when deciding what reperformance evidence supports, or when combining reperformance with other procedures to conclude on a control. The goal is to obtain reperformance evidence that genuinely tests the control and to draw conclusions no broader than the evidence supports.

## Core Rules

### Understand What Reperformance Proves And What It Does Not

Reperformance is powerful, but its scope must be understood precisely. A single reperformance demonstrates that the control can operate as intended under the conditions tested; it does not by itself prove the control operated effectively throughout the period.

Reperformance provides evidence of:

- whether the control is designed to achieve its objective;
- whether the control, when executed as designed, produces the correct result;
- whether the system or tool supporting the control functions correctly;
- whether the person performing the control, when observed, has the competence to execute it.

Reperformance does not, by itself, provide evidence of:

- whether the control operated on every occasion throughout the period;
- whether exceptions were consistently caught and followed up;
- whether the control operated at times the auditor was not observing.

Combine reperformance with inspection of evidence of operation over time, such as signatures, logs, or exception records, to support a conclusion about operating effectiveness.

### Reperform Faithfully To The Real Process

The value of reperformance depends on executing the control exactly as it should operate, not a simplified or idealized version. Shortcuts in reperformance produce a result that looks correct but does not test the real control.

Reperform faithfully by:

- obtaining the actual procedure documentation or system the operator uses;
- using the same data, system access, and reports the operator would use;
- following each step, including exception handling and escalation;
- not skipping steps that seem trivial, because those are often where controls break;
- executing at the same point in the process, such as before or after posting, as the real control requires.

If the auditor re-runs a "clean" version of the process that the operator would never actually perform, the reperformance tests an idealized control, not the real one.

### Test Whether The Control Catches Or Prevents Errors

A well-designed control should either prevent an error from occurring or detect and correct it. Reperformance can test this by introducing or identifying a condition the control should catch, then observing whether it does.

Test detection or prevention by:

- including an item the control should flag, such as an unauthorized transaction or an out-of-balance figure;
- introducing a deliberate error in a test environment where it is safe to do so;
- observing whether the reconciliation or report surfaces the discrepancy;
- checking whether the exception is escalated and followed up as the procedure requires.

A control that fails to catch an introduced error has a design or operating deficiency, even if it otherwise produces correct results. Document the test condition, the expected outcome, and the actual outcome.

### Distinguish Manual Reperformance From System Reperformance

Controls may be manual, system-automated, or a combination, and reperformance differs for each. Confusing the type leads to testing the wrong thing.

For manual controls:

- reperform the steps the human operator performs, using the same information and tools;
- observe whether the operator's judgment, where required, is applied correctly;
- test whether the evidence of performance, such as sign-off, is retained.

For system-automated controls:

- re-run the report, interface, or validation the system performs;
- confirm the system configuration, such as the rule or threshold, matches what is intended;
- test that the system processes the full population, not only a subset;
- consider IT general controls that support the automated control's reliability over time.

For combination controls, test both the system element and the manual follow-up. An automated report that no one reviews is not an effective control.

### Combine Reperformance With Evidence Of Operation Over Time

Because a single reperformance does not prove operation throughout the period, it must be combined with other procedures to support a conclusion on operating effectiveness.

Combine with:

- inspection of evidence that the control operated on other occasions, such as dated sign-offs, logs, or system records;
- inquiry about how exceptions were handled during the period;
- reperformance or inspection across multiple points in time, not only one;
- for automated controls, testing of IT general controls that govern changes and access.

The conclusion on operating effectiveness rests on the combination, not on the single reperformance. Document how the procedures together support the conclusion.

### Document The Reperformance Specifically And Reproducibly

Reperformance evidence must be documented so a reviewer can see what was reperformed, how, and with what result. Vague documentation such as "reperformed reconciliation, no exceptions" is not reviewable.

Document:

- the specific control or step reperformed;
- the data, system, and tools used;
- the steps executed, in sequence;
- any test conditions introduced and the outcomes;
- the result, including any discrepancies found;
- how the reperformance combines with other evidence to support the conclusion.

Where the reperformance involves a system report or calculation, retain or reference the output so the result can be verified.

## Common Traps

### Concluding Operating Effectiveness From A Single Reperformance

One reperformance proves design and capability, not operation throughout the period. Combine with evidence of operation over time.

### Reperforming An Idealized Version Of The Process

Skipping exception handling or using cleaner data than the real process produces a result that does not test the actual control.

### Confusing Design Effectiveness With Operating Effectiveness

A control that works when reperformed is well-designed; whether it operated all period is a separate question requiring additional evidence.

### Not Testing Whether The Control Catches Errors

A control that produces correct results on clean data but fails to flag exceptions is not effective. Test detection and prevention, not only the happy path.

### Treating An Automated Report No One Reviews As A Control

An automated report is only a control if someone reviews it and acts on exceptions. Test the human follow-up, not only the system output.

### Vague Documentation

"Reperformed, no exceptions" cannot be reviewed. Document the steps, data, test conditions, and result specifically.

### Overstating The Scope Of The Conclusion

Reperformance of a sample does not prove the whole population. State the conclusion for the scope actually tested.

## Self-Check

- Is it clear that the reperformance proves design and capability, and that operating effectiveness throughout the period requires additional evidence?
- Was the control reperformed faithfully to the real process, using the same data, system, steps, and exception handling the operator would use?
- Was the control tested for whether it catches or prevents errors, through introduced conditions or identified exceptions, not only the happy path?
- Is the type of control, manual, automated, or combination, correctly identified and tested, including any human follow-up to automated outputs?
- Is the reperformance combined with inspection of evidence of operation over time, such as logs, sign-offs, or IT general controls?
- Is the documentation specific enough that a reviewer can see what was reperformed, with what data, what test conditions, and what result?
- Is the conclusion limited to the scope actually tested, without overstating coverage of the population or period?
- For automated controls, has the system configuration and the supporting IT general controls been considered, not only the single re-run?
