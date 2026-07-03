---
name: remediation_tracking_and_verification.md
description: Use when the agent is tracking audit remediation, verifying management actions, retesting controls, deciding whether findings can be closed, managing open-issue logs, or evaluating whether corrective action actually fixed the root cause rather than only the symptom.
---

# Remediation Tracking And Verification

Reporting a finding is not the end of the audit cycle. Until the agreed action is implemented and verified, the risk that motivated the finding usually still exists. Remediation tracking is where many audits fail silently: management reports a fix, the auditor marks the issue closed, and the underlying control never actually operated. A policy is drafted, training is scheduled, a system setting is changed once, and the auditor accepts the change as remediation without testing whether it works in practice.

Use this skill when following up on prior audit findings, managing remediation status, verifying corrective action plans, retesting controls, deciding closure, or reporting open issues to governance. The goal is to prevent premature closure and to ensure verification evidence matches the original finding's cause and risk.

## Core Rules

### Define What Remediation Must Achieve

Before tracking status, be clear about what successful remediation looks like. Remediation is not the existence of a plan, a policy, a meeting, or a system change. Remediation is the demonstrated reduction of the risk that created the finding.

For each open finding, define:

- the original condition, cause, effect, and risk;
- the agreed corrective action and its owner;
- the control or process change expected;
- the evidence that would prove the change is in place;
- the evidence that would prove the change actually operates;
- the population and period over which operation must be shown;
- the residual risk expected after remediation.

If these are not defined, closure decisions will be subjective and inconsistent. A finding about missing approvals cannot be closed by a new policy alone; it requires evidence that approvals now occur and exceptions are caught.

### Separate Implementation From Operating Effectiveness

A common distinction auditors must preserve is the difference between a control being implemented and a control operating effectively over time.

Implementation evidence includes:

- new or revised policy issued;
- system configuration changed;
- role or access updated;
- procedure documented;
- training delivered;
- new report created;
- vendor or contract term updated.

Operating effectiveness evidence includes:

- sample of transactions showing the control operated during the period;
- logs showing the review occurred at the required frequency;
- exception tracking showing deviations were caught and followed up;
- evidence the reviewer had authority and competence;
- evidence the control operated throughout the period, not only at one point.

Do not close a finding on implementation evidence alone when the original finding concerned operating effectiveness. Match the verification to the nature of the original issue.

### Tie Verification Back To Root Cause

Remediation that fixes the symptom but not the cause will recur. When verifying, confirm the action addresses the cause identified in the original finding.

Check:

- Does the action address the control gap, design flaw, missing monitoring, unclear ownership, system limitation, incentive problem, or knowledge gap that caused the issue?
- Does it cover the whole affected population, not only the sample items that triggered the finding?
- Does it prevent recurrence, or only detect it after the fact?
- Are there compensating controls being relied upon, and have they been tested?

If management's action addresses a different problem than the original finding, the finding is not remediated. Document the gap and keep the issue open.

### Set A Verification Procedure For Each Finding

Each open finding should have a defined verification approach before closure is considered. Generic statements such as "management confirms remediation complete" are not verification.

Define:

- what population will be examined;
- what period of operation will be covered;
- what sample size or method applies;
- what attribute or evidence confirms operation;
- who performs the verification and with what independence;
- what result constitutes pass versus fail;
- whether retesting is performed by the auditor or relied upon from management's testing.

For higher-risk findings, the auditor should perform independent retesting rather than relying solely on management self-assessment. For lower-risk findings, reviewing management's evidence may be acceptable if the evidence is objective and complete.

### Apply Risk-Based Prioritization To Follow-Up

Not all findings need the same follow-up intensity. Prioritize verification effort by the original risk and by the consequences of failed remediation.

Prioritize for close verification:

- findings involving fraud, override, financial reporting, legal compliance, safety, privacy, or public funds;
- findings rated high or significant deficiency;
- findings where prior remediation attempts failed;
- findings affecting a large population or critical system;
- findings where management has a history of partial or cosmetic fixes.

Lower-risk findings may use lighter-touch verification, but the rationale for the lighter approach should be documented.

### Handle Partial And Overdue Remediation Explicitly

Remediation is frequently partial. Some actions are done, some are late, some are abandoned, and some are replaced by alternative actions. The tracker must reflect reality, not a binary open or closed.

Handle these states explicitly:

- fully implemented and verified: eligible for closure;
- implemented but not verified: keep open pending retest;
- partially implemented: keep open, document what remains;
- overdue with no credible plan: escalate;
- abandoned or replaced: assess whether the alternative is adequate;
- disputed: document the disagreement and the auditor's position.

Do not let stale items linger in "in progress" indefinitely. Set review dates and escalate items that miss milestones without credible justification.

### Escalate Non-Remediation And Recurrence

When management fails to remediate within a reasonable time, or when the same issue recurs after claimed remediation, escalation is required. This is a governance matter, not a clerical update.

Escalate when:

- a high-risk finding remains open beyond its committed date without a credible revised plan;
- the same root cause appears in a new finding;
- management disputes the finding but provides no testable counter-evidence;
- remediation evidence is missing, contradictory, or appears fabricated;
- a control that was verified as remediated later fails in subsequent testing.

Escalation paths include the audit committee, board, regulator, or executive sponsor depending on the engagement. Document the escalation, the recipient, and the response.

### Preserve Independence In Verification

The auditor who verifies remediation must be independent of both the original finding's management response and the implementation. Internal auditors should not verify their own consulting recommendations without disclosing the self-review threat.

Check:

- did the auditor help design or implement the remediation?
- is the verifier the same person who accepted management's action plan?
- does the verifier have a relationship that could bias the closure decision?
- is verification being pressured by deadlines, budget, or management preference?

If independence is impaired, disclose the threat, have a different reviewer verify, or qualify the verification conclusion.

### Document Closure With A Defensible Record

Closure should be supported by a record that an independent reviewer could understand later. The record should show what was found, what was agreed, what was done, what was tested, and why closure is now appropriate.

A closure record typically includes:

- original finding reference and summary;
- agreed action and owner;
- implementation evidence reviewed;
- verification procedure performed;
- sample, period, and result;
- residual risk after remediation;
- closure approver and date;
- any conditions or follow-up still required.

Do not delete the finding history when closing. Closed findings remain relevant for trend analysis, recurrence detection, and future risk assessment.

## Common Traps

### Closing On Policy Or Training Alone

A new policy, a signed acknowledgment, or a training session proves intent, not operation. If the original finding was about controls failing in practice, verify operation through transaction testing, logs, or exception evidence.

### Trusting Self-Reported Status

Management status updates that say "complete" without supporting evidence are not verification. Request objective evidence and test a sample where risk warrants it.

### Verifying Only The Sample That Failed

If the original finding came from a sample, remediation must cover the whole population, not only the specific exceptions. Verify that the corrective action applies broadly and operates across the relevant population and period.

### Accepting A Fix For The Wrong Cause

Management may implement a control that addresses a convenient symptom while leaving the root cause untouched. Compare the action to the documented cause, not to management's preferred framing of the problem.

### Premature Closure To Meet A Deadline

Reporting a clean remediation status by a target date can pressure premature closure. The verification must follow the evidence, not the calendar. If evidence is incomplete, keep the finding open and report it as such.

### Losing Track Of Overdue Items

Open-issue logs that are never reviewed become graveyards. Items sit in "in progress" for years. Schedule periodic status reviews, flag aging items, and escalate those without movement.

### Recurrence Treated As A New Issue

When a finding recurs, it often means the prior remediation was inadequate. Investigate why the earlier closure was allowed, whether verification was sufficient, and whether the root cause was truly addressed. Do not simply open a new finding and ignore the history.

### Letting Management Define Closure Criteria

Management may argue a finding is closed because they believe the action is sufficient. The auditor owns the closure decision based on verification evidence. Document disagreements and retain the auditor's conclusion.

## Self-Check

- For each open finding, is the expected remediation outcome defined in terms of risk reduction, not just activity?
- Is implementation evidence distinguished from operating effectiveness evidence, and does the verification match the original finding's nature?
- Does the verification confirm the action addresses the documented root cause and covers the full affected population?
- Does each finding have a defined verification procedure specifying population, period, sample, attribute, and pass or fail criteria?
- Is verification effort prioritized by risk, with independent retesting for higher-risk findings?
- Are partial, overdue, abandoned, and disputed remediation states tracked explicitly rather than collapsed into a single status?
- Are non-remediation and recurrence escalated to the appropriate governance level with documented response?
- Has independence in verification been considered, including any auditor involvement in designing or implementing the fix?
- Does each closure record include original finding, agreed action, implementation evidence, verification result, residual risk, and approver?
- Are closed findings retained for trend and recurrence analysis rather than deleted?
