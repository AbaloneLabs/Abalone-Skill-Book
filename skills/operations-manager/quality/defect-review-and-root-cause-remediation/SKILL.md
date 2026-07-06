---
name: defect-review-and-root-cause-remediation.md
description: Use when the agent is reviewing operational defects, customer-impacting errors, recurring quality failures, rework patterns, root causes, corrective actions, preventive actions, defect remediation, or whether a quality issue has actually been fixed.
---

# Defect Review And Root Cause Remediation

A defect review should explain how an error happened, who or what was affected, what must be corrected now, and what will prevent recurrence. Agents often stop at the apparent mistake: a person clicked the wrong option, missed a field, or sent the wrong answer. That may be true, but it is rarely enough. Operational defects usually involve upstream inputs, tool design, policy ambiguity, workload pressure, incentives, review gaps, or handoff failures.

Use this skill before summarizing a defect, writing a root-cause review, proposing corrective action, closing a quality incident, or deciding whether a recurring error pattern needs process remediation.

## Core Rules

### Start with impact and containment

Before analyzing root cause, determine who or what may be affected. Identify customers, accounts, orders, payments, records, vendors, employees, compliance obligations, safety exposure, and downstream systems. Contain ongoing harm before debating cause.

Containment may include pausing a process, blocking further releases, correcting records, notifying customers, reversing transactions, quarantining work, escalating to compliance or legal, or adding temporary review. Make containment owner and completion evidence explicit.

### Describe the defect in concrete terms

State what was expected, what actually happened, when it happened, how it was detected, how many items may be affected, and what evidence supports the finding. Avoid vague labels such as "process issue" or "agent mistake."

The defect description should be specific enough that another reviewer can verify it. If the defect cannot be reproduced or traced, state uncertainty rather than inventing a clean story.

### Separate proximate cause from system cause

The proximate cause may be a missed step, incorrect decision, data entry error, or failed handoff. The system cause explains why that proximate error was likely or not caught. Look for unclear instructions, missing validation, poor tool design, insufficient training, excessive workload, conflicting metrics, weak review, bad intake, policy ambiguity, or unavailable expertise.

Do not stop at "human error." Human error is often the starting point for investigation, not the root cause.

### Use a structured cause method without forcing it

Methods such as five whys, fishbone, fault tree, or process mapping can help, but they are tools, not rituals. Use them to test whether the proposed cause explains the evidence and recurrence pattern.

Ask whether the same defect could recur under the current process. If yes, the cause analysis is not complete or the corrective action is insufficient.

### Distinguish correction, corrective action, and preventive action

Correction fixes the specific defective item. Corrective action fixes the cause of this defect. Preventive action reduces the chance of similar defects elsewhere. A complete remediation plan often needs all three.

For example, correcting a wrong invoice is not enough if the pricing table remains wrong. Updating the pricing table may not be enough if no one reviews future price changes. Define all levels explicitly.

### Assign owners and verification methods

Every action needs an owner, due date, evidence of completion, and verification method. Verification should show that the action worked, not only that it was performed. Training was delivered is weaker evidence than subsequent audited cases showing the defect rate fell.

Avoid actions such as "remind the team" unless the defect truly came from awareness and the reminder's effect can be checked. Most repeated defects need process, tool, control, or policy changes.

### Decide when to broaden the review

A defect may be isolated or may indicate a larger population at risk. Determine whether similar work, time periods, customers, staff groups, vendors, regions, products, or systems should be reviewed. Use sampling or full population review depending on severity and detectability.

Do not assume the detected case is the only case merely because only one customer complained or one auditor found it.

### Communicate without blame or concealment

Internal communication should be factual and oriented toward learning. Customer or stakeholder communication should acknowledge impact, explain what can be shared, state what was corrected, and avoid unsupported certainty. Sensitive matters may require legal, compliance, privacy, safety, or leadership review.

Blame-heavy communication discourages reporting. Overly vague communication damages trust and prevents action.

### Track recurrence and close only with evidence

A defect is not remediated because action items were created. Track recurrence, audit results, rework, complaints, control performance, and staff questions after the fix. Close only when the specific correction is complete and the recurrence risk has been reduced or accepted by the right owner.

If recurrence continues, reopen the root-cause analysis rather than adding more reminders.

## Common Traps

- Jumping to root cause before containing customer, safety, financial, or compliance harm.
- Writing a defect description so vague that another person cannot verify the issue.
- Treating the person who touched the work last as the root cause.
- Using five whys mechanically until a convenient answer appears.
- Confusing correction of the affected item with prevention of recurrence.
- Assigning corrective actions without owner, due date, verification evidence, or follow-up review.
- Assuming one detected case is isolated without checking the possible affected population; choosing "training" as the default fix for tool, policy, workload, or control design problems
- Communicating defects in a way that hides impact, creates unsupported blame, or discourages reporting; closing remediation because the action was completed, not because effectiveness was verified

## Self-Check

- Has immediate impact been assessed across customers, accounts, records, payments, safety, compliance, vendors, and downstream systems?
- Are containment actions defined with owner, due date, and completion evidence?
- Is the defect described with expected state, actual state, timing, detection method, scope, and supporting evidence?
- Are proximate cause and system cause separated?
- Does the root cause explain both the event and why it was not prevented or detected earlier?
- Are correction, corrective action, and preventive action distinguished?
- Does each remediation action have owner, due date, evidence, and effectiveness verification?
- Has the potential affected population been considered beyond the detected case?
- Is communication factual, learning-oriented, and appropriate for customer, legal, compliance, privacy, or safety risk?
- Will recurrence be monitored after closure, with a trigger to reopen analysis if defects continue?
