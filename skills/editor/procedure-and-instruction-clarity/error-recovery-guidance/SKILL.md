---
name: error_recovery_guidance.md
description: Use when the agent is editing procedural or instructional content and must verify that guidance for handling errors, unexpected results, and failure recovery is provided, checking that users know what to do when a step does not produce the expected result and how to recover without losing work or causing harm.
---

# Error Recovery Guidance

Procedures assume success, but users encounter errors. A step produces an unexpected result, a system behaves differently than described, a prerequisite turns out to be missing. Without error recovery guidance, users are stranded, unsure whether to retry, skip, seek help, or abandon the procedure, sometimes after causing partial changes they cannot undo. Error recovery guidance is the discipline of anticipating where procedures fail and telling users what to do when they do. Editors who edit only the happy path leave users helpless at the first deviation.

Use this skill when editing procedures or instructional content to verify error recovery guidance. It covers anticipating failure points, providing recovery actions, and ensuring users can safely navigate unexpected results. The goal is a procedure that remains usable when things go wrong, not only when they go right.

## Core Rules

### Anticipate Common Failure Points

Error recovery begins with anticipating where the procedure commonly fails. Some steps are error-prone due to complexity, environmental variability, or common mistakes. Identify and address these. Anticipate failures.

Review the procedure for steps that commonly fail. These include steps dependent on network conditions, system configuration, user permissions, or precise inputs. They include steps where users frequently misread, mistype, or misunderstand. For each anticipated failure point, consider what goes wrong and why. Anticipation allows you to provide recovery guidance at the point of failure, rather than leaving users to improvise. Procedures that address only the happy path fail users at the first error; procedures that anticipate failures keep users moving.

### State What To Do When Expected Results Do Not Appear

For key steps where expected results are stated, provide guidance for when those results do not appear. A user who follows a step correctly but does not see the expected result needs to know what it means and what to do. Provide deviation guidance.

For each step with a stated expected result, add brief guidance for the case where the result differs. If the expected save confirmation does not appear, what might be wrong, and what should the user do? If the connection indicator stays red instead of turning green, what are the likely causes and remedies? Deviation guidance turns a dead end into a diagnosable situation. Without it, users who do not see expected results may retry blindly, skip ahead, or abandon the procedure, sometimes after partial changes. With it, users can troubleshoot or decide on an appropriate next step.

### Provide Recovery Actions For Irreversible Or Risky Steps

For steps that are irreversible or carry risk, provide explicit recovery actions if something goes wrong. Users need to know how to back out, restore, or undo before they encounter the problem. Provide recovery.

For risky or irreversible steps, describe what to do if the step fails or produces an unwanted result. If a step deletes data, describe how to restore from backup. If a step changes configuration, describe how to revert. If a step cannot be undone, state this explicitly so the user understands the commitment before acting. Recovery actions should be specific and testable, not vague assurances. The more consequential the step, the more detailed the recovery guidance should be. Users who know how to recover are more confident proceeding; users who do not may either avoid necessary steps or blunder through dangerous ones.

### Distinguish User Errors From System Or Environmental Issues

Errors arise from different sources: user mistakes, system problems, environmental conditions, or procedure flaws. Recovery guidance should help users distinguish these, because the remedy differs. Distinguish error sources.

Where an error might arise from multiple sources, help the user diagnose. For example, if a connection fails, the guidance might note that the most common cause is incorrect credentials, followed by network issues, followed by service outages, and how to check each. Distinguishing sources prevents users from repeating a wrong action, blaming themselves for a system problem, or giving up when a retry would work. Diagnostic guidance turns a confusing failure into a structured troubleshooting process.

### Provide Escape Hatches And escalation Paths

Some errors cannot be resolved by the user following written guidance. The procedure should provide escape hatches, ways to safely stop or pause, and escalation paths, how to get help. Provide escapes.

Identify points where a user may be unable to proceed and provide guidance for those situations. This might include how to safely exit the procedure partway through, what state to leave the system in, and how to resume later. It includes how and when to contact support, what information to provide, and what alternatives exist. Escape hatches prevent users from making desperate choices when stuck, such as force-quitting, hard-resetting, or improvising destructive actions. Escalation paths ensure users know that help exists and how to reach it. Procedures without escape hatches leave stuck users to their own devices, which can cause harm.

### Use Consistent Error And Warning Formatting

Error recovery guidance is most useful when users can find and recognize it quickly. Consistent formatting for errors, warnings, and troubleshooting helps users locate guidance when they need it. Format consistently.

Use consistent visual or structural conventions for error guidance. This might include a distinct callout style for warnings, a consistent heading for troubleshooting sections, or a standard format for "If this happens, do that" entries. Consistent formatting allows users who are skimming for help during a failure to find it quickly, rather than reading the whole procedure. Inconsistent or buried error guidance may as well not exist, because stressed users will not find it. Make recovery guidance visually distinct and consistently placed.

### Calibrate Recovery Detail To Risk And Likelihood

Not every step needs extensive error recovery guidance. Calibrate the detail to the risk of failure and the consequence of error. High-risk, high-likelihood steps need thorough guidance; low-risk steps need little. Calibrate.

Assess each step's likelihood of failure and the consequence if it fails. Provide detailed recovery guidance for steps that are likely to fail or whose failure is costly. Provide brief or no guidance for steps that rarely fail or whose failure is trivial. Over-guidance clutters the procedure and buries important recovery information; under-guidance leaves users stranded at real failure points. Calibration ensures the recovery guidance matches the actual risk profile of the procedure.

## Common Traps

### Editing Only The Happy Path

Procedures fail. Anticipate and address error points.

### No Guidance For Unexpected Results

Tell users what to do when expected results do not appear.

### Irreversible Steps Without Recovery Actions

Provide explicit recovery or clearly state irreversibility before the step.

### Conflating User And System Errors

Help users diagnose the source, because remedies differ.

### No Escape Hatch Or Escalation Path

Provide ways to safely stop and to get help when stuck.

### Inconsistent Or Buried Error Formatting

Make recovery guidance visually distinct and consistently placed.

### Uniform Recovery Detail Regardless Of Risk

Calibrate guidance to the likelihood and consequence of failure.

## Self-Check

Before treating error recovery guidance as complete, verify:

- Common failure points have been anticipated and addressed with recovery guidance.
- For key steps with expected results, guidance exists for when those results do not appear.
- Irreversible or risky steps have explicit recovery actions or clear irreversibility warnings.
- Error sources are distinguished so users can diagnose user errors versus system issues.
- Escape hatches allow users to safely stop or pause, with guidance on state and resumption.
- Escalation paths tell users how and when to get help and what information to provide.
- Error, warning, and troubleshooting guidance uses consistent, findable formatting.
- Recovery detail is calibrated to the risk and likelihood of failure at each point.
- A user who deviates from the happy path could find guidance on what to do next.
- The procedure remains usable and safe when things go wrong, not only when they go right.
