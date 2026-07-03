---
name: corrective_action_verification.md
description: Use when the agent is verifying whether management's corrective actions actually remediated an audit finding, distinguishing implementation from operating effectiveness, testing whether the fix addressed root cause, evaluating remediation evidence, or deciding whether a finding can be closed based on verified evidence rather than management self-report.
---

# Corrective Action Verification

Reporting a finding does not reduce the risk it identified; only a corrective action that actually works does. And the gap between a corrective action being claimed and a corrective action being verified is where most remediation silently fails. Management reports that a fix is in place: a policy was issued, a system setting changed, training delivered. The auditor marks the finding closed. Later, the same problem recurs, because the policy was never followed, the setting was changed back, or the training changed nothing. Verification is the procedure that closes this gap, by testing whether the corrective action is actually in place, whether it operates as intended, and whether it addresses the root cause rather than the symptom. The skill is in defining what successful remediation looks like, matching the verification to the nature of the original finding, and refusing to close a finding on activity or self-report alone.

Use this skill when verifying corrective actions, deciding whether remediation is sufficient, distinguishing implementation from operating effectiveness, testing root cause coverage, or determining whether a finding can be closed. The goal is to close findings only on verified evidence that the risk has actually been reduced.

## Core Rules

### Define What Successful Remediation Looks Like Before Verifying

Verification requires a clear definition of what would constitute successful remediation. Without it, closure decisions become subjective and inconsistent. Define success in terms of risk reduction, not activity.

For each finding, define:

- the original condition, cause, effect, and risk;
- the corrective action agreed and its owner;
- the control or process change expected;
- the evidence that would prove the change is in place;
- the evidence that would prove the change actually operates;
- the population and period over which operation must be shown;
- the residual risk expected after remediation.

If these are undefined, closure will be based on management's assertion rather than on a testable standard. A finding about missing approvals cannot be closed by a new policy alone; it requires evidence that approvals now occur.

### Distinguish Implementation From Operating Effectiveness

A critical distinction in verification is whether a control is implemented versus whether it operates effectively over time. Implementation evidence proves the control exists; operating effectiveness evidence proves it works in practice.

Implementation evidence includes:

- a new or revised policy issued;
- a system configuration changed;
- a role or access updated;
- a procedure documented;
- training delivered.

Operating effectiveness evidence includes:

- a sample of transactions showing the control operated during the period;
- logs showing review occurred at the required frequency;
- exception tracking showing deviations were caught and followed up;
- evidence the reviewer had authority and competence;
- evidence the control operated throughout the period, not at one point.

Do not close a finding on implementation evidence alone when the original finding concerned operating effectiveness. Match the verification to the nature of the original issue.

### Verify The Action Addresses The Root Cause

Remediation that fixes the symptom but not the cause will recur. When verifying, confirm the action addresses the cause identified in the original finding, not a more convenient problem.

Check that the action:

- addresses the control gap, design flaw, missing monitoring, or knowledge gap that caused the issue;
- covers the whole affected population, not only the sample items that triggered the finding;
- prevents recurrence, or at least detects it reliably, rather than only reacting;
- includes any compensating controls being relied upon, which must themselves be tested.

If management's action addresses a different problem than the original finding, the finding is not remediated. Document the gap and keep the issue open.

### Require Objective Evidence, Not Self-Report

Management status updates that say "complete" are not verification. Verification requires objective evidence that the auditor can examine and test.

Require evidence such as:

- system configurations or reports showing the control in operation;
- samples of transactions demonstrating the control operated;
- logs or sign-offs showing review at the required frequency;
- exception records showing deviations were caught and followed up;
- independent confirmation rather than management's summary assertion.

For higher-risk findings, perform independent retesting rather than relying on management's self-assessment. For lower-risk findings, reviewing management's objective evidence may be acceptable if it is complete and verifiable, with the rationale documented.

### Match Verification Depth To Risk

Not all findings warrant the same verification intensity. Depth should be driven by the original risk and the consequences of failed remediation.

Verify more deeply when:

- the finding involved fraud, override, financial reporting, legal compliance, safety, privacy, or public funds;
- the finding was rated high or significant deficiency;
- prior remediation attempts for the issue failed;
- the finding affects a large population or critical system;
- management has a history of partial or cosmetic fixes.

Lower-risk findings may use lighter verification, but the rationale for the lighter approach should be documented, not assumed. The risk of the original finding, not the convenience of the verifier, sets the depth.

### Verify Over An Appropriate Period

A control verified at a single point may not operate over time. For operating effectiveness, verification should cover a period sufficient to show sustained operation.

Verify over an appropriate period by:

- testing operation across multiple points in time, not only immediately after implementation;
- including periods after the change has settled, when workarounds may reappear;
- for automated controls, confirming the configuration remained in place and was not reverted;
- considering whether the control operated for the full period since remediation.

A control that works in the first week and fails thereafter has not remediated the finding. Period coverage matters as much as point-in-time confirmation.

### Document The Verification And Closure Decision

Closure should be supported by a record that an independent reviewer could understand later, showing what was found, what was agreed, what was tested, and why closure is appropriate.

A closure record typically includes:

- the original finding reference and summary;
- the agreed action and owner;
- the implementation evidence reviewed;
- the verification procedure performed, including sample, period, and result;
- the residual risk after remediation;
- the closure approver and date;
- any conditions or follow-up still required.

Do not delete finding history when closing. Closed findings remain relevant for trend analysis, recurrence detection, and future risk assessment.

## Common Traps

### Closing On Policy Or Training Alone

A new policy or training session proves intent, not operation. If the finding concerned controls failing in practice, verify operation through transaction testing or logs.

### Trusting Self-Reported Status

Management's "complete" without supporting evidence is not verification. Require objective, testable evidence.

### Verifying Only The Sample That Failed

If the finding came from a sample, remediation must cover the whole population. Verify the corrective action applies broadly and operates across the relevant population and period.

### Accepting A Fix For The Wrong Cause

Management may implement a control addressing a convenient symptom while leaving the root cause untouched. Compare the action to the documented cause.

### Single-Point Verification

A control verified once immediately after implementation may not operate over time. Verify across an appropriate period.

### Premature Closure To Meet A Deadline

Reporting clean remediation status by a target date pressures premature closure. Verification must follow the evidence, not the calendar.

### Uniform Verification Depth

Applying the same depth to all findings misallocates effort. Match depth to risk and document the rationale.

### Deleting Closed Findings

Closed findings remain relevant for trends and recurrence. Retain the history.

## Self-Check

- For each finding, is successful remediation defined in terms of risk reduction, with the expected control change and evidence specified?
- Is implementation evidence distinguished from operating effectiveness evidence, and does the verification match the original finding's nature?
- Does the verification confirm the action addresses the documented root cause and covers the full affected population?
- Is objective, testable evidence required rather than management self-report, with independent retesting for higher-risk findings?
- Is verification depth matched to the original risk, with the rationale for any lighter approach documented?
- Does verification cover an appropriate period, showing sustained operation rather than only a single point after implementation?
- Does each closure record include the original finding, agreed action, implementation evidence, verification result, residual risk, and approver?
- Are closed findings retained for trend and recurrence analysis rather than deleted?
- Would an independent reviewer concur that the verification supports closure based on the documented evidence?
